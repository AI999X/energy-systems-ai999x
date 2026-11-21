# Energy Systems (energy-systems-ai999x)

[![tests](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/energy-systems-ai999x)](https://pypi.org/project/energy-systems-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/energy-systems-ai999x)

Energy Systems covers power systems and load flow, three-phase power, transformers and machines, solar photovoltaics, wind power, batteries, fuel cells and hydrogen: 83 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Energy Systems desktop app: Balanced three-phase power from line quantities, with its worked example and live scene](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/hero.webp)](https://www.ai999x.com/models/energy-systems-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-energy-systems-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-energy-systems-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `energy-systems` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/energy-systems-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install energy-systems-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Energy Systems desktop app: every scene and chart is drawn from the computed answer.

[![Energy Systems desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/clip.mp4)*

| | |
|---|---|
| [![Balanced three-phase power from line quantities: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-1.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_three_phase_power)<br>Balanced three-phase power from line quantities | [![Sun position at a place and local time: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-2.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_solar_position)<br>Sun position at a place and local time |
| [![Power in the wind and the Betz limit: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-3.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_wind_power)<br>Power in the wind and the Betz limit | [![Hydrogen stored in a tank at pressure (real gas): the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-4.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_hydrogen_tank)<br>Hydrogen stored in a tank at pressure (real gas) |
| [![Economic dispatch with quadratic costs and limits: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-5.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_economic_dispatch)<br>Economic dispatch with quadratic costs and limits | [![Battery pack series-parallel sizing: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-6.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_battery_pack_sizing)<br>Battery pack series-parallel sizing |
| [![Electrode kinetics: Butler-Volmer current, overpotential and Tafel slopes: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-7.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_butler_volmer)<br>Electrode kinetics: Butler-Volmer current, overpotential and Tafel slopes | [![Capacity factor and energy of a generating plant: the live scene in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-8.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_capacity_factor)<br>Capacity factor and energy of a generating plant |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Balanced three-phase power from line quantities: a chart in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/chart-1.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_three_phase_power)<br>Balanced three-phase power from line quantities | [![Sun position at a place and local time: a chart in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/chart-2.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_solar_position)<br>Sun position at a place and local time | [![Power in the wind and the Betz limit: a chart in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/chart-3.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_wind_power)<br>Power in the wind and the Betz limit | [![Hydrogen stored in a tank at pressure (real gas): a chart in the Energy Systems desktop app](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/chart-4.webp)](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_hydrogen_tank)<br>Hydrogen stored in a tank at pressure (real gas) |

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

## Command line: 15 examples

Each command below was run with `energy-systems` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ energy-systems solve energy_battery_pack_sizing "V_target=48 V" "E_target=10 kWh" "V_cell=3.2 V" "Q_cell=360000 C"
N_series = 15
N_parallel = 3
N_cells = 45
V_pack = 48 V
E_pack_kWh = 14.4
Q_pack_Ah = 300

$ energy-systems solve energy_stack_electrical_power "I=200 A" N=100 "V_cell=0.7 V"
P = 14000 W
P_kW = 14
V_stack = 70 V

$ energy-systems solve energy_capacity_factor "P_rated=2 MW" "E=5256 MWh"
capacity_factor = 0.3
capacity_factor_pct = 30
E = 1.89216e+13 J
E_MWh = 5256
full_load_hours = 2628
average_power = 600000 W

$ energy-systems solve energy_hydrogen_tank "V=1 m^3" "p=700 bar" "T=288.15 K"
rho = 40.17216108 kg/m^3
Z = 1.466169638
mass = 40.17216108 kg
rho_ideal = 58.89920286 kg/m^3
mass_ideal = 58.89920286 kg
ideal_over_real = 1.466169638

$ energy-systems solve energy_induction_max_torque "VL=460 V" "f=60 Hz" poles=4 "R1=0 ohm" "X1=1 ohm" "R2=0.2 ohm" "X2=1 ohm" "Xm=1e+12 ohm" connection=star
s_maxT = 0.1
T_max = 280.6432163 N*m
T_start = 55.57291412 N*m
I_start = 132.1315476 A
V_th = 265.5811238 V
R_th = 0 ohm

$ energy-systems solve energy_pvwatts_losses soiling=0 shading=0 snow=0 mismatch=0 wiring=0 connections=0 lid=0 nameplate_rating=0 age=0 availability=0.1
total_loss = 0.1
performance_factor = 0.9
total_loss_pct = 10

$ energy-systems solve energy_newton_load_flow "lines=1 2 0.0 0.5 0.0" "loads=2 0.0 0.0"
V = [1.0,1.0]
angle_deg = [0.0,0.0]
P_slack = 0
Q_slack = 0
P_loss = 0
Q_gen = []

$ energy-systems solve energy_angle_of_incidence tilt=0 surface_azimuth=180 zenith=60 azimuth=100
aoi_deg = 60
cos_aoi = 0.5

$ energy-systems solve energy_three_phase_power "VL=400 V" "IL=50 A" pf=0.8 connection=star
P = 27712.81292 W
Q = 20784.60969 var
S = 34641.01615 VA
V_phase = 230.9401077 V
I_phase = 50 A

$ energy-systems solve energy_transformer_max_efficiency "S=100 kVA" "P_core=500 W" "P_cu_fl=2000 W" pf=1
load_fraction = 0.5
S_at_max = 50000 VA
efficiency_max_pct = 98.03921569
efficiency_full_load_pct = 97.56097561

$ energy-systems solve energy_actuator_disc a=0.25 "rho=1.2 kg/m^3" "D=10 m" "v=8 m/s"
Cp = 0.5625
Ct = 0.75
v_disc = 6 m/s
v_wake = 4 m/s
P = 13571.68026 W
T = 2261.946711 N

$ energy-systems solve energy_battery_rc_discharge "OCV=4 V" "R0=10 mohm" "R1=20 mohm" "C1=1000 F" "I=10 A" "t=0 s"
V_terminal = 3.9 V
V_rc = 0 V
E_delivered = 0 J
E_Wh = 0
heat = 0 J
tau = 20 s

$ energy-systems solve energy_butler_volmer j0=10 alpha_a=0.5 alpha_c=0.5 "T=298.15 K" "eta=100 mV"
j = 68.58407791
j_mA_cm2 = 6.858407791
eta = 0.1 V
eta_mV = 100
j_anodic = 70.0123964
j_cathodic = -1.428318486

$ energy-systems solve energy_hydro_power "Q=10 m^3/s" "H=50 m" eta=0.9
P = 4414500 W
P_kW = 4414.5
P_MW = 4.4145
P_hydraulic = 4905000 W
E = 1.39215672e+14 J
E_MWh = 38671.02

$ energy-systems solve energy_hydrogen_isentropic_compression "p1=30 bar" "T1=300 K" "p2=100 bar"
T2 = 423.2935564 K
w = 1833212.524 J/kg
w_kWh_per_kg = 0.5092257011
```

## Rust library: 15 examples

The crate `energy-systems-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `energy_systems_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/energy-systems/examples/readme.rs`: `cd rust && cargo run --release --example readme -p energy-systems-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&energy_systems_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "energy_battery_pack_sizing", "inputs": {"V_target": "48 V", "E_target": "10 kWh", "V_cell": "3.2 V", "Q_cell": "360000 C"}}));
// SOLVED
// N_series = 15
// N_parallel = 3
// N_cells = 45
// V_pack = 48.0 V
// E_pack_kWh = 14.4
// Q_pack_Ah = 300.0

show(json!({"key": "energy_stack_electrical_power", "inputs": {"I": "200 A", "N": "100", "V_cell": "0.7 V"}}));
// SOLVED
// P = 14000.0 W
// P_kW = 14.0
// V_stack = 70.0 V

show(json!({"key": "energy_capacity_factor", "inputs": {"P_rated": "2 MW", "E": "5256 MWh"}}));
// SOLVED
// capacity_factor = 0.3
// capacity_factor_pct = 30.0
// E = 18921600000000.0 J
// E_MWh = 5256.0
// full_load_hours = 2628.0
// average_power = 600000.0 W

show(json!({"key": "energy_hydrogen_tank", "inputs": {"V": "1 m^3", "p": "700 bar", "T": "288.15 K"}}));
// SOLVED
// rho = 40.1721610778 kg/m^3
// Z = 1.46616963797
// mass = 40.1721610778 kg
// rho_ideal = 58.899202864 kg/m^3
// mass_ideal = 58.899202864 kg
// ideal_over_real = 1.46616963797

show(json!({"key": "energy_induction_max_torque", "inputs": {"VL": "460 V", "f": "60 Hz", "poles": "4", "R1": "0 ohm", "X1": "1 ohm", "R2": "0.2 ohm", "X2": "1 ohm", "Xm": "1e+12 ohm", "connection": "star"}}));
// SOLVED
// s_maxT = 0.1
// T_max = 280.643216318 N*m
// T_start = 55.5729141225 N*m
// I_start = 132.131547613 A
// V_th = 265.581123827 V
// R_th = 0.0 ohm

show(json!({"key": "energy_pvwatts_losses", "inputs": {"soiling": "0", "shading": "0", "snow": "0", "mismatch": "0", "wiring": "0", "connections": "0", "lid": "0", "nameplate_rating": "0", "age": "0", "availability": "0.1"}}));
// SOLVED
// total_loss = 0.1
// performance_factor = 0.9
// total_loss_pct = 10.0

show(json!({"key": "energy_newton_load_flow", "inputs": {"lines": "1 2 0.0 0.5 0.0", "loads": "2 0.0 0.0"}}));
// SOLVED
// V = [1.0,1.0]
// angle_deg = [0.0,0.0]
// P_slack = 0.0
// Q_slack = 0.0
// P_loss = 0.0
// Q_gen = []

show(json!({"key": "energy_angle_of_incidence", "inputs": {"tilt": "0", "surface_azimuth": "180", "zenith": "60", "azimuth": "100"}}));
// SOLVED
// aoi_deg = 60.0
// cos_aoi = 0.5

show(json!({"key": "energy_three_phase_power", "inputs": {"VL": "400 V", "IL": "50 A", "pf": "0.8", "connection": "star"}}));
// SOLVED
// P = 27712.8129211 W
// Q = 20784.6096908 var
// S = 34641.0161514 VA
// V_phase = 230.940107676 V
// I_phase = 50.0 A

show(json!({"key": "energy_transformer_max_efficiency", "inputs": {"S": "100 kVA", "P_core": "500 W", "P_cu_fl": "2000 W", "pf": "1"}}));
// SOLVED
// load_fraction = 0.5
// S_at_max = 50000.0 VA
// efficiency_max_pct = 98.0392156863
// efficiency_full_load_pct = 97.5609756098

show(json!({"key": "energy_actuator_disc", "inputs": {"a": "0.25", "rho": "1.2 kg/m^3", "D": "10 m", "v": "8 m/s"}}));
// SOLVED
// Cp = 0.5625
// Ct = 0.75
// v_disc = 6.0 m/s
// v_wake = 4.0 m/s
// P = 13571.6802635 W
// T = 2261.94671058 N

show(json!({"key": "energy_battery_rc_discharge", "inputs": {"OCV": "4 V", "R0": "10 mohm", "R1": "20 mohm", "C1": "1000 F", "I": "10 A", "t": "0 s"}}));
// SOLVED
// V_terminal = 3.9 V
// V_rc = 0.0 V
// E_delivered = 0.0 J
// E_Wh = 0.0
// heat = 0.0 J
// tau = 20.0 s

show(json!({"key": "energy_butler_volmer", "inputs": {"j0": "10", "alpha_a": "0.5", "alpha_c": "0.5", "T": "298.15 K", "eta": "100 mV"}}));
// SOLVED
// j = 68.5840779148
// j_mA_cm2 = 6.85840779148
// eta = 0.1 V
// eta_mV = 100.0
// j_anodic = 70.0123964004
// j_cathodic = -1.4283184856

show(json!({"key": "energy_hydro_power", "inputs": {"Q": "10 m^3/s", "H": "50 m", "eta": "0.9"}}));
// SOLVED
// P = 4414500.0 W
// P_kW = 4414.5
// P_MW = 4.4145
// P_hydraulic = 4905000.0 W
// E = 139215672000000.0 J
// E_MWh = 38671.02

show(json!({"key": "energy_hydrogen_isentropic_compression", "inputs": {"p1": "30 bar", "T1": "300 K", "p2": "100 bar"}}));
// SOLVED
// T2 = 423.293556425 K
// w = 1833212.52408 J/kg
// w_kWh_per_kg = 0.509225701134
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Energy Systems on ai999x.com](https://www.ai999x.com/models/energy-systems-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/energy-systems-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/energy-systems-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `energy-systems` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/energy-systems`) and its data (`rust/models/energy_systems/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p energy-systems-ai999x
./target/release/energy-systems --help
cargo test -p energy-systems-ai999x
```
