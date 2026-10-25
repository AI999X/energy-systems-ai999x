"""Energy systems procedures: deterministic capabilities with typed inputs and several outputs (domain code, loaded by
the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: solar resource and photovoltaics (sun position, irradiance on tilted planes, clear-sky and decomposition
models, cell temperature, the single-diode model, PVWatts), wind power (power in the wind, actuator-disc momentum
theory, wind shear, air density at hub height, power curves, Weibull statistics and annual energy), power systems
(three-phase power, per-unit quantities, line performance, load flow, economic dispatch, faults, stability),
transformers and machines, batteries, fuel cells and hydrogen.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, cmath, re, itertools) plus mpmath where stated.

How the references differ from the shipped code:
- solar geometry, irradiance, cell temperature, PV and inverter models: pvlib's own implementations;
- wind shear, air density, power curves, power-coefficient power: windpowerlib's own implementations; Weibull
  statistics and energy by scipy quadrature (scipy.stats.weibull_min) and root finding;
- load flow, DC power flow, transformer loading and parallel sharing, power transfer: pandapower network solves;
- line, machine and transformer circuits: nodal network solves (numpy), telegrapher equations integrated by scipy,
  power (energy) balances, or the phase-domain circuit for unsymmetrical faults;
- dispatch, sizing and optimisation results: enumeration of the optimality (KKT) conditions, scipy.optimize (milp,
  brentq) or sympy;
- hydrogen properties: CoolProp (Leachman 2009 equation of state for normal hydrogen).

Conventions (fixed, so run and reference agree exactly):
- Angles are given and reported in degrees unless the name ends in _rad. Solar azimuths are measured clockwise from
  north (east 90, south 180, west 270); longitudes are positive east; hour angles are negative in the morning.
- Day of year n runs from 1 (1 January). Day angle B = 2 pi (n - 1)/365 (Spencer). Cooper's declination is
  23.45 sin(360 (284 + n)/365) degrees. Where a method is not named: declination and equation of time by Spencer
  (1971), extraterrestrial irradiance by Spencer with solar constant 1366.1 W/m^2 (pvlib's default), air mass by
  Kasten and Young (1989).
- Temperatures are read in K or degC; cell temperatures are reported in K (T_cell) and in degC (T_cell_C).
- Three-phase quantities: voltages are line-to-line rms, powers are three-phase totals, impedances are per phase.
  Power factor is positive; lagging (inductive) unless stated "leading".
- Per-unit: three-phase base S_base, line-to-line base voltage V_base, Z_base = V_base^2/S_base.
- Battery capacities are in A*h (written "Ah" is converted here: 1 Ah = 3600 C); battery current is positive on
  discharge.
- Hydrogen: molar mass 2.01588 g/mol, Faraday constant 96485.33212 C/mol, lower heating value 241.826 kJ/mol
  (119.96 MJ/kg) and higher heating value 285.826 kJ/mol (141.79 MJ/kg) at 25 degC.
- Results built from cos, sin, exp or log are rounded to 12 significant digits (identical on every platform) and
  values that are zero by physics are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Solar declination: n = 172`); physical values carry a unit, angles are plain numbers in degrees (an optional
"deg" is accepted), percentages may be written with %. Units the shared unit reader lacks are converted here: hPa,
mbar, psi, Ah and mAh. Lists are written in brackets (`speeds = [3, 5, 8, 12]`) and carry the unit stated in the
input's meaning. Any other number or an unknown name is refused.
"""
from __future__ import annotations

import cmath
import itertools
import math
import re

# ================================================================================================ constants
F_FARADAY = 96485.33212                 # C/mol (CODATA 2018, exact from e and N_A)
M_H2 = 2.01588e-3                       # kg/mol
M_O2 = 31.9988e-3                       # kg/mol
M_H2O = 18.01528e-3                     # kg/mol
LHV_H2 = 241826.0                       # J/mol (water vapour product, 25 degC)
HHV_H2 = 285826.0                       # J/mol (liquid water product, 25 degC)
K_B_EV = 8.617333262145179e-05          # Boltzmann constant in eV/K (CODATA 2018)
DEG = math.pi / 180.0
SOLAR_CONSTANT = 1366.1                 # W/m^2 (pvlib default)


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, complex):
        return complex(_r(x.real), _r(x.imag))
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = complex(a), complex(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    s = max(abs(a), abs(b))
    return d / s if s > 1e-300 else math.inf


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            d = _rel(outputs[k], v)
            if d > 0 and isinstance(v, (int, float)) and not isinstance(v, bool) and abs(v) < 1e-15 \
                    and abs(outputs[k]) < 1e-15:
                d = 0.0                                  # both zero to within the comparison's floor
            worst = max(worst, d)
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(name, v):
    _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), f"{name} must be a finite number")
    return float(v)


def _pos(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and math.isfinite(v) and v > 0, f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and math.isfinite(v) and v >= 0, f"{name} must be zero or positive")


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _bisect(f, a, b, n=400):
    """Root of f on [a, b] by bisection (f(a), f(b) of opposite signs), to full double precision."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa > 0) != (fb > 0), "no root in the interval")
    for _ in range(n):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


def _illinois(f, a, b, n=200):
    """Root of f on [a, b] (opposite signs) by the Illinois variant of regula falsi, to full double precision; a step
    that would leave the bracket or stall falls back to bisection."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa > 0) != (fb > 0), "no root in the interval")
    side = 0
    for _ in range(n):
        c = (a * fb - b * fa) / (fb - fa)
        if not (min(a, b) < c < max(a, b)):
            c = 0.5 * (a + b)
        fc = f(c)
        if fc == 0 or abs(b - a) <= 4e-16 * max(abs(a), abs(b)):
            return c
        if (fc > 0) == (fb > 0):
            b, fb = c, fc
            if side == -1:
                fa *= 0.5
            side = -1
        else:
            a, fa = c, fc
            if side == 1:
                fb *= 0.5
            side = 1
    return c


def _gauss_solve(A, b):
    """Solve A x = b (real or complex) by Gaussian elimination with partial pivoting; singular -> ValueError."""
    n = len(A)
    M = [list(map(lambda v: v, row)) + [b[i]] for i, row in enumerate(A)]
    scale = max((abs(x) for row in A for x in row), default=1.0) or 1.0
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        _need(abs(M[p][c]) > 1e-13 * scale, "the system is singular: the network does not determine every unknown")
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f != 0:
                for k in range(c, n + 1):
                    M[r][k] -= f * M[c][k]
    x = [0.0] * n
    for r in reversed(range(n)):
        x[r] = (M[r][n] - sum(M[r][k] * x[k] for k in range(r + 1, n))) / M[r][r]
    return x


# --- the one documented question form: '<phrase>: name = value, ...' --------------------------------------------
_CONV = {"hPa": (100.0, "Pa"), "mbar": (100.0, "Pa"), "mb": (100.0, "Pa"), "psi": (6894.757293168361, "Pa"),
         "Ah": (3600.0, "C"), "mAh": (3.6, "C"), "kAh": (3.6e6, "C"), "A*h": (3600.0, "C")}
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\](?:\s*;\s*\[[^\]]*\])*|\[\[.*\]\])\s*$")
_CLOCK = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\d{1,2}):(\d{2})\s*$")
DEG_WORDS = ("deg", "°", "degrees", "degree")
_TEXT_UNIT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _pairs(question, phrase, names, phys=(), words=(), vectors=(), num_units=None, clocks=(), matrices=()):
    """{symbol: value} from '<phrase>: name = value, ...'. None unless the phrase opens the question and every part is
    a 'name = value' pair of a known name. `names` maps written names to symbols. `phys`: symbols that need a unit
    (returned as 'value unit' strings, converted here when the shared reader lacks the unit); `words`: symbols whose
    value is a word (a choice); `vectors`/`matrices`: symbols whose value is a bracketed list (returned as text, read
    by the runtime); `num_units`: {symbol: [allowed unit words]} for plain numbers (degrees, percent); `clocks`:
    symbols that may be written hh:mm (read as decimal hours)."""
    if not isinstance(question, str):
        return None
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", question, re.I)
    if not m:
        return None
    rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
    if not rest:
        return {}
    out = {}
    num_units = num_units or {}
    for seg in _split(rest):
        if not seg.strip():
            continue
        mv = _VEC.match(seg)
        if mv:
            key = names.get(mv.group(1), names.get(mv.group(1).lower()))
            if key is None or key in out or key not in set(vectors) | set(matrices):
                return None
            text = mv.group(2).strip()
            if key in matrices:
                text = text.strip()
                if text.startswith("[["):
                    text = text[1:-1]
                rows = re.findall(r"\[([^\]]*)\]", text)
                text = "; ".join(r.strip() for r in rows)
            out[key] = text
            continue
        mc = _CLOCK.match(seg)
        if mc:
            key = names.get(mc.group(1), names.get(mc.group(1).lower()))
            if key is None or key in out or key not in set(clocks):
                return None
            hh, mm = int(mc.group(2)), int(mc.group(3))
            if mm >= 60:
                return None
            out[key] = hh + mm / 60.0
            continue
        mw = _WORD.match(seg)
        if mw:
            key = names.get(mw.group(1), names.get(mw.group(1).lower()))
            if key in set(words):
                if key in out:
                    return None
                out[key] = mw.group(2).strip().lower().replace(" ", "_").replace("-", "_")
                continue
        ms = _SEG.match(seg)
        if not ms:
            return None
        key = names.get(ms.group(1), names.get(ms.group(1).lower()))
        if key is None or key in out or key in words or key in vectors or key in matrices:
            return None
        num, unit = ms.group(2), ms.group(3).strip()
        if key in phys:
            if not unit:
                return None
            if unit in _CONV:
                fac, si = _CONV[unit]
                out[key] = f"{float(num) * fac!r} {si}"
            else:
                out[key] = f"{num} {unit}"
        else:
            if unit:
                allowed = num_units.get(key, ())
                if unit in _TEXT_UNIT and unit in allowed:
                    out[key] = float(num) * _TEXT_UNIT[unit]
                    continue
                if unit not in allowed:
                    return None
            out[key] = float(num)
    return out


def _phys(sym, kind, meaning, required=True):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}


def _numin(sym, meaning, required=True, **extra):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=50, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _matin(sym, meaning, required=True):
    return {"symbol": sym, "type": "matrix", "kind": "number", "meaning": meaning, "required": required}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    spec.setdefault("verify", _recheck(spec["run"]))
    PROCEDURES[key] = spec


def _deg_units(*syms):
    return {s: DEG_WORDS for s in syms}


def _pct_units(*syms):
    return {s: ("%", "percent") for s in syms}


def _cosd(x):
    return math.cos(math.radians(x))


def _sind(x):
    return math.sin(math.radians(x))


_PP_EMPTY: dict = {}


def _pp_empty(sn_mva=1.0, f_hz=50.0):
    """A fresh empty pandapower network for a reference: pandapower takes about 0.2 s to build one, a deep copy of a
    kept empty network about 0.02 s (certification speed only; the network is the same)."""
    import copy
    import pandapower as pp
    if f_hz not in _PP_EMPTY:
        _PP_EMPTY[f_hz] = pp.create_empty_network(sn_mva=1.0, f_hz=f_hz)
    net = copy.deepcopy(_PP_EMPTY[f_hz])
    net.sn_mva = sn_mva
    return net


def _np_scalar(x):
    """A float from a numpy scalar, 0-d array, 1-element array or pandas object."""
    try:
        import numpy as np
        a = np.asarray(x, dtype=float).ravel()
        return float(a[0])
    except Exception:
        return float(x)


def _degC(T):
    """degC from K (the cell-temperature models of pvlib are written in degC)."""
    return T - 273.15


# ================================================================================================ SOLAR GEOMETRY
def _day_angle(n, offset=1):
    return (2.0 * math.pi / 365.0) * (n - offset)


def _check_day(n):
    _need(isinstance(n, int) and 1 <= n <= 366, "the day of the year n must be a whole number from 1 to 366")


def _decl_cooper(n):
    return math.radians(23.45 * math.sin(_day_angle(n) + (2.0 * math.pi / 365.0) * 285.0))


def _decl_spencer(n):
    b = _day_angle(n)
    return (0.006918 - 0.399912 * math.cos(b) + 0.070257 * math.sin(b) - 0.006758 * math.cos(2. * b)
            + 0.000907 * math.sin(2. * b) - 0.002697 * math.cos(3. * b) + 0.00148 * math.sin(3. * b))


def _eot_spencer(n):
    b = _day_angle(n)
    return (1440.0 / 2 / math.pi) * (0.0000075 + 0.001868 * math.cos(b) - 0.032077 * math.sin(b)
                                     - 0.014615 * math.cos(2.0 * b) - 0.040849 * math.sin(2.0 * b))


def _eot_pvcdrom(n):
    bd = _day_angle(n) - (2.0 * math.pi / 365.0) * 80.0
    return 9.87 * math.sin(2.0 * bd) - 7.53 * math.cos(bd) - 1.5 * math.sin(bd)


def solar_declination(n, method="cooper"):
    _check_day(n)
    d = _decl_cooper(n) if method == "cooper" else _decl_spencer(n)
    return _out(declination_deg=math.degrees(d), declination_rad=d)


def _ref_declination(n, method="cooper"):
    from pvlib import solarposition as sp
    d = _np_scalar(sp.declination_cooper69(float(n)) if method == "cooper" else sp.declination_spencer71(float(n)))
    return {"declination_deg": math.degrees(d), "declination_rad": d}


def _wrong_declination(n, method="cooper"):
    if method == "cooper":                              # the 284-day shift forgotten
        d = math.radians(23.45 * math.sin(2 * math.pi * n / 365.0))
    else:                                               # the day angle taken as 2 pi n/365 instead of 2 pi (n - 1)/365
        b = 2 * math.pi * n / 365.0 + 0.3
        d = 0.006918 - 0.399912 * math.cos(b) + 0.070257 * math.sin(b)
    return _out(declination_deg=math.degrees(d), declination_rad=d)


_DAY = _intin("n", "day of the year (1 = 1 January)", 1, 366)
_N_NAMES = {"n": "n", "day": "n", "day_of_year": "n", "doy": "n"}

_add("energy_solar_declination",
     name="Solar declination on a day of the year", topic="solar resource / sun position",
     purpose="The declination of the sun (the latitude where the sun is overhead at noon) on day n of the year, by Cooper's equation or Spencer's Fourier series.",
     inputs=[_DAY, _choice("method", ["cooper", "spencer"], "formula: cooper (Cooper 1969, the textbook default) or spencer (Spencer 1971)", required=False)],
     outputs=[_o("declination_deg", "number", "declination in degrees"), _o("declination_rad", "number", "declination in radians")],
     primary="declination_deg", conditions=["1 <= n <= 366"],
     method="Cooper: delta = 23.45 sin(360 (284 + n)/365) degrees; Spencer: a Fourier series in the day angle B = 2 pi (n - 1)/365",
     latex=r"\delta = 23.45^\circ \sin\!\left(\frac{360^\circ (284 + n)}{365}\right)",
     run=lambda n, method="cooper": solar_declination(n, method),
     parse=lambda q: _pairs(q, r"solar declination|declination of the sun|sun declination|declination", dict(_N_NAMES, method="method"), words={"method"}),
     reference=_ref_declination, wrong=_safe(_wrong_declination),
     random_inputs=lambda rng: {"n": rng.randint(1, 365), "method": rng.choice(["cooper", "spencer"])},
     templates=["Solar declination: n = 172", "Solar declination: n = 355, method = spencer"],
     examples=[{"inputs": {"n": 172}, "outputs": {"declination_deg": _r(23.45 * math.sin(2 * math.pi * 456 / 365))}},
               {"inputs": {"n": 1, "method": "spencer"}, "outputs": {"declination_rad": _r(0.006918 - 0.399912 - 0.006758 - 0.002697)}}],
     questions=[{"q": "Solar declination: n = 172", "outputs": {"declination_deg": _r(23.45 * math.sin(2 * math.pi * 456 / 365))}},
                {"q": "Solar declination: day = 1, method = spencer", "outputs": {"declination_rad": _r(0.006918 - 0.399912 - 0.006758 - 0.002697)}},
                {"q": "Solar declination: n = 400", "outcome": "INVALID_INPUT"},
                {"q": "Solar declination: n = 172, latitude = 40", "outcome": "UNRECOGNIZED_FORMAT"},
                {"q": "Solar declination:", "outcome": "NEEDS_INPUT"}])


def equation_of_time(n, method="spencer"):
    _check_day(n)
    e = _eot_spencer(n) if method == "spencer" else _eot_pvcdrom(n)
    return _out(eot_min=e, eot_s=60.0 * e)


def _ref_eot(n, method="spencer"):
    from pvlib import solarposition as sp
    e = _np_scalar(sp.equation_of_time_spencer71(float(n)) if method == "spencer" else sp.equation_of_time_pvcdrom(float(n)))
    return {"eot_min": e, "eot_s": 60 * e}


_add("energy_equation_of_time",
     name="Equation of time", topic="solar resource / solar time",
     purpose="The equation of time (apparent minus mean solar time) on day n, by Spencer's Fourier series or the PVCDROM formula 9.87 sin 2B - 7.53 cos B - 1.5 sin B.",
     inputs=[_DAY, _choice("method", ["spencer", "pvcdrom"], "formula: spencer (default) or pvcdrom (B = 360 (n - 81)/365)", required=False)],
     outputs=[_o("eot_min", "number", "equation of time in minutes"), _o("eot_s", "number", "equation of time in seconds")],
     primary="eot_min", conditions=["1 <= n <= 366"],
     method="Spencer: E = 229.18 (0.000075 + 0.001868 cos B - 0.032077 sin B - 0.014615 cos 2B - 0.040849 sin 2B), B = 2 pi (n - 1)/365",
     latex=r"E = 229.18\,(0.000075 + 0.001868\cos B - 0.032077\sin B - 0.014615\cos 2B - 0.040849\sin 2B)",
     run=lambda n, method="spencer": equation_of_time(n, method),
     parse=lambda q: _pairs(q, r"equation of time", dict(_N_NAMES, method="method"), words={"method"}),
     reference=_ref_eot,
     wrong=_safe(lambda n, method="spencer": _out(eot_min=-equation_of_time(n, method)["eot_min"], eot_s=-equation_of_time(n, method)["eot_s"])),
     random_inputs=lambda rng: {"n": rng.randint(1, 365), "method": rng.choice(["spencer", "pvcdrom"])},
     templates=["Equation of time: n = 45", "Equation of time: n = 300, method = pvcdrom"],
     examples=[{"inputs": {"n": 81, "method": "pvcdrom"}, "outputs": {"eot_min": -7.53}},
               {"inputs": {"n": 1}, "outputs": {"eot_min": _r((1440.0 / 2 / math.pi) * (0.0000075 + 0.001868 - 0.014615))}}],
     questions=[{"q": "Equation of time: n = 81, method = pvcdrom", "outputs": {"eot_min": -7.53}},
                {"q": "Equation of time: n = 0", "outcome": "INVALID_INPUT"}])


def _hour_angle(n, clock, longitude, utc_offset):
    return 15.0 * (clock - utc_offset - 12.0) + longitude + _eot_spencer(n) / 4.0


def _check_place(latitude=None, longitude=None, utc_offset=None):
    if latitude is not None:
        _need(isinstance(latitude, (int, float)) and -90 < latitude < 90, "the latitude must lie between -90 and 90 degrees")
    if longitude is not None:
        _need(isinstance(longitude, (int, float)) and -180 <= longitude <= 180, "the longitude must lie between -180 and 180 degrees (east positive)")
    if utc_offset is not None:
        _need(isinstance(utc_offset, (int, float)) and -12 <= utc_offset <= 14, "the UTC offset must lie between -12 and +14 hours")


def solar_time_hour_angle(n, clock, longitude, utc_offset):
    _check_day(n)
    _check_place(longitude=longitude, utc_offset=utc_offset)
    _need(0 <= clock < 24, "the clock time must be from 0 to 24 hours (decimal hours or hh:mm)")
    eot = _eot_spencer(n)
    ha = 15.0 * (clock - utc_offset - 12.0) + longitude + eot / 4.0
    return _out(eot_min=eot, hour_angle_deg=ha, solar_time_h=12.0 + ha / 15.0)


def _local_index(n, clock, utc_offset, year=2023):
    import datetime as dt
    import pandas as pd
    tz = dt.timezone(dt.timedelta(minutes=round(utc_offset * 60)))
    t0 = pd.Timestamp(year=year, month=1, day=1, tz=tz) + pd.Timedelta(days=n - 1) + pd.Timedelta(minutes=round(clock * 60, 9))
    return pd.DatetimeIndex([t0])


def _ref_hour_angle(n, clock, longitude, utc_offset):
    from pvlib import solarposition as sp
    eot = _np_scalar(sp.equation_of_time_spencer71(float(n)))
    ha = _np_scalar(sp.hour_angle(_local_index(n, clock, utc_offset), longitude, eot))
    return {"eot_min": eot, "hour_angle_deg": ha, "solar_time_h": 12 + ha / 15}


_PLACE_NAMES = {"latitude": "latitude", "lat": "latitude", "phi": "latitude", "longitude": "longitude", "lon": "longitude",
                "utc_offset": "utc_offset", "utc": "utc_offset", "timezone": "utc_offset", "tz": "utc_offset"}

_add("energy_solar_time_hour_angle",
     name="Solar time and hour angle from clock time", topic="solar resource / solar time",
     purpose="Local apparent solar time and the hour angle of the sun from the local clock time, the longitude and the time zone (UTC offset), with the equation of time (Spencer).",
     inputs=[_DAY, _numin("clock", "local clock (standard) time in decimal hours or hh:mm"),
             _numin("longitude", "longitude in degrees, east positive"), _numin("utc_offset", "time zone as hours from UTC (e.g. -5, 1, 5.5)")],
     outputs=[_o("eot_min", "number", "equation of time in minutes"), _o("hour_angle_deg", "number", "hour angle in degrees (negative before solar noon)"),
              _o("solar_time_h", "number", "apparent solar time in decimal hours")],
     primary="hour_angle_deg", conditions=["1 <= n <= 366", "0 <= clock < 24", "-180 <= longitude <= 180"],
     method="omega = 15 (t_clock - UTC offset - 12) + longitude + E/4 (degrees); solar time = 12 + omega/15",
     latex=r"\omega = 15^\circ\,(t - \Delta t_{UTC} - 12) + \lambda + \frac{E}{4}",
     run=solar_time_hour_angle,
     parse=lambda q: _pairs(q, r"solar time and hour angle|hour angle|solar time", dict(_N_NAMES, clock="clock", time="clock", **_PLACE_NAMES),
                            num_units=_deg_units("longitude"), clocks={"clock"}),
     reference=_ref_hour_angle,
     wrong=_safe(lambda n, clock, longitude, utc_offset: (lambda r: _out(eot_min=r["eot_min"], hour_angle_deg=r["hour_angle_deg"] - 2 * longitude,
                                                                         solar_time_h=r["solar_time_h"] - 2 * longitude / 15))(solar_time_hour_angle(n, clock, longitude, utc_offset))),
     random_inputs=lambda rng: (lambda tz: {"n": rng.randint(1, 365), "clock": rng.randint(0, 24 * 60 - 1) / 60.0, "utc_offset": float(tz),
                                            "longitude": round(15 * tz + rng.uniform(-14, 14), 4)})(rng.randint(-10, 12)),
     templates=["Hour angle: n = 172, clock = 14:30, longitude = -75, utc_offset = -5"],
     examples=[{"inputs": {"n": 172, "clock": 14.5, "longitude": -75, "utc_offset": -5}, "outputs": {"hour_angle_deg": _r(37.5 + _eot_spencer(172) / 4)}},
               {"inputs": {"n": 81, "clock": 12.0, "longitude": 15, "utc_offset": 1}, "outputs": {"hour_angle_deg": _r(_eot_spencer(81) / 4)}}],
     questions=[{"q": "Hour angle: n = 172, clock = 14:30, longitude = -75, utc_offset = -5", "outputs": {"hour_angle_deg": _r(37.5 + _eot_spencer(172) / 4)}},
                {"q": "Solar time: n = 81, time = 12, longitude = 15 deg, utc = 1", "outputs": {"solar_time_h": _r(12 + _eot_spencer(81) / 60)}},
                {"q": "Hour angle: n = 172, clock = 25, longitude = -75, utc_offset = -5", "outcome": "INVALID_INPUT"},
                {"q": "Hour angle: n = 172, clock = 14:30, longitude = -75", "outcome": "NEEDS_INPUT"}])


def _sun_angles(latitude, decl_rad, ha_deg):
    lat, h = math.radians(latitude), math.radians(ha_deg)
    cz = math.cos(decl_rad) * math.cos(lat) * math.cos(h) + math.sin(decl_rad) * math.sin(lat)
    z = math.acos(max(-1.0, min(1.0, cz)))
    numer = math.cos(z) * math.sin(lat) - math.sin(decl_rad)
    denom = math.sin(z) * math.cos(lat)
    ca = 1.0 if abs(denom) <= 1e-8 else numer / denom
    if abs(ca - 1.0) <= 1e-8:
        ca = 1.0
    elif abs(ca + 1.0) <= 1e-8:
        ca = -1.0
    sgn = (ha_deg > 0) - (ha_deg < 0)
    az = sgn * math.acos(max(-1.0, min(1.0, ca))) + math.pi
    return math.degrees(z), math.degrees(az)


def sun_angles(latitude, declination, hour_angle):
    _check_place(latitude=latitude)
    _need(-23.5 <= declination <= 23.5, "the declination must lie between -23.5 and 23.5 degrees")
    _need(-180 <= hour_angle <= 180, "the hour angle must lie between -180 and 180 degrees")
    z, az = _sun_angles(latitude, math.radians(declination), hour_angle)
    return _out(zenith_deg=z, elevation_deg=90.0 - z, azimuth_deg=az)


def _ref_sun_angles(latitude, declination, hour_angle):
    from pvlib import solarposition as sp
    lat, d, h = math.radians(latitude), math.radians(declination), math.radians(hour_angle)
    z = _np_scalar(sp.solar_zenith_analytical(lat, h, d))
    az = _np_scalar(sp.solar_azimuth_analytical(lat, h, d, z))
    return {"zenith_deg": math.degrees(z), "elevation_deg": 90 - math.degrees(z), "azimuth_deg": math.degrees(az)}


def _wrong_sun_angles(latitude, declination, hour_angle):
    # latitude and colatitude exchanged in the zenith formula
    lat, d, h = math.radians(latitude), math.radians(declination), math.radians(hour_angle)
    cz = math.cos(d) * math.sin(lat) * math.cos(h) + math.sin(d) * math.cos(lat)
    z = math.degrees(math.acos(max(-1, min(1, cz))))
    return _out(zenith_deg=z, elevation_deg=90 - z, azimuth_deg=sun_angles(latitude, declination, hour_angle)["azimuth_deg"])


_add("energy_sun_angles",
     name="Solar zenith, elevation and azimuth from latitude, declination and hour angle", topic="solar resource / sun position",
     purpose="The sun's zenith angle, elevation (altitude) and azimuth (clockwise from north) at a latitude for a given declination and hour angle.",
     inputs=[_numin("latitude", "latitude in degrees (north positive)"), _numin("declination", "solar declination in degrees"),
             _numin("hour_angle", "hour angle in degrees (negative in the morning)")],
     outputs=[_o("zenith_deg", "number", "solar zenith angle in degrees"), _o("elevation_deg", "number", "solar elevation (altitude) in degrees"),
              _o("azimuth_deg", "number", "solar azimuth in degrees clockwise from north")],
     primary="elevation_deg", conditions=["-90 < latitude < 90", "|declination| <= 23.5", "|hour angle| <= 180"],
     method="cos z = cos(delta) cos(phi) cos(omega) + sin(delta) sin(phi); cos(azimuth - 180) = (cos z sin phi - sin delta)/(sin z cos phi), sign of omega",
     latex=r"\cos\theta_z = \cos\phi\cos\delta\cos\omega + \sin\phi\sin\delta",
     run=sun_angles,
     parse=lambda q: _pairs(q, r"sun angles|solar zenith and azimuth|solar elevation and azimuth|sun position from hour angle",
                            {"latitude": "latitude", "lat": "latitude", "phi": "latitude", "declination": "declination", "delta": "declination",
                             "hour_angle": "hour_angle", "omega": "hour_angle"}, num_units=_deg_units("latitude", "declination", "hour_angle")),
     reference=_ref_sun_angles, wrong=_safe(_wrong_sun_angles),
     random_inputs=lambda rng: {"latitude": round(rng.uniform(-66, 66), 3), "declination": round(rng.uniform(-23.4, 23.4), 3),
                                "hour_angle": round(rng.choice([-1, 1]) * rng.uniform(1, 120), 3)},
     templates=["Sun angles: latitude = 40, declination = 23.45, hour_angle = 30"],
     examples=[{"inputs": {"latitude": 40, "declination": 0, "hour_angle": 0}, "outputs": {"zenith_deg": 40.0, "elevation_deg": 50.0, "azimuth_deg": 180.0}},
               {"inputs": {"latitude": 0, "declination": 0, "hour_angle": -90}, "outputs": {"zenith_deg": 90.0, "azimuth_deg": 90.0}}],
     questions=[{"q": "Sun angles: latitude = 40, declination = 0, hour_angle = 0", "outputs": {"elevation_deg": 50.0, "azimuth_deg": 180.0}},
                {"q": "Solar elevation and azimuth: lat = 0 deg, delta = 0, omega = -90", "outputs": {"azimuth_deg": 90.0}},
                {"q": "Sun angles: latitude = 40, declination = 30, hour_angle = 0", "outcome": "INVALID_INPUT"}])


def solar_position(latitude, longitude, n, clock, utc_offset):
    _check_day(n)
    _check_place(latitude, longitude, utc_offset)
    _need(0 <= clock < 24, "the clock time must be from 0 to 24 hours (decimal hours or hh:mm)")
    d = _decl_spencer(n)
    eot = _eot_spencer(n)
    ha = 15.0 * (clock - utc_offset - 12.0) + longitude + eot / 4.0
    z, az = _sun_angles(latitude, d, ha)
    return _out(declination_deg=math.degrees(d), eot_min=eot, hour_angle_deg=ha, zenith_deg=z, elevation_deg=90.0 - z, azimuth_deg=az)


def _ref_solar_position(latitude, longitude, n, clock, utc_offset):
    from pvlib import solarposition as sp
    d = _np_scalar(sp.declination_spencer71(float(n)))
    eot = _np_scalar(sp.equation_of_time_spencer71(float(n)))
    ha = _np_scalar(sp.hour_angle(_local_index(n, clock, utc_offset), longitude, eot))
    lat, h = math.radians(latitude), math.radians(ha)
    z = _np_scalar(sp.solar_zenith_analytical(lat, h, d))
    az = _np_scalar(sp.solar_azimuth_analytical(lat, h, d, z))
    return {"declination_deg": math.degrees(d), "eot_min": eot, "hour_angle_deg": ha, "zenith_deg": math.degrees(z),
            "elevation_deg": 90 - math.degrees(z), "azimuth_deg": math.degrees(az)}


def _rnd_place_time(rng):
    tz = rng.randint(-10, 12)
    for _ in range(100):
        x = {"latitude": round(rng.uniform(-60, 60), 3), "longitude": round(15 * tz + rng.uniform(-14, 14), 3),
             "n": rng.randint(1, 365), "clock": rng.randint(5 * 60, 19 * 60) / 60.0, "utc_offset": float(tz)}
        if abs(_hour_angle(x["n"], x["clock"], x["longitude"], x["utc_offset"])) > 1.0:
            return x
    return x


_add("energy_solar_position",
     name="Sun position at a place and local time", topic="solar resource / sun position",
     purpose="Declination, equation of time, hour angle, zenith, elevation and azimuth of the sun at a latitude and longitude on day n at a local clock time (Spencer declination and equation of time).",
     inputs=[_numin("latitude", "latitude in degrees (north positive)"), _numin("longitude", "longitude in degrees, east positive"), _DAY,
             _numin("clock", "local clock (standard) time in decimal hours or hh:mm"), _numin("utc_offset", "time zone as hours from UTC")],
     outputs=[_o("declination_deg", "number", "declination in degrees"), _o("eot_min", "number", "equation of time in minutes"),
              _o("hour_angle_deg", "number", "hour angle in degrees"), _o("zenith_deg", "number", "solar zenith angle in degrees"),
              _o("elevation_deg", "number", "solar elevation in degrees"), _o("azimuth_deg", "number", "solar azimuth in degrees clockwise from north")],
     primary="elevation_deg", conditions=["1 <= n <= 366", "0 <= clock < 24", "-90 < latitude < 90"],
     method="Spencer declination and equation of time; omega = 15 (t - UTC offset - 12) + longitude + E/4; cos z = cos(delta) cos(phi) cos(omega) + sin(delta) sin(phi)",
     latex=r"\cos\theta_z = \cos\phi\cos\delta\cos\omega + \sin\phi\sin\delta,\quad \omega = 15^\circ(t_s - 12)",
     run=solar_position,
     parse=lambda q: _pairs(q, r"solar position|sun position", dict(_N_NAMES, clock="clock", time="clock", **_PLACE_NAMES),
                            num_units=_deg_units("latitude", "longitude"), clocks={"clock"}),
     reference=_ref_solar_position,
     wrong=_safe(lambda latitude, longitude, n, clock, utc_offset: (lambda r: dict(r, elevation_deg=_r(r["zenith_deg"]), zenith_deg=_r(90 - r["zenith_deg"])))(
         solar_position(latitude, longitude, n, clock, utc_offset))),
     random_inputs=_rnd_place_time,
     templates=["Solar position: latitude = 40, longitude = -105, n = 172, clock = 13:00, utc_offset = -7"],
     examples=[{"inputs": {"latitude": 40, "longitude": -105, "n": 172, "clock": 13.0, "utc_offset": -7},
                "outputs": {"hour_angle_deg": _r(15 * (13 + 7 - 12) - 105 + _eot_spencer(172) / 4), "declination_deg": _r(math.degrees(_decl_spencer(172)))}}],
     questions=[{"q": "Solar position: latitude = 40, longitude = -105, n = 172, clock = 13:00, utc_offset = -7",
                 "outputs": {"hour_angle_deg": _r(15 * (13 + 7 - 12) - 105 + _eot_spencer(172) / 4)}},
                {"q": "Solar position: latitude = 95, longitude = -105, n = 172, clock = 13:00, utc_offset = -7", "outcome": "INVALID_INPUT"}])


def sunrise_sunset(latitude, longitude, n, utc_offset):
    _check_day(n)
    _check_place(latitude, longitude, utc_offset)
    d = _decl_spencer(n)
    eot = _eot_spencer(n)
    x = -math.tan(d) * math.tan(math.radians(latitude))
    _need(-1 < x < 1, "the sun does not rise or does not set on this day at this latitude (polar day or night)")
    ws = math.degrees(math.acos(x))
    to_h = lambda ha: (ha - longitude - eot / 4.) / 15. + 12. + utc_offset   # noqa: E731
    rise, sset, transit = to_h(-ws), to_h(ws), to_h(0.0)
    return _out(sunrise_h=rise, sunset_h=sset, solar_noon_h=transit, day_length_h=sset - rise, sunset_hour_angle_deg=ws)


def _ref_sunrise(latitude, longitude, n, utc_offset):
    import pandas as pd
    from pvlib import solarposition as sp
    idx = _local_index(n, 12.0, utc_offset)
    d = sp.declination_spencer71(float(n))
    e = sp.equation_of_time_spencer71(float(n))
    r, s, t = sp.sun_rise_set_transit_geometric(idx, latitude, longitude, d, e)
    mid = idx.normalize()
    h = lambda ts: float((ts - mid)[0] / pd.Timedelta("1h"))   # noqa: E731
    return {"sunrise_h": h(r), "sunset_h": h(s), "solar_noon_h": h(t), "day_length_h": h(s) - h(r),
            "sunset_hour_angle_deg": 15.0 * (h(s) - h(t))}


_add("energy_sunrise_sunset",
     name="Sunrise, sunset, solar noon and day length", topic="solar resource / sun position",
     purpose="Geometric sunrise and sunset times (local clock time), solar noon, the day length and the sunset hour angle at a place on day n (no refraction, sun centre on the horizon).",
     inputs=[_numin("latitude", "latitude in degrees (north positive)"), _numin("longitude", "longitude in degrees, east positive"), _DAY,
             _numin("utc_offset", "time zone as hours from UTC")],
     outputs=[_o("sunrise_h", "number", "sunrise, local clock time in decimal hours"), _o("sunset_h", "number", "sunset, local clock time in decimal hours"),
              _o("solar_noon_h", "number", "solar noon, local clock time in decimal hours"), _o("day_length_h", "number", "day length in hours"),
              _o("sunset_hour_angle_deg", "number", "sunset hour angle omega_s in degrees")],
     primary="day_length_h", conditions=["the sun rises and sets on that day (no polar day or night)"],
     method="omega_s = acos(-tan(phi) tan(delta)); t = (omega - longitude - E/4)/15 + 12 + UTC offset; day length = 2 omega_s/15",
     latex=r"\cos\omega_s = -\tan\phi\tan\delta,\quad N = \frac{2\,\omega_s}{15^\circ}",
     run=sunrise_sunset,
     parse=lambda q: _pairs(q, r"sunrise and sunset|sunrise|sunset|day length", dict(_N_NAMES, **_PLACE_NAMES), num_units=_deg_units("latitude", "longitude")),
     reference=_ref_sunrise,
     wrong=_safe(lambda latitude, longitude, n, utc_offset: (lambda r: _out(sunrise_h=r["sunrise_h"] + r["sunset_hour_angle_deg"] / 30, sunset_h=r["sunset_h"] - r["sunset_hour_angle_deg"] / 30,
                                                                              solar_noon_h=r["solar_noon_h"], day_length_h=r["day_length_h"] / 2,
                                                                              sunset_hour_angle_deg=r["sunset_hour_angle_deg"]))(sunrise_sunset(latitude, longitude, n, utc_offset))),
     random_inputs=lambda rng: (lambda tz: {"latitude": round(rng.uniform(-60, 60), 3), "longitude": round(15 * tz + rng.uniform(-14, 14), 3),
                                            "n": rng.randint(1, 365), "utc_offset": float(tz)})(rng.randint(-10, 12)),
     templates=["Sunrise and sunset: latitude = 51.5, longitude = 0, n = 172, utc_offset = 0"],
     examples=[{"inputs": {"latitude": 0, "longitude": 0, "n": 81, "utc_offset": 0}, "outputs": {"day_length_h": 12.0, "sunset_hour_angle_deg": 90.0}},
               {"inputs": {"latitude": 40, "longitude": 0, "n": 172, "utc_offset": 0},
                "outputs": {"day_length_h": _r(2 * math.degrees(math.acos(-math.tan(_decl_spencer(172)) * math.tan(math.radians(40)))) / 15)}}],
     questions=[{"q": "Day length: latitude = 0, longitude = 0, n = 81, utc_offset = 0", "outputs": {"day_length_h": 12.0}},
                {"q": "Sunrise and sunset: latitude = 80, longitude = 0, n = 172, utc_offset = 0", "outcome": "INVALID_INPUT"}])


# ================================================================================================ SOLAR IRRADIANCE
def extraterrestrial_irradiance(n, method="spencer", solar_constant=SOLAR_CONSTANT):
    _check_day(n)
    _pos("the solar constant", solar_constant)
    if method == "asce":
        f = 1 + 0.033 * math.cos(_day_angle(n, offset=0))
    else:
        b = _day_angle(n)
        f = 1.00011 + 0.034221 * math.cos(b) + 0.00128 * math.sin(b) + 0.000719 * math.cos(2 * b) + 7.7e-05 * math.sin(2 * b)
    return _out(G_on=solar_constant * f, distance_factor=f)


def _ref_extra(n, method="spencer", solar_constant=SOLAR_CONSTANT):
    from pvlib import irradiance as irr
    g = _np_scalar(irr.get_extra_radiation(float(n), solar_constant=solar_constant, method=method))
    return {"G_on": g, "distance_factor": g / solar_constant}


_add("energy_extraterrestrial_irradiance",
     name="Extraterrestrial (top-of-atmosphere) normal irradiance", topic="solar resource / irradiance",
     purpose="The solar irradiance on a plane normal to the sun's rays outside the atmosphere on day n, and the Earth-Sun distance factor (R0/R)^2.",
     inputs=[_DAY, _choice("method", ["spencer", "asce"], "formula: spencer (default, Fourier series) or asce (1 + 0.033 cos(360 n/365))", required=False),
             _phys("solar_constant", "irradiance", "solar constant (default 1366.1 W/m^2)", required=False)],
     outputs=[_o("G_on", "irradiance", "extraterrestrial normal irradiance"), _o("distance_factor", "number", "Earth-Sun distance factor (R0/R)^2")],
     primary="G_on", conditions=["1 <= n <= 366"],
     method="G_on = G_sc (1.00011 + 0.034221 cos B + 0.00128 sin B + 0.000719 cos 2B + 0.000077 sin 2B) (Spencer) or G_sc (1 + 0.033 cos(360 n/365))",
     latex=r"G_{on} = G_{sc}\left(1 + 0.033\cos\frac{360^\circ n}{365}\right)",
     run=extraterrestrial_irradiance,
     parse=lambda q: _pairs(q, r"extraterrestrial (?:normal )?irradiance|extraterrestrial radiation|top of atmosphere irradiance",
                            dict(_N_NAMES, method="method", solar_constant="solar_constant", Gsc="solar_constant", G_sc="solar_constant"),
                            phys={"solar_constant"}, words={"method"}),
     reference=_ref_extra,
     wrong=_safe(lambda n, method="spencer", solar_constant=SOLAR_CONSTANT: _out(G_on=solar_constant * (1 - 0.033 * math.cos(2 * math.pi * n / 365)),
                                                                                 distance_factor=1 - 0.033 * math.cos(2 * math.pi * n / 365))),
     random_inputs=lambda rng: {"n": rng.randint(1, 365), "method": rng.choice(["spencer", "asce"]), "solar_constant": rng.choice([1366.1, 1367.0, 1361.0, 1353.0])},
     templates=["Extraterrestrial irradiance: n = 1", "Extraterrestrial irradiance: n = 172, method = asce, solar_constant = 1367 W/m^2"],
     examples=[{"inputs": {"n": 365, "method": "asce", "solar_constant": 1367.0}, "outputs": {"G_on": _r(1367 * 1.033)}},
               {"inputs": {"n": 1}, "outputs": {"G_on": _r(1366.1 * (1.00011 + 0.034221 + 0.000719))}}],
     questions=[{"q": "Extraterrestrial irradiance: n = 365, method = asce, solar_constant = 1367 W/m^2", "outputs": {"G_on": _r(1367 * 1.033)}},
                {"q": "Extraterrestrial irradiance: n = 1", "outputs": {"G_on": _r(1366.1 * (1.00011 + 0.034221 + 0.000719))}},
                {"q": "Extraterrestrial irradiance: n = 1, solar_constant = 1367", "outcome": "UNRECOGNIZED_FORMAT"}])


_AM_MODELS = ["kastenyoung1989", "kasten1966", "simple", "pickering2002", "youngirvine1967", "young1994", "gueymard1993", "gueymard2003"]


def _airmass_rel(z, model):
    zr = math.radians(z)
    c = math.cos(zr)
    if model == "kastenyoung1989":
        return 1.0 / (c + 0.50572 * ((6.07995 + (90 - z)) ** -1.6364))
    if model == "kasten1966":
        return 1.0 / (c + 0.15 * ((93.885 - z) ** -1.253))
    if model == "simple":
        return 1.0 / c
    if model == "pickering2002":
        return 1.0 / math.sin(math.radians(90 - z + 244.0 / (165 + 47.0 * (90 - z) ** 1.1)))
    if model == "youngirvine1967":
        s = 1.0 / c
        return s * (1 - 0.0012 * (s * s - 1))
    if model == "young1994":
        return (1.002432 * c ** 2 + 0.148386 * c + 0.0096467) / (c ** 3 + 0.149864 * c ** 2 + 0.0102963 * c + 0.000303978)
    if model == "gueymard1993":
        return 1.0 / (c + 0.00176759 * z * ((94.37515 - z) ** -1.21563))
    return 1.0 / (c + 0.48353 * (z ** 0.095846) / (96.741 - z) ** 1.754)


def air_mass(zenith, model="kastenyoung1989", pressure=101325.0):
    _need(isinstance(zenith, (int, float)) and 0 <= zenith < 90, "the zenith angle must be from 0 up to (not including) 90 degrees")
    _pos("the pressure", pressure)
    am = _airmass_rel(zenith, model)
    return _out(AM_relative=am, AM_absolute=am * pressure / 101325.)


def _ref_air_mass(zenith, model="kastenyoung1989", pressure=101325.0):
    from pvlib import atmosphere as atm
    am = _np_scalar(atm.get_relative_airmass(float(zenith), model))
    return {"AM_relative": am, "AM_absolute": _np_scalar(atm.get_absolute_airmass(am, pressure))}


_add("energy_air_mass",
     name="Optical air mass", topic="solar resource / atmosphere",
     purpose="The relative optical air mass for a solar zenith angle (Kasten-Young by default, or another named model) and the pressure-corrected absolute air mass.",
     inputs=[_numin("zenith", "solar zenith angle in degrees (apparent zenith for the refraction-based models)"),
             _choice("model", _AM_MODELS, "air-mass model (default kastenyoung1989)", required=False),
             _phys("pressure", "pressure", "site pressure (default 101325 Pa)", required=False)],
     outputs=[_o("AM_relative", "number", "relative air mass (sea level)"), _o("AM_absolute", "number", "absolute air mass AM_rel p/101325 Pa")],
     primary="AM_relative", conditions=["0 <= zenith < 90"],
     method="Kasten-Young: AM = 1/(cos z + 0.50572 (96.07995 - z)^-1.6364); absolute AM = AM p/p0",
     latex=r"AM = \frac{1}{\cos z + 0.50572\,(96.07995^\circ - z)^{-1.6364}}",
     run=air_mass,
     parse=lambda q: _pairs(q, r"air mass|airmass", {"zenith": "zenith", "z": "zenith", "model": "model", "pressure": "pressure", "p": "pressure"},
                            phys={"pressure"}, words={"model"}, num_units=_deg_units("zenith")),
     reference=_ref_air_mass,
     wrong=_safe(lambda zenith, model="kastenyoung1989", pressure=101325.0: _out(AM_relative=math.cos(math.radians(zenith)), AM_absolute=math.cos(math.radians(zenith)) * pressure / 101325)),
     random_inputs=lambda rng: {"zenith": round(rng.uniform(0, 85), 3), "model": rng.choice(_AM_MODELS), "pressure": round(rng.uniform(6e4, 1.03e5), 1)},
     templates=["Air mass: zenith = 60", "Air mass: zenith = 48.2, model = simple, pressure = 850 hPa"],
     examples=[{"inputs": {"zenith": 60, "model": "simple"}, "outputs": {"AM_relative": 2.0}},
               {"inputs": {"zenith": 0, "model": "simple", "pressure": 50662.5}, "outputs": {"AM_absolute": 0.5}}],
     questions=[{"q": "Air mass: zenith = 60, model = simple", "outputs": {"AM_relative": 2.0}},
                {"q": "Air mass: zenith = 0, model = simple, pressure = 506.625 hPa", "outputs": {"AM_absolute": 0.5}},
                {"q": "Air mass: zenith = 95", "outcome": "INVALID_INPUT"}])


def _projection(tilt, saz, z, az):
    p = _cosd(tilt) * _cosd(z) + _sind(tilt) * _sind(z) * _cosd(az - saz)
    return max(-1.0, min(1.0, p))


def _check_surface(tilt, saz):
    _need(isinstance(tilt, (int, float)) and 0 <= tilt <= 180, "the tilt must lie between 0 and 180 degrees")
    _need(isinstance(saz, (int, float)) and 0 <= saz <= 360, "the surface azimuth must lie between 0 and 360 degrees (clockwise from north)")


def _check_sun(z, az):
    _need(isinstance(z, (int, float)) and 0 <= z <= 180, "the solar zenith must lie between 0 and 180 degrees")
    _need(isinstance(az, (int, float)) and 0 <= az <= 360, "the solar azimuth must lie between 0 and 360 degrees")


def angle_of_incidence(tilt, surface_azimuth, zenith, azimuth):
    _check_surface(tilt, surface_azimuth)
    _check_sun(zenith, azimuth)
    p = _projection(tilt, surface_azimuth, zenith, azimuth)
    return _out(aoi_deg=math.degrees(math.acos(p)), cos_aoi=p)


def _ref_aoi(tilt, surface_azimuth, zenith, azimuth):
    from pvlib import irradiance as irr
    return {"aoi_deg": _np_scalar(irr.aoi(tilt, surface_azimuth, zenith, azimuth)),
            "cos_aoi": _np_scalar(irr.aoi_projection(tilt, surface_azimuth, zenith, azimuth))}


def _rnd_geom(rng, need_day=False):
    for _ in range(200):
        x = {"tilt": round(rng.uniform(0, 75), 2), "surface_azimuth": round(rng.uniform(90, 270), 2),
             "zenith": round(rng.uniform(5, 80), 2), "azimuth": round(rng.uniform(60, 300), 2)}
        if 0.05 < _projection(x["tilt"], x["surface_azimuth"], x["zenith"], x["azimuth"]) < 0.995:
            return x
    return x


_SURF_NAMES = {"tilt": "tilt", "beta": "tilt", "surface_tilt": "tilt", "surface_azimuth": "surface_azimuth", "gamma": "surface_azimuth",
               "panel_azimuth": "surface_azimuth", "zenith": "zenith", "solar_zenith": "zenith", "azimuth": "azimuth", "solar_azimuth": "azimuth"}

_add("energy_angle_of_incidence",
     name="Angle of incidence of the sun on a tilted surface", topic="solar resource / tilted surfaces",
     purpose="The angle between the sun's rays and the normal of a surface of given tilt and azimuth, and its cosine (the beam projection factor).",
     inputs=[_numin("tilt", "surface tilt from horizontal in degrees"), _numin("surface_azimuth", "surface azimuth in degrees clockwise from north (south = 180)"),
             _numin("zenith", "solar zenith angle in degrees"), _numin("azimuth", "solar azimuth in degrees clockwise from north")],
     outputs=[_o("aoi_deg", "number", "angle of incidence in degrees"), _o("cos_aoi", "number", "cosine of the angle of incidence")],
     primary="aoi_deg", conditions=["0 <= tilt <= 180", "azimuths from 0 to 360 degrees"],
     method="cos(theta) = cos(beta) cos(z) + sin(beta) sin(z) cos(azimuth_sun - azimuth_surface)",
     latex=r"\cos\theta = \cos\beta\cos\theta_z + \sin\beta\sin\theta_z\cos(\gamma_s - \gamma)",
     run=angle_of_incidence,
     parse=lambda q: _pairs(q, r"angle of incidence|incidence angle|aoi", _SURF_NAMES, num_units=_deg_units(*set(_SURF_NAMES.values()))),
     reference=_ref_aoi,
     wrong=_safe(lambda tilt, surface_azimuth, zenith, azimuth: (lambda p: _out(aoi_deg=math.degrees(math.acos(p)), cos_aoi=p))(
         max(-1, min(1, _cosd(tilt) * _cosd(zenith) + _sind(tilt) * _sind(zenith) * _cosd(azimuth + surface_azimuth))))),
     random_inputs=_rnd_geom,
     templates=["Angle of incidence: tilt = 30, surface_azimuth = 180, zenith = 40, azimuth = 160"],
     examples=[{"inputs": {"tilt": 30, "surface_azimuth": 180, "zenith": 30, "azimuth": 180}, "outputs": {"aoi_deg": 0.0, "cos_aoi": 1.0}},
               {"inputs": {"tilt": 0, "surface_azimuth": 180, "zenith": 60, "azimuth": 100}, "outputs": {"aoi_deg": 60.0, "cos_aoi": 0.5}}],
     questions=[{"q": "Angle of incidence: tilt = 0, surface_azimuth = 180, zenith = 60, azimuth = 100", "outputs": {"cos_aoi": 0.5}},
                {"q": "Angle of incidence: tilt = 30, surface_azimuth = 400, zenith = 60, azimuth = 100", "outcome": "INVALID_INPUT"}])


# --- plane-of-array (transposition) models ------------------------------------------------------------------------
def _check_irr(dni, ghi, dhi):
    _nonneg("the irradiances", dni, ghi, dhi)
    _need(dhi <= ghi + 1e-9, "the diffuse horizontal irradiance cannot exceed the global horizontal irradiance")


def _poa(model, tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n=None):
    _check_surface(tilt, surface_azimuth)
    _need(isinstance(zenith, (int, float)) and 0 <= zenith < 90, "the sun must be above the horizon (zenith below 90 degrees)")
    _need(0 <= azimuth <= 360, "the solar azimuth must lie between 0 and 360 degrees")
    _check_irr(dni, ghi, dhi)
    _need(0 <= albedo <= 1, "the albedo must lie between 0 and 1")
    p = _projection(tilt, surface_azimuth, zenith, azimuth)
    cos_tt = max(p, 0.0)
    svf = 0.5 * (1 + _cosd(tilt))
    if model == "isotropic":
        sky = dhi * (1 + _cosd(tilt)) * 0.5
    elif model == "klucher":
        F = 1 - (dhi / ghi) ** 2 if ghi > 0 else 0.0
        sky = dhi * svf * (1 + F * (_sind(0.5 * tilt) ** 3)) * (1 + F * (cos_tt ** 2) * (_sind(zenith) ** 3))
    else:
        _check_day(n)
        dni_extra = _g_on_spencer(n)
        Rb = cos_tt / max(_cosd(zenith), 0.01745)
        AI = dni / dni_extra
        if model == "haydavies":
            sky = max(dhi * (1 - AI) * svf, 0.0) + max(dhi * (AI * Rb), 0.0)
        else:                                            # reindl
            HB = max(dni * _cosd(zenith), 0.0)
            h = math.sqrt(HB / ghi if ghi != 0 else 0.0) * (_sind(tilt / 2) ** 3)
            t1 = (1 - AI) * svf
            sky = dhi * (t1 + AI * Rb + t1 * h)
    ground = ghi * albedo * (1 - math.cos(math.radians(tilt))) * 0.5
    direct = max(dni * math.cos(math.radians(math.degrees(math.acos(p)))), 0.0)
    return _out(poa_global=direct + sky + ground, poa_direct=direct, poa_sky_diffuse=sky, poa_ground_diffuse=ground,
                aoi_deg=math.degrees(math.acos(p)))


def _ref_poa(model, tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n=None):
    from pvlib import irradiance as irr
    extra = None if n is None else _np_scalar(irr.get_extra_radiation(float(n)))
    r = irr.get_total_irradiance(tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, dni_extra=extra, albedo=albedo, model=model)
    return {"poa_global": _np_scalar(r["poa_global"]), "poa_direct": _np_scalar(r["poa_direct"]), "poa_sky_diffuse": _np_scalar(r["poa_sky_diffuse"]),
            "poa_ground_diffuse": _np_scalar(r["poa_ground_diffuse"]), "aoi_deg": _np_scalar(irr.aoi(tilt, surface_azimuth, zenith, azimuth))}


def _rnd_poa(rng, with_day):
    g = _rnd_geom(rng)
    z = g["zenith"]
    dni = round(rng.uniform(50, 950), 1)
    dhi = round(rng.uniform(20, 300), 1)
    x = dict(g, dni=dni, dhi=dhi, ghi=round(dni * _cosd(z) + dhi, 3), albedo=round(rng.uniform(0.1, 0.5), 3))
    if with_day:
        x["n"] = rng.randint(1, 365)
    return x


_POA_IN = [_numin("tilt", "surface tilt from horizontal in degrees"), _numin("surface_azimuth", "surface azimuth in degrees clockwise from north (south = 180)"),
           _numin("zenith", "solar zenith angle in degrees"), _numin("azimuth", "solar azimuth in degrees clockwise from north"),
           _phys("dni", "irradiance", "direct normal irradiance"), _phys("ghi", "irradiance", "global horizontal irradiance"),
           _phys("dhi", "irradiance", "diffuse horizontal irradiance"), _numin("albedo", "ground reflectance (0 to 1)")]
_POA_OUT = [_o("poa_global", "irradiance", "total irradiance on the plane"), _o("poa_direct", "irradiance", "beam irradiance on the plane"),
            _o("poa_sky_diffuse", "irradiance", "sky-diffuse irradiance on the plane"), _o("poa_ground_diffuse", "irradiance", "ground-reflected irradiance on the plane"),
            _o("aoi_deg", "number", "angle of incidence in degrees")]
_POA_NAMES = dict(_SURF_NAMES, dni="dni", DNI="dni", ghi="ghi", GHI="ghi", dhi="dhi", DHI="dhi", albedo="albedo", rho="albedo", **_N_NAMES)
_POA_EX = {"tilt": 30, "surface_azimuth": 180, "zenith": 30, "azimuth": 180, "dni": 800, "ghi": 800 * math.cos(math.radians(30)) + 100, "dhi": 100, "albedo": 0.2}


def _poa_spec(key, model, name, purpose, method, latex, wrong_fn, level_day, example_out, tpl):
    inputs = list(_POA_IN) + ([_DAY] if level_day else [])
    if level_day:
        run = lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n: _poa(model, tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n)  # noqa: E731
        ref = lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n: _ref_poa(model, tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n)  # noqa: E731
    else:
        run = lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo: _poa(model, tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo)  # noqa: E731
        ref = lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo: _ref_poa(model, tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo)  # noqa: E731
    ex = dict(_POA_EX, **({"n": 172} if level_day else {}))
    q = tpl
    _add(key, name=name, topic="solar resource / tilted surfaces", purpose=purpose, inputs=inputs, outputs=_POA_OUT, primary="poa_global",
         conditions=["sun above the horizon", "DHI <= GHI", "0 <= albedo <= 1"], method=method, latex=latex, run=run,
         parse=lambda qq: _pairs(qq, name_phrase[key], _POA_NAMES, phys={"dni", "ghi", "dhi"}, num_units=_deg_units("tilt", "surface_azimuth", "zenith", "azimuth")),
         reference=ref, wrong=_safe(wrong_fn), random_inputs=lambda rng: _rnd_poa(rng, level_day),
         templates=[q], examples=[{"inputs": ex, "outputs": example_out}],
         questions=[{"q": q, "outputs": example_out},
                    {"q": q.replace("albedo = 0.2", "albedo = 1.5"), "outcome": "INVALID_INPUT"},
                    {"q": q.replace(", albedo = 0.2", ""), "outcome": "NEEDS_INPUT"}])


name_phrase = {"energy_poa_isotropic": r"plane of array irradiance \(isotropic\)|poa irradiance isotropic|isotropic sky poa|plane of array irradiance isotropic",
               "energy_poa_klucher": r"plane of array irradiance \(klucher\)|poa irradiance klucher|plane of array irradiance klucher",
               "energy_poa_haydavies": r"plane of array irradiance \(hay[- ]davies\)|poa irradiance hay[- ]davies|plane of array irradiance hay[- ]davies",
               "energy_poa_reindl": r"plane of array irradiance \(reindl\)|poa irradiance reindl|plane of array irradiance reindl"}
_GHI_EX = 800 * math.cos(math.radians(30)) + 100
_Q_POA = ("tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = " + repr(_GHI_EX)
          + " W/m^2, dhi = 100 W/m^2, albedo = 0.2")

_poa_spec("energy_poa_isotropic", "isotropic", "Irradiance on a tilted plane (isotropic sky)",
          "Beam, sky-diffuse (isotropic sky, Liu-Jordan/Hottel-Woertz) and ground-reflected irradiance on a tilted surface from DNI, GHI and DHI.",
          "G_poa = DNI cos(theta) + DHI (1 + cos beta)/2 + GHI rho (1 - cos beta)/2", r"G_{poa} = G_{bn}\cos\theta + G_d\frac{1+\cos\beta}{2} + G\rho\frac{1-\cos\beta}{2}",
          lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo: (lambda r: dict(r, poa_ground_diffuse=_r(ghi * albedo * (1 + _cosd(tilt)) / 2),
                                                                                                  poa_global=_r(r["poa_direct"] + r["poa_sky_diffuse"] + ghi * albedo * (1 + _cosd(tilt)) / 2)))(
              _poa("isotropic", tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo)),
          False, {"poa_direct": 800.0, "poa_sky_diffuse": _r(100 * (1 + math.cos(math.radians(30))) / 2),
                  "poa_ground_diffuse": _r(_GHI_EX * 0.2 * (1 - math.cos(math.radians(30))) / 2)},
          "Plane of array irradiance (isotropic): " + _Q_POA)

_poa_spec("energy_poa_klucher", "klucher", "Irradiance on a tilted plane (Klucher sky model)",
          "Beam, sky-diffuse (Klucher 1979: isotropic sky brightened near the horizon and around the sun under clear skies) and ground-reflected irradiance on a tilted surface.",
          "sky = DHI (1 + cos beta)/2 (1 + F sin^3(beta/2)) (1 + F cos^2(theta) sin^3(z)), F = 1 - (DHI/GHI)^2",
          r"G_{d,t} = G_d\frac{1+\cos\beta}{2}\left(1 + F\sin^3\frac{\beta}{2}\right)\left(1 + F\cos^2\theta\sin^3\theta_z\right)",
          lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo: _poa("isotropic", tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo),
          False, {"poa_direct": 800.0}, "Plane of array irradiance (Klucher): " + _Q_POA)

_poa_spec("energy_poa_haydavies", "haydavies", "Irradiance on a tilted plane (Hay-Davies sky model)",
          "Beam, sky-diffuse (Hay and Davies 1980: isotropic plus circumsolar weighted by the anisotropy index DNI/G_on) and ground-reflected irradiance on a tilted surface; G_on from day n (Spencer, 1366.1 W/m^2).",
          "sky = DHI [A R_b + (1 - A)(1 + cos beta)/2], A = DNI/G_on, R_b = cos(theta)/cos(z)",
          r"G_{d,t} = G_d\left[A R_b + (1 - A)\frac{1+\cos\beta}{2}\right],\ A = \frac{G_{bn}}{G_{on}}",
          lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n: _poa("isotropic", tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo),
          True, {"poa_direct": 800.0}, "Plane of array irradiance (Hay-Davies): " + _Q_POA + ", n = 172")

_poa_spec("energy_poa_reindl", "reindl", "Irradiance on a tilted plane (Reindl sky model)",
          "Beam, sky-diffuse (Reindl 1990: Hay-Davies plus horizon brightening sqrt(BHI/GHI) sin^3(beta/2)) and ground-reflected irradiance on a tilted surface; G_on from day n (Spencer).",
          "sky = DHI [(1 - A)(1 + cos beta)/2 (1 + sqrt(BHI/GHI) sin^3(beta/2)) + A R_b]",
          r"G_{d,t} = G_d\left[(1-A)\frac{1+\cos\beta}{2}\left(1+\sqrt{\frac{G_b}{G}}\sin^3\frac{\beta}{2}\right) + A R_b\right]",
          lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n: _poa("haydavies", tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n),
          True, {"poa_direct": 800.0}, "Plane of array irradiance (Reindl): " + _Q_POA + ", n = 172")


# --- decomposition and clear sky ----------------------------------------------------------------------------------
def _g_on_spencer(n):
    b = _day_angle(n)
    return SOLAR_CONSTANT * (1.00011 + 0.034221 * math.cos(b) + 0.00128 * math.sin(b) + 0.000719 * math.cos(2 * b) + 7.7e-05 * math.sin(2 * b))


def erbs(ghi, zenith, n):
    _check_day(n)
    _need(isinstance(zenith, (int, float)) and 0 <= zenith <= 85, "the zenith angle must lie between 0 and 85 degrees")
    _pos("the global horizontal irradiance", ghi)
    kt = ghi / (_g_on_spencer(n) * max(_cosd(zenith), 0.065))
    kt = min(max(kt, 0.0), 1.0)
    if kt <= 0.22:
        df = 1 - 0.09 * kt
    elif kt <= 0.8:
        df = 0.9511 - 0.1604 * kt + 4.388 * kt ** 2 - 16.638 * kt ** 3 + 12.336 * kt ** 4
    else:
        df = 0.165
    dhi = df * ghi
    return _out(kt=kt, diffuse_fraction=df, dhi=dhi, dni=(ghi - dhi) / _cosd(zenith))


def _ref_erbs(ghi, zenith, n):
    from pvlib import irradiance as irr
    r = irr.erbs(ghi, zenith, float(n))
    return {"kt": _np_scalar(r["kt"]), "dhi": _np_scalar(r["dhi"]), "dni": _np_scalar(r["dni"]),
            "diffuse_fraction": _np_scalar(r["dhi"]) / ghi}


def _rnd_erbs(rng):
    n, z = rng.randint(1, 365), round(rng.uniform(0, 80), 2)
    kt = rng.uniform(0.05, 0.95)
    return {"ghi": round(kt * _g_on_spencer(n) * _cosd(z), 3) or 1.0, "zenith": z, "n": n}


_add("energy_erbs_decomposition",
     name="Diffuse and direct irradiance from global horizontal irradiance (Erbs)", topic="solar resource / irradiance models",
     purpose="The clearness index, diffuse fraction, diffuse horizontal and direct normal irradiance estimated from a measured global horizontal irradiance by the Erbs (1982) correlation.",
     inputs=[_phys("ghi", "irradiance", "global horizontal irradiance"), _numin("zenith", "solar zenith angle in degrees"), _DAY],
     outputs=[_o("kt", "number", "clearness index GHI/(G_on cos z)"), _o("diffuse_fraction", "number", "diffuse fraction DHI/GHI"),
              _o("dhi", "irradiance", "diffuse horizontal irradiance"), _o("dni", "irradiance", "direct normal irradiance")],
     primary="dhi", conditions=["0 <= zenith <= 85", "GHI > 0", "kt is capped at 1"],
     method="kt = GHI/(G_on cos z); DHI/GHI = 1 - 0.09 kt (kt <= 0.22), 0.9511 - 0.1604 kt + 4.388 kt^2 - 16.638 kt^3 + 12.336 kt^4 (<= 0.8), 0.165; DNI = (GHI - DHI)/cos z",
     latex=r"\frac{G_d}{G} = 0.9511 - 0.1604k_t + 4.388k_t^2 - 16.638k_t^3 + 12.336k_t^4",
     run=erbs,
     parse=lambda q: _pairs(q, r"erbs(?: decomposition)?|diffuse fraction \(erbs\)", dict(_N_NAMES, ghi="ghi", GHI="ghi", zenith="zenith", z="zenith"),
                            phys={"ghi"}, num_units=_deg_units("zenith")),
     reference=_ref_erbs,
     wrong=_safe(lambda ghi, zenith, n: (lambda r: (lambda df: _out(kt=r["kt"], diffuse_fraction=df, dhi=df * ghi, dni=(1 - df) * ghi / _cosd(zenith)))(
         (1 - 0.09 * r["kt"]) if r["kt"] <= 0.22 else (0.9511 + 0.1604 * r["kt"] + 4.388 * r["kt"] ** 2 - 16.638 * r["kt"] ** 3 + 12.336 * r["kt"] ** 4) if r["kt"] <= 0.8 else 0.165))(erbs(ghi, zenith, n))),
     random_inputs=_rnd_erbs,
     templates=["Erbs decomposition: ghi = 600 W/m^2, zenith = 35, n = 172"],
     examples=[{"inputs": {"ghi": 0.8 * _g_on_spencer(172) * 0.5 + 0.0, "zenith": 60, "n": 172}, "outputs": {"kt": 0.8, "diffuse_fraction": _r(0.9511 - 0.1604 * 0.8 + 4.388 * 0.64 - 16.638 * 0.512 + 12.336 * 0.4096)}},
               {"inputs": {"ghi": 0.1 * _g_on_spencer(1), "zenith": 0, "n": 1}, "outputs": {"diffuse_fraction": 0.991}}],
     questions=[{"q": "Erbs decomposition: ghi = " + repr(0.1 * _g_on_spencer(1)) + " W/m^2, zenith = 0, n = 1", "outputs": {"diffuse_fraction": 0.991}},
                {"q": "Erbs decomposition: ghi = 600 W/m^2, zenith = 88, n = 172", "outcome": "INVALID_INPUT"}])


def haurwitz(zenith):
    _need(isinstance(zenith, (int, float)) and 0 <= zenith <= 180, "the zenith angle must lie between 0 and 180 degrees")
    c = _cosd(zenith)
    g = 1098.0 * c * math.exp(-0.059 / c) if c > 0 else 0.0
    return _out(ghi=g)


def _ref_haurwitz(zenith):
    import pandas as pd
    from pvlib import clearsky
    return {"ghi": float(clearsky.haurwitz(pd.Series([float(zenith)]))["ghi"].iloc[0])}


_add("energy_clear_sky_haurwitz",
     name="Clear-sky global horizontal irradiance (Haurwitz)", topic="solar resource / clear sky",
     purpose="The global horizontal irradiance under a clear sky from the solar zenith angle alone (Haurwitz 1945): 1098 cos z exp(-0.059/cos z).",
     inputs=[_numin("zenith", "apparent solar zenith angle in degrees")],
     outputs=[_o("ghi", "irradiance", "clear-sky global horizontal irradiance (0 when the sun is below the horizon)")],
     primary="ghi", conditions=["0 <= zenith <= 180"],
     method="GHI = 1098 cos z exp(-0.059/cos z) for cos z > 0, else 0", latex=r"G_{cs} = 1098\cos z\; e^{-0.059/\cos z}",
     run=haurwitz,
     parse=lambda q: _pairs(q, r"clear[- ]sky (?:ghi|irradiance)(?: \(haurwitz\))?|haurwitz(?: clear[- ]sky)?", {"zenith": "zenith", "z": "zenith"}, num_units=_deg_units("zenith")),
     reference=_ref_haurwitz,
     wrong=_safe(lambda zenith: _out(ghi=1098.0 * _cosd(zenith) * math.exp(0.059 / _cosd(zenith)) if _cosd(zenith) > 0 else 1.0)),
     random_inputs=lambda rng: {"zenith": round(rng.uniform(0, 88), 3)},
     templates=["Clear-sky GHI (Haurwitz): zenith = 30"],
     examples=[{"inputs": {"zenith": 0}, "outputs": {"ghi": _r(1098 * math.exp(-0.059))}}, {"inputs": {"zenith": 95}, "outputs": {"ghi": 0.0}}],
     questions=[{"q": "Clear-sky GHI (Haurwitz): zenith = 0", "outputs": {"ghi": _r(1098 * math.exp(-0.059))}},
                {"q": "Haurwitz clear sky: zenith = 200", "outcome": "INVALID_INPUT"}])


# ================================================================================================ PV TEMPERATURE AND POWER
def _check_temp(name, T):
    _need(isinstance(T, (int, float)) and 150 <= T <= 400, f"the {name} must lie between 150 K and 400 K")


def cell_temperature_noct(poa, T_air, noct):
    _nonneg("the plane-of-array irradiance", poa)
    _check_temp("air temperature", T_air)
    _check_temp("NOCT", noct)
    t = _degC(T_air) + (_degC(noct) - 20.) / 80. * poa * 0.1
    return _out(T_cell=t + 273.15, T_cell_C=t)


def _ref_noct(poa, T_air, noct):
    from pvlib import temperature as tmp
    t = _np_scalar(tmp.ross(poa, _degC(T_air), noct=_degC(noct)))
    return {"T_cell": t + 273.15, "T_cell_C": t}


_TEMP_NAMES = {"poa": "poa", "G": "poa", "irradiance": "poa", "T_air": "T_air", "Ta": "T_air", "T_ambient": "T_air", "ambient": "T_air",
               "noct": "noct", "NOCT": "noct", "wind": "wind", "wind_speed": "wind", "ws": "wind"}

_add("energy_cell_temperature_noct",
     name="PV cell temperature from NOCT", topic="photovoltaics / cell temperature",
     purpose="The operating temperature of a PV cell from the plane-of-array irradiance, the air temperature and the module's nominal operating cell temperature (NOCT): T_c = T_a + (NOCT - 20 degC) G/800.",
     inputs=[_phys("poa", "irradiance", "plane-of-array irradiance"), _phys("T_air", "temperature", "ambient air temperature"),
             _phys("noct", "temperature", "nominal operating cell temperature (at 800 W/m^2, 20 degC, 1 m/s)")],
     outputs=[_o("T_cell", "temperature", "cell temperature"), _o("T_cell_C", "number", "cell temperature in degC")],
     primary="T_cell_C", conditions=["irradiance >= 0"],
     method="T_c = T_a + (NOCT - 20)/800 G (Ross model with the NOCT coefficient)", latex=r"T_c = T_a + \frac{NOCT - 20^\circ\mathrm{C}}{800\ \mathrm{W/m^2}}\,G",
     run=cell_temperature_noct,
     parse=lambda q: _pairs(q, r"cell temperature \(noct\)|cell temperature from noct|noct cell temperature", _TEMP_NAMES, phys={"poa", "T_air", "noct"}),
     reference=_ref_noct,
     wrong=_safe(lambda poa, T_air, noct: (lambda t: _out(T_cell=t + 273.15, T_cell_C=t))(_degC(T_air) + (_degC(noct) - 25.) / 800. * poa)),
     random_inputs=lambda rng: {"poa": round(rng.uniform(100, 1100), 1), "T_air": round(rng.uniform(253, 318), 2), "noct": round(rng.uniform(315, 325), 2)},
     templates=["Cell temperature (NOCT): poa = 800 W/m^2, T_air = 25 degC, noct = 45 degC"],
     examples=[{"inputs": {"poa": 800, "T_air": 298.15, "noct": 318.15}, "outputs": {"T_cell_C": 50.0}},
               {"inputs": {"poa": 1000, "T_air": 293.15, "noct": 320.15}, "outputs": {"T_cell_C": 53.75}}],
     questions=[{"q": "Cell temperature (NOCT): poa = 800 W/m^2, T_air = 25 degC, noct = 45 degC", "outputs": {"T_cell_C": 50.0}},
                {"q": "Cell temperature (NOCT): poa = -800 W/m^2, T_air = 25 degC, noct = 45 degC", "outcome": "INVALID_INPUT"}])


def cell_temperature_faiman(poa, T_air, wind, u0=25.0, u1=6.84):
    _nonneg("the plane-of-array irradiance", poa)
    _check_temp("air temperature", T_air)
    _nonneg("the wind speed", wind)
    _pos("u0", u0)
    _nonneg("u1", u1)
    t = _degC(T_air) + poa / (u0 + u1 * wind)
    return _out(T_cell=t + 273.15, T_cell_C=t)


def _ref_faiman(poa, T_air, wind, u0=25.0, u1=6.84):
    from pvlib import temperature as tmp
    t = _np_scalar(tmp.faiman(poa, _degC(T_air), wind, u0, u1))
    return {"T_cell": t + 273.15, "T_cell_C": t}


_add("energy_cell_temperature_faiman",
     name="PV module temperature (Faiman model)", topic="photovoltaics / cell temperature",
     purpose="Module (cell) temperature by the Faiman heat-loss model used in IEC 61853: T = T_a + G/(u0 + u1 v).",
     inputs=[_phys("poa", "irradiance", "plane-of-array irradiance"), _phys("T_air", "temperature", "ambient air temperature"),
             _phys("wind", "speed", "wind speed at module height"),
             _numin("u0", "constant heat-loss coefficient in W/(m^2 K) (default 25)", required=False),
             _numin("u1", "wind heat-loss coefficient in W s/(m^3 K) (default 6.84)", required=False)],
     outputs=[_o("T_cell", "temperature", "module temperature"), _o("T_cell_C", "number", "module temperature in degC")],
     primary="T_cell_C", conditions=["G >= 0", "u0 > 0"],
     method="T = T_a + G/(u0 + u1 v)", latex=r"T_m = T_a + \frac{G}{u_0 + u_1 v}",
     run=cell_temperature_faiman,
     parse=lambda q: _pairs(q, r"module temperature \(faiman\)|cell temperature \(faiman\)|faiman (?:module |cell )?temperature", dict(_TEMP_NAMES, u0="u0", u1="u1"),
                            phys={"poa", "T_air", "wind"}),
     reference=_ref_faiman,
     wrong=_safe(lambda poa, T_air, wind, u0=25.0, u1=6.84: (lambda t: _out(T_cell=t + 273.15, T_cell_C=t))(_degC(T_air) + poa / (u0 + u1))),
     random_inputs=lambda rng: {"poa": round(rng.uniform(100, 1100), 1), "T_air": round(rng.uniform(253, 318), 2), "wind": round(rng.uniform(0.5, 10), 2),
                                "u0": round(rng.uniform(15, 35), 2), "u1": round(rng.uniform(2, 10), 2)},
     templates=["Module temperature (Faiman): poa = 1000 W/m^2, T_air = 25 degC, wind = 2 m/s"],
     examples=[{"inputs": {"poa": 1000, "T_air": 298.15, "wind": 0}, "outputs": {"T_cell_C": 65.0}},
               {"inputs": {"poa": 800, "T_air": 293.15, "wind": 1, "u0": 30, "u1": 10}, "outputs": {"T_cell_C": 40.0}}],
     questions=[{"q": "Module temperature (Faiman): poa = 1000 W/m^2, T_air = 25 degC, wind = 0 m/s", "outputs": {"T_cell_C": 65.0}},
                {"q": "Faiman temperature: poa = 800 W/m^2, T_air = 20 degC, wind = 1 m/s, u0 = 30, u1 = 10", "outputs": {"T_cell_C": 40.0}},
                {"q": "Module temperature (Faiman): poa = 1000 W/m^2, T_air = 25 degC", "outcome": "NEEDS_INPUT"}])

_SAPM_MOUNTS = {"open_rack_glass_glass": (-3.47, -0.0594, 3.0), "close_mount_glass_glass": (-2.98, -0.0471, 1.0),
                "open_rack_glass_polymer": (-3.56, -0.075, 3.0), "insulated_back_glass_polymer": (-2.81, -0.0455, 0.0)}


def cell_temperature_sapm(poa, T_air, wind, mount):
    _nonneg("the plane-of-array irradiance", poa)
    _check_temp("air temperature", T_air)
    _nonneg("the wind speed", wind)
    a, b, dT = _SAPM_MOUNTS[mount]
    tm = poa * math.exp(a + b * wind) + _degC(T_air)
    tc = tm + poa / 1000. * dT
    return _out(T_module_C=tm, T_cell_C=tc, T_cell=tc + 273.15)


def _ref_sapm_temp(poa, T_air, wind, mount):
    from pvlib import temperature as tmp
    p = tmp.TEMPERATURE_MODEL_PARAMETERS["sapm"][mount]
    tc = _np_scalar(tmp.sapm_cell(poa, _degC(T_air), wind, **p))
    return {"T_module_C": _np_scalar(tmp.sapm_module(poa, _degC(T_air), wind, p["a"], p["b"])), "T_cell_C": tc, "T_cell": tc + 273.15}


_add("energy_cell_temperature_sapm",
     name="PV cell and module temperature (Sandia SAPM model)", topic="photovoltaics / cell temperature",
     purpose="Back-of-module and cell temperature by the Sandia Array Performance Model, T_m = G exp(a + b v) + T_a, T_c = T_m + (G/1000) dT, for the standard module/mounting parameter sets.",
     inputs=[_phys("poa", "irradiance", "plane-of-array irradiance"), _phys("T_air", "temperature", "ambient air temperature"),
             _phys("wind", "speed", "wind speed at 10 m"), _choice("mount", list(_SAPM_MOUNTS), "module construction and mounting (Sandia parameter set)")],
     outputs=[_o("T_module_C", "number", "back-of-module temperature in degC"), _o("T_cell_C", "number", "cell temperature in degC"), _o("T_cell", "temperature", "cell temperature")],
     primary="T_cell_C", conditions=["G >= 0"],
     method="T_m = G exp(a + b v) + T_a; T_c = T_m + G dT/1000 (a, b, dT from the Sandia table)", latex=r"T_m = G\,e^{a + b v} + T_a,\quad T_c = T_m + \frac{G}{1000}\Delta T",
     run=cell_temperature_sapm,
     parse=lambda q: _pairs(q, r"cell temperature \(sapm\)|sapm cell temperature|sandia cell temperature", dict(_TEMP_NAMES, mount="mount", mounting="mount"),
                            phys={"poa", "T_air", "wind"}, words={"mount"}),
     reference=_ref_sapm_temp,
     wrong=_safe(lambda poa, T_air, wind, mount: (lambda a, b, dT: (lambda tm: _out(T_module_C=tm, T_cell_C=tm + poa / 1000 * dT, T_cell=tm + poa / 1000 * dT + 273.15))(
         poa * math.exp(a - b * wind) + _degC(T_air)))(*_SAPM_MOUNTS[mount])),
     random_inputs=lambda rng: {"poa": round(rng.uniform(100, 1100), 1), "T_air": round(rng.uniform(253, 318), 2), "wind": round(rng.uniform(0.5, 10), 2),
                                "mount": rng.choice(list(_SAPM_MOUNTS))},
     templates=["Cell temperature (SAPM): poa = 1000 W/m^2, T_air = 10 degC, wind = 0 m/s, mount = open_rack_glass_glass"],
     examples=[{"inputs": {"poa": 1000, "T_air": 283.15, "wind": 0, "mount": "open_rack_glass_glass"}, "outputs": {"T_cell_C": _r(1000 * math.exp(-3.47) + 13)}}],
     questions=[{"q": "Cell temperature (SAPM): poa = 1000 W/m^2, T_air = 10 degC, wind = 0 m/s, mount = open_rack_glass_glass", "outputs": {"T_cell_C": _r(1000 * math.exp(-3.47) + 13)}},
                {"q": "Cell temperature (SAPM): poa = 1000 W/m^2, T_air = 10 degC, wind = 0 m/s, mount = on_the_moon", "outcome": "INVALID_INPUT"}])


def pvwatts_dc(G_eff, T_cell, pdc0, gamma):
    _nonneg("the effective irradiance", G_eff)
    _check_temp("cell temperature", T_cell)
    _pos("the DC rating pdc0", pdc0)
    _need(-0.02 <= gamma <= 0.01, "the temperature coefficient gamma must lie between -0.02 and 0.01 per K (e.g. -0.004 or -0.4%)")
    p = G_eff * 0.001 * pdc0 * (1 + gamma * (_degC(T_cell) - 25.))
    return _out(pdc=p, pdc_kW=p / 1000)


def _ref_pvwatts_dc(G_eff, T_cell, pdc0, gamma):
    from pvlib import pvsystem
    p = _np_scalar(pvsystem.pvwatts_dc(G_eff, _degC(T_cell), pdc0, gamma))
    return {"pdc": p, "pdc_kW": p / 1000}


_add("energy_pvwatts_dc",
     name="PV array DC power with temperature derating (PVWatts)", topic="photovoltaics / array output",
     purpose="DC power of a PV array from the effective irradiance and cell temperature (PVWatts): P = (G/1000) P_dc0 (1 + gamma (T_c - 25 degC)).",
     inputs=[_phys("G_eff", "irradiance", "effective (transmitted) irradiance on the cells"), _phys("T_cell", "temperature", "cell temperature"),
             _phys("pdc0", "power", "DC rating at 1000 W/m^2 and 25 degC"), _numin("gamma", "power temperature coefficient per K (e.g. -0.004, or -0.4%)")],
     outputs=[_o("pdc", "power", "DC power"), _o("pdc_kW", "number", "DC power in kW")],
     primary="pdc", conditions=["-0.02 <= gamma <= 0.01 per K"],
     method="P_dc = (G/1000) P_dc0 [1 + gamma (T_c - 25)]", latex=r"P_{dc} = \frac{G}{1000}P_{dc0}\left[1 + \gamma(T_c - 25^\circ\mathrm{C})\right]",
     run=pvwatts_dc,
     parse=lambda q: _pairs(q, r"pvwatts dc(?: power)?|pv dc power|dc power of a pv array", {"G_eff": "G_eff", "G": "G_eff", "irradiance": "G_eff", "T_cell": "T_cell", "Tc": "T_cell",
                                                                                           "pdc0": "pdc0", "P_dc0": "pdc0", "rating": "pdc0", "gamma": "gamma"},
                            phys={"G_eff", "T_cell", "pdc0"}, num_units=_pct_units("gamma")),
     reference=_ref_pvwatts_dc,
     wrong=_safe(lambda G_eff, T_cell, pdc0, gamma: (lambda p: _out(pdc=p, pdc_kW=p / 1000))(G_eff * 0.001 * pdc0 * (1 + gamma * (_degC(T_cell) - 20.)))),
     random_inputs=lambda rng: {"G_eff": round(rng.uniform(100, 1100), 1), "T_cell": round(rng.uniform(263, 348), 2), "pdc0": round(rng.uniform(300, 5e6), 0),
                                "gamma": round(rng.uniform(-0.005, -0.002), 5)},
     templates=["PVWatts DC power: G_eff = 800 W/m^2, T_cell = 45 degC, pdc0 = 5 kW, gamma = -0.4%"],
     examples=[{"inputs": {"G_eff": 800, "T_cell": 318.15, "pdc0": 5000, "gamma": -0.004}, "outputs": {"pdc": 3680.0}},
               {"inputs": {"G_eff": 1000, "T_cell": 298.15, "pdc0": 300, "gamma": -0.0035}, "outputs": {"pdc": 300.0}}],
     questions=[{"q": "PVWatts DC power: G_eff = 800 W/m^2, T_cell = 45 degC, pdc0 = 5 kW, gamma = -0.4%", "outputs": {"pdc": 3680.0}},
                {"q": "PVWatts DC power: G_eff = 800 W/m^2, T_cell = 45 degC, pdc0 = 5 kW, gamma = -0.4", "outcome": "INVALID_INPUT"}])


def pv_array_output(poa, T_air, noct, pdc0, gamma):
    t = cell_temperature_noct(poa, T_air, noct)
    p = pvwatts_dc(poa, t["T_cell"], pdc0, gamma)
    return _out(T_cell=t["T_cell"], T_cell_C=t["T_cell_C"], pdc=p["pdc"], derate=p["pdc"] / (poa * 0.001 * pdc0) if poa > 0 else 1.0)


def _ref_pv_array(poa, T_air, noct, pdc0, gamma):
    from pvlib import pvsystem, temperature as tmp
    tc = _np_scalar(tmp.ross(poa, _degC(T_air), noct=_degC(noct)))
    p = _np_scalar(pvsystem.pvwatts_dc(poa, tc, pdc0, gamma))
    return {"T_cell": tc + 273.15, "T_cell_C": tc, "pdc": p, "derate": p / (poa * 0.001 * pdc0)}


_add("energy_pv_array_output",
     name="PV array output at operating temperature", topic="photovoltaics / array output",
     purpose="Cell temperature from NOCT and the resulting DC output of an array with a power temperature coefficient, from the plane-of-array irradiance and the air temperature.",
     inputs=[_phys("poa", "irradiance", "plane-of-array irradiance"), _phys("T_air", "temperature", "ambient air temperature"),
             _phys("noct", "temperature", "nominal operating cell temperature"), _phys("pdc0", "power", "array DC rating at STC"),
             _numin("gamma", "power temperature coefficient per K (e.g. -0.004, or -0.4%)")],
     outputs=[_o("T_cell", "temperature", "cell temperature"), _o("T_cell_C", "number", "cell temperature in degC"), _o("pdc", "power", "DC output"),
              _o("derate", "number", "temperature derating factor 1 + gamma (T_c - 25)")],
     primary="pdc", conditions=["G > 0", "-0.02 <= gamma <= 0.01 per K"],
     method="T_c = T_a + (NOCT - 20) G/800; P = (G/1000) P_dc0 [1 + gamma (T_c - 25)]",
     latex=r"P = \frac{G}{1000}P_{dc0}\left[1 + \gamma\left(T_a + \frac{NOCT-20}{800}G - 25\right)\right]",
     run=pv_array_output,
     parse=lambda q: _pairs(q, r"pv array output|pv output at operating temperature|solar array output", dict(_TEMP_NAMES, pdc0="pdc0", rating="pdc0", gamma="gamma"),
                            phys={"poa", "T_air", "noct", "pdc0"}, num_units=_pct_units("gamma")),
     reference=_ref_pv_array,
     wrong=_safe(lambda poa, T_air, noct, pdc0, gamma: (lambda p: _out(T_cell=T_air, T_cell_C=_degC(T_air), pdc=p, derate=p / (poa * 0.001 * pdc0)))(
         poa * 0.001 * pdc0 * (1 + gamma * (_degC(T_air) - 25)))),
     random_inputs=lambda rng: {"poa": round(rng.uniform(200, 1100), 1), "T_air": round(rng.uniform(263, 318), 2), "noct": round(rng.uniform(315, 325), 2),
                                "pdc0": round(rng.uniform(300, 5e6), 0), "gamma": round(rng.uniform(-0.005, -0.002), 5)},
     templates=["PV array output: poa = 800 W/m^2, T_air = 25 degC, noct = 45 degC, pdc0 = 10 kW, gamma = -0.4%"],
     examples=[{"inputs": {"poa": 800, "T_air": 298.15, "noct": 318.15, "pdc0": 10000, "gamma": -0.004}, "outputs": {"T_cell_C": 50.0, "pdc": 7200.0, "derate": 0.9}}],
     questions=[{"q": "PV array output: poa = 800 W/m^2, T_air = 25 degC, noct = 45 degC, pdc0 = 10 kW, gamma = -0.4%", "outputs": {"pdc": 7200.0}},
                {"q": "PV array output: poa = 0 W/m^2, T_air = 25 degC, noct = 45 degC, pdc0 = 10 kW, gamma = -0.4%", "outcome": "INVALID_INPUT"}])


def _pv_array_check(poa, T_air, noct, pdc0, gamma):
    _pos("the plane-of-array irradiance", poa)
    return pv_array_output(poa, T_air, noct, pdc0, gamma)


PROCEDURES["energy_pv_array_output"]["run"] = _pv_array_check
PROCEDURES["energy_pv_array_output"]["verify"] = _recheck(_pv_array_check)


def pvwatts_inverter(pdc, pdc0, eta_nom=0.96, eta_ref=0.9637):
    _pos("the DC input power", pdc)
    _pos("the inverter DC rating pdc0", pdc0)
    _need(0.5 < eta_nom <= 1 and 0.5 < eta_ref <= 1, "the efficiencies must lie between 0.5 and 1")
    z = pdc / pdc0
    eta = eta_nom / eta_ref * (-0.0162 * z - 0.0059 / z + 0.9858)
    pac = max(0.0, min(eta_nom * pdc0, eta * pdc))
    return _out(pac=pac, efficiency=pac / pdc, clipped=pac == eta_nom * pdc0)


def _ref_pvwatts_inv(pdc, pdc0, eta_nom=0.96, eta_ref=0.9637):
    from pvlib import inverter
    pac = _np_scalar(inverter.pvwatts(pdc, pdc0, eta_nom, eta_ref))
    return {"pac": pac, "efficiency": pac / pdc, "clipped": bool(pac >= eta_nom * pdc0)}


_add("energy_pvwatts_inverter",
     name="Inverter AC output and efficiency (PVWatts)", topic="photovoltaics / inverters",
     purpose="AC output and efficiency of an inverter at a DC input by the PVWatts part-load efficiency curve, with clipping at the AC rating eta_nom P_dc0.",
     inputs=[_phys("pdc", "power", "DC input power"), _phys("pdc0", "power", "inverter DC input rating"),
             _numin("eta_nom", "nominal efficiency (default 0.96)", required=False, ), _numin("eta_ref", "reference efficiency of the curve (default 0.9637)", required=False)],
     outputs=[_o("pac", "power", "AC output power"), _o("efficiency", "number", "conversion efficiency P_ac/P_dc"), _o("clipped", "boolean", "true when the output is limited at the AC rating")],
     primary="pac", conditions=["P_dc > 0"],
     method="zeta = P_dc/P_dc0; eta = (eta_nom/eta_ref)(-0.0162 zeta - 0.0059/zeta + 0.9858); P_ac = min(eta P_dc, eta_nom P_dc0)",
     latex=r"\eta = \frac{\eta_{nom}}{\eta_{ref}}\left(-0.0162\zeta - \frac{0.0059}{\zeta} + 0.9858\right),\ \zeta = \frac{P_{dc}}{P_{dc0}}",
     run=pvwatts_inverter,
     parse=lambda q: _pairs(q, r"inverter (?:ac )?output(?: \(pvwatts\))?|pvwatts inverter", {"pdc": "pdc", "P_dc": "pdc", "pdc0": "pdc0", "P_dc0": "pdc0", "rating": "pdc0",
                                                                                        "eta_nom": "eta_nom", "eta_ref": "eta_ref"},
                            phys={"pdc", "pdc0"}, num_units=_pct_units("eta_nom", "eta_ref")),
     reference=_ref_pvwatts_inv,
     wrong=_safe(lambda pdc, pdc0, eta_nom=0.96, eta_ref=0.9637: _out(pac=eta_nom * pdc, efficiency=eta_nom, clipped=False)),
     random_inputs=lambda rng: (lambda p0: {"pdc": round(p0 * rng.uniform(0.05, 1.2), 2), "pdc0": p0, "eta_nom": round(rng.uniform(0.94, 0.985), 4),
                                            "eta_ref": 0.9637})(round(rng.uniform(1e3, 1e6), 0)),
     templates=["Inverter output (PVWatts): pdc = 2 kW, pdc0 = 5 kW"],
     examples=[{"inputs": {"pdc": 5000, "pdc0": 5000}, "outputs": {"pac": _r(min(4800, 0.96 / 0.9637 * (-0.0162 - 0.0059 + 0.9858) * 5000))}},
               {"inputs": {"pdc": 6000, "pdc0": 5000}, "outputs": {"pac": 4800.0, "clipped": True}}],
     questions=[{"q": "Inverter output (PVWatts): pdc = 6 kW, pdc0 = 5 kW", "outputs": {"pac": 4800.0}},
                {"q": "Inverter output (PVWatts): pdc = 0 kW, pdc0 = 5 kW", "outcome": "INVALID_INPUT"}])


def iam_ashrae(aoi, b0=0.05):
    _need(isinstance(aoi, (int, float)) and 0 <= aoi <= 180, "the angle of incidence must lie between 0 and 180 degrees")
    _need(0 < b0 < 1, "b0 must lie between 0 and 1")
    if aoi >= 90:
        return _out(iam=0.0)
    return _out(iam=max(0.0, 1 - b0 * (1 / math.cos(math.radians(aoi)) - 1)))


def _ref_iam_ashrae(aoi, b0=0.05):
    from pvlib import iam
    return {"iam": _np_scalar(iam.ashrae(float(aoi), b0))}


_add("energy_iam_ashrae",
     name="Incidence angle modifier (ASHRAE)", topic="photovoltaics / optical losses",
     purpose="The fraction of beam irradiance transmitted into a PV module at an angle of incidence, by the ASHRAE model IAM = 1 - b0 (1/cos(theta) - 1), floored at 0.",
     inputs=[_numin("aoi", "angle of incidence in degrees"), _numin("b0", "ASHRAE coefficient (default 0.05)", required=False)],
     outputs=[_o("iam", "number", "incidence angle modifier")], primary="iam", conditions=["0 <= aoi <= 180"],
     method="IAM = max(0, 1 - b0 (sec(theta) - 1)); 0 for theta >= 90", latex=r"IAM = 1 - b_0\left(\frac{1}{\cos\theta} - 1\right)",
     run=iam_ashrae,
     parse=lambda q: _pairs(q, r"incidence angle modifier \(ashrae\)|ashrae iam|iam \(ashrae\)", {"aoi": "aoi", "theta": "aoi", "b0": "b0", "b": "b0"}, num_units=_deg_units("aoi")),
     reference=_ref_iam_ashrae,
     wrong=_safe(lambda aoi, b0=0.05: _out(iam=1 - b0 * (1 - math.cos(math.radians(aoi))))),
     random_inputs=lambda rng: {"aoi": round(rng.uniform(5, 85), 3), "b0": round(rng.uniform(0.03, 0.07), 4)},
     templates=["Incidence angle modifier (ASHRAE): aoi = 60"],
     examples=[{"inputs": {"aoi": 60}, "outputs": {"iam": 0.95}}, {"inputs": {"aoi": 0}, "outputs": {"iam": 1.0}}],
     questions=[{"q": "Incidence angle modifier (ASHRAE): aoi = 60", "outputs": {"iam": 0.95}},
                {"q": "ASHRAE IAM: aoi = 60, b0 = 2", "outcome": "INVALID_INPUT"}])


def iam_physical(aoi, n=1.526, K=4.0, L=0.002):
    _need(isinstance(aoi, (int, float)) and 0 <= aoi < 90, "the angle of incidence must lie between 0 and 90 degrees")
    _need(1.0 < n < 3.0, "the refractive index must lie between 1 and 3")
    _nonneg("the extinction coefficient K and the thickness L", K, L)
    c1 = max(0.0, _cosd(aoi))
    s1 = math.sqrt(1 - c1 ** 2)
    s2 = s1 / n
    c2 = math.sqrt(1 - s2 ** 2)
    rs = ((c1 - n * c2) / (c1 + n * c2)) ** 2
    rp = ((c2 - n * c1) / (c2 + n * c1)) ** 2
    r0 = ((1 - n) / (1 + n)) ** 2
    ts = (1 - rs) * math.exp(-K * L / c2)
    tp = (1 - rp) * math.exp(-K * L / c2)
    t0 = (1 - r0) * math.exp(-K * L)
    return _out(iam=(ts + tp) / 2 / t0)


def _ref_iam_physical(aoi, n=1.526, K=4.0, L=0.002):
    from pvlib import iam
    return {"iam": _np_scalar(iam.physical(float(aoi), n, K, L))}


_add("energy_iam_physical",
     name="Incidence angle modifier from Fresnel reflection and absorption", topic="photovoltaics / optical losses",
     purpose="The incidence angle modifier of a glass cover from Fresnel reflection (both polarizations) and absorption in the glass (De Soto / physical model), normalised to normal incidence.",
     inputs=[_numin("aoi", "angle of incidence in degrees"), _numin("n", "refractive index of the glass (default 1.526)", required=False),
             _numin("K", "glazing extinction coefficient in 1/m (default 4)", required=False), _phys("L", "length", "glazing thickness (default 2 mm)", required=False)],
     outputs=[_o("iam", "number", "incidence angle modifier")], primary="iam", conditions=["0 <= aoi < 90", "1 < n < 3"],
     method="Snell's law, Fresnel reflectances r_s, r_p, transmission tau = (1 - r) exp(-K L/cos theta_r), IAM = (tau_s + tau_p)/2/tau(0)",
     latex=r"IAM = \frac{\tfrac12\left[(1-r_s)+(1-r_p)\right]e^{-KL/\cos\theta_r}}{(1-r_0)e^{-KL}}",
     run=iam_physical,
     parse=lambda q: _pairs(q, r"incidence angle modifier \(physical\)|physical iam|fresnel iam|iam \(physical\)", {"aoi": "aoi", "theta": "aoi", "n": "n", "K": "K", "L": "L"},
                            phys={"L"}, num_units=_deg_units("aoi")),
     reference=_ref_iam_physical,
     wrong=_safe(lambda aoi, n=1.526, K=4.0, L=0.002: (lambda r: _out(iam=1 - (1 - r["iam"]) * 2))(iam_physical(aoi, n, K, L))),
     random_inputs=lambda rng: {"aoi": round(rng.uniform(5, 85), 3), "n": round(rng.uniform(1.3, 1.7), 4), "K": round(rng.uniform(2, 8), 3), "L": round(rng.uniform(0.001, 0.004), 5)},
     templates=["Incidence angle modifier (physical): aoi = 60"],
     examples=[{"inputs": {"aoi": 0}, "outputs": {"iam": 1.0}}],
     questions=[{"q": "Incidence angle modifier (physical): aoi = 0", "outputs": {"iam": 1.0}},
                {"q": "Incidence angle modifier (physical): aoi = 95", "outcome": "INVALID_INPUT"}])


_PVW_LOSSES = [("soiling", 2.0), ("shading", 3.0), ("snow", 0.0), ("mismatch", 2.0), ("wiring", 2.0), ("connections", 0.5), ("lid", 1.5),
               ("nameplate_rating", 1.0), ("age", 0.0), ("availability", 3.0)]


def pvwatts_losses(**kw):
    perf = 1.0
    for name, default in _PVW_LOSSES:
        v = kw.get(name, default / 100.0)
        _need(isinstance(v, (int, float)) and 0 <= v < 1, f"the {name} loss must lie between 0 and 100%")
        perf *= 1 - v
    return _out(total_loss=1 - perf, performance_factor=perf, total_loss_pct=(1 - perf) * 100)


def _ref_pvwatts_losses(**kw):
    from pvlib import pvsystem
    args = {name: kw.get(name, d / 100.0) * 100.0 for name, d in _PVW_LOSSES}
    L = _np_scalar(pvsystem.pvwatts_losses(**args))
    return {"total_loss": L / 100, "performance_factor": 1 - L / 100, "total_loss_pct": L}


_add("energy_pvwatts_losses",
     name="Combined PV system losses (PVWatts)", topic="photovoltaics / system losses",
     purpose="The total system loss of a PV system from its individual loss percentages, combined multiplicatively as in PVWatts: 1 - prod(1 - L_i); unstated losses take the PVWatts defaults (soiling 2%, shading 3%, snow 0%, mismatch 2%, wiring 2%, connections 0.5%, light-induced degradation 1.5%, nameplate 1%, age 0%, availability 3%).",
     inputs=[_numin(nm, f"{nm.replace('_', ' ')} loss as a fraction or a percentage (default {d}%)", required=False) for nm, d in _PVW_LOSSES],
     outputs=[_o("total_loss", "number", "total loss as a fraction"), _o("performance_factor", "number", "product of (1 - loss)"), _o("total_loss_pct", "number", "total loss in percent")],
     primary="total_loss_pct", conditions=["each loss between 0 and 100%"],
     method="L = 1 - prod(1 - L_i)", latex=r"L = 1 - \prod_i (1 - L_i)",
     run=pvwatts_losses,
     parse=lambda q: _pairs(q, r"pv system losses|pvwatts losses|total system losses", {nm: nm for nm, _ in _PVW_LOSSES}, num_units=_pct_units(*[nm for nm, _ in _PVW_LOSSES])),
     reference=_ref_pvwatts_losses,
     wrong=_safe(lambda **kw: (lambda s: _out(total_loss=s, performance_factor=1 - s, total_loss_pct=100 * s))(sum(kw.get(nm, d / 100) for nm, d in _PVW_LOSSES))),
     random_inputs=lambda rng: {nm: round(rng.uniform(0, 0.08), 4) for nm, _ in _PVW_LOSSES if rng.random() < 0.7},
     templates=["PV system losses: soiling = 2%, shading = 3%", "PVWatts losses: shading = 0%"],
     examples=[{"inputs": {}, "outputs": {"total_loss_pct": _r((1 - 0.98 * 0.97 * 1 * 0.98 * 0.98 * 0.995 * 0.985 * 0.99 * 1 * 0.97) * 100)}},
               {"inputs": {nm: 0.0 for nm, _ in _PVW_LOSSES}, "outputs": {"total_loss": 0.0}}],
     questions=[{"q": "PVWatts losses: soiling = 0%, shading = 0%, snow = 0%, mismatch = 0%, wiring = 0%, connections = 0%, lid = 0%, nameplate_rating = 0%, age = 0%, availability = 10%",
                 "outputs": {"total_loss_pct": 10.0}},
                {"q": "PV system losses: soiling = 120%", "outcome": "INVALID_INPUT"}])


# ================================================================================================ WIND POWER
BETZ = 16.0 / 27.0


def wind_power(rho, D, v, cp=None):
    _pos("the air density, rotor diameter and wind speed", rho, D, v)
    A = math.pi * D * D / 4
    pw = 0.5 * rho * A * v ** 3
    out = {"P_wind": pw, "P_betz": BETZ * pw, "power_density": 0.5 * rho * v ** 3, "area": A}
    if cp is not None:
        _need(0 < cp <= BETZ + 1e-12, "the power coefficient must lie between 0 and the Betz limit 16/27 = 0.593")
        out["P"] = cp * pw
    return _out(**out)


def _ref_wind_power(rho, D, v, cp=None):
    import numpy as np
    from windpowerlib import power_output
    pc = lambda c, d: float(power_output.power_coefficient_curve(np.array([v]), [0.0, 200.0], [c, c], d, np.array([rho]))[0])   # noqa: E731
    pw = pc(1.0, D)
    pd_ = pc(1.0, math.sqrt(4 / math.pi))                 # a rotor of 1 m^2
    out = {"P_wind": pw, "P_betz": pc(16 / 27, D), "power_density": pd_, "area": pw / pd_}
    if cp is not None:
        out["P"] = pc(cp, D)
    return out


_add("energy_wind_power",
     name="Power in the wind and the Betz limit", topic="wind power / wind resource",
     purpose="The kinetic power of the wind through a rotor's swept area, the maximum a turbine can extract (Betz limit 16/27), the power density, and the power at a given power coefficient.",
     inputs=[_phys("rho", "density", "air density"), _phys("D", "length", "rotor diameter"), _phys("v", "speed", "wind speed"),
             _numin("cp", "power coefficient (optional, at most 16/27)", required=False)],
     outputs=[_o("P_wind", "power", "power in the wind through the swept area"), _o("P_betz", "power", "maximum extractable power (Betz)"),
              _o("power_density", "number", "wind power density in W/m^2"), _o("area", "area", "swept area"), _o("P", "power", "power at the given power coefficient")],
     primary="P_wind", conditions=["rho, D, v > 0", "0 < cp <= 16/27"],
     method="P = (1/2) rho A v^3, A = pi D^2/4; P_Betz = (16/27) P; P(cp) = cp P", latex=r"P = \tfrac12\rho A v^3,\quad P_{max} = \tfrac{16}{27}\cdot\tfrac12\rho A v^3",
     run=wind_power,
     parse=lambda q: _pairs(q, r"power in the wind|wind power|betz limit", {"rho": "rho", "density": "rho", "D": "D", "diameter": "D", "rotor_diameter": "D",
                                                                           "v": "v", "wind_speed": "v", "speed": "v", "cp": "cp", "Cp": "cp"},
                            phys={"rho", "D", "v"}, num_units=_pct_units("cp")),
     reference=_ref_wind_power,
     wrong=_safe(lambda rho, D, v, cp=None: (lambda pw: _out(**dict({"P_wind": pw, "P_betz": BETZ * pw, "power_density": 0.5 * rho * v * v, "area": math.pi * D * D / 4},
                                                                     **({"P": cp * pw} if cp is not None else {}))))(0.5 * rho * math.pi * D * D / 4 * v * v)),
     random_inputs=lambda rng: dict({"rho": round(rng.uniform(1.0, 1.3), 4), "D": round(rng.uniform(1, 170), 2), "v": round(rng.uniform(2, 25), 2)},
                                    **({"cp": round(rng.uniform(0.2, 0.59), 3)} if rng.random() < 0.5 else {})),
     templates=["Power in the wind: rho = 1.225 kg/m^3, D = 80 m, v = 10 m/s", "Wind power: rho = 1.2 kg/m^3, D = 100 m, v = 12 m/s, cp = 0.45"],
     examples=[{"inputs": {"rho": 1.2, "D": 2 / math.sqrt(math.pi), "v": 10}, "outputs": {"P_wind": 600.0, "power_density": 600.0, "P_betz": _r(600 * 16 / 27)}},
               {"inputs": {"rho": 1.225, "D": 80, "v": 10, "cp": 0.4}, "outputs": {"P": _r(0.4 * 0.5 * 1.225 * math.pi * 1600 * 1000)}}],
     questions=[{"q": "Power in the wind: rho = 1.225 kg/m^3, D = 80 m, v = 10 m/s, cp = 0.4", "outputs": {"P": _r(0.4 * 0.5 * 1.225 * math.pi * 1600 * 1000)}},
                {"q": "Wind power: rho = 1.2 kg/m^3, D = 100 m, v = 12 m/s, cp = 0.7", "outcome": "INVALID_INPUT"},
                {"q": "Wind power: rho = 1.2 kg/m^3, D = 100 m", "outcome": "NEEDS_INPUT"}])


def actuator_disc(a, rho, D, v):
    _need(isinstance(a, (int, float)) and 0 <= a < 0.5, "the axial induction factor a must lie in [0, 0.5) (momentum theory fails beyond 0.5)")
    _pos("the air density, rotor diameter and wind speed", rho, D, v)
    A = math.pi * D * D / 4
    cp, ct = 4 * a * (1 - a) ** 2, 4 * a * (1 - a)
    q = 0.5 * rho * A * v ** 2
    return _out(Cp=cp, Ct=ct, v_disc=v * (1 - a), v_wake=v * (1 - 2 * a), P=cp * q * v, T=ct * q)


def _ref_actuator(a, rho, D, v):
    """Momentum and energy balance on the stream tube (no closed-form coefficients): mass flow through the disc,
    thrust from the momentum change, power from the kinetic-energy change."""
    A = math.pi * D * D / 4
    v2, v3 = v * (1 - a), v * (1 - 2 * a)
    mdot = rho * A * v2
    T = mdot * (v - v3)
    P = 0.5 * mdot * (v * v - v3 * v3)
    return {"Cp": P / (0.5 * rho * A * v ** 3), "Ct": T / (0.5 * rho * A * v * v), "v_disc": v2, "v_wake": v3, "P": P, "T": T}


_add("energy_actuator_disc",
     name="Actuator-disc (momentum theory) rotor performance", topic="wind power / rotor aerodynamics",
     purpose="Power and thrust coefficients, disc and far-wake speeds, power and thrust of an ideal rotor (actuator disc) with axial induction factor a; a = 1/3 gives the Betz optimum Cp = 16/27.",
     inputs=[_numin("a", "axial induction factor (0 <= a < 0.5)"), _phys("rho", "density", "air density"), _phys("D", "length", "rotor diameter"),
             _phys("v", "speed", "free-stream wind speed")],
     outputs=[_o("Cp", "number", "power coefficient 4a(1-a)^2"), _o("Ct", "number", "thrust coefficient 4a(1-a)"), _o("v_disc", "speed", "speed at the disc"),
              _o("v_wake", "speed", "far-wake speed"), _o("P", "power", "power extracted"), _o("T", "force", "thrust on the rotor")],
     primary="Cp", conditions=["0 <= a < 0.5"],
     method="Cp = 4a(1 - a)^2, Ct = 4a(1 - a), v_disc = v(1 - a), v_wake = v(1 - 2a); P = Cp (1/2) rho A v^3, T = Ct (1/2) rho A v^2",
     latex=r"C_P = 4a(1-a)^2,\quad C_T = 4a(1-a)",
     run=actuator_disc,
     parse=lambda q: _pairs(q, r"actuator disc|momentum theory rotor|axial induction", {"a": "a", "rho": "rho", "density": "rho", "D": "D", "diameter": "D", "v": "v", "wind_speed": "v"},
                            phys={"rho", "D", "v"}),
     reference=_ref_actuator,
     wrong=_safe(lambda a, rho, D, v: (lambda r: dict(r, Cp=_r(4 * a * (1 - a)), P=_r(4 * a * (1 - a) * 0.5 * rho * math.pi * D * D / 4 * v ** 3)))(actuator_disc(a, rho, D, v))),
     random_inputs=lambda rng: {"a": round(rng.uniform(0.02, 0.48), 4), "rho": round(rng.uniform(1.0, 1.3), 4), "D": round(rng.uniform(1, 170), 2), "v": round(rng.uniform(2, 25), 2)},
     templates=["Actuator disc: a = 0.3333333333333333, rho = 1.225 kg/m^3, D = 80 m, v = 10 m/s"],
     examples=[{"inputs": {"a": 1 / 3, "rho": 1.2, "D": 10, "v": 8}, "outputs": {"Cp": _r(16 / 27), "Ct": _r(8 / 9), "v_wake": _r(8 / 3)}},
               {"inputs": {"a": 0.25, "rho": 1.2, "D": 10, "v": 8}, "outputs": {"Cp": 0.5625, "Ct": 0.75, "v_disc": 6.0}}],
     questions=[{"q": "Actuator disc: a = 0.25, rho = 1.2 kg/m^3, D = 10 m, v = 8 m/s", "outputs": {"Cp": 0.5625, "v_wake": 4.0}},
                {"q": "Actuator disc: a = 0.6, rho = 1.2 kg/m^3, D = 10 m, v = 8 m/s", "outcome": "INVALID_INPUT"}])


def wind_shear_power_law(v1, h1, h2, alpha=1 / 7):
    _pos("the wind speed and the heights", v1, h1, h2)
    _need(0 <= alpha <= 1, "the shear exponent alpha must lie between 0 and 1")
    return _out(v2=v1 * (h2 / h1) ** alpha)


def _ref_hellman(v1, h1, h2, alpha=1 / 7):
    from windpowerlib import wind_speed
    return {"v2": float(wind_speed.hellman(v1, h1, h2, hellman_exponent=alpha))}


_SHEAR_NAMES = {"v1": "v1", "v": "v1", "v_ref": "v1", "h1": "h1", "h_ref": "h1", "z1": "h1", "h2": "h2", "hub": "h2", "hub_height": "h2", "z2": "h2",
                "alpha": "alpha", "z0": "z0", "roughness": "z0", "roughness_length": "z0", "d": "d", "obstacle_height": "d"}

_add("energy_wind_shear_power_law",
     name="Wind speed at another height (power law)", topic="wind power / wind resource",
     purpose="The wind speed at height h2 from a measurement at h1 by the power (Hellman) law v2 = v1 (h2/h1)^alpha; alpha = 1/7 when not stated (the one-seventh law).",
     inputs=[_phys("v1", "speed", "measured wind speed"), _phys("h1", "length", "measurement height"), _phys("h2", "length", "target height (e.g. hub height)"),
             _numin("alpha", "shear exponent (default 1/7)", required=False)],
     outputs=[_o("v2", "speed", "wind speed at h2")], primary="v2", conditions=["heights > 0", "0 <= alpha <= 1"],
     method="v2 = v1 (h2/h1)^alpha", latex=r"v_2 = v_1\left(\frac{h_2}{h_1}\right)^{\alpha}",
     run=wind_shear_power_law,
     parse=lambda q: _pairs(q, r"wind shear \(power law\)|power[- ]law wind shear|wind speed at height \(power law\)|hellman", _SHEAR_NAMES, phys={"v1", "h1", "h2"}),
     reference=_ref_hellman,
     wrong=_safe(lambda v1, h1, h2, alpha=1 / 7: _out(v2=v1 * (h1 / h2) ** alpha)),
     random_inputs=lambda rng: (lambda h1: {"v1": round(rng.uniform(2, 15), 2), "h1": h1, "h2": round(h1 * rng.uniform(1.5, 12), 1),
                                            "alpha": round(rng.uniform(0.08, 0.4), 4)})(round(rng.uniform(2, 50), 1)),
     templates=["Wind shear (power law): v1 = 5 m/s, h1 = 10 m, h2 = 80 m", "Wind shear (power law): v1 = 6 m/s, h1 = 10 m, h2 = 100 m, alpha = 0.2"],
     examples=[{"inputs": {"v1": 5, "h1": 10, "h2": 80, "alpha": 1 / 3}, "outputs": {"v2": 10.0}},
               {"inputs": {"v1": 6, "h1": 10, "h2": 10 * 2 ** 7}, "outputs": {"v2": 12.0}}],
     questions=[{"q": "Wind shear (power law): v1 = 6 m/s, h1 = 10 m, h2 = 1280 m", "outputs": {"v2": 12.0}},
                {"q": "Wind shear (power law): v1 = 5 m/s, h1 = 10 m, h2 = 80 m, alpha = 0.3333333333333333", "outputs": {"v2": 10.0}},
                {"q": "Wind shear (power law): v1 = 5 m/s, h1 = 0 m, h2 = 80 m", "outcome": "INVALID_INPUT"}])


def wind_shear_log(v1, h1, h2, z0, d=0.0):
    _pos("the wind speed, the heights and the roughness length", v1, h1, h2, z0)
    _nonneg("the obstacle height", d)
    _need(h1 - 0.7 * d > z0 and h2 - 0.7 * d > z0, "both heights must exceed the displacement height 0.7 d plus the roughness length")
    return _out(v2=v1 * math.log((h2 - 0.7 * d) / z0) / math.log((h1 - 0.7 * d) / z0))


def _ref_logprof(v1, h1, h2, z0, d=0.0):
    from windpowerlib import wind_speed
    return {"v2": float(wind_speed.logarithmic_profile(v1, h1, h2, z0, d))}


_add("energy_wind_shear_log",
     name="Wind speed at another height (logarithmic profile)", topic="wind power / wind resource",
     purpose="The wind speed at height h2 from a measurement at h1 by the logarithmic wind profile with roughness length z0 and an optional obstacle height d (displacement 0.7 d).",
     inputs=[_phys("v1", "speed", "measured wind speed"), _phys("h1", "length", "measurement height"), _phys("h2", "length", "target height"),
             _phys("z0", "length", "surface roughness length"), _phys("d", "length", "obstacle height (default 0; displacement height 0.7 d)", required=False)],
     outputs=[_o("v2", "speed", "wind speed at h2")], primary="v2", conditions=["h - 0.7 d > z0 at both heights"],
     method="v2 = v1 ln((h2 - 0.7 d)/z0)/ln((h1 - 0.7 d)/z0)", latex=r"v_2 = v_1\,\frac{\ln\left((h_2 - 0.7d)/z_0\right)}{\ln\left((h_1 - 0.7d)/z_0\right)}",
     run=wind_shear_log,
     parse=lambda q: _pairs(q, r"wind shear \(log(?:arithmic)?(?: law| profile)?\)|log(?:arithmic)? wind profile|log[- ]law wind shear", _SHEAR_NAMES, phys={"v1", "h1", "h2", "z0", "d"}),
     reference=_ref_logprof,
     wrong=_safe(lambda v1, h1, h2, z0, d=0.0: _out(v2=v1 * math.log((h1 - 0.7 * d) / z0) / math.log((h2 - 0.7 * d) / z0))),
     random_inputs=lambda rng: (lambda h1, z0: {"v1": round(rng.uniform(2, 15), 2), "h1": h1, "h2": round(h1 * rng.uniform(1.5, 12), 1), "z0": z0,
                                                "d": rng.choice([0.0, round(rng.uniform(0, h1), 2)])})(round(rng.uniform(5, 50), 1), round(_lu(rng, 0.0002, 1.0), 5)),
     templates=["Wind shear (log profile): v1 = 5 m/s, h1 = 10 m, h2 = 80 m, z0 = 0.1 m"],
     examples=[{"inputs": {"v1": 5, "h1": 10, "h2": 100, "z0": 1}, "outputs": {"v2": 10.0}},
               {"inputs": {"v1": 4, "h1": 0.7 * 10 + math.e, "h2": 0.7 * 10 + math.e ** 3, "z0": 1, "d": 10}, "outputs": {"v2": 12.0}}],
     questions=[{"q": "Wind shear (log profile): v1 = 5 m/s, h1 = 10 m, h2 = 100 m, z0 = 1 m", "outputs": {"v2": 10.0}},
                {"q": "Wind shear (log profile): v1 = 5 m/s, h1 = 10 m, h2 = 100 m, z0 = 20 m", "outcome": "INVALID_INPUT"}])


def hub_air_density(p, h_p, T, h_T, hub, model="barometric"):
    _pos("the pressure and the temperature", p, T)
    _nonneg("the heights", h_p, h_T, hub)
    T_hub = T - 0.0065 * (hub - h_T)
    _need(T_hub > 150, "the temperature at hub height must stay above 150 K")
    p_hpa = p / 100 - (hub - h_p) * 1 / 8
    _need(p_hpa > 0, "the pressure at hub height must stay positive")
    rho = p_hpa * 1.225 * 288.15 * 100 / (101330 * T_hub) if model == "barometric" else p_hpa * 100 / (287.058 * T_hub)
    return _out(T_hub=T_hub, p_hub=p_hpa * 100, rho=rho)


def _ref_hub_density(p, h_p, T, h_T, hub, model="barometric"):
    from windpowerlib import density, temperature
    th = float(temperature.linear_gradient(T, h_T, hub))
    rho = float((density.barometric if model == "barometric" else density.ideal_gas)(p, h_p, hub, th))
    return {"T_hub": th, "rho": rho, "p_hub": rho * th * (101330 / (1.225 * 288.15) if model == "barometric" else 287.058)}


_add("energy_hub_air_density",
     name="Air density at hub height", topic="wind power / wind resource",
     purpose="Temperature, pressure and air density at a turbine's hub height from a pressure and a temperature measured at other heights: temperature gradient -6.5 K/km, pressure gradient -1/8 hPa per m, then the barometric (1.225 kg/m^3 at 288.15 K, 1013.3 hPa) or the ideal-gas (R = 287.058 J/(kg K)) density.",
     inputs=[_phys("p", "pressure", "measured air pressure"), _phys("h_p", "length", "height of the pressure measurement"), _phys("T", "temperature", "measured air temperature"),
             _phys("h_T", "length", "height of the temperature measurement"), _phys("hub", "length", "hub height"),
             _choice("model", ["barometric", "ideal_gas"], "density model (default barometric)", required=False)],
     outputs=[_o("T_hub", "temperature", "temperature at hub height"), _o("p_hub", "pressure", "pressure at hub height"), _o("rho", "density", "air density at hub height")],
     primary="rho", conditions=["pressure and temperature stay positive at hub height"],
     method="T_hub = T - 0.0065 (h_hub - h_T); p_hub = p - 12.5 Pa/m (h_hub - h_p); rho = p_hub/(R T_hub) (ideal gas) or 1.225 (p_hub/101330)(288.15/T_hub) (barometric)",
     latex=r"\rho_{hub} = 1.225\,\frac{p_{hub}}{101330\ \mathrm{Pa}}\,\frac{288.15\ \mathrm{K}}{T_{hub}}",
     run=hub_air_density,
     parse=lambda q: _pairs(q, r"air density at hub height|hub[- ]height air density", {"p": "p", "pressure": "p", "h_p": "h_p", "T": "T", "temperature": "T", "h_T": "h_T",
                                                                                     "hub": "hub", "hub_height": "hub", "model": "model"},
                            phys={"p", "h_p", "T", "h_T", "hub"}, words={"model"}),
     reference=_ref_hub_density,
     wrong=_safe(lambda p, h_p, T, h_T, hub, model="barometric": (lambda th: _out(T_hub=th, p_hub=p, rho=p / (287.058 * th) if model != "barometric" else p * 1.225 * 288.15 / (101330 * th)))(
         T - 0.0065 * (hub - h_T))),
     random_inputs=lambda rng: (lambda hp: {"p": round(rng.uniform(9e4, 1.03e5), 1), "h_p": hp, "T": round(rng.uniform(263, 308), 2), "h_T": round(rng.uniform(0, 10), 1),
                                            "hub": round(hp + rng.uniform(20, 150), 1), "model": rng.choice(["barometric", "ideal_gas"])})(round(rng.uniform(0, 10), 1)),
     templates=["Air density at hub height: p = 1013 hPa, h_p = 0 m, T = 15 degC, h_T = 2 m, hub = 100 m"],
     examples=[{"inputs": {"p": 101330, "h_p": 0, "T": 288.15, "h_T": 0, "hub": 0}, "outputs": {"rho": 1.225}},
               {"inputs": {"p": 101330, "h_p": 0, "T": 288.15, "h_T": 0, "hub": 0, "model": "ideal_gas"}, "outputs": {"rho": _r(101330 / (287.058 * 288.15))}}],
     questions=[{"q": "Air density at hub height: p = 1013.3 hPa, h_p = 0 m, T = 288.15 K, h_T = 0 m, hub = 0 m", "outputs": {"rho": 1.225}},
                {"q": "Air density at hub height: p = 1013 hPa, h_p = 0 m, T = 15 degC, h_T = 2 m", "outcome": "NEEDS_INPUT"}])


def _check_curve(speeds, powers):
    _need(len(speeds) == len(powers), "give one power for every wind speed of the curve")
    _need(len(speeds) >= 2, "the power curve needs at least two points")
    _need(all(b > a for a, b in zip(speeds, speeds[1:])), "the wind speeds of the curve must increase strictly")
    _need(speeds[0] >= 0 and all(p >= 0 for p in powers), "speeds and powers must not be negative")


def _interp(x, xs, ys):
    """np.interp with left = right = 0."""
    if x < xs[0] or x > xs[-1]:
        return 0.0
    for i in range(len(xs) - 1):
        if xs[i] <= x <= xs[i + 1]:
            if x == xs[i + 1]:
                return float(ys[i + 1])
            return ys[i] + (ys[i + 1] - ys[i]) * (x - xs[i]) / (xs[i + 1] - xs[i])
    return float(ys[-1])


def power_curve_output(speeds, powers, v):
    _check_curve(speeds, powers)
    _nonneg("the wind speed", v)
    p = _interp(v, speeds, powers)
    return _out(P_kW=p, P=1000 * p)


def _ref_power_curve(speeds, powers, v):
    import numpy as np
    from windpowerlib import power_output
    p = float(np.asarray(power_output.power_curve(np.array([v]), np.array(speeds), np.array(powers))).ravel()[0])
    return {"P_kW": p, "P": 1000 * p}


def _rnd_curve(rng):
    vr = round(rng.uniform(10, 14), 1)
    speeds = [round(x, 2) for x in [3.0 + i * (vr - 3) / 8 for i in range(9)]] + [vr + 2, 25.0]
    pr = round(rng.uniform(500, 8000), 0)
    powers = [round(pr * ((s - 3) / (vr - 3)) ** 3, 3) if s < vr else pr for s in speeds]
    return speeds, powers


_CURVE_NAMES = {"speeds": "speeds", "wind_speeds": "speeds", "powers": "powers", "power": "powers", "v": "v", "wind_speed": "v", "rho": "rho", "density": "rho"}

_add("energy_power_curve_output",
     name="Turbine output from its power curve", topic="wind power / turbine output",
     purpose="The electrical output of a wind turbine at a wind speed by linear interpolation of its power curve (zero below the first and above the last listed speed).",
     inputs=[_vecin("speeds", "power-curve wind speeds in m/s (increasing)", 2, 60), _vecin("powers", "power-curve outputs in kW", 2, 60), _phys("v", "speed", "wind speed at hub height")],
     outputs=[_o("P_kW", "number", "output in kW"), _o("P", "power", "output")], primary="P", conditions=["speeds strictly increasing", "one power per speed"],
     method="linear interpolation between the curve points; 0 outside the listed speeds", latex=r"P(v) = P_i + (P_{i+1} - P_i)\frac{v - v_i}{v_{i+1} - v_i}",
     run=power_curve_output,
     parse=lambda q: _pairs(q, r"power curve output|turbine output from (?:its |the )?power curve|power from power curve", _CURVE_NAMES, phys={"v"}, vectors={"speeds", "powers"}),
     reference=_ref_power_curve,
     wrong=_safe(lambda speeds, powers, v: (lambda p: _out(P_kW=p, P=1000 * p))(next((powers[i] for i in range(len(speeds) - 1) if speeds[i] <= v < speeds[i + 1]), 0.0))),
     random_inputs=lambda rng: (lambda s, p: {"speeds": s, "powers": p, "v": round(rng.uniform(s[0] + 0.1, s[-3] - 0.1), 3)})(*_rnd_curve(rng)),
     templates=["Power curve output: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 7 m/s"],
     examples=[{"inputs": {"speeds": "3, 5, 8, 12, 25", "powers": "0, 150, 800, 2000, 2000", "v": 6.5}, "outputs": {"P_kW": 475.0, "P": 475000.0}},
               {"inputs": {"speeds": [3, 5, 8, 12, 25], "powers": [0, 150, 800, 2000, 2000], "v": 26}, "outputs": {"P_kW": 0.0}}],
     questions=[{"q": "Power curve output: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 6.5 m/s", "outputs": {"P_kW": 475.0}},
                {"q": "Power curve output: speeds = [3, 5, 4, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 6.5 m/s", "outcome": "INVALID_INPUT"},
                {"q": "Power curve output: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000]", "outcome": "NEEDS_INPUT"}])


def power_curve_density_corrected(speeds, powers, v, rho):
    _check_curve(speeds, powers)
    _nonneg("the wind speed", v)
    _need(0.5 < rho < 1.6, "the air density must lie between 0.5 and 1.6 kg/m^3")
    corr = []
    for s in speeds:
        e = 1 / 3 if s <= 7.5 else (2 / 3 if s >= 12.5 else 1 / 3 + (s - 7.5) * (2 / 3 - 1 / 3) / (12.5 - 7.5))
        corr.append((1.225 / rho) ** e * s)
    _need(all(b > a for a, b in zip(corr, corr[1:])), "the density-corrected curve is not increasing; the correction does not apply")
    p = _interp(v, corr, powers)
    return _out(P_kW=p, P=1000 * p)


def _ref_power_curve_rho(speeds, powers, v, rho):
    import numpy as np
    from windpowerlib import power_output
    p = float(np.asarray(power_output.power_curve_density_correction(np.array([v]), np.array(speeds), np.array(powers), np.array([rho]))).ravel()[0])
    return {"P_kW": p, "P": 1000 * p}


_add("energy_power_curve_density_corrected",
     name="Turbine output from a density-corrected power curve", topic="wind power / turbine output",
     purpose="Turbine output at a wind speed and air density, with the power curve's speeds rescaled by (1.225/rho)^e, e rising from 1/3 below 7.5 m/s to 2/3 above 12.5 m/s (Svenningsen's correction, as in windpowerlib).",
     inputs=[_vecin("speeds", "power-curve wind speeds in m/s at 1.225 kg/m^3 (increasing)", 2, 60), _vecin("powers", "power-curve outputs in kW", 2, 60),
             _phys("v", "speed", "wind speed at hub height"), _phys("rho", "density", "air density at hub height")],
     outputs=[_o("P_kW", "number", "output in kW"), _o("P", "power", "output")], primary="P", conditions=["0.5 < rho < 1.6 kg/m^3"],
     method="v_i' = v_i (1.225/rho)^e(v_i); interpolate P(v) on (v_i', P_i)", latex=r"v_i' = v_i\left(\frac{1.225}{\rho}\right)^{e(v_i)},\ e\in[\tfrac13,\tfrac23]",
     run=power_curve_density_corrected,
     parse=lambda q: _pairs(q, r"density[- ]corrected power curve(?: output)?|power curve output with density correction", _CURVE_NAMES, phys={"v", "rho"}, vectors={"speeds", "powers"}),
     reference=_ref_power_curve_rho,
     wrong=_safe(lambda speeds, powers, v, rho: power_curve_output(speeds, [p * rho / 1.225 for p in powers], v)),
     random_inputs=lambda rng: (lambda s, p: {"speeds": s, "powers": p, "v": round(rng.uniform(s[0] + 0.5, s[-3] - 0.1), 3), "rho": round(rng.uniform(0.9, 1.35), 4)})(*_rnd_curve(rng)),
     templates=["Density-corrected power curve: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 7 m/s, rho = 1.1 kg/m^3"],
     examples=[{"inputs": {"speeds": [3, 5, 8, 12, 25], "powers": [0, 150, 800, 2000, 2000], "v": 6.5, "rho": 1.225}, "outputs": {"P_kW": 475.0}}],
     questions=[{"q": "Density-corrected power curve: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 6.5 m/s, rho = 1.225 kg/m^3", "outputs": {"P_kW": 475.0}},
                {"q": "Density-corrected power curve: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 6.5 m/s, rho = 3 kg/m^3", "outcome": "INVALID_INPUT"}])


def _check_weibull(k, c):
    _need(isinstance(k, (int, float)) and 1.0 < k <= 5.0, "the Weibull shape k must lie in (1, 5]")
    _pos("the Weibull scale c", c)


def weibull_wind(k, c, rho):
    _check_weibull(k, c)
    _pos("the air density", rho)
    g1, g3 = math.gamma(1 + 1 / k), math.gamma(1 + 3 / k)
    return _out(v_mean=c * g1, power_density=0.5 * rho * c ** 3 * g3, energy_pattern_factor=g3 / g1 ** 3,
                v_most_probable=c * ((k - 1) / k) ** (1 / k), v_max_energy=c * ((k + 2) / k) ** (1 / k))


def _ref_weibull(k, c, rho):
    """scipy.stats.weibull_min moments and quadrature; the two characteristic speeds as roots (bisection) of the
    symbolic derivative (sympy) of the density and of v^3 times the density."""
    import sympy as sp
    from scipy import integrate, stats
    w = stats.weibull_min(k, scale=c)
    mean = float(w.mean())
    m3, _ = integrate.quad(lambda x: x ** 3 * w.pdf(x), 0, math.inf, epsabs=0, epsrel=1e-13, limit=200)
    x = sp.Symbol("x", positive=True)
    K, Cc = sp.Float(k), sp.Float(c)
    logpdf = sp.log(K / Cc) + (K - 1) * sp.log(x / Cc) - (x / Cc) ** K
    roots = []
    for m in (0, 3):
        f = sp.lambdify(x, sp.diff(logpdf + m * sp.log(x), x), "math")
        roots.append(_bisect(f, c * 1e-6, 20 * c))
    return {"v_mean": mean, "power_density": 0.5 * rho * m3, "energy_pattern_factor": m3 / mean ** 3, "v_most_probable": roots[0], "v_max_energy": roots[1]}


_add("energy_weibull_wind",
     name="Weibull wind statistics and mean power density", topic="wind power / wind resource",
     purpose="Mean wind speed, mean wind power density, energy pattern factor, most probable speed and the speed carrying most energy for a Weibull wind-speed distribution with shape k and scale c.",
     inputs=[_numin("k", "Weibull shape factor (1 < k <= 5)"), _phys("c", "speed", "Weibull scale factor"), _phys("rho", "density", "air density")],
     outputs=[_o("v_mean", "speed", "mean wind speed c Gamma(1 + 1/k)"), _o("power_density", "number", "mean wind power density in W/m^2"),
              _o("energy_pattern_factor", "number", "mean of v^3 over the cube of the mean"), _o("v_most_probable", "speed", "most probable wind speed"),
              _o("v_max_energy", "speed", "wind speed carrying the most energy")],
     primary="power_density", conditions=["1 < k <= 5", "c > 0"],
     method="v_mean = c Gamma(1 + 1/k); P/A = (1/2) rho c^3 Gamma(1 + 3/k); v_mp = c ((k - 1)/k)^(1/k); v_maxE = c ((k + 2)/k)^(1/k)",
     latex=r"\bar v = c\,\Gamma\!\left(1 + \tfrac1k\right),\quad \frac{\bar P}{A} = \tfrac12\rho c^3\,\Gamma\!\left(1 + \tfrac3k\right)",
     run=weibull_wind,
     parse=lambda q: _pairs(q, r"weibull wind(?: statistics)?|weibull wind power density|mean wind power density", {"k": "k", "shape": "k", "c": "c", "scale": "c", "rho": "rho", "density": "rho"},
                            phys={"c", "rho"}),
     reference=_ref_weibull,
     wrong=_safe(lambda k, c, rho: (lambda r: dict(r, v_mean=_r(c * math.gamma(1 / k)), power_density=_r(0.5 * rho * (c * math.gamma(1 + 1 / k)) ** 3)))(weibull_wind(k, c, rho))),
     random_inputs=lambda rng: {"k": round(rng.uniform(1.3, 4.0), 3), "c": round(rng.uniform(3, 12), 3), "rho": round(rng.uniform(1.0, 1.3), 4)},
     templates=["Weibull wind statistics: k = 2, c = 8 m/s, rho = 1.225 kg/m^3"],
     examples=[{"inputs": {"k": 2, "c": 8, "rho": 1.225}, "outputs": {"v_mean": _r(4 * math.sqrt(math.pi)), "v_most_probable": _r(8 / math.sqrt(2)),
                                                                     "power_density": _r(0.5 * 1.225 * 512 * 0.75 * math.sqrt(math.pi)), "energy_pattern_factor": _r(6 / math.pi)}}],
     questions=[{"q": "Weibull wind statistics: k = 2, c = 8 m/s, rho = 1.225 kg/m^3", "outputs": {"v_mean": _r(4 * math.sqrt(math.pi))}},
                {"q": "Weibull wind statistics: k = 0.5, c = 8 m/s, rho = 1.225 kg/m^3", "outcome": "INVALID_INPUT"}])


def weibull_aep(speeds, powers, k, c, hours=8760.0):
    """Mean output of a power curve (linear between points) over a Weibull distribution: each segment's integral in
    closed form with the regularized incomplete gamma function (mpmath)."""
    import mpmath
    _check_curve(speeds, powers)
    _check_weibull(k, c)
    _pos("the hours per year", hours)
    F = lambda v: -math.expm1(-((v / c) ** k))           # noqa: E731
    total = mpmath.mpf(0)
    s = 1 + 1 / k
    for i in range(len(speeds) - 1):
        a, b = speeds[i], speeds[i + 1]
        slope = (powers[i + 1] - powers[i]) / (b - a)
        base = powers[i] - slope * a
        pa = F(b) - F(a)
        mv = c * math.gamma(s) * mpmath.gammainc(s, (a / c) ** k, (b / c) ** k, regularized=True)
        total += base * pa + slope * mv
    pm = float(total)
    pr = max(powers)
    return _out(mean_power_kW=pm, AEP_MWh=pm * hours / 1000.0, capacity_factor=pm / pr if pr > 0 else 0.0)


def _ref_aep(speeds, powers, k, c, hours=8760.0):
    import numpy as np
    from scipy import integrate, stats
    w = stats.weibull_min(k, scale=c)
    tot = 0.0
    for i in range(len(speeds) - 1):
        val, _ = integrate.quad(lambda x: float(np.interp(x, speeds, powers, left=0, right=0)) * w.pdf(x), speeds[i], speeds[i + 1],
                                epsabs=0, epsrel=1e-13, limit=200)
        tot += val
    return {"mean_power_kW": tot, "AEP_MWh": tot * hours / 1000, "capacity_factor": tot / max(powers)}


_add("energy_weibull_aep",
     name="Annual energy production from a power curve and a Weibull distribution", topic="wind power / annual energy",
     purpose="Mean output, annual energy production and capacity factor of a wind turbine whose power curve (linear between points, zero outside) operates in winds with a Weibull distribution (shape k, scale c).",
     inputs=[_vecin("speeds", "power-curve wind speeds in m/s (increasing)", 2, 60), _vecin("powers", "power-curve outputs in kW", 2, 60),
             _numin("k", "Weibull shape factor (1 < k <= 5)"), _phys("c", "speed", "Weibull scale factor"),
             _numin("hours", "hours in the period (default 8760)", required=False)],
     outputs=[_o("mean_power_kW", "number", "mean output in kW"), _o("AEP_MWh", "number", "energy over the period in MWh"),
              _o("capacity_factor", "number", "mean output over the largest output of the curve")],
     primary="AEP_MWh", conditions=["1 < k <= 5", "speeds strictly increasing"],
     method="P_mean = sum over segments of integral (P_i + s (v - v_i)) f(v) dv, each in closed form: F(b) - F(a) and c Gamma(1 + 1/k) [P(1 + 1/k, (b/c)^k) - P(1 + 1/k, (a/c)^k)]",
     latex=r"\bar P = \int_0^\infty P(v)\,\frac{k}{c}\left(\frac vc\right)^{k-1}e^{-(v/c)^k}\,dv,\quad AEP = 8760\,\mathrm{h}\cdot\bar P",
     run=weibull_aep,
     parse=lambda q: _pairs(q, r"annual energy production|aep from (?:a )?power curve|weibull aep", dict(_CURVE_NAMES, k="k", shape="k", c="c", scale="c", hours="hours"),
                            phys={"c"}, vectors={"speeds", "powers"}),
     reference=_ref_aep,
     wrong=_safe(lambda speeds, powers, k, c, hours=8760.0: weibull_aep(speeds, powers, 2.0, c, hours)),
     random_inputs=lambda rng: (lambda s, p: {"speeds": s, "powers": p, "k": rng.choice([round(rng.uniform(1.3, 1.9), 3), round(rng.uniform(2.1, 3.5), 3)]),
                                              "c": round(rng.uniform(5, 11), 3)})(*_rnd_curve(rng)),
     templates=["Annual energy production: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], k = 2, c = 7 m/s"],
     examples=[{"inputs": {"speeds": [0, 100], "powers": [1000, 1000], "k": 2, "c": 7}, "outputs": {"mean_power_kW": _r(1000 * (1 - math.exp(-(100 / 7) ** 2)))}},
               {"inputs": {"speeds": [0, 1000], "powers": [0, 1000], "k": 2, "c": 7}, "outputs": {"mean_power_kW": _r(7 * math.sqrt(math.pi) / 2)}}],
     questions=[{"q": "Annual energy production: speeds = [0, 1000], powers = [0, 1000], k = 2, c = 7 m/s", "outputs": {"mean_power_kW": _r(7 * math.sqrt(math.pi) / 2),
                                                                                                                           "AEP_MWh": _r(7 * math.sqrt(math.pi) / 2 * 8.76)}},
                {"q": "Annual energy production: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], k = 7, c = 7 m/s", "outcome": "INVALID_INPUT"}])


# ================================================================================================ THREE-PHASE POWER
SQ3 = math.sqrt(3.0)


def _pf_angle(pf, pf_type):
    _need(isinstance(pf, (int, float)) and 0 < pf <= 1, "the power factor must lie in (0, 1]")
    phi = math.acos(pf)
    return phi if pf_type != "leading" else -phi


def three_phase_power(VL, IL, pf, connection, pf_type="lagging"):
    _pos("the line voltage and line current", VL, IL)
    phi = _pf_angle(pf, pf_type)
    S = SQ3 * VL * IL
    vph, iph = (VL / SQ3, IL) if connection == "star" else (VL, IL / SQ3)
    Q = S * math.sin(phi)
    return _out(P=S * pf, Q=0.0 if pf == 1 else Q, S=S, V_phase=vph, I_phase=iph)


def _ref_three_phase(VL, IL, pf, connection, pf_type="lagging"):
    """Instantaneous three-phase powers averaged over one cycle by scipy quadrature: p = va ia + vb ib + vc ic and the
    instantaneous reactive power q = [(vb - vc) ia + (vc - va) ib + (va - vb) ic]/sqrt(3); the phase quantities from the
    geometry of the phasors (numpy)."""
    import numpy as np
    from scipy import integrate
    phi = math.acos(pf) * (1 if pf_type != "leading" else -1)
    rot = np.exp(-2j * np.pi / 3)
    k = 1.0 / abs(1 - rot)                          # phase-to-line ratio from the phasor geometry
    Vp = VL * k                                     # star-equivalent phase voltage (rms)
    w = 2 * np.pi
    va = lambda t, s: np.sqrt(2) * Vp * np.cos(w * t - s * 2 * np.pi / 3)        # noqa: E731
    ia = lambda t, s: np.sqrt(2) * IL * np.cos(w * t - s * 2 * np.pi / 3 - phi)  # noqa: E731
    p = lambda t: sum(va(t, s) * ia(t, s) for s in range(3))                     # noqa: E731
    q = lambda t: ((va(t, 1) - va(t, 2)) * ia(t, 0) + (va(t, 2) - va(t, 0)) * ia(t, 1) + (va(t, 0) - va(t, 1)) * ia(t, 2)) / np.sqrt(3)  # noqa: E731
    P, _ = integrate.quad(p, 0, 1, epsabs=0, epsrel=1e-13, limit=200)
    Q, _ = integrate.quad(q, 0, 1, epsabs=0, epsrel=1e-13, limit=200)
    out = {"P": P, "Q": Q if abs(Q) > 1e-12 * abs(P) else 0.0, "S": math.hypot(P, Q)}
    out.update({"V_phase": Vp, "I_phase": IL} if connection == "star" else {"V_phase": VL, "I_phase": IL * k})
    return out


_add("energy_three_phase_power",
     name="Balanced three-phase power from line quantities", topic="three-phase power / balanced systems",
     purpose="Real, reactive and apparent power of a balanced three-phase load from the line voltage, line current and power factor, with the phase voltage and phase current of a star or delta connection.",
     inputs=[_phys("VL", "voltage", "line-to-line rms voltage"), _phys("IL", "current", "line current (rms)"), _numin("pf", "power factor (0 < pf <= 1)"),
             _choice("connection", ["star", "delta"], "connection of the load (star = wye)"),
             _choice("pf_type", ["lagging", "leading"], "lagging (inductive, default) or leading (capacitive)", required=False)],
     outputs=[_o("P", "power", "real power (three-phase total)"), _o("Q", "power", "reactive power in var (positive lagging)"), _o("S", "power", "apparent power in VA"),
              _o("V_phase", "voltage", "voltage across each phase of the load"), _o("I_phase", "current", "current in each phase of the load")],
     primary="P", conditions=["balanced load", "0 < pf <= 1"],
     method="S = sqrt(3) V_L I_L, P = S pf, Q = S sin(acos pf); star: V_ph = V_L/sqrt(3), I_ph = I_L; delta: V_ph = V_L, I_ph = I_L/sqrt(3)",
     latex=r"P = \sqrt3\,V_L I_L\cos\phi,\quad Q = \sqrt3\,V_L I_L\sin\phi",
     run=three_phase_power,
     parse=lambda q: _pairs(q, r"three[- ]phase power|3[- ]phase power", {"VL": "VL", "V": "VL", "V_line": "VL", "IL": "IL", "I": "IL", "I_line": "IL", "pf": "pf",
                                                                       "connection": "connection", "pf_type": "pf_type"},
                            phys={"VL", "IL"}, words={"connection", "pf_type"}, num_units=_pct_units("pf")),
     reference=_ref_three_phase,
     wrong=_safe(lambda VL, IL, pf, connection, pf_type="lagging": (lambda r: dict(r, P=_r(VL * IL * pf), S=_r(VL * IL)))(three_phase_power(VL, IL, pf, connection, pf_type))),
     random_inputs=lambda rng: {"VL": rng.choice([208.0, 400.0, 415.0, 480.0, 690.0, 3300.0, 11000.0, 33000.0]), "IL": round(_lu(rng, 1, 2000), 3),
                                "pf": round(rng.uniform(0.5, 1.0), 4), "connection": rng.choice(["star", "delta"]), "pf_type": rng.choice(["lagging", "leading"])},
     templates=["Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star", "Three-phase power: VL = 11 kV, IL = 100 A, pf = 0.9, connection = delta, pf_type = leading"],
     examples=[{"inputs": {"VL": 400, "IL": 50, "pf": 0.8, "connection": "star"}, "outputs": {"P": _r(SQ3 * 400 * 50 * 0.8), "Q": _r(SQ3 * 400 * 50 * 0.6), "V_phase": _r(400 / SQ3)}},
               {"inputs": {"VL": 400, "IL": SQ3 * 10, "pf": 1.0, "connection": "delta"}, "outputs": {"P": 12000.0, "Q": 0.0, "I_phase": 10.0}}],
     questions=[{"q": "Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star", "outputs": {"P": _r(SQ3 * 400 * 50 * 0.8)}},
                {"q": "Three-phase power: VL = 400 V, IL = 50 A, pf = 1.2, connection = star", "outcome": "INVALID_INPUT"},
                {"q": "Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8", "outcome": "NEEDS_INPUT"}])


def per_unit_change_of_base(R_pu, X_pu, S_old, V_old, S_new, V_new):
    _nonneg("the per-unit resistance", R_pu)
    _pos("the per-unit reactance and the bases", X_pu, S_old, V_old, S_new, V_new)
    f = (V_old / V_new) ** 2 * (S_new / S_old)
    zb = V_new ** 2 / S_new
    return _out(R_pu_new=R_pu * f, X_pu_new=X_pu * f, Z_pu_new=math.hypot(R_pu, X_pu) * f)


def _ref_change_of_base(R_pu, X_pu, S_old, V_old, S_new, V_new):
    """pandapower's own conversion of a transformer's nameplate impedance (vk, vkr on its rating sn at its rated
    voltage) to the per-unit system of the network (net.sn_mva, bus voltage), read from its internal branch table."""
    import pandapower as pp
    net = _pp_empty(sn_mva=S_new / 1e6)
    hv = pp.create_bus(net, vn_kv=10 * V_new / 1e3)
    lv = pp.create_bus(net, vn_kv=V_new / 1e3)
    pp.create_ext_grid(net, hv)
    pp.create_transformer_from_parameters(net, hv, lv, sn_mva=S_old / 1e6, vn_hv_kv=10 * V_new / 1e3, vn_lv_kv=V_old / 1e3,
                                          vkr_percent=100 * R_pu, vk_percent=100 * math.hypot(R_pu, X_pu), pfe_kw=0.0, i0_percent=0.0)
    pp.runpp(net, numba=False, check_connectivity=False)
    br = net._ppc["branch"]
    r, x = float(br[0, 2].real), float(br[0, 3].real)
    zb = (V_new / 1e3) ** 2 / (S_new / 1e6)
    return {"R_pu_new": r, "X_pu_new": x, "Z_pu_new": math.hypot(r, x)}


_add("energy_per_unit_change_of_base",
     name="Per-unit impedance on a new base", topic="power systems / per-unit system",
     purpose="An impedance given in per unit on equipment ratings (S_old, V_old) converted to the per-unit system of the network (S_new, V_new).",
     inputs=[_numin("R_pu", "per-unit resistance on the old base"), _numin("X_pu", "per-unit reactance on the old base"),
             _phys("S_old", "power", "old base (equipment rating) in VA"), _phys("V_old", "voltage", "old base voltage (line-to-line)"),
             _phys("S_new", "power", "new (system) base power in VA"), _phys("V_new", "voltage", "new base voltage (line-to-line)")],
     outputs=[_o("R_pu_new", "number", "per-unit resistance on the new base"), _o("X_pu_new", "number", "per-unit reactance on the new base"),
              _o("Z_pu_new", "number", "per-unit impedance magnitude on the new base")],
     primary="X_pu_new", conditions=["X_pu > 0", "positive bases"],
     method="Z_new = Z_old (V_old/V_new)^2 (S_new/S_old)", latex=r"Z_{pu,new} = Z_{pu,old}\left(\frac{V_{old}}{V_{new}}\right)^2\frac{S_{new}}{S_{old}}",
     run=per_unit_change_of_base,
     parse=lambda q: _pairs(q, r"per[- ]unit change of base|change of base|per[- ]unit impedance on a new base", {"R_pu": "R_pu", "X_pu": "X_pu", "S_old": "S_old", "V_old": "V_old",
                                                                                                         "S_new": "S_new", "V_new": "V_new"},
                            phys={"S_old", "V_old", "S_new", "V_new"}),
     reference=_ref_change_of_base,
     wrong=_safe(lambda R_pu, X_pu, S_old, V_old, S_new, V_new: (lambda f: _out(R_pu_new=R_pu * f, X_pu_new=X_pu * f, Z_pu_new=math.hypot(R_pu, X_pu) * f))(
         (V_new / V_old) ** 2 * (S_new / S_old))),
     random_inputs=lambda rng: (lambda V: {"R_pu": round(rng.uniform(0.002, 0.02), 5), "X_pu": round(rng.uniform(0.04, 0.2), 5), "S_old": round(_lu(rng, 1e5, 5e8), -3),
                                           "V_old": round(V * rng.uniform(0.9, 1.1), 0), "S_new": rng.choice([1e6, 1e7, 1e8]), "V_new": V})(rng.choice([400.0, 11000.0, 13800.0, 33000.0, 132000.0])),
     templates=["Per-unit change of base: R_pu = 0.01, X_pu = 0.1, S_old = 50 MVA, V_old = 132 kV, S_new = 100 MVA, V_new = 138 kV"],
     examples=[{"inputs": {"R_pu": 0.0, "X_pu": 0.1, "S_old": 5e7, "V_old": 1.32e5, "S_new": 1e8, "V_new": 1.38e5}, "outputs": {"X_pu_new": _r(0.1 * (132 / 138) ** 2 * 2)}},
               {"inputs": {"R_pu": 0.01, "X_pu": 0.1, "S_old": 1e7, "V_old": 1.1e4, "S_new": 1e8, "V_new": 1.1e4}, "outputs": {"X_pu_new": 1.0, "R_pu_new": 0.1}}],
     questions=[{"q": "Per-unit change of base: R_pu = 0.01, X_pu = 0.1, S_old = 10 MVA, V_old = 11 kV, S_new = 100 MVA, V_new = 11 kV", "outputs": {"X_pu_new": 1.0, "R_pu_new": 0.1}},
                {"q": "Per-unit change of base: R_pu = 0.01, X_pu = 0.1, S_old = 10 MVA, V_old = 11 kV, S_new = 100 MVA", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ LINES AND LOAD FLOW
def _load_current(Vr, P, pf, pf_type):
    phi = _pf_angle(pf, pf_type)
    I = P / (SQ3 * Vr * pf)
    return phi, I, cmath.rect(I, -phi)


def short_line(Vr, P, pf, R, X, pf_type="lagging"):
    _pos("the receiving-end voltage and the load power", Vr, P)
    _nonneg("the line resistance and reactance", R, X)
    phi, I, Ic = _load_current(Vr, P, pf, pf_type)
    vr = Vr / SQ3
    vs = vr + Ic * complex(R, X)
    Vs = SQ3 * abs(vs)
    Ps = P + 3 * I * I * R
    return _out(Vs=Vs, delta_deg=math.degrees(cmath.phase(vs)), I=I, losses=3 * I * I * R, Ps=Ps, regulation_pct=100 * (Vs - Vr) / Vr, efficiency_pct=100 * P / Ps)


def _ref_short_line(Vr, P, pf, R, X, pf_type="lagging"):
    """Energy (complex power) balance instead of the phasor sum: the sending-end power is the load plus the line's
    I^2 Z, and the sending voltage follows from |S_s| = sqrt(3) |V_s| I."""
    phi = math.acos(pf) * (1 if pf_type != "leading" else -1)
    S_r = complex(P, P * math.tan(phi))
    I = abs(S_r) / (SQ3 * Vr)
    S_s = S_r + 3 * I * I * complex(R, X)
    Vs = abs(S_s) / (SQ3 * I)
    return {"Vs": Vs, "delta_deg": math.degrees(cmath.phase(S_s)) - math.degrees(phi), "I": I, "losses": S_s.real - P, "Ps": S_s.real,
            "regulation_pct": 100 * (Vs - Vr) / Vr, "efficiency_pct": 100 * P / S_s.real}


_LINE_NAMES = {"Vr": "Vr", "V_r": "Vr", "receiving_voltage": "Vr", "P": "P", "load": "P", "pf": "pf", "pf_type": "pf_type", "R": "R", "X": "X", "B": "B", "Y": "B"}
_LINE_OUT = [_o("Vs", "voltage", "sending-end line voltage"), _o("delta_deg", "number", "angle of the sending voltage ahead of the receiving voltage, degrees"),
             _o("I", "current", "receiving-end line current"), _o("losses", "power", "line losses (three-phase)"), _o("Ps", "power", "sending-end real power"),
             _o("regulation_pct", "number", "voltage regulation in percent"), _o("efficiency_pct", "number", "transmission efficiency in percent")]
_LINE_IN = [_phys("Vr", "voltage", "receiving-end line-to-line voltage"), _phys("P", "power", "load real power (three-phase)"), _numin("pf", "load power factor"),
            _phys("R", "resistance", "line resistance per phase (total)"), _phys("X", "resistance", "line reactance per phase (total)")]


def _rnd_line(rng, with_B=False, long=False):
    V = rng.choice([11e3, 33e3, 66e3, 132e3, 220e3, 400e3])
    L = rng.uniform(20, 400 if long else 150)
    x = {"Vr": V, "P": round(V * V / 400 * rng.uniform(0.05, 1.0) / (10 if V < 1e5 else 1), -3) + 1000, "pf": round(rng.uniform(0.7, 1.0), 3),
         "R": round(0.05 * L, 3), "X": round(0.4 * L, 3), "pf_type": rng.choice(["lagging", "leading"])}
    if with_B:
        x["B"] = round(2.8e-6 * L, 10)
    return x


_add("energy_short_line",
     name="Short transmission line: sending-end voltage, regulation and efficiency", topic="power systems / transmission lines",
     purpose="Sending-end voltage and its angle, line current, losses, voltage regulation and efficiency of a short three-phase line (series R + jX, no shunt admittance) feeding a load at a known receiving-end voltage.",
     inputs=list(_LINE_IN) + [_choice("pf_type", ["lagging", "leading"], "lagging (default) or leading load", required=False)],
     outputs=_LINE_OUT, primary="Vs", conditions=["short line: shunt admittance neglected", "balanced three-phase load"],
     method="V_s = V_r + I (R + jX) per phase, I = P/(sqrt(3) V_r pf) at angle -acos(pf); regulation = (V_s - V_r)/V_r; efficiency = P/(P + 3 I^2 R)",
     latex=r"\mathbf V_s = \mathbf V_r + \mathbf I\,(R + jX),\quad \text{VR} = \frac{|V_s| - |V_r|}{|V_r|}",
     run=short_line,
     parse=lambda q: _pairs(q, r"short (?:transmission )?line", _LINE_NAMES, phys={"Vr", "P", "R", "X"}, words={"pf_type"}, num_units=_pct_units("pf")),
     reference=_ref_short_line,
     wrong=_safe(lambda Vr, P, pf, R, X, pf_type="lagging": (lambda r: dict(r, Vs=_r(Vr + SQ3 * r["I"] * (R * pf + X * math.sin(math.acos(pf))) * (1 if pf_type != "leading" else 1)),
                                                                              regulation_pct=_r(100 * SQ3 * r["I"] * (R * pf + X * math.sin(math.acos(pf))) / Vr)))(short_line(Vr, P, pf, R, X, pf_type))),
     random_inputs=lambda rng: _rnd_line(rng),
     templates=["Short line: Vr = 33 kV, P = 10 MW, pf = 0.8, R = 5 ohm, X = 20 ohm"],
     examples=[{"inputs": {"Vr": 33e3, "P": 10e6, "pf": 1.0, "R": 0.0, "X": 0.0}, "outputs": {"Vs": 33000.0, "regulation_pct": 0.0, "efficiency_pct": 100.0}},
               {"inputs": {"Vr": SQ3 * 1000, "P": 3 * 1000 * 100, "pf": 1.0, "R": 1.0, "X": 0.0}, "outputs": {"Vs": _r(SQ3 * 1100), "losses": 30000.0}}],
     questions=[{"q": "Short line: Vr = 33 kV, P = 10 MW, pf = 1, R = 0 ohm, X = 0 ohm", "outputs": {"Vs": 33000.0}},
                {"q": "Short line: Vr = 33 kV, P = 10 MW, pf = 0.8, R = 5 ohm, X = 20 ohm, pf_type = sideways", "outcome": "INVALID_INPUT"},
                {"q": "Short line: Vr = 33 kV, P = 10 MW, pf = 0.8, R = 5 ohm", "outcome": "NEEDS_INPUT"}])


def _habs(z):
    """|z| by math.hypot (Python's own implementation, the same on every platform); abs() of a complex uses the C
    library's hypot, whose last bit differs between Linux and Windows and is amplified ~1e7 by a line's regulation."""
    return math.hypot(z.real, z.imag)


def _line_ends(A, B, C, D, Vr, P, pf, pf_type, R):
    phi, I, Ic = _load_current(Vr, P, pf, pf_type)
    vr = Vr / SQ3
    vs = A * vr + B * Ic
    is_ = C * vr + D * Ic
    Ss = 3 * vs * is_.conjugate()
    Vs = SQ3 * _habs(vs)
    # a line with no resistance (the shunt is a pure susceptance) has exactly zero losses; Ps - P is then only rounding
    # noise, which differed between Linux and Windows (2026-10-23)
    Ps = P if R == 0 else Ss.real
    return _out(Vs=Vs, delta_deg=math.degrees(cmath.phase(vs)), Is=_habs(is_), Ps=Ps, losses=Ps - P,
                regulation_pct=100 * (Vs / _habs(A) - Vr) / Vr, efficiency_pct=100 * P / Ps, A_mag=_habs(A), A_angle_deg=math.degrees(cmath.phase(A)),
                B_mag=_habs(B), B_angle_deg=math.degrees(cmath.phase(B)))


def medium_line(Vr, P, pf, R, X, B, pf_type="lagging"):
    _pos("the receiving-end voltage and the load power", Vr, P)
    _nonneg("the line resistance and reactance", R, X)
    _pos("the shunt susceptance", B)
    _need(R + X > 0, "the series impedance must not be zero")
    Z, Y = complex(R, X), complex(0.0, B)
    A = 1 + Z * Y / 2
    return _line_ends(A, Z, Y * (1 + Z * Y / 4), A, Vr, P, pf, pf_type, R)


def _ref_medium_line(Vr, P, pf, R, X, B, pf_type="lagging"):
    """The nominal-pi circuit as a two-node admittance matrix [I_s, -I_r] = Y_bus [V_s, V_r], solved for V_s and I_s
    with numpy (no ABCD constants)."""
    import numpy as np
    Z, Y = complex(R, X), complex(0.0, B)
    phi = math.acos(pf) * (1 if pf_type != "leading" else -1)
    I = P / (SQ3 * Vr * pf)
    Ir = cmath.rect(I, -phi)
    vr = Vr / SQ3
    Yb = np.array([[1 / Z + Y / 2, -1 / Z], [-1 / Z, 1 / Z + Y / 2]], dtype=complex)
    # unknowns: V_s, I_s ; equations: row 1: Yb00 Vs - Is = -Yb01 vr ; row 2: Yb10 Vs = -Ir - Yb11 vr
    M = np.array([[Yb[0, 0], -1], [Yb[1, 0], 0]], dtype=complex)
    rhs = np.array([-Yb[0, 1] * vr, -Ir - Yb[1, 1] * vr], dtype=complex)
    vs, is_ = np.linalg.solve(M, rhs)
    vs_nl = np.linalg.solve(np.array([[Yb[1, 0]]]), np.array([-Yb[1, 1] * 1.0]))[0]   # no load: I_r = 0, V_r = 1
    Bsc = np.linalg.solve(np.array([[Yb[1, 0]]]), np.array([-1.0]))[0]                 # receiving end shorted: V_r = 0, I_r = 1
    Ss = 3 * vs * np.conj(is_)
    A = vs_nl
    Vs = SQ3 * abs(vs)
    return {"Vs": Vs, "delta_deg": math.degrees(cmath.phase(complex(vs))), "Is": abs(is_), "Ps": float(Ss.real), "losses": float(Ss.real) - P,
            "regulation_pct": 100 * (Vs / abs(A) - Vr) / Vr, "efficiency_pct": 100 * P / float(Ss.real), "A_mag": abs(A), "A_angle_deg": math.degrees(cmath.phase(complex(A))),
            "B_mag": abs(Bsc), "B_angle_deg": math.degrees(cmath.phase(complex(Bsc)))}


_LINE2_OUT = [_o("Vs", "voltage", "sending-end line voltage"), _o("delta_deg", "number", "angle of the sending voltage ahead of the receiving voltage, degrees"),
              _o("Is", "current", "sending-end current"), _o("Ps", "power", "sending-end real power"), _o("losses", "power", "line losses (three-phase)"),
              _o("regulation_pct", "number", "voltage regulation (V_s/|A| - V_r)/V_r in percent"), _o("efficiency_pct", "number", "transmission efficiency in percent"),
              _o("A_mag", "number", "|A| (= |D|)"), _o("A_angle_deg", "number", "angle of A in degrees"), _o("B_mag", "resistance", "|B| in ohms"),
              _o("B_angle_deg", "number", "angle of B in degrees")]
_LINE2_IN = list(_LINE_IN) + [_phys("B", "susceptance", "total shunt susceptance of the line per phase"),
                              _choice("pf_type", ["lagging", "leading"], "lagging (default) or leading load", required=False)]

_add("energy_medium_line",
     name="Medium transmission line (nominal pi)", topic="power systems / transmission lines",
     purpose="ABCD constants of the nominal-pi model and the sending-end voltage, current and power, losses, regulation and efficiency of a medium-length three-phase line feeding a load.",
     inputs=_LINE2_IN, outputs=_LINE2_OUT, primary="Vs", conditions=["nominal-pi model (half the shunt admittance at each end)", "balanced load"],
     method="A = D = 1 + ZY/2, B = Z, C = Y(1 + ZY/4); V_s = A V_r + B I_r, I_s = C V_r + D I_r; regulation = (V_s/|A| - V_r)/V_r",
     latex=r"\begin{bmatrix}V_s\\I_s\end{bmatrix} = \begin{bmatrix}1+\frac{ZY}{2} & Z\\ Y\left(1+\frac{ZY}{4}\right) & 1+\frac{ZY}{2}\end{bmatrix}\begin{bmatrix}V_r\\I_r\end{bmatrix}",
     run=medium_line,
     parse=lambda q: _pairs(q, r"medium (?:transmission )?line(?: \(nominal pi\))?|nominal pi line", _LINE_NAMES, phys={"Vr", "P", "R", "X", "B"}, words={"pf_type"}, num_units=_pct_units("pf")),
     reference=_ref_medium_line,
     wrong=_safe(lambda Vr, P, pf, R, X, B, pf_type="lagging": (lambda Z, Y: _line_ends(1 + Z * Y, Z, Y, 1 + Z * Y, Vr, P, pf, pf_type, R))(complex(R, X), complex(0, B))),
     random_inputs=lambda rng: _rnd_line(rng, True),
     templates=["Medium line: Vr = 132 kV, P = 50 MW, pf = 0.8, R = 10 ohm, X = 40 ohm, B = 0.0003 S"],
     examples=[{"inputs": {"Vr": 132e3, "P": 50e6, "pf": 1.0, "R": 0.0, "X": 40.0, "B": 0.0005}, "outputs": {"A_mag": 0.99, "B_mag": 40.0}}],
     questions=[{"q": "Medium line: Vr = 132 kV, P = 50 MW, pf = 1, R = 0 ohm, X = 40 ohm, B = 500 uS", "outputs": {"A_mag": 0.99}},
                {"q": "Medium line: Vr = 132 kV, P = 50 MW, pf = 0.8, R = 10 ohm, X = 40 ohm", "outcome": "NEEDS_INPUT"}])


def long_line(Vr, P, pf, R, X, B, pf_type="lagging"):
    _pos("the receiving-end voltage and the load power", Vr, P)
    _nonneg("the line resistance and reactance", R, X)
    _pos("the shunt susceptance", B)
    _need(R + X > 0, "the series impedance must not be zero")
    # complex sqrt/cosh/sinh differ in the last bit between platforms' maths libraries, and the losses (Ps - P) amplify
    # that; mpmath at 30 digits is pure Python, so A, B, C are the same correctly rounded doubles everywhere
    # (Linux vs Windows, 2026-10-23). Rounding the intermediates instead lost accuracy (cosh is close to 1).
    import mpmath
    with mpmath.workdps(30):
        z, y = mpmath.mpc(R, X), mpmath.mpc(0, B)
        g = mpmath.sqrt(z * y)
        if g.real < 0:
            g = -g
        zc = mpmath.sqrt(z / y)
        if zc.real < 0:
            zc = -zc
        gl, Zc = complex(g), complex(zc)
        A, Bc, C = complex(mpmath.cosh(g)), complex(zc * mpmath.sinh(g)), complex(mpmath.sinh(g) / zc)
    out = _line_ends(A, Bc, C, A, Vr, P, pf, pf_type, R)
    out.update(_out(Zc_mag=abs(Zc), Zc_angle_deg=math.degrees(cmath.phase(Zc)), alpha_l=gl.real, beta_l=gl.imag))
    return out


def _ref_long_line(Vr, P, pf, R, X, B, pf_type="lagging"):
    """The telegrapher equations dV/ds = Z I, dI/ds = Y V (s = 0 at the receiving end, 1 at the sending end) integrated
    by scipy solve_ivp (DOP853, rtol 1e-13); A, B, C, D from two integrations with unit end conditions."""
    import numpy as np
    from scipy import integrate
    Z, Y = complex(R, X), complex(0.0, B)

    def shoot(v0, i0):
        def f(s, y):
            v, i = complex(y[0], y[1]), complex(y[2], y[3])
            dv, di = Z * i, Y * v
            return [dv.real, dv.imag, di.real, di.imag]
        sol = integrate.solve_ivp(f, (0, 1), [v0.real, v0.imag, i0.real, i0.imag], method="DOP853", rtol=1e-13, atol=1e-15)
        y = sol.y[:, -1]
        return complex(y[0], y[1]), complex(y[2], y[3])
    A, C = shoot(1 + 0j, 0j)
    Bc, D = shoot(0j, 1 + 0j)
    phi = math.acos(pf) * (1 if pf_type != "leading" else -1)
    Ir = cmath.rect(P / (SQ3 * Vr * pf), -phi)
    vr = Vr / SQ3
    vs, is_ = A * vr + Bc * Ir, C * vr + D * Ir
    Ss = 3 * vs * is_.conjugate()
    Vs = SQ3 * abs(vs)
    Zc = np.sqrt(Bc / C)
    gl = np.arccosh(A)
    if gl.real < 0:
        gl = -gl
    return {"Vs": Vs, "delta_deg": math.degrees(cmath.phase(vs)), "Is": abs(is_), "Ps": Ss.real, "losses": Ss.real - P, "regulation_pct": 100 * (Vs / abs(A) - Vr) / Vr,
            "efficiency_pct": 100 * P / Ss.real, "A_mag": abs(A), "A_angle_deg": math.degrees(cmath.phase(A)), "B_mag": abs(Bc), "B_angle_deg": math.degrees(cmath.phase(Bc)),
            "Zc_mag": abs(complex(Zc)), "Zc_angle_deg": math.degrees(cmath.phase(complex(Zc))), "alpha_l": float(complex(gl).real), "beta_l": float(complex(gl).imag)}


_add("energy_long_line",
     name="Long transmission line (exact distributed parameters)", topic="power systems / transmission lines",
     purpose="Characteristic impedance, propagation constant, exact ABCD constants and the sending-end quantities, regulation and efficiency of a long three-phase line with uniformly distributed R, X and shunt B (the Ferranti rise appears as a negative regulation at light load).",
     inputs=_LINE2_IN, outputs=_LINE2_OUT + [_o("Zc_mag", "resistance", "characteristic (surge) impedance magnitude"), _o("Zc_angle_deg", "number", "angle of Zc in degrees"),
                                             _o("alpha_l", "number", "attenuation alpha l in nepers"), _o("beta_l", "number", "phase shift beta l in radians")],
     primary="Vs", conditions=["uniformly distributed parameters", "balanced load"],
     method="gamma l = sqrt(ZY), Zc = sqrt(Z/Y); A = D = cosh(gamma l), B = Zc sinh(gamma l), C = sinh(gamma l)/Zc",
     latex=r"A = \cosh\gamma l,\ B = Z_c\sinh\gamma l,\ C = \frac{\sinh\gamma l}{Z_c},\quad Z_c = \sqrt{\frac{z}{y}},\ \gamma = \sqrt{zy}",
     run=long_line,
     parse=lambda q: _pairs(q, r"long (?:transmission )?line", _LINE_NAMES, phys={"Vr", "P", "R", "X", "B"}, words={"pf_type"}, num_units=_pct_units("pf")),
     reference=_ref_long_line,
     wrong=_safe(lambda Vr, P, pf, R, X, B, pf_type="lagging": dict(medium_line(Vr, P, pf, R, X, B, pf_type), Zc_mag=0.0, Zc_angle_deg=0.0, alpha_l=0.0, beta_l=0.0)),
     random_inputs=lambda rng: _rnd_line(rng, True, True),
     templates=["Long line: Vr = 400 kV, P = 500 MW, pf = 0.95, R = 10 ohm, X = 120 ohm, B = 0.0012 S"],
     examples=[{"inputs": {"Vr": 400e3, "P": 1e6, "pf": 1.0, "R": 0.0, "X": 100.0, "B": 0.0004}, "outputs": {"Zc_mag": 500.0, "beta_l": 0.2, "A_mag": _r(math.cos(0.2))}}],
     questions=[{"q": "Long line: Vr = 400 kV, P = 1 MW, pf = 1, R = 0 ohm, X = 100 ohm, B = 400 uS", "outputs": {"Zc_mag": 500.0, "beta_l": 0.2}},
                {"q": "Long line: Vr = 400 kV, P = 1 MW, pf = 1, R = 0 ohm, X = 100 ohm, B = 0 uS", "outcome": "INVALID_INPUT"}])


MU0 = 4e-7 * math.pi                      # H/m (the conventional value used for line constants: 2e-7 ln(D/r))
EPS0 = 8.8541878188e-12                   # F/m (CODATA 2022)


def _bundle_equiv(r_or_gmr, n, d):
    if n == 1:
        return r_or_gmr
    if n == 2:
        return math.sqrt(r_or_gmr * d)
    if n == 3:
        return (r_or_gmr * d * d) ** (1 / 3)
    return 2 ** 0.125 * (r_or_gmr * d ** 3) ** 0.25


def line_parameters(D_ab, D_bc, D_ca, r, f, gmr=None, n_bundle=1, spacing=None, V=None):
    _pos("the phase spacings, the conductor radius and the frequency", D_ab, D_bc, D_ca, r, f)
    _need(isinstance(n_bundle, int) and 1 <= n_bundle <= 4, "the bundle has 1 to 4 subconductors")
    g = r * math.exp(-0.25) if gmr is None else gmr
    _pos("the geometric mean radius", g)
    _need(g <= r, "the geometric mean radius cannot exceed the conductor radius")
    if n_bundle > 1:
        _need(spacing is not None and spacing > 2 * r, "a bundle needs a subconductor spacing larger than the conductor diameter")
    d = spacing or 0.0
    gmd = (D_ab * D_bc * D_ca) ** (1 / 3)
    gmr_b, r_b = _bundle_equiv(g, n_bundle, d), _bundle_equiv(r, n_bundle, d)
    _need(gmd > 10 * r_b, "the phase spacing must be much larger than the conductor (bundle) size")
    L = MU0 / (2 * math.pi) * math.log(gmd / gmr_b)
    C = 2 * math.pi * EPS0 / math.log(gmd / r_b)
    w = 2 * math.pi * f
    out = dict(GMR=gmr_b, L_mH_per_km=L * 1e6, C_nF_per_km=C * 1e12, X_ohm_per_km=w * L * 1e3, B_uS_per_km=w * C * 1e9, Zc=math.sqrt(L / C))
    if V is not None:
        _pos("the line voltage", V)
        out["SIL_MW"] = V * V / math.sqrt(L / C) / 1e6
    return _out(**out)


def _ref_line_parameters(D_ab, D_bc, D_ca, r, f, gmr=None, n_bundle=1, spacing=None, V=None):
    """The phase inductance and potential-coefficient matrices built from every subconductor's position (numpy), the
    bundles reduced by averaging, and the transposed line's positive-sequence values L1 = Ls - Lm, 1/C1 = Ps - Pm."""
    import numpy as np
    g = r * math.exp(-0.25) if gmr is None else gmr
    # triangle with the given sides: a at origin, b on the x axis
    xa, ya = 0.0, 0.0
    xb, yb = D_ab, 0.0
    xc = (D_ab ** 2 + D_ca ** 2 - D_bc ** 2) / (2 * D_ab)
    yc = math.sqrt(max(D_ca ** 2 - xc ** 2, 0.0))
    centres = [(xa, ya), (xb, yb), (xc, yc)]
    n = n_bundle
    d = spacing or 0.0
    R = 0.0 if n == 1 else d / (2 * math.sin(math.pi / n))       # radius of the regular polygon of side d
    pts = []
    for (cx, cy) in centres:
        for k in range(n):
            a = 2 * math.pi * k / n
            pts.append((cx + R * math.cos(a), cy + R * math.sin(a)))
    pts = np.array(pts)
    N = len(pts)
    Dm = np.sqrt(((pts[:, None, :] - pts[None, :, :]) ** 2).sum(-1))
    phase = np.repeat(np.arange(3), n)

    def reduced(self_r):
        M = np.where(np.eye(N, dtype=bool), 0.0, np.log(1.0 / np.where(Dm == 0, 1.0, Dm)))
        M[np.eye(N, dtype=bool)] = math.log(1.0 / self_r)
        # equal current (or charge) sharing in a bundle: the phase coefficient is the mean of the subconductor block
        P3 = np.array([[M[np.ix_(phase == i, phase == j)].mean() if i == j else math.log(1.0 / math.dist(centres[i], centres[j]))
                        for j in range(3)] for i in range(3)])
        return P3
    Lm3 = MU0 / (2 * math.pi) * reduced(g)
    Pm3 = 1 / (2 * math.pi * EPS0) * reduced(r)
    Ls, Lmut = np.mean(np.diag(Lm3)), np.mean(Lm3[~np.eye(3, dtype=bool)])
    Ps, Pmut = np.mean(np.diag(Pm3)), np.mean(Pm3[~np.eye(3, dtype=bool)])
    L, C = Ls - Lmut, 1.0 / (Ps - Pmut)
    w = 2 * math.pi * f
    gm_a = float(np.exp(-np.mean([np.log(1 / x) for x in [Dm[0, n], Dm[n, 2 * n], Dm[2 * n, 0]]]) * -1)) if n == 1 else None
    out = {"L_mH_per_km": L * 1e6, "C_nF_per_km": C * 1e12, "X_ohm_per_km": w * L * 1e3, "B_uS_per_km": w * C * 1e9, "Zc": math.sqrt(L / C),
           "GMR": float(np.exp(-(reduced(g)[0, 0])))}
    if V is not None:
        out["SIL_MW"] = V * V / math.sqrt(L / C) / 1e6
    return out


_add("energy_line_parameters",
     name="Inductance, capacitance and surge impedance of a three-phase overhead line", topic="power systems / line parameters",
     purpose="Per-phase inductance, capacitance, reactance, susceptance, surge impedance (and surge-impedance loading at a voltage) of a fully transposed three-phase line from its phase spacings, conductor radius and geometric mean radius, with optional bundled conductors.",
     inputs=[_phys("D_ab", "length", "distance between phases a and b"), _phys("D_bc", "length", "distance between phases b and c"),
             _phys("D_ca", "length", "distance between phases c and a"), _phys("r", "length", "conductor (outer) radius"), _phys("f", "frequency", "system frequency"),
             _phys("gmr", "length", "conductor geometric mean radius (default r e^(-1/4), a solid round conductor)", required=False),
             _intin("n_bundle", "subconductors per phase (1 to 4, regular polygon)", 1, 4, required=False),
             _phys("spacing", "length", "distance between neighbouring subconductors of a bundle", required=False),
             _phys("V", "voltage", "line voltage for the surge-impedance loading (optional)", required=False)],
     outputs=[_o("GMR", "length", "geometric mean radius of a phase (bundle)"),
              _o("L_mH_per_km", "number", "inductance per phase in mH/km"), _o("C_nF_per_km", "number", "capacitance to neutral in nF/km"),
              _o("X_ohm_per_km", "number", "series reactance in ohm/km"), _o("B_uS_per_km", "number", "shunt susceptance in uS/km"),
              _o("Zc", "resistance", "surge impedance of the lossless line sqrt(L/C)"), _o("SIL_MW", "number", "surge-impedance loading V^2/Zc in MW")],
     primary="L_mH_per_km", conditions=["fully transposed line", "spacings much larger than the conductors", "ground effect neglected"],
     method="GMD = (D_ab D_bc D_ca)^(1/3); L = 2e-7 ln(GMD/GMR) H/m; C = 2 pi eps0/ln(GMD/r) F/m; bundles: GMR_b = (GMR d^(n-1))^(1/n) (2^(1/8) (GMR d^3)^(1/4) = 1.09 (GMR d^3)^(1/4) for 4)",
     latex=r"L = 2\times10^{-7}\ln\frac{GMD}{GMR}\ \mathrm{H/m},\quad C = \frac{2\pi\varepsilon_0}{\ln(GMD/r)}\ \mathrm{F/m}",
     run=line_parameters,
     parse=lambda q: _pairs(q, r"line parameters|line inductance and capacitance|transmission line parameters", {"D_ab": "D_ab", "D_bc": "D_bc", "D_ca": "D_ca", "r": "r", "radius": "r",
                                                                                                          "f": "f", "frequency": "f", "gmr": "gmr", "GMR": "gmr", "n_bundle": "n_bundle",
                                                                                                          "bundle": "n_bundle", "spacing": "spacing", "d": "spacing", "V": "V"},
                            phys={"D_ab", "D_bc", "D_ca", "r", "f", "gmr", "spacing", "V"}),
     reference=_ref_line_parameters,
     wrong=_safe(lambda D_ab, D_bc, D_ca, r, f, gmr=None, n_bundle=1, spacing=None, V=None: (lambda o: dict(o, L_mH_per_km=_r(o["L_mH_per_km"] * math.log((D_ab + D_bc + D_ca) / 3 / o["GMR"]) / math.log((D_ab * D_bc * D_ca) ** (1 / 3) / o["GMR"]))))(
         line_parameters(D_ab, D_bc, D_ca, r, f, gmr, n_bundle, spacing, V))),
     random_inputs=lambda rng: (lambda n, r: dict({"D_ab": round(rng.uniform(3, 12), 3), "D_bc": round(rng.uniform(3, 12), 3), "D_ca": None, "r": r, "f": rng.choice([50.0, 60.0]),
                                                   "n_bundle": n}, **({"spacing": round(rng.uniform(0.3, 0.5), 3)} if n > 1 else {}),
                                                  **({"gmr": round(r * rng.uniform(0.7, 0.8), 6)} if rng.random() < 0.5 else {}),
                                                  **({"V": rng.choice([132e3, 220e3, 400e3])} if rng.random() < 0.5 else {})))(rng.randint(1, 4), round(rng.uniform(0.008, 0.02), 5)),
     templates=["Line parameters: D_ab = 4 m, D_bc = 4 m, D_ca = 8 m, r = 1.5 cm, f = 50 Hz"],
     examples=[{"inputs": {"D_ab": 5, "D_bc": 5, "D_ca": 5, "r": 0.01, "f": 50, "gmr": 0.005}, "outputs": {"GMR": 0.005, "L_mH_per_km": _r(0.2 * math.log(1000))}},
               {"inputs": {"D_ab": 6, "D_bc": 6, "D_ca": 6, "r": 0.012, "f": 60, "gmr": 0.01, "n_bundle": 2, "spacing": 0.4},
                "outputs": {"GMR": _r(math.sqrt(0.004)), "L_mH_per_km": _r(0.2 * math.log(6 / math.sqrt(0.004)))}}],
     questions=[{"q": "Line parameters: D_ab = 5 m, D_bc = 5 m, D_ca = 5 m, r = 1 cm, f = 50 Hz, gmr = 5 mm", "outputs": {"L_mH_per_km": _r(0.2 * math.log(1000))}},
                {"q": "Line parameters: D_ab = 5 m, D_bc = 5 m, D_ca = 5 m, r = 1 cm, f = 50 Hz, gmr = 2 cm", "outcome": "INVALID_INPUT"}])


def _fix_line_rng(spec):
    base = spec["random_inputs"]

    def rnd(rng):
        for _ in range(100):
            x = base(rng)
            a, b = x["D_ab"], x["D_bc"]
            x["D_ca"] = round(rng.uniform(abs(a - b) + 0.5, a + b - 0.5), 3)       # a real triangle
            return x
    spec["random_inputs"] = rnd


_fix_line_rng(PROCEDURES["energy_line_parameters"])


def two_bus_load_flow(Vs, R, X, P, Q):
    _pos("the sending-end voltage", Vs)
    _nonneg("the line resistance and reactance", R, X)
    _need(R + X > 0, "the line impedance must not be zero")
    _nonneg("the load real power", P)
    a = 2 * (P * R + Q * X) - Vs * Vs
    disc = a * a - 4 * (P * P + Q * Q) * (R * R + X * X)
    _need(disc >= 0, "the load exceeds what the line can deliver: the load flow has no solution")
    V2 = (-a + math.sqrt(disc)) / 2
    Vr = math.sqrt(V2)
    re, im = Vr + (P * R + Q * X) / Vr, (P * X - Q * R) / Vr
    I = math.hypot(P, Q) / (SQ3 * Vr)
    Sl = (P * P + Q * Q) / V2
    return _out(Vr=Vr, delta_deg=-math.degrees(math.atan2(im, re)), I=I, P_loss=Sl * R, Q_loss=Sl * X, voltage_drop_pct=100 * (Vs - Vr) / Vs)


def _ref_two_bus(Vs, R, X, P, Q):
    import pandapower as pp
    net = _pp_empty(sn_mva=1.0)
    b1 = pp.create_bus(net, vn_kv=Vs / 1e3)
    b2 = pp.create_bus(net, vn_kv=Vs / 1e3)
    pp.create_ext_grid(net, b1, vm_pu=1.0)
    pp.create_line_from_parameters(net, b1, b2, 1.0, R, X, 0.0, 1e9)
    pp.create_load(net, b2, p_mw=P / 1e6, q_mvar=Q / 1e6)
    pp.runpp(net, tolerance_mva=1e-12 * max(1.0, math.hypot(P, Q) / 1e6), max_iteration=60, numba=False, check_connectivity=False)
    Vr = float(net.res_bus.vm_pu.iloc[1]) * Vs
    return {"Vr": Vr, "delta_deg": float(net.res_bus.va_degree.iloc[1]), "I": float(net.res_line.i_ka.iloc[0]) * 1e3,
            "P_loss": float(net.res_line.pl_mw.iloc[0]) * 1e6, "Q_loss": float(net.res_line.ql_mvar.iloc[0]) * 1e6, "voltage_drop_pct": 100 * (Vs - Vr) / Vs}


def _rnd_two_bus(rng):
    V = rng.choice([11e3, 33e3, 66e3, 132e3])
    R, X = round(rng.uniform(0.5, 10), 3), round(rng.uniform(2, 40), 3)
    smax = V * V / (2 * math.hypot(R, X))
    s = smax * rng.uniform(0.05, 0.5)
    pf = rng.uniform(0.7, 1.0)
    return {"Vs": V, "R": R, "X": X, "P": round(s * pf, 0), "Q": round(s * math.sin(math.acos(pf)) * rng.choice([1, -0.3]), 0)}


_add("energy_two_bus_load_flow",
     name="Receiving-end voltage of a line feeding a constant-power load (two-bus load flow)", topic="power systems / load flow",
     purpose="The receiving-end voltage magnitude and angle, current and losses when a three-phase line of impedance R + jX supplies a constant P + jQ load from a fixed sending-end voltage (the exact two-bus load flow).",
     inputs=[_phys("Vs", "voltage", "sending-end line-to-line voltage (slack)"), _phys("R", "resistance", "line resistance per phase"), _phys("X", "resistance", "line reactance per phase"),
             _phys("P", "power", "load real power (three-phase)"), _phys("Q", "power", "load reactive power in var (negative for a capacitive load)")],
     outputs=[_o("Vr", "voltage", "receiving-end line voltage"), _o("delta_deg", "number", "receiving-end voltage angle in degrees (negative: lagging the source)"),
              _o("I", "current", "line current"), _o("P_loss", "power", "real power loss"), _o("Q_loss", "power", "reactive power absorbed by the line in var"),
              _o("voltage_drop_pct", "number", "(V_s - V_r)/V_s in percent")],
     primary="Vr", conditions=["the load is within the line's transfer limit (a real solution exists)", "the high-voltage (normal) solution is reported"],
     method="V_r^4 + [2(PR + QX) - V_s^2] V_r^2 + (P^2 + Q^2)(R^2 + X^2) = 0, larger root; angle from V_s = V_r + (PR + QX)/V_r + j(PX - QR)/V_r",
     latex=r"V_r^4 + \left[2(PR+QX) - V_s^2\right]V_r^2 + (P^2+Q^2)(R^2+X^2) = 0",
     run=two_bus_load_flow,
     parse=lambda q: _pairs(q, r"two[- ]bus load flow|receiving[- ]end voltage of a line", {"Vs": "Vs", "V_s": "Vs", "R": "R", "X": "X", "P": "P", "Q": "Q"}, phys={"Vs", "R", "X", "P", "Q"}),
     reference=_ref_two_bus,
     wrong=_safe(lambda Vs, R, X, P, Q: (lambda dv: _out(Vr=Vs - dv, delta_deg=-math.degrees((P * X - Q * R) / (Vs * Vs)), I=math.hypot(P, Q) / (SQ3 * (Vs - dv)),
                                                         P_loss=(P * P + Q * Q) / Vs ** 2 * R, Q_loss=(P * P + Q * Q) / Vs ** 2 * X, voltage_drop_pct=100 * dv / Vs))((P * R + Q * X) / Vs)),
     random_inputs=_rnd_two_bus,
     templates=["Two-bus load flow: Vs = 33 kV, R = 3 ohm, X = 12 ohm, P = 10 MW, Q = 5 Mvar"],
     examples=[{"inputs": {"Vs": 33e3, "R": 0.0, "X": 10.0, "P": 0.0, "Q": 0.0}, "outputs": {"Vr": 33000.0, "I": 0.0}},
               {"inputs": {"Vs": 2000.0, "R": 0.0, "X": 1.0, "P": 1e6, "Q": 0.0}, "outputs": {"Vr": _r(1000 * math.sqrt(2 + math.sqrt(3))), "delta_deg": -15.0, "P_loss": 0.0}}],
     questions=[{"q": "Two-bus load flow: Vs = 2 kV, R = 0 ohm, X = 1 ohm, P = 1 MW, Q = 0 var", "outputs": {"Vr": _r(1000 * math.sqrt(2 + math.sqrt(3))), "delta_deg": -15.0}},
                {"q": "Two-bus load flow: Vs = 33 kV, R = 3 ohm, X = 12 ohm, P = 100 MW, Q = 50 Mvar", "outcome": "INVALID_INPUT"}])


def _bus_index(name, n=None):
    _need(re.fullmatch(r"\d+", str(name)) is not None, f"bus names must be whole numbers 1, 2, 3, ... (got {name!r})")
    return int(name)


def dc_power_flow(lines, P):
    _need(len(lines) >= 1, "give at least one line")
    edges = []
    for e in lines:
        _need(len(e) == 3, "each line is 'i-j x' with its reactance in per unit")
        i, j, x = _bus_index(e[0]), _bus_index(e[1]), float(e[2])
        _need(i != j, "a line must join two different buses")
        _need(x > 0, "line reactances must be positive")
        edges.append((i, j, x))
    nb = max(max(i, j) for i, j, _ in edges)
    _need(set(range(1, nb + 1)) == {b for i, j, _ in edges for b in (i, j)}, "buses must be numbered 1 to n without gaps, every bus on a line")
    _need(len(P) == nb - 1, f"give the net injections of buses 2 to {nb} ({nb - 1} values); bus 1 is the slack")
    B = [[0.0] * nb for _ in range(nb)]
    for i, j, x in edges:
        b = 1.0 / x
        B[i - 1][i - 1] += b
        B[j - 1][j - 1] += b
        B[i - 1][j - 1] -= b
        B[j - 1][i - 1] -= b
    th = _gauss_solve([row[1:] for row in B[1:]], list(P))
    theta = [0.0] + th
    flows = [(theta[i - 1] - theta[j - 1]) / x for i, j, x in edges]
    return _out(theta_deg=[math.degrees(t) for t in theta], flows=flows, P_slack=-sum(P))


def _ref_dc_pf(lines, P):
    import pandapower as pp
    nb = max(max(int(e[0]), int(e[1])) for e in lines)
    net = _pp_empty(sn_mva=1.0, f_hz=50)
    buses = [pp.create_bus(net, vn_kv=1.0) for _ in range(nb)]
    pp.create_ext_grid(net, buses[0])
    for e in lines:
        pp.create_line_from_parameters(net, buses[int(e[0]) - 1], buses[int(e[1]) - 1], 1.0, 0.0, float(e[2]), 0.0, 1e9)
    for k, p in enumerate(P, start=2):
        pp.create_load(net, buses[k - 1], p_mw=-p, q_mvar=0.0)
    pp.rundcpp(net)
    return {"theta_deg": [float(v) for v in net.res_bus.va_degree], "flows": [float(v) for v in net.res_line.p_from_mw],
            "P_slack": float(net.res_ext_grid.p_mw.iloc[0])}


def _rnd_network(rng, nb, extra):
    edges = []
    for b in range(2, nb + 1):
        edges.append((rng.randint(1, b - 1), b))
    for _ in range(extra):
        i, j = rng.sample(range(1, nb + 1), 2)
        if (i, j) not in edges and (j, i) not in edges:
            edges.append((min(i, j), max(i, j)))
    return edges


def _rnd_dcpf(rng):
    nb = rng.randint(3, 6)
    edges = _rnd_network(rng, nb, rng.randint(0, 3))
    lines = ", ".join(f"{i}-{j} {round(rng.uniform(0.05, 0.4), 4)}" for i, j in edges)
    P = [round(rng.uniform(-1.0, 0.6), 3) for _ in range(nb - 1)]
    return {"lines": lines, "P": P}


_add("energy_dc_power_flow",
     name="DC power flow", topic="power systems / load flow",
     purpose="Bus voltage angles and line flows of a network by the DC (linearised) power flow: lossless lines, flat 1 pu voltages, P_ij = (theta_i - theta_j)/x_ij; bus 1 is the slack (angle 0) and takes the balance.",
     inputs=[{"symbol": "lines", "type": "edges", "required": True, "min_length": 1, "max_length": 40,
              "meaning": "lines as 'i-j x' with the series reactance x in per unit (buses numbered 1..n)"},
             _vecin("P", "net injections (generation minus load) of buses 2..n in per unit", 1, 30)],
     outputs=[_o("theta_deg", "number", "bus voltage angles in degrees, buses 1..n"), _o("flows", "number", "line flows from the first to the second named bus, per unit, in the order given"),
              _o("P_slack", "number", "slack (bus 1) injection in per unit")],
     primary="P_slack", conditions=["connected network", "positive reactances", "bus 1 is the slack"],
     method="B' theta = P (reduced susceptance matrix, slack removed), solved by Gaussian elimination; P_ij = (theta_i - theta_j)/x_ij",
     latex=r"\mathbf B'\boldsymbol\theta = \mathbf P,\quad P_{ij} = \frac{\theta_i - \theta_j}{x_{ij}}",
     run=dc_power_flow,
     parse=lambda q: _pairs(q, r"dc power flow|dc load flow", {"lines": "lines", "P": "P", "injections": "P"}, vectors={"P"}, words=()) if not re.search(r"lines\s*=", q or "") else
     _dc_parse(q),
     reference=_ref_dc_pf,
     wrong=_safe(lambda lines, P: (lambda o: dict(o, flows=_r([f * 2 for f in o["flows"]]), theta_deg=_r([t * 2 for t in o["theta_deg"]])))(dc_power_flow(lines, P))),
     random_inputs=_rnd_dcpf,
     templates=["DC power flow: lines = 1-2 0.1; 1-3 0.2; 2-3 0.25, P = [0.5, -1.2]"],
     examples=[{"inputs": {"lines": "1-2 0.1, 2-3 0.2", "P": "0, -1"}, "outputs": {"theta_deg": _r([0.0, math.degrees(-0.1), math.degrees(-0.3)]), "flows": [1.0, 1.0], "P_slack": 1.0}},
               {"inputs": {"lines": "1-2 0.1, 1-2 0.1", "P": "-1"}, "outputs": {"flows": [0.5, 0.5]}}],
     questions=[{"q": "DC power flow: lines = 1-2 0.1; 2-3 0.2, P = [0, -1]", "outputs": {"flows": [1.0, 1.0], "P_slack": 1.0}},
                {"q": "DC power flow: lines = 1-2 0.1; 2-3 0.2, P = [0, -1, 2]", "outcome": "INVALID_INPUT"},
                {"q": "DC power flow: lines = 1-2 0.1; 2-3 0.2", "outcome": "NEEDS_INPUT"}])


def _dc_parse(q):
    """'lines = 1-2 0.1; 2-3 0.2' (edges separated by semicolons), then the other pairs."""
    m = re.match(r"\s*(?:dc power flow|dc load flow)\s*:\s*", q, re.I)
    if not m:
        return None
    rest = q[m.end():]
    ml = re.search(r"\blines\s*=\s*((?:\d+\s*-\s*\d+\s+[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?\s*;?\s*)+)", rest)
    if not ml:
        return None
    lines = ", ".join(p.strip() for p in ml.group(1).split(";") if p.strip())
    other = (rest[:ml.start()] + rest[ml.end():]).strip().strip(",").strip()
    out = _pairs("x: " + other, "x", {"P": "P", "injections": "P"}, vectors={"P"}) if other else {}
    if out is None:
        return None
    out["lines"] = lines
    return out


PROCEDURES["energy_dc_power_flow"]["parse"] = _dc_parse


def newton_load_flow(lines, loads, gens=None, V1=1.0):
    """Newton-Raphson load flow in polar form; bus 1 is the slack (V1 at angle 0)."""
    _need(0.8 <= V1 <= 1.2, "the slack voltage must lie between 0.8 and 1.2 pu")
    br = []
    for row in lines:
        _need(len(row) == 5, "each line row is: from bus, to bus, r, x, total shunt b (per unit)")
        i, j = int(row[0]), int(row[1])
        _need(row[0] == i and row[1] == j and i != j and i >= 1 and j >= 1, "line buses must be different whole numbers from 1")
        r, x, b = row[2], row[3], row[4]
        _need(r >= 0 and x > 0 and b >= 0, "line r and b must not be negative and x must be positive")
        br.append((i, j, complex(r, x), b))
    nb = max(max(i, j) for i, j, _, _ in br)
    _need({k for i, j, _, _ in br for k in (i, j)} == set(range(1, nb + 1)), "buses must be numbered 1 to n without gaps")
    Pd, Qd = [0.0] * nb, [0.0] * nb
    for row in loads:
        _need(len(row) == 3 and row[0] == int(row[0]) and 2 <= int(row[0]) <= nb, "each load row is: bus (2..n), P, Q in per unit")
        Pd[int(row[0]) - 1] += row[1]
        Qd[int(row[0]) - 1] += row[2]
    pv, Pg, Vset = [], [0.0] * nb, {}
    for row in (gens or []):
        _need(len(row) == 3 and row[0] == int(row[0]) and 2 <= int(row[0]) <= nb, "each generator row is: bus (2..n), P, V (per unit)")
        k = int(row[0]) - 1
        _need(k not in Vset, "one generator row per bus")
        _need(0.8 <= row[2] <= 1.2, "generator voltages must lie between 0.8 and 1.2 pu")
        Pg[k] += row[1]
        Vset[k] = row[2]
    Y = [[0j] * nb for _ in range(nb)]
    for i, j, z, b in br:
        y = 1 / z
        Y[i - 1][i - 1] += y + 0.5j * b
        Y[j - 1][j - 1] += y + 0.5j * b
        Y[i - 1][j - 1] -= y
        Y[j - 1][i - 1] -= y
    V = [1.0] * nb
    V[0] = V1
    for k, v in Vset.items():
        V[k] = v
    th = [0.0] * nb
    pq = [k for k in range(1, nb) if k not in Vset]
    pvb = [k for k in range(1, nb) if k in Vset]
    nsl = list(range(1, nb))
    Psp = [Pg[k] - Pd[k] for k in range(nb)]
    Qsp = [-Qd[k] for k in range(nb)]

    def calc():
        Vc = [cmath.rect(V[k], th[k]) for k in range(nb)]
        S = [Vc[k] * sum(Y[k][m] * Vc[m] for m in range(nb)).conjugate() for k in range(nb)]
        return Vc, S
    for it in range(40):
        Vc, S = calc()
        mis = [Psp[k] - S[k].real for k in nsl] + [Qsp[k] - S[k].imag for k in pq]
        if max(abs(m) for m in mis) < 1e-13:
            break
        # Jacobian by the standard polar derivatives
        n1, n2 = len(nsl), len(pq)
        J = [[0.0] * (n1 + n2) for _ in range(n1 + n2)]
        for a, k in enumerate(nsl):
            for c, m in enumerate(nsl):
                if k == m:
                    J[a][c] = -S[k].imag - V[k] ** 2 * Y[k][k].imag
                else:
                    Ykm = Y[k][m]
                    t = th[k] - th[m]
                    J[a][c] = V[k] * V[m] * (Ykm.real * math.sin(t) - Ykm.imag * math.cos(t))
            for c, m in enumerate(pq):
                if k == m:
                    J[a][n1 + c] = S[k].real / V[k] + V[k] * Y[k][k].real
                else:
                    Ykm = Y[k][m]
                    t = th[k] - th[m]
                    J[a][n1 + c] = V[k] * (Ykm.real * math.cos(t) + Ykm.imag * math.sin(t))
        for a, k in enumerate(pq):
            for c, m in enumerate(nsl):
                if k == m:
                    J[n1 + a][c] = S[k].real - V[k] ** 2 * Y[k][k].real
                else:
                    Ykm = Y[k][m]
                    t = th[k] - th[m]
                    J[n1 + a][c] = -V[k] * V[m] * (Ykm.real * math.cos(t) + Ykm.imag * math.sin(t))
            for c, m in enumerate(pq):
                if k == m:
                    J[n1 + a][n1 + c] = S[k].imag / V[k] - V[k] * Y[k][k].imag
                else:
                    Ykm = Y[k][m]
                    t = th[k] - th[m]
                    J[n1 + a][n1 + c] = V[k] * (Ykm.real * math.sin(t) - Ykm.imag * math.cos(t))
        dx = _gauss_solve(J, mis)
        for a, k in enumerate(nsl):
            th[k] += dx[a]
        for c, m in enumerate(pq):
            V[m] += dx[n1 + c]
        _need(all(0.3 < v < 2 for v in V), "the load flow diverged (the network cannot carry this loading)")
    else:
        raise ValueError("the load flow did not converge in 40 iterations")
    Vc, S = calc()
    _need(max(abs(Psp[k] - S[k].real) for k in nsl) < 1e-10, "the load flow did not converge")
    loss = sum(s.real for s in S)
    return _out(V=list(V), angle_deg=[math.degrees(t) for t in th], P_slack=S[0].real, Q_slack=S[0].imag, P_loss=loss,
                Q_gen=[S[k].imag + Qd[k] for k in pvb])


def _ref_newton_lf(lines, loads, gens=None, V1=1.0):
    import pandapower as pp
    nb = int(max(max(r[0], r[1]) for r in lines))
    f = 50.0
    net = _pp_empty(sn_mva=1.0, f_hz=f)
    buses = [pp.create_bus(net, vn_kv=1.0) for _ in range(nb)]
    pp.create_ext_grid(net, buses[0], vm_pu=V1)
    for r in lines:
        pp.create_line_from_parameters(net, buses[int(r[0]) - 1], buses[int(r[1]) - 1], 1.0, r[2], r[3], r[4] / (2 * math.pi * f) * 1e9, 1e9)
    for r in loads:
        pp.create_load(net, buses[int(r[0]) - 1], p_mw=r[1], q_mvar=r[2])
    for r in (gens or []):
        pp.create_gen(net, buses[int(r[0]) - 1], p_mw=r[1], vm_pu=r[2])
    pp.runpp(net, tolerance_mva=1e-12, max_iteration=60, numba=False, check_connectivity=False)
    out = {"V": [float(v) for v in net.res_bus.vm_pu], "angle_deg": [float(v) for v in net.res_bus.va_degree],
           "P_slack": float(net.res_ext_grid.p_mw.iloc[0]), "Q_slack": float(net.res_ext_grid.q_mvar.iloc[0]),
           "P_loss": float(net.res_line.pl_mw.sum())}
    # generator reactive power from pandapower's solved voltages and its own admittance matrix: its reported generator
    # Q is rounded to about 7 digits
    qload = {}
    for r in loads:
        qload[int(r[0])] = qload.get(int(r[0]), 0.0) + r[2]
    S = _pp_injections(net)
    out["Q_gen"] = [S[int(r[0]) - 1].imag + qload.get(int(r[0]), 0.0) for r in (gens or [])]
    return out


def _pp_injections(net):
    """Complex power injected at every bus (in the network's MVA) from pandapower's solved voltages and its own bus
    admittance matrix (both double precision)."""
    import numpy as np
    Y = net._ppc["internal"]["Ybus"].toarray()
    lk = net._pd2ppc_lookups["bus"]
    V = np.zeros(Y.shape[0], dtype=complex)
    for b in net.bus.index:
        V[lk[b]] = net.res_bus.vm_pu[b] * np.exp(1j * np.radians(net.res_bus.va_degree[b]))
    S = V * np.conj(Y @ V) * net.sn_mva
    return [complex(S[lk[b]]) for b in net.bus.index]


def _rnd_newton(rng):
    nb = rng.randint(3, 5)
    edges = _rnd_network(rng, nb, rng.randint(1, 2))
    rows = []
    for i, j in edges:
        r = round(rng.uniform(0.01, 0.06), 4)
        rows.append(f"{i} {j} {r} {round(r * rng.uniform(3, 6), 4)} {round(rng.uniform(0, 0.06), 4)}")
    gens = []
    gb = None
    if rng.random() < 0.6:
        gb = rng.randint(2, nb)
        gens.append(f"{gb} {round(rng.uniform(0.1, 0.5), 3)} {round(rng.uniform(0.98, 1.04), 3)}")
    loads = [f"{b} {round(rng.uniform(0.05, 0.4), 3)} {round(rng.uniform(0.0, 0.2), 3)}" for b in range(2, nb + 1) if b != gb or rng.random() < 0.3]
    x = {"lines": "; ".join(rows), "loads": "; ".join(loads) or f"2 0.2 0.1", "V1": round(rng.uniform(0.98, 1.05), 3)}
    if gens:
        x["gens"] = "; ".join(gens)
    return x


_add("energy_newton_load_flow",
     name="AC load flow by Newton-Raphson", topic="power systems / load flow",
     purpose="Bus voltage magnitudes and angles, slack-bus power, PV-bus reactive power and losses of a small network by the Newton-Raphson load flow (per unit; pi-model lines; bus 1 is the slack, generator buses hold their voltage; reactive limits are not enforced).",
     inputs=[_matin("lines", "one row per line: from bus, to bus, r, x, total shunt b (per unit)"),
             _matin("loads", "one row per load: bus, P, Q (per unit)"),
             _matin("gens", "optional rows per voltage-controlled (PV) generator: bus, P, V (per unit)", required=False),
             _numin("V1", "slack-bus voltage magnitude in per unit (default 1.0)", required=False)],
     outputs=[_o("V", "number", "voltage magnitudes in per unit, buses 1..n"), _o("angle_deg", "number", "voltage angles in degrees, buses 1..n"),
              _o("P_slack", "number", "slack real power injection (pu)"), _o("Q_slack", "number", "slack reactive power injection (pu)"),
              _o("P_loss", "number", "total real power loss (pu)"), _o("Q_gen", "number", "reactive power of each PV generator (pu), in the order given")],
     primary="P_slack", conditions=["buses numbered 1..n, bus 1 the slack", "the load flow converges from a flat start"],
     method="polar Newton-Raphson on the P and Q mismatches with the analytic Jacobian, solved by Gaussian elimination, to a mismatch below 1e-13 pu",
     latex=r"\begin{bmatrix}\Delta\mathbf P\\ \Delta\mathbf Q\end{bmatrix} = \begin{bmatrix}\mathbf H & \mathbf N\\ \mathbf M & \mathbf L\end{bmatrix}\begin{bmatrix}\Delta\boldsymbol\theta\\ \Delta|\mathbf V|\end{bmatrix}",
     run=newton_load_flow,
     parse=lambda q: _pairs(q, r"newton(?:[- ]raphson)? load flow|ac load flow|load flow", {"lines": "lines", "loads": "loads", "gens": "gens", "generators": "gens", "V1": "V1", "V_slack": "V1"},
                            matrices={"lines", "loads", "gens"}),
     reference=_ref_newton_lf,
     wrong=_safe(lambda lines, loads, gens=None, V1=1.0: newton_load_flow([[r[0], r[1], r[2], r[3], 0.0] for r in lines], loads, gens, V1)
                 if any(r[4] > 0 for r in lines) else newton_load_flow([[r[0], r[1], 2 * r[2], r[3], r[4]] for r in lines], loads, gens, V1)),
     random_inputs=_rnd_newton,
     templates=["Newton-Raphson load flow: lines = [1 2 0.02 0.06 0.03]; [1 3 0.08 0.24 0.025]; [2 3 0.06 0.18 0.02], loads = [2 0.5 0.2]; [3 0.4 0.15]"],
     examples=[{"inputs": {"lines": "1 2 0.0 0.5 0.0", "loads": "2 0.0 0.0"}, "outputs": {"V": [1.0, 1.0], "P_slack": 0.0}}],
     questions=[{"q": "Load flow: lines = [1 2 0 0.5 0], loads = [2 0 0]", "outputs": {"V": [1.0, 1.0]}},
                {"q": "Load flow: lines = [1 2 0.02 0.06 0.03], loads = [2 50 20]", "outcome": "INVALID_INPUT"},
                {"q": "Load flow: loads = [2 0.5 0.2]", "outcome": "NEEDS_INPUT"}])


def economic_dispatch(costs, demand):
    _need(len(costs) >= 1, "give at least one generator")
    units = []
    for row in costs:
        _need(len(row) == 5, "each generator row is: a, b, c, Pmin, Pmax (cost a + b P + c P^2 in $/h, P in MW)")
        a, b, c, lo, hi = row
        _need(c > 0, "the quadratic cost coefficient c must be positive")
        _need(0 <= lo < hi, "each generator needs 0 <= Pmin < Pmax")
        units.append((a, b, c, lo, hi))
    _need(sum(u[3] for u in units) <= demand <= sum(u[4] for u in units), "the demand must lie between the total minimum and total maximum output")
    P = lambda lam: [min(max((lam - b) / (2 * c), lo), hi) for a, b, c, lo, hi in units]   # noqa: E731
    lo_l = min(b + 2 * c * lo for a, b, c, lo, hi in units) - 1
    hi_l = max(b + 2 * c * hi for a, b, c, lo, hi in units) + 1
    lam = _bisect(lambda l: sum(P(l)) - demand, lo_l, hi_l, 2000)
    Ps = P(lam)
    free = [k for k, (a, b, c, lo, hi) in enumerate(units) if lo < Ps[k] < hi]
    _need(bool(free), "every unit is at a limit: the incremental cost is not unique")
    # with at least one unit between its limits the incremental cost is exact: lambda = b + 2 c P of a free unit
    k = free[0]
    lam = units[k][1] + 2 * units[k][2] * Ps[k]
    cost = sum(a + b * p + c * p * p for (a, b, c, lo, hi), p in zip(units, Ps))
    return _out(lambda_=lam, P=Ps, cost=cost)


def _ref_dispatch(costs, demand):
    """Enumeration of the optimality (KKT) conditions: every assignment of the units to 'between limits', 'at Pmin' or
    'at Pmax'; the free units share one incremental cost from a linear solve (numpy); the assignment whose
    multipliers have the right signs is the optimum."""
    import numpy as np
    n = len(costs)
    for states in itertools.product((0, 1, 2), repeat=n):
        free = [k for k in range(n) if states[k] == 0]
        if not free:
            continue
        fixed = sum(costs[k][3] if states[k] == 1 else costs[k][4] for k in range(n) if states[k] != 0)
        # unknowns: lambda and P of free units: 2 c P - lambda = -b ; sum P = demand - fixed
        m = len(free) + 1
        A = np.zeros((m, m))
        rhs = np.zeros(m)
        for r, k in enumerate(free):
            A[r, r] = 2 * costs[k][2]
            A[r, -1] = -1
            rhs[r] = -costs[k][1]
        A[-1, :-1] = 1
        rhs[-1] = demand - fixed
        sol = np.linalg.solve(A, rhs)
        lam = sol[-1]
        P = [0.0] * n
        ok = True
        for r, k in enumerate(free):
            P[k] = sol[r]
            ok &= costs[k][3] - 1e-9 <= P[k] <= costs[k][4] + 1e-9
        for k in range(n):
            if states[k] == 1:
                P[k] = costs[k][3]
                ok &= costs[k][1] + 2 * costs[k][2] * P[k] >= lam - 1e-9
            elif states[k] == 2:
                P[k] = costs[k][4]
                ok &= costs[k][1] + 2 * costs[k][2] * P[k] <= lam + 1e-9
        if ok:
            return {"lambda_": float(lam), "P": [float(p) for p in P], "cost": float(sum(c[0] + c[1] * p + c[2] * p * p for c, p in zip(costs, P)))}
    raise ValueError("no feasible dispatch")


def _rnd_dispatch(rng):
    n = rng.randint(2, 5)
    rows = []
    for _ in range(n):
        lo = round(rng.uniform(10, 100), 1)
        rows.append([round(rng.uniform(100, 600), 1), round(rng.uniform(5, 12), 3), round(rng.uniform(0.001, 0.01), 5), lo, round(lo + rng.uniform(100, 400), 1)])
    tmin, tmax = sum(r[3] for r in rows), sum(r[4] for r in rows)
    return {"costs": "; ".join(" ".join(str(v) for v in r) for r in rows), "demand": round(rng.uniform(tmin + 0.05 * (tmax - tmin), tmax - 0.05 * (tmax - tmin)), 1)}


_add("energy_economic_dispatch",
     name="Economic dispatch with quadratic costs and limits", topic="power systems / economic dispatch",
     purpose="The least-cost outputs of thermal units with costs a + b P + c P^2 ($/h) and output limits that together meet a demand (losses neglected): every unit between its limits runs at the same incremental cost lambda.",
     inputs=[_matin("costs", "one row per unit: a ($/h), b ($/MWh), c ($/MW^2 h), Pmin (MW), Pmax (MW)"), _numin("demand", "total demand in MW")],
     outputs=[_o("lambda_", "number", "system incremental cost in $/MWh"), _o("P", "number", "unit outputs in MW, in the order given"), _o("cost", "number", "total cost in $/h")],
     primary="cost", conditions=["c > 0 for every unit", "sum Pmin <= demand <= sum Pmax", "at least one unit between its limits"],
     method="equal incremental cost: P_i(lambda) = clip((lambda - b_i)/(2 c_i), Pmin_i, Pmax_i), lambda by bisection on sum P_i = D",
     latex=r"\frac{dC_i}{dP_i} = b_i + 2c_iP_i = \lambda\ \ (P_{min} < P_i < P_{max}),\quad \sum_i P_i = D",
     run=economic_dispatch,
     parse=lambda q: _pairs(q, r"economic dispatch", {"costs": "costs", "units": "costs", "demand": "demand", "D": "demand", "load": "demand"}, matrices={"costs"}, num_units={"demand": ("MW",)}),
     reference=_ref_dispatch,
     wrong=_safe(lambda costs, demand: economic_dispatch([[r[0], r[1], 2 * r[2], r[3], r[4]] for r in costs], demand)),
     random_inputs=_rnd_dispatch,
     templates=["Economic dispatch: costs = [500 5.3 0.004 200 450]; [400 5.5 0.006 150 350]; [200 5.8 0.009 100 225], demand = 800 MW"],
     examples=[{"inputs": {"costs": "500 5.3 0.004 200 450; 400 5.5 0.006 150 350; 200 5.8 0.009 100 225", "demand": 800},
                "outputs": {"lambda_": 8.5, "P": [400.0, 250.0, 150.0]}},
               {"inputs": {"costs": [[0, 10, 0.01, 0, 500], [0, 10, 0.01, 0, 500]], "demand": 400}, "outputs": {"P": [200.0, 200.0], "lambda_": 14.0, "cost": 4800.0}}],
     questions=[{"q": "Economic dispatch: costs = [500 5.3 0.004 200 450]; [400 5.5 0.006 150 350]; [200 5.8 0.009 100 225], demand = 800 MW", "outputs": {"lambda_": 8.5, "P": [400.0, 250.0, 150.0]}},
                {"q": "Economic dispatch: costs = [500 5.3 0.004 200 450]; [400 5.5 0.006 150 350], demand = 900 MW", "outcome": "INVALID_INPUT"}])


# ================================================================================================ FAULTS
def _fault_network(network):
    edges = []
    for e in network:
        _need(len(e) == 3, "each branch is 'i-j x' with its reactance in per unit; node 0 is the reference")
        i, j, x = _bus_index(e[0]), _bus_index(e[1]), float(e[2])
        _need(i != j, "a branch must join two different nodes")
        _need(x > 0, "branch reactances must be positive")
        edges.append((i, j, x))
    nb = max(max(i, j) for i, j, _ in edges)
    _need({k for i, j, _ in edges for k in (i, j)} - {0} == set(range(1, nb + 1)), "buses must be numbered 1 to n without gaps")
    _need(any(0 in (i, j) for i, j, _ in edges), "at least one branch must connect to the reference node 0 (a source)")
    return edges, nb


def symmetrical_fault(network, fault_bus, Vf=1.0, Xf=0.0, S_base=None, V_base=None):
    edges, nb = _fault_network(network)
    _need(isinstance(fault_bus, int) and 1 <= fault_bus <= nb, f"the faulted bus must be one of 1..{nb}")
    _need(0.5 <= Vf <= 1.5, "the prefault voltage must lie between 0.5 and 1.5 pu")
    _nonneg("the fault reactance", Xf)
    Y = [[0j] * nb for _ in range(nb)]
    for i, j, x in edges:
        y = 1 / complex(0, x)
        if i:
            Y[i - 1][i - 1] += y
        if j:
            Y[j - 1][j - 1] += y
        if i and j:
            Y[i - 1][j - 1] -= y
            Y[j - 1][i - 1] -= y
    e = [0j] * nb
    e[fault_bus - 1] = 1 + 0j
    z = _gauss_solve(Y, e)                               # column of Zbus at the faulted bus
    zff = z[fault_bus - 1]
    If = Vf / (zff + complex(0, Xf))
    Vd = [abs(Vf - z[k] * If) for k in range(nb)]
    out = {"If": abs(If), "X_th": zff.imag, "V": Vd}
    if S_base is not None and V_base is not None:
        _pos("the base power and voltage", S_base, V_base)
        out["If_A"] = abs(If) * S_base / (SQ3 * V_base)
    return _out(**out)


def _ref_sym_fault(network, fault_bus, Vf=1.0, Xf=0.0, S_base=None, V_base=None):
    """The faulted network solved directly (numpy): every branch to node 0 is a source of EMF Vf behind its reactance,
    the faulted bus is tied to the reference through jXf (or to 0 V); node voltages from the nodal equations, the fault
    current as the current into the fault. No bus impedance matrix."""
    import numpy as np
    edges = [(int(e[0]), int(e[1]), float(e[2])) for e in network]
    nb = max(max(i, j) for i, j, _ in edges)
    Y = np.zeros((nb, nb), dtype=complex)
    I = np.zeros(nb, dtype=complex)
    for i, j, x in edges:
        y = 1 / complex(0, x)
        if i and j:
            Y[i - 1, i - 1] += y
            Y[j - 1, j - 1] += y
            Y[i - 1, j - 1] -= y
            Y[j - 1, i - 1] -= y
        else:
            k = (i or j) - 1
            Y[k, k] += y
            I[k] += Vf * y                              # Norton equivalent of the source
    f = fault_bus - 1
    if Xf > 0:
        Y2 = Y.copy()
        Y2[f, f] += 1 / complex(0, Xf)
        V = np.linalg.solve(Y2, I)
        If = abs(V[f] / complex(0, Xf))
    else:
        keep = [k for k in range(nb) if k != f]
        V = np.zeros(nb, dtype=complex)
        V[keep] = np.linalg.solve(Y[np.ix_(keep, keep)], I[keep])
        If = abs(I[f] - Y[f, :] @ V)
    Vpre = np.linalg.solve(Y, I)                        # prefault (no-load) voltages, all Vf
    Zth = Vpre[f] / complex(If, 0) if Xf == 0 else None
    out = {"If": float(If), "V": [float(abs(v)) for v in V], "X_th": float(abs(Vpre[f]) / If - Xf)}
    if S_base is not None and V_base is not None:
        out["If_A"] = float(If) * S_base / (SQ3 * V_base)
    return out


def _rnd_fault_net(rng):
    nb = rng.randint(2, 5)
    edges = _rnd_network(rng, nb, rng.randint(0, 2))
    branches = [f"{i}-{j} {round(rng.uniform(0.05, 0.4), 4)}" for i, j in edges]
    srcs = rng.sample(range(1, nb + 1), rng.randint(1, min(2, nb)))
    branches += [f"0-{s} {round(rng.uniform(0.1, 0.3), 4)}" for s in srcs]
    return ", ".join(branches), nb


def _rnd_symfault(rng):
    net, nb = _rnd_fault_net(rng)
    x = {"network": net, "fault_bus": rng.randint(1, nb), "Vf": round(rng.uniform(0.95, 1.05), 3), "Xf": rng.choice([0.0, round(rng.uniform(0.01, 0.2), 4)])}
    if rng.random() < 0.5:
        x.update({"S_base": 1e8, "V_base": rng.choice([11e3, 33e3, 132e3])})
    return x


def _parse_network(q, phrase, names, phys=(), words=()):
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", q or "", re.I)
    if not m:
        return None
    rest = q[m.end():]
    ml = re.search(r"\bnetwork\s*=\s*((?:\d+\s*-\s*\d+\s+[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?\s*;?\s*)+)", rest)
    if not ml:
        out = _pairs("x: " + rest.strip(), "x", names, phys=phys, words=words)
        return out
    net = ", ".join(p.strip() for p in ml.group(1).split(";") if p.strip())
    other = (rest[:ml.start()] + rest[ml.end():]).strip().strip(",").strip()
    out = _pairs("x: " + other, "x", names, phys=phys, words=words) if other else {}
    if out is None:
        return None
    out["network"] = net
    return out


_FAULT_NAMES = {"fault_bus": "fault_bus", "bus": "fault_bus", "Vf": "Vf", "V_prefault": "Vf", "Xf": "Xf", "S_base": "S_base", "V_base": "V_base"}

_add("energy_symmetrical_fault",
     name="Three-phase (symmetrical) fault current by the bus impedance matrix", topic="power systems / faults",
     purpose="The fault current, Thevenin reactance and the bus voltages during a balanced three-phase fault at a bus of a reactance network (per unit, prefault no-load voltage Vf everywhere; every branch to node 0 is a source).",
     inputs=[{"symbol": "network", "type": "edges", "required": True, "min_length": 1, "max_length": 40,
              "meaning": "branches as 'i-j x' with reactances in per unit; node 0 is the reference, each branch to 0 a source (generator or infinite bus)"},
             _intin("fault_bus", "faulted bus", 1, 99), _numin("Vf", "prefault voltage in per unit (default 1.0)", required=False),
             _numin("Xf", "fault reactance in per unit (default 0, a bolted fault)", required=False),
             _phys("S_base", "power", "base power, for the fault current in amperes (optional)", required=False),
             _phys("V_base", "voltage", "base line voltage at the faulted bus (optional)", required=False)],
     outputs=[_o("If", "number", "fault current in per unit"), _o("X_th", "number", "Thevenin reactance at the faulted bus (Z_ff) in per unit"),
              _o("V", "number", "bus voltage magnitudes during the fault (pu), buses 1..n"), _o("If_A", "current", "fault current in amperes")],
     primary="If", conditions=["reactance network", "no prefault load current (all prefault voltages Vf)"],
     method="column f of Zbus from Ybus z = e_f; I_f = V_f/(Z_ff + jX_f); V_i = V_f - Z_if I_f",
     latex=r"I_f = \frac{V_f}{Z_{ff} + Z_f},\quad V_i = V_f - Z_{if}I_f",
     run=symmetrical_fault,
     parse=lambda q: _parse_network(q, r"symmetrical fault|three[- ]phase fault", _FAULT_NAMES, phys={"S_base", "V_base"}),
     reference=_ref_sym_fault,
     wrong=_safe(lambda network, fault_bus, Vf=1.0, Xf=0.0, S_base=None, V_base=None: (lambda o: dict(o, If=_r(o["If"] * (1 + 0.5 / (1 + o["X_th"]))),
                                                                                                  X_th=_r(o["X_th"] / (1 + 0.5 / (1 + o["X_th"])))))(
         symmetrical_fault(network, fault_bus, Vf, Xf, S_base, V_base))),
     random_inputs=_rnd_symfault,
     templates=["Symmetrical fault: network = 0-1 0.2; 0-2 0.15; 1-2 0.1; 2-3 0.25, fault_bus = 3"],
     examples=[{"inputs": {"network": "0-1 0.2, 1-2 0.3", "fault_bus": 2}, "outputs": {"If": 2.0, "X_th": 0.5, "V": [0.6, 0.0]}},
               {"inputs": {"network": "0-1 0.1", "fault_bus": 1, "S_base": 1e8, "V_base": 1e5 / SQ3 * SQ3}, "outputs": {"If": 10.0, "If_A": _r(10 * 1e8 / (SQ3 * 1e5))}}],
     questions=[{"q": "Symmetrical fault: network = 0-1 0.2; 1-2 0.3, fault_bus = 2", "outputs": {"If": 2.0, "V": [0.6, 0.0]}},
                {"q": "Three-phase fault: network = 0-1 0.1, bus = 1, S_base = 100 MVA, V_base = 100 kV", "outputs": {"If_A": _r(10 * 1e8 / (SQ3 * 1e5))}},
                {"q": "Symmetrical fault: network = 0-1 0.2; 1-2 0.3, fault_bus = 5", "outcome": "INVALID_INPUT"}])

_A = cmath.rect(1.0, 2 * math.pi / 3)


def _seq_check(X1, X2, X0, Rf, Xf, Vf):
    _pos("the sequence reactances", X1, X2, X0)
    _nonneg("the fault resistance and reactance", Rf, Xf)
    _need(0.5 <= Vf <= 1.5, "the prefault voltage must lie between 0.5 and 1.5 pu")


def _phase_from_seq(I0, I1, I2):
    return (I0 + I1 + I2, I0 + _A * _A * I1 + _A * I2, I0 + _A * I1 + _A * _A * I2)


def _snap0(x, scale=1.0):
    return 0.0 if abs(x) <= 1e-12 * max(scale, 1e-300) else x


def unsym_fault(kind, X1, X2, X0, Rf=0.0, Xf=0.0, Vf=1.0):
    _seq_check(X1, X2, X0, Rf, Xf, Vf)
    Z1, Z2, Z0, Zf = complex(0, X1), complex(0, X2), complex(0, X0), complex(Rf, Xf)
    if kind == "slg":
        I1 = Vf / (Z1 + Z2 + Z0 + 3 * Zf)
        I2, I0 = I1, I1
    elif kind == "ll":
        I1 = Vf / (Z1 + Z2 + Zf)
        I2, I0 = -I1, 0j
    else:                                                # dlg
        Zp = Z2 * (Z0 + 3 * Zf) / (Z2 + Z0 + 3 * Zf)
        I1 = Vf / (Z1 + Zp)
        I2 = -I1 * (Z0 + 3 * Zf) / (Z2 + Z0 + 3 * Zf)
        I0 = -I1 * Z2 / (Z2 + Z0 + 3 * Zf)
    V1, V2, V0 = Vf - Z1 * I1, -Z2 * I2, -Z0 * I0
    Ia, Ib, Ic = _phase_from_seq(I0, I1, I2)
    Va, Vb, Vc = _phase_from_seq(V0, V1, V2)
    s = abs(I1)
    return _out(Ia=_snap0(abs(Ia), s), Ib=_snap0(abs(Ib), s), Ic=_snap0(abs(Ic), s), In=_snap0(abs(3 * I0), s), I1=abs(I1), I2=_snap0(abs(I2), s), I0=_snap0(abs(I0), s),
                Va=_snap0(abs(Va), Vf), Vb=_snap0(abs(Vb), Vf), Vc=_snap0(abs(Vc), Vf))


def _ref_unsym(kind, X1, X2, X0, Rf=0.0, Xf=0.0, Vf=1.0):
    """Phase-domain solution (numpy): the source E_abc = Vf (1, a^2, a) behind the phase impedance matrix
    Z_abc = A diag(Z0, Z1, Z2) A^-1, with the fault's boundary conditions written on phase currents and voltages and
    the 6 x 6 linear system solved directly. Sequence quantities only from transforming the phase result."""
    import numpy as np
    a = np.exp(2j * np.pi / 3)
    A = np.array([[1, 1, 1], [1, a * a, a], [1, a, a * a]])
    Zabc = A @ np.diag([1j * X0, 1j * X1, 1j * X2]) @ np.linalg.inv(A)
    E = Vf * np.array([1, a * a, a])
    Zf = complex(Rf, Xf)
    M = np.zeros((6, 6), dtype=complex)
    rhs = np.zeros(6, dtype=complex)
    # unknowns x = [Va, Vb, Vc, Ia, Ib, Ic]; source: V + Zabc I = E
    M[:3, :3] = np.eye(3)
    M[:3, 3:] = Zabc
    rhs[:3] = E
    if kind == "slg":                      # Ib = Ic = 0, Va = Zf Ia
        M[3, 4] = 1
        M[4, 5] = 1
        M[5, 0], M[5, 3] = 1, -Zf
    elif kind == "ll":                     # Ia = 0, Ib + Ic = 0, Vb - Vc = Zf Ib
        M[3, 3] = 1
        M[4, 4], M[4, 5] = 1, 1
        M[5, 1], M[5, 2], M[5, 4] = 1, -1, -Zf
    else:                                  # Ia = 0, Vb = Zf (Ib + Ic), Vc = Zf (Ib + Ic)
        M[3, 3] = 1
        M[4, 1], M[4, 4], M[4, 5] = 1, -Zf, -Zf
        M[5, 2], M[5, 4], M[5, 5] = 1, -Zf, -Zf
    x = np.linalg.solve(M, rhs)
    V, I = x[:3], x[3:]
    seq = np.linalg.inv(A) @ I
    s = abs(seq[1])
    sn = lambda v, sc: 0.0 if abs(v) <= 1e-12 * sc else float(abs(v))   # noqa: E731
    return {"Ia": sn(I[0], s), "Ib": sn(I[1], s), "Ic": sn(I[2], s), "In": sn(I.sum(), s), "I1": float(abs(seq[1])), "I2": sn(seq[2], s), "I0": sn(seq[0], s),
            "Va": sn(V[0], Vf), "Vb": sn(V[1], Vf), "Vc": sn(V[2], Vf)}


_SEQ_IN = [_numin("X1", "positive-sequence Thevenin reactance (pu)"), _numin("X2", "negative-sequence reactance (pu)"), _numin("X0", "zero-sequence reactance (pu)"),
           _numin("Rf", "fault resistance (pu, default 0)", required=False), _numin("Xf", "fault reactance (pu, default 0)", required=False),
           _numin("Vf", "prefault voltage (pu, default 1.0)", required=False)]
_SEQ_OUT = [_o("Ia", "number", "|I_a| (pu)"), _o("Ib", "number", "|I_b| (pu)"), _o("Ic", "number", "|I_c| (pu)"), _o("In", "number", "neutral (earth) current |3 I_0| (pu)"),
            _o("I1", "number", "|I_1| (pu)"), _o("I2", "number", "|I_2| (pu)"), _o("I0", "number", "|I_0| (pu)"), _o("Va", "number", "|V_a| (pu)"),
            _o("Vb", "number", "|V_b| (pu)"), _o("Vc", "number", "|V_c| (pu)")]
_SEQ_NAMES = {"X1": "X1", "X2": "X2", "X0": "X0", "Rf": "Rf", "Xf": "Xf", "Vf": "Vf"}


def _rnd_seq(rng):
    x1 = round(rng.uniform(0.05, 0.4), 4)
    return {"X1": x1, "X2": round(x1 * rng.uniform(0.8, 1.2), 4), "X0": round(rng.uniform(0.02, 0.6), 4), "Rf": rng.choice([0.0, round(rng.uniform(0.0, 0.2), 4)]),
            "Xf": rng.choice([0.0, round(rng.uniform(0.0, 0.1), 4)]), "Vf": round(rng.uniform(0.95, 1.05), 3)}


for _key, _kind, _name, _purpose, _method, _latex, _primary, _wrongf, _ex in [
    ("energy_slg_fault", "slg", "Single line-to-ground fault (symmetrical components)",
     "Phase and sequence currents and phase voltages for a fault from phase a to ground through Zf: the three sequence networks in series.",
     "I1 = I2 = I0 = Vf/(Z1 + Z2 + Z0 + 3 Zf); I_a = 3 I_0", r"I_a = 3I_0 = \frac{3V_f}{Z_1 + Z_2 + Z_0 + 3Z_f}", "Ia",
     lambda X1, X2, X0, Rf=0.0, Xf=0.0, Vf=1.0: (lambda o: dict(o, Ia=o["I1"], In=o["I1"]))(unsym_fault("slg", X1, X2, X0, Rf, Xf, Vf)),
     ({"X1": 0.1, "X2": 0.1, "X0": 0.1}, {"Ia": 10.0, "I1": _r(10 / 3), "Va": 0.0})),
    ("energy_ll_fault", "ll", "Line-to-line fault (symmetrical components)",
     "Phase and sequence currents and phase voltages for a fault between phases b and c through Zf: positive and negative networks in parallel opposition.",
     "I1 = -I2 = Vf/(Z1 + Z2 + Zf), I0 = 0; |I_b| = sqrt(3) |I_1|", r"I_b = -I_c = -j\sqrt3\,\frac{V_f}{Z_1 + Z_2 + Z_f}", "Ib",
     lambda X1, X2, X0, Rf=0.0, Xf=0.0, Vf=1.0: (lambda o: dict(o, Ib=o["I1"], Ic=o["I1"]))(unsym_fault("ll", X1, X2, X0, Rf, Xf, Vf)),
     ({"X1": 0.1, "X2": 0.1, "X0": 0.1}, {"Ib": _r(5 * SQ3), "I1": 5.0, "Va": 1.0, "Vb": 0.5})),
    ("energy_dlg_fault", "dlg", "Double line-to-ground fault (symmetrical components)",
     "Phase, neutral and sequence currents and phase voltages for a fault from phases b and c to ground through Zf: negative and (zero + 3Zf) networks in parallel.",
     "I1 = Vf/(Z1 + Z2 || (Z0 + 3Zf)); I2 = -I1 (Z0 + 3Zf)/(Z2 + Z0 + 3Zf); I0 = -I1 Z2/(Z2 + Z0 + 3Zf)",
     r"I_1 = \frac{V_f}{Z_1 + \frac{Z_2(Z_0 + 3Z_f)}{Z_2 + Z_0 + 3Z_f}}", "Ib",
     lambda X1, X2, X0, Rf=0.0, Xf=0.0, Vf=1.0: (lambda o: dict(o, Ib=_r(o["Ib"] * 1.5), Ic=_r(o["Ic"] * 1.5)))(unsym_fault("slg", X1, X2, X0, Rf, Xf, Vf)),
     ({"X1": 0.1, "X2": 0.1, "X0": 0.1}, {"I1": _r(1 / 0.15), "In": _r(3 / 0.3), "Va": 1.0}))]:
    _phr = {"slg": r"single line[- ]to[- ]ground fault|slg fault|line[- ]to[- ]ground fault",
            "ll": r"line[- ]to[- ]line fault|ll fault", "dlg": r"double line[- ]to[- ]ground fault|dlg fault"}[_kind]
    _add(_key, name=_name, topic="power systems / faults", purpose=_purpose, inputs=_SEQ_IN, outputs=_SEQ_OUT, primary=_primary,
         conditions=["sequence networks from the Thevenin reactances at the fault", "prefault no-load voltage Vf"],
         method=_method, latex=_latex,
         run=(lambda kk: (lambda X1, X2, X0, Rf=0.0, Xf=0.0, Vf=1.0: unsym_fault(kk, X1, X2, X0, Rf, Xf, Vf)))(_kind),
         parse=(lambda ph: (lambda q: _pairs(q, ph, _SEQ_NAMES)))(_phr),
         reference=(lambda kk: (lambda X1, X2, X0, Rf=0.0, Xf=0.0, Vf=1.0: _ref_unsym(kk, X1, X2, X0, Rf, Xf, Vf)))(_kind),
         wrong=_safe(_wrongf), random_inputs=_rnd_seq,
         templates=[{"slg": "Single line-to-ground fault: X1 = 0.25, X2 = 0.25, X0 = 0.35",
                     "ll": "Line-to-line fault: X1 = 0.25, X2 = 0.25, X0 = 0.35",
                     "dlg": "Double line-to-ground fault: X1 = 0.25, X2 = 0.25, X0 = 0.35"}[_kind]],
         examples=[{"inputs": _ex[0], "outputs": _ex[1]}],
         questions=[{"q": {"slg": "SLG fault", "ll": "LL fault", "dlg": "DLG fault"}[_kind] + ": X1 = 0.1, X2 = 0.1, X0 = 0.1", "outputs": _ex[1]},
                    {"q": {"slg": "SLG fault", "ll": "LL fault", "dlg": "DLG fault"}[_kind] + ": X1 = 0.1, X2 = 0.1, X0 = -0.1", "outcome": "INVALID_INPUT"},
                    {"q": {"slg": "SLG fault", "ll": "LL fault", "dlg": "DLG fault"}[_kind] + ": X1 = 0.1, X2 = 0.1", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ STABILITY AND CONTROL
def power_angle(V1, V2, X, P):
    _pos("the voltages and the reactance", V1, V2, X)
    Pmax = V1 * V2 / X
    _need(abs(P) < Pmax, "the power exceeds the steady-state limit V1 V2/X")
    d = math.asin(P / Pmax)
    return _out(delta_deg=math.degrees(d), Pmax=Pmax, Q_sent=(V2 * V2 - V1 * V2 * math.cos(d)) / X, Q_received=(V1 * V2 * math.cos(d) - V1 * V1) / X,
                margin_pct=100 * (Pmax - P) / Pmax)


def _ref_power_angle(V1, V2, X, P):
    """pandapower: bus 1 an infinite bus (slack) at V1, bus 2 a voltage-controlled generator at V2 delivering P over a
    lossless line of reactance X; the steady-state limit from a second load flow sweep is replaced by the transfer at
    90 degrees computed from the solved network's admittance (V1 V2 |Y12|)."""
    import pandapower as pp
    net = _pp_empty(sn_mva=1.0)
    b1 = pp.create_bus(net, vn_kv=V1 / 1e3)
    b2 = pp.create_bus(net, vn_kv=V1 / 1e3)
    pp.create_ext_grid(net, b1, vm_pu=1.0)
    pp.create_line_from_parameters(net, b1, b2, 1.0, 0.0, X, 0.0, 1e9)
    pp.create_gen(net, b2, p_mw=P / 1e6, vm_pu=V2 / V1)
    pp.runpp(net, tolerance_mva=1e-12 * max(1.0, P / 1e6), max_iteration=60, numba=False, check_connectivity=False)
    Y = net._ppc["internal"]["Ybus"].toarray() if "internal" in net._ppc else None
    zb = (V1 / 1e3) ** 2 / 1.0
    y12 = abs(Y[0, 1]) / zb if Y is not None else 1 / X
    return {"delta_deg": float(net.res_bus.va_degree.iloc[1]), "Q_sent": _pp_injections(net)[1].imag * 1e6,
            "Q_received": -float(net.res_ext_grid.q_mvar.iloc[0]) * 1e6, "Pmax": V1 * V2 * y12, "margin_pct": 100 * (V1 * V2 * y12 - P) / (V1 * V2 * y12)}


_add("energy_power_angle",
     name="Power transfer and power angle between two voltages", topic="power systems / stability",
     purpose="The power angle needed to transfer P over a lossless reactance X between a sending voltage V2 (generator) and a receiving voltage V1 (infinite bus), the steady-state limit V1 V2/X, the reactive power at both ends and the stability margin.",
     inputs=[_phys("V1", "voltage", "receiving (infinite-bus) line voltage"), _phys("V2", "voltage", "sending (generator) line voltage"),
             _phys("X", "resistance", "transfer reactance per phase"), _phys("P", "power", "real power transferred (three-phase)")],
     outputs=[_o("delta_deg", "number", "power angle (V2 ahead of V1) in degrees"), _o("Pmax", "power", "steady-state limit V1 V2/X"),
              _o("Q_sent", "power", "reactive power sent from the generator end in var"), _o("Q_received", "power", "reactive power arriving at the infinite bus in var"),
              _o("margin_pct", "number", "stability margin (Pmax - P)/Pmax in percent")],
     primary="delta_deg", conditions=["lossless transfer reactance", "|P| < V1 V2/X"],
     method="P = V1 V2 sin(delta)/X; Q_sent = (V2^2 - V1 V2 cos delta)/X; Q_received = (V1 V2 cos delta - V1^2)/X",
     latex=r"P = \frac{V_1V_2}{X}\sin\delta,\quad P_{max} = \frac{V_1V_2}{X}",
     run=power_angle,
     parse=lambda q: _pairs(q, r"power angle|power transfer angle|steady[- ]state stability limit", {"V1": "V1", "V2": "V2", "E": "V2", "V": "V1", "X": "X", "P": "P"}, phys={"V1", "V2", "X", "P"}),
     reference=_ref_power_angle,
     wrong=_safe(lambda V1, V2, X, P: (lambda o: dict(o, delta_deg=_r(math.degrees(math.acos(min(1, P * X / (V1 * V2)))))))(power_angle(V1, V2, X, P))),
     random_inputs=lambda rng: (lambda V, X: (lambda V2: {"V1": V, "V2": V2, "X": X, "P": round(V * V2 / X * rng.uniform(0.05, 0.9), 0)})(round(V * rng.uniform(0.95, 1.1), 0)))(
         rng.choice([11e3, 33e3, 132e3, 400e3]), round(rng.uniform(5, 100), 3)),
     templates=["Power angle: V1 = 132 kV, V2 = 138 kV, X = 50 ohm, P = 150 MW"],
     examples=[{"inputs": {"V1": 1e5, "V2": 1e5, "X": 100.0, "P": 5e7}, "outputs": {"delta_deg": 30.0, "Pmax": 1e8}}],
     questions=[{"q": "Power angle: V1 = 100 kV, V2 = 100 kV, X = 100 ohm, P = 50 MW", "outputs": {"delta_deg": 30.0, "Pmax": 1e8}},
                {"q": "Power angle: V1 = 100 kV, V2 = 100 kV, X = 100 ohm, P = 150 MW", "outcome": "INVALID_INPUT"}])


def critical_clearing(Pm, Pmax_pre, Pmax_fault, Pmax_post, H=None, f=None):
    _pos("the mechanical power and the pre- and post-fault limits", Pm, Pmax_pre, Pmax_post)
    _nonneg("the during-fault limit", Pmax_fault)
    _need(Pm < Pmax_pre and Pm < Pmax_post, "the mechanical power must be below the pre- and post-fault transfer limits")
    _need(Pmax_fault < Pm, "the during-fault transfer limit must be below the mechanical power (otherwise the rotor does not accelerate)")
    d0 = math.asin(Pm / Pmax_pre)
    dm = math.pi - math.asin(Pm / Pmax_post)
    _need(dm > d0, "the post-fault network cannot hold the rotor")
    num = Pm * (dm - d0) + Pmax_post * math.cos(dm) - Pmax_fault * math.cos(d0)
    cc = num / (Pmax_post - Pmax_fault)
    _need(-1 < cc < 1, "no clearing angle keeps the system stable (the decelerating area is too small)")
    dc = math.acos(cc)
    out = {"delta0_deg": math.degrees(d0), "delta_max_deg": math.degrees(dm), "delta_cr_deg": math.degrees(dc)}
    if H is not None or f is not None:
        _need(H is not None and f is not None, "the clearing time needs both H and f")
        _pos("H and f", H, f)
        _need(Pmax_fault == 0, "a closed-form clearing time exists only for a fault with zero transfer (Pmax_fault = 0)")
        out["t_cr"] = math.sqrt(4 * H * (dc - d0) / (2 * math.pi * f * Pm))
    return _out(**out)


def _ref_ccl(Pm, Pmax_pre, Pmax_fault, Pmax_post, H=None, f=None):
    """Equal areas found numerically: the accelerating and decelerating areas by scipy quadrature and their balance
    by brentq; the clearing time by integrating the swing equation (DOP853) and locating the clearing angle (brentq on
    the dense output)."""
    from scipy import integrate, optimize
    d0 = optimize.brentq(lambda d: Pmax_pre * math.sin(d) - Pm, 0, math.pi / 2, xtol=1e-15, rtol=8.9e-16)
    dm = optimize.brentq(lambda d: Pmax_post * math.sin(d) - Pm, math.pi / 2, math.pi, xtol=1e-15, rtol=8.9e-16)

    def bal(dc):
        a1, _ = integrate.quad(lambda d: Pm - Pmax_fault * math.sin(d), d0, dc, epsabs=1e-16, epsrel=1e-13)
        a2, _ = integrate.quad(lambda d: Pmax_post * math.sin(d) - Pm, dc, dm, epsabs=1e-16, epsrel=1e-13)
        return a1 - a2
    dc = optimize.brentq(bal, d0, dm, xtol=1e-15, rtol=8.9e-16)
    out = {"delta0_deg": math.degrees(d0), "delta_max_deg": math.degrees(dm), "delta_cr_deg": math.degrees(dc)}
    if H is not None:
        ws = 2 * math.pi * f
        sol = integrate.solve_ivp(lambda t, y: [y[1], ws / (2 * H) * Pm], (0, 10), [d0, 0.0], method="DOP853", rtol=1e-13, atol=1e-15, dense_output=True)
        tc = optimize.brentq(lambda t: sol.sol(t)[0] - dc, 0, 10, xtol=1e-15, rtol=8.9e-16)
        out["t_cr"] = tc
    return out


def _rnd_ccl(rng):
    for _ in range(200):
        pre = round(rng.uniform(1.5, 3), 3)
        post = round(pre * rng.uniform(0.6, 1.0), 3)
        pm = round(min(pre, post) * rng.uniform(0.3, 0.7), 3)
        flt = rng.choice([0.0, round(pm * rng.uniform(0, 0.8), 3)])
        x = {"Pm": pm, "Pmax_pre": pre, "Pmax_fault": flt, "Pmax_post": post}
        if flt == 0 and rng.random() < 0.7:
            x.update({"H": round(rng.uniform(2, 8), 2), "f": rng.choice([50.0, 60.0])})
        try:
            critical_clearing(**x)
            return x
        except ValueError:
            continue
    return x


_add("energy_critical_clearing",
     name="Critical clearing angle and time (equal-area criterion)", topic="power systems / transient stability",
     purpose="The initial rotor angle, the largest allowable angle and the critical clearing angle of a machine against an infinite bus by the equal-area criterion, for given pre-fault, during-fault and post-fault transfer limits; with H and f, the critical clearing time for a fault that blocks all transfer.",
     inputs=[_numin("Pm", "mechanical input power (pu)"), _numin("Pmax_pre", "pre-fault transfer limit (pu)"), _numin("Pmax_fault", "transfer limit during the fault (pu; 0 for a fault at the terminals)"),
             _numin("Pmax_post", "post-fault transfer limit (pu)"), _phys("H", "time", "inertia constant H (MJ/MVA, i.e. seconds), for the clearing time", required=False),
             _phys("f", "frequency", "system frequency, for the clearing time", required=False)],
     outputs=[_o("delta0_deg", "number", "initial rotor angle in degrees"), _o("delta_max_deg", "number", "largest angle before loss of synchronism in degrees"),
              _o("delta_cr_deg", "number", "critical clearing angle in degrees"), _o("t_cr", "time", "critical clearing time (Pmax_fault = 0)")],
     primary="delta_cr_deg", conditions=["Pmax_fault < Pm < Pmax_pre, Pmax_post", "classical model (constant voltage behind transient reactance, no damping)"],
     method="cos(delta_cr) = [Pm (delta_max - delta0) + Pmax_post cos(delta_max) - Pmax_fault cos(delta0)]/(Pmax_post - Pmax_fault); t_cr = sqrt(4 H (delta_cr - delta0)/(omega_s Pm))",
     latex=r"\cos\delta_{cr} = \frac{P_m(\delta_{max}-\delta_0) + P_{max,3}\cos\delta_{max} - P_{max,2}\cos\delta_0}{P_{max,3} - P_{max,2}}",
     run=critical_clearing,
     parse=lambda q: _pairs(q, r"critical clearing(?: angle| time)?|equal[- ]area criterion", {"Pm": "Pm", "Pmax_pre": "Pmax_pre", "Pmax_fault": "Pmax_fault", "Pmax_post": "Pmax_post",
                                                                                            "H": "H", "f": "f"}, phys={"H", "f"}),
     reference=_ref_ccl,
     wrong=_safe(lambda Pm, Pmax_pre, Pmax_fault, Pmax_post, H=None, f=None: (lambda o: dict(o, delta_cr_deg=_r(0.5 * (o["delta0_deg"] + o["delta_max_deg"])),
                                                                                              **({"t_cr": _r(o["t_cr"] * 1.3)} if "t_cr" in o else {})))(
         critical_clearing(Pm, Pmax_pre, Pmax_fault, Pmax_post, H, f))),
     random_inputs=_rnd_ccl,
     templates=["Critical clearing: Pm = 1, Pmax_pre = 2, Pmax_fault = 0, Pmax_post = 2, H = 5 s, f = 50 Hz"],
     examples=[{"inputs": {"Pm": 1.0, "Pmax_pre": 2.0, "Pmax_fault": 0.0, "Pmax_post": 2.0},
                "outputs": {"delta0_deg": 30.0, "delta_max_deg": 150.0, "delta_cr_deg": _r(math.degrees(math.acos((2 * math.pi / 3 + 2 * math.cos(5 * math.pi / 6)) / 2)))}}],
     questions=[{"q": "Critical clearing: Pm = 1, Pmax_pre = 2, Pmax_fault = 0, Pmax_post = 2", "outputs": {"delta0_deg": 30.0, "delta_max_deg": 150.0}},
                {"q": "Critical clearing: Pm = 1, Pmax_pre = 2, Pmax_fault = 1.5, Pmax_post = 2", "outcome": "INVALID_INPUT"}])


def droop_sharing(R, ratings, dP, f0, D=0.0):
    _need(len(R) == len(ratings) and len(R) >= 1, "give one droop and one rating per generator")
    for r, s in zip(R, ratings):
        _need(r > 0 and s > 0, "droops and ratings must be positive")
    _pos("the frequency", f0)
    _nonneg("the load damping D", D)
    beta = sum(s / (r * f0) for r, s in zip(R, ratings)) + D
    df = -dP / beta
    return _out(df=df, f=f0 + df, dP_gen=[-df * s / (r * f0) for r, s in zip(R, ratings)], dP_load_damping=-D * df)


def _ref_droop(R, ratings, dP, f0, D=0.0):
    """Steady state of the linear load-frequency dynamic model (swing equation, first-order governors and turbines)
    found by solving A x + b = 0 for every state (numpy), not from the closed-form droop characteristic."""
    import numpy as np
    n = len(R)
    M, Tg, Tt = 10.0, 0.2, 0.5
    N = 1 + 2 * n
    A = np.zeros((N, N))
    b = np.zeros(N)
    A[0, 0] = -D / M
    b[0] = -dP / M
    for i in range(n):
        A[0, 1 + i] = 1 / M                         # mechanical powers drive the frequency
        A[1 + i, 1 + i] = -1 / Tt
        A[1 + i, 1 + n + i] = 1 / Tt                # turbine follows the valve
        A[1 + n + i, 1 + n + i] = -1 / Tg
        A[1 + n + i, 0] = -ratings[i] / (R[i] * f0) / Tg
    x = np.linalg.solve(A, -b)
    df = float(x[0])
    return {"df": df, "f": f0 + df, "dP_gen": [float(v) for v in x[1:1 + n]], "dP_load_damping": -D * df}


_add("energy_droop_sharing",
     name="Frequency deviation and load sharing under governor droop", topic="power systems / frequency control",
     purpose="The steady-state frequency drop after a load step and each generator's output change when governors with droop R (per unit on their own rating) share the load, with optional load damping D.",
     inputs=[_vecin("R", "droop of each governor in per unit on its own rating (e.g. 0.05)", 1, 20), _vecin("ratings", "rating of each generator in MW", 1, 20),
             _numin("dP", "load increase in MW (negative for a load decrease)"), _phys("f0", "frequency", "nominal frequency"),
             _numin("D", "load damping in MW/Hz (default 0)", required=False)],
     outputs=[_o("df", "number", "frequency change in Hz"), _o("f", "number", "new frequency in Hz"), _o("dP_gen", "number", "output change of each generator in MW"),
              _o("dP_load_damping", "number", "load relief from damping in MW")],
     primary="df", conditions=["primary control only (no secondary/AGC)", "R, ratings > 0"],
     method="beta = sum(S_i/(R_i f0)) + D; df = -dP/beta; dP_i = -df S_i/(R_i f0)", latex=r"\Delta f = -\frac{\Delta P_L}{\sum_i \frac{S_i}{R_i f_0} + D}",
     run=droop_sharing,
     parse=lambda q: _pairs(q, r"droop (?:load )?sharing|frequency droop|governor droop", {"R": "R", "droops": "R", "ratings": "ratings", "S": "ratings", "dP": "dP", "load_step": "dP",
                                                                                         "f0": "f0", "f": "f0", "D": "D"},
                            phys={"f0"}, vectors={"R", "ratings"}, num_units={"dP": ("MW",), "D": ("MW/Hz",)}),
     reference=_ref_droop,
     wrong=_safe(lambda R, ratings, dP, f0, D=0.0: (lambda df: _out(df=df, f=f0 + df, dP_gen=[dP / len(R)] * len(R), dP_load_damping=-D * df))(-dP * f0 / sum(1 / r for r in R) / 100)),
     random_inputs=lambda rng: (lambda n: {"R": [round(rng.uniform(0.03, 0.08), 4) for _ in range(n)], "ratings": [round(rng.uniform(50, 800), 0) for _ in range(n)],
                                           "dP": round(rng.uniform(-200, 300), 1) or 50.0, "f0": rng.choice([50.0, 60.0]), "D": rng.choice([0.0, round(rng.uniform(1, 30), 2)])})(rng.randint(1, 4)),
     templates=["Droop sharing: R = [0.05, 0.04], ratings = [500, 300], dP = 100 MW, f0 = 50 Hz"],
     examples=[{"inputs": {"R": [0.05, 0.05], "ratings": [500, 500], "dP": 100, "f0": 50}, "outputs": {"df": -0.25, "dP_gen": [50.0, 50.0]}}],
     questions=[{"q": "Droop sharing: R = [0.05, 0.05], ratings = [500, 500], dP = 100 MW, f0 = 50 Hz", "outputs": {"df": -0.25}},
                {"q": "Droop sharing: R = [0.05, 0.05], ratings = [500], dP = 100 MW, f0 = 50 Hz", "outcome": "INVALID_INPUT"}])


# ================================================================================================ TRANSFORMERS
def transformer_performance(S, V1, V2, Req, Xeq, Rc, Xm, load, pf, pf_type="lagging"):
    _pos("the rating and the rated voltages", S, V1, V2)
    _nonneg("the series resistance and reactance", Req, Xeq)
    _pos("the core-loss resistance and the magnetizing reactance", Rc, Xm)
    _need(0 < load <= 2, "the load must lie between 0 and 2 times the rating (a fraction, or a percentage)")
    phi = _pf_angle(pf, pf_type)
    a = V1 / V2
    I2p = cmath.rect(load * S / V2 / a, -phi)
    V1v = a * V2 + I2p * complex(Req, Xeq)
    Ish = V1v * (1 / Rc + 1 / complex(0, Xm))
    I1 = I2p + Ish
    Pout = load * S * pf
    Pcu = abs(I2p) ** 2 * Req
    Pcore = abs(V1v) ** 2 / Rc
    return _out(V1_required=abs(V1v), regulation_pct=100 * (abs(V1v) / a - V2) / V2, I1=abs(I1), pf_in=math.cos(cmath.phase(V1v) - cmath.phase(I1)),
                P_out=Pout, P_cu=Pcu, P_core=Pcore, efficiency_pct=100 * Pout / (Pout + Pcu + Pcore))


def _ref_transformer_performance(S, V1, V2, Req, Xeq, Rc, Xm, load, pf, pf_type="lagging"):
    """The approximate equivalent circuit solved as a network (numpy): a unit source at the primary, the shunt branch
    there, the series impedance and the load as an impedance (referred to the primary); the linear solution scaled
    to rated secondary voltage; losses and efficiency from the solved branch currents."""
    import numpy as np
    a = V1 / V2
    phi = math.acos(pf) * (1 if pf_type != "leading" else -1)
    Sl = cmath.rect(load * S, phi)
    ZL = (a * V2) ** 2 / np.conj(Sl)
    Z = complex(Req, Xeq)
    Ysh = 1 / Rc + 1 / complex(0, Xm)
    V2u = np.linalg.solve(np.array([[1 / Z + 1 / ZL]]), np.array([1 / Z]))[0]
    k = a * V2 / abs(V2u)
    V1v, V2v = k, k * V2u
    Iser = (V1v - V2v) / Z
    I1 = V1v * Ysh + Iser
    Pout = float((V2v * np.conj(V2v / ZL)).real)
    Pcu = abs(Iser) ** 2 * Req
    Pcore = abs(V1v) ** 2 / Rc
    return {"V1_required": abs(V1v), "regulation_pct": 100 * (abs(V1v) / a - V2) / V2, "I1": abs(I1), "pf_in": math.cos(-cmath.phase(complex(I1))),
            "P_out": Pout, "P_cu": Pcu, "P_core": Pcore, "efficiency_pct": 100 * Pout / (Pout + Pcu + Pcore)}


_XF_NAMES = {"S": "S", "rating": "S", "V1": "V1", "V_primary": "V1", "V2": "V2", "V_secondary": "V2", "Req": "Req", "Xeq": "Xeq", "Rc": "Rc", "Xm": "Xm",
             "load": "load", "pf": "pf", "pf_type": "pf_type"}

_add("energy_transformer_performance",
     name="Transformer regulation and efficiency from its equivalent circuit", topic="transformers / equivalent circuit",
     purpose="Primary voltage needed, voltage regulation, primary current and input power factor, copper and core losses and efficiency of a single-phase transformer (approximate equivalent circuit referred to the primary, shunt branch at the primary terminals) delivering a fraction of its rating at rated secondary voltage.",
     inputs=[_phys("S", "power", "rating in VA"), _phys("V1", "voltage", "rated primary voltage"), _phys("V2", "voltage", "rated secondary voltage"),
             _phys("Req", "resistance", "series resistance referred to the primary"), _phys("Xeq", "resistance", "series leakage reactance referred to the primary"),
             _phys("Rc", "resistance", "core-loss resistance (primary side)"), _phys("Xm", "resistance", "magnetizing reactance (primary side)"),
             _numin("load", "load as a fraction of the rating (or a percentage)"), _numin("pf", "load power factor"),
             _choice("pf_type", ["lagging", "leading"], "lagging (default) or leading load", required=False)],
     outputs=[_o("V1_required", "voltage", "primary voltage for rated secondary voltage at this load"), _o("regulation_pct", "number", "voltage regulation in percent"),
              _o("I1", "current", "primary current"), _o("pf_in", "number", "input power factor"), _o("P_out", "power", "output power"),
              _o("P_cu", "power", "copper loss"), _o("P_core", "power", "core loss"), _o("efficiency_pct", "number", "efficiency in percent")],
     primary="regulation_pct", conditions=["approximate equivalent circuit (shunt branch at the primary terminals)", "0 < load <= 2"],
     method="V1 = a V2 + I2' (Req + jXeq), I2' = I2/a at angle -acos(pf); P_cu = |I2'|^2 Req; P_core = V1^2/Rc; eta = P_out/(P_out + P_cu + P_core)",
     latex=r"\mathbf V_1 = a\mathbf V_2 + \frac{\mathbf I_2}{a}(R_{eq} + jX_{eq}),\quad \eta = \frac{P_{out}}{P_{out} + P_{cu} + P_{core}}",
     run=transformer_performance,
     parse=lambda q: _pairs(q, r"transformer (?:regulation and efficiency|performance|equivalent circuit)", _XF_NAMES, phys={"S", "V1", "V2", "Req", "Xeq", "Rc", "Xm"},
                            words={"pf_type"}, num_units=_pct_units("load", "pf")),
     reference=_ref_transformer_performance,
     wrong=_safe(lambda S, V1, V2, Req, Xeq, Rc, Xm, load, pf, pf_type="lagging": (lambda o: dict(o, efficiency_pct=_r(100 * o["P_out"] / (o["P_out"] + o["P_cu"]))))(
         transformer_performance(S, V1, V2, Req, Xeq, Rc, Xm, load, pf, pf_type))),
     random_inputs=lambda rng: (lambda S, V1, V2: (lambda zb: {"S": S, "V1": V1, "V2": V2, "Req": round(zb * rng.uniform(0.005, 0.02), 6), "Xeq": round(zb * rng.uniform(0.02, 0.08), 6),
                                                              "Rc": round(zb * rng.uniform(200, 800), 3), "Xm": round(zb * rng.uniform(50, 200), 3), "load": round(rng.uniform(0.1, 1.25), 3),
                                                              "pf": round(rng.uniform(0.6, 1.0), 3), "pf_type": rng.choice(["lagging", "leading"])})(V1 * V1 / S))(
         rng.choice([1e4, 2.5e4, 5e4, 1e5, 5e5]), rng.choice([2400.0, 6600.0, 11000.0]), rng.choice([240.0, 415.0, 480.0])),
     templates=["Transformer regulation and efficiency: S = 50 kVA, V1 = 2400 V, V2 = 240 V, Req = 1.5 ohm, Xeq = 4 ohm, Rc = 50 kohm, Xm = 10 kohm, load = 100%, pf = 0.8"],
     examples=[{"inputs": {"S": 1e4, "V1": 1000, "V2": 100, "Req": 1.0, "Xeq": 0.0, "Rc": 1e6, "Xm": 1e9, "load": 1.0, "pf": 1.0},
                "outputs": {"V1_required": 1010.0, "regulation_pct": 1.0, "P_cu": 100.0, "P_core": 1.0201, "efficiency_pct": _r(100 * 1e4 / (1e4 + 100 + 1.0201))}},
               {"inputs": {"S": 1e4, "V1": 1000, "V2": 100, "Req": 1.0, "Xeq": 0.0, "Rc": 1e9, "Xm": 1e9, "load": 1.0, "pf": 1.0}, "outputs": {"V1_required": 1010.0, "P_cu": 100.0}}],
     questions=[{"q": "Transformer regulation and efficiency: S = 10 kVA, V1 = 1000 V, V2 = 100 V, Req = 1 ohm, Xeq = 0 ohm, Rc = 1e9 ohm, Xm = 1e9 ohm, load = 100%, pf = 1",
                 "outputs": {"regulation_pct": 1.0}},
                {"q": "Transformer regulation and efficiency: S = 10 kVA, V1 = 1000 V, V2 = 100 V, Req = 1 ohm, Xeq = 0 ohm, Rc = 1e9 ohm, Xm = 1e9 ohm, load = 300%, pf = 1",
                 "outcome": "INVALID_INPUT"}])


def transformer_tests(Voc, Ioc, Poc, Vsc, Isc, Psc):
    _pos("the open-circuit readings", Voc, Ioc, Poc)
    _pos("the short-circuit readings", Vsc, Isc, Psc)
    _need(Poc < Voc * Ioc, "the open-circuit power must be below V I (power factor below 1)")
    _need(Psc < Vsc * Isc, "the short-circuit power must be below V I (power factor below 1)")
    G, Y = Poc / Voc ** 2, Ioc / Voc
    Bm = math.sqrt(Y * Y - G * G)
    Z, R = Vsc / Isc, Psc / Isc ** 2
    X = math.sqrt(Z * Z - R * R)
    return _out(Rc=1 / G, Xm=1 / Bm, Req=R, Xeq=X, Zeq=Z, pf_oc=Poc / (Voc * Ioc), pf_sc=Psc / (Vsc * Isc))


def _ref_transformer_tests(Voc, Ioc, Poc, Vsc, Isc, Psc):
    """The parameters found by matching simulated test readings (scipy.optimize.root on the logarithms of the
    parameters): the open-circuit test drives V across Rc || jXm, the short-circuit test drives I through Req + jXeq."""
    from scipy import optimize

    def oc(u):
        Rc, Xm = math.exp(u[0]), math.exp(u[1])
        I = Voc * abs(1 / Rc + 1 / complex(0, Xm))
        return [I / Ioc - 1, Voc * Voc / Rc / Poc - 1]

    def sc(u):
        R, X = math.exp(u[0]), math.exp(u[1])
        I = Vsc / abs(complex(R, X))
        return [I / Isc - 1, I * I * R / Psc - 1]
    a = optimize.root(oc, [math.log(Voc * Voc / Poc * 1.2), math.log(Voc / Ioc * 1.1)], method="hybr", tol=1e-15)
    b = optimize.root(sc, [math.log(Psc / Isc ** 2 * 0.8), math.log(Vsc / Isc * 0.9)], method="hybr", tol=1e-15)
    Rc, Xm = math.exp(a.x[0]), math.exp(a.x[1])
    R, X = math.exp(b.x[0]), math.exp(b.x[1])
    Ioc_s = Voc * abs(1 / Rc + 1 / complex(0, Xm))
    Isc_s = Vsc / abs(complex(R, X))
    return {"Rc": Rc, "Xm": Xm, "Req": R, "Xeq": X, "Zeq": abs(complex(R, X)), "pf_oc": Voc * Voc / Rc / (Voc * Ioc_s), "pf_sc": Isc_s ** 2 * R / (Vsc * Isc_s)}


_add("energy_transformer_tests",
     name="Transformer parameters from open-circuit and short-circuit tests", topic="transformers / testing",
     purpose="Core-loss resistance and magnetizing reactance from the open-circuit test, and the equivalent series resistance, reactance and impedance from the short-circuit test (each on the side where the test was made).",
     inputs=[_phys("Voc", "voltage", "open-circuit test voltage"), _phys("Ioc", "current", "open-circuit test current"), _phys("Poc", "power", "open-circuit test power"),
             _phys("Vsc", "voltage", "short-circuit test voltage"), _phys("Isc", "current", "short-circuit test current"), _phys("Psc", "power", "short-circuit test power")],
     outputs=[_o("Rc", "resistance", "core-loss resistance (open-circuit side)"), _o("Xm", "resistance", "magnetizing reactance (open-circuit side)"),
              _o("Req", "resistance", "equivalent series resistance (short-circuit side)"), _o("Xeq", "resistance", "equivalent leakage reactance (short-circuit side)"),
              _o("Zeq", "resistance", "equivalent series impedance magnitude"), _o("pf_oc", "number", "open-circuit power factor"), _o("pf_sc", "number", "short-circuit power factor")],
     primary="Xeq", conditions=["P < V I in both tests"],
     method="Rc = V_oc^2/P_oc, Xm = 1/sqrt((I_oc/V_oc)^2 - (P_oc/V_oc^2)^2); Req = P_sc/I_sc^2, Xeq = sqrt((V_sc/I_sc)^2 - Req^2)",
     latex=r"R_c = \frac{V_{oc}^2}{P_{oc}},\ X_m = \frac{1}{\sqrt{Y_{oc}^2 - G_c^2}},\ R_{eq} = \frac{P_{sc}}{I_{sc}^2},\ X_{eq} = \sqrt{Z_{eq}^2 - R_{eq}^2}",
     run=transformer_tests,
     parse=lambda q: _pairs(q, r"transformer (?:open[- ]circuit and short[- ]circuit )?tests|transformer test parameters|oc and sc tests", {k: k for k in ("Voc", "Ioc", "Poc", "Vsc", "Isc", "Psc")},
                            phys={"Voc", "Ioc", "Poc", "Vsc", "Isc", "Psc"}),
     reference=_ref_transformer_tests,
     wrong=_safe(lambda Voc, Ioc, Poc, Vsc, Isc, Psc: (lambda o: dict(o, Xm=_r(Voc / Ioc), Xeq=_r(Vsc / Isc)))(transformer_tests(Voc, Ioc, Poc, Vsc, Isc, Psc))),
     random_inputs=lambda rng: (lambda V, I: {"Voc": V, "Ioc": round(I * rng.uniform(0.01, 0.05), 5), "Poc": None, "Vsc": round(V * rng.uniform(0.03, 0.1) * 10, 3),
                                              "Isc": round(I / 10, 4), "Psc": None, "_u": (rng.uniform(0.1, 0.5), rng.uniform(0.1, 0.6))})(rng.choice([240.0, 415.0, 480.0]), round(rng.uniform(10, 400), 3)),
     templates=["Transformer tests: Voc = 240 V, Ioc = 4.8 A, Poc = 400 W, Vsc = 120 V, Isc = 20 A, Psc = 900 W"],
     examples=[{"inputs": {"Voc": 100, "Ioc": 1, "Poc": 60, "Vsc": 50, "Isc": 10, "Psc": 300}, "outputs": {"Rc": _r(10000 / 60), "Xm": 125.0, "Req": 3.0, "Xeq": 4.0, "Zeq": 5.0}}],
     questions=[{"q": "Transformer tests: Voc = 100 V, Ioc = 1 A, Poc = 60 W, Vsc = 50 V, Isc = 10 A, Psc = 300 W", "outputs": {"Xm": 125.0, "Xeq": 4.0}},
                {"q": "Transformer tests: Voc = 100 V, Ioc = 1 A, Poc = 160 W, Vsc = 50 V, Isc = 10 A, Psc = 300 W", "outcome": "INVALID_INPUT"}])


def _rnd_tests(rng):
    V, I = rng.choice([240.0, 415.0, 480.0]), round(rng.uniform(10, 400), 3)
    Ioc = round(I * rng.uniform(0.01, 0.05), 5)
    Isc = round(I / 10, 4)
    Vsc = round(V * rng.uniform(0.3, 1.0), 3)
    return {"Voc": V, "Ioc": Ioc, "Poc": round(V * Ioc * rng.uniform(0.1, 0.5), 4), "Vsc": Vsc, "Isc": Isc, "Psc": round(Vsc * Isc * rng.uniform(0.1, 0.6), 4)}


PROCEDURES["energy_transformer_tests"]["random_inputs"] = _rnd_tests


def transformer_max_efficiency(S, P_core, P_cu_fl, pf):
    _pos("the rating and the losses", S, P_core, P_cu_fl)
    _need(0 < pf <= 1, "the power factor must lie in (0, 1]")
    x = math.sqrt(P_core / P_cu_fl)
    return _out(load_fraction=x, S_at_max=x * S, efficiency_max_pct=100 * x * S * pf / (x * S * pf + 2 * P_core),
                efficiency_full_load_pct=100 * S * pf / (S * pf + P_core + P_cu_fl))


def _ref_transformer_maxeff(S, P_core, P_cu_fl, pf):
    """sympy: the efficiency as a function of the load fraction, its derivative set to zero and solved."""
    xf, etaf = _maxeff_symbolic()
    xs = float(xf(S, P_core, P_cu_fl, pf))
    return {"load_fraction": xs, "S_at_max": xs * S, "efficiency_max_pct": 100 * float(etaf(xs, S, P_core, P_cu_fl, pf)),
            "efficiency_full_load_pct": 100 * float(etaf(1.0, S, P_core, P_cu_fl, pf))}


_SYM_CACHE: dict = {}


def _maxeff_symbolic():
    """sympy, once: eta(x) with symbolic S, P_core, P_cu, pf; d(eta)/dx = 0 solved for the positive root."""
    if "maxeff" not in _SYM_CACHE:
        import sympy as sp
        x, S, Pc, Pu, pf = sp.symbols("x S Pc Pu pf", positive=True)
        eta = x * S * pf / (x * S * pf + Pc + x ** 2 * Pu)
        sol = [r for r in sp.solve(sp.diff(eta, x), x) if r.is_positive is not False]
        assert len(sol) == 1
        _SYM_CACHE["maxeff"] = (sp.lambdify((S, Pc, Pu, pf), sol[0], "math"), sp.lambdify((x, S, Pc, Pu, pf), eta, "math"))
    return _SYM_CACHE["maxeff"]


_add("energy_transformer_max_efficiency",
     name="Load for maximum transformer efficiency", topic="transformers / efficiency",
     purpose="The load (fraction of rating and apparent power) at which a transformer's efficiency is greatest (copper loss equal to core loss), that maximum efficiency and the full-load efficiency at a power factor.",
     inputs=[_phys("S", "power", "rating in VA"), _phys("P_core", "power", "core (no-load) loss"), _phys("P_cu_fl", "power", "copper loss at full load"), _numin("pf", "load power factor")],
     outputs=[_o("load_fraction", "number", "load fraction for maximum efficiency sqrt(P_core/P_cu)"), _o("S_at_max", "power", "apparent power at maximum efficiency in VA"),
              _o("efficiency_max_pct", "number", "maximum efficiency in percent"), _o("efficiency_full_load_pct", "number", "full-load efficiency in percent")],
     primary="load_fraction", conditions=["constant core loss, copper loss proportional to the load squared"],
     method="x = sqrt(P_core/P_cu,fl); eta_max = x S pf/(x S pf + 2 P_core)", latex=r"x_{\eta,max} = \sqrt{\frac{P_{core}}{P_{cu,fl}}},\quad \eta_{max} = \frac{xS\cos\phi}{xS\cos\phi + 2P_{core}}",
     run=transformer_max_efficiency,
     parse=lambda q: _pairs(q, r"maximum (?:transformer )?efficiency|transformer maximum efficiency|load for maximum efficiency", {"S": "S", "rating": "S", "P_core": "P_core", "P0": "P_core",
                                                                                                                            "P_cu_fl": "P_cu_fl", "P_cu": "P_cu_fl", "pf": "pf"},
                            phys={"S", "P_core", "P_cu_fl"}, num_units=_pct_units("pf")),
     reference=_ref_transformer_maxeff,
     wrong=_safe(lambda S, P_core, P_cu_fl, pf: (lambda o: dict(o, load_fraction=_r(P_core / P_cu_fl), S_at_max=_r(P_core / P_cu_fl * S)))(transformer_max_efficiency(S, P_core, P_cu_fl, pf))),
     random_inputs=lambda rng: (lambda S: {"S": S, "P_core": round(S * rng.uniform(0.002, 0.01), 3), "P_cu_fl": round(S * rng.uniform(0.008, 0.03), 3), "pf": round(rng.uniform(0.7, 1), 3)})(
         rng.choice([1e4, 5e4, 2.5e5, 1e6, 1e7])),
     templates=["Maximum transformer efficiency: S = 100 kVA, P_core = 500 W, P_cu_fl = 2000 W, pf = 0.8"],
     examples=[{"inputs": {"S": 1e5, "P_core": 500, "P_cu_fl": 2000, "pf": 1.0}, "outputs": {"load_fraction": 0.5, "S_at_max": 5e4, "efficiency_max_pct": _r(100 * 5e4 / 5.1e4)}}],
     questions=[{"q": "Maximum transformer efficiency: S = 100 kVA, P_core = 500 W, P_cu_fl = 2000 W, pf = 1", "outputs": {"load_fraction": 0.5}},
                {"q": "Maximum transformer efficiency: S = 100 kVA, P_core = 0 W, P_cu_fl = 2000 W, pf = 1", "outcome": "INVALID_INPUT"}])


def _trafo_t_solve(Z, Ym, S):
    """Load-bus voltage (pu) of the T model fed at 1 pu: Newton iteration with a finite-difference Jacobian."""
    def v1_of(v2):
        i2 = (S / v2).conjugate()
        vm = v2 + i2 * Z / 2
        i1 = i2 + vm * Ym
        return vm + i1 * Z / 2, i1, i2
    v2 = 1 + 0j
    for _ in range(100):
        f = v1_of(v2)[0] - 1
        if abs(f) < 1e-15:
            break
        h = 1e-7
        fr = (v1_of(v2 + h)[0] - 1 - f) / h
        fi = (v1_of(v2 + 1j * h)[0] - 1 - f) / h
        dx = _gauss_solve([[fr.real, fi.real], [fr.imag, fi.imag]], [-f.real, -f.imag])
        v2 += complex(dx[0], dx[1])
        _need(0.3 < abs(v2) < 2, "the transformer cannot supply this load (no load-flow solution)")
    _need(abs(v1_of(v2)[0] - 1) < 1e-12, "the transformer load flow did not converge")
    v1, i1, i2 = v1_of(v2)
    return v2, i1, i2


def transformer_loading(sn, V_hv, V_lv, vk, vkr, P0, i0, P, Q):
    _pos("the rating and the voltages", sn, V_hv, V_lv)
    _need(V_hv > V_lv, "the high-voltage side must have the higher rated voltage")
    _need(0 < vkr < vk < 0.5, "need 0 < vkr < vk (short-circuit voltages as fractions or percentages, vk below 50%)")
    _nonneg("the no-load loss and current", P0, i0)
    _need(P0 / sn <= i0 or i0 == 0 and P0 == 0, "the no-load current must cover the no-load loss (i0 >= P0/sn)")
    _nonneg("the load real power", P)
    rk, zk = vkr, vk
    Z = complex(rk, math.sqrt(zk * zk - rk * rk))
    gm = P0 / sn
    Ym = complex(gm, -math.sqrt(max(i0 * i0 - gm * gm, 0.0)))
    v2, i1, i2 = _trafo_t_solve(Z, Ym, complex(P, Q) / sn)
    S1 = 1 * i1.conjugate() * sn
    loss = S1 - complex(P, Q)
    return _out(V_lv_actual=abs(v2) * V_lv, V_lv_pu=abs(v2), delta_deg=math.degrees(cmath.phase(v2)), P_loss=loss.real, Q_loss=loss.imag,
                efficiency_pct=100 * P / S1.real if S1.real > 0 else 0.0, loading_pct=100 * max(abs(i1), abs(i2)))


def _ref_transformer_loading(sn, V_hv, V_lv, vk, vkr, P0, i0, P, Q):
    import pandapower as pp
    net = _pp_empty(sn_mva=1.0)
    b1 = pp.create_bus(net, vn_kv=V_hv / 1e3)
    b2 = pp.create_bus(net, vn_kv=V_lv / 1e3)
    pp.create_ext_grid(net, b1, vm_pu=1.0)
    pp.create_transformer_from_parameters(net, b1, b2, sn / 1e6, V_hv / 1e3, V_lv / 1e3, 100 * vkr, 100 * vk, P0 / 1e3, 100 * i0)
    pp.create_load(net, b2, p_mw=P / 1e6, q_mvar=Q / 1e6)
    pp.runpp(net, tolerance_mva=1e-13 * max(1.0, sn / 1e6), max_iteration=60, trafo_model="t", numba=False, check_connectivity=False)
    t = net.res_trafo.iloc[0]
    return {"V_lv_actual": float(net.res_bus.vm_pu.iloc[1]) * V_lv, "V_lv_pu": float(net.res_bus.vm_pu.iloc[1]), "delta_deg": float(net.res_bus.va_degree.iloc[1]),
            "P_loss": float(t.pl_mw) * 1e6, "Q_loss": float(t.ql_mvar) * 1e6, "efficiency_pct": 100 * P / (float(t.p_hv_mw) * 1e6), "loading_pct": float(t.loading_percent)}


def _rnd_trafo_loading(rng):
    sn = rng.choice([1.6e5, 2.5e5, 4e5, 6.3e5, 1e6, 1.6e6])
    vk = round(rng.uniform(0.04, 0.08), 4)
    vkr = round(vk * rng.uniform(0.1, 0.4), 5)
    i0 = round(rng.uniform(0.002, 0.02), 5)
    P0 = round(sn * i0 * rng.uniform(0.1, 0.8), 2)
    s = sn * rng.uniform(0.1, 1.1)
    pf = rng.uniform(0.7, 1.0)
    return {"sn": sn, "V_hv": rng.choice([1e4, 2e4, 3.3e4]), "V_lv": rng.choice([400.0, 690.0]), "vk": vk, "vkr": vkr, "P0": P0, "i0": i0,
            "P": round(s * pf, 1), "Q": round(s * math.sin(math.acos(pf)), 1)}


_add("energy_transformer_loading",
     name="Transformer under load from nameplate data", topic="transformers / loading",
     purpose="Secondary voltage, losses, efficiency and loading of a transformer fed at rated primary voltage and supplying a P + jQ load, from its nameplate: rating, rated voltages, short-circuit voltage vk and its resistive part vkr, no-load loss and no-load current (T equivalent circuit).",
     inputs=[_phys("sn", "power", "rating in VA"), _phys("V_hv", "voltage", "rated (and applied) high-voltage side voltage"), _phys("V_lv", "voltage", "rated low-voltage side voltage"),
             _numin("vk", "short-circuit voltage as a fraction or percentage of rated voltage"), _numin("vkr", "resistive part of the short-circuit voltage (fraction or %)"),
             _phys("P0", "power", "no-load (iron) loss"), _numin("i0", "no-load current as a fraction or percentage of rated current"),
             _phys("P", "power", "load real power on the low-voltage side"), _phys("Q", "power", "load reactive power in var")],
     outputs=[_o("V_lv_actual", "voltage", "low-voltage side voltage under load"), _o("V_lv_pu", "number", "low-voltage side voltage in per unit"),
              _o("delta_deg", "number", "phase shift of the secondary voltage in degrees (no vector-group shift)"), _o("P_loss", "power", "total real loss"),
              _o("Q_loss", "power", "reactive power absorbed in var"), _o("efficiency_pct", "number", "efficiency in percent"),
              _o("loading_pct", "number", "loading: the larger winding current over its rated current, percent")],
     primary="V_lv_actual", conditions=["T equivalent circuit (series impedance halved on each side of the magnetizing branch)", "rated voltage applied, nominal tap"],
     method="per unit on the rating: z = vkr + j sqrt(vk^2 - vkr^2), y_m = P0/S - j sqrt(i0^2 - (P0/S)^2); the T circuit solved for the load-bus voltage by Newton iteration",
     latex=r"z_k = v_{kr} + j\sqrt{v_k^2 - v_{kr}^2},\quad y_m = \frac{P_0}{S_n} - j\sqrt{i_0^2 - \left(\frac{P_0}{S_n}\right)^2}",
     run=transformer_loading,
     parse=lambda q: _pairs(q, r"transformer (?:under load|loading)(?: from nameplate)?", {"sn": "sn", "S": "sn", "rating": "sn", "V_hv": "V_hv", "V_lv": "V_lv", "vk": "vk", "uk": "vk",
                                                                                         "vkr": "vkr", "ur": "vkr", "P0": "P0", "pfe": "P0", "i0": "i0", "P": "P", "Q": "Q"},
                            phys={"sn", "V_hv", "V_lv", "P0", "P", "Q"}, num_units=_pct_units("vk", "vkr", "i0")),
     reference=_ref_transformer_loading,
     wrong=_safe(lambda sn, V_hv, V_lv, vk, vkr, P0, i0, P, Q: transformer_loading(sn, V_hv, V_lv, vk, vkr, 0.0, 0.0, P, Q) if P0 > 0 else transformer_loading(sn, V_hv, V_lv, 2 * vk, vkr, P0, i0, P, Q)),
     random_inputs=_rnd_trafo_loading,
     templates=["Transformer under load: sn = 630 kVA, V_hv = 20 kV, V_lv = 400 V, vk = 6%, vkr = 1.2%, P0 = 1.5 kW, i0 = 0.3%, P = 400 kW, Q = 150 kvar"],
     examples=[{"inputs": {"sn": 6.3e5, "V_hv": 2e4, "V_lv": 400, "vk": 0.06, "vkr": 0.012, "P0": 1500, "i0": 0.003, "P": 4e5, "Q": 1.5e5},
                "outputs": {"V_lv_pu": 0.977188783296, "P_loss": 5120.79462414}}],
     questions=[{"q": "Transformer under load: sn = 630 kVA, V_hv = 20 kV, V_lv = 400 V, vk = 6%, vkr = 1.2%, P0 = 1.5 kW, i0 = 0.3%, P = 400 kW, Q = 150 kvar",
                 "outputs": {"V_lv_pu": 0.977188783296}},
                {"q": "Transformer under load: sn = 630 kVA, V_hv = 20 kV, V_lv = 400 V, vk = 6%, vkr = 7%, P0 = 1.5 kW, i0 = 0.3%, P = 400 kW, Q = 150 kvar", "outcome": "INVALID_INPUT"}])


def parallel_transformers(S1, vk1, vkr1, S2, vk2, vkr2, V_lv, P, Q):
    _pos("the ratings and the voltage", S1, S2, V_lv)
    for vk, vkr in ((vk1, vkr1), (vk2, vkr2)):
        _need(0 <= vkr < vk < 0.5, "need 0 <= vkr < vk below 50% for each transformer")
    zb = lambda vk, vkr, S: complex(vkr, math.sqrt(vk * vk - vkr * vkr)) * V_lv * V_lv / S   # noqa: E731
    Z1, Z2 = zb(vk1, vkr1, S1), zb(vk2, vkr2, S2)
    SL = complex(P, Q)
    s1, s2 = SL * (Z2 / (Z1 + Z2)).conjugate(), SL * (Z1 / (Z1 + Z2)).conjugate()
    return _out(S1_share=abs(s1), S2_share=abs(s2), P1=s1.real, Q1=s1.imag, P2=s2.real, Q2=s2.imag, share1_pct_of_rating=100 * abs(s1) / S1, share2_pct_of_rating=100 * abs(s2) / S2)


def _ref_parallel(S1, vk1, vkr1, S2, vk2, vkr2, V_lv, P, Q):
    import pandapower as pp
    net = _pp_empty(sn_mva=1.0)
    hv = pp.create_bus(net, vn_kv=20.0)
    lv = pp.create_bus(net, vn_kv=V_lv / 1e3)
    pp.create_ext_grid(net, hv)
    pp.create_transformer_from_parameters(net, hv, lv, S1 / 1e6, 20.0, V_lv / 1e3, 100 * vkr1, 100 * vk1, 0.0, 0.0)
    pp.create_transformer_from_parameters(net, hv, lv, S2 / 1e6, 20.0, V_lv / 1e3, 100 * vkr2, 100 * vk2, 0.0, 0.0)
    pp.create_load(net, lv, p_mw=P / 1e6, q_mvar=Q / 1e6)
    pp.runpp(net, tolerance_mva=1e-13 * max(1.0, abs(complex(P, Q)) / 1e6), max_iteration=60, numba=False, check_connectivity=False)
    t = net.res_trafo
    s1 = complex(-float(t.p_lv_mw.iloc[0]), -float(t.q_lv_mvar.iloc[0])) * 1e6
    s2 = complex(-float(t.p_lv_mw.iloc[1]), -float(t.q_lv_mvar.iloc[1])) * 1e6
    return {"S1_share": abs(s1), "S2_share": abs(s2), "P1": s1.real, "Q1": s1.imag, "P2": s2.real, "Q2": s2.imag,
            "share1_pct_of_rating": 100 * abs(s1) / S1, "share2_pct_of_rating": 100 * abs(s2) / S2}


_add("energy_parallel_transformers",
     name="Load sharing between two transformers in parallel", topic="transformers / parallel operation",
     purpose="How two transformers with equal voltage ratios but different ratings and short-circuit impedances share a load: the apparent, real and reactive power each delivers and its loading on its own rating.",
     inputs=[_phys("S1", "power", "rating of transformer 1 (VA)"), _numin("vk1", "short-circuit voltage of transformer 1 (fraction or %)"), _numin("vkr1", "its resistive part (fraction or %)"),
             _phys("S2", "power", "rating of transformer 2 (VA)"), _numin("vk2", "short-circuit voltage of transformer 2 (fraction or %)"), _numin("vkr2", "its resistive part (fraction or %)"),
             _phys("V_lv", "voltage", "common secondary rated voltage"), _phys("P", "power", "load real power"), _phys("Q", "power", "load reactive power in var")],
     outputs=[_o("S1_share", "power", "apparent power delivered by transformer 1 in VA"), _o("S2_share", "power", "apparent power delivered by transformer 2 in VA"),
              _o("P1", "power", "real power of transformer 1"), _o("Q1", "power", "reactive power of transformer 1 in var"), _o("P2", "power", "real power of transformer 2"),
              _o("Q2", "power", "reactive power of transformer 2 in var"), _o("share1_pct_of_rating", "number", "transformer 1 output over its rating, percent"),
              _o("share2_pct_of_rating", "number", "transformer 2 output over its rating, percent")],
     primary="S1_share", conditions=["equal voltage ratios (no circulating current)", "magnetizing branches neglected"],
     method="impedances in ohms on the common secondary: Z_i = (vkr_i + j sqrt(vk_i^2 - vkr_i^2)) V^2/S_i; S_1 = S_L conj(Z_2/(Z_1 + Z_2)), S_2 = S_L conj(Z_1/(Z_1 + Z_2))",
     latex=r"\mathbf S_1 = \mathbf S_L\left(\frac{\mathbf Z_2}{\mathbf Z_1 + \mathbf Z_2}\right)^*,\quad \mathbf S_2 = \mathbf S_L\left(\frac{\mathbf Z_1}{\mathbf Z_1 + \mathbf Z_2}\right)^*",
     run=parallel_transformers,
     parse=lambda q: _pairs(q, r"parallel transformers|transformers in parallel|load sharing between (?:two )?transformers", {k: k for k in ("S1", "vk1", "vkr1", "S2", "vk2", "vkr2", "V_lv", "P", "Q")},
                            phys={"S1", "S2", "V_lv", "P", "Q"}, num_units=_pct_units("vk1", "vkr1", "vk2", "vkr2")),
     reference=_ref_parallel,
     wrong=_safe(lambda S1, vk1, vkr1, S2, vk2, vkr2, V_lv, P, Q: (lambda SL: _out(S1_share=abs(SL) * S1 / (S1 + S2), S2_share=abs(SL) * S2 / (S1 + S2), P1=P * S1 / (S1 + S2), Q1=Q * S1 / (S1 + S2),
                                                                                  P2=P * S2 / (S1 + S2), Q2=Q * S2 / (S1 + S2), share1_pct_of_rating=100 * abs(SL) / (S1 + S2),
                                                                                  share2_pct_of_rating=100 * abs(SL) / (S1 + S2)))(complex(P, Q))),
     random_inputs=lambda rng: (lambda S1, S2: {"S1": S1, "vk1": round(rng.uniform(0.04, 0.08), 4), "vkr1": round(rng.uniform(0.005, 0.015), 4), "S2": S2,
                                                "vk2": round(rng.uniform(0.04, 0.08), 4), "vkr2": round(rng.uniform(0.005, 0.015), 4), "V_lv": rng.choice([400.0, 690.0]),
                                                "P": round((S1 + S2) * rng.uniform(0.2, 0.9), 0), "Q": round((S1 + S2) * rng.uniform(0.0, 0.4), 0)})(
         rng.choice([2.5e5, 4e5, 6.3e5, 1e6]), rng.choice([2.5e5, 4e5, 6.3e5, 1e6])),
     templates=["Parallel transformers: S1 = 630 kVA, vk1 = 6%, vkr1 = 1%, S2 = 400 kVA, vk2 = 4%, vkr2 = 1.5%, V_lv = 400 V, P = 800 kW, Q = 400 kvar"],
     examples=[{"inputs": {"S1": 5e5, "vk1": 0.05, "vkr1": 0.01, "S2": 5e5, "vk2": 0.05, "vkr2": 0.01, "V_lv": 400, "P": 6e5, "Q": 2e5}, "outputs": {"P1": 3e5, "Q2": 1e5}}],
     questions=[{"q": "Parallel transformers: S1 = 500 kVA, vk1 = 5%, vkr1 = 1%, S2 = 500 kVA, vk2 = 5%, vkr2 = 1%, V_lv = 400 V, P = 600 kW, Q = 200 kvar", "outputs": {"P1": 3e5}},
                {"q": "Parallel transformers: S1 = 500 kVA, vk1 = 5%, vkr1 = 6%, S2 = 500 kVA, vk2 = 5%, vkr2 = 1%, V_lv = 400 V, P = 600 kW, Q = 200 kvar", "outcome": "INVALID_INPUT"}])


# ================================================================================================ MACHINES
def _im_phase(VL, connection):
    return VL / SQ3 if connection == "star" else VL


def _check_im(VL, f, poles, R1, X1, R2, X2, Xm):
    _pos("the line voltage and frequency", VL, f)
    _need(isinstance(poles, int) and poles >= 2 and poles % 2 == 0, "the number of poles must be an even whole number (2, 4, 6, ...)")
    _nonneg("the stator resistance and reactance", R1, X1)
    _pos("the rotor resistance and reactance and the magnetizing reactance", R2, X2, Xm)


def induction_motor(VL, f, poles, R1, X1, R2, X2, Xm, slip, connection, P_rot=0.0):
    _check_im(VL, f, poles, R1, X1, R2, X2, Xm)
    _need(0 < slip <= 1, "the slip must lie in (0, 1] (motoring)")
    _nonneg("the rotational loss", P_rot)
    ns = 120 * f / poles
    ws = 4 * math.pi * f / poles
    V = _im_phase(VL, connection)
    Z1, Zm, Z2 = complex(R1, X1), complex(0, Xm), complex(R2 / slip, X2)
    Zin = Z1 + Zm * Z2 / (Zm + Z2)
    I1 = V / Zin
    E = V - I1 * Z1
    I2 = E / Z2
    Pin = 3 * (V * I1.conjugate()).real
    Pag = 3 * abs(I2) ** 2 * R2 / slip
    Pconv = (1 - slip) * Pag
    Pout = Pconv - P_rot
    IL = abs(I1) if connection == "star" else SQ3 * abs(I1)
    return _out(I_line=IL, I2=abs(I2), pf=math.cos(-cmath.phase(I1)), P_in=Pin, P_ag=Pag, P_conv=Pconv, T=Pag / ws,
                P_out=Pout, efficiency_pct=100 * Pout / Pin)


def _ref_induction(VL, f, poles, R1, X1, R2, X2, Xm, slip, connection, P_rot=0.0):
    """The per-phase circuit solved by nodal analysis at the air-gap node (numpy); torque from the converted power and
    the rotor's mechanical speed."""
    import numpy as np
    V = VL / SQ3 if connection == "star" else VL
    Z1, Zm, Z2 = complex(R1, X1), complex(0, Xm), complex(R2 / slip, X2)
    Vm = np.linalg.solve(np.array([[1 / Z1 + 1 / Zm + 1 / Z2]]), np.array([V / Z1]))[0]
    I1 = (V - Vm) / Z1
    I2 = Vm / Z2
    Pin = 3 * float((V * np.conj(I1)).real)
    Pconv = 3 * abs(I2) ** 2 * R2 * (1 - slip) / slip
    Pag = 3 * abs(I2) ** 2 * R2 / slip
    wm = (1 - slip) * 4 * math.pi * f / poles
    T = Pconv / wm if slip < 1 else Pag / (4 * math.pi * f / poles)
    ns = 120 * f / poles
    return {"I_line": abs(I1) * (1 if connection == "star" else SQ3), "I2": abs(I2), "pf": math.cos(cmath.phase(V) - cmath.phase(complex(I1))),
            "P_in": Pin, "P_ag": Pag, "P_conv": Pconv, "T": T, "P_out": Pconv - P_rot, "efficiency_pct": 100 * (Pconv - P_rot) / Pin}


_IM_IN = [_phys("VL", "voltage", "line voltage"), _phys("f", "frequency", "supply frequency"), _intin("poles", "number of poles (even)", 2, 64),
          _phys("R1", "resistance", "stator resistance per phase"), _phys("X1", "resistance", "stator leakage reactance per phase"),
          _phys("R2", "resistance", "rotor resistance per phase referred to the stator"), _phys("X2", "resistance", "rotor leakage reactance referred to the stator"),
          _phys("Xm", "resistance", "magnetizing reactance per phase")]
_IM_NAMES = {"VL": "VL", "V": "VL", "f": "f", "poles": "poles", "p": "poles", "R1": "R1", "X1": "X1", "R2": "R2", "X2": "X2", "Xm": "Xm", "slip": "slip", "s": "slip",
             "connection": "connection", "P_rot": "P_rot", "rotational_loss": "P_rot"}


def _rnd_im(rng, with_slip=True):
    V = rng.choice([400.0, 415.0, 460.0, 690.0, 3300.0])
    x = {"VL": V, "f": rng.choice([50.0, 60.0]), "poles": rng.choice([2, 4, 6, 8]), "R1": round(rng.uniform(0.1, 1.0), 4), "X1": round(rng.uniform(0.3, 2.0), 4),
         "R2": round(rng.uniform(0.1, 1.0), 4), "X2": round(rng.uniform(0.3, 2.0), 4), "Xm": round(rng.uniform(15, 60), 3), "connection": rng.choice(["star", "delta"])}
    if with_slip:
        x.update({"slip": round(rng.uniform(0.005, 0.2), 5), "P_rot": rng.choice([0.0, round(rng.uniform(50, 800), 1)])})
    return x


_add("energy_induction_motor",
     name="Induction motor performance from its equivalent circuit", topic="machines / induction motors",
     purpose="Line and rotor currents, power factor, air-gap and converted power, induced torque, output and efficiency of a three-phase induction motor at a slip, from its per-phase equivalent circuit.",
     inputs=list(_IM_IN) + [_numin("slip", "slip (0 < s <= 1)"), _choice("connection", ["star", "delta"], "stator connection"),
                            _phys("P_rot", "power", "rotational (friction, windage and core) loss, default 0", required=False)],
     outputs=[_o("I_line", "current", "line current"),
              _o("I2", "current", "rotor current referred to the stator"), _o("pf", "number", "power factor"), _o("P_in", "power", "input power"), _o("P_ag", "power", "air-gap power"),
              _o("P_conv", "power", "converted (developed) power"), _o("T", "torque", "induced torque"), _o("P_out", "power", "output power"), _o("efficiency_pct", "number", "efficiency in percent")],
     primary="T", conditions=["steady state, balanced supply", "0 < slip <= 1"],
     method="Z = R1 + jX1 + jXm || (R2/s + jX2); P_ag = 3 I2^2 R2/s; P_conv = (1 - s) P_ag; T = P_ag/omega_s; n_s = 120 f/p",
     latex=r"T_{ind} = \frac{3 I_2^2 R_2/s}{\omega_s},\quad n_s = \frac{120 f}{p}",
     run=induction_motor,
     parse=lambda q: _pairs(q, r"induction motor(?: performance)?(?: from (?:its |the )?equivalent circuit)?", _IM_NAMES, phys={"VL", "f", "R1", "X1", "R2", "X2", "Xm", "P_rot"},
                            words={"connection"}, num_units=_pct_units("slip")),
     reference=_ref_induction,
     wrong=_safe(lambda VL, f, poles, R1, X1, R2, X2, Xm, slip, connection, P_rot=0.0: (lambda o: dict(o, T=_r(o["P_conv"] / (4 * math.pi * f / poles))))(
         induction_motor(VL, f, poles, R1, X1, R2, X2, Xm, slip, connection, P_rot))),
     random_inputs=lambda rng: _rnd_im(rng),
     templates=["Induction motor: VL = 460 V, f = 60 Hz, poles = 4, R1 = 0.641 ohm, X1 = 1.106 ohm, R2 = 0.332 ohm, X2 = 0.464 ohm, Xm = 26.3 ohm, slip = 2.2%, connection = star"],
     examples=[{"inputs": {"VL": 400 * SQ3, "f": 50, "poles": 4, "R1": 0.0, "X1": 2.0, "R2": 1.5, "X2": 2.0, "Xm": 1e12, "slip": 0.5, "connection": "star"},
                "outputs": {"I_line": 80.0, "pf": 0.6, "P_ag": 57600.0, "T": _r(57600 / (50 * math.pi)), "efficiency_pct": 50.0}}],
     questions=[{"q": "Induction motor: VL = 460 V, f = 60 Hz, poles = 4, R1 = 0.641 ohm, X1 = 1.106 ohm, R2 = 0.332 ohm, X2 = 0.464 ohm, Xm = 26.3 ohm, slip = 2.2%, connection = star",
                 "outputs": {"P_conv": _r(0.978 * induction_motor(460, 60, 4, 0.641, 1.106, 0.332, 0.464, 26.3, 0.022, "star")["P_ag"])}},
                {"q": "Induction motor: VL = 460 V, f = 60 Hz, poles = 5, R1 = 0.641 ohm, X1 = 1.106 ohm, R2 = 0.332 ohm, X2 = 0.464 ohm, Xm = 26.3 ohm, slip = 2.2%, connection = star",
                 "outcome": "INVALID_INPUT"}])


def induction_max_torque(VL, f, poles, R1, X1, R2, X2, Xm, connection):
    _check_im(VL, f, poles, R1, X1, R2, X2, Xm)
    V = _im_phase(VL, connection)
    ws = 4 * math.pi * f / poles
    Z1, Zm = complex(R1, X1), complex(0, Xm)
    Vth = abs(V * Zm / (Z1 + Zm))
    Zth = Z1 * Zm / (Z1 + Zm)
    root = math.sqrt(Zth.real ** 2 + (Zth.imag + X2) ** 2)
    s = R2 / root
    Tmax = 3 * Vth * Vth / (2 * ws * (Zth.real + root))
    st = induction_motor(VL, f, poles, R1, X1, R2, X2, Xm, 1.0, connection)
    return _out(s_maxT=s, T_max=Tmax, T_start=st["T"], I_start=st["I_line"], V_th=Vth, R_th=Zth.real, X_th=Zth.imag)


def _im_symbolic():
    """sympy, once: T(s) = 3 V^2 Xm^2 (R2/s)/(omega_s |A + B (R2/s + j X2)|^2) and d(ln T)/ds with symbolic constants."""
    if "im" not in _SYM_CACHE:
        import sympy as sp
        s, ar, ai, br, bi, r2, x2, v, xm, ws = sp.symbols("s ar ai br bi r2 x2 v xm ws", real=True)
        re = ar + br * r2 / s - bi * x2
        im = ai + bi * r2 / s + br * x2
        T = 3 * v * v * xm * xm * (r2 / s) / (ws * (re * re + im * im))
        args = (s, ar, ai, br, bi, r2, x2, v, xm, ws)
        _SYM_CACHE["im"] = (sp.lambdify(args, sp.diff(sp.log(T), s), "math"), sp.lambdify(args, T, "math"))
    return _SYM_CACHE["im"]


def _ref_im_maxT(VL, f, poles, R1, X1, R2, X2, Xm, connection):
    """The full-circuit torque T(s) written symbolically (sympy, no Thevenin reduction), its derivative's root found by
    bisection; starting torque and current from the circuit at s = 1 (numpy); the Thevenin source from two network
    tests (open-circuit voltage, short-circuit current)."""
    import numpy as np
    import sympy as sp
    V = VL / SQ3 if connection == "star" else VL
    ws = 4 * math.pi * f / poles
    Z1, Zm = complex(R1, X1), complex(0, Xm)
    A, B = Z1 * Zm, Z1 + Zm
    dlog_f, T_f = _im_symbolic()
    c = (A.real, A.imag, B.real, B.imag, R2, X2, V, Xm, ws)
    sm = _bisect(lambda x: dlog_f(x, *c), 1e-9, 1e3)
    Tf = lambda x: T_f(x, *c)   # noqa: E731

    def circuit(slip):
        Z2 = complex(R2 / slip, X2)
        Vm = np.linalg.solve(np.array([[1 / Z1 + 1 / Zm + 1 / Z2]]), np.array([V / Z1]))[0]
        return (V - Vm) / Z1, Vm / Z2
    I1s, I2s = circuit(1.0)
    voc = V * Zm / (Z1 + Zm)                           # open rotor: divider solved as a network
    voc = np.linalg.solve(np.array([[1 / Z1 + 1 / Zm]]), np.array([V / Z1]))[0]
    isc = V / Z1                                       # rotor terminals shorted: the magnetizing branch is bypassed
    zth = voc / isc
    return {"s_maxT": sm, "T_max": float(Tf(sm)), "T_start": 3 * abs(I2s) ** 2 * R2 / ws,
            "I_start": abs(I1s) * (1 if connection == "star" else SQ3), "V_th": abs(voc), "R_th": float(zth.real), "X_th": float(zth.imag)}


_add("energy_induction_max_torque",
     name="Induction motor breakdown (maximum) torque and starting torque", topic="machines / induction motors",
     purpose="The slip at maximum torque, the maximum (breakdown) torque, the starting torque and current, and the Thevenin equivalent of the stator and magnetizing branch, for a three-phase induction motor.",
     inputs=list(_IM_IN) + [_choice("connection", ["star", "delta"], "stator connection")],
     outputs=[_o("s_maxT", "number", "slip at maximum torque"), _o("T_max", "torque", "maximum (pull-out) torque"),
              _o("T_start", "torque", "starting torque (s = 1)"), _o("I_start", "current", "starting line current"), _o("V_th", "voltage", "Thevenin voltage per phase"),
              _o("R_th", "resistance", "Thevenin resistance"), _o("X_th", "resistance", "Thevenin reactance")],
     primary="T_max", conditions=["constant parameters (no deep-bar or saturation effects)"],
     method="Thevenin: V_th = V jXm/(R1 + j(X1 + Xm)), Z_th = Z1 || jXm; s_max = R2/sqrt(R_th^2 + (X_th + X2)^2); T_max = 3 V_th^2/(2 omega_s (R_th + sqrt(R_th^2 + (X_th + X2)^2)))",
     latex=r"s_{max} = \frac{R_2}{\sqrt{R_{th}^2 + (X_{th} + X_2)^2}},\quad T_{max} = \frac{3V_{th}^2}{2\omega_s\left[R_{th} + \sqrt{R_{th}^2 + (X_{th}+X_2)^2}\right]}",
     run=induction_max_torque,
     parse=lambda q: _pairs(q, r"induction motor (?:maximum|breakdown|pull[- ]out) torque|maximum torque of an induction motor", _IM_NAMES, phys={"VL", "f", "R1", "X1", "R2", "X2", "Xm"},
                            words={"connection"}),
     reference=_ref_im_maxT,
     wrong=_safe(lambda VL, f, poles, R1, X1, R2, X2, Xm, connection: (lambda o: dict(o, s_maxT=_r(R2 / (o["X_th"] + X2)), T_max=_r(3 * o["V_th"] ** 2 / (2 * 4 * math.pi * f / poles * (o["X_th"] + X2)))))(
         induction_max_torque(VL, f, poles, R1, X1, R2, X2, Xm, connection))),
     random_inputs=lambda rng: _rnd_im(rng, False),
     templates=["Induction motor maximum torque: VL = 460 V, f = 60 Hz, poles = 4, R1 = 0.641 ohm, X1 = 1.106 ohm, R2 = 0.332 ohm, X2 = 0.464 ohm, Xm = 26.3 ohm, connection = star"],
     examples=[{"inputs": {"VL": 400 * SQ3, "f": 50, "poles": 4, "R1": 0.0, "X1": 1.0, "R2": 0.2, "X2": 1.0, "Xm": 1e12, "connection": "star"},
                "outputs": {"s_maxT": 0.1, "T_max": _r(3 * 400 * 400 / (2 * 50 * math.pi * 2))}}],
     questions=[{"q": "Induction motor maximum torque: VL = 460 V, f = 60 Hz, poles = 4, R1 = 0.641 ohm, X1 = 1.106 ohm, R2 = 0.332 ohm, X2 = 0.464 ohm, Xm = 26.3 ohm",
                 "outcome": "NEEDS_INPUT"},
                {"q": "Induction motor breakdown torque: VL = 460 V, f = 60 Hz, poles = 4, R1 = 0 ohm, X1 = 1 ohm, R2 = 0.2 ohm, X2 = 1 ohm, Xm = 1e12 ohm, connection = star",
                 "outputs": {"s_maxT": 0.1}}])


def synchronous_generator(VL, IL, pf, Xs, Ra=0.0, pf_type="lagging"):
    _pos("the terminal voltage, the current and the synchronous reactance", VL, IL, Xs)
    _nonneg("the armature resistance", Ra)
    phi = _pf_angle(pf, pf_type)
    V = VL / SQ3
    I = cmath.rect(IL, -phi)
    E = V + complex(Ra, Xs) * I
    return _out(E_line=SQ3 * abs(E), E_phase=abs(E), delta_deg=math.degrees(cmath.phase(E)), regulation_pct=100 * (abs(E) - V) / V,
                P=3 * V * IL * pf, Q=0.0 if pf == 1 else 3 * V * IL * math.sin(phi))


def _ref_syncgen(VL, IL, pf, Xs, Ra=0.0, pf_type="lagging"):
    """The internal EMF and power angle as the solution (scipy.optimize.root) of the machine's power equations at the
    terminals, P + jQ = 3 V conj((E e^{j delta} - V)/Z), not the phasor sum."""
    from scipy import optimize
    V = VL / SQ3
    phi = math.acos(pf) * (1 if pf_type != "leading" else -1)
    P, Q = 3 * V * IL * pf, 3 * V * IL * math.sin(phi)
    Z = complex(Ra, Xs)
    S0 = max(abs(complex(P, Q)), 1.0)

    def eqs(u):
        E, d = V * math.exp(u[0]), u[1]
        S = 3 * V * ((cmath.rect(E, d) - V) / Z).conjugate()
        return [(S.real - P) / S0, (S.imag - Q) / S0]
    sol = optimize.root(eqs, [0.2, 0.3], method="hybr", tol=1e-15)
    E, d = V * math.exp(sol.x[0]), sol.x[1]
    S = 3 * V * ((cmath.rect(E, d) - V) / Z).conjugate()
    return {"E_line": SQ3 * E, "E_phase": E, "delta_deg": math.degrees(d), "regulation_pct": 100 * (E - V) / V, "P": S.real, "Q": S.imag if abs(S.imag) > 1e-12 * S0 else 0.0}


_add("energy_synchronous_generator",
     name="Synchronous generator internal EMF, power angle and regulation", topic="machines / synchronous machines",
     purpose="The internal (excitation) EMF, power angle and voltage regulation of a star-connected cylindrical-rotor synchronous generator delivering a current at a power factor, from its synchronous reactance and armature resistance.",
     inputs=[_phys("VL", "voltage", "terminal line voltage"), _phys("IL", "current", "armature (line) current"), _numin("pf", "power factor"),
             _phys("Xs", "resistance", "synchronous reactance per phase"), _phys("Ra", "resistance", "armature resistance per phase (default 0)", required=False),
             _choice("pf_type", ["lagging", "leading"], "lagging (default) or leading", required=False)],
     outputs=[_o("E_line", "voltage", "internal EMF (line value)"), _o("E_phase", "voltage", "internal EMF per phase"), _o("delta_deg", "number", "power (torque) angle in degrees"),
              _o("regulation_pct", "number", "voltage regulation (E - V)/V in percent"), _o("P", "power", "real power output"), _o("Q", "power", "reactive power output in var")],
     primary="E_line", conditions=["cylindrical rotor (no saliency), star connection, unsaturated"],
     method="E = V + (Ra + jXs) I per phase, I at angle -acos(pf)", latex=r"\mathbf E_A = \mathbf V_\phi + (R_A + jX_S)\mathbf I_A",
     run=synchronous_generator,
     parse=lambda q: _pairs(q, r"synchronous generator(?: emf| internal emf)?", {"VL": "VL", "V": "VL", "IL": "IL", "I": "IL", "pf": "pf", "Xs": "Xs", "Ra": "Ra", "pf_type": "pf_type"},
                            phys={"VL", "IL", "Xs", "Ra"}, words={"pf_type"}, num_units=_pct_units("pf")),
     reference=_ref_syncgen,
     wrong=_safe(lambda VL, IL, pf, Xs, Ra=0.0, pf_type="lagging": (lambda o: dict(o, E_phase=_r(VL / SQ3 + Xs * IL), E_line=_r(SQ3 * (VL / SQ3 + Xs * IL))))(synchronous_generator(VL, IL, pf, Xs, Ra, pf_type))),
     random_inputs=lambda rng: (lambda V, I: {"VL": V, "IL": I, "pf": round(rng.uniform(0.6, 1.0), 3), "Xs": round(rng.uniform(0.2, 1.2) * V / SQ3 / I, 6),
                                              "Ra": rng.choice([0.0, round(rng.uniform(0.002, 0.03) * V / SQ3 / I, 6)]), "pf_type": rng.choice(["lagging", "leading"])})(
         rng.choice([480.0, 3300.0, 11000.0, 13800.0]), round(_lu(rng, 10, 3000), 3)),
     templates=["Synchronous generator: VL = 480 V, IL = 60 A, pf = 0.8, Xs = 1 ohm, Ra = 0.015 ohm"],
     examples=[{"inputs": {"VL": 100 * SQ3, "IL": 10, "pf": 1.0, "Xs": 10.0}, "outputs": {"E_phase": _r(100 * math.sqrt(2)), "delta_deg": 45.0, "Q": 0.0}}],
     questions=[{"q": "Synchronous generator: VL = 173.20508075688772 V, IL = 10 A, pf = 1, Xs = 10 ohm", "outputs": {"delta_deg": 45.0}},
                {"q": "Synchronous generator: VL = 480 V, IL = 60 A, pf = 0.8, Xs = 0 ohm", "outcome": "INVALID_INPUT"}])


# ================================================================================================ BATTERIES
def peukert(currents, times, I):
    _need(len(currents) == len(times) and len(currents) >= 2, "give at least two discharge tests (a current and its run time each)")
    for c, t in zip(currents, times):
        _need(c > 0 and t > 0, "test currents and run times must be positive")
    _need(len(set(currents)) == len(currents), "the test currents must be different")
    _pos("the new discharge current", I)
    xs, ys = [math.log(c) for c in currents], [math.log(t) for t in times]
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    a = my - b * mx
    k = -b
    _need(k > 0, "the run time must fall as the current rises (Peukert exponent k > 0)")
    t = math.exp(a + b * math.log(I))
    return _out(k=k, C_p=math.exp(a), runtime_h=t, capacity_Ah=I * t)


def _ref_peukert(currents, times, I):
    import math as m
    from scipy import stats
    r = stats.linregress([m.log(c) for c in currents], [m.log(t) for t in times])
    t = m.exp(r.intercept + r.slope * m.log(I))
    return {"k": -r.slope, "C_p": m.exp(r.intercept), "runtime_h": t, "capacity_Ah": I * t}


_add("energy_peukert",
     name="Peukert exponent and run time at a new discharge current", topic="batteries / capacity",
     purpose="Peukert's law t = C_p/I^k fitted (least squares on the logarithms) to two or more constant-current discharge tests, then the run time and delivered capacity at another current.",
     inputs=[_vecin("currents", "test discharge currents in A", 2, 20), _vecin("times", "run time of each test in hours", 2, 20), _phys("I", "current", "new discharge current")],
     outputs=[_o("k", "number", "Peukert exponent"), _o("C_p", "number", "Peukert capacity C_p in A^k h"), _o("runtime_h", "number", "run time at I in hours"),
              _o("capacity_Ah", "number", "capacity delivered at I in Ah")],
     primary="runtime_h", conditions=["at least two tests at different currents", "constant temperature and cut-off voltage"],
     method="ln t = ln C_p - k ln I fitted by least squares; t(I) = C_p I^-k", latex=r"t = \frac{C_p}{I^k}\;\Rightarrow\; \ln t = \ln C_p - k\ln I",
     run=peukert,
     parse=lambda q: _pairs(q, r"peukert(?:'s law| exponent)?", {"currents": "currents", "times": "times", "runtimes": "times", "I": "I", "current": "I"}, phys={"I"}, vectors={"currents", "times"}),
     reference=_ref_peukert,
     wrong=_safe(lambda currents, times, I: (lambda o: dict(o, runtime_h=_r(o["C_p"] / I), capacity_Ah=_r(o["C_p"])))(peukert(currents, times, I))),
     random_inputs=lambda rng: (lambda k, cp, n: (lambda cs: {"currents": cs, "times": [round(cp / c ** k * rng.uniform(0.98, 1.02), 4) for c in cs], "I": round(rng.uniform(1, 50), 3)})(
         sorted(rng.sample(range(2, 80), n))))(rng.uniform(1.05, 1.4), rng.uniform(50, 500), rng.randint(2, 5)),
     templates=["Peukert: currents = [5, 20], times = [20, 4], I = 10 A"],
     examples=[{"inputs": {"currents": [1, 10], "times": [100, 10], "I": 5}, "outputs": {"k": 1.0, "C_p": 100.0, "runtime_h": 20.0}},
               {"inputs": {"currents": "2, 8", "times": "50, 10", "I": 4}, "outputs": {"k": _r(math.log(5) / math.log(4))}}],
     questions=[{"q": "Peukert: currents = [1, 10], times = [100, 10], I = 5 A", "outputs": {"runtime_h": 20.0}},
                {"q": "Peukert: currents = [5, 5], times = [20, 4], I = 10 A", "outcome": "INVALID_INPUT"}])


def _ceil_tol(x):
    n = math.ceil(x)
    return n - 1 if n - 1 > 0 and abs(x - (n - 1)) <= 1e-9 * max(1.0, x) else n


def pack_sizing(V_target, E_target, V_cell, Q_cell):
    _pos("the target voltage and energy and the cell voltage and capacity", V_target, E_target, V_cell, Q_cell)
    ns = max(1, _ceil_tol(V_target / V_cell))
    np_ = max(1, _ceil_tol(E_target / (ns * V_cell * Q_cell)))
    _need(ns * np_ <= 10 ** 7, "more than ten million cells")
    return {"N_series": ns, "N_parallel": np_, "N_cells": ns * np_, "V_pack": _r(ns * V_cell), "E_pack_kWh": _r(ns * np_ * V_cell * Q_cell / 3.6e6),
            "Q_pack_Ah": _r(np_ * Q_cell / 3600)}


def _ref_pack(V_target, E_target, V_cell, Q_cell):
    """Two integer programmes (scipy.optimize.milp): the fewest cells in series reaching the voltage, then the fewest
    strings in parallel reaching the energy."""
    import numpy as np
    from scipy.optimize import Bounds, LinearConstraint, milp
    r1 = milp(c=[1.0], constraints=LinearConstraint([[V_cell]], lb=V_target * (1 - 1e-9), ub=np.inf), integrality=[1], bounds=Bounds(1, 1e8))
    ns = int(round(r1.x[0]))
    e1 = ns * V_cell * Q_cell
    r2 = milp(c=[1.0], constraints=LinearConstraint([[e1]], lb=E_target * (1 - 1e-9), ub=np.inf), integrality=[1], bounds=Bounds(1, 1e8))
    np_ = int(round(r2.x[0]))
    return {"N_series": ns, "N_parallel": np_, "N_cells": ns * np_, "V_pack": ns * V_cell, "E_pack_kWh": ns * np_ * V_cell * Q_cell / 3.6e6, "Q_pack_Ah": np_ * Q_cell / 3600}


_add("energy_battery_pack_sizing",
     name="Battery pack series-parallel sizing", topic="batteries / pack design",
     purpose="The number of cells in series to reach a pack voltage and of strings in parallel to reach a pack energy, with the resulting pack voltage, energy and capacity (fewest cells; nominal values).",
     inputs=[_phys("V_target", "voltage", "required nominal pack voltage"), _phys("E_target", "energy", "required pack energy (Wh or kWh)"),
             _phys("V_cell", "voltage", "nominal cell voltage"), _phys("Q_cell", "charge", "cell capacity (Ah accepted)")],
     outputs=[_o("N_series", "number", "cells in series"), _o("N_parallel", "number", "strings in parallel"), _o("N_cells", "number", "total cells"),
              _o("V_pack", "voltage", "nominal pack voltage"), _o("E_pack_kWh", "number", "pack energy in kWh"), _o("Q_pack_Ah", "number", "pack capacity in Ah")],
     primary="N_cells", conditions=["nominal voltages and capacities", "a target met within 1e-9 counts as met"],
     method="N_s = ceil(V/V_cell); N_p = ceil(E/(N_s V_cell Q_cell))", latex=r"N_s = \left\lceil\frac{V_{pack}}{V_{cell}}\right\rceil,\quad N_p = \left\lceil\frac{E_{pack}}{N_s V_{cell} Q_{cell}}\right\rceil",
     run=pack_sizing,
     parse=lambda q: _pairs(q, r"battery pack sizing|pack sizing|series[- ]parallel pack", {"V_target": "V_target", "V_pack": "V_target", "E_target": "E_target", "E_pack": "E_target",
                                                                                       "V_cell": "V_cell", "Q_cell": "Q_cell", "C_cell": "Q_cell"},
                            phys={"V_target", "E_target", "V_cell", "Q_cell"}),
     reference=_ref_pack,
     wrong=_safe(lambda V_target, E_target, V_cell, Q_cell: (lambda ns: (lambda np_: {"N_series": ns, "N_parallel": np_, "N_cells": ns * np_, "V_pack": _r(ns * V_cell),
                                                                                     "E_pack_kWh": _r(ns * np_ * V_cell * Q_cell / 3.6e6), "Q_pack_Ah": _r(np_ * Q_cell / 3600)})(
         max(1, math.floor(E_target / (ns * V_cell * Q_cell)))))(max(1, math.floor(V_target / V_cell)))),
     random_inputs=lambda rng: {"V_target": round(rng.uniform(12, 800), 1), "E_target": round(rng.uniform(0.5, 100), 2) * 3.6e6, "V_cell": rng.choice([3.2, 3.6, 3.7, 2.0, 1.2]),
                                "Q_cell": rng.choice([2.5, 3.0, 5.0, 50.0, 100.0, 280.0]) * 3600},
     templates=["Battery pack sizing: V_target = 48 V, E_target = 10 kWh, V_cell = 3.2 V, Q_cell = 100 Ah"],
     examples=[{"inputs": {"V_target": 48, "E_target": 3.6e7, "V_cell": 3.2, "Q_cell": 360000}, "outputs": {"N_series": 15, "N_parallel": 3, "N_cells": 45, "V_pack": 48.0}},
               {"inputs": {"V_target": 400, "E_target": 2.16e8, "V_cell": 3.7, "Q_cell": 18000}, "outputs": {"N_series": 109, "N_parallel": 30}}],
     questions=[{"q": "Battery pack sizing: V_target = 48 V, E_target = 10 kWh, V_cell = 3.2 V, Q_cell = 100 Ah", "outputs": {"N_series": 15, "N_parallel": 3}},
                {"q": "Battery pack sizing: V_target = 48 V, E_target = 10 kWh, V_cell = 0 V, Q_cell = 100 Ah", "outcome": "INVALID_INPUT"}])


def battery_rc_discharge(OCV, R0, R1, C1, I, t):
    _pos("the open-circuit voltage, R1 and C1", OCV, R1, C1)
    _nonneg("R0, the current and the time", R0, I, t)
    tau = R1 * C1
    e = math.exp(-t / tau)
    V1 = I * R1 * (1 - e)
    Vt = OCV - I * R0 - V1
    _need(Vt > 0, "the terminal voltage would fall to zero: the current is too large for this cell model")
    E = I * ((OCV - I * R0 - I * R1) * t + I * R1 * tau * (1 - e))
    heat = I * I * R0 * t + I * I * R1 * (t - 2 * tau * (1 - e) + tau / 2 * (1 - math.exp(-2 * t / tau)))
    return _out(V_terminal=Vt, V_rc=V1, E_delivered=E, E_Wh=E / 3600, heat=heat, tau=tau)


def _ref_rc(OCV, R0, R1, C1, I, t):
    """The equivalent circuit's state equation C1 dV1/dt = I - V1/R1 integrated with the delivered energy and the heat
    as extra states (scipy solve_ivp, DOP853)."""
    from scipy import integrate
    tau = R1 * C1
    if t == 0:
        return {"V_terminal": OCV - I * R0, "V_rc": 0.0, "E_delivered": 0.0, "E_Wh": 0.0, "heat": 0.0, "tau": tau}
    f = lambda tt, y: [(I - y[0] / R1) / C1, (OCV - I * R0 - y[0]) * I, I * I * R0 + y[0] * y[0] / R1]   # noqa: E731
    sol = integrate.solve_ivp(f, (0, t), [0.0, 0.0, 0.0], method="DOP853", rtol=1e-13, atol=1e-14 * max(1.0, OCV * I * t))
    V1, E, H = sol.y[:, -1]
    return {"V_terminal": OCV - I * R0 - V1, "V_rc": V1, "E_delivered": E, "E_Wh": E / 3600, "heat": H, "tau": tau}


_add("energy_battery_rc_discharge",
     name="Battery terminal voltage under constant current (one-RC equivalent circuit)", topic="batteries / equivalent circuits",
     purpose="Terminal voltage, polarization (RC) voltage, energy delivered and heat generated after a constant-current discharge of duration t, for a cell modelled as an open-circuit voltage with a series resistance R0 and one R1 || C1 branch (constant OCV).",
     inputs=[_phys("OCV", "voltage", "open-circuit voltage (held constant)"), _phys("R0", "resistance", "series (ohmic) resistance"), _phys("R1", "resistance", "polarization resistance"),
             _phys("C1", "capacitance", "polarization capacitance"), _phys("I", "current", "discharge current"), _phys("t", "time", "time since the current step")],
     outputs=[_o("V_terminal", "voltage", "terminal voltage"), _o("V_rc", "voltage", "voltage across the RC branch"), _o("E_delivered", "energy", "energy delivered to the load"),
              _o("E_Wh", "number", "energy delivered in Wh"), _o("heat", "energy", "heat generated in R0 and R1"), _o("tau", "time", "time constant R1 C1")],
     primary="V_terminal", conditions=["constant open-circuit voltage over the interval (short pulse or flat OCV curve)", "RC branch initially relaxed"],
     method="V1 = I R1 (1 - e^(-t/tau)), V_t = OCV - I R0 - V1; energy and heat integrated in closed form",
     latex=r"V_t(t) = V_{oc} - IR_0 - IR_1\left(1 - e^{-t/R_1C_1}\right)",
     run=battery_rc_discharge,
     parse=lambda q: _pairs(q, r"battery (?:rc|one[- ]rc|thevenin) (?:model|discharge)|one[- ]rc battery model|battery terminal voltage", {k: k for k in ("OCV", "R0", "R1", "C1", "I", "t")},
                            phys={"OCV", "R0", "R1", "C1", "I", "t"}),
     reference=_ref_rc,
     wrong=_safe(lambda OCV, R0, R1, C1, I, t: (lambda o: dict(o, V_rc=_r(I * R1 * math.exp(-t / (R1 * C1))), V_terminal=_r(OCV - I * R0 - I * R1 * math.exp(-t / (R1 * C1)))))(
         battery_rc_discharge(OCV, R0, R1, C1, I, t))),
     random_inputs=lambda rng: (lambda R1, C1: {"OCV": round(rng.uniform(3.0, 4.2), 3), "R0": round(rng.uniform(0.005, 0.05), 5), "R1": R1, "C1": C1, "I": round(rng.uniform(0.5, 20), 3),
                                                "t": round(R1 * C1 * rng.uniform(0.1, 5), 6)})(round(rng.uniform(0.005, 0.05), 5), round(rng.uniform(500, 5000), 1)),
     templates=["Battery RC model: OCV = 3.7 V, R0 = 20 mohm, R1 = 15 mohm, C1 = 2000 F, I = 10 A, t = 60 s"],
     examples=[{"inputs": {"OCV": 4.0, "R0": 0.01, "R1": 0.02, "C1": 1000, "I": 10, "t": 0}, "outputs": {"V_terminal": 3.9, "V_rc": 0.0}},
               {"inputs": {"OCV": 4.0, "R0": 0.01, "R1": 0.02, "C1": 1000, "I": 10, "t": 20 * math.log(2)}, "outputs": {"V_rc": 0.1, "V_terminal": 3.8, "tau": 20.0}}],
     questions=[{"q": "Battery RC model: OCV = 4 V, R0 = 10 mohm, R1 = 20 mohm, C1 = 1000 F, I = 10 A, t = 0 s", "outputs": {"V_terminal": 3.9}},
                {"q": "Battery RC model: OCV = 4 V, R0 = 10 mohm, R1 = 20 mohm, C1 = 1000 F, I = 500 A, t = 60 s", "outcome": "INVALID_INPUT"}])


# ================================================================================================ FUEL CELLS AND HYDROGEN
def fuel_cell_stack(I, N, V_cell, utilization=1.0):
    _pos("the current and the cell voltage", I, V_cell)
    _need(isinstance(N, int) and N >= 1, "the number of cells must be a whole number >= 1")
    _need(V_cell < 1.25, "a hydrogen fuel cell's voltage must stay below the thermoneutral (LHV) voltage of about 1.25 V")
    _need(0 < utilization <= 1, "the fuel utilization must lie in (0, 1]")
    n_h2 = N * I / (2 * F_FARADAY)
    P = N * I * V_cell
    return _out(H2_kg_s=n_h2 * M_H2, H2_g_h=n_h2 * M_H2 * 3.6e6, H2_fed_kg_s=n_h2 * M_H2 / utilization, O2_kg_s=n_h2 / 2 * M_O2, H2O_kg_s=n_h2 * M_H2O,
                efficiency_LHV=P / (n_h2 * LHV_H2), efficiency_HHV=P / (n_h2 * HHV_H2), heat_LHV=n_h2 * LHV_H2 - P)


def _balances():
    """Charge numbers and stoichiometry from element and charge balances (numpy linear solves)."""
    import numpy as np
    # anode: H2 -> a H+ + b e-   (H balance, charge balance)
    a, b = np.linalg.solve(np.array([[1.0, 0.0], [1.0, -1.0]]), np.array([2.0, 0.0]))
    # overall: H2 + x O2 -> y H2O   (H balance, O balance)
    y, x = np.linalg.solve(np.array([[2.0, 0.0], [1.0, -2.0]]), np.array([2.0, 0.0]))
    return float(b), float(x), float(y)


def _ref_fuel_cell(I, N, V_cell, utilization=1.0):
    ne, xo2, yh2o = _balances()
    n_h2 = N * I / (ne * F_FARADAY)
    P = N * I * V_cell
    return {"H2_kg_s": n_h2 * M_H2, "H2_g_h": n_h2 * M_H2 * 1000 * 3600, "H2_fed_kg_s": n_h2 * M_H2 / utilization, "O2_kg_s": xo2 * n_h2 * M_O2,
            "H2O_kg_s": yh2o * n_h2 * M_H2O, "efficiency_LHV": P / (n_h2 * LHV_H2), "efficiency_HHV": P / (n_h2 * HHV_H2), "heat_LHV": n_h2 * LHV_H2 - P}


_add("energy_fuel_cell_stack",
     name="Hydrogen fuel-cell stack: consumption, power and efficiency", topic="fuel cells / hydrogen fuel cells",
     purpose="Hydrogen and oxygen consumption, water production (Faraday's law), efficiency on the lower and higher heating value and heat released of a hydrogen fuel-cell stack of N cells at a current and cell voltage.",
     inputs=[_phys("I", "current", "stack current"), _intin("N", "number of cells in series", 1, 100000), _phys("V_cell", "voltage", "average cell voltage"),
             _numin("utilization", "fuel utilization (fraction or %, default 1)", required=False)],
     outputs=[_o("H2_kg_s", "mass_flow_rate", "hydrogen consumed"), _o("H2_g_h", "number", "hydrogen consumed in g/h"), _o("H2_fed_kg_s", "mass_flow_rate", "hydrogen fed (consumed/utilization)"),
              _o("O2_kg_s", "mass_flow_rate", "oxygen consumed"), _o("H2O_kg_s", "mass_flow_rate", "water produced"),
              _o("efficiency_LHV", "number", "efficiency on the lower heating value"), _o("efficiency_HHV", "number", "efficiency on the higher heating value"),
              _o("heat_LHV", "power", "heat released (water vapour product)")],
     primary="H2_kg_s", conditions=["hydrogen/oxygen cell, 2 electrons per H2", "heating values at 25 degC"],
     method="n_H2 = N I/(2F); n_O2 = n_H2/2; n_H2O = n_H2; P = N I V; eta = P/(n_H2 LHV)", latex=r"\dot n_{H_2} = \frac{N I}{2F},\quad \eta_{LHV} = \frac{N I V_{cell}}{\dot n_{H_2}\,\Delta h_{LHV}}",
     run=fuel_cell_stack,
     parse=lambda q: _pairs(q, r"fuel[- ]cell stack|fuel[- ]cell hydrogen consumption|hydrogen fuel cell", {"I": "I", "current": "I", "N": "N", "cells": "N", "V_cell": "V_cell", "V": "V_cell",
                                                                                                   "utilization": "utilization", "u": "utilization"},
                            phys={"I", "V_cell"}, num_units=_pct_units("utilization")),
     reference=_ref_fuel_cell,
     wrong=_safe(lambda I, N, V_cell, utilization=1.0: (lambda o: dict(o, H2_kg_s=_r(N * I / F_FARADAY * M_H2), H2_g_h=_r(N * I / F_FARADAY * M_H2 * 3.6e6)))(fuel_cell_stack(I, N, V_cell, utilization))),
     random_inputs=lambda rng: {"I": round(rng.uniform(1, 500), 3), "N": rng.randint(1, 400), "V_cell": round(rng.uniform(0.5, 0.9), 4), "utilization": round(rng.uniform(0.7, 1.0), 3)},
     templates=["Fuel-cell stack: I = 200 A, N = 100, V_cell = 0.7 V"],
     examples=[{"inputs": {"I": 2 * F_FARADAY, "N": 1, "V_cell": 0.7}, "outputs": {"H2_kg_s": M_H2, "O2_kg_s": M_O2 / 2, "efficiency_LHV": _r(2 * F_FARADAY * 0.7 / LHV_H2)}}],
     questions=[{"q": "Fuel-cell stack: I = 192970.66424 A, N = 1, V_cell = 0.7 V", "outputs": {"H2_kg_s": M_H2}},
                {"q": "Fuel-cell stack: I = 200 A, N = 100, V_cell = 1.4 V", "outcome": "INVALID_INPUT"}])


def electrolyser(I, N, V_cell, faradaic=1.0):
    _pos("the current", I)
    _need(isinstance(N, int) and N >= 1, "the number of cells must be a whole number >= 1")
    _need(1.23 <= V_cell <= 3.0, "an electrolysis cell operates between the reversible voltage 1.23 V and about 3 V")
    _need(0 < faradaic <= 1, "the Faradaic efficiency must lie in (0, 1]")
    n_h2 = faradaic * N * I / (2 * F_FARADAY)
    P = N * I * V_cell
    return _out(H2_kg_s=n_h2 * M_H2, H2_kg_h=n_h2 * M_H2 * 3600, O2_kg_s=n_h2 / 2 * M_O2, H2O_kg_s=n_h2 * M_H2O,
                specific_energy_kWh_per_kg=P / (n_h2 * M_H2) / 3.6e6, efficiency_HHV=n_h2 * HHV_H2 / P, efficiency_LHV=n_h2 * LHV_H2 / P)


def _ref_electrolyser(I, N, V_cell, faradaic=1.0):
    ne, xo2, yh2o = _balances()
    n_h2 = faradaic * N * I / (ne * F_FARADAY)
    P = N * I * V_cell
    return {"H2_kg_s": n_h2 * M_H2, "H2_kg_h": n_h2 * M_H2 * 3600, "O2_kg_s": xo2 * n_h2 * M_O2, "H2O_kg_s": yh2o * n_h2 * M_H2O,
            "specific_energy_kWh_per_kg": P / (n_h2 * M_H2) / 3.6e6, "efficiency_HHV": n_h2 * HHV_H2 / P, "efficiency_LHV": n_h2 * LHV_H2 / P}


_add("energy_electrolyser",
     name="Water electrolyser: hydrogen production and energy per kilogram", topic="fuel cells / electrolysis",
     purpose="Hydrogen and oxygen production, water consumption, specific energy (kWh per kg H2) and efficiency on the higher and lower heating value of a water electrolyser of N cells.",
     inputs=[_phys("I", "current", "stack current"), _intin("N", "number of cells in series", 1, 100000), _phys("V_cell", "voltage", "cell voltage (1.23 to 3 V)"),
             _numin("faradaic", "Faradaic (current) efficiency, fraction or % (default 1)", required=False)],
     outputs=[_o("H2_kg_s", "mass_flow_rate", "hydrogen produced"), _o("H2_kg_h", "number", "hydrogen produced in kg/h"), _o("O2_kg_s", "mass_flow_rate", "oxygen produced"),
              _o("H2O_kg_s", "mass_flow_rate", "water consumed"), _o("specific_energy_kWh_per_kg", "number", "electricity per kg of hydrogen in kWh/kg"),
              _o("efficiency_HHV", "number", "efficiency on the higher heating value"), _o("efficiency_LHV", "number", "efficiency on the lower heating value")],
     primary="specific_energy_kWh_per_kg", conditions=["water electrolysis, 2 electrons per H2", "heating values at 25 degC"],
     method="n_H2 = eta_F N I/(2F); specific energy = N I V/(n_H2 M_H2); eta_HHV = n_H2 HHV/P", latex=r"\frac{E}{m_{H_2}} = \frac{2FV_{cell}}{\eta_F M_{H_2}}",
     run=electrolyser,
     parse=lambda q: _pairs(q, r"electroly[sz]er|water electrolysis|hydrogen production by electrolysis", {"I": "I", "current": "I", "N": "N", "cells": "N", "V_cell": "V_cell", "V": "V_cell",
                                                                                                    "faradaic": "faradaic", "eta_F": "faradaic"},
                            phys={"I", "V_cell"}, num_units=_pct_units("faradaic")),
     reference=_ref_electrolyser,
     wrong=_safe(lambda I, N, V_cell, faradaic=1.0: (lambda o: dict(o, specific_energy_kWh_per_kg=_r(o["specific_energy_kWh_per_kg"] / 2), H2_kg_s=_r(o["H2_kg_s"] * 2), H2_kg_h=_r(o["H2_kg_h"] * 2)))(
         electrolyser(I, N, V_cell, faradaic))),
     random_inputs=lambda rng: {"I": round(rng.uniform(10, 5000), 2), "N": rng.randint(1, 300), "V_cell": round(rng.uniform(1.6, 2.2), 4), "faradaic": round(rng.uniform(0.9, 1.0), 4)},
     templates=["Electrolyser: I = 1000 A, N = 100, V_cell = 1.9 V"],
     examples=[{"inputs": {"I": 2 * F_FARADAY, "N": 1, "V_cell": 2.0}, "outputs": {"H2_kg_s": M_H2, "specific_energy_kWh_per_kg": _r(2 * F_FARADAY * 2.0 / M_H2 / 3.6e6)}}],
     questions=[{"q": "Electrolyser: I = 192970.66424 A, N = 1, V_cell = 2 V", "outputs": {"specific_energy_kWh_per_kg": _r(2 * F_FARADAY * 2.0 / M_H2 / 3.6e6)}},
                {"q": "Electrolyser: I = 1000 A, N = 100, V_cell = 1 V", "outcome": "INVALID_INPUT"}])


# --- hydrogen properties: Leachman et al. (2009) equation of state for normal hydrogen (as in CoolProp) -------------
_H2_R = 8.314472                       # J/(mol K), the equation's gas constant
_H2_TC, _H2_RHOC = 33.145, 15508.0     # reducing temperature (K) and molar density (mol/m^3)
_LP_N = [-6.93643, 0.01, 2.1101, 4.52059, 0.732564, -1.34086, 0.130985, -0.777414, 0.351944]
_LP_T = [0.6844, 1, 0.989, 0.489, 0.803, 1.1444, 1.409, 1.754, 1.311]
_LP_D = [1, 4, 1, 1, 2, 2, 3, 1, 3]
_LP_L = [0, 0, 0, 0, 0, 0, 0, 1, 1]
_LG_N = [-0.0211716, 0.0226312, 0.032187, -0.0231752, 0.0557346]
_LG_T = [4.187, 5.646, 0.791, 7.249, 2.986]
_LG_D = [2, 1, 3, 1, 1]
_LG_ETA = [1.685, 0.489, 0.103, 2.506, 1.607]
_LG_EPS = [1.506, 0.156, 1.736, 0.67, 1.662]
_LG_BETA = [0.171, 0.2245, 0.1304, 0.2785, 0.3967]
_LG_GAM = [0.7164, 1.3444, 1.4517, 0.7204, 1.5445]
_L0_A1, _L0_A2, _L0_LOGTAU = -1.4579856475, 1.888076782, 1.5
_L0_N = [1.616, -0.4117, -0.792, 0.758, 1.217]
_L0_V = [531, 751, 1989, 2484, 6859]


def _h2_ar(d, t):
    """Residual Helmholtz energy and its derivatives: (ar, d ar/d delta, d ar/d tau)."""
    ar = ard = art = 0.0
    for n, tt, dd, l in zip(_LP_N, _LP_T, _LP_D, _LP_L):
        e = math.exp(-d ** l) if l else 1.0
        term = n * d ** dd * t ** tt * e
        ar += term
        ard += term * (dd - l * d ** l) / d
        art += term * tt / t
    for n, tt, dd, eta, eps, beta, gam in zip(_LG_N, _LG_T, _LG_D, _LG_ETA, _LG_EPS, _LG_BETA, _LG_GAM):
        term = n * d ** dd * t ** tt * math.exp(-eta * (d - eps) ** 2 - beta * (t - gam) ** 2)
        ar += term
        ard += term * (dd / d - 2 * eta * (d - eps))
        art += term * (tt / t - 2 * beta * (t - gam))
    return ar, ard, art


def _h2_a0(d, t):
    """Ideal-gas Helmholtz energy (up to a constant) and its tau derivative."""
    a0 = math.log(d) + _L0_A1 + _L0_A2 * t + _L0_LOGTAU * math.log(t)
    a0t = _L0_A2 + _L0_LOGTAU / t
    for n, v in zip(_L0_N, _L0_V):
        x = v * t / _H2_TC
        a0 += n * math.log(1 - math.exp(-x))
        a0t += n * (v / _H2_TC) / (math.exp(x) - 1)
    return a0, a0t


def _h2_p(rho, T):
    d, t = rho / _H2_RHOC, _H2_TC / T
    return rho * _H2_R * T * (1 + d * _h2_ar(d, t)[1])


def _h2_rho(p, T):
    """Molar density at p, T (gas, T >= 200 K, where 1 <= Z < 1000): root of p(rho) = p between p/(1000 R T) and the
    ideal-gas density p/(R T)."""
    hi = p / (_H2_R * T)
    return _illinois(lambda r: _h2_p(r, T) / p - 1.0, hi * 1e-3, hi)


def _h2_check(p, T):
    _need(1e3 <= p <= 1.5e8, "the pressure must lie between 1 kPa and 1500 bar")
    _need(200 <= T <= 700, "the temperature must lie between 200 K and 700 K (gaseous hydrogen)")


def hydrogen_tank(V, p, T):
    _pos("the tank volume", V)
    _h2_check(p, T)
    rho_m = _h2_rho(p, T)
    rho = rho_m * M_H2
    rho_id = p / (_H2_R * T) * M_H2
    m = rho * V
    return _out(rho=rho, Z=p / (rho_m * _H2_R * T), mass=m, rho_ideal=rho_id, mass_ideal=rho_id * V, ideal_over_real=rho_id / rho,
                energy_LHV_kWh=m / M_H2 * LHV_H2 / 3.6e6)


def _ref_h2_tank(V, p, T):
    from CoolProp.CoolProp import PropsSI
    rho = PropsSI("D", "P", p, "T", T, "Hydrogen")
    Z = PropsSI("Z", "P", p, "T", T, "Hydrogen")
    rho_id = rho * Z
    return {"rho": rho, "Z": Z, "mass": rho * V, "rho_ideal": rho_id, "mass_ideal": rho_id * V, "ideal_over_real": Z, "energy_LHV_kWh": rho * V / M_H2 * LHV_H2 / 3.6e6}


_add("energy_hydrogen_tank",
     name="Hydrogen stored in a tank at pressure (real gas)", topic="hydrogen / storage",
     purpose="Density, compressibility factor, stored mass and energy (lower heating value) of compressed hydrogen in a tank at a pressure and temperature, by the Leachman (2009) equation of state for normal hydrogen, compared with the ideal-gas estimate.",
     inputs=[_phys("V", "volume", "internal tank volume"), _phys("p", "pressure", "gas pressure (absolute)"), _phys("T", "temperature", "gas temperature")],
     outputs=[_o("rho", "density", "hydrogen density (real gas)"), _o("Z", "number", "compressibility factor"), _o("mass", "mass", "stored hydrogen mass"),
              _o("rho_ideal", "density", "ideal-gas density p M/(R T)"), _o("mass_ideal", "mass", "ideal-gas mass"), _o("ideal_over_real", "number", "ideal-gas over real density (= Z)"),
              _o("energy_LHV_kWh", "number", "stored chemical energy (LHV) in kWh")],
     primary="mass", conditions=["1 kPa <= p <= 1500 bar", "200 K <= T <= 700 K", "normal hydrogen, uniform state"],
     method="Leachman 2009 Helmholtz equation: p = rho R T (1 + delta d(alpha_r)/d(delta)), solved for rho; Z = p/(rho R T)",
     latex=r"p = \rho R T\left(1 + \delta\,\frac{\partial\alpha^r}{\partial\delta}\right),\quad Z = \frac{p}{\rho R T}",
     run=hydrogen_tank,
     parse=lambda q: _pairs(q, r"hydrogen (?:tank|storage)(?: at pressure)?|compressed hydrogen", {"V": "V", "volume": "V", "p": "p", "pressure": "p", "T": "T", "temperature": "T"}, phys={"V", "p", "T"}),
     reference=_ref_h2_tank,
     wrong=_safe(lambda V, p, T: (lambda r: dict(r, rho=r["rho_ideal"], mass=r["mass_ideal"], Z=1.0, energy_LHV_kWh=_r(r["mass_ideal"] / M_H2 * LHV_H2 / 3.6e6)))(hydrogen_tank(V, p, T))),
     random_inputs=lambda rng: {"V": round(rng.uniform(0.01, 2), 4), "p": round(_lu(rng, 1e5, 1e8), 0), "T": round(rng.uniform(220, 400), 2)},
     templates=["Hydrogen tank: V = 125 L, p = 700 bar, T = 15 degC"],
     examples=[{"inputs": {"V": 1.0, "p": 7e7, "T": 288.15}, "outputs": {"rho": 40.1721610778, "Z": 1.46616963787}}],
     questions=[{"q": "Hydrogen tank: V = 1 m^3, p = 700 bar, T = 288.15 K", "outputs": {"rho": 40.1721610778}},
                {"q": "Hydrogen tank: V = 1 m^3, p = 700 bar, T = 50 K", "outcome": "INVALID_INPUT"}])


def hydrogen_isothermal_compression(p1, p2, T):
    _h2_check(p1, T)
    _h2_check(p2, T)
    _need(p2 > p1, "the final pressure must exceed the initial pressure")
    t = _H2_TC / T
    g = []
    for p in (p1, p2):
        rm = _h2_rho(p, T)
        d = rm / _H2_RHOC
        ar, ard, _ = _h2_ar(d, t)
        g.append((math.log(d) + ar + d * ard) * _H2_R * T)
    w = (g[1] - g[0]) / M_H2
    wid = _H2_R * T * math.log(p2 / p1) / M_H2
    return _out(w=w, w_kWh_per_kg=w / 3.6e6, w_ideal=wid, real_over_ideal=w / wid)


def _ref_h2_isothermal(p1, p2, T):
    """CoolProp's Gibbs energy at both states: reversible isothermal work = g2 - g1 (integral of v dp at constant T);
    the ideal-gas value from CoolProp's own low-pressure limit of p/rho."""
    from CoolProp.CoolProp import PropsSI
    w = PropsSI("Gmass", "P", p2, "T", T, "Hydrogen") - PropsSI("Gmass", "P", p1, "T", T, "Hydrogen")
    # ideal gas: CoolProp's low-pressure limit, p/rho at a vanishing pressure, gives R T/M
    rt = 1e-4 / PropsSI("D", "P", 1e-4, "T", T, "Hydrogen")
    wid = rt * math.log(p2 / p1)
    return {"w": w, "w_kWh_per_kg": w / 3.6e6, "w_ideal": wid, "real_over_ideal": w / wid}


_add("energy_hydrogen_isothermal_compression",
     name="Minimum (isothermal) work to compress hydrogen", topic="hydrogen / compression",
     purpose="The reversible isothermal work per kilogram to compress hydrogen from p1 to p2 at temperature T (the integral of v dp, equal to the Gibbs-energy rise) for the real gas (Leachman equation of state) and the ideal gas.",
     inputs=[_phys("p1", "pressure", "initial pressure"), _phys("p2", "pressure", "final pressure"), _phys("T", "temperature", "compression temperature")],
     outputs=[_o("w", "specific_energy", "real-gas isothermal work per kg"), _o("w_kWh_per_kg", "number", "the same in kWh/kg"),
              _o("w_ideal", "specific_energy", "ideal-gas work R T ln(p2/p1)/M"), _o("real_over_ideal", "number", "real over ideal work")],
     primary="w", conditions=["reversible and isothermal", "200 K <= T <= 700 K, pressures up to 1500 bar"],
     method="w = g(p2, T) - g(p1, T), g/(RT) = ln(delta) + alpha_r + delta d(alpha_r)/d(delta) + (terms in T only)",
     latex=r"w = \int_{p_1}^{p_2} v\,dp = g(p_2,T) - g(p_1,T),\quad w_{ideal} = \frac{RT}{M}\ln\frac{p_2}{p_1}",
     run=hydrogen_isothermal_compression,
     parse=lambda q: _pairs(q, r"hydrogen isothermal compression|isothermal compression of hydrogen|hydrogen compression work", {"p1": "p1", "p2": "p2", "T": "T"}, phys={"p1", "p2", "T"}),
     reference=_ref_h2_isothermal,
     wrong=_safe(lambda p1, p2, T: (lambda o: dict(o, w=o["w_ideal"], w_kWh_per_kg=_r(o["w_ideal"] / 3.6e6), real_over_ideal=1.0))(hydrogen_isothermal_compression(p1, p2, T))),
     random_inputs=lambda rng: (lambda p1: {"p1": p1, "p2": round(p1 * _lu(rng, 3, 500), 0), "T": round(rng.uniform(250, 350), 2)})(round(_lu(rng, 1e5, 5e6), 0)),
     templates=["Hydrogen isothermal compression: p1 = 20 bar, p2 = 700 bar, T = 300 K"],
     examples=[{"inputs": {"p1": 1e5, "p2": 2e5, "T": 300}, "outputs": {"w_ideal": _r(8.314472 * 300 * math.log(2) / M_H2)}}],
     questions=[{"q": "Hydrogen isothermal compression: p1 = 1 bar, p2 = 2 bar, T = 300 K", "outputs": {"w_ideal": _r(8.314472 * 300 * math.log(2) / M_H2)}},
                {"q": "Hydrogen isothermal compression: p1 = 700 bar, p2 = 20 bar, T = 300 K", "outcome": "INVALID_INPUT"}])


def _h2_s_h(p, T):
    rm = _h2_rho(p, T)
    d, t = rm / _H2_RHOC, _H2_TC / T
    ar, ard, art = _h2_ar(d, t)
    a0, a0t = _h2_a0(d, t)
    s = _H2_R * (t * (a0t + art) - a0 - ar)
    h = _H2_R * T * (1 + t * (a0t + art) + d * ard)
    return s / M_H2, h / M_H2


def hydrogen_isentropic_compression(p1, T1, p2):
    _h2_check(p1, T1)
    _need(p2 > p1, "the final pressure must exceed the initial pressure")
    _need(p2 <= 1.5e8, "the final pressure must not exceed 1500 bar")
    s1, h1 = _h2_s_h(p1, T1)
    T2 = _illinois(lambda T: _h2_s_h(p2, T)[0] - s1, T1, 20 * T1)
    _need(T2 <= 1000, "the outlet temperature leaves the range of the equation of state (1000 K)")
    h2 = _h2_s_h(p2, T2)[1]
    return _out(T2=T2, w=h2 - h1, w_kWh_per_kg=(h2 - h1) / 3.6e6)


def _ref_h2_isentropic(p1, T1, p2):
    from CoolProp.CoolProp import PropsSI
    s1 = PropsSI("Smass", "P", p1, "T", T1, "Hydrogen")
    h1 = PropsSI("Hmass", "P", p1, "T", T1, "Hydrogen")
    from scipy.optimize import brentq            # CoolProp's (p, s) flash is looser than its (p, T) properties
    T2 = brentq(lambda T: PropsSI("Smass", "P", p2, "T", T, "Hydrogen") - s1, T1, 20 * T1, xtol=1e-12, rtol=8.9e-16, maxiter=300)
    h2 = PropsSI("Hmass", "P", p2, "T", T2, "Hydrogen")
    return {"T2": T2, "w": h2 - h1, "w_kWh_per_kg": (h2 - h1) / 3.6e6}


_add("energy_hydrogen_isentropic_compression",
     name="Isentropic (adiabatic) compression of hydrogen", topic="hydrogen / compression",
     purpose="Outlet temperature and work per kilogram of a reversible adiabatic compression of hydrogen from (p1, T1) to p2, from the full Leachman equation of state (real gas, variable heat capacity).",
     inputs=[_phys("p1", "pressure", "inlet pressure"), _phys("T1", "temperature", "inlet temperature"), _phys("p2", "pressure", "outlet pressure")],
     outputs=[_o("T2", "temperature", "outlet temperature"), _o("w", "specific_energy", "compression work per kg (h2 - h1)"), _o("w_kWh_per_kg", "number", "work in kWh/kg")],
     primary="w", conditions=["reversible adiabatic, single stage", "outlet temperature below 1000 K"],
     method="s(p2, T2) = s(p1, T1) solved for T2 by bisection; w = h(p2, T2) - h(p1, T1); s and h from the Helmholtz energy (ideal + residual parts)",
     latex=r"s(p_2, T_2) = s(p_1, T_1),\quad w = h_2 - h_1",
     run=hydrogen_isentropic_compression,
     parse=lambda q: _pairs(q, r"hydrogen isentropic compression|isentropic compression of hydrogen|adiabatic compression of hydrogen", {"p1": "p1", "T1": "T1", "p2": "p2"}, phys={"p1", "T1", "p2"}),
     reference=_ref_h2_isentropic,
     wrong=_safe(lambda p1, T1, p2: (lambda T2: _out(T2=T2, w=14300.0 * (T2 - T1), w_kWh_per_kg=14300.0 * (T2 - T1) / 3.6e6))(T1 * (p2 / p1) ** (0.4 / 1.4))),
     random_inputs=lambda rng: (lambda p1: {"p1": p1, "T1": round(rng.uniform(250, 330), 2), "p2": round(p1 * _lu(rng, 1.5, 8), 0)})(round(_lu(rng, 1e5, 1e7), 0)),
     templates=["Hydrogen isentropic compression: p1 = 30 bar, T1 = 300 K, p2 = 100 bar"],
     examples=[{"inputs": {"p1": 3e6, "T1": 300.0, "p2": 1e7}, "outputs": {"T2": 423.293556425, "w": 1833212.52408}}],
     questions=[{"q": "Hydrogen isentropic compression: p1 = 30 bar, T1 = 300 K, p2 = 100 bar", "outputs": {"T2": 423.293556425}},
                {"q": "Hydrogen isentropic compression: p1 = 30 bar, T1 = 300 K, p2 = 10 bar", "outcome": "INVALID_INPUT"}])


# ================================================================================================ PV SINGLE-DIODE MODEL
def _sd_check(IL, I0, Rs, Rsh, a):
    _pos("the photocurrent, saturation current, shunt resistance and nNsVth", IL, I0, Rsh, a)
    _nonneg("the series resistance", Rs)


def _sd_g(I, V, IL, I0, Rs, Rsh, a):
    x = (V + I * Rs) / a
    if x > 700:
        return -math.inf
    return IL - I0 * math.expm1(x) - (V + I * Rs) / Rsh - I


def _sd_current(V, IL, I0, Rs, Rsh, a):
    lo, hi = -1.0, IL + I0 + 1.0
    while _sd_g(lo, V, IL, I0, Rs, Rsh, a) < 0:
        lo *= 2
        _need(lo > -1e12, "no solution for the current at this voltage")
    return _bisect(lambda I: _sd_g(I, V, IL, I0, Rs, Rsh, a), lo, hi, 400)


def _sd_voltage(I, IL, I0, Rs, Rsh, a):
    hi = 1.0
    while _sd_g(I, hi, IL, I0, Rs, Rsh, a) > 0:
        hi *= 2
        _need(hi < 1e9, "no solution for the voltage at this current")
    lo = -1.0
    while _sd_g(I, lo, IL, I0, Rs, Rsh, a) < 0:
        lo *= 2
        _need(lo > -1e12, "no solution for the voltage at this current")
    return _bisect(lambda V: _sd_g(I, V, IL, I0, Rs, Rsh, a), lo, hi, 400)


def single_diode_current(V, IL, I0, Rs, Rsh, nNsVth):
    _sd_check(IL, I0, Rs, Rsh, nNsVth)
    I = _sd_current(V, IL, I0, Rs, Rsh, nNsVth)
    return _out(I=I, P=V * I)


def _ref_sd_i(V, IL, I0, Rs, Rsh, nNsVth):
    from pvlib import pvsystem
    I = _np_scalar(pvsystem.i_from_v(V, IL, I0, Rs, Rsh, nNsVth, method="lambertw"))
    return {"I": I, "P": V * I}


def _rnd_sd(rng):
    ncell = rng.choice([1, 36, 60, 72])
    a = round(ncell * rng.uniform(1.0, 1.5) * 0.025693, 6)
    IL = round(rng.uniform(1, 12), 4)
    I0 = 10 ** rng.uniform(-11, -8)
    return {"IL": IL, "I0": float(f"{I0:.4g}"), "Rs": round(rng.uniform(0.0, 0.01 * ncell), 5), "Rsh": round(rng.uniform(5, 50) * ncell, 3), "nNsVth": a}


def _sd_voc(IL, I0, Rs, Rsh, a):
    return _sd_voltage(0.0, IL, I0, Rs, Rsh, a)


_SD_IN = [_phys("IL", "current", "photocurrent (light-generated current)"), _phys("I0", "current", "diode saturation current"),
          _phys("Rs", "resistance", "series resistance"), _phys("Rsh", "resistance", "shunt resistance"), _phys("nNsVth", "voltage", "modified ideality factor n Ns k T/q")]
_SD_NAMES = {"V": "V", "I": "I", "IL": "IL", "Iph": "IL", "I0": "I0", "Rs": "Rs", "Rsh": "Rsh", "nNsVth": "nNsVth", "a": "nNsVth"}

_add("energy_single_diode_current",
     name="PV single-diode model: current at a voltage", topic="photovoltaics / single-diode model",
     purpose="The current (and power) of a PV cell or module at a terminal voltage from the five-parameter single-diode equation I = IL - I0 [exp((V + I Rs)/a) - 1] - (V + I Rs)/Rsh, solved implicitly.",
     inputs=[_phys("V", "voltage", "terminal voltage")] + _SD_IN,
     outputs=[_o("I", "current", "terminal current"), _o("P", "power", "power V I")], primary="I", conditions=["IL, I0, Rsh, a > 0", "Rs >= 0"],
     method="the implicit single-diode equation solved for I by bisection to full precision", latex=r"I = I_L - I_0\left[e^{(V + IR_s)/a} - 1\right] - \frac{V + IR_s}{R_{sh}}",
     run=single_diode_current,
     parse=lambda q: _pairs(q, r"single[- ]diode (?:model )?current|pv current at a voltage|single[- ]diode current at a voltage", _SD_NAMES, phys={"V", "IL", "I0", "Rs", "Rsh", "nNsVth"}),
     reference=_ref_sd_i,
     wrong=_safe(lambda V, IL, I0, Rs, Rsh, nNsVth: single_diode_current(V, IL, I0, -Rs if Rs > 0 else 0.0, Rsh, nNsVth) if Rs > 0 else _out(I=IL - V / Rsh, P=V * (IL - V / Rsh))),
     random_inputs=lambda rng: (lambda x: dict(x, V=round(_sd_voc(x["IL"], x["I0"], x["Rs"], x["Rsh"], x["nNsVth"]) * rng.uniform(0.0, 0.9), 6)))(_rnd_sd(rng)),
     templates=["Single-diode current: V = 30 V, IL = 9 A, I0 = 1e-10 A, Rs = 0.3 ohm, Rsh = 400 ohm, nNsVth = 1.9 V"],
     examples=[{"inputs": {"V": 0.0, "IL": 5.0, "I0": 1e-10, "Rs": 0.0, "Rsh": 1e9, "nNsVth": 1.0}, "outputs": {"I": 5.0, "P": 0.0}},
               {"inputs": {"V": 1.0, "IL": 5.0, "I0": 1e-10, "Rs": 0.0, "Rsh": 100.0, "nNsVth": 1.0}, "outputs": {"I": _r(5 - 1e-10 * math.expm1(1) - 0.01)}}],
     questions=[{"q": "Single-diode current: V = 1 V, IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, Rsh = 100 ohm, nNsVth = 1 V", "outputs": {"I": _r(5 - 1e-10 * math.expm1(1) - 0.01)}},
                {"q": "Single-diode current: V = 1 V, IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, Rsh = 0 ohm, nNsVth = 1 V", "outcome": "INVALID_INPUT"}])


def single_diode_voltage(I, IL, I0, Rs, Rsh, nNsVth):
    _sd_check(IL, I0, Rs, Rsh, nNsVth)
    V = _sd_voltage(I, IL, I0, Rs, Rsh, nNsVth)
    return _out(V=V, P=V * I)


def _ref_sd_v(I, IL, I0, Rs, Rsh, nNsVth):
    from pvlib import pvsystem
    V = _np_scalar(pvsystem.v_from_i(I, IL, I0, Rs, Rsh, nNsVth, method="lambertw"))
    return {"V": V, "P": V * I}


_add("energy_single_diode_voltage",
     name="PV single-diode model: voltage at a current", topic="photovoltaics / single-diode model",
     purpose="The terminal voltage (and power) of a PV cell or module carrying a current, from the five-parameter single-diode equation solved implicitly for V; I = 0 gives the open-circuit voltage.",
     inputs=[_phys("I", "current", "terminal current")] + _SD_IN,
     outputs=[_o("V", "voltage", "terminal voltage"), _o("P", "power", "power V I")], primary="V", conditions=["IL, I0, Rsh, a > 0", "Rs >= 0"],
     method="the implicit single-diode equation solved for V by bisection to full precision", latex=r"I = I_L - I_0\left[e^{(V + IR_s)/a} - 1\right] - \frac{V + IR_s}{R_{sh}}",
     run=single_diode_voltage,
     parse=lambda q: _pairs(q, r"single[- ]diode (?:model )?voltage|pv voltage at a current|single[- ]diode voltage at a current", _SD_NAMES, phys={"I", "IL", "I0", "Rs", "Rsh", "nNsVth"}),
     reference=_ref_sd_v,
     wrong=_safe(lambda I, IL, I0, Rs, Rsh, nNsVth: (lambda V: _out(V=V, P=V * I))(nNsVth * math.log1p((IL - I) / I0))),
     random_inputs=lambda rng: (lambda x: dict(x, I=round(_sd_current(0.0, x["IL"], x["I0"], x["Rs"], x["Rsh"], x["nNsVth"]) * rng.uniform(0.0, 0.95), 6)))(_rnd_sd(rng)),
     templates=["Single-diode voltage: I = 8 A, IL = 9 A, I0 = 1e-10 A, Rs = 0.3 ohm, Rsh = 400 ohm, nNsVth = 1.9 V"],
     examples=[{"inputs": {"I": 8.0, "IL": 9.0, "I0": 1e-10, "Rs": 0.3, "Rsh": 400.0, "nNsVth": 1.9}, "outputs": {"V": 41.1302081792, "P": 329.041665434}}],
     questions=[{"q": "Single-diode voltage: I = 0 A, IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, Rsh = 1e15 ohm, nNsVth = 1 V", "outputs": {"V": _r(math.log1p(5e10))}},
                {"q": "Single-diode voltage: I = 0 A, IL = 5 A, I0 = 0 A, Rs = 0 ohm, Rsh = 100 ohm, nNsVth = 1 V", "outcome": "INVALID_INPUT"}])


def _sd_mpp(IL, I0, Rs, Rsh, a):
    Isc = _sd_current(0.0, IL, I0, Rs, Rsh, a)
    Voc = _sd_voc(IL, I0, Rs, Rsh, a)
    # parametrize by the diode voltage vd: I(vd), V(vd) = vd - I Rs; dP/dvd = 0 on (0, Voc)
    Iv = lambda vd: IL - I0 * math.expm1(vd / a) - vd / Rsh      # noqa: E731

    def dP(vd):
        i = Iv(vd)
        di = -I0 * math.exp(vd / a) / a - 1 / Rsh
        dv = 1 - Rs * di
        return i * dv + (vd - i * Rs) * di
    vd = _bisect(dP, 0.0, Voc, 400)
    Imp = Iv(vd)
    Vmp = vd - Imp * Rs
    return Isc, Voc, Imp, Vmp


def single_diode_mpp(IL, I0, Rs, Rsh, nNsVth):
    _sd_check(IL, I0, Rs, Rsh, nNsVth)
    Isc, Voc, Imp, Vmp = _sd_mpp(IL, I0, Rs, Rsh, nNsVth)
    _need(Vmp > 0 and Imp > 0, "the maximum power point is not in the first quadrant for these parameters")
    return _out(Isc=Isc, Voc=Voc, Imp=Imp, Vmp=Vmp, Pmp=Imp * Vmp, FF=Imp * Vmp / (Isc * Voc))


def _ref_sd_mpp(IL, I0, Rs, Rsh, nNsVth):
    from pvlib import pvsystem
    from pvlib import singlediode as sdm
    isc = _np_scalar(pvsystem.i_from_v(0.0, IL, I0, Rs, Rsh, nNsVth, method="lambertw"))
    voc = _np_scalar(pvsystem.v_from_i(0.0, IL, I0, Rs, Rsh, nNsVth, method="lambertw"))
    i, v, p = sdm.bishop88_mpp(IL, I0, Rs, Rsh, nNsVth, method="brentq", method_kwargs={"xtol": 1e-14, "rtol": 8.9e-16, "maxiter": 500})
    i, v = _np_scalar(i), _np_scalar(v)
    return {"Isc": isc, "Voc": voc, "Imp": i, "Vmp": v, "Pmp": i * v, "FF": i * v / (isc * voc)}


_add("energy_single_diode_mpp",
     name="PV single-diode model: short-circuit, open-circuit and maximum power point", topic="photovoltaics / single-diode model",
     purpose="Short-circuit current, open-circuit voltage, the maximum power point (current, voltage, power) and the fill factor of a PV cell or module described by the five-parameter single-diode model.",
     inputs=list(_SD_IN),
     outputs=[_o("Isc", "current", "short-circuit current"), _o("Voc", "voltage", "open-circuit voltage"), _o("Imp", "current", "current at maximum power"),
              _o("Vmp", "voltage", "voltage at maximum power"), _o("Pmp", "power", "maximum power"), _o("FF", "number", "fill factor Pmp/(Isc Voc)")],
     primary="Pmp", conditions=["IL, I0, Rsh, a > 0", "Rs >= 0"],
     method="I(0) and V(I = 0) from the implicit equation; maximum power where dP/dV_d = 0 along the curve parametrized by the diode voltage (bisection)",
     latex=r"\frac{dP}{dV} = 0,\quad FF = \frac{I_{mp}V_{mp}}{I_{sc}V_{oc}}",
     run=single_diode_mpp,
     parse=lambda q: _pairs(q, r"single[- ]diode (?:maximum power point|mpp|key points)|maximum power point of a pv (?:cell|module)|pv mpp", _SD_NAMES, phys={"IL", "I0", "Rs", "Rsh", "nNsVth"}),
     reference=_ref_sd_mpp,
     wrong=_safe(lambda IL, I0, Rs, Rsh, nNsVth: (lambda o: dict(o, Vmp=_r(0.8 * o["Voc"]), Imp=_r(single_diode_current(0.8 * o["Voc"], IL, I0, Rs, Rsh, nNsVth)["I"]),
                                                             Pmp=_r(0.8 * o["Voc"] * single_diode_current(0.8 * o["Voc"], IL, I0, Rs, Rsh, nNsVth)["I"])))(single_diode_mpp(IL, I0, Rs, Rsh, nNsVth))),
     random_inputs=_rnd_sd,
     templates=["Single-diode maximum power point: IL = 9 A, I0 = 1e-10 A, Rs = 0.3 ohm, Rsh = 400 ohm, nNsVth = 1.9 V"],
     examples=[{"inputs": {"IL": 9.0, "I0": 1e-10, "Rs": 0.3, "Rsh": 400.0, "nNsVth": 1.9},
                "outputs": {"Isc": 8.99325505839, "Voc": 47.8983940745, "Imp": 8.46542140962, "Vmp": 39.6025119317, "Pmp": 335.251952381, "FF": 0.778275666973}}],
     questions=[{"q": "PV MPP: IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, Rsh = 1e15 ohm, nNsVth = 1 V", "outputs": {"Isc": 5.0}},
                {"q": "PV MPP: IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, nNsVth = 1 V", "outcome": "NEEDS_INPUT"}])


def _desoto(G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=1.121, dEgdT=-0.0002677):
    _pos("the effective irradiance", G)
    _need(200 <= T_cell <= 400, "the cell temperature must lie between 200 K and 400 K")
    _pos("a_ref, IL_ref, I0_ref, Rsh_ref and EgRef", a_ref, IL_ref, I0_ref, Rsh_ref, EgRef)
    _nonneg("Rs", Rs)
    Tref, Tc = 25 + 273.15, _degC(T_cell) + 273.15
    Eg = EgRef * (1 + dEgdT * (Tc - Tref))
    a = a_ref * (Tc / Tref)
    IL = G / 1000 * (IL_ref + alpha_sc * (Tc - Tref))
    I0 = I0_ref * ((Tc / Tref) ** 3) * math.exp(EgRef / (K_B_EV * Tref) - Eg / (K_B_EV * Tc))
    Rsh = Rsh_ref * (1000 / G)
    return IL, I0, Rs, Rsh, a


def desoto_parameters(G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=1.121, dEgdT=-0.0002677):
    IL, I0, Rs_, Rsh, a = _desoto(G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef, dEgdT)
    return _out(IL=IL, I0=I0, Rs=Rs_, Rsh=Rsh, nNsVth=a)


def _ref_desoto(G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=1.121, dEgdT=-0.0002677):
    from pvlib import pvsystem
    IL, I0, Rs_, Rsh, a = pvsystem.calcparams_desoto(G, _degC(T_cell), alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=EgRef, dEgdT=dEgdT)
    return {"IL": _np_scalar(IL), "I0": _np_scalar(I0), "Rs": _np_scalar(Rs_), "Rsh": _np_scalar(Rsh), "nNsVth": _np_scalar(a)}


_DESOTO_IN = [_phys("G", "irradiance", "effective irradiance"), _phys("T_cell", "temperature", "cell temperature"),
              _numin("alpha_sc", "short-circuit current temperature coefficient in A/K"), _phys("a_ref", "voltage", "modified ideality factor at reference conditions"),
              _phys("IL_ref", "current", "photocurrent at reference conditions"), _phys("I0_ref", "current", "saturation current at reference conditions"),
              _phys("Rsh_ref", "resistance", "shunt resistance at reference conditions"), _phys("Rs", "resistance", "series resistance"),
              _numin("EgRef", "band gap at 25 degC in eV (default 1.121, silicon)", required=False), _numin("dEgdT", "band-gap temperature coefficient in 1/K (default -0.0002677)", required=False)]
_DESOTO_NAMES = {k: k for k in ("G", "T_cell", "alpha_sc", "a_ref", "IL_ref", "I0_ref", "Rsh_ref", "Rs", "EgRef", "dEgdT")}


def _rnd_desoto(rng):
    ncell = rng.choice([36, 60, 72])
    return {"G": round(rng.uniform(100, 1100), 1), "T_cell": round(rng.uniform(263, 348), 2), "alpha_sc": round(rng.uniform(0.002, 0.006), 5),
            "a_ref": round(ncell * rng.uniform(1.0, 1.3) * 0.025693, 5), "IL_ref": round(rng.uniform(5, 11), 4), "I0_ref": float(f"{10 ** rng.uniform(-11, -9):.4g}"),
            "Rsh_ref": round(rng.uniform(100, 800), 2), "Rs": round(rng.uniform(0.1, 0.5), 4)}


_add("energy_desoto_parameters",
     name="PV module single-diode parameters at operating conditions (De Soto)", topic="photovoltaics / single-diode model",
     purpose="The five single-diode parameters (photocurrent, saturation current, series and shunt resistance, modified ideality factor) of a PV module at an irradiance and cell temperature from its reference-condition values, by the De Soto (2006) model.",
     inputs=_DESOTO_IN,
     outputs=[_o("IL", "current", "photocurrent"), _o("I0", "current", "saturation current"), _o("Rs", "resistance", "series resistance"),
              _o("Rsh", "resistance", "shunt resistance"), _o("nNsVth", "voltage", "modified ideality factor")],
     primary="I0", conditions=["reference conditions 1000 W/m^2 and 25 degC"],
     method="a = a_ref T/T_ref; IL = (G/1000)(IL_ref + alpha_sc (T - T_ref)); I0 = I0_ref (T/T_ref)^3 exp(Eg_ref/(k T_ref) - Eg/(k T)), Eg = Eg_ref (1 + dEg/dT (T - T_ref)); Rsh = Rsh_ref 1000/G",
     latex=r"I_0 = I_{0,ref}\left(\frac{T}{T_{ref}}\right)^3\exp\left(\frac{E_{g,ref}}{kT_{ref}} - \frac{E_g}{kT}\right)",
     run=desoto_parameters,
     parse=lambda q: _pairs(q, r"de ?soto parameters|single[- ]diode parameters at operating conditions", _DESOTO_NAMES, phys={"G", "T_cell", "a_ref", "IL_ref", "I0_ref", "Rsh_ref", "Rs"}),
     reference=_ref_desoto,
     wrong=_safe(lambda G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=1.121, dEgdT=-0.0002677: (lambda o: dict(o, I0=_r(I0_ref * (T_cell / 298.15) ** 3), Rsh=Rsh_ref))(
         desoto_parameters(G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef, dEgdT))),
     random_inputs=_rnd_desoto,
     templates=["De Soto parameters: G = 800 W/m^2, T_cell = 45 degC, alpha_sc = 0.004, a_ref = 1.9 V, IL_ref = 9 A, I0_ref = 1e-10 A, Rsh_ref = 400 ohm, Rs = 0.3 ohm"],
     examples=[{"inputs": {"G": 1000, "T_cell": 298.15, "alpha_sc": 0.004, "a_ref": 1.9, "IL_ref": 9.0, "I0_ref": 1e-10, "Rsh_ref": 400.0, "Rs": 0.3},
                "outputs": {"IL": 9.0, "I0": 1e-10, "Rsh": 400.0, "nNsVth": 1.9}}],
     questions=[{"q": "De Soto parameters: G = 1000 W/m^2, T_cell = 25 degC, alpha_sc = 0.004, a_ref = 1.9 V, IL_ref = 9 A, I0_ref = 1e-10 A, Rsh_ref = 400 ohm, Rs = 0.3 ohm",
                 "outputs": {"IL": 9.0}},
                {"q": "De Soto parameters: G = 0 W/m^2, T_cell = 25 degC, alpha_sc = 0.004, a_ref = 1.9 V, IL_ref = 9 A, I0_ref = 1e-10 A, Rsh_ref = 400 ohm, Rs = 0.3 ohm",
                 "outcome": "INVALID_INPUT"}])


def pv_module_at_conditions(G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=1.121, dEgdT=-0.0002677):
    p = _desoto(G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef, dEgdT)
    Isc, Voc, Imp, Vmp = _sd_mpp(*p)
    _need(Vmp > 0 and Imp > 0, "the maximum power point is not in the first quadrant for these parameters")
    return _out(Isc=Isc, Voc=Voc, Imp=Imp, Vmp=Vmp, Pmp=Imp * Vmp, FF=Imp * Vmp / (Isc * Voc))


def _ref_module_conditions(G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=1.121, dEgdT=-0.0002677):
    from pvlib import pvsystem
    p = pvsystem.calcparams_desoto(G, _degC(T_cell), alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=EgRef, dEgdT=dEgdT)
    return _ref_sd_mpp(*[_np_scalar(v) for v in p])


_add("energy_pv_module_at_conditions",
     name="PV module maximum power at an irradiance and temperature", topic="photovoltaics / module performance",
     purpose="Short-circuit current, open-circuit voltage, maximum-power current, voltage and power, and fill factor of a PV module at an irradiance and cell temperature: the De Soto translation of its reference parameters, then the single-diode maximum power point.",
     inputs=_DESOTO_IN,
     outputs=[_o("Isc", "current", "short-circuit current"), _o("Voc", "voltage", "open-circuit voltage"), _o("Imp", "current", "current at maximum power"),
              _o("Vmp", "voltage", "voltage at maximum power"), _o("Pmp", "power", "maximum power"), _o("FF", "number", "fill factor")],
     primary="Pmp", conditions=["reference conditions 1000 W/m^2 and 25 degC"],
     method="De Soto parameter translation, then I_sc, V_oc and the maximum power point of the single-diode equation",
     latex=r"P_{mp} = \max_V V\,I(V;\,I_L(G,T), I_0(T), R_s, R_{sh}(G), a(T))",
     run=pv_module_at_conditions,
     parse=lambda q: _pairs(q, r"pv module (?:maximum power )?at (?:operating )?conditions|module maximum power at irradiance and temperature", _DESOTO_NAMES,
                            phys={"G", "T_cell", "a_ref", "IL_ref", "I0_ref", "Rsh_ref", "Rs"}),
     reference=_ref_module_conditions,
     wrong=_safe(lambda G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef=1.121, dEgdT=-0.0002677: (lambda o: dict(o, Pmp=_r(o["Pmp"] * G / 1000), Imp=_r(o["Imp"] * G / 1000)))(
         pv_module_at_conditions(1000.0, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef, dEgdT))),
     random_inputs=_rnd_desoto,
     templates=["PV module at conditions: G = 800 W/m^2, T_cell = 45 degC, alpha_sc = 0.004, a_ref = 1.9 V, IL_ref = 9 A, I0_ref = 1e-10 A, Rsh_ref = 400 ohm, Rs = 0.3 ohm"],
     examples=[{"inputs": {"G": 800, "T_cell": 318.15, "alpha_sc": 0.004, "a_ref": 1.9, "IL_ref": 9.0, "I0_ref": 1e-10, "Rsh_ref": 400.0, "Rs": 0.3},
                "outputs": {"Isc": 7.25964420895, "Voc": 44.279581395, "Pmp": 246.986606665, "FF": 0.768341976009}}],
     questions=[{"q": "PV module at conditions: G = 1000 W/m^2, T_cell = 25 degC, alpha_sc = 0.004, a_ref = 1 V, IL_ref = 5 A, I0_ref = 1e-10 A, Rsh_ref = 1e15 ohm, Rs = 0 ohm",
                 "outputs": {"Isc": 5.0}},
                {"q": "PV module at conditions: G = 1000 W/m^2, T_cell = 500 K, alpha_sc = 0.004, a_ref = 1 V, IL_ref = 5 A, I0_ref = 1e-10 A, Rsh_ref = 1e15 ohm, Rs = 0 ohm",
                 "outcome": "INVALID_INPUT"}])


# --- Perez transposition (expert) --------------------------------------------------------------------------------
_PEREZ_1990 = [[-0.0080, 0.5880, -0.0620, -0.0600, 0.0720, -0.0220], [0.1300, 0.6830, -0.1510, -0.0190, 0.0660, -0.0290],
               [0.3300, 0.4870, -0.2210, 0.0550, -0.0640, -0.0260], [0.5680, 0.1870, -0.2950, 0.1090, -0.1520, -0.0140],
               [0.8730, -0.3920, -0.3620, 0.2260, -0.4620, 0.0010], [1.1320, -1.2370, -0.4120, 0.2880, -0.8230, 0.0560],
               [1.0600, -1.6000, -0.3590, 0.2640, -1.1270, 0.1310], [0.6780, -0.3270, -0.2500, 0.1560, -1.3770, 0.2510]]
_PEREZ_EDGES = [0., 1.065, 1.23, 1.5, 1.95, 2.8, 4.5, 6.2]


def poa_perez(tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n):
    _check_surface(tilt, surface_azimuth)
    _need(isinstance(zenith, (int, float)) and 0 <= zenith < 87, "the zenith angle must lie between 0 and 87 degrees")
    _need(0 <= azimuth <= 360, "the solar azimuth must lie between 0 and 360 degrees")
    _check_irr(dni, ghi, dhi)
    _need(dhi > 0, "the Perez model needs a positive diffuse irradiance")
    _need(0 <= albedo <= 1, "the albedo must lie between 0 and 1")
    _check_day(n)
    am = _airmass_rel(zenith, "kastenyoung1989")
    z = math.radians(zenith)
    delta = dhi * am / _g_on_spencer(n)
    eps = ((dhi + dni) / dhi + 1.041 * z ** 3) / (1 + 1.041 * z ** 3)
    b = max(i for i, e in enumerate(_PEREZ_EDGES) if eps >= e)
    c = _PEREZ_1990[b]
    F1 = max(c[0] + c[1] * delta + c[2] * z, 0.0)
    F2 = c[3] + c[4] * delta + c[5] * z
    p = _projection(tilt, surface_azimuth, zenith, azimuth)
    A = max(p, 0.0)
    B = max(_cosd(zenith), _cosd(85))
    sky = max(dhi * (0.5 * (1 - F1) * (1 + _cosd(tilt)) + F1 * A / B + F2 * _sind(tilt)), 0.0)
    ground = ghi * albedo * (1 - math.cos(math.radians(tilt))) * 0.5
    aoi_d = math.degrees(math.acos(p))
    direct = max(dni * math.cos(math.radians(aoi_d)), 0.0)
    return _out(poa_global=direct + sky + ground, poa_direct=direct, poa_sky_diffuse=sky, poa_ground_diffuse=ground, aoi_deg=aoi_d)


def _ref_perez(tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n):
    from pvlib import atmosphere, irradiance as irr
    extra = _np_scalar(irr.get_extra_radiation(float(n)))
    am = _np_scalar(atmosphere.get_relative_airmass(float(zenith)))
    r = irr.get_total_irradiance(tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, dni_extra=extra, airmass=am, albedo=albedo, model="perez")
    return {"poa_global": _np_scalar(r["poa_global"]), "poa_direct": _np_scalar(r["poa_direct"]), "poa_sky_diffuse": _np_scalar(r["poa_sky_diffuse"]),
            "poa_ground_diffuse": _np_scalar(r["poa_ground_diffuse"]), "aoi_deg": _np_scalar(irr.aoi(tilt, surface_azimuth, zenith, azimuth))}


_add("energy_poa_perez",
     name="Irradiance on a tilted plane (Perez sky model)", topic="solar resource / tilted surfaces",
     purpose="Beam, sky-diffuse and ground-reflected irradiance on a tilted surface with the Perez (1990) anisotropic sky (circumsolar and horizon brightening coefficients by sky-clearness bin, all-sites composite set), the air mass by Kasten-Young and G_on from day n (Spencer).",
     inputs=list(_POA_IN) + [_DAY],
     outputs=list(_POA_OUT),
     primary="poa_global", conditions=["0 <= zenith < 87", "DHI > 0", "DHI <= GHI"],
     method="eps = ((DHI + DNI)/DHI + 1.041 z^3)/(1 + 1.041 z^3), Delta = DHI AM/G_on; F1, F2 from the bin's coefficients; sky = DHI [(1 - F1)(1 + cos beta)/2 + F1 a/b + F2 sin beta]",
     latex=r"G_{d,t} = G_d\left[(1 - F_1)\frac{1+\cos\beta}{2} + F_1\frac{a}{b} + F_2\sin\beta\right]",
     run=poa_perez,
     parse=lambda q: _pairs(q, r"plane of array irradiance \(perez\)|poa irradiance perez|plane of array irradiance perez", _POA_NAMES, phys={"dni", "ghi", "dhi"},
                            num_units=_deg_units("tilt", "surface_azimuth", "zenith", "azimuth")),
     reference=_ref_perez,
     wrong=_safe(lambda tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n: _poa("haydavies", tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n)),
     random_inputs=lambda rng: _rnd_poa(rng, True),
     templates=["Plane of array irradiance (Perez): " + _Q_POA + ", n = 172"],
     examples=[{"inputs": dict(_POA_EX, n=172), "outputs": {"poa_direct": 800.0}}],
     questions=[{"q": "Plane of array irradiance (Perez): " + _Q_POA + ", n = 172", "outputs": {"poa_direct": 800.0}},
                {"q": "Plane of array irradiance (Perez): " + _Q_POA.replace("dhi = 100", "dhi = 0") + ", n = 172", "outcome": "INVALID_INPUT"}])


def _alt2pres(h):
    return 100 * ((44331.514 - h) / 11880.516) ** (1 / 0.1902632)


def clear_sky_ineichen(zenith, linke_turbidity, altitude, n):
    _need(isinstance(zenith, (int, float)) and 0 <= zenith < 90, "the sun must be above the horizon (apparent zenith below 90 degrees)")
    _need(1 <= linke_turbidity <= 10, "the Linke turbidity must lie between 1 and 10")
    _need(-500 <= altitude <= 9000, "the altitude must lie between -500 m and 9000 m")
    _check_day(n)
    am = _airmass_rel(zenith, "kastenyoung1989") * _alt2pres(altitude) / 101325.
    extra = _g_on_spencer(n)
    cz = max(_cosd(zenith), 0.0)
    tl = linke_turbidity
    fh1, fh2 = math.exp(-altitude / 8000.), math.exp(-altitude / 1250.)
    cg1, cg2 = 5.09e-05 * altitude + 0.868, 3.92e-05 * altitude + 0.0387
    ghi = cg1 * extra * cz * tl / tl * max(math.exp(-cg2 * am * (fh1 + fh2 * (tl - 1))), 0.0)
    bnci = extra * max((0.664 + 0.163 / fh1) * math.exp(-0.09 * am * (tl - 1)), 0.0)
    bnci2 = ghi * min(max((1 - (0.1 - 0.2 * math.exp(-tl)) / (0.1 + 0.882 / fh1)) / cz, 0.0), 1e20)
    dni = min(bnci, bnci2)
    return _out(ghi=ghi, dni=dni, dhi=ghi - dni * cz, AM_absolute=am)


def _ref_ineichen(zenith, linke_turbidity, altitude, n):
    import pandas as pd
    from pvlib import atmosphere, clearsky, irradiance as irr
    am = _np_scalar(atmosphere.get_absolute_airmass(atmosphere.get_relative_airmass(float(zenith)), atmosphere.alt2pres(altitude)))
    r = clearsky.ineichen(pd.Series([float(zenith)]), pd.Series([am]), linke_turbidity, altitude=altitude, dni_extra=_np_scalar(irr.get_extra_radiation(float(n))))
    return {"ghi": float(r["ghi"].iloc[0]), "dni": float(r["dni"].iloc[0]), "dhi": float(r["dhi"].iloc[0]), "AM_absolute": am}


_add("energy_clear_sky_ineichen",
     name="Clear-sky irradiance (Ineichen-Perez with Linke turbidity)", topic="solar resource / clear sky",
     purpose="Clear-sky global horizontal, direct normal and diffuse horizontal irradiance from the apparent solar zenith, the Linke turbidity and the site altitude (Ineichen and Perez 2002), with the pressure-corrected air mass and G_on from day n.",
     inputs=[_numin("zenith", "apparent solar zenith angle in degrees"), _numin("linke_turbidity", "Linke turbidity factor (1 to 10; about 2 to 5 for clear skies)"),
             _phys("altitude", "length", "site altitude above sea level"), _DAY],
     outputs=[_o("ghi", "irradiance", "clear-sky global horizontal irradiance"), _o("dni", "irradiance", "clear-sky direct normal irradiance"),
              _o("dhi", "irradiance", "clear-sky diffuse horizontal irradiance"), _o("AM_absolute", "number", "absolute air mass (Kasten-Young, standard pressure at the altitude)")],
     primary="ghi", conditions=["sun above the horizon", "1 <= TL <= 10"],
     method="GHI = cg1 G_on cos z exp(-cg2 AM (fh1 + fh2 (TL - 1))); DNI = min(b G_on exp(-0.09 AM (TL - 1)), GHI (1 - (0.1 - 0.2 e^-TL)/(0.1 + 0.882/fh1))/cos z); fh1 = e^(-h/8000), fh2 = e^(-h/1250)",
     latex=r"G_{cs} = c_{g1}G_{on}\cos z\,e^{-c_{g2}AM\left(f_{h1} + f_{h2}(T_L - 1)\right)}",
     run=clear_sky_ineichen,
     parse=lambda q: _pairs(q, r"clear[- ]sky (?:irradiance )?\(ineichen\)|ineichen clear[- ]sky|clear[- ]sky ineichen", dict(_N_NAMES, zenith="zenith", z="zenith", linke_turbidity="linke_turbidity",
                                                                                                                   TL="linke_turbidity", altitude="altitude", elevation="altitude"),
                            phys={"altitude"}, num_units=_deg_units("zenith")),
     reference=_ref_ineichen,
     wrong=_safe(lambda zenith, linke_turbidity, altitude, n: (lambda o: dict(o, ghi=_r(o["ghi"] * math.exp(0.0387 * o["AM_absolute"] * (linke_turbidity - 1) * 0.5))))(
         clear_sky_ineichen(zenith, linke_turbidity, altitude, n))),
     random_inputs=lambda rng: {"zenith": round(rng.uniform(0, 85), 3), "linke_turbidity": round(rng.uniform(1.5, 6), 3), "altitude": round(rng.uniform(0, 3000), 1), "n": rng.randint(1, 365)},
     templates=["Clear-sky irradiance (Ineichen): zenith = 30, linke_turbidity = 3, altitude = 500 m, n = 172"],
     examples=[{"inputs": {"zenith": 0, "linke_turbidity": 1, "altitude": 0, "n": 1},
                "outputs": {"dni": _r(min(_g_on_spencer(1) * 0.827 * math.exp(0), 0.868 * _g_on_spencer(1) * math.exp(-0.0387 * _airmass_rel(0, "kastenyoung1989") * _alt2pres(0) / 101325)
                                      * (1 - (0.1 - 0.2 * math.exp(-1)) / 0.982)))}}],
     questions=[{"q": "Clear-sky irradiance (Ineichen): zenith = 30, linke_turbidity = 3, altitude = 500 m, n = 172", "outputs": {"ghi": clear_sky_ineichen(30, 3, 500, 172)["ghi"]}},
                {"q": "Clear-sky irradiance (Ineichen): zenith = 95, linke_turbidity = 3, altitude = 500 m, n = 172", "outcome": "INVALID_INPUT"},
                {"q": "Clear-sky irradiance (Ineichen): zenith = 30, linke_turbidity = 3, n = 172", "outcome": "NEEDS_INPUT"}])


def disc(ghi, zenith, n, pressure=101325.0):
    _check_day(n)
    _need(isinstance(zenith, (int, float)) and 0 <= zenith <= 85, "the zenith angle must lie between 0 and 85 degrees")
    _pos("the global horizontal irradiance and the pressure", ghi, pressure)
    b = _day_angle(n)
    I0 = 1370. * (1.00011 + 0.034221 * math.cos(b) + 0.00128 * math.sin(b) + 0.000719 * math.cos(2 * b) + 7.7e-05 * math.sin(2 * b))
    kt = min(max(ghi / (I0 * max(_cosd(zenith), 0.065)), 0.0), 1.0)
    am = min(_airmass_rel(zenith, "kasten1966") * pressure / 101325., 12.0)
    if kt <= 0.6:
        a = 0.512 + kt * (-1.56 + kt * (2.286 - 2.222 * kt))
        bb = 0.37 + 0.962 * kt
        c = -0.28 + kt * (0.932 - 2.048 * kt)
    else:
        a = -5.743 + kt * (21.77 + kt * (-27.49 + 11.56 * kt))
        bb = 41.4 + kt * (-118.5 + kt * (66.05 + 31.9 * kt))
        c = -47.01 + kt * (184.2 + kt * (-222.0 + 73.81 * kt))
    Kn = 0.866 + am * (-0.122 + am * (0.0121 + am * (-0.000653 + 1.4e-05 * am))) - (a + bb * math.exp(c * am))
    dni = max(Kn * I0, 0.0)
    return _out(dni=dni, kt=kt, airmass=am)


def _ref_disc(ghi, zenith, n, pressure=101325.0):
    from pvlib import irradiance as irr
    r = irr.disc(ghi, zenith, float(n), pressure=pressure)
    return {"dni": _np_scalar(r["dni"]), "kt": _np_scalar(r["kt"]), "airmass": _np_scalar(r["airmass"])}


def _rnd_disc(rng):
    n, z = rng.randint(1, 365), round(rng.uniform(0, 80), 2)
    for _ in range(100):
        kt = rng.uniform(0.1, 0.85)
        x = {"ghi": round(kt * 1370 * _cosd(z), 3) or 1.0, "zenith": z, "n": n, "pressure": round(rng.uniform(7e4, 1.03e5), 1)}
        if disc(**x)["dni"] > 1.0:
            return x
    return x


_add("energy_disc_dni",
     name="Direct normal irradiance from global horizontal irradiance (DISC model)", topic="solar resource / irradiance models",
     purpose="The direct normal irradiance estimated from global horizontal irradiance by Maxwell's DISC model (clearness index and pressure-corrected air mass), with the clearness index and the air mass used.",
     inputs=[_phys("ghi", "irradiance", "global horizontal irradiance"), _numin("zenith", "solar zenith angle in degrees"), _DAY,
             _phys("pressure", "pressure", "site pressure (default 101325 Pa)", required=False)],
     outputs=[_o("dni", "irradiance", "direct normal irradiance (0 when the model gives a negative value)"), _o("kt", "number", "clearness index (solar constant 1370 W/m^2)"),
              _o("airmass", "number", "absolute air mass (Kasten 1966, capped at 12)")],
     primary="dni", conditions=["0 <= zenith <= 85", "GHI > 0"],
     method="kt = GHI/(I0 cos z); Kn = Knc(AM) - (a(kt) + b(kt) exp(c(kt) AM)); DNI = Kn I0", latex=r"G_{bn} = I_0\left[K_{nc}(AM) - \left(a + b\,e^{c\,AM}\right)\right]",
     run=disc,
     parse=lambda q: _pairs(q, r"disc (?:model|dni)|dni from ghi \(disc\)|disc", dict(_N_NAMES, ghi="ghi", GHI="ghi", zenith="zenith", z="zenith", pressure="pressure", p="pressure"),
                            phys={"ghi", "pressure"}, num_units=_deg_units("zenith")),
     reference=_ref_disc,
     wrong=_safe(lambda ghi, zenith, n, pressure=101325.0: disc(ghi, zenith, n, 101325.0 * 101325.0 / pressure) if abs(pressure - 101325) > 1000 else (lambda o: dict(o, dni=_r(o["dni"] * 1.1)))(disc(ghi, zenith, n, pressure))),
     random_inputs=_rnd_disc,
     templates=["DISC model: ghi = 600 W/m^2, zenith = 35, n = 172"],
     examples=[{"inputs": {"ghi": 0.5 * 1370 * (1.00011 + 0.034221 + 0.000719), "zenith": 0, "n": 1}, "outputs": {"kt": 0.5}}],
     questions=[{"q": "DISC model: ghi = " + repr(0.5 * 1370 * (1.00011 + 0.034221 + 0.000719)) + " W/m^2, zenith = 0, n = 1", "outputs": {"kt": 0.5}},
                {"q": "DISC model: ghi = 600 W/m^2, zenith = 89, n = 172", "outcome": "INVALID_INPUT"}])


# ================================================================================================ definitional additions
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). Base and per-unit
# impedance, geometric mean distance, synchronous and rotor speed, stack electrical power, coulomb counting.

def per_unit_impedance(R, X, S_base, V_base):
    _nonneg("the resistance", R)
    _nonneg("the reactance", X)
    _need(R > 0 or X > 0, "the impedance must not be zero")
    _pos("the base power and base voltage", S_base, V_base)
    Zb = V_base * V_base / S_base
    return _out(Z_base=Zb, I_base=S_base / (math.sqrt(3) * V_base), R_pu=R / Zb, X_pu=X / Zb, Z_pu=math.hypot(R, X) / Zb)


def _ref_per_unit_impedance(R, X, S_base, V_base):
    """The base impedance as the impedance that carries the base current at the base phase voltage (V/sqrt(3))/I in
    exact arithmetic (sympy); per-unit values as ratios, the magnitude by numpy's hypot."""
    import numpy as np
    import sympy as sp
    S, V = sp.Rational(repr(S_base)), sp.Rational(repr(V_base))
    Ib = S / (sp.sqrt(3) * V)
    Zb = (V / sp.sqrt(3)) / Ib
    zb = float(Zb)
    return {"Z_base": zb, "I_base": float(Ib), "R_pu": R / zb, "X_pu": X / zb, "Z_pu": float(np.hypot(R, X)) / zb}


_add("energy_per_unit_impedance",
     name="Base impedance and per-unit impedance", topic="power systems / per-unit system",
     purpose="The base impedance Z_base = V_base^2/S_base and base current of a three-phase per-unit system, and an impedance in ohms expressed in per unit.",
     inputs=[_phys("R", "resistance", "resistance per phase (ohm)"), _phys("X", "resistance", "reactance per phase (ohm)"),
             _phys("S_base", "apparent_power", "three-phase base power (VA)"), _phys("V_base", "voltage", "line-to-line base voltage")],
     outputs=[_o("Z_base", "resistance", "base impedance V_base^2/S_base"), _o("I_base", "current", "base current S_base/(sqrt(3) V_base)"),
              _o("R_pu", "number", "per-unit resistance"), _o("X_pu", "number", "per-unit reactance"), _o("Z_pu", "number", "per-unit impedance magnitude")],
     primary="X_pu", conditions=["R, X >= 0, not both 0", "positive bases", "three-phase base, line-to-line voltage"],
     method="Z_base = V_base^2/S_base, I_base = S_base/(sqrt(3) V_base), Z_pu = Z/Z_base", latex=r"Z_{base} = \frac{V_{base}^2}{S_{base}},\quad Z_{pu} = \frac{Z}{Z_{base}}",
     run=per_unit_impedance,
     parse=lambda q: _pairs(q, r"per[- ]unit impedance|base impedance", {"R": "R", "X": "X", "S_base": "S_base", "S": "S_base", "V_base": "V_base", "V": "V_base"},
                            phys={"R", "X", "S_base", "V_base"}),
     reference=_ref_per_unit_impedance,
     wrong=_safe(lambda R, X, S_base, V_base: (lambda Zb: _out(Z_base=Zb, I_base=S_base / (math.sqrt(3) * V_base), R_pu=R / Zb, X_pu=X / Zb, Z_pu=math.hypot(R, X) / Zb))(
         V_base * V_base / (3 * S_base))),
     random_inputs=lambda rng: {"R": rng.choice([0.0, _lu(rng, 0.01, 50)]), "X": _lu(rng, 0.05, 200), "S_base": _lu(rng, 1e5, 1e9), "V_base": _lu(rng, 400, 7.65e5)},
     templates=["Per-unit impedance: R = 5 ohm, X = 20 ohm, S_base = 100 MVA, V_base = 132 kV"],
     examples=[{"inputs": {"R": 5.0, "X": 20.0, "S_base": 1e8, "V_base": 1.32e5}, "outputs": {"Z_base": 174.24, "X_pu": 20 / 174.24, "R_pu": 5 / 174.24}},
               {"inputs": {"R": 0.0, "X": 10.0, "S_base": 1e6, "V_base": 1e4}, "outputs": {"Z_base": 100.0, "Z_pu": 0.1, "I_base": 1e6 / (math.sqrt(3) * 1e4)}}],
     questions=[{"q": "Per-unit impedance: R = 5 ohm, X = 20 ohm, S_base = 100 MVA, V_base = 132 kV", "outputs": {"X_pu": 20 / 174.24}},
                {"q": "Per-unit impedance: R = 5 ohm, X = 20 ohm, S_base = 0 MVA, V_base = 132 kV", "outcome": "INVALID_INPUT"},
                {"q": "Per-unit impedance: R = 5 ohm, X = 20 ohm, S_base = 100 MVA", "outcome": "NEEDS_INPUT"}])


def gmd(D_ab, D_bc, D_ca):
    _pos("the phase spacings", D_ab, D_bc, D_ca)
    _need(D_ab + D_bc >= D_ca and D_bc + D_ca >= D_ab and D_ca + D_ab >= D_bc, "the three spacings must form a triangle (or a line)")
    return _out(GMD=(D_ab * D_bc * D_ca) ** (1.0 / 3.0), ratio_to_mean=(D_ab * D_bc * D_ca) ** (1.0 / 3.0) / ((D_ab + D_bc + D_ca) / 3))


def _ref_gmd(D_ab, D_bc, D_ca):
    """scipy.stats.gmean of the three spacings (the mean of the logarithms, exponentiated)."""
    from scipy.stats import gmean
    g = float(gmean([D_ab, D_bc, D_ca]))
    return {"GMD": g, "ratio_to_mean": g / ((D_ab + D_bc + D_ca) / 3)}


_add("energy_geometric_mean_distance",
     name="Geometric mean distance of a three-phase line", topic="power systems / line parameters",
     purpose="The geometric mean distance GMD = (D_ab D_bc D_ca)^(1/3) between the phases of a fully transposed three-phase line.",
     inputs=[_phys("D_ab", "length", "distance between phases a and b"), _phys("D_bc", "length", "distance between phases b and c"),
             _phys("D_ca", "length", "distance between phases c and a")],
     outputs=[_o("GMD", "length", "geometric mean distance"), _o("ratio_to_mean", "number", "GMD divided by the arithmetic mean spacing")],
     primary="GMD", conditions=["positive spacings that form a triangle (or a straight line)", "fully transposed line"],
     method="GMD = (D_ab D_bc D_ca)^(1/3)", latex=r"GMD = \sqrt[3]{D_{ab}D_{bc}D_{ca}}",
     run=gmd,
     parse=lambda q: _pairs(q, r"geometric mean distance|gmd", {"D_ab": "D_ab", "D_bc": "D_bc", "D_ca": "D_ca"}, phys={"D_ab", "D_bc", "D_ca"}),
     reference=_ref_gmd,
     wrong=_safe(lambda D_ab, D_bc, D_ca: _out(GMD=(D_ab + D_bc + D_ca) / 3, ratio_to_mean=1.0)),
     random_inputs=lambda rng: (lambda a, b: {"D_ab": a, "D_bc": b, "D_ca": rng.uniform(abs(a - b) * 1.01 + 0.01, (a + b) * 0.99)})(rng.uniform(0.5, 15), rng.uniform(0.5, 15)),
     templates=["Geometric mean distance: D_ab = 4 m, D_bc = 4 m, D_ca = 8 m"],
     examples=[{"inputs": {"D_ab": 4.0, "D_bc": 4.0, "D_ca": 8.0}, "outputs": {"GMD": 128 ** (1 / 3)}},
               {"inputs": {"D_ab": 2.0, "D_bc": 2.0, "D_ca": 2.0}, "outputs": {"GMD": 2.0, "ratio_to_mean": 1.0}}],
     questions=[{"q": "Geometric mean distance: D_ab = 4 m, D_bc = 4 m, D_ca = 8 m", "outputs": {"GMD": 128 ** (1 / 3)}},
                {"q": "Geometric mean distance: D_ab = 1 m, D_bc = 1 m, D_ca = 8 m", "outcome": "INVALID_INPUT"},
                {"q": "Geometric mean distance: D_ab = 4 m, D_bc = 4 m", "outcome": "NEEDS_INPUT"}])


def machine_speed(f, poles, s=None, n_r=None):
    _pos("the frequency", f)
    _need(isinstance(poles, int) and poles >= 2 and poles % 2 == 0 and poles <= 200, "the number of poles must be an even whole number from 2 to 200")
    _need(s is None or n_r is None, "give the slip or the rotor speed, not both")
    ns = 120.0 * f / poles
    ws = 4 * math.pi * f / poles
    out = dict(n_sync_rpm=ns, omega_sync=ws)
    if s is not None:
        _fin("the slip", s)
        _need(-1 < s < 2, "the slip must lie between -1 and 2")
        out.update(n_rotor_rpm=ns * (1 - s), slip=s, slip_frequency=s * f)
    if n_r is not None:
        _fin("the rotor speed", n_r)
        nr = n_r * 60 / (2 * math.pi)
        sl = (ns - nr) / ns
        out.update(n_rotor_rpm=nr, slip=sl, slip_frequency=sl * f)
    return _out(**out)


def _ref_machine_speed(f, poles, s=None, n_r=None):
    """The speed of the rotating field as the electrical angular frequency divided by the pole pairs, in exact rational
    arithmetic (sympy, rad/s and rpm); slip and rotor speed from the relative speed."""
    import sympy as sp
    F = sp.Rational(repr(f))
    ws = 2 * sp.pi * F / sp.Rational(poles, 2)
    ns = ws * 60 / (2 * sp.pi)
    out = {"n_sync_rpm": float(ns), "omega_sync": float(ws)}
    if s is not None:
        out.update(n_rotor_rpm=float(ns * (1 - sp.Rational(repr(s)))), slip=s, slip_frequency=float(sp.Rational(repr(s)) * F))
    if n_r is not None:
        nr = sp.Rational(repr(n_r)) * 60 / (2 * sp.pi)
        sl = (ns - nr) / ns
        out.update(n_rotor_rpm=float(nr), slip=float(sl), slip_frequency=float(sl * F))
    return out


def _rnd_machine_speed(rng):
    x = {"f": rng.choice([50.0, 60.0, rng.uniform(1, 400)]), "poles": rng.choice([2, 4, 6, 8, 10, 12, 16, 24, 48])}
    u = rng.random()
    if u < 0.4:
        x["s"] = rng.uniform(-0.2, 0.2)
    elif u < 0.8:
        x["n_r"] = 4 * math.pi * x["f"] / x["poles"] * rng.uniform(0.8, 1.1)
    return x


_add("energy_synchronous_speed",
     name="Synchronous speed, rotor speed and slip of an AC machine", topic="machines / speed and slip",
     purpose="The synchronous speed n_s = 120 f/P of an AC machine and, with the slip, the rotor speed n = n_s (1 - s) (or, with the rotor speed, the slip) and the rotor (slip) frequency.",
     inputs=[_phys("f", "frequency", "supply frequency"), _intin("poles", "number of poles (even)", 2, 200),
             _numin("s", "slip (fraction or %, optional)", required=False), _phys("n_r", "angular_frequency", "rotor speed (rpm, optional)", required=False)],
     outputs=[_o("n_sync_rpm", "number", "synchronous speed in rpm"), _o("omega_sync", "angular_frequency", "synchronous speed in rad/s"),
              _o("n_rotor_rpm", "number", "rotor speed in rpm"), _o("slip", "number", "slip (n_s - n)/n_s"), _o("slip_frequency", "frequency", "rotor current frequency s f")],
     primary="n_sync_rpm", conditions=["f > 0", "even number of poles", "at most one of slip and rotor speed"],
     method="n_s = 120 f/P; n = n_s (1 - s); s = (n_s - n)/n_s; f_r = s f", latex=r"n_s = \frac{120 f}{P},\quad n = n_s(1 - s)",
     run=machine_speed,
     parse=lambda q: _pairs(q, r"synchronous speed(?: and slip)?|rotor speed and slip", {"f": "f", "poles": "poles", "P": "poles", "s": "s", "slip": "s", "n_r": "n_r", "n": "n_r"},
                            phys={"f", "n_r"}, num_units=_pct_units("s")),
     reference=_ref_machine_speed,
     wrong=_safe(lambda f, poles, s=None, n_r=None: (lambda o: dict(o, n_sync_rpm=_r(o["n_sync_rpm"] * 2), omega_sync=_r(o["omega_sync"] * 2),
                                                                   **({"n_rotor_rpm": _r(o["n_rotor_rpm"] * 2)} if s is not None else {})))(machine_speed(f, poles, s, n_r))),
     random_inputs=_rnd_machine_speed,
     templates=["Synchronous speed: f = 50 Hz, poles = 4", "Synchronous speed: f = 60 Hz, poles = 4, s = 3%", "Synchronous speed: f = 60 Hz, poles = 6, n_r = 1150 rpm"],
     examples=[{"inputs": {"f": 50.0, "poles": 4}, "outputs": {"n_sync_rpm": 1500.0, "omega_sync": 50 * math.pi}},
               {"inputs": {"f": 60.0, "poles": 4, "s": 0.03}, "outputs": {"n_rotor_rpm": 1746.0, "slip_frequency": 1.8}},
               {"inputs": {"f": 60.0, "poles": 6, "n_r": 1150 * 2 * math.pi / 60}, "outputs": {"slip": 50 / 1200}}],
     questions=[{"q": "Synchronous speed: f = 50 Hz, poles = 4", "outputs": {"n_sync_rpm": 1500.0}},
                {"q": "Synchronous speed: f = 60 Hz, poles = 4, s = 3%", "outputs": {"n_rotor_rpm": 1746.0}},
                {"q": "Synchronous speed: f = 60 Hz, poles = 6, n_r = 1150 rpm", "outputs": {"slip": 50 / 1200}},
                {"q": "Synchronous speed: f = 50 Hz, poles = 5", "outcome": "INVALID_INPUT"},
                {"q": "Synchronous speed: f = 50 Hz", "outcome": "NEEDS_INPUT"}])


def stack_power(I, N, V_cell):
    _pos("the current and the cell voltage", I, V_cell)
    _need(isinstance(N, int) and 1 <= N <= 100000, "the number of cells must be a whole number from 1 to 100,000")
    _need(V_cell <= 5, "the cell voltage must be at most 5 V")
    return _out(P=N * I * V_cell, P_kW=N * I * V_cell / 1e3, V_stack=N * V_cell)


def _ref_stack_power(I, N, V_cell):
    """The power as the sum of N cell powers (math.fsum of N equal terms, the cells in series carrying the same
    current) and the stack voltage as the sum of N cell voltages."""
    P = math.fsum([I * V_cell] * N)
    return {"P": P, "P_kW": P / 1e3, "V_stack": math.fsum([V_cell] * N)}


_add("energy_stack_electrical_power",
     name="Electrical power of a fuel-cell or electrolyser stack", topic="fuel cells / stack power",
     purpose="The electrical power P = N I V_cell delivered by a fuel-cell stack (or drawn by an electrolyser stack) of N cells in series at a current and an average cell voltage, and the stack voltage.",
     inputs=[_phys("I", "current", "stack current"), _intin("N", "number of cells in series", 1, 100000), _phys("V_cell", "voltage", "average cell voltage")],
     outputs=[_o("P", "power", "electrical power N I V_cell"), _o("P_kW", "number", "electrical power in kW"), _o("V_stack", "voltage", "stack voltage N V_cell")],
     primary="P", conditions=["cells in series carrying the same current", "0 < V_cell <= 5 V"],
     method="P = N I V_cell, V_stack = N V_cell", latex=r"P = N\,I\,V_{cell}",
     run=stack_power,
     parse=lambda q: _pairs(q, r"stack (?:electrical )?power|fuel[- ]cell stack power|electroly[sz]er stack power",
                            {"I": "I", "current": "I", "N": "N", "cells": "N", "V_cell": "V_cell"}, phys={"I", "V_cell"}),
     reference=_ref_stack_power,
     wrong=_safe(lambda I, N, V_cell: _out(P=I * V_cell, P_kW=I * V_cell / 1e3, V_stack=N * V_cell)),
     random_inputs=lambda rng: {"I": _lu(rng, 0.1, 5000), "N": rng.randint(2, 2000), "V_cell": rng.uniform(0.4, 2.5)},
     templates=["Stack electrical power: I = 200 A, N = 100, V_cell = 0.7 V"],
     examples=[{"inputs": {"I": 200.0, "N": 100, "V_cell": 0.7}, "outputs": {"P": 14000.0, "V_stack": 70.0}},
               {"inputs": {"I": 1000.0, "N": 100, "V_cell": 1.9}, "outputs": {"P_kW": 190.0}}],
     questions=[{"q": "Stack electrical power: I = 200 A, N = 100, V_cell = 0.7 V", "outputs": {"P": 14000.0}},
                {"q": "Stack electrical power: I = 200 A, N = 0, V_cell = 0.7 V", "outcome": "INVALID_INPUT"},
                {"q": "Stack electrical power: I = 200 A, N = 100", "outcome": "NEEDS_INPUT"}])


def coulomb_counting(SOC0, Q, currents, dt):
    _need(isinstance(SOC0, (int, float)) and 0 <= SOC0 <= 1, "the initial state of charge must lie between 0 and 1 (or 0 and 100%)")
    _pos("the capacity and the time step", Q, dt)
    _need(len(currents) >= 1, "give at least one current")
    soc, lo, hi, net = SOC0, SOC0, SOC0, 0.0
    for i in currents:
        _fin("each current", i)
        net += i * dt
        soc = SOC0 - net / Q
        lo, hi = min(lo, soc), max(hi, soc)
    _need(lo >= -1e-12 and hi <= 1 + 1e-12, "the state of charge leaves the range 0 to 100% (more charge than the capacity)")
    return _out(SOC=soc, SOC_min=lo, SOC_max=hi, charge_out=net, charge_out_Ah=net / 3600)


def _ref_coulomb_counting(SOC0, Q, currents, dt):
    """The current integrated over the piecewise-constant profile (numpy cumulative sum of I dt), the state of charge
    as SOC0 minus the delivered charge over the capacity."""
    import numpy as np
    q = np.cumsum(np.asarray(currents, float) * dt)
    soc = SOC0 - q / Q
    return {"SOC": float(soc[-1]), "SOC_min": float(min(SOC0, soc.min())), "SOC_max": float(max(SOC0, soc.max())), "charge_out": float(q[-1]),
            "charge_out_Ah": float(q[-1]) / 3600}


def _rnd_coulomb(rng):
    Q = rng.uniform(1, 200) * 3600
    n = rng.randint(1, 12)
    dt = rng.uniform(10, 900)
    budget = rng.uniform(0.05, 0.4) * Q
    cur = [round(rng.uniform(-0.3, 1.0) * budget / (n * dt), 4) for _ in range(n)]
    return {"SOC0": rng.uniform(0.5, 0.95), "Q": Q, "currents": cur, "dt": dt}


_add("energy_coulomb_counting",
     name="State of charge by coulomb counting", topic="batteries / state of charge",
     purpose="The state of charge of a battery after a sequence of constant currents over equal time steps (positive on discharge), by coulomb counting SOC = SOC0 - integral(I dt)/Q, with its lowest and highest value and the charge delivered.",
     inputs=[_numin("SOC0", "initial state of charge (fraction or %)"), _phys("Q", "charge", "capacity (Ah, mAh, C)"),
             _vecin("currents", "current in each time step in A (positive on discharge)", 1, 50), _phys("dt", "time", "length of each time step")],
     outputs=[_o("SOC", "number", "final state of charge"), _o("SOC_min", "number", "lowest state of charge"), _o("SOC_max", "number", "highest state of charge"),
              _o("charge_out", "charge", "net charge delivered"), _o("charge_out_Ah", "number", "net charge delivered in Ah")],
     primary="SOC", conditions=["0 <= SOC0 <= 1", "Q, dt > 0", "the state of charge stays within 0 to 100%", "no coulombic losses"],
     method="SOC_k = SOC0 - (sum_{j<=k} I_j dt)/Q", latex=r"SOC(t) = SOC_0 - \frac{1}{Q}\int_0^t I\,dt",
     run=coulomb_counting,
     parse=lambda q: _pairs(q, r"coulomb counting|state of charge by coulomb counting", {"SOC0": "SOC0", "soc0": "SOC0", "Q": "Q", "capacity": "Q", "currents": "currents", "dt": "dt"},
                            phys={"Q", "dt"}, vectors={"currents"}, num_units=_pct_units("SOC0")),
     reference=_ref_coulomb_counting,
     wrong=_safe(lambda SOC0, Q, currents, dt: (lambda o: dict(o, SOC=_r(SOC0 + o["charge_out"] / Q)))(coulomb_counting(SOC0, Q, currents, dt))),
     random_inputs=_rnd_coulomb,
     templates=["Coulomb counting: SOC0 = 90%, Q = 50 Ah, currents = [10, 20, -5], dt = 1 h"],
     examples=[{"inputs": {"SOC0": 0.9, "Q": 50 * 3600.0, "currents": [10, 20, -5], "dt": 3600.0}, "outputs": {"SOC": 0.4, "SOC_min": 0.3, "charge_out_Ah": 25.0}},
               {"inputs": {"SOC0": 1.0, "Q": 2 * 3600.0, "currents": "1, 1", "dt": 1800.0}, "outputs": {"SOC": 0.5}}],
     questions=[{"q": "Coulomb counting: SOC0 = 90%, Q = 50 Ah, currents = [10, 20, -5], dt = 1 h", "outputs": {"SOC": 0.4}},
                {"q": "Coulomb counting: SOC0 = 90%, Q = 50 Ah, currents = [100, 20], dt = 1 h", "outcome": "INVALID_INPUT"},
                {"q": "Coulomb counting: SOC0 = 90%, Q = 50 Ah, currents = [10, 20, -5]", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ power kinds
# Reactive power is reported in var and apparent power in VA, never in W: their own kinds (same dimension as power).
_REACTIVE = [('energy_parallel_transformers', 'Q'), ('energy_parallel_transformers', 'Q1'), ('energy_parallel_transformers', 'Q2'), ('energy_power_angle', 'Q_received'), ('energy_power_angle', 'Q_sent'), ('energy_synchronous_generator', 'Q'), ('energy_three_phase_power', 'Q'), ('energy_transformer_loading', 'Q'), ('energy_transformer_loading', 'Q_loss'), ('energy_two_bus_load_flow', 'Q'), ('energy_two_bus_load_flow', 'Q_loss')]
_APPARENT = [('energy_parallel_transformers', 'S1'), ('energy_parallel_transformers', 'S1_share'), ('energy_parallel_transformers', 'S2'), ('energy_parallel_transformers', 'S2_share'), ('energy_per_unit_change_of_base', 'S_new'), ('energy_per_unit_change_of_base', 'S_old'), ('energy_symmetrical_fault', 'S_base'), ('energy_three_phase_power', 'S'), ('energy_transformer_loading', 'sn'), ('energy_transformer_max_efficiency', 'S'), ('energy_transformer_max_efficiency', 'S_at_max'), ('energy_transformer_performance', 'S')]
for _k, _sym in _REACTIVE + _APPARENT:
    _kind = "reactive_power" if (_k, _sym) in _REACTIVE else "apparent_power"
    _hits = [i for i in PROCEDURES[_k]["inputs"] + PROCEDURES[_k]["outputs"] if i["symbol"] == _sym]
    assert len(_hits) == 1 and _hits[0]["kind"] == "power", (_k, _sym)
    _hits[0]["kind"] = _kind
