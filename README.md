# Energy Systems (energy-systems-ai999x)

[![tests](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/energy-systems-ai999x)](https://pypi.org/project/energy-systems-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/energy-systems-ai999x)

Energy Systems covers power systems and load flow, three-phase power, transformers and machines, solar photovoltaics, wind power, batteries, fuel cells and hydrogen: 83 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Energy Systems desktop app: Incidence angle modifier (ASHRAE), with its worked example and live scene](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/hero.webp)](https://www.ai999x.com/models/energy-systems-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `energy-systems` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/energy-systems-ai999x/#try) |
| **Python** 3.10+ | `pip install energy-systems-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import energy_systems_ai999x as energy_systems

a = energy_systems.ask(
    "Stack electrical power: I = 200 A, N = 100, V_cell = 0.7 V"
)
print(a.outcome, a.values)
# SOLVED {'P': 14000.0, 'P_kW': 14.0, 'V_stack': 70.0}

a = energy_systems.ask(
    "Angle of incidence: tilt = 0, surface_azimuth = 180, zenith = 60, "
    "azimuth = 100"
)
print(a.outcome, a.values)
# SOLVED {'aoi_deg': 60.0, 'cos_aoi': 0.5}

a = energy_systems.ask("Incidence angle modifier (ASHRAE): aoi = 60")
print(a.outcome, a.values)
# SOLVED {'iam': 0.95}

a = energy_systems.ask("Clear-sky GHI (Haurwitz): zenith = 0")
print(a.outcome, a.values)
# SOLVED {'ghi': 1035.09203253}

a = energy_systems.ask(
    "Air density at hub height: p = 1013.3 hPa, h_p = 0 m, T = 288.15 "
    "K, h_T = 0 m, hub = 0 m"
)
print(a.outcome, a.values)
# SOLVED {'T_hub': 288.15, 'p_hub': 101330.0, 'rho': 1.225}

a = energy_systems.ask(
    "Peukert: currents = [1, 10], times = [100, 10], I = 5 A"
)
print(a.outcome, a.values)
# SOLVED {'k': 1.0, 'C_p': 100.0, 'runtime_h': 20.0, 'capacity_Ah': 100.0}

a = energy_systems.ask("Incidence angle modifier (physical): aoi = 0")
print(a.outcome, a.values)
# SOLVED {'iam': 1.0}

a = energy_systems.ask(
    "Inverter output (PVWatts): pdc = 6 kW, pdc0 = 5 kW"
)
print(a.outcome, a.values)
# SOLVED {'pac': 4800.0, 'efficiency': 0.8, 'clipped': True}

a = energy_systems.ask("Synchronous speed: f = 50 Hz, poles = 4")
print(a.outcome, a.values)
# SOLVED {'n_sync_rpm': 1500.0, 'omega_sync': 157.079632679}

a = energy_systems.ask(
    "PVWatts DC power: G_eff = 800 W/m^2, T_cell = 45 degC, pdc0 = 5 "
    "kW, gamma = -0.4%"
)
print(a.outcome, a.values)
# SOLVED {'pdc': 3680.0, 'pdc_kW': 3.68}

a = energy_systems.ask(
    "Density-corrected power curve: speeds = [3, 5, 8, 12, 25], powers "
    "= [0, 150, 800, 2000, 2000], v = 6.5 m/s, rho = 1.225 kg/m^3"
)
print(a.outcome, a.values)
# SOLVED {'P_kW': 475.0, 'P': 475000.0}

a = energy_systems.ask("Equation of time: n = 81, method = pvcdrom")
print(a.outcome, a.values)
# SOLVED {'eot_min': -7.53, 'eot_s': -451.8}

a = energy_systems.ask(
    "Power curve output: speeds = [3, 5, 8, 12, 25], powers = [0, 150, "
    "800, 2000, 2000], v = 6.5 m/s"
)
print(a.outcome, a.values)
# SOLVED {'P_kW': 475.0, 'P': 475000.0}

a = energy_systems.ask(
    "Extraterrestrial irradiance: n = 365, method = asce, "
    "solar_constant = 1367 W/m^2"
)
print(a.outcome, a.values)
# SOLVED {'G_on': 1412.111, 'distance_factor': 1.033}

a = energy_systems.ask(
    "Cell temperature (NOCT): poa = 800 W/m^2, T_air = 25 degC, noct = "
    "45 degC"
)
print(a.outcome, a.values)
# SOLVED {'T_cell': 323.15, 'T_cell_C': 50.0}
```

## Screenshots

[![The Energy Systems desktop app: Incidence angle modifier (ASHRAE), with its worked example and live scene](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/hero-dark.webp)](https://www.ai999x.com/models/energy-systems-ai999x/)

[![Incidence angle modifier from Fresnel reflection and absorption: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/see-1.webp)](https://www.ai999x.com/models/energy-systems-ai999x/)

*Incidence angle modifier from Fresnel reflection and absorption*

[![Base impedance and per-unit impedance: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/see-2.webp)](https://www.ai999x.com/models/energy-systems-ai999x/)

*Base impedance and per-unit impedance*

[![Critical clearing angle and time (equal-area criterion): the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/see-3.webp)](https://www.ai999x.com/models/energy-systems-ai999x/)

*Critical clearing angle and time (equal-area criterion)*

Every calculation, with its formula, a worked example and its figures: **[Energy Systems on ai999x.com](https://www.ai999x.com/models/energy-systems-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/energy-systems-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/energy-systems-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `energy-systems` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/energy-systems`) and its data (`rust/models/energy_systems/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p energy-systems-ai999x
./target/release/energy-systems --help
cargo test -p energy-systems-ai999x
```
