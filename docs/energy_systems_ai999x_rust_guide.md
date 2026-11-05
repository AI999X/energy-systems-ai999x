# energy-systems: the Energy Systems program — user guide

**energy-systems-ai999x 1.0.3**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Energy Systems covers power systems and load flow, three-phase power, transformers and machines, solar photovoltaics, wind power, batteries, fuel cells and hydrogen. This guide is about `energy-systems`, the model as a self-contained program for Windows and Linux: 79 capabilities in 10 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/energy-systems-ai999x/); the Python package (`pip install energy-systems-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.4.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS; the newest version is always under https://www.ai999x.com/downloads/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Energy Systems | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-energy-systems-1.4.0-windows-x86_64.zip |
| Desktop app: Energy Systems | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-energy-systems-1.4.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz |

### Desktop app

Energy Systems in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-energy-systems-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-energy-systems.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-energy-systems-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-energy-systems-1.4.0-linux-x86_64.tar.gz
./ai999x-energy-systems-1.4.0-linux-x86_64/ai999x-energy-systems
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.4.0-linux-x86_64.tar.gz
./ai999x-1.4.0-linux-x86_64/ai999x
```

### Command line — Linux

The bundle holds one program per model; `energy-systems` is this one. The checksum line should print OK.

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
energy-systems --version
```

### Command line — Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
energy-systems --version
```

### In your browser

Open https://www.ai999x.com/models/energy-systems-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install energy-systems-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ energy-systems --version
energy-systems-ai999x 1.0.3 (Rust build, ax-engine 0.1.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ energy-systems search "balanced three-phase power from line quantities"
energy_three_phase_power             Balanced three-phase power from line quantities  (three-phase power / balanced systems)
energy_geometric_mean_distance       Geometric mean distance of a three-phase line  (power systems / line parameters)
energy_line_parameters               Inductance, capacitance and surge impedance of a three-phase overhead line  (power systems / line parameters)
energy_symmetrical_fault             Three-phase (symmetrical) fault current by the bus impedance matrix  (power systems / faults)
energy_two_bus_load_flow             Receiving-end voltage of a line feeding a constant-power load (two-bus load flow)  (power systems / load flow)
energy_medium_line                   Medium transmission line (nominal pi)  (power systems / transmission lines)
energy_long_line                     Long transmission line (exact distributed parameters)  (power systems / transmission lines)
energy_short_line                    Short transmission line: sending-end voltage, regulation and efficiency  (power systems / transmission lines)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ energy-systems list
energy_actuator_disc                     Actuator-disc (momentum theory) rotor performance
energy_air_mass                          Optical air mass
energy_angle_of_incidence                Angle of incidence of the sun on a tilted surface
energy_battery_pack_sizing               Battery pack series-parallel sizing
energy_battery_rc_discharge              Battery terminal voltage under constant current (one-RC equivalent circuit)
energy_cell_temperature_faiman           PV module temperature (Faiman model)
energy_cell_temperature_noct             PV cell temperature from NOCT
energy_cell_temperature_sapm             PV cell and module temperature (Sandia SAPM model)
… 71 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ energy-systems solve energy_three_phase_power "VL=400 V" "IL=50 A" pf=0.8 connection=star --json
{"outcome":"SOLVED","message":"","value":27712.8129211,"unit":"W","solved_for":"P","si":[["VL",400.0],["IL",50.0],["pf",0.8]],"values":[["P",27712.8129211,"W"],["Q",20784.6096908,"var"],["S",34641.0161514,"VA"],["V_phase",230.940107676,"V"],["I_phase",50.0,"A"]],"target":"P","phrase":"real power (three-phase total)","assumptions":["balanced load","0 < pf <= 1"],"name":"Balanced three-phase power from line quantities","equation":"S = sqrt(3) V_L I_L, P = S pf, Q = S sin(acos pf); star: V_ph = V_L/sqrt(3), I_ph = I_L; delta: V_ph = V_L, I_ph = I_L/sqrt(3)"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ energy-systems solve energy_three_phase_power "VL=400 V" "IL=50 A" pf=0.8
NEEDS_INPUT: energy_three_phase_power needs connection (connection of the load (star = wye)). Example: solve('energy_three_phase_power', VL='400 V', IL='50 A', pf=0.8, connection='star').
```

### 2.9 Draw the figures

The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```shell
$ energy-systems figures energy_three_phase_power "VL=400 V" "IL=50 A" pf=0.8 connection=star --out .
[
  {
    "slug": "energy-three-phase-power-phasor",
    "title": "Three-phase power triangle",
    "caption": "Power triangle of the three-phase load: the model returns P = 27.71 kW, Q = 20.78 kvar and S = 34.64 kVA (phase voltage 230.9 V, phase current 50 A); φ = 36.87° is the angle of the returned P and Q. In SI units: P = 2.771e+04 W. Your inputs: VL = 400 V, IL = 50 A, pf = 0.8, connection = star.",
    "file": ".\\energy-three-phase-power-phasor.svg"
  },
  {
    "slug": "energy-three-phase-power-live-1",
    "title": "Real power (three-phase total) as line-to-line rms voltage varies",
    "caption": "Real power (three-phase total) as VL runs from 100 to 700 V, every other input as you entered it (41 solutions). Marked: your VL = 400 V gives P = 2.771e+04. Your inputs: VL = 400 V, IL = 50 A, pf = 0.8, connection = star.",
    "file": ".\\energy-three-phase-power-live-1.svg"
  },
  {
    "slug": "energy-three-phase-power-live-2",
    "title": "Real power (three-phase total) as line current (rms) varies",
    "caption": "Real power (three-phase total) as IL runs from 12.5 to 87.5 A, every other input as you entered it (29 solutions). Marked: your IL = 50 A gives P = 2.771e+04. Your inputs: VL = 400 V, IL = 50 A, pf = 0.8, connection = star.",
    "file": ".\\energy-three-phase-power-live-2.svg"
  }
]
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | energy-systems batch

`cases.jsonl`:

```json
{"key": "energy_three_phase_power", "inputs": {"VL": "400 V", "IL": "50 A", "pf": 0.8, "connection": "star"}}
{"key": "energy_solar_position", "inputs": {"latitude": 40.0, "longitude": -105.0, "n": "172", "clock": 13.0, "utc_offset": -7.0}}
```

```shell
$ energy-systems batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":27712.8129211,"unit":"W","solved_for":"P","si":[["VL",400.0],["IL",50.0],["pf",0.8]],"values":[["P",27712.8129211,"W"],["Q",20784.6096908,"var"],["S",34641.0161514,"VA"],["V_phase",230.940107676,"V"],["I_phase",50.0,"A"]],"target":"P","phrase":"real power (three-phase total)","assumptions":["balanced load","0 < pf <= 1"],"name":"Balanced three-phase power from line quantities","equation":"S = sqrt(3) V_L I_L, P = S pf, Q = S sin(acos pf); star: V_ph = V_L/sqrt(3), I_ph = I_L; delta: V_ph = V_L, I_ph = I_L/sqrt(3)"}
{"outcome":"SOLVED","message":"","value":69.3399621499,"unit":"","solved_for":"elevation_deg","si":[["latitude",40.0],["longitude",-105.0],["clock",13.0],["utc_offset",-7.0]],"values":[["declination_deg",23.4520460745,""],["eot_min",-1.34372473169,""],["hour_angle_deg",14.6640688171,""],["zenith_deg",20.6600378501,""],["elevation_deg",69.3399621499,""],["azimuth_deg",221.165264952,""]],"target":"elevation_deg","phrase":"declination in degrees","assumptions":["1 <= n <= 366","0 <= clock < 24","-90 < latitude < 90"],"name":"Sun position at a place and local time","equation":"Spencer declination and equation of time; omega = 15 (t - UTC offset - 12) + longitude + E/4; cos z = cos(delta) cos(phi) cos(omega) + sin(delta) sin(phi)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `energy-systems info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`energy-systems solve ... --json` prints one answer as one line of JSON. `energy-systems batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | energy-systems batch`. `energy-systems catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["energy-systems", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`energy-systems figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

```shell
$ energy-systems figures energy_three_phase_power "VL=400 V" "IL=50 A" pf=0.8 connection=star --out .
[
  {
    "slug": "energy-three-phase-power-phasor",
    "title": "Three-phase power triangle",
    "caption": "Power triangle of the three-phase load: the model returns P = 27.71 kW, Q = 20.78 kvar and S = 34.64 kVA (phase voltage 230.9 V, phase current 50 A); φ = 36.87° is the angle of the returned P and Q. In SI units: P = 2.771e+04 W. Your inputs: VL = 400 V, IL = 50 A, pf = 0.8, connection = star.",
    "file": ".\\energy-three-phase-power-phasor.svg"
  },
  {
    "slug": "energy-three-phase-power-live-1",
    "title": "Real power (three-phase total) as line-to-line rms voltage varies",
    "caption": "Real power (three-phase total) as VL runs from 100 to 700 V, every other input as you entered it (41 solutions). Marked: your VL = 400 V gives P = 2.771e+04. Your inputs: VL = 400 V, IL = 50 A, pf = 0.8, connection = star.",
    "file": ".\\energy-three-phase-power-live-1.svg"
  },
  {
    "slug": "energy-three-phase-power-live-2",
    "title": "Real power (three-phase total) as line current (rms) varies",
    "caption": "Real power (three-phase total) as IL runs from 12.5 to 87.5 A, every other input as you entered it (29 solutions). Marked: your IL = 50 A gives P = 2.771e+04. Your inputs: VL = 400 V, IL = 50 A, pf = 0.8, connection = star.",
    "file": ".\\energy-three-phase-power-live-2.svg"
  }
]
```

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `energy_systems_ai999x.wasm` (https://www.ai999x.com/compute/rust/energy_systems_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("energy_systems_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `energy-systems info <id>` shows the same inputs and outputs in the terminal.

### Batteries (4)

<a id="energy_battery_pack_sizing"></a>

#### Battery pack series-parallel sizing — `energy_battery_pack_sizing`

The number of cells in series to reach a pack voltage and of strings in parallel to reach a pack energy, with the resulting pack voltage, energy and capacity (fewest cells; nominal values).

$$
N_s = \left\lceil\frac{V_{pack}}{V_{cell}}\right\rceil,\quad N_p = \left\lceil\frac{E_{pack}}{N_s V_{cell} Q_{cell}}\right\rceil
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V_target` | in | required nominal pack voltage | V |
| `E_target` | in | required pack energy (Wh or kWh) | J |
| `V_cell` | in | nominal cell voltage | V |
| `Q_cell` | in | cell capacity (Ah accepted) | C |
| `N_series` | out | cells in series | — |
| `N_parallel` | out | strings in parallel | — |
| `N_cells` | out | total cells | — |
| `V_pack` | out | nominal pack voltage | V |
| `E_pack_kWh` | out | pack energy in kWh | — |
| `Q_pack_Ah` | out | pack capacity in Ah | — |

**Applies when:** nominal voltages and capacities; a target met within 1e-9 counts as met.

```shell
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

**What the worked example means:** This calculation determines the minimum number of identical cells needed to meet the target voltage of 48 V and energy of 10 kWh. Using a nominal cell voltage of 3.2 V and capacity of 360000 C, the algorithm first rounds up the series count to satisfy the voltage requirement, yielding 15 cells in series. It then calculates the necessary parallel strings to meet the energy demand with those series cells, resulting in 3 strings. The final configuration provides a nominal pack voltage of 48 V and stores 14.4 of energy, ensuring all targets are met or exceeded with the fewest total cells possible.

<a id="energy_battery_rc_discharge"></a>

#### Battery terminal voltage under constant current (one-RC equivalent circuit) — `energy_battery_rc_discharge`

Terminal voltage, polarization (RC) voltage, energy delivered and heat generated after a constant-current discharge of duration t, for a cell modelled as an open-circuit voltage with a series resistance R0 and one R1 || C1 branch (constant OCV).

$$
V_t(t) = V_{oc} - IR_0 - IR_1\left(1 - e^{-t/R_1C_1}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `OCV` | in | open-circuit voltage (held constant) | V |
| `R0` | in | series (ohmic) resistance | ohm |
| `R1` | in | polarization resistance | ohm |
| `C1` | in | polarization capacitance | F |
| `I` | in | discharge current | A |
| `t` | in | time since the current step | s |
| `V_terminal` | out | terminal voltage | V |
| `V_rc` | out | voltage across the RC branch | V |
| `E_delivered` | out | energy delivered to the load | J |
| `E_Wh` | out | energy delivered in Wh | — |
| `heat` | out | heat generated in R0 and R1 | J |
| `tau` | out | time constant R1 C1 | s |

**Applies when:** constant open-circuit voltage over the interval (short pulse or flat OCV curve); RC branch initially relaxed.

```shell
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

**What the worked example means:** This calculation determines the battery's terminal voltage after a constant current discharge of duration 0 s. It starts with the open circuit voltage 4 V and subtracts the immediate ohmic drop caused by series resistance 10 mohm as well as the gradual polarization voltage 0 V from the RC branch. The resulting terminal voltage is 3.9 V. During this process, the system delivered energy of 0 J to the load while generating 0 J of internal heat due to resistive losses.

<a id="energy_peukert"></a>

#### Peukert exponent and run time at a new discharge current — `energy_peukert`

Peukert's law t = C_p/I^k fitted (least squares on the logarithms) to two or more constant-current discharge tests, then the run time and delivered capacity at another current.

$$
t = \frac{C_p}{I^k}\;\Rightarrow\; \ln t = \ln C_p - k\ln I
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `currents` | in | test discharge currents in A | — |
| `times` | in | run time of each test in hours | — |
| `I` | in | new discharge current | A |
| `k` | out | Peukert exponent | — |
| `C_p` | out | Peukert capacity C_p in A^k h | — |
| `runtime_h` | out | run time at I in hours | — |
| `capacity_Ah` | out | capacity delivered at I in Ah | — |

**Applies when:** at least two tests at different currents; constant temperature and cut-off voltage.

```shell
$ energy-systems solve energy_peukert "currents=[1, 10]" "times=[100, 10]" "I=5 A"
k = 1
C_p = 100
runtime_h = 20
capacity_Ah = 100
  by: Peukert exponent and run time at a new discharge current  (ln t = ln C_p - k ln I fitted by least squares; t(I) = C_p I^-k)
  I = 5 A
  assumes: at least two tests at different currents; constant temperature and cut-off voltage
```

**What the worked example means:** This calculation determines how long a battery will last under a new discharge condition by applying Peukert's law. Using least squares regression on logarithmic data from previous tests gives the Peukert capacity 100 and the exponent 1. These parameters predict that at the specific current 5 A, the run time will be 20 hours, delivering a total capacity of 100 amp-hours.

<a id="energy_coulomb_counting"></a>

#### State of charge by coulomb counting — `energy_coulomb_counting`

The state of charge of a battery after a sequence of constant currents over equal time steps (positive on discharge), by coulomb counting SOC = SOC0 - integral(I dt)/Q, with its lowest and highest value and the charge delivered.

$$
SOC(t) = SOC_0 - \frac{1}{Q}\int_0^t I\,dt
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SOC0` | in | initial state of charge (fraction or %) | — |
| `Q` | in | capacity (Ah, mAh, C) | C |
| `currents` | in | current in each time step in A (positive on discharge) | — |
| `dt` | in | length of each time step | s |
| `SOC` | out | final state of charge | — |
| `SOC_min` | out | lowest state of charge | — |
| `SOC_max` | out | highest state of charge | — |
| `charge_out` | out | net charge delivered | C |
| `charge_out_Ah` | out | net charge delivered in Ah | — |

**Applies when:** 0 <= SOC0 <= 1; Q, dt > 0; the state of charge stays within 0 to 100%; no coulombic losses.

```shell
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

**What the worked example means:** This calculation determines the battery's state of charge by tracking the current flow over time. Starting from an initial value of 0.9, the system subtracts the total charge removed from the capacity 180000 C. The result shows that the state of charge ended at 0.4, having varied between 0.3 and 0.9 during the interval, while a net amount of 90000 C coulombs was delivered to the load.

### Fuel cells (3)

<a id="energy_stack_electrical_power"></a>

#### Electrical power of a fuel-cell or electrolyser stack — `energy_stack_electrical_power`

The electrical power P = N I V_cell delivered by a fuel-cell stack (or drawn by an electrolyser stack) of N cells in series at a current and an average cell voltage, and the stack voltage.

$$
P = N\,I\,V_{cell}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | stack current | A |
| `N` | in | number of cells in series | — |
| `V_cell` | in | average cell voltage | V |
| `P` | out | electrical power N I V_cell | W |
| `P_kW` | out | electrical power in kW | — |
| `V_stack` | out | stack voltage N V_cell | V |

**Applies when:** cells in series carrying the same current; 0 < V_cell <= 5 V.

```shell
$ energy-systems solve energy_stack_electrical_power "I=200 A" N=100 "V_cell=0.7 V"
P = 14000 W
P_kW = 14
V_stack = 70 V
  by: Electrical power of a fuel-cell or electrolyser stack  (P = N I V_cell, V_stack = N V_cell)
  I = 200 A
  V_cell = 0.7 V
  assumes: cells in series carrying the same current; 0 < V_cell <= 5 V
```

**What the worked example means:** This calculation determines the electrical power generated by a fuel-cell stack or consumed by an electrolyser stack. It uses the number of cells in series, the current flowing through them, and the average voltage per cell. The result shows that the total power is 14000 W, which converts to 14 kilowatts. The overall voltage across the entire stack is calculated as 70 V, derived from multiplying the individual cell voltage by the count of cells.

<a id="energy_fuel_cell_stack"></a>

#### Hydrogen fuel-cell stack: consumption, power and efficiency — `energy_fuel_cell_stack`

Hydrogen and oxygen consumption, water production (Faraday's law), efficiency on the lower and higher heating value and heat released of a hydrogen fuel-cell stack of N cells at a current and cell voltage.

$$
\dot n_{H_2} = \frac{N I}{2F},\quad \eta_{LHV} = \frac{N I V_{cell}}{\dot n_{H_2}\,\Delta h_{LHV}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | stack current | A |
| `N` | in | number of cells in series | — |
| `V_cell` | in | average cell voltage | V |
| `utilization` | in | fuel utilization (fraction or %, default 1) (optional) | — |
| `H2_kg_s` | out | hydrogen consumed | kg/s |
| `H2_g_h` | out | hydrogen consumed in g/h | — |
| `H2_fed_kg_s` | out | hydrogen fed (consumed/utilization) | kg/s |
| `O2_kg_s` | out | oxygen consumed | kg/s |
| `H2O_kg_s` | out | water produced | kg/s |
| `efficiency_LHV` | out | efficiency on the lower heating value | — |
| `efficiency_HHV` | out | efficiency on the higher heating value | — |
| `heat_LHV` | out | heat released (water vapour product) | W |

**Applies when:** hydrogen/oxygen cell, 2 electrons per H2; heating values at 25 degC.

```shell
$ energy-systems solve energy_fuel_cell_stack "I=192970.6642 A" N=1 "V_cell=0.7 V"
H2_kg_s = 0.00201588 kg/s
H2_g_h = 7257.167999
H2_fed_kg_s = 0.00201588 kg/s
O2_kg_s = 0.0159994 kg/s
H2O_kg_s = 0.01801528 kg/s
efficiency_LHV = 0.5585812318
efficiency_HHV = 0.4725933434
heat_LHV = 106746.535 W
  by: Hydrogen fuel-cell stack: consumption, power and efficiency  (n_H2 = N I/(2F); n_O2 = n_H2/2; n_H2O = n_H2; P = N I V; eta = P/(n_H2 LHV))
  I = 192970.6642 A
  V_cell = 0.7 V
  assumes: hydrogen/oxygen cell, 2 electrons per H2; heating values at 25 degC
```

**What the worked example means:** This calculation determines the mass flow rates of reactants and products for a hydrogen fuel-cell stack using Faraday's law. By inputting the stack current, number of cells, and average cell voltage, it computes the hydrogen consumption rate as 0.00201588 kg/s and the corresponding oxygen usage. The electrical power output is compared against the lower heating value of the consumed fuel to yield an efficiency of 0.558581, while also calculating the heat released during operation.

<a id="energy_electrolyser"></a>

#### Water electrolyser: hydrogen production and energy per kilogram — `energy_electrolyser`

Hydrogen and oxygen production, water consumption, specific energy (kWh per kg H2) and efficiency on the higher and lower heating value of a water electrolyser of N cells.

$$
\frac{E}{m_{H_2}} = \frac{2FV_{cell}}{\eta_F M_{H_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | stack current | A |
| `N` | in | number of cells in series | — |
| `V_cell` | in | cell voltage (1.23 to 3 V) | V |
| `faradaic` | in | Faradaic (current) efficiency, fraction or % (default 1) (optional) | — |
| `H2_kg_s` | out | hydrogen produced | kg/s |
| `H2_kg_h` | out | hydrogen produced in kg/h | — |
| `O2_kg_s` | out | oxygen produced | kg/s |
| `H2O_kg_s` | out | water consumed | kg/s |
| `specific_energy_kWh_per_kg` | out | electricity per kg of hydrogen in kWh/kg | — |
| `efficiency_HHV` | out | efficiency on the higher heating value | — |
| `efficiency_LHV` | out | efficiency on the lower heating value | — |

**Applies when:** water electrolysis, 2 electrons per H2; heating values at 25 degC.

```shell
$ energy-systems solve energy_electrolyser "I=192970.6642 A" N=1 "V_cell=2 V"
H2_kg_s = 0.00201588 kg/s
H2_kg_h = 7.257167998
O2_kg_s = 0.0159994 kg/s
H2O_kg_s = 0.01801528 kg/s
specific_energy_kWh_per_kg = 53.18070747
efficiency_HHV = 0.7405944347
efficiency_LHV = 0.6265874685
  by: Water electrolyser: hydrogen production and energy per kilogram  (n_H2 = eta_F N I/(2F); specific energy = N I V/(n_H2 M_H2); eta_HHV = n_H2 HHV/P)
  I = 192970.6642 A
  V_cell = 2 V
  assumes: water electrolysis, 2 electrons per H2; heating values at 25 degC
```

**What the worked example means:** This calculation determines the hydrogen output rate, water consumption, and electrical efficiency of an electrolyser stack based on the applied current and cell voltage. By applying Faraday's law, it computes that the system generates 0.00201588 kg/s of hydrogen per second while consuming 0.0180153 kg/s of water. The resulting specific energy is 53.1807, indicating how much electricity is required for each kilogram of hydrogen produced, which directly influences the overall efficiency values.

### Hydrogen (3)

<a id="energy_hydrogen_tank"></a>

#### Hydrogen stored in a tank at pressure (real gas) — `energy_hydrogen_tank`

Density, compressibility factor, stored mass and energy (lower heating value) of compressed hydrogen in a tank at a pressure and temperature, by the Leachman (2009) equation of state for normal hydrogen, compared with the ideal-gas estimate.

$$
p = \rho R T\left(1 + \delta\,\frac{\partial\alpha^r}{\partial\delta}\right),\quad Z = \frac{p}{\rho R T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | internal tank volume | m^3 |
| `p` | in | gas pressure (absolute) | Pa |
| `T` | in | gas temperature | K |
| `rho` | out | hydrogen density (real gas) | kg/m^3 |
| `Z` | out | compressibility factor | — |
| `mass` | out | stored hydrogen mass | kg |
| `rho_ideal` | out | ideal-gas density p M/(R T) | kg/m^3 |
| `mass_ideal` | out | ideal-gas mass | kg |
| `ideal_over_real` | out | ideal-gas over real density (= Z) | — |
| `energy_LHV_kWh` | out | stored chemical energy (LHV) in kWh | — |

**Applies when:** 1 kPa <= p <= 1500 bar; 200 K <= T <= 700 K; normal hydrogen, uniform state.

```shell
$ energy-systems solve energy_hydrogen_tank "V=1 m^3" "p=700 bar" "T=288.15 K"
rho = 40.17216108 kg/m^3
Z = 1.466169638
mass = 40.17216108 kg
rho_ideal = 58.89920286 kg/m^3
mass_ideal = 58.89920286 kg
ideal_over_real = 1.466169638
energy_LHV_kWh = 1338.631409
  by: Hydrogen stored in a tank at pressure (real gas)  (Leachman 2009 Helmholtz equation: p = rho R T (1 + delta d(alpha_r)/d(delta)), solved for rho; Z = p/(rho R T))
  V = 1 m^3
  p = 70000000 Pa
  T = 288.15 K
  assumes: 1 kPa <= p <= 1500 bar; 200 K <= T <= 700 K; normal hydrogen, uniform state
```

**What the worked example means:** This calculation determines the actual physical state of compressed hydrogen in a tank using the Leachman equation of state. Given the internal volume of 1 m^3, absolute pressure of 700 bar, and temperature of 288.15 K, it computes the real gas density as 40.1722 kg/m^3. This results in a total stored mass of 40.1722 kg and chemical energy (lower heating value) of 1338.63. The compressibility factor Z is found to be 1.46617, indicating that the ideal gas law would predict a density of 58.8992 kg/m^3 and a mass of 58.8992 kg, which is an overestimate by a factor of 1.46617.

<a id="energy_hydrogen_isentropic_compression"></a>

#### Isentropic (adiabatic) compression of hydrogen — `energy_hydrogen_isentropic_compression`

Outlet temperature and work per kilogram of a reversible adiabatic compression of hydrogen from (p1, T1) to p2, from the full Leachman equation of state (real gas, variable heat capacity).

$$
s(p_2, T_2) = s(p_1, T_1),\quad w = h_2 - h_1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p1` | in | inlet pressure | Pa |
| `T1` | in | inlet temperature | K |
| `p2` | in | outlet pressure | Pa |
| `T2` | out | outlet temperature | K |
| `w` | out | compression work per kg (h2 - h1) | J/kg |
| `w_kWh_per_kg` | out | work in kWh/kg | — |

**Applies when:** reversible adiabatic, single stage; outlet temperature below 1000 K.

```shell
$ energy-systems solve energy_hydrogen_isentropic_compression "p1=30 bar" "T1=300 K" "p2=100 bar"
T2 = 423.2935564 K
w = 1833212.524 J/kg
w_kWh_per_kg = 0.5092257011
  by: Isentropic (adiabatic) compression of hydrogen  (s(p2, T2) = s(p1, T1) solved for T2 by bisection; w = h(p2, T2) - h(p1, T1); s and h from the Helmholtz energy (ideal + residual parts))
  p1 = 3000000 Pa
  T1 = 300 K
  p2 = 10000000 Pa
  assumes: reversible adiabatic, single stage; outlet temperature below 1000 K
```

**What the worked example means:** This calculation determines the thermodynamic state after a reversible adiabatic compression of hydrogen. Starting at inlet pressure 30 bar and temperature 300 K, the gas is compressed to outlet pressure 100 bar. By enforcing constant entropy, the method solves for the final temperature, which is found to be 423.294 K. The specific work input required for this process, calculated as the change in enthalpy, is 1.83321e+06 J/kg. This value represents the energy needed per kilogram of hydrogen under ideal conditions.

<a id="energy_hydrogen_isothermal_compression"></a>

#### Minimum (isothermal) work to compress hydrogen — `energy_hydrogen_isothermal_compression`

The reversible isothermal work per kilogram to compress hydrogen from p1 to p2 at temperature T (the integral of v dp, equal to the Gibbs-energy rise) for the real gas (Leachman equation of state) and the ideal gas.

$$
w = \int_{p_1}^{p_2} v\,dp = g(p_2,T) - g(p_1,T),\quad w_{ideal} = \frac{RT}{M}\ln\frac{p_2}{p_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p1` | in | initial pressure | Pa |
| `p2` | in | final pressure | Pa |
| `T` | in | compression temperature | K |
| `w` | out | real-gas isothermal work per kg | J/kg |
| `w_kWh_per_kg` | out | the same in kWh/kg | — |
| `w_ideal` | out | ideal-gas work R T ln(p2/p1)/M | J/kg |
| `real_over_ideal` | out | real over ideal work | — |

**Applies when:** reversible and isothermal; 200 K <= T <= 700 K, pressures up to 1500 bar.

```shell
$ energy-systems solve energy_hydrogen_isothermal_compression "p1=1 bar" "p2=2 bar" "T=300 K"
w = 858386.4204 J/kg
w_kWh_per_kg = 0.2384406723
w_ideal = 857663.0789 J/kg
real_over_ideal = 1.000843387
  by: Minimum (isothermal) work to compress hydrogen  (w = g(p2, T) - g(p1, T), g/(RT) = ln(delta) + alpha_r + delta d(alpha_r)/d(delta) + (terms in T only))
  p1 = 100000 Pa
  p2 = 200000 Pa
  T = 300 K
  assumes: reversible and isothermal; 200 K <= T <= 700 K, pressures up to 1500 bar
```

**What the worked example means:** This calculation determines the theoretical minimum energy required to compress hydrogen gas reversibly and isothermally. Starting at an initial pressure of 1 bar and ending at a final pressure of 2 bar, while maintaining a constant temperature of 300 K, the real-gas work is found to be 858386 J/kg. This value represents the increase in Gibbs free energy per unit mass for the actual hydrogen state, accounting for non-ideal behavior via the Leachman equation of state. For comparison, the ideal-gas approximation yields 857663 J/kg, showing that the real process requires 1.00084 times the ideal work under these conditions.

### Machines (4)

<a id="energy_induction_max_torque"></a>

#### Induction motor breakdown (maximum) torque and starting torque — `energy_induction_max_torque`

The slip at maximum torque, the maximum (breakdown) torque, the starting torque and current, and the Thevenin equivalent of the stator and magnetizing branch, for a three-phase induction motor.

$$
s_{max} = \frac{R_2}{\sqrt{R_{th}^2 + (X_{th} + X_2)^2}},\quad T_{max} = \frac{3V_{th}^2}{2\omega_s\left[R_{th} + \sqrt{R_{th}^2 + (X_{th}+X_2)^2}\right]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VL` | in | line voltage | V |
| `f` | in | supply frequency | Hz |
| `poles` | in | number of poles (even) | — |
| `R1` | in | stator resistance per phase | ohm |
| `X1` | in | stator leakage reactance per phase | ohm |
| `R2` | in | rotor resistance per phase referred to the stator | ohm |
| `X2` | in | rotor leakage reactance referred to the stator | ohm |
| `Xm` | in | magnetizing reactance per phase | ohm |
| `connection` | in | stator connection | — |
| `s_maxT` | out | slip at maximum torque | — |
| `T_max` | out | maximum (pull-out) torque | N*m |
| `T_start` | out | starting torque (s = 1) | N*m |
| `I_start` | out | starting line current | A |
| `V_th` | out | Thevenin voltage per phase | V |
| `R_th` | out | Thevenin resistance | ohm |
| `X_th` | out | Thevenin reactance | ohm |

**Applies when:** constant parameters (no deep-bar or saturation effects).

```shell
$ energy-systems solve energy_induction_max_torque "VL=460 V" "f=60 Hz" poles=4 "R1=0 ohm" "X1=1 ohm" "R2=0.2 ohm" "X2=1 ohm" "Xm=1e+12 ohm" connection=star
s_maxT = 0.1
T_max = 280.6432163 N*m
T_start = 55.57291412 N*m
I_start = 132.1315476 A
V_th = 265.5811238 V
R_th = 0 ohm
X_th = 1 ohm
  by: Induction motor breakdown (maximum) torque and starting torque  (Thevenin: V_th = V jXm/(R1 + j(X1 + Xm)), Z_th = Z1 || jXm; s_max = R2/sqrt(R_th^2 + (X_th + X2)^2); T_max = 3 V_th^2/(2 omega_s (R_th + sqrt(R_th^2 + (X_th + X2)^2))))
  VL = 460 V
  f = 60 Hz
  R1 = 0 ohm
  X1 = 1 ohm
  R2 = 0.2 ohm
  X2 = 1 ohm
  Xm = 1e+12 ohm
  connection = star
  assumes: constant parameters (no deep-bar or saturation effects)
```

In the worked example, VL = 460 V, f = 60 Hz, poles = 4, R1 = 0 ohm, X1 = 1 ohm, R2 = 0.2 ohm, X2 = 1 ohm, Xm = 1e+12 ohm, connection = star go through the equation above and give s_maxT = 0.1, T_max = 280.6432163 N*m, T_start = 55.57291412 N*m, I_start = 132.1315476 A. It applies only when constant parameters (no deep-bar or saturation effects): outside that it says what is wrong instead of returning a number.

<a id="energy_induction_motor"></a>

#### Induction motor performance from its equivalent circuit — `energy_induction_motor`

Line and rotor currents, power factor, air-gap and converted power, induced torque, output and efficiency of a three-phase induction motor at a slip, from its per-phase equivalent circuit.

$$
T_{ind} = \frac{3 I_2^2 R_2/s}{\omega_s},\quad n_s = \frac{120 f}{p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VL` | in | line voltage | V |
| `f` | in | supply frequency | Hz |
| `poles` | in | number of poles (even) | — |
| `R1` | in | stator resistance per phase | ohm |
| `X1` | in | stator leakage reactance per phase | ohm |
| `R2` | in | rotor resistance per phase referred to the stator | ohm |
| `X2` | in | rotor leakage reactance referred to the stator | ohm |
| `Xm` | in | magnetizing reactance per phase | ohm |
| `slip` | in | slip (0 < s <= 1) | — |
| `connection` | in | stator connection | — |
| `P_rot` | in | rotational (friction, windage and core) loss, default 0 (optional) | W |
| `I_line` | out | line current | A |
| `I2` | out | rotor current referred to the stator | A |
| `pf` | out | power factor | — |
| `P_in` | out | input power | W |
| `P_ag` | out | air-gap power | W |
| `P_conv` | out | converted (developed) power | W |
| `T` | out | induced torque | N*m |
| `P_out` | out | output power | W |
| `efficiency_pct` | out | efficiency in percent | — |

**Applies when:** steady state, balanced supply; 0 < slip <= 1.

```shell
$ energy-systems solve energy_induction_motor "VL=460 V" "f=60 Hz" poles=4 "R1=0.641 ohm" "X1=1.106 ohm" "R2=0.332 ohm" "X2=0.464 ohm" "Xm=26.3 ohm" slip=0.022 connection=star
I_line = 18.89194855 A
I2 = 16.17095396 A
pf = 0.8321224896
P_in = 12525.13665 W
P_ag = 11838.80695 W
P_conv = 11578.3532 W
T = 62.80682157 N*m
P_out = 11578.3532 W
efficiency_pct = 92.44093315
  by: Induction motor performance from its equivalent circuit  (Z = R1 + jX1 + jXm || (R2/s + jX2); P_ag = 3 I2^2 R2/s; P_conv = (1 - s) P_ag; T = P_ag/omega_s; n_s = 120 f/p)
  VL = 460 V
  f = 60 Hz
  R1 = 0.641 ohm
  X1 = 1.106 ohm
  R2 = 0.332 ohm
  X2 = 0.464 ohm
  Xm = 26.3 ohm
  slip = 0.022
  connection = star
  assumes: steady state, balanced supply; 0 < slip <= 1
```

**What the worked example means:** This calculation determines the performance of a three-phase induction motor based on its per-phase equivalent circuit. Using inputs such as line voltage, frequency, pole count, and impedance parameters, the model computes key outputs including rotor current, air-gap power, induced torque, and overall efficiency. For instance, the resulting line current is 18.8919 A, which contributes to an input power of 12525.1 W. The converted mechanical power is 11578.4 W, leading to a final shaft output of 11578.4 W. These values allow engineers to assess whether the motor meets the specific load requirements under steady-state conditions.

<a id="energy_synchronous_generator"></a>

#### Synchronous generator internal EMF, power angle and regulation — `energy_synchronous_generator`

The internal (excitation) EMF, power angle and voltage regulation of a star-connected cylindrical-rotor synchronous generator delivering a current at a power factor, from its synchronous reactance and armature resistance.

$$
\mathbf E_A = \mathbf V_\phi + (R_A + jX_S)\mathbf I_A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VL` | in | terminal line voltage | V |
| `IL` | in | armature (line) current | A |
| `pf` | in | power factor | — |
| `Xs` | in | synchronous reactance per phase | ohm |
| `Ra` | in | armature resistance per phase (default 0) (optional) | ohm |
| `pf_type` | in | lagging (default) or leading (optional) | — |
| `E_line` | out | internal EMF (line value) | V |
| `E_phase` | out | internal EMF per phase | V |
| `delta_deg` | out | power (torque) angle in degrees | — |
| `regulation_pct` | out | voltage regulation (E - V)/V in percent | — |
| `P` | out | real power output | W |
| `Q` | out | reactive power output in var | var |

**Applies when:** cylindrical rotor (no saliency), star connection, unsaturated.

```shell
$ energy-systems solve energy_synchronous_generator "VL=173.2050808 V" "IL=10 A" pf=1 "Xs=10 ohm"
E_line = 244.9489743 V
E_phase = 141.4213563 V
delta_deg = 44.99999999
regulation_pct = 41.42135622
P = 3000.000001 W
Q = 0 var
  by: Synchronous generator internal EMF, power angle and regulation  (E = V + (Ra + jXs) I per phase, I at angle -acos(pf))
  VL = 173.2050808 V
  IL = 10 A
  pf = 1
  Xs = 10 ohm
  assumes: cylindrical rotor (no saliency), star connection, unsaturated
```

**What the worked example means:** This calculation determines the synchronous generator's internal excitation voltage, load angle, and voltage regulation. Using a terminal line voltage of 173.205 V, an armature current of 10 A, and a power factor of 1, the model computes the per-phase internal EMF as 141.421 V and the corresponding line value as 244.949 V. The resulting power angle is 45, while the voltage regulation is calculated to be 41.4214. These values indicate the required field strength to maintain terminal voltage under the specified load conditions.

<a id="energy_synchronous_speed"></a>

#### Synchronous speed, rotor speed and slip of an AC machine — `energy_synchronous_speed`

The synchronous speed n_s = 120 f/P of an AC machine and, with the slip, the rotor speed n = n_s (1 - s) (or, with the rotor speed, the slip) and the rotor (slip) frequency.

$$
n_s = \frac{120 f}{P},\quad n = n_s(1 - s)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | supply frequency | Hz |
| `poles` | in | number of poles (even) | — |
| `s` | in | slip (fraction or %, optional) (optional) | — |
| `n_r` | in | rotor speed (rpm, optional) (optional) | rad/s |
| `n_sync_rpm` | out | synchronous speed in rpm | — |
| `omega_sync` | out | synchronous speed in rad/s | rad/s |
| `n_rotor_rpm` | out | rotor speed in rpm | — |
| `slip` | out | slip (n_s - n)/n_s | — |
| `slip_frequency` | out | rotor current frequency s f | Hz |

**Applies when:** f > 0; even number of poles; at most one of slip and rotor speed.

```shell
$ energy-systems solve energy_synchronous_speed "f=50 Hz" poles=4
n_sync_rpm = 1500
omega_sync = 157.0796327 rad/s
  by: Synchronous speed, rotor speed and slip of an AC machine  (n_s = 120 f/P; n = n_s (1 - s); s = (n_s - n)/n_s; f_r = s f)
  f = 50 Hz
  assumes: f > 0; even number of poles; at most one of slip and rotor speed
```

In the worked example, f = 50 Hz, poles = 4 go through the equation above and give n_sync_rpm = 1500, omega_sync = 157.0796327 rad/s. It applies only when f > 0; even number of poles; at most one of slip and rotor speed: outside that it says what is wrong instead of returning a number.

### Photovoltaics (14)

<a id="energy_pvwatts_losses"></a>

#### Combined PV system losses (PVWatts) — `energy_pvwatts_losses`

The total system loss of a PV system from its individual loss percentages, combined multiplicatively as in PVWatts: 1 - prod(1 - L_i); unstated losses take the PVWatts defaults (soiling 2%, shading 3%, snow 0%, mismatch 2%, wiring 2%, connections 0.5%, light-induced degradation 1.5%, nameplate 1%, age 0%, availability 3%).

$$
L = 1 - \prod_i (1 - L_i)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `soiling` | in | soiling loss as a fraction or a percentage (default 2.0%) (optional) | — |
| `shading` | in | shading loss as a fraction or a percentage (default 3.0%) (optional) | — |
| `snow` | in | snow loss as a fraction or a percentage (default 0.0%) (optional) | — |
| `mismatch` | in | mismatch loss as a fraction or a percentage (default 2.0%) (optional) | — |
| `wiring` | in | wiring loss as a fraction or a percentage (default 2.0%) (optional) | — |
| `connections` | in | connections loss as a fraction or a percentage (default 0.5%) (optional) | — |
| `lid` | in | lid loss as a fraction or a percentage (default 1.5%) (optional) | — |
| `nameplate_rating` | in | nameplate rating loss as a fraction or a percentage (default 1.0%) (optional) | — |
| `age` | in | age loss as a fraction or a percentage (default 0.0%) (optional) | — |
| `availability` | in | availability loss as a fraction or a percentage (default 3.0%) (optional) | — |
| `total_loss` | out | total loss as a fraction | — |
| `performance_factor` | out | product of (1 - loss) | — |
| `total_loss_pct` | out | total loss in percent | — |

**Applies when:** each loss between 0 and 100%.

```shell
$ energy-systems solve energy_pvwatts_losses soiling=0 shading=0 snow=0 mismatch=0 wiring=0 connections=0 lid=0 nameplate_rating=0 age=0 availability=0.1
total_loss = 0.1
performance_factor = 0.9
total_loss_pct = 10
  by: Combined PV system losses (PVWatts)  (L = 1 - prod(1 - L_i))
  soiling = 0
  shading = 0
  snow = 0
  mismatch = 0
  wiring = 0
  connections = 0
  lid = 0
  nameplate_rating = 0
  age = 0
  availability = 0.1
  assumes: each loss between 0 and 100%
```

**What the worked example means:** This calculation determines the total energy loss for a photovoltaic system by combining individual loss percentages multiplicatively. Using the provided values for soiling, shading, snow, mismatch, wiring, connections, light-induced degradation, nameplate rating, age, and availability, the model computes a total loss fraction of 0.1 and its percentage equivalent 10. The resulting performance factor, which represents the portion of energy remaining after all losses are applied, is 0.9. This approach ensures that overlapping or sequential losses are not simply added together, but rather compounded to reflect real-world system efficiency.

<a id="energy_iam_ashrae"></a>

#### Incidence angle modifier (ASHRAE) — `energy_iam_ashrae`

The fraction of beam irradiance transmitted into a PV module at an angle of incidence, by the ASHRAE model IAM = 1 - b0 (1/cos(theta) - 1), floored at 0.

$$
IAM = 1 - b_0\left(\frac{1}{\cos\theta} - 1\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `aoi` | in | angle of incidence in degrees | — |
| `b0` | in | ASHRAE coefficient (default 0.05) (optional) | — |
| `iam` | out | incidence angle modifier | — |

**Applies when:** 0 <= aoi <= 180.

```shell
$ energy-systems solve energy_iam_ashrae aoi=60
iam = 0.95
  by: Incidence angle modifier (ASHRAE)  (IAM = max(0, 1 - b0 (sec(theta) - 1)); 0 for theta >= 90)
  aoi = 60
  assumes: 0 <= aoi <= 180
```

In the worked example, aoi = 60 go through the equation above and give iam = 0.95. It applies only when 0 <= aoi <= 180: outside that it says what is wrong instead of returning a number.

<a id="energy_iam_physical"></a>

#### Incidence angle modifier from Fresnel reflection and absorption — `energy_iam_physical`

The incidence angle modifier of a glass cover from Fresnel reflection (both polarizations) and absorption in the glass (De Soto / physical model), normalised to normal incidence.

$$
IAM = \frac{\tfrac12\left[(1-r_s)+(1-r_p)\right]e^{-KL/\cos\theta_r}}{(1-r_0)e^{-KL}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `aoi` | in | angle of incidence in degrees | — |
| `n` | in | refractive index of the glass (default 1.526) (optional) | — |
| `K` | in | glazing extinction coefficient in 1/m (default 4) (optional) | — |
| `L` | in | glazing thickness (default 2 mm) (optional) | m |
| `iam` | out | incidence angle modifier | — |

**Applies when:** 0 <= aoi < 90; 1 < n < 3.

```shell
$ energy-systems solve energy_iam_physical aoi=0
iam = 1
  by: Incidence angle modifier from Fresnel reflection and absorption  (Snell's law, Fresnel reflectances r_s, r_p, transmission tau = (1 - r) exp(-K L/cos theta_r), IAM = (tau_s + tau_p)/2/tau(0))
  aoi = 0
  assumes: 0 <= aoi < 90; 1 < n < 3
```

In the worked example, aoi = 0 go through the equation above and give iam = 1. It applies only when 0 <= aoi < 90; 1 < n < 3: outside that it says what is wrong instead of returning a number.

<a id="energy_pvwatts_inverter"></a>

#### Inverter AC output and efficiency (PVWatts) — `energy_pvwatts_inverter`

AC output and efficiency of an inverter at a DC input by the PVWatts part-load efficiency curve, with clipping at the AC rating eta_nom P_dc0.

$$
\eta = \frac{\eta_{nom}}{\eta_{ref}}\left(-0.0162\zeta - \frac{0.0059}{\zeta} + 0.9858\right),\ \zeta = \frac{P_{dc}}{P_{dc0}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pdc` | in | DC input power | W |
| `pdc0` | in | inverter DC input rating | W |
| `eta_nom` | in | nominal efficiency (default 0.96) (optional) | — |
| `eta_ref` | in | reference efficiency of the curve (default 0.9637) (optional) | — |
| `pac` | out | AC output power | W |
| `efficiency` | out | conversion efficiency P_ac/P_dc | — |
| `clipped` | out | true when the output is limited at the AC rating | — |

**Applies when:** P_dc > 0.

```shell
$ energy-systems solve energy_pvwatts_inverter "pdc=6 kW" "pdc0=5 kW"
pac = 4800 W
efficiency = 0.8
clipped = true
  by: Inverter AC output and efficiency (PVWatts)  (zeta = P_dc/P_dc0; eta = (eta_nom/eta_ref)(-0.0162 zeta - 0.0059/zeta + 0.9858); P_ac = min(eta P_dc, eta_nom P_dc0))
  pdc = 6000 W
  pdc0 = 5000 W
  assumes: P_dc > 0
```

**What the worked example means:** This calculation determines the alternating current output and conversion efficiency for a solar inverter given its direct current input. Using the PVWatts part-load efficiency curve, the system computes the efficiency based on the load factor derived from the DC input of 6 kW relative to the rated capacity of 5 kW. The resulting AC output is 4800 W, which reflects the applied efficiency of 0.8. If this value reaches the maximum limit defined by the nominal efficiency and rating, the status indicates clipping as yes.

<a id="energy_pvwatts_dc"></a>

#### PV array DC power with temperature derating (PVWatts) — `energy_pvwatts_dc`

DC power of a PV array from the effective irradiance and cell temperature (PVWatts): P = (G/1000) P_dc0 (1 + gamma (T_c - 25 degC)).

$$
P_{dc} = \frac{G}{1000}P_{dc0}\left[1 + \gamma(T_c - 25^\circ\mathrm{C})\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `G_eff` | in | effective (transmitted) irradiance on the cells | W/m^2 |
| `T_cell` | in | cell temperature | K |
| `pdc0` | in | DC rating at 1000 W/m^2 and 25 degC | W |
| `gamma` | in | power temperature coefficient per K (e.g. -0.004, or -0.4%) | — |
| `pdc` | out | DC power | W |
| `pdc_kW` | out | DC power in kW | — |

**Applies when:** -0.02 <= gamma <= 0.01 per K.

```shell
$ energy-systems solve energy_pvwatts_dc "G_eff=800 W/m^2" "T_cell=45 degC" "pdc0=5 kW" gamma=-0.004
pdc = 3680 W
pdc_kW = 3.68
  by: PV array DC power with temperature derating (PVWatts)  (P_dc = (G/1000) P_dc0 [1 + gamma (T_c - 25)])
  G_eff = 800 W/m^2
  T_cell = 318.15 K
  pdc0 = 5000 W
  gamma = -0.004
  assumes: -0.02 <= gamma <= 0.01 per K
```

**What the worked example means:** This calculation determines the direct current power generated by a photovoltaic array. It uses the effective irradiance on the cells, the cell temperature, the rated power at standard conditions, and the power temperature coefficient. The result shows that under these specific environmental conditions, the array produces 3680 W.

<a id="energy_pv_array_output"></a>

#### PV array output at operating temperature — `energy_pv_array_output`

Cell temperature from NOCT and the resulting DC output of an array with a power temperature coefficient, from the plane-of-array irradiance and the air temperature.

$$
P = \frac{G}{1000}P_{dc0}\left[1 + \gamma\left(T_a + \frac{NOCT-20}{800}G - 25\right)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `poa` | in | plane-of-array irradiance | W/m^2 |
| `T_air` | in | ambient air temperature | K |
| `noct` | in | nominal operating cell temperature | K |
| `pdc0` | in | array DC rating at STC | W |
| `gamma` | in | power temperature coefficient per K (e.g. -0.004, or -0.4%) | — |
| `T_cell` | out | cell temperature | K |
| `T_cell_C` | out | cell temperature in degC | — |
| `pdc` | out | DC output | W |
| `derate` | out | temperature derating factor 1 + gamma (T_c - 25) | — |

**Applies when:** G > 0; -0.02 <= gamma <= 0.01 per K.

```shell
$ energy-systems solve energy_pv_array_output "poa=800 W/m^2" "T_air=25 degC" "noct=45 degC" "pdc0=10 kW" gamma=-0.004
T_cell = 323.15 K
T_cell_C = 50
pdc = 7200 W
derate = 0.9
  by: PV array output at operating temperature  (T_c = T_a + (NOCT - 20) G/800; P = (G/1000) P_dc0 [1 + gamma (T_c - 25)])
  poa = 800 W/m^2
  T_air = 298.15 K
  noct = 318.15 K
  pdc0 = 10000 W
  gamma = -0.004
  assumes: G > 0; -0.02 <= gamma <= 0.01 per K
```

**What the worked example means:** This calculation determines the direct current output of a photovoltaic array by first estimating the operating cell temperature using the Nominal Operating Cell Temperature concept. The resulting power, 7200 W, reflects the combined effect of plane-of-array irradiance and thermal derating based on the ambient air temperature and module characteristics.

<a id="energy_cell_temperature_sapm"></a>

#### PV cell and module temperature (Sandia SAPM model) — `energy_cell_temperature_sapm`

Back-of-module and cell temperature by the Sandia Array Performance Model, T_m = G exp(a + b v) + T_a, T_c = T_m + (G/1000) dT, for the standard module/mounting parameter sets.

$$
T_m = G\,e^{a + b v} + T_a,\quad T_c = T_m + \frac{G}{1000}\Delta T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `poa` | in | plane-of-array irradiance | W/m^2 |
| `T_air` | in | ambient air temperature | K |
| `wind` | in | wind speed at 10 m | m/s |
| `mount` | in | module construction and mounting (Sandia parameter set) | — |
| `T_module_C` | out | back-of-module temperature in degC | — |
| `T_cell_C` | out | cell temperature in degC | — |
| `T_cell` | out | cell temperature | K |

**Applies when:** G >= 0.

```shell
$ energy-systems solve energy_cell_temperature_sapm "poa=1000 W/m^2" "T_air=10 degC" "wind=0 m/s" mount=open_rack_glass_glass
T_module_C = 41.11703066
T_cell_C = 44.11703066
T_cell = 317.2670307 K
  by: PV cell and module temperature (Sandia SAPM model)  (T_m = G exp(a + b v) + T_a; T_c = T_m + G dT/1000 (a, b, dT from the Sandia table))
  poa = 1000 W/m^2
  T_air = 283.15 K
  wind = 0 m/s
  mount = open_rack_glass_glass
  assumes: G >= 0
```

**What the worked example means:** This calculation determines the thermal state of a photovoltaic module using the Sandia Array Performance Model. It takes the plane-of-array irradiance, which is 1000 W/m^2, along with the ambient air temperature of 10 degC and wind speed at 0 m/s, to compute the back-of-module temperature as 41.117. Using specific parameter sets for the chosen mounting configuration, it then derives the internal cell temperature, resulting in 44.117 or 317.267 K in Kelvin. These values indicate how hot the solar cells are operating, which directly affects their voltage output and overall efficiency.

<a id="energy_cell_temperature_noct"></a>

#### PV cell temperature from NOCT — `energy_cell_temperature_noct`

The operating temperature of a PV cell from the plane-of-array irradiance, the air temperature and the module's nominal operating cell temperature (NOCT): T_c = T_a + (NOCT - 20 degC) G/800.

$$
T_c = T_a + \frac{NOCT - 20^\circ\mathrm{C}}{800\ \mathrm{W/m^2}}\,G
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `poa` | in | plane-of-array irradiance | W/m^2 |
| `T_air` | in | ambient air temperature | K |
| `noct` | in | nominal operating cell temperature (at 800 W/m^2, 20 degC, 1 m/s) | K |
| `T_cell` | out | cell temperature | K |
| `T_cell_C` | out | cell temperature in degC | — |

**Applies when:** irradiance >= 0.

```shell
$ energy-systems solve energy_cell_temperature_noct "poa=800 W/m^2" "T_air=25 degC" "noct=45 degC"
T_cell = 323.15 K
T_cell_C = 50
  by: PV cell temperature from NOCT  (T_c = T_a + (NOCT - 20)/800 G (Ross model with the NOCT coefficient))
  poa = 800 W/m^2
  T_air = 298.15 K
  noct = 318.15 K
  assumes: irradiance >= 0
```

**What the worked example means:** This calculation determines the operating temperature of a photovoltaic cell using the Ross model. It starts with the ambient air temperature of 25 degC K and adds a heating term derived from the module's nominal operating cell temperature of 45 degC K and the current plane-of-array irradiance of 800 W/m^2 W/m^2. The resulting cell temperature is 323.15 K K, which equals 50 in degrees Celsius. This value is critical because solar panel efficiency typically drops as the cell gets hotter than the surrounding air.

<a id="energy_pv_module_at_conditions"></a>

#### PV module maximum power at an irradiance and temperature — `energy_pv_module_at_conditions`

Short-circuit current, open-circuit voltage, maximum-power current, voltage and power, and fill factor of a PV module at an irradiance and cell temperature: the De Soto translation of its reference parameters, then the single-diode maximum power point.

$$
P_{mp} = \max_V V\,I(V;\,I_L(G,T), I_0(T), R_s, R_{sh}(G), a(T))
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `G` | in | effective irradiance | W/m^2 |
| `T_cell` | in | cell temperature | K |
| `alpha_sc` | in | short-circuit current temperature coefficient in A/K | — |
| `a_ref` | in | modified ideality factor at reference conditions | V |
| `IL_ref` | in | photocurrent at reference conditions | A |
| `I0_ref` | in | saturation current at reference conditions | A |
| `Rsh_ref` | in | shunt resistance at reference conditions | ohm |
| `Rs` | in | series resistance | ohm |
| `EgRef` | in | band gap at 25 degC in eV (default 1.121, silicon) (optional) | — |
| `dEgdT` | in | band-gap temperature coefficient in 1/K (default -0.0002677) (optional) | — |
| `Isc` | out | short-circuit current | A |
| `Voc` | out | open-circuit voltage | V |
| `Imp` | out | current at maximum power | A |
| `Vmp` | out | voltage at maximum power | V |
| `Pmp` | out | maximum power | W |
| `FF` | out | fill factor | — |

**Applies when:** reference conditions 1000 W/m^2 and 25 degC.

```shell
$ energy-systems solve energy_pv_module_at_conditions "G=1000 W/m^2" "T_cell=25 degC" alpha_sc=0.004 "a_ref=1 V" "IL_ref=5 A" "I0_ref=1e-10 A" "Rsh_ref=1e+15 ohm" "Rs=0 ohm"
Isc = 5 A
Voc = 24.63528884 V
Imp = 4.777983488 A
Vmp = 21.52084741 V
Pmp = 102.8262536 W
FF = 0.8347882927
  by: PV module maximum power at an irradiance and temperature  (De Soto parameter translation, then I_sc, V_oc and the maximum power point of the single-diode equation)
  G = 1000 W/m^2
  T_cell = 298.15 K
  alpha_sc = 0.004
  a_ref = 1 V
  IL_ref = 5 A
  I0_ref = 1e-10 A
  Rsh_ref = 1e+15 ohm
  Rs = 0 ohm
  assumes: reference conditions 1000 W/m^2 and 25 degC
```

**What the worked example means:** This calculation determines the performance of a photovoltaic module under specific environmental conditions. It begins by translating reference parameters to the current irradiance of 1000 W/m^2 and cell temperature of 25 degC using the De Soto method. These translated values feed into a single-diode model to find the short-circuit current of 5 A amperes and open-circuit voltage of 24.6353 V volts. The system then solves for the point where power is maximized, yielding a current at maximum power of 4.77798 A amperes and a voltage at maximum power of 21.5208 V volts. The final result, 102.826 W watts, represents the usable electrical output, while the fill factor of 0.834788 indicates how effectively the module converts its theoretical potential into actual power.

<a id="energy_desoto_parameters"></a>

#### PV module single-diode parameters at operating conditions (De Soto) — `energy_desoto_parameters`

The five single-diode parameters (photocurrent, saturation current, series and shunt resistance, modified ideality factor) of a PV module at an irradiance and cell temperature from its reference-condition values, by the De Soto (2006) model.

$$
I_0 = I_{0,ref}\left(\frac{T}{T_{ref}}\right)^3\exp\left(\frac{E_{g,ref}}{kT_{ref}} - \frac{E_g}{kT}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `G` | in | effective irradiance | W/m^2 |
| `T_cell` | in | cell temperature | K |
| `alpha_sc` | in | short-circuit current temperature coefficient in A/K | — |
| `a_ref` | in | modified ideality factor at reference conditions | V |
| `IL_ref` | in | photocurrent at reference conditions | A |
| `I0_ref` | in | saturation current at reference conditions | A |
| `Rsh_ref` | in | shunt resistance at reference conditions | ohm |
| `Rs` | in | series resistance | ohm |
| `EgRef` | in | band gap at 25 degC in eV (default 1.121, silicon) (optional) | — |
| `dEgdT` | in | band-gap temperature coefficient in 1/K (default -0.0002677) (optional) | — |
| `IL` | out | photocurrent | A |
| `I0` | out | saturation current | A |
| `Rs` | out | series resistance | ohm |
| `Rsh` | out | shunt resistance | ohm |
| `nNsVth` | out | modified ideality factor | V |

**Applies when:** reference conditions 1000 W/m^2 and 25 degC.

```shell
$ energy-systems solve energy_desoto_parameters "G=1000 W/m^2" "T_cell=25 degC" alpha_sc=0.004 "a_ref=1.9 V" "IL_ref=9 A" "I0_ref=1e-10 A" "Rsh_ref=400 ohm" "Rs=0.3 ohm"
IL = 9 A
I0 = 1e-10 A
Rs = 0.3 ohm
Rsh = 400 ohm
nNsVth = 1.9 V
  by: PV module single-diode parameters at operating conditions (De Soto)  (a = a_ref T/T_ref; IL = (G/1000)(IL_ref + alpha_sc (T - T_ref)); I0 = I0_ref (T/T_ref)^3 exp(Eg_ref/(k T_ref) - Eg/(k T)), Eg = Eg_ref (1 + dEg/dT (T - T_ref)); Rsh = Rsh_ref 1000/G)
  G = 1000 W/m^2
  T_cell = 298.15 K
  alpha_sc = 0.004
  a_ref = 1.9 V
  IL_ref = 9 A
  I0_ref = 1e-10 A
  Rsh_ref = 400 ohm
  Rs = 0.3 ohm
  assumes: reference conditions 1000 W/m^2 and 25 degC
```

**What the worked example means:** This calculation determines the five single-diode model parameters for a photovoltaic module at specific operating conditions. Using the effective irradiance of 1000 W/m^2 and cell temperature of 25 degC, it adjusts reference values to produce a photocurrent of 9 A, a saturation current of 1e-10 A, a series resistance of 0.3 ohm, a shunt resistance of 400 ohm, and a modified ideality factor of 1.9 V. These values describe how the module behaves electrically under the given environmental conditions.

<a id="energy_cell_temperature_faiman"></a>

#### PV module temperature (Faiman model) — `energy_cell_temperature_faiman`

Module (cell) temperature by the Faiman heat-loss model used in IEC 61853: T = T_a + G/(u0 + u1 v).

$$
T_m = T_a + \frac{G}{u_0 + u_1 v}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `poa` | in | plane-of-array irradiance | W/m^2 |
| `T_air` | in | ambient air temperature | K |
| `wind` | in | wind speed at module height | m/s |
| `u0` | in | constant heat-loss coefficient in W/(m^2 K) (default 25) (optional) | — |
| `u1` | in | wind heat-loss coefficient in W s/(m^3 K) (default 6.84) (optional) | — |
| `T_cell` | out | module temperature | K |
| `T_cell_C` | out | module temperature in degC | — |

**Applies when:** G >= 0; u0 > 0.

```shell
$ energy-systems solve energy_cell_temperature_faiman "poa=1000 W/m^2" "T_air=25 degC" "wind=0 m/s"
T_cell = 338.15 K
T_cell_C = 65
  by: PV module temperature (Faiman model)  (T = T_a + G/(u0 + u1 v))
  poa = 1000 W/m^2
  T_air = 298.15 K
  wind = 0 m/s
  assumes: G >= 0; u0 > 0
```

**What the worked example means:** The module temperature is 338.15 K Kelvin, or 65 degrees Celsius.

<a id="energy_single_diode_current"></a>

#### PV single-diode model: current at a voltage — `energy_single_diode_current`

The current (and power) of a PV cell or module at a terminal voltage from the five-parameter single-diode equation I = IL - I0 [exp((V + I Rs)/a) - 1] - (V + I Rs)/Rsh, solved implicitly.

$$
I = I_L - I_0\left[e^{(V + IR_s)/a} - 1\right] - \frac{V + IR_s}{R_{sh}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | terminal voltage | V |
| `IL` | in | photocurrent (light-generated current) | A |
| `I0` | in | diode saturation current | A |
| `Rs` | in | series resistance | ohm |
| `Rsh` | in | shunt resistance | ohm |
| `nNsVth` | in | modified ideality factor n Ns k T/q | V |
| `I` | out | terminal current | A |
| `P` | out | power V I | W |

**Applies when:** IL, I0, Rsh, a > 0; Rs >= 0.

```shell
$ energy-systems solve energy_single_diode_current "V=1 V" "IL=5 A" "I0=1e-10 A" "Rs=0 ohm" "Rsh=100 ohm" "nNsVth=1 V"
I = 4.99 A
P = 4.99 W
  by: PV single-diode model: current at a voltage  (the implicit single-diode equation solved for I by bisection to full precision)
  V = 1 V
  IL = 5 A
  I0 = 1e-10 A
  Rs = 0 ohm
  Rsh = 100 ohm
  nNsVth = 1 V
  assumes: IL, I0, Rsh, a > 0; Rs >= 0
```

**What the worked example means:** This calculation determines the operating point of a photovoltaic cell by solving the implicit single-diode equation. Given a terminal voltage of 1 V, the model finds the corresponding current 4.99 A that satisfies the balance between the photocurrent 5 A, diode saturation current 1e-10 A, series resistance 0 ohm, shunt resistance 100 ohm, and thermal voltage factor 1 V. The resulting electrical power delivered to the load is calculated as the product of this voltage and current, yielding 4.99 W. This approach ensures high precision by using bisection to resolve the non-linear relationship where current appears on both sides of the equation.

<a id="energy_single_diode_mpp"></a>

#### PV single-diode model: short-circuit, open-circuit and maximum power point — `energy_single_diode_mpp`

Short-circuit current, open-circuit voltage, the maximum power point (current, voltage, power) and the fill factor of a PV cell or module described by the five-parameter single-diode model.

$$
\frac{dP}{dV} = 0,\quad FF = \frac{I_{mp}V_{mp}}{I_{sc}V_{oc}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `IL` | in | photocurrent (light-generated current) | A |
| `I0` | in | diode saturation current | A |
| `Rs` | in | series resistance | ohm |
| `Rsh` | in | shunt resistance | ohm |
| `nNsVth` | in | modified ideality factor n Ns k T/q | V |
| `Isc` | out | short-circuit current | A |
| `Voc` | out | open-circuit voltage | V |
| `Imp` | out | current at maximum power | A |
| `Vmp` | out | voltage at maximum power | V |
| `Pmp` | out | maximum power | W |
| `FF` | out | fill factor Pmp/(Isc Voc) | — |

**Applies when:** IL, I0, Rsh, a > 0; Rs >= 0.

```shell
$ energy-systems solve energy_single_diode_mpp "IL=5 A" "I0=1e-10 A" "Rs=0 ohm" "Rsh=1e+15 ohm" "nNsVth=1 V"
Isc = 5 A
Voc = 24.63528884 V
Imp = 4.777983488 A
Vmp = 21.52084741 V
Pmp = 102.8262536 W
FF = 0.8347882927
  by: PV single-diode model: short-circuit, open-circuit and maximum power point  (I(0) and V(I = 0) from the implicit equation; maximum power where dP/dV_d = 0 along the curve parametrized by the diode voltage (bisection))
  IL = 5 A
  I0 = 1e-10 A
  Rs = 0 ohm
  Rsh = 1e+15 ohm
  nNsVth = 1 V
  assumes: IL, I0, Rsh, a > 0; Rs >= 0
```

**What the worked example means:** This calculation determines the key operating characteristics of a photovoltaic cell using the single-diode model. It computes the short-circuit current, which is 5 A, and the open-circuit voltage, which is 24.6353 V. By finding where the derivative of power with respect to voltage equals zero, it identifies the current at maximum power as 4.77798 A and the voltage at maximum power as 21.5208 V, resulting in a maximum power output of 102.826 W. The efficiency of this extraction relative to the ideal square product of current and voltage is quantified by the fill factor, calculated here as 0.834788.

<a id="energy_single_diode_voltage"></a>

#### PV single-diode model: voltage at a current — `energy_single_diode_voltage`

The terminal voltage (and power) of a PV cell or module carrying a current, from the five-parameter single-diode equation solved implicitly for V; I = 0 gives the open-circuit voltage.

$$
I = I_L - I_0\left[e^{(V + IR_s)/a} - 1\right] - \frac{V + IR_s}{R_{sh}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | terminal current | A |
| `IL` | in | photocurrent (light-generated current) | A |
| `I0` | in | diode saturation current | A |
| `Rs` | in | series resistance | ohm |
| `Rsh` | in | shunt resistance | ohm |
| `nNsVth` | in | modified ideality factor n Ns k T/q | V |
| `V` | out | terminal voltage | V |
| `P` | out | power V I | W |

**Applies when:** IL, I0, Rsh, a > 0; Rs >= 0.

```shell
$ energy-systems solve energy_single_diode_voltage "I=0 A" "IL=5 A" "I0=1e-10 A" "Rs=0 ohm" "Rsh=1e+15 ohm" "nNsVth=1 V"
V = 24.63528884 V
P = 0 W
  by: PV single-diode model: voltage at a current  (the implicit single-diode equation solved for V by bisection to full precision)
  I = 0 A
  IL = 5 A
  I0 = 1e-10 A
  Rs = 0 ohm
  Rsh = 1e+15 ohm
  nNsVth = 1 V
  assumes: IL, I0, Rsh, a > 0; Rs >= 0
```

**What the worked example means:** This calculation determines the operating point of a photovoltaic device by solving the implicit single-diode equation for voltage given a specific current. Using bisection to ensure full precision, the method balances the photocurrent against the diode and shunt currents to find the unique terminal voltage that satisfies the circuit laws. The resulting voltage of 24.6353 V yields a power output of 0 W, representing the electrical energy available at this constrained current level.

### Power systems (18)

<a id="energy_newton_load_flow"></a>

#### AC load flow by Newton-Raphson — `energy_newton_load_flow`

Bus voltage magnitudes and angles, slack-bus power, PV-bus reactive power and losses of a small network by the Newton-Raphson load flow (per unit; pi-model lines; bus 1 is the slack, generator buses hold their voltage; reactive limits are not enforced).

$$
\begin{bmatrix}\Delta\mathbf P\\ \Delta\mathbf Q\end{bmatrix} = \begin{bmatrix}\mathbf H & \mathbf N\\ \mathbf M & \mathbf L\end{bmatrix}\begin{bmatrix}\Delta\boldsymbol\theta\\ \Delta|\mathbf V|\end{bmatrix}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lines` | in | one row per line: from bus, to bus, r, x, total shunt b (per unit) | — |
| `loads` | in | one row per load: bus, P, Q (per unit) | — |
| `gens` | in | optional rows per voltage-controlled (PV) generator: bus, P, V (per unit) (optional) | — |
| `V1` | in | slack-bus voltage magnitude in per unit (default 1.0) (optional) | — |
| `V` | out | voltage magnitudes in per unit, buses 1..n | — |
| `angle_deg` | out | voltage angles in degrees, buses 1..n | — |
| `P_slack` | out | slack real power injection (pu) | — |
| `Q_slack` | out | slack reactive power injection (pu) | — |
| `P_loss` | out | total real power loss (pu) | — |
| `Q_gen` | out | reactive power of each PV generator (pu), in the order given | — |

**Applies when:** buses numbered 1..n, bus 1 the slack; the load flow converges from a flat start.

```shell
$ energy-systems solve energy_newton_load_flow "lines=1 2 0.0 0.5 0.0" "loads=2 0.0 0.0"
V = [1.0,1.0]
angle_deg = [0.0,0.0]
P_slack = 0
Q_slack = 0
P_loss = 0
Q_gen = []
  by: AC load flow by Newton-Raphson  (polar Newton-Raphson on the P and Q mismatches with the analytic Jacobian, solved by Gaussian elimination, to a mismatch below 1e-13 pu)
  assumes: buses numbered 1..n, bus 1 the slack; the load flow converges from a flat start
```

**What the worked example means:** This calculation determines the steady-state voltages and power flows in an electrical network using the Newton-Raphson method. It takes line parameters, load demands, and generator settings as inputs to solve for bus voltage magnitudes given by [1, 1] and phase angles provided by [0, 0]. The resulting slack bus real power injection is 0 and reactive power is 0, while total system losses are calculated as 0. Generator reactive outputs are listed in [], reflecting the balance between generation and demand under the assumed flat start convergence.

<a id="energy_per_unit_impedance"></a>

#### Base impedance and per-unit impedance — `energy_per_unit_impedance`

The base impedance Z_base = V_base^2/S_base and base current of a three-phase per-unit system, and an impedance in ohms expressed in per unit.

$$
Z_{base} = \frac{V_{base}^2}{S_{base}},\quad Z_{pu} = \frac{Z}{Z_{base}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | resistance per phase (ohm) | ohm |
| `X` | in | reactance per phase (ohm) | ohm |
| `S_base` | in | three-phase base power (VA) | VA |
| `V_base` | in | line-to-line base voltage | V |
| `Z_base` | out | base impedance V_base^2/S_base | ohm |
| `I_base` | out | base current S_base/(sqrt(3) V_base) | A |
| `R_pu` | out | per-unit resistance | — |
| `X_pu` | out | per-unit reactance | — |
| `Z_pu` | out | per-unit impedance magnitude | — |

**Applies when:** R, X >= 0, not both 0; positive bases; three-phase base, line-to-line voltage.

```shell
$ energy-systems solve energy_per_unit_impedance "R=5 ohm" "X=20 ohm" "S_base=100 MVA" "V_base=132 kV"
Z_base = 174.24 ohm
I_base = 437.3865676 A
R_pu = 0.02869605142
X_pu = 0.1147842057
Z_pu = 0.1183168511
  by: Base impedance and per-unit impedance  (Z_base = V_base^2/S_base, I_base = S_base/(sqrt(3) V_base), Z_pu = Z/Z_base)
  R = 5 ohm
  X = 20 ohm
  S_base = 100000000 VA
  V_base = 132000 V
  assumes: R, X >= 0, not both 0; positive bases; three-phase base, line-to-line voltage
```

**What the worked example means:** This calculation determines the base impedance and current for a three-phase system using the specified base voltage and power. The actual resistance and reactance are converted into per-unit values by dividing them by the calculated base impedance. This normalization allows for easier comparison across different equipment ratings and voltage levels, resulting in a base impedance of 174.24 ohm ohms and a base current of 437.387 A amperes.

<a id="energy_critical_clearing"></a>

#### Critical clearing angle and time (equal-area criterion) — `energy_critical_clearing`

The initial rotor angle, the largest allowable angle and the critical clearing angle of a machine against an infinite bus by the equal-area criterion, for given pre-fault, during-fault and post-fault transfer limits; with H and f, the critical clearing time for a fault that blocks all transfer.

$$
\cos\delta_{cr} = \frac{P_m(\delta_{max}-\delta_0) + P_{max,3}\cos\delta_{max} - P_{max,2}\cos\delta_0}{P_{max,3} - P_{max,2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Pm` | in | mechanical input power (pu) | — |
| `Pmax_pre` | in | pre-fault transfer limit (pu) | — |
| `Pmax_fault` | in | transfer limit during the fault (pu; 0 for a fault at the terminals) | — |
| `Pmax_post` | in | post-fault transfer limit (pu) | — |
| `H` | in | inertia constant H (MJ/MVA, i.e. seconds), for the clearing time (optional) | s |
| `f` | in | system frequency, for the clearing time (optional) | Hz |
| `delta0_deg` | out | initial rotor angle in degrees | — |
| `delta_max_deg` | out | largest angle before loss of synchronism in degrees | — |
| `delta_cr_deg` | out | critical clearing angle in degrees | — |
| `t_cr` | out | critical clearing time (Pmax_fault = 0) | s |

**Applies when:** Pmax_fault < Pm < Pmax_pre, Pmax_post; classical model (constant voltage behind transient reactance, no damping).

```shell
$ energy-systems solve energy_critical_clearing Pm=1 Pmax_pre=2 Pmax_fault=0 Pmax_post=2
delta0_deg = 30
delta_max_deg = 150
delta_cr_deg = 79.56195848
  by: Critical clearing angle and time (equal-area criterion)  (cos(delta_cr) = [Pm (delta_max - delta0) + Pmax_post cos(delta_max) - Pmax_fault cos(delta0)]/(Pmax_post - Pmax_fault); t_cr = sqrt(4 H (delta_cr - delta0)/(omega_s Pm)))
  Pm = 1
  Pmax_pre = 2
  Pmax_fault = 0
  Pmax_post = 2
  assumes: Pmax_fault < Pm < Pmax_pre, Pmax_post; classical model (constant voltage behind transient reactance, no damping)
```

**What the worked example means:** The critical clearing angle is 79.562, meaning the fault must be cleared before the rotor reaches this point to maintain stability.

<a id="energy_dc_power_flow"></a>

#### DC power flow — `energy_dc_power_flow`

Bus voltage angles and line flows of a network by the DC (linearised) power flow: lossless lines, flat 1 pu voltages, P_ij = (theta_i - theta_j)/x_ij; bus 1 is the slack (angle 0) and takes the balance.

$$
\mathbf B'\boldsymbol\theta = \mathbf P,\quad P_{ij} = \frac{\theta_i - \theta_j}{x_{ij}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lines` | in | lines as 'i-j x' with the series reactance x in per unit (buses numbered 1..n) | — |
| `P` | in | net injections (generation minus load) of buses 2..n in per unit | — |
| `theta_deg` | out | bus voltage angles in degrees, buses 1..n | — |
| `flows` | out | line flows from the first to the second named bus, per unit, in the order given | — |
| `P_slack` | out | slack (bus 1) injection in per unit | — |

**Applies when:** connected network; positive reactances; bus 1 is the slack.

```shell
$ energy-systems solve energy_dc_power_flow "lines=1-2 0.1, 2-3 0.2" "P=0, -1"
theta_deg = [0.0,-5.72957795131,-17.1887338539]
flows = [1.0,1.0]
P_slack = 1
  by: DC power flow  (B' theta = P (reduced susceptance matrix, slack removed), solved by Gaussian elimination; P_ij = (theta_i - theta_j)/x_ij)
  assumes: connected network; positive reactances; bus 1 is the slack
```

**What the worked example means:** This calculation determines the steady-state operating point of the power network using a linearized model. Solving the system of equations relating net injections to voltage angles shows that the bus angles are [0, -5.72958, -17.1887]. These angles drive the active power flow through each transmission line, calculated as [1, 1]. The difference between total generation and load is balanced by the slack bus, which provides an injection of 1. This method assumes flat voltage magnitudes and ignores losses, providing a quick approximation suitable for planning studies.

<a id="energy_dlg_fault"></a>

#### Double line-to-ground fault (symmetrical components) — `energy_dlg_fault`

Phase, neutral and sequence currents and phase voltages for a fault from phases b and c to ground through Zf: negative and (zero + 3Zf) networks in parallel.

$$
I_1 = \frac{V_f}{Z_1 + \frac{Z_2(Z_0 + 3Z_f)}{Z_2 + Z_0 + 3Z_f}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `X1` | in | positive-sequence Thevenin reactance (pu) | — |
| `X2` | in | negative-sequence reactance (pu) | — |
| `X0` | in | zero-sequence reactance (pu) | — |
| `Rf` | in | fault resistance (pu, default 0) (optional) | — |
| `Xf` | in | fault reactance (pu, default 0) (optional) | — |
| `Vf` | in | prefault voltage (pu, default 1.0) (optional) | — |
| `Ia` | out | \|I_a\| (pu) | — |
| `Ib` | out | \|I_b\| (pu) | — |
| `Ic` | out | \|I_c\| (pu) | — |
| `In` | out | neutral (earth) current \|3 I_0\| (pu) | — |
| `I1` | out | \|I_1\| (pu) | — |
| `I2` | out | \|I_2\| (pu) | — |
| `I0` | out | \|I_0\| (pu) | — |
| `Va` | out | \|V_a\| (pu) | — |
| `Vb` | out | \|V_b\| (pu) | — |
| `Vc` | out | \|V_c\| (pu) | — |

**Applies when:** sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf.

```shell
$ energy-systems solve energy_dlg_fault X1=0.1 X2=0.1 X0=0.1
Ia = 0
Ib = 10
Ic = 10
In = 10
I1 = 6.666666667
I2 = 3.333333333
I0 = 3.333333333
Va = 1
Vb = 0
Vc = 0
  by: Double line-to-ground fault (symmetrical components)  (I1 = Vf/(Z1 + Z2 || (Z0 + 3Zf)); I2 = -I1 (Z0 + 3Zf)/(Z2 + Z0 + 3Zf); I0 = -I1 Z2/(Z2 + Z0 + 3Zf))
  X1 = 0.1
  X2 = 0.1
  X0 = 0.1
  assumes: sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf
```

**What the worked example means:** The computed fault currents show phase A at 0 pu, with the neutral carrying 10 pu.

<a id="energy_economic_dispatch"></a>

#### Economic dispatch with quadratic costs and limits — `energy_economic_dispatch`

The least-cost outputs of thermal units with costs a + b P + c P^2 ($/h) and output limits that together meet a demand (losses neglected): every unit between its limits runs at the same incremental cost lambda.

$$
\frac{dC_i}{dP_i} = b_i + 2c_iP_i = \lambda\ \ (P_{min} < P_i < P_{max}),\quad \sum_i P_i = D
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `costs` | in | one row per unit: a ($/h), b ($/MWh), c ($/MW^2 h), Pmin (MW), Pmax (MW) | — |
| `demand` | in | total demand in MW | — |
| `lambda_` | out | system incremental cost in $/MWh | — |
| `P` | out | unit outputs in MW, in the order given | — |
| `cost` | out | total cost in $/h | — |

**Applies when:** c > 0 for every unit; sum Pmin <= demand <= sum Pmax; at least one unit between its limits.

```shell
$ energy-systems solve energy_economic_dispatch "costs=[[500, 5.3, 0.004, 200, 450], [400, 5.5, 0.006, 150, 350], [200, 5.8, 0.009, 100, 225]]" demand=800
lambda_ = 8.5
P = [400.0,250.0,150.0]
cost = 6682.5
  by: Economic dispatch with quadratic costs and limits  (equal incremental cost: P_i(lambda) = clip((lambda - b_i)/(2 c_i), Pmin_i, Pmax_i), lambda by bisection on sum P_i = D)
  demand = 800
  assumes: c > 0 for every unit; sum Pmin <= demand <= sum Pmax; at least one unit between its limits
```

**What the worked example means:** This calculation determines the least-cost generation schedule for thermal units with quadratic cost curves. By equating the incremental costs of all unconstrained units to a common system value, lambda, the method finds the optimal output for each generator within its minimum and maximum limits. The result shows that meeting the specified demand requires a system incremental cost of 8.5, which leads to specific power outputs for each unit and a total operating cost of 6682.5 per hour.

<a id="energy_droop_sharing"></a>

#### Frequency deviation and load sharing under governor droop — `energy_droop_sharing`

The steady-state frequency drop after a load step and each generator's output change when governors with droop R (per unit on their own rating) share the load, with optional load damping D.

$$
\Delta f = -\frac{\Delta P_L}{\sum_i \frac{S_i}{R_i f_0} + D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | droop of each governor in per unit on its own rating (e.g. 0.05) | — |
| `ratings` | in | rating of each generator in MW | — |
| `dP` | in | load increase in MW (negative for a load decrease) | — |
| `f0` | in | nominal frequency | Hz |
| `D` | in | load damping in MW/Hz (default 0) (optional) | — |
| `df` | out | frequency change in Hz | — |
| `f` | out | new frequency in Hz | — |
| `dP_gen` | out | output change of each generator in MW | — |
| `dP_load_damping` | out | load relief from damping in MW | — |

**Applies when:** primary control only (no secondary/AGC); R, ratings > 0.

```shell
$ energy-systems solve energy_droop_sharing "R=[0.05, 0.05]" "ratings=[500, 500]" dP=100 "f0=50 Hz"
df = -0.25
f = 49.75
dP_gen = [50.0,50.0]
dP_load_damping = 0
  by: Frequency deviation and load sharing under governor droop  (beta = sum(S_i/(R_i f0)) + D; df = -dP/beta; dP_i = -df S_i/(R_i f0))
  dP = 100
  f0 = 50 Hz
  assumes: primary control only (no secondary/AGC); R, ratings > 0
```

**What the worked example means:** This calculation determines the steady-state frequency deviation and individual generator power adjustments following a load change. By aggregating the inverse of droop settings scaled by generator ratings and adding load damping, it computes the area frequency response characteristic. The resulting frequency drop is -0.25, leading to a new operating frequency of 49.75. Each generator increases or decreases its output by [50, 50] in proportion to its capability, while load damping contributes 0 to the balance.

<a id="energy_geometric_mean_distance"></a>

#### Geometric mean distance of a three-phase line — `energy_geometric_mean_distance`

The geometric mean distance GMD = (D_ab D_bc D_ca)^(1/3) between the phases of a fully transposed three-phase line.

$$
GMD = \sqrt[3]{D_{ab}D_{bc}D_{ca}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D_ab` | in | distance between phases a and b | m |
| `D_bc` | in | distance between phases b and c | m |
| `D_ca` | in | distance between phases c and a | m |
| `GMD` | out | geometric mean distance | m |
| `ratio_to_mean` | out | GMD divided by the arithmetic mean spacing | — |

**Applies when:** positive spacings that form a triangle (or a straight line); fully transposed line.

```shell
$ energy-systems solve energy_geometric_mean_distance "D_ab=4 m" "D_bc=4 m" "D_ca=8 m"
GMD = 5.0396842 m
ratio_to_mean = 0.9449407874
  by: Geometric mean distance of a three-phase line  (GMD = (D_ab D_bc D_ca)^(1/3))
  D_ab = 4 m
  D_bc = 4 m
  D_ca = 8 m
  assumes: positive spacings that form a triangle (or a straight line); fully transposed line
```

**What the worked example means:** This calculation determines the equivalent spacing for a fully transposed three-phase line by taking the cube root of the product of the distances between phases. Using the input spacings of 4 m, 4 m, and 8 m, the resulting geometric mean distance is 5.03968 m. This value simplifies the computation of inductance and capacitance per phase. The output also includes the ratio of this geometric mean to the arithmetic mean, which is 0.944941, indicating how much the effective spacing deviates from a simple average.

<a id="energy_line_parameters"></a>

#### Inductance, capacitance and surge impedance of a three-phase overhead line — `energy_line_parameters`

Per-phase inductance, capacitance, reactance, susceptance, surge impedance (and surge-impedance loading at a voltage) of a fully transposed three-phase line from its phase spacings, conductor radius and geometric mean radius, with optional bundled conductors.

$$
L = 2\times10^{-7}\ln\frac{GMD}{GMR}\ \mathrm{H/m},\quad C = \frac{2\pi\varepsilon_0}{\ln(GMD/r)}\ \mathrm{F/m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D_ab` | in | distance between phases a and b | m |
| `D_bc` | in | distance between phases b and c | m |
| `D_ca` | in | distance between phases c and a | m |
| `r` | in | conductor (outer) radius | m |
| `f` | in | system frequency | Hz |
| `gmr` | in | conductor geometric mean radius (default r e^(-1/4), a solid round conductor) (optional) | m |
| `n_bundle` | in | subconductors per phase (1 to 4, regular polygon) (optional) | — |
| `spacing` | in | distance between neighbouring subconductors of a bundle (optional) | m |
| `V` | in | line voltage for the surge-impedance loading (optional) (optional) | V |
| `GMR` | out | geometric mean radius of a phase (bundle) | m |
| `L_mH_per_km` | out | inductance per phase in mH/km | — |
| `C_nF_per_km` | out | capacitance to neutral in nF/km | — |
| `X_ohm_per_km` | out | series reactance in ohm/km | — |
| `B_uS_per_km` | out | shunt susceptance in uS/km | — |
| `Zc` | out | surge impedance of the lossless line sqrt(L/C) | ohm |
| `SIL_MW` | out | surge-impedance loading V^2/Zc in MW | — |

**Applies when:** fully transposed line; spacings much larger than the conductors; ground effect neglected.

```shell
$ energy-systems solve energy_line_parameters "D_ab=5 m" "D_bc=5 m" "D_ca=5 m" "r=1 cm" "f=50 Hz" "gmr=5 mm"
GMR = 0.005 m
L_mH_per_km = 1.381551056
C_nF_per_km = 8.951892369
X_ohm_per_km = 0.4340270647
B_uS_per_km = 2.81231993
Zc = 392.8493542 ohm
  by: Inductance, capacitance and surge impedance of a three-phase overhead line  (GMD = (D_ab D_bc D_ca)^(1/3); L = 2e-7 ln(GMD/GMR) H/m; C = 2 pi eps0/ln(GMD/r) F/m; bundles: GMR_b = (GMR d^(n-1))^(1/n) (2^(1/8) (GMR d^3)^(1/4) = 1.09 (GMR d^3)^(1/4) for 4))
  D_ab = 5 m
  D_bc = 5 m
  D_ca = 5 m
  r = 0.01 m
  f = 50 Hz
  gmr = 0.005 m
  assumes: fully transposed line; spacings much larger than the conductors; ground effect neglected
```

**What the worked example means:** This calculation determines the electrical characteristics of a three-phase overhead transmission line based on its physical geometry and conductor properties. It computes the geometric mean radius for bundled conductors if applicable, then uses phase spacings to find the geometric mean distance. From these, it derives the inductance per unit length, which is 1.38155, and the capacitance to neutral, which is 8.95189. These values are further used to calculate the series reactance, shunt susceptance, and the surge impedance of the lossless line, found here as 392.849 ohm. If a voltage was provided, the surge-impedance loading is also computed.

<a id="energy_ll_fault"></a>

#### Line-to-line fault (symmetrical components) — `energy_ll_fault`

Phase and sequence currents and phase voltages for a fault between phases b and c through Zf: positive and negative networks in parallel opposition.

$$
I_b = -I_c = -j\sqrt3\,\frac{V_f}{Z_1 + Z_2 + Z_f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `X1` | in | positive-sequence Thevenin reactance (pu) | — |
| `X2` | in | negative-sequence reactance (pu) | — |
| `X0` | in | zero-sequence reactance (pu) | — |
| `Rf` | in | fault resistance (pu, default 0) (optional) | — |
| `Xf` | in | fault reactance (pu, default 0) (optional) | — |
| `Vf` | in | prefault voltage (pu, default 1.0) (optional) | — |
| `Ia` | out | \|I_a\| (pu) | — |
| `Ib` | out | \|I_b\| (pu) | — |
| `Ic` | out | \|I_c\| (pu) | — |
| `In` | out | neutral (earth) current \|3 I_0\| (pu) | — |
| `I1` | out | \|I_1\| (pu) | — |
| `I2` | out | \|I_2\| (pu) | — |
| `I0` | out | \|I_0\| (pu) | — |
| `Va` | out | \|V_a\| (pu) | — |
| `Vb` | out | \|V_b\| (pu) | — |
| `Vc` | out | \|V_c\| (pu) | — |

**Applies when:** sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf.

```shell
$ energy-systems solve energy_ll_fault X1=0.1 X2=0.1 X0=0.1
Ia = 0
Ib = 8.660254038
Ic = 8.660254038
In = 0
I1 = 5
I2 = 5
I0 = 0
Va = 1
Vb = 0.5
Vc = 0.5
  by: Line-to-line fault (symmetrical components)  (I1 = -I2 = Vf/(Z1 + Z2 + Zf), I0 = 0; |I_b| = sqrt(3) |I_1|)
  X1 = 0.1
  X2 = 0.1
  X0 = 0.1
  assumes: sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf
```

**What the worked example means:** The calculation determines currents and voltages for a fault between phases b and c. By connecting the positive and negative sequence networks in opposition, the model yields a positive-sequence current of 5 and a negative-sequence current of 5. The resulting phase currents are 0, 8.66025, and 8.66025, with the neutral current calculated as 0. This approach assumes no ground path exists, so the zero-sequence network is effectively disconnected.

<a id="energy_long_line"></a>

#### Long transmission line (exact distributed parameters) — `energy_long_line`

Characteristic impedance, propagation constant, exact ABCD constants and the sending-end quantities, regulation and efficiency of a long three-phase line with uniformly distributed R, X and shunt B (the Ferranti rise appears as a negative regulation at light load).

$$
A = \cosh\gamma l,\ B = Z_c\sinh\gamma l,\ C = \frac{\sinh\gamma l}{Z_c},\quad Z_c = \sqrt{\frac{z}{y}},\ \gamma = \sqrt{zy}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vr` | in | receiving-end line-to-line voltage | V |
| `P` | in | load real power (three-phase) | W |
| `pf` | in | load power factor | — |
| `R` | in | line resistance per phase (total) | ohm |
| `X` | in | line reactance per phase (total) | ohm |
| `B` | in | total shunt susceptance of the line per phase | S |
| `pf_type` | in | lagging (default) or leading load (optional) | — |
| `Vs` | out | sending-end line voltage | V |
| `delta_deg` | out | angle of the sending voltage ahead of the receiving voltage, degrees | — |
| `Is` | out | sending-end current | A |
| `Ps` | out | sending-end real power | W |
| `losses` | out | line losses (three-phase) | W |
| `regulation_pct` | out | voltage regulation (V_s/\|A\| - V_r)/V_r in percent | — |
| `efficiency_pct` | out | transmission efficiency in percent | — |
| `A_mag` | out | \|A\| (= \|D\|) | — |
| `A_angle_deg` | out | angle of A in degrees | — |
| `B_mag` | out | \|B\| in ohms | ohm |
| `B_angle_deg` | out | angle of B in degrees | — |
| `Zc_mag` | out | characteristic (surge) impedance magnitude | ohm |
| `Zc_angle_deg` | out | angle of Zc in degrees | — |
| `alpha_l` | out | attenuation alpha l in nepers | — |
| `beta_l` | out | phase shift beta l in radians | — |

**Applies when:** uniformly distributed parameters; balanced load.

```shell
$ energy-systems solve energy_long_line "Vr=400 kV" "P=1 MW" pf=1 "R=0 ohm" "X=100 ohm" "B=400 uS"
Vs = 392026.7098 V
delta_deg = 0.03629508733
Is = 91.77233649 A
Ps = 1000000 W
losses = 0 W
regulation_pct = 2.006413788e-05
efficiency_pct = 100
A_mag = 0.9800665778
A_angle_deg = 0
B_mag = 99.3346654 ohm
B_angle_deg = 90
Zc_mag = 500 ohm
Zc_angle_deg = 0
alpha_l = 0
beta_l = 0.2
  by: Long transmission line (exact distributed parameters)  (gamma l = sqrt(ZY), Zc = sqrt(Z/Y); A = D = cosh(gamma l), B = Zc sinh(gamma l), C = sinh(gamma l)/Zc)
  Vr = 400000 V
  P = 1000000 W
  pf = 1
  R = 0 ohm
  X = 100 ohm
  B = 0.0004 S
  assumes: uniformly distributed parameters; balanced load
```

**What the worked example means:** This calculation determines the precise electrical behavior of a long transmission line by treating resistance, reactance, and susceptance as uniformly distributed rather than lumped. Using the receiving-end voltage 400 kV, load power 1 MW, and line parameters 0 ohm, 100 ohm, and 400 uS, it computes the propagation constant and characteristic impedance to find the ABCD constants. The resulting sending-end voltage is 392027 V, with a phase angle shift of 0.0362951. The calculated voltage regulation is 2.00641e-05 percent, which may be negative under light loads due to the Ferranti effect, while the total line losses are 0 W watts.

<a id="energy_medium_line"></a>

#### Medium transmission line (nominal pi) — `energy_medium_line`

ABCD constants of the nominal-pi model and the sending-end voltage, current and power, losses, regulation and efficiency of a medium-length three-phase line feeding a load.

$$
\begin{bmatrix}V_s\\I_s\end{bmatrix} = \begin{bmatrix}1+\frac{ZY}{2} & Z\\ Y\left(1+\frac{ZY}{4}\right) & 1+\frac{ZY}{2}\end{bmatrix}\begin{bmatrix}V_r\\I_r\end{bmatrix}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vr` | in | receiving-end line-to-line voltage | V |
| `P` | in | load real power (three-phase) | W |
| `pf` | in | load power factor | — |
| `R` | in | line resistance per phase (total) | ohm |
| `X` | in | line reactance per phase (total) | ohm |
| `B` | in | total shunt susceptance of the line per phase | S |
| `pf_type` | in | lagging (default) or leading load (optional) | — |
| `Vs` | out | sending-end line voltage | V |
| `delta_deg` | out | angle of the sending voltage ahead of the receiving voltage, degrees | — |
| `Is` | out | sending-end current | A |
| `Ps` | out | sending-end real power | W |
| `losses` | out | line losses (three-phase) | W |
| `regulation_pct` | out | voltage regulation (V_s/\|A\| - V_r)/V_r in percent | — |
| `efficiency_pct` | out | transmission efficiency in percent | — |
| `A_mag` | out | \|A\| (= \|D\|) | — |
| `A_angle_deg` | out | angle of A in degrees | — |
| `B_mag` | out | \|B\| in ohms | ohm |
| `B_angle_deg` | out | angle of B in degrees | — |

**Applies when:** nominal-pi model (half the shunt admittance at each end); balanced load.

```shell
$ energy-systems solve energy_medium_line "Vr=132 kV" "P=50 MW" pf=1 "R=0 ohm" "X=40 ohm" "B=500 uS"
Vs = 131555.4287 V
delta_deg = 6.613551682
Is = 219.8010835 A
Ps = 50000000 W
losses = 0 W
regulation_pct = 0.6699025601
efficiency_pct = 100
A_mag = 0.99
A_angle_deg = 0
B_mag = 40 ohm
B_angle_deg = 90
  by: Medium transmission line (nominal pi)  (A = D = 1 + ZY/2, B = Z, C = Y(1 + ZY/4); V_s = A V_r + B I_r, I_s = C V_r + D I_r; regulation = (V_s/|A| - V_r)/V_r)
  Vr = 132000 V
  P = 50000000 W
  pf = 1
  R = 0 ohm
  X = 40 ohm
  B = 0.0005 S
  assumes: nominal-pi model (half the shunt admittance at each end); balanced load
```

**What the worked example means:** This calculation determines the performance of a medium-length transmission line using the nominal-pi equivalent circuit. Given the receiving-end voltage 132 kV, load power 50 MW, and line parameters, it computes the ABCD constants to find the sending-end voltage 131555 V, current 219.801 A, and real power 5e+07 W. The result indicates how much the voltage changes along the line, with a regulation of 0.669903 percent and an efficiency of 100 percent.

<a id="energy_per_unit_change_of_base"></a>

#### Per-unit impedance on a new base — `energy_per_unit_change_of_base`

An impedance given in per unit on equipment ratings (S_old, V_old) converted to the per-unit system of the network (S_new, V_new).

$$
Z_{pu,new} = Z_{pu,old}\left(\frac{V_{old}}{V_{new}}\right)^2\frac{S_{new}}{S_{old}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R_pu` | in | per-unit resistance on the old base | — |
| `X_pu` | in | per-unit reactance on the old base | — |
| `S_old` | in | old base (equipment rating) in VA | VA |
| `V_old` | in | old base voltage (line-to-line) | V |
| `S_new` | in | new (system) base power in VA | VA |
| `V_new` | in | new base voltage (line-to-line) | V |
| `R_pu_new` | out | per-unit resistance on the new base | — |
| `X_pu_new` | out | per-unit reactance on the new base | — |
| `Z_pu_new` | out | per-unit impedance magnitude on the new base | — |

**Applies when:** X_pu > 0; positive bases.

```shell
$ energy-systems solve energy_per_unit_change_of_base R_pu=0.01 X_pu=0.1 "S_old=10 MVA" "V_old=11 kV" "S_new=100 MVA" "V_new=11 kV"
R_pu_new = 0.1
X_pu_new = 1
Z_pu_new = 1.004987562
  by: Per-unit impedance on a new base  (Z_new = Z_old (V_old/V_new)^2 (S_new/S_old))
  R_pu = 0.01
  X_pu = 0.1
  S_old = 10000000 VA
  V_old = 11000 V
  S_new = 100000000 VA
  V_new = 11000 V
  assumes: X_pu > 0; positive bases
```

**What the worked example means:** This calculation converts equipment impedance values from their rated base to the system base. Starting with a per-unit resistance of 0.01 and reactance of 0.1, the method scales these by the square of the voltage ratio and the power ratio. The resulting per-unit resistance on the new base is 0.1, while the reactance becomes 1. This standardization allows components with different ratings to be analyzed together within a unified network model.

<a id="energy_power_angle"></a>

#### Power transfer and power angle between two voltages — `energy_power_angle`

The power angle needed to transfer P over a lossless reactance X between a sending voltage V2 (generator) and a receiving voltage V1 (infinite bus), the steady-state limit V1 V2/X, the reactive power at both ends and the stability margin.

$$
P = \frac{V_1V_2}{X}\sin\delta,\quad P_{max} = \frac{V_1V_2}{X}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V1` | in | receiving (infinite-bus) line voltage | V |
| `V2` | in | sending (generator) line voltage | V |
| `X` | in | transfer reactance per phase | ohm |
| `P` | in | real power transferred (three-phase) | W |
| `delta_deg` | out | power angle (V2 ahead of V1) in degrees | — |
| `Pmax` | out | steady-state limit V1 V2/X | W |
| `Q_sent` | out | reactive power sent from the generator end in var | var |
| `Q_received` | out | reactive power arriving at the infinite bus in var | var |
| `margin_pct` | out | stability margin (Pmax - P)/Pmax in percent | — |

**Applies when:** lossless transfer reactance; |P| < V1 V2/X.

```shell
$ energy-systems solve energy_power_angle "V1=100 kV" "V2=100 kV" "X=100 ohm" "P=50 MW"
delta_deg = 30
Pmax = 100000000 W
Q_sent = 13397459.62 var
Q_received = -13397459.62 var
margin_pct = 50
  by: Power transfer and power angle between two voltages  (P = V1 V2 sin(delta)/X; Q_sent = (V2^2 - V1 V2 cos delta)/X; Q_received = (V1 V2 cos delta - V1^2)/X)
  V1 = 100000 V
  V2 = 100000 V
  X = 100 ohm
  P = 50000000 W
  assumes: lossless transfer reactance; |P| < V1 V2/X
```

**What the worked example means:** This calculation determines the operating point for real power transfer between a generator bus and an infinite bus connected by a lossless reactance. Given the sending voltage 100 kV, receiving voltage 100 kV, reactance 100 ohm, and target power 50 MW, it computes the required power angle 30 in degrees. It also finds the maximum possible transfer capability 1e+08 W in watts, the reactive power supplied by the generator 1.33975e+07 var in vars, the reactive power absorbed at the receiving end -1.33975e+07 var in vars, and the percentage of headroom remaining before instability occurs, known as the stability margin 50.

<a id="energy_two_bus_load_flow"></a>

#### Receiving-end voltage of a line feeding a constant-power load (two-bus load flow) — `energy_two_bus_load_flow`

The receiving-end voltage magnitude and angle, current and losses when a three-phase line of impedance R + jX supplies a constant P + jQ load from a fixed sending-end voltage (the exact two-bus load flow).

$$
V_r^4 + \left[2(PR+QX) - V_s^2\right]V_r^2 + (P^2+Q^2)(R^2+X^2) = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vs` | in | sending-end line-to-line voltage (slack) | V |
| `R` | in | line resistance per phase | ohm |
| `X` | in | line reactance per phase | ohm |
| `P` | in | load real power (three-phase) | W |
| `Q` | in | load reactive power in var (negative for a capacitive load) | var |
| `Vr` | out | receiving-end line voltage | V |
| `delta_deg` | out | receiving-end voltage angle in degrees (negative: lagging the source) | — |
| `I` | out | line current | A |
| `P_loss` | out | real power loss | W |
| `Q_loss` | out | reactive power absorbed by the line in var | var |
| `voltage_drop_pct` | out | (V_s - V_r)/V_s in percent | — |

**Applies when:** the load is within the line's transfer limit (a real solution exists); the high-voltage (normal) solution is reported.

```shell
$ energy-systems solve energy_two_bus_load_flow "Vs=2 kV" "R=0 ohm" "X=1 ohm" "P=1 MW" "Q=0 var"
Vr = 1931.851653 V
delta_deg = -15
I = 298.8584907 A
P_loss = 0 W
Q_loss = 267949.1924 var
voltage_drop_pct = 3.407417371
  by: Receiving-end voltage of a line feeding a constant-power load (two-bus load flow)  (V_r^4 + [2(PR + QX) - V_s^2] V_r^2 + (P^2 + Q^2)(R^2 + X^2) = 0, larger root; angle from V_s = V_r + (PR + QX)/V_r + j(PX - QR)/V_r)
  Vs = 2000 V
  R = 0 ohm
  X = 1 ohm
  P = 1000000 W
  Q = 0 var
  assumes: the load is within the line's transfer limit (a real solution exists); the high-voltage (normal) solution is reported
```

In the worked example, Vs = 2 kV, R = 0 ohm, X = 1 ohm, P = 1 MW, Q = 0 var go through the equation above and give Vr = 1931.851653 V, delta_deg = -15, I = 298.8584907 A, P_loss = 0 W. It applies only when the load is within the line's transfer limit (a real solution exists); the high-voltage (normal) solution is reported: outside that it says what is wrong instead of returning a number.

<a id="energy_short_line"></a>

#### Short transmission line: sending-end voltage, regulation and efficiency — `energy_short_line`

Sending-end voltage and its angle, line current, losses, voltage regulation and efficiency of a short three-phase line (series R + jX, no shunt admittance) feeding a load at a known receiving-end voltage.

$$
\mathbf V_s = \mathbf V_r + \mathbf I\,(R + jX),\quad \text{VR} = \frac{|V_s| - |V_r|}{|V_r|}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vr` | in | receiving-end line-to-line voltage | V |
| `P` | in | load real power (three-phase) | W |
| `pf` | in | load power factor | — |
| `R` | in | line resistance per phase (total) | ohm |
| `X` | in | line reactance per phase (total) | ohm |
| `pf_type` | in | lagging (default) or leading load (optional) | — |
| `Vs` | out | sending-end line voltage | V |
| `delta_deg` | out | angle of the sending voltage ahead of the receiving voltage, degrees | — |
| `I` | out | receiving-end line current | A |
| `losses` | out | line losses (three-phase) | W |
| `Ps` | out | sending-end real power | W |
| `regulation_pct` | out | voltage regulation in percent | — |
| `efficiency_pct` | out | transmission efficiency in percent | — |

**Applies when:** short line: shunt admittance neglected; balanced three-phase load.

```shell
$ energy-systems solve energy_short_line "Vr=33 kV" "P=10 MW" pf=1 "R=0 ohm" "X=0 ohm"
Vs = 33000 V
delta_deg = 0
I = 174.954627 A
losses = 0 W
Ps = 10000000 W
regulation_pct = 0
efficiency_pct = 100
  by: Short transmission line: sending-end voltage, regulation and efficiency  (V_s = V_r + I (R + jX) per phase, I = P/(sqrt(3) V_r pf) at angle -acos(pf); regulation = (V_s - V_r)/V_r; efficiency = P/(P + 3 I^2 R))
  Vr = 33000 V
  P = 10000000 W
  pf = 1
  R = 0 ohm
  X = 0 ohm
  assumes: short line: shunt admittance neglected; balanced three-phase load
```

**What the worked example means:** This calculation determines the performance of a short three-phase transmission line modeled as a series impedance. Given a receiving-end line-to-line voltage of 33 kV, a total real power load of 10 MW, and a power factor of 1, the model computes the line current 174.955 A and the resulting sending-end voltage 33000 V. The difference between these voltages yields a voltage regulation of 0, while the comparison of input and output power reveals an efficiency of 100. These metrics indicate how much the voltage drops along the line and how effectively energy is transmitted.

<a id="energy_slg_fault"></a>

#### Single line-to-ground fault (symmetrical components) — `energy_slg_fault`

Phase and sequence currents and phase voltages for a fault from phase a to ground through Zf: the three sequence networks in series.

$$
I_a = 3I_0 = \frac{3V_f}{Z_1 + Z_2 + Z_0 + 3Z_f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `X1` | in | positive-sequence Thevenin reactance (pu) | — |
| `X2` | in | negative-sequence reactance (pu) | — |
| `X0` | in | zero-sequence reactance (pu) | — |
| `Rf` | in | fault resistance (pu, default 0) (optional) | — |
| `Xf` | in | fault reactance (pu, default 0) (optional) | — |
| `Vf` | in | prefault voltage (pu, default 1.0) (optional) | — |
| `Ia` | out | \|I_a\| (pu) | — |
| `Ib` | out | \|I_b\| (pu) | — |
| `Ic` | out | \|I_c\| (pu) | — |
| `In` | out | neutral (earth) current \|3 I_0\| (pu) | — |
| `I1` | out | \|I_1\| (pu) | — |
| `I2` | out | \|I_2\| (pu) | — |
| `I0` | out | \|I_0\| (pu) | — |
| `Va` | out | \|V_a\| (pu) | — |
| `Vb` | out | \|V_b\| (pu) | — |
| `Vc` | out | \|V_c\| (pu) | — |

**Applies when:** sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf.

```shell
$ energy-systems solve energy_slg_fault X1=0.1 X2=0.1 X0=0.1
Ia = 10
Ib = 0
Ic = 0
In = 10
I1 = 3.333333333
I2 = 3.333333333
I0 = 3.333333333
Va = 0
Vb = 1
Vc = 1
  by: Single line-to-ground fault (symmetrical components)  (I1 = I2 = I0 = Vf/(Z1 + Z2 + Z0 + 3 Zf); I_a = 3 I_0)
  X1 = 0.1
  X2 = 0.1
  X0 = 0.1
  assumes: sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf
```

**What the worked example means:** The fault current is 10, with zero-sequence current 3.33333 flowing through the ground path.

<a id="energy_symmetrical_fault"></a>

#### Three-phase (symmetrical) fault current by the bus impedance matrix — `energy_symmetrical_fault`

The fault current, Thevenin reactance and the bus voltages during a balanced three-phase fault at a bus of a reactance network (per unit, prefault no-load voltage Vf everywhere; every branch to node 0 is a source).

$$
I_f = \frac{V_f}{Z_{ff} + Z_f},\quad V_i = V_f - Z_{if}I_f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `network` | in | branches as 'i-j x' with reactances in per unit; node 0 is the reference, each branch to 0 a source (generator or infinite bus) | — |
| `fault_bus` | in | faulted bus | — |
| `Vf` | in | prefault voltage in per unit (default 1.0) (optional) | — |
| `Xf` | in | fault reactance in per unit (default 0, a bolted fault) (optional) | — |
| `S_base` | in | base power, for the fault current in amperes (optional) (optional) | VA |
| `V_base` | in | base line voltage at the faulted bus (optional) (optional) | V |
| `If` | out | fault current in per unit | — |
| `X_th` | out | Thevenin reactance at the faulted bus (Z_ff) in per unit | — |
| `V` | out | bus voltage magnitudes during the fault (pu), buses 1..n | — |
| `If_A` | out | fault current in amperes | A |

**Applies when:** reactance network; no prefault load current (all prefault voltages Vf).

```shell
$ energy-systems solve energy_symmetrical_fault "network=0-1 0.2, 1-2 0.3" fault_bus=2
If = 2
X_th = 0.5
V = [0.6,0.0]
  by: Three-phase (symmetrical) fault current by the bus impedance matrix  (column f of Zbus from Ybus z = e_f; I_f = V_f/(Z_ff + jX_f); V_i = V_f - Z_if I_f)
  assumes: reactance network; no prefault load current (all prefault voltages Vf)
```

**What the worked example means:** This calculation determines the symmetrical three-phase fault current at the specified bus using the bus impedance matrix. The result, 2 per unit, represents the magnitude of the short-circuit current flowing into the fault location. The associated Thevenin equivalent reactance seen from the faulted bus is 0.5 per unit, which characterizes the network's strength against faults.

### Solar resource (18)

<a id="energy_angle_of_incidence"></a>

#### Angle of incidence of the sun on a tilted surface — `energy_angle_of_incidence`

The angle between the sun's rays and the normal of a surface of given tilt and azimuth, and its cosine (the beam projection factor).

$$
\cos\theta = \cos\beta\cos\theta_z + \sin\beta\sin\theta_z\cos(\gamma_s - \gamma)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tilt` | in | surface tilt from horizontal in degrees | — |
| `surface_azimuth` | in | surface azimuth in degrees clockwise from north (south = 180) | — |
| `zenith` | in | solar zenith angle in degrees | — |
| `azimuth` | in | solar azimuth in degrees clockwise from north | — |
| `aoi_deg` | out | angle of incidence in degrees | — |
| `cos_aoi` | out | cosine of the angle of incidence | — |

**Applies when:** 0 <= tilt <= 180; azimuths from 0 to 360 degrees.

```shell
$ energy-systems solve energy_angle_of_incidence tilt=0 surface_azimuth=180 zenith=60 azimuth=100
aoi_deg = 60
cos_aoi = 0.5
  by: Angle of incidence of the sun on a tilted surface  (cos(theta) = cos(beta) cos(z) + sin(beta) sin(z) cos(azimuth_sun - azimuth_surface))
  tilt = 0
  surface_azimuth = 180
  zenith = 60
  azimuth = 100
  assumes: 0 <= tilt <= 180; azimuths from 0 to 360 degrees
```

**What the worked example means:** This calculation determines the angle between the sun's rays and the normal to a tilted surface. Using the surface tilt of 0 degrees and azimuth of 180 degrees, along with the solar zenith of 60 degrees and azimuth of 100 degrees, it finds that the angle of incidence is 60 degrees. The resulting cosine of this angle, 0.5, represents the beam projection factor, indicating how directly sunlight strikes the panel for energy estimation.

<a id="energy_clear_sky_haurwitz"></a>

#### Clear-sky global horizontal irradiance (Haurwitz) — `energy_clear_sky_haurwitz`

The global horizontal irradiance under a clear sky from the solar zenith angle alone (Haurwitz 1945): 1098 cos z exp(-0.059/cos z).

$$
G_{cs} = 1098\cos z\; e^{-0.059/\cos z}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `zenith` | in | apparent solar zenith angle in degrees | — |
| `ghi` | out | clear-sky global horizontal irradiance (0 when the sun is below the horizon) | W/m^2 |

**Applies when:** 0 <= zenith <= 180.

```shell
$ energy-systems solve energy_clear_sky_haurwitz zenith=0
ghi = 1035.092033 W/m^2
  by: Clear-sky global horizontal irradiance (Haurwitz)  (GHI = 1098 cos z exp(-0.059/cos z) for cos z > 0, else 0)
  zenith = 0
  assumes: 0 <= zenith <= 180
```

**What the worked example means:** This calculation determines the theoretical solar power reaching a horizontal surface under clear skies, using only the apparent solar zenith angle. With an input zenith of 0, the resulting clear-sky global horizontal irradiance is computed as 1035.09 W/m^2. This value represents the maximum potential energy available from direct and diffuse sunlight before accounting for clouds or atmospheric variations.

<a id="energy_clear_sky_ineichen"></a>

#### Clear-sky irradiance (Ineichen-Perez with Linke turbidity) — `energy_clear_sky_ineichen`

Clear-sky global horizontal, direct normal and diffuse horizontal irradiance from the apparent solar zenith, the Linke turbidity and the site altitude (Ineichen and Perez 2002), with the pressure-corrected air mass and G_on from day n.

$$
G_{cs} = c_{g1}G_{on}\cos z\,e^{-c_{g2}AM\left(f_{h1} + f_{h2}(T_L - 1)\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `zenith` | in | apparent solar zenith angle in degrees | — |
| `linke_turbidity` | in | Linke turbidity factor (1 to 10; about 2 to 5 for clear skies) | — |
| `altitude` | in | site altitude above sea level | m |
| `n` | in | day of the year (1 = 1 January) | — |
| `ghi` | out | clear-sky global horizontal irradiance | W/m^2 |
| `dni` | out | clear-sky direct normal irradiance | W/m^2 |
| `dhi` | out | clear-sky diffuse horizontal irradiance | W/m^2 |
| `AM_absolute` | out | absolute air mass (Kasten-Young, standard pressure at the altitude) | — |

**Applies when:** sun above the horizon; 1 <= TL <= 10.

```shell
$ energy-systems solve energy_clear_sky_ineichen zenith=30 linke_turbidity=3 "altitude=500 m" n=172
ghi = 885.00309 W/m^2
dni = 910.1415626 W/m^2
dhi = 96.79737577 W/m^2
AM_absolute = 1.087206272
  by: Clear-sky irradiance (Ineichen-Perez with Linke turbidity)  (GHI = cg1 G_on cos z exp(-cg2 AM (fh1 + fh2 (TL - 1))); DNI = min(b G_on exp(-0.09 AM (TL - 1)), GHI (1 - (0.1 - 0.2 e^-TL)/(0.1 + 0.882/fh1))/cos z); fh1 = e^(-h/8000), fh2 = e^(-h/1250))
  zenith = 30
  linke_turbidity = 3
  altitude = 500 m
  assumes: sun above the horizon; 1 <= TL <= 10
```

**What the worked example means:** This calculation determines the theoretical maximum solar radiation reaching a horizontal surface under cloudless conditions. Using the apparent solar zenith angle 30, Linke turbidity factor 3, site altitude 500 m, and day of year 172, it computes the clear-sky global horizontal irradiance as 885.003 W/m^2, direct normal irradiance as 910.142 W/m^2, and diffuse horizontal irradiance as 96.7974 W/m^2. The absolute air mass, calculated via the Kasten-Young formula, is 1.08721. These values represent the upper limit of solar energy available at this location and time, assuming no cloud cover or unusual atmospheric aerosols beyond those captured by the turbidity index.

<a id="energy_erbs_decomposition"></a>

#### Diffuse and direct irradiance from global horizontal irradiance (Erbs) — `energy_erbs_decomposition`

The clearness index, diffuse fraction, diffuse horizontal and direct normal irradiance estimated from a measured global horizontal irradiance by the Erbs (1982) correlation.

$$
\frac{G_d}{G} = 0.9511 - 0.1604k_t + 4.388k_t^2 - 16.638k_t^3 + 12.336k_t^4
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ghi` | in | global horizontal irradiance | W/m^2 |
| `zenith` | in | solar zenith angle in degrees | — |
| `n` | in | day of the year (1 = 1 January) | — |
| `kt` | out | clearness index GHI/(G_on cos z) | — |
| `diffuse_fraction` | out | diffuse fraction DHI/GHI | — |
| `dhi` | out | diffuse horizontal irradiance | W/m^2 |
| `dni` | out | direct normal irradiance | W/m^2 |

**Applies when:** 0 <= zenith <= 85; GHI > 0; kt is capped at 1.

```shell
$ energy-systems solve energy_erbs_decomposition "ghi=141.3981805 W/m^2" zenith=0 n=1
kt = 0.1
diffuse_fraction = 0.991
dhi = 140.1255969 W/m^2
dni = 1.272583624 W/m^2
  by: Diffuse and direct irradiance from global horizontal irradiance (Erbs)  (kt = GHI/(G_on cos z); DHI/GHI = 1 - 0.09 kt (kt <= 0.22), 0.9511 - 0.1604 kt + 4.388 kt^2 - 16.638 kt^3 + 12.336 kt^4 (<= 0.8), 0.165; DNI = (GHI - DHI)/cos z)
  ghi = 141.3981805 W/m^2
  zenith = 0
  assumes: 0 <= zenith <= 85; GHI > 0; kt is capped at 1
```

**What the worked example means:** This calculation decomposes the measured global horizontal irradiance into its diffuse and direct components using the Erbs correlation. By comparing the observed sunlight to the theoretical maximum, it determines the clearness index 0.1. This index drives the estimation of the diffuse fraction 0.991, which allows for the derivation of diffuse horizontal irradiance 140.126 W/m^2 and direct normal irradiance 1.27258 W/m^2. These values are essential for accurately sizing photovoltaic systems and predicting energy yield under varying sky conditions.

<a id="energy_disc_dni"></a>

#### Direct normal irradiance from global horizontal irradiance (DISC model) — `energy_disc_dni`

The direct normal irradiance estimated from global horizontal irradiance by Maxwell's DISC model (clearness index and pressure-corrected air mass), with the clearness index and the air mass used.

$$
G_{bn} = I_0\left[K_{nc}(AM) - \left(a + b\,e^{c\,AM}\right)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ghi` | in | global horizontal irradiance | W/m^2 |
| `zenith` | in | solar zenith angle in degrees | — |
| `n` | in | day of the year (1 = 1 January) | — |
| `pressure` | in | site pressure (default 101325 Pa) (optional) | Pa |
| `dni` | out | direct normal irradiance (0 when the model gives a negative value) | W/m^2 |
| `kt` | out | clearness index (solar constant 1370 W/m^2) | — |
| `airmass` | out | absolute air mass (Kasten 1966, capped at 12) | — |

**Applies when:** 0 <= zenith <= 85; GHI > 0.

```shell
$ energy-systems solve energy_disc_dni "ghi=709.00925 W/m^2" zenith=0 n=1
dni = 163.6448616 W/m^2
kt = 0.5
airmass = 0.9994939326
  by: Direct normal irradiance from global horizontal irradiance (DISC model)  (kt = GHI/(I0 cos z); Kn = Knc(AM) - (a(kt) + b(kt) exp(c(kt) AM)); DNI = Kn I0)
  ghi = 709.00925 W/m^2
  zenith = 0
  assumes: 0 <= zenith <= 85; GHI > 0
```

**What the worked example means:** This calculation determines the direct normal irradiance using the DISC model. It takes the global horizontal irradiance of 709.009 W/m^2 W/m^2 and the solar zenith angle to compute a clearness index of 0.5 and an absolute air mass of 0.999494. These values are used in empirical correlations to estimate that the direct beam component of sunlight is 163.645 W/m^2 W/m^2.

<a id="energy_equation_of_time"></a>

#### Equation of time — `energy_equation_of_time`

The equation of time (apparent minus mean solar time) on day n, by Spencer's Fourier series or the PVCDROM formula 9.87 sin 2B - 7.53 cos B - 1.5 sin B.

$$
E = 229.18\,(0.000075 + 0.001868\cos B - 0.032077\sin B - 0.014615\cos 2B - 0.040849\sin 2B)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | day of the year (1 = 1 January) | — |
| `method` | in | formula: spencer (default) or pvcdrom (B = 360 (n - 81)/365) (optional) | — |
| `eot_min` | out | equation of time in minutes | — |
| `eot_s` | out | equation of time in seconds | — |

**Applies when:** 1 <= n <= 366.

```shell
$ energy-systems solve energy_equation_of_time n=81 method=pvcdrom
eot_min = -7.53
eot_s = -451.8
  by: Equation of time  (Spencer: E = 229.18 (0.000075 + 0.001868 cos B - 0.032077 sin B - 0.014615 cos 2B - 0.040849 sin 2B), B = 2 pi (n - 1)/365)
  method = pvcdrom
  assumes: 1 <= n <= 366
```

**What the worked example means:** This calculation determines the difference between apparent solar time and mean solar time for a specific day of the year. Using the provided day number 81, the algorithm computes the equation of time, resulting in a value of -7.53 minutes or -451.8 seconds. This offset is necessary to convert standard clock time into true solar time, which is critical for accurately determining the sun's position relative to an observer on that date.

<a id="energy_extraterrestrial_irradiance"></a>

#### Extraterrestrial (top-of-atmosphere) normal irradiance — `energy_extraterrestrial_irradiance`

The solar irradiance on a plane normal to the sun's rays outside the atmosphere on day n, and the Earth-Sun distance factor (R0/R)^2.

$$
G_{on} = G_{sc}\left(1 + 0.033\cos\frac{360^\circ n}{365}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | day of the year (1 = 1 January) | — |
| `method` | in | formula: spencer (default, Fourier series) or asce (1 + 0.033 cos(360 n/365)) (optional) | — |
| `solar_constant` | in | solar constant (default 1366.1 W/m^2) (optional) | W/m^2 |
| `G_on` | out | extraterrestrial normal irradiance | W/m^2 |
| `distance_factor` | out | Earth-Sun distance factor (R0/R)^2 | — |

**Applies when:** 1 <= n <= 366.

```shell
$ energy-systems solve energy_extraterrestrial_irradiance n=365 method=asce "solar_constant=1367 W/m^2"
G_on = 1412.111 W/m^2
distance_factor = 1.033
  by: Extraterrestrial (top-of-atmosphere) normal irradiance  (G_on = G_sc (1.00011 + 0.034221 cos B + 0.00128 sin B + 0.000719 cos 2B + 0.000077 sin 2B) (Spencer) or G_sc (1 + 0.033 cos(360 n/365)))
  solar_constant = 1367 W/m^2
  method = asce
  assumes: 1 <= n <= 366
```

**What the worked example means:** This calculation determines the solar power density on a surface perpendicular to the sun's rays at the top of the atmosphere for day 365, yielding an irradiance of 1412.11 W/m^2. The result depends directly on the assumed solar constant of 1367 W/m^2 and the orbital position defined by the day number. Additionally, the Earth-Sun distance factor, which represents the square of the ratio of mean distance to actual distance, is computed as 1.033. This value serves as the theoretical maximum available solar energy before atmospheric attenuation occurs.

<a id="energy_poa_haydavies"></a>

#### Irradiance on a tilted plane (Hay-Davies sky model) — `energy_poa_haydavies`

Beam, sky-diffuse (Hay and Davies 1980: isotropic plus circumsolar weighted by the anisotropy index DNI/G_on) and ground-reflected irradiance on a tilted surface; G_on from day n (Spencer, 1366.1 W/m^2).

$$
G_{d,t} = G_d\left[A R_b + (1 - A)\frac{1+\cos\beta}{2}\right],\ A = \frac{G_{bn}}{G_{on}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tilt` | in | surface tilt from horizontal in degrees | — |
| `surface_azimuth` | in | surface azimuth in degrees clockwise from north (south = 180) | — |
| `zenith` | in | solar zenith angle in degrees | — |
| `azimuth` | in | solar azimuth in degrees clockwise from north | — |
| `dni` | in | direct normal irradiance | W/m^2 |
| `ghi` | in | global horizontal irradiance | W/m^2 |
| `dhi` | in | diffuse horizontal irradiance | W/m^2 |
| `albedo` | in | ground reflectance (0 to 1) | — |
| `n` | in | day of the year (1 = 1 January) | — |
| `poa_global` | out | total irradiance on the plane | W/m^2 |
| `poa_direct` | out | beam irradiance on the plane | W/m^2 |
| `poa_sky_diffuse` | out | sky-diffuse irradiance on the plane | W/m^2 |
| `poa_ground_diffuse` | out | ground-reflected irradiance on the plane | W/m^2 |
| `aoi_deg` | out | angle of incidence in degrees | — |

**Applies when:** sun above the horizon; DHI <= GHI; 0 <= albedo <= 1.

```shell
$ energy-systems solve energy_poa_haydavies tilt=30 surface_azimuth=180 zenith=30 azimuth=180 "dni=800 W/m^2" "ghi=792.820323 W/m^2" "dhi=100 W/m^2" albedo=0.2 n=172
poa_global = 917.3421695 W/m^2
poa_direct = 800 W/m^2
poa_sky_diffuse = 106.7203912 W/m^2
poa_ground_diffuse = 10.62177826 W/m^2
aoi_deg = 0
  by: Irradiance on a tilted plane (Hay-Davies sky model)  (sky = DHI [A R_b + (1 - A)(1 + cos beta)/2], A = DNI/G_on, R_b = cos(theta)/cos(z))
  tilt = 30
  surface_azimuth = 180
  zenith = 30
  azimuth = 180
  dni = 800 W/m^2
  ghi = 792.820323 W/m^2
  dhi = 100 W/m^2
  albedo = 0.2
  assumes: sun above the horizon; DHI <= GHI; 0 <= albedo <= 1
```

**What the worked example means:** This calculation determines the solar energy available on a tilted surface by combining direct beam, diffuse sky, and ground-reflected irradiance. Using the Hay-Davies model, the sky-diffuse component is adjusted for circumsolar radiation based on the ratio of direct normal to extraterrestrial irradiance. The resulting total irradiance is 917.342 W/m^2, with the beam contribution being 800 W/m^2 and the sky-diffuse contribution 106.72 W/m^2.

<a id="energy_poa_isotropic"></a>

#### Irradiance on a tilted plane (isotropic sky) — `energy_poa_isotropic`

Beam, sky-diffuse (isotropic sky, Liu-Jordan/Hottel-Woertz) and ground-reflected irradiance on a tilted surface from DNI, GHI and DHI.

$$
G_{poa} = G_{bn}\cos\theta + G_d\frac{1+\cos\beta}{2} + G\rho\frac{1-\cos\beta}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tilt` | in | surface tilt from horizontal in degrees | — |
| `surface_azimuth` | in | surface azimuth in degrees clockwise from north (south = 180) | — |
| `zenith` | in | solar zenith angle in degrees | — |
| `azimuth` | in | solar azimuth in degrees clockwise from north | — |
| `dni` | in | direct normal irradiance | W/m^2 |
| `ghi` | in | global horizontal irradiance | W/m^2 |
| `dhi` | in | diffuse horizontal irradiance | W/m^2 |
| `albedo` | in | ground reflectance (0 to 1) | — |
| `poa_global` | out | total irradiance on the plane | W/m^2 |
| `poa_direct` | out | beam irradiance on the plane | W/m^2 |
| `poa_sky_diffuse` | out | sky-diffuse irradiance on the plane | W/m^2 |
| `poa_ground_diffuse` | out | ground-reflected irradiance on the plane | W/m^2 |
| `aoi_deg` | out | angle of incidence in degrees | — |

**Applies when:** sun above the horizon; DHI <= GHI; 0 <= albedo <= 1.

```shell
$ energy-systems solve energy_poa_isotropic tilt=30 surface_azimuth=180 zenith=30 azimuth=180 "dni=800 W/m^2" "ghi=792.820323 W/m^2" "dhi=100 W/m^2" albedo=0.2
poa_global = 903.9230485 W/m^2
poa_direct = 800 W/m^2
poa_sky_diffuse = 93.30127019 W/m^2
poa_ground_diffuse = 10.62177826 W/m^2
aoi_deg = 0
  by: Irradiance on a tilted plane (isotropic sky)  (G_poa = DNI cos(theta) + DHI (1 + cos beta)/2 + GHI rho (1 - cos beta)/2)
  tilt = 30
  surface_azimuth = 180
  zenith = 30
  azimuth = 180
  dni = 800 W/m^2
  ghi = 792.820323 W/m^2
  dhi = 100 W/m^2
  albedo = 0.2
  assumes: sun above the horizon; DHI <= GHI; 0 <= albedo <= 1
```

**What the worked example means:** This calculation determines the solar energy received by a surface tilted at 30 degrees with an azimuth of 180 degrees. It combines the direct beam, which contributes 800 W/m^2 based on an angle of incidence of 0 degrees, with isotropic sky diffuse radiation of 93.3013 W/m^2 derived from 100 W/m^2. The result also includes ground-reflected light of 10.6218 W/m^2, calculated using 792.82 W/m^2 and an albedo of 0.2. The sum yields a total plane-of-array irradiance of 903.923 W/m^2.

<a id="energy_poa_klucher"></a>

#### Irradiance on a tilted plane (Klucher sky model) — `energy_poa_klucher`

Beam, sky-diffuse (Klucher 1979: isotropic sky brightened near the horizon and around the sun under clear skies) and ground-reflected irradiance on a tilted surface.

$$
G_{d,t} = G_d\frac{1+\cos\beta}{2}\left(1 + F\sin^3\frac{\beta}{2}\right)\left(1 + F\cos^2\theta\sin^3\theta_z\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tilt` | in | surface tilt from horizontal in degrees | — |
| `surface_azimuth` | in | surface azimuth in degrees clockwise from north (south = 180) | — |
| `zenith` | in | solar zenith angle in degrees | — |
| `azimuth` | in | solar azimuth in degrees clockwise from north | — |
| `dni` | in | direct normal irradiance | W/m^2 |
| `ghi` | in | global horizontal irradiance | W/m^2 |
| `dhi` | in | diffuse horizontal irradiance | W/m^2 |
| `albedo` | in | ground reflectance (0 to 1) | — |
| `poa_global` | out | total irradiance on the plane | W/m^2 |
| `poa_direct` | out | beam irradiance on the plane | W/m^2 |
| `poa_sky_diffuse` | out | sky-diffuse irradiance on the plane | W/m^2 |
| `poa_ground_diffuse` | out | ground-reflected irradiance on the plane | W/m^2 |
| `aoi_deg` | out | angle of incidence in degrees | — |

**Applies when:** sun above the horizon; DHI <= GHI; 0 <= albedo <= 1.

```shell
$ energy-systems solve energy_poa_klucher tilt=30 surface_azimuth=180 zenith=30 azimuth=180 "dni=800 W/m^2" "ghi=792.820323 W/m^2" "dhi=100 W/m^2" albedo=0.2
poa_global = 917.1878664 W/m^2
poa_direct = 800 W/m^2
poa_sky_diffuse = 106.5660881 W/m^2
poa_ground_diffuse = 10.62177826 W/m^2
aoi_deg = 0
  by: Irradiance on a tilted plane (Klucher sky model)  (sky = DHI (1 + cos beta)/2 (1 + F sin^3(beta/2)) (1 + F cos^2(theta) sin^3(z)), F = 1 - (DHI/GHI)^2)
  tilt = 30
  surface_azimuth = 180
  zenith = 30
  azimuth = 180
  dni = 800 W/m^2
  ghi = 792.820323 W/m^2
  dhi = 100 W/m^2
  albedo = 0.2
  assumes: sun above the horizon; DHI <= GHI; 0 <= albedo <= 1
```

**What the worked example means:** This calculation determines the total solar irradiance on a tilted surface using the Klucher model. It sums the beam component, which depends on the angle of incidence, with an adjusted sky-diffuse term that accounts for horizon brightening and circumsolar radiation based on clearness index derived from global and diffuse horizontal irradiances. The result, 917.188 W/m^2, represents the effective energy available to a collector at this orientation, incorporating contributions from the sun's position and ground reflectance.

<a id="energy_poa_perez"></a>

#### Irradiance on a tilted plane (Perez sky model) — `energy_poa_perez`

Beam, sky-diffuse and ground-reflected irradiance on a tilted surface with the Perez (1990) anisotropic sky (circumsolar and horizon brightening coefficients by sky-clearness bin, all-sites composite set), the air mass by Kasten-Young and G_on from day n (Spencer).

$$
G_{d,t} = G_d\left[(1 - F_1)\frac{1+\cos\beta}{2} + F_1\frac{a}{b} + F_2\sin\beta\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tilt` | in | surface tilt from horizontal in degrees | — |
| `surface_azimuth` | in | surface azimuth in degrees clockwise from north (south = 180) | — |
| `zenith` | in | solar zenith angle in degrees | — |
| `azimuth` | in | solar azimuth in degrees clockwise from north | — |
| `dni` | in | direct normal irradiance | W/m^2 |
| `ghi` | in | global horizontal irradiance | W/m^2 |
| `dhi` | in | diffuse horizontal irradiance | W/m^2 |
| `albedo` | in | ground reflectance (0 to 1) | — |
| `n` | in | day of the year (1 = 1 January) | — |
| `poa_global` | out | total irradiance on the plane | W/m^2 |
| `poa_direct` | out | beam irradiance on the plane | W/m^2 |
| `poa_sky_diffuse` | out | sky-diffuse irradiance on the plane | W/m^2 |
| `poa_ground_diffuse` | out | ground-reflected irradiance on the plane | W/m^2 |
| `aoi_deg` | out | angle of incidence in degrees | — |

**Applies when:** 0 <= zenith < 87; DHI > 0; DHI <= GHI.

```shell
$ energy-systems solve energy_poa_perez tilt=30 surface_azimuth=180 zenith=30 azimuth=180 "dni=800 W/m^2" "ghi=792.820323 W/m^2" "dhi=100 W/m^2" albedo=0.2 n=172
poa_global = 923.7780649 W/m^2
poa_direct = 800 W/m^2
poa_sky_diffuse = 113.1562866 W/m^2
poa_ground_diffuse = 10.62177826 W/m^2
aoi_deg = 0
  by: Irradiance on a tilted plane (Perez sky model)  (eps = ((DHI + DNI)/DHI + 1.041 z^3)/(1 + 1.041 z^3), Delta = DHI AM/G_on; F1, F2 from the bin's coefficients; sky = DHI [(1 - F1)(1 + cos beta)/2 + F1 a/b + F2 sin beta])
  tilt = 30
  surface_azimuth = 180
  zenith = 30
  azimuth = 180
  dni = 800 W/m^2
  ghi = 792.820323 W/m^2
  dhi = 100 W/m^2
  albedo = 0.2
  assumes: 0 <= zenith < 87; DHI > 0; DHI <= GHI
```

**What the worked example means:** This calculation determines the total solar energy reaching a tilted surface by summing three distinct contributions. The direct beam component, calculated as 800 W/m^2, depends on the angle of incidence between the sun and the panel. The sky-diffuse component, valued at 113.156 W/m^2, uses the Perez model to account for anisotropic scattering, where brightness varies with proximity to the sun and horizon. Finally, the ground-reflected component, measured as 10.6218 W/m^2, scales with the surface tilt and albedo. Together, these yield a total plane-of-array irradiance of 923.778 W/m^2.

<a id="energy_poa_reindl"></a>

#### Irradiance on a tilted plane (Reindl sky model) — `energy_poa_reindl`

Beam, sky-diffuse (Reindl 1990: Hay-Davies plus horizon brightening sqrt(BHI/GHI) sin^3(beta/2)) and ground-reflected irradiance on a tilted surface; G_on from day n (Spencer).

$$
G_{d,t} = G_d\left[(1-A)\frac{1+\cos\beta}{2}\left(1+\sqrt{\frac{G_b}{G}}\sin^3\frac{\beta}{2}\right) + A R_b\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tilt` | in | surface tilt from horizontal in degrees | — |
| `surface_azimuth` | in | surface azimuth in degrees clockwise from north (south = 180) | — |
| `zenith` | in | solar zenith angle in degrees | — |
| `azimuth` | in | solar azimuth in degrees clockwise from north | — |
| `dni` | in | direct normal irradiance | W/m^2 |
| `ghi` | in | global horizontal irradiance | W/m^2 |
| `dhi` | in | diffuse horizontal irradiance | W/m^2 |
| `albedo` | in | ground reflectance (0 to 1) | — |
| `n` | in | day of the year (1 = 1 January) | — |
| `poa_global` | out | total irradiance on the plane | W/m^2 |
| `poa_direct` | out | beam irradiance on the plane | W/m^2 |
| `poa_sky_diffuse` | out | sky-diffuse irradiance on the plane | W/m^2 |
| `poa_ground_diffuse` | out | ground-reflected irradiance on the plane | W/m^2 |
| `aoi_deg` | out | angle of incidence in degrees | — |

**Applies when:** sun above the horizon; DHI <= GHI; 0 <= albedo <= 1.

```shell
$ energy-systems solve energy_poa_reindl tilt=30 surface_azimuth=180 zenith=30 azimuth=180 "dni=800 W/m^2" "ghi=792.820323 W/m^2" "dhi=100 W/m^2" albedo=0.2 n=172
poa_global = 917.9389967 W/m^2
poa_direct = 800 W/m^2
poa_sky_diffuse = 107.3172184 W/m^2
poa_ground_diffuse = 10.62177826 W/m^2
aoi_deg = 0
  by: Irradiance on a tilted plane (Reindl sky model)  (sky = DHI [(1 - A)(1 + cos beta)/2 (1 + sqrt(BHI/GHI) sin^3(beta/2)) + A R_b])
  tilt = 30
  surface_azimuth = 180
  zenith = 30
  azimuth = 180
  dni = 800 W/m^2
  ghi = 792.820323 W/m^2
  dhi = 100 W/m^2
  albedo = 0.2
  assumes: sun above the horizon; DHI <= GHI; 0 <= albedo <= 1
```

**What the worked example means:** This calculation determines the total solar energy reaching a tilted surface by summing direct beam, sky-diffuse, and ground-reflected components. Using the Reindl model for diffuse radiation, which accounts for horizon brightening, the method combines inputs such as global horizontal irradiance of 792.82 W/m^2 W/m^2 and diffuse horizontal irradiance of 100 W/m^2 W/m^2 with geometric factors derived from the tilt of 30 degrees and solar zenith angle of 30 degrees. The resulting total irradiance on the plane is 917.939 W/m^2 W/m^2, indicating the potential power generation capacity for photovoltaic or thermal collectors at this specific orientation.

<a id="energy_air_mass"></a>

#### Optical air mass — `energy_air_mass`

The relative optical air mass for a solar zenith angle (Kasten-Young by default, or another named model) and the pressure-corrected absolute air mass.

$$
AM = \frac{1}{\cos z + 0.50572\,(96.07995^\circ - z)^{-1.6364}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `zenith` | in | solar zenith angle in degrees (apparent zenith for the refraction-based models) | — |
| `model` | in | air-mass model (default kastenyoung1989) (optional) | — |
| `pressure` | in | site pressure (default 101325 Pa) (optional) | Pa |
| `AM_relative` | out | relative air mass (sea level) | — |
| `AM_absolute` | out | absolute air mass AM_rel p/101325 Pa | — |

**Applies when:** 0 <= zenith < 90.

```shell
$ energy-systems solve energy_air_mass zenith=60 model=simple
AM_relative = 2
AM_absolute = 2
  by: Optical air mass  (Kasten-Young: AM = 1/(cos z + 0.50572 (96.07995 - z)^-1.6364); absolute AM = AM p/p0)
  zenith = 60
  model = simple
  assumes: 0 <= zenith < 90
```

**What the worked example means:** The relative optical air mass is 2 and the pressure-corrected absolute air mass is 2.

<a id="energy_solar_declination"></a>

#### Solar declination on a day of the year — `energy_solar_declination`

The declination of the sun (the latitude where the sun is overhead at noon) on day n of the year, by Cooper's equation or Spencer's Fourier series.

$$
\delta = 23.45^\circ \sin\!\left(\frac{360^\circ (284 + n)}{365}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | day of the year (1 = 1 January) | — |
| `method` | in | formula: cooper (Cooper 1969, the textbook default) or spencer (Spencer 1971) (optional) | — |
| `declination_deg` | out | declination in degrees | — |
| `declination_rad` | out | declination in radians | — |

**Applies when:** 1 <= n <= 366.

```shell
$ energy-systems solve energy_solar_declination n=172
declination_deg = 23.44978285
declination_rad = 0.4092759196
  by: Solar declination on a day of the year  (Cooper: delta = 23.45 sin(360 (284 + n)/365) degrees; Spencer: a Fourier series in the day angle B = 2 pi (n - 1)/365)
  assumes: 1 <= n <= 366
```

**What the worked example means:** This calculation determines the latitude where the sun is directly overhead at solar noon on day 172 of the year. Using the selected method, the resulting solar declination is 23.4498, which is also expressed as 0.409276. This value indicates the seasonal position of the sun relative to the equator, influencing how high the sun appears in the sky and affecting solar energy availability.

<a id="energy_solar_time_hour_angle"></a>

#### Solar time and hour angle from clock time — `energy_solar_time_hour_angle`

Local apparent solar time and the hour angle of the sun from the local clock time, the longitude and the time zone (UTC offset), with the equation of time (Spencer).

$$
\omega = 15^\circ\,(t - \Delta t_{UTC} - 12) + \lambda + \frac{E}{4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | day of the year (1 = 1 January) | — |
| `clock` | in | local clock (standard) time in decimal hours or hh:mm | — |
| `longitude` | in | longitude in degrees, east positive | — |
| `utc_offset` | in | time zone as hours from UTC (e.g. -5, 1, 5.5) | — |
| `eot_min` | out | equation of time in minutes | — |
| `hour_angle_deg` | out | hour angle in degrees (negative before solar noon) | — |
| `solar_time_h` | out | apparent solar time in decimal hours | — |

**Applies when:** 1 <= n <= 366; 0 <= clock < 24; -180 <= longitude <= 180.

```shell
$ energy-systems solve energy_solar_time_hour_angle n=172 clock=14.5 longitude=-75 utc_offset=-5
eot_min = -1.343724732
hour_angle_deg = 37.16406882
solar_time_h = 14.47760459
  by: Solar time and hour angle from clock time  (omega = 15 (t_clock - UTC offset - 12) + longitude + E/4 (degrees); solar time = 12 + omega/15)
  clock = 14.5
  longitude = -75
  utc_offset = -5
  assumes: 1 <= n <= 366; 0 <= clock < 24; -180 <= longitude <= 180
```

**What the worked example means:** This calculation determines the local apparent solar time and the sun's hour angle from your clock reading. Using the day of year 172, it first finds the equation of time, which is -1.34372. Then, combining the clock time 14.5, longitude -75, and UTC offset -5, it computes the hour angle as 37.1641. This results in an apparent solar time of 14.4776, indicating exactly where the sun stands relative to noon.

<a id="energy_sun_angles"></a>

#### Solar zenith, elevation and azimuth from latitude, declination and hour angle — `energy_sun_angles`

The sun's zenith angle, elevation (altitude) and azimuth (clockwise from north) at a latitude for a given declination and hour angle.

$$
\cos\theta_z = \cos\phi\cos\delta\cos\omega + \sin\phi\sin\delta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `latitude` | in | latitude in degrees (north positive) | — |
| `declination` | in | solar declination in degrees | — |
| `hour_angle` | in | hour angle in degrees (negative in the morning) | — |
| `zenith_deg` | out | solar zenith angle in degrees | — |
| `elevation_deg` | out | solar elevation (altitude) in degrees | — |
| `azimuth_deg` | out | solar azimuth in degrees clockwise from north | — |

**Applies when:** -90 < latitude < 90; |declination| <= 23.5; |hour angle| <= 180.

```shell
$ energy-systems solve energy_sun_angles latitude=40 declination=0 hour_angle=0
zenith_deg = 40
elevation_deg = 50
azimuth_deg = 180
  by: Solar zenith, elevation and azimuth from latitude, declination and hour angle  (cos z = cos(delta) cos(phi) cos(omega) + sin(delta) sin(phi); cos(azimuth - 180) = (cos z sin phi - sin delta)/(sin z cos phi), sign of omega)
  latitude = 40
  declination = 0
  hour_angle = 0
  assumes: -90 < latitude < 90; |declination| <= 23.5; |hour angle| <= 180
```

**What the worked example means:** This calculation determines the precise direction of the sun relative to a specific location. Using the provided latitude, solar declination, and hour angle, it computes the zenith angle as 40 degrees, the elevation angle as 50 degrees, and the azimuth angle as 180 degrees measured clockwise from true north. These geometric values define the incident angle of sunlight, which is critical for assessing how much solar energy reaches a surface at that moment.

<a id="energy_solar_position"></a>

#### Sun position at a place and local time — `energy_solar_position`

Declination, equation of time, hour angle, zenith, elevation and azimuth of the sun at a latitude and longitude on day n at a local clock time (Spencer declination and equation of time).

$$
\cos\theta_z = \cos\phi\cos\delta\cos\omega + \sin\phi\sin\delta,\quad \omega = 15^\circ(t_s - 12)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `latitude` | in | latitude in degrees (north positive) | — |
| `longitude` | in | longitude in degrees, east positive | — |
| `n` | in | day of the year (1 = 1 January) | — |
| `clock` | in | local clock (standard) time in decimal hours or hh:mm | — |
| `utc_offset` | in | time zone as hours from UTC | — |
| `declination_deg` | out | declination in degrees | — |
| `eot_min` | out | equation of time in minutes | — |
| `hour_angle_deg` | out | hour angle in degrees | — |
| `zenith_deg` | out | solar zenith angle in degrees | — |
| `elevation_deg` | out | solar elevation in degrees | — |
| `azimuth_deg` | out | solar azimuth in degrees clockwise from north | — |

**Applies when:** 1 <= n <= 366; 0 <= clock < 24; -90 < latitude < 90.

```shell
$ energy-systems solve energy_solar_position latitude=40 longitude=-105 n=172 clock=13 utc_offset=-7
declination_deg = 23.45204607
eot_min = -1.343724732
hour_angle_deg = 14.66406882
zenith_deg = 20.66003785
elevation_deg = 69.33996215
azimuth_deg = 221.165265
  by: Sun position at a place and local time  (Spencer declination and equation of time; omega = 15 (t - UTC offset - 12) + longitude + E/4; cos z = cos(delta) cos(phi) cos(omega) + sin(delta) sin(phi))
  latitude = 40
  longitude = -105
  clock = 13
  utc_offset = -7
  assumes: 1 <= n <= 366; 0 <= clock < 24; -90 < latitude < 90
```

**What the worked example means:** This calculation finds the sun's exact location in the sky using the day number, local time, and geographic coordinates. It computes the solar declination as 23.452 and the equation of time as -1.34372 to adjust for orbital eccentricity. The hour angle is calculated as 14.6641, which leads to a solar zenith angle of 20.66, an elevation of 69.34, and an azimuth of 221.165. These values describe where the sun is relative to the observer at that specific moment.

<a id="energy_sunrise_sunset"></a>

#### Sunrise, sunset, solar noon and day length — `energy_sunrise_sunset`

Geometric sunrise and sunset times (local clock time), solar noon, the day length and the sunset hour angle at a place on day n (no refraction, sun centre on the horizon).

$$
\cos\omega_s = -\tan\phi\tan\delta,\quad N = \frac{2\,\omega_s}{15^\circ}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `latitude` | in | latitude in degrees (north positive) | — |
| `longitude` | in | longitude in degrees, east positive | — |
| `n` | in | day of the year (1 = 1 January) | — |
| `utc_offset` | in | time zone as hours from UTC | — |
| `sunrise_h` | out | sunrise, local clock time in decimal hours | — |
| `sunset_h` | out | sunset, local clock time in decimal hours | — |
| `solar_noon_h` | out | solar noon, local clock time in decimal hours | — |
| `day_length_h` | out | day length in hours | — |
| `sunset_hour_angle_deg` | out | sunset hour angle omega_s in degrees | — |

**Applies when:** the sun rises and sets on that day (no polar day or night).

```shell
$ energy-systems solve energy_sunrise_sunset latitude=0 longitude=0 n=81 utc_offset=0
sunrise_h = 6.126077339
sunset_h = 18.12607734
solar_noon_h = 12.12607734
day_length_h = 12
sunset_hour_angle_deg = 90
  by: Sunrise, sunset, solar noon and day length  (omega_s = acos(-tan(phi) tan(delta)); t = (omega - longitude - E/4)/15 + 12 + UTC offset; day length = 2 omega_s/15)
  latitude = 0
  longitude = 0
  utc_offset = 0
  assumes: the sun rises and sets on that day (no polar day or night)
```

**What the worked example means:** This calculation determines the local clock times for sunrise, sunset, and solar noon based on the site's latitude, longitude, and time zone offset. It also computes the total length of daylight and the sunset hour angle. For example, with a latitude of 0 and day number 81, the sun rises at 6.12608 and sets at 18.1261, resulting in a day length of 12. Solar noon happens at 12.1261, when the sun is highest in the sky.

### Three-phase power (1)

<a id="energy_three_phase_power"></a>

#### Balanced three-phase power from line quantities — `energy_three_phase_power`

Real, reactive and apparent power of a balanced three-phase load from the line voltage, line current and power factor, with the phase voltage and phase current of a star or delta connection.

$$
P = \sqrt3\,V_L I_L\cos\phi,\quad Q = \sqrt3\,V_L I_L\sin\phi
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VL` | in | line-to-line rms voltage | V |
| `IL` | in | line current (rms) | A |
| `pf` | in | power factor (0 < pf <= 1) | — |
| `connection` | in | connection of the load (star = wye) | — |
| `pf_type` | in | lagging (inductive, default) or leading (capacitive) (optional) | — |
| `P` | out | real power (three-phase total) | W |
| `Q` | out | reactive power in var (positive lagging) | var |
| `S` | out | apparent power in VA | VA |
| `V_phase` | out | voltage across each phase of the load | V |
| `I_phase` | out | current in each phase of the load | A |

**Applies when:** balanced load; 0 < pf <= 1.

```shell
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

**What the worked example means:** This calculation determines the total power consumed by a balanced three-phase load using the line voltage, line current, and power factor. The apparent power is derived as 34641 VA, which splits into real power 27712.8 W that performs useful work and reactive power 20784.6 var that sustains magnetic or electric fields. Depending on the connection type, the phase voltage is 230.94 V and the phase current is 50 A. A lagging power factor indicates an inductive load where current trails voltage, while a leading factor suggests capacitance.

### Transformers (5)

<a id="energy_transformer_max_efficiency"></a>

#### Load for maximum transformer efficiency — `energy_transformer_max_efficiency`

The load (fraction of rating and apparent power) at which a transformer's efficiency is greatest (copper loss equal to core loss), that maximum efficiency and the full-load efficiency at a power factor.

$$
x_{\eta,max} = \sqrt{\frac{P_{core}}{P_{cu,fl}}},\quad \eta_{max} = \frac{xS\cos\phi}{xS\cos\phi + 2P_{core}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | rating in VA | VA |
| `P_core` | in | core (no-load) loss | W |
| `P_cu_fl` | in | copper loss at full load | W |
| `pf` | in | load power factor | — |
| `load_fraction` | out | load fraction for maximum efficiency sqrt(P_core/P_cu) | — |
| `S_at_max` | out | apparent power at maximum efficiency in VA | VA |
| `efficiency_max_pct` | out | maximum efficiency in percent | — |
| `efficiency_full_load_pct` | out | full-load efficiency in percent | — |

**Applies when:** constant core loss, copper loss proportional to the load squared.

```shell
$ energy-systems solve energy_transformer_max_efficiency "S=100 kVA" "P_core=500 W" "P_cu_fl=2000 W" pf=1
load_fraction = 0.5
S_at_max = 50000 VA
efficiency_max_pct = 98.03921569
efficiency_full_load_pct = 97.56097561
  by: Load for maximum transformer efficiency  (x = sqrt(P_core/P_cu,fl); eta_max = x S pf/(x S pf + 2 P_core))
  S = 100000 VA
  P_core = 500 W
  P_cu_fl = 2000 W
  pf = 1
  assumes: constant core loss, copper loss proportional to the load squared
```

**What the worked example means:** This calculation identifies the specific operating point where copper losses equal core losses, resulting in maximum efficiency. The optimal load fraction is computed as 0.5, which translates to an apparent power demand of 50000 VA for this 100 kVA rated transformer. At this point, the efficiency reaches 98.0392, compared to the full-load efficiency of 97.561. Engineers use these values to size transformers for typical daily loads rather than just peak capacity.

<a id="energy_parallel_transformers"></a>

#### Load sharing between two transformers in parallel — `energy_parallel_transformers`

How two transformers with equal voltage ratios but different ratings and short-circuit impedances share a load: the apparent, real and reactive power each delivers and its loading on its own rating.

$$
\mathbf S_1 = \mathbf S_L\left(\frac{\mathbf Z_2}{\mathbf Z_1 + \mathbf Z_2}\right)^*,\quad \mathbf S_2 = \mathbf S_L\left(\frac{\mathbf Z_1}{\mathbf Z_1 + \mathbf Z_2}\right)^*
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S1` | in | rating of transformer 1 (VA) | VA |
| `vk1` | in | short-circuit voltage of transformer 1 (fraction or %) | — |
| `vkr1` | in | its resistive part (fraction or %) | — |
| `S2` | in | rating of transformer 2 (VA) | VA |
| `vk2` | in | short-circuit voltage of transformer 2 (fraction or %) | — |
| `vkr2` | in | its resistive part (fraction or %) | — |
| `V_lv` | in | common secondary rated voltage | V |
| `P` | in | load real power | W |
| `Q` | in | load reactive power in var | var |
| `S1_share` | out | apparent power delivered by transformer 1 in VA | VA |
| `S2_share` | out | apparent power delivered by transformer 2 in VA | VA |
| `P1` | out | real power of transformer 1 | W |
| `Q1` | out | reactive power of transformer 1 in var | var |
| `P2` | out | real power of transformer 2 | W |
| `Q2` | out | reactive power of transformer 2 in var | var |
| `share1_pct_of_rating` | out | transformer 1 output over its rating, percent | — |
| `share2_pct_of_rating` | out | transformer 2 output over its rating, percent | — |

**Applies when:** equal voltage ratios (no circulating current); magnetizing branches neglected.

```shell
$ energy-systems solve energy_parallel_transformers "S1=500 kVA" vk1=0.05 vkr1=0.01 "S2=500 kVA" vk2=0.05 vkr2=0.01 "V_lv=400 V" "P=600 kW" "Q=200 kvar"
S1_share = 316227.766 VA
S2_share = 316227.766 VA
P1 = 300000 W
Q1 = 100000 var
P2 = 300000 W
Q2 = 100000 var
share1_pct_of_rating = 63.2455532
share2_pct_of_rating = 63.2455532
  by: Load sharing between two transformers in parallel  (impedances in ohms on the common secondary: Z_i = (vkr_i + j sqrt(vk_i^2 - vkr_i^2)) V^2/S_i; S_1 = S_L conj(Z_2/(Z_1 + Z_2)), S_2 = S_L conj(Z_1/(Z_1 + Z_2)))
  S1 = 500000 VA
  vk1 = 0.05
  vkr1 = 0.01
  S2 = 500000 VA
  vk2 = 0.05
  vkr2 = 0.01
  V_lv = 400 V
  P = 600000 W
  Q = 200000 var
  assumes: equal voltage ratios (no circulating current); magnetizing branches neglected
```

**What the worked example means:** This calculation determines how a total load is split between two parallel transformers based on their individual impedances. By converting short-circuit data into ohmic values, the model computes the complex power share for each unit. The result shows that transformer one handles 316228 VA while transformer two handles 316228 VA. This distribution reflects the inverse relationship between impedance and current share, showing which unit bears more stress relative to its capacity.

<a id="energy_transformer_tests"></a>

#### Transformer parameters from open-circuit and short-circuit tests — `energy_transformer_tests`

Core-loss resistance and magnetizing reactance from the open-circuit test, and the equivalent series resistance, reactance and impedance from the short-circuit test (each on the side where the test was made).

$$
R_c = \frac{V_{oc}^2}{P_{oc}},\ X_m = \frac{1}{\sqrt{Y_{oc}^2 - G_c^2}},\ R_{eq} = \frac{P_{sc}}{I_{sc}^2},\ X_{eq} = \sqrt{Z_{eq}^2 - R_{eq}^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Voc` | in | open-circuit test voltage | V |
| `Ioc` | in | open-circuit test current | A |
| `Poc` | in | open-circuit test power | W |
| `Vsc` | in | short-circuit test voltage | V |
| `Isc` | in | short-circuit test current | A |
| `Psc` | in | short-circuit test power | W |
| `Rc` | out | core-loss resistance (open-circuit side) | ohm |
| `Xm` | out | magnetizing reactance (open-circuit side) | ohm |
| `Req` | out | equivalent series resistance (short-circuit side) | ohm |
| `Xeq` | out | equivalent leakage reactance (short-circuit side) | ohm |
| `Zeq` | out | equivalent series impedance magnitude | ohm |
| `pf_oc` | out | open-circuit power factor | — |
| `pf_sc` | out | short-circuit power factor | — |

**Applies when:** P < V I in both tests.

```shell
$ energy-systems solve energy_transformer_tests "Voc=100 V" "Ioc=1 A" "Poc=60 W" "Vsc=50 V" "Isc=10 A" "Psc=300 W"
Rc = 166.6666667 ohm
Xm = 125 ohm
Req = 3 ohm
Xeq = 4 ohm
Zeq = 5 ohm
pf_oc = 0.6
pf_sc = 0.6
  by: Transformer parameters from open-circuit and short-circuit tests  (Rc = V_oc^2/P_oc, Xm = 1/sqrt((I_oc/V_oc)^2 - (P_oc/V_oc^2)^2); Req = P_sc/I_sc^2, Xeq = sqrt((V_sc/I_sc)^2 - Req^2))
  Voc = 100 V
  Ioc = 1 A
  Poc = 60 W
  Vsc = 50 V
  Isc = 10 A
  Psc = 300 W
  assumes: P < V I in both tests
```

**What the worked example means:** This calculation determines the transformer's equivalent circuit parameters using open-circuit and short-circuit test data. The open-circuit test yields the core-loss resistance of 166.667 ohm and the magnetizing reactance of 125 ohm, representing the shunt branch on the low-voltage side. The short-circuit test provides the equivalent series resistance of 3 ohm and leakage reactance of 4 ohm, which combine to form a total impedance magnitude of 5 ohm. These values allow for accurate modeling of voltage regulation and efficiency under load conditions.

<a id="energy_transformer_performance"></a>

#### Transformer regulation and efficiency from its equivalent circuit — `energy_transformer_performance`

Primary voltage needed, voltage regulation, primary current and input power factor, copper and core losses and efficiency of a single-phase transformer (approximate equivalent circuit referred to the primary, shunt branch at the primary terminals) delivering a fraction of its rating at rated secondary voltage.

$$
\mathbf V_1 = a\mathbf V_2 + \frac{\mathbf I_2}{a}(R_{eq} + jX_{eq}),\quad \eta = \frac{P_{out}}{P_{out} + P_{cu} + P_{core}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | rating in VA | VA |
| `V1` | in | rated primary voltage | V |
| `V2` | in | rated secondary voltage | V |
| `Req` | in | series resistance referred to the primary | ohm |
| `Xeq` | in | series leakage reactance referred to the primary | ohm |
| `Rc` | in | core-loss resistance (primary side) | ohm |
| `Xm` | in | magnetizing reactance (primary side) | ohm |
| `load` | in | load as a fraction of the rating (or a percentage) | — |
| `pf` | in | load power factor | — |
| `pf_type` | in | lagging (default) or leading load (optional) | — |
| `V1_required` | out | primary voltage for rated secondary voltage at this load | V |
| `regulation_pct` | out | voltage regulation in percent | — |
| `I1` | out | primary current | A |
| `pf_in` | out | input power factor | — |
| `P_out` | out | output power | W |
| `P_cu` | out | copper loss | W |
| `P_core` | out | core loss | W |
| `efficiency_pct` | out | efficiency in percent | — |

**Applies when:** approximate equivalent circuit (shunt branch at the primary terminals); 0 < load <= 2.

```shell
$ energy-systems solve energy_transformer_performance "S=10 kVA" "V1=1000 V" "V2=100 V" "Req=1 ohm" "Xeq=0 ohm" "Rc=1000000000 ohm" "Xm=1000000000 ohm" load=1 pf=1
V1_required = 1010 V
regulation_pct = 1
I1 = 10.00000101 A
pf_in = 1
P_out = 10000 W
P_cu = 100 W
P_core = 0.0010201 W
efficiency_pct = 99.00989099
  by: Transformer regulation and efficiency from its equivalent circuit  (V1 = a V2 + I2' (Req + jXeq), I2' = I2/a at angle -acos(pf); P_cu = |I2'|^2 Req; P_core = V1^2/Rc; eta = P_out/(P_out + P_cu + P_core))
  S = 10000 VA
  V1 = 1000 V
  V2 = 100 V
  Req = 1 ohm
  Xeq = 0 ohm
  Rc = 1000000000 ohm
  Xm = 1000000000 ohm
  load = 1
  pf = 1
  assumes: approximate equivalent circuit (shunt branch at the primary terminals); 0 < load <= 2
```

**What the worked example means:** This calculation determines the operating characteristics of a single-phase transformer using its approximate equivalent circuit referred to the primary side. Applying the load fraction and power factor gives the necessary primary voltage, which results in 1010 V. The analysis also yields the voltage regulation of 1, the primary current of 10 A, and the input power factor of 1. Losses are separated into copper loss of 100 W and core loss of 0.0010201 W, leading to a final efficiency of 99.0099 for the delivered output power of 10000 W.

<a id="energy_transformer_loading"></a>

#### Transformer under load from nameplate data — `energy_transformer_loading`

Secondary voltage, losses, efficiency and loading of a transformer fed at rated primary voltage and supplying a P + jQ load, from its nameplate: rating, rated voltages, short-circuit voltage vk and its resistive part vkr, no-load loss and no-load current (T equivalent circuit).

$$
z_k = v_{kr} + j\sqrt{v_k^2 - v_{kr}^2},\quad y_m = \frac{P_0}{S_n} - j\sqrt{i_0^2 - \left(\frac{P_0}{S_n}\right)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sn` | in | rating in VA | VA |
| `V_hv` | in | rated (and applied) high-voltage side voltage | V |
| `V_lv` | in | rated low-voltage side voltage | V |
| `vk` | in | short-circuit voltage as a fraction or percentage of rated voltage | — |
| `vkr` | in | resistive part of the short-circuit voltage (fraction or %) | — |
| `P0` | in | no-load (iron) loss | W |
| `i0` | in | no-load current as a fraction or percentage of rated current | — |
| `P` | in | load real power on the low-voltage side | W |
| `Q` | in | load reactive power in var | var |
| `V_lv_actual` | out | low-voltage side voltage under load | V |
| `V_lv_pu` | out | low-voltage side voltage in per unit | — |
| `delta_deg` | out | phase shift of the secondary voltage in degrees (no vector-group shift) | — |
| `P_loss` | out | total real loss | W |
| `Q_loss` | out | reactive power absorbed in var | var |
| `efficiency_pct` | out | efficiency in percent | — |
| `loading_pct` | out | loading: the larger winding current over its rated current, percent | — |

**Applies when:** T equivalent circuit (series impedance halved on each side of the magnetizing branch); rated voltage applied, nominal tap.

```shell
$ energy-systems solve energy_transformer_loading "sn=630 kVA" "V_hv=20 kV" "V_lv=400 V" vk=0.06 vkr=0.012 "P0=1.5 kW" i0=0.003 "P=400 kW" "Q=150 kvar"
V_lv_actual = 390.8755133 V
V_lv_pu = 0.9771887833
delta_deg = -2.024828877
P_loss = 5120.794624 W
Q_loss = 19030.91932 var
efficiency_pct = 98.73598327
loading_pct = 69.67771432
  by: Transformer under load from nameplate data  (per unit on the rating: z = vkr + j sqrt(vk^2 - vkr^2), y_m = P0/S - j sqrt(i0^2 - (P0/S)^2); the T circuit solved for the load-bus voltage by Newton iteration)
  sn = 630000 VA
  V_hv = 20000 V
  V_lv = 400 V
  vk = 0.06
  vkr = 0.012
  P0 = 1500 W
  i0 = 0.003
  P = 400000 W
  Q = 150000 var
  assumes: T equivalent circuit (series impedance halved on each side of the magnetizing branch); rated voltage applied, nominal tap
```

**What the worked example means:** This calculation determines the operating state of a two-winding transformer using its nameplate data. It computes the actual secondary voltage, which is found to be 390.876 V, along with the resulting phase shift of -2.02483 degrees. The model also evaluates power losses, showing a real loss of 5120.79 W and reactive absorption of 19030.9 var. Based on these values, the overall efficiency is calculated as 98.736, while the loading percentage indicates how close the transformer is to its rated capacity.

### Wind power (9)

<a id="energy_actuator_disc"></a>

#### Actuator-disc (momentum theory) rotor performance — `energy_actuator_disc`

Power and thrust coefficients, disc and far-wake speeds, power and thrust of an ideal rotor (actuator disc) with axial induction factor a; a = 1/3 gives the Betz optimum Cp = 16/27.

$$
C_P = 4a(1-a)^2,\quad C_T = 4a(1-a)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | axial induction factor (0 <= a < 0.5) | — |
| `rho` | in | air density | kg/m^3 |
| `D` | in | rotor diameter | m |
| `v` | in | free-stream wind speed | m/s |
| `Cp` | out | power coefficient 4a(1-a)^2 | — |
| `Ct` | out | thrust coefficient 4a(1-a) | — |
| `v_disc` | out | speed at the disc | m/s |
| `v_wake` | out | far-wake speed | m/s |
| `P` | out | power extracted | W |
| `T` | out | thrust on the rotor | N |

**Applies when:** 0 <= a < 0.5.

```shell
$ energy-systems solve energy_actuator_disc a=0.25 "rho=1.2 kg/m^3" "D=10 m" "v=8 m/s"
Cp = 0.5625
Ct = 0.75
v_disc = 6 m/s
v_wake = 4 m/s
P = 13571.68026 W
T = 2261.946711 N
  by: Actuator-disc (momentum theory) rotor performance  (Cp = 4a(1 - a)^2, Ct = 4a(1 - a), v_disc = v(1 - a), v_wake = v(1 - 2a); P = Cp (1/2) rho A v^3, T = Ct (1/2) rho A v^2)
  a = 0.25
  rho = 1.2 kg/m^3
  D = 10 m
  v = 8 m/s
  assumes: 0 <= a < 0.5
```

**What the worked example means:** This calculation determines the performance of an actuator disc model for a wind turbine. Using an axial induction factor of 0.25, the method computes a power coefficient and a thrust coefficient, which are then applied to the swept area defined by diameter 10 m and free-stream speed 8 m/s. The resulting extracted power is 13571.7 W and the mechanical thrust on the rotor is 2261.95 N. The flow slows to 6 m/s at the disc plane and further decelerates to a far-wake speed of 4 m/s. These values represent the theoretical maximum performance under momentum theory assumptions.

<a id="energy_hub_air_density"></a>

#### Air density at hub height — `energy_hub_air_density`

Temperature, pressure and air density at a turbine's hub height from a pressure and a temperature measured at other heights: temperature gradient -6.5 K/km, pressure gradient -1/8 hPa per m, then the barometric (1.225 kg/m^3 at 288.15 K, 1013.3 hPa) or the ideal-gas (R = 287.058 J/(kg K)) density.

$$
\rho_{hub} = 1.225\,\frac{p_{hub}}{101330\ \mathrm{Pa}}\,\frac{288.15\ \mathrm{K}}{T_{hub}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | measured air pressure | Pa |
| `h_p` | in | height of the pressure measurement | m |
| `T` | in | measured air temperature | K |
| `h_T` | in | height of the temperature measurement | m |
| `hub` | in | hub height | m |
| `model` | in | density model (default barometric) (optional) | — |
| `T_hub` | out | temperature at hub height | K |
| `p_hub` | out | pressure at hub height | Pa |
| `rho` | out | air density at hub height | kg/m^3 |

**Applies when:** pressure and temperature stay positive at hub height.

```shell
$ energy-systems solve energy_hub_air_density "p=101330 Pa" "h_p=0 m" "T=288.15 K" "h_T=0 m" "hub=0 m"
T_hub = 288.15 K
p_hub = 101330 Pa
rho = 1.225 kg/m^3
  by: Air density at hub height  (T_hub = T - 0.0065 (h_hub - h_T); p_hub = p - 12.5 Pa/m (h_hub - h_p); rho = p_hub/(R T_hub) (ideal gas) or 1.225 (p_hub/101330)(288.15/T_hub) (barometric))
  p = 101330 Pa
  h_p = 0 m
  T = 288.15 K
  h_T = 0 m
  hub = 0 m
  assumes: pressure and temperature stay positive at hub height
```

**What the worked example means:** This calculation determines the air temperature, pressure, and density at the turbine's hub height using measurements taken at different altitudes. Starting with a measured pressure of 101330 Pa and temperature of 288.15 K, the model adjusts these values for the height difference to find the hub pressure of 101330 Pa and hub temperature of 288.15 K. These adjusted values are then used to compute the final air density of 1.225 kg/m^3, which is critical for estimating wind power potential.

<a id="energy_weibull_aep"></a>

#### Annual energy production from a power curve and a Weibull distribution — `energy_weibull_aep`

Mean output, annual energy production and capacity factor of a wind turbine whose power curve (linear between points, zero outside) operates in winds with a Weibull distribution (shape k, scale c).

$$
\bar P = \int_0^\infty P(v)\,\frac{k}{c}\left(\frac vc\right)^{k-1}e^{-(v/c)^k}\,dv,\quad AEP = 8760\,\mathrm{h}\cdot\bar P
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `speeds` | in | power-curve wind speeds in m/s (increasing) | — |
| `powers` | in | power-curve outputs in kW | — |
| `k` | in | Weibull shape factor (1 < k <= 5) | — |
| `c` | in | Weibull scale factor | m/s |
| `hours` | in | hours in the period (default 8760) (optional) | — |
| `mean_power_kW` | out | mean output in kW | — |
| `AEP_MWh` | out | energy over the period in MWh | — |
| `capacity_factor` | out | mean output over the largest output of the curve | — |

**Applies when:** 1 < k <= 5; speeds strictly increasing.

```shell
$ energy-systems solve energy_weibull_aep "speeds=[0, 1000]" "powers=[0, 1000]" k=2 "c=7 m/s"
mean_power_kW = 6.203588478
AEP_MWh = 54.34343507
capacity_factor = 0.006203588478
  by: Annual energy production from a power curve and a Weibull distribution  (P_mean = sum over segments of integral (P_i + s (v - v_i)) f(v) dv, each in closed form: F(b) - F(a) and c Gamma(1 + 1/k) [P(1 + 1/k, (b/c)^k) - P(1 + 1/k, (a/c)^k)])
  k = 2
  c = 7 m/s
  assumes: 1 < k <= 5; speeds strictly increasing
```

**What the worked example means:** This calculation determines the average electrical output of a wind turbine by integrating its piecewise linear power curve against a Weibull distribution of wind speeds. The resulting mean power is 6.20359, which leads to an annual energy production of 54.3434 over the specified period. This value represents the expected energy yield assuming the site's wind conditions follow the provided statistical model.

<a id="energy_wind_power"></a>

#### Power in the wind and the Betz limit — `energy_wind_power`

The kinetic power of the wind through a rotor's swept area, the maximum a turbine can extract (Betz limit 16/27), the power density, and the power at a given power coefficient.

$$
P = \tfrac12\rho A v^3,\quad P_{max} = \tfrac{16}{27}\cdot\tfrac12\rho A v^3
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | air density | kg/m^3 |
| `D` | in | rotor diameter | m |
| `v` | in | wind speed | m/s |
| `cp` | in | power coefficient (optional, at most 16/27) (optional) | — |
| `P_wind` | out | power in the wind through the swept area | W |
| `P_betz` | out | maximum extractable power (Betz) | W |
| `power_density` | out | wind power density in W/m^2 | — |
| `area` | out | swept area | m^2 |
| `P` | out | power at the given power coefficient | W |

**Applies when:** rho, D, v > 0; 0 < cp <= 16/27.

```shell
$ energy-systems solve energy_wind_power "rho=1.225 kg/m^3" "D=80 m" "v=10 m/s" cp=0.4
P_wind = 3078760.801 W
P_betz = 1824450.845 W
power_density = 612.5
area = 5026.548246 m^2
P = 1231504.32 W
  by: Power in the wind and the Betz limit  (P = (1/2) rho A v^3, A = pi D^2/4; P_Betz = (16/27) P; P(cp) = cp P)
  rho = 1.225 kg/m^3
  D = 80 m
  v = 10 m/s
  cp = 0.4
  assumes: rho, D, v > 0; 0 < cp <= 16/27
```

**What the worked example means:** This calculation determines the total kinetic energy available in the wind passing through the rotor's swept area, which equals 3.07876e+06 W. It then applies the theoretical Betz limit to find the absolute maximum power that can be extracted, resulting in 1.82445e+06 W. Finally, it computes the actual power output for a specific efficiency coefficient, yielding 1.2315e+06 W. The swept area used for these calculations is 5026.55 m^2, and the power density of the wind is 612.5. These values illustrate the gap between raw wind energy and practical mechanical extraction limits.

<a id="energy_power_curve_density_corrected"></a>

#### Turbine output from a density-corrected power curve — `energy_power_curve_density_corrected`

Turbine output at a wind speed and air density, with the power curve's speeds rescaled by (1.225/rho)^e, e rising from 1/3 below 7.5 m/s to 2/3 above 12.5 m/s (Svenningsen's correction, as in windpowerlib).

$$
v_i' = v_i\left(\frac{1.225}{\rho}\right)^{e(v_i)},\ e\in[\tfrac13,\tfrac23]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `speeds` | in | power-curve wind speeds in m/s at 1.225 kg/m^3 (increasing) | — |
| `powers` | in | power-curve outputs in kW | — |
| `v` | in | wind speed at hub height | m/s |
| `rho` | in | air density at hub height | kg/m^3 |
| `P_kW` | out | output in kW | — |
| `P` | out | output | W |

**Applies when:** 0.5 < rho < 1.6 kg/m^3.

```shell
$ energy-systems solve energy_power_curve_density_corrected "speeds=[3, 5, 8, 12, 25]" "powers=[0, 150, 800, 2000, 2000]" "v=6.5 m/s" "rho=1.225 kg/m^3"
P_kW = 475
P = 475000 W
  by: Turbine output from a density-corrected power curve  (v_i' = v_i (1.225/rho)^e(v_i); interpolate P(v) on (v_i', P_i))
  v = 6.5 m/s
  rho = 1.225 kg/m^3
  assumes: 0.5 < rho < 1.6 kg/m^3
```

**What the worked example means:** This calculation determines the expected power output by adjusting the standard power curve for actual atmospheric conditions. The input wind speed of 6.5 m/s m/s is corrected using the air density of 1.225 kg/m^3 to find an equivalent speed on the reference curve, which then yields a predicted output of 475. This method ensures that power estimates remain accurate even when the air is denser or thinner than the standard value used by manufacturers.

<a id="energy_power_curve_output"></a>

#### Turbine output from its power curve — `energy_power_curve_output`

The electrical output of a wind turbine at a wind speed by linear interpolation of its power curve (zero below the first and above the last listed speed).

$$
P(v) = P_i + (P_{i+1} - P_i)\frac{v - v_i}{v_{i+1} - v_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `speeds` | in | power-curve wind speeds in m/s (increasing) | — |
| `powers` | in | power-curve outputs in kW | — |
| `v` | in | wind speed at hub height | m/s |
| `P_kW` | out | output in kW | — |
| `P` | out | output | W |

**Applies when:** speeds strictly increasing; one power per speed.

```shell
$ energy-systems solve energy_power_curve_output "speeds=[3, 5, 8, 12, 25]" "powers=[0, 150, 800, 2000, 2000]" "v=6.5 m/s"
P_kW = 475
P = 475000 W
  by: Turbine output from its power curve  (linear interpolation between the curve points; 0 outside the listed speeds)
  v = 6.5 m/s
  assumes: speeds strictly increasing; one power per speed
```

**What the worked example means:** This calculation determines the electrical output by linearly interpolating between points on the manufacturer's power curve. At the specific wind speed of 6.5 m/s, the resulting power is 475000 W. If the speed falls outside the range defined by the curve data, the output is zero. The value in kilowatts is 475.

<a id="energy_weibull_wind"></a>

#### Weibull wind statistics and mean power density — `energy_weibull_wind`

Mean wind speed, mean wind power density, energy pattern factor, most probable speed and the speed carrying most energy for a Weibull wind-speed distribution with shape k and scale c.

$$
\bar v = c\,\Gamma\!\left(1 + \tfrac1k\right),\quad \frac{\bar P}{A} = \tfrac12\rho c^3\,\Gamma\!\left(1 + \tfrac3k\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | Weibull shape factor (1 < k <= 5) | — |
| `c` | in | Weibull scale factor | m/s |
| `rho` | in | air density | kg/m^3 |
| `v_mean` | out | mean wind speed c Gamma(1 + 1/k) | m/s |
| `power_density` | out | mean wind power density in W/m^2 | — |
| `energy_pattern_factor` | out | mean of v^3 over the cube of the mean | — |
| `v_most_probable` | out | most probable wind speed | m/s |
| `v_max_energy` | out | wind speed carrying the most energy | m/s |

**Applies when:** 1 < k <= 5; c > 0.

```shell
$ energy-systems solve energy_weibull_wind k=2 "c=8 m/s" "rho=1.225 kg/m^3"
v_mean = 7.089815404 m/s
power_density = 416.8811457
energy_pattern_factor = 1.909859317
v_most_probable = 5.656854249 m/s
v_max_energy = 11.3137085 m/s
  by: Weibull wind statistics and mean power density  (v_mean = c Gamma(1 + 1/k); P/A = (1/2) rho c^3 Gamma(1 + 3/k); v_mp = c ((k - 1)/k)^(1/k); v_maxE = c ((k + 2)/k)^(1/k))
  k = 2
  c = 8 m/s
  rho = 1.225 kg/m^3
  assumes: 1 < k <= 5; c > 0
```

**What the worked example means:** This calculation determines key statistical properties of the wind resource using the Weibull probability distribution. Given a shape factor of 2 and a scale factor of 8 m/s, along with an air density of 1.225 kg/m^3, it computes the mean wind speed as 7.08982 m/s and the mean wind power density as 416.881. It also identifies the most probable wind speed at 5.65685 m/s and the speed carrying the maximum energy at 11.3137 m/s. The energy pattern factor, calculated as 1.90986, indicates how much more energetic the variable wind is compared to a steady wind at the average speed.

<a id="energy_wind_shear_log"></a>

#### Wind speed at another height (logarithmic profile) — `energy_wind_shear_log`

The wind speed at height h2 from a measurement at h1 by the logarithmic wind profile with roughness length z0 and an optional obstacle height d (displacement 0.7 d).

$$
v_2 = v_1\,\frac{\ln\left((h_2 - 0.7d)/z_0\right)}{\ln\left((h_1 - 0.7d)/z_0\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v1` | in | measured wind speed | m/s |
| `h1` | in | measurement height | m |
| `h2` | in | target height | m |
| `z0` | in | surface roughness length | m |
| `d` | in | obstacle height (default 0; displacement height 0.7 d) (optional) | m |
| `v2` | out | wind speed at h2 | m/s |

**Applies when:** h - 0.7 d > z0 at both heights.

```shell
$ energy-systems solve energy_wind_shear_log "v1=5 m/s" "h1=10 m" "h2=100 m" "z0=1 m"
v2 = 10 m/s
  by: Wind speed at another height (logarithmic profile)  (v2 = v1 ln((h2 - 0.7 d)/z0)/ln((h1 - 0.7 d)/z0))
  v1 = 5 m/s
  h1 = 10 m
  h2 = 100 m
  z0 = 1 m
  assumes: h - 0.7 d > z0 at both heights
```

**What the worked example means:** The wind speed at height 100 m is calculated as 10 m/s, based on a measurement of 5 m/s at 10 m.

<a id="energy_wind_shear_power_law"></a>

#### Wind speed at another height (power law) — `energy_wind_shear_power_law`

The wind speed at height h2 from a measurement at h1 by the power (Hellman) law v2 = v1 (h2/h1)^alpha; alpha = 1/7 when not stated (the one-seventh law).

$$
v_2 = v_1\left(\frac{h_2}{h_1}\right)^{\alpha}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v1` | in | measured wind speed | m/s |
| `h1` | in | measurement height | m |
| `h2` | in | target height (e.g. hub height) | m |
| `alpha` | in | shear exponent (default 1/7) (optional) | — |
| `v2` | out | wind speed at h2 | m/s |

**Applies when:** heights > 0; 0 <= alpha <= 1.

```shell
$ energy-systems solve energy_wind_shear_power_law "v1=6 m/s" "h1=10 m" "h2=1280 m"
v2 = 12 m/s
  by: Wind speed at another height (power law)  (v2 = v1 (h2/h1)^alpha)
  v1 = 6 m/s
  h1 = 10 m
  h2 = 1280 m
  assumes: heights > 0; 0 <= alpha <= 1
```

**What the worked example means:** The wind speed at the target height is 12 m/s, derived from a measurement of 6 m/s at 10 m.

Questions and reports: support@ai999x.com. AI999X — The Future Is Self-Improving.
