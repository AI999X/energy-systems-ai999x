# energy-systems-ai999x: capability guide

**Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X. Ask the package `Who created you?` or call `energy_systems_ai999x.about()` for the same answer.

Every capability energy-systems-ai999x supports, generated from its certified capability contracts. Every example below was executed when this guide was built. The structured call `solve("<capability id>", ...)` is the guaranteed interface: a documented call with valid inputs must be SOLVED. Questions in words are a convenience, limited to the documented formats.

## Quick start

```python
import energy_systems_ai999x

result = energy_systems_ai999x.solve('energy_solar_declination', n=172)
result.show()
# declination_deg = 23.4498, declination_rad = 0.409276
```

## By topic

| topic | capabilities |
|---|---|
| Batteries / capacity | [Peukert exponent and run time at a new discharge current](#energy_peukert) |
| Batteries / equivalent circuits | [Battery terminal voltage under constant current (one-RC equivalent circuit)](#energy_battery_rc_discharge) |
| Batteries / pack design | [Battery pack series-parallel sizing](#energy_battery_pack_sizing) |
| Batteries / state of charge | [State of charge by coulomb counting](#energy_coulomb_counting) |
| Fuel cells / electrolysis | [Water electrolyser: hydrogen production and energy per kilogram](#energy_electrolyser) |
| Fuel cells / hydrogen fuel cells | [Hydrogen fuel-cell stack: consumption, power and efficiency](#energy_fuel_cell_stack) |
| Fuel cells / stack power | [Electrical power of a fuel-cell or electrolyser stack](#energy_stack_electrical_power) |
| Hydrogen / compression | [Minimum (isothermal) work to compress hydrogen](#energy_hydrogen_isothermal_compression), [Isentropic (adiabatic) compression of hydrogen](#energy_hydrogen_isentropic_compression) |
| Hydrogen / storage | [Hydrogen stored in a tank at pressure (real gas)](#energy_hydrogen_tank) |
| Machines / induction motors | [Induction motor performance from its equivalent circuit](#energy_induction_motor), [Induction motor breakdown (maximum) torque and starting torque](#energy_induction_max_torque) |
| Machines / speed and slip | [Synchronous speed, rotor speed and slip of an AC machine](#energy_synchronous_speed) |
| Machines / synchronous machines | [Synchronous generator internal EMF, power angle and regulation](#energy_synchronous_generator) |
| Photovoltaics / array output | [PV array DC power with temperature derating (PVWatts)](#energy_pvwatts_dc), [PV array output at operating temperature](#energy_pv_array_output) |
| Photovoltaics / cell temperature | [PV cell temperature from NOCT](#energy_cell_temperature_noct), [PV module temperature (Faiman model)](#energy_cell_temperature_faiman), [PV cell and module temperature (Sandia SAPM model)](#energy_cell_temperature_sapm) |
| Photovoltaics / inverters | [Inverter AC output and efficiency (PVWatts)](#energy_pvwatts_inverter) |
| Photovoltaics / module performance | [PV module maximum power at an irradiance and temperature](#energy_pv_module_at_conditions) |
| Photovoltaics / optical losses | [Incidence angle modifier (ASHRAE)](#energy_iam_ashrae), [Incidence angle modifier from Fresnel reflection and absorption](#energy_iam_physical) |
| Photovoltaics / single-diode model | [PV single-diode model: current at a voltage](#energy_single_diode_current), [PV single-diode model: voltage at a current](#energy_single_diode_voltage), [PV single-diode model: short-circuit, open-circuit and maximum power point](#energy_single_diode_mpp), [PV module single-diode parameters at operating conditions (De Soto)](#energy_desoto_parameters) |
| Photovoltaics / system losses | [Combined PV system losses (PVWatts)](#energy_pvwatts_losses) |
| Power systems / economic dispatch | [Economic dispatch with quadratic costs and limits](#energy_economic_dispatch) |
| Power systems / faults | [Three-phase (symmetrical) fault current by the bus impedance matrix](#energy_symmetrical_fault), [Single line-to-ground fault (symmetrical components)](#energy_slg_fault), [Line-to-line fault (symmetrical components)](#energy_ll_fault), [Double line-to-ground fault (symmetrical components)](#energy_dlg_fault) |
| Power systems / frequency control | [Frequency deviation and load sharing under governor droop](#energy_droop_sharing) |
| Power systems / line parameters | [Inductance, capacitance and surge impedance of a three-phase overhead line](#energy_line_parameters), [Geometric mean distance of a three-phase line](#energy_geometric_mean_distance) |
| Power systems / load flow | [Receiving-end voltage of a line feeding a constant-power load (two-bus load flow)](#energy_two_bus_load_flow), [DC power flow](#energy_dc_power_flow), [AC load flow by Newton-Raphson](#energy_newton_load_flow) |
| Power systems / per-unit system | [Per-unit impedance on a new base](#energy_per_unit_change_of_base), [Base impedance and per-unit impedance](#energy_per_unit_impedance) |
| Power systems / stability | [Power transfer and power angle between two voltages](#energy_power_angle) |
| Power systems / transient stability | [Critical clearing angle and time (equal-area criterion)](#energy_critical_clearing) |
| Power systems / transmission lines | [Short transmission line: sending-end voltage, regulation and efficiency](#energy_short_line), [Medium transmission line (nominal pi)](#energy_medium_line), [Long transmission line (exact distributed parameters)](#energy_long_line) |
| Solar resource / atmosphere | [Optical air mass](#energy_air_mass) |
| Solar resource / clear sky | [Clear-sky global horizontal irradiance (Haurwitz)](#energy_clear_sky_haurwitz), [Clear-sky irradiance (Ineichen-Perez with Linke turbidity)](#energy_clear_sky_ineichen) |
| Solar resource / irradiance | [Extraterrestrial (top-of-atmosphere) normal irradiance](#energy_extraterrestrial_irradiance) |
| Solar resource / irradiance models | [Diffuse and direct irradiance from global horizontal irradiance (Erbs)](#energy_erbs_decomposition), [Direct normal irradiance from global horizontal irradiance (DISC model)](#energy_disc_dni) |
| Solar resource / solar time | [Equation of time](#energy_equation_of_time), [Solar time and hour angle from clock time](#energy_solar_time_hour_angle) |
| Solar resource / sun position | [Solar declination on a day of the year](#energy_solar_declination), [Solar zenith, elevation and azimuth from latitude, declination and hour angle](#energy_sun_angles), [Sun position at a place and local time](#energy_solar_position), [Sunrise, sunset, solar noon and day length](#energy_sunrise_sunset) |
| Solar resource / tilted surfaces | [Angle of incidence of the sun on a tilted surface](#energy_angle_of_incidence), [Irradiance on a tilted plane (isotropic sky)](#energy_poa_isotropic), [Irradiance on a tilted plane (Klucher sky model)](#energy_poa_klucher), [Irradiance on a tilted plane (Hay-Davies sky model)](#energy_poa_haydavies), [Irradiance on a tilted plane (Reindl sky model)](#energy_poa_reindl), [Irradiance on a tilted plane (Perez sky model)](#energy_poa_perez) |
| Three-phase power / balanced systems | [Balanced three-phase power from line quantities](#energy_three_phase_power) |
| Transformers / efficiency | [Load for maximum transformer efficiency](#energy_transformer_max_efficiency) |
| Transformers / equivalent circuit | [Transformer regulation and efficiency from its equivalent circuit](#energy_transformer_performance) |
| Transformers / loading | [Transformer under load from nameplate data](#energy_transformer_loading) |
| Transformers / parallel operation | [Load sharing between two transformers in parallel](#energy_parallel_transformers) |
| Transformers / testing | [Transformer parameters from open-circuit and short-circuit tests](#energy_transformer_tests) |
| Wind power / annual energy | [Annual energy production from a power curve and a Weibull distribution](#energy_weibull_aep) |
| Wind power / rotor aerodynamics | [Actuator-disc (momentum theory) rotor performance](#energy_actuator_disc) |
| Wind power / turbine output | [Turbine output from its power curve](#energy_power_curve_output), [Turbine output from a density-corrected power curve](#energy_power_curve_density_corrected) |
| Wind power / wind resource | [Power in the wind and the Betz limit](#energy_wind_power), [Wind speed at another height (power law)](#energy_wind_shear_power_law), [Wind speed at another height (logarithmic profile)](#energy_wind_shear_log), [Air density at hub height](#energy_hub_air_density), [Weibull wind statistics and mean power density](#energy_weibull_wind) |

## Outcomes

| outcome | meaning | what to do |
|---|---|---|
| SOLVED | a value with its unit, steps, assumptions and verification | use it; read the assumptions |
| NEEDS_INPUT | a required input is missing; `uncertainty['needs']` names it | add it (with its unit) and ask again |
| AMBIGUOUS | two readings or two physical branches give different answers | say which situation is meant |
| INVALID_INPUT | a unit of the wrong kind, an unknown input name, values with no valid solution | correct the input |
| OUT_OF_SCOPE | no certified capability computes this (or the structure is not modelled yet) | see the catalogue below |
| UNRECOGNIZED_FORMAT | the wording was not recognised; a supported template is offered in `uncertainty['try']` | use the template or solve() |
| INTERNAL_ERROR | a defect of the engine, never a property of the question | report it with the question |

## Result fields

| field | meaning |
|---|---|
| value, unit | the result in SI units |
| values | every output of a procedure (x1, x2, discriminant ...) |
| outcome | one of the seven outcomes above |
| steps | each equation used, its inputs (with where each came from) and its result |
| assumptions | conditions and declared defaults the answer relies on |
| verification | residual of every equation, dimensional consistency, independent checks |
| latex() | the derivation as LaTeX |
| show() | renders in a notebook, prints in a terminal |

## Common mistakes

- Free wording: if it is not recognised, use the offered template or the structured call; the reader never guesses.
- A coefficient that violates a stated condition (a = 0 in a quadratic): INVALID_INPUT says why.
- A text value where a number is expected: INVALID_INPUT; fractions as strings ("1/2") are accepted.
- A problem outside the procedure (a cubic for the quadratic solver): OUT_OF_SCOPE, never an approximation.

## Procedures: algorithms with several outputs

Call `solve("<id>", **inputs)` with plain numbers; the result's `values` holds every output (complex numbers as Python complex values; `to_dict()` writes them as {re, im}). Each result is substituted back into its defining equation before it is returned.

| id | operation | inputs | outputs |
|---|---|---|---|
| energy_solar_declination | Solar declination on a day of the year | n, method | declination_deg, declination_rad |
| energy_equation_of_time | Equation of time | n, method | eot_min, eot_s |
| energy_solar_time_hour_angle | Solar time and hour angle from clock time | n, clock, longitude, utc_offset | eot_min, hour_angle_deg, solar_time_h |
| energy_sun_angles | Solar zenith, elevation and azimuth from latitude, declination and hour angle | latitude, declination, hour_angle | zenith_deg, elevation_deg, azimuth_deg |
| energy_solar_position | Sun position at a place and local time | latitude, longitude, n, clock, utc_offset | declination_deg, eot_min, hour_angle_deg, zenith_deg, elevation_deg, azimuth_deg |
| energy_sunrise_sunset | Sunrise, sunset, solar noon and day length | latitude, longitude, n, utc_offset | sunrise_h, sunset_h, solar_noon_h, day_length_h, sunset_hour_angle_deg |
| energy_extraterrestrial_irradiance | Extraterrestrial (top-of-atmosphere) normal irradiance | n, method, solar_constant | G_on, distance_factor |
| energy_air_mass | Optical air mass | zenith, model, pressure | AM_relative, AM_absolute |
| energy_angle_of_incidence | Angle of incidence of the sun on a tilted surface | tilt, surface_azimuth, zenith, azimuth | aoi_deg, cos_aoi |
| energy_poa_isotropic | Irradiance on a tilted plane (isotropic sky) | tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo | poa_global, poa_direct, poa_sky_diffuse, poa_ground_diffuse, aoi_deg |
| energy_poa_klucher | Irradiance on a tilted plane (Klucher sky model) | tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo | poa_global, poa_direct, poa_sky_diffuse, poa_ground_diffuse, aoi_deg |
| energy_poa_haydavies | Irradiance on a tilted plane (Hay-Davies sky model) | tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n | poa_global, poa_direct, poa_sky_diffuse, poa_ground_diffuse, aoi_deg |
| energy_poa_reindl | Irradiance on a tilted plane (Reindl sky model) | tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n | poa_global, poa_direct, poa_sky_diffuse, poa_ground_diffuse, aoi_deg |
| energy_erbs_decomposition | Diffuse and direct irradiance from global horizontal irradiance (Erbs) | ghi, zenith, n | kt, diffuse_fraction, dhi, dni |
| energy_clear_sky_haurwitz | Clear-sky global horizontal irradiance (Haurwitz) | zenith | ghi |
| energy_cell_temperature_noct | PV cell temperature from NOCT | poa, T_air, noct | T_cell, T_cell_C |
| energy_cell_temperature_faiman | PV module temperature (Faiman model) | poa, T_air, wind, u0, u1 | T_cell, T_cell_C |
| energy_cell_temperature_sapm | PV cell and module temperature (Sandia SAPM model) | poa, T_air, wind, mount | T_module_C, T_cell_C, T_cell |
| energy_pvwatts_dc | PV array DC power with temperature derating (PVWatts) | G_eff, T_cell, pdc0, gamma | pdc, pdc_kW |
| energy_pv_array_output | PV array output at operating temperature | poa, T_air, noct, pdc0, gamma | T_cell, T_cell_C, pdc, derate |
| energy_pvwatts_inverter | Inverter AC output and efficiency (PVWatts) | pdc, pdc0, eta_nom, eta_ref | pac, efficiency, clipped |
| energy_iam_ashrae | Incidence angle modifier (ASHRAE) | aoi, b0 | iam |
| energy_iam_physical | Incidence angle modifier from Fresnel reflection and absorption | aoi, n, K, L | iam |
| energy_pvwatts_losses | Combined PV system losses (PVWatts) | soiling, shading, snow, mismatch, wiring, connections, lid, nameplate_rating, age, availability | total_loss, performance_factor, total_loss_pct |
| energy_wind_power | Power in the wind and the Betz limit | rho, D, v, cp | P_wind, P_betz, power_density, area, P |
| energy_actuator_disc | Actuator-disc (momentum theory) rotor performance | a, rho, D, v | Cp, Ct, v_disc, v_wake, P, T |
| energy_wind_shear_power_law | Wind speed at another height (power law) | v1, h1, h2, alpha | v2 |
| energy_wind_shear_log | Wind speed at another height (logarithmic profile) | v1, h1, h2, z0, d | v2 |
| energy_hub_air_density | Air density at hub height | p, h_p, T, h_T, hub, model | T_hub, p_hub, rho |
| energy_power_curve_output | Turbine output from its power curve | speeds, powers, v | P_kW, P |
| energy_power_curve_density_corrected | Turbine output from a density-corrected power curve | speeds, powers, v, rho | P_kW, P |
| energy_weibull_wind | Weibull wind statistics and mean power density | k, c, rho | v_mean, power_density, energy_pattern_factor, v_most_probable, v_max_energy |
| energy_weibull_aep | Annual energy production from a power curve and a Weibull distribution | speeds, powers, k, c, hours | mean_power_kW, AEP_MWh, capacity_factor |
| energy_three_phase_power | Balanced three-phase power from line quantities | VL, IL, pf, connection, pf_type | P, Q, S, V_phase, I_phase |
| energy_per_unit_change_of_base | Per-unit impedance on a new base | R_pu, X_pu, S_old, V_old, S_new, V_new | R_pu_new, X_pu_new, Z_pu_new |
| energy_short_line | Short transmission line: sending-end voltage, regulation and efficiency | Vr, P, pf, R, X, pf_type | Vs, delta_deg, I, losses, Ps, regulation_pct, efficiency_pct |
| energy_medium_line | Medium transmission line (nominal pi) | Vr, P, pf, R, X, B, pf_type | Vs, delta_deg, Is, Ps, losses, regulation_pct, efficiency_pct, A_mag, A_angle_deg, B_mag, B_angle_deg |
| energy_long_line | Long transmission line (exact distributed parameters) | Vr, P, pf, R, X, B, pf_type | Vs, delta_deg, Is, Ps, losses, regulation_pct, efficiency_pct, A_mag, A_angle_deg, B_mag, B_angle_deg, Zc_mag, Zc_angle_deg, alpha_l, beta_l |
| energy_line_parameters | Inductance, capacitance and surge impedance of a three-phase overhead line | D_ab, D_bc, D_ca, r, f, gmr, n_bundle, spacing, V | GMR, L_mH_per_km, C_nF_per_km, X_ohm_per_km, B_uS_per_km, Zc, SIL_MW |
| energy_two_bus_load_flow | Receiving-end voltage of a line feeding a constant-power load (two-bus load flow) | Vs, R, X, P, Q | Vr, delta_deg, I, P_loss, Q_loss, voltage_drop_pct |
| energy_dc_power_flow | DC power flow | lines, P | theta_deg, flows, P_slack |
| energy_newton_load_flow | AC load flow by Newton-Raphson | lines, loads, gens, V1 | V, angle_deg, P_slack, Q_slack, P_loss, Q_gen |
| energy_economic_dispatch | Economic dispatch with quadratic costs and limits | costs, demand | lambda_, P, cost |
| energy_symmetrical_fault | Three-phase (symmetrical) fault current by the bus impedance matrix | network, fault_bus, Vf, Xf, S_base, V_base | If, X_th, V, If_A |
| energy_slg_fault | Single line-to-ground fault (symmetrical components) | X1, X2, X0, Rf, Xf, Vf | Ia, Ib, Ic, In, I1, I2, I0, Va, Vb, Vc |
| energy_ll_fault | Line-to-line fault (symmetrical components) | X1, X2, X0, Rf, Xf, Vf | Ia, Ib, Ic, In, I1, I2, I0, Va, Vb, Vc |
| energy_dlg_fault | Double line-to-ground fault (symmetrical components) | X1, X2, X0, Rf, Xf, Vf | Ia, Ib, Ic, In, I1, I2, I0, Va, Vb, Vc |
| energy_power_angle | Power transfer and power angle between two voltages | V1, V2, X, P | delta_deg, Pmax, Q_sent, Q_received, margin_pct |
| energy_critical_clearing | Critical clearing angle and time (equal-area criterion) | Pm, Pmax_pre, Pmax_fault, Pmax_post, H, f | delta0_deg, delta_max_deg, delta_cr_deg, t_cr |
| energy_droop_sharing | Frequency deviation and load sharing under governor droop | R, ratings, dP, f0, D | df, f, dP_gen, dP_load_damping |
| energy_transformer_performance | Transformer regulation and efficiency from its equivalent circuit | S, V1, V2, Req, Xeq, Rc, Xm, load, pf, pf_type | V1_required, regulation_pct, I1, pf_in, P_out, P_cu, P_core, efficiency_pct |
| energy_transformer_tests | Transformer parameters from open-circuit and short-circuit tests | Voc, Ioc, Poc, Vsc, Isc, Psc | Rc, Xm, Req, Xeq, Zeq, pf_oc, pf_sc |
| energy_transformer_max_efficiency | Load for maximum transformer efficiency | S, P_core, P_cu_fl, pf | load_fraction, S_at_max, efficiency_max_pct, efficiency_full_load_pct |
| energy_transformer_loading | Transformer under load from nameplate data | sn, V_hv, V_lv, vk, vkr, P0, i0, P, Q | V_lv_actual, V_lv_pu, delta_deg, P_loss, Q_loss, efficiency_pct, loading_pct |
| energy_parallel_transformers | Load sharing between two transformers in parallel | S1, vk1, vkr1, S2, vk2, vkr2, V_lv, P, Q | S1_share, S2_share, P1, Q1, P2, Q2, share1_pct_of_rating, share2_pct_of_rating |
| energy_induction_motor | Induction motor performance from its equivalent circuit | VL, f, poles, R1, X1, R2, X2, Xm, slip, connection, P_rot | I_line, I2, pf, P_in, P_ag, P_conv, T, P_out, efficiency_pct |
| energy_induction_max_torque | Induction motor breakdown (maximum) torque and starting torque | VL, f, poles, R1, X1, R2, X2, Xm, connection | s_maxT, T_max, T_start, I_start, V_th, R_th, X_th |
| energy_synchronous_generator | Synchronous generator internal EMF, power angle and regulation | VL, IL, pf, Xs, Ra, pf_type | E_line, E_phase, delta_deg, regulation_pct, P, Q |
| energy_peukert | Peukert exponent and run time at a new discharge current | currents, times, I | k, C_p, runtime_h, capacity_Ah |
| energy_battery_pack_sizing | Battery pack series-parallel sizing | V_target, E_target, V_cell, Q_cell | N_series, N_parallel, N_cells, V_pack, E_pack_kWh, Q_pack_Ah |
| energy_battery_rc_discharge | Battery terminal voltage under constant current (one-RC equivalent circuit) | OCV, R0, R1, C1, I, t | V_terminal, V_rc, E_delivered, E_Wh, heat, tau |
| energy_fuel_cell_stack | Hydrogen fuel-cell stack: consumption, power and efficiency | I, N, V_cell, utilization | H2_kg_s, H2_g_h, H2_fed_kg_s, O2_kg_s, H2O_kg_s, efficiency_LHV, efficiency_HHV, heat_LHV |
| energy_electrolyser | Water electrolyser: hydrogen production and energy per kilogram | I, N, V_cell, faradaic | H2_kg_s, H2_kg_h, O2_kg_s, H2O_kg_s, specific_energy_kWh_per_kg, efficiency_HHV, efficiency_LHV |
| energy_hydrogen_tank | Hydrogen stored in a tank at pressure (real gas) | V, p, T | rho, Z, mass, rho_ideal, mass_ideal, ideal_over_real, energy_LHV_kWh |
| energy_hydrogen_isothermal_compression | Minimum (isothermal) work to compress hydrogen | p1, p2, T | w, w_kWh_per_kg, w_ideal, real_over_ideal |
| energy_hydrogen_isentropic_compression | Isentropic (adiabatic) compression of hydrogen | p1, T1, p2 | T2, w, w_kWh_per_kg |
| energy_single_diode_current | PV single-diode model: current at a voltage | V, IL, I0, Rs, Rsh, nNsVth | I, P |
| energy_single_diode_voltage | PV single-diode model: voltage at a current | I, IL, I0, Rs, Rsh, nNsVth | V, P |
| energy_single_diode_mpp | PV single-diode model: short-circuit, open-circuit and maximum power point | IL, I0, Rs, Rsh, nNsVth | Isc, Voc, Imp, Vmp, Pmp, FF |
| energy_desoto_parameters | PV module single-diode parameters at operating conditions (De Soto) | G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef, dEgdT | IL, I0, Rs, Rsh, nNsVth |
| energy_pv_module_at_conditions | PV module maximum power at an irradiance and temperature | G, T_cell, alpha_sc, a_ref, IL_ref, I0_ref, Rsh_ref, Rs, EgRef, dEgdT | Isc, Voc, Imp, Vmp, Pmp, FF |
| energy_poa_perez | Irradiance on a tilted plane (Perez sky model) | tilt, surface_azimuth, zenith, azimuth, dni, ghi, dhi, albedo, n | poa_global, poa_direct, poa_sky_diffuse, poa_ground_diffuse, aoi_deg |
| energy_clear_sky_ineichen | Clear-sky irradiance (Ineichen-Perez with Linke turbidity) | zenith, linke_turbidity, altitude, n | ghi, dni, dhi, AM_absolute |
| energy_disc_dni | Direct normal irradiance from global horizontal irradiance (DISC model) | ghi, zenith, n, pressure | dni, kt, airmass |
| energy_per_unit_impedance | Base impedance and per-unit impedance | R, X, S_base, V_base | Z_base, I_base, R_pu, X_pu, Z_pu |
| energy_geometric_mean_distance | Geometric mean distance of a three-phase line | D_ab, D_bc, D_ca | GMD, ratio_to_mean |
| energy_synchronous_speed | Synchronous speed, rotor speed and slip of an AC machine | f, poles, s, n_r | n_sync_rpm, omega_sync, n_rotor_rpm, slip, slip_frequency |
| energy_stack_electrical_power | Electrical power of a fuel-cell or electrolyser stack | I, N, V_cell | P, P_kW, V_stack |
| energy_coulomb_counting | State of charge by coulomb counting | SOC0, Q, currents, dt | SOC, SOC_min, SOC_max, charge_out, charge_out_Ah |

<a id="energy_solar_declination"></a>

### Solar declination on a day of the year — `energy_solar_declination`

The declination of the sun (the latitude where the sun is overhead at noon) on day n of the year, by Cooper's equation or Spencer's Fourier series.

$$
\delta = 23.45^\circ \sin\!\left(\frac{360^\circ (284 + n)}{365}\right)
$$

| input | meaning | condition |
|---|---|---|
| `n` | day of the year (1 = 1 January) |  |
| `method` | formula: cooper (Cooper 1969, the textbook default) or spencer (Spencer 1971) |  |

| output | meaning |
|---|---|
| `declination_deg` | declination in degrees |
| `declination_rad` | declination in radians |

- **Method:** Cooper: $\delta = 23.45$ sin(360 (284 + n)/365) degrees; Spencer: a Fourier series in the day angle $B = 2$ pi $(n - 1)/365$
- **Assumes:** $1  \leq  n  \leq  366$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_solar_declination', n=172)
# -> declination_deg = 23.4498, declination_rad = 0.409276   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_solar_declination', n=1, method='spencer')
# -> declination_deg = -23.0586, declination_rad = -0.402449   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solar declination: n = 172 | SOLVED | declination_deg = 23.4498, declination_rad = 0.409276 |
| Solar declination: day = 1, method = spencer | SOLVED | declination_deg = -23.0586, declination_rad = -0.402449 |
| Solar declination: n = 400 | INVALID_INPUT | n: must be at most 366, got 400. |
| Solar declination: n = 172, latitude = 40 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: energy_systems_ai999x.templates() |
| Solar declination: | NEEDS_INPUT | energy_solar_declination needs n (day of the year (1 = 1 January)). Example: solve('energy_solar_declination', n=172). |

<a id="energy_equation_of_time"></a>

### Equation of time — `energy_equation_of_time`

The equation of time (apparent minus mean solar time) on day n, by Spencer's Fourier series or the PVCDROM formula 9.87 sin $2B - 7.53$ cos $B - 1.5$ sin B.

$$
E = 229.18\,(0.000075 + 0.001868\cos B - 0.032077\sin B - 0.014615\cos 2B - 0.040849\sin 2B)
$$

| input | meaning | condition |
|---|---|---|
| `n` | day of the year (1 = 1 January) |  |
| `method` | formula: spencer (default) or pvcdrom (B = 360 (n - 81)/365) |  |

| output | meaning |
|---|---|
| `eot_min` | equation of time in minutes |
| `eot_s` | equation of time in seconds |

- **Method:** Spencer: $E = 229.18$ (0.000075 + 0.001868 cos $B - 0.032077$ sin $B - 0.014615$ cos $2B - 0.040849$ sin 2B), $B = 2$ pi $(n - 1)/365$
- **Assumes:** $1  \leq  n  \leq  366$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_equation_of_time', n=81, method='pvcdrom')
# -> eot_min = -7.53, eot_s = -451.8   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_equation_of_time', n=1)
# -> eot_min = -2.91968, eot_s = -175.181   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Equation of time: n = 81, method = pvcdrom | SOLVED | eot_min = -7.53, eot_s = -451.8 |
| Equation of time: n = 0 | INVALID_INPUT | n: must be at least 1, got 0. |

<a id="energy_solar_time_hour_angle"></a>

### Solar time and hour angle from clock time — `energy_solar_time_hour_angle`

Local apparent solar time and the hour angle of the sun from the local clock time, the longitude and the time zone (UTC offset), with the equation of time (Spencer).

$$
\omega = 15^\circ\,(t - \Delta t_{UTC} - 12) + \lambda + \frac{E}{4}
$$

| input | meaning | condition |
|---|---|---|
| `n` | day of the year (1 = 1 January) |  |
| `clock` | local clock (standard) time in decimal hours or hh:mm |  |
| `longitude` | longitude in degrees, east positive |  |
| `utc_offset` | time zone as hours from UTC (e.g. -5, 1, 5.5) |  |

| output | meaning |
|---|---|
| `eot_min` | equation of time in minutes |
| `hour_angle_deg` | hour angle in degrees (negative before solar noon) |
| `solar_time_h` | apparent solar time in decimal hours |

- **Method:** $\omega = 15$ (t_clock - UTC offset - 12) + longitude + E/4 (degrees); solar time = 12 + omega/15
- **Assumes:** $1  \leq  n  \leq  366$; 0 <= clock < 24; -180 <= longitude <= 180
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_solar_time_hour_angle', n=172, clock=14.5, longitude=-75, utc_offset=-5)
# -> eot_min = -1.34372, hour_angle_deg = 37.1641, solar_time_h = 14.4776   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_solar_time_hour_angle', n=81, clock=12.0, longitude=15, utc_offset=1)
# -> eot_min = -7.56464, hour_angle_deg = -1.89116, solar_time_h = 11.8739   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hour angle: n = 172, clock = 14:30, longitude = -75, utc_offset = -5 | SOLVED | eot_min = -1.34372, hour_angle_deg = 37.1641, solar_time_h = 14.4776 |
| Solar time: n = 81, time = 12, longitude = 15 deg, utc = 1 | SOLVED | eot_min = -7.56464, hour_angle_deg = -1.89116, solar_time_h = 11.8739 |
| Hour angle: n = 172, clock = 25, longitude = -75, utc_offset = -5 | INVALID_INPUT | Solar time and hour angle from clock time: the clock time must be from 0 to 24 hours (decimal hours or hh:mm). |
| Hour angle: n = 172, clock = 14:30, longitude = -75 | NEEDS_INPUT | energy_solar_time_hour_angle needs utc_offset (time zone as hours from UTC (e.g. -5, 1, 5.5)). Example: solve('energy_so |

<a id="energy_sun_angles"></a>

### Solar zenith, elevation and azimuth from latitude, declination and hour angle — `energy_sun_angles`

The sun's zenith angle, elevation (altitude) and azimuth (clockwise from north) at a latitude for a given declination and hour angle.

$$
\cos\theta_z = \cos\phi\cos\delta\cos\omega + \sin\phi\sin\delta
$$

| input | meaning | condition |
|---|---|---|
| `latitude` | latitude in degrees (north positive) |  |
| `declination` | solar declination in degrees |  |
| `hour_angle` | hour angle in degrees (negative in the morning) |  |

| output | meaning |
|---|---|
| `zenith_deg` | solar zenith angle in degrees |
| `elevation_deg` | solar elevation (altitude) in degrees |
| `azimuth_deg` | solar azimuth in degrees clockwise from north |

- **Method:** cos $z = cos$(delta) cos(phi) cos(omega) + sin(delta) sin(phi); cos(azimuth - 180) = (cos z sin phi - sin delta)/(sin z cos phi), sign of omega
- **Assumes:** -90 < latitude < 90; |declination| <= 23.5; |hour angle| <= 180
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_sun_angles', latitude=40, declination=0, hour_angle=0)
# -> zenith_deg = 40, elevation_deg = 50, azimuth_deg = 180   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_sun_angles', latitude=0, declination=0, hour_angle=-90)
# -> zenith_deg = 90, elevation_deg = 0, azimuth_deg = 90   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Sun angles: latitude = 40, declination = 0, hour_angle = 0 | SOLVED | zenith_deg = 40, elevation_deg = 50, azimuth_deg = 180 |
| Solar elevation and azimuth: lat = 0 deg, delta = 0, omega = -90 | SOLVED | zenith_deg = 90, elevation_deg = 0, azimuth_deg = 90 |
| Sun angles: latitude = 40, declination = 30, hour_angle = 0 | INVALID_INPUT | Solar zenith, elevation and azimuth from latitude, declination and hour angle: the declination must lie between -23.5 an |

<a id="energy_solar_position"></a>

### Sun position at a place and local time — `energy_solar_position`

Declination, equation of time, hour angle, zenith, elevation and azimuth of the sun at a latitude and longitude on day n at a local clock time (Spencer declination and equation of time).

$$
\cos\theta_z = \cos\phi\cos\delta\cos\omega + \sin\phi\sin\delta,\quad \omega = 15^\circ(t_s - 12)
$$

| input | meaning | condition |
|---|---|---|
| `latitude` | latitude in degrees (north positive) |  |
| `longitude` | longitude in degrees, east positive |  |
| `n` | day of the year (1 = 1 January) |  |
| `clock` | local clock (standard) time in decimal hours or hh:mm |  |
| `utc_offset` | time zone as hours from UTC |  |

| output | meaning |
|---|---|
| `declination_deg` | declination in degrees |
| `eot_min` | equation of time in minutes |
| `hour_angle_deg` | hour angle in degrees |
| `zenith_deg` | solar zenith angle in degrees |
| `elevation_deg` | solar elevation in degrees |
| `azimuth_deg` | solar azimuth in degrees clockwise from north |

- **Method:** Spencer declination and equation of time; $\omega = 15$ (t - UTC offset - 12) + longitude + E/4; cos $z = cos$(delta) cos(phi) cos(omega) + sin(delta) sin(phi)
- **Assumes:** $1  \leq  n  \leq  366$; 0 <= clock < 24; -90 < latitude < 90
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The Sun's path across the sky at latitude 40°, longitude -105° (UTC-7) on three days: azimuth around the rim (N at the top) and elevation inward (rings at 0°, 30° and 60°), computed by the package every 15 minutes while the Sun is up; dots every 2 h of clock time](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-solar-position-sky.svg)

*The Sun's path across the sky at latitude 40°, longitude -105° (UTC-7) on three days: azimuth around the rim (N at the top) and elevation inward (rings at 0°, 30° and 60°), computed by the package every 15 minutes while the Sun is up; dots every 2 h of clock time. Highest computed elevations: 21 June 73.4496°, 22 March 50.2928°, 21 December 26.5781°. Marked: 21 June at 13 h, elevation 69.34°, azimuth 221.165°.*

```python
energy_systems_ai999x.solve('energy_solar_position', latitude=40, longitude=-105, n=172, clock=13.0, utc_offset=-7)
# -> declination_deg = 23.452, eot_min = -1.34372, hour_angle_deg = 14.6641, zenith_deg = 20.66, elevation_deg = 69.34, azimuth_deg = 221.165   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solar position: latitude = 40, longitude = -105, n = 172, clock = 13:00, utc_offset = -7 | SOLVED | declination_deg = 23.452, eot_min = -1.34372, hour_angle_deg = 14.6641, zenith_deg = 20.66, elevation_deg = 69.34, azimuth_deg = 221.165 |
| Solar position: latitude = 95, longitude = -105, n = 172, clock = 13:00, utc_offset = -7 | INVALID_INPUT | Sun position at a place and local time: the latitude must lie between -90 and 90 degrees. |

<a id="energy_sunrise_sunset"></a>

### Sunrise, sunset, solar noon and day length — `energy_sunrise_sunset`

Geometric sunrise and sunset times (local clock time), solar noon, the day length and the sunset hour angle at a place on day n (no refraction, sun centre on the horizon).

$$
\cos\omega_s = -\tan\phi\tan\delta,\quad N = \frac{2\,\omega_s}{15^\circ}
$$

| input | meaning | condition |
|---|---|---|
| `latitude` | latitude in degrees (north positive) |  |
| `longitude` | longitude in degrees, east positive |  |
| `n` | day of the year (1 = 1 January) |  |
| `utc_offset` | time zone as hours from UTC |  |

| output | meaning |
|---|---|
| `sunrise_h` | sunrise, local clock time in decimal hours |
| `sunset_h` | sunset, local clock time in decimal hours |
| `solar_noon_h` | solar noon, local clock time in decimal hours |
| `day_length_h` | day length in hours |
| `sunset_hour_angle_deg` | sunset hour angle omega_s in degrees |

- **Method:** omega_s = acos(-tan(phi) tan(delta)); t = (omega - longitude - E/4)/15 + 12 + UTC offset; day length = 2 $omega_s/15$
- **Assumes:** the sun rises and sets on that day (no polar day or night)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_sunrise_sunset', latitude=0, longitude=0, n=81, utc_offset=0)
# -> sunrise_h = 6.12608, sunset_h = 18.1261, solar_noon_h = 12.1261, day_length_h = 12, sunset_hour_angle_deg = 90   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_sunrise_sunset', latitude=40, longitude=0, n=172, utc_offset=0)
# -> sunrise_h = 4.59926, sunset_h = 19.4455, solar_noon_h = 12.0224, day_length_h = 14.8463, sunset_hour_angle_deg = 111.347   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Day length: latitude = 0, longitude = 0, n = 81, utc_offset = 0 | SOLVED | sunrise_h = 6.12608, sunset_h = 18.1261, solar_noon_h = 12.1261, day_length_h = 12, sunset_hour_angle_deg = 90 |
| Sunrise and sunset: latitude = 80, longitude = 0, n = 172, utc_offset = 0 | INVALID_INPUT | Sunrise, sunset, solar noon and day length: the sun does not rise or does not set on this day at this latitude (polar da |

<a id="energy_extraterrestrial_irradiance"></a>

### Extraterrestrial (top-of-atmosphere) normal irradiance — `energy_extraterrestrial_irradiance`

The solar irradiance on a plane normal to the sun's rays outside the atmosphere on day n, and the Earth-Sun distance factor $(R_{0}/R)^{2.}$

$$
G_{on} = G_{sc}\left(1 + 0.033\cos\frac{360^\circ n}{365}\right)
$$

| input | meaning | condition |
|---|---|---|
| `n` | day of the year (1 = 1 January) |  |
| `method` | formula: spencer (default, Fourier series) or asce (1 + 0.033 cos(360 n/365)) |  |
| `solar_constant` | solar constant (default 1366.1 W/m^2) |  |

| output | meaning |
|---|---|
| `G_on` | extraterrestrial normal irradiance |
| `distance_factor` | Earth-Sun distance factor $(R_{0}/R)^{2}$ |

- **Method:** $G_on = G_sc$ (1.00011 + 0.034221 cos $B + 0.00128$ sin $B + 0.000719$ cos $2B + 0.000077$ sin 2B) (Spencer) or G_sc (1 + 0.033 cos(360 n/365))
- **Assumes:** $1  \leq  n  \leq  366$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_extraterrestrial_irradiance', n=365, method='asce', solar_constant=1367.0)
# -> G_on = 1412.11 W/m^2, distance_factor = 1.033   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_extraterrestrial_irradiance', n=1)
# -> G_on = 1413.98 W/m^2, distance_factor = 1.03505   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Extraterrestrial irradiance: n = 365, method = asce, solar_constant = 1367 W/m^2 | SOLVED | G_on = 1412.11 W/m^2, distance_factor = 1.033 |
| Extraterrestrial irradiance: n = 1 | SOLVED | G_on = 1413.98 W/m^2, distance_factor = 1.03505 |
| Extraterrestrial irradiance: n = 1, solar_constant = 1367 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: energy_systems_ai999x.templates() |

<a id="energy_air_mass"></a>

### Optical air mass — `energy_air_mass`

The relative optical air mass for a solar zenith angle (Kasten-Young by default, or another named model) and the pressure-corrected absolute air mass.

$$
AM = \frac{1}{\cos z + 0.50572\,(96.07995^\circ - z)^{-1.6364}}
$$

| input | meaning | condition |
|---|---|---|
| `zenith` | solar zenith angle in degrees (apparent zenith for the refraction-based models) |  |
| `model` | air-mass model (default kastenyoung1989) |  |
| `pressure` | site pressure (default 101325 Pa) |  |

| output | meaning |
|---|---|
| `AM_relative` | relative air mass (sea level) |
| `AM_absolute` | absolute air mass AM_rel $p/101325$ Pa |

- **Method:** Kasten-Young: AM = 1/(cos $z + 0.50572$ (96.07995 - z)^-1.6364); absolute $AM = AM$ $p/p_{0}$
- **Assumes:** 0 <= zenith < 90
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_air_mass', zenith=60, model='simple')
# -> AM_relative = 2, AM_absolute = 2   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_air_mass', zenith=0, model='simple', pressure=50662.5)
# -> AM_relative = 1, AM_absolute = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Air mass: zenith = 60, model = simple | SOLVED | AM_relative = 2, AM_absolute = 2 |
| Air mass: zenith = 0, model = simple, pressure = 506.625 hPa | SOLVED | AM_relative = 1, AM_absolute = 0.5 |
| Air mass: zenith = 95 | INVALID_INPUT | Optical air mass: the zenith angle must be from 0 up to (not including) 90 degrees. |

<a id="energy_angle_of_incidence"></a>

### Angle of incidence of the sun on a tilted surface — `energy_angle_of_incidence`

The angle between the sun's rays and the normal of a surface of given tilt and azimuth, and its cosine (the beam projection factor).

$$
\cos\theta = \cos\beta\cos\theta_z + \sin\beta\sin\theta_z\cos(\gamma_s - \gamma)
$$

| input | meaning | condition |
|---|---|---|
| `tilt` | surface tilt from horizontal in degrees |  |
| `surface_azimuth` | surface azimuth in degrees clockwise from north (south = 180) |  |
| `zenith` | solar zenith angle in degrees |  |
| `azimuth` | solar azimuth in degrees clockwise from north |  |

| output | meaning |
|---|---|
| `aoi_deg` | angle of incidence in degrees |
| `cos_aoi` | cosine of the angle of incidence |

- **Method:** cos(theta) = cos(beta) cos(z) + sin(beta) sin(z) cos(azimuth_sun - azimuth_surface)
- **Assumes:** 0 <= tilt <= 180; azimuths from 0 to 360 degrees
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_angle_of_incidence', tilt=30, surface_azimuth=180, zenith=30, azimuth=180)
# -> aoi_deg = 0, cos_aoi = 1   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_angle_of_incidence', tilt=0, surface_azimuth=180, zenith=60, azimuth=100)
# -> aoi_deg = 60, cos_aoi = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Angle of incidence: tilt = 0, surface_azimuth = 180, zenith = 60, azimuth = 100 | SOLVED | aoi_deg = 60, cos_aoi = 0.5 |
| Angle of incidence: tilt = 30, surface_azimuth = 400, zenith = 60, azimuth = 100 | INVALID_INPUT | Angle of incidence of the sun on a tilted surface: the surface azimuth must lie between 0 and 360 degrees (clockwise fro |

<a id="energy_poa_isotropic"></a>

### Irradiance on a tilted plane (isotropic sky) — `energy_poa_isotropic`

Beam, sky-diffuse (isotropic sky, Liu-Jordan/Hottel-Woertz) and ground-reflected irradiance on a tilted surface from DNI, GHI and DHI.

$$
G_{poa} = G_{bn}\cos\theta + G_d\frac{1+\cos\beta}{2} + G\rho\frac{1-\cos\beta}{2}
$$

| input | meaning | condition |
|---|---|---|
| `tilt` | surface tilt from horizontal in degrees |  |
| `surface_azimuth` | surface azimuth in degrees clockwise from north (south = 180) |  |
| `zenith` | solar zenith angle in degrees |  |
| `azimuth` | solar azimuth in degrees clockwise from north |  |
| `dni` | direct normal irradiance |  |
| `ghi` | global horizontal irradiance |  |
| `dhi` | diffuse horizontal irradiance |  |
| `albedo` | ground reflectance (0 to 1) |  |

| output | meaning |
|---|---|
| `poa_global` | total irradiance on the plane |
| `poa_direct` | beam irradiance on the plane |
| `poa_sky_diffuse` | sky-diffuse irradiance on the plane |
| `poa_ground_diffuse` | ground-reflected irradiance on the plane |
| `aoi_deg` | angle of incidence in degrees |

- **Method:** G_poa = DNI cos(theta) + DHI (1 + cos beta)/2 + GHI rho (1 - cos beta)/2
- **Assumes:** sun above the horizon; DHI <= GHI; 0 <= albedo <= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_poa_isotropic', tilt=30, surface_azimuth=180, zenith=30, azimuth=180, dni=800, ghi=792.820323027551, dhi=100, albedo=0.2)
# -> poa_global = 903.923 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 93.3013 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plane of array irradiance (isotropic): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 0.2 | SOLVED | poa_global = 903.923 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 93.3013 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0 |
| Plane of array irradiance (isotropic): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 1.5 | INVALID_INPUT | Irradiance on a tilted plane (isotropic sky): the albedo must lie between 0 and 1. |
| Plane of array irradiance (isotropic): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2 | NEEDS_INPUT | energy_poa_isotropic needs albedo (ground reflectance (0 to 1)). Example: solve('energy_poa_isotropic', tilt=30, surface |

<a id="energy_poa_klucher"></a>

### Irradiance on a tilted plane (Klucher sky model) — `energy_poa_klucher`

Beam, sky-diffuse (Klucher 1979: isotropic sky brightened near the horizon and around the sun under clear skies) and ground-reflected irradiance on a tilted surface.

$$
G_{d,t} = G_d\frac{1+\cos\beta}{2}\left(1 + F\sin^3\frac{\beta}{2}\right)\left(1 + F\cos^2\theta\sin^3\theta_z\right)
$$

| input | meaning | condition |
|---|---|---|
| `tilt` | surface tilt from horizontal in degrees |  |
| `surface_azimuth` | surface azimuth in degrees clockwise from north (south = 180) |  |
| `zenith` | solar zenith angle in degrees |  |
| `azimuth` | solar azimuth in degrees clockwise from north |  |
| `dni` | direct normal irradiance |  |
| `ghi` | global horizontal irradiance |  |
| `dhi` | diffuse horizontal irradiance |  |
| `albedo` | ground reflectance (0 to 1) |  |

| output | meaning |
|---|---|
| `poa_global` | total irradiance on the plane |
| `poa_direct` | beam irradiance on the plane |
| `poa_sky_diffuse` | sky-diffuse irradiance on the plane |
| `poa_ground_diffuse` | ground-reflected irradiance on the plane |
| `aoi_deg` | angle of incidence in degrees |

- **Method:** sky = DHI (1 + cos beta)/2 (1 + F $sin^{3}$(beta/2)) (1 + F $cos^{2}$(theta) $sin^{3}$(z)), F = 1 - (DHI/GHI)^2
- **Assumes:** sun above the horizon; DHI <= GHI; 0 <= albedo <= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_poa_klucher', tilt=30, surface_azimuth=180, zenith=30, azimuth=180, dni=800, ghi=792.820323027551, dhi=100, albedo=0.2)
# -> poa_global = 917.188 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 106.566 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plane of array irradiance (Klucher): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 0.2 | SOLVED | poa_global = 917.188 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 106.566 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0 |
| Plane of array irradiance (Klucher): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 1.5 | INVALID_INPUT | Irradiance on a tilted plane (Klucher sky model): the albedo must lie between 0 and 1. |
| Plane of array irradiance (Klucher): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2 | NEEDS_INPUT | energy_poa_klucher needs albedo (ground reflectance (0 to 1)). Example: solve('energy_poa_klucher', tilt=30, surface_azi |

<a id="energy_poa_haydavies"></a>

### Irradiance on a tilted plane (Hay-Davies sky model) — `energy_poa_haydavies`

Beam, sky-diffuse (Hay and Davies 1980: isotropic plus circumsolar weighted by the anisotropy index DNI/G_on) and ground-reflected irradiance on a tilted surface; G_on from day n (Spencer, 1366.1 W/m^2).

$$
G_{d,t} = G_d\left[A R_b + (1 - A)\frac{1+\cos\beta}{2}\right],\ A = \frac{G_{bn}}{G_{on}}
$$

| input | meaning | condition |
|---|---|---|
| `tilt` | surface tilt from horizontal in degrees |  |
| `surface_azimuth` | surface azimuth in degrees clockwise from north (south = 180) |  |
| `zenith` | solar zenith angle in degrees |  |
| `azimuth` | solar azimuth in degrees clockwise from north |  |
| `dni` | direct normal irradiance |  |
| `ghi` | global horizontal irradiance |  |
| `dhi` | diffuse horizontal irradiance |  |
| `albedo` | ground reflectance (0 to 1) |  |
| `n` | day of the year (1 = 1 January) |  |

| output | meaning |
|---|---|
| `poa_global` | total irradiance on the plane |
| `poa_direct` | beam irradiance on the plane |
| `poa_sky_diffuse` | sky-diffuse irradiance on the plane |
| `poa_ground_diffuse` | ground-reflected irradiance on the plane |
| `aoi_deg` | angle of incidence in degrees |

- **Method:** sky = DHI [A $R_b + (1 - A)$(1 + cos beta)/2], A = DNI/G_on, $R_{b} = \frac{\cos{\left(\theta \right)}}{\cos{\left(z \right)}}$
- **Assumes:** sun above the horizon; DHI <= GHI; 0 <= albedo <= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_poa_haydavies', tilt=30, surface_azimuth=180, zenith=30, azimuth=180, dni=800, ghi=792.820323027551, dhi=100, albedo=0.2, n=172)
# -> poa_global = 917.342 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 106.72 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plane of array irradiance (Hay-Davies): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 0.2, n = 172 | SOLVED | poa_global = 917.342 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 106.72 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0 |
| Plane of array irradiance (Hay-Davies): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 1.5, n = 172 | INVALID_INPUT | Irradiance on a tilted plane (Hay-Davies sky model): the albedo must lie between 0 and 1. |
| Plane of array irradiance (Hay-Davies): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, n = 172 | NEEDS_INPUT | energy_poa_haydavies needs albedo (ground reflectance (0 to 1)). Example: solve('energy_poa_haydavies', tilt=30, surface |

<a id="energy_poa_reindl"></a>

### Irradiance on a tilted plane (Reindl sky model) — `energy_poa_reindl`

Beam, sky-diffuse (Reindl 1990: Hay-Davies plus horizon brightening sqrt(BHI/GHI) $sin^{3}$(beta/2)) and ground-reflected irradiance on a tilted surface; G_on from day n (Spencer).

$$
G_{d,t} = G_d\left[(1-A)\frac{1+\cos\beta}{2}\left(1+\sqrt{\frac{G_b}{G}}\sin^3\frac{\beta}{2}\right) + A R_b\right]
$$

| input | meaning | condition |
|---|---|---|
| `tilt` | surface tilt from horizontal in degrees |  |
| `surface_azimuth` | surface azimuth in degrees clockwise from north (south = 180) |  |
| `zenith` | solar zenith angle in degrees |  |
| `azimuth` | solar azimuth in degrees clockwise from north |  |
| `dni` | direct normal irradiance |  |
| `ghi` | global horizontal irradiance |  |
| `dhi` | diffuse horizontal irradiance |  |
| `albedo` | ground reflectance (0 to 1) |  |
| `n` | day of the year (1 = 1 January) |  |

| output | meaning |
|---|---|
| `poa_global` | total irradiance on the plane |
| `poa_direct` | beam irradiance on the plane |
| `poa_sky_diffuse` | sky-diffuse irradiance on the plane |
| `poa_ground_diffuse` | ground-reflected irradiance on the plane |
| `aoi_deg` | angle of incidence in degrees |

- **Method:** sky = DHI [$(1 - A)$(1 + cos beta)/2 (1 + sqrt(BHI/GHI) $sin^{3}$(beta/2)) + A R_b]
- **Assumes:** sun above the horizon; DHI <= GHI; 0 <= albedo <= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_poa_reindl', tilt=30, surface_azimuth=180, zenith=30, azimuth=180, dni=800, ghi=792.820323027551, dhi=100, albedo=0.2, n=172)
# -> poa_global = 917.939 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 107.317 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plane of array irradiance (Reindl): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 0.2, n = 172 | SOLVED | poa_global = 917.939 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 107.317 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0 |
| Plane of array irradiance (Reindl): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 1.5, n = 172 | INVALID_INPUT | Irradiance on a tilted plane (Reindl sky model): the albedo must lie between 0 and 1. |
| Plane of array irradiance (Reindl): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, n = 172 | NEEDS_INPUT | energy_poa_reindl needs albedo (ground reflectance (0 to 1)). Example: solve('energy_poa_reindl', tilt=30, surface_azimu |

<a id="energy_erbs_decomposition"></a>

### Diffuse and direct irradiance from global horizontal irradiance (Erbs) — `energy_erbs_decomposition`

The clearness index, diffuse fraction, diffuse horizontal and direct normal irradiance estimated from a measured global horizontal irradiance by the Erbs (1982) correlation.

$$
\frac{G_d}{G} = 0.9511 - 0.1604k_t + 4.388k_t^2 - 16.638k_t^3 + 12.336k_t^4
$$

| input | meaning | condition |
|---|---|---|
| `ghi` | global horizontal irradiance |  |
| `zenith` | solar zenith angle in degrees |  |
| `n` | day of the year (1 = 1 January) |  |

| output | meaning |
|---|---|
| `kt` | clearness index GHI/(G_on cos z) |
| `diffuse_fraction` | diffuse fraction DHI/GHI |
| `dhi` | diffuse horizontal irradiance |
| `dni` | direct normal irradiance |

- **Method:** kt = GHI/(G_on cos z); DHI/GHI = 1 - 0.09 kt $(kt  \leq  0.22)$, $0.9511 - 0.1604$ $kt + 4.388$ $kt^{2} - 16.638$ $kt^{3} + 12.336$ $kt^{4}$ (<= 0.8), 0.165; DNI = (GHI - DHI)/cos z
- **Assumes:** 0 <= zenith <= 85; GHI > 0; kt is capped at 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_erbs_decomposition', ghi=528.6494370285654, zenith=60, n=172)
# -> kt = 0.8, diffuse_fraction = 0.16527, dhi = 87.3697 W/m^2, dni = 882.56 W/m^2   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_erbs_decomposition', ghi=141.3981805, zenith=0, n=1)
# -> kt = 0.1, diffuse_fraction = 0.991, dhi = 140.126 W/m^2, dni = 1.27258 W/m^2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Erbs decomposition: ghi = 141.3981805 W/m^2, zenith = 0, n = 1 | SOLVED | kt = 0.1, diffuse_fraction = 0.991, dhi = 140.126 W/m^2, dni = 1.27258 W/m^2 |
| Erbs decomposition: ghi = 600 W/m^2, zenith = 88, n = 172 | INVALID_INPUT | Diffuse and direct irradiance from global horizontal irradiance (Erbs): the zenith angle must lie between 0 and 85 degre |

<a id="energy_clear_sky_haurwitz"></a>

### Clear-sky global horizontal irradiance (Haurwitz) — `energy_clear_sky_haurwitz`

The global horizontal irradiance under a clear sky from the solar zenith angle alone (Haurwitz 1945): 1098 cos z exp($-0.059/cos$ z).

$$
G_{cs} = 1098\cos z\; e^{-0.059/\cos z}
$$

| input | meaning | condition |
|---|---|---|
| `zenith` | apparent solar zenith angle in degrees |  |

| output | meaning |
|---|---|
| `ghi` | clear-sky global horizontal irradiance (0 when the sun is below the horizon) |

- **Method:** GHI = 1098 cos z exp($-0.059/cos$ z) for cos $z > 0$, else 0
- **Assumes:** 0 <= zenith <= 180
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_clear_sky_haurwitz', zenith=0)
# -> ghi = 1035.09 W/m^2   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_clear_sky_haurwitz', zenith=95)
# -> ghi = 0 W/m^2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Clear-sky GHI (Haurwitz): zenith = 0 | SOLVED | ghi = 1035.09 W/m^2 |
| Haurwitz clear sky: zenith = 200 | INVALID_INPUT | Clear-sky global horizontal irradiance (Haurwitz): the zenith angle must lie between 0 and 180 degrees. |

<a id="energy_cell_temperature_noct"></a>

### PV cell temperature from NOCT — `energy_cell_temperature_noct`

The operating temperature of a PV cell from the plane-of-array irradiance, the air temperature and the module's nominal operating cell temperature (NOCT): T_c = T_a + (NOCT - 20 degC) $G/800.$

$$
T_c = T_a + \frac{NOCT - 20^\circ\mathrm{C}}{800\ \mathrm{W/m^2}}\,G
$$

| input | meaning | condition |
|---|---|---|
| `poa` | plane-of-array irradiance |  |
| `T_air` | ambient air temperature |  |
| `noct` | nominal operating cell temperature (at 800 $W/m^{2}$, 20 degC, 1 m/s) |  |

| output | meaning |
|---|---|
| `T_cell` | cell temperature |
| `T_cell_C` | cell temperature in degC |

- **Method:** T_c = T_a + (NOCT - 20)/800 G (Ross model with the NOCT coefficient)
- **Assumes:** irradiance >= 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_cell_temperature_noct', poa=800, T_air=298.15, noct=318.15)
# -> T_cell = 323.15 K, T_cell_C = 50   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_cell_temperature_noct', poa=1000, T_air=293.15, noct=320.15)
# -> T_cell = 326.9 K, T_cell_C = 53.75   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cell temperature (NOCT): poa = 800 W/m^2, T_air = 25 degC, noct = 45 degC | SOLVED | T_cell = 323.15 K, T_cell_C = 50 |
| Cell temperature (NOCT): poa = -800 W/m^2, T_air = 25 degC, noct = 45 degC | INVALID_INPUT | PV cell temperature from NOCT: the plane-of-array irradiance must be zero or positive. |

<a id="energy_cell_temperature_faiman"></a>

### PV module temperature (Faiman model) — `energy_cell_temperature_faiman`

Module (cell) temperature by the Faiman heat-loss model used in IEC 61853: T = T_a + G/(u0 + u1 v).

$$
T_m = T_a + \frac{G}{u_0 + u_1 v}
$$

| input | meaning | condition |
|---|---|---|
| `poa` | plane-of-array irradiance |  |
| `T_air` | ambient air temperature |  |
| `wind` | wind speed at module height |  |
| `u0` | constant heat-loss coefficient in W/(m^2 K) (default 25) |  |
| `u1` | wind heat-loss coefficient in W s/(m^3 K) (default 6.84) |  |

| output | meaning |
|---|---|
| `T_cell` | module temperature |
| `T_cell_C` | module temperature in degC |

- **Method:** T = T_a + G/(u0 + u1 v)
- **Assumes:** $G  \geq  0$; $u_{0} > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_cell_temperature_faiman', poa=1000, T_air=298.15, wind=0)
# -> T_cell = 338.15 K, T_cell_C = 65   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_cell_temperature_faiman', poa=800, T_air=293.15, wind=1, u0=30, u1=10)
# -> T_cell = 313.15 K, T_cell_C = 40   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Module temperature (Faiman): poa = 1000 W/m^2, T_air = 25 degC, wind = 0 m/s | SOLVED | T_cell = 338.15 K, T_cell_C = 65 |
| Faiman temperature: poa = 800 W/m^2, T_air = 20 degC, wind = 1 m/s, u0 = 30, u1 = 10 | SOLVED | T_cell = 313.15 K, T_cell_C = 40 |
| Module temperature (Faiman): poa = 1000 W/m^2, T_air = 25 degC | NEEDS_INPUT | energy_cell_temperature_faiman needs wind (wind speed at module height). Example: solve('energy_cell_temperature_faiman' |

<a id="energy_cell_temperature_sapm"></a>

### PV cell and module temperature (Sandia SAPM model) — `energy_cell_temperature_sapm`

Back-of-module and cell temperature by the Sandia Array Performance Model, $T_m = G$ exp(a + b v) + T_a, $T_c = T_m + (G/1000)$ dT, for the standard module/mounting parameter sets.

$$
T_m = G\,e^{a + b v} + T_a,\quad T_c = T_m + \frac{G}{1000}\Delta T
$$

| input | meaning | condition |
|---|---|---|
| `poa` | plane-of-array irradiance |  |
| `T_air` | ambient air temperature |  |
| `wind` | wind speed at 10 m |  |
| `mount` | module construction and mounting (Sandia parameter set) |  |

| output | meaning |
|---|---|
| `T_module_C` | back-of-module temperature in degC |
| `T_cell_C` | cell temperature in degC |
| `T_cell` | cell temperature |

- **Method:** $T_m = G$ exp(a + b v) + T_a; $T_c = T_m + G$ $dT/1000$ (a, b, dT from the Sandia table)
- **Assumes:** $G  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_cell_temperature_sapm', poa=1000, T_air=283.15, wind=0, mount='open_rack_glass_glass')
# -> T_module_C = 41.117, T_cell_C = 44.117, T_cell = 317.267 K   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cell temperature (SAPM): poa = 1000 W/m^2, T_air = 10 degC, wind = 0 m/s, mount = open_rack_glass_glass | SOLVED | T_module_C = 41.117, T_cell_C = 44.117, T_cell = 317.267 K |
| Cell temperature (SAPM): poa = 1000 W/m^2, T_air = 10 degC, wind = 0 m/s, mount = on_the_moon | INVALID_INPUT | mount: one of open_rack_glass_glass, close_mount_glass_glass, open_rack_glass_polymer, insulated_back_glass_polymer; got |

<a id="energy_pvwatts_dc"></a>

### PV array DC power with temperature derating (PVWatts) — `energy_pvwatts_dc`

DC power of a PV array from the effective irradiance and cell temperature (PVWatts): $P = (G/1000)$ P_dc0 (1 + gamma (T_c - 25 degC)).

$$
P_{dc} = \frac{G}{1000}P_{dc0}\left[1 + \gamma(T_c - 25^\circ\mathrm{C})\right]
$$

| input | meaning | condition |
|---|---|---|
| `G_eff` | effective (transmitted) irradiance on the cells |  |
| `T_cell` | cell temperature |  |
| `pdc0` | DC rating at 1000 $W/m^{2}$ and 25 degC |  |
| `gamma` | power temperature coefficient per K (e.g. -0.004, $or -0.4$%) |  |

| output | meaning |
|---|---|
| `pdc` | DC power |
| `pdc_kW` | DC power in kW |

- **Method:** $P_dc = (G/1000)$ P_dc0 [$1 + \gamma$ $(T_c - 25)$]
- **Assumes:** $-0.02  \leq  \gamma  \leq  0.01$ per K
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_pvwatts_dc', G_eff=800, T_cell=318.15, pdc0=5000, gamma=-0.004)
# -> pdc = 3680 W, pdc_kW = 3.68   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_pvwatts_dc', G_eff=1000, T_cell=298.15, pdc0=300, gamma=-0.0035)
# -> pdc = 300 W, pdc_kW = 0.3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PVWatts DC power: G_eff = 800 W/m^2, T_cell = 45 degC, pdc0 = 5 kW, gamma = -0.4% | SOLVED | pdc = 3680 W, pdc_kW = 3.68 |
| PVWatts DC power: G_eff = 800 W/m^2, T_cell = 45 degC, pdc0 = 5 kW, gamma = -0.4 | INVALID_INPUT | PV array DC power with temperature derating (PVWatts): the temperature coefficient gamma must lie between -0.02 and 0.01 |

<a id="energy_pv_array_output"></a>

### PV array output at operating temperature — `energy_pv_array_output`

Cell temperature from NOCT and the resulting DC output of an array with a power temperature coefficient, from the plane-of-array irradiance and the air temperature.

$$
P = \frac{G}{1000}P_{dc0}\left[1 + \gamma\left(T_a + \frac{NOCT-20}{800}G - 25\right)\right]
$$

| input | meaning | condition |
|---|---|---|
| `poa` | plane-of-array irradiance |  |
| `T_air` | ambient air temperature |  |
| `noct` | nominal operating cell temperature |  |
| `pdc0` | array DC rating at STC |  |
| `gamma` | power temperature coefficient per K (e.g. -0.004, $or -0.4$%) |  |

| output | meaning |
|---|---|
| `T_cell` | cell temperature |
| `T_cell_C` | cell temperature in degC |
| `pdc` | DC output |
| `derate` | temperature derating factor $1 + \gamma$ $(T_c - 25)$ |

- **Method:** T_c = T_a + (NOCT - 20) $G/800$; $P = (G/1000)$ P_dc0 [$1 + \gamma$ $(T_c - 25)$]
- **Assumes:** $G > 0$; $-0.02  \leq  \gamma  \leq  0.01$ per K
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_pv_array_output', poa=800, T_air=298.15, noct=318.15, pdc0=10000, gamma=-0.004)
# -> T_cell = 323.15 K, T_cell_C = 50, pdc = 7200 W, derate = 0.9   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PV array output: poa = 800 W/m^2, T_air = 25 degC, noct = 45 degC, pdc0 = 10 kW, gamma = -0.4% | SOLVED | T_cell = 323.15 K, T_cell_C = 50, pdc = 7200 W, derate = 0.9 |
| PV array output: poa = 0 W/m^2, T_air = 25 degC, noct = 45 degC, pdc0 = 10 kW, gamma = -0.4% | INVALID_INPUT | PV array output at operating temperature: the plane-of-array irradiance must be a positive number. |

<a id="energy_pvwatts_inverter"></a>

### Inverter AC output and efficiency (PVWatts) — `energy_pvwatts_inverter`

AC output and efficiency of an inverter at a DC input by the PVWatts part-load efficiency curve, with clipping at the AC rating eta_nom P_dc0.

$$
\eta = \frac{\eta_{nom}}{\eta_{ref}}\left(-0.0162\zeta - \frac{0.0059}{\zeta} + 0.9858\right),\ \zeta = \frac{P_{dc}}{P_{dc0}}
$$

| input | meaning | condition |
|---|---|---|
| `pdc` | DC input power |  |
| `pdc0` | inverter DC input rating |  |
| `eta_nom` | nominal efficiency (default 0.96) |  |
| `eta_ref` | reference efficiency of the curve (default 0.9637) |  |

| output | meaning |
|---|---|
| `pac` | AC output power |
| `efficiency` | conversion efficiency P_ac/P_dc |
| `clipped` | true when the output is limited at the AC rating |

- **Method:** $\zeta = P_dc/P_dc0$; eta = (eta_nom/eta_ref)(-0.0162 zeta - 0.0059/zeta + 0.9858); P_ac = min(eta P_dc, eta_nom P_dc0)
- **Assumes:** $P_dc > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_pvwatts_inverter', pdc=5000, pdc0=5000)
# -> pac = 4800 W, efficiency = 0.96, clipped = True   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_pvwatts_inverter', pdc=6000, pdc0=5000)
# -> pac = 4800 W, efficiency = 0.8, clipped = True   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Inverter output (PVWatts): pdc = 6 kW, pdc0 = 5 kW | SOLVED | pac = 4800 W, efficiency = 0.8, clipped = True |
| Inverter output (PVWatts): pdc = 0 kW, pdc0 = 5 kW | INVALID_INPUT | Inverter AC output and efficiency (PVWatts): the DC input power must be a positive number. |

<a id="energy_iam_ashrae"></a>

### Incidence angle modifier (ASHRAE) — `energy_iam_ashrae`

The fraction of beam irradiance transmitted into a PV module at an angle of incidence, by the ASHRAE model IAM = 1 - b0 (1/cos(theta) - 1), floored at 0.

$$
IAM = 1 - b_0\left(\frac{1}{\cos\theta} - 1\right)
$$

| input | meaning | condition |
|---|---|---|
| `aoi` | angle of incidence in degrees |  |
| `b0` | ASHRAE coefficient (default 0.05) |  |

| output | meaning |
|---|---|
| `iam` | incidence angle modifier |

- **Method:** IAM = max(0, $1 - b_{0}$ (sec(theta) - 1)); 0 for $\theta  \geq  90$
- **Assumes:** 0 <= aoi <= 180
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_iam_ashrae', aoi=60)
# -> iam = 0.95   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_iam_ashrae', aoi=0)
# -> iam = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Incidence angle modifier (ASHRAE): aoi = 60 | SOLVED | iam = 0.95 |
| ASHRAE IAM: aoi = 60, b0 = 2 | INVALID_INPUT | Incidence angle modifier (ASHRAE): b0 must lie between 0 and 1. |

<a id="energy_iam_physical"></a>

### Incidence angle modifier from Fresnel reflection and absorption — `energy_iam_physical`

The incidence angle modifier of a glass cover from Fresnel reflection (both polarizations) and absorption in the glass (De Soto / physical model), normalised to normal incidence.

$$
IAM = \frac{\tfrac12\left[(1-r_s)+(1-r_p)\right]e^{-KL/\cos\theta_r}}{(1-r_0)e^{-KL}}
$$

| input | meaning | condition |
|---|---|---|
| `aoi` | angle of incidence in degrees |  |
| `n` | refractive index of the glass (default 1.526) |  |
| `K` | glazing extinction coefficient in $1/m$ (default 4) |  |
| `L` | glazing thickness (default 2 mm) |  |

| output | meaning |
|---|---|
| `iam` | incidence angle modifier |

- **Method:** Snell's law, Fresnel reflectances r_s, r_p, transmission $\tau = (1 - r)$ exp(-K L/cos theta_r), IAM = (tau_s + tau_p)/2/tau(0)
- **Assumes:** 0 <= aoi < 90; $1 < n < 3$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_iam_physical', aoi=0)
# -> iam = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Incidence angle modifier (physical): aoi = 0 | SOLVED | iam = 1 |
| Incidence angle modifier (physical): aoi = 95 | INVALID_INPUT | Incidence angle modifier from Fresnel reflection and absorption: the angle of incidence must lie between 0 and 90 degree |

<a id="energy_pvwatts_losses"></a>

### Combined PV system losses (PVWatts) — `energy_pvwatts_losses`

The total system loss of a PV system from its individual loss percentages, combined multiplicatively as in PVWatts: 1 - prod(1 - L_i); unstated losses take the PVWatts defaults (soiling 2%, shading 3%, snow 0%, mismatch 2%, wiring 2%, connections 0.5%, light-induced degradation 1.5%, nameplate 1%, age 0%, availability 3%).

$$
L = 1 - \prod_i (1 - L_i)
$$

| input | meaning | condition |
|---|---|---|
| `soiling` | soiling loss as a fraction or a percentage (default 2.0%) |  |
| `shading` | shading loss as a fraction or a percentage (default 3.0%) |  |
| `snow` | snow loss as a fraction or a percentage (default 0.0%) |  |
| `mismatch` | mismatch loss as a fraction or a percentage (default 2.0%) |  |
| `wiring` | wiring loss as a fraction or a percentage (default 2.0%) |  |
| `connections` | connections loss as a fraction or a percentage (default 0.5%) |  |
| `lid` | lid loss as a fraction or a percentage (default 1.5%) |  |
| `nameplate_rating` | nameplate rating loss as a fraction or a percentage (default 1.0%) |  |
| `age` | age loss as a fraction or a percentage (default 0.0%) |  |
| `availability` | availability loss as a fraction or a percentage (default 3.0%) |  |

| output | meaning |
|---|---|
| `total_loss` | total loss as a fraction |
| `performance_factor` | product of (1 - loss) |
| `total_loss_pct` | total loss in percent |

- **Method:** L = 1 - prod(1 - L_i)
- **Assumes:** each loss between 0 and 100%
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_pvwatts_losses', )
# -> total_loss = 0.140757, performance_factor = 0.859243, total_loss_pct = 14.0757   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_pvwatts_losses', soiling=0.0, shading=0.0, snow=0.0, mismatch=0.0, wiring=0.0, connections=0.0, lid=0.0, nameplate_rating=0.0, age=0.0, availability=0.0)
# -> total_loss = 0, performance_factor = 1, total_loss_pct = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PVWatts losses: soiling = 0%, shading = 0%, snow = 0%, mismatch = 0%, wiring = 0%, connections = 0%, lid = 0%, nameplate_rating = 0%, age = 0%, availability = 10% | SOLVED | total_loss = 0.1, performance_factor = 0.9, total_loss_pct = 10 |
| PV system losses: soiling = 120% | INVALID_INPUT | Combined PV system losses (PVWatts): the soiling loss must lie between 0 and 100%. |

<a id="energy_wind_power"></a>

### Power in the wind and the Betz limit — `energy_wind_power`

The kinetic power of the wind through a rotor's swept area, the maximum a turbine can extract (Betz limit 16/27), the power density, and the power at a given power coefficient.

$$
P = \tfrac12\rho A v^3,\quad P_{max} = \tfrac{16}{27}\cdot\tfrac12\rho A v^3
$$

| input | meaning | condition |
|---|---|---|
| `rho` | air density |  |
| `D` | rotor diameter |  |
| `v` | wind speed |  |
| `cp` | power coefficient (optional, at most 16/27) |  |

| output | meaning |
|---|---|
| `P_wind` | power in the wind through the swept area |
| `P_betz` | maximum extractable power (Betz) |
| `power_density` | wind power density in $W/m^{2}$ |
| `area` | swept area |
| `P` | power at the given power coefficient |

- **Method:** $P = (1/2)$ rho A $v^{3}$, $A = pi$ $D^{2}/4$; P_Betz = (16/27) P; P(cp) = cp P
- **Assumes:** rho, D, $v > 0$; $0 < cp  \leq  16/27$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_wind_power', rho=1.2, D=1.1283791670955126, v=10)
# -> P_wind = 600 W, P_betz = 355.556 W, power_density = 600, area = 1 m^2   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_wind_power', rho=1.225, D=80, v=10, cp=0.4)
# -> P_wind = 3.07876e+06 W, P_betz = 1.82445e+06 W, power_density = 612.5, area = 5026.55 m^2, P = 1.2315e+06 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Power in the wind: rho = 1.225 kg/m^3, D = 80 m, v = 10 m/s, cp = 0.4 | SOLVED | P_wind = 3.07876e+06 W, P_betz = 1.82445e+06 W, power_density = 612.5, area = 5026.55 m^2, P = 1.2315e+06 W |
| Wind power: rho = 1.2 kg/m^3, D = 100 m, v = 12 m/s, cp = 0.7 | INVALID_INPUT | Power in the wind and the Betz limit: the power coefficient must lie between 0 and the Betz limit 16/27 = 0.593. |
| Wind power: rho = 1.2 kg/m^3, D = 100 m | NEEDS_INPUT | energy_wind_power needs v (wind speed). Example: solve('energy_wind_power', rho='1.2 kg/m^3', D='1.12838 m', v='10 m/s') |

<a id="energy_actuator_disc"></a>

### Actuator-disc (momentum theory) rotor performance — `energy_actuator_disc`

Power and thrust coefficients, disc and far-wake speeds, power and thrust of an ideal rotor (actuator disc) with axial induction factor a; $a = 1/3$ gives the Betz optimum $Cp = 16/27.$

$$
C_P = 4a(1-a)^2,\quad C_T = 4a(1-a)
$$

| input | meaning | condition |
|---|---|---|
| `a` | axial induction factor $(0  \leq  a < 0.5)$ |  |
| `rho` | air density |  |
| `D` | rotor diameter |  |
| `v` | free-stream wind speed |  |

| output | meaning |
|---|---|
| `Cp` | power coefficient 4a(1-a)^2 |
| `Ct` | thrust coefficient 4a(1-a) |
| `v_disc` | speed at the disc |
| `v_wake` | far-wake speed |
| `P` | power extracted |
| `T` | thrust on the rotor |

- **Method:** $Cp = 4a$(1 - a)^2, $Ct = 4a$(1 - a), v_disc = v(1 - a), v_wake = v(1 - 2a); $P = Cp$ $(1/2)$ rho A $v^{3}$, $T = Ct$ $(1/2)$ rho A $v^{2}$
- **Assumes:** $0  \leq  a < 0.5$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_actuator_disc', a=0.3333333333333333, rho=1.2, D=10, v=8)
# -> Cp = 0.592593, Ct = 0.888889, v_disc = 5.33333 m/s, v_wake = 2.66667 m/s, P = 14297.7 W, T = 2680.83 N   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_actuator_disc', a=0.25, rho=1.2, D=10, v=8)
# -> Cp = 0.5625, Ct = 0.75, v_disc = 6 m/s, v_wake = 4 m/s, P = 13571.7 W, T = 2261.95 N   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Actuator disc: a = 0.25, rho = 1.2 kg/m^3, D = 10 m, v = 8 m/s | SOLVED | Cp = 0.5625, Ct = 0.75, v_disc = 6 m/s, v_wake = 4 m/s, P = 13571.7 W, T = 2261.95 N |
| Actuator disc: a = 0.6, rho = 1.2 kg/m^3, D = 10 m, v = 8 m/s | INVALID_INPUT | Actuator-disc (momentum theory) rotor performance: the axial induction factor a must lie in [0, 0.5) (momentum theory fa |

<a id="energy_wind_shear_power_law"></a>

### Wind speed at another height (power law) — `energy_wind_shear_power_law`

The wind speed at height h2 from a measurement at h1 by the power (Hellman) law $v_{2} = v_{1}$ $(h_{2}/h_{1})^{\alpha}$; $\alpha = 1/7$ when not stated (the one-seventh law).

$$
v_2 = v_1\left(\frac{h_2}{h_1}\right)^{\alpha}
$$

| input | meaning | condition |
|---|---|---|
| `v1` | measured wind speed |  |
| `h1` | measurement height |  |
| `h2` | target height (e.g. hub height) |  |
| `alpha` | shear exponent (default 1/7) |  |

| output | meaning |
|---|---|
| `v2` | wind speed at h2 |

- **Method:** $v_{2} = v_{1}$ $(h_{2}/h_{1})^{\alpha}$
- **Assumes:** heights > 0; $0  \leq  \alpha  \leq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_wind_shear_power_law', v1=5, h1=10, h2=80, alpha=0.3333333333333333)
# -> v2 = 10 m/s   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_wind_shear_power_law', v1=6, h1=10, h2=1280)
# -> v2 = 12 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Wind shear (power law): v1 = 6 m/s, h1 = 10 m, h2 = 1280 m | SOLVED | v2 = 12 m/s |
| Wind shear (power law): v1 = 5 m/s, h1 = 10 m, h2 = 80 m, alpha = 0.3333333333333333 | SOLVED | v2 = 10 m/s |
| Wind shear (power law): v1 = 5 m/s, h1 = 0 m, h2 = 80 m | INVALID_INPUT | Wind speed at another height (power law): the wind speed and the heights must be a positive number. |

<a id="energy_wind_shear_log"></a>

### Wind speed at another height (logarithmic profile) — `energy_wind_shear_log`

The wind speed at height h2 from a measurement at h1 by the logarithmic wind profile with roughness length z0 and an optional obstacle height d (displacement 0.7 d).

$$
v_2 = v_1\,\frac{\ln\left((h_2 - 0.7d)/z_0\right)}{\ln\left((h_1 - 0.7d)/z_0\right)}
$$

| input | meaning | condition |
|---|---|---|
| `v1` | measured wind speed |  |
| `h1` | measurement height |  |
| `h2` | target height |  |
| `z0` | surface roughness length |  |
| `d` | obstacle height (default 0; displacement height 0.7 d) |  |

| output | meaning |
|---|---|
| `v2` | wind speed at h2 |

- **Method:** $v_{2} = v_{1}$ ln((h2 - 0.7 d)/z0)/ln((h1 - 0.7 d)/z0)
- **Assumes:** $h - 0.7$ $d > z_{0}$ at both heights
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Wind speed at nine heights extrapolated from 5 m/s measured at 10 m, drawn as arrows whose length is the speed: logarithmic law (roughness length 1 m) and power law (exponent 0](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-wind-shear-log-profile.svg)

*Wind speed at nine heights extrapolated from 5 m/s measured at 10 m, drawn as arrows whose length is the speed: logarithmic law (roughness length 1 m) and power law (exponent 0.333333), each computed by the package. At 100 m: 10 and 10.7722 m/s; at 150 m: 10.8805 and 12.3311 m/s.*

```python
energy_systems_ai999x.solve('energy_wind_shear_log', v1=5, h1=10, h2=100, z0=1)
# -> v2 = 10 m/s   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_wind_shear_log', v1=4, h1=9.718281828459045, h2=27.085536923187664, z0=1, d=10)
# -> v2 = 12 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Wind shear (log profile): v1 = 5 m/s, h1 = 10 m, h2 = 100 m, z0 = 1 m | SOLVED | v2 = 10 m/s |
| Wind shear (log profile): v1 = 5 m/s, h1 = 10 m, h2 = 100 m, z0 = 20 m | INVALID_INPUT | Wind speed at another height (logarithmic profile): both heights must exceed the displacement height 0.7 d plus the roug |

<a id="energy_hub_air_density"></a>

### Air density at hub height — `energy_hub_air_density`

Temperature, pressure and air density at a turbine's hub height from a pressure and a temperature measured at other heights: temperature gradient -6.5 K/km, pressure gradient -1/8 hPa per m, then the barometric (1.225 $kg/m^{3}$ at 288.15 K, 1013.3 hPa) or the ideal-gas (R = 287.058 J/(kg K)) density.

$$
\rho_{hub} = 1.225\,\frac{p_{hub}}{101330\ \mathrm{Pa}}\,\frac{288.15\ \mathrm{K}}{T_{hub}}
$$

| input | meaning | condition |
|---|---|---|
| `p` | measured air pressure |  |
| `h_p` | height of the pressure measurement |  |
| `T` | measured air temperature |  |
| `h_T` | height of the temperature measurement |  |
| `hub` | hub height |  |
| `model` | density model (default barometric) |  |

| output | meaning |
|---|---|
| `T_hub` | temperature at hub height |
| `p_hub` | pressure at hub height |
| `rho` | air density at hub height |

- **Method:** T_hub = T - 0.0065 (h_hub - h_T); p_hub = p - 12.5 Pa/m (h_hub - h_p); rho = p_hub/(R T_hub) (ideal gas) or 1.225 (p_hub/101330)(288.15/T_hub) (barometric)
- **Assumes:** pressure and temperature stay positive at hub height
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_hub_air_density', p=101330, h_p=0, T=288.15, h_T=0, hub=0)
# -> T_hub = 288.15 K, p_hub = 101330 Pa, rho = 1.225 kg/m^3   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_hub_air_density', p=101330, h_p=0, T=288.15, h_T=0, hub=0, model='ideal_gas')
# -> T_hub = 288.15 K, p_hub = 101330 Pa, rho = 1.22504 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Air density at hub height: p = 1013.3 hPa, h_p = 0 m, T = 288.15 K, h_T = 0 m, hub = 0 m | SOLVED | T_hub = 288.15 K, p_hub = 101330 Pa, rho = 1.225 kg/m^3 |
| Air density at hub height: p = 1013 hPa, h_p = 0 m, T = 15 degC, h_T = 2 m | NEEDS_INPUT | energy_hub_air_density needs hub (hub height). Example: solve('energy_hub_air_density', p='101330 Pa', h_p='0 m', T='288 |

<a id="energy_power_curve_output"></a>

### Turbine output from its power curve — `energy_power_curve_output`

The electrical output of a wind turbine at a wind speed by linear interpolation of its power curve (zero below the first and above the last listed speed).

$$
P(v) = P_i + (P_{i+1} - P_i)\frac{v - v_i}{v_{i+1} - v_i}
$$

| input | meaning | condition |
|---|---|---|
| `speeds` | power-curve wind speeds in m/s (increasing) |  |
| `powers` | power-curve outputs in kW |  |
| `v` | wind speed at hub height |  |

| output | meaning |
|---|---|
| `P_kW` | output in kW |
| `P` | output |

- **Method:** linear interpolation between the curve points; 0 outside the listed speeds
- **Assumes:** speeds strictly increasing; one power per speed
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Wind turbine output against wind speed from the tabulated power curve (speeds 3, 5, 8, 12, 25 m/s, powers 0, 150, 800, 2000, 2000 kW), computed by the package at 121 speeds from 0 to 30 m/s: nothing below cut-in at 3 m/s, rated 2000 kW up to cut-out at 25 m/s, nothing above](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-power-curve-output-curve.svg)

*Wind turbine output against wind speed from the tabulated power curve (speeds 3, 5, 8, 12, 25 m/s, powers 0, 150, 800, 2000, 2000 kW), computed by the package at 121 speeds from 0 to 30 m/s: nothing below cut-in at 3 m/s, rated 2000 kW up to cut-out at 25 m/s, nothing above. Marked: v = 6.5 m/s, P = 475 kW.*

```python
energy_systems_ai999x.solve('energy_power_curve_output', speeds='3, 5, 8, 12, 25', powers='0, 150, 800, 2000, 2000', v=6.5)
# -> P_kW = 475, P = 475000 W   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_power_curve_output', speeds=[3, 5, 8, 12, 25], powers=[0, 150, 800, 2000, 2000], v=26)
# -> P_kW = 0, P = 0 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Power curve output: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 6.5 m/s | SOLVED | P_kW = 475, P = 475000 W |
| Power curve output: speeds = [3, 5, 4, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 6.5 m/s | INVALID_INPUT | Turbine output from its power curve: the wind speeds of the curve must increase strictly. |
| Power curve output: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000] | NEEDS_INPUT | energy_power_curve_output needs v (wind speed at hub height). Example: solve('energy_power_curve_output', speeds='3, 5,  |

<a id="energy_power_curve_density_corrected"></a>

### Turbine output from a density-corrected power curve — `energy_power_curve_density_corrected`

Turbine output at a wind speed and air density, with the power curve's speeds rescaled by $(1.225/\rho)^{e}$, e rising from $1/3$ below 7.5 m/s to $2/3$ above 12.5 m/s (Svenningsen's correction, as in windpowerlib).

$$
v_i' = v_i\left(\frac{1.225}{\rho}\right)^{e(v_i)},\ e\in[\tfrac13,\tfrac23]
$$

| input | meaning | condition |
|---|---|---|
| `speeds` | power-curve wind speeds in m/s at 1.225 $kg/m^{3}$ (increasing) |  |
| `powers` | power-curve outputs in kW |  |
| `v` | wind speed at hub height |  |
| `rho` | air density at hub height |  |

| output | meaning |
|---|---|
| `P_kW` | output in kW |
| `P` | output |

- **Method:** v_i' = v_i $(1.225/\rho)^{e}$(v_i); interpolate P(v) on (v_i', P_i)
- **Assumes:** $0.5 < \rho < 1.6$ $kg/m^{3}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_power_curve_density_corrected', speeds=[3, 5, 8, 12, 25], powers=[0, 150, 800, 2000, 2000], v=6.5, rho=1.225)
# -> P_kW = 475, P = 475000 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Density-corrected power curve: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 6.5 m/s, rho = 1.225 kg/m^3 | SOLVED | P_kW = 475, P = 475000 W |
| Density-corrected power curve: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], v = 6.5 m/s, rho = 3 kg/m^3 | INVALID_INPUT | Turbine output from a density-corrected power curve: the air density must lie between 0.5 and 1.6 kg/m^3. |

<a id="energy_weibull_wind"></a>

### Weibull wind statistics and mean power density — `energy_weibull_wind`

Mean wind speed, mean wind power density, energy pattern factor, most probable speed and the speed carrying most energy for a Weibull wind-speed distribution with shape k and scale c.

$$
\bar v = c\,\Gamma\!\left(1 + \tfrac1k\right),\quad \frac{\bar P}{A} = \tfrac12\rho c^3\,\Gamma\!\left(1 + \tfrac3k\right)
$$

| input | meaning | condition |
|---|---|---|
| `k` | Weibull shape factor $(1 < k  \leq  5)$ |  |
| `c` | Weibull scale factor |  |
| `rho` | air density |  |

| output | meaning |
|---|---|
| `v_mean` | mean wind speed c Gamma(1 + 1/k) |
| `power_density` | mean wind power density in $W/m^{2}$ |
| `energy_pattern_factor` | mean of $v^{3}$ over the cube of the mean |
| `v_most_probable` | most probable wind speed |
| `v_max_energy` | wind speed carrying the most energy |

- **Method:** v_mean = c Gamma(1 + 1/k); $P/A = (1/2)$ rho $c^{3}$ Gamma(1 + 3/k); $v_mp = c$ ((k - 1)/k)^(1/k); v_maxE = c ((k + 2)/k)^(1/k)
- **Assumes:** $1 < k  \leq  5$; $c > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_weibull_wind', k=2, c=8, rho=1.225)
# -> v_mean = 7.08982 m/s, power_density = 416.881, energy_pattern_factor = 1.90986, v_most_probable = 5.65685 m/s, v_max_energy = 11.3137 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Weibull wind statistics: k = 2, c = 8 m/s, rho = 1.225 kg/m^3 | SOLVED | v_mean = 7.08982 m/s, power_density = 416.881, energy_pattern_factor = 1.90986, v_most_probable = 5.65685 m/s, v_max_energy = 11.3137 m/s |
| Weibull wind statistics: k = 0.5, c = 8 m/s, rho = 1.225 kg/m^3 | INVALID_INPUT | Weibull wind statistics and mean power density: the Weibull shape k must lie in (1, 5]. |

<a id="energy_weibull_aep"></a>

### Annual energy production from a power curve and a Weibull distribution — `energy_weibull_aep`

Mean output, annual energy production and capacity factor of a wind turbine whose power curve (linear between points, zero outside) operates in winds with a Weibull distribution (shape k, scale c).

$$
\bar P = \int_0^\infty P(v)\,\frac{k}{c}\left(\frac vc\right)^{k-1}e^{-(v/c)^k}\,dv,\quad AEP = 8760\,\mathrm{h}\cdot\bar P
$$

| input | meaning | condition |
|---|---|---|
| `speeds` | power-curve wind speeds in m/s (increasing) |  |
| `powers` | power-curve outputs in kW |  |
| `k` | Weibull shape factor $(1 < k  \leq  5)$ |  |
| `c` | Weibull scale factor |  |
| `hours` | hours in the period (default 8760) |  |

| output | meaning |
|---|---|
| `mean_power_kW` | mean output in kW |
| `AEP_MWh` | energy over the period in MWh |
| `capacity_factor` | mean output over the largest output of the curve |

- **Method:** P_mean = sum over segments of integral (P_i + s (v - v_i)) f(v) dv, each in closed form: F(b) - F(a) and c Gamma(1 + 1/k) [P($1 + 1/k$, (b/c)^k) - P($1 + 1/k$, (a/c)^k)]
- **Assumes:** $1 < k  \leq  5$; speeds strictly increasing
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_weibull_aep', speeds=[0, 100], powers=[1000, 1000], k=2, c=7)
# -> mean_power_kW = 1000, AEP_MWh = 8760, capacity_factor = 1   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_weibull_aep', speeds=[0, 1000], powers=[0, 1000], k=2, c=7)
# -> mean_power_kW = 6.20359, AEP_MWh = 54.3434, capacity_factor = 0.00620359   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Annual energy production: speeds = [0, 1000], powers = [0, 1000], k = 2, c = 7 m/s | SOLVED | mean_power_kW = 6.20359, AEP_MWh = 54.3434, capacity_factor = 0.00620359 |
| Annual energy production: speeds = [3, 5, 8, 12, 25], powers = [0, 150, 800, 2000, 2000], k = 7, c = 7 m/s | INVALID_INPUT | Annual energy production from a power curve and a Weibull distribution: the Weibull shape k must lie in (1, 5]. |

<a id="energy_three_phase_power"></a>

### Balanced three-phase power from line quantities — `energy_three_phase_power`

Real, reactive and apparent power of a balanced three-phase load from the line voltage, line current and power factor, with the phase voltage and phase current of a star or delta connection.

$$
P = \sqrt3\,V_L I_L\cos\phi,\quad Q = \sqrt3\,V_L I_L\sin\phi
$$

| input | meaning | condition |
|---|---|---|
| `VL` | line-to-line rms voltage |  |
| `IL` | line current (rms) |  |
| `pf` | power factor $(0 < pf  \leq  1)$ |  |
| `connection` | connection of the load (star = wye) |  |
| `pf_type` | lagging (inductive, default) or leading (capacitive) |  |

| output | meaning |
|---|---|
| `P` | real power (three-phase total) |
| `Q` | reactive power in var (positive lagging) |
| `S` | apparent power in VA |
| `V_phase` | voltage across each phase of the load |
| `I_phase` | current in each phase of the load |

- **Method:** $S = sqrt$(3) V_L I_L, $P = S$ pf, $Q = S$ sin(acos pf); star: $V_{ph} = \frac{V_{L}}{\sqrt{3}}$, $I_ph = I_L$; delta: $V_ph = V_L$, $I_{ph} = \frac{I_{L}}{\sqrt{3}}$
- **Assumes:** balanced load; $0 < pf  \leq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_three_phase_power', VL=400, IL=50, pf=0.8, connection='star')
# -> P = 27712.8 W, Q = 20784.6 var, S = 34641 VA, V_phase = 230.94 V, I_phase = 50 A   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_three_phase_power', VL=400, IL=17.32050807568877, pf=1.0, connection='delta')
# -> P = 12000 W, Q = 0 var, S = 12000 VA, V_phase = 400 V, I_phase = 10 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star | SOLVED | P = 27712.8 W, Q = 20784.6 var, S = 34641 VA, V_phase = 230.94 V, I_phase = 50 A |
| Three-phase power: VL = 400 V, IL = 50 A, pf = 1.2, connection = star | INVALID_INPUT | Balanced three-phase power from line quantities: the power factor must lie in (0, 1]. |
| Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8 | NEEDS_INPUT | energy_three_phase_power needs connection (connection of the load (star = wye)). Example: solve('energy_three_phase_powe |

<a id="energy_per_unit_change_of_base"></a>

### Per-unit impedance on a new base — `energy_per_unit_change_of_base`

An impedance given in per unit on equipment ratings (S_old, V_old) converted to the per-unit system of the network (S_new, V_new).

$$
Z_{pu,new} = Z_{pu,old}\left(\frac{V_{old}}{V_{new}}\right)^2\frac{S_{new}}{S_{old}}
$$

| input | meaning | condition |
|---|---|---|
| `R_pu` | per-unit resistance on the old base |  |
| `X_pu` | per-unit reactance on the old base |  |
| `S_old` | old base (equipment rating) in VA |  |
| `V_old` | old base voltage (line-to-line) |  |
| `S_new` | new (system) base power in VA |  |
| `V_new` | new base voltage (line-to-line) |  |

| output | meaning |
|---|---|
| `R_pu_new` | per-unit resistance on the new base |
| `X_pu_new` | per-unit reactance on the new base |
| `Z_pu_new` | per-unit impedance magnitude on the new base |

- **Method:** Z_new = Z_old (V_old/V_new)^2 (S_new/S_old)
- **Assumes:** $X_pu > 0$; positive bases
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_per_unit_change_of_base', R_pu=0.0, X_pu=0.1, S_old=50000000.0, V_old=132000.0, S_new=100000000.0, V_new=138000.0)
# -> R_pu_new = 0, X_pu_new = 0.182987, Z_pu_new = 0.182987   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_per_unit_change_of_base', R_pu=0.01, X_pu=0.1, S_old=10000000.0, V_old=11000.0, S_new=100000000.0, V_new=11000.0)
# -> R_pu_new = 0.1, X_pu_new = 1, Z_pu_new = 1.00499   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Per-unit change of base: R_pu = 0.01, X_pu = 0.1, S_old = 10 MVA, V_old = 11 kV, S_new = 100 MVA, V_new = 11 kV | SOLVED | R_pu_new = 0.1, X_pu_new = 1, Z_pu_new = 1.00499 |
| Per-unit change of base: R_pu = 0.01, X_pu = 0.1, S_old = 10 MVA, V_old = 11 kV, S_new = 100 MVA | NEEDS_INPUT | energy_per_unit_change_of_base needs V_new (new base voltage (line-to-line)). Example: solve('energy_per_unit_change_of_ |

<a id="energy_short_line"></a>

### Short transmission line: sending-end voltage, regulation and efficiency — `energy_short_line`

Sending-end voltage and its angle, line current, losses, voltage regulation and efficiency of a short three-phase line (series R + jX, no shunt admittance) feeding a load at a known receiving-end voltage.

$$
\mathbf V_s = \mathbf V_r + \mathbf I\,(R + jX),\quad \text{VR} = \frac{|V_s| - |V_r|}{|V_r|}
$$

| input | meaning | condition |
|---|---|---|
| `Vr` | receiving-end line-to-line voltage |  |
| `P` | load real power (three-phase) |  |
| `pf` | load power factor |  |
| `R` | line resistance per phase (total) |  |
| `X` | line reactance per phase (total) |  |
| `pf_type` | lagging (default) or leading load |  |

| output | meaning |
|---|---|
| `Vs` | sending-end line voltage |
| `delta_deg` | angle of the sending voltage ahead of the receiving voltage, degrees |
| `I` | receiving-end line current |
| `losses` | line losses (three-phase) |
| `Ps` | sending-end real power |
| `regulation_pct` | voltage regulation in percent |
| `efficiency_pct` | transmission efficiency in percent |

- **Method:** $V_s = V_r + I$ (R + jX) per phase, I = P/(sqrt(3) V_r pf) at angle -acos(pf); regulation = (V_s - V_r)/V_r; efficiency = P/(P + 3 $I^{2}$ R)
- **Assumes:** short line: shunt admittance neglected; balanced three-phase load
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_short_line', Vr=33000.0, P=10000000.0, pf=1.0, R=0.0, X=0.0)
# -> Vs = 33000 V, delta_deg = 0, I = 174.955 A, losses = 0 W, Ps = 1e+07 W, regulation_pct = 0, efficiency_pct = 100   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_short_line', Vr=1732.0508075688772, P=300000, pf=1.0, R=1.0, X=0.0)
# -> Vs = 1905.26 V, delta_deg = 0, I = 100 A, losses = 30000 W, Ps = 330000 W, regulation_pct = 10, efficiency_pct = 90.9091   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Short line: Vr = 33 kV, P = 10 MW, pf = 1, R = 0 ohm, X = 0 ohm | SOLVED | Vs = 33000 V, delta_deg = 0, I = 174.955 A, losses = 0 W, Ps = 1e+07 W, regulation_pct = 0, efficiency_pct = 100 |
| Short line: Vr = 33 kV, P = 10 MW, pf = 0.8, R = 5 ohm, X = 20 ohm, pf_type = sideways | INVALID_INPUT | pf_type: one of lagging, leading; got 'sideways'. |
| Short line: Vr = 33 kV, P = 10 MW, pf = 0.8, R = 5 ohm | NEEDS_INPUT | energy_short_line needs X (line reactance per phase (total)). Example: solve('energy_short_line', Vr='33000 V', P='1e+07 |

<a id="energy_medium_line"></a>

### Medium transmission line (nominal pi) — `energy_medium_line`

ABCD constants of the nominal-pi model and the sending-end voltage, current and power, losses, regulation and efficiency of a medium-length three-phase line feeding a load.

$$
\begin{bmatrix}V_s\\I_s\end{bmatrix} = \begin{bmatrix}1+\frac{ZY}{2} & Z\\ Y\left(1+\frac{ZY}{4}\right) & 1+\frac{ZY}{2}\end{bmatrix}\begin{bmatrix}V_r\\I_r\end{bmatrix}
$$

| input | meaning | condition |
|---|---|---|
| `Vr` | receiving-end line-to-line voltage |  |
| `P` | load real power (three-phase) |  |
| `pf` | load power factor |  |
| `R` | line resistance per phase (total) |  |
| `X` | line reactance per phase (total) |  |
| `B` | total shunt susceptance of the line per phase |  |
| `pf_type` | lagging (default) or leading load |  |

| output | meaning |
|---|---|
| `Vs` | sending-end line voltage |
| `delta_deg` | angle of the sending voltage ahead of the receiving voltage, degrees |
| `Is` | sending-end current |
| `Ps` | sending-end real power |
| `losses` | line losses (three-phase) |
| `regulation_pct` | voltage regulation (V_s/\|A\| - V_r)/V_r in percent |
| `efficiency_pct` | transmission efficiency in percent |
| `A_mag` | \|A\| (= \|D\|) |
| `A_angle_deg` | angle of A in degrees |
| `B_mag` | \|B\| in ohms |
| `B_angle_deg` | angle of B in degrees |

- **Method:** $A = D = 1 + ZY/2$, $B = Z$, $C = Y$(1 + ZY/4); $V_s = A$ V_r + B I_r, $I_s = C$ V_r + D I_r; regulation = (V_s/|A| - V_r)/V_r
- **Assumes:** nominal-pi model (half the shunt admittance at each end); balanced load
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_medium_line', Vr=132000.0, P=50000000.0, pf=1.0, R=0.0, X=40.0, B=0.0005)
# -> Vs = 131555 V, delta_deg = 6.61355, Is = 219.801 A, Ps = 5e+07 W, losses = 0 W, regulation_pct = 0.669903, efficiency_pct = 100, A_mag = 0.99, A_angle_deg = 0, B_mag = 40 ohm, B_angle_deg = 90   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Medium line: Vr = 132 kV, P = 50 MW, pf = 1, R = 0 ohm, X = 40 ohm, B = 500 uS | SOLVED | Vs = 131555 V, delta_deg = 6.61355, Is = 219.801 A, Ps = 5e+07 W, losses = 0 W, regulation_pct = 0.669903, efficiency_pct = 100, A_mag = 0.99, A_angle_deg = 0, B_mag = 40 ohm, B_angle_deg = 90 |
| Medium line: Vr = 132 kV, P = 50 MW, pf = 0.8, R = 10 ohm, X = 40 ohm | NEEDS_INPUT | energy_medium_line needs B (total shunt susceptance of the line per phase). Example: solve('energy_medium_line', Vr='132 |

<a id="energy_long_line"></a>

### Long transmission line (exact distributed parameters) — `energy_long_line`

Characteristic impedance, propagation constant, exact ABCD constants and the sending-end quantities, regulation and efficiency of a long three-phase line with uniformly distributed R, X and shunt B (the Ferranti rise appears as a negative regulation at light load).

$$
A = \cosh\gamma l,\ B = Z_c\sinh\gamma l,\ C = \frac{\sinh\gamma l}{Z_c},\quad Z_c = \sqrt{\frac{z}{y}},\ \gamma = \sqrt{zy}
$$

| input | meaning | condition |
|---|---|---|
| `Vr` | receiving-end line-to-line voltage |  |
| `P` | load real power (three-phase) |  |
| `pf` | load power factor |  |
| `R` | line resistance per phase (total) |  |
| `X` | line reactance per phase (total) |  |
| `B` | total shunt susceptance of the line per phase |  |
| `pf_type` | lagging (default) or leading load |  |

| output | meaning |
|---|---|
| `Vs` | sending-end line voltage |
| `delta_deg` | angle of the sending voltage ahead of the receiving voltage, degrees |
| `Is` | sending-end current |
| `Ps` | sending-end real power |
| `losses` | line losses (three-phase) |
| `regulation_pct` | voltage regulation (V_s/\|A\| - V_r)/V_r in percent |
| `efficiency_pct` | transmission efficiency in percent |
| `A_mag` | \|A\| (= \|D\|) |
| `A_angle_deg` | angle of A in degrees |
| `B_mag` | \|B\| in ohms |
| `B_angle_deg` | angle of B in degrees |
| `Zc_mag` | characteristic (surge) impedance magnitude |
| `Zc_angle_deg` | angle of Zc in degrees |
| `alpha_l` | attenuation alpha l in nepers |
| `beta_l` | phase shift beta l in radians |

- **Method:** gamma $l = \sqrt{ZY}$, $Zc = \sqrt{\frac{Z}{Y}}$; A = D = cosh(gamma l), $B = Zc$ sinh(gamma l), C = sinh(gamma l)/Zc
- **Assumes:** uniformly distributed parameters; balanced load
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_long_line', Vr=400000.0, P=1000000.0, pf=1.0, R=0.0, X=100.0, B=0.0004)
# -> Vs = 392027 V, delta_deg = 0.0362951, Is = 91.7723 A, Ps = 1e+06 W, losses = 0 W, regulation_pct = 2.00641e-05, efficiency_pct = 100, A_mag = 0.980067, A_angle_deg = 0, B_mag = 99.3347 ohm, B_angle_deg = 90, Zc_mag = 500 ohm, Zc_angle_deg = 0, alpha_l = 0, beta_l = 0.2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Long line: Vr = 400 kV, P = 1 MW, pf = 1, R = 0 ohm, X = 100 ohm, B = 400 uS | SOLVED | Vs = 392027 V, delta_deg = 0.0362951, Is = 91.7723 A, Ps = 1e+06 W, losses = 0 W, regulation_pct = 2.00641e-05, efficiency_pct = 100, A_mag = 0.980067, A_angle_deg = 0, B_mag = 99.3347 ohm, B_angle_deg = 90, Zc_mag = 500 ohm, Zc_angle_deg = 0, alpha_l = 0, beta_l = 0.2 |
| Long line: Vr = 400 kV, P = 1 MW, pf = 1, R = 0 ohm, X = 100 ohm, B = 0 uS | INVALID_INPUT | Long transmission line (exact distributed parameters): the shunt susceptance must be a positive number. |

<a id="energy_line_parameters"></a>

### Inductance, capacitance and surge impedance of a three-phase overhead line — `energy_line_parameters`

Per-phase inductance, capacitance, reactance, susceptance, surge impedance (and surge-impedance loading at a voltage) of a fully transposed three-phase line from its phase spacings, conductor radius and geometric mean radius, with optional bundled conductors.

$$
L = 2\times10^{-7}\ln\frac{GMD}{GMR}\ \mathrm{H/m},\quad C = \frac{2\pi\varepsilon_0}{\ln(GMD/r)}\ \mathrm{F/m}
$$

| input | meaning | condition |
|---|---|---|
| `D_ab` | distance between phases a and b |  |
| `D_bc` | distance between phases b and c |  |
| `D_ca` | distance between phases c and a |  |
| `r` | conductor (outer) radius |  |
| `f` | system frequency |  |
| `gmr` | conductor geometric mean radius (default r $e^{-1/4}$, a solid round conductor) |  |
| `n_bundle` | subconductors per phase (1 to 4, regular polygon) |  |
| `spacing` | distance between neighbouring subconductors of a bundle |  |
| `V` | line voltage for the surge-impedance loading (optional) |  |

| output | meaning |
|---|---|
| `GMR` | geometric mean radius of a phase (bundle) |
| `L_mH_per_km` | inductance per phase in mH/km |
| `C_nF_per_km` | capacitance to neutral in nF/km |
| `X_ohm_per_km` | series reactance in ohm/km |
| `B_uS_per_km` | shunt susceptance in uS/km |
| `Zc` | surge impedance of the lossless line sqrt(L/C) |
| `SIL_MW` | surge-impedance loading $V^{2}/Zc$ in MW |

- **Method:** GMD = (D_ab D_bc D_ca)^(1/3); $L = 2e-7$ ln(GMD/GMR) H/m; $C = 2$ pi eps0/ln(GMD/r) F/m; bundles: GMR_b = (GMR $d^{n-1}$)^$(1/n)$ (2^(1/8) (GMR d^3)^(1/4) = 1.09 (GMR d^3)^(1/4) for 4)
- **Assumes:** fully transposed line; spacings much larger than the conductors; ground effect neglected
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_line_parameters', D_ab=5, D_bc=5, D_ca=5, r=0.01, f=50, gmr=0.005)
# -> GMR = 0.005 m, L_mH_per_km = 1.38155, C_nF_per_km = 8.95189, X_ohm_per_km = 0.434027, B_uS_per_km = 2.81232, Zc = 392.849 ohm   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_line_parameters', D_ab=6, D_bc=6, D_ca=6, r=0.012, f=60, gmr=0.01, n_bundle=2, spacing=0.4)
# -> GMR = 0.0632456 m, L_mH_per_km = 0.910498, C_nF_per_km = 12.4699, X_ohm_per_km = 0.34325, B_uS_per_km = 4.70106, Zc = 270.214 ohm   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Line parameters: D_ab = 5 m, D_bc = 5 m, D_ca = 5 m, r = 1 cm, f = 50 Hz, gmr = 5 mm | SOLVED | GMR = 0.005 m, L_mH_per_km = 1.38155, C_nF_per_km = 8.95189, X_ohm_per_km = 0.434027, B_uS_per_km = 2.81232, Zc = 392.849 ohm |
| Line parameters: D_ab = 5 m, D_bc = 5 m, D_ca = 5 m, r = 1 cm, f = 50 Hz, gmr = 2 cm | INVALID_INPUT | Inductance, capacitance and surge impedance of a three-phase overhead line: the geometric mean radius cannot exceed the  |

<a id="energy_two_bus_load_flow"></a>

### Receiving-end voltage of a line feeding a constant-power load (two-bus load flow) — `energy_two_bus_load_flow`

The receiving-end voltage magnitude and angle, current and losses when a three-phase line of impedance R + jX supplies a constant P + jQ load from a fixed sending-end voltage (the exact two-bus load flow).

$$
V_r^4 + \left[2(PR+QX) - V_s^2\right]V_r^2 + (P^2+Q^2)(R^2+X^2) = 0
$$

| input | meaning | condition |
|---|---|---|
| `Vs` | sending-end line-to-line voltage (slack) |  |
| `R` | line resistance per phase |  |
| `X` | line reactance per phase |  |
| `P` | load real power (three-phase) |  |
| `Q` | load reactive power in var (negative for a capacitive load) |  |

| output | meaning |
|---|---|
| `Vr` | receiving-end line voltage |
| `delta_deg` | receiving-end voltage angle in degrees (negative: lagging the source) |
| `I` | line current |
| `P_loss` | real power loss |
| `Q_loss` | reactive power absorbed by the line in var |
| `voltage_drop_pct` | (V_s - V_r)/V_s in percent |

- **Method:** $V_r^{4}$ + [2(PR + QX) - V_s^2] $V_r^{2} + (P^{2} + Q^{2})$$(R^{2} + X^{2}) = 0$, larger root; angle from $V_s = V_r + (PR + QX)/V_r + j$(PX - QR)/V_r
- **Assumes:** the load is within the line's transfer limit (a real solution exists); the high-voltage (normal) solution is reported
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_two_bus_load_flow', Vs=33000.0, R=0.0, X=10.0, P=0.0, Q=0.0)
# -> Vr = 33000 V, delta_deg = 0, I = 0 A, P_loss = 0 W, Q_loss = 0 var, voltage_drop_pct = 0   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_two_bus_load_flow', Vs=2000.0, R=0.0, X=1.0, P=1000000.0, Q=0.0)
# -> Vr = 1931.85 V, delta_deg = -15, I = 298.858 A, P_loss = 0 W, Q_loss = 267949 var, voltage_drop_pct = 3.40742   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Two-bus load flow: Vs = 2 kV, R = 0 ohm, X = 1 ohm, P = 1 MW, Q = 0 var | SOLVED | Vr = 1931.85 V, delta_deg = -15, I = 298.858 A, P_loss = 0 W, Q_loss = 267949 var, voltage_drop_pct = 3.40742 |
| Two-bus load flow: Vs = 33 kV, R = 3 ohm, X = 12 ohm, P = 100 MW, Q = 50 Mvar | INVALID_INPUT | Receiving-end voltage of a line feeding a constant-power load (two-bus load flow): the load exceeds what the line can de |

<a id="energy_dc_power_flow"></a>

### DC power flow — `energy_dc_power_flow`

Bus voltage angles and line flows of a network by the DC (linearised) power flow: lossless lines, flat 1 pu voltages, $P_ij = (theta_i - theta_j)/x_ij$; bus 1 is the slack (angle 0) and takes the balance.

$$
\mathbf B'\boldsymbol\theta = \mathbf P,\quad P_{ij} = \frac{\theta_i - \theta_j}{x_{ij}}
$$

| input | meaning | condition |
|---|---|---|
| `lines` | lines as 'i-j x' with the series reactance x in per unit (buses numbered 1..n) |  |
| `P` | net injections (generation minus load) of buses 2..n in per unit |  |

| output | meaning |
|---|---|
| `theta_deg` | bus voltage angles in degrees, buses 1..n |
| `flows` | line flows from the first to the second named bus, per unit, in the order given |
| `P_slack` | slack (bus 1) injection in per unit |

- **Method:** B' $\theta = P$ (reduced susceptance matrix, slack removed), solved by Gaussian elimination; $P_ij = (theta_i - theta_j)/x_ij$
- **Assumes:** connected network; positive reactances; bus 1 is the slack
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_dc_power_flow', lines='1-2 0.1, 2-3 0.2', P='0, -1')
# -> theta_deg = [0.0, -5.72957795131, -17.1887338539], flows = [1.0, 1.0], P_slack = 1   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_dc_power_flow', lines='1-2 0.1, 1-2 0.1', P='-1')
# -> theta_deg = [0.0, -2.86478897565], flows = [0.5, 0.5], P_slack = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| DC power flow: lines = 1-2 0.1; 2-3 0.2, P = [0, -1] | SOLVED | theta_deg = [0.0, -5.72957795131, -17.1887338539], flows = [1.0, 1.0], P_slack = 1 |
| DC power flow: lines = 1-2 0.1; 2-3 0.2, P = [0, -1, 2] | INVALID_INPUT | DC power flow: give the net injections of buses 2 to 3 (2 values); bus 1 is the slack. |
| DC power flow: lines = 1-2 0.1; 2-3 0.2 | NEEDS_INPUT | energy_dc_power_flow needs P (net injections (generation minus load) of buses 2..n in per unit). Example: solve('energy_ |

<a id="energy_newton_load_flow"></a>

### AC load flow by Newton-Raphson — `energy_newton_load_flow`

Bus voltage magnitudes and angles, slack-bus power, PV-bus reactive power and losses of a small network by the Newton-Raphson load flow (per unit; pi-model lines; bus 1 is the slack, generator buses hold their voltage; reactive limits are not enforced).

$$
\begin{bmatrix}\Delta\mathbf P\\ \Delta\mathbf Q\end{bmatrix} = \begin{bmatrix}\mathbf H & \mathbf N\\ \mathbf M & \mathbf L\end{bmatrix}\begin{bmatrix}\Delta\boldsymbol\theta\\ \Delta|\mathbf V|\end{bmatrix}
$$

| input | meaning | condition |
|---|---|---|
| `lines` | one row per line: from bus, to bus, r, x, total shunt b (per unit) |  |
| `loads` | one row per load: bus, P, Q (per unit) |  |
| `gens` | optional rows per voltage-controlled (PV) generator: bus, P, V (per unit) |  |
| `V1` | slack-bus voltage magnitude in per unit (default 1.0) |  |

| output | meaning |
|---|---|
| `V` | voltage magnitudes in per unit, buses 1..n |
| `angle_deg` | voltage angles in degrees, buses 1..n |
| `P_slack` | slack real power injection (pu) |
| `Q_slack` | slack reactive power injection (pu) |
| `P_loss` | total real power loss (pu) |
| `Q_gen` | reactive power of each PV generator (pu), in the order given |

- **Method:** polar Newton-Raphson on the P and Q mismatches with the analytic Jacobian, solved by Gaussian elimination, to a mismatch below $1e-13$ pu
- **Assumes:** buses numbered 1..n, bus 1 the slack; the load flow converges from a flat start
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_newton_load_flow', lines='1 2 0.0 0.5 0.0', loads='2 0.0 0.0')
# -> V = [1.0, 1.0], angle_deg = [0.0, 0.0], P_slack = 0, Q_slack = 0, P_loss = 0, Q_gen = []   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Load flow: lines = [1 2 0 0.5 0], loads = [2 0 0] | SOLVED | V = [1.0, 1.0], angle_deg = [0.0, 0.0], P_slack = 0, Q_slack = 0, P_loss = 0, Q_gen = [] |
| Load flow: lines = [1 2 0.02 0.06 0.03], loads = [2 50 20] | INVALID_INPUT | AC load flow by Newton-Raphson: the load flow diverged (the network cannot carry this loading). |
| Load flow: loads = [2 0.5 0.2] | NEEDS_INPUT | energy_newton_load_flow needs lines (one row per line: from bus, to bus, r, x, total shunt b (per unit)). Example: solve |

<a id="energy_economic_dispatch"></a>

### Economic dispatch with quadratic costs and limits — `energy_economic_dispatch`

The least-cost outputs of thermal units with costs a + b P + c $P^{2}$ ($/h) and output limits that together meet a demand (losses neglected): every unit between its limits runs at the same incremental cost lambda.

$$
\frac{dC_i}{dP_i} = b_i + 2c_iP_i = \lambda\ \ (P_{min} < P_i < P_{max}),\quad \sum_i P_i = D
$$

| input | meaning | condition |
|---|---|---|
| `costs` | one row per unit: a ($/h), b ($/MWh), c ($/$MW^{2}$ h), Pmin (MW), Pmax (MW) |  |
| `demand` | total demand in MW |  |

| output | meaning |
|---|---|
| `lambda_` | system incremental cost in $/MWh |
| `P` | unit outputs in MW, in the order given |
| `cost` | total cost in $/h |

- **Method:** equal incremental cost: P_i(lambda) = clip((lambda - b_i)/(2 c_i), Pmin_i, Pmax_i), lambda by bisection on sum $P_i = D$
- **Assumes:** $c > 0$ for every unit; sum Pmin <= demand <= sum Pmax; at least one unit between its limits
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Economic dispatch of 3 generators with quadratic cost curves (rows a, b, c, Pmin, Pmax: 500 5](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-economic-dispatch-bars.svg)

*Economic dispatch of 3 generators with quadratic cost curves (rows a, b, c, Pmin, Pmax: 500 5.3 0.004 200 450; 400 5.5 0.006 150 350; 200 5.8 0.009 100 225) meeting a demand of 800 MW: the package returns outputs 400, 250, 150 MW (bars, dashed lines the limits) at a common incremental cost λ = 8.5 per MWh, total cost 6682.5 per hour.*

```python
energy_systems_ai999x.solve('energy_economic_dispatch', costs='500 5.3 0.004 200 450; 400 5.5 0.006 150 350; 200 5.8 0.009 100 225', demand=800)
# -> lambda_ = 8.5, P = [400.0, 250.0, 150.0], cost = 6682.5   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_economic_dispatch', costs=[[0, 10, 0.01, 0, 500], [0, 10, 0.01, 0, 500]], demand=400)
# -> lambda_ = 14, P = [200.0, 200.0], cost = 4800   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Economic dispatch: costs = [500 5.3 0.004 200 450]; [400 5.5 0.006 150 350]; [200 5.8 0.009 100 225], demand = 800 MW | SOLVED | lambda_ = 8.5, P = [400.0, 250.0, 150.0], cost = 6682.5 |
| Economic dispatch: costs = [500 5.3 0.004 200 450]; [400 5.5 0.006 150 350], demand = 900 MW | INVALID_INPUT | Economic dispatch with quadratic costs and limits: the demand must lie between the total minimum and total maximum outpu |

<a id="energy_symmetrical_fault"></a>

### Three-phase (symmetrical) fault current by the bus impedance matrix — `energy_symmetrical_fault`

The fault current, Thevenin reactance and the bus voltages during a balanced three-phase fault at a bus of a reactance network (per unit, prefault no-load voltage Vf everywhere; every branch to node 0 is a source).

$$
I_f = \frac{V_f}{Z_{ff} + Z_f},\quad V_i = V_f - Z_{if}I_f
$$

| input | meaning | condition |
|---|---|---|
| `network` | branches as 'i-j x' with reactances in per unit; node 0 is the reference, each branch to 0 a source (generator or infinite bus) |  |
| `fault_bus` | faulted bus |  |
| `Vf` | prefault voltage in per unit (default 1.0) |  |
| `Xf` | fault reactance in per unit (default 0, a bolted fault) |  |
| `S_base` | base power, for the fault current in amperes (optional) |  |
| `V_base` | base line voltage at the faulted bus (optional) |  |

| output | meaning |
|---|---|
| `If` | fault current in per unit |
| `X_th` | Thevenin reactance at the faulted bus (Z_ff) in per unit |
| `V` | bus voltage magnitudes during the fault (pu), buses 1..n |
| `If_A` | fault current in amperes |

- **Method:** column f of Zbus from Ybus $z = e_f$; $I_f = V_f/(Z_ff + jX_f)$; $V_i = V_f - Z_if$ I_f
- **Assumes:** reactance network; no prefault load current (all prefault voltages Vf)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_symmetrical_fault', network='0-1 0.2, 1-2 0.3', fault_bus=2)
# -> If = 2, X_th = 0.5, V = [0.6, 0.0]   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_symmetrical_fault', network='0-1 0.1', fault_bus=1, S_base=100000000.0, V_base=100000.0)
# -> If = 10, X_th = 0.1, V = [0.0], If_A = 5773.5 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Symmetrical fault: network = 0-1 0.2; 1-2 0.3, fault_bus = 2 | SOLVED | If = 2, X_th = 0.5, V = [0.6, 0.0] |
| Three-phase fault: network = 0-1 0.1, bus = 1, S_base = 100 MVA, V_base = 100 kV | SOLVED | If = 10, X_th = 0.1, V = [0.0], If_A = 5773.5 A |
| Symmetrical fault: network = 0-1 0.2; 1-2 0.3, fault_bus = 5 | INVALID_INPUT | Three-phase (symmetrical) fault current by the bus impedance matrix: the faulted bus must be one of 1..2. |

<a id="energy_slg_fault"></a>

### Single line-to-ground fault (symmetrical components) — `energy_slg_fault`

Phase and sequence currents and phase voltages for a fault from phase a to ground through Zf: the three sequence networks in series.

$$
I_a = 3I_0 = \frac{3V_f}{Z_1 + Z_2 + Z_0 + 3Z_f}
$$

| input | meaning | condition |
|---|---|---|
| `X1` | positive-sequence Thevenin reactance (pu) |  |
| `X2` | negative-sequence reactance (pu) |  |
| `X0` | zero-sequence reactance (pu) |  |
| `Rf` | fault resistance (pu, default 0) |  |
| `Xf` | fault reactance (pu, default 0) |  |
| `Vf` | prefault voltage (pu, default 1.0) |  |

| output | meaning |
|---|---|
| `Ia` | \|I_a\| (pu) |
| `Ib` | \|I_b\| (pu) |
| `Ic` | \|I_c\| (pu) |
| `In` | neutral (earth) current \|3 I_0\| (pu) |
| `I1` | \|I_1\| (pu) |
| `I2` | \|I_2\| (pu) |
| `I0` | \|I_0\| (pu) |
| `Va` | \|V_a\| (pu) |
| `Vb` | \|V_b\| (pu) |
| `Vc` | \|V_c\| (pu) |

- **Method:** I1 = I2 = I0 = Vf/(Z1 + Z2 + Z0 + 3 Zf); $I_a = 3$ I_0
- **Assumes:** sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_slg_fault', X1=0.1, X2=0.1, X0=0.1)
# -> Ia = 10, Ib = 0, Ic = 0, In = 10, I1 = 3.33333, I2 = 3.33333, I0 = 3.33333, Va = 0, Vb = 1, Vc = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SLG fault: X1 = 0.1, X2 = 0.1, X0 = 0.1 | SOLVED | Ia = 10, Ib = 0, Ic = 0, In = 10, I1 = 3.33333, I2 = 3.33333, I0 = 3.33333, Va = 0, Vb = 1, Vc = 1 |
| SLG fault: X1 = 0.1, X2 = 0.1, X0 = -0.1 | INVALID_INPUT | Single line-to-ground fault (symmetrical components): the sequence reactances must be a positive number. |
| SLG fault: X1 = 0.1, X2 = 0.1 | NEEDS_INPUT | energy_slg_fault needs X0 (zero-sequence reactance (pu)). Example: solve('energy_slg_fault', X1=0.1, X2=0.1, X0=0.1). |

<a id="energy_ll_fault"></a>

### Line-to-line fault (symmetrical components) — `energy_ll_fault`

Phase and sequence currents and phase voltages for a fault between phases b and c through Zf: positive and negative networks in parallel opposition.

$$
I_b = -I_c = -j\sqrt3\,\frac{V_f}{Z_1 + Z_2 + Z_f}
$$

| input | meaning | condition |
|---|---|---|
| `X1` | positive-sequence Thevenin reactance (pu) |  |
| `X2` | negative-sequence reactance (pu) |  |
| `X0` | zero-sequence reactance (pu) |  |
| `Rf` | fault resistance (pu, default 0) |  |
| `Xf` | fault reactance (pu, default 0) |  |
| `Vf` | prefault voltage (pu, default 1.0) |  |

| output | meaning |
|---|---|
| `Ia` | \|I_a\| (pu) |
| `Ib` | \|I_b\| (pu) |
| `Ic` | \|I_c\| (pu) |
| `In` | neutral (earth) current \|3 I_0\| (pu) |
| `I1` | \|I_1\| (pu) |
| `I2` | \|I_2\| (pu) |
| `I0` | \|I_0\| (pu) |
| `Va` | \|V_a\| (pu) |
| `Vb` | \|V_b\| (pu) |
| `Vc` | \|V_c\| (pu) |

- **Method:** $I_{1} = -I_{2} = Vf/(Z_{1} + Z_{2} + Zf)$, $I_{0} = 0$; |I_b| = sqrt(3) |I_1|
- **Assumes:** sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_ll_fault', X1=0.1, X2=0.1, X0=0.1)
# -> Ia = 0, Ib = 8.66025, Ic = 8.66025, In = 0, I1 = 5, I2 = 5, I0 = 0, Va = 1, Vb = 0.5, Vc = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| LL fault: X1 = 0.1, X2 = 0.1, X0 = 0.1 | SOLVED | Ia = 0, Ib = 8.66025, Ic = 8.66025, In = 0, I1 = 5, I2 = 5, I0 = 0, Va = 1, Vb = 0.5, Vc = 0.5 |
| LL fault: X1 = 0.1, X2 = 0.1, X0 = -0.1 | INVALID_INPUT | Line-to-line fault (symmetrical components): the sequence reactances must be a positive number. |
| LL fault: X1 = 0.1, X2 = 0.1 | NEEDS_INPUT | energy_ll_fault needs X0 (zero-sequence reactance (pu)). Example: solve('energy_ll_fault', X1=0.1, X2=0.1, X0=0.1). |

<a id="energy_dlg_fault"></a>

### Double line-to-ground fault (symmetrical components) — `energy_dlg_fault`

Phase, neutral and sequence currents and phase voltages for a fault from phases b and c to ground through Zf: negative and (zero + 3Zf) networks in parallel.

$$
I_1 = \frac{V_f}{Z_1 + \frac{Z_2(Z_0 + 3Z_f)}{Z_2 + Z_0 + 3Z_f}}
$$

| input | meaning | condition |
|---|---|---|
| `X1` | positive-sequence Thevenin reactance (pu) |  |
| `X2` | negative-sequence reactance (pu) |  |
| `X0` | zero-sequence reactance (pu) |  |
| `Rf` | fault resistance (pu, default 0) |  |
| `Xf` | fault reactance (pu, default 0) |  |
| `Vf` | prefault voltage (pu, default 1.0) |  |

| output | meaning |
|---|---|
| `Ia` | \|I_a\| (pu) |
| `Ib` | \|I_b\| (pu) |
| `Ic` | \|I_c\| (pu) |
| `In` | neutral (earth) current \|3 I_0\| (pu) |
| `I1` | \|I_1\| (pu) |
| `I2` | \|I_2\| (pu) |
| `I0` | \|I_0\| (pu) |
| `Va` | \|V_a\| (pu) |
| `Vb` | \|V_b\| (pu) |
| `Vc` | \|V_c\| (pu) |

- **Method:** I1 = Vf/(Z1 + Z2 || $(Z_{0} + 3Zf)$); $I_{2} = -I_{1}$ $(Z_{0} + 3Zf)/(Z_{2} + Z_{0} + 3Zf)$; $I_{0} = -I_{1}$ $Z_{2}/(Z_{2} + Z_{0} + 3Zf)$
- **Assumes:** sequence networks from the Thevenin reactances at the fault; prefault no-load voltage Vf
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_dlg_fault', X1=0.1, X2=0.1, X0=0.1)
# -> Ia = 0, Ib = 10, Ic = 10, In = 10, I1 = 6.66667, I2 = 3.33333, I0 = 3.33333, Va = 1, Vb = 0, Vc = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| DLG fault: X1 = 0.1, X2 = 0.1, X0 = 0.1 | SOLVED | Ia = 0, Ib = 10, Ic = 10, In = 10, I1 = 6.66667, I2 = 3.33333, I0 = 3.33333, Va = 1, Vb = 0, Vc = 0 |
| DLG fault: X1 = 0.1, X2 = 0.1, X0 = -0.1 | INVALID_INPUT | Double line-to-ground fault (symmetrical components): the sequence reactances must be a positive number. |
| DLG fault: X1 = 0.1, X2 = 0.1 | NEEDS_INPUT | energy_dlg_fault needs X0 (zero-sequence reactance (pu)). Example: solve('energy_dlg_fault', X1=0.1, X2=0.1, X0=0.1). |

<a id="energy_power_angle"></a>

### Power transfer and power angle between two voltages — `energy_power_angle`

The power angle needed to transfer P over a lossless reactance X between a sending voltage V2 (generator) and a receiving voltage V1 (infinite bus), the steady-state limit V1 $V_{2}/X$, the reactive power at both ends and the stability margin.

$$
P = \frac{V_1V_2}{X}\sin\delta,\quad P_{max} = \frac{V_1V_2}{X}
$$

| input | meaning | condition |
|---|---|---|
| `V1` | receiving (infinite-bus) line voltage |  |
| `V2` | sending (generator) line voltage |  |
| `X` | transfer reactance per phase |  |
| `P` | real power transferred (three-phase) |  |

| output | meaning |
|---|---|
| `delta_deg` | power angle (V2 ahead of V1) in degrees |
| `Pmax` | steady-state limit V1 $V_{2}/X$ |
| `Q_sent` | reactive power sent from the generator end in var |
| `Q_received` | reactive power arriving at the infinite bus in var |
| `margin_pct` | stability margin (Pmax - P)/Pmax in percent |

- **Method:** $P = V_{1}$ V2 sin(delta)/X; Q_sent = (V2^2 - V1 V2 cos delta)/X; Q_received = (V1 V2 cos delta - V1^2)/X
- **Assumes:** lossless transfer reactance; |P| < V1 $V_{2}/X$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_power_angle', V1=100000.0, V2=100000.0, X=100.0, P=50000000.0)
# -> delta_deg = 30, Pmax = 1e+08 W, Q_sent = 1.33975e+07 var, Q_received = -1.33975e+07 var, margin_pct = 50   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Power angle: V1 = 100 kV, V2 = 100 kV, X = 100 ohm, P = 50 MW | SOLVED | delta_deg = 30, Pmax = 1e+08 W, Q_sent = 1.33975e+07 var, Q_received = -1.33975e+07 var, margin_pct = 50 |
| Power angle: V1 = 100 kV, V2 = 100 kV, X = 100 ohm, P = 150 MW | INVALID_INPUT | Power transfer and power angle between two voltages: the power exceeds the steady-state limit V1 V2/X. |

<a id="energy_critical_clearing"></a>

### Critical clearing angle and time (equal-area criterion) — `energy_critical_clearing`

The initial rotor angle, the largest allowable angle and the critical clearing angle of a machine against an infinite bus by the equal-area criterion, for given pre-fault, during-fault and post-fault transfer limits; with H and f, the critical clearing time for a fault that blocks all transfer.

$$
\cos\delta_{cr} = \frac{P_m(\delta_{max}-\delta_0) + P_{max,3}\cos\delta_{max} - P_{max,2}\cos\delta_0}{P_{max,3} - P_{max,2}}
$$

| input | meaning | condition |
|---|---|---|
| `Pm` | mechanical input power (pu) |  |
| `Pmax_pre` | pre-fault transfer limit (pu) |  |
| `Pmax_fault` | transfer limit during the fault (pu; 0 for a fault at the terminals) |  |
| `Pmax_post` | post-fault transfer limit (pu) |  |
| `H` | inertia constant H (MJ/MVA, i.e. seconds), for the clearing time |  |
| `f` | system frequency, for the clearing time |  |

| output | meaning |
|---|---|
| `delta0_deg` | initial rotor angle in degrees |
| `delta_max_deg` | largest angle before loss of synchronism in degrees |
| `delta_cr_deg` | critical clearing angle in degrees |
| `t_cr` | critical clearing time (Pmax_fault = 0) |

- **Method:** cos(delta_cr) = [Pm (delta_max - delta0) + Pmax_post cos(delta_max) - Pmax_fault cos(delta0)]/(Pmax_post - Pmax_fault); $t_cr = sqrt$(4 H (delta_cr - delta0)/(omega_s Pm))
- **Assumes:** Pmax_fault < Pm < Pmax_pre, Pmax_post; classical model (constant voltage behind transient reactance, no damping)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_critical_clearing', Pm=1.0, Pmax_pre=2.0, Pmax_fault=0.0, Pmax_post=2.0)
# -> delta0_deg = 30, delta_max_deg = 150, delta_cr_deg = 79.562   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Critical clearing: Pm = 1, Pmax_pre = 2, Pmax_fault = 0, Pmax_post = 2 | SOLVED | delta0_deg = 30, delta_max_deg = 150, delta_cr_deg = 79.562 |
| Critical clearing: Pm = 1, Pmax_pre = 2, Pmax_fault = 1.5, Pmax_post = 2 | INVALID_INPUT | Critical clearing angle and time (equal-area criterion): the during-fault transfer limit must be below the mechanical po |

<a id="energy_droop_sharing"></a>

### Frequency deviation and load sharing under governor droop — `energy_droop_sharing`

The steady-state frequency drop after a load step and each generator's output change when governors with droop R (per unit on their own rating) share the load, with optional load damping D.

$$
\Delta f = -\frac{\Delta P_L}{\sum_i \frac{S_i}{R_i f_0} + D}
$$

| input | meaning | condition |
|---|---|---|
| `R` | droop of each governor in per unit on its own rating (e.g. 0.05) |  |
| `ratings` | rating of each generator in MW |  |
| `dP` | load increase in MW (negative for a load decrease) |  |
| `f0` | nominal frequency |  |
| `D` | load damping in MW/Hz (default 0) |  |

| output | meaning |
|---|---|
| `df` | frequency change in Hz |
| `f` | new frequency in Hz |
| `dP_gen` | output change of each generator in MW |
| `dP_load_damping` | load relief from damping in MW |

- **Method:** beta = sum(S_i/(R_i f0)) + D; $df = -dP/\beta$; $dP_i = -df$ S_i/(R_i f0)
- **Assumes:** primary control only (no secondary/AGC); R, ratings > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_droop_sharing', R=[0.05, 0.05], ratings=[500, 500], dP=100, f0=50)
# -> df = -0.25, f = 49.75, dP_gen = [50.0, 50.0], dP_load_damping = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Droop sharing: R = [0.05, 0.05], ratings = [500, 500], dP = 100 MW, f0 = 50 Hz | SOLVED | df = -0.25, f = 49.75, dP_gen = [50.0, 50.0], dP_load_damping = 0 |
| Droop sharing: R = [0.05, 0.05], ratings = [500], dP = 100 MW, f0 = 50 Hz | INVALID_INPUT | Frequency deviation and load sharing under governor droop: give one droop and one rating per generator. |

<a id="energy_transformer_performance"></a>

### Transformer regulation and efficiency from its equivalent circuit — `energy_transformer_performance`

Primary voltage needed, voltage regulation, primary current and input power factor, copper and core losses and efficiency of a single-phase transformer (approximate equivalent circuit referred to the primary, shunt branch at the primary terminals) delivering a fraction of its rating at rated secondary voltage.

$$
\mathbf V_1 = a\mathbf V_2 + \frac{\mathbf I_2}{a}(R_{eq} + jX_{eq}),\quad \eta = \frac{P_{out}}{P_{out} + P_{cu} + P_{core}}
$$

| input | meaning | condition |
|---|---|---|
| `S` | rating in VA |  |
| `V1` | rated primary voltage |  |
| `V2` | rated secondary voltage |  |
| `Req` | series resistance referred to the primary |  |
| `Xeq` | series leakage reactance referred to the primary |  |
| `Rc` | core-loss resistance (primary side) |  |
| `Xm` | magnetizing reactance (primary side) |  |
| `load` | load as a fraction of the rating (or a percentage) |  |
| `pf` | load power factor |  |
| `pf_type` | lagging (default) or leading load |  |

| output | meaning |
|---|---|
| `V1_required` | primary voltage for rated secondary voltage at this load |
| `regulation_pct` | voltage regulation in percent |
| `I1` | primary current |
| `pf_in` | input power factor |
| `P_out` | output power |
| `P_cu` | copper loss |
| `P_core` | core loss |
| `efficiency_pct` | efficiency in percent |

- **Method:** $V_{1} = a$ $V_{2} + I_{2}$' (Req + jXeq), I2' = $I_{2}/a$ at angle -acos(pf); P_cu = |I2'|^2 Req; P_core = V1^2/Rc; eta = P_out/(P_out + P_cu + P_core)
- **Assumes:** approximate equivalent circuit (shunt branch at the primary terminals); 0 < load <= 2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_transformer_performance', S=10000.0, V1=1000, V2=100, Req=1.0, Xeq=0.0, Rc=1000000.0, Xm=1000000000.0, load=1.0, pf=1.0)
# -> V1_required = 1010 V, regulation_pct = 1, I1 = 10.001 A, pf_in = 1, P_out = 10000 W, P_cu = 100 W, P_core = 1.0201 W, efficiency_pct = 98.9999   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_transformer_performance', S=10000.0, V1=1000, V2=100, Req=1.0, Xeq=0.0, Rc=1000000000.0, Xm=1000000000.0, load=1.0, pf=1.0)
# -> V1_required = 1010 V, regulation_pct = 1, I1 = 10 A, pf_in = 1, P_out = 10000 W, P_cu = 100 W, P_core = 0.0010201 W, efficiency_pct = 99.0099   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Transformer regulation and efficiency: S = 10 kVA, V1 = 1000 V, V2 = 100 V, Req = 1 ohm, Xeq = 0 ohm, Rc = 1e9 ohm, Xm = 1e9 ohm, load = 100%, pf = 1 | SOLVED | V1_required = 1010 V, regulation_pct = 1, I1 = 10 A, pf_in = 1, P_out = 10000 W, P_cu = 100 W, P_core = 0.0010201 W, efficiency_pct = 99.0099 |
| Transformer regulation and efficiency: S = 10 kVA, V1 = 1000 V, V2 = 100 V, Req = 1 ohm, Xeq = 0 ohm, Rc = 1e9 ohm, Xm = 1e9 ohm, load = 300%, pf = 1 | INVALID_INPUT | Transformer regulation and efficiency from its equivalent circuit: the load must lie between 0 and 2 times the rating (a |

<a id="energy_transformer_tests"></a>

### Transformer parameters from open-circuit and short-circuit tests — `energy_transformer_tests`

Core-loss resistance and magnetizing reactance from the open-circuit test, and the equivalent series resistance, reactance and impedance from the short-circuit test (each on the side where the test was made).

$$
R_c = \frac{V_{oc}^2}{P_{oc}},\ X_m = \frac{1}{\sqrt{Y_{oc}^2 - G_c^2}},\ R_{eq} = \frac{P_{sc}}{I_{sc}^2},\ X_{eq} = \sqrt{Z_{eq}^2 - R_{eq}^2}
$$

| input | meaning | condition |
|---|---|---|
| `Voc` | open-circuit test voltage |  |
| `Ioc` | open-circuit test current |  |
| `Poc` | open-circuit test power |  |
| `Vsc` | short-circuit test voltage |  |
| `Isc` | short-circuit test current |  |
| `Psc` | short-circuit test power |  |

| output | meaning |
|---|---|
| `Rc` | core-loss resistance (open-circuit side) |
| `Xm` | magnetizing reactance (open-circuit side) |
| `Req` | equivalent series resistance (short-circuit side) |
| `Xeq` | equivalent leakage reactance (short-circuit side) |
| `Zeq` | equivalent series impedance magnitude |
| `pf_oc` | open-circuit power factor |
| `pf_sc` | short-circuit power factor |

- **Method:** $Rc = V_oc^{2}/P_oc$, $Xm = \frac{1}{\sqrt{\left(\frac{I_{oc}}{V_{oc}}\right)^{2} - \left(\frac{P_{oc}}{V_{oc}^{2}}\right)^{2}}}$; Req = P_sc/I_sc^2, $Xeq = \sqrt{\left(\frac{V_{sc}}{I_{sc}}\right)^{2} - Req^{2}}$
- **Assumes:** $P < V$ I in both tests
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_transformer_tests', Voc=100, Ioc=1, Poc=60, Vsc=50, Isc=10, Psc=300)
# -> Rc = 166.667 ohm, Xm = 125 ohm, Req = 3 ohm, Xeq = 4 ohm, Zeq = 5 ohm, pf_oc = 0.6, pf_sc = 0.6   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Transformer tests: Voc = 100 V, Ioc = 1 A, Poc = 60 W, Vsc = 50 V, Isc = 10 A, Psc = 300 W | SOLVED | Rc = 166.667 ohm, Xm = 125 ohm, Req = 3 ohm, Xeq = 4 ohm, Zeq = 5 ohm, pf_oc = 0.6, pf_sc = 0.6 |
| Transformer tests: Voc = 100 V, Ioc = 1 A, Poc = 160 W, Vsc = 50 V, Isc = 10 A, Psc = 300 W | INVALID_INPUT | Transformer parameters from open-circuit and short-circuit tests: the open-circuit power must be below V I (power factor |

<a id="energy_transformer_max_efficiency"></a>

### Load for maximum transformer efficiency — `energy_transformer_max_efficiency`

The load (fraction of rating and apparent power) at which a transformer's efficiency is greatest (copper loss equal to core loss), that maximum efficiency and the full-load efficiency at a power factor.

$$
x_{\eta,max} = \sqrt{\frac{P_{core}}{P_{cu,fl}}},\quad \eta_{max} = \frac{xS\cos\phi}{xS\cos\phi + 2P_{core}}
$$

| input | meaning | condition |
|---|---|---|
| `S` | rating in VA |  |
| `P_core` | core (no-load) loss |  |
| `P_cu_fl` | copper loss at full load |  |
| `pf` | load power factor |  |

| output | meaning |
|---|---|
| `load_fraction` | load fraction for maximum efficiency sqrt(P_core/P_cu) |
| `S_at_max` | apparent power at maximum efficiency in VA |
| `efficiency_max_pct` | maximum efficiency in percent |
| `efficiency_full_load_pct` | full-load efficiency in percent |

- **Method:** $x = sqrt$(P_core/P_cu,fl); eta_max = x S pf/(x S $pf + 2$ P_core)
- **Assumes:** constant core loss, copper loss proportional to the load squared
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_transformer_max_efficiency', S=100000.0, P_core=500, P_cu_fl=2000, pf=1.0)
# -> load_fraction = 0.5, S_at_max = 50000 VA, efficiency_max_pct = 98.0392, efficiency_full_load_pct = 97.561   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Maximum transformer efficiency: S = 100 kVA, P_core = 500 W, P_cu_fl = 2000 W, pf = 1 | SOLVED | load_fraction = 0.5, S_at_max = 50000 VA, efficiency_max_pct = 98.0392, efficiency_full_load_pct = 97.561 |
| Maximum transformer efficiency: S = 100 kVA, P_core = 0 W, P_cu_fl = 2000 W, pf = 1 | INVALID_INPUT | Load for maximum transformer efficiency: the rating and the losses must be a positive number. |

<a id="energy_transformer_loading"></a>

### Transformer under load from nameplate data — `energy_transformer_loading`

Secondary voltage, losses, efficiency and loading of a transformer fed at rated primary voltage and supplying a P + jQ load, from its nameplate: rating, rated voltages, short-circuit voltage vk and its resistive part vkr, no-load loss and no-load current (T equivalent circuit).

$$
z_k = v_{kr} + j\sqrt{v_k^2 - v_{kr}^2},\quad y_m = \frac{P_0}{S_n} - j\sqrt{i_0^2 - \left(\frac{P_0}{S_n}\right)^2}
$$

| input | meaning | condition |
|---|---|---|
| `sn` | rating in VA |  |
| `V_hv` | rated (and applied) high-voltage side voltage |  |
| `V_lv` | rated low-voltage side voltage |  |
| `vk` | short-circuit voltage as a fraction or percentage of rated voltage |  |
| `vkr` | resistive part of the short-circuit voltage (fraction or %) |  |
| `P0` | no-load (iron) loss |  |
| `i0` | no-load current as a fraction or percentage of rated current |  |
| `P` | load real power on the low-voltage side |  |
| `Q` | load reactive power in var |  |

| output | meaning |
|---|---|
| `V_lv_actual` | low-voltage side voltage under load |
| `V_lv_pu` | low-voltage side voltage in per unit |
| `delta_deg` | phase shift of the secondary voltage in degrees (no vector-group shift) |
| `P_loss` | total real loss |
| `Q_loss` | reactive power absorbed in var |
| `efficiency_pct` | efficiency in percent |
| `loading_pct` | loading: the larger winding current over its rated current, percent |

- **Method:** per unit on the rating: z = vkr + j sqrt(vk^2 - vkr^2), $y_m = P_{0}/S - j$ sqrt(i0^2 - (P0/S)^2); the T circuit solved for the load-bus voltage by Newton iteration
- **Assumes:** T equivalent circuit (series impedance halved on each side of the magnetizing branch); rated voltage applied, nominal tap
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_transformer_loading', sn=630000.0, V_hv=20000.0, V_lv=400, vk=0.06, vkr=0.012, P0=1500, i0=0.003, P=400000.0, Q=150000.0)
# -> V_lv_actual = 390.876 V, V_lv_pu = 0.977189, delta_deg = -2.02483, P_loss = 5120.79 W, Q_loss = 19030.9 var, efficiency_pct = 98.736, loading_pct = 69.6777   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Transformer under load: sn = 630 kVA, V_hv = 20 kV, V_lv = 400 V, vk = 6%, vkr = 1.2%, P0 = 1.5 kW, i0 = 0.3%, P = 400 kW, Q = 150 kvar | SOLVED | V_lv_actual = 390.876 V, V_lv_pu = 0.977189, delta_deg = -2.02483, P_loss = 5120.79 W, Q_loss = 19030.9 var, efficiency_pct = 98.736, loading_pct = 69.6777 |
| Transformer under load: sn = 630 kVA, V_hv = 20 kV, V_lv = 400 V, vk = 6%, vkr = 7%, P0 = 1.5 kW, i0 = 0.3%, P = 400 kW, Q = 150 kvar | INVALID_INPUT | Transformer under load from nameplate data: need 0 < vkr < vk (short-circuit voltages as fractions or percentages, vk be |

<a id="energy_parallel_transformers"></a>

### Load sharing between two transformers in parallel — `energy_parallel_transformers`

How two transformers with equal voltage ratios but different ratings and short-circuit impedances share a load: the apparent, real and reactive power each delivers and its loading on its own rating.

$$
\mathbf S_1 = \mathbf S_L\left(\frac{\mathbf Z_2}{\mathbf Z_1 + \mathbf Z_2}\right)^*,\quad \mathbf S_2 = \mathbf S_L\left(\frac{\mathbf Z_1}{\mathbf Z_1 + \mathbf Z_2}\right)^*
$$

| input | meaning | condition |
|---|---|---|
| `S1` | rating of transformer 1 (VA) |  |
| `vk1` | short-circuit voltage of transformer 1 (fraction or %) |  |
| `vkr1` | its resistive part (fraction or %) |  |
| `S2` | rating of transformer 2 (VA) |  |
| `vk2` | short-circuit voltage of transformer 2 (fraction or %) |  |
| `vkr2` | its resistive part (fraction or %) |  |
| `V_lv` | common secondary rated voltage |  |
| `P` | load real power |  |
| `Q` | load reactive power in var |  |

| output | meaning |
|---|---|
| `S1_share` | apparent power delivered by transformer 1 in VA |
| `S2_share` | apparent power delivered by transformer 2 in VA |
| `P1` | real power of transformer 1 |
| `Q1` | reactive power of transformer 1 in var |
| `P2` | real power of transformer 2 |
| `Q2` | reactive power of transformer 2 in var |
| `share1_pct_of_rating` | transformer 1 output over its rating, percent |
| `share2_pct_of_rating` | transformer 2 output over its rating, percent |

- **Method:** impedances in ohms on the common secondary: Z_i = (vkr_i + j sqrt(vk_i^2 - vkr_i^2)) $V^{2}/S_i$; $S_1 = S_L$ conj($Z_2/(Z_1 + Z_2)$), $S_2 = S_L$ conj($Z_1/(Z_1 + Z_2)$)
- **Assumes:** equal voltage ratios (no circulating current); magnetizing branches neglected
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_parallel_transformers', S1=500000.0, vk1=0.05, vkr1=0.01, S2=500000.0, vk2=0.05, vkr2=0.01, V_lv=400, P=600000.0, Q=200000.0)
# -> S1_share = 316228 VA, S2_share = 316228 VA, P1 = 300000 W, Q1 = 100000 var, P2 = 300000 W, Q2 = 100000 var, share1_pct_of_rating = 63.2456, share2_pct_of_rating = 63.2456   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Parallel transformers: S1 = 500 kVA, vk1 = 5%, vkr1 = 1%, S2 = 500 kVA, vk2 = 5%, vkr2 = 1%, V_lv = 400 V, P = 600 kW, Q = 200 kvar | SOLVED | S1_share = 316228 VA, S2_share = 316228 VA, P1 = 300000 W, Q1 = 100000 var, P2 = 300000 W, Q2 = 100000 var, share1_pct_of_rating = 63.2456, share2_pct_of_rating = 63.2456 |
| Parallel transformers: S1 = 500 kVA, vk1 = 5%, vkr1 = 6%, S2 = 500 kVA, vk2 = 5%, vkr2 = 1%, V_lv = 400 V, P = 600 kW, Q = 200 kvar | INVALID_INPUT | Load sharing between two transformers in parallel: need 0 <= vkr < vk below 50% for each transformer. |

<a id="energy_induction_motor"></a>

### Induction motor performance from its equivalent circuit — `energy_induction_motor`

Line and rotor currents, power factor, air-gap and converted power, induced torque, output and efficiency of a three-phase induction motor at a slip, from its per-phase equivalent circuit.

$$
T_{ind} = \frac{3 I_2^2 R_2/s}{\omega_s},\quad n_s = \frac{120 f}{p}
$$

| input | meaning | condition |
|---|---|---|
| `VL` | line voltage |  |
| `f` | supply frequency |  |
| `poles` | number of poles (even) |  |
| `R1` | stator resistance per phase |  |
| `X1` | stator leakage reactance per phase |  |
| `R2` | rotor resistance per phase referred to the stator |  |
| `X2` | rotor leakage reactance referred to the stator |  |
| `Xm` | magnetizing reactance per phase |  |
| `slip` | slip $(0 < s  \leq  1)$ |  |
| `connection` | stator connection |  |
| `P_rot` | rotational (friction, windage and core) loss, default 0 |  |

| output | meaning |
|---|---|
| `I_line` | line current |
| `I2` | rotor current referred to the stator |
| `pf` | power factor |
| `P_in` | input power |
| `P_ag` | air-gap power |
| `P_conv` | converted (developed) power |
| `T` | induced torque |
| `P_out` | output power |
| `efficiency_pct` | efficiency in percent |

- **Method:** Z = R1 + jX1 + jXm || $(R_{2}/s + jX2)$; $P_ag = 3$ $I_{2}^{2}$ $R_{2}/s$; P_conv = (1 - s) P_ag; $T = P_ag/omega_s$; $n_s = 120$ f/p
- **Assumes:** steady state, balanced supply; 0 < slip <= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_induction_motor', VL=692.8203230275509, f=50, poles=4, R1=0.0, X1=2.0, R2=1.5, X2=2.0, Xm=1000000000000.0, slip=0.5, connection='star')
# -> I_line = 80 A, I2 = 80 A, pf = 0.6, P_in = 57600 W, P_ag = 57600 W, P_conv = 28800 W, T = 366.693 N*m, P_out = 28800 W, efficiency_pct = 50   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Induction motor: VL = 460 V, f = 60 Hz, poles = 4, R1 = 0.641 ohm, X1 = 1.106 ohm, R2 = 0.332 ohm, X2 = 0.464 ohm, Xm = 26.3 ohm, slip = 2.2%, connection = star | SOLVED | I_line = 18.8919 A, I2 = 16.171 A, pf = 0.832122, P_in = 12525.1 W, P_ag = 11838.8 W, P_conv = 11578.4 W, T = 62.8068 N*m, P_out = 11578.4 W, efficiency_pct = 92.4409 |
| Induction motor: VL = 460 V, f = 60 Hz, poles = 5, R1 = 0.641 ohm, X1 = 1.106 ohm, R2 = 0.332 ohm, X2 = 0.464 ohm, Xm = 26.3 ohm, slip = 2.2%, connection = star | INVALID_INPUT | Induction motor performance from its equivalent circuit: the number of poles must be an even whole number (2, 4, 6, ...) |

<a id="energy_induction_max_torque"></a>

### Induction motor breakdown (maximum) torque and starting torque — `energy_induction_max_torque`

The slip at maximum torque, the maximum (breakdown) torque, the starting torque and current, and the Thevenin equivalent of the stator and magnetizing branch, for a three-phase induction motor.

$$
s_{max} = \frac{R_2}{\sqrt{R_{th}^2 + (X_{th} + X_2)^2}},\quad T_{max} = \frac{3V_{th}^2}{2\omega_s\left[R_{th} + \sqrt{R_{th}^2 + (X_{th}+X_2)^2}\right]}
$$

| input | meaning | condition |
|---|---|---|
| `VL` | line voltage |  |
| `f` | supply frequency |  |
| `poles` | number of poles (even) |  |
| `R1` | stator resistance per phase |  |
| `X1` | stator leakage reactance per phase |  |
| `R2` | rotor resistance per phase referred to the stator |  |
| `X2` | rotor leakage reactance referred to the stator |  |
| `Xm` | magnetizing reactance per phase |  |
| `connection` | stator connection |  |

| output | meaning |
|---|---|
| `s_maxT` | slip at maximum torque |
| `T_max` | maximum (pull-out) torque |
| `T_start` | starting torque $(s = 1)$ |
| `I_start` | starting line current |
| `V_th` | Thevenin voltage per phase |
| `R_th` | Thevenin resistance |
| `X_th` | Thevenin reactance |

- **Method:** Thevenin: $V_th = V$ jXm/(R1 + j(X1 + Xm)), $Z_th = Z_{1}$ || jXm; $s_{max} = \frac{R_{2}}{\sqrt{R_{th}^{2} + \left(X_{th} + X_{2}\right)^{2}}}$; T_max = 3 V_th^2/(2 omega_s (R_th + sqrt(R_th^2 + (X_th + X2)^2)))
- **Assumes:** constant parameters (no deep-bar or saturation effects)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_induction_max_torque', VL=692.8203230275509, f=50, poles=4, R1=0.0, X1=1.0, R2=0.2, X2=1.0, Xm=1000000000000.0, connection='star')
# -> s_maxT = 0.1, T_max = 763.944 N*m, T_start = 151.276 N*m, I_start = 199.007 A, V_th = 400 V, R_th = 0 ohm, X_th = 1 ohm   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Induction motor maximum torque: VL = 460 V, f = 60 Hz, poles = 4, R1 = 0.641 ohm, X1 = 1.106 ohm, R2 = 0.332 ohm, X2 = 0.464 ohm, Xm = 26.3 ohm | NEEDS_INPUT | energy_induction_max_torque needs connection (stator connection). Example: solve('energy_induction_max_torque', VL='692. |
| Induction motor breakdown torque: VL = 460 V, f = 60 Hz, poles = 4, R1 = 0 ohm, X1 = 1 ohm, R2 = 0.2 ohm, X2 = 1 ohm, Xm = 1e12 ohm, connection = star | SOLVED | s_maxT = 0.1, T_max = 280.643 N*m, T_start = 55.5729 N*m, I_start = 132.132 A, V_th = 265.581 V, R_th = 0 ohm, X_th = 1 ohm |

<a id="energy_synchronous_generator"></a>

### Synchronous generator internal EMF, power angle and regulation — `energy_synchronous_generator`

The internal (excitation) EMF, power angle and voltage regulation of a star-connected cylindrical-rotor synchronous generator delivering a current at a power factor, from its synchronous reactance and armature resistance.

$$
\mathbf E_A = \mathbf V_\phi + (R_A + jX_S)\mathbf I_A
$$

| input | meaning | condition |
|---|---|---|
| `VL` | terminal line voltage |  |
| `IL` | armature (line) current |  |
| `pf` | power factor |  |
| `Xs` | synchronous reactance per phase |  |
| `Ra` | armature resistance per phase (default 0) |  |
| `pf_type` | lagging (default) or leading |  |

| output | meaning |
|---|---|
| `E_line` | internal EMF (line value) |
| `E_phase` | internal EMF per phase |
| `delta_deg` | power (torque) angle in degrees |
| `regulation_pct` | voltage regulation (E - V)/V in percent |
| `P` | real power output |
| `Q` | reactive power output in var |

- **Method:** E = V + (Ra + jXs) I per phase, I at angle -acos(pf)
- **Assumes:** cylindrical rotor (no saliency), star connection, unsaturated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_synchronous_generator', VL=173.20508075688772, IL=10, pf=1.0, Xs=10.0)
# -> E_line = 244.949 V, E_phase = 141.421 V, delta_deg = 45, regulation_pct = 41.4214, P = 3000 W, Q = 0 var   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Synchronous generator: VL = 173.20508075688772 V, IL = 10 A, pf = 1, Xs = 10 ohm | SOLVED | E_line = 244.949 V, E_phase = 141.421 V, delta_deg = 45, regulation_pct = 41.4214, P = 3000 W, Q = 0 var |
| Synchronous generator: VL = 480 V, IL = 60 A, pf = 0.8, Xs = 0 ohm | INVALID_INPUT | Synchronous generator internal EMF, power angle and regulation: the terminal voltage, the current and the synchronous re |

<a id="energy_peukert"></a>

### Peukert exponent and run time at a new discharge current — `energy_peukert`

Peukert's law $t = C_p/I^{k}$ fitted (least squares on the logarithms) to two or more constant-current discharge tests, then the run time and delivered capacity at another current.

$$
t = \frac{C_p}{I^k}\;\Rightarrow\; \ln t = \ln C_p - k\ln I
$$

| input | meaning | condition |
|---|---|---|
| `currents` | test discharge currents in A |  |
| `times` | run time of each test in hours |  |
| `I` | new discharge current |  |

| output | meaning |
|---|---|
| `k` | Peukert exponent |
| `C_p` | Peukert capacity C_p in $A^{k}$ h |
| `runtime_h` | run time at I in hours |
| `capacity_Ah` | capacity delivered at I in Ah |

- **Method:** ln $t = ln$ C_p - k ln I fitted by least squares; t(I) = C_p $I^{-k}$
- **Assumes:** at least two tests at different currents; constant temperature and cut-off voltage
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_peukert', currents=[1, 10], times=[100, 10], I=5)
# -> k = 1, C_p = 100, runtime_h = 20, capacity_Ah = 100   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_peukert', currents='2, 8', times='50, 10', I=4)
# -> k = 1.16096, C_p = 111.803, runtime_h = 22.3607, capacity_Ah = 89.4427   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Peukert: currents = [1, 10], times = [100, 10], I = 5 A | SOLVED | k = 1, C_p = 100, runtime_h = 20, capacity_Ah = 100 |
| Peukert: currents = [5, 5], times = [20, 4], I = 10 A | INVALID_INPUT | Peukert exponent and run time at a new discharge current: the test currents must be different. |

<a id="energy_battery_pack_sizing"></a>

### Battery pack series-parallel sizing — `energy_battery_pack_sizing`

The number of cells in series to reach a pack voltage and of strings in parallel to reach a pack energy, with the resulting pack voltage, energy and capacity (fewest cells; nominal values).

$$
N_s = \left\lceil\frac{V_{pack}}{V_{cell}}\right\rceil,\quad N_p = \left\lceil\frac{E_{pack}}{N_s V_{cell} Q_{cell}}\right\rceil
$$

| input | meaning | condition |
|---|---|---|
| `V_target` | required nominal pack voltage |  |
| `E_target` | required pack energy (Wh or kWh) |  |
| `V_cell` | nominal cell voltage |  |
| `Q_cell` | cell capacity (Ah accepted) |  |

| output | meaning |
|---|---|
| `N_series` | cells in series |
| `N_parallel` | strings in parallel |
| `N_cells` | total cells |
| `V_pack` | nominal pack voltage |
| `E_pack_kWh` | pack energy in kWh |
| `Q_pack_Ah` | pack capacity in Ah |

- **Method:** N_s = ceil(V/V_cell); N_p = ceil(E/(N_s V_cell Q_cell))
- **Assumes:** nominal voltages and capacities; a target met within $1e-9$ counts as met
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_battery_pack_sizing', V_target=48, E_target=36000000.0, V_cell=3.2, Q_cell=360000)
# -> N_series = 15, N_parallel = 3, N_cells = 45, V_pack = 48 V, E_pack_kWh = 14.4, Q_pack_Ah = 300   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_battery_pack_sizing', V_target=400, E_target=216000000.0, V_cell=3.7, Q_cell=18000)
# -> N_series = 109, N_parallel = 30, N_cells = 3270, V_pack = 403.3 V, E_pack_kWh = 60.495, Q_pack_Ah = 150   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Battery pack sizing: V_target = 48 V, E_target = 10 kWh, V_cell = 3.2 V, Q_cell = 100 Ah | SOLVED | N_series = 15, N_parallel = 3, N_cells = 45, V_pack = 48 V, E_pack_kWh = 14.4, Q_pack_Ah = 300 |
| Battery pack sizing: V_target = 48 V, E_target = 10 kWh, V_cell = 0 V, Q_cell = 100 Ah | INVALID_INPUT | Battery pack series-parallel sizing: the target voltage and energy and the cell voltage and capacity must be a positive  |

<a id="energy_battery_rc_discharge"></a>

### Battery terminal voltage under constant current (one-RC equivalent circuit) — `energy_battery_rc_discharge`

Terminal voltage, polarization (RC) voltage, energy delivered and heat generated after a constant-current discharge of duration t, for a cell modelled as an open-circuit voltage with a series resistance R0 and one R1 || C1 branch (constant OCV).

$$
V_t(t) = V_{oc} - IR_0 - IR_1\left(1 - e^{-t/R_1C_1}\right)
$$

| input | meaning | condition |
|---|---|---|
| `OCV` | open-circuit voltage (held constant) |  |
| `R0` | series (ohmic) resistance |  |
| `R1` | polarization resistance |  |
| `C1` | polarization capacitance |  |
| `I` | discharge current |  |
| `t` | time since the current step |  |

| output | meaning |
|---|---|
| `V_terminal` | terminal voltage |
| `V_rc` | voltage across the RC branch |
| `E_delivered` | energy delivered to the load |
| `E_Wh` | energy delivered in Wh |
| `heat` | heat generated in R0 and R1 |
| `tau` | time constant R1 C1 |

- **Method:** $V_{1} = I$ R1 (1 - e^(-t/tau)), V_t = OCV - I $R_{0} - V_{1}$; energy and heat integrated in closed form
- **Assumes:** constant open-circuit voltage over the interval (short pulse or flat OCV curve); RC branch initially relaxed
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_battery_rc_discharge', OCV=4.0, R0=0.01, R1=0.02, C1=1000, I=10, t=0)
# -> V_terminal = 3.9 V, V_rc = 0 V, E_delivered = 0 J, E_Wh = 0, heat = 0 J, tau = 20 s   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_battery_rc_discharge', OCV=4.0, R0=0.01, R1=0.02, C1=1000, I=10, t=13.862943611198906)
# -> V_terminal = 3.8 V, V_rc = 0.1 V, E_delivered = 532.929 J, E_Wh = 0.148036, heat = 16.5888 J, tau = 20 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Battery RC model: OCV = 4 V, R0 = 10 mohm, R1 = 20 mohm, C1 = 1000 F, I = 10 A, t = 0 s | SOLVED | V_terminal = 3.9 V, V_rc = 0 V, E_delivered = 0 J, E_Wh = 0, heat = 0 J, tau = 20 s |
| Battery RC model: OCV = 4 V, R0 = 10 mohm, R1 = 20 mohm, C1 = 1000 F, I = 500 A, t = 60 s | INVALID_INPUT | Battery terminal voltage under constant current (one-RC equivalent circuit): the terminal voltage would fall to zero: th |

<a id="energy_fuel_cell_stack"></a>

### Hydrogen fuel-cell stack: consumption, power and efficiency — `energy_fuel_cell_stack`

Hydrogen and oxygen consumption, water production (Faraday's law), efficiency on the lower and higher heating value and heat released of a hydrogen fuel-cell stack of N cells at a current and cell voltage.

$$
\dot n_{H_2} = \frac{N I}{2F},\quad \eta_{LHV} = \frac{N I V_{cell}}{\dot n_{H_2}\,\Delta h_{LHV}}
$$

| input | meaning | condition |
|---|---|---|
| `I` | stack current |  |
| `N` | number of cells in series |  |
| `V_cell` | average cell voltage |  |
| `utilization` | fuel utilization (fraction or %, default 1) |  |

| output | meaning |
|---|---|
| `H2_kg_s` | hydrogen consumed |
| `H2_g_h` | hydrogen consumed in g/h |
| `H2_fed_kg_s` | hydrogen fed (consumed/utilization) |
| `O2_kg_s` | oxygen consumed |
| `H2O_kg_s` | water produced |
| `efficiency_LHV` | efficiency on the lower heating value |
| `efficiency_HHV` | efficiency on the higher heating value |
| `heat_LHV` | heat released (water vapour product) |

- **Method:** $n_H2 = N$ $I/(2F)$; $n_O2 = n_H2/2$; $n_H2O = n_H2$; $P = N$ I V; eta = P/(n_H2 LHV)
- **Assumes:** hydrogen/oxygen cell, 2 electrons per H2; heating values at 25 degC
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_fuel_cell_stack', I=192970.66424, N=1, V_cell=0.7)
# -> H2_kg_s = 0.00201588 kg/s, H2_g_h = 7257.17, H2_fed_kg_s = 0.00201588 kg/s, O2_kg_s = 0.0159994 kg/s, H2O_kg_s = 0.0180153 kg/s, efficiency_LHV = 0.558581, efficiency_HHV = 0.472593, heat_LHV = 106747 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Fuel-cell stack: I = 192970.66424 A, N = 1, V_cell = 0.7 V | SOLVED | H2_kg_s = 0.00201588 kg/s, H2_g_h = 7257.17, H2_fed_kg_s = 0.00201588 kg/s, O2_kg_s = 0.0159994 kg/s, H2O_kg_s = 0.0180153 kg/s, efficiency_LHV = 0.558581, efficiency_HHV = 0.472593, heat_LHV = 106747 W |
| Fuel-cell stack: I = 200 A, N = 100, V_cell = 1.4 V | INVALID_INPUT | Hydrogen fuel-cell stack: consumption, power and efficiency: a hydrogen fuel cell's voltage must stay below the thermone |

<a id="energy_electrolyser"></a>

### Water electrolyser: hydrogen production and energy per kilogram — `energy_electrolyser`

Hydrogen and oxygen production, water consumption, specific energy (kWh per kg H2) and efficiency on the higher and lower heating value of a water electrolyser of N cells.

$$
\frac{E}{m_{H_2}} = \frac{2FV_{cell}}{\eta_F M_{H_2}}
$$

| input | meaning | condition |
|---|---|---|
| `I` | stack current |  |
| `N` | number of cells in series |  |
| `V_cell` | cell voltage (1.23 to 3 V) |  |
| `faradaic` | Faradaic (current) efficiency, fraction or % (default 1) |  |

| output | meaning |
|---|---|
| `H2_kg_s` | hydrogen produced |
| `H2_kg_h` | hydrogen produced in kg/h |
| `O2_kg_s` | oxygen produced |
| `H2O_kg_s` | water consumed |
| `specific_energy_kWh_per_kg` | electricity per kg of hydrogen in kWh/kg |
| `efficiency_HHV` | efficiency on the higher heating value |
| `efficiency_LHV` | efficiency on the lower heating value |

- **Method:** $n_H2 = eta_F$ N $I/(2F)$; specific energy = N I V/(n_H2 M_H2); eta_HHV = n_H2 HHV/P
- **Assumes:** water electrolysis, 2 electrons per H2; heating values at 25 degC
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_electrolyser', I=192970.66424, N=1, V_cell=2.0)
# -> H2_kg_s = 0.00201588 kg/s, H2_kg_h = 7.25717, O2_kg_s = 0.0159994 kg/s, H2O_kg_s = 0.0180153 kg/s, specific_energy_kWh_per_kg = 53.1807, efficiency_HHV = 0.740594, efficiency_LHV = 0.626587   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Electrolyser: I = 192970.66424 A, N = 1, V_cell = 2 V | SOLVED | H2_kg_s = 0.00201588 kg/s, H2_kg_h = 7.25717, O2_kg_s = 0.0159994 kg/s, H2O_kg_s = 0.0180153 kg/s, specific_energy_kWh_per_kg = 53.1807, efficiency_HHV = 0.740594, efficiency_LHV = 0.626587 |
| Electrolyser: I = 1000 A, N = 100, V_cell = 1 V | INVALID_INPUT | Water electrolyser: hydrogen production and energy per kilogram: an electrolysis cell operates between the reversible vo |

<a id="energy_hydrogen_tank"></a>

### Hydrogen stored in a tank at pressure (real gas) — `energy_hydrogen_tank`

Density, compressibility factor, stored mass and energy (lower heating value) of compressed hydrogen in a tank at a pressure and temperature, by the Leachman (2009) equation of state for normal hydrogen, compared with the ideal-gas estimate.

$$
p = \rho R T\left(1 + \delta\,\frac{\partial\alpha^r}{\partial\delta}\right),\quad Z = \frac{p}{\rho R T}
$$

| input | meaning | condition |
|---|---|---|
| `V` | internal tank volume |  |
| `p` | gas pressure (absolute) |  |
| `T` | gas temperature |  |

| output | meaning |
|---|---|
| `rho` | hydrogen density (real gas) |
| `Z` | compressibility factor |
| `mass` | stored hydrogen mass |
| `rho_ideal` | ideal-gas density p M/(R T) |
| `mass_ideal` | ideal-gas mass |
| `ideal_over_real` | ideal-gas over real density (= Z) |
| `energy_LHV_kWh` | stored chemical energy (LHV) in kWh |

- **Method:** Leachman 2009 Helmholtz equation: $p = \rho$ R T (1 + delta d(alpha_r)/d(delta)), solved for rho; Z = p/(rho R T)
- **Assumes:** 1 kPa <= p <= 1500 bar; 200 $K  \leq  T  \leq  700$ K; normal hydrogen, uniform state
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_hydrogen_tank', V=1.0, p=70000000.0, T=288.15)
# -> rho = 40.1722 kg/m^3, Z = 1.46617, mass = 40.1722 kg, rho_ideal = 58.8992 kg/m^3, mass_ideal = 58.8992 kg, ideal_over_real = 1.46617, energy_LHV_kWh = 1338.63   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hydrogen tank: V = 1 m^3, p = 700 bar, T = 288.15 K | SOLVED | rho = 40.1722 kg/m^3, Z = 1.46617, mass = 40.1722 kg, rho_ideal = 58.8992 kg/m^3, mass_ideal = 58.8992 kg, ideal_over_real = 1.46617, energy_LHV_kWh = 1338.63 |
| Hydrogen tank: V = 1 m^3, p = 700 bar, T = 50 K | INVALID_INPUT | Hydrogen stored in a tank at pressure (real gas): the temperature must lie between 200 K and 700 K (gaseous hydrogen). |

<a id="energy_hydrogen_isothermal_compression"></a>

### Minimum (isothermal) work to compress hydrogen — `energy_hydrogen_isothermal_compression`

The reversible isothermal work per kilogram to compress hydrogen from p1 to p2 at temperature T (the integral of v dp, equal to the Gibbs-energy rise) for the real gas (Leachman equation of state) and the ideal gas.

$$
w = \int_{p_1}^{p_2} v\,dp = g(p_2,T) - g(p_1,T),\quad w_{ideal} = \frac{RT}{M}\ln\frac{p_2}{p_1}
$$

| input | meaning | condition |
|---|---|---|
| `p1` | initial pressure |  |
| `p2` | final pressure |  |
| `T` | compression temperature |  |

| output | meaning |
|---|---|
| `w` | real-gas isothermal work per kg |
| `w_kWh_per_kg` | the same in kWh/kg |
| `w_ideal` | ideal-gas work R T ln(p2/p1)/M |
| `real_over_ideal` | real over ideal work |

- **Method:** $w = g$(p2, T) - g(p1, T), $g/(RT) = ln$(delta) + alpha_r + delta d(alpha_r)/d(delta) + (terms in T only)
- **Assumes:** reversible and isothermal; 200 $K  \leq  T  \leq  700$ K, pressures up to 1500 bar
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_hydrogen_isothermal_compression', p1=100000.0, p2=200000.0, T=300)
# -> w = 858386 J/kg, w_kWh_per_kg = 0.238441, w_ideal = 857663 J/kg, real_over_ideal = 1.00084   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hydrogen isothermal compression: p1 = 1 bar, p2 = 2 bar, T = 300 K | SOLVED | w = 858386 J/kg, w_kWh_per_kg = 0.238441, w_ideal = 857663 J/kg, real_over_ideal = 1.00084 |
| Hydrogen isothermal compression: p1 = 700 bar, p2 = 20 bar, T = 300 K | INVALID_INPUT | Minimum (isothermal) work to compress hydrogen: the final pressure must exceed the initial pressure. |

<a id="energy_hydrogen_isentropic_compression"></a>

### Isentropic (adiabatic) compression of hydrogen — `energy_hydrogen_isentropic_compression`

Outlet temperature and work per kilogram of a reversible adiabatic compression of hydrogen from (p1, T1) to p2, from the full Leachman equation of state (real gas, variable heat capacity).

$$
s(p_2, T_2) = s(p_1, T_1),\quad w = h_2 - h_1
$$

| input | meaning | condition |
|---|---|---|
| `p1` | inlet pressure |  |
| `T1` | inlet temperature |  |
| `p2` | outlet pressure |  |

| output | meaning |
|---|---|
| `T2` | outlet temperature |
| `w` | compression work per kg $(h_{2} - h_{1})$ |
| `w_kWh_per_kg` | work in kWh/kg |

- **Method:** s(p2, T2) = s(p1, T1) solved for T2 by bisection; $w = h$(p2, T2) - h(p1, T1); s and h from the Helmholtz energy (ideal + residual parts)
- **Assumes:** reversible adiabatic, single stage; outlet temperature below 1000 K
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_hydrogen_isentropic_compression', p1=3000000.0, T1=300.0, p2=10000000.0)
# -> T2 = 423.294 K, w = 1.83321e+06 J/kg, w_kWh_per_kg = 0.509226   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hydrogen isentropic compression: p1 = 30 bar, T1 = 300 K, p2 = 100 bar | SOLVED | T2 = 423.294 K, w = 1.83321e+06 J/kg, w_kWh_per_kg = 0.509226 |
| Hydrogen isentropic compression: p1 = 30 bar, T1 = 300 K, p2 = 10 bar | INVALID_INPUT | Isentropic (adiabatic) compression of hydrogen: the final pressure must exceed the initial pressure. |

<a id="energy_single_diode_current"></a>

### PV single-diode model: current at a voltage — `energy_single_diode_current`

The current (and power) of a PV cell or module at a terminal voltage from the five-parameter single-diode equation $I = IL - I_{0}$ [exp((V + I Rs)/a) - 1] - (V + I Rs)/Rsh, solved implicitly.

$$
I = I_L - I_0\left[e^{(V + IR_s)/a} - 1\right] - \frac{V + IR_s}{R_{sh}}
$$

| input | meaning | condition |
|---|---|---|
| `V` | terminal voltage |  |
| `IL` | photocurrent (light-generated current) |  |
| `I0` | diode saturation current |  |
| `Rs` | series resistance |  |
| `Rsh` | shunt resistance |  |
| `nNsVth` | modified ideality factor n Ns k T/q |  |

| output | meaning |
|---|---|
| `I` | terminal current |
| `P` | power V I |

- **Method:** the implicit single-diode equation solved for I by bisection to full precision
- **Assumes:** IL, I0, Rsh, $a > 0$; $Rs  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_single_diode_current', V=0.0, IL=5.0, I0=1e-10, Rs=0.0, Rsh=1000000000.0, nNsVth=1.0)
# -> I = 5 A, P = 0 W   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_single_diode_current', V=1.0, IL=5.0, I0=1e-10, Rs=0.0, Rsh=100.0, nNsVth=1.0)
# -> I = 4.99 A, P = 4.99 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Single-diode current: V = 1 V, IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, Rsh = 100 ohm, nNsVth = 1 V | SOLVED | I = 4.99 A, P = 4.99 W |
| Single-diode current: V = 1 V, IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, Rsh = 0 ohm, nNsVth = 1 V | INVALID_INPUT | PV single-diode model: current at a voltage: the photocurrent, saturation current, shunt resistance and nNsVth must be a |

<a id="energy_single_diode_voltage"></a>

### PV single-diode model: voltage at a current — `energy_single_diode_voltage`

The terminal voltage (and power) of a PV cell or module carrying a current, from the five-parameter single-diode equation solved implicitly for V; $I = 0$ gives the open-circuit voltage.

$$
I = I_L - I_0\left[e^{(V + IR_s)/a} - 1\right] - \frac{V + IR_s}{R_{sh}}
$$

| input | meaning | condition |
|---|---|---|
| `I` | terminal current |  |
| `IL` | photocurrent (light-generated current) |  |
| `I0` | diode saturation current |  |
| `Rs` | series resistance |  |
| `Rsh` | shunt resistance |  |
| `nNsVth` | modified ideality factor n Ns k T/q |  |

| output | meaning |
|---|---|
| `V` | terminal voltage |
| `P` | power V I |

- **Method:** the implicit single-diode equation solved for V by bisection to full precision
- **Assumes:** IL, I0, Rsh, $a > 0$; $Rs  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_single_diode_voltage', I=8.0, IL=9.0, I0=1e-10, Rs=0.3, Rsh=400.0, nNsVth=1.9)
# -> V = 41.1302 V, P = 329.042 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Single-diode voltage: I = 0 A, IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, Rsh = 1e15 ohm, nNsVth = 1 V | SOLVED | V = 24.6353 V, P = 0 W |
| Single-diode voltage: I = 0 A, IL = 5 A, I0 = 0 A, Rs = 0 ohm, Rsh = 100 ohm, nNsVth = 1 V | INVALID_INPUT | PV single-diode model: voltage at a current: the photocurrent, saturation current, shunt resistance and nNsVth must be a |

<a id="energy_single_diode_mpp"></a>

### PV single-diode model: short-circuit, open-circuit and maximum power point — `energy_single_diode_mpp`

Short-circuit current, open-circuit voltage, the maximum power point (current, voltage, power) and the fill factor of a PV cell or module described by the five-parameter single-diode model.

$$
\frac{dP}{dV} = 0,\quad FF = \frac{I_{mp}V_{mp}}{I_{sc}V_{oc}}
$$

| input | meaning | condition |
|---|---|---|
| `IL` | photocurrent (light-generated current) |  |
| `I0` | diode saturation current |  |
| `Rs` | series resistance |  |
| `Rsh` | shunt resistance |  |
| `nNsVth` | modified ideality factor n Ns k T/q |  |

| output | meaning |
|---|---|
| `Isc` | short-circuit current |
| `Voc` | open-circuit voltage |
| `Imp` | current at maximum power |
| `Vmp` | voltage at maximum power |
| `Pmp` | maximum power |
| `FF` | fill factor Pmp/(Isc Voc) |

- **Method:** I(0) and V(I = 0) from the implicit equation; maximum power where $dP/dV_d = 0$ along the curve parametrized by the diode voltage (bisection)
- **Assumes:** IL, I0, Rsh, $a > 0$; $Rs  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Current–voltage curve of a PV module (single-diode model: photocurrent 9 A, saturation current 1](https://huggingface.co/AI999X/energy-systems-ai999x/resolve/main/figures/energy-single-diode-mpp-curve.svg)

*Current–voltage curve of a PV module (single-diode model: photocurrent 9 A, saturation current 1.0000e-10 A, Rs = 0.3 Ω, Rsh = 400 Ω), computed by the package at 81 voltages. The shaded rectangle is the maximum power the package finds, P = 335.252 W at V = 39.6025 V, I = 8.46542 A; it fills 0.778276 of Isc × Voc (8.99326 A × 47.8984 V), the fill factor.*

```python
energy_systems_ai999x.solve('energy_single_diode_mpp', IL=9.0, I0=1e-10, Rs=0.3, Rsh=400.0, nNsVth=1.9)
# -> Isc = 8.99326 A, Voc = 47.8984 V, Imp = 8.46542 A, Vmp = 39.6025 V, Pmp = 335.252 W, FF = 0.778276   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PV MPP: IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, Rsh = 1e15 ohm, nNsVth = 1 V | SOLVED | Isc = 5 A, Voc = 24.6353 V, Imp = 4.77798 A, Vmp = 21.5208 V, Pmp = 102.826 W, FF = 0.834788 |
| PV MPP: IL = 5 A, I0 = 1e-10 A, Rs = 0 ohm, nNsVth = 1 V | NEEDS_INPUT | energy_single_diode_mpp needs Rsh (shunt resistance). Example: solve('energy_single_diode_mpp', IL='9 A', I0='1e-10 A',  |

<a id="energy_desoto_parameters"></a>

### PV module single-diode parameters at operating conditions (De Soto) — `energy_desoto_parameters`

The five single-diode parameters (photocurrent, saturation current, series and shunt resistance, modified ideality factor) of a PV module at an irradiance and cell temperature from its reference-condition values, by the De Soto (2006) model.

$$
I_0 = I_{0,ref}\left(\frac{T}{T_{ref}}\right)^3\exp\left(\frac{E_{g,ref}}{kT_{ref}} - \frac{E_g}{kT}\right)
$$

| input | meaning | condition |
|---|---|---|
| `G` | effective irradiance |  |
| `T_cell` | cell temperature |  |
| `alpha_sc` | short-circuit current temperature coefficient in A/K |  |
| `a_ref` | modified ideality factor at reference conditions |  |
| `IL_ref` | photocurrent at reference conditions |  |
| `I0_ref` | saturation current at reference conditions |  |
| `Rsh_ref` | shunt resistance at reference conditions |  |
| `Rs` | series resistance |  |
| `EgRef` | band gap at 25 degC in eV (default 1.121, silicon) |  |
| `dEgdT` | band-gap temperature coefficient in $1/K$ (default -0.0002677) |  |

| output | meaning |
|---|---|
| `IL` | photocurrent |
| `I0` | saturation current |
| `Rs` | series resistance |
| `Rsh` | shunt resistance |
| `nNsVth` | modified ideality factor |

- **Method:** a = a_ref T/T_ref; $IL = (G/1000)$(IL_ref + alpha_sc (T - T_ref)); I0 = I0_ref (T/T_ref)^3 exp(Eg_ref/(k T_ref) - Eg/(k T)), Eg = Eg_ref (1 + dEg/dT (T - T_ref)); Rsh = Rsh_ref $1000/G$
- **Assumes:** reference conditions 1000 $W/m^{2}$ and 25 degC
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_desoto_parameters', G=1000, T_cell=298.15, alpha_sc=0.004, a_ref=1.9, IL_ref=9.0, I0_ref=1e-10, Rsh_ref=400.0, Rs=0.3)
# -> IL = 9 A, I0 = 1e-10 A, Rs = 0.3 ohm, Rsh = 400 ohm, nNsVth = 1.9 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| De Soto parameters: G = 1000 W/m^2, T_cell = 25 degC, alpha_sc = 0.004, a_ref = 1.9 V, IL_ref = 9 A, I0_ref = 1e-10 A, Rsh_ref = 400 ohm, Rs = 0.3 ohm | SOLVED | IL = 9 A, I0 = 1e-10 A, Rs = 0.3 ohm, Rsh = 400 ohm, nNsVth = 1.9 V |
| De Soto parameters: G = 0 W/m^2, T_cell = 25 degC, alpha_sc = 0.004, a_ref = 1.9 V, IL_ref = 9 A, I0_ref = 1e-10 A, Rsh_ref = 400 ohm, Rs = 0.3 ohm | INVALID_INPUT | PV module single-diode parameters at operating conditions (De Soto): the effective irradiance must be a positive number. |

<a id="energy_pv_module_at_conditions"></a>

### PV module maximum power at an irradiance and temperature — `energy_pv_module_at_conditions`

Short-circuit current, open-circuit voltage, maximum-power current, voltage and power, and fill factor of a PV module at an irradiance and cell temperature: the De Soto translation of its reference parameters, then the single-diode maximum power point.

$$
P_{mp} = \max_V V\,I(V;\,I_L(G,T), I_0(T), R_s, R_{sh}(G), a(T))
$$

| input | meaning | condition |
|---|---|---|
| `G` | effective irradiance |  |
| `T_cell` | cell temperature |  |
| `alpha_sc` | short-circuit current temperature coefficient in A/K |  |
| `a_ref` | modified ideality factor at reference conditions |  |
| `IL_ref` | photocurrent at reference conditions |  |
| `I0_ref` | saturation current at reference conditions |  |
| `Rsh_ref` | shunt resistance at reference conditions |  |
| `Rs` | series resistance |  |
| `EgRef` | band gap at 25 degC in eV (default 1.121, silicon) |  |
| `dEgdT` | band-gap temperature coefficient in $1/K$ (default -0.0002677) |  |

| output | meaning |
|---|---|
| `Isc` | short-circuit current |
| `Voc` | open-circuit voltage |
| `Imp` | current at maximum power |
| `Vmp` | voltage at maximum power |
| `Pmp` | maximum power |
| `FF` | fill factor |

- **Method:** De Soto parameter translation, then I_sc, V_oc and the maximum power point of the single-diode equation
- **Assumes:** reference conditions 1000 $W/m^{2}$ and 25 degC
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_pv_module_at_conditions', G=800, T_cell=318.15, alpha_sc=0.004, a_ref=1.9, IL_ref=9.0, I0_ref=1e-10, Rsh_ref=400.0, Rs=0.3)
# -> Isc = 7.25964 A, Voc = 44.2796 V, Imp = 6.79029 A, Vmp = 36.3735 V, Pmp = 246.987 W, FF = 0.768342   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PV module at conditions: G = 1000 W/m^2, T_cell = 25 degC, alpha_sc = 0.004, a_ref = 1 V, IL_ref = 5 A, I0_ref = 1e-10 A, Rsh_ref = 1e15 ohm, Rs = 0 ohm | SOLVED | Isc = 5 A, Voc = 24.6353 V, Imp = 4.77798 A, Vmp = 21.5208 V, Pmp = 102.826 W, FF = 0.834788 |
| PV module at conditions: G = 1000 W/m^2, T_cell = 500 K, alpha_sc = 0.004, a_ref = 1 V, IL_ref = 5 A, I0_ref = 1e-10 A, Rsh_ref = 1e15 ohm, Rs = 0 ohm | INVALID_INPUT | PV module maximum power at an irradiance and temperature: the cell temperature must lie between 200 K and 400 K. |

<a id="energy_poa_perez"></a>

### Irradiance on a tilted plane (Perez sky model) — `energy_poa_perez`

Beam, sky-diffuse and ground-reflected irradiance on a tilted surface with the Perez (1990) anisotropic sky (circumsolar and horizon brightening coefficients by sky-clearness bin, all-sites composite set), the air mass by Kasten-Young and G_on from day n (Spencer).

$$
G_{d,t} = G_d\left[(1 - F_1)\frac{1+\cos\beta}{2} + F_1\frac{a}{b} + F_2\sin\beta\right]
$$

| input | meaning | condition |
|---|---|---|
| `tilt` | surface tilt from horizontal in degrees |  |
| `surface_azimuth` | surface azimuth in degrees clockwise from north (south = 180) |  |
| `zenith` | solar zenith angle in degrees |  |
| `azimuth` | solar azimuth in degrees clockwise from north |  |
| `dni` | direct normal irradiance |  |
| `ghi` | global horizontal irradiance |  |
| `dhi` | diffuse horizontal irradiance |  |
| `albedo` | ground reflectance (0 to 1) |  |
| `n` | day of the year (1 = 1 January) |  |

| output | meaning |
|---|---|
| `poa_global` | total irradiance on the plane |
| `poa_direct` | beam irradiance on the plane |
| `poa_sky_diffuse` | sky-diffuse irradiance on the plane |
| `poa_ground_diffuse` | ground-reflected irradiance on the plane |
| `aoi_deg` | angle of incidence in degrees |

- **Method:** eps = ((DHI + DNI)/DHI + 1.041 $z^{3})/(1 + 1.041$ z^3), Delta = DHI AM/G_on; F1, F2 from the bin's coefficients; sky = DHI [$(1 - F_{1})$(1 + cos beta)/2 + F1 $a/b + F_{2}$ sin beta]
- **Assumes:** 0 <= zenith < 87; DHI > 0; DHI <= GHI
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_poa_perez', tilt=30, surface_azimuth=180, zenith=30, azimuth=180, dni=800, ghi=792.820323027551, dhi=100, albedo=0.2, n=172)
# -> poa_global = 923.778 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 113.156 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plane of array irradiance (Perez): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 100 W/m^2, albedo = 0.2, n = 172 | SOLVED | poa_global = 923.778 W/m^2, poa_direct = 800 W/m^2, poa_sky_diffuse = 113.156 W/m^2, poa_ground_diffuse = 10.6218 W/m^2, aoi_deg = 0 |
| Plane of array irradiance (Perez): tilt = 30, surface_azimuth = 180, zenith = 30, azimuth = 180, dni = 800 W/m^2, ghi = 792.820323027551 W/m^2, dhi = 0 W/m^2, albedo = 0.2, n = 172 | INVALID_INPUT | Irradiance on a tilted plane (Perez sky model): the Perez model needs a positive diffuse irradiance. |

<a id="energy_clear_sky_ineichen"></a>

### Clear-sky irradiance (Ineichen-Perez with Linke turbidity) — `energy_clear_sky_ineichen`

Clear-sky global horizontal, direct normal and diffuse horizontal irradiance from the apparent solar zenith, the Linke turbidity and the site altitude (Ineichen and Perez 2002), with the pressure-corrected air mass and G_on from day n.

$$
G_{cs} = c_{g1}G_{on}\cos z\,e^{-c_{g2}AM\left(f_{h1} + f_{h2}(T_L - 1)\right)}
$$

| input | meaning | condition |
|---|---|---|
| `zenith` | apparent solar zenith angle in degrees |  |
| `linke_turbidity` | Linke turbidity factor (1 to 10; about 2 to 5 for clear skies) |  |
| `altitude` | site altitude above sea level |  |
| `n` | day of the year (1 = 1 January) |  |

| output | meaning |
|---|---|
| `ghi` | clear-sky global horizontal irradiance |
| `dni` | clear-sky direct normal irradiance |
| `dhi` | clear-sky diffuse horizontal irradiance |
| `AM_absolute` | absolute air mass (Kasten-Young, standard pressure at the altitude) |

- **Method:** GHI = cg1 G_on cos z exp(-cg2 AM (fh1 + fh2 $(TL - 1)$)); DNI = min(b G_on exp(-0.09 AM $(TL - 1)$), GHI (1 - (0.1 - 0.2 e^-TL)/(0.1 + 0.882/fh1))/cos z); $fh1 = e^{-h/8000}$, $fh2 = e^{-h/1250}$
- **Assumes:** sun above the horizon; $1  \leq  TL  \leq  10$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_clear_sky_ineichen', zenith=0, linke_turbidity=1, altitude=0, n=1)
# -> ghi = 1180.76 W/m^2, dni = 1148.99 W/m^2, dhi = 31.7724 W/m^2, AM_absolute = 0.999712   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Clear-sky irradiance (Ineichen): zenith = 30, linke_turbidity = 3, altitude = 500 m, n = 172 | SOLVED | ghi = 885.003 W/m^2, dni = 910.142 W/m^2, dhi = 96.7974 W/m^2, AM_absolute = 1.08721 |
| Clear-sky irradiance (Ineichen): zenith = 95, linke_turbidity = 3, altitude = 500 m, n = 172 | INVALID_INPUT | Clear-sky irradiance (Ineichen-Perez with Linke turbidity): the sun must be above the horizon (apparent zenith below 90  |
| Clear-sky irradiance (Ineichen): zenith = 30, linke_turbidity = 3, n = 172 | NEEDS_INPUT | energy_clear_sky_ineichen needs altitude (site altitude above sea level). Example: solve('energy_clear_sky_ineichen', ze |

<a id="energy_disc_dni"></a>

### Direct normal irradiance from global horizontal irradiance (DISC model) — `energy_disc_dni`

The direct normal irradiance estimated from global horizontal irradiance by Maxwell's DISC model (clearness index and pressure-corrected air mass), with the clearness index and the air mass used.

$$
G_{bn} = I_0\left[K_{nc}(AM) - \left(a + b\,e^{c\,AM}\right)\right]
$$

| input | meaning | condition |
|---|---|---|
| `ghi` | global horizontal irradiance |  |
| `zenith` | solar zenith angle in degrees |  |
| `n` | day of the year (1 = 1 January) |  |
| `pressure` | site pressure (default 101325 Pa) |  |

| output | meaning |
|---|---|
| `dni` | direct normal irradiance (0 when the model gives a negative value) |
| `kt` | clearness index (solar constant 1370 W/m^2) |
| `airmass` | absolute air mass (Kasten 1966, capped at 12) |

- **Method:** kt = GHI/(I0 cos z); Kn = Knc(AM) - (a(kt) + b(kt) exp(c(kt) AM)); DNI = Kn I0
- **Assumes:** 0 <= zenith <= 85; GHI > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_disc_dni', ghi=709.0092500000001, zenith=0, n=1)
# -> dni = 163.645 W/m^2, kt = 0.5, airmass = 0.999494   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| DISC model: ghi = 709.0092500000001 W/m^2, zenith = 0, n = 1 | SOLVED | dni = 163.645 W/m^2, kt = 0.5, airmass = 0.999494 |
| DISC model: ghi = 600 W/m^2, zenith = 89, n = 172 | INVALID_INPUT | Direct normal irradiance from global horizontal irradiance (DISC model): the zenith angle must lie between 0 and 85 degr |

<a id="energy_per_unit_impedance"></a>

### Base impedance and per-unit impedance — `energy_per_unit_impedance`

The base impedance Z_base = V_base^2/S_base and base current of a three-phase per-unit system, and an impedance in ohms expressed in per unit.

$$
Z_{base} = \frac{V_{base}^2}{S_{base}},\quad Z_{pu} = \frac{Z}{Z_{base}}
$$

| input | meaning | condition |
|---|---|---|
| `R` | resistance per phase (ohm) |  |
| `X` | reactance per phase (ohm) |  |
| `S_base` | three-phase base power (VA) |  |
| `V_base` | line-to-line base voltage |  |

| output | meaning |
|---|---|
| `Z_base` | base impedance V_base^2/S_base |
| `I_base` | base current S_base/(sqrt(3) V_base) |
| `R_pu` | per-unit resistance |
| `X_pu` | per-unit reactance |
| `Z_pu` | per-unit impedance magnitude |

- **Method:** Z_base = V_base^2/S_base, I_base = S_base/(sqrt(3) V_base), Z_pu = Z/Z_base
- **Assumes:** R, $X  \geq  0$, not both 0; positive bases; three-phase base, line-to-line voltage
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_per_unit_impedance', R=5.0, X=20.0, S_base=100000000.0, V_base=132000.0)
# -> Z_base = 174.24 ohm, I_base = 437.387 A, R_pu = 0.0286961, X_pu = 0.114784, Z_pu = 0.118317   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_per_unit_impedance', R=0.0, X=10.0, S_base=1000000.0, V_base=10000.0)
# -> Z_base = 100 ohm, I_base = 57.735 A, R_pu = 0, X_pu = 0.1, Z_pu = 0.1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Per-unit impedance: R = 5 ohm, X = 20 ohm, S_base = 100 MVA, V_base = 132 kV | SOLVED | Z_base = 174.24 ohm, I_base = 437.387 A, R_pu = 0.0286961, X_pu = 0.114784, Z_pu = 0.118317 |
| Per-unit impedance: R = 5 ohm, X = 20 ohm, S_base = 0 MVA, V_base = 132 kV | INVALID_INPUT | Base impedance and per-unit impedance: the base power and base voltage must be a positive number. |
| Per-unit impedance: R = 5 ohm, X = 20 ohm, S_base = 100 MVA | NEEDS_INPUT | energy_per_unit_impedance needs V_base (line-to-line base voltage). Example: solve('energy_per_unit_impedance', R='5 ohm |

<a id="energy_geometric_mean_distance"></a>

### Geometric mean distance of a three-phase line — `energy_geometric_mean_distance`

The geometric mean distance GMD = (D_ab D_bc D_ca)^(1/3) between the phases of a fully transposed three-phase line.

$$
GMD = \sqrt[3]{D_{ab}D_{bc}D_{ca}}
$$

| input | meaning | condition |
|---|---|---|
| `D_ab` | distance between phases a and b |  |
| `D_bc` | distance between phases b and c |  |
| `D_ca` | distance between phases c and a |  |

| output | meaning |
|---|---|
| `GMD` | geometric mean distance |
| `ratio_to_mean` | GMD divided by the arithmetic mean spacing |

- **Method:** GMD = (D_ab D_bc D_ca)^(1/3)
- **Assumes:** positive spacings that form a triangle (or a straight line); fully transposed line
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_geometric_mean_distance', D_ab=4.0, D_bc=4.0, D_ca=8.0)
# -> GMD = 5.03968 m, ratio_to_mean = 0.944941   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_geometric_mean_distance', D_ab=2.0, D_bc=2.0, D_ca=2.0)
# -> GMD = 2 m, ratio_to_mean = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Geometric mean distance: D_ab = 4 m, D_bc = 4 m, D_ca = 8 m | SOLVED | GMD = 5.03968 m, ratio_to_mean = 0.944941 |
| Geometric mean distance: D_ab = 1 m, D_bc = 1 m, D_ca = 8 m | INVALID_INPUT | Geometric mean distance of a three-phase line: the three spacings must form a triangle (or a line). |
| Geometric mean distance: D_ab = 4 m, D_bc = 4 m | NEEDS_INPUT | energy_geometric_mean_distance needs D_ca (distance between phases c and a). Example: solve('energy_geometric_mean_dista |

<a id="energy_synchronous_speed"></a>

### Synchronous speed, rotor speed and slip of an AC machine — `energy_synchronous_speed`

The synchronous speed $n_s = 120$ f/P of an AC machine and, with the slip, the rotor speed $n = n_s$ $(1 - s)$ (or, with the rotor speed, the slip) and the rotor (slip) frequency.

$$
n_s = \frac{120 f}{P},\quad n = n_s(1 - s)
$$

| input | meaning | condition |
|---|---|---|
| `f` | supply frequency |  |
| `poles` | number of poles (even) |  |
| `s` | slip (fraction or %, optional) |  |
| `n_r` | rotor speed (rpm, optional) |  |

| output | meaning |
|---|---|
| `n_sync_rpm` | synchronous speed in rpm |
| `omega_sync` | synchronous speed in rad/s |
| `n_rotor_rpm` | rotor speed in rpm |
| `slip` | slip (n_s - n)/n_s |
| `slip_frequency` | rotor current frequency s f |

- **Method:** $n_s = 120$ f/P; $n = n_s$ $(1 - s)$; $s = (n_s - n)/n_s$; $f_r = s$ f
- **Assumes:** $f > 0$; even number of poles; at most one of slip and rotor speed
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_synchronous_speed', f=50.0, poles=4)
# -> n_sync_rpm = 1500, omega_sync = 157.08 rad/s   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_synchronous_speed', f=60.0, poles=4, s=0.03)
# -> n_sync_rpm = 1800, omega_sync = 188.496 rad/s, n_rotor_rpm = 1746, slip = 0.03, slip_frequency = 1.8 Hz   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_synchronous_speed', f=60.0, poles=6, n_r=120.42771838760873)
# -> n_sync_rpm = 1200, omega_sync = 125.664 rad/s, n_rotor_rpm = 1150, slip = 0.0416667, slip_frequency = 2.5 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Synchronous speed: f = 50 Hz, poles = 4 | SOLVED | n_sync_rpm = 1500, omega_sync = 157.08 rad/s |
| Synchronous speed: f = 60 Hz, poles = 4, s = 3% | SOLVED | n_sync_rpm = 1800, omega_sync = 188.496 rad/s, n_rotor_rpm = 1746, slip = 0.03, slip_frequency = 1.8 Hz |
| Synchronous speed: f = 60 Hz, poles = 6, n_r = 1150 rpm | SOLVED | n_sync_rpm = 1200, omega_sync = 125.664 rad/s, n_rotor_rpm = 1150, slip = 0.0416667, slip_frequency = 2.5 Hz |
| Synchronous speed: f = 50 Hz, poles = 5 | INVALID_INPUT | Synchronous speed, rotor speed and slip of an AC machine: the number of poles must be an even whole number from 2 to 200 |
| Synchronous speed: f = 50 Hz | NEEDS_INPUT | energy_synchronous_speed needs poles (number of poles (even)). Example: solve('energy_synchronous_speed', f='50 Hz', pol |

<a id="energy_stack_electrical_power"></a>

### Electrical power of a fuel-cell or electrolyser stack — `energy_stack_electrical_power`

The electrical power $P = N$ I V_cell delivered by a fuel-cell stack (or drawn by an electrolyser stack) of N cells in series at a current and an average cell voltage, and the stack voltage.

$$
P = N\,I\,V_{cell}
$$

| input | meaning | condition |
|---|---|---|
| `I` | stack current |  |
| `N` | number of cells in series |  |
| `V_cell` | average cell voltage |  |

| output | meaning |
|---|---|
| `P` | electrical power N I V_cell |
| `P_kW` | electrical power in kW |
| `V_stack` | stack voltage N V_cell |

- **Method:** $P = N$ I V_cell, V_stack = N V_cell
- **Assumes:** cells in series carrying the same current; 0 < V_cell <= 5 V
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_stack_electrical_power', I=200.0, N=100, V_cell=0.7)
# -> P = 14000 W, P_kW = 14, V_stack = 70 V   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_stack_electrical_power', I=1000.0, N=100, V_cell=1.9)
# -> P = 190000 W, P_kW = 190, V_stack = 190 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Stack electrical power: I = 200 A, N = 100, V_cell = 0.7 V | SOLVED | P = 14000 W, P_kW = 14, V_stack = 70 V |
| Stack electrical power: I = 200 A, N = 0, V_cell = 0.7 V | INVALID_INPUT | N: must be at least 1, got 0. |
| Stack electrical power: I = 200 A, N = 100 | NEEDS_INPUT | energy_stack_electrical_power needs V_cell (average cell voltage). Example: solve('energy_stack_electrical_power', I='20 |

<a id="energy_coulomb_counting"></a>

### State of charge by coulomb counting — `energy_coulomb_counting`

The state of charge of a battery after a sequence of constant currents over equal time steps (positive on discharge), by coulomb counting SOC = SOC0 - integral(I dt)/Q, with its lowest and highest value and the charge delivered.

$$
SOC(t) = SOC_0 - \frac{1}{Q}\int_0^t I\,dt
$$

| input | meaning | condition |
|---|---|---|
| `SOC0` | initial state of charge (fraction or %) |  |
| `Q` | capacity (Ah, mAh, C) |  |
| `currents` | current in each time step in A (positive on discharge) |  |
| `dt` | length of each time step |  |

| output | meaning |
|---|---|
| `SOC` | final state of charge |
| `SOC_min` | lowest state of charge |
| `SOC_max` | highest state of charge |
| `charge_out` | net charge delivered |
| `charge_out_Ah` | net charge delivered in Ah |

- **Method:** SOC_k = SOC0 - (sum_{$j \leq k$} I_j dt)/Q
- **Assumes:** 0 <= SOC0 <= 1; Q, $dt > 0$; the state of charge stays within 0 to 100%; no coulombic losses
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
energy_systems_ai999x.solve('energy_coulomb_counting', SOC0=0.9, Q=180000.0, currents=[10, 20, -5], dt=3600.0)
# -> SOC = 0.4, SOC_min = 0.3, SOC_max = 0.9, charge_out = 90000 C, charge_out_Ah = 25   (SOLVED)
```

```python
energy_systems_ai999x.solve('energy_coulomb_counting', SOC0=1.0, Q=7200.0, currents='1, 1', dt=1800.0)
# -> SOC = 0.5, SOC_min = 0.5, SOC_max = 1, charge_out = 3600 C, charge_out_Ah = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Coulomb counting: SOC0 = 90%, Q = 50 Ah, currents = [10, 20, -5], dt = 1 h | SOLVED | SOC = 0.4, SOC_min = 0.3, SOC_max = 0.9, charge_out = 90000 C, charge_out_Ah = 25 |
| Coulomb counting: SOC0 = 90%, Q = 50 Ah, currents = [100, 20], dt = 1 h | INVALID_INPUT | State of charge by coulomb counting: the state of charge leaves the range 0 to 100% (more charge than the capacity). |
| Coulomb counting: SOC0 = 90%, Q = 50 Ah, currents = [10, 20, -5] | NEEDS_INPUT | energy_coulomb_counting needs dt (length of each time step). Example: solve('energy_coulomb_counting', SOC0=0.9, Q='1800 |
