"""energy-systems-ai999x — offline energy systems answers with their formula, steps and verification.

    import energy_systems_ai999x as m
    m.ask("Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star").show()   # P = 27712.8 W
    m.ask("Hydrogen tank: V = 125 L, p = 700 bar, T = 15 degC").show()                     # 5.0215 kg, Z = 1.4662
    m.solve("energy_economic_dispatch", demand=800,
            costs="500 5.3 0.004 200 450; 400 5.5 0.006 150 350; 200 5.8 0.009 100 225")  # lambda = 8.5 $/MWh
    m.capabilities()                   # every certified capability

74 certified capabilities: solar resource (declination, equation of time, solar time and hour angle, sun position,
sunrise and sunset, extraterrestrial irradiance, air mass, angle of incidence, isotropic, Klucher, Hay-Davies, Reindl
and Perez irradiance on tilted planes, Erbs and DISC decomposition, Haurwitz and Ineichen clear sky), photovoltaics
(NOCT, Faiman and Sandia cell temperature, PVWatts DC output, array output with temperature derating, inverter
efficiency, system losses, incidence-angle modifiers, the single-diode model: current, voltage, short-circuit,
open-circuit and maximum power point, fill factor, De Soto parameters at operating conditions), wind power (power in
the wind and the Betz limit, actuator-disc theory, power-law and logarithmic wind shear, hub-height air density,
power curves with density correction, Weibull statistics and annual energy), power systems (three-phase power, per-unit
change of base, short, medium and long lines, line inductance and capacitance, two-bus, DC and Newton-Raphson load
flow, economic dispatch, symmetrical and unsymmetrical faults, power-angle transfer, critical clearing by equal areas,
droop frequency control), transformers and machines (equivalent-circuit regulation and efficiency, open- and
short-circuit tests, maximum efficiency, loading from nameplate data, parallel sharing, induction motor performance and
breakdown torque, synchronous generator EMF), batteries (Peukert's law, pack sizing, one-RC
discharge) and fuel cells and hydrogen (stack consumption and efficiency, electrolyser energy per kg, real-gas storage
and compression by the Leachman equation of state).
Questions use the form "<capability phrase>: name = value, ..."; angles are plain numbers in degrees, lists and
matrices are written in brackets, and ask() also reads hPa, mbar, psi, Ah and mAh. Voltages of three-phase systems are
line-to-line, powers three-phase totals.
Built by the AI999X model machine from the same engine as every domain; a question that is not determined
is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.3"
_P = Product("energy_systems", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
