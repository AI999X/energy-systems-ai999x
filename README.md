# energy-systems-ai999x — offline, verified energy systems solver with step-by-step answers

[![tests](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/energy-systems-ai999x)](https://pypi.org/project/energy-systems-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/energy-systems-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/energy-systems-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/energy-systems-ai999x/)** · **[Guides](https://huggingface.co/AI999X/energy-systems-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/energy-systems-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

energy-systems-ai999x is an offline, deterministic Python solver that answers energy systems questions with verified results from 79 certified capabilities, gives 311 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**energy-systems-ai999x** is a compact, deterministic energy systems engine for Python. It reads a question in plain English (or a typed call), picks a certified energy systems capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** energy-systems-ai999x comes as the AI999X desktop app and as the command-line program `energy-systems` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Energy Systems desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-energy-systems-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-energy-systems-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`energy-systems`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/energy-systems-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install energy-systems-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Energy Systems** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Energy Systems among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-energy-systems-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-energy-systems-1.4.0-linux-x86_64.tar.gz
./ai999x-energy-systems-1.4.0-linux-x86_64/ai999x-energy-systems
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
energy-systems --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
energy-systems --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ energy-systems ask "Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star"
Read as: Balanced three-phase power from line quantities
P = 27712.81292 W
Q = 20784.60969 var
S = 34641.01615 VA
V_phase = 230.9401077 V
I_phase = 50 A
  by: Balanced three-phase power from line quantities  (S = sqrt(3) V_L I_L, P = S pf, Q = S sin(acos pf); star: V_ph = V_L/sqrt(3), I_ph = I_L; delta: V_ph = V_L, I_ph = I_L/sqrt(3))
  IL = 50 A
  VL = 400 V
  pf = 0.8
  connection = star
  assumes: balanced load; 0 < pf <= 1
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ energy-systems info energy_three_phase_power
Balanced three-phase power from line quantities
  S = sqrt(3) V_L I_L, P = S pf, Q = S sin(acos pf); star: V_ph = V_L/sqrt(3), I_ph = I_L; delta: V_ph = V_L, I_ph = I_L/sqrt(3)
  in   VL       line-to-line rms voltage [V]
  in   IL       line current (rms) [A]
  in   pf       power factor (0 < pf <= 1) []
  in   connection connection of the load (star = wye) (choice) []
  in   pf_type  lagging (inductive, default) or leading (capacitive) (choice) []
  out  P        real power (three-phase total) [W]
  out  Q        reactive power in var (positive lagging) [var]
  out  S        apparent power in VA [VA]
  out  V_phase  voltage across each phase of the load [V]
  out  I_phase  current in each phase of the load [A]
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ energy-systems solve energy_three_phase_power "VL=400 V" "IL=50 A" pf=0.8 connection=star
P = 27712.81292 W
Q = 20784.60969 var
S = 34641.01615 VA
V_phase = 230.9401077 V
I_phase = 50 A
  by: Balanced three-phase power from line quantities  (S = sqrt(3) V_L I_L, P = S pf, Q = S sin(acos pf); star: V_ph = V_L/sqrt(3), I_ph = I_L; delta: V_ph = V_L, I_ph = I_L/sqrt(3))
  VL = 400 V
  IL = 50 A
  pf = 0.8
  connection = star
  assumes: balanced load; 0 < pf <= 1
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | energy-systems batch

`cases.jsonl`:

```json
{"key": "energy_three_phase_power", "inputs": {"VL": "400 V", "IL": "50 A", "pf": 0.8, "connection": "star"}}
{"key": "energy_solar_position", "inputs": {"latitude": 40.0, "longitude": -105.0, "n": "172", "clock": 13.0, "utc_offset": -7.0}}
```

```console
$ energy-systems batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":27712.8129211,"unit":"W","solved_for":"P","si":[["VL",400.0],["IL",50.0],["pf",0.8]],"values":[["P",27712.8129211,"W"],["Q",20784.6096908,"var"],["S",34641.0161514,"VA"],["V_phase",230.940107676,"V"],["I_phase",50.0,"A"]],"target":"P","phrase":"real power (three-phase total)","assumptions":["balanced load","0 < pf <= 1"],"name":"Balanced three-phase power from line quantities","equation":"S = sqrt(3) V_L I_L, P = S pf, Q = S sin(acos pf); star: V_ph = V_L/sqrt(3), I_ph = I_L; delta: V_ph = V_L, I_ph = I_L/sqrt(3)"}
{"outcome":"SOLVED","message":"","value":69.3399621499,"unit":"","solved_for":"elevation_deg","si":[["latitude",40.0],["longitude",-105.0],["clock",13.0],["utc_offset",-7.0]],"values":[["declination_deg",23.4520460745,""],["eot_min",-1.34372473169,""],["hour_angle_deg",14.6640688171,""],["zenith_deg",20.6600378501,""],["elevation_deg",69.3399621499,""],["azimuth_deg",221.165264952,""]],"target":"elevation_deg","phrase":"declination in degrees","assumptions":["1 <= n <= 366","0 <= clock < 24","-90 < latitude < 90"],"name":"Sun position at a place and local time","equation":"Spencer declination and equation of time; omega = 15 (t - UTC offset - 12) + longitude + E/4; cos z = cos(delta) cos(phi) cos(omega) + sin(delta) sin(phi)"}
```

### In your browser

Open [Energy Systems on ai999x.com](https://www.ai999x.com/models/energy-systems-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `energy_systems_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install energy-systems-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Battery pack series-parallel sizing

The number of cells in series to reach a pack voltage and of strings in parallel to reach a pack energy, with the resulting pack voltage, energy and capacity (fewest cells; nominal values).

Command line:

```console
$ energy-systems solve energy_battery_pack_sizing "V_target=48 V" "E_target=10 kWh" "V_cell=3.2 V" "Q_cell=360000 C"
N_series = 15
N_parallel = 3
N_cells = 45
V_pack = 48 V
E_pack_kWh = 14.4
Q_pack_Ah = 300
  by: Battery pack series-parallel sizing  (N_s = ceil(V/V_cell); N_p = ceil(E/(N_s V_cell Q_cell)))
  V_target = 48 V
  E_target = 36000000 J
  V_cell = 3.2 V
  Q_cell = 360000 C
  assumes: nominal voltages and capacities; a target met within 1e-9 counts as met
```

Python:

```python
import energy_systems_ai999x as m

a = m.solve("energy_battery_pack_sizing", V_target="48 V", E_target="10 kWh", V_cell="3.2 V", Q_cell="360000 C")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_battery_pack_sizing) and press **Compute in your browser**.

_What this means:_ This calculation determines the minimum number of identical cells needed to meet the target voltage of 48 V and energy of 10 kWh. Using a nominal cell voltage of 3.2 V and capacity of 360000 C, the algorithm first rounds up the series count to satisfy the voltage requirement, yielding 15 cells in series. It then calculates the necessary parallel strings to meet the energy demand with those series cells, resulting in 3 strings. The final configuration provides a nominal pack voltage of 48 V and stores 14.4 of energy, ensuring all targets are met or exceeded with the fewest total cells possible.

### Battery terminal voltage under constant current (one-RC equivalent circuit)

Terminal voltage, polarization (RC) voltage, energy delivered and heat generated after a constant-current discharge of duration t, for a cell modelled as an open-circuit voltage with a series resistance R0 and one R1 || C1 branch (constant OCV).

Command line:

```console
$ energy-systems solve energy_battery_rc_discharge "OCV=4 V" "R0=10 mohm" "R1=20 mohm" "C1=1000 F" "I=10 A" "t=0 s"
V_terminal = 3.9 V
V_rc = 0 V
E_delivered = 0 J
E_Wh = 0
heat = 0 J
tau = 20 s
  by: Battery terminal voltage under constant current (one-RC equivalent circuit)  (V1 = I R1 (1 - e^(-t/tau)), V_t = OCV - I R0 - V1; energy and heat integrated in closed form)
  OCV = 4 V
  R0 = 0.01 ohm
  R1 = 0.02 ohm
  C1 = 1000 F
  I = 10 A
  t = 0 s
  assumes: constant open-circuit voltage over the interval (short pulse or flat OCV curve); RC branch initially relaxed
```

Python:

```python
import energy_systems_ai999x as m

a = m.solve("energy_battery_rc_discharge", OCV="4 V", R0="10 mohm", R1="20 mohm", C1="1000 F", I="10 A", t="0 s")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_battery_rc_discharge) and press **Compute in your browser**.

_What this means:_ This calculation determines the battery's terminal voltage after a constant current discharge of duration 0 s. It starts with the open circuit voltage 4 V and subtracts the immediate ohmic drop caused by series resistance 10 mohm as well as the gradual polarization voltage 0 V from the RC branch. The resulting terminal voltage is 3.9 V. During this process, the system delivered energy of 0 J to the load while generating 0 J of internal heat due to resistive losses.

### Peukert exponent and run time at a new discharge current

Peukert's law t = C_p/I^k fitted (least squares on the logarithms) to two or more constant-current discharge tests, then the run time and delivered capacity at another current.

Command line:

```console
$ energy-systems solve energy_peukert "currents=[1, 10]" "times=[100, 10]" "I=5 A"
k = 1
C_p = 100
runtime_h = 20
capacity_Ah = 100
  by: Peukert exponent and run time at a new discharge current  (ln t = ln C_p - k ln I fitted by least squares; t(I) = C_p I^-k)
  I = 5 A
  assumes: at least two tests at different currents; constant temperature and cut-off voltage
```

Python:

```python
import energy_systems_ai999x as m

a = m.solve("energy_peukert", currents=[1, 10], times=[100, 10], I="5 A")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_peukert) and press **Compute in your browser**.

_What this means:_ This calculation determines how long a battery will last under a new discharge condition by applying Peukert's law. Using least squares regression on logarithmic data from previous tests gives the Peukert capacity 100 and the exponent 1. These parameters predict that at the specific current 5 A, the run time will be 20 hours, delivering a total capacity of 100 amp-hours.

### State of charge by coulomb counting

The state of charge of a battery after a sequence of constant currents over equal time steps (positive on discharge), by coulomb counting SOC = SOC0 - integral(I dt)/Q, with its lowest and highest value and the charge delivered.

Command line:

```console
$ energy-systems solve energy_coulomb_counting SOC0=0.9 "Q=180000 C" "currents=[10, 20, -5]" "dt=1 h"
SOC = 0.4
SOC_min = 0.3
SOC_max = 0.9
charge_out = 90000 C
charge_out_Ah = 25
  by: State of charge by coulomb counting  (SOC_k = SOC0 - (sum_{j<=k} I_j dt)/Q)
  SOC0 = 0.9
  Q = 180000 C
  dt = 3600 s
  assumes: 0 <= SOC0 <= 1; Q, dt > 0; the state of charge stays within 0 to 100%; no coulombic losses
```

Python:

```python
import energy_systems_ai999x as m

a = m.solve("energy_coulomb_counting", SOC0="0.9", Q="180000 C", currents=[10, 20, -5], dt="1 h")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/energy-systems-ai999x/#cap-energy_coulomb_counting) and press **Compute in your browser**.

_What this means:_ This calculation determines the battery's state of charge by tracking the current flow over time. Starting from an initial value of 0.9, the system subtracts the total charge removed from the capacity 180000 C. The result shows that the state of charge ended at 0.4, having varied between 0.3 and 0.9 during the interval, while a net amount of 90000 C coulombs was delivered to the load.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/energy-systems-ai999x/blob/main/docs/energy_systems_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/energy-systems-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified energy systems capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 311 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `energy-systems` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** solar resource and photovoltaics, wind power, three-phase power, transmission lines, load flow and economic dispatch, faults and stability, transformers and machines, batteries, fuel cells and hydrogen.

## Model overview

| Property | Value |
|---|---|
| Package | `energy-systems-ai999x` 1.1.0 (import `energy_systems_ai999x`) |
| Type | Deterministic neuro-symbolic energy systems engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 79 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 311 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `energy-systems-ai999x "<question>"`; for the native program use `energy-systems` (above). The outputs above were produced by running this exact code on energy-systems-ai999x 1.1.0.

## Find the right capability, and draw it

`energy_systems.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "energy-systems-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import energy_systems_ai999x as energy_systems

energy_systems.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `energy-systems-ai999x`; the answer is the one it returned, with the steps it took.

**Balanced three-phase power from line quantities** · _Three-phase power_

> Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star

→ **P = 27712.81292 W, Q = 20784.60969 var, S = 34641.01615 VA, V_phase = 230.9401077 V, I_phase = 50 A**

<details><summary>Steps</summary>

- Balanced three-phase power from line quantities: VL = 400 V, IL = 50 A, pf = 0.8, connection = star → P = 27712.81292 W, Q = 20784.60969 var, S = 34641.01615 VA, V_phase = 230.9401077 V, I_phase = 50 A

</details>

**Sun position at a place and local time** · _Solar resource_

> Solar position: latitude = 40, longitude = -105, n = 172, clock = 13:00, utc_offset = -7

→ **declination_deg = 23.45204607, eot_min = -1.343724732, hour_angle_deg = 14.66406882, zenith_deg = 20.66003785, elevation_deg = 69.33996215, azimuth_deg = 221.165265**

<details><summary>Steps</summary>

- Sun position at a place and local time: latitude = 40, longitude = -105, n = 172, clock = 13, utc_offset = -7 → declination_deg = 23.45204607, eot_min = -1.343724732, hour_angle_deg = 14.66406882, zenith_deg = 20.66003785, elevation_deg = 69.33996215, azimuth_deg = 221.165265

</details>

**Power in the wind and the Betz limit** · _Wind power_

> Power in the wind: rho = 1.225 kg/m^3, D = 80 m, v = 10 m/s, cp = 0.4

→ **P_wind = 3078760.801 W, P_betz = 1824450.845 W, power_density = 612.5, area = 5026.548246 m^2, P = 1231504.32 W**

<details><summary>Steps</summary>

- Power in the wind and the Betz limit: rho = 1.225 kg/m^3, D = 80 m, v = 10 m/s, cp = 0.4 → P_wind = 3078760.801 W, P_betz = 1824450.845 W, power_density = 612.5 W, area = 5026.548246 m^2, P = 1231504.32 W

</details>

**Hydrogen stored in a tank at pressure (real gas)** · _Hydrogen_

> Hydrogen tank: V = 125 L, p = 700 bar, T = 15 degC

→ **rho = 40.17216108 kg/m^3, Z = 1.466169638, mass = 5.021520135 kg, rho_ideal = 58.89920286 kg/m^3, mass_ideal = 7.362400358 kg, ideal_over_real = 1.466169638, energy_LHV_kWh = 167.3289261**

<details><summary>Steps</summary>

- Hydrogen stored in a tank at pressure (real gas): V = 125 L, p = 700 bar, T = 15 degC → rho = 40.17216108 kg/m^3, Z = 1.466169638 kg, mass = 5.021520135 kg, rho_ideal = 58.89920286 kg/m^3, mass_ideal = 7.362400358 kg, ideal_over_real = 1.466169638 kg, energy_LHV_kWh = 167.3289261 kg

</details>

**Economic dispatch with quadratic costs and limits** · _Power systems_

`m.solve("energy_economic_dispatch", costs="500 5.3 0.004 200 450; 400 5.5 0.006 150 350; 200 5.8 0.009 100 225", demand=800)`

→ **lambda_ = 8.5, P = [400, 250, 150], cost = 6682.5**

<details><summary>Steps</summary>

- Economic dispatch with quadratic costs and limits: costs = [500 5.3 0.004 200 450; 400 5.5 0.006 150 350; 200 5.8 0.009 100 225], demand = 800 → lambda_ = 8.5, P = [400, 250, 150], cost = 6682.5

</details>

**Battery pack series-parallel sizing** · _Batteries_

> Battery pack sizing: V_target = 48 V, E_target = 10 kWh, V_cell = 3.2 V, Q_cell = 100 Ah

→ **N_series = 15, N_parallel = 3, N_cells = 45, V_pack = 48 V, E_pack_kWh = 14.4, Q_pack_Ah = 300**

<details><summary>Steps</summary>

- Battery pack series-parallel sizing: V_target = 48 V, E_target = 10 kWh, V_cell = 3.2 V, Q_cell = 360000 C → N_series = 15, N_parallel = 3, N_cells = 45, V_pack = 48 V, E_pack_kWh = 14.4, Q_pack_Ah = 300

</details>

## Figures

Every point was computed by `energy-systems-ai999x`.

**Photovoltaic module: current–voltage curve and maximum power**

![Photovoltaic module: current–voltage curve and maximum power — computed by energy-systems-ai999x](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-single-diode-mpp-curve.svg)

_Current–voltage curve of a PV module (single-diode model: photocurrent 9 A, saturation current 1.0000e-10 A, Rs = 0.3 Ω, Rsh = 400 Ω), computed by the package at 81 voltages. The shaded rectangle is the maximum power the package finds, P = 335.252 W at V = 39.6025 V, I = 8.46542 A; it fills 0.778276 of Isc × Voc (8.99326 A × 47.8984 V), the fill factor._

**Wind turbine power curve**

![Wind turbine power curve — computed by energy-systems-ai999x](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-power-curve-output-curve.svg)

_Wind turbine output against wind speed from the tabulated power curve (speeds 3, 5, 8, 12, 25 m/s, powers 0, 150, 800, 2000, 2000 kW), computed by the package at 121 speeds from 0 to 30 m/s: nothing below cut-in at 3 m/s, rated 2000 kW up to cut-out at 25 m/s, nothing above. Marked: v = 6.5 m/s, P = 475 kW._

**The Sun's path across the sky: polar sky chart**

![The Sun's path across the sky: polar sky chart — computed by energy-systems-ai999x](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-solar-position-sky.svg)

_The Sun's path across the sky at latitude 40°, longitude -105° (UTC-7) on three days: azimuth around the rim (N at the top) and elevation inward (rings at 0°, 30° and 60°), computed by the package every 15 minutes while the Sun is up; dots every 2 h of clock time. Highest computed elevations: 21 June 73.4496°, 22 March 50.2928°, 21 December 26.5781°. Marked: 21 June at 13 h, elevation 69.34°, azimuth 221.165°._

**Economic dispatch: three generators sharing a load**

![Economic dispatch: three generators sharing a load — computed by energy-systems-ai999x](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-economic-dispatch-bars.svg)

_Economic dispatch of 3 generators with quadratic cost curves (rows a, b, c, Pmin, Pmax: 500 5.3 0.004 200 450; 400 5.5 0.006 150 350; 200 5.8 0.009 100 225) meeting a demand of 800 MW: the package returns outputs 400, 250, 150 MW (bars, dashed lines the limits) at a common incremental cost λ = 8.5 per MWh, total cost 6682.5 per hour._

**Wind speed with height: the shear profile as arrows**

![Wind speed with height: the shear profile as arrows — computed by energy-systems-ai999x](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-wind-shear-log-profile.svg)

_Wind speed at nine heights extrapolated from 5 m/s measured at 10 m, drawn as arrows whose length is the speed: logarithmic law (roughness length 1 m) and power law (exponent 0.333333), each computed by the package. At 100 m: 10 and 10.7722 m/s; at 150 m: 10.8805 and 12.3311 m/s._

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
@software{energy_systems_ai999x_1_1_0,
  title  = {energy-systems-ai999x: an offline, verified energy systems solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
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
