# Energy Systems: the desktop app, user guide

**Energy Systems**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Energy Systems covers power systems and load flow, three-phase power, transformers and machines, solar photovoltaics, wind power, batteries, fuel cells and hydrogen. Its desktop app holds all 83 calculations in 11 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-energy-systems-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-energy-systems-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-energy-systems-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-energy-systems.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-energy-systems-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-energy-systems-1.6.5-linux-x86_64.tar.gz
./ai999x-energy-systems-1.6.5-linux-x86_64/ai999x-energy-systems
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Energy Systems app, opened on Balanced three-phase power from line quantities with its worked example.](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/hero.webp)

*The Energy Systems app, opened on Balanced three-phase power from line quantities with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Three-phase power › Balanced three-phase power from line quantities (or type part of its name in the search box). It opens with a worked example filled in: “Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star” The answer: P = 27712.81292 W, Q = 20784.60969 var, S = 34641.01615 VA, V_phase = 230.9401077 V.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Balanced three-phase power from line quantities: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-1.webp)

*Balanced three-phase power from line quantities: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Balanced three-phase power from line quantities
P = 27712.81292 W
Q = 20784.60969 var
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Sun position at a place and local time](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-2.webp)

*Sun position at a place and local time*

![Power in the wind and the Betz limit](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-3.webp)

*Power in the wind and the Betz limit*

![Hydrogen stored in a tank at pressure (real gas)](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-4.webp)

*Hydrogen stored in a tank at pressure (real gas)*

![Economic dispatch with quadratic costs and limits](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-5.webp)

*Economic dispatch with quadratic costs and limits*

![Battery pack series-parallel sizing](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-6.webp)

*Battery pack series-parallel sizing*

![Electrode kinetics: Butler-Volmer current, overpotential and Tafel slopes](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-7.webp)

*Electrode kinetics: Butler-Volmer current, overpotential and Tafel slopes*

![Capacity factor and energy of a generating plant](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/scene-8.webp)

*Capacity factor and energy of a generating plant*

![Balanced three-phase power from line quantities: a chart, enlarged.](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/chart-1.webp)

*Balanced three-phase power from line quantities: a chart, enlarged.*

![Sun position at a place and local time: a chart, enlarged.](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/chart-2.webp)

*Sun position at a place and local time: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Energy Systems app in the dark theme.](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/hero-dark.webp)

*The Energy Systems app in the dark theme.*

## 7. Every calculation

83 calculations in 11 topics; each has its own entry on the model's page (https://www.ai999x.com/models/energy-systems-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Batteries | Battery pack series-parallel sizing, Battery terminal voltage under constant current (one-RC equivalent circuit), Peukert exponent and run time at a new discharge current, State of charge by coulomb counting |
| Fuel cells | Electrical power of a fuel-cell or electrolyser stack, Electrode kinetics: Butler-Volmer current, overpotential and Tafel slopes, Hydrogen fuel-cell stack: consumption, power and efficiency, Water electrolyser: hydrogen production and energy per kilogram |
| Generation | Capacity factor and energy of a generating plant, Hydroelectric power from flow and head, and the energy over a period |
| Hydrogen | Hydrogen stored in a tank at pressure (real gas), Isentropic (adiabatic) compression of hydrogen, Minimum (isothermal) work to compress hydrogen |
| Machines | Induction motor breakdown (maximum) torque and starting torque, Induction motor performance from its equivalent circuit, Induction motor power flow from slip and losses, Synchronous generator internal EMF, power angle and regulation, Synchronous speed, rotor speed and slip of an AC machine |
| Photovoltaics | Combined PV system losses (PVWatts), Incidence angle modifier (ASHRAE), Incidence angle modifier from Fresnel reflection and absorption, Inverter AC output and efficiency (PVWatts), PV array DC power with temperature derating (PVWatts), PV array output at operating temperature, PV cell and module temperature (Sandia SAPM model), PV cell temperature from NOCT, PV module maximum power at an irradiance and temperature, PV module single-diode parameters at operating conditions (De Soto), PV module temperature (Faiman model), PV single-diode model: current at a voltage, PV single-diode model: short-circuit, open-circuit and maximum power point, PV single-diode model: voltage at a current |
| Power systems | AC load flow by Newton-Raphson, Base impedance and per-unit impedance, Critical clearing angle and time (equal-area criterion), DC power flow, Double line-to-ground fault (symmetrical components), Economic dispatch with quadratic costs and limits, Frequency deviation and load sharing under governor droop, Geometric mean distance of a three-phase line, Inductance, capacitance and surge impedance of a three-phase overhead line, Line-to-line fault (symmetrical components), Long transmission line (exact distributed parameters), Medium transmission line (nominal pi), Per-unit impedance on a new base, Power transfer and power angle between two voltages, Receiving-end voltage of a line feeding a constant-power load (two-bus load flow), Short transmission line: sending-end voltage, regulation and efficiency, Single line-to-ground fault (symmetrical components), Three-phase (symmetrical) fault current by the bus impedance matrix |
| Solar resource | Angle of incidence of the sun on a tilted surface, Clear-sky global horizontal irradiance (Haurwitz), Clear-sky irradiance (Ineichen-Perez with Linke turbidity), Diffuse and direct irradiance from global horizontal irradiance (Erbs), Direct normal irradiance from global horizontal irradiance (DISC model), Equation of time, Extraterrestrial (top-of-atmosphere) normal irradiance, Irradiance on a tilted plane (Hay-Davies sky model), Irradiance on a tilted plane (isotropic sky), Irradiance on a tilted plane (Klucher sky model), Irradiance on a tilted plane (Perez sky model), Irradiance on a tilted plane (Reindl sky model), Optical air mass, Solar declination on a day of the year, Solar time and hour angle from clock time, Solar zenith, elevation and azimuth from latitude, declination and hour angle, Sun position at a place and local time, Sunrise, sunset, solar noon and day length |
| Three-phase power | Balanced three-phase power from line quantities |
| Transformers | Load for maximum transformer efficiency, Load sharing between two transformers in parallel, Transformer parameters from open-circuit and short-circuit tests, Transformer regulation and efficiency from its equivalent circuit, Transformer under load from nameplate data |
| Wind power | Actuator-disc (momentum theory) rotor performance, Air density at hub height, Annual energy production from a power curve and a Weibull distribution, Power in the wind and the Betz limit, Turbine output from a density-corrected power curve, Turbine output from its power curve, Weibull wind statistics and mean power density, Wind speed at another height (logarithmic profile), Wind speed at another height (power law) |

## 8. Other ways to use it

The same engine runs on the command line (`energy-systems`, its own guide), in Python (`pip install energy-systems-ai999x`, `import energy_systems_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/energy-systems-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/energy-systems-ai999x/
