# energy-systems-ai999x — offline, verified energy systems solver with step-by-step answers

[![tests](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/energy-systems-ai999x)](https://pypi.org/project/energy-systems-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/energy-systems-ai999x)

**[Install](https://pypi.org/project/energy-systems-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/energy-systems-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/energy-systems-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

energy-systems-ai999x is an offline, deterministic Python solver that answers energy systems questions with verified results from 79 certified capabilities, gives 311 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**energy-systems-ai999x** is a compact, deterministic energy systems engine for Python. It reads a question in plain English (or a typed call), picks a certified energy systems capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified energy systems capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 311 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** solar resource and photovoltaics, wind power, three-phase power, transmission lines, load flow and economic dispatch, faults and stability, transformers and machines, batteries, fuel cells and hydrogen.

## Model overview

| Property | Value |
|---|---|
| Package | `energy-systems-ai999x` 1.0.0 (import `energy_systems_ai999x`) |
| Type | Deterministic neuro-symbolic energy systems engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 79 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 311 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install energy-systems-ai999x
```

```python
import energy_systems_ai999x as energy_systems

energy_systems.ask("Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star").show()
# -> SOLVED — P = 27712.8, Q = 20784.6, S = 34641, V_phase = 230.94  (energy_three_phase_power)
energy_systems.ask("Solar position: latitude = 40, longitude = -105, n = 172, clock = 13:00, utc_offset = -7").show()
# -> SOLVED — declination_deg = 23.452, eot_min = -1.34372, hour_angle_deg = 14.6641, zenith_deg = 20.66  (energy_solar_position)
energy_systems.ask("Power in the wind: rho = 1.225 kg/m^3, D = 80 m, v = 10 m/s, cp = 0.4").show()
# -> SOLVED — P_wind = 3.07876e+06, P_betz = 1.82445e+06, power_density = 612.5, area = 5026.55  (energy_wind_power)
energy_systems.ask("Hydrogen tank: V = 125 L, p = 700 bar, T = 15 degC").show()
# -> SOLVED — rho = 40.1722, Z = 1.46617, mass = 5.02152, rho_ideal = 58.8992  (energy_hydrogen_tank)
energy_systems.solve("energy_economic_dispatch", costs="500 5.3 0.004 200 450; 400 5.5 0.006 150 350; 200 5.8 0.009 100 225", demand=800).show()
# -> SOLVED — lambda_ = 8.5, P = [400.0, 250.0, 150.0], cost = 6682.5  (energy_economic_dispatch)
energy_systems.ask("Who created you?").message
# -> energy-systems-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `energy-systems-ai999x "<question>"`, `energy-systems-ai999x --capabilities`. The outputs above were produced by running this exact code on energy-systems-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

79 certified procedures. A few of them:

- **Procedures:** AC load flow by Newton-Raphson; Actuator-disc (momentum theory) rotor performance; Air density at hub height; Angle of incidence of the sun on a tilted surface; Annual energy production from a power curve and a Weibull distribution; Balanced three-phase power from line quantities; Base impedance and per-unit impedance; Battery pack series-parallel sizing; Battery terminal voltage under constant current (one-RC equivalent circuit); Clear-sky global horizontal irradiance (Haurwitz); Clear-sky irradiance (Ineichen-Perez with Linke turbidity); Combined PV system losses (PVWatts); Critical clearing angle and time (equal-area criterion); DC power flow; Diffuse and direct irradiance from global horizontal irradiance (Erbs); Direct normal irradiance from global horizontal irradiance (DISC model); Double line-to-ground fault (symmetrical components); Economic dispatch with quadratic costs and limits; Electrical power of a fuel-cell or electrolyser stack; Equation of time; Extraterrestrial (top-of-atmosphere) normal irradiance; Frequency deviation and load sharing under governor droop; Geometric mean distance of a three-phase line; Hydrogen fuel-cell stack: consumption, power and efficiency …

`energy_systems_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified energy systems capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (solar resource and photovoltaics, wind power, three-phase power, transmission lines, load flow and economic dispatch, faults and stability, transformers and machines, batteries, fuel cells and hydrogen); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{energy_systems_ai999x_1_0_0,
  title  = {energy-systems-ai999x: an offline, verified energy systems solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/energy-systems-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/energy-systems-ai999x/discussions or visit https://ai999x.com.

## Support the project

energy-systems-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/energy-systems-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/energy-systems-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/energy-systems-ai999x/tree/main/onnx).
