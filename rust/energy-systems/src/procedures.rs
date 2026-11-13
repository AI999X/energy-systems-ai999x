//! energy_systems: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/energy_systems/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "energy_actuator_disc",
    "energy_air_mass",
    "energy_angle_of_incidence",
    "energy_battery_pack_sizing",
    "energy_battery_rc_discharge",
    "energy_butler_volmer",
    "energy_capacity_factor",
    "energy_cell_temperature_faiman",
    "energy_cell_temperature_noct",
    "energy_cell_temperature_sapm",
    "energy_clear_sky_haurwitz",
    "energy_clear_sky_ineichen",
    "energy_coulomb_counting",
    "energy_critical_clearing",
    "energy_dc_power_flow",
    "energy_desoto_parameters",
    "energy_disc_dni",
    "energy_dlg_fault",
    "energy_droop_sharing",
    "energy_economic_dispatch",
    "energy_electrolyser",
    "energy_equation_of_time",
    "energy_erbs_decomposition",
    "energy_extraterrestrial_irradiance",
    "energy_fuel_cell_stack",
    "energy_geometric_mean_distance",
    "energy_hub_air_density",
    "energy_hydro_power",
    "energy_hydrogen_isentropic_compression",
    "energy_hydrogen_isothermal_compression",
    "energy_hydrogen_tank",
    "energy_iam_ashrae",
    "energy_iam_physical",
    "energy_induction_max_torque",
    "energy_induction_motor",
    "energy_induction_power_flow",
    "energy_line_parameters",
    "energy_ll_fault",
    "energy_long_line",
    "energy_medium_line",
    "energy_newton_load_flow",
    "energy_parallel_transformers",
    "energy_per_unit_change_of_base",
    "energy_per_unit_impedance",
    "energy_peukert",
    "energy_poa_haydavies",
    "energy_poa_isotropic",
    "energy_poa_klucher",
    "energy_poa_perez",
    "energy_poa_reindl",
    "energy_power_angle",
    "energy_power_curve_density_corrected",
    "energy_power_curve_output",
    "energy_pv_array_output",
    "energy_pv_module_at_conditions",
    "energy_pvwatts_dc",
    "energy_pvwatts_inverter",
    "energy_pvwatts_losses",
    "energy_short_line",
    "energy_single_diode_current",
    "energy_single_diode_mpp",
    "energy_single_diode_voltage",
    "energy_slg_fault",
    "energy_solar_declination",
    "energy_solar_position",
    "energy_solar_time_hour_angle",
    "energy_stack_electrical_power",
    "energy_sun_angles",
    "energy_sunrise_sunset",
    "energy_symmetrical_fault",
    "energy_synchronous_generator",
    "energy_synchronous_speed",
    "energy_three_phase_power",
    "energy_transformer_loading",
    "energy_transformer_max_efficiency",
    "energy_transformer_performance",
    "energy_transformer_tests",
    "energy_two_bus_load_flow",
    "energy_weibull_aep",
    "energy_weibull_wind",
    "energy_wind_power",
    "energy_wind_shear_log",
    "energy_wind_shear_power_law",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "energy_actuator_disc" => {
            let r = f_actuator_disc(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_23(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_air_mass" => {
            let r = f_air_mass(match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("kastenyoung1989")) })().ok()? } }, match v.get("pressure") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(101325.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_91(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_angle_of_incidence" => {
            let r = f_angle_of_incidence(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_152(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_battery_pack_sizing" => {
            let r = f_pack_sizing(match v.get("V_target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q_cell") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_207(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_battery_rc_discharge" => {
            let r = f_battery_rc_discharge(match v.get("OCV") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_264(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_butler_volmer" => {
            let r = f_butler_volmer(match v.get("j0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("j") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_388(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_capacity_factor" => {
            let r = f_capacity_factor(match v.get("P_rated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("CF") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("hours") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(8760.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_456(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_cell_temperature_faiman" => {
            let r = f_cell_temperature_faiman(match v.get("poa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wind") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(25.0)) })().ok()? } }, match v.get("u1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(6.84)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_515(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_cell_temperature_noct" => {
            let r = f_cell_temperature_noct(match v.get("poa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("noct") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_569(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_cell_temperature_sapm" => {
            let r = f_cell_temperature_sapm(match v.get("poa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wind") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mount") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_624(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_clear_sky_haurwitz" => {
            let r = f_haurwitz(match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_675(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_clear_sky_ineichen" => {
            let r = f_clear_sky_ineichen(match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("linke_turbidity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("altitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_750(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_coulomb_counting" => {
            let r = f_coulomb_counting(match v.get("SOC0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("currents") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_816(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_critical_clearing" => {
            let r = f_critical_clearing(match v.get("Pm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pmax_pre") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pmax_fault") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pmax_post") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("f") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_896(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_dc_power_flow" => {
            let r = f_dc_power_flow(match v.get("lines") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_994(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_desoto_parameters" => {
            let r = f_desoto_parameters(match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha_sc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EgRef") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.121)) })().ok()? } }, match v.get("dEgdT") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(py::neg(&Py::Float(0.0002677))?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1054(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_disc_dni" => {
            let r = f_disc(match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pressure") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(101325.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1128(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_dlg_fault" => {
            let r = s_1161(match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Xf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1199(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_droop_sharing" => {
            let r = f_droop_sharing(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ratings") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1270(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_economic_dispatch" => {
            let r = f_economic_dispatch(match v.get("costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1372(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_electrolyser" => {
            let r = f_electrolyser(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("faradaic") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1431(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_equation_of_time" => {
            let r = s_1464(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("spencer")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_erbs_decomposition" => {
            let r = f_erbs(match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1545(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_extraterrestrial_irradiance" => {
            let r = f_extraterrestrial_irradiance(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("spencer")) })().ok()? } }, match v.get("solar_constant") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(c_SOLAR_CONSTANT()?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1601(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_fuel_cell_stack" => {
            let r = f_fuel_cell_stack(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("utilization") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1659(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_geometric_mean_distance" => {
            let r = f_gmd(match v.get("D_ab") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_bc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_ca") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hub_air_density" => {
            let r = f_hub_air_density(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hub") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("barometric")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1778(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hydro_power" => {
            let r = f_hydro_power(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1000.0)) })().ok()? } }, match v.get("g") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(9.81)) })().ok()? } }, match v.get("CF") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("hours") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(8760.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1840(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hydrogen_isentropic_compression" => {
            let r = f_hydrogen_isentropic_compression(match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1941(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hydrogen_isothermal_compression" => {
            let r = f_hydrogen_isothermal_compression(match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2034(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hydrogen_tank" => {
            let r = f_hydrogen_tank(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2128(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_iam_ashrae" => {
            let r = f_iam_ashrae(match v.get("aoi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.05)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2182(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_iam_physical" => {
            let r = f_iam_physical(match v.get("aoi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.526)) })().ok()? } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(4.0)) })().ok()? } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.002)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2239(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_induction_max_torque" => {
            let r = f_induction_max_torque(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("connection") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2309(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_induction_motor" => {
            let r = f_induction_motor(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("slip") { Some(x) => py::from_val(x), None => { return None } }, match v.get("connection") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_rot") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2378(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_induction_power_flow" => {
            let r = f_induction_power_flow(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_scl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("P_core") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("P_fw") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("E2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2460(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_line_parameters" => {
            let r = f_line_parameters(match v.get("D_ab") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_bc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_ca") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gmr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n_bundle") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("spacing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2538(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_ll_fault" => {
            let r = s_2571(match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Xf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2609(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_long_line" => {
            let r = f_long_line(match v.get("Vr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2682(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_medium_line" => {
            let r = f_medium_line(match v.get("Vr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2749(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_newton_load_flow" => {
            let r = f_newton_load_flow(match v.get("lines") { Some(x) => py::from_val(x), None => { return None } }, match v.get("loads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gens") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2955(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_parallel_transformers" => {
            let r = f_parallel_transformers(match v.get("S1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vk1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vkr1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vk2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vkr2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_lv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3013(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_per_unit_change_of_base" => {
            let r = f_per_unit_change_of_base(match v.get("R_pu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X_pu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S_old") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_old") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S_new") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_new") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3068(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_per_unit_impedance" => {
            let r = f_per_unit_impedance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S_base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_base") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3128(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_peukert" => {
            let r = f_peukert(match v.get("currents") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3201(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_haydavies" => {
            let r = s_3234(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3286(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_isotropic" => {
            let r = s_3319(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3371(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_klucher" => {
            let r = s_3404(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3456(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_perez" => {
            let r = f_poa_perez(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3550(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_reindl" => {
            let r = s_3583(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3635(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_power_angle" => {
            let r = f_power_angle(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3687(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_power_curve_density_corrected" => {
            let r = f_power_curve_density_corrected(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("powers") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3774(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_power_curve_output" => {
            let r = f_power_curve_output(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("powers") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3849(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pv_array_output" => {
            let r = f__pv_array_check(match v.get("poa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("noct") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pdc0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3913(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pv_module_at_conditions" => {
            let r = f_pv_module_at_conditions(match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha_sc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EgRef") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.121)) })().ok()? } }, match v.get("dEgdT") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(py::neg(&Py::Float(0.0002677))?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4019(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pvwatts_dc" => {
            let r = f_pvwatts_dc(match v.get("G_eff") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pdc0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4081(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pvwatts_inverter" => {
            let r = f_pvwatts_inverter(match v.get("pdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pdc0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta_nom") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.96)) })().ok()? } }, match v.get("eta_ref") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.9637)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4140(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pvwatts_losses" => {
            let r = f_pvwatts_losses({ let mut d = vec![]; for k in ["soiling", "shading", "snow", "mismatch", "wiring", "connections", "lid", "nameplate_rating", "age", "availability"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4191(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_short_line" => {
            let r = f_short_line(match v.get("Vr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_single_diode_current" => {
            let r = f_single_diode_current(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nNsVth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4341(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_single_diode_mpp" => {
            let r = f_single_diode_mpp(match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nNsVth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4443(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_single_diode_voltage" => {
            let r = f_single_diode_voltage(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nNsVth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4534(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_slg_fault" => {
            let r = s_4567(match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Xf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4605(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_solar_declination" => {
            let r = s_4638(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("cooper")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4657(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_solar_position" => {
            let r = f_solar_position(match v.get("latitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("longitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("clock") { Some(x) => py::from_val(x), None => { return None } }, match v.get("utc_offset") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4739(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_solar_time_hour_angle" => {
            let r = f_solar_time_hour_angle(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("clock") { Some(x) => py::from_val(x), None => { return None } }, match v.get("longitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("utc_offset") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4809(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_stack_electrical_power" => {
            let r = f_stack_power(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_cell") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4865(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_sun_angles" => {
            let r = f_sun_angles(match v.get("latitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("declination") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hour_angle") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4946(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_sunrise_sunset" => {
            let r = f_sunrise_sunset(match v.get("latitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("longitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("utc_offset") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5017(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_symmetrical_fault" => {
            let r = f_symmetrical_fault(match v.get("network") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fault_bus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("Xf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("S_base") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V_base") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5137(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_synchronous_generator" => {
            let r = f_synchronous_generator(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5200(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_synchronous_speed" => {
            let r = f_machine_speed(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5276(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_three_phase_power" => {
            let r = f_three_phase_power(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("connection") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5337(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_transformer_loading" => {
            let r = f_transformer_loading(match v.get("sn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_hv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_lv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vkr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_transformer_max_efficiency" => {
            let r = f_transformer_max_efficiency(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_core") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_cu_fl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5489(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_transformer_performance" => {
            let r = f_transformer_performance(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Req") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xeq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("load") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5553(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_transformer_tests" => {
            let r = f_transformer_tests(match v.get("Voc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ioc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Poc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vsc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Isc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Psc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5609(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_two_bus_load_flow" => {
            let r = f_two_bus_load_flow(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5669(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_weibull_aep" => {
            let r = f_weibull_aep(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("powers") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hours") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(8760.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5741(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_weibull_wind" => {
            let r = f_weibull_wind(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5796(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_wind_power" => {
            let r = f_wind_power(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5852(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_wind_shear_log" => {
            let r = f_wind_shear_log(match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5912(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_wind_shear_power_law" => {
            let r = f_wind_shear_power_law(match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(py::truediv(&Py::Int(1i64), &Py::Int(7i64))?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5965(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_BETZ() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.5925925925925926)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_EPS0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.8541878188e-12)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_F_BV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(96485.33212331001)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_F_FARADAY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(96485.33212)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_HHV_H2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(285826.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_K_B_EV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.617333262145179e-05)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LHV_H2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(241826.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MU0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.2566370614359173e-06)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_M_H2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.00201588)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_M_H2O() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.01801528)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_M_O2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.0319988)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_BV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SOLAR_CONSTANT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1366.1)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SQ3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.7320508075688772)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__A() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Complex(-0.4999999999999998, 0.8660254037844387)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__H2_R() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.314472)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__H2_RHOC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(15508.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__H2_TC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(33.145)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__L0_A1() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(-1.4579856475)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__L0_A2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.888076782)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__L0_LOGTAU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.5)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__L0_N() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(1.616), Py::Float(-0.4117), Py::Float(-0.792), Py::Float(0.758), Py::Float(1.217)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__L0_V() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Int(531i64), Py::Int(751i64), Py::Int(1989i64), Py::Int(2484i64), Py::Int(6859i64)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LG_BETA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.171), Py::Float(0.2245), Py::Float(0.1304), Py::Float(0.2785), Py::Float(0.3967)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LG_D() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Int(2i64), Py::Int(1i64), Py::Int(3i64), Py::Int(1i64), Py::Int(1i64)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LG_EPS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(1.506), Py::Float(0.156), Py::Float(1.736), Py::Float(0.67), Py::Float(1.662)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LG_ETA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(1.685), Py::Float(0.489), Py::Float(0.103), Py::Float(2.506), Py::Float(1.607)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LG_GAM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.7164), Py::Float(1.3444), Py::Float(1.4517), Py::Float(0.7204), Py::Float(1.5445)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LG_N() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(-0.0211716), Py::Float(0.0226312), Py::Float(0.032187), Py::Float(-0.0231752), Py::Float(0.0557346)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LG_T() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(4.187), Py::Float(5.646), Py::Float(0.791), Py::Float(7.249), Py::Float(2.986)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LP_D() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Int(1i64), Py::Int(4i64), Py::Int(1i64), Py::Int(1i64), Py::Int(2i64), Py::Int(2i64), Py::Int(3i64), Py::Int(1i64), Py::Int(3i64)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LP_L() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64), Py::Int(0i64), Py::Int(0i64), Py::Int(0i64), Py::Int(0i64), Py::Int(1i64), Py::Int(1i64)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LP_N() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(-6.93643), Py::Float(0.01), Py::Float(2.1101), Py::Float(4.52059), Py::Float(0.732564), Py::Float(-1.34086), Py::Float(0.130985), Py::Float(-0.777414), Py::Float(0.351944)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LP_T() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.6844), Py::Int(1i64), Py::Float(0.989), Py::Float(0.489), Py::Float(0.803), Py::Float(1.1444), Py::Float(1.409), Py::Float(1.754), Py::Float(1.311)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PEREZ_1990() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::list(vec![Py::Float(-0.008), Py::Float(0.588), Py::Float(-0.062), Py::Float(-0.06), Py::Float(0.072), Py::Float(-0.022)]), Py::list(vec![Py::Float(0.13), Py::Float(0.683), Py::Float(-0.151), Py::Float(-0.019), Py::Float(0.066), Py::Float(-0.029)]), Py::list(vec![Py::Float(0.33), Py::Float(0.487), Py::Float(-0.221), Py::Float(0.055), Py::Float(-0.064), Py::Float(-0.026)]), Py::list(vec![Py::Float(0.568), Py::Float(0.187), Py::Float(-0.295), Py::Float(0.109), Py::Float(-0.152), Py::Float(-0.014)]), Py::list(vec![Py::Float(0.873), Py::Float(-0.392), Py::Float(-0.362), Py::Float(0.226), Py::Float(-0.462), Py::Float(0.001)]), Py::list(vec![Py::Float(1.132), Py::Float(-1.237), Py::Float(-0.412), Py::Float(0.288), Py::Float(-0.823), Py::Float(0.056)]), Py::list(vec![Py::Float(1.06), Py::Float(-1.6), Py::Float(-0.359), Py::Float(0.264), Py::Float(-1.127), Py::Float(0.131)]), Py::list(vec![Py::Float(0.678), Py::Float(-0.327), Py::Float(-0.25), Py::Float(0.156), Py::Float(-1.377), Py::Float(0.251)])])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PEREZ_EDGES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.0), Py::Float(1.065), Py::Float(1.23), Py::Float(1.5), Py::Float(1.95), Py::Float(2.8), Py::Float(4.5), Py::Float(6.2)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PVW_LOSSES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::tuple(vec![Py::str("soiling"), Py::Float(2.0)]), Py::tuple(vec![Py::str("shading"), Py::Float(3.0)]), Py::tuple(vec![Py::str("snow"), Py::Float(0.0)]), Py::tuple(vec![Py::str("mismatch"), Py::Float(2.0)]), Py::tuple(vec![Py::str("wiring"), Py::Float(2.0)]), Py::tuple(vec![Py::str("connections"), Py::Float(0.5)]), Py::tuple(vec![Py::str("lid"), Py::Float(1.5)]), Py::tuple(vec![Py::str("nameplate_rating"), Py::Float(1.0)]), Py::tuple(vec![Py::str("age"), Py::Float(0.0)]), Py::tuple(vec![Py::str("availability"), Py::Float(3.0)])])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SAPM_MOUNTS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("open_rack_glass_glass"), Py::tuple(vec![Py::Float(-3.47), Py::Float(-0.0594), Py::Float(3.0)])), (Py::str("close_mount_glass_glass"), Py::tuple(vec![Py::Float(-2.98), Py::Float(-0.0471), Py::Float(1.0)])), (Py::str("open_rack_glass_polymer"), Py::tuple(vec![Py::Float(-3.56), Py::Float(-0.075), Py::Float(3.0)])), (Py::str("insulated_back_glass_polymer"), Py::tuple(vec![Py::Float(-2.81), Py::Float(-0.0455), Py::Float(0.0)]))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__airmass_rel(mut v_z: Py, mut v_model: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_zr: Py = Py::None;
    v_zr = py::math1("radians", &v_z.clone())?;
    v_c = py::math1("cos", &v_zr.clone())?;
    if ({ let t3520 = v_model.clone(); Py::Bool({ let t3521 = Py::str("kastenyoung1989"); if py::eq(&t3520, &t3521) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &py::add(&v_c.clone(), &py::mul(&Py::Float(0.50572), &py::pow(&py::add(&Py::Float(6.07995), &py::sub(&Py::Int(90i64), &v_z.clone())?)?, &py::neg(&Py::Float(1.6364))?)?)?)?)?);
    }
    if ({ let t3522 = v_model.clone(); Py::Bool({ let t3523 = Py::str("kasten1966"); if py::eq(&t3522, &t3523) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &py::add(&v_c.clone(), &py::mul(&Py::Float(0.15), &py::pow(&py::sub(&Py::Float(93.885), &v_z.clone())?, &py::neg(&Py::Float(1.253))?)?)?)?)?);
    }
    if ({ let t3524 = v_model.clone(); Py::Bool({ let t3525 = Py::str("simple"); if py::eq(&t3524, &t3525) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &v_c.clone())?);
    }
    if ({ let t3526 = v_model.clone(); Py::Bool({ let t3527 = Py::str("pickering2002"); if py::eq(&t3526, &t3527) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &py::math1("sin", &py::math1("radians", &py::add(&py::sub(&Py::Int(90i64), &v_z.clone())?, &py::truediv(&Py::Float(244.0), &py::add(&Py::Int(165i64), &py::mul(&Py::Float(47.0), &py::pow(&py::sub(&Py::Int(90i64), &v_z.clone())?, &Py::Float(1.1))?)?)?)?)?)?)?)?);
    }
    if ({ let t3528 = v_model.clone(); Py::Bool({ let t3529 = Py::str("youngirvine1967"); if py::eq(&t3528, &t3529) { true } else { false } }) }).truthy() {
        v_s = py::truediv(&Py::Float(1.0), &v_c.clone())?;
        return Ok(py::mul(&v_s.clone(), &py::sub(&Py::Int(1i64), &py::mul(&Py::Float(0.0012), &py::sub(&py::mul(&v_s.clone(), &v_s.clone())?, &Py::Int(1i64))?)?)?)?);
    }
    if ({ let t3530 = v_model.clone(); Py::Bool({ let t3531 = Py::str("young1994"); if py::eq(&t3530, &t3531) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::add(&py::add(&py::mul(&Py::Float(1.002432), &py::pow(&v_c.clone(), &Py::Int(2i64))?)?, &py::mul(&Py::Float(0.148386), &v_c.clone())?)?, &Py::Float(0.0096467))?, &py::add(&py::add(&py::add(&py::pow(&v_c.clone(), &Py::Int(3i64))?, &py::mul(&Py::Float(0.149864), &py::pow(&v_c.clone(), &Py::Int(2i64))?)?)?, &py::mul(&Py::Float(0.0102963), &v_c.clone())?)?, &Py::Float(0.000303978))?)?);
    }
    if ({ let t3532 = v_model.clone(); Py::Bool({ let t3533 = Py::str("gueymard1993"); if py::eq(&t3532, &t3533) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &py::add(&v_c.clone(), &py::mul(&py::mul(&Py::Float(0.00176759), &v_z.clone())?, &py::pow(&py::sub(&Py::Float(94.37515), &v_z.clone())?, &py::neg(&Py::Float(1.21563))?)?)?)?)?);
    }
    return Ok(py::truediv(&Py::Float(1.0), &py::add(&v_c.clone(), &py::truediv(&py::mul(&Py::Float(0.48353), &py::pow(&v_z.clone(), &Py::Float(0.095846))?)?, &py::pow(&py::sub(&Py::Float(96.741), &v_z.clone())?, &Py::Float(1.754))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__alt2pres(mut v_h: Py) -> R {
    return Ok(py::mul(&Py::Int(100i64), &py::pow(&py::truediv(&py::sub(&Py::Float(44331.514), &v_h.clone())?, &Py::Float(11880.516))?, &py::truediv(&Py::Int(1i64), &Py::Float(0.1902632))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4496 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t4496.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t4496[0].clone();
    v_fb = t4496[1].clone();
    if ({ let t4497 = v_fa.clone(); Py::Bool({ let t4498 = Py::Int(0i64); if py::eq(&t4497, &t4498) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t4499 = v_fb.clone(); Py::Bool({ let t4500 = Py::Int(0i64); if py::eq(&t4499, &t4500) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t4501 = { let t4503 = v_fa.clone(); Py::Bool({ let t4504 = Py::Int(0i64); if py::gt(&t4503, &t4504)? { true } else { false } }) }; Py::Bool({ let t4502 = { let t4505 = v_fb.clone(); Py::Bool({ let t4506 = Py::Int(0i64); if py::gt(&t4505, &t4506)? { true } else { false } }) }; if py::ne(&t4501, &t4502) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t4507 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t4507;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t4512 = { let t4508 = v_m.clone(); Py::Bool({ let t4509 = v_a.clone(); if py::eq(&t4508, &t4509) { true } else { false } }) }; if t4512.truthy() { t4512 } else { { let t4510 = v_m.clone(); Py::Bool({ let t4511 = v_b.clone(); if py::eq(&t4510, &t4511) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t4513 = v_fm.clone(); Py::Bool({ let t4514 = Py::Int(0i64); if py::eq(&t4513, &t4514) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t4515 = { let t4517 = v_fm.clone(); Py::Bool({ let t4518 = Py::Int(0i64); if py::gt(&t4517, &t4518)? { true } else { false } }) }; Py::Bool({ let t4516 = { let t4519 = v_fa.clone(); Py::Bool({ let t4520 = Py::Int(0i64); if py::gt(&t4519, &t4520)? { true } else { false } }) }; if py::eq(&t4515, &t4516) { true } else { false } }) }).truthy() {
            let t4521 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t4521.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4521[0].clone();
            v_fa = t4521[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bundle_equiv(mut v_r_or_gmr: Py, mut v_n: Py, mut v_d: Py) -> R {
    if ({ let t2514 = v_n.clone(); Py::Bool({ let t2515 = Py::Int(1i64); if py::eq(&t2514, &t2515) { true } else { false } }) }).truthy() {
        return Ok(v_r_or_gmr.clone());
    }
    if ({ let t2516 = v_n.clone(); Py::Bool({ let t2517 = Py::Int(2i64); if py::eq(&t2516, &t2517) { true } else { false } }) }).truthy() {
        return Ok(py::math1("sqrt", &py::mul(&v_r_or_gmr.clone(), &v_d.clone())?)?);
    }
    if ({ let t2518 = v_n.clone(); Py::Bool({ let t2519 = Py::Int(3i64); if py::eq(&t2518, &t2519) { true } else { false } }) }).truthy() {
        return Ok(py::pow(&py::mul(&py::mul(&v_r_or_gmr.clone(), &v_d.clone())?, &v_d.clone())?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?);
    }
    return Ok(py::mul(&py::pow(&Py::Int(2i64), &Py::Float(0.125))?, &py::pow(&py::mul(&v_r_or_gmr.clone(), &py::pow(&v_d.clone(), &Py::Int(3i64))?)?, &Py::Float(0.25))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bus_index(mut v_name: Py, mut v_n: Py) -> R {
    let _ = f__need({ let t5053 = ax_engine::pyre::module_call("fullmatch", &Py::str("\\d+"), &[Py::str(&py::to_str(&v_name.clone()))], 0)?; Py::Bool({ let t5054 = Py::None; if !py::is(&t5053, &t5054) { true } else { false } }) }, Py::str(&["bus names must be whole numbers 1, 2, 3, ... (got ".to_string(), py::format(&Py::str(&py::repr(&v_name.clone())), "")?, ")".to_string()].concat()))?;
    return Ok(py::int(&v_name.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ceil_tol(mut v_x: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::math1("ceil", &v_x.clone())?;
    return Ok((if ({ let t194 = { let t190 = py::sub(&v_n.clone(), &Py::Int(1i64))?; Py::Bool({ let t191 = Py::Int(0i64); if py::gt(&t190, &t191)? { true } else { false } }) }; if !t194.truthy() { t194 } else { { let t192 = py::abs(&py::sub(&v_x.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t193 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), v_x.clone()], true)?)?; if py::le(&t192, &t193)? { true } else { false } }) } } }).truthy() { py::sub(&v_n.clone(), &Py::Int(1i64))? } else { v_n.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_curve(mut v_speeds: Py, mut v_powers: Py) -> R {
    let _ = f__need({ let t5702 = py::len(&v_speeds.clone())?; Py::Bool({ let t5703 = py::len(&v_powers.clone())?; if py::eq(&t5702, &t5703) { true } else { false } }) }, Py::str("give one power for every wind speed of the curve"))?;
    let _ = f__need({ let t5704 = py::len(&v_speeds.clone())?; Py::Bool({ let t5705 = Py::Int(2i64); if py::ge(&t5704, &t5705)? { true } else { false } }) }, Py::str("the power curve needs at least two points"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5706 in py::iter(&(py::zip(&[v_speeds.clone(), py::slice(&v_speeds.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5707 = (t5706).items()?;
if t5707.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5707[0].clone();
v_b = t5707[1].clone();
out.push({ let t5708 = v_b.clone(); Py::Bool({ let t5709 = v_a.clone(); if py::gt(&t5708, &t5709)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the wind speeds of the curve must increase strictly"))?;
    let _ = f__need({ let t5715 = { let t5710 = py::getitem(&v_speeds.clone(), &Py::Int(0i64))?; Py::Bool({ let t5711 = Py::Int(0i64); if py::ge(&t5710, &t5711)? { true } else { false } }) }; if !t5715.truthy() { t5715 } else { Py::Bool(({ let mut out = vec![]; for t5712 in py::iter(&(v_powers.clone()))? { let mut v_p: Py; 
v_p = t5712;
out.push({ let t5713 = v_p.clone(); Py::Bool({ let t5714 = Py::Int(0i64); if py::ge(&t5713, &t5714)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("speeds and powers must not be negative"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_day(mut v_n: Py) -> R {
    let _ = f__need({ let t4982 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4982.truthy() { t4982 } else { { let t4979 = Py::Int(1i64); Py::Bool({ let t4980 = v_n.clone(); if py::le(&t4979, &t4980)? { { let t4981 = Py::Int(366i64); if py::le(&t4980, &t4981)? { true } else { false } } } else { false } }) } } }, Py::str("the day of the year n must be a whole number from 1 to 366"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_im(mut v_VL: Py, mut v_f: Py, mut v_poles: Py, mut v_R1: Py, mut v_X1: Py, mut v_R2: Py, mut v_X2: Py, mut v_Xm: Py) -> R {
    let _ = f__pos(Py::str("the line voltage and frequency"), Py::tuple(vec![v_VL.clone(), v_f.clone()]))?;
    let _ = f__need({ let t2352 = Py::Bool(matches!(v_poles.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2352.truthy() { t2352 } else { { let t2351 = { let t2347 = v_poles.clone(); Py::Bool({ let t2348 = Py::Int(2i64); if py::ge(&t2347, &t2348)? { true } else { false } }) }; if !t2351.truthy() { t2351 } else { { let t2349 = py::modulo(&v_poles.clone(), &Py::Int(2i64))?; Py::Bool({ let t2350 = Py::Int(0i64); if py::eq(&t2349, &t2350) { true } else { false } }) } } } } }, Py::str("the number of poles must be an even whole number (2, 4, 6, ...)"))?;
    let _ = f__nonneg(Py::str("the stator resistance and reactance"), Py::tuple(vec![v_R1.clone(), v_X1.clone()]))?;
    let _ = f__pos(Py::str("the rotor resistance and reactance and the magnetizing reactance"), Py::tuple(vec![v_R2.clone(), v_X2.clone(), v_Xm.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_irr(mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py) -> R {
    let _ = f__nonneg(Py::str("the irradiances"), Py::tuple(vec![v_dni.clone(), v_ghi.clone(), v_dhi.clone()]))?;
    let _ = f__need({ let t3604 = v_dhi.clone(); Py::Bool({ let t3605 = py::add(&v_ghi.clone(), &Py::Float(1e-09))?; if py::le(&t3604, &t3605)? { true } else { false } }) }, Py::str("the diffuse horizontal irradiance cannot exceed the global horizontal irradiance"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_place(mut v_latitude: Py, mut v_longitude: Py, mut v_utc_offset: Py) -> R {
    if ({ let t4983 = v_latitude.clone(); Py::Bool({ let t4984 = Py::None; if !py::is(&t4983, &t4984) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4988 = Py::Bool(matches!(v_latitude.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4988.truthy() { t4988 } else { { let t4985 = py::neg(&Py::Int(90i64))?; Py::Bool({ let t4986 = v_latitude.clone(); if py::lt(&t4985, &t4986)? { { let t4987 = Py::Int(90i64); if py::lt(&t4986, &t4987)? { true } else { false } } } else { false } }) } } }, Py::str("the latitude must lie between -90 and 90 degrees"))?;
    }
    if ({ let t4989 = v_longitude.clone(); Py::Bool({ let t4990 = Py::None; if !py::is(&t4989, &t4990) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4994 = Py::Bool(matches!(v_longitude.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4994.truthy() { t4994 } else { { let t4991 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t4992 = v_longitude.clone(); if py::le(&t4991, &t4992)? { { let t4993 = Py::Int(180i64); if py::le(&t4992, &t4993)? { true } else { false } } } else { false } }) } } }, Py::str("the longitude must lie between -180 and 180 degrees (east positive)"))?;
    }
    if ({ let t4995 = v_utc_offset.clone(); Py::Bool({ let t4996 = Py::None; if !py::is(&t4995, &t4996) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t5000 = Py::Bool(matches!(v_utc_offset.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5000.truthy() { t5000 } else { { let t4997 = py::neg(&Py::Int(12i64))?; Py::Bool({ let t4998 = v_utc_offset.clone(); if py::le(&t4997, &t4998)? { { let t4999 = Py::Int(14i64); if py::le(&t4998, &t4999)? { true } else { false } } } else { false } }) } } }, Py::str("the UTC offset must lie between -12 and +14 hours"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_sun(mut v_z: Py, mut v_az: Py) -> R {
    let _ = f__need({ let t135 = Py::Bool(matches!(v_z.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t135.truthy() { t135 } else { { let t132 = Py::Int(0i64); Py::Bool({ let t133 = v_z.clone(); if py::le(&t132, &t133)? { { let t134 = Py::Int(180i64); if py::le(&t133, &t134)? { true } else { false } } } else { false } }) } } }, Py::str("the solar zenith must lie between 0 and 180 degrees"))?;
    let _ = f__need({ let t139 = Py::Bool(matches!(v_az.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t139.truthy() { t139 } else { { let t136 = Py::Int(0i64); Py::Bool({ let t137 = v_az.clone(); if py::le(&t136, &t137)? { { let t138 = Py::Int(360i64); if py::le(&t137, &t138)? { true } else { false } } } else { false } }) } } }, Py::str("the solar azimuth must lie between 0 and 360 degrees"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_surface(mut v_tilt: Py, mut v_saz: Py) -> R {
    let _ = f__need({ let t3587 = Py::Bool(matches!(v_tilt.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3587.truthy() { t3587 } else { { let t3584 = Py::Int(0i64); Py::Bool({ let t3585 = v_tilt.clone(); if py::le(&t3584, &t3585)? { { let t3586 = Py::Int(180i64); if py::le(&t3585, &t3586)? { true } else { false } } } else { false } }) } } }, Py::str("the tilt must lie between 0 and 180 degrees"))?;
    let _ = f__need({ let t3591 = Py::Bool(matches!(v_saz.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3591.truthy() { t3591 } else { { let t3588 = Py::Int(0i64); Py::Bool({ let t3589 = v_saz.clone(); if py::le(&t3588, &t3589)? { { let t3590 = Py::Int(360i64); if py::le(&t3589, &t3590)? { true } else { false } } } else { false } }) } } }, Py::str("the surface azimuth must lie between 0 and 360 degrees (clockwise from north)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_temp(mut v_name: Py, mut v_T: Py) -> R {
    let _ = f__need({ let t4060 = Py::Bool(matches!(v_T.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4060.truthy() { t4060 } else { { let t4057 = Py::Int(150i64); Py::Bool({ let t4058 = v_T.clone(); if py::le(&t4057, &t4058)? { { let t4059 = Py::Int(400i64); if py::le(&t4058, &t4059)? { true } else { false } } } else { false } }) } } }, Py::str(&["the ".to_string(), py::format(&v_name.clone(), "")?, " must lie between 150 K and 400 K".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_weibull(mut v_k: Py, mut v_c: Py) -> R {
    let _ = f__need({ let t5777 = Py::Bool(matches!(v_k.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5777.truthy() { t5777 } else { { let t5774 = Py::Float(1.0); Py::Bool({ let t5775 = v_k.clone(); if py::lt(&t5774, &t5775)? { { let t5776 = Py::Float(5.0); if py::le(&t5775, &t5776)? { true } else { false } } } else { false } }) } } }, Py::str("the Weibull shape k must lie in (1, 5]"))?;
    let _ = f__pos(Py::str("the Weibull scale c"), Py::tuple(vec![v_c.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cosd(mut v_x: Py) -> R {
    return Ok(py::math1("cos", &py::math1("radians", &v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__day_angle(mut v_n: Py, mut v_offset: Py) -> R {
    return Ok(py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &Py::Float(365.0))?, &py::sub(&v_n.clone(), &v_offset.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__decl_cooper(mut v_n: Py) -> R {
    return Ok(py::math1("radians", &py::mul(&Py::Float(23.45), &py::math1("sin", &py::add(&f__day_angle(v_n.clone(), Py::Int(1i64))?, &py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &Py::Float(365.0))?, &Py::Float(285.0))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__decl_spencer(mut v_n: Py) -> R {
    let mut v_b: Py = Py::None;
    v_b = f__day_angle(v_n.clone(), Py::Int(1i64))?;
    return Ok(py::add(&py::sub(&py::add(&py::sub(&py::add(&py::sub(&Py::Float(0.006918), &py::mul(&Py::Float(0.399912), &py::math1("cos", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.070257), &py::math1("sin", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.006758), &py::math1("cos", &py::mul(&Py::Float(2.0), &v_b.clone())?)?)?)?, &py::mul(&Py::Float(0.000907), &py::math1("sin", &py::mul(&Py::Float(2.0), &v_b.clone())?)?)?)?, &py::mul(&Py::Float(0.002697), &py::math1("cos", &py::mul(&Py::Float(3.0), &v_b.clone())?)?)?)?, &py::mul(&Py::Float(0.00148), &py::math1("sin", &py::mul(&Py::Float(3.0), &v_b.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__degC(mut v_T: Py) -> R {
    return Ok(py::sub(&v_T.clone(), &Py::Float(273.15))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__desoto(mut v_G: Py, mut v_T_cell: Py, mut v_alpha_sc: Py, mut v_a_ref: Py, mut v_IL_ref: Py, mut v_I0_ref: Py, mut v_Rsh_ref: Py, mut v_Rs: Py, mut v_EgRef: Py, mut v_dEgdT: Py) -> R {
    let mut v_Eg: Py = Py::None;
    let mut v_I0: Py = Py::None;
    let mut v_IL: Py = Py::None;
    let mut v_Rsh: Py = Py::None;
    let mut v_Tc: Py = Py::None;
    let mut v_Tref: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::str("the effective irradiance"), Py::tuple(vec![v_G.clone()]))?;
    let _ = f__need({ let t3951 = Py::Int(200i64); Py::Bool({ let t3952 = v_T_cell.clone(); if py::le(&t3951, &t3952)? { { let t3953 = Py::Int(400i64); if py::le(&t3952, &t3953)? { true } else { false } } } else { false } }) }, Py::str("the cell temperature must lie between 200 K and 400 K"))?;
    let _ = f__pos(Py::str("a_ref, IL_ref, I0_ref, Rsh_ref and EgRef"), Py::tuple(vec![v_a_ref.clone(), v_IL_ref.clone(), v_I0_ref.clone(), v_Rsh_ref.clone(), v_EgRef.clone()]))?;
    let _ = f__nonneg(Py::str("Rs"), Py::tuple(vec![v_Rs.clone()]))?;
    let t3959 = (Py::tuple(vec![py::add(&Py::Int(25i64), &Py::Float(273.15))?, py::add(&f__degC(v_T_cell.clone())?, &Py::Float(273.15))?])).items()?;
    if t3959.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Tref = t3959[0].clone();
    v_Tc = t3959[1].clone();
    v_Eg = py::mul(&v_EgRef.clone(), &py::add(&Py::Int(1i64), &py::mul(&v_dEgdT.clone(), &py::sub(&v_Tc.clone(), &v_Tref.clone())?)?)?)?;
    v_a = py::mul(&v_a_ref.clone(), &py::truediv(&v_Tc.clone(), &v_Tref.clone())?)?;
    v_IL = py::mul(&py::truediv(&v_G.clone(), &Py::Int(1000i64))?, &py::add(&v_IL_ref.clone(), &py::mul(&v_alpha_sc.clone(), &py::sub(&v_Tc.clone(), &v_Tref.clone())?)?)?)?;
    v_I0 = py::mul(&py::mul(&v_I0_ref.clone(), &py::pow(&py::truediv(&v_Tc.clone(), &v_Tref.clone())?, &Py::Int(3i64))?)?, &py::math1("exp", &py::sub(&py::truediv(&v_EgRef.clone(), &py::mul(&c_K_B_EV()?, &v_Tref.clone())?)?, &py::truediv(&v_Eg.clone(), &py::mul(&c_K_B_EV()?, &v_Tc.clone())?)?)?)?)?;
    v_Rsh = py::mul(&v_Rsh_ref.clone(), &py::truediv(&Py::Int(1000i64), &v_G.clone())?)?;
    return Ok(Py::tuple(vec![v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eot_pvcdrom(mut v_n: Py) -> R {
    let mut v_bd: Py = Py::None;
    v_bd = py::sub(&f__day_angle(v_n.clone(), Py::Int(1i64))?, &py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &Py::Float(365.0))?, &Py::Float(80.0))?)?;
    return Ok(py::sub(&py::sub(&py::mul(&Py::Float(9.87), &py::math1("sin", &py::mul(&Py::Float(2.0), &v_bd.clone())?)?)?, &py::mul(&Py::Float(7.53), &py::math1("cos", &v_bd.clone())?)?)?, &py::mul(&Py::Float(1.5), &py::math1("sin", &v_bd.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eot_spencer(mut v_n: Py) -> R {
    let mut v_b: Py = Py::None;
    v_b = f__day_angle(v_n.clone(), Py::Int(1i64))?;
    return Ok(py::mul(&py::truediv(&py::truediv(&Py::Float(1440.0), &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &py::sub(&py::sub(&py::sub(&py::add(&Py::Float(7.5e-06), &py::mul(&Py::Float(0.001868), &py::math1("cos", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.032077), &py::math1("sin", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.014615), &py::math1("cos", &py::mul(&Py::Float(2.0), &v_b.clone())?)?)?)?, &py::mul(&Py::Float(0.040849), &py::math1("sin", &py::mul(&Py::Float(2.0), &v_b.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fault_network(mut v_network: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_edges: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_edges = Py::list(vec![]);
    for t5050 in py::iter(&(v_network.clone()))? {
        v_e = t5050;
        let _ = f__need({ let t5051 = py::len(&v_e.clone())?; Py::Bool({ let t5052 = Py::Int(3i64); if py::eq(&t5051, &t5052) { true } else { false } }) }, Py::str("each branch is 'i-j x' with its reactance in per unit; node 0 is the reference"))?;
        let t5055 = (Py::tuple(vec![f__bus_index(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::None)?, f__bus_index(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::None)?, py::float(&py::getitem(&v_e.clone(), &Py::Int(2i64))?)?])).items()?;
        if t5055.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_i = t5055[0].clone();
        v_j = t5055[1].clone();
        v_x = t5055[2].clone();
        let _ = f__need({ let t5056 = v_i.clone(); Py::Bool({ let t5057 = v_j.clone(); if py::ne(&t5056, &t5057) { true } else { false } }) }, Py::str("a branch must join two different nodes"))?;
        let _ = f__need({ let t5058 = v_x.clone(); Py::Bool({ let t5059 = Py::Int(0i64); if py::gt(&t5058, &t5059)? { true } else { false } }) }, Py::str("branch reactances must be positive"))?;
        let _ = py::method(&v_edges.clone(), "append", &[Py::tuple(vec![v_i.clone(), v_j.clone(), v_x.clone()])])?;
    }
    v_nb = py::minmax(&[{ let mut out = vec![]; for t5060 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t5061 = (t5060).items()?;
if t5061.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t5061[0].clone();
v_j = t5061[1].clone();
v__ = t5061[2].clone();
out.push(py::minmax(&[v_i.clone(), v_j.clone()], true)?); } Py::list(out) }], true)?;
    let _ = f__need({ let t5062 = py::sub(&py::set(&{ let mut out = vec![]; for t5064 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t5065 = (t5064).items()?;
if t5065.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t5065[0].clone();
v_j = t5065[1].clone();
v__ = t5065[2].clone();
for t5066 in py::iter(&(Py::tuple(vec![v_i.clone(), v_j.clone()])))? { let mut v_k: Py; 
v_k = t5066;
out.push(v_k.clone()); }} Py::list(out) })?, &py::set(&Py::list(vec![Py::Int(0i64)]))?)?; Py::Bool({ let t5063 = py::set(&Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nb.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?; if py::eq(&t5062, &t5063) { true } else { false } }) }, Py::str("buses must be numbered 1 to n without gaps"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5067 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t5068 = (t5067).items()?;
if t5068.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t5068[0].clone();
v_j = t5068[1].clone();
v__ = t5068[2].clone();
out.push({ let t5069 = Py::Int(0i64); Py::Bool({ let t5070 = Py::tuple(vec![v_i.clone(), v_j.clone()]); if py::contains(&t5070, &t5069)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())), Py::str("at least one branch must connect to the reference node 0 (a source)"))?;
    return Ok(Py::tuple(vec![v_edges.clone(), v_nb.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_v: Py) -> R {
    let _ = f__need({ let t5255 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5255.truthy() { t5255 } else { { let t5254 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5254.truthy() { t5254 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__g_on_spencer(mut v_n: Py) -> R {
    let mut v_b: Py = Py::None;
    v_b = f__day_angle(v_n.clone(), Py::Int(1i64))?;
    return Ok(py::mul(&c_SOLAR_CONSTANT()?, &py::add(&py::add(&py::add(&py::add(&Py::Float(1.00011), &py::mul(&Py::Float(0.034221), &py::math1("cos", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.00128), &py::math1("sin", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.000719), &py::math1("cos", &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?, &py::mul(&Py::Float(7.7e-05), &py::math1("sin", &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gauss_solve(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t5398 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t5399 = (t5398).items()?;
if t5399.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t5399[0].clone();
v_row = t5399[1].clone();
out.push(py::add(&Py::list((py::map(&{ let v_A = v_A.clone(); let v_b = v_b.clone(); let v_i = v_i.clone(); let v_n = v_n.clone(); let v_row = v_row.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_v.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &v_row.clone())?).items()?), &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_scale = { let t5402 = { let t = { let mut out = vec![]; for t5400 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t5400;
for t5401 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t5401;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(1.0) } else { py::minmax(&[t], true)? } }; if t5402.truthy() { t5402 } else { Py::Float(1.0) } };
    for t5403 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t5403;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        let _ = f__need({ let t5404 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t5405 = py::mul(&Py::Float(1e-13), &v_scale.clone())?; if py::gt(&t5404, &t5405)? { true } else { false } }) }, Py::str("the system is singular: the network does not determine every unknown"))?;
        let t5406 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t5406.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t5406[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t5406[1].clone())?;
        for t5407 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t5407;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t5408 = v_f.clone(); Py::Bool({ let t5409 = Py::Int(0i64); if py::ne(&t5408, &t5409) { true } else { false } }) }).truthy() {
                for t5410 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t5410;
                    let t5411 = py::getitem(&v_M.clone(), &v_r.clone())?; let t5412 = v_k.clone();
                    py::setitem(&t5411, &t5412, py::sub(&py::getitem(&t5411, &t5412)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t5413 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t5413;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::sum(&{ let mut out = vec![]; for t5414 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5414;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__h2_a0(mut v_d: Py, mut v_t: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_a0t: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_a0 = py::add(&py::add(&py::add(&py::math1("log", &v_d.clone())?, &c__L0_A1()?)?, &py::mul(&c__L0_A2()?, &v_t.clone())?)?, &py::mul(&c__L0_LOGTAU()?, &py::math1("log", &v_t.clone())?)?)?;
    v_a0t = py::add(&c__L0_A2()?, &py::truediv(&c__L0_LOGTAU()?, &v_t.clone())?)?;
    for t1923 in py::iter(&(py::zip(&[c__L0_N()?, c__L0_V()?])?))? {
        let t1924 = (t1923).items()?;
        if t1924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1924[0].clone();
        v_v = t1924[1].clone();
        v_x = py::truediv(&py::mul(&v_v.clone(), &v_t.clone())?, &c__H2_TC()?)?;
        v_a0 = py::iadd(&v_a0, &py::mul(&v_n.clone(), &py::math1("log", &py::sub(&Py::Int(1i64), &py::math1("exp", &py::neg(&v_x.clone())?)?)?)?)?)?;
        v_a0t = py::iadd(&v_a0t, &py::truediv(&py::mul(&v_n.clone(), &py::truediv(&v_v.clone(), &c__H2_TC()?)?)?, &py::sub(&py::math1("exp", &v_x.clone())?, &Py::Int(1i64))?)?)?;
    }
    return Ok(Py::tuple(vec![v_a0.clone(), v_a0t.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__h2_ar(mut v_d: Py, mut v_t: Py) -> R {
    let mut v_ar: Py = Py::None;
    let mut v_ard: Py = Py::None;
    let mut v_art: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_gam: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_term: Py = Py::None;
    let mut v_tt: Py = Py::None;
    let t2111 = Py::Float(0.0);
    v_ar = t2111.clone();
    v_ard = t2111.clone();
    v_art = t2111.clone();
    for t2112 in py::iter(&(py::zip(&[c__LP_N()?, c__LP_T()?, c__LP_D()?, c__LP_L()?])?))? {
        let t2113 = (t2112).items()?;
        if t2113.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_n = t2113[0].clone();
        v_tt = t2113[1].clone();
        v_dd = t2113[2].clone();
        v_l = t2113[3].clone();
        v_e = (if (v_l.clone()).truthy() { py::math1("exp", &py::neg(&py::pow(&v_d.clone(), &v_l.clone())?)?)? } else { Py::Float(1.0) });
        v_term = py::mul(&py::mul(&py::mul(&v_n.clone(), &py::pow(&v_d.clone(), &v_dd.clone())?)?, &py::pow(&v_t.clone(), &v_tt.clone())?)?, &v_e.clone())?;
        v_ar = py::iadd(&v_ar, &v_term.clone())?;
        v_ard = py::iadd(&v_ard, &py::truediv(&py::mul(&v_term.clone(), &py::sub(&v_dd.clone(), &py::mul(&v_l.clone(), &py::pow(&v_d.clone(), &v_l.clone())?)?)?)?, &v_d.clone())?)?;
        v_art = py::iadd(&v_art, &py::truediv(&py::mul(&v_term.clone(), &v_tt.clone())?, &v_t.clone())?)?;
    }
    for t2114 in py::iter(&(py::zip(&[c__LG_N()?, c__LG_T()?, c__LG_D()?, c__LG_ETA()?, c__LG_EPS()?, c__LG_BETA()?, c__LG_GAM()?])?))? {
        let t2115 = (t2114).items()?;
        if t2115.len() != 7 { return Err(PyErr::Value("unpack".into())); }
        v_n = t2115[0].clone();
        v_tt = t2115[1].clone();
        v_dd = t2115[2].clone();
        v_eta = t2115[3].clone();
        v_eps = t2115[4].clone();
        v_beta = t2115[5].clone();
        v_gam = t2115[6].clone();
        v_term = py::mul(&py::mul(&py::mul(&v_n.clone(), &py::pow(&v_d.clone(), &v_dd.clone())?)?, &py::pow(&v_t.clone(), &v_tt.clone())?)?, &py::math1("exp", &py::sub(&py::mul(&py::neg(&v_eta.clone())?, &py::pow(&py::sub(&v_d.clone(), &v_eps.clone())?, &Py::Int(2i64))?)?, &py::mul(&v_beta.clone(), &py::pow(&py::sub(&v_t.clone(), &v_gam.clone())?, &Py::Int(2i64))?)?)?)?)?;
        v_ar = py::iadd(&v_ar, &v_term.clone())?;
        v_ard = py::iadd(&v_ard, &py::mul(&v_term.clone(), &py::sub(&py::truediv(&v_dd.clone(), &v_d.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_eta.clone())?, &py::sub(&v_d.clone(), &v_eps.clone())?)?)?)?)?;
        v_art = py::iadd(&v_art, &py::mul(&v_term.clone(), &py::sub(&py::truediv(&v_tt.clone(), &v_t.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_beta.clone())?, &py::sub(&v_t.clone(), &v_gam.clone())?)?)?)?)?;
    }
    return Ok(Py::tuple(vec![v_ar.clone(), v_ard.clone(), v_art.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__h2_check(mut v_p: Py, mut v_T: Py) -> R {
    let _ = f__need({ let t2072 = Py::Float(1000.0); Py::Bool({ let t2073 = v_p.clone(); if py::le(&t2072, &t2073)? { { let t2074 = Py::Float(150000000.0); if py::le(&t2073, &t2074)? { true } else { false } } } else { false } }) }, Py::str("the pressure must lie between 1 kPa and 1500 bar"))?;
    let _ = f__need({ let t2075 = Py::Int(200i64); Py::Bool({ let t2076 = v_T.clone(); if py::le(&t2075, &t2076)? { { let t2077 = Py::Int(700i64); if py::le(&t2076, &t2077)? { true } else { false } } } else { false } }) }, Py::str("the temperature must lie between 200 K and 700 K (gaseous hydrogen)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__h2_p(mut v_rho: Py, mut v_T: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t2110 = (Py::tuple(vec![py::truediv(&v_rho.clone(), &c__H2_RHOC()?)?, py::truediv(&c__H2_TC()?, &v_T.clone())?])).items()?;
    if t2110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t2110[0].clone();
    v_t = t2110[1].clone();
    return Ok(py::mul(&py::mul(&py::mul(&v_rho.clone(), &c__H2_R()?)?, &v_T.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_d.clone(), &py::getitem(&f__h2_ar(v_d.clone(), v_t.clone())?, &Py::Int(1i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__h2_rho(mut v_p: Py, mut v_T: Py) -> R {
    let mut v_hi: Py = Py::None;
    v_hi = py::truediv(&v_p.clone(), &py::mul(&c__H2_R()?, &v_T.clone())?)?;
    return Ok(f__illinois({ let v_T = v_T.clone(); let v_hi = v_hi.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::truediv(&f__h2_p(v_r.clone(), v_T.clone())?, &v_p.clone())?, &Py::Float(1.0))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::mul(&v_hi.clone(), &Py::Float(0.001))?, v_hi.clone(), Py::Int(200i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__h2_s_h(mut v_p: Py, mut v_T: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_a0t: Py = Py::None;
    let mut v_ar: Py = Py::None;
    let mut v_ard: Py = Py::None;
    let mut v_art: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_rm: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_rm = f__h2_rho(v_p.clone(), v_T.clone())?;
    let t1921 = (Py::tuple(vec![py::truediv(&v_rm.clone(), &c__H2_RHOC()?)?, py::truediv(&c__H2_TC()?, &v_T.clone())?])).items()?;
    if t1921.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1921[0].clone();
    v_t = t1921[1].clone();
    let t1922 = (f__h2_ar(v_d.clone(), v_t.clone())?).items()?;
    if t1922.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ar = t1922[0].clone();
    v_ard = t1922[1].clone();
    v_art = t1922[2].clone();
    let t1925 = (f__h2_a0(v_d.clone(), v_t.clone())?).items()?;
    if t1925.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a0 = t1925[0].clone();
    v_a0t = t1925[1].clone();
    v_s = py::mul(&c__H2_R()?, &py::sub(&py::sub(&py::mul(&v_t.clone(), &py::add(&v_a0t.clone(), &v_art.clone())?)?, &v_a0.clone())?, &v_ar.clone())?)?;
    v_h = py::mul(&py::mul(&c__H2_R()?, &v_T.clone())?, &py::add(&py::add(&Py::Int(1i64), &py::mul(&v_t.clone(), &py::add(&v_a0t.clone(), &v_art.clone())?)?)?, &py::mul(&v_d.clone(), &v_ard.clone())?)?)?;
    return Ok(Py::tuple(vec![py::truediv(&v_s.clone(), &c_M_H2()?)?, py::truediv(&v_h.clone(), &c_M_H2()?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__habs(mut v_z: Py) -> R {
    return Ok(py::hypot(&[py::real(&v_z.clone())?, py::imag(&v_z.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__illinois(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_side: Py = Py::None;
    let t2078 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t2078.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t2078[0].clone();
    v_fb = t2078[1].clone();
    if ({ let t2079 = v_fa.clone(); Py::Bool({ let t2080 = Py::Int(0i64); if py::eq(&t2079, &t2080) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t2081 = v_fb.clone(); Py::Bool({ let t2082 = Py::Int(0i64); if py::eq(&t2081, &t2082) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t2083 = { let t2085 = v_fa.clone(); Py::Bool({ let t2086 = Py::Int(0i64); if py::gt(&t2085, &t2086)? { true } else { false } }) }; Py::Bool({ let t2084 = { let t2087 = v_fb.clone(); Py::Bool({ let t2088 = Py::Int(0i64); if py::gt(&t2087, &t2088)? { true } else { false } }) }; if py::ne(&t2083, &t2084) { true } else { false } }) }, Py::str("no root in the interval"))?;
    v_side = Py::Int(0i64);
    for t2089 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t2089;
        v_c = py::truediv(&py::sub(&py::mul(&v_a.clone(), &v_fb.clone())?, &py::mul(&v_b.clone(), &v_fa.clone())?)?, &py::sub(&v_fb.clone(), &v_fa.clone())?)?;
        if (py::not(&{ let t2090 = py::minmax(&[v_a.clone(), v_b.clone()], false)?; Py::Bool({ let t2091 = v_c.clone(); if py::lt(&t2090, &t2091)? { { let t2092 = py::minmax(&[v_a.clone(), v_b.clone()], true)?; if py::lt(&t2091, &t2092)? { true } else { false } } } else { false } }) })).truthy() {
            v_c = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        }
        v_fc = py::call(&v_f.clone(), &[v_c.clone()])?;
        if ({ let t2097 = { let t2093 = v_fc.clone(); Py::Bool({ let t2094 = Py::Int(0i64); if py::eq(&t2093, &t2094) { true } else { false } }) }; if t2097.truthy() { t2097 } else { { let t2095 = py::abs(&py::sub(&v_b.clone(), &v_a.clone())?)?; Py::Bool({ let t2096 = py::mul(&Py::Float(4e-16), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?; if py::le(&t2095, &t2096)? { true } else { false } }) } } }).truthy() {
            return Ok(v_c.clone());
        }
        if ({ let t2098 = { let t2100 = v_fc.clone(); Py::Bool({ let t2101 = Py::Int(0i64); if py::gt(&t2100, &t2101)? { true } else { false } }) }; Py::Bool({ let t2099 = { let t2102 = v_fb.clone(); Py::Bool({ let t2103 = Py::Int(0i64); if py::gt(&t2102, &t2103)? { true } else { false } }) }; if py::eq(&t2098, &t2099) { true } else { false } }) }).truthy() {
            let t2104 = (Py::tuple(vec![v_c.clone(), v_fc.clone()])).items()?;
            if t2104.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_b = t2104[0].clone();
            v_fb = t2104[1].clone();
            if ({ let t2105 = v_side.clone(); Py::Bool({ let t2106 = py::neg(&Py::Int(1i64))?; if py::eq(&t2105, &t2106) { true } else { false } }) }).truthy() {
                v_fa = py::mul(&v_fa, &Py::Float(0.5))?;
            }
            v_side = py::neg(&Py::Int(1i64))?;
        } else {
            let t2107 = (Py::tuple(vec![v_c.clone(), v_fc.clone()])).items()?;
            if t2107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2107[0].clone();
            v_fa = t2107[1].clone();
            if ({ let t2108 = v_side.clone(); Py::Bool({ let t2109 = Py::Int(1i64); if py::eq(&t2108, &t2109) { true } else { false } }) }).truthy() {
                v_fb = py::mul(&v_fb, &Py::Float(0.5))?;
            }
            v_side = Py::Int(1i64);
        }
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__im_phase(mut v_VL: Py, mut v_connection: Py) -> R {
    return Ok((if ({ let t2361 = v_connection.clone(); Py::Bool({ let t2362 = Py::str("star"); if py::eq(&t2361, &t2362) { true } else { false } }) }).truthy() { py::truediv(&v_VL.clone(), &c_SQ3()?)? } else { v_VL.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__interp(mut v_x: Py, mut v_xs: Py, mut v_ys: Py) -> R {
    let mut v_i: Py = Py::None;
    if ({ let t3830 = { let t3826 = v_x.clone(); Py::Bool({ let t3827 = py::getitem(&v_xs.clone(), &Py::Int(0i64))?; if py::lt(&t3826, &t3827)? { true } else { false } }) }; if t3830.truthy() { t3830 } else { { let t3828 = v_x.clone(); Py::Bool({ let t3829 = py::getitem(&v_xs.clone(), &py::neg(&Py::Int(1i64))?)?; if py::gt(&t3828, &t3829)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    for t3831 in py::range_iter(&[(py::sub(&py::len(&v_xs.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t3831;
        if ({ let t3832 = py::getitem(&v_xs.clone(), &v_i.clone())?; Py::Bool({ let t3833 = v_x.clone(); if py::le(&t3832, &t3833)? { { let t3834 = py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?; if py::le(&t3833, &t3834)? { true } else { false } } } else { false } }) }).truthy() {
            if ({ let t3835 = v_x.clone(); Py::Bool({ let t3836 = py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?; if py::eq(&t3835, &t3836) { true } else { false } }) }).truthy() {
                return Ok(py::float(&py::getitem(&v_ys.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?);
            }
            return Ok(py::add(&py::getitem(&v_ys.clone(), &v_i.clone())?, &py::truediv(&py::mul(&py::sub(&py::getitem(&v_ys.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_ys.clone(), &v_i.clone())?)?, &py::sub(&v_x.clone(), &py::getitem(&v_xs.clone(), &v_i.clone())?)?)?, &py::sub(&py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_xs.clone(), &v_i.clone())?)?)?)?);
        }
    }
    return Ok(py::float(&py::getitem(&v_ys.clone(), &py::neg(&Py::Int(1i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__line_ends(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py, mut v_Vr: Py, mut v_P: Py, mut v_pf: Py, mut v_pf_type: Py, mut v_R: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_Ic: Py = Py::None;
    let mut v_Ps: Py = Py::None;
    let mut v_Ss: Py = Py::None;
    let mut v_Vs: Py = Py::None;
    let mut v_is_: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_vr: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let t2734 = (f__load_current(v_Vr.clone(), v_P.clone(), v_pf.clone(), v_pf_type.clone())?).items()?;
    if t2734.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t2734[0].clone();
    v_I = t2734[1].clone();
    v_Ic = t2734[2].clone();
    v_vr = py::truediv(&v_Vr.clone(), &c_SQ3()?)?;
    v_vs = py::add(&py::mul(&v_A.clone(), &v_vr.clone())?, &py::mul(&v_B.clone(), &v_Ic.clone())?)?;
    v_is_ = py::add(&py::mul(&v_C.clone(), &v_vr.clone())?, &py::mul(&v_D.clone(), &v_Ic.clone())?)?;
    v_Ss = py::mul(&py::mul(&Py::Int(3i64), &v_vs.clone())?, &py::method(&v_is_.clone(), "conjugate", &[])?)?;
    v_Vs = py::mul(&c_SQ3()?, &f__habs(v_vs.clone())?)?;
    v_Ps = (if ({ let t2735 = v_R.clone(); Py::Bool({ let t2736 = Py::Int(0i64); if py::eq(&t2735, &t2736) { true } else { false } }) }).truthy() { v_P.clone() } else { py::real(&v_Ss.clone())? });
    return Ok(f__out(Py::dict(vec![(Py::str("Vs"), v_Vs.clone()), (Py::str("delta_deg"), py::math1("degrees", &py::cmath("phase", &[v_vs.clone()])?)?), (Py::str("Is"), f__habs(v_is_.clone())?), (Py::str("Ps"), v_Ps.clone()), (Py::str("losses"), py::sub(&v_Ps.clone(), &v_P.clone())?), (Py::str("regulation_pct"), py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&py::truediv(&v_Vs.clone(), &f__habs(v_A.clone())?)?, &v_Vr.clone())?)?, &v_Vr.clone())?), (Py::str("efficiency_pct"), py::truediv(&py::mul(&Py::Int(100i64), &v_P.clone())?, &v_Ps.clone())?), (Py::str("A_mag"), f__habs(v_A.clone())?), (Py::str("A_angle_deg"), py::math1("degrees", &py::cmath("phase", &[v_A.clone()])?)?), (Py::str("B_mag"), f__habs(v_B.clone())?), (Py::str("B_angle_deg"), py::math1("degrees", &py::cmath("phase", &[v_B.clone()])?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__load_current(mut v_Vr: Py, mut v_P: Py, mut v_pf: Py, mut v_pf_type: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_phi: Py = Py::None;
    v_phi = f__pf_angle(v_pf.clone(), v_pf_type.clone())?;
    v_I = py::truediv(&v_P.clone(), &py::mul(&py::mul(&c_SQ3()?, &v_Vr.clone())?, &v_pf.clone())?)?;
    return Ok(Py::tuple(vec![v_phi.clone(), v_I.clone(), py::cmath("rect", &[v_I.clone(), py::neg(&v_phi.clone())?])?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5890 in py::iter(&(v_vals.clone()))? {
        v_v = t5890;
        let _ = f__need({ let t5894 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5894.truthy() { t5894 } else { { let t5893 = py::math1("isfinite", &v_v.clone())?; if !t5893.truthy() { t5893 } else { { let t5891 = v_v.clone(); Py::Bool({ let t5892 = Py::Int(0i64); if py::ge(&t5891, &t5892)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t5953 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5954 = (t5953).items()?;
if t5954.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5954[0].clone();
v_v = t5954[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pf_angle(mut v_pf: Py, mut v_pf_type: Py) -> R {
    let mut v_phi: Py = Py::None;
    let _ = f__need({ let t5538 = Py::Bool(matches!(v_pf.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5538.truthy() { t5538 } else { { let t5535 = Py::Int(0i64); Py::Bool({ let t5536 = v_pf.clone(); if py::lt(&t5535, &t5536)? { { let t5537 = Py::Int(1i64); if py::le(&t5536, &t5537)? { true } else { false } } } else { false } }) } } }, Py::str("the power factor must lie in (0, 1]"))?;
    v_phi = py::math1("acos", &v_pf.clone())?;
    return Ok((if ({ let t5539 = v_pf_type.clone(); Py::Bool({ let t5540 = Py::str("leading"); if py::ne(&t5539, &t5540) { true } else { false } }) }).truthy() { v_phi.clone() } else { py::neg(&v_phi.clone())? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__phase_from_seq(mut v_I0: Py, mut v_I1: Py, mut v_I2: Py) -> R {
    return Ok(Py::tuple(vec![py::add(&py::add(&v_I0.clone(), &v_I1.clone())?, &v_I2.clone())?, py::add(&py::add(&v_I0.clone(), &py::mul(&py::mul(&c__A()?, &c__A()?)?, &v_I1.clone())?)?, &py::mul(&c__A()?, &v_I2.clone())?)?, py::add(&py::add(&v_I0.clone(), &py::mul(&c__A()?, &v_I1.clone())?)?, &py::mul(&py::mul(&c__A()?, &c__A()?)?, &v_I2.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poa(mut v_model: Py, mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py, mut v_n: Py) -> R {
    let mut v_AI: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_HB: Py = Py::None;
    let mut v_Rb: Py = Py::None;
    let mut v_cos_tt: Py = Py::None;
    let mut v_direct: Py = Py::None;
    let mut v_dni_extra: Py = Py::None;
    let mut v_ground: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sky: Py = Py::None;
    let mut v_svf: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let _ = f__check_surface(v_tilt.clone(), v_surface_azimuth.clone())?;
    let _ = f__need({ let t3595 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3595.truthy() { t3595 } else { { let t3592 = Py::Int(0i64); Py::Bool({ let t3593 = v_zenith.clone(); if py::le(&t3592, &t3593)? { { let t3594 = Py::Int(90i64); if py::lt(&t3593, &t3594)? { true } else { false } } } else { false } }) } } }, Py::str("the sun must be above the horizon (zenith below 90 degrees)"))?;
    let _ = f__need({ let t3596 = Py::Int(0i64); Py::Bool({ let t3597 = v_azimuth.clone(); if py::le(&t3596, &t3597)? { { let t3598 = Py::Int(360i64); if py::le(&t3597, &t3598)? { true } else { false } } } else { false } }) }, Py::str("the solar azimuth must lie between 0 and 360 degrees"))?;
    let _ = f__check_irr(v_dni.clone(), v_ghi.clone(), v_dhi.clone())?;
    let _ = f__need({ let t3606 = Py::Int(0i64); Py::Bool({ let t3607 = v_albedo.clone(); if py::le(&t3606, &t3607)? { { let t3608 = Py::Int(1i64); if py::le(&t3607, &t3608)? { true } else { false } } } else { false } }) }, Py::str("the albedo must lie between 0 and 1"))?;
    v_p = f__projection(v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone())?;
    v_cos_tt = py::minmax(&[v_p.clone(), Py::Float(0.0)], true)?;
    v_svf = py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &f__cosd(v_tilt.clone())?)?)?;
    if ({ let t3609 = v_model.clone(); Py::Bool({ let t3610 = Py::str("isotropic"); if py::eq(&t3609, &t3610) { true } else { false } }) }).truthy() {
        v_sky = py::mul(&py::mul(&v_dhi.clone(), &py::add(&Py::Int(1i64), &f__cosd(v_tilt.clone())?)?)?, &Py::Float(0.5))?;
    } else {
        if ({ let t3611 = v_model.clone(); Py::Bool({ let t3612 = Py::str("klucher"); if py::eq(&t3611, &t3612) { true } else { false } }) }).truthy() {
            v_F = (if ({ let t3613 = v_ghi.clone(); Py::Bool({ let t3614 = Py::Int(0i64); if py::gt(&t3613, &t3614)? { true } else { false } }) }).truthy() { py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_dhi.clone(), &v_ghi.clone())?, &Py::Int(2i64))?)? } else { Py::Float(0.0) });
            v_sky = py::mul(&py::mul(&py::mul(&v_dhi.clone(), &v_svf.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_F.clone(), &py::pow(&f__sind(py::mul(&Py::Float(0.5), &v_tilt.clone())?)?, &Py::Int(3i64))?)?)?)?, &py::add(&Py::Int(1i64), &py::mul(&py::mul(&v_F.clone(), &py::pow(&v_cos_tt.clone(), &Py::Int(2i64))?)?, &py::pow(&f__sind(v_zenith.clone())?, &Py::Int(3i64))?)?)?)?;
        } else {
            let _ = f__check_day(v_n.clone())?;
            v_dni_extra = f__g_on_spencer(v_n.clone())?;
            v_Rb = py::truediv(&v_cos_tt.clone(), &py::minmax(&[f__cosd(v_zenith.clone())?, Py::Float(0.01745)], true)?)?;
            v_AI = py::truediv(&v_dni.clone(), &v_dni_extra.clone())?;
            if ({ let t3619 = v_model.clone(); Py::Bool({ let t3620 = Py::str("haydavies"); if py::eq(&t3619, &t3620) { true } else { false } }) }).truthy() {
                v_sky = py::add(&py::minmax(&[py::mul(&py::mul(&v_dhi.clone(), &py::sub(&Py::Int(1i64), &v_AI.clone())?)?, &v_svf.clone())?, Py::Float(0.0)], true)?, &py::minmax(&[py::mul(&v_dhi.clone(), &py::mul(&v_AI.clone(), &v_Rb.clone())?)?, Py::Float(0.0)], true)?)?;
            } else {
                v_HB = py::minmax(&[py::mul(&v_dni.clone(), &f__cosd(v_zenith.clone())?)?, Py::Float(0.0)], true)?;
                v_h = py::mul(&py::math1("sqrt", &(if ({ let t3621 = v_ghi.clone(); Py::Bool({ let t3622 = Py::Int(0i64); if py::ne(&t3621, &t3622) { true } else { false } }) }).truthy() { py::truediv(&v_HB.clone(), &v_ghi.clone())? } else { Py::Float(0.0) }))?, &py::pow(&f__sind(py::truediv(&v_tilt.clone(), &Py::Int(2i64))?)?, &Py::Int(3i64))?)?;
                v_t1 = py::mul(&py::sub(&Py::Int(1i64), &v_AI.clone())?, &v_svf.clone())?;
                v_sky = py::mul(&v_dhi.clone(), &py::add(&py::add(&v_t1.clone(), &py::mul(&v_AI.clone(), &v_Rb.clone())?)?, &py::mul(&v_t1.clone(), &v_h.clone())?)?)?;
            }
        }
    }
    v_ground = py::mul(&py::mul(&py::mul(&v_ghi.clone(), &v_albedo.clone())?, &py::sub(&Py::Int(1i64), &py::math1("cos", &py::math1("radians", &v_tilt.clone())?)?)?)?, &Py::Float(0.5))?;
    v_direct = py::minmax(&[py::mul(&v_dni.clone(), &py::math1("cos", &py::math1("radians", &py::math1("degrees", &py::math1("acos", &v_p.clone())?)?)?)?)?, Py::Float(0.0)], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("poa_global"), py::add(&py::add(&v_direct.clone(), &v_sky.clone())?, &v_ground.clone())?), (Py::str("poa_direct"), v_direct.clone()), (Py::str("poa_sky_diffuse"), v_sky.clone()), (Py::str("poa_ground_diffuse"), v_ground.clone()), (Py::str("aoi_deg"), py::math1("degrees", &py::math1("acos", &v_p.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5945 in py::iter(&(v_vals.clone()))? {
        v_v = t5945;
        let _ = f__need({ let t5949 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5949.truthy() { t5949 } else { { let t5948 = py::math1("isfinite", &v_v.clone())?; if !t5948.truthy() { t5948 } else { { let t5946 = v_v.clone(); Py::Bool({ let t5947 = Py::Int(0i64); if py::gt(&t5946, &t5947)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__projection(mut v_tilt: Py, mut v_saz: Py, mut v_z: Py, mut v_az: Py) -> R {
    let mut v_p: Py = Py::None;
    v_p = py::add(&py::mul(&f__cosd(v_tilt.clone())?, &f__cosd(v_z.clone())?)?, &py::mul(&py::mul(&f__sind(v_tilt.clone())?, &f__sind(v_z.clone())?)?, &f__cosd(py::sub(&v_az.clone(), &v_saz.clone())?)?)?)?;
    return Ok(py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), v_p.clone()], false)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pv_array_check(mut v_poa: Py, mut v_T_air: Py, mut v_noct: Py, mut v_pdc0: Py, mut v_gamma: Py) -> R {
    let _ = f__pos(Py::str("the plane-of-array irradiance"), Py::tuple(vec![v_poa.clone()]))?;
    return Ok(f_pv_array_output(v_poa.clone(), v_T_air.clone(), v_noct.clone(), v_pdc0.clone(), v_gamma.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t5958 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5958.truthy() { t5958 } else { { let t5957 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5957.truthy() { t5957 } else { { let t5955 = v_x.clone(); Py::Bool({ let t5956 = Py::None; if py::is(&t5955, &t5956) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Complex(..)))).truthy() {
        return Ok(py::complex(&[f__r(py::real(&v_x.clone())?)?, f__r(py::imag(&v_x.clone())?)?])?);
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5959 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5959;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5962 = { let t5960 = v_x.clone(); Py::Bool({ let t5961 = Py::Int(0i64); if py::eq(&t5960, &t5961) { true } else { false } }) }; if t5962.truthy() { t5962 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t5963 = v_x.clone(); Py::Bool({ let t5964 = Py::Int(0i64); if py::eq(&t5963, &t5964) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t5971 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5971.truthy() { t5971 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5975 = py::not(&{ let t5972 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5972.truthy() { t5972 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5975.truthy() { t5975 } else { { let t5973 = py::len(&v_a.clone())?; Py::Bool({ let t5974 = py::len(&v_b.clone())?; if py::ne(&t5973, &t5974) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t5976 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5977 = (t5976).items()?;
if t5977.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5977[0].clone();
v_y = t5977[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t5978 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t5978.truthy() { t5978 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t5979 = v_a.clone(); Py::Bool({ let t5980 = v_b.clone(); if py::eq(&t5979, &t5980) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t5982: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5981 = (Py::tuple(vec![py::complex(&[v_a.clone()])?, py::complex(&[v_b.clone()])?])).items()?;
            if t5981.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5981[0].clone();
            v_b = t5981[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5982 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5983 = v_d.clone(); Py::Bool({ let t5984 = Py::Int(0i64); if py::eq(&t5983, &t5984) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t5985 = v_s.clone(); Py::Bool({ let t5986 = Py::Float(1e-300); if py::gt(&t5985, &t5986)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sd_check(mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_a: Py) -> R {
    let _ = f__pos(Py::str("the photocurrent, saturation current, shunt resistance and nNsVth"), Py::tuple(vec![v_IL.clone(), v_I0.clone(), v_Rsh.clone(), v_a.clone()]))?;
    let _ = f__nonneg(Py::str("the series resistance"), Py::tuple(vec![v_Rs.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sd_current(mut v_V: Py, mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_a: Py) -> R {
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let t4384 = (Py::tuple(vec![py::neg(&Py::Float(1.0))?, py::add(&py::add(&v_IL.clone(), &v_I0.clone())?, &Py::Float(1.0))?])).items()?;
    if t4384.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4384[0].clone();
    v_hi = t4384[1].clone();
    while ({ let t4387 = f__sd_g(v_lo.clone(), v_V.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?; Py::Bool({ let t4388 = Py::Int(0i64); if py::lt(&t4387, &t4388)? { true } else { false } }) }).truthy() {
        v_lo = py::mul(&v_lo, &Py::Int(2i64))?;
        let _ = f__need({ let t4385 = v_lo.clone(); Py::Bool({ let t4386 = py::neg(&Py::Float(1000000000000.0))?; if py::gt(&t4385, &t4386)? { true } else { false } }) }, Py::str("no solution for the current at this voltage"))?;
    }
    return Ok(f__bisect({ let v_I0 = v_I0.clone(); let v_IL = v_IL.clone(); let v_Rs = v_Rs.clone(); let v_Rsh = v_Rsh.clone(); let v_V = v_V.clone(); let v_a = v_a.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_I: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__sd_g(v_I.clone(), v_V.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_lo.clone(), v_hi.clone(), Py::Int(400i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sd_g(mut v_I: Py, mut v_V: Py, mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_a: Py) -> R {
    let mut v_x: Py = Py::None;
    v_x = py::truediv(&py::add(&v_V.clone(), &py::mul(&v_I.clone(), &v_Rs.clone())?)?, &v_a.clone())?;
    if ({ let t4490 = v_x.clone(); Py::Bool({ let t4491 = Py::Int(700i64); if py::gt(&t4490, &t4491)? { true } else { false } }) }).truthy() {
        return Ok(py::neg(&Py::Float(f64::INFINITY))?);
    }
    return Ok(py::sub(&py::sub(&py::sub(&v_IL.clone(), &py::mul(&v_I0.clone(), &py::math1("expm1", &v_x.clone())?)?)?, &py::truediv(&py::add(&v_V.clone(), &py::mul(&v_I.clone(), &v_Rs.clone())?)?, &v_Rsh.clone())?)?, &v_I.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sd_mpp(mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_a: Py) -> R {
    let mut v_Imp: Py = Py::None;
    let mut v_Isc: Py = Py::None;
    let mut v_Iv: Py = Py::None;
    let mut v_Vmp: Py = Py::None;
    let mut v_Voc: Py = Py::None;
    let mut v_dP: Py = Py::None;
    let mut v_vd: Py = Py::None;
    v_Isc = f__sd_current(Py::Float(0.0), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?;
    v_Voc = f__sd_voc(v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?;
    v_Iv = { let v_I0 = v_I0.clone(); let v_IL = v_IL.clone(); let v_Isc = v_Isc.clone(); let v_Rs = v_Rs.clone(); let v_Rsh = v_Rsh.clone(); let v_Voc = v_Voc.clone(); let v_a = v_a.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_vd: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&v_IL.clone(), &py::mul(&v_I0.clone(), &py::math1("expm1", &py::truediv(&v_vd.clone(), &v_a.clone())?)?)?)?, &py::truediv(&v_vd.clone(), &v_Rsh.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_dP = { let v_I0 = v_I0.clone(); let v_IL = v_IL.clone(); let v_Isc = v_Isc.clone(); let v_Iv = v_Iv.clone(); let v_Rs = v_Rs.clone(); let v_Rsh = v_Rsh.clone(); let v_Voc = v_Voc.clone(); let v_a = v_a.clone(); let v_dP = v_dP.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_vd: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_di: Py = Py::None;
            let mut v_dv: Py = Py::None;
            let mut v_i: Py = Py::None;
            v_i = py::call(&v_Iv.clone(), &[v_vd.clone()])?;
            v_di = py::sub(&py::truediv(&py::mul(&py::neg(&v_I0.clone())?, &py::math1("exp", &py::truediv(&v_vd.clone(), &v_a.clone())?)?)?, &v_a.clone())?, &py::truediv(&Py::Int(1i64), &v_Rsh.clone())?)?;
            v_dv = py::sub(&Py::Int(1i64), &py::mul(&v_Rs.clone(), &v_di.clone())?)?;
            return Ok(py::add(&py::mul(&v_i.clone(), &v_dv.clone())?, &py::mul(&py::sub(&v_vd.clone(), &py::mul(&v_i.clone(), &v_Rs.clone())?)?, &v_di.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_vd = f__bisect(v_dP.clone(), Py::Float(0.0), v_Voc.clone(), Py::Int(400i64))?;
    v_Imp = py::call(&v_Iv.clone(), &[v_vd.clone()])?;
    v_Vmp = py::sub(&v_vd.clone(), &py::mul(&v_Imp.clone(), &v_Rs.clone())?)?;
    return Ok(Py::tuple(vec![v_Isc.clone(), v_Voc.clone(), v_Imp.clone(), v_Vmp.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sd_voc(mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_a: Py) -> R {
    return Ok(f__sd_voltage(Py::Float(0.0), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sd_voltage(mut v_I: Py, mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_a: Py) -> R {
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    v_hi = Py::Float(1.0);
    while ({ let t4488 = f__sd_g(v_I.clone(), v_hi.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?; Py::Bool({ let t4489 = Py::Int(0i64); if py::gt(&t4488, &t4489)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Int(2i64))?;
        let _ = f__need({ let t4486 = v_hi.clone(); Py::Bool({ let t4487 = Py::Float(1000000000.0); if py::lt(&t4486, &t4487)? { true } else { false } }) }, Py::str("no solution for the voltage at this current"))?;
    }
    v_lo = py::neg(&Py::Float(1.0))?;
    while ({ let t4494 = f__sd_g(v_I.clone(), v_lo.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?; Py::Bool({ let t4495 = Py::Int(0i64); if py::lt(&t4494, &t4495)? { true } else { false } }) }).truthy() {
        v_lo = py::mul(&v_lo, &Py::Int(2i64))?;
        let _ = f__need({ let t4492 = v_lo.clone(); Py::Bool({ let t4493 = py::neg(&Py::Float(1000000000000.0))?; if py::gt(&t4492, &t4493)? { true } else { false } }) }, Py::str("no solution for the voltage at this current"))?;
    }
    return Ok(f__bisect({ let v_I = v_I.clone(); let v_I0 = v_I0.clone(); let v_IL = v_IL.clone(); let v_Rs = v_Rs.clone(); let v_Rsh = v_Rsh.clone(); let v_a = v_a.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_V: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__sd_g(v_I.clone(), v_V.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_lo.clone(), v_hi.clone(), Py::Int(400i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__seq_check(mut v_X1: Py, mut v_X2: Py, mut v_X0: Py, mut v_Rf: Py, mut v_Xf: Py, mut v_Vf: Py) -> R {
    let _ = f__pos(Py::str("the sequence reactances"), Py::tuple(vec![v_X1.clone(), v_X2.clone(), v_X0.clone()]))?;
    let _ = f__nonneg(Py::str("the fault resistance and reactance"), Py::tuple(vec![v_Rf.clone(), v_Xf.clone()]))?;
    let _ = f__need({ let t4578 = Py::Float(0.5); Py::Bool({ let t4579 = v_Vf.clone(); if py::le(&t4578, &t4579)? { { let t4580 = Py::Float(1.5); if py::le(&t4579, &t4580)? { true } else { false } } } else { false } }) }, Py::str("the prefault voltage must lie between 0.5 and 1.5 pu"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sind(mut v_x: Py) -> R {
    return Ok(py::math1("sin", &py::math1("radians", &v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap0(mut v_x: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t4603 = py::abs(&v_x.clone())?; Py::Bool({ let t4604 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_scale.clone(), Py::Float(1e-300)], true)?)?; if py::le(&t4603, &t4604)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sun_angles(mut v_latitude: Py, mut v_decl_rad: Py, mut v_ha_deg: Py) -> R {
    let mut v_az: Py = Py::None;
    let mut v_ca: Py = Py::None;
    let mut v_cz: Py = Py::None;
    let mut v_denom: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_lat: Py = Py::None;
    let mut v_numer: Py = Py::None;
    let mut v_sgn: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t4922 = (Py::tuple(vec![py::math1("radians", &v_latitude.clone())?, py::math1("radians", &v_ha_deg.clone())?])).items()?;
    if t4922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lat = t4922[0].clone();
    v_h = t4922[1].clone();
    v_cz = py::add(&py::mul(&py::mul(&py::math1("cos", &v_decl_rad.clone())?, &py::math1("cos", &v_lat.clone())?)?, &py::math1("cos", &v_h.clone())?)?, &py::mul(&py::math1("sin", &v_decl_rad.clone())?, &py::math1("sin", &v_lat.clone())?)?)?;
    v_z = py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), v_cz.clone()], false)?], true)?)?;
    v_numer = py::sub(&py::mul(&py::math1("cos", &v_z.clone())?, &py::math1("sin", &v_lat.clone())?)?, &py::math1("sin", &v_decl_rad.clone())?)?;
    v_denom = py::mul(&py::math1("sin", &v_z.clone())?, &py::math1("cos", &v_lat.clone())?)?;
    v_ca = (if ({ let t4923 = py::abs(&v_denom.clone())?; Py::Bool({ let t4924 = Py::Float(1e-08); if py::le(&t4923, &t4924)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&v_numer.clone(), &v_denom.clone())? });
    if ({ let t4925 = py::abs(&py::sub(&v_ca.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t4926 = Py::Float(1e-08); if py::le(&t4925, &t4926)? { true } else { false } }) }).truthy() {
        v_ca = Py::Float(1.0);
    } else {
        if ({ let t4927 = py::abs(&py::add(&v_ca.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t4928 = Py::Float(1e-08); if py::le(&t4927, &t4928)? { true } else { false } }) }).truthy() {
            v_ca = py::neg(&Py::Float(1.0))?;
        }
    }
    v_sgn = py::sub(&{ let t4929 = v_ha_deg.clone(); Py::Bool({ let t4930 = Py::Int(0i64); if py::gt(&t4929, &t4930)? { true } else { false } }) }, &{ let t4931 = v_ha_deg.clone(); Py::Bool({ let t4932 = Py::Int(0i64); if py::lt(&t4931, &t4932)? { true } else { false } }) })?;
    v_az = py::add(&py::mul(&v_sgn.clone(), &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), v_ca.clone()], false)?], true)?)?)?, &Py::Float(std::f64::consts::PI))?;
    return Ok(Py::tuple(vec![py::math1("degrees", &v_z.clone())?, py::math1("degrees", &v_az.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trafo_t_solve(mut v_Z: Py, mut v_Ym: Py, mut v_S: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_fi: Py = Py::None;
    let mut v_fr: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i1: Py = Py::None;
    let mut v_i2: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v1_of: Py = Py::None;
    let mut v_v2: Py = Py::None;
    v_v1_of = { let v_S = v_S.clone(); let v_Ym = v_Ym.clone(); let v_Z = v_Z.clone(); let v_v1_of = v_v1_of.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v2: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_i1: Py = Py::None;
            let mut v_i2: Py = Py::None;
            let mut v_vm: Py = Py::None;
            v_i2 = py::method(&py::truediv(&v_S.clone(), &v_v2.clone())?, "conjugate", &[])?;
            v_vm = py::add(&v_v2.clone(), &py::truediv(&py::mul(&v_i2.clone(), &v_Z.clone())?, &Py::Int(2i64))?)?;
            v_i1 = py::add(&v_i2.clone(), &py::mul(&v_vm.clone(), &v_Ym.clone())?)?;
            return Ok(Py::tuple(vec![py::add(&v_vm.clone(), &py::truediv(&py::mul(&v_i1.clone(), &v_Z.clone())?, &Py::Int(2i64))?)?, v_i1.clone(), v_i2.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_v2 = py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?;
    for t5395 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t5395;
        v_f = py::sub(&py::getitem(&py::call(&v_v1_of.clone(), &[v_v2.clone()])?, &Py::Int(0i64))?, &Py::Int(1i64))?;
        if ({ let t5396 = py::abs(&v_f.clone())?; Py::Bool({ let t5397 = Py::Float(1e-15); if py::lt(&t5396, &t5397)? { true } else { false } }) }).truthy() {
            break;
        }
        v_h = Py::Float(1e-07);
        v_fr = py::truediv(&py::sub(&py::sub(&py::getitem(&py::call(&v_v1_of.clone(), &[py::add(&v_v2.clone(), &v_h.clone())?])?, &Py::Int(0i64))?, &Py::Int(1i64))?, &v_f.clone())?, &v_h.clone())?;
        v_fi = py::truediv(&py::sub(&py::sub(&py::getitem(&py::call(&v_v1_of.clone(), &[py::add(&v_v2.clone(), &py::mul(&Py::Complex(0.0, 1.0), &v_h.clone())?)?])?, &Py::Int(0i64))?, &Py::Int(1i64))?, &v_f.clone())?, &v_h.clone())?;
        v_dx = f__gauss_solve(Py::list(vec![Py::list(vec![py::real(&v_fr.clone())?, py::real(&v_fi.clone())?]), Py::list(vec![py::imag(&v_fr.clone())?, py::imag(&v_fi.clone())?])]), Py::list(vec![py::neg(&py::real(&v_f.clone())?)?, py::neg(&py::imag(&v_f.clone())?)?]))?;
        v_v2 = py::iadd(&v_v2, &py::complex(&[py::getitem(&v_dx.clone(), &Py::Int(0i64))?, py::getitem(&v_dx.clone(), &Py::Int(1i64))?])?)?;
        let _ = f__need({ let t5415 = Py::Float(0.3); Py::Bool({ let t5416 = py::abs(&v_v2.clone())?; if py::lt(&t5415, &t5416)? { { let t5417 = Py::Int(2i64); if py::lt(&t5416, &t5417)? { true } else { false } } } else { false } }) }, Py::str("the transformer cannot supply this load (no load-flow solution)"))?;
    }
    let _ = f__need({ let t5418 = py::abs(&py::sub(&py::getitem(&py::call(&v_v1_of.clone(), &[v_v2.clone()])?, &Py::Int(0i64))?, &Py::Int(1i64))?)?; Py::Bool({ let t5419 = Py::Float(1e-12); if py::lt(&t5418, &t5419)? { true } else { false } }) }, Py::str("the transformer load flow did not converge"))?;
    let t5420 = (py::call(&v_v1_of.clone(), &[v_v2.clone()])?).items()?;
    if t5420.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t5420[0].clone();
    v_i1 = t5420[1].clone();
    v_i2 = t5420[2].clone();
    return Ok(Py::tuple(vec![v_v2.clone(), v_i1.clone(), v_i2.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_actuator_disc(mut v_a: Py, mut v_rho: Py, mut v_D: Py, mut v_v: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_ct: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__need({ let t4 = Py::Bool(matches!(v_a.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4.truthy() { t4 } else { { let t1 = Py::Int(0i64); Py::Bool({ let t2 = v_a.clone(); if py::le(&t1, &t2)? { { let t3 = Py::Float(0.5); if py::lt(&t2, &t3)? { true } else { false } } } else { false } }) } } }, Py::str("the axial induction factor a must lie in [0, 0.5) (momentum theory fails beyond 0.5)"))?;
    let _ = f__pos(Py::str("the air density, rotor diameter and wind speed"), Py::tuple(vec![v_rho.clone(), v_D.clone(), v_v.clone()]))?;
    v_A = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?;
    let t10 = (Py::tuple(vec![py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &py::pow(&py::sub(&Py::Int(1i64), &v_a.clone())?, &Py::Int(2i64))?)?, py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &py::sub(&Py::Int(1i64), &v_a.clone())?)?])).items()?;
    if t10.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cp = t10[0].clone();
    v_ct = t10[1].clone();
    v_q = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_A.clone())?, &py::pow(&v_v.clone(), &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Cp"), v_cp.clone()), (Py::str("Ct"), v_ct.clone()), (Py::str("v_disc"), py::mul(&v_v.clone(), &py::sub(&Py::Int(1i64), &v_a.clone())?)?), (Py::str("v_wake"), py::mul(&v_v.clone(), &py::sub(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_a.clone())?)?)?), (Py::str("P"), py::mul(&py::mul(&v_cp.clone(), &v_q.clone())?, &v_v.clone())?), (Py::str("T"), py::mul(&v_ct.clone(), &v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_air_mass(mut v_zenith: Py, mut v_model: Py, mut v_pressure: Py) -> R {
    let mut v_am: Py = Py::None;
    let _ = f__need({ let t59 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t59.truthy() { t59 } else { { let t56 = Py::Int(0i64); Py::Bool({ let t57 = v_zenith.clone(); if py::le(&t56, &t57)? { { let t58 = Py::Int(90i64); if py::lt(&t57, &t58)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must be from 0 up to (not including) 90 degrees"))?;
    let _ = f__pos(Py::str("the pressure"), Py::tuple(vec![v_pressure.clone()]))?;
    v_am = f__airmass_rel(v_zenith.clone(), v_model.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("AM_relative"), v_am.clone()), (Py::str("AM_absolute"), py::truediv(&py::mul(&v_am.clone(), &v_pressure.clone())?, &Py::Float(101325.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angle_of_incidence(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py) -> R {
    let mut v_p: Py = Py::None;
    let _ = f__check_surface(v_tilt.clone(), v_surface_azimuth.clone())?;
    let _ = f__check_sun(v_zenith.clone(), v_azimuth.clone())?;
    v_p = f__projection(v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("aoi_deg"), py::math1("degrees", &py::math1("acos", &v_p.clone())?)?), (Py::str("cos_aoi"), v_p.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_battery_rc_discharge(mut v_OCV: Py, mut v_R0: Py, mut v_R1: Py, mut v_C1: Py, mut v_I: Py, mut v_t: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_V1: Py = Py::None;
    let mut v_Vt: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_heat: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("the open-circuit voltage, R1 and C1"), Py::tuple(vec![v_OCV.clone(), v_R1.clone(), v_C1.clone()]))?;
    let _ = f__nonneg(Py::str("R0, the current and the time"), Py::tuple(vec![v_R0.clone(), v_I.clone(), v_t.clone()]))?;
    v_tau = py::mul(&v_R1.clone(), &v_C1.clone())?;
    v_e = py::math1("exp", &py::truediv(&py::neg(&v_t.clone())?, &v_tau.clone())?)?;
    v_V1 = py::mul(&py::mul(&v_I.clone(), &v_R1.clone())?, &py::sub(&Py::Int(1i64), &v_e.clone())?)?;
    v_Vt = py::sub(&py::sub(&v_OCV.clone(), &py::mul(&v_I.clone(), &v_R0.clone())?)?, &v_V1.clone())?;
    let _ = f__need({ let t250 = v_Vt.clone(); Py::Bool({ let t251 = Py::Int(0i64); if py::gt(&t250, &t251)? { true } else { false } }) }, Py::str("the terminal voltage would fall to zero: the current is too large for this cell model"))?;
    v_E = py::mul(&v_I.clone(), &py::add(&py::mul(&py::sub(&py::sub(&v_OCV.clone(), &py::mul(&v_I.clone(), &v_R0.clone())?)?, &py::mul(&v_I.clone(), &v_R1.clone())?)?, &v_t.clone())?, &py::mul(&py::mul(&py::mul(&v_I.clone(), &v_R1.clone())?, &v_tau.clone())?, &py::sub(&Py::Int(1i64), &v_e.clone())?)?)?)?;
    v_heat = py::add(&py::mul(&py::mul(&py::mul(&v_I.clone(), &v_I.clone())?, &v_R0.clone())?, &v_t.clone())?, &py::mul(&py::mul(&py::mul(&v_I.clone(), &v_I.clone())?, &v_R1.clone())?, &py::add(&py::sub(&v_t.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_tau.clone())?, &py::sub(&Py::Int(1i64), &v_e.clone())?)?)?, &py::mul(&py::truediv(&v_tau.clone(), &Py::Int(2i64))?, &py::sub(&Py::Int(1i64), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&Py::Int(2i64))?, &v_t.clone())?, &v_tau.clone())?)?)?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_terminal"), v_Vt.clone()), (Py::str("V_rc"), v_V1.clone()), (Py::str("E_delivered"), v_E.clone()), (Py::str("E_Wh"), py::truediv(&v_E.clone(), &Py::Int(3600i64))?), (Py::str("heat"), v_heat.clone()), (Py::str("tau"), v_tau.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_butler_volmer(mut v_j0: Py, mut v_alpha_a: Py, mut v_alpha_c: Py, mut v_T: Py, mut v_eta: Py, mut v_j: Py) -> R {
    let mut v_ba: Py = Py::None;
    let mut v_bc: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_jj: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_rct: Py = Py::None;
    let _ = f__pos(Py::str("the exchange current density j0"), Py::tuple(vec![v_j0.clone()]))?;
    let _ = f__need({ let t302 = v_j0.clone(); Py::Bool({ let t303 = Py::Float(1000000.0); if py::le(&t302, &t303)? { true } else { false } }) }, Py::str("the exchange current density j0 (A/m^2) must be at most 1e6"))?;
    let _ = f__pos(Py::str("the transfer coefficients"), Py::tuple(vec![v_alpha_a.clone(), v_alpha_c.clone()]))?;
    let _ = f__need({ let t308 = { let t304 = v_alpha_a.clone(); Py::Bool({ let t305 = Py::Int(4i64); if py::le(&t304, &t305)? { true } else { false } }) }; if !t308.truthy() { t308 } else { { let t306 = v_alpha_c.clone(); Py::Bool({ let t307 = Py::Int(4i64); if py::le(&t306, &t307)? { true } else { false } }) } } }, Py::str("the transfer coefficients (electrons included) must be at most 4"))?;
    let _ = f__pos(Py::str("the temperature"), Py::tuple(vec![v_T.clone()]))?;
    let _ = f__need({ let t309 = Py::Int(200i64); Py::Bool({ let t310 = v_T.clone(); if py::le(&t309, &t310)? { { let t311 = Py::Int(1500i64); if py::le(&t310, &t311)? { true } else { false } } } else { false } }) }, Py::str("the temperature must lie between 200 and 1500 K"))?;
    let _ = f__need({ let t312 = { let t314 = v_eta.clone(); Py::Bool({ let t315 = Py::None; if py::is(&t314, &t315) { true } else { false } }) }; Py::Bool({ let t313 = { let t316 = v_j.clone(); Py::Bool({ let t317 = Py::None; if py::is(&t316, &t317) { true } else { false } }) }; if py::ne(&t312, &t313) { true } else { false } }) }, Py::str("give exactly one of the overpotential eta and the current density j"))?;
    v_f = py::truediv(&c_F_BV()?, &py::mul(&c_R_BV()?, &v_T.clone())?)?;
    if ({ let t318 = v_eta.clone(); Py::Bool({ let t319 = Py::None; if !py::is(&t318, &t319) { true } else { false } }) }).truthy() {
        let _ = f__fin(Py::str("the overpotential"), v_eta.clone())?;
        let _ = f__need({ let t322 = py::abs(&v_eta.clone())?; Py::Bool({ let t323 = Py::Float(2.0); if py::le(&t322, &t323)? { true } else { false } }) }, Py::str("the overpotential must lie within +-2 V"))?;
        let _ = f__need({ let t328 = { let t324 = py::mul(&py::mul(&v_alpha_a.clone(), &v_f.clone())?, &v_eta.clone())?; Py::Bool({ let t325 = Py::Int(700i64); if py::le(&t324, &t325)? { true } else { false } }) }; if !t328.truthy() { t328 } else { { let t326 = py::mul(&py::mul(&py::neg(&v_alpha_c.clone())?, &v_f.clone())?, &v_eta.clone())?; Py::Bool({ let t327 = Py::Int(700i64); if py::le(&t326, &t327)? { true } else { false } }) } } }, Py::str("the overpotential is too large for this temperature (the current overflows)"))?;
        v_e = v_eta.clone();
        v_jj = py::mul(&v_j0.clone(), &py::sub(&py::math1("expm1", &py::mul(&py::mul(&v_alpha_a.clone(), &v_f.clone())?, &v_e.clone())?)?, &py::math1("expm1", &py::mul(&py::mul(&py::neg(&v_alpha_c.clone())?, &v_f.clone())?, &v_e.clone())?)?)?)?;
    } else {
        let _ = f__fin(Py::str("the current density"), v_j.clone())?;
        let _ = f__need({ let t329 = py::abs(&v_j.clone())?; Py::Bool({ let t330 = Py::Float(100000000.0); if py::le(&t329, &t330)? { true } else { false } }) }, Py::str("the current density (A/m^2) must be at most 1e8 in magnitude"))?;
        if ({ let t331 = v_j.clone(); Py::Bool({ let t332 = Py::Int(0i64); if py::eq(&t331, &t332) { true } else { false } }) }).truthy() {
            v_e = Py::Float(0.0);
        } else {
            if ({ let t333 = v_j.clone(); Py::Bool({ let t334 = Py::Int(0i64); if py::gt(&t333, &t334)? { true } else { false } }) }).truthy() {
                let t335 = (Py::tuple(vec![Py::Float(0.0), py::truediv(&py::math1("log1p", &py::truediv(&v_j.clone(), &v_j0.clone())?)?, &py::mul(&v_alpha_a.clone(), &v_f.clone())?)?])).items()?;
                if t335.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_lo = t335[0].clone();
                v_hi = t335[1].clone();
            } else {
                let t336 = (Py::tuple(vec![py::truediv(&py::neg(&py::math1("log1p", &py::truediv(&py::neg(&v_j.clone())?, &v_j0.clone())?)?)?, &py::mul(&v_alpha_c.clone(), &v_f.clone())?)?, Py::Float(0.0)])).items()?;
                if t336.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_lo = t336[0].clone();
                v_hi = t336[1].clone();
            }
            v_e = f__illinois({ let v_T = v_T.clone(); let v_alpha_a = v_alpha_a.clone(); let v_alpha_c = v_alpha_c.clone(); let v_e = v_e.clone(); let v_eta = v_eta.clone(); let v_f = v_f.clone(); let v_hi = v_hi.clone(); let v_j = v_j.clone(); let v_j0 = v_j0.clone(); let v_jj = v_jj.clone(); let v_lo = v_lo.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::mul(&v_j0.clone(), &py::sub(&py::math1("expm1", &py::mul(&py::mul(&v_alpha_a.clone(), &v_f.clone())?, &v_x.clone())?)?, &py::math1("expm1", &py::mul(&py::mul(&py::neg(&v_alpha_c.clone())?, &v_f.clone())?, &v_x.clone())?)?)?)?, &v_j.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_lo.clone(), v_hi.clone(), Py::Int(200i64))?;
        }
        let _ = f__need({ let t369 = py::abs(&v_e.clone())?; Py::Bool({ let t370 = Py::Float(2.0); if py::le(&t369, &t370)? { true } else { false } }) }, Py::str("the current density needs an overpotential beyond +-2 V"))?;
        v_jj = v_j.clone();
    }
    v_ba = py::truediv(&py::math1("log", &Py::Float(10.0))?, &py::mul(&v_alpha_a.clone(), &v_f.clone())?)?;
    v_bc = py::truediv(&py::math1("log", &Py::Float(10.0))?, &py::mul(&v_alpha_c.clone(), &v_f.clone())?)?;
    v_rct = py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&py::add(&v_alpha_a.clone(), &v_alpha_c.clone())?, &v_f.clone())?, &v_j0.clone())?)?;
    v_out = Py::dict(vec![(Py::str("j"), v_jj.clone()), (Py::str("j_mA_cm2"), py::truediv(&v_jj.clone(), &Py::Float(10.0))?), (Py::str("eta"), v_e.clone()), (Py::str("eta_mV"), py::mul(&v_e.clone(), &Py::Float(1000.0))?), (Py::str("j_anodic"), py::mul(&v_j0.clone(), &py::math1("exp", &py::mul(&py::mul(&v_alpha_a.clone(), &v_f.clone())?, &v_e.clone())?)?)?), (Py::str("j_cathodic"), py::mul(&py::neg(&v_j0.clone())?, &py::math1("exp", &py::mul(&py::mul(&py::neg(&v_alpha_c.clone())?, &v_f.clone())?, &v_e.clone())?)?)?), (Py::str("b_a"), v_ba.clone()), (Py::str("b_c"), v_bc.clone()), (Py::str("b_a_mV"), py::mul(&v_ba.clone(), &Py::Float(1000.0))?), (Py::str("b_c_mV"), py::mul(&v_bc.clone(), &Py::Float(1000.0))?), (Py::str("R_ct"), v_rct.clone()), (Py::str("R_ct_ohm_cm2"), py::mul(&v_rct.clone(), &Py::Float(10000.0))?)]);
    if ({ let t371 = py::abs(&v_jj.clone())?; Py::Bool({ let t372 = v_j0.clone(); if py::gt(&t371, &t372)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("eta_tafel"), (if ({ let t373 = v_jj.clone(); Py::Bool({ let t374 = Py::Int(0i64); if py::gt(&t373, &t374)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_ba.clone(), &py::math1("log1p", &py::truediv(&py::sub(&v_jj.clone(), &v_j0.clone())?, &v_j0.clone())?)?)?, &py::math1("log", &Py::Float(10.0))?)? } else { py::truediv(&py::mul(&py::neg(&v_bc.clone())?, &py::math1("log1p", &py::truediv(&py::sub(&py::neg(&v_jj.clone())?, &v_j0.clone())?, &v_j0.clone())?)?)?, &py::math1("log", &Py::Float(10.0))?)? }))?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_capacity_factor(mut v_P_rated: Py, mut v_E: Py, mut v_CF: Py, mut v_hours: Py) -> R {
    let mut v_Emax: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let _ = f__pos(Py::str("the rated power"), Py::tuple(vec![v_P_rated.clone()]))?;
    let _ = f__need({ let t429 = Py::Bool(matches!(v_hours.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t429.truthy() { t429 } else { { let t426 = Py::Int(0i64); Py::Bool({ let t427 = v_hours.clone(); if py::lt(&t426, &t427)? { { let t428 = Py::Int(8784i64); if py::le(&t427, &t428)? { true } else { false } } } else { false } }) } } }, Py::str("the period must be between 0 and 8784 hours"))?;
    let _ = f__need({ let t430 = { let t432 = v_E.clone(); Py::Bool({ let t433 = Py::None; if py::is(&t432, &t433) { true } else { false } }) }; Py::Bool({ let t431 = { let t434 = v_CF.clone(); Py::Bool({ let t435 = Py::None; if py::is(&t434, &t435) { true } else { false } }) }; if py::ne(&t430, &t431) { true } else { false } }) }, Py::str("give exactly one of the energy produced E and the capacity factor CF"))?;
    v_Emax = py::mul(&py::mul(&v_P_rated.clone(), &v_hours.clone())?, &Py::Float(3600.0))?;
    if ({ let t436 = v_E.clone(); Py::Bool({ let t437 = Py::None; if !py::is(&t436, &t437) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the energy produced"), Py::tuple(vec![v_E.clone()]))?;
        let _ = f__need({ let t438 = v_E.clone(); Py::Bool({ let t439 = py::mul(&v_Emax.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t438, &t439)? { true } else { false } }) }, Py::str("the energy produced cannot exceed rated power x period (capacity factor above 1)"))?;
        v_cf = py::truediv(&v_E.clone(), &v_Emax.clone())?;
    } else {
        let _ = f__need({ let t443 = Py::Bool(matches!(v_CF.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t443.truthy() { t443 } else { { let t440 = Py::Int(0i64); Py::Bool({ let t441 = v_CF.clone(); if py::lt(&t440, &t441)? { { let t442 = Py::Int(1i64); if py::le(&t441, &t442)? { true } else { false } } } else { false } }) } } }, Py::str("the capacity factor must lie in (0, 1]"))?;
        v_cf = v_CF.clone();
        v_E = py::mul(&v_cf.clone(), &v_Emax.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("capacity_factor"), v_cf.clone()), (Py::str("capacity_factor_pct"), py::mul(&Py::Int(100i64), &v_cf.clone())?), (Py::str("E"), v_E.clone()), (Py::str("E_MWh"), py::truediv(&v_E.clone(), &Py::Float(3600000000.0))?), (Py::str("full_load_hours"), py::mul(&v_cf.clone(), &v_hours.clone())?), (Py::str("average_power"), py::mul(&v_cf.clone(), &v_P_rated.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_temperature_faiman(mut v_poa: Py, mut v_T_air: Py, mut v_wind: Py, mut v_u0: Py, mut v_u1: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__nonneg(Py::str("the plane-of-array irradiance"), Py::tuple(vec![v_poa.clone()]))?;
    let _ = f__check_temp(Py::str("air temperature"), v_T_air.clone())?;
    let _ = f__nonneg(Py::str("the wind speed"), Py::tuple(vec![v_wind.clone()]))?;
    let _ = f__pos(Py::str("u0"), Py::tuple(vec![v_u0.clone()]))?;
    let _ = f__nonneg(Py::str("u1"), Py::tuple(vec![v_u1.clone()]))?;
    v_t = py::add(&f__degC(v_T_air.clone())?, &py::truediv(&v_poa.clone(), &py::add(&v_u0.clone(), &py::mul(&v_u1.clone(), &v_wind.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_cell"), py::add(&v_t.clone(), &Py::Float(273.15))?), (Py::str("T_cell_C"), v_t.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_temperature_noct(mut v_poa: Py, mut v_T_air: Py, mut v_noct: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__nonneg(Py::str("the plane-of-array irradiance"), Py::tuple(vec![v_poa.clone()]))?;
    let _ = f__check_temp(Py::str("air temperature"), v_T_air.clone())?;
    let _ = f__check_temp(Py::str("NOCT"), v_noct.clone())?;
    v_t = py::add(&f__degC(v_T_air.clone())?, &py::mul(&py::mul(&py::truediv(&py::sub(&f__degC(v_noct.clone())?, &Py::Float(20.0))?, &Py::Float(80.0))?, &v_poa.clone())?, &Py::Float(0.1))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_cell"), py::add(&v_t.clone(), &Py::Float(273.15))?), (Py::str("T_cell_C"), v_t.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_temperature_sapm(mut v_poa: Py, mut v_T_air: Py, mut v_wind: Py, mut v_mount: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dT: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let mut v_tm: Py = Py::None;
    let _ = f__nonneg(Py::str("the plane-of-array irradiance"), Py::tuple(vec![v_poa.clone()]))?;
    let _ = f__check_temp(Py::str("air temperature"), v_T_air.clone())?;
    let _ = f__nonneg(Py::str("the wind speed"), Py::tuple(vec![v_wind.clone()]))?;
    let t611 = (py::getitem(&c__SAPM_MOUNTS()?, &v_mount.clone())?).items()?;
    if t611.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t611[0].clone();
    v_b = t611[1].clone();
    v_dT = t611[2].clone();
    v_tm = py::add(&py::mul(&v_poa.clone(), &py::math1("exp", &py::add(&v_a.clone(), &py::mul(&v_b.clone(), &v_wind.clone())?)?)?)?, &f__degC(v_T_air.clone())?)?;
    v_tc = py::add(&v_tm.clone(), &py::mul(&py::truediv(&v_poa.clone(), &Py::Float(1000.0))?, &v_dT.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_module_C"), v_tm.clone()), (Py::str("T_cell_C"), v_tc.clone()), (Py::str("T_cell"), py::add(&v_tc.clone(), &Py::Float(273.15))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_clear_sky_ineichen(mut v_zenith: Py, mut v_linke_turbidity: Py, mut v_altitude: Py, mut v_n: Py) -> R {
    let mut v_am: Py = Py::None;
    let mut v_bnci: Py = Py::None;
    let mut v_bnci2: Py = Py::None;
    let mut v_cg1: Py = Py::None;
    let mut v_cg2: Py = Py::None;
    let mut v_cz: Py = Py::None;
    let mut v_dni: Py = Py::None;
    let mut v_extra: Py = Py::None;
    let mut v_fh1: Py = Py::None;
    let mut v_fh2: Py = Py::None;
    let mut v_ghi: Py = Py::None;
    let mut v_tl: Py = Py::None;
    let _ = f__need({ let t711 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t711.truthy() { t711 } else { { let t708 = Py::Int(0i64); Py::Bool({ let t709 = v_zenith.clone(); if py::le(&t708, &t709)? { { let t710 = Py::Int(90i64); if py::lt(&t709, &t710)? { true } else { false } } } else { false } }) } } }, Py::str("the sun must be above the horizon (apparent zenith below 90 degrees)"))?;
    let _ = f__need({ let t712 = Py::Int(1i64); Py::Bool({ let t713 = v_linke_turbidity.clone(); if py::le(&t712, &t713)? { { let t714 = Py::Int(10i64); if py::le(&t713, &t714)? { true } else { false } } } else { false } }) }, Py::str("the Linke turbidity must lie between 1 and 10"))?;
    let _ = f__need({ let t715 = py::neg(&Py::Int(500i64))?; Py::Bool({ let t716 = v_altitude.clone(); if py::le(&t715, &t716)? { { let t717 = Py::Int(9000i64); if py::le(&t716, &t717)? { true } else { false } } } else { false } }) }, Py::str("the altitude must lie between -500 m and 9000 m"))?;
    let _ = f__check_day(v_n.clone())?;
    v_am = py::truediv(&py::mul(&f__airmass_rel(v_zenith.clone(), Py::str("kastenyoung1989"))?, &f__alt2pres(v_altitude.clone())?)?, &Py::Float(101325.0))?;
    v_extra = f__g_on_spencer(v_n.clone())?;
    v_cz = py::minmax(&[f__cosd(v_zenith.clone())?, Py::Float(0.0)], true)?;
    v_tl = v_linke_turbidity.clone();
    let t736 = (Py::tuple(vec![py::math1("exp", &py::truediv(&py::neg(&v_altitude.clone())?, &Py::Float(8000.0))?)?, py::math1("exp", &py::truediv(&py::neg(&v_altitude.clone())?, &Py::Float(1250.0))?)?])).items()?;
    if t736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fh1 = t736[0].clone();
    v_fh2 = t736[1].clone();
    let t737 = (Py::tuple(vec![py::add(&py::mul(&Py::Float(5.09e-05), &v_altitude.clone())?, &Py::Float(0.868))?, py::add(&py::mul(&Py::Float(3.92e-05), &v_altitude.clone())?, &Py::Float(0.0387))?])).items()?;
    if t737.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cg1 = t737[0].clone();
    v_cg2 = t737[1].clone();
    v_ghi = py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&v_cg1.clone(), &v_extra.clone())?, &v_cz.clone())?, &v_tl.clone())?, &v_tl.clone())?, &py::minmax(&[py::math1("exp", &py::mul(&py::mul(&py::neg(&v_cg2.clone())?, &v_am.clone())?, &py::add(&v_fh1.clone(), &py::mul(&v_fh2.clone(), &py::sub(&v_tl.clone(), &Py::Int(1i64))?)?)?)?)?, Py::Float(0.0)], true)?)?;
    v_bnci = py::mul(&v_extra.clone(), &py::minmax(&[py::mul(&py::add(&Py::Float(0.664), &py::truediv(&Py::Float(0.163), &v_fh1.clone())?)?, &py::math1("exp", &py::mul(&py::mul(&py::neg(&Py::Float(0.09))?, &v_am.clone())?, &py::sub(&v_tl.clone(), &Py::Int(1i64))?)?)?)?, Py::Float(0.0)], true)?)?;
    v_bnci2 = py::mul(&v_ghi.clone(), &py::minmax(&[py::minmax(&[py::truediv(&py::sub(&Py::Int(1i64), &py::truediv(&py::sub(&Py::Float(0.1), &py::mul(&Py::Float(0.2), &py::math1("exp", &py::neg(&v_tl.clone())?)?)?)?, &py::add(&Py::Float(0.1), &py::truediv(&Py::Float(0.882), &v_fh1.clone())?)?)?)?, &v_cz.clone())?, Py::Float(0.0)], true)?, Py::Float(1e+20)], false)?)?;
    v_dni = py::minmax(&[v_bnci.clone(), v_bnci2.clone()], false)?;
    return Ok(f__out(Py::dict(vec![(Py::str("ghi"), v_ghi.clone()), (Py::str("dni"), v_dni.clone()), (Py::str("dhi"), py::sub(&v_ghi.clone(), &py::mul(&v_dni.clone(), &v_cz.clone())?)?), (Py::str("AM_absolute"), v_am.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coulomb_counting(mut v_SOC0: Py, mut v_Q: Py, mut v_currents: Py, mut v_dt: Py) -> R {
    let mut v_hi: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_net: Py = Py::None;
    let mut v_soc: Py = Py::None;
    let _ = f__need({ let t786 = Py::Bool(matches!(v_SOC0.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t786.truthy() { t786 } else { { let t783 = Py::Int(0i64); Py::Bool({ let t784 = v_SOC0.clone(); if py::le(&t783, &t784)? { { let t785 = Py::Int(1i64); if py::le(&t784, &t785)? { true } else { false } } } else { false } }) } } }, Py::str("the initial state of charge must lie between 0 and 1 (or 0 and 100%)"))?;
    let _ = f__pos(Py::str("the capacity and the time step"), Py::tuple(vec![v_Q.clone(), v_dt.clone()]))?;
    let _ = f__need({ let t792 = py::len(&v_currents.clone())?; Py::Bool({ let t793 = Py::Int(1i64); if py::ge(&t792, &t793)? { true } else { false } }) }, Py::str("give at least one current"))?;
    let t794 = (Py::tuple(vec![v_SOC0.clone(), v_SOC0.clone(), v_SOC0.clone(), Py::Float(0.0)])).items()?;
    if t794.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_soc = t794[0].clone();
    v_lo = t794[1].clone();
    v_hi = t794[2].clone();
    v_net = t794[3].clone();
    for t795 in py::iter(&(v_currents.clone()))? {
        v_i = t795;
        let _ = f__fin(Py::str("each current"), v_i.clone())?;
        v_net = py::iadd(&v_net, &py::mul(&v_i.clone(), &v_dt.clone())?)?;
        v_soc = py::sub(&v_SOC0.clone(), &py::truediv(&v_net.clone(), &v_Q.clone())?)?;
        let t798 = (Py::tuple(vec![py::minmax(&[v_lo.clone(), v_soc.clone()], false)?, py::minmax(&[v_hi.clone(), v_soc.clone()], true)?])).items()?;
        if t798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t798[0].clone();
        v_hi = t798[1].clone();
    }
    let _ = f__need({ let t803 = { let t799 = v_lo.clone(); Py::Bool({ let t800 = py::neg(&Py::Float(1e-12))?; if py::ge(&t799, &t800)? { true } else { false } }) }; if !t803.truthy() { t803 } else { { let t801 = v_hi.clone(); Py::Bool({ let t802 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t801, &t802)? { true } else { false } }) } } }, Py::str("the state of charge leaves the range 0 to 100% (more charge than the capacity)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("SOC"), v_soc.clone()), (Py::str("SOC_min"), v_lo.clone()), (Py::str("SOC_max"), v_hi.clone()), (Py::str("charge_out"), v_net.clone()), (Py::str("charge_out_Ah"), py::truediv(&v_net.clone(), &Py::Int(3600i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_clearing(mut v_Pm: Py, mut v_Pmax_pre: Py, mut v_Pmax_fault: Py, mut v_Pmax_post: Py, mut v_H: Py, mut v_f: Py) -> R {
    let mut v_cc: Py = Py::None;
    let mut v_d0: Py = Py::None;
    let mut v_dc: Py = Py::None;
    let mut v_dm: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the mechanical power and the pre- and post-fault limits"), Py::tuple(vec![v_Pm.clone(), v_Pmax_pre.clone(), v_Pmax_post.clone()]))?;
    let _ = f__nonneg(Py::str("the during-fault limit"), Py::tuple(vec![v_Pmax_fault.clone()]))?;
    let _ = f__need({ let t863 = { let t859 = v_Pm.clone(); Py::Bool({ let t860 = v_Pmax_pre.clone(); if py::lt(&t859, &t860)? { true } else { false } }) }; if !t863.truthy() { t863 } else { { let t861 = v_Pm.clone(); Py::Bool({ let t862 = v_Pmax_post.clone(); if py::lt(&t861, &t862)? { true } else { false } }) } } }, Py::str("the mechanical power must be below the pre- and post-fault transfer limits"))?;
    let _ = f__need({ let t864 = v_Pmax_fault.clone(); Py::Bool({ let t865 = v_Pm.clone(); if py::lt(&t864, &t865)? { true } else { false } }) }, Py::str("the during-fault transfer limit must be below the mechanical power (otherwise the rotor does not accelerate)"))?;
    v_d0 = py::math1("asin", &py::truediv(&v_Pm.clone(), &v_Pmax_pre.clone())?)?;
    v_dm = py::sub(&Py::Float(std::f64::consts::PI), &py::math1("asin", &py::truediv(&v_Pm.clone(), &v_Pmax_post.clone())?)?)?;
    let _ = f__need({ let t866 = v_dm.clone(); Py::Bool({ let t867 = v_d0.clone(); if py::gt(&t866, &t867)? { true } else { false } }) }, Py::str("the post-fault network cannot hold the rotor"))?;
    v_num = py::sub(&py::add(&py::mul(&v_Pm.clone(), &py::sub(&v_dm.clone(), &v_d0.clone())?)?, &py::mul(&v_Pmax_post.clone(), &py::math1("cos", &v_dm.clone())?)?)?, &py::mul(&v_Pmax_fault.clone(), &py::math1("cos", &v_d0.clone())?)?)?;
    v_cc = py::truediv(&v_num.clone(), &py::sub(&v_Pmax_post.clone(), &v_Pmax_fault.clone())?)?;
    let _ = f__need({ let t868 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t869 = v_cc.clone(); if py::lt(&t868, &t869)? { { let t870 = Py::Int(1i64); if py::lt(&t869, &t870)? { true } else { false } } } else { false } }) }, Py::str("no clearing angle keeps the system stable (the decelerating area is too small)"))?;
    v_dc = py::math1("acos", &v_cc.clone())?;
    v_out = Py::dict(vec![(Py::str("delta0_deg"), py::math1("degrees", &v_d0.clone())?), (Py::str("delta_max_deg"), py::math1("degrees", &v_dm.clone())?), (Py::str("delta_cr_deg"), py::math1("degrees", &v_dc.clone())?)]);
    if ({ let t875 = { let t871 = v_H.clone(); Py::Bool({ let t872 = Py::None; if !py::is(&t871, &t872) { true } else { false } }) }; if t875.truthy() { t875 } else { { let t873 = v_f.clone(); Py::Bool({ let t874 = Py::None; if !py::is(&t873, &t874) { true } else { false } }) } } }).truthy() {
        let _ = f__need({ let t880 = { let t876 = v_H.clone(); Py::Bool({ let t877 = Py::None; if !py::is(&t876, &t877) { true } else { false } }) }; if !t880.truthy() { t880 } else { { let t878 = v_f.clone(); Py::Bool({ let t879 = Py::None; if !py::is(&t878, &t879) { true } else { false } }) } } }, Py::str("the clearing time needs both H and f"))?;
        let _ = f__pos(Py::str("H and f"), Py::tuple(vec![v_H.clone(), v_f.clone()]))?;
        let _ = f__need({ let t881 = v_Pmax_fault.clone(); Py::Bool({ let t882 = Py::Int(0i64); if py::eq(&t881, &t882) { true } else { false } }) }, Py::str("a closed-form clearing time exists only for a fault with zero transfer (Pmax_fault = 0)"))?;
        py::setitem(&v_out.clone(), &Py::str("t_cr"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &v_H.clone())?, &py::sub(&v_dc.clone(), &v_d0.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_Pm.clone())?)?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_power_flow(mut v_lines: Py, mut v_P: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_edges: Py = Py::None;
    let mut v_flows: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_theta: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t929 = py::len(&v_lines.clone())?; Py::Bool({ let t930 = Py::Int(1i64); if py::ge(&t929, &t930)? { true } else { false } }) }, Py::str("give at least one line"))?;
    v_edges = Py::list(vec![]);
    for t931 in py::iter(&(v_lines.clone()))? {
        v_e = t931;
        let _ = f__need({ let t932 = py::len(&v_e.clone())?; Py::Bool({ let t933 = Py::Int(3i64); if py::eq(&t932, &t933) { true } else { false } }) }, Py::str("each line is 'i-j x' with its reactance in per unit"))?;
        let t936 = (Py::tuple(vec![f__bus_index(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::None)?, f__bus_index(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::None)?, py::float(&py::getitem(&v_e.clone(), &Py::Int(2i64))?)?])).items()?;
        if t936.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_i = t936[0].clone();
        v_j = t936[1].clone();
        v_x = t936[2].clone();
        let _ = f__need({ let t937 = v_i.clone(); Py::Bool({ let t938 = v_j.clone(); if py::ne(&t937, &t938) { true } else { false } }) }, Py::str("a line must join two different buses"))?;
        let _ = f__need({ let t939 = v_x.clone(); Py::Bool({ let t940 = Py::Int(0i64); if py::gt(&t939, &t940)? { true } else { false } }) }, Py::str("line reactances must be positive"))?;
        let _ = py::method(&v_edges.clone(), "append", &[Py::tuple(vec![v_i.clone(), v_j.clone(), v_x.clone()])])?;
    }
    v_nb = py::minmax(&[{ let mut out = vec![]; for t941 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t942 = (t941).items()?;
if t942.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t942[0].clone();
v_j = t942[1].clone();
v__ = t942[2].clone();
out.push(py::minmax(&[v_i.clone(), v_j.clone()], true)?); } Py::list(out) }], true)?;
    let _ = f__need({ let t943 = py::set(&Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nb.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?; Py::Bool({ let t944 = py::set(&{ let mut out = vec![]; for t945 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t946 = (t945).items()?;
if t946.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t946[0].clone();
v_j = t946[1].clone();
v__ = t946[2].clone();
for t947 in py::iter(&(Py::tuple(vec![v_i.clone(), v_j.clone()])))? { let mut v_b: Py; 
v_b = t947;
out.push(v_b.clone()); }} Py::list(out) })?; if py::eq(&t943, &t944) { true } else { false } }) }, Py::str("buses must be numbered 1 to n without gaps, every bus on a line"))?;
    let _ = f__need({ let t948 = py::len(&v_P.clone())?; Py::Bool({ let t949 = py::sub(&v_nb.clone(), &Py::Int(1i64))?; if py::eq(&t948, &t949) { true } else { false } }) }, Py::str(&["give the net injections of buses 2 to ".to_string(), py::format(&v_nb.clone(), "")?, " (".to_string(), py::format(&py::sub(&v_nb.clone(), &Py::Int(1i64))?, "")?, " values); bus 1 is the slack".to_string()].concat()))?;
    v_B = { let mut out = vec![]; for t950 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v__: Py; 
v__ = t950;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?); } Py::list(out) };
    for t951 in py::iter(&(v_edges.clone()))? {
        let t960 = (t951).items()?;
        if t960.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_i = t960[0].clone();
        v_j = t960[1].clone();
        v_x = t960[2].clone();
        v_b = py::truediv(&Py::Float(1.0), &v_x.clone())?;
        let t952 = py::getitem(&v_B.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t953 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t952, &t953, py::iadd(&py::getitem(&t952, &t953)?, &v_b.clone())?)?;
        let t954 = py::getitem(&v_B.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t955 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
        py::setitem(&t954, &t955, py::iadd(&py::getitem(&t954, &t955)?, &v_b.clone())?)?;
        let t956 = py::getitem(&v_B.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t957 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
        py::setitem(&t956, &t957, py::sub(&py::getitem(&t956, &t957)?, &v_b.clone())?)?;
        let t958 = py::getitem(&v_B.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t959 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t958, &t959, py::sub(&py::getitem(&t958, &t959)?, &v_b.clone())?)?;
    }
    v_th = f__gauss_solve({ let mut out = vec![]; for t978 in py::iter(&(py::slice(&v_B.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_row: Py; 
v_row = t978;
out.push(py::slice(&v_row.clone(), Some((Py::Int(1i64)).i()?), None, None)?); } Py::list(out) }, Py::list((v_P.clone()).items()?))?;
    v_theta = py::add(&Py::list(vec![Py::Float(0.0)]), &v_th.clone())?;
    v_flows = { let mut out = vec![]; for t979 in py::iter(&(v_edges.clone()))? { let mut v_i: Py; let mut v_j: Py; let mut v_x: Py; 
let t980 = (t979).items()?;
if t980.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t980[0].clone();
v_j = t980[1].clone();
v_x = t980[2].clone();
out.push(py::truediv(&py::sub(&py::getitem(&v_theta.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_theta.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?)?, &v_x.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("theta_deg"), { let mut out = vec![]; for t993 in py::iter(&(v_theta.clone()))? { let mut v_t: Py; 
v_t = t993;
out.push(py::math1("degrees", &v_t.clone())?); } Py::list(out) }), (Py::str("flows"), v_flows.clone()), (Py::str("P_slack"), py::neg(&py::sum(&v_P.clone(), None)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_desoto_parameters(mut v_G: Py, mut v_T_cell: Py, mut v_alpha_sc: Py, mut v_a_ref: Py, mut v_IL_ref: Py, mut v_I0_ref: Py, mut v_Rsh_ref: Py, mut v_Rs: Py, mut v_EgRef: Py, mut v_dEgdT: Py) -> R {
    let mut v_I0: Py = Py::None;
    let mut v_IL: Py = Py::None;
    let mut v_Rs_: Py = Py::None;
    let mut v_Rsh: Py = Py::None;
    let mut v_a: Py = Py::None;
    let t1041 = (f__desoto(v_G.clone(), v_T_cell.clone(), v_alpha_sc.clone(), v_a_ref.clone(), v_IL_ref.clone(), v_I0_ref.clone(), v_Rsh_ref.clone(), v_Rs.clone(), v_EgRef.clone(), v_dEgdT.clone())?).items()?;
    if t1041.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_IL = t1041[0].clone();
    v_I0 = t1041[1].clone();
    v_Rs_ = t1041[2].clone();
    v_Rsh = t1041[3].clone();
    v_a = t1041[4].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("IL"), v_IL.clone()), (Py::str("I0"), v_I0.clone()), (Py::str("Rs"), v_Rs_.clone()), (Py::str("Rsh"), v_Rsh.clone()), (Py::str("nNsVth"), v_a.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_disc(mut v_ghi: Py, mut v_zenith: Py, mut v_n: Py, mut v_pressure: Py) -> R {
    let mut v_I0: Py = Py::None;
    let mut v_Kn: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_am: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bb: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dni: Py = Py::None;
    let mut v_kt: Py = Py::None;
    let _ = f__check_day(v_n.clone())?;
    let _ = f__need({ let t1094 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1094.truthy() { t1094 } else { { let t1091 = Py::Int(0i64); Py::Bool({ let t1092 = v_zenith.clone(); if py::le(&t1091, &t1092)? { { let t1093 = Py::Int(85i64); if py::le(&t1092, &t1093)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must lie between 0 and 85 degrees"))?;
    let _ = f__pos(Py::str("the global horizontal irradiance and the pressure"), Py::tuple(vec![v_ghi.clone(), v_pressure.clone()]))?;
    v_b = f__day_angle(v_n.clone(), Py::Int(1i64))?;
    v_I0 = py::mul(&Py::Float(1370.0), &py::add(&py::add(&py::add(&py::add(&Py::Float(1.00011), &py::mul(&Py::Float(0.034221), &py::math1("cos", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.00128), &py::math1("sin", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.000719), &py::math1("cos", &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?, &py::mul(&Py::Float(7.7e-05), &py::math1("sin", &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?)?;
    v_kt = py::minmax(&[py::minmax(&[py::truediv(&v_ghi.clone(), &py::mul(&v_I0.clone(), &py::minmax(&[f__cosd(v_zenith.clone())?, Py::Float(0.065)], true)?)?)?, Py::Float(0.0)], true)?, Py::Float(1.0)], false)?;
    v_am = py::minmax(&[py::truediv(&py::mul(&f__airmass_rel(v_zenith.clone(), Py::str("kasten1966"))?, &v_pressure.clone())?, &Py::Float(101325.0))?, Py::Float(12.0)], false)?;
    if ({ let t1114 = v_kt.clone(); Py::Bool({ let t1115 = Py::Float(0.6); if py::le(&t1114, &t1115)? { true } else { false } }) }).truthy() {
        v_a = py::add(&Py::Float(0.512), &py::mul(&v_kt.clone(), &py::add(&py::neg(&Py::Float(1.56))?, &py::mul(&v_kt.clone(), &py::sub(&Py::Float(2.286), &py::mul(&Py::Float(2.222), &v_kt.clone())?)?)?)?)?)?;
        v_bb = py::add(&Py::Float(0.37), &py::mul(&Py::Float(0.962), &v_kt.clone())?)?;
        v_c = py::add(&py::neg(&Py::Float(0.28))?, &py::mul(&v_kt.clone(), &py::sub(&Py::Float(0.932), &py::mul(&Py::Float(2.048), &v_kt.clone())?)?)?)?;
    } else {
        v_a = py::add(&py::neg(&Py::Float(5.743))?, &py::mul(&v_kt.clone(), &py::add(&Py::Float(21.77), &py::mul(&v_kt.clone(), &py::add(&py::neg(&Py::Float(27.49))?, &py::mul(&Py::Float(11.56), &v_kt.clone())?)?)?)?)?)?;
        v_bb = py::add(&Py::Float(41.4), &py::mul(&v_kt.clone(), &py::add(&py::neg(&Py::Float(118.5))?, &py::mul(&v_kt.clone(), &py::add(&Py::Float(66.05), &py::mul(&Py::Float(31.9), &v_kt.clone())?)?)?)?)?)?;
        v_c = py::add(&py::neg(&Py::Float(47.01))?, &py::mul(&v_kt.clone(), &py::add(&Py::Float(184.2), &py::mul(&v_kt.clone(), &py::add(&py::neg(&Py::Float(222.0))?, &py::mul(&Py::Float(73.81), &v_kt.clone())?)?)?)?)?)?;
    }
    v_Kn = py::sub(&py::add(&Py::Float(0.866), &py::mul(&v_am.clone(), &py::add(&py::neg(&Py::Float(0.122))?, &py::mul(&v_am.clone(), &py::add(&Py::Float(0.0121), &py::mul(&v_am.clone(), &py::add(&py::neg(&Py::Float(0.000653))?, &py::mul(&Py::Float(1.4e-05), &v_am.clone())?)?)?)?)?)?)?)?, &py::add(&v_a.clone(), &py::mul(&v_bb.clone(), &py::math1("exp", &py::mul(&v_c.clone(), &v_am.clone())?)?)?)?)?;
    v_dni = py::minmax(&[py::mul(&v_Kn.clone(), &v_I0.clone())?, Py::Float(0.0)], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dni"), v_dni.clone()), (Py::str("kt"), v_kt.clone()), (Py::str("airmass"), v_am.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_droop_sharing(mut v_R: Py, mut v_ratings: Py, mut v_dP: Py, mut v_f0: Py, mut v_D: Py) -> R {
    let mut v_beta: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__need({ let t1236 = { let t1232 = py::len(&v_R.clone())?; Py::Bool({ let t1233 = py::len(&v_ratings.clone())?; if py::eq(&t1232, &t1233) { true } else { false } }) }; if !t1236.truthy() { t1236 } else { { let t1234 = py::len(&v_R.clone())?; Py::Bool({ let t1235 = Py::Int(1i64); if py::ge(&t1234, &t1235)? { true } else { false } }) } } }, Py::str("give one droop and one rating per generator"))?;
    for t1237 in py::iter(&(py::zip(&[v_R.clone(), v_ratings.clone()])?))? {
        let t1243 = (t1237).items()?;
        if t1243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t1243[0].clone();
        v_s = t1243[1].clone();
        let _ = f__need({ let t1242 = { let t1238 = v_r.clone(); Py::Bool({ let t1239 = Py::Int(0i64); if py::gt(&t1238, &t1239)? { true } else { false } }) }; if !t1242.truthy() { t1242 } else { { let t1240 = v_s.clone(); Py::Bool({ let t1241 = Py::Int(0i64); if py::gt(&t1240, &t1241)? { true } else { false } }) } } }, Py::str("droops and ratings must be positive"))?;
    }
    let _ = f__pos(Py::str("the frequency"), Py::tuple(vec![v_f0.clone()]))?;
    let _ = f__nonneg(Py::str("the load damping D"), Py::tuple(vec![v_D.clone()]))?;
    v_beta = py::add(&py::sum(&{ let mut out = vec![]; for t1254 in py::iter(&(py::zip(&[v_R.clone(), v_ratings.clone()])?))? { let mut v_r: Py; let mut v_s: Py; 
let t1255 = (t1254).items()?;
if t1255.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1255[0].clone();
v_s = t1255[1].clone();
out.push(py::truediv(&v_s.clone(), &py::mul(&v_r.clone(), &v_f0.clone())?)?); } Py::list(out) }, None)?, &v_D.clone())?;
    v_df = py::truediv(&py::neg(&v_dP.clone())?, &v_beta.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("df"), v_df.clone()), (Py::str("f"), py::add(&v_f0.clone(), &v_df.clone())?), (Py::str("dP_gen"), { let mut out = vec![]; for t1268 in py::iter(&(py::zip(&[v_R.clone(), v_ratings.clone()])?))? { let mut v_r: Py; let mut v_s: Py; 
let t1269 = (t1268).items()?;
if t1269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1269[0].clone();
v_s = t1269[1].clone();
out.push(py::truediv(&py::mul(&py::neg(&v_df.clone())?, &v_s.clone())?, &py::mul(&v_r.clone(), &v_f0.clone())?)?); } Py::list(out) }), (Py::str("dP_load_damping"), py::mul(&py::neg(&v_D.clone())?, &v_df.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_economic_dispatch(mut v_costs: Py, mut v_demand: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Ps: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_free: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_hi_l: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_lo_l: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_units: Py = Py::None;
    let _ = f__need({ let t1303 = py::len(&v_costs.clone())?; Py::Bool({ let t1304 = Py::Int(1i64); if py::ge(&t1303, &t1304)? { true } else { false } }) }, Py::str("give at least one generator"))?;
    v_units = Py::list(vec![]);
    for t1305 in py::iter(&(v_costs.clone()))? {
        v_row = t1305;
        let _ = f__need({ let t1306 = py::len(&v_row.clone())?; Py::Bool({ let t1307 = Py::Int(5i64); if py::eq(&t1306, &t1307) { true } else { false } }) }, Py::str("each generator row is: a, b, c, Pmin, Pmax (cost a + b P + c P^2 in $/h, P in MW)"))?;
        let t1308 = (v_row.clone()).items()?;
        if t1308.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1308[0].clone();
        v_b = t1308[1].clone();
        v_c = t1308[2].clone();
        v_lo = t1308[3].clone();
        v_hi = t1308[4].clone();
        let _ = f__need({ let t1309 = v_c.clone(); Py::Bool({ let t1310 = Py::Int(0i64); if py::gt(&t1309, &t1310)? { true } else { false } }) }, Py::str("the quadratic cost coefficient c must be positive"))?;
        let _ = f__need({ let t1311 = Py::Int(0i64); Py::Bool({ let t1312 = v_lo.clone(); if py::le(&t1311, &t1312)? { { let t1313 = v_hi.clone(); if py::lt(&t1312, &t1313)? { true } else { false } } } else { false } }) }, Py::str("each generator needs 0 <= Pmin < Pmax"))?;
        let _ = py::method(&v_units.clone(), "append", &[Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_lo.clone(), v_hi.clone()])])?;
    }
    let _ = f__need({ let t1314 = py::sum(&{ let mut out = vec![]; for t1317 in py::iter(&(v_units.clone()))? { let mut v_u: Py; 
v_u = t1317;
out.push(py::getitem(&v_u.clone(), &Py::Int(3i64))?); } Py::list(out) }, None)?; Py::Bool({ let t1315 = v_demand.clone(); if py::le(&t1314, &t1315)? { { let t1316 = py::sum(&{ let mut out = vec![]; for t1318 in py::iter(&(v_units.clone()))? { let mut v_u: Py; 
v_u = t1318;
out.push(py::getitem(&v_u.clone(), &Py::Int(4i64))?); } Py::list(out) }, None)?; if py::le(&t1315, &t1316)? { true } else { false } } } else { false } }) }, Py::str("the demand must lie between the total minimum and total maximum output"))?;
    v_P = { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_costs = v_costs.clone(); let v_demand = v_demand.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_row = v_row.clone(); let v_units = v_units.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_lam: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t1319 in py::iter(&(v_units.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_lo: Py; 
let t1320 = (t1319).items()?;
if t1320.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1320[0].clone();
v_b = t1320[1].clone();
v_c = t1320[2].clone();
v_lo = t1320[3].clone();
v_hi = t1320[4].clone();
out.push(py::minmax(&[py::minmax(&[py::truediv(&py::sub(&v_lam.clone(), &v_b.clone())?, &py::mul(&Py::Int(2i64), &v_c.clone())?)?, v_lo.clone()], true)?, v_hi.clone()], false)?); } Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_lo_l = py::sub(&py::minmax(&[{ let mut out = vec![]; for t1321 in py::iter(&(v_units.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_lo: Py; 
let t1322 = (t1321).items()?;
if t1322.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1322[0].clone();
v_b = t1322[1].clone();
v_c = t1322[2].clone();
v_lo = t1322[3].clone();
v_hi = t1322[4].clone();
out.push(py::add(&v_b.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_lo.clone())?)?); } Py::list(out) }], false)?, &Py::Int(1i64))?;
    v_hi_l = py::add(&py::minmax(&[{ let mut out = vec![]; for t1323 in py::iter(&(v_units.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_lo: Py; 
let t1324 = (t1323).items()?;
if t1324.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1324[0].clone();
v_b = t1324[1].clone();
v_c = t1324[2].clone();
v_lo = t1324[3].clone();
v_hi = t1324[4].clone();
out.push(py::add(&v_b.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_hi.clone())?)?); } Py::list(out) }], true)?, &Py::Int(1i64))?;
    v_lam = f__bisect({ let v_P = v_P.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_costs = v_costs.clone(); let v_demand = v_demand.clone(); let v_hi = v_hi.clone(); let v_hi_l = v_hi_l.clone(); let v_lo = v_lo.clone(); let v_lo_l = v_lo_l.clone(); let v_row = v_row.clone(); let v_units = v_units.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_l: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sum(&py::call(&v_P.clone(), &[v_l.clone()])?, None)?, &v_demand.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_lo_l.clone(), v_hi_l.clone(), Py::Int(2000i64))?;
    v_Ps = py::call(&v_P.clone(), &[v_lam.clone()])?;
    v_free = { let mut out = vec![]; for t1351 in py::iter(&(py::enumerate(&v_units.clone(), 0)?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_k: Py; let mut v_lo: Py; 
let t1352 = (t1351).items()?;
if t1352.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1352[0].clone();
let t1353 = (t1352[1].clone()).items()?;
if t1353.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1353[0].clone();
v_b = t1353[1].clone();
v_c = t1353[2].clone();
v_lo = t1353[3].clone();
v_hi = t1353[4].clone();
if ({ let t1354 = v_lo.clone(); Py::Bool({ let t1355 = py::getitem(&v_Ps.clone(), &v_k.clone())?; if py::lt(&t1354, &t1355)? { { let t1356 = v_hi.clone(); if py::lt(&t1355, &t1356)? { true } else { false } } } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    let _ = f__need(Py::Bool((v_free.clone()).truthy()), Py::str("every unit is at a limit: the incremental cost is not unique"))?;
    v_k = py::getitem(&v_free.clone(), &Py::Int(0i64))?;
    v_lam = py::add(&py::getitem(&py::getitem(&v_units.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_units.clone(), &v_k.clone())?, &Py::Int(2i64))?)?, &py::getitem(&v_Ps.clone(), &v_k.clone())?)?)?;
    v_cost = py::sum(&{ let mut out = vec![]; for t1357 in py::iter(&(py::zip(&[v_units.clone(), v_Ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_lo: Py; let mut v_p: Py; 
let t1358 = (t1357).items()?;
if t1358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
let t1359 = (t1358[0].clone()).items()?;
if t1359.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1359[0].clone();
v_b = t1359[1].clone();
v_c = t1359[2].clone();
v_lo = t1359[3].clone();
v_hi = t1359[4].clone();
v_p = t1358[1].clone();
out.push(py::add(&py::add(&v_a.clone(), &py::mul(&v_b.clone(), &v_p.clone())?)?, &py::mul(&py::mul(&v_c.clone(), &v_p.clone())?, &v_p.clone())?)?); } Py::list(out) }, None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("lambda_"), v_lam.clone()), (Py::str("P"), v_Ps.clone()), (Py::str("cost"), v_cost.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_electrolyser(mut v_I: Py, mut v_N: Py, mut v_V_cell: Py, mut v_faradaic: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_n_h2: Py = Py::None;
    let _ = f__pos(Py::str("the current"), Py::tuple(vec![v_I.clone()]))?;
    let _ = f__need({ let t1412 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1412.truthy() { t1412 } else { { let t1410 = v_N.clone(); Py::Bool({ let t1411 = Py::Int(1i64); if py::ge(&t1410, &t1411)? { true } else { false } }) } } }, Py::str("the number of cells must be a whole number >= 1"))?;
    let _ = f__need({ let t1413 = Py::Float(1.23); Py::Bool({ let t1414 = v_V_cell.clone(); if py::le(&t1413, &t1414)? { { let t1415 = Py::Float(3.0); if py::le(&t1414, &t1415)? { true } else { false } } } else { false } }) }, Py::str("an electrolysis cell operates between the reversible voltage 1.23 V and about 3 V"))?;
    let _ = f__need({ let t1416 = Py::Int(0i64); Py::Bool({ let t1417 = v_faradaic.clone(); if py::lt(&t1416, &t1417)? { { let t1418 = Py::Int(1i64); if py::le(&t1417, &t1418)? { true } else { false } } } else { false } }) }, Py::str("the Faradaic efficiency must lie in (0, 1]"))?;
    v_n_h2 = py::truediv(&py::mul(&py::mul(&v_faradaic.clone(), &v_N.clone())?, &v_I.clone())?, &py::mul(&Py::Int(2i64), &c_F_FARADAY()?)?)?;
    v_P = py::mul(&py::mul(&v_N.clone(), &v_I.clone())?, &v_V_cell.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("H2_kg_s"), py::mul(&v_n_h2.clone(), &c_M_H2()?)?), (Py::str("H2_kg_h"), py::mul(&py::mul(&v_n_h2.clone(), &c_M_H2()?)?, &Py::Int(3600i64))?), (Py::str("O2_kg_s"), py::mul(&py::truediv(&v_n_h2.clone(), &Py::Int(2i64))?, &c_M_O2()?)?), (Py::str("H2O_kg_s"), py::mul(&v_n_h2.clone(), &c_M_H2O()?)?), (Py::str("specific_energy_kWh_per_kg"), py::truediv(&py::truediv(&v_P.clone(), &py::mul(&v_n_h2.clone(), &c_M_H2()?)?)?, &Py::Float(3600000.0))?), (Py::str("efficiency_HHV"), py::truediv(&py::mul(&v_n_h2.clone(), &c_HHV_H2()?)?, &v_P.clone())?), (Py::str("efficiency_LHV"), py::truediv(&py::mul(&v_n_h2.clone(), &c_LHV_H2()?)?, &v_P.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equation_of_time(mut v_n: Py, mut v_method: Py) -> R {
    let mut v_e: Py = Py::None;
    let _ = f__check_day(v_n.clone())?;
    v_e = (if ({ let t1469 = v_method.clone(); Py::Bool({ let t1470 = Py::str("spencer"); if py::eq(&t1469, &t1470) { true } else { false } }) }).truthy() { f__eot_spencer(v_n.clone())? } else { f__eot_pvcdrom(v_n.clone())? });
    return Ok(f__out(Py::dict(vec![(Py::str("eot_min"), v_e.clone()), (Py::str("eot_s"), py::mul(&Py::Float(60.0), &v_e.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_erbs(mut v_ghi: Py, mut v_zenith: Py, mut v_n: Py) -> R {
    let mut v_df: Py = Py::None;
    let mut v_dhi: Py = Py::None;
    let mut v_kt: Py = Py::None;
    let _ = f__check_day(v_n.clone())?;
    let _ = f__need({ let t1523 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1523.truthy() { t1523 } else { { let t1520 = Py::Int(0i64); Py::Bool({ let t1521 = v_zenith.clone(); if py::le(&t1520, &t1521)? { { let t1522 = Py::Int(85i64); if py::le(&t1521, &t1522)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must lie between 0 and 85 degrees"))?;
    let _ = f__pos(Py::str("the global horizontal irradiance"), Py::tuple(vec![v_ghi.clone()]))?;
    v_kt = py::truediv(&v_ghi.clone(), &py::mul(&f__g_on_spencer(v_n.clone())?, &py::minmax(&[f__cosd(v_zenith.clone())?, Py::Float(0.065)], true)?)?)?;
    v_kt = py::minmax(&[py::minmax(&[v_kt.clone(), Py::Float(0.0)], true)?, Py::Float(1.0)], false)?;
    if ({ let t1529 = v_kt.clone(); Py::Bool({ let t1530 = Py::Float(0.22); if py::le(&t1529, &t1530)? { true } else { false } }) }).truthy() {
        v_df = py::sub(&Py::Int(1i64), &py::mul(&Py::Float(0.09), &v_kt.clone())?)?;
    } else {
        if ({ let t1531 = v_kt.clone(); Py::Bool({ let t1532 = Py::Float(0.8); if py::le(&t1531, &t1532)? { true } else { false } }) }).truthy() {
            v_df = py::add(&py::sub(&py::add(&py::sub(&Py::Float(0.9511), &py::mul(&Py::Float(0.1604), &v_kt.clone())?)?, &py::mul(&Py::Float(4.388), &py::pow(&v_kt.clone(), &Py::Int(2i64))?)?)?, &py::mul(&Py::Float(16.638), &py::pow(&v_kt.clone(), &Py::Int(3i64))?)?)?, &py::mul(&Py::Float(12.336), &py::pow(&v_kt.clone(), &Py::Int(4i64))?)?)?;
        } else {
            v_df = Py::Float(0.165);
        }
    }
    v_dhi = py::mul(&v_df.clone(), &v_ghi.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("kt"), v_kt.clone()), (Py::str("diffuse_fraction"), v_df.clone()), (Py::str("dhi"), v_dhi.clone()), (Py::str("dni"), py::truediv(&py::sub(&v_ghi.clone(), &v_dhi.clone())?, &f__cosd(v_zenith.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_extraterrestrial_irradiance(mut v_n: Py, mut v_method: Py, mut v_solar_constant: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__check_day(v_n.clone())?;
    let _ = f__pos(Py::str("the solar constant"), Py::tuple(vec![v_solar_constant.clone()]))?;
    if ({ let t1587 = v_method.clone(); Py::Bool({ let t1588 = Py::str("asce"); if py::eq(&t1587, &t1588) { true } else { false } }) }).truthy() {
        v_f = py::add(&Py::Int(1i64), &py::mul(&Py::Float(0.033), &py::math1("cos", &f__day_angle(v_n.clone(), Py::Int(0i64))?)?)?)?;
    } else {
        v_b = f__day_angle(v_n.clone(), Py::Int(1i64))?;
        v_f = py::add(&py::add(&py::add(&py::add(&Py::Float(1.00011), &py::mul(&Py::Float(0.034221), &py::math1("cos", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.00128), &py::math1("sin", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.000719), &py::math1("cos", &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?, &py::mul(&Py::Float(7.7e-05), &py::math1("sin", &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("G_on"), py::mul(&v_solar_constant.clone(), &v_f.clone())?), (Py::str("distance_factor"), v_f.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fuel_cell_stack(mut v_I: Py, mut v_N: Py, mut v_V_cell: Py, mut v_utilization: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_n_h2: Py = Py::None;
    let _ = f__pos(Py::str("the current and the cell voltage"), Py::tuple(vec![v_I.clone(), v_V_cell.clone()]))?;
    let _ = f__need({ let t1641 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1641.truthy() { t1641 } else { { let t1639 = v_N.clone(); Py::Bool({ let t1640 = Py::Int(1i64); if py::ge(&t1639, &t1640)? { true } else { false } }) } } }, Py::str("the number of cells must be a whole number >= 1"))?;
    let _ = f__need({ let t1642 = v_V_cell.clone(); Py::Bool({ let t1643 = Py::Float(1.25); if py::lt(&t1642, &t1643)? { true } else { false } }) }, Py::str("a hydrogen fuel cell's voltage must stay below the thermoneutral (LHV) voltage of about 1.25 V"))?;
    let _ = f__need({ let t1644 = Py::Int(0i64); Py::Bool({ let t1645 = v_utilization.clone(); if py::lt(&t1644, &t1645)? { { let t1646 = Py::Int(1i64); if py::le(&t1645, &t1646)? { true } else { false } } } else { false } }) }, Py::str("the fuel utilization must lie in (0, 1]"))?;
    v_n_h2 = py::truediv(&py::mul(&v_N.clone(), &v_I.clone())?, &py::mul(&Py::Int(2i64), &c_F_FARADAY()?)?)?;
    v_P = py::mul(&py::mul(&v_N.clone(), &v_I.clone())?, &v_V_cell.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("H2_kg_s"), py::mul(&v_n_h2.clone(), &c_M_H2()?)?), (Py::str("H2_g_h"), py::mul(&py::mul(&v_n_h2.clone(), &c_M_H2()?)?, &Py::Float(3600000.0))?), (Py::str("H2_fed_kg_s"), py::truediv(&py::mul(&v_n_h2.clone(), &c_M_H2()?)?, &v_utilization.clone())?), (Py::str("O2_kg_s"), py::mul(&py::truediv(&v_n_h2.clone(), &Py::Int(2i64))?, &c_M_O2()?)?), (Py::str("H2O_kg_s"), py::mul(&v_n_h2.clone(), &c_M_H2O()?)?), (Py::str("efficiency_LHV"), py::truediv(&v_P.clone(), &py::mul(&v_n_h2.clone(), &c_LHV_H2()?)?)?), (Py::str("efficiency_HHV"), py::truediv(&v_P.clone(), &py::mul(&v_n_h2.clone(), &c_HHV_H2()?)?)?), (Py::str("heat_LHV"), py::sub(&py::mul(&v_n_h2.clone(), &c_LHV_H2()?)?, &v_P.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gmd(mut v_D_ab: Py, mut v_D_bc: Py, mut v_D_ca: Py) -> R {
    let _ = f__pos(Py::str("the phase spacings"), Py::tuple(vec![v_D_ab.clone(), v_D_bc.clone(), v_D_ca.clone()]))?;
    let _ = f__need({ let t1704 = { let t1697 = py::add(&v_D_ab.clone(), &v_D_bc.clone())?; Py::Bool({ let t1698 = v_D_ca.clone(); if py::ge(&t1697, &t1698)? { true } else { false } }) }; if !t1704.truthy() { t1704 } else { { let t1703 = { let t1699 = py::add(&v_D_bc.clone(), &v_D_ca.clone())?; Py::Bool({ let t1700 = v_D_ab.clone(); if py::ge(&t1699, &t1700)? { true } else { false } }) }; if !t1703.truthy() { t1703 } else { { let t1701 = py::add(&v_D_ca.clone(), &v_D_ab.clone())?; Py::Bool({ let t1702 = v_D_bc.clone(); if py::ge(&t1701, &t1702)? { true } else { false } }) } } } } }, Py::str("the three spacings must form a triangle (or a line)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("GMD"), py::pow(&py::mul(&py::mul(&v_D_ab.clone(), &v_D_bc.clone())?, &v_D_ca.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?), (Py::str("ratio_to_mean"), py::truediv(&py::pow(&py::mul(&py::mul(&v_D_ab.clone(), &v_D_bc.clone())?, &v_D_ca.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &py::truediv(&py::add(&py::add(&v_D_ab.clone(), &v_D_bc.clone())?, &v_D_ca.clone())?, &Py::Int(3i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_haurwitz(mut v_zenith: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let _ = f__need({ let t660 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t660.truthy() { t660 } else { { let t657 = Py::Int(0i64); Py::Bool({ let t658 = v_zenith.clone(); if py::le(&t657, &t658)? { { let t659 = Py::Int(180i64); if py::le(&t658, &t659)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must lie between 0 and 180 degrees"))?;
    v_c = f__cosd(v_zenith.clone())?;
    v_g = (if ({ let t661 = v_c.clone(); Py::Bool({ let t662 = Py::Int(0i64); if py::gt(&t661, &t662)? { true } else { false } }) }).truthy() { py::mul(&py::mul(&Py::Float(1098.0), &v_c.clone())?, &py::math1("exp", &py::truediv(&py::neg(&Py::Float(0.059))?, &v_c.clone())?)?)? } else { Py::Float(0.0) });
    return Ok(f__out(Py::dict(vec![(Py::str("ghi"), v_g.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hub_air_density(mut v_p: Py, mut v_h_p: Py, mut v_T: Py, mut v_h_T: Py, mut v_hub: Py, mut v_model: Py) -> R {
    let mut v_T_hub: Py = Py::None;
    let mut v_p_hpa: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__pos(Py::str("the pressure and the temperature"), Py::tuple(vec![v_p.clone(), v_T.clone()]))?;
    let _ = f__nonneg(Py::str("the heights"), Py::tuple(vec![v_h_p.clone(), v_h_T.clone(), v_hub.clone()]))?;
    v_T_hub = py::sub(&v_T.clone(), &py::mul(&Py::Float(0.0065), &py::sub(&v_hub.clone(), &v_h_T.clone())?)?)?;
    let _ = f__need({ let t1760 = v_T_hub.clone(); Py::Bool({ let t1761 = Py::Int(150i64); if py::gt(&t1760, &t1761)? { true } else { false } }) }, Py::str("the temperature at hub height must stay above 150 K"))?;
    v_p_hpa = py::sub(&py::truediv(&v_p.clone(), &Py::Int(100i64))?, &py::truediv(&py::mul(&py::sub(&v_hub.clone(), &v_h_p.clone())?, &Py::Int(1i64))?, &Py::Int(8i64))?)?;
    let _ = f__need({ let t1762 = v_p_hpa.clone(); Py::Bool({ let t1763 = Py::Int(0i64); if py::gt(&t1762, &t1763)? { true } else { false } }) }, Py::str("the pressure at hub height must stay positive"))?;
    v_rho = (if ({ let t1764 = v_model.clone(); Py::Bool({ let t1765 = Py::str("barometric"); if py::eq(&t1764, &t1765) { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&py::mul(&v_p_hpa.clone(), &Py::Float(1.225))?, &Py::Float(288.15))?, &Py::Int(100i64))?, &py::mul(&Py::Int(101330i64), &v_T_hub.clone())?)? } else { py::truediv(&py::mul(&v_p_hpa.clone(), &Py::Int(100i64))?, &py::mul(&Py::Float(287.058), &v_T_hub.clone())?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("T_hub"), v_T_hub.clone()), (Py::str("p_hub"), py::mul(&v_p_hpa.clone(), &Py::Int(100i64))?), (Py::str("rho"), v_rho.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydro_power(mut v_Q: Py, mut v_H: Py, mut v_eta: Py, mut v_rho: Py, mut v_g: Py, mut v_CF: Py, mut v_hours: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Ph: Py = Py::None;
    let _ = f__pos(Py::str("the flow rate Q and the head H"), Py::tuple(vec![v_Q.clone(), v_H.clone()]))?;
    let _ = f__need({ let t1819 = Py::Bool(matches!(v_eta.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1819.truthy() { t1819 } else { { let t1816 = Py::Int(0i64); Py::Bool({ let t1817 = v_eta.clone(); if py::lt(&t1816, &t1817)? { { let t1818 = Py::Int(1i64); if py::le(&t1817, &t1818)? { true } else { false } } } else { false } }) } } }, Py::str("the efficiency must lie in (0, 1] (e.g. 0.9 or 90%)"))?;
    let _ = f__pos(Py::str("the density and g"), Py::tuple(vec![v_rho.clone(), v_g.clone()]))?;
    let _ = f__need({ let t1823 = Py::Bool(matches!(v_CF.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1823.truthy() { t1823 } else { { let t1820 = Py::Int(0i64); Py::Bool({ let t1821 = v_CF.clone(); if py::lt(&t1820, &t1821)? { { let t1822 = Py::Int(1i64); if py::le(&t1821, &t1822)? { true } else { false } } } else { false } }) } } }, Py::str("the capacity factor must lie in (0, 1]"))?;
    let _ = f__need({ let t1827 = Py::Bool(matches!(v_hours.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1827.truthy() { t1827 } else { { let t1824 = Py::Int(0i64); Py::Bool({ let t1825 = v_hours.clone(); if py::lt(&t1824, &t1825)? { { let t1826 = Py::Int(8784i64); if py::le(&t1825, &t1826)? { true } else { false } } } else { false } }) } } }, Py::str("the period must be between 0 and 8784 hours"))?;
    v_Ph = py::mul(&py::mul(&py::mul(&v_rho.clone(), &v_g.clone())?, &v_Q.clone())?, &v_H.clone())?;
    v_P = py::mul(&v_eta.clone(), &v_Ph.clone())?;
    v_E = py::mul(&py::mul(&py::mul(&v_P.clone(), &v_CF.clone())?, &v_hours.clone())?, &Py::Float(3600.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("P_kW"), py::truediv(&v_P.clone(), &Py::Float(1000.0))?), (Py::str("P_MW"), py::truediv(&v_P.clone(), &Py::Float(1000000.0))?), (Py::str("P_hydraulic"), v_Ph.clone()), (Py::str("E"), v_E.clone()), (Py::str("E_MWh"), py::truediv(&v_E.clone(), &Py::Float(3600000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrogen_isentropic_compression(mut v_p1: Py, mut v_T1: Py, mut v_p2: Py) -> R {
    let mut v_T2: Py = Py::None;
    let mut v_h1: Py = Py::None;
    let mut v_h2: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let _ = f__h2_check(v_p1.clone(), v_T1.clone())?;
    let _ = f__need({ let t1879 = v_p2.clone(); Py::Bool({ let t1880 = v_p1.clone(); if py::gt(&t1879, &t1880)? { true } else { false } }) }, Py::str("the final pressure must exceed the initial pressure"))?;
    let _ = f__need({ let t1881 = v_p2.clone(); Py::Bool({ let t1882 = Py::Float(150000000.0); if py::le(&t1881, &t1882)? { true } else { false } }) }, Py::str("the final pressure must not exceed 1500 bar"))?;
    let t1926 = (f__h2_s_h(v_p1.clone(), v_T1.clone())?).items()?;
    if t1926.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t1926[0].clone();
    v_h1 = t1926[1].clone();
    v_T2 = f__illinois({ let v_T1 = v_T1.clone(); let v_h1 = v_h1.clone(); let v_p1 = v_p1.clone(); let v_p2 = v_p2.clone(); let v_s1 = v_s1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::getitem(&f__h2_s_h(v_p2.clone(), v_T.clone())?, &Py::Int(0i64))?, &v_s1.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_T1.clone(), py::mul(&Py::Int(20i64), &v_T1.clone())?, Py::Int(200i64))?;
    let _ = f__need({ let t1927 = v_T2.clone(); Py::Bool({ let t1928 = Py::Int(1000i64); if py::le(&t1927, &t1928)? { true } else { false } }) }, Py::str("the outlet temperature leaves the range of the equation of state (1000 K)"))?;
    v_h2 = py::getitem(&f__h2_s_h(v_p2.clone(), v_T2.clone())?, &Py::Int(1i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("T2"), v_T2.clone()), (Py::str("w"), py::sub(&v_h2.clone(), &v_h1.clone())?), (Py::str("w_kWh_per_kg"), py::truediv(&py::sub(&v_h2.clone(), &v_h1.clone())?, &Py::Float(3600000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrogen_isothermal_compression(mut v_p1: Py, mut v_p2: Py, mut v_T: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_ar: Py = Py::None;
    let mut v_ard: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rm: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wid: Py = Py::None;
    let _ = f__h2_check(v_p1.clone(), v_T.clone())?;
    let _ = f__h2_check(v_p2.clone(), v_T.clone())?;
    let _ = f__need({ let t1980 = v_p2.clone(); Py::Bool({ let t1981 = v_p1.clone(); if py::gt(&t1980, &t1981)? { true } else { false } }) }, Py::str("the final pressure must exceed the initial pressure"))?;
    v_t = py::truediv(&c__H2_TC()?, &v_T.clone())?;
    v_g = Py::list(vec![]);
    for t1982 in py::iter(&(Py::tuple(vec![v_p1.clone(), v_p2.clone()])))? {
        v_p = t1982;
        v_rm = f__h2_rho(v_p.clone(), v_T.clone())?;
        v_d = py::truediv(&v_rm.clone(), &c__H2_RHOC()?)?;
        let t2021 = (f__h2_ar(v_d.clone(), v_t.clone())?).items()?;
        if t2021.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_ar = t2021[0].clone();
        v_ard = t2021[1].clone();
        v__ = t2021[2].clone();
        let _ = py::method(&v_g.clone(), "append", &[py::mul(&py::mul(&py::add(&py::add(&py::math1("log", &v_d.clone())?, &v_ar.clone())?, &py::mul(&v_d.clone(), &v_ard.clone())?)?, &c__H2_R()?)?, &v_T.clone())?])?;
    }
    v_w = py::truediv(&py::sub(&py::getitem(&v_g.clone(), &Py::Int(1i64))?, &py::getitem(&v_g.clone(), &Py::Int(0i64))?)?, &c_M_H2()?)?;
    v_wid = py::truediv(&py::mul(&py::mul(&c__H2_R()?, &v_T.clone())?, &py::math1("log", &py::truediv(&v_p2.clone(), &v_p1.clone())?)?)?, &c_M_H2()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("w"), v_w.clone()), (Py::str("w_kWh_per_kg"), py::truediv(&v_w.clone(), &Py::Float(3600000.0))?), (Py::str("w_ideal"), v_wid.clone()), (Py::str("real_over_ideal"), py::truediv(&v_w.clone(), &v_wid.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrogen_tank(mut v_V: Py, mut v_p: Py, mut v_T: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_rho_id: Py = Py::None;
    let mut v_rho_m: Py = Py::None;
    let _ = f__pos(Py::str("the tank volume"), Py::tuple(vec![v_V.clone()]))?;
    let _ = f__h2_check(v_p.clone(), v_T.clone())?;
    v_rho_m = f__h2_rho(v_p.clone(), v_T.clone())?;
    v_rho = py::mul(&v_rho_m.clone(), &c_M_H2()?)?;
    v_rho_id = py::mul(&py::truediv(&v_p.clone(), &py::mul(&c__H2_R()?, &v_T.clone())?)?, &c_M_H2()?)?;
    v_m = py::mul(&v_rho.clone(), &v_V.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("Z"), py::truediv(&v_p.clone(), &py::mul(&py::mul(&v_rho_m.clone(), &c__H2_R()?)?, &v_T.clone())?)?), (Py::str("mass"), v_m.clone()), (Py::str("rho_ideal"), v_rho_id.clone()), (Py::str("mass_ideal"), py::mul(&v_rho_id.clone(), &v_V.clone())?), (Py::str("ideal_over_real"), py::truediv(&v_rho_id.clone(), &v_rho.clone())?), (Py::str("energy_LHV_kWh"), py::truediv(&py::mul(&py::truediv(&v_m.clone(), &c_M_H2()?)?, &c_LHV_H2()?)?, &Py::Float(3600000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_iam_ashrae(mut v_aoi: Py, mut v_b0: Py) -> R {
    let _ = f__need({ let t2164 = Py::Bool(matches!(v_aoi.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2164.truthy() { t2164 } else { { let t2161 = Py::Int(0i64); Py::Bool({ let t2162 = v_aoi.clone(); if py::le(&t2161, &t2162)? { { let t2163 = Py::Int(180i64); if py::le(&t2162, &t2163)? { true } else { false } } } else { false } }) } } }, Py::str("the angle of incidence must lie between 0 and 180 degrees"))?;
    let _ = f__need({ let t2165 = Py::Int(0i64); Py::Bool({ let t2166 = v_b0.clone(); if py::lt(&t2165, &t2166)? { { let t2167 = Py::Int(1i64); if py::lt(&t2166, &t2167)? { true } else { false } } } else { false } }) }, Py::str("b0 must lie between 0 and 1"))?;
    if ({ let t2168 = v_aoi.clone(); Py::Bool({ let t2169 = Py::Int(90i64); if py::ge(&t2168, &t2169)? { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("iam"), Py::Float(0.0))]))?);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("iam"), py::minmax(&[Py::Float(0.0), py::sub(&Py::Int(1i64), &py::mul(&v_b0.clone(), &py::sub(&py::truediv(&Py::Int(1i64), &py::math1("cos", &py::math1("radians", &v_aoi.clone())?)?)?, &Py::Int(1i64))?)?)?], true)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_iam_physical(mut v_aoi: Py, mut v_n: Py, mut v_K: Py, mut v_L: Py) -> R {
    let mut v_c1: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_rp: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let _ = f__need({ let t2218 = Py::Bool(matches!(v_aoi.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2218.truthy() { t2218 } else { { let t2215 = Py::Int(0i64); Py::Bool({ let t2216 = v_aoi.clone(); if py::le(&t2215, &t2216)? { { let t2217 = Py::Int(90i64); if py::lt(&t2216, &t2217)? { true } else { false } } } else { false } }) } } }, Py::str("the angle of incidence must lie between 0 and 90 degrees"))?;
    let _ = f__need({ let t2219 = Py::Float(1.0); Py::Bool({ let t2220 = v_n.clone(); if py::lt(&t2219, &t2220)? { { let t2221 = Py::Float(3.0); if py::lt(&t2220, &t2221)? { true } else { false } } } else { false } }) }, Py::str("the refractive index must lie between 1 and 3"))?;
    let _ = f__nonneg(Py::str("the extinction coefficient K and the thickness L"), Py::tuple(vec![v_K.clone(), v_L.clone()]))?;
    v_c1 = py::minmax(&[Py::Float(0.0), f__cosd(v_aoi.clone())?], true)?;
    v_s1 = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::pow(&v_c1.clone(), &Py::Int(2i64))?)?)?;
    v_s2 = py::truediv(&v_s1.clone(), &v_n.clone())?;
    v_c2 = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::pow(&v_s2.clone(), &Py::Int(2i64))?)?)?;
    v_rs = py::pow(&py::truediv(&py::sub(&v_c1.clone(), &py::mul(&v_n.clone(), &v_c2.clone())?)?, &py::add(&v_c1.clone(), &py::mul(&v_n.clone(), &v_c2.clone())?)?)?, &Py::Int(2i64))?;
    v_rp = py::pow(&py::truediv(&py::sub(&v_c2.clone(), &py::mul(&v_n.clone(), &v_c1.clone())?)?, &py::add(&v_c2.clone(), &py::mul(&v_n.clone(), &v_c1.clone())?)?)?, &Py::Int(2i64))?;
    v_r0 = py::pow(&py::truediv(&py::sub(&Py::Int(1i64), &v_n.clone())?, &py::add(&Py::Int(1i64), &v_n.clone())?)?, &Py::Int(2i64))?;
    v_ts = py::mul(&py::sub(&Py::Int(1i64), &v_rs.clone())?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_K.clone())?, &v_L.clone())?, &v_c2.clone())?)?)?;
    v_tp = py::mul(&py::sub(&Py::Int(1i64), &v_rp.clone())?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_K.clone())?, &v_L.clone())?, &v_c2.clone())?)?)?;
    v_t0 = py::mul(&py::sub(&Py::Int(1i64), &v_r0.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_K.clone())?, &v_L.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("iam"), py::truediv(&py::truediv(&py::add(&v_ts.clone(), &v_tp.clone())?, &Py::Int(2i64))?, &v_t0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_induction_max_torque(mut v_VL: Py, mut v_f: Py, mut v_poles: Py, mut v_R1: Py, mut v_X1: Py, mut v_R2: Py, mut v_X2: Py, mut v_Xm: Py, mut v_connection: Py) -> R {
    let mut v_Tmax: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_Vth: Py = Py::None;
    let mut v_Z1: Py = Py::None;
    let mut v_Zm: Py = Py::None;
    let mut v_Zth: Py = Py::None;
    let mut v_root: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let _ = f__check_im(v_VL.clone(), v_f.clone(), v_poles.clone(), v_R1.clone(), v_X1.clone(), v_R2.clone(), v_X2.clone(), v_Xm.clone())?;
    v_V = f__im_phase(v_VL.clone(), v_connection.clone())?;
    v_ws = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_poles.clone())?;
    let t2290 = (Py::tuple(vec![py::complex(&[v_R1.clone(), v_X1.clone()])?, py::complex(&[Py::Int(0i64), v_Xm.clone()])?])).items()?;
    if t2290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z1 = t2290[0].clone();
    v_Zm = t2290[1].clone();
    v_Vth = py::abs(&py::truediv(&py::mul(&v_V.clone(), &v_Zm.clone())?, &py::add(&v_Z1.clone(), &v_Zm.clone())?)?)?;
    v_Zth = py::truediv(&py::mul(&v_Z1.clone(), &v_Zm.clone())?, &py::add(&v_Z1.clone(), &v_Zm.clone())?)?;
    v_root = py::math1("sqrt", &py::add(&py::pow(&py::real(&v_Zth.clone())?, &Py::Int(2i64))?, &py::pow(&py::add(&py::imag(&v_Zth.clone())?, &v_X2.clone())?, &Py::Int(2i64))?)?)?;
    v_s = py::truediv(&v_R2.clone(), &v_root.clone())?;
    v_Tmax = py::truediv(&py::mul(&py::mul(&Py::Int(3i64), &v_Vth.clone())?, &v_Vth.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_ws.clone())?, &py::add(&py::real(&v_Zth.clone())?, &v_root.clone())?)?)?;
    v_st = f_induction_motor(v_VL.clone(), v_f.clone(), v_poles.clone(), v_R1.clone(), v_X1.clone(), v_R2.clone(), v_X2.clone(), v_Xm.clone(), Py::Float(1.0), v_connection.clone(), Py::Float(0.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("s_maxT"), v_s.clone()), (Py::str("T_max"), v_Tmax.clone()), (Py::str("T_start"), py::getitem(&v_st.clone(), &Py::str("T"))?), (Py::str("I_start"), py::getitem(&v_st.clone(), &Py::str("I_line"))?), (Py::str("V_th"), v_Vth.clone()), (Py::str("R_th"), py::real(&v_Zth.clone())?), (Py::str("X_th"), py::imag(&v_Zth.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_induction_motor(mut v_VL: Py, mut v_f: Py, mut v_poles: Py, mut v_R1: Py, mut v_X1: Py, mut v_R2: Py, mut v_X2: Py, mut v_Xm: Py, mut v_slip: Py, mut v_connection: Py, mut v_P_rot: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_I1: Py = Py::None;
    let mut v_I2: Py = Py::None;
    let mut v_IL: Py = Py::None;
    let mut v_Pag: Py = Py::None;
    let mut v_Pconv: Py = Py::None;
    let mut v_Pin: Py = Py::None;
    let mut v_Pout: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_Z1: Py = Py::None;
    let mut v_Z2: Py = Py::None;
    let mut v_Zin: Py = Py::None;
    let mut v_Zm: Py = Py::None;
    let mut v_ns: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let _ = f__check_im(v_VL.clone(), v_f.clone(), v_poles.clone(), v_R1.clone(), v_X1.clone(), v_R2.clone(), v_X2.clone(), v_Xm.clone())?;
    let _ = f__need({ let t2358 = Py::Int(0i64); Py::Bool({ let t2359 = v_slip.clone(); if py::lt(&t2358, &t2359)? { { let t2360 = Py::Int(1i64); if py::le(&t2359, &t2360)? { true } else { false } } } else { false } }) }, Py::str("the slip must lie in (0, 1] (motoring)"))?;
    let _ = f__nonneg(Py::str("the rotational loss"), Py::tuple(vec![v_P_rot.clone()]))?;
    v_ns = py::truediv(&py::mul(&Py::Int(120i64), &v_f.clone())?, &v_poles.clone())?;
    v_ws = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_poles.clone())?;
    v_V = f__im_phase(v_VL.clone(), v_connection.clone())?;
    let t2363 = (Py::tuple(vec![py::complex(&[v_R1.clone(), v_X1.clone()])?, py::complex(&[Py::Int(0i64), v_Xm.clone()])?, py::complex(&[py::truediv(&v_R2.clone(), &v_slip.clone())?, v_X2.clone()])?])).items()?;
    if t2363.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Z1 = t2363[0].clone();
    v_Zm = t2363[1].clone();
    v_Z2 = t2363[2].clone();
    v_Zin = py::add(&v_Z1.clone(), &py::truediv(&py::mul(&v_Zm.clone(), &v_Z2.clone())?, &py::add(&v_Zm.clone(), &v_Z2.clone())?)?)?;
    v_I1 = py::truediv(&v_V.clone(), &v_Zin.clone())?;
    v_E = py::sub(&v_V.clone(), &py::mul(&v_I1.clone(), &v_Z1.clone())?)?;
    v_I2 = py::truediv(&v_E.clone(), &v_Z2.clone())?;
    v_Pin = py::mul(&Py::Int(3i64), &py::real(&py::mul(&v_V.clone(), &py::method(&v_I1.clone(), "conjugate", &[])?)?)?)?;
    v_Pag = py::truediv(&py::mul(&py::mul(&Py::Int(3i64), &py::pow(&py::abs(&v_I2.clone())?, &Py::Int(2i64))?)?, &v_R2.clone())?, &v_slip.clone())?;
    v_Pconv = py::mul(&py::sub(&Py::Int(1i64), &v_slip.clone())?, &v_Pag.clone())?;
    v_Pout = py::sub(&v_Pconv.clone(), &v_P_rot.clone())?;
    v_IL = (if ({ let t2364 = v_connection.clone(); Py::Bool({ let t2365 = Py::str("star"); if py::eq(&t2364, &t2365) { true } else { false } }) }).truthy() { py::abs(&v_I1.clone())? } else { py::mul(&c_SQ3()?, &py::abs(&v_I1.clone())?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("I_line"), v_IL.clone()), (Py::str("I2"), py::abs(&v_I2.clone())?), (Py::str("pf"), py::math1("cos", &py::neg(&py::cmath("phase", &[v_I1.clone()])?)?)?), (Py::str("P_in"), v_Pin.clone()), (Py::str("P_ag"), v_Pag.clone()), (Py::str("P_conv"), v_Pconv.clone()), (Py::str("T"), py::truediv(&v_Pag.clone(), &v_ws.clone())?), (Py::str("P_out"), v_Pout.clone()), (Py::str("efficiency_pct"), py::truediv(&py::mul(&Py::Int(100i64), &v_Pout.clone())?, &v_Pin.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_induction_power_flow(mut v_f: Py, mut v_poles: Py, mut v_P_in: Py, mut v_P_scl: Py, mut v_s: Py, mut v_n_r: Py, mut v_P_core: Py, mut v_P_fw: Py, mut v_E2: Py) -> R {
    let mut v_Pag: Py = Py::None;
    let mut v_Pconv: Py = Py::None;
    let mut v_Pout: Py = Py::None;
    let mut v_Prcl: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_wm: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let _ = f__pos(Py::str("the supply frequency and the input power"), Py::tuple(vec![v_f.clone(), v_P_in.clone()]))?;
    let _ = f__need({ let t2421 = Py::Bool(matches!(v_poles.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2421.truthy() { t2421 } else { { let t2420 = { let t2416 = v_poles.clone(); Py::Bool({ let t2417 = Py::Int(2i64); if py::ge(&t2416, &t2417)? { true } else { false } }) }; if !t2420.truthy() { t2420 } else { { let t2418 = py::modulo(&v_poles.clone(), &Py::Int(2i64))?; Py::Bool({ let t2419 = Py::Int(0i64); if py::eq(&t2418, &t2419) { true } else { false } }) } } } } }, Py::str("the number of poles must be an even whole number (2, 4, 6, ...)"))?;
    let _ = f__nonneg(Py::str("the stator copper, core and friction losses"), Py::tuple(vec![v_P_scl.clone(), v_P_core.clone(), v_P_fw.clone()]))?;
    let _ = f__need({ let t2427 = { let t2429 = v_s.clone(); Py::Bool({ let t2430 = Py::None; if py::is(&t2429, &t2430) { true } else { false } }) }; Py::Bool({ let t2428 = { let t2431 = v_n_r.clone(); Py::Bool({ let t2432 = Py::None; if py::is(&t2431, &t2432) { true } else { false } }) }; if py::ne(&t2427, &t2428) { true } else { false } }) }, Py::str("give exactly one of the slip s and the rotor speed n_r"))?;
    v_ws = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_poles.clone())?;
    if ({ let t2433 = v_s.clone(); Py::Bool({ let t2434 = Py::None; if py::is(&t2433, &t2434) { true } else { false } }) }).truthy() {
        let _ = f__fin(Py::str("the rotor speed"), v_n_r.clone())?;
        v_s = py::truediv(&py::sub(&v_ws.clone(), &v_n_r.clone())?, &v_ws.clone())?;
    }
    let _ = f__need({ let t2440 = Py::Bool(matches!(v_s.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2440.truthy() { t2440 } else { { let t2437 = Py::Int(0i64); Py::Bool({ let t2438 = v_s.clone(); if py::lt(&t2437, &t2438)? { { let t2439 = Py::Int(1i64); if py::lt(&t2438, &t2439)? { true } else { false } } } else { false } }) } } }, Py::str("the slip must lie strictly between 0 and 1 (motoring below synchronous speed)"))?;
    v_Pag = py::sub(&py::sub(&v_P_in.clone(), &v_P_scl.clone())?, &v_P_core.clone())?;
    let _ = f__need({ let t2441 = v_Pag.clone(); Py::Bool({ let t2442 = Py::Int(0i64); if py::gt(&t2441, &t2442)? { true } else { false } }) }, Py::str("the stator losses must be less than the input power"))?;
    v_Prcl = py::mul(&v_s.clone(), &v_Pag.clone())?;
    v_Pconv = py::mul(&py::sub(&Py::Int(1i64), &v_s.clone())?, &v_Pag.clone())?;
    v_Pout = py::sub(&v_Pconv.clone(), &v_P_fw.clone())?;
    let _ = f__need({ let t2443 = v_Pout.clone(); Py::Bool({ let t2444 = Py::Int(0i64); if py::gt(&t2443, &t2444)? { true } else { false } }) }, Py::str("the friction and windage loss must be less than the converted power"))?;
    v_wm = py::mul(&py::sub(&Py::Int(1i64), &v_s.clone())?, &v_ws.clone())?;
    v_out = Py::dict(vec![(Py::str("n_sync_rpm"), py::truediv(&py::mul(&Py::Int(120i64), &v_f.clone())?, &v_poles.clone())?), (Py::str("n_rotor_rpm"), py::truediv(&py::mul(&py::mul(&py::sub(&Py::Int(1i64), &v_s.clone())?, &Py::Int(120i64))?, &v_f.clone())?, &v_poles.clone())?), (Py::str("slip"), v_s.clone()), (Py::str("f_rotor"), py::mul(&v_s.clone(), &v_f.clone())?), (Py::str("P_ag"), v_Pag.clone()), (Py::str("P_rcl"), v_Prcl.clone()), (Py::str("P_conv"), v_Pconv.clone()), (Py::str("P_out"), v_Pout.clone()), (Py::str("T_ind"), py::truediv(&v_Pag.clone(), &v_ws.clone())?), (Py::str("T_shaft"), py::truediv(&v_Pout.clone(), &v_wm.clone())?), (Py::str("efficiency_pct"), py::truediv(&py::mul(&Py::Int(100i64), &v_Pout.clone())?, &v_P_in.clone())?)]);
    if ({ let t2445 = v_E2.clone(); Py::Bool({ let t2446 = Py::None; if !py::is(&t2445, &t2446) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the standstill rotor emf"), Py::tuple(vec![v_E2.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("E_rotor"), py::mul(&v_s.clone(), &v_E2.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_parameters(mut v_D_ab: Py, mut v_D_bc: Py, mut v_D_ca: Py, mut v_r: Py, mut v_f: Py, mut v_gmr: Py, mut v_n_bundle: Py, mut v_spacing: Py, mut v_V: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gmd: Py = Py::None;
    let mut v_gmr_b: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r_b: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("the phase spacings, the conductor radius and the frequency"), Py::tuple(vec![v_D_ab.clone(), v_D_bc.clone(), v_D_ca.clone(), v_r.clone(), v_f.clone()]))?;
    let _ = f__need({ let t2501 = Py::Bool(matches!(v_n_bundle.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2501.truthy() { t2501 } else { { let t2498 = Py::Int(1i64); Py::Bool({ let t2499 = v_n_bundle.clone(); if py::le(&t2498, &t2499)? { { let t2500 = Py::Int(4i64); if py::le(&t2499, &t2500)? { true } else { false } } } else { false } }) } } }, Py::str("the bundle has 1 to 4 subconductors"))?;
    v_g = (if ({ let t2502 = v_gmr.clone(); Py::Bool({ let t2503 = Py::None; if py::is(&t2502, &t2503) { true } else { false } }) }).truthy() { py::mul(&v_r.clone(), &py::math1("exp", &py::neg(&Py::Float(0.25))?)?)? } else { v_gmr.clone() });
    let _ = f__pos(Py::str("the geometric mean radius"), Py::tuple(vec![v_g.clone()]))?;
    let _ = f__need({ let t2504 = v_g.clone(); Py::Bool({ let t2505 = v_r.clone(); if py::le(&t2504, &t2505)? { true } else { false } }) }, Py::str("the geometric mean radius cannot exceed the conductor radius"))?;
    if ({ let t2506 = v_n_bundle.clone(); Py::Bool({ let t2507 = Py::Int(1i64); if py::gt(&t2506, &t2507)? { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2512 = { let t2508 = v_spacing.clone(); Py::Bool({ let t2509 = Py::None; if !py::is(&t2508, &t2509) { true } else { false } }) }; if !t2512.truthy() { t2512 } else { { let t2510 = v_spacing.clone(); Py::Bool({ let t2511 = py::mul(&Py::Int(2i64), &v_r.clone())?; if py::gt(&t2510, &t2511)? { true } else { false } }) } } }, Py::str("a bundle needs a subconductor spacing larger than the conductor diameter"))?;
    }
    v_d = { let t2513 = v_spacing.clone(); if t2513.truthy() { t2513 } else { Py::Float(0.0) } };
    v_gmd = py::pow(&py::mul(&py::mul(&v_D_ab.clone(), &v_D_bc.clone())?, &v_D_ca.clone())?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    let t2520 = (Py::tuple(vec![f__bundle_equiv(v_g.clone(), v_n_bundle.clone(), v_d.clone())?, f__bundle_equiv(v_r.clone(), v_n_bundle.clone(), v_d.clone())?])).items()?;
    if t2520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gmr_b = t2520[0].clone();
    v_r_b = t2520[1].clone();
    let _ = f__need({ let t2521 = v_gmd.clone(); Py::Bool({ let t2522 = py::mul(&Py::Int(10i64), &v_r_b.clone())?; if py::gt(&t2521, &t2522)? { true } else { false } }) }, Py::str("the phase spacing must be much larger than the conductor (bundle) size"))?;
    v_L = py::mul(&py::truediv(&c_MU0()?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, &py::math1("log", &py::truediv(&v_gmd.clone(), &v_gmr_b.clone())?)?)?;
    v_C = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &c_EPS0()?)?, &py::math1("log", &py::truediv(&v_gmd.clone(), &v_r_b.clone())?)?)?;
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("GMR"), v_gmr_b.clone()), (Py::str("L_mH_per_km"), py::mul(&v_L.clone(), &Py::Float(1000000.0))?), (Py::str("C_nF_per_km"), py::mul(&v_C.clone(), &Py::Float(1000000000000.0))?), (Py::str("X_ohm_per_km"), py::mul(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Float(1000.0))?), (Py::str("B_uS_per_km"), py::mul(&py::mul(&v_w.clone(), &v_C.clone())?, &Py::Float(1000000000.0))?), (Py::str("Zc"), py::math1("sqrt", &py::truediv(&v_L.clone(), &v_C.clone())?)?)])?;
    if ({ let t2523 = v_V.clone(); Py::Bool({ let t2524 = Py::None; if !py::is(&t2523, &t2524) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the line voltage"), Py::tuple(vec![v_V.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("SIL_MW"), py::truediv(&py::truediv(&py::mul(&v_V.clone(), &v_V.clone())?, &py::math1("sqrt", &py::truediv(&v_L.clone(), &v_C.clone())?)?)?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_long_line(mut v_Vr: Py, mut v_P: Py, mut v_pf: Py, mut v_R: Py, mut v_X: Py, mut v_B: Py, mut v_pf_type: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Bc: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_Zc: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gl: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zc: Py = Py::None;
    let _ = f__pos(Py::str("the receiving-end voltage and the load power"), Py::tuple(vec![v_Vr.clone(), v_P.clone()]))?;
    let _ = f__nonneg(Py::str("the line resistance and reactance"), Py::tuple(vec![v_R.clone(), v_X.clone()]))?;
    let _ = f__pos(Py::str("the shunt susceptance"), Py::tuple(vec![v_B.clone()]))?;
    let _ = f__need({ let t2652 = py::add(&v_R.clone(), &v_X.clone())?; Py::Bool({ let t2653 = Py::Int(0i64); if py::gt(&t2652, &t2653)? { true } else { false } }) }, Py::str("the series impedance must not be zero"))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t2654 = (Py::tuple(vec![ax_engine::pymp::call("mpc", &[v_R.clone(), v_X.clone()], &[])?, ax_engine::pymp::call("mpc", &[Py::Int(0i64), v_B.clone()], &[])?])).items()?;
        if t2654.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_z = t2654[0].clone();
        v_y = t2654[1].clone();
        v_g = ax_engine::pymp::call("sqrt", &[py::mul(&v_z.clone(), &v_y.clone())?], &[])?;
        if ({ let t2655 = py::real(&v_g.clone())?; Py::Bool({ let t2656 = Py::Int(0i64); if py::lt(&t2655, &t2656)? { true } else { false } }) }).truthy() {
            v_g = py::neg(&v_g.clone())?;
        }
        v_zc = ax_engine::pymp::call("sqrt", &[py::truediv(&v_z.clone(), &v_y.clone())?], &[])?;
        if ({ let t2657 = py::real(&v_zc.clone())?; Py::Bool({ let t2658 = Py::Int(0i64); if py::lt(&t2657, &t2658)? { true } else { false } }) }).truthy() {
            v_zc = py::neg(&v_zc.clone())?;
        }
        let t2659 = (Py::tuple(vec![py::complex(&[v_g.clone()])?, py::complex(&[v_zc.clone()])?])).items()?;
        if t2659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_gl = t2659[0].clone();
        v_Zc = t2659[1].clone();
        let t2660 = (Py::tuple(vec![py::complex(&[ax_engine::pymp::call("cosh", &[v_g.clone()], &[])?])?, py::complex(&[py::mul(&v_zc.clone(), &ax_engine::pymp::call("sinh", &[v_g.clone()], &[])?)?])?, py::complex(&[py::truediv(&ax_engine::pymp::call("sinh", &[v_g.clone()], &[])?, &v_zc.clone())?])?])).items()?;
        if t2660.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_A = t2660[0].clone();
        v_Bc = t2660[1].clone();
        v_C = t2660[2].clone();
    }
    v_out = f__line_ends(v_A.clone(), v_Bc.clone(), v_C.clone(), v_A.clone(), v_Vr.clone(), v_P.clone(), v_pf.clone(), v_pf_type.clone(), v_R.clone())?;
    let _ = py::method(&v_out.clone(), "update", &[f__out(Py::dict(vec![(Py::str("Zc_mag"), py::abs(&v_Zc.clone())?), (Py::str("Zc_angle_deg"), py::math1("degrees", &py::cmath("phase", &[v_Zc.clone()])?)?), (Py::str("alpha_l"), py::real(&v_gl.clone())?), (Py::str("beta_l"), py::imag(&v_gl.clone())?)]))?])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_machine_speed(mut v_f: Py, mut v_poles: Py, mut v_s: Py, mut v_n_r: Py) -> R {
    let mut v_nr: Py = Py::None;
    let mut v_ns: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let _ = f__pos(Py::str("the frequency"), Py::tuple(vec![v_f.clone()]))?;
    let _ = f__need({ let t5246 = Py::Bool(matches!(v_poles.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5246.truthy() { t5246 } else { { let t5245 = { let t5238 = v_poles.clone(); Py::Bool({ let t5239 = Py::Int(2i64); if py::ge(&t5238, &t5239)? { true } else { false } }) }; if !t5245.truthy() { t5245 } else { { let t5244 = { let t5240 = py::modulo(&v_poles.clone(), &Py::Int(2i64))?; Py::Bool({ let t5241 = Py::Int(0i64); if py::eq(&t5240, &t5241) { true } else { false } }) }; if !t5244.truthy() { t5244 } else { { let t5242 = v_poles.clone(); Py::Bool({ let t5243 = Py::Int(200i64); if py::le(&t5242, &t5243)? { true } else { false } }) } } } } } } }, Py::str("the number of poles must be an even whole number from 2 to 200"))?;
    let _ = f__need({ let t5251 = { let t5247 = v_s.clone(); Py::Bool({ let t5248 = Py::None; if py::is(&t5247, &t5248) { true } else { false } }) }; if t5251.truthy() { t5251 } else { { let t5249 = v_n_r.clone(); Py::Bool({ let t5250 = Py::None; if py::is(&t5249, &t5250) { true } else { false } }) } } }, Py::str("give the slip or the rotor speed, not both"))?;
    v_ns = py::truediv(&py::mul(&Py::Float(120.0), &v_f.clone())?, &v_poles.clone())?;
    v_ws = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_poles.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("n_sync_rpm"), v_ns.clone()), (Py::str("omega_sync"), v_ws.clone())])?;
    if ({ let t5252 = v_s.clone(); Py::Bool({ let t5253 = Py::None; if !py::is(&t5252, &t5253) { true } else { false } }) }).truthy() {
        let _ = f__fin(Py::str("the slip"), v_s.clone())?;
        let _ = f__need({ let t5256 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t5257 = v_s.clone(); if py::lt(&t5256, &t5257)? { { let t5258 = Py::Int(2i64); if py::lt(&t5257, &t5258)? { true } else { false } } } else { false } }) }, Py::str("the slip must lie between -1 and 2"))?;
        let _ = { let t5259 = v_out.clone(); py::setitem(&t5259, &Py::str("n_rotor_rpm"), py::mul(&v_ns.clone(), &py::sub(&Py::Int(1i64), &v_s.clone())?)?)?; py::setitem(&t5259, &Py::str("slip"), v_s.clone())?; py::setitem(&t5259, &Py::str("slip_frequency"), py::mul(&v_s.clone(), &v_f.clone())?)?; Py::None };
    }
    if ({ let t5260 = v_n_r.clone(); Py::Bool({ let t5261 = Py::None; if !py::is(&t5260, &t5261) { true } else { false } }) }).truthy() {
        let _ = f__fin(Py::str("the rotor speed"), v_n_r.clone())?;
        v_nr = py::truediv(&py::mul(&v_n_r.clone(), &Py::Int(60i64))?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
        v_sl = py::truediv(&py::sub(&v_ns.clone(), &v_nr.clone())?, &v_ns.clone())?;
        let _ = { let t5262 = v_out.clone(); py::setitem(&t5262, &Py::str("n_rotor_rpm"), v_nr.clone())?; py::setitem(&t5262, &Py::str("slip"), v_sl.clone())?; py::setitem(&t5262, &Py::str("slip_frequency"), py::mul(&v_sl.clone(), &v_f.clone())?)?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_medium_line(mut v_Vr: Py, mut v_P: Py, mut v_pf: Py, mut v_R: Py, mut v_X: Py, mut v_B: Py, mut v_pf_type: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let _ = f__pos(Py::str("the receiving-end voltage and the load power"), Py::tuple(vec![v_Vr.clone(), v_P.clone()]))?;
    let _ = f__nonneg(Py::str("the line resistance and reactance"), Py::tuple(vec![v_R.clone(), v_X.clone()]))?;
    let _ = f__pos(Py::str("the shunt susceptance"), Py::tuple(vec![v_B.clone()]))?;
    let _ = f__need({ let t2725 = py::add(&v_R.clone(), &v_X.clone())?; Py::Bool({ let t2726 = Py::Int(0i64); if py::gt(&t2725, &t2726)? { true } else { false } }) }, Py::str("the series impedance must not be zero"))?;
    let t2727 = (Py::tuple(vec![py::complex(&[v_R.clone(), v_X.clone()])?, py::complex(&[Py::Float(0.0), v_B.clone()])?])).items()?;
    if t2727.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z = t2727[0].clone();
    v_Y = t2727[1].clone();
    v_A = py::add(&Py::Int(1i64), &py::truediv(&py::mul(&v_Z.clone(), &v_Y.clone())?, &Py::Int(2i64))?)?;
    return Ok(f__line_ends(v_A.clone(), v_Z.clone(), py::mul(&v_Y.clone(), &py::add(&Py::Int(1i64), &py::truediv(&py::mul(&v_Z.clone(), &v_Y.clone())?, &Py::Int(4i64))?)?)?, v_A.clone(), v_Vr.clone(), v_P.clone(), v_pf.clone(), v_pf_type.clone(), v_R.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newton_load_flow(mut v_lines: Py, mut v_loads: Py, mut v_gens: Py, mut v_V1: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_Pd: Py = Py::None;
    let mut v_Pg: Py = Py::None;
    let mut v_Psp: Py = Py::None;
    let mut v_Qd: Py = Py::None;
    let mut v_Qsp: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_Vc: Py = Py::None;
    let mut v_Vset: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_Ykm: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_br: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_calc: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_it: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_loss: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mis: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_nsl: Py = Py::None;
    let mut v_pq: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_pvb: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__need({ let t2782 = Py::Float(0.8); Py::Bool({ let t2783 = v_V1.clone(); if py::le(&t2782, &t2783)? { { let t2784 = Py::Float(1.2); if py::le(&t2783, &t2784)? { true } else { false } } } else { false } }) }, Py::str("the slack voltage must lie between 0.8 and 1.2 pu"))?;
    v_br = Py::list(vec![]);
    for t2785 in py::iter(&(v_lines.clone()))? {
        v_row = t2785;
        let _ = f__need({ let t2786 = py::len(&v_row.clone())?; Py::Bool({ let t2787 = Py::Int(5i64); if py::eq(&t2786, &t2787) { true } else { false } }) }, Py::str("each line row is: from bus, to bus, r, x, total shunt b (per unit)"))?;
        let t2788 = (Py::tuple(vec![py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, py::int(&py::getitem(&v_row.clone(), &Py::Int(1i64))?)?])).items()?;
        if t2788.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2788[0].clone();
        v_j = t2788[1].clone();
        let _ = f__need({ let t2802 = { let t2789 = py::getitem(&v_row.clone(), &Py::Int(0i64))?; Py::Bool({ let t2790 = v_i.clone(); if py::eq(&t2789, &t2790) { true } else { false } }) }; if !t2802.truthy() { t2802 } else { { let t2801 = { let t2791 = py::getitem(&v_row.clone(), &Py::Int(1i64))?; Py::Bool({ let t2792 = v_j.clone(); if py::eq(&t2791, &t2792) { true } else { false } }) }; if !t2801.truthy() { t2801 } else { { let t2800 = { let t2793 = v_i.clone(); Py::Bool({ let t2794 = v_j.clone(); if py::ne(&t2793, &t2794) { true } else { false } }) }; if !t2800.truthy() { t2800 } else { { let t2799 = { let t2795 = v_i.clone(); Py::Bool({ let t2796 = Py::Int(1i64); if py::ge(&t2795, &t2796)? { true } else { false } }) }; if !t2799.truthy() { t2799 } else { { let t2797 = v_j.clone(); Py::Bool({ let t2798 = Py::Int(1i64); if py::ge(&t2797, &t2798)? { true } else { false } }) } } } } } } } } }, Py::str("line buses must be different whole numbers from 1"))?;
        let t2803 = (Py::tuple(vec![py::getitem(&v_row.clone(), &Py::Int(2i64))?, py::getitem(&v_row.clone(), &Py::Int(3i64))?, py::getitem(&v_row.clone(), &Py::Int(4i64))?])).items()?;
        if t2803.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_r = t2803[0].clone();
        v_x = t2803[1].clone();
        v_b = t2803[2].clone();
        let _ = f__need({ let t2811 = { let t2804 = v_r.clone(); Py::Bool({ let t2805 = Py::Int(0i64); if py::ge(&t2804, &t2805)? { true } else { false } }) }; if !t2811.truthy() { t2811 } else { { let t2810 = { let t2806 = v_x.clone(); Py::Bool({ let t2807 = Py::Int(0i64); if py::gt(&t2806, &t2807)? { true } else { false } }) }; if !t2810.truthy() { t2810 } else { { let t2808 = v_b.clone(); Py::Bool({ let t2809 = Py::Int(0i64); if py::ge(&t2808, &t2809)? { true } else { false } }) } } } } }, Py::str("line r and b must not be negative and x must be positive"))?;
        let _ = py::method(&v_br.clone(), "append", &[Py::tuple(vec![v_i.clone(), v_j.clone(), py::complex(&[v_r.clone(), v_x.clone()])?, v_b.clone()])])?;
    }
    v_nb = py::minmax(&[{ let mut out = vec![]; for t2812 in py::iter(&(v_br.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t2813 = (t2812).items()?;
if t2813.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_i = t2813[0].clone();
v_j = t2813[1].clone();
v__ = t2813[2].clone();
v__ = t2813[3].clone();
out.push(py::minmax(&[v_i.clone(), v_j.clone()], true)?); } Py::list(out) }], true)?;
    let _ = f__need({ let t2814 = py::set(&{ let mut out = vec![]; for t2816 in py::iter(&(v_br.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t2817 = (t2816).items()?;
if t2817.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_i = t2817[0].clone();
v_j = t2817[1].clone();
v__ = t2817[2].clone();
v__ = t2817[3].clone();
for t2818 in py::iter(&(Py::tuple(vec![v_i.clone(), v_j.clone()])))? { let mut v_k: Py; 
v_k = t2818;
out.push(v_k.clone()); }} Py::list(out) })?; Py::Bool({ let t2815 = py::set(&Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nb.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?; if py::eq(&t2814, &t2815) { true } else { false } }) }, Py::str("buses must be numbered 1 to n without gaps"))?;
    let t2819 = (Py::tuple(vec![py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?, py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?])).items()?;
    if t2819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Pd = t2819[0].clone();
    v_Qd = t2819[1].clone();
    for t2820 in py::iter(&(v_loads.clone()))? {
        v_row = t2820;
        let _ = f__need({ let t2829 = { let t2821 = py::len(&v_row.clone())?; Py::Bool({ let t2822 = Py::Int(3i64); if py::eq(&t2821, &t2822) { true } else { false } }) }; if !t2829.truthy() { t2829 } else { { let t2828 = { let t2823 = py::getitem(&v_row.clone(), &Py::Int(0i64))?; Py::Bool({ let t2824 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::eq(&t2823, &t2824) { true } else { false } }) }; if !t2828.truthy() { t2828 } else { { let t2825 = Py::Int(2i64); Py::Bool({ let t2826 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::le(&t2825, &t2826)? { { let t2827 = v_nb.clone(); if py::le(&t2826, &t2827)? { true } else { false } } } else { false } }) } } } } }, Py::str("each load row is: bus (2..n), P, Q in per unit"))?;
        let t2830 = v_Pd.clone(); let t2831 = py::sub(&py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?;
        py::setitem(&t2830, &t2831, py::iadd(&py::getitem(&t2830, &t2831)?, &py::getitem(&v_row.clone(), &Py::Int(1i64))?)?)?;
        let t2832 = v_Qd.clone(); let t2833 = py::sub(&py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?;
        py::setitem(&t2832, &t2833, py::iadd(&py::getitem(&t2832, &t2833)?, &py::getitem(&v_row.clone(), &Py::Int(2i64))?)?)?;
    }
    let t2834 = (Py::tuple(vec![Py::list(vec![]), py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?, Py::dict(vec![])])).items()?;
    if t2834.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_pv = t2834[0].clone();
    v_Pg = t2834[1].clone();
    v_Vset = t2834[2].clone();
    for t2836 in py::iter(&({ let t2835 = v_gens.clone(); if t2835.truthy() { t2835 } else { Py::list(vec![]) } }))? {
        v_row = t2836;
        let _ = f__need({ let t2845 = { let t2837 = py::len(&v_row.clone())?; Py::Bool({ let t2838 = Py::Int(3i64); if py::eq(&t2837, &t2838) { true } else { false } }) }; if !t2845.truthy() { t2845 } else { { let t2844 = { let t2839 = py::getitem(&v_row.clone(), &Py::Int(0i64))?; Py::Bool({ let t2840 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::eq(&t2839, &t2840) { true } else { false } }) }; if !t2844.truthy() { t2844 } else { { let t2841 = Py::Int(2i64); Py::Bool({ let t2842 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::le(&t2841, &t2842)? { { let t2843 = v_nb.clone(); if py::le(&t2842, &t2843)? { true } else { false } } } else { false } }) } } } } }, Py::str("each generator row is: bus (2..n), P, V (per unit)"))?;
        v_k = py::sub(&py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?;
        let _ = f__need({ let t2846 = v_k.clone(); Py::Bool({ let t2847 = v_Vset.clone(); if !py::contains(&t2847, &t2846)? { true } else { false } }) }, Py::str("one generator row per bus"))?;
        let _ = f__need({ let t2848 = Py::Float(0.8); Py::Bool({ let t2849 = py::getitem(&v_row.clone(), &Py::Int(2i64))?; if py::le(&t2848, &t2849)? { { let t2850 = Py::Float(1.2); if py::le(&t2849, &t2850)? { true } else { false } } } else { false } }) }, Py::str("generator voltages must lie between 0.8 and 1.2 pu"))?;
        let t2851 = v_Pg.clone(); let t2852 = v_k.clone();
        py::setitem(&t2851, &t2852, py::iadd(&py::getitem(&t2851, &t2852)?, &py::getitem(&v_row.clone(), &Py::Int(1i64))?)?)?;
        py::setitem(&v_Vset.clone(), &v_k.clone(), py::getitem(&v_row.clone(), &Py::Int(2i64))?)?;
    }
    v_Y = { let mut out = vec![]; for t2853 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v__: Py; 
v__ = t2853;
out.push(py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_nb.clone())?); } Py::list(out) };
    for t2854 in py::iter(&(v_br.clone()))? {
        let t2863 = (t2854).items()?;
        if t2863.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2863[0].clone();
        v_j = t2863[1].clone();
        v_z = t2863[2].clone();
        v_b = t2863[3].clone();
        v_y = py::truediv(&Py::Int(1i64), &v_z.clone())?;
        let t2855 = py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t2856 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t2855, &t2856, py::iadd(&py::getitem(&t2855, &t2856)?, &py::add(&v_y.clone(), &py::mul(&Py::Complex(0.0, 0.5), &v_b.clone())?)?)?)?;
        let t2857 = py::getitem(&v_Y.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t2858 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
        py::setitem(&t2857, &t2858, py::iadd(&py::getitem(&t2857, &t2858)?, &py::add(&v_y.clone(), &py::mul(&Py::Complex(0.0, 0.5), &v_b.clone())?)?)?)?;
        let t2859 = py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t2860 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
        py::setitem(&t2859, &t2860, py::sub(&py::getitem(&t2859, &t2860)?, &v_y.clone())?)?;
        let t2861 = py::getitem(&v_Y.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t2862 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t2861, &t2862, py::sub(&py::getitem(&t2861, &t2862)?, &v_y.clone())?)?;
    }
    v_V = py::mul(&Py::list(vec![Py::Float(1.0)]), &v_nb.clone())?;
    py::setitem(&v_V.clone(), &Py::Int(0i64), v_V1.clone())?;
    for t2864 in py::iter(&(py::method(&v_Vset.clone(), "items", &[])?))? {
        let t2865 = (t2864).items()?;
        if t2865.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2865[0].clone();
        v_v = t2865[1].clone();
        py::setitem(&v_V.clone(), &v_k.clone(), v_v.clone())?;
    }
    v_th = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?;
    v_pq = { let mut out = vec![]; for t2866 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2866;
if ({ let t2867 = v_k.clone(); Py::Bool({ let t2868 = v_Vset.clone(); if !py::contains(&t2868, &t2867)? { true } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    v_pvb = { let mut out = vec![]; for t2869 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2869;
if ({ let t2870 = v_k.clone(); Py::Bool({ let t2871 = v_Vset.clone(); if py::contains(&t2871, &t2870)? { true } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    v_nsl = Py::list((Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (v_nb.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
    v_Psp = { let mut out = vec![]; for t2872 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2872;
out.push(py::sub(&py::getitem(&v_Pg.clone(), &v_k.clone())?, &py::getitem(&v_Pd.clone(), &v_k.clone())?)?); } Py::list(out) };
    v_Qsp = { let mut out = vec![]; for t2873 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2873;
out.push(py::neg(&py::getitem(&v_Qd.clone(), &v_k.clone())?)?); } Py::list(out) };
    v_calc = { let v_Pd = v_Pd.clone(); let v_Pg = v_Pg.clone(); let v_Psp = v_Psp.clone(); let v_Qd = v_Qd.clone(); let v_Qsp = v_Qsp.clone(); let v_V = v_V.clone(); let v_V1 = v_V1.clone(); let v_Vset = v_Vset.clone(); let v_Y = v_Y.clone(); let v_b = v_b.clone(); let v_br = v_br.clone(); let v_calc = v_calc.clone(); let v_gens = v_gens.clone(); let v_i = v_i.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_lines = v_lines.clone(); let v_loads = v_loads.clone(); let v_nb = v_nb.clone(); let v_nsl = v_nsl.clone(); let v_pq = v_pq.clone(); let v_pv = v_pv.clone(); let v_pvb = v_pvb.clone(); let v_r = v_r.clone(); let v_row = v_row.clone(); let v_th = v_th.clone(); let v_v = v_v.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_S: Py = Py::None;
            let mut v_Vc: Py = Py::None;
            v_Vc = { let mut out = vec![]; for t2874 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2874;
out.push(py::cmath("rect", &[py::getitem(&v_V.clone(), &v_k.clone())?, py::getitem(&v_th.clone(), &v_k.clone())?])?); } Py::list(out) };
            v_S = { let mut out = vec![]; for t2875 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2875;
out.push(py::mul(&py::getitem(&v_Vc.clone(), &v_k.clone())?, &py::method(&py::sum(&{ let mut out = vec![]; for t2876 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_m: Py; 
v_m = t2876;
out.push(py::mul(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?, &py::getitem(&v_Vc.clone(), &v_m.clone())?)?); } Py::list(out) }, None)?, "conjugate", &[])?)?); } Py::list(out) };
            return Ok(Py::tuple(vec![v_Vc.clone(), v_S.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let mut t2877 = false;
    for t2878 in py::range_iter(&[(Py::Int(40i64)).i()?]) {
        v_it = t2878;
        let t2879 = (py::call(&v_calc.clone(), &[])?).items()?;
        if t2879.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Vc = t2879[0].clone();
        v_S = t2879[1].clone();
        v_mis = py::add(&{ let mut out = vec![]; for t2880 in py::iter(&(v_nsl.clone()))? { let mut v_k: Py; 
v_k = t2880;
out.push(py::sub(&py::getitem(&v_Psp.clone(), &v_k.clone())?, &py::real(&py::getitem(&v_S.clone(), &v_k.clone())?)?)?); } Py::list(out) }, &{ let mut out = vec![]; for t2881 in py::iter(&(v_pq.clone()))? { let mut v_k: Py; 
v_k = t2881;
out.push(py::sub(&py::getitem(&v_Qsp.clone(), &v_k.clone())?, &py::imag(&py::getitem(&v_S.clone(), &v_k.clone())?)?)?); } Py::list(out) })?;
        if ({ let t2882 = py::minmax(&[{ let mut out = vec![]; for t2884 in py::iter(&(v_mis.clone()))? { let mut v_m: Py; 
v_m = t2884;
out.push(py::abs(&v_m.clone())?); } Py::list(out) }], true)?; Py::Bool({ let t2883 = Py::Float(1e-13); if py::lt(&t2882, &t2883)? { true } else { false } }) }).truthy() {
            { t2877 = true; break; }
        }
        let t2885 = (Py::tuple(vec![py::len(&v_nsl.clone())?, py::len(&v_pq.clone())?])).items()?;
        if t2885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n1 = t2885[0].clone();
        v_n2 = t2885[1].clone();
        v_J = { let mut out = vec![]; for t2886 in py::range_iter(&[(py::add(&v_n1.clone(), &v_n2.clone())?).i()?]) { let mut v__: Py; 
v__ = t2886;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n1.clone(), &v_n2.clone())?)?); } Py::list(out) };
        for t2887 in py::iter(&(py::enumerate(&v_nsl.clone(), 0)?))? {
            let t2896 = (t2887).items()?;
            if t2896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2896[0].clone();
            v_k = t2896[1].clone();
            for t2888 in py::iter(&(py::enumerate(&v_nsl.clone(), 0)?))? {
                let t2891 = (t2888).items()?;
                if t2891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_c = t2891[0].clone();
                v_m = t2891[1].clone();
                if ({ let t2889 = v_k.clone(); Py::Bool({ let t2890 = v_m.clone(); if py::eq(&t2889, &t2890) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_J.clone(), &v_a.clone())?, &v_c.clone(), py::sub(&py::neg(&py::imag(&py::getitem(&v_S.clone(), &v_k.clone())?)?)?, &py::mul(&py::pow(&py::getitem(&v_V.clone(), &v_k.clone())?, &Py::Int(2i64))?, &py::imag(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_k.clone())?)?)?)?)?;
                } else {
                    v_Ykm = py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?;
                    v_t = py::sub(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&v_th.clone(), &v_m.clone())?)?;
                    py::setitem(&py::getitem(&v_J.clone(), &v_a.clone())?, &v_c.clone(), py::mul(&py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::getitem(&v_V.clone(), &v_m.clone())?)?, &py::sub(&py::mul(&py::real(&v_Ykm.clone())?, &py::math1("sin", &v_t.clone())?)?, &py::mul(&py::imag(&v_Ykm.clone())?, &py::math1("cos", &v_t.clone())?)?)?)?)?;
                }
            }
            for t2892 in py::iter(&(py::enumerate(&v_pq.clone(), 0)?))? {
                let t2895 = (t2892).items()?;
                if t2895.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_c = t2895[0].clone();
                v_m = t2895[1].clone();
                if ({ let t2893 = v_k.clone(); Py::Bool({ let t2894 = v_m.clone(); if py::eq(&t2893, &t2894) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_J.clone(), &v_a.clone())?, &py::add(&v_n1.clone(), &v_c.clone())?, py::add(&py::truediv(&py::real(&py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::getitem(&v_V.clone(), &v_k.clone())?)?, &py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::real(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_k.clone())?)?)?)?)?;
                } else {
                    v_Ykm = py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?;
                    v_t = py::sub(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&v_th.clone(), &v_m.clone())?)?;
                    py::setitem(&py::getitem(&v_J.clone(), &v_a.clone())?, &py::add(&v_n1.clone(), &v_c.clone())?, py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::add(&py::mul(&py::real(&v_Ykm.clone())?, &py::math1("cos", &v_t.clone())?)?, &py::mul(&py::imag(&v_Ykm.clone())?, &py::math1("sin", &v_t.clone())?)?)?)?)?;
                }
            }
        }
        for t2897 in py::iter(&(py::enumerate(&v_pq.clone(), 0)?))? {
            let t2906 = (t2897).items()?;
            if t2906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2906[0].clone();
            v_k = t2906[1].clone();
            for t2898 in py::iter(&(py::enumerate(&v_nsl.clone(), 0)?))? {
                let t2901 = (t2898).items()?;
                if t2901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_c = t2901[0].clone();
                v_m = t2901[1].clone();
                if ({ let t2899 = v_k.clone(); Py::Bool({ let t2900 = v_m.clone(); if py::eq(&t2899, &t2900) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_J.clone(), &py::add(&v_n1.clone(), &v_a.clone())?)?, &v_c.clone(), py::sub(&py::real(&py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::mul(&py::pow(&py::getitem(&v_V.clone(), &v_k.clone())?, &Py::Int(2i64))?, &py::real(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_k.clone())?)?)?)?)?;
                } else {
                    v_Ykm = py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?;
                    v_t = py::sub(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&v_th.clone(), &v_m.clone())?)?;
                    py::setitem(&py::getitem(&v_J.clone(), &py::add(&v_n1.clone(), &v_a.clone())?)?, &v_c.clone(), py::mul(&py::mul(&py::neg(&py::getitem(&v_V.clone(), &v_k.clone())?)?, &py::getitem(&v_V.clone(), &v_m.clone())?)?, &py::add(&py::mul(&py::real(&v_Ykm.clone())?, &py::math1("cos", &v_t.clone())?)?, &py::mul(&py::imag(&v_Ykm.clone())?, &py::math1("sin", &v_t.clone())?)?)?)?)?;
                }
            }
            for t2902 in py::iter(&(py::enumerate(&v_pq.clone(), 0)?))? {
                let t2905 = (t2902).items()?;
                if t2905.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_c = t2905[0].clone();
                v_m = t2905[1].clone();
                if ({ let t2903 = v_k.clone(); Py::Bool({ let t2904 = v_m.clone(); if py::eq(&t2903, &t2904) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_J.clone(), &py::add(&v_n1.clone(), &v_a.clone())?)?, &py::add(&v_n1.clone(), &v_c.clone())?, py::sub(&py::truediv(&py::imag(&py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::getitem(&v_V.clone(), &v_k.clone())?)?, &py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::imag(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_k.clone())?)?)?)?)?;
                } else {
                    v_Ykm = py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?;
                    v_t = py::sub(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&v_th.clone(), &v_m.clone())?)?;
                    py::setitem(&py::getitem(&v_J.clone(), &py::add(&v_n1.clone(), &v_a.clone())?)?, &py::add(&v_n1.clone(), &v_c.clone())?, py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::sub(&py::mul(&py::real(&v_Ykm.clone())?, &py::math1("sin", &v_t.clone())?)?, &py::mul(&py::imag(&v_Ykm.clone())?, &py::math1("cos", &v_t.clone())?)?)?)?)?;
                }
            }
        }
        v_dx = f__gauss_solve(v_J.clone(), v_mis.clone())?;
        for t2924 in py::iter(&(py::enumerate(&v_nsl.clone(), 0)?))? {
            let t2927 = (t2924).items()?;
            if t2927.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2927[0].clone();
            v_k = t2927[1].clone();
            let t2925 = v_th.clone(); let t2926 = v_k.clone();
            py::setitem(&t2925, &t2926, py::iadd(&py::getitem(&t2925, &t2926)?, &py::getitem(&v_dx.clone(), &v_a.clone())?)?)?;
        }
        for t2928 in py::iter(&(py::enumerate(&v_pq.clone(), 0)?))? {
            let t2931 = (t2928).items()?;
            if t2931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t2931[0].clone();
            v_m = t2931[1].clone();
            let t2929 = v_V.clone(); let t2930 = v_m.clone();
            py::setitem(&t2929, &t2930, py::iadd(&py::getitem(&t2929, &t2930)?, &py::getitem(&v_dx.clone(), &py::add(&v_n1.clone(), &v_c.clone())?)?)?)?;
        }
        let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2932 in py::iter(&(v_V.clone()))? { let mut v_v: Py; 
v_v = t2932;
out.push({ let t2933 = Py::Float(0.3); Py::Bool({ let t2934 = v_v.clone(); if py::lt(&t2933, &t2934)? { { let t2935 = Py::Int(2i64); if py::lt(&t2934, &t2935)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the load flow diverged (the network cannot carry this loading)"))?;
    }
    if !t2877 {
        return Err(py::value_error(py::to_str(&Py::str("the load flow did not converge in 40 iterations"))));
    }
    let t2936 = (py::call(&v_calc.clone(), &[])?).items()?;
    if t2936.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Vc = t2936[0].clone();
    v_S = t2936[1].clone();
    let _ = f__need({ let t2937 = py::minmax(&[{ let mut out = vec![]; for t2939 in py::iter(&(v_nsl.clone()))? { let mut v_k: Py; 
v_k = t2939;
out.push(py::abs(&py::sub(&py::getitem(&v_Psp.clone(), &v_k.clone())?, &py::real(&py::getitem(&v_S.clone(), &v_k.clone())?)?)?)?); } Py::list(out) }], true)?; Py::Bool({ let t2938 = Py::Float(1e-10); if py::lt(&t2937, &t2938)? { true } else { false } }) }, Py::str("the load flow did not converge"))?;
    v_loss = py::sum(&{ let mut out = vec![]; for t2940 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t2940;
out.push(py::real(&v_s.clone())?); } Py::list(out) }, None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), Py::list((v_V.clone()).items()?)), (Py::str("angle_deg"), { let mut out = vec![]; for t2953 in py::iter(&(v_th.clone()))? { let mut v_t: Py; 
v_t = t2953;
out.push(py::math1("degrees", &v_t.clone())?); } Py::list(out) }), (Py::str("P_slack"), py::real(&py::getitem(&v_S.clone(), &Py::Int(0i64))?)?), (Py::str("Q_slack"), py::imag(&py::getitem(&v_S.clone(), &Py::Int(0i64))?)?), (Py::str("P_loss"), v_loss.clone()), (Py::str("Q_gen"), { let mut out = vec![]; for t2954 in py::iter(&(v_pvb.clone()))? { let mut v_k: Py; 
v_k = t2954;
out.push(py::add(&py::imag(&py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::getitem(&v_Qd.clone(), &v_k.clone())?)?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pack_sizing(mut v_V_target: Py, mut v_E_target: Py, mut v_V_cell: Py, mut v_Q_cell: Py) -> R {
    let mut v_np_: Py = Py::None;
    let mut v_ns: Py = Py::None;
    let _ = f__pos(Py::str("the target voltage and energy and the cell voltage and capacity"), Py::tuple(vec![v_V_target.clone(), v_E_target.clone(), v_V_cell.clone(), v_Q_cell.clone()]))?;
    v_ns = py::minmax(&[Py::Int(1i64), f__ceil_tol(py::truediv(&v_V_target.clone(), &v_V_cell.clone())?)?], true)?;
    v_np_ = py::minmax(&[Py::Int(1i64), f__ceil_tol(py::truediv(&v_E_target.clone(), &py::mul(&py::mul(&v_ns.clone(), &v_V_cell.clone())?, &v_Q_cell.clone())?)?)?], true)?;
    let _ = f__need({ let t195 = py::mul(&v_ns.clone(), &v_np_.clone())?; Py::Bool({ let t196 = py::pow(&Py::Int(10i64), &Py::Int(7i64))?; if py::le(&t195, &t196)? { true } else { false } }) }, Py::str("more than ten million cells"))?;
    return Ok(Py::dict(vec![(Py::str("N_series"), v_ns.clone()), (Py::str("N_parallel"), v_np_.clone()), (Py::str("N_cells"), py::mul(&v_ns.clone(), &v_np_.clone())?), (Py::str("V_pack"), f__r(py::mul(&v_ns.clone(), &v_V_cell.clone())?)?), (Py::str("E_pack_kWh"), f__r(py::truediv(&py::mul(&py::mul(&py::mul(&v_ns.clone(), &v_np_.clone())?, &v_V_cell.clone())?, &v_Q_cell.clone())?, &Py::Float(3600000.0))?)?), (Py::str("Q_pack_Ah"), f__r(py::truediv(&py::mul(&v_np_.clone(), &v_Q_cell.clone())?, &Py::Int(3600i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_transformers(mut v_S1: Py, mut v_vk1: Py, mut v_vkr1: Py, mut v_S2: Py, mut v_vk2: Py, mut v_vkr2: Py, mut v_V_lv: Py, mut v_P: Py, mut v_Q: Py) -> R {
    let mut v_SL: Py = Py::None;
    let mut v_Z1: Py = Py::None;
    let mut v_Z2: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_vk: Py = Py::None;
    let mut v_vkr: Py = Py::None;
    let mut v_zb: Py = Py::None;
    let _ = f__pos(Py::str("the ratings and the voltage"), Py::tuple(vec![v_S1.clone(), v_S2.clone(), v_V_lv.clone()]))?;
    for t2993 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_vk1.clone(), v_vkr1.clone()]), Py::tuple(vec![v_vk2.clone(), v_vkr2.clone()])])))? {
        let t2998 = (t2993).items()?;
        if t2998.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_vk = t2998[0].clone();
        v_vkr = t2998[1].clone();
        let _ = f__need({ let t2994 = Py::Int(0i64); Py::Bool({ let t2995 = v_vkr.clone(); if py::le(&t2994, &t2995)? { { let t2996 = v_vk.clone(); if py::lt(&t2995, &t2996)? { { let t2997 = Py::Float(0.5); if py::lt(&t2996, &t2997)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("need 0 <= vkr < vk below 50% for each transformer"))?;
    }
    v_zb = { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_S1 = v_S1.clone(); let v_S2 = v_S2.clone(); let v_V_lv = v_V_lv.clone(); let v_vk1 = v_vk1.clone(); let v_vk2 = v_vk2.clone(); let v_vkr1 = v_vkr1.clone(); let v_vkr2 = v_vkr2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_vk: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_vkr: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_S: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&py::mul(&py::complex(&[v_vkr.clone(), py::math1("sqrt", &py::sub(&py::mul(&v_vk.clone(), &v_vk.clone())?, &py::mul(&v_vkr.clone(), &v_vkr.clone())?)?)?])?, &v_V_lv.clone())?, &v_V_lv.clone())?, &v_S.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2999 = (Py::tuple(vec![py::call(&v_zb.clone(), &[v_vk1.clone(), v_vkr1.clone(), v_S1.clone()])?, py::call(&v_zb.clone(), &[v_vk2.clone(), v_vkr2.clone(), v_S2.clone()])?])).items()?;
    if t2999.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z1 = t2999[0].clone();
    v_Z2 = t2999[1].clone();
    v_SL = py::complex(&[v_P.clone(), v_Q.clone()])?;
    let t3000 = (Py::tuple(vec![py::mul(&v_SL.clone(), &py::method(&py::truediv(&v_Z2.clone(), &py::add(&v_Z1.clone(), &v_Z2.clone())?)?, "conjugate", &[])?)?, py::mul(&v_SL.clone(), &py::method(&py::truediv(&v_Z1.clone(), &py::add(&v_Z1.clone(), &v_Z2.clone())?)?, "conjugate", &[])?)?])).items()?;
    if t3000.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t3000[0].clone();
    v_s2 = t3000[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("S1_share"), py::abs(&v_s1.clone())?), (Py::str("S2_share"), py::abs(&v_s2.clone())?), (Py::str("P1"), py::real(&v_s1.clone())?), (Py::str("Q1"), py::imag(&v_s1.clone())?), (Py::str("P2"), py::real(&v_s2.clone())?), (Py::str("Q2"), py::imag(&v_s2.clone())?), (Py::str("share1_pct_of_rating"), py::truediv(&py::mul(&Py::Int(100i64), &py::abs(&v_s1.clone())?)?, &v_S1.clone())?), (Py::str("share2_pct_of_rating"), py::truediv(&py::mul(&Py::Int(100i64), &py::abs(&v_s2.clone())?)?, &v_S2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_per_unit_change_of_base(mut v_R_pu: Py, mut v_X_pu: Py, mut v_S_old: Py, mut v_V_old: Py, mut v_S_new: Py, mut v_V_new: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_zb: Py = Py::None;
    let _ = f__nonneg(Py::str("the per-unit resistance"), Py::tuple(vec![v_R_pu.clone()]))?;
    let _ = f__pos(Py::str("the per-unit reactance and the bases"), Py::tuple(vec![v_X_pu.clone(), v_S_old.clone(), v_V_old.clone(), v_S_new.clone(), v_V_new.clone()]))?;
    v_f = py::mul(&py::pow(&py::truediv(&v_V_old.clone(), &v_V_new.clone())?, &Py::Int(2i64))?, &py::truediv(&v_S_new.clone(), &v_S_old.clone())?)?;
    v_zb = py::truediv(&py::pow(&v_V_new.clone(), &Py::Int(2i64))?, &v_S_new.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("R_pu_new"), py::mul(&v_R_pu.clone(), &v_f.clone())?), (Py::str("X_pu_new"), py::mul(&v_X_pu.clone(), &v_f.clone())?), (Py::str("Z_pu_new"), py::mul(&py::hypot(&[v_R_pu.clone(), v_X_pu.clone()])?, &v_f.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_per_unit_impedance(mut v_R: Py, mut v_X: Py, mut v_S_base: Py, mut v_V_base: Py) -> R {
    let mut v_Zb: Py = Py::None;
    let _ = f__nonneg(Py::str("the resistance"), Py::tuple(vec![v_R.clone()]))?;
    let _ = f__nonneg(Py::str("the reactance"), Py::tuple(vec![v_X.clone()]))?;
    let _ = f__need({ let t3110 = { let t3106 = v_R.clone(); Py::Bool({ let t3107 = Py::Int(0i64); if py::gt(&t3106, &t3107)? { true } else { false } }) }; if t3110.truthy() { t3110 } else { { let t3108 = v_X.clone(); Py::Bool({ let t3109 = Py::Int(0i64); if py::gt(&t3108, &t3109)? { true } else { false } }) } } }, Py::str("the impedance must not be zero"))?;
    let _ = f__pos(Py::str("the base power and base voltage"), Py::tuple(vec![v_S_base.clone(), v_V_base.clone()]))?;
    v_Zb = py::truediv(&py::mul(&v_V_base.clone(), &v_V_base.clone())?, &v_S_base.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Z_base"), v_Zb.clone()), (Py::str("I_base"), py::truediv(&v_S_base.clone(), &py::mul(&py::math1("sqrt", &Py::Int(3i64))?, &v_V_base.clone())?)?), (Py::str("R_pu"), py::truediv(&v_R.clone(), &v_Zb.clone())?), (Py::str("X_pu"), py::truediv(&v_X.clone(), &v_Zb.clone())?), (Py::str("Z_pu"), py::truediv(&py::hypot(&[v_R.clone(), v_X.clone()])?, &v_Zb.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_peukert(mut v_currents: Py, mut v_times: Py, mut v_I: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let _ = f__need({ let t3165 = { let t3161 = py::len(&v_currents.clone())?; Py::Bool({ let t3162 = py::len(&v_times.clone())?; if py::eq(&t3161, &t3162) { true } else { false } }) }; if !t3165.truthy() { t3165 } else { { let t3163 = py::len(&v_currents.clone())?; Py::Bool({ let t3164 = Py::Int(2i64); if py::ge(&t3163, &t3164)? { true } else { false } }) } } }, Py::str("give at least two discharge tests (a current and its run time each)"))?;
    for t3166 in py::iter(&(py::zip(&[v_currents.clone(), v_times.clone()])?))? {
        let t3172 = (t3166).items()?;
        if t3172.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t3172[0].clone();
        v_t = t3172[1].clone();
        let _ = f__need({ let t3171 = { let t3167 = v_c.clone(); Py::Bool({ let t3168 = Py::Int(0i64); if py::gt(&t3167, &t3168)? { true } else { false } }) }; if !t3171.truthy() { t3171 } else { { let t3169 = v_t.clone(); Py::Bool({ let t3170 = Py::Int(0i64); if py::gt(&t3169, &t3170)? { true } else { false } }) } } }, Py::str("test currents and run times must be positive"))?;
    }
    let _ = f__need({ let t3173 = py::len(&py::set(&v_currents.clone())?)?; Py::Bool({ let t3174 = py::len(&v_currents.clone())?; if py::eq(&t3173, &t3174) { true } else { false } }) }, Py::str("the test currents must be different"))?;
    let _ = f__pos(Py::str("the new discharge current"), Py::tuple(vec![v_I.clone()]))?;
    let t3182 = (Py::tuple(vec![{ let mut out = vec![]; for t3180 in py::iter(&(v_currents.clone()))? { let mut v_c: Py; 
v_c = t3180;
out.push(py::math1("log", &v_c.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3181 in py::iter(&(v_times.clone()))? { let mut v_t: Py; 
v_t = t3181;
out.push(py::math1("log", &v_t.clone())?); } Py::list(out) }])).items()?;
    if t3182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t3182[0].clone();
    v_ys = t3182[1].clone();
    v_n = py::len(&v_xs.clone())?;
    let t3183 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t3183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t3183[0].clone();
    v_my = t3183[1].clone();
    v_b = py::truediv(&py::sum(&{ let mut out = vec![]; for t3184 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3185 = (t3184).items()?;
if t3185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3185[0].clone();
v_y = t3185[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &py::sum(&{ let mut out = vec![]; for t3186 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t3186;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?)?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    v_k = py::neg(&v_b.clone())?;
    let _ = f__need({ let t3187 = v_k.clone(); Py::Bool({ let t3188 = Py::Int(0i64); if py::gt(&t3187, &t3188)? { true } else { false } }) }, Py::str("the run time must fall as the current rises (Peukert exponent k > 0)"))?;
    v_t = py::math1("exp", &py::add(&v_a.clone(), &py::mul(&v_b.clone(), &py::math1("log", &v_I.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("C_p"), py::math1("exp", &v_a.clone())?), (Py::str("runtime_h"), v_t.clone()), (Py::str("capacity_Ah"), py::mul(&v_I.clone(), &v_t.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poa_perez(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py, mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_F1: Py = Py::None;
    let mut v_F2: Py = Py::None;
    let mut v_am: Py = Py::None;
    let mut v_aoi_d: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let mut v_direct: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_ground: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sky: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__check_surface(v_tilt.clone(), v_surface_azimuth.clone())?;
    let _ = f__need({ let t3500 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3500.truthy() { t3500 } else { { let t3497 = Py::Int(0i64); Py::Bool({ let t3498 = v_zenith.clone(); if py::le(&t3497, &t3498)? { { let t3499 = Py::Int(87i64); if py::lt(&t3498, &t3499)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must lie between 0 and 87 degrees"))?;
    let _ = f__need({ let t3501 = Py::Int(0i64); Py::Bool({ let t3502 = v_azimuth.clone(); if py::le(&t3501, &t3502)? { { let t3503 = Py::Int(360i64); if py::le(&t3502, &t3503)? { true } else { false } } } else { false } }) }, Py::str("the solar azimuth must lie between 0 and 360 degrees"))?;
    let _ = f__check_irr(v_dni.clone(), v_ghi.clone(), v_dhi.clone())?;
    let _ = f__need({ let t3511 = v_dhi.clone(); Py::Bool({ let t3512 = Py::Int(0i64); if py::gt(&t3511, &t3512)? { true } else { false } }) }, Py::str("the Perez model needs a positive diffuse irradiance"))?;
    let _ = f__need({ let t3513 = Py::Int(0i64); Py::Bool({ let t3514 = v_albedo.clone(); if py::le(&t3513, &t3514)? { { let t3515 = Py::Int(1i64); if py::le(&t3514, &t3515)? { true } else { false } } } else { false } }) }, Py::str("the albedo must lie between 0 and 1"))?;
    let _ = f__check_day(v_n.clone())?;
    v_am = f__airmass_rel(v_zenith.clone(), Py::str("kastenyoung1989"))?;
    v_z = py::math1("radians", &v_zenith.clone())?;
    v_delta = py::truediv(&py::mul(&v_dhi.clone(), &v_am.clone())?, &f__g_on_spencer(v_n.clone())?)?;
    v_eps = py::truediv(&py::add(&py::truediv(&py::add(&v_dhi.clone(), &v_dni.clone())?, &v_dhi.clone())?, &py::mul(&Py::Float(1.041), &py::pow(&v_z.clone(), &Py::Int(3i64))?)?)?, &py::add(&Py::Int(1i64), &py::mul(&Py::Float(1.041), &py::pow(&v_z.clone(), &Py::Int(3i64))?)?)?)?;
    v_b = py::minmax(&[{ let mut out = vec![]; for t3534 in py::iter(&(py::enumerate(&c__PEREZ_EDGES()?, 0)?))? { let mut v_e: Py; let mut v_i: Py; 
let t3535 = (t3534).items()?;
if t3535.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t3535[0].clone();
v_e = t3535[1].clone();
if ({ let t3536 = v_eps.clone(); Py::Bool({ let t3537 = v_e.clone(); if py::ge(&t3536, &t3537)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); }} Py::list(out) }], true)?;
    v_c = py::getitem(&c__PEREZ_1990()?, &v_b.clone())?;
    v_F1 = py::minmax(&[py::add(&py::add(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_delta.clone())?)?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(2i64))?, &v_z.clone())?)?, Py::Float(0.0)], true)?;
    v_F2 = py::add(&py::add(&py::getitem(&v_c.clone(), &Py::Int(3i64))?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(4i64))?, &v_delta.clone())?)?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(5i64))?, &v_z.clone())?)?;
    v_p = f__projection(v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone())?;
    v_A = py::minmax(&[v_p.clone(), Py::Float(0.0)], true)?;
    v_B = py::minmax(&[f__cosd(v_zenith.clone())?, f__cosd(Py::Int(85i64))?], true)?;
    v_sky = py::minmax(&[py::mul(&v_dhi.clone(), &py::add(&py::add(&py::mul(&py::mul(&Py::Float(0.5), &py::sub(&Py::Int(1i64), &v_F1.clone())?)?, &py::add(&Py::Int(1i64), &f__cosd(v_tilt.clone())?)?)?, &py::truediv(&py::mul(&v_F1.clone(), &v_A.clone())?, &v_B.clone())?)?, &py::mul(&v_F2.clone(), &f__sind(v_tilt.clone())?)?)?)?, Py::Float(0.0)], true)?;
    v_ground = py::mul(&py::mul(&py::mul(&v_ghi.clone(), &v_albedo.clone())?, &py::sub(&Py::Int(1i64), &py::math1("cos", &py::math1("radians", &v_tilt.clone())?)?)?)?, &Py::Float(0.5))?;
    v_aoi_d = py::math1("degrees", &py::math1("acos", &v_p.clone())?)?;
    v_direct = py::minmax(&[py::mul(&v_dni.clone(), &py::math1("cos", &py::math1("radians", &v_aoi_d.clone())?)?)?, Py::Float(0.0)], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("poa_global"), py::add(&py::add(&v_direct.clone(), &v_sky.clone())?, &v_ground.clone())?), (Py::str("poa_direct"), v_direct.clone()), (Py::str("poa_sky_diffuse"), v_sky.clone()), (Py::str("poa_ground_diffuse"), v_ground.clone()), (Py::str("aoi_deg"), v_aoi_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_angle(mut v_V1: Py, mut v_V2: Py, mut v_X: Py, mut v_P: Py) -> R {
    let mut v_Pmax: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the voltages and the reactance"), Py::tuple(vec![v_V1.clone(), v_V2.clone(), v_X.clone()]))?;
    v_Pmax = py::truediv(&py::mul(&v_V1.clone(), &v_V2.clone())?, &v_X.clone())?;
    let _ = f__need({ let t3673 = py::abs(&v_P.clone())?; Py::Bool({ let t3674 = v_Pmax.clone(); if py::lt(&t3673, &t3674)? { true } else { false } }) }, Py::str("the power exceeds the steady-state limit V1 V2/X"))?;
    v_d = py::math1("asin", &py::truediv(&v_P.clone(), &v_Pmax.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("delta_deg"), py::math1("degrees", &v_d.clone())?), (Py::str("Pmax"), v_Pmax.clone()), (Py::str("Q_sent"), py::truediv(&py::sub(&py::mul(&v_V2.clone(), &v_V2.clone())?, &py::mul(&py::mul(&v_V1.clone(), &v_V2.clone())?, &py::math1("cos", &v_d.clone())?)?)?, &v_X.clone())?), (Py::str("Q_received"), py::truediv(&py::sub(&py::mul(&py::mul(&v_V1.clone(), &v_V2.clone())?, &py::math1("cos", &v_d.clone())?)?, &py::mul(&v_V1.clone(), &v_V1.clone())?)?, &v_X.clone())?), (Py::str("margin_pct"), py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&v_Pmax.clone(), &v_P.clone())?)?, &v_Pmax.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_curve_density_corrected(mut v_speeds: Py, mut v_powers: Py, mut v_v: Py, mut v_rho: Py) -> R {
    let mut v_corr: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__check_curve(v_speeds.clone(), v_powers.clone())?;
    let _ = f__nonneg(Py::str("the wind speed"), Py::tuple(vec![v_v.clone()]))?;
    let _ = f__need({ let t3739 = Py::Float(0.5); Py::Bool({ let t3740 = v_rho.clone(); if py::lt(&t3739, &t3740)? { { let t3741 = Py::Float(1.6); if py::lt(&t3740, &t3741)? { true } else { false } } } else { false } }) }, Py::str("the air density must lie between 0.5 and 1.6 kg/m^3"))?;
    v_corr = Py::list(vec![]);
    for t3742 in py::iter(&(v_speeds.clone()))? {
        v_s = t3742;
        v_e = (if ({ let t3743 = v_s.clone(); Py::Bool({ let t3744 = Py::Float(7.5); if py::le(&t3743, &t3744)? { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &Py::Int(3i64))? } else { (if ({ let t3745 = v_s.clone(); Py::Bool({ let t3746 = Py::Float(12.5); if py::ge(&t3745, &t3746)? { true } else { false } }) }).truthy() { py::truediv(&Py::Int(2i64), &Py::Int(3i64))? } else { py::add(&py::truediv(&Py::Int(1i64), &Py::Int(3i64))?, &py::truediv(&py::mul(&py::sub(&v_s.clone(), &Py::Float(7.5))?, &py::sub(&py::truediv(&Py::Int(2i64), &Py::Int(3i64))?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?, &py::sub(&Py::Float(12.5), &Py::Float(7.5))?)?)? }) });
        let _ = py::method(&v_corr.clone(), "append", &[py::mul(&py::pow(&py::truediv(&Py::Float(1.225), &v_rho.clone())?, &v_e.clone())?, &v_s.clone())?])?;
    }
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3747 in py::iter(&(py::zip(&[v_corr.clone(), py::slice(&v_corr.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3748 = (t3747).items()?;
if t3748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3748[0].clone();
v_b = t3748[1].clone();
out.push({ let t3749 = v_b.clone(); Py::Bool({ let t3750 = v_a.clone(); if py::gt(&t3749, &t3750)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the density-corrected curve is not increasing; the correction does not apply"))?;
    v_p = f__interp(v_v.clone(), v_corr.clone(), v_powers.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_kW"), v_p.clone()), (Py::str("P"), py::mul(&Py::Int(1000i64), &v_p.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_curve_output(mut v_speeds: Py, mut v_powers: Py, mut v_v: Py) -> R {
    let mut v_p: Py = Py::None;
    let _ = f__check_curve(v_speeds.clone(), v_powers.clone())?;
    let _ = f__nonneg(Py::str("the wind speed"), Py::tuple(vec![v_v.clone()]))?;
    v_p = f__interp(v_v.clone(), v_speeds.clone(), v_powers.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_kW"), v_p.clone()), (Py::str("P"), py::mul(&Py::Int(1000i64), &v_p.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pv_array_output(mut v_poa: Py, mut v_T_air: Py, mut v_noct: Py, mut v_pdc0: Py, mut v_gamma: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = f_cell_temperature_noct(v_poa.clone(), v_T_air.clone(), v_noct.clone())?;
    v_p = f_pvwatts_dc(v_poa.clone(), py::getitem(&v_t.clone(), &Py::str("T_cell"))?, v_pdc0.clone(), v_gamma.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_cell"), py::getitem(&v_t.clone(), &Py::str("T_cell"))?), (Py::str("T_cell_C"), py::getitem(&v_t.clone(), &Py::str("T_cell_C"))?), (Py::str("pdc"), py::getitem(&v_p.clone(), &Py::str("pdc"))?), (Py::str("derate"), (if ({ let t3911 = v_poa.clone(); Py::Bool({ let t3912 = Py::Int(0i64); if py::gt(&t3911, &t3912)? { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_p.clone(), &Py::str("pdc"))?, &py::mul(&py::mul(&v_poa.clone(), &Py::Float(0.001))?, &v_pdc0.clone())?)? } else { Py::Float(1.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pv_module_at_conditions(mut v_G: Py, mut v_T_cell: Py, mut v_alpha_sc: Py, mut v_a_ref: Py, mut v_IL_ref: Py, mut v_I0_ref: Py, mut v_Rsh_ref: Py, mut v_Rs: Py, mut v_EgRef: Py, mut v_dEgdT: Py) -> R {
    let mut v_Imp: Py = Py::None;
    let mut v_Isc: Py = Py::None;
    let mut v_Vmp: Py = Py::None;
    let mut v_Voc: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = f__desoto(v_G.clone(), v_T_cell.clone(), v_alpha_sc.clone(), v_a_ref.clone(), v_IL_ref.clone(), v_I0_ref.clone(), v_Rsh_ref.clone(), v_Rs.clone(), v_EgRef.clone(), v_dEgdT.clone())?;
    let t4001 = ({ let mut v: Vec<Py> = vec![]; v.extend((v_p.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "IL") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument IL".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "I0") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument I0".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "Rs") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument Rs".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "Rsh") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument Rsh".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument a".into())) } } }; if v.len() > 5 { return Err(PyErr::Type("_sd_mpp() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_sd_mpp() got an unexpected keyword argument".into())) } f__sd_mpp(a0, a1, a2, a3, a4)? }).items()?;
    if t4001.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Isc = t4001[0].clone();
    v_Voc = t4001[1].clone();
    v_Imp = t4001[2].clone();
    v_Vmp = t4001[3].clone();
    let _ = f__need({ let t4006 = { let t4002 = v_Vmp.clone(); Py::Bool({ let t4003 = Py::Int(0i64); if py::gt(&t4002, &t4003)? { true } else { false } }) }; if !t4006.truthy() { t4006 } else { { let t4004 = v_Imp.clone(); Py::Bool({ let t4005 = Py::Int(0i64); if py::gt(&t4004, &t4005)? { true } else { false } }) } } }, Py::str("the maximum power point is not in the first quadrant for these parameters"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Isc"), v_Isc.clone()), (Py::str("Voc"), v_Voc.clone()), (Py::str("Imp"), v_Imp.clone()), (Py::str("Vmp"), v_Vmp.clone()), (Py::str("Pmp"), py::mul(&v_Imp.clone(), &v_Vmp.clone())?), (Py::str("FF"), py::truediv(&py::mul(&v_Imp.clone(), &v_Vmp.clone())?, &py::mul(&v_Isc.clone(), &v_Voc.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pvwatts_dc(mut v_G_eff: Py, mut v_T_cell: Py, mut v_pdc0: Py, mut v_gamma: Py) -> R {
    let mut v_p: Py = Py::None;
    let _ = f__nonneg(Py::str("the effective irradiance"), Py::tuple(vec![v_G_eff.clone()]))?;
    let _ = f__check_temp(Py::str("cell temperature"), v_T_cell.clone())?;
    let _ = f__pos(Py::str("the DC rating pdc0"), Py::tuple(vec![v_pdc0.clone()]))?;
    let _ = f__need({ let t4066 = py::neg(&Py::Float(0.02))?; Py::Bool({ let t4067 = v_gamma.clone(); if py::le(&t4066, &t4067)? { { let t4068 = Py::Float(0.01); if py::le(&t4067, &t4068)? { true } else { false } } } else { false } }) }, Py::str("the temperature coefficient gamma must lie between -0.02 and 0.01 per K (e.g. -0.004 or -0.4%)"))?;
    v_p = py::mul(&py::mul(&py::mul(&v_G_eff.clone(), &Py::Float(0.001))?, &v_pdc0.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_gamma.clone(), &py::sub(&f__degC(v_T_cell.clone())?, &Py::Float(25.0))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("pdc"), v_p.clone()), (Py::str("pdc_kW"), py::truediv(&v_p.clone(), &Py::Int(1000i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pvwatts_inverter(mut v_pdc: Py, mut v_pdc0: Py, mut v_eta_nom: Py, mut v_eta_ref: Py) -> R {
    let mut v_eta: Py = Py::None;
    let mut v_pac: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("the DC input power"), Py::tuple(vec![v_pdc.clone()]))?;
    let _ = f__pos(Py::str("the inverter DC rating pdc0"), Py::tuple(vec![v_pdc0.clone()]))?;
    let _ = f__need({ let t4125 = { let t4119 = Py::Float(0.5); Py::Bool({ let t4120 = v_eta_nom.clone(); if py::lt(&t4119, &t4120)? { { let t4121 = Py::Int(1i64); if py::le(&t4120, &t4121)? { true } else { false } } } else { false } }) }; if !t4125.truthy() { t4125 } else { { let t4122 = Py::Float(0.5); Py::Bool({ let t4123 = v_eta_ref.clone(); if py::lt(&t4122, &t4123)? { { let t4124 = Py::Int(1i64); if py::le(&t4123, &t4124)? { true } else { false } } } else { false } }) } } }, Py::str("the efficiencies must lie between 0.5 and 1"))?;
    v_z = py::truediv(&v_pdc.clone(), &v_pdc0.clone())?;
    v_eta = py::mul(&py::truediv(&v_eta_nom.clone(), &v_eta_ref.clone())?, &py::add(&py::sub(&py::mul(&py::neg(&Py::Float(0.0162))?, &v_z.clone())?, &py::truediv(&Py::Float(0.0059), &v_z.clone())?)?, &Py::Float(0.9858))?)?;
    v_pac = py::minmax(&[Py::Float(0.0), py::minmax(&[py::mul(&v_eta_nom.clone(), &v_pdc0.clone())?, py::mul(&v_eta.clone(), &v_pdc.clone())?], false)?], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("pac"), v_pac.clone()), (Py::str("efficiency"), py::truediv(&v_pac.clone(), &v_pdc.clone())?), (Py::str("clipped"), { let t4138 = v_pac.clone(); Py::Bool({ let t4139 = py::mul(&v_eta_nom.clone(), &v_pdc0.clone())?; if py::eq(&t4138, &t4139) { true } else { false } }) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pvwatts_losses(mut v_kw: Py) -> R {
    let mut v_default: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_perf: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_perf = Py::Float(1.0);
    for t4173 in py::iter(&(c__PVW_LOSSES()?))? {
        let t4178 = (t4173).items()?;
        if t4178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t4178[0].clone();
        v_default = t4178[1].clone();
        v_v = py::method(&v_kw.clone(), "get", &[v_name.clone(), py::truediv(&v_default.clone(), &Py::Float(100.0))?])?;
        let _ = f__need({ let t4177 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4177.truthy() { t4177 } else { { let t4174 = Py::Int(0i64); Py::Bool({ let t4175 = v_v.clone(); if py::le(&t4174, &t4175)? { { let t4176 = Py::Int(1i64); if py::lt(&t4175, &t4176)? { true } else { false } } } else { false } }) } } }, Py::str(&["the ".to_string(), py::format(&v_name.clone(), "")?, " loss must lie between 0 and 100%".to_string()].concat()))?;
        v_perf = py::mul(&v_perf, &py::sub(&Py::Int(1i64), &v_v.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("total_loss"), py::sub(&Py::Int(1i64), &v_perf.clone())?), (Py::str("performance_factor"), v_perf.clone()), (Py::str("total_loss_pct"), py::mul(&py::sub(&Py::Int(1i64), &v_perf.clone())?, &Py::Int(100i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_short_line(mut v_Vr: Py, mut v_P: Py, mut v_pf: Py, mut v_R: Py, mut v_X: Py, mut v_pf_type: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_Ic: Py = Py::None;
    let mut v_Ps: Py = Py::None;
    let mut v_Vs: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_vr: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let _ = f__pos(Py::str("the receiving-end voltage and the load power"), Py::tuple(vec![v_Vr.clone(), v_P.clone()]))?;
    let _ = f__nonneg(Py::str("the line resistance and reactance"), Py::tuple(vec![v_R.clone(), v_X.clone()]))?;
    let t4240 = (f__load_current(v_Vr.clone(), v_P.clone(), v_pf.clone(), v_pf_type.clone())?).items()?;
    if t4240.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t4240[0].clone();
    v_I = t4240[1].clone();
    v_Ic = t4240[2].clone();
    v_vr = py::truediv(&v_Vr.clone(), &c_SQ3()?)?;
    v_vs = py::add(&v_vr.clone(), &py::mul(&v_Ic.clone(), &py::complex(&[v_R.clone(), v_X.clone()])?)?)?;
    v_Vs = py::mul(&c_SQ3()?, &py::abs(&v_vs.clone())?)?;
    v_Ps = py::add(&v_P.clone(), &py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_I.clone())?, &v_I.clone())?, &v_R.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vs"), v_Vs.clone()), (Py::str("delta_deg"), py::math1("degrees", &py::cmath("phase", &[v_vs.clone()])?)?), (Py::str("I"), v_I.clone()), (Py::str("losses"), py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_I.clone())?, &v_I.clone())?, &v_R.clone())?), (Py::str("Ps"), v_Ps.clone()), (Py::str("regulation_pct"), py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&v_Vs.clone(), &v_Vr.clone())?)?, &v_Vr.clone())?), (Py::str("efficiency_pct"), py::truediv(&py::mul(&Py::Int(100i64), &v_P.clone())?, &v_Ps.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_diode_current(mut v_V: Py, mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_nNsVth: Py) -> R {
    let mut v_I: Py = Py::None;
    let _ = f__sd_check(v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_nNsVth.clone())?;
    v_I = f__sd_current(v_V.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_nNsVth.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("I"), v_I.clone()), (Py::str("P"), py::mul(&v_V.clone(), &v_I.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_diode_mpp(mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_nNsVth: Py) -> R {
    let mut v_Imp: Py = Py::None;
    let mut v_Isc: Py = Py::None;
    let mut v_Vmp: Py = Py::None;
    let mut v_Voc: Py = Py::None;
    let _ = f__sd_check(v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_nNsVth.clone())?;
    let t4425 = (f__sd_mpp(v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_nNsVth.clone())?).items()?;
    if t4425.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Isc = t4425[0].clone();
    v_Voc = t4425[1].clone();
    v_Imp = t4425[2].clone();
    v_Vmp = t4425[3].clone();
    let _ = f__need({ let t4430 = { let t4426 = v_Vmp.clone(); Py::Bool({ let t4427 = Py::Int(0i64); if py::gt(&t4426, &t4427)? { true } else { false } }) }; if !t4430.truthy() { t4430 } else { { let t4428 = v_Imp.clone(); Py::Bool({ let t4429 = Py::Int(0i64); if py::gt(&t4428, &t4429)? { true } else { false } }) } } }, Py::str("the maximum power point is not in the first quadrant for these parameters"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Isc"), v_Isc.clone()), (Py::str("Voc"), v_Voc.clone()), (Py::str("Imp"), v_Imp.clone()), (Py::str("Vmp"), v_Vmp.clone()), (Py::str("Pmp"), py::mul(&v_Imp.clone(), &v_Vmp.clone())?), (Py::str("FF"), py::truediv(&py::mul(&v_Imp.clone(), &v_Vmp.clone())?, &py::mul(&v_Isc.clone(), &v_Voc.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_diode_voltage(mut v_I: Py, mut v_IL: Py, mut v_I0: Py, mut v_Rs: Py, mut v_Rsh: Py, mut v_nNsVth: Py) -> R {
    let mut v_V: Py = Py::None;
    let _ = f__sd_check(v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_nNsVth.clone())?;
    v_V = f__sd_voltage(v_I.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_nNsVth.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("P"), py::mul(&v_V.clone(), &v_I.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solar_declination(mut v_n: Py, mut v_method: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__check_day(v_n.clone())?;
    v_d = (if ({ let t4643 = v_method.clone(); Py::Bool({ let t4644 = Py::str("cooper"); if py::eq(&t4643, &t4644) { true } else { false } }) }).truthy() { f__decl_cooper(v_n.clone())? } else { f__decl_spencer(v_n.clone())? });
    return Ok(f__out(Py::dict(vec![(Py::str("declination_deg"), py::math1("degrees", &v_d.clone())?), (Py::str("declination_rad"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solar_position(mut v_latitude: Py, mut v_longitude: Py, mut v_n: Py, mut v_clock: Py, mut v_utc_offset: Py) -> R {
    let mut v_az: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_eot: Py = Py::None;
    let mut v_ha: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__check_day(v_n.clone())?;
    let _ = f__check_place(v_latitude.clone(), v_longitude.clone(), v_utc_offset.clone())?;
    let _ = f__need({ let t4712 = Py::Int(0i64); Py::Bool({ let t4713 = v_clock.clone(); if py::le(&t4712, &t4713)? { { let t4714 = Py::Int(24i64); if py::lt(&t4713, &t4714)? { true } else { false } } } else { false } }) }, Py::str("the clock time must be from 0 to 24 hours (decimal hours or hh:mm)"))?;
    v_d = f__decl_spencer(v_n.clone())?;
    v_eot = f__eot_spencer(v_n.clone())?;
    v_ha = py::add(&py::add(&py::mul(&Py::Float(15.0), &py::sub(&py::sub(&v_clock.clone(), &v_utc_offset.clone())?, &Py::Float(12.0))?)?, &v_longitude.clone())?, &py::truediv(&v_eot.clone(), &Py::Float(4.0))?)?;
    let t4726 = (f__sun_angles(v_latitude.clone(), v_d.clone(), v_ha.clone())?).items()?;
    if t4726.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t4726[0].clone();
    v_az = t4726[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("declination_deg"), py::math1("degrees", &v_d.clone())?), (Py::str("eot_min"), v_eot.clone()), (Py::str("hour_angle_deg"), v_ha.clone()), (Py::str("zenith_deg"), v_z.clone()), (Py::str("elevation_deg"), py::sub(&Py::Float(90.0), &v_z.clone())?), (Py::str("azimuth_deg"), v_az.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solar_time_hour_angle(mut v_n: Py, mut v_clock: Py, mut v_longitude: Py, mut v_utc_offset: Py) -> R {
    let mut v_eot: Py = Py::None;
    let mut v_ha: Py = Py::None;
    let _ = f__check_day(v_n.clone())?;
    let _ = f__check_place(Py::None, v_longitude.clone(), v_utc_offset.clone())?;
    let _ = f__need({ let t4794 = Py::Int(0i64); Py::Bool({ let t4795 = v_clock.clone(); if py::le(&t4794, &t4795)? { { let t4796 = Py::Int(24i64); if py::lt(&t4795, &t4796)? { true } else { false } } } else { false } }) }, Py::str("the clock time must be from 0 to 24 hours (decimal hours or hh:mm)"))?;
    v_eot = f__eot_spencer(v_n.clone())?;
    v_ha = py::add(&py::add(&py::mul(&Py::Float(15.0), &py::sub(&py::sub(&v_clock.clone(), &v_utc_offset.clone())?, &Py::Float(12.0))?)?, &v_longitude.clone())?, &py::truediv(&v_eot.clone(), &Py::Float(4.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("eot_min"), v_eot.clone()), (Py::str("hour_angle_deg"), v_ha.clone()), (Py::str("solar_time_h"), py::add(&Py::Float(12.0), &py::truediv(&v_ha.clone(), &Py::Float(15.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stack_power(mut v_I: Py, mut v_N: Py, mut v_V_cell: Py) -> R {
    let _ = f__pos(Py::str("the current and the cell voltage"), Py::tuple(vec![v_I.clone(), v_V_cell.clone()]))?;
    let _ = f__need({ let t4850 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4850.truthy() { t4850 } else { { let t4847 = Py::Int(1i64); Py::Bool({ let t4848 = v_N.clone(); if py::le(&t4847, &t4848)? { { let t4849 = Py::Int(100000i64); if py::le(&t4848, &t4849)? { true } else { false } } } else { false } }) } } }, Py::str("the number of cells must be a whole number from 1 to 100,000"))?;
    let _ = f__need({ let t4851 = v_V_cell.clone(); Py::Bool({ let t4852 = Py::Int(5i64); if py::le(&t4851, &t4852)? { true } else { false } }) }, Py::str("the cell voltage must be at most 5 V"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::mul(&py::mul(&v_N.clone(), &v_I.clone())?, &v_V_cell.clone())?), (Py::str("P_kW"), py::truediv(&py::mul(&py::mul(&v_N.clone(), &v_I.clone())?, &v_V_cell.clone())?, &Py::Float(1000.0))?), (Py::str("V_stack"), py::mul(&v_N.clone(), &v_V_cell.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sun_angles(mut v_latitude: Py, mut v_declination: Py, mut v_hour_angle: Py) -> R {
    let mut v_az: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__check_place(v_latitude.clone(), Py::None, Py::None)?;
    let _ = f__need({ let t4916 = py::neg(&Py::Float(23.5))?; Py::Bool({ let t4917 = v_declination.clone(); if py::le(&t4916, &t4917)? { { let t4918 = Py::Float(23.5); if py::le(&t4917, &t4918)? { true } else { false } } } else { false } }) }, Py::str("the declination must lie between -23.5 and 23.5 degrees"))?;
    let _ = f__need({ let t4919 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t4920 = v_hour_angle.clone(); if py::le(&t4919, &t4920)? { { let t4921 = Py::Int(180i64); if py::le(&t4920, &t4921)? { true } else { false } } } else { false } }) }, Py::str("the hour angle must lie between -180 and 180 degrees"))?;
    let t4933 = (f__sun_angles(v_latitude.clone(), py::math1("radians", &v_declination.clone())?, v_hour_angle.clone())?).items()?;
    if t4933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t4933[0].clone();
    v_az = t4933[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("zenith_deg"), v_z.clone()), (Py::str("elevation_deg"), py::sub(&Py::Float(90.0), &v_z.clone())?), (Py::str("azimuth_deg"), v_az.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sunrise_sunset(mut v_latitude: Py, mut v_longitude: Py, mut v_n: Py, mut v_utc_offset: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_eot: Py = Py::None;
    let mut v_rise: Py = Py::None;
    let mut v_sset: Py = Py::None;
    let mut v_to_h: Py = Py::None;
    let mut v_transit: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__check_day(v_n.clone())?;
    let _ = f__check_place(v_latitude.clone(), v_longitude.clone(), v_utc_offset.clone())?;
    v_d = f__decl_spencer(v_n.clone())?;
    v_eot = f__eot_spencer(v_n.clone())?;
    v_x = py::mul(&py::neg(&py::math1("tan", &v_d.clone())?)?, &py::math1("tan", &py::math1("radians", &v_latitude.clone())?)?)?;
    let _ = f__need({ let t5001 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t5002 = v_x.clone(); if py::lt(&t5001, &t5002)? { { let t5003 = Py::Int(1i64); if py::lt(&t5002, &t5003)? { true } else { false } } } else { false } }) }, Py::str("the sun does not rise or does not set on this day at this latitude (polar day or night)"))?;
    v_ws = py::math1("degrees", &py::math1("acos", &v_x.clone())?)?;
    v_to_h = { let v_d = v_d.clone(); let v_eot = v_eot.clone(); let v_latitude = v_latitude.clone(); let v_longitude = v_longitude.clone(); let v_n = v_n.clone(); let v_utc_offset = v_utc_offset.clone(); let v_ws = v_ws.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_ha: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&py::truediv(&py::sub(&py::sub(&v_ha.clone(), &v_longitude.clone())?, &py::truediv(&v_eot.clone(), &Py::Float(4.0))?)?, &Py::Float(15.0))?, &Py::Float(12.0))?, &v_utc_offset.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t5004 = (Py::tuple(vec![py::call(&v_to_h.clone(), &[py::neg(&v_ws.clone())?])?, py::call(&v_to_h.clone(), &[v_ws.clone()])?, py::call(&v_to_h.clone(), &[Py::Float(0.0)])?])).items()?;
    if t5004.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_rise = t5004[0].clone();
    v_sset = t5004[1].clone();
    v_transit = t5004[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("sunrise_h"), v_rise.clone()), (Py::str("sunset_h"), v_sset.clone()), (Py::str("solar_noon_h"), v_transit.clone()), (Py::str("day_length_h"), py::sub(&v_sset.clone(), &v_rise.clone())?), (Py::str("sunset_hour_angle_deg"), v_ws.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_symmetrical_fault(mut v_network: Py, mut v_fault_bus: Py, mut v_Vf: Py, mut v_Xf: Py, mut v_S_base: Py, mut v_V_base: Py) -> R {
    let mut v_If: Py = Py::None;
    let mut v_Vd: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_edges: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zff: Py = Py::None;
    let t5071 = (f__fault_network(v_network.clone())?).items()?;
    if t5071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_edges = t5071[0].clone();
    v_nb = t5071[1].clone();
    let _ = f__need({ let t5075 = Py::Bool(matches!(v_fault_bus.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5075.truthy() { t5075 } else { { let t5072 = Py::Int(1i64); Py::Bool({ let t5073 = v_fault_bus.clone(); if py::le(&t5072, &t5073)? { { let t5074 = v_nb.clone(); if py::le(&t5073, &t5074)? { true } else { false } } } else { false } }) } } }, Py::str(&["the faulted bus must be one of 1..".to_string(), py::format(&v_nb.clone(), "")?].concat()))?;
    let _ = f__need({ let t5076 = Py::Float(0.5); Py::Bool({ let t5077 = v_Vf.clone(); if py::le(&t5076, &t5077)? { { let t5078 = Py::Float(1.5); if py::le(&t5077, &t5078)? { true } else { false } } } else { false } }) }, Py::str("the prefault voltage must lie between 0.5 and 1.5 pu"))?;
    let _ = f__nonneg(Py::str("the fault reactance"), Py::tuple(vec![v_Xf.clone()]))?;
    v_Y = { let mut out = vec![]; for t5084 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v__: Py; 
v__ = t5084;
out.push(py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_nb.clone())?); } Py::list(out) };
    for t5085 in py::iter(&(v_edges.clone()))? {
        let t5095 = (t5085).items()?;
        if t5095.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_i = t5095[0].clone();
        v_j = t5095[1].clone();
        v_x = t5095[2].clone();
        v_y = py::truediv(&Py::Int(1i64), &py::complex(&[Py::Int(0i64), v_x.clone()])?)?;
        if (v_i.clone()).truthy() {
            let t5086 = py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t5087 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t5086, &t5087, py::iadd(&py::getitem(&t5086, &t5087)?, &v_y.clone())?)?;
        }
        if (v_j.clone()).truthy() {
            let t5088 = py::getitem(&v_Y.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t5089 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
            py::setitem(&t5088, &t5089, py::iadd(&py::getitem(&t5088, &t5089)?, &v_y.clone())?)?;
        }
        if ({ let t5090 = v_i.clone(); if !t5090.truthy() { t5090 } else { v_j.clone() } }).truthy() {
            let t5091 = py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t5092 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
            py::setitem(&t5091, &t5092, py::sub(&py::getitem(&t5091, &t5092)?, &v_y.clone())?)?;
            let t5093 = py::getitem(&v_Y.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t5094 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t5093, &t5094, py::sub(&py::getitem(&t5093, &t5094)?, &v_y.clone())?)?;
        }
    }
    v_e = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_nb.clone())?;
    py::setitem(&v_e.clone(), &py::sub(&v_fault_bus.clone(), &Py::Int(1i64))?, py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?)?;
    v_z = f__gauss_solve(v_Y.clone(), v_e.clone())?;
    v_zff = py::getitem(&v_z.clone(), &py::sub(&v_fault_bus.clone(), &Py::Int(1i64))?)?;
    v_If = py::truediv(&v_Vf.clone(), &py::add(&v_zff.clone(), &py::complex(&[Py::Int(0i64), v_Xf.clone()])?)?)?;
    v_Vd = { let mut out = vec![]; for t5113 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t5113;
out.push(py::abs(&py::sub(&v_Vf.clone(), &py::mul(&py::getitem(&v_z.clone(), &v_k.clone())?, &v_If.clone())?)?)?); } Py::list(out) };
    v_out = Py::dict(vec![(Py::str("If"), py::abs(&v_If.clone())?), (Py::str("X_th"), py::imag(&v_zff.clone())?), (Py::str("V"), v_Vd.clone())]);
    if ({ let t5118 = { let t5114 = v_S_base.clone(); Py::Bool({ let t5115 = Py::None; if !py::is(&t5114, &t5115) { true } else { false } }) }; if !t5118.truthy() { t5118 } else { { let t5116 = v_V_base.clone(); Py::Bool({ let t5117 = Py::None; if !py::is(&t5116, &t5117) { true } else { false } }) } } }).truthy() {
        let _ = f__pos(Py::str("the base power and voltage"), Py::tuple(vec![v_S_base.clone(), v_V_base.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("If_A"), py::truediv(&py::mul(&py::abs(&v_If.clone())?, &v_S_base.clone())?, &py::mul(&c_SQ3()?, &v_V_base.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_synchronous_generator(mut v_VL: Py, mut v_IL: Py, mut v_pf: Py, mut v_Xs: Py, mut v_Ra: Py, mut v_pf_type: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let _ = f__pos(Py::str("the terminal voltage, the current and the synchronous reactance"), Py::tuple(vec![v_VL.clone(), v_IL.clone(), v_Xs.clone()]))?;
    let _ = f__nonneg(Py::str("the armature resistance"), Py::tuple(vec![v_Ra.clone()]))?;
    v_phi = f__pf_angle(v_pf.clone(), v_pf_type.clone())?;
    v_V = py::truediv(&v_VL.clone(), &c_SQ3()?)?;
    v_I = py::cmath("rect", &[v_IL.clone(), py::neg(&v_phi.clone())?])?;
    v_E = py::add(&v_V.clone(), &py::mul(&py::complex(&[v_Ra.clone(), v_Xs.clone()])?, &v_I.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_line"), py::mul(&c_SQ3()?, &py::abs(&v_E.clone())?)?), (Py::str("E_phase"), py::abs(&v_E.clone())?), (Py::str("delta_deg"), py::math1("degrees", &py::cmath("phase", &[v_E.clone()])?)?), (Py::str("regulation_pct"), py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&py::abs(&v_E.clone())?, &v_V.clone())?)?, &v_V.clone())?), (Py::str("P"), py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_V.clone())?, &v_IL.clone())?, &v_pf.clone())?), (Py::str("Q"), (if ({ let t5198 = v_pf.clone(); Py::Bool({ let t5199 = Py::Int(1i64); if py::eq(&t5198, &t5199) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_V.clone())?, &v_IL.clone())?, &py::math1("sin", &v_phi.clone())?)? }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_phase_power(mut v_VL: Py, mut v_IL: Py, mut v_pf: Py, mut v_connection: Py, mut v_pf_type: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_iph: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_vph: Py = Py::None;
    let _ = f__pos(Py::str("the line voltage and line current"), Py::tuple(vec![v_VL.clone(), v_IL.clone()]))?;
    v_phi = f__pf_angle(v_pf.clone(), v_pf_type.clone())?;
    v_S = py::mul(&py::mul(&c_SQ3()?, &v_VL.clone())?, &v_IL.clone())?;
    let t5322 = ((if ({ let t5320 = v_connection.clone(); Py::Bool({ let t5321 = Py::str("star"); if py::eq(&t5320, &t5321) { true } else { false } }) }).truthy() { Py::tuple(vec![py::truediv(&v_VL.clone(), &c_SQ3()?)?, v_IL.clone()]) } else { Py::tuple(vec![v_VL.clone(), py::truediv(&v_IL.clone(), &c_SQ3()?)?]) })).items()?;
    if t5322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vph = t5322[0].clone();
    v_iph = t5322[1].clone();
    v_Q = py::mul(&v_S.clone(), &py::math1("sin", &v_phi.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::mul(&v_S.clone(), &v_pf.clone())?), (Py::str("Q"), (if ({ let t5335 = v_pf.clone(); Py::Bool({ let t5336 = Py::Int(1i64); if py::eq(&t5335, &t5336) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_Q.clone() })), (Py::str("S"), v_S.clone()), (Py::str("V_phase"), v_vph.clone()), (Py::str("I_phase"), v_iph.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformer_loading(mut v_sn: Py, mut v_V_hv: Py, mut v_V_lv: Py, mut v_vk: Py, mut v_vkr: Py, mut v_P0: Py, mut v_i0: Py, mut v_P: Py, mut v_Q: Py) -> R {
    let mut v_S1: Py = Py::None;
    let mut v_Ym: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_i1: Py = Py::None;
    let mut v_i2: Py = Py::None;
    let mut v_loss: Py = Py::None;
    let mut v_rk: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_zk: Py = Py::None;
    let _ = f__pos(Py::str("the rating and the voltages"), Py::tuple(vec![v_sn.clone(), v_V_hv.clone(), v_V_lv.clone()]))?;
    let _ = f__need({ let t5375 = v_V_hv.clone(); Py::Bool({ let t5376 = v_V_lv.clone(); if py::gt(&t5375, &t5376)? { true } else { false } }) }, Py::str("the high-voltage side must have the higher rated voltage"))?;
    let _ = f__need({ let t5377 = Py::Int(0i64); Py::Bool({ let t5378 = v_vkr.clone(); if py::lt(&t5377, &t5378)? { { let t5379 = v_vk.clone(); if py::lt(&t5378, &t5379)? { { let t5380 = Py::Float(0.5); if py::lt(&t5379, &t5380)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("need 0 < vkr < vk (short-circuit voltages as fractions or percentages, vk below 50%)"))?;
    let _ = f__nonneg(Py::str("the no-load loss and current"), Py::tuple(vec![v_P0.clone(), v_i0.clone()]))?;
    let _ = f__need({ let t5393 = { let t5386 = py::truediv(&v_P0.clone(), &v_sn.clone())?; Py::Bool({ let t5387 = v_i0.clone(); if py::le(&t5386, &t5387)? { true } else { false } }) }; if t5393.truthy() { t5393 } else { { let t5392 = { let t5388 = v_i0.clone(); Py::Bool({ let t5389 = Py::Int(0i64); if py::eq(&t5388, &t5389) { true } else { false } }) }; if !t5392.truthy() { t5392 } else { { let t5390 = v_P0.clone(); Py::Bool({ let t5391 = Py::Int(0i64); if py::eq(&t5390, &t5391) { true } else { false } }) } } } } }, Py::str("the no-load current must cover the no-load loss (i0 >= P0/sn)"))?;
    let _ = f__nonneg(Py::str("the load real power"), Py::tuple(vec![v_P.clone()]))?;
    let t5394 = (Py::tuple(vec![v_vkr.clone(), v_vk.clone()])).items()?;
    if t5394.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rk = t5394[0].clone();
    v_zk = t5394[1].clone();
    v_Z = py::complex(&[v_rk.clone(), py::math1("sqrt", &py::sub(&py::mul(&v_zk.clone(), &v_zk.clone())?, &py::mul(&v_rk.clone(), &v_rk.clone())?)?)?])?;
    v_gm = py::truediv(&v_P0.clone(), &v_sn.clone())?;
    v_Ym = py::complex(&[v_gm.clone(), py::neg(&py::math1("sqrt", &py::minmax(&[py::sub(&py::mul(&v_i0.clone(), &v_i0.clone())?, &py::mul(&v_gm.clone(), &v_gm.clone())?)?, Py::Float(0.0)], true)?)?)?])?;
    let t5421 = (f__trafo_t_solve(v_Z.clone(), v_Ym.clone(), py::truediv(&py::complex(&[v_P.clone(), v_Q.clone()])?, &v_sn.clone())?)?).items()?;
    if t5421.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t5421[0].clone();
    v_i1 = t5421[1].clone();
    v_i2 = t5421[2].clone();
    v_S1 = py::mul(&py::mul(&Py::Int(1i64), &py::method(&v_i1.clone(), "conjugate", &[])?)?, &v_sn.clone())?;
    v_loss = py::sub(&v_S1.clone(), &py::complex(&[v_P.clone(), v_Q.clone()])?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_lv_actual"), py::mul(&py::abs(&v_v2.clone())?, &v_V_lv.clone())?), (Py::str("V_lv_pu"), py::abs(&v_v2.clone())?), (Py::str("delta_deg"), py::math1("degrees", &py::cmath("phase", &[v_v2.clone()])?)?), (Py::str("P_loss"), py::real(&v_loss.clone())?), (Py::str("Q_loss"), py::imag(&v_loss.clone())?), (Py::str("efficiency_pct"), (if ({ let t5434 = py::real(&v_S1.clone())?; Py::Bool({ let t5435 = Py::Int(0i64); if py::gt(&t5434, &t5435)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&Py::Int(100i64), &v_P.clone())?, &py::real(&v_S1.clone())?)? } else { Py::Float(0.0) })), (Py::str("loading_pct"), py::mul(&Py::Int(100i64), &py::minmax(&[py::abs(&v_i1.clone())?, py::abs(&v_i2.clone())?], true)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformer_max_efficiency(mut v_S: Py, mut v_P_core: Py, mut v_P_cu_fl: Py, mut v_pf: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the rating and the losses"), Py::tuple(vec![v_S.clone(), v_P_core.clone(), v_P_cu_fl.clone()]))?;
    let _ = f__need({ let t5474 = Py::Int(0i64); Py::Bool({ let t5475 = v_pf.clone(); if py::lt(&t5474, &t5475)? { { let t5476 = Py::Int(1i64); if py::le(&t5475, &t5476)? { true } else { false } } } else { false } }) }, Py::str("the power factor must lie in (0, 1]"))?;
    v_x = py::math1("sqrt", &py::truediv(&v_P_core.clone(), &v_P_cu_fl.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("load_fraction"), v_x.clone()), (Py::str("S_at_max"), py::mul(&v_x.clone(), &v_S.clone())?), (Py::str("efficiency_max_pct"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(100i64), &v_x.clone())?, &v_S.clone())?, &v_pf.clone())?, &py::add(&py::mul(&py::mul(&v_x.clone(), &v_S.clone())?, &v_pf.clone())?, &py::mul(&Py::Int(2i64), &v_P_core.clone())?)?)?), (Py::str("efficiency_full_load_pct"), py::truediv(&py::mul(&py::mul(&Py::Int(100i64), &v_S.clone())?, &v_pf.clone())?, &py::add(&py::add(&py::mul(&v_S.clone(), &v_pf.clone())?, &v_P_core.clone())?, &v_P_cu_fl.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformer_performance(mut v_S: Py, mut v_V1: Py, mut v_V2: Py, mut v_Req: Py, mut v_Xeq: Py, mut v_Rc: Py, mut v_Xm: Py, mut v_load: Py, mut v_pf: Py, mut v_pf_type: Py) -> R {
    let mut v_I1: Py = Py::None;
    let mut v_I2p: Py = Py::None;
    let mut v_Ish: Py = Py::None;
    let mut v_Pcore: Py = Py::None;
    let mut v_Pcu: Py = Py::None;
    let mut v_Pout: Py = Py::None;
    let mut v_V1v: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let _ = f__pos(Py::str("the rating and the rated voltages"), Py::tuple(vec![v_S.clone(), v_V1.clone(), v_V2.clone()]))?;
    let _ = f__nonneg(Py::str("the series resistance and reactance"), Py::tuple(vec![v_Req.clone(), v_Xeq.clone()]))?;
    let _ = f__pos(Py::str("the core-loss resistance and the magnetizing reactance"), Py::tuple(vec![v_Rc.clone(), v_Xm.clone()]))?;
    let _ = f__need({ let t5532 = Py::Int(0i64); Py::Bool({ let t5533 = v_load.clone(); if py::lt(&t5532, &t5533)? { { let t5534 = Py::Int(2i64); if py::le(&t5533, &t5534)? { true } else { false } } } else { false } }) }, Py::str("the load must lie between 0 and 2 times the rating (a fraction, or a percentage)"))?;
    v_phi = f__pf_angle(v_pf.clone(), v_pf_type.clone())?;
    v_a = py::truediv(&v_V1.clone(), &v_V2.clone())?;
    v_I2p = py::cmath("rect", &[py::truediv(&py::truediv(&py::mul(&v_load.clone(), &v_S.clone())?, &v_V2.clone())?, &v_a.clone())?, py::neg(&v_phi.clone())?])?;
    v_V1v = py::add(&py::mul(&v_a.clone(), &v_V2.clone())?, &py::mul(&v_I2p.clone(), &py::complex(&[v_Req.clone(), v_Xeq.clone()])?)?)?;
    v_Ish = py::mul(&v_V1v.clone(), &py::add(&py::truediv(&Py::Int(1i64), &v_Rc.clone())?, &py::truediv(&Py::Int(1i64), &py::complex(&[Py::Int(0i64), v_Xm.clone()])?)?)?)?;
    v_I1 = py::add(&v_I2p.clone(), &v_Ish.clone())?;
    v_Pout = py::mul(&py::mul(&v_load.clone(), &v_S.clone())?, &v_pf.clone())?;
    v_Pcu = py::mul(&py::pow(&py::abs(&v_I2p.clone())?, &Py::Int(2i64))?, &v_Req.clone())?;
    v_Pcore = py::truediv(&py::pow(&py::abs(&v_V1v.clone())?, &Py::Int(2i64))?, &v_Rc.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V1_required"), py::abs(&v_V1v.clone())?), (Py::str("regulation_pct"), py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&py::truediv(&py::abs(&v_V1v.clone())?, &v_a.clone())?, &v_V2.clone())?)?, &v_V2.clone())?), (Py::str("I1"), py::abs(&v_I1.clone())?), (Py::str("pf_in"), py::math1("cos", &py::sub(&py::cmath("phase", &[v_V1v.clone()])?, &py::cmath("phase", &[v_I1.clone()])?)?)?), (Py::str("P_out"), v_Pout.clone()), (Py::str("P_cu"), v_Pcu.clone()), (Py::str("P_core"), v_Pcore.clone()), (Py::str("efficiency_pct"), py::truediv(&py::mul(&Py::Int(100i64), &v_Pout.clone())?, &py::add(&py::add(&v_Pout.clone(), &v_Pcu.clone())?, &v_Pcore.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformer_tests(mut v_Voc: Py, mut v_Ioc: Py, mut v_Poc: Py, mut v_Vsc: Py, mut v_Isc: Py, mut v_Psc: Py) -> R {
    let mut v_Bm: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let _ = f__pos(Py::str("the open-circuit readings"), Py::tuple(vec![v_Voc.clone(), v_Ioc.clone(), v_Poc.clone()]))?;
    let _ = f__pos(Py::str("the short-circuit readings"), Py::tuple(vec![v_Vsc.clone(), v_Isc.clone(), v_Psc.clone()]))?;
    let _ = f__need({ let t5591 = v_Poc.clone(); Py::Bool({ let t5592 = py::mul(&v_Voc.clone(), &v_Ioc.clone())?; if py::lt(&t5591, &t5592)? { true } else { false } }) }, Py::str("the open-circuit power must be below V I (power factor below 1)"))?;
    let _ = f__need({ let t5593 = v_Psc.clone(); Py::Bool({ let t5594 = py::mul(&v_Vsc.clone(), &v_Isc.clone())?; if py::lt(&t5593, &t5594)? { true } else { false } }) }, Py::str("the short-circuit power must be below V I (power factor below 1)"))?;
    let t5595 = (Py::tuple(vec![py::truediv(&v_Poc.clone(), &py::pow(&v_Voc.clone(), &Py::Int(2i64))?)?, py::truediv(&v_Ioc.clone(), &v_Voc.clone())?])).items()?;
    if t5595.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_G = t5595[0].clone();
    v_Y = t5595[1].clone();
    v_Bm = py::math1("sqrt", &py::sub(&py::mul(&v_Y.clone(), &v_Y.clone())?, &py::mul(&v_G.clone(), &v_G.clone())?)?)?;
    let t5596 = (Py::tuple(vec![py::truediv(&v_Vsc.clone(), &v_Isc.clone())?, py::truediv(&v_Psc.clone(), &py::pow(&v_Isc.clone(), &Py::Int(2i64))?)?])).items()?;
    if t5596.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z = t5596[0].clone();
    v_R = t5596[1].clone();
    v_X = py::math1("sqrt", &py::sub(&py::mul(&v_Z.clone(), &v_Z.clone())?, &py::mul(&v_R.clone(), &v_R.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Rc"), py::truediv(&Py::Int(1i64), &v_G.clone())?), (Py::str("Xm"), py::truediv(&Py::Int(1i64), &v_Bm.clone())?), (Py::str("Req"), v_R.clone()), (Py::str("Xeq"), v_X.clone()), (Py::str("Zeq"), v_Z.clone()), (Py::str("pf_oc"), py::truediv(&v_Poc.clone(), &py::mul(&v_Voc.clone(), &v_Ioc.clone())?)?), (Py::str("pf_sc"), py::truediv(&v_Psc.clone(), &py::mul(&v_Vsc.clone(), &v_Isc.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_bus_load_flow(mut v_Vs: Py, mut v_R: Py, mut v_X: Py, mut v_P: Py, mut v_Q: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_Sl: Py = Py::None;
    let mut v_V2: Py = Py::None;
    let mut v_Vr: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_re: Py = Py::None;
    let _ = f__pos(Py::str("the sending-end voltage"), Py::tuple(vec![v_Vs.clone()]))?;
    let _ = f__nonneg(Py::str("the line resistance and reactance"), Py::tuple(vec![v_R.clone(), v_X.clone()]))?;
    let _ = f__need({ let t5652 = py::add(&v_R.clone(), &v_X.clone())?; Py::Bool({ let t5653 = Py::Int(0i64); if py::gt(&t5652, &t5653)? { true } else { false } }) }, Py::str("the line impedance must not be zero"))?;
    let _ = f__nonneg(Py::str("the load real power"), Py::tuple(vec![v_P.clone()]))?;
    v_a = py::sub(&py::mul(&Py::Int(2i64), &py::add(&py::mul(&v_P.clone(), &v_R.clone())?, &py::mul(&v_Q.clone(), &v_X.clone())?)?)?, &py::mul(&v_Vs.clone(), &v_Vs.clone())?)?;
    v_disc = py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &py::add(&py::mul(&v_P.clone(), &v_P.clone())?, &py::mul(&v_Q.clone(), &v_Q.clone())?)?)?, &py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_X.clone(), &v_X.clone())?)?)?)?;
    let _ = f__need({ let t5654 = v_disc.clone(); Py::Bool({ let t5655 = Py::Int(0i64); if py::ge(&t5654, &t5655)? { true } else { false } }) }, Py::str("the load exceeds what the line can deliver: the load flow has no solution"))?;
    v_V2 = py::truediv(&py::add(&py::neg(&v_a.clone())?, &py::math1("sqrt", &v_disc.clone())?)?, &Py::Int(2i64))?;
    v_Vr = py::math1("sqrt", &v_V2.clone())?;
    let t5656 = (Py::tuple(vec![py::add(&v_Vr.clone(), &py::truediv(&py::add(&py::mul(&v_P.clone(), &v_R.clone())?, &py::mul(&v_Q.clone(), &v_X.clone())?)?, &v_Vr.clone())?)?, py::truediv(&py::sub(&py::mul(&v_P.clone(), &v_X.clone())?, &py::mul(&v_Q.clone(), &v_R.clone())?)?, &v_Vr.clone())?])).items()?;
    if t5656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re = t5656[0].clone();
    v_im = t5656[1].clone();
    v_I = py::truediv(&py::hypot(&[v_P.clone(), v_Q.clone()])?, &py::mul(&c_SQ3()?, &v_Vr.clone())?)?;
    v_Sl = py::truediv(&py::add(&py::mul(&v_P.clone(), &v_P.clone())?, &py::mul(&v_Q.clone(), &v_Q.clone())?)?, &v_V2.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vr"), v_Vr.clone()), (Py::str("delta_deg"), py::neg(&py::math1("degrees", &py::atan2(&v_im.clone(), &v_re.clone())?)?)?), (Py::str("I"), v_I.clone()), (Py::str("P_loss"), py::mul(&v_Sl.clone(), &v_R.clone())?), (Py::str("Q_loss"), py::mul(&v_Sl.clone(), &v_X.clone())?), (Py::str("voltage_drop_pct"), py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&v_Vs.clone(), &v_Vr.clone())?)?, &v_Vs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unsym_fault(mut v_kind: Py, mut v_X1: Py, mut v_X2: Py, mut v_X0: Py, mut v_Rf: Py, mut v_Xf: Py, mut v_Vf: Py) -> R {
    let mut v_I0: Py = Py::None;
    let mut v_I1: Py = Py::None;
    let mut v_I2: Py = Py::None;
    let mut v_Ia: Py = Py::None;
    let mut v_Ib: Py = Py::None;
    let mut v_Ic: Py = Py::None;
    let mut v_V0: Py = Py::None;
    let mut v_V1: Py = Py::None;
    let mut v_V2: Py = Py::None;
    let mut v_Va: Py = Py::None;
    let mut v_Vb: Py = Py::None;
    let mut v_Vc: Py = Py::None;
    let mut v_Z0: Py = Py::None;
    let mut v_Z1: Py = Py::None;
    let mut v_Z2: Py = Py::None;
    let mut v_Zf: Py = Py::None;
    let mut v_Zp: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__seq_check(v_X1.clone(), v_X2.clone(), v_X0.clone(), v_Rf.clone(), v_Xf.clone(), v_Vf.clone())?;
    let t4581 = (Py::tuple(vec![py::complex(&[Py::Int(0i64), v_X1.clone()])?, py::complex(&[Py::Int(0i64), v_X2.clone()])?, py::complex(&[Py::Int(0i64), v_X0.clone()])?, py::complex(&[v_Rf.clone(), v_Xf.clone()])?])).items()?;
    if t4581.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Z1 = t4581[0].clone();
    v_Z2 = t4581[1].clone();
    v_Z0 = t4581[2].clone();
    v_Zf = t4581[3].clone();
    if ({ let t4582 = v_kind.clone(); Py::Bool({ let t4583 = Py::str("slg"); if py::eq(&t4582, &t4583) { true } else { false } }) }).truthy() {
        v_I1 = py::truediv(&v_Vf.clone(), &py::add(&py::add(&py::add(&v_Z1.clone(), &v_Z2.clone())?, &v_Z0.clone())?, &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?;
        let t4584 = (Py::tuple(vec![v_I1.clone(), v_I1.clone()])).items()?;
        if t4584.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_I2 = t4584[0].clone();
        v_I0 = t4584[1].clone();
    } else {
        if ({ let t4585 = v_kind.clone(); Py::Bool({ let t4586 = Py::str("ll"); if py::eq(&t4585, &t4586) { true } else { false } }) }).truthy() {
            v_I1 = py::truediv(&v_Vf.clone(), &py::add(&py::add(&v_Z1.clone(), &v_Z2.clone())?, &v_Zf.clone())?)?;
            let t4587 = (Py::tuple(vec![py::neg(&v_I1.clone())?, Py::Complex(0.0, 0.0)])).items()?;
            if t4587.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_I2 = t4587[0].clone();
            v_I0 = t4587[1].clone();
        } else {
            v_Zp = py::truediv(&py::mul(&v_Z2.clone(), &py::add(&v_Z0.clone(), &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?, &py::add(&py::add(&v_Z2.clone(), &v_Z0.clone())?, &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?;
            v_I1 = py::truediv(&v_Vf.clone(), &py::add(&v_Z1.clone(), &v_Zp.clone())?)?;
            v_I2 = py::truediv(&py::mul(&py::neg(&v_I1.clone())?, &py::add(&v_Z0.clone(), &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?, &py::add(&py::add(&v_Z2.clone(), &v_Z0.clone())?, &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?;
            v_I0 = py::truediv(&py::mul(&py::neg(&v_I1.clone())?, &v_Z2.clone())?, &py::add(&py::add(&v_Z2.clone(), &v_Z0.clone())?, &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?;
        }
    }
    let t4588 = (Py::tuple(vec![py::sub(&v_Vf.clone(), &py::mul(&v_Z1.clone(), &v_I1.clone())?)?, py::mul(&py::neg(&v_Z2.clone())?, &v_I2.clone())?, py::mul(&py::neg(&v_Z0.clone())?, &v_I0.clone())?])).items()?;
    if t4588.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V1 = t4588[0].clone();
    v_V2 = t4588[1].clone();
    v_V0 = t4588[2].clone();
    let t4589 = (f__phase_from_seq(v_I0.clone(), v_I1.clone(), v_I2.clone())?).items()?;
    if t4589.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Ia = t4589[0].clone();
    v_Ib = t4589[1].clone();
    v_Ic = t4589[2].clone();
    let t4590 = (f__phase_from_seq(v_V0.clone(), v_V1.clone(), v_V2.clone())?).items()?;
    if t4590.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Va = t4590[0].clone();
    v_Vb = t4590[1].clone();
    v_Vc = t4590[2].clone();
    v_s = py::abs(&v_I1.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Ia"), f__snap0(py::abs(&v_Ia.clone())?, v_s.clone())?), (Py::str("Ib"), f__snap0(py::abs(&v_Ib.clone())?, v_s.clone())?), (Py::str("Ic"), f__snap0(py::abs(&v_Ic.clone())?, v_s.clone())?), (Py::str("In"), f__snap0(py::abs(&py::mul(&Py::Int(3i64), &v_I0.clone())?)?, v_s.clone())?), (Py::str("I1"), py::abs(&v_I1.clone())?), (Py::str("I2"), f__snap0(py::abs(&v_I2.clone())?, v_s.clone())?), (Py::str("I0"), f__snap0(py::abs(&v_I0.clone())?, v_s.clone())?), (Py::str("Va"), f__snap0(py::abs(&v_Va.clone())?, v_Vf.clone())?), (Py::str("Vb"), f__snap0(py::abs(&v_Vb.clone())?, v_Vf.clone())?), (Py::str("Vc"), f__snap0(py::abs(&v_Vc.clone())?, v_Vf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_aep(mut v_speeds: Py, mut v_powers: Py, mut v_k: Py, mut v_c: Py, mut v_hours: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_base: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_mv: Py = Py::None;
    let mut v_pa: Py = Py::None;
    let mut v_pm: Py = Py::None;
    let mut v_pr: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_total: Py = Py::None;
    let _ = f__check_curve(v_speeds.clone(), v_powers.clone())?;
    let _ = f__check_weibull(v_k.clone(), v_c.clone())?;
    let _ = f__pos(Py::str("the hours per year"), Py::tuple(vec![v_hours.clone()]))?;
    v_F = { let v_c = v_c.clone(); let v_hours = v_hours.clone(); let v_k = v_k.clone(); let v_powers = v_powers.clone(); let v_speeds = v_speeds.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::neg(&py::math1("expm1", &py::neg(&py::pow(&py::truediv(&v_v.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_total = ax_engine::pymp::call("mpf", &[Py::Int(0i64)], &[])?;
    v_s = py::add(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_k.clone())?)?;
    for t5725 in py::range_iter(&[(py::sub(&py::len(&v_speeds.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t5725;
        let t5726 = (Py::tuple(vec![py::getitem(&v_speeds.clone(), &v_i.clone())?, py::getitem(&v_speeds.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])).items()?;
        if t5726.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t5726[0].clone();
        v_b = t5726[1].clone();
        v_slope = py::truediv(&py::sub(&py::getitem(&v_powers.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_powers.clone(), &v_i.clone())?)?, &py::sub(&v_b.clone(), &v_a.clone())?)?;
        v_base = py::sub(&py::getitem(&v_powers.clone(), &v_i.clone())?, &py::mul(&v_slope.clone(), &v_a.clone())?)?;
        v_pa = py::sub(&py::call(&v_F.clone(), &[v_b.clone()])?, &py::call(&v_F.clone(), &[v_a.clone()])?)?;
        v_mv = py::mul(&py::mul(&v_c.clone(), &py::math1("gamma", &v_s.clone())?)?, &ax_engine::pymp::call("gammainc", &[v_s.clone(), py::pow(&py::truediv(&v_a.clone(), &v_c.clone())?, &v_k.clone())?, py::pow(&py::truediv(&v_b.clone(), &v_c.clone())?, &v_k.clone())?], &[(String::from("regularized"), Py::Bool(true))])?)?;
        v_total = py::iadd(&v_total, &py::add(&py::mul(&v_base.clone(), &v_pa.clone())?, &py::mul(&v_slope.clone(), &v_mv.clone())?)?)?;
    }
    v_pm = py::float(&v_total.clone())?;
    v_pr = py::minmax(&[v_powers.clone()], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("mean_power_kW"), v_pm.clone()), (Py::str("AEP_MWh"), py::truediv(&py::mul(&v_pm.clone(), &v_hours.clone())?, &Py::Float(1000.0))?), (Py::str("capacity_factor"), (if ({ let t5739 = v_pr.clone(); Py::Bool({ let t5740 = Py::Int(0i64); if py::gt(&t5739, &t5740)? { true } else { false } }) }).truthy() { py::truediv(&v_pm.clone(), &v_pr.clone())? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_wind(mut v_k: Py, mut v_c: Py, mut v_rho: Py) -> R {
    let mut v_g1: Py = Py::None;
    let mut v_g3: Py = Py::None;
    let _ = f__check_weibull(v_k.clone(), v_c.clone())?;
    let _ = f__pos(Py::str("the air density"), Py::tuple(vec![v_rho.clone()]))?;
    let t5783 = (Py::tuple(vec![py::math1("gamma", &py::add(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_k.clone())?)?)?, py::math1("gamma", &py::add(&Py::Int(1i64), &py::truediv(&Py::Int(3i64), &v_k.clone())?)?)?])).items()?;
    if t5783.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g1 = t5783[0].clone();
    v_g3 = t5783[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("v_mean"), py::mul(&v_c.clone(), &v_g1.clone())?), (Py::str("power_density"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::pow(&v_c.clone(), &Py::Int(3i64))?)?, &v_g3.clone())?), (Py::str("energy_pattern_factor"), py::truediv(&v_g3.clone(), &py::pow(&v_g1.clone(), &Py::Int(3i64))?)?), (Py::str("v_most_probable"), py::mul(&v_c.clone(), &py::pow(&py::truediv(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_k.clone())?, &py::truediv(&Py::Int(1i64), &v_k.clone())?)?)?), (Py::str("v_max_energy"), py::mul(&v_c.clone(), &py::pow(&py::truediv(&py::add(&v_k.clone(), &Py::Int(2i64))?, &v_k.clone())?, &py::truediv(&Py::Int(1i64), &v_k.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_power(mut v_rho: Py, mut v_D: Py, mut v_v: Py, mut v_cp: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pw: Py = Py::None;
    let _ = f__pos(Py::str("the air density, rotor diameter and wind speed"), Py::tuple(vec![v_rho.clone(), v_D.clone(), v_v.clone()]))?;
    v_A = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?;
    v_pw = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_A.clone())?, &py::pow(&v_v.clone(), &Py::Int(3i64))?)?;
    v_out = Py::dict(vec![(Py::str("P_wind"), v_pw.clone()), (Py::str("P_betz"), py::mul(&c_BETZ()?, &v_pw.clone())?), (Py::str("power_density"), py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::pow(&v_v.clone(), &Py::Int(3i64))?)?), (Py::str("area"), v_A.clone())]);
    if ({ let t5834 = v_cp.clone(); Py::Bool({ let t5835 = Py::None; if !py::is(&t5834, &t5835) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t5836 = Py::Int(0i64); Py::Bool({ let t5837 = v_cp.clone(); if py::lt(&t5836, &t5837)? { { let t5838 = py::add(&c_BETZ()?, &Py::Float(1e-12))?; if py::le(&t5837, &t5838)? { true } else { false } } } else { false } }) }, Py::str("the power coefficient must lie between 0 and the Betz limit 16/27 = 0.593"))?;
        py::setitem(&v_out.clone(), &Py::str("P"), py::mul(&v_cp.clone(), &v_pw.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_shear_log(mut v_v1: Py, mut v_h1: Py, mut v_h2: Py, mut v_z0: Py, mut v_d: Py) -> R {
    let _ = f__pos(Py::str("the wind speed, the heights and the roughness length"), Py::tuple(vec![v_v1.clone(), v_h1.clone(), v_h2.clone(), v_z0.clone()]))?;
    let _ = f__nonneg(Py::str("the obstacle height"), Py::tuple(vec![v_d.clone()]))?;
    let _ = f__need({ let t5899 = { let t5895 = py::sub(&v_h1.clone(), &py::mul(&Py::Float(0.7), &v_d.clone())?)?; Py::Bool({ let t5896 = v_z0.clone(); if py::gt(&t5895, &t5896)? { true } else { false } }) }; if !t5899.truthy() { t5899 } else { { let t5897 = py::sub(&v_h2.clone(), &py::mul(&Py::Float(0.7), &v_d.clone())?)?; Py::Bool({ let t5898 = v_z0.clone(); if py::gt(&t5897, &t5898)? { true } else { false } }) } } }, Py::str("both heights must exceed the displacement height 0.7 d plus the roughness length"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("v2"), py::truediv(&py::mul(&v_v1.clone(), &py::math1("log", &py::truediv(&py::sub(&v_h2.clone(), &py::mul(&Py::Float(0.7), &v_d.clone())?)?, &v_z0.clone())?)?)?, &py::math1("log", &py::truediv(&py::sub(&v_h1.clone(), &py::mul(&Py::Float(0.7), &v_d.clone())?)?, &v_z0.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_shear_power_law(mut v_v1: Py, mut v_h1: Py, mut v_h2: Py, mut v_alpha: Py) -> R {
    let _ = f__pos(Py::str("the wind speed and the heights"), Py::tuple(vec![v_v1.clone(), v_h1.clone(), v_h2.clone()]))?;
    let _ = f__need({ let t5950 = Py::Int(0i64); Py::Bool({ let t5951 = v_alpha.clone(); if py::le(&t5950, &t5951)? { { let t5952 = Py::Int(1i64); if py::le(&t5951, &t5952)? { true } else { false } } } else { false } }) }, Py::str("the shear exponent alpha must lie between 0 and 1"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("v2"), py::mul(&v_v1.clone(), &py::pow(&py::truediv(&v_h2.clone(), &v_h1.clone())?, &v_alpha.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1054(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1056: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1055 = v_inputs.clone(); f_desoto_parameters(py::getitem(&t1055, &Py::str("G"))?, py::getitem(&t1055, &Py::str("T_cell"))?, py::getitem(&t1055, &Py::str("alpha_sc"))?, py::getitem(&t1055, &Py::str("a_ref"))?, py::getitem(&t1055, &Py::str("IL_ref"))?, py::getitem(&t1055, &Py::str("I0_ref"))?, py::getitem(&t1055, &Py::str("Rsh_ref"))?, py::getitem(&t1055, &Py::str("Rs"))?, py::method(&t1055, "get", &[Py::str("EgRef"), Py::Float(1.121)])?, py::method(&t1055, "get", &[Py::str("dEgdT"), py::neg(&Py::Float(0.0002677))?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1056 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1057 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1086 = (t1057).items()?;
        if t1086.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1086[0].clone();
        v_v = t1086[1].clone();
        if ({ let t1058 = v_k.clone(); Py::Bool({ let t1059 = v_outputs.clone(); if !py::contains(&t1059, &t1058)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1085 = { let t1076 = v_d.clone(); Py::Bool({ let t1077 = Py::Int(0i64); if py::gt(&t1076, &t1077)? { true } else { false } }) }; if !t1085.truthy() { t1085 } else { { let t1084 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1084.truthy() { t1084 } else { { let t1083 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1083.truthy() { t1083 } else { { let t1082 = { let t1078 = py::abs(&v_v.clone())?; Py::Bool({ let t1079 = Py::Float(1e-15); if py::lt(&t1078, &t1079)? { true } else { false } }) }; if !t1082.truthy() { t1082 } else { { let t1080 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1081 = Py::Float(1e-15); if py::lt(&t1080, &t1081)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1128(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1130: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1129 = v_inputs.clone(); f_disc(py::getitem(&t1129, &Py::str("ghi"))?, py::getitem(&t1129, &Py::str("zenith"))?, py::getitem(&t1129, &Py::str("n"))?, py::method(&t1129, "get", &[Py::str("pressure"), Py::Float(101325.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1130 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1131 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1160 = (t1131).items()?;
        if t1160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1160[0].clone();
        v_v = t1160[1].clone();
        if ({ let t1132 = v_k.clone(); Py::Bool({ let t1133 = v_outputs.clone(); if !py::contains(&t1133, &t1132)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1159 = { let t1150 = v_d.clone(); Py::Bool({ let t1151 = Py::Int(0i64); if py::gt(&t1150, &t1151)? { true } else { false } }) }; if !t1159.truthy() { t1159 } else { { let t1158 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1158.truthy() { t1158 } else { { let t1157 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1157.truthy() { t1157 } else { { let t1156 = { let t1152 = py::abs(&v_v.clone())?; Py::Bool({ let t1153 = Py::Float(1e-15); if py::lt(&t1152, &t1153)? { true } else { false } }) }; if !t1156.truthy() { t1156 } else { { let t1154 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1155 = Py::Float(1e-15); if py::lt(&t1154, &t1155)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1161(mut v_X1: Py, mut v_X2: Py, mut v_X0: Py, mut v_Rf: Py, mut v_Xf: Py, mut v_Vf: Py) -> R {
    return Ok(f_unsym_fault(Py::str("dlg"), v_X1.clone(), v_X2.clone(), v_X0.clone(), v_Rf.clone(), v_Xf.clone(), v_Vf.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1199(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1201: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1200 = v_inputs.clone(); s_1161(py::getitem(&t1200, &Py::str("X1"))?, py::getitem(&t1200, &Py::str("X2"))?, py::getitem(&t1200, &Py::str("X0"))?, py::method(&t1200, "get", &[Py::str("Rf"), Py::Float(0.0)])?, py::method(&t1200, "get", &[Py::str("Xf"), Py::Float(0.0)])?, py::method(&t1200, "get", &[Py::str("Vf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1201 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1202 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1231 = (t1202).items()?;
        if t1231.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1231[0].clone();
        v_v = t1231[1].clone();
        if ({ let t1203 = v_k.clone(); Py::Bool({ let t1204 = v_outputs.clone(); if !py::contains(&t1204, &t1203)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1230 = { let t1221 = v_d.clone(); Py::Bool({ let t1222 = Py::Int(0i64); if py::gt(&t1221, &t1222)? { true } else { false } }) }; if !t1230.truthy() { t1230 } else { { let t1229 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1229.truthy() { t1229 } else { { let t1228 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1228.truthy() { t1228 } else { { let t1227 = { let t1223 = py::abs(&v_v.clone())?; Py::Bool({ let t1224 = Py::Float(1e-15); if py::lt(&t1223, &t1224)? { true } else { false } }) }; if !t1227.truthy() { t1227 } else { { let t1225 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1226 = Py::Float(1e-15); if py::lt(&t1225, &t1226)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1270(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1272: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1271 = v_inputs.clone(); f_droop_sharing(py::getitem(&t1271, &Py::str("R"))?, py::getitem(&t1271, &Py::str("ratings"))?, py::getitem(&t1271, &Py::str("dP"))?, py::getitem(&t1271, &Py::str("f0"))?, py::method(&t1271, "get", &[Py::str("D"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1272 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1273 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1302 = (t1273).items()?;
        if t1302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1302[0].clone();
        v_v = t1302[1].clone();
        if ({ let t1274 = v_k.clone(); Py::Bool({ let t1275 = v_outputs.clone(); if !py::contains(&t1275, &t1274)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1301 = { let t1292 = v_d.clone(); Py::Bool({ let t1293 = Py::Int(0i64); if py::gt(&t1292, &t1293)? { true } else { false } }) }; if !t1301.truthy() { t1301 } else { { let t1300 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1300.truthy() { t1300 } else { { let t1299 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1299.truthy() { t1299 } else { { let t1298 = { let t1294 = py::abs(&v_v.clone())?; Py::Bool({ let t1295 = Py::Float(1e-15); if py::lt(&t1294, &t1295)? { true } else { false } }) }; if !t1298.truthy() { t1298 } else { { let t1296 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1297 = Py::Float(1e-15); if py::lt(&t1296, &t1297)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1372(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1374: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1373 = v_inputs.clone(); f_economic_dispatch(py::getitem(&t1373, &Py::str("costs"))?, py::getitem(&t1373, &Py::str("demand"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1374 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1375 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1404 = (t1375).items()?;
        if t1404.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1404[0].clone();
        v_v = t1404[1].clone();
        if ({ let t1376 = v_k.clone(); Py::Bool({ let t1377 = v_outputs.clone(); if !py::contains(&t1377, &t1376)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1403 = { let t1394 = v_d.clone(); Py::Bool({ let t1395 = Py::Int(0i64); if py::gt(&t1394, &t1395)? { true } else { false } }) }; if !t1403.truthy() { t1403 } else { { let t1402 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1402.truthy() { t1402 } else { { let t1401 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1401.truthy() { t1401 } else { { let t1400 = { let t1396 = py::abs(&v_v.clone())?; Py::Bool({ let t1397 = Py::Float(1e-15); if py::lt(&t1396, &t1397)? { true } else { false } }) }; if !t1400.truthy() { t1400 } else { { let t1398 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1399 = Py::Float(1e-15); if py::lt(&t1398, &t1399)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1431(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1433: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1432 = v_inputs.clone(); f_electrolyser(py::getitem(&t1432, &Py::str("I"))?, py::getitem(&t1432, &Py::str("N"))?, py::getitem(&t1432, &Py::str("V_cell"))?, py::method(&t1432, "get", &[Py::str("faradaic"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1433 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1434 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1463 = (t1434).items()?;
        if t1463.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1463[0].clone();
        v_v = t1463[1].clone();
        if ({ let t1435 = v_k.clone(); Py::Bool({ let t1436 = v_outputs.clone(); if !py::contains(&t1436, &t1435)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1462 = { let t1453 = v_d.clone(); Py::Bool({ let t1454 = Py::Int(0i64); if py::gt(&t1453, &t1454)? { true } else { false } }) }; if !t1462.truthy() { t1462 } else { { let t1461 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1461.truthy() { t1461 } else { { let t1460 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1460.truthy() { t1460 } else { { let t1459 = { let t1455 = py::abs(&v_v.clone())?; Py::Bool({ let t1456 = Py::Float(1e-15); if py::lt(&t1455, &t1456)? { true } else { false } }) }; if !t1459.truthy() { t1459 } else { { let t1457 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1458 = Py::Float(1e-15); if py::lt(&t1457, &t1458)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1464(mut v_n: Py, mut v_method: Py) -> R {
    return Ok(f_equation_of_time(v_n.clone(), v_method.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1485: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1484 = v_inputs.clone(); s_1464(py::getitem(&t1484, &Py::str("n"))?, py::method(&t1484, "get", &[Py::str("method"), Py::str("spencer")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1485 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1486 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1515 = (t1486).items()?;
        if t1515.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1515[0].clone();
        v_v = t1515[1].clone();
        if ({ let t1487 = v_k.clone(); Py::Bool({ let t1488 = v_outputs.clone(); if !py::contains(&t1488, &t1487)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1514 = { let t1505 = v_d.clone(); Py::Bool({ let t1506 = Py::Int(0i64); if py::gt(&t1505, &t1506)? { true } else { false } }) }; if !t1514.truthy() { t1514 } else { { let t1513 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1513.truthy() { t1513 } else { { let t1512 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1512.truthy() { t1512 } else { { let t1511 = { let t1507 = py::abs(&v_v.clone())?; Py::Bool({ let t1508 = Py::Float(1e-15); if py::lt(&t1507, &t1508)? { true } else { false } }) }; if !t1511.truthy() { t1511 } else { { let t1509 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1510 = Py::Float(1e-15); if py::lt(&t1509, &t1510)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_152(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t154: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t153 = v_inputs.clone(); f_angle_of_incidence(py::getitem(&t153, &Py::str("tilt"))?, py::getitem(&t153, &Py::str("surface_azimuth"))?, py::getitem(&t153, &Py::str("zenith"))?, py::getitem(&t153, &Py::str("azimuth"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t154 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t155 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t184 = (t155).items()?;
        if t184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t184[0].clone();
        v_v = t184[1].clone();
        if ({ let t156 = v_k.clone(); Py::Bool({ let t157 = v_outputs.clone(); if !py::contains(&t157, &t156)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t183 = { let t174 = v_d.clone(); Py::Bool({ let t175 = Py::Int(0i64); if py::gt(&t174, &t175)? { true } else { false } }) }; if !t183.truthy() { t183 } else { { let t182 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t182.truthy() { t182 } else { { let t181 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t181.truthy() { t181 } else { { let t180 = { let t176 = py::abs(&v_v.clone())?; Py::Bool({ let t177 = Py::Float(1e-15); if py::lt(&t176, &t177)? { true } else { false } }) }; if !t180.truthy() { t180 } else { { let t178 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t179 = Py::Float(1e-15); if py::lt(&t178, &t179)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1545(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1547: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1546 = v_inputs.clone(); f_erbs(py::getitem(&t1546, &Py::str("ghi"))?, py::getitem(&t1546, &Py::str("zenith"))?, py::getitem(&t1546, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1547 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1548 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1577 = (t1548).items()?;
        if t1577.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1577[0].clone();
        v_v = t1577[1].clone();
        if ({ let t1549 = v_k.clone(); Py::Bool({ let t1550 = v_outputs.clone(); if !py::contains(&t1550, &t1549)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1576 = { let t1567 = v_d.clone(); Py::Bool({ let t1568 = Py::Int(0i64); if py::gt(&t1567, &t1568)? { true } else { false } }) }; if !t1576.truthy() { t1576 } else { { let t1575 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1575.truthy() { t1575 } else { { let t1574 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1574.truthy() { t1574 } else { { let t1573 = { let t1569 = py::abs(&v_v.clone())?; Py::Bool({ let t1570 = Py::Float(1e-15); if py::lt(&t1569, &t1570)? { true } else { false } }) }; if !t1573.truthy() { t1573 } else { { let t1571 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1572 = Py::Float(1e-15); if py::lt(&t1571, &t1572)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1601(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1603: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1602 = v_inputs.clone(); f_extraterrestrial_irradiance(py::getitem(&t1602, &Py::str("n"))?, py::method(&t1602, "get", &[Py::str("method"), Py::str("spencer")])?, py::method(&t1602, "get", &[Py::str("solar_constant"), c_SOLAR_CONSTANT()?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1603 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1604 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1633 = (t1604).items()?;
        if t1633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1633[0].clone();
        v_v = t1633[1].clone();
        if ({ let t1605 = v_k.clone(); Py::Bool({ let t1606 = v_outputs.clone(); if !py::contains(&t1606, &t1605)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1632 = { let t1623 = v_d.clone(); Py::Bool({ let t1624 = Py::Int(0i64); if py::gt(&t1623, &t1624)? { true } else { false } }) }; if !t1632.truthy() { t1632 } else { { let t1631 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1631.truthy() { t1631 } else { { let t1630 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1630.truthy() { t1630 } else { { let t1629 = { let t1625 = py::abs(&v_v.clone())?; Py::Bool({ let t1626 = Py::Float(1e-15); if py::lt(&t1625, &t1626)? { true } else { false } }) }; if !t1629.truthy() { t1629 } else { { let t1627 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1628 = Py::Float(1e-15); if py::lt(&t1627, &t1628)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1659(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1661: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1660 = v_inputs.clone(); f_fuel_cell_stack(py::getitem(&t1660, &Py::str("I"))?, py::getitem(&t1660, &Py::str("N"))?, py::getitem(&t1660, &Py::str("V_cell"))?, py::method(&t1660, "get", &[Py::str("utilization"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1661 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1662 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1691 = (t1662).items()?;
        if t1691.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1691[0].clone();
        v_v = t1691[1].clone();
        if ({ let t1663 = v_k.clone(); Py::Bool({ let t1664 = v_outputs.clone(); if !py::contains(&t1664, &t1663)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1690 = { let t1681 = v_d.clone(); Py::Bool({ let t1682 = Py::Int(0i64); if py::gt(&t1681, &t1682)? { true } else { false } }) }; if !t1690.truthy() { t1690 } else { { let t1689 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1689.truthy() { t1689 } else { { let t1688 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1688.truthy() { t1688 } else { { let t1687 = { let t1683 = py::abs(&v_v.clone())?; Py::Bool({ let t1684 = Py::Float(1e-15); if py::lt(&t1683, &t1684)? { true } else { false } }) }; if !t1687.truthy() { t1687 } else { { let t1685 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1686 = Py::Float(1e-15); if py::lt(&t1685, &t1686)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1717(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1719: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1718 = v_inputs.clone(); f_gmd(py::getitem(&t1718, &Py::str("D_ab"))?, py::getitem(&t1718, &Py::str("D_bc"))?, py::getitem(&t1718, &Py::str("D_ca"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1719 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1720 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1749 = (t1720).items()?;
        if t1749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1749[0].clone();
        v_v = t1749[1].clone();
        if ({ let t1721 = v_k.clone(); Py::Bool({ let t1722 = v_outputs.clone(); if !py::contains(&t1722, &t1721)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1748 = { let t1739 = v_d.clone(); Py::Bool({ let t1740 = Py::Int(0i64); if py::gt(&t1739, &t1740)? { true } else { false } }) }; if !t1748.truthy() { t1748 } else { { let t1747 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1747.truthy() { t1747 } else { { let t1746 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1746.truthy() { t1746 } else { { let t1745 = { let t1741 = py::abs(&v_v.clone())?; Py::Bool({ let t1742 = Py::Float(1e-15); if py::lt(&t1741, &t1742)? { true } else { false } }) }; if !t1745.truthy() { t1745 } else { { let t1743 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1744 = Py::Float(1e-15); if py::lt(&t1743, &t1744)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1778(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1780: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1779 = v_inputs.clone(); f_hub_air_density(py::getitem(&t1779, &Py::str("p"))?, py::getitem(&t1779, &Py::str("h_p"))?, py::getitem(&t1779, &Py::str("T"))?, py::getitem(&t1779, &Py::str("h_T"))?, py::getitem(&t1779, &Py::str("hub"))?, py::method(&t1779, "get", &[Py::str("model"), Py::str("barometric")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1780 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1781 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1810 = (t1781).items()?;
        if t1810.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1810[0].clone();
        v_v = t1810[1].clone();
        if ({ let t1782 = v_k.clone(); Py::Bool({ let t1783 = v_outputs.clone(); if !py::contains(&t1783, &t1782)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1809 = { let t1800 = v_d.clone(); Py::Bool({ let t1801 = Py::Int(0i64); if py::gt(&t1800, &t1801)? { true } else { false } }) }; if !t1809.truthy() { t1809 } else { { let t1808 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1808.truthy() { t1808 } else { { let t1807 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1807.truthy() { t1807 } else { { let t1806 = { let t1802 = py::abs(&v_v.clone())?; Py::Bool({ let t1803 = Py::Float(1e-15); if py::lt(&t1802, &t1803)? { true } else { false } }) }; if !t1806.truthy() { t1806 } else { { let t1804 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1805 = Py::Float(1e-15); if py::lt(&t1804, &t1805)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1840(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1842: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1841 = v_inputs.clone(); f_hydro_power(py::getitem(&t1841, &Py::str("Q"))?, py::getitem(&t1841, &Py::str("H"))?, py::getitem(&t1841, &Py::str("eta"))?, py::method(&t1841, "get", &[Py::str("rho"), Py::Float(1000.0)])?, py::method(&t1841, "get", &[Py::str("g"), Py::Float(9.81)])?, py::method(&t1841, "get", &[Py::str("CF"), Py::Float(1.0)])?, py::method(&t1841, "get", &[Py::str("hours"), Py::Float(8760.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1842 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1843 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1872 = (t1843).items()?;
        if t1872.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1872[0].clone();
        v_v = t1872[1].clone();
        if ({ let t1844 = v_k.clone(); Py::Bool({ let t1845 = v_outputs.clone(); if !py::contains(&t1845, &t1844)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1871 = { let t1862 = v_d.clone(); Py::Bool({ let t1863 = Py::Int(0i64); if py::gt(&t1862, &t1863)? { true } else { false } }) }; if !t1871.truthy() { t1871 } else { { let t1870 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1870.truthy() { t1870 } else { { let t1869 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1869.truthy() { t1869 } else { { let t1868 = { let t1864 = py::abs(&v_v.clone())?; Py::Bool({ let t1865 = Py::Float(1e-15); if py::lt(&t1864, &t1865)? { true } else { false } }) }; if !t1868.truthy() { t1868 } else { { let t1866 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1867 = Py::Float(1e-15); if py::lt(&t1866, &t1867)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1941(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1943: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1942 = v_inputs.clone(); f_hydrogen_isentropic_compression(py::getitem(&t1942, &Py::str("p1"))?, py::getitem(&t1942, &Py::str("T1"))?, py::getitem(&t1942, &Py::str("p2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1943 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1944 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1973 = (t1944).items()?;
        if t1973.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1973[0].clone();
        v_v = t1973[1].clone();
        if ({ let t1945 = v_k.clone(); Py::Bool({ let t1946 = v_outputs.clone(); if !py::contains(&t1946, &t1945)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1972 = { let t1963 = v_d.clone(); Py::Bool({ let t1964 = Py::Int(0i64); if py::gt(&t1963, &t1964)? { true } else { false } }) }; if !t1972.truthy() { t1972 } else { { let t1971 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1971.truthy() { t1971 } else { { let t1970 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1970.truthy() { t1970 } else { { let t1969 = { let t1965 = py::abs(&v_v.clone())?; Py::Bool({ let t1966 = Py::Float(1e-15); if py::lt(&t1965, &t1966)? { true } else { false } }) }; if !t1969.truthy() { t1969 } else { { let t1967 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1968 = Py::Float(1e-15); if py::lt(&t1967, &t1968)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2034(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2036: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2035 = v_inputs.clone(); f_hydrogen_isothermal_compression(py::getitem(&t2035, &Py::str("p1"))?, py::getitem(&t2035, &Py::str("p2"))?, py::getitem(&t2035, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2036 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2037 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2066 = (t2037).items()?;
        if t2066.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2066[0].clone();
        v_v = t2066[1].clone();
        if ({ let t2038 = v_k.clone(); Py::Bool({ let t2039 = v_outputs.clone(); if !py::contains(&t2039, &t2038)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2065 = { let t2056 = v_d.clone(); Py::Bool({ let t2057 = Py::Int(0i64); if py::gt(&t2056, &t2057)? { true } else { false } }) }; if !t2065.truthy() { t2065 } else { { let t2064 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2064.truthy() { t2064 } else { { let t2063 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2063.truthy() { t2063 } else { { let t2062 = { let t2058 = py::abs(&v_v.clone())?; Py::Bool({ let t2059 = Py::Float(1e-15); if py::lt(&t2058, &t2059)? { true } else { false } }) }; if !t2062.truthy() { t2062 } else { { let t2060 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2061 = Py::Float(1e-15); if py::lt(&t2060, &t2061)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_207(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t209: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t208 = v_inputs.clone(); f_pack_sizing(py::getitem(&t208, &Py::str("V_target"))?, py::getitem(&t208, &Py::str("E_target"))?, py::getitem(&t208, &Py::str("V_cell"))?, py::getitem(&t208, &Py::str("Q_cell"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t209 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t210 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t239 = (t210).items()?;
        if t239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t239[0].clone();
        v_v = t239[1].clone();
        if ({ let t211 = v_k.clone(); Py::Bool({ let t212 = v_outputs.clone(); if !py::contains(&t212, &t211)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t238 = { let t229 = v_d.clone(); Py::Bool({ let t230 = Py::Int(0i64); if py::gt(&t229, &t230)? { true } else { false } }) }; if !t238.truthy() { t238 } else { { let t237 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t237.truthy() { t237 } else { { let t236 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t236.truthy() { t236 } else { { let t235 = { let t231 = py::abs(&v_v.clone())?; Py::Bool({ let t232 = Py::Float(1e-15); if py::lt(&t231, &t232)? { true } else { false } }) }; if !t235.truthy() { t235 } else { { let t233 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t234 = Py::Float(1e-15); if py::lt(&t233, &t234)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2128(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2130: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2129 = v_inputs.clone(); f_hydrogen_tank(py::getitem(&t2129, &Py::str("V"))?, py::getitem(&t2129, &Py::str("p"))?, py::getitem(&t2129, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2130 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2131 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2160 = (t2131).items()?;
        if t2160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2160[0].clone();
        v_v = t2160[1].clone();
        if ({ let t2132 = v_k.clone(); Py::Bool({ let t2133 = v_outputs.clone(); if !py::contains(&t2133, &t2132)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2159 = { let t2150 = v_d.clone(); Py::Bool({ let t2151 = Py::Int(0i64); if py::gt(&t2150, &t2151)? { true } else { false } }) }; if !t2159.truthy() { t2159 } else { { let t2158 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2158.truthy() { t2158 } else { { let t2157 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2157.truthy() { t2157 } else { { let t2156 = { let t2152 = py::abs(&v_v.clone())?; Py::Bool({ let t2153 = Py::Float(1e-15); if py::lt(&t2152, &t2153)? { true } else { false } }) }; if !t2156.truthy() { t2156 } else { { let t2154 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2155 = Py::Float(1e-15); if py::lt(&t2154, &t2155)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2182(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2184: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2183 = v_inputs.clone(); f_iam_ashrae(py::getitem(&t2183, &Py::str("aoi"))?, py::method(&t2183, "get", &[Py::str("b0"), Py::Float(0.05)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2184 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2185 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2214 = (t2185).items()?;
        if t2214.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2214[0].clone();
        v_v = t2214[1].clone();
        if ({ let t2186 = v_k.clone(); Py::Bool({ let t2187 = v_outputs.clone(); if !py::contains(&t2187, &t2186)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2213 = { let t2204 = v_d.clone(); Py::Bool({ let t2205 = Py::Int(0i64); if py::gt(&t2204, &t2205)? { true } else { false } }) }; if !t2213.truthy() { t2213 } else { { let t2212 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2212.truthy() { t2212 } else { { let t2211 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2211.truthy() { t2211 } else { { let t2210 = { let t2206 = py::abs(&v_v.clone())?; Py::Bool({ let t2207 = Py::Float(1e-15); if py::lt(&t2206, &t2207)? { true } else { false } }) }; if !t2210.truthy() { t2210 } else { { let t2208 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2209 = Py::Float(1e-15); if py::lt(&t2208, &t2209)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2239(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2241: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2240 = v_inputs.clone(); f_iam_physical(py::getitem(&t2240, &Py::str("aoi"))?, py::method(&t2240, "get", &[Py::str("n"), Py::Float(1.526)])?, py::method(&t2240, "get", &[Py::str("K"), Py::Float(4.0)])?, py::method(&t2240, "get", &[Py::str("L"), Py::Float(0.002)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2241 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2242 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2271 = (t2242).items()?;
        if t2271.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2271[0].clone();
        v_v = t2271[1].clone();
        if ({ let t2243 = v_k.clone(); Py::Bool({ let t2244 = v_outputs.clone(); if !py::contains(&t2244, &t2243)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2270 = { let t2261 = v_d.clone(); Py::Bool({ let t2262 = Py::Int(0i64); if py::gt(&t2261, &t2262)? { true } else { false } }) }; if !t2270.truthy() { t2270 } else { { let t2269 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2269.truthy() { t2269 } else { { let t2268 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2268.truthy() { t2268 } else { { let t2267 = { let t2263 = py::abs(&v_v.clone())?; Py::Bool({ let t2264 = Py::Float(1e-15); if py::lt(&t2263, &t2264)? { true } else { false } }) }; if !t2267.truthy() { t2267 } else { { let t2265 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2266 = Py::Float(1e-15); if py::lt(&t2265, &t2266)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_23(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t25: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t24 = v_inputs.clone(); f_actuator_disc(py::getitem(&t24, &Py::str("a"))?, py::getitem(&t24, &Py::str("rho"))?, py::getitem(&t24, &Py::str("D"))?, py::getitem(&t24, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t25 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t26 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t55 = (t26).items()?;
        if t55.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t55[0].clone();
        v_v = t55[1].clone();
        if ({ let t27 = v_k.clone(); Py::Bool({ let t28 = v_outputs.clone(); if !py::contains(&t28, &t27)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t54 = { let t45 = v_d.clone(); Py::Bool({ let t46 = Py::Int(0i64); if py::gt(&t45, &t46)? { true } else { false } }) }; if !t54.truthy() { t54 } else { { let t53 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t53.truthy() { t53 } else { { let t52 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t52.truthy() { t52 } else { { let t51 = { let t47 = py::abs(&v_v.clone())?; Py::Bool({ let t48 = Py::Float(1e-15); if py::lt(&t47, &t48)? { true } else { false } }) }; if !t51.truthy() { t51 } else { { let t49 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t50 = Py::Float(1e-15); if py::lt(&t49, &t50)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2309(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2311: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2310 = v_inputs.clone(); f_induction_max_torque(py::getitem(&t2310, &Py::str("VL"))?, py::getitem(&t2310, &Py::str("f"))?, py::getitem(&t2310, &Py::str("poles"))?, py::getitem(&t2310, &Py::str("R1"))?, py::getitem(&t2310, &Py::str("X1"))?, py::getitem(&t2310, &Py::str("R2"))?, py::getitem(&t2310, &Py::str("X2"))?, py::getitem(&t2310, &Py::str("Xm"))?, py::getitem(&t2310, &Py::str("connection"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2311 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2312 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2341 = (t2312).items()?;
        if t2341.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2341[0].clone();
        v_v = t2341[1].clone();
        if ({ let t2313 = v_k.clone(); Py::Bool({ let t2314 = v_outputs.clone(); if !py::contains(&t2314, &t2313)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2340 = { let t2331 = v_d.clone(); Py::Bool({ let t2332 = Py::Int(0i64); if py::gt(&t2331, &t2332)? { true } else { false } }) }; if !t2340.truthy() { t2340 } else { { let t2339 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2339.truthy() { t2339 } else { { let t2338 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2338.truthy() { t2338 } else { { let t2337 = { let t2333 = py::abs(&v_v.clone())?; Py::Bool({ let t2334 = Py::Float(1e-15); if py::lt(&t2333, &t2334)? { true } else { false } }) }; if !t2337.truthy() { t2337 } else { { let t2335 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2336 = Py::Float(1e-15); if py::lt(&t2335, &t2336)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2378(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2380: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2379 = v_inputs.clone(); f_induction_motor(py::getitem(&t2379, &Py::str("VL"))?, py::getitem(&t2379, &Py::str("f"))?, py::getitem(&t2379, &Py::str("poles"))?, py::getitem(&t2379, &Py::str("R1"))?, py::getitem(&t2379, &Py::str("X1"))?, py::getitem(&t2379, &Py::str("R2"))?, py::getitem(&t2379, &Py::str("X2"))?, py::getitem(&t2379, &Py::str("Xm"))?, py::getitem(&t2379, &Py::str("slip"))?, py::getitem(&t2379, &Py::str("connection"))?, py::method(&t2379, "get", &[Py::str("P_rot"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2380 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2381 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2410 = (t2381).items()?;
        if t2410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2410[0].clone();
        v_v = t2410[1].clone();
        if ({ let t2382 = v_k.clone(); Py::Bool({ let t2383 = v_outputs.clone(); if !py::contains(&t2383, &t2382)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2409 = { let t2400 = v_d.clone(); Py::Bool({ let t2401 = Py::Int(0i64); if py::gt(&t2400, &t2401)? { true } else { false } }) }; if !t2409.truthy() { t2409 } else { { let t2408 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2408.truthy() { t2408 } else { { let t2407 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2407.truthy() { t2407 } else { { let t2406 = { let t2402 = py::abs(&v_v.clone())?; Py::Bool({ let t2403 = Py::Float(1e-15); if py::lt(&t2402, &t2403)? { true } else { false } }) }; if !t2406.truthy() { t2406 } else { { let t2404 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2405 = Py::Float(1e-15); if py::lt(&t2404, &t2405)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2460(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2462: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2461 = v_inputs.clone(); f_induction_power_flow(py::getitem(&t2461, &Py::str("f"))?, py::getitem(&t2461, &Py::str("poles"))?, py::getitem(&t2461, &Py::str("P_in"))?, py::getitem(&t2461, &Py::str("P_scl"))?, py::method(&t2461, "get", &[Py::str("s"), Py::None])?, py::method(&t2461, "get", &[Py::str("n_r"), Py::None])?, py::method(&t2461, "get", &[Py::str("P_core"), Py::Float(0.0)])?, py::method(&t2461, "get", &[Py::str("P_fw"), Py::Float(0.0)])?, py::method(&t2461, "get", &[Py::str("E2"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2462 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2463 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2492 = (t2463).items()?;
        if t2492.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2492[0].clone();
        v_v = t2492[1].clone();
        if ({ let t2464 = v_k.clone(); Py::Bool({ let t2465 = v_outputs.clone(); if !py::contains(&t2465, &t2464)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2491 = { let t2482 = v_d.clone(); Py::Bool({ let t2483 = Py::Int(0i64); if py::gt(&t2482, &t2483)? { true } else { false } }) }; if !t2491.truthy() { t2491 } else { { let t2490 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2490.truthy() { t2490 } else { { let t2489 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2489.truthy() { t2489 } else { { let t2488 = { let t2484 = py::abs(&v_v.clone())?; Py::Bool({ let t2485 = Py::Float(1e-15); if py::lt(&t2484, &t2485)? { true } else { false } }) }; if !t2488.truthy() { t2488 } else { { let t2486 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2487 = Py::Float(1e-15); if py::lt(&t2486, &t2487)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2538(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2540: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2539 = v_inputs.clone(); f_line_parameters(py::getitem(&t2539, &Py::str("D_ab"))?, py::getitem(&t2539, &Py::str("D_bc"))?, py::getitem(&t2539, &Py::str("D_ca"))?, py::getitem(&t2539, &Py::str("r"))?, py::getitem(&t2539, &Py::str("f"))?, py::method(&t2539, "get", &[Py::str("gmr"), Py::None])?, py::method(&t2539, "get", &[Py::str("n_bundle"), Py::Int(1i64)])?, py::method(&t2539, "get", &[Py::str("spacing"), Py::None])?, py::method(&t2539, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2540 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2541 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2570 = (t2541).items()?;
        if t2570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2570[0].clone();
        v_v = t2570[1].clone();
        if ({ let t2542 = v_k.clone(); Py::Bool({ let t2543 = v_outputs.clone(); if !py::contains(&t2543, &t2542)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2569 = { let t2560 = v_d.clone(); Py::Bool({ let t2561 = Py::Int(0i64); if py::gt(&t2560, &t2561)? { true } else { false } }) }; if !t2569.truthy() { t2569 } else { { let t2568 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2568.truthy() { t2568 } else { { let t2567 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2567.truthy() { t2567 } else { { let t2566 = { let t2562 = py::abs(&v_v.clone())?; Py::Bool({ let t2563 = Py::Float(1e-15); if py::lt(&t2562, &t2563)? { true } else { false } }) }; if !t2566.truthy() { t2566 } else { { let t2564 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2565 = Py::Float(1e-15); if py::lt(&t2564, &t2565)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2571(mut v_X1: Py, mut v_X2: Py, mut v_X0: Py, mut v_Rf: Py, mut v_Xf: Py, mut v_Vf: Py) -> R {
    return Ok(f_unsym_fault(Py::str("ll"), v_X1.clone(), v_X2.clone(), v_X0.clone(), v_Rf.clone(), v_Xf.clone(), v_Vf.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2609(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2611: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2610 = v_inputs.clone(); s_2571(py::getitem(&t2610, &Py::str("X1"))?, py::getitem(&t2610, &Py::str("X2"))?, py::getitem(&t2610, &Py::str("X0"))?, py::method(&t2610, "get", &[Py::str("Rf"), Py::Float(0.0)])?, py::method(&t2610, "get", &[Py::str("Xf"), Py::Float(0.0)])?, py::method(&t2610, "get", &[Py::str("Vf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2611 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2612 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2641 = (t2612).items()?;
        if t2641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2641[0].clone();
        v_v = t2641[1].clone();
        if ({ let t2613 = v_k.clone(); Py::Bool({ let t2614 = v_outputs.clone(); if !py::contains(&t2614, &t2613)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2640 = { let t2631 = v_d.clone(); Py::Bool({ let t2632 = Py::Int(0i64); if py::gt(&t2631, &t2632)? { true } else { false } }) }; if !t2640.truthy() { t2640 } else { { let t2639 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2639.truthy() { t2639 } else { { let t2638 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2638.truthy() { t2638 } else { { let t2637 = { let t2633 = py::abs(&v_v.clone())?; Py::Bool({ let t2634 = Py::Float(1e-15); if py::lt(&t2633, &t2634)? { true } else { false } }) }; if !t2637.truthy() { t2637 } else { { let t2635 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2636 = Py::Float(1e-15); if py::lt(&t2635, &t2636)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_264(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t266: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t265 = v_inputs.clone(); f_battery_rc_discharge(py::getitem(&t265, &Py::str("OCV"))?, py::getitem(&t265, &Py::str("R0"))?, py::getitem(&t265, &Py::str("R1"))?, py::getitem(&t265, &Py::str("C1"))?, py::getitem(&t265, &Py::str("I"))?, py::getitem(&t265, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t266 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t267 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t296 = (t267).items()?;
        if t296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t296[0].clone();
        v_v = t296[1].clone();
        if ({ let t268 = v_k.clone(); Py::Bool({ let t269 = v_outputs.clone(); if !py::contains(&t269, &t268)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t295 = { let t286 = v_d.clone(); Py::Bool({ let t287 = Py::Int(0i64); if py::gt(&t286, &t287)? { true } else { false } }) }; if !t295.truthy() { t295 } else { { let t294 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t294.truthy() { t294 } else { { let t293 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t293.truthy() { t293 } else { { let t292 = { let t288 = py::abs(&v_v.clone())?; Py::Bool({ let t289 = Py::Float(1e-15); if py::lt(&t288, &t289)? { true } else { false } }) }; if !t292.truthy() { t292 } else { { let t290 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t291 = Py::Float(1e-15); if py::lt(&t290, &t291)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2682(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2684: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2683 = v_inputs.clone(); f_long_line(py::getitem(&t2683, &Py::str("Vr"))?, py::getitem(&t2683, &Py::str("P"))?, py::getitem(&t2683, &Py::str("pf"))?, py::getitem(&t2683, &Py::str("R"))?, py::getitem(&t2683, &Py::str("X"))?, py::getitem(&t2683, &Py::str("B"))?, py::method(&t2683, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2684 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2685 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2714 = (t2685).items()?;
        if t2714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2714[0].clone();
        v_v = t2714[1].clone();
        if ({ let t2686 = v_k.clone(); Py::Bool({ let t2687 = v_outputs.clone(); if !py::contains(&t2687, &t2686)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2713 = { let t2704 = v_d.clone(); Py::Bool({ let t2705 = Py::Int(0i64); if py::gt(&t2704, &t2705)? { true } else { false } }) }; if !t2713.truthy() { t2713 } else { { let t2712 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2712.truthy() { t2712 } else { { let t2711 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2711.truthy() { t2711 } else { { let t2710 = { let t2706 = py::abs(&v_v.clone())?; Py::Bool({ let t2707 = Py::Float(1e-15); if py::lt(&t2706, &t2707)? { true } else { false } }) }; if !t2710.truthy() { t2710 } else { { let t2708 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2709 = Py::Float(1e-15); if py::lt(&t2708, &t2709)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2749(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2751: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2750 = v_inputs.clone(); f_medium_line(py::getitem(&t2750, &Py::str("Vr"))?, py::getitem(&t2750, &Py::str("P"))?, py::getitem(&t2750, &Py::str("pf"))?, py::getitem(&t2750, &Py::str("R"))?, py::getitem(&t2750, &Py::str("X"))?, py::getitem(&t2750, &Py::str("B"))?, py::method(&t2750, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2751 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2752 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2781 = (t2752).items()?;
        if t2781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2781[0].clone();
        v_v = t2781[1].clone();
        if ({ let t2753 = v_k.clone(); Py::Bool({ let t2754 = v_outputs.clone(); if !py::contains(&t2754, &t2753)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2780 = { let t2771 = v_d.clone(); Py::Bool({ let t2772 = Py::Int(0i64); if py::gt(&t2771, &t2772)? { true } else { false } }) }; if !t2780.truthy() { t2780 } else { { let t2779 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2779.truthy() { t2779 } else { { let t2778 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2778.truthy() { t2778 } else { { let t2777 = { let t2773 = py::abs(&v_v.clone())?; Py::Bool({ let t2774 = Py::Float(1e-15); if py::lt(&t2773, &t2774)? { true } else { false } }) }; if !t2777.truthy() { t2777 } else { { let t2775 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2776 = Py::Float(1e-15); if py::lt(&t2775, &t2776)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2955(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2957: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2956 = v_inputs.clone(); f_newton_load_flow(py::getitem(&t2956, &Py::str("lines"))?, py::getitem(&t2956, &Py::str("loads"))?, py::method(&t2956, "get", &[Py::str("gens"), Py::None])?, py::method(&t2956, "get", &[Py::str("V1"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2957 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2958 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2987 = (t2958).items()?;
        if t2987.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2987[0].clone();
        v_v = t2987[1].clone();
        if ({ let t2959 = v_k.clone(); Py::Bool({ let t2960 = v_outputs.clone(); if !py::contains(&t2960, &t2959)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2986 = { let t2977 = v_d.clone(); Py::Bool({ let t2978 = Py::Int(0i64); if py::gt(&t2977, &t2978)? { true } else { false } }) }; if !t2986.truthy() { t2986 } else { { let t2985 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2985.truthy() { t2985 } else { { let t2984 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2984.truthy() { t2984 } else { { let t2983 = { let t2979 = py::abs(&v_v.clone())?; Py::Bool({ let t2980 = Py::Float(1e-15); if py::lt(&t2979, &t2980)? { true } else { false } }) }; if !t2983.truthy() { t2983 } else { { let t2981 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2982 = Py::Float(1e-15); if py::lt(&t2981, &t2982)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3013(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3015: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3014 = v_inputs.clone(); f_parallel_transformers(py::getitem(&t3014, &Py::str("S1"))?, py::getitem(&t3014, &Py::str("vk1"))?, py::getitem(&t3014, &Py::str("vkr1"))?, py::getitem(&t3014, &Py::str("S2"))?, py::getitem(&t3014, &Py::str("vk2"))?, py::getitem(&t3014, &Py::str("vkr2"))?, py::getitem(&t3014, &Py::str("V_lv"))?, py::getitem(&t3014, &Py::str("P"))?, py::getitem(&t3014, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3015 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3016 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3045 = (t3016).items()?;
        if t3045.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3045[0].clone();
        v_v = t3045[1].clone();
        if ({ let t3017 = v_k.clone(); Py::Bool({ let t3018 = v_outputs.clone(); if !py::contains(&t3018, &t3017)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3044 = { let t3035 = v_d.clone(); Py::Bool({ let t3036 = Py::Int(0i64); if py::gt(&t3035, &t3036)? { true } else { false } }) }; if !t3044.truthy() { t3044 } else { { let t3043 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3043.truthy() { t3043 } else { { let t3042 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3042.truthy() { t3042 } else { { let t3041 = { let t3037 = py::abs(&v_v.clone())?; Py::Bool({ let t3038 = Py::Float(1e-15); if py::lt(&t3037, &t3038)? { true } else { false } }) }; if !t3041.truthy() { t3041 } else { { let t3039 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3040 = Py::Float(1e-15); if py::lt(&t3039, &t3040)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3068(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3070: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3069 = v_inputs.clone(); f_per_unit_change_of_base(py::getitem(&t3069, &Py::str("R_pu"))?, py::getitem(&t3069, &Py::str("X_pu"))?, py::getitem(&t3069, &Py::str("S_old"))?, py::getitem(&t3069, &Py::str("V_old"))?, py::getitem(&t3069, &Py::str("S_new"))?, py::getitem(&t3069, &Py::str("V_new"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3070 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3071 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3100 = (t3071).items()?;
        if t3100.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3100[0].clone();
        v_v = t3100[1].clone();
        if ({ let t3072 = v_k.clone(); Py::Bool({ let t3073 = v_outputs.clone(); if !py::contains(&t3073, &t3072)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3099 = { let t3090 = v_d.clone(); Py::Bool({ let t3091 = Py::Int(0i64); if py::gt(&t3090, &t3091)? { true } else { false } }) }; if !t3099.truthy() { t3099 } else { { let t3098 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3098.truthy() { t3098 } else { { let t3097 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3097.truthy() { t3097 } else { { let t3096 = { let t3092 = py::abs(&v_v.clone())?; Py::Bool({ let t3093 = Py::Float(1e-15); if py::lt(&t3092, &t3093)? { true } else { false } }) }; if !t3096.truthy() { t3096 } else { { let t3094 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3095 = Py::Float(1e-15); if py::lt(&t3094, &t3095)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3128(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3130: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3129 = v_inputs.clone(); f_per_unit_impedance(py::getitem(&t3129, &Py::str("R"))?, py::getitem(&t3129, &Py::str("X"))?, py::getitem(&t3129, &Py::str("S_base"))?, py::getitem(&t3129, &Py::str("V_base"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3130 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3131 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3160 = (t3131).items()?;
        if t3160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3160[0].clone();
        v_v = t3160[1].clone();
        if ({ let t3132 = v_k.clone(); Py::Bool({ let t3133 = v_outputs.clone(); if !py::contains(&t3133, &t3132)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3159 = { let t3150 = v_d.clone(); Py::Bool({ let t3151 = Py::Int(0i64); if py::gt(&t3150, &t3151)? { true } else { false } }) }; if !t3159.truthy() { t3159 } else { { let t3158 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3158.truthy() { t3158 } else { { let t3157 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3157.truthy() { t3157 } else { { let t3156 = { let t3152 = py::abs(&v_v.clone())?; Py::Bool({ let t3153 = Py::Float(1e-15); if py::lt(&t3152, &t3153)? { true } else { false } }) }; if !t3156.truthy() { t3156 } else { { let t3154 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3155 = Py::Float(1e-15); if py::lt(&t3154, &t3155)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3201(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3203: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3202 = v_inputs.clone(); f_peukert(py::getitem(&t3202, &Py::str("currents"))?, py::getitem(&t3202, &Py::str("times"))?, py::getitem(&t3202, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3203 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3204 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3233 = (t3204).items()?;
        if t3233.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3233[0].clone();
        v_v = t3233[1].clone();
        if ({ let t3205 = v_k.clone(); Py::Bool({ let t3206 = v_outputs.clone(); if !py::contains(&t3206, &t3205)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3232 = { let t3223 = v_d.clone(); Py::Bool({ let t3224 = Py::Int(0i64); if py::gt(&t3223, &t3224)? { true } else { false } }) }; if !t3232.truthy() { t3232 } else { { let t3231 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3231.truthy() { t3231 } else { { let t3230 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3230.truthy() { t3230 } else { { let t3229 = { let t3225 = py::abs(&v_v.clone())?; Py::Bool({ let t3226 = Py::Float(1e-15); if py::lt(&t3225, &t3226)? { true } else { false } }) }; if !t3229.truthy() { t3229 } else { { let t3227 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3228 = Py::Float(1e-15); if py::lt(&t3227, &t3228)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3234(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py, mut v_n: Py) -> R {
    return Ok(f__poa(Py::str("haydavies"), v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone(), v_dni.clone(), v_ghi.clone(), v_dhi.clone(), v_albedo.clone(), v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3286(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3288: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3287 = v_inputs.clone(); s_3234(py::getitem(&t3287, &Py::str("tilt"))?, py::getitem(&t3287, &Py::str("surface_azimuth"))?, py::getitem(&t3287, &Py::str("zenith"))?, py::getitem(&t3287, &Py::str("azimuth"))?, py::getitem(&t3287, &Py::str("dni"))?, py::getitem(&t3287, &Py::str("ghi"))?, py::getitem(&t3287, &Py::str("dhi"))?, py::getitem(&t3287, &Py::str("albedo"))?, py::getitem(&t3287, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3288 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3289 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3318 = (t3289).items()?;
        if t3318.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3318[0].clone();
        v_v = t3318[1].clone();
        if ({ let t3290 = v_k.clone(); Py::Bool({ let t3291 = v_outputs.clone(); if !py::contains(&t3291, &t3290)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3317 = { let t3308 = v_d.clone(); Py::Bool({ let t3309 = Py::Int(0i64); if py::gt(&t3308, &t3309)? { true } else { false } }) }; if !t3317.truthy() { t3317 } else { { let t3316 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3316.truthy() { t3316 } else { { let t3315 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3315.truthy() { t3315 } else { { let t3314 = { let t3310 = py::abs(&v_v.clone())?; Py::Bool({ let t3311 = Py::Float(1e-15); if py::lt(&t3310, &t3311)? { true } else { false } }) }; if !t3314.truthy() { t3314 } else { { let t3312 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3313 = Py::Float(1e-15); if py::lt(&t3312, &t3313)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3319(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py) -> R {
    return Ok(f__poa(Py::str("isotropic"), v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone(), v_dni.clone(), v_ghi.clone(), v_dhi.clone(), v_albedo.clone(), Py::None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3371(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3373: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3372 = v_inputs.clone(); s_3319(py::getitem(&t3372, &Py::str("tilt"))?, py::getitem(&t3372, &Py::str("surface_azimuth"))?, py::getitem(&t3372, &Py::str("zenith"))?, py::getitem(&t3372, &Py::str("azimuth"))?, py::getitem(&t3372, &Py::str("dni"))?, py::getitem(&t3372, &Py::str("ghi"))?, py::getitem(&t3372, &Py::str("dhi"))?, py::getitem(&t3372, &Py::str("albedo"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3373 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3374 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3403 = (t3374).items()?;
        if t3403.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3403[0].clone();
        v_v = t3403[1].clone();
        if ({ let t3375 = v_k.clone(); Py::Bool({ let t3376 = v_outputs.clone(); if !py::contains(&t3376, &t3375)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3402 = { let t3393 = v_d.clone(); Py::Bool({ let t3394 = Py::Int(0i64); if py::gt(&t3393, &t3394)? { true } else { false } }) }; if !t3402.truthy() { t3402 } else { { let t3401 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3401.truthy() { t3401 } else { { let t3400 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3400.truthy() { t3400 } else { { let t3399 = { let t3395 = py::abs(&v_v.clone())?; Py::Bool({ let t3396 = Py::Float(1e-15); if py::lt(&t3395, &t3396)? { true } else { false } }) }; if !t3399.truthy() { t3399 } else { { let t3397 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3398 = Py::Float(1e-15); if py::lt(&t3397, &t3398)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3404(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py) -> R {
    return Ok(f__poa(Py::str("klucher"), v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone(), v_dni.clone(), v_ghi.clone(), v_dhi.clone(), v_albedo.clone(), Py::None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3456(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3458: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3457 = v_inputs.clone(); s_3404(py::getitem(&t3457, &Py::str("tilt"))?, py::getitem(&t3457, &Py::str("surface_azimuth"))?, py::getitem(&t3457, &Py::str("zenith"))?, py::getitem(&t3457, &Py::str("azimuth"))?, py::getitem(&t3457, &Py::str("dni"))?, py::getitem(&t3457, &Py::str("ghi"))?, py::getitem(&t3457, &Py::str("dhi"))?, py::getitem(&t3457, &Py::str("albedo"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3458 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3459 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3488 = (t3459).items()?;
        if t3488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3488[0].clone();
        v_v = t3488[1].clone();
        if ({ let t3460 = v_k.clone(); Py::Bool({ let t3461 = v_outputs.clone(); if !py::contains(&t3461, &t3460)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3487 = { let t3478 = v_d.clone(); Py::Bool({ let t3479 = Py::Int(0i64); if py::gt(&t3478, &t3479)? { true } else { false } }) }; if !t3487.truthy() { t3487 } else { { let t3486 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3486.truthy() { t3486 } else { { let t3485 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3485.truthy() { t3485 } else { { let t3484 = { let t3480 = py::abs(&v_v.clone())?; Py::Bool({ let t3481 = Py::Float(1e-15); if py::lt(&t3480, &t3481)? { true } else { false } }) }; if !t3484.truthy() { t3484 } else { { let t3482 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3483 = Py::Float(1e-15); if py::lt(&t3482, &t3483)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3550(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3552: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3551 = v_inputs.clone(); f_poa_perez(py::getitem(&t3551, &Py::str("tilt"))?, py::getitem(&t3551, &Py::str("surface_azimuth"))?, py::getitem(&t3551, &Py::str("zenith"))?, py::getitem(&t3551, &Py::str("azimuth"))?, py::getitem(&t3551, &Py::str("dni"))?, py::getitem(&t3551, &Py::str("ghi"))?, py::getitem(&t3551, &Py::str("dhi"))?, py::getitem(&t3551, &Py::str("albedo"))?, py::getitem(&t3551, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3552 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3553 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3582 = (t3553).items()?;
        if t3582.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3582[0].clone();
        v_v = t3582[1].clone();
        if ({ let t3554 = v_k.clone(); Py::Bool({ let t3555 = v_outputs.clone(); if !py::contains(&t3555, &t3554)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3581 = { let t3572 = v_d.clone(); Py::Bool({ let t3573 = Py::Int(0i64); if py::gt(&t3572, &t3573)? { true } else { false } }) }; if !t3581.truthy() { t3581 } else { { let t3580 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3580.truthy() { t3580 } else { { let t3579 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3579.truthy() { t3579 } else { { let t3578 = { let t3574 = py::abs(&v_v.clone())?; Py::Bool({ let t3575 = Py::Float(1e-15); if py::lt(&t3574, &t3575)? { true } else { false } }) }; if !t3578.truthy() { t3578 } else { { let t3576 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3577 = Py::Float(1e-15); if py::lt(&t3576, &t3577)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3583(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py, mut v_n: Py) -> R {
    return Ok(f__poa(Py::str("reindl"), v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone(), v_dni.clone(), v_ghi.clone(), v_dhi.clone(), v_albedo.clone(), v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3635(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3637: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3636 = v_inputs.clone(); s_3583(py::getitem(&t3636, &Py::str("tilt"))?, py::getitem(&t3636, &Py::str("surface_azimuth"))?, py::getitem(&t3636, &Py::str("zenith"))?, py::getitem(&t3636, &Py::str("azimuth"))?, py::getitem(&t3636, &Py::str("dni"))?, py::getitem(&t3636, &Py::str("ghi"))?, py::getitem(&t3636, &Py::str("dhi"))?, py::getitem(&t3636, &Py::str("albedo"))?, py::getitem(&t3636, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3637 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3638 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3667 = (t3638).items()?;
        if t3667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3667[0].clone();
        v_v = t3667[1].clone();
        if ({ let t3639 = v_k.clone(); Py::Bool({ let t3640 = v_outputs.clone(); if !py::contains(&t3640, &t3639)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3666 = { let t3657 = v_d.clone(); Py::Bool({ let t3658 = Py::Int(0i64); if py::gt(&t3657, &t3658)? { true } else { false } }) }; if !t3666.truthy() { t3666 } else { { let t3665 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3665.truthy() { t3665 } else { { let t3664 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3664.truthy() { t3664 } else { { let t3663 = { let t3659 = py::abs(&v_v.clone())?; Py::Bool({ let t3660 = Py::Float(1e-15); if py::lt(&t3659, &t3660)? { true } else { false } }) }; if !t3663.truthy() { t3663 } else { { let t3661 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3662 = Py::Float(1e-15); if py::lt(&t3661, &t3662)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3687(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3689: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3688 = v_inputs.clone(); f_power_angle(py::getitem(&t3688, &Py::str("V1"))?, py::getitem(&t3688, &Py::str("V2"))?, py::getitem(&t3688, &Py::str("X"))?, py::getitem(&t3688, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3689 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3690 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3719 = (t3690).items()?;
        if t3719.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3719[0].clone();
        v_v = t3719[1].clone();
        if ({ let t3691 = v_k.clone(); Py::Bool({ let t3692 = v_outputs.clone(); if !py::contains(&t3692, &t3691)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3718 = { let t3709 = v_d.clone(); Py::Bool({ let t3710 = Py::Int(0i64); if py::gt(&t3709, &t3710)? { true } else { false } }) }; if !t3718.truthy() { t3718 } else { { let t3717 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3717.truthy() { t3717 } else { { let t3716 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3716.truthy() { t3716 } else { { let t3715 = { let t3711 = py::abs(&v_v.clone())?; Py::Bool({ let t3712 = Py::Float(1e-15); if py::lt(&t3711, &t3712)? { true } else { false } }) }; if !t3715.truthy() { t3715 } else { { let t3713 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3714 = Py::Float(1e-15); if py::lt(&t3713, &t3714)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3774(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3776: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3775 = v_inputs.clone(); f_power_curve_density_corrected(py::getitem(&t3775, &Py::str("speeds"))?, py::getitem(&t3775, &Py::str("powers"))?, py::getitem(&t3775, &Py::str("v"))?, py::getitem(&t3775, &Py::str("rho"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3776 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3777 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3806 = (t3777).items()?;
        if t3806.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3806[0].clone();
        v_v = t3806[1].clone();
        if ({ let t3778 = v_k.clone(); Py::Bool({ let t3779 = v_outputs.clone(); if !py::contains(&t3779, &t3778)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3805 = { let t3796 = v_d.clone(); Py::Bool({ let t3797 = Py::Int(0i64); if py::gt(&t3796, &t3797)? { true } else { false } }) }; if !t3805.truthy() { t3805 } else { { let t3804 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3804.truthy() { t3804 } else { { let t3803 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3803.truthy() { t3803 } else { { let t3802 = { let t3798 = py::abs(&v_v.clone())?; Py::Bool({ let t3799 = Py::Float(1e-15); if py::lt(&t3798, &t3799)? { true } else { false } }) }; if !t3802.truthy() { t3802 } else { { let t3800 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3801 = Py::Float(1e-15); if py::lt(&t3800, &t3801)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3849(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3851: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3850 = v_inputs.clone(); f_power_curve_output(py::getitem(&t3850, &Py::str("speeds"))?, py::getitem(&t3850, &Py::str("powers"))?, py::getitem(&t3850, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3851 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3852 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3881 = (t3852).items()?;
        if t3881.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3881[0].clone();
        v_v = t3881[1].clone();
        if ({ let t3853 = v_k.clone(); Py::Bool({ let t3854 = v_outputs.clone(); if !py::contains(&t3854, &t3853)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3880 = { let t3871 = v_d.clone(); Py::Bool({ let t3872 = Py::Int(0i64); if py::gt(&t3871, &t3872)? { true } else { false } }) }; if !t3880.truthy() { t3880 } else { { let t3879 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3879.truthy() { t3879 } else { { let t3878 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3878.truthy() { t3878 } else { { let t3877 = { let t3873 = py::abs(&v_v.clone())?; Py::Bool({ let t3874 = Py::Float(1e-15); if py::lt(&t3873, &t3874)? { true } else { false } }) }; if !t3877.truthy() { t3877 } else { { let t3875 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3876 = Py::Float(1e-15); if py::lt(&t3875, &t3876)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_388(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t390: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t389 = v_inputs.clone(); f_butler_volmer(py::getitem(&t389, &Py::str("j0"))?, py::getitem(&t389, &Py::str("alpha_a"))?, py::getitem(&t389, &Py::str("alpha_c"))?, py::getitem(&t389, &Py::str("T"))?, py::method(&t389, "get", &[Py::str("eta"), Py::None])?, py::method(&t389, "get", &[Py::str("j"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t390 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t391 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t420 = (t391).items()?;
        if t420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t420[0].clone();
        v_v = t420[1].clone();
        if ({ let t392 = v_k.clone(); Py::Bool({ let t393 = v_outputs.clone(); if !py::contains(&t393, &t392)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t419 = { let t410 = v_d.clone(); Py::Bool({ let t411 = Py::Int(0i64); if py::gt(&t410, &t411)? { true } else { false } }) }; if !t419.truthy() { t419 } else { { let t418 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t418.truthy() { t418 } else { { let t417 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t417.truthy() { t417 } else { { let t416 = { let t412 = py::abs(&v_v.clone())?; Py::Bool({ let t413 = Py::Float(1e-15); if py::lt(&t412, &t413)? { true } else { false } }) }; if !t416.truthy() { t416 } else { { let t414 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t415 = Py::Float(1e-15); if py::lt(&t414, &t415)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3913(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3915: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3914 = v_inputs.clone(); f__pv_array_check(py::getitem(&t3914, &Py::str("poa"))?, py::getitem(&t3914, &Py::str("T_air"))?, py::getitem(&t3914, &Py::str("noct"))?, py::getitem(&t3914, &Py::str("pdc0"))?, py::getitem(&t3914, &Py::str("gamma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3915 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3916 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3945 = (t3916).items()?;
        if t3945.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3945[0].clone();
        v_v = t3945[1].clone();
        if ({ let t3917 = v_k.clone(); Py::Bool({ let t3918 = v_outputs.clone(); if !py::contains(&t3918, &t3917)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3944 = { let t3935 = v_d.clone(); Py::Bool({ let t3936 = Py::Int(0i64); if py::gt(&t3935, &t3936)? { true } else { false } }) }; if !t3944.truthy() { t3944 } else { { let t3943 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3943.truthy() { t3943 } else { { let t3942 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3942.truthy() { t3942 } else { { let t3941 = { let t3937 = py::abs(&v_v.clone())?; Py::Bool({ let t3938 = Py::Float(1e-15); if py::lt(&t3937, &t3938)? { true } else { false } }) }; if !t3941.truthy() { t3941 } else { { let t3939 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3940 = Py::Float(1e-15); if py::lt(&t3939, &t3940)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4019(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4021: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4020 = v_inputs.clone(); f_pv_module_at_conditions(py::getitem(&t4020, &Py::str("G"))?, py::getitem(&t4020, &Py::str("T_cell"))?, py::getitem(&t4020, &Py::str("alpha_sc"))?, py::getitem(&t4020, &Py::str("a_ref"))?, py::getitem(&t4020, &Py::str("IL_ref"))?, py::getitem(&t4020, &Py::str("I0_ref"))?, py::getitem(&t4020, &Py::str("Rsh_ref"))?, py::getitem(&t4020, &Py::str("Rs"))?, py::method(&t4020, "get", &[Py::str("EgRef"), Py::Float(1.121)])?, py::method(&t4020, "get", &[Py::str("dEgdT"), py::neg(&Py::Float(0.0002677))?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4021 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4022 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4051 = (t4022).items()?;
        if t4051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4051[0].clone();
        v_v = t4051[1].clone();
        if ({ let t4023 = v_k.clone(); Py::Bool({ let t4024 = v_outputs.clone(); if !py::contains(&t4024, &t4023)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4050 = { let t4041 = v_d.clone(); Py::Bool({ let t4042 = Py::Int(0i64); if py::gt(&t4041, &t4042)? { true } else { false } }) }; if !t4050.truthy() { t4050 } else { { let t4049 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4049.truthy() { t4049 } else { { let t4048 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4048.truthy() { t4048 } else { { let t4047 = { let t4043 = py::abs(&v_v.clone())?; Py::Bool({ let t4044 = Py::Float(1e-15); if py::lt(&t4043, &t4044)? { true } else { false } }) }; if !t4047.truthy() { t4047 } else { { let t4045 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4046 = Py::Float(1e-15); if py::lt(&t4045, &t4046)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4081(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4083: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4082 = v_inputs.clone(); f_pvwatts_dc(py::getitem(&t4082, &Py::str("G_eff"))?, py::getitem(&t4082, &Py::str("T_cell"))?, py::getitem(&t4082, &Py::str("pdc0"))?, py::getitem(&t4082, &Py::str("gamma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4083 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4084 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4113 = (t4084).items()?;
        if t4113.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4113[0].clone();
        v_v = t4113[1].clone();
        if ({ let t4085 = v_k.clone(); Py::Bool({ let t4086 = v_outputs.clone(); if !py::contains(&t4086, &t4085)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4112 = { let t4103 = v_d.clone(); Py::Bool({ let t4104 = Py::Int(0i64); if py::gt(&t4103, &t4104)? { true } else { false } }) }; if !t4112.truthy() { t4112 } else { { let t4111 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4111.truthy() { t4111 } else { { let t4110 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4110.truthy() { t4110 } else { { let t4109 = { let t4105 = py::abs(&v_v.clone())?; Py::Bool({ let t4106 = Py::Float(1e-15); if py::lt(&t4105, &t4106)? { true } else { false } }) }; if !t4109.truthy() { t4109 } else { { let t4107 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4108 = Py::Float(1e-15); if py::lt(&t4107, &t4108)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4140(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4142: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4141 = v_inputs.clone(); f_pvwatts_inverter(py::getitem(&t4141, &Py::str("pdc"))?, py::getitem(&t4141, &Py::str("pdc0"))?, py::method(&t4141, "get", &[Py::str("eta_nom"), Py::Float(0.96)])?, py::method(&t4141, "get", &[Py::str("eta_ref"), Py::Float(0.9637)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4142 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4143 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4172 = (t4143).items()?;
        if t4172.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4172[0].clone();
        v_v = t4172[1].clone();
        if ({ let t4144 = v_k.clone(); Py::Bool({ let t4145 = v_outputs.clone(); if !py::contains(&t4145, &t4144)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4171 = { let t4162 = v_d.clone(); Py::Bool({ let t4163 = Py::Int(0i64); if py::gt(&t4162, &t4163)? { true } else { false } }) }; if !t4171.truthy() { t4171 } else { { let t4170 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4170.truthy() { t4170 } else { { let t4169 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4169.truthy() { t4169 } else { { let t4168 = { let t4164 = py::abs(&v_v.clone())?; Py::Bool({ let t4165 = Py::Float(1e-15); if py::lt(&t4164, &t4165)? { true } else { false } }) }; if !t4168.truthy() { t4168 } else { { let t4166 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4167 = Py::Float(1e-15); if py::lt(&t4166, &t4167)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4191(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4193: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = f_pvwatts_losses(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4193 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4194 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4223 = (t4194).items()?;
        if t4223.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4223[0].clone();
        v_v = t4223[1].clone();
        if ({ let t4195 = v_k.clone(); Py::Bool({ let t4196 = v_outputs.clone(); if !py::contains(&t4196, &t4195)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4222 = { let t4213 = v_d.clone(); Py::Bool({ let t4214 = Py::Int(0i64); if py::gt(&t4213, &t4214)? { true } else { false } }) }; if !t4222.truthy() { t4222 } else { { let t4221 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4221.truthy() { t4221 } else { { let t4220 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4220.truthy() { t4220 } else { { let t4219 = { let t4215 = py::abs(&v_v.clone())?; Py::Bool({ let t4216 = Py::Float(1e-15); if py::lt(&t4215, &t4216)? { true } else { false } }) }; if !t4219.truthy() { t4219 } else { { let t4217 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4218 = Py::Float(1e-15); if py::lt(&t4217, &t4218)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4255: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4254 = v_inputs.clone(); f_short_line(py::getitem(&t4254, &Py::str("Vr"))?, py::getitem(&t4254, &Py::str("P"))?, py::getitem(&t4254, &Py::str("pf"))?, py::getitem(&t4254, &Py::str("R"))?, py::getitem(&t4254, &Py::str("X"))?, py::method(&t4254, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4255 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4256 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4285 = (t4256).items()?;
        if t4285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4285[0].clone();
        v_v = t4285[1].clone();
        if ({ let t4257 = v_k.clone(); Py::Bool({ let t4258 = v_outputs.clone(); if !py::contains(&t4258, &t4257)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4284 = { let t4275 = v_d.clone(); Py::Bool({ let t4276 = Py::Int(0i64); if py::gt(&t4275, &t4276)? { true } else { false } }) }; if !t4284.truthy() { t4284 } else { { let t4283 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4283.truthy() { t4283 } else { { let t4282 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4282.truthy() { t4282 } else { { let t4281 = { let t4277 = py::abs(&v_v.clone())?; Py::Bool({ let t4278 = Py::Float(1e-15); if py::lt(&t4277, &t4278)? { true } else { false } }) }; if !t4281.truthy() { t4281 } else { { let t4279 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4280 = Py::Float(1e-15); if py::lt(&t4279, &t4280)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4341(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4343: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4342 = v_inputs.clone(); f_single_diode_current(py::getitem(&t4342, &Py::str("V"))?, py::getitem(&t4342, &Py::str("IL"))?, py::getitem(&t4342, &Py::str("I0"))?, py::getitem(&t4342, &Py::str("Rs"))?, py::getitem(&t4342, &Py::str("Rsh"))?, py::getitem(&t4342, &Py::str("nNsVth"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4343 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4344 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4373 = (t4344).items()?;
        if t4373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4373[0].clone();
        v_v = t4373[1].clone();
        if ({ let t4345 = v_k.clone(); Py::Bool({ let t4346 = v_outputs.clone(); if !py::contains(&t4346, &t4345)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4372 = { let t4363 = v_d.clone(); Py::Bool({ let t4364 = Py::Int(0i64); if py::gt(&t4363, &t4364)? { true } else { false } }) }; if !t4372.truthy() { t4372 } else { { let t4371 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4371.truthy() { t4371 } else { { let t4370 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4370.truthy() { t4370 } else { { let t4369 = { let t4365 = py::abs(&v_v.clone())?; Py::Bool({ let t4366 = Py::Float(1e-15); if py::lt(&t4365, &t4366)? { true } else { false } }) }; if !t4369.truthy() { t4369 } else { { let t4367 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4368 = Py::Float(1e-15); if py::lt(&t4367, &t4368)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4443(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4445: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4444 = v_inputs.clone(); f_single_diode_mpp(py::getitem(&t4444, &Py::str("IL"))?, py::getitem(&t4444, &Py::str("I0"))?, py::getitem(&t4444, &Py::str("Rs"))?, py::getitem(&t4444, &Py::str("Rsh"))?, py::getitem(&t4444, &Py::str("nNsVth"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4445 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4446 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4475 = (t4446).items()?;
        if t4475.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4475[0].clone();
        v_v = t4475[1].clone();
        if ({ let t4447 = v_k.clone(); Py::Bool({ let t4448 = v_outputs.clone(); if !py::contains(&t4448, &t4447)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4474 = { let t4465 = v_d.clone(); Py::Bool({ let t4466 = Py::Int(0i64); if py::gt(&t4465, &t4466)? { true } else { false } }) }; if !t4474.truthy() { t4474 } else { { let t4473 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4473.truthy() { t4473 } else { { let t4472 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4472.truthy() { t4472 } else { { let t4471 = { let t4467 = py::abs(&v_v.clone())?; Py::Bool({ let t4468 = Py::Float(1e-15); if py::lt(&t4467, &t4468)? { true } else { false } }) }; if !t4471.truthy() { t4471 } else { { let t4469 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4470 = Py::Float(1e-15); if py::lt(&t4469, &t4470)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4534(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4536: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4535 = v_inputs.clone(); f_single_diode_voltage(py::getitem(&t4535, &Py::str("I"))?, py::getitem(&t4535, &Py::str("IL"))?, py::getitem(&t4535, &Py::str("I0"))?, py::getitem(&t4535, &Py::str("Rs"))?, py::getitem(&t4535, &Py::str("Rsh"))?, py::getitem(&t4535, &Py::str("nNsVth"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4536 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4537 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4566 = (t4537).items()?;
        if t4566.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4566[0].clone();
        v_v = t4566[1].clone();
        if ({ let t4538 = v_k.clone(); Py::Bool({ let t4539 = v_outputs.clone(); if !py::contains(&t4539, &t4538)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4565 = { let t4556 = v_d.clone(); Py::Bool({ let t4557 = Py::Int(0i64); if py::gt(&t4556, &t4557)? { true } else { false } }) }; if !t4565.truthy() { t4565 } else { { let t4564 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4564.truthy() { t4564 } else { { let t4563 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4563.truthy() { t4563 } else { { let t4562 = { let t4558 = py::abs(&v_v.clone())?; Py::Bool({ let t4559 = Py::Float(1e-15); if py::lt(&t4558, &t4559)? { true } else { false } }) }; if !t4562.truthy() { t4562 } else { { let t4560 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4561 = Py::Float(1e-15); if py::lt(&t4560, &t4561)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_456(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t458: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t457 = v_inputs.clone(); f_capacity_factor(py::getitem(&t457, &Py::str("P_rated"))?, py::method(&t457, "get", &[Py::str("E"), Py::None])?, py::method(&t457, "get", &[Py::str("CF"), Py::None])?, py::method(&t457, "get", &[Py::str("hours"), Py::Float(8760.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t458 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t459 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t488 = (t459).items()?;
        if t488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t488[0].clone();
        v_v = t488[1].clone();
        if ({ let t460 = v_k.clone(); Py::Bool({ let t461 = v_outputs.clone(); if !py::contains(&t461, &t460)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t487 = { let t478 = v_d.clone(); Py::Bool({ let t479 = Py::Int(0i64); if py::gt(&t478, &t479)? { true } else { false } }) }; if !t487.truthy() { t487 } else { { let t486 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t486.truthy() { t486 } else { { let t485 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t485.truthy() { t485 } else { { let t484 = { let t480 = py::abs(&v_v.clone())?; Py::Bool({ let t481 = Py::Float(1e-15); if py::lt(&t480, &t481)? { true } else { false } }) }; if !t484.truthy() { t484 } else { { let t482 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t483 = Py::Float(1e-15); if py::lt(&t482, &t483)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4567(mut v_X1: Py, mut v_X2: Py, mut v_X0: Py, mut v_Rf: Py, mut v_Xf: Py, mut v_Vf: Py) -> R {
    return Ok(f_unsym_fault(Py::str("slg"), v_X1.clone(), v_X2.clone(), v_X0.clone(), v_Rf.clone(), v_Xf.clone(), v_Vf.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4605(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4607: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4606 = v_inputs.clone(); s_4567(py::getitem(&t4606, &Py::str("X1"))?, py::getitem(&t4606, &Py::str("X2"))?, py::getitem(&t4606, &Py::str("X0"))?, py::method(&t4606, "get", &[Py::str("Rf"), Py::Float(0.0)])?, py::method(&t4606, "get", &[Py::str("Xf"), Py::Float(0.0)])?, py::method(&t4606, "get", &[Py::str("Vf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4607 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4608 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4637 = (t4608).items()?;
        if t4637.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4637[0].clone();
        v_v = t4637[1].clone();
        if ({ let t4609 = v_k.clone(); Py::Bool({ let t4610 = v_outputs.clone(); if !py::contains(&t4610, &t4609)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4636 = { let t4627 = v_d.clone(); Py::Bool({ let t4628 = Py::Int(0i64); if py::gt(&t4627, &t4628)? { true } else { false } }) }; if !t4636.truthy() { t4636 } else { { let t4635 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4635.truthy() { t4635 } else { { let t4634 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4634.truthy() { t4634 } else { { let t4633 = { let t4629 = py::abs(&v_v.clone())?; Py::Bool({ let t4630 = Py::Float(1e-15); if py::lt(&t4629, &t4630)? { true } else { false } }) }; if !t4633.truthy() { t4633 } else { { let t4631 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4632 = Py::Float(1e-15); if py::lt(&t4631, &t4632)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4638(mut v_n: Py, mut v_method: Py) -> R {
    return Ok(f_solar_declination(v_n.clone(), v_method.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4657(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4659: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4658 = v_inputs.clone(); s_4638(py::getitem(&t4658, &Py::str("n"))?, py::method(&t4658, "get", &[Py::str("method"), Py::str("cooper")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4659 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4660 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4689 = (t4660).items()?;
        if t4689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4689[0].clone();
        v_v = t4689[1].clone();
        if ({ let t4661 = v_k.clone(); Py::Bool({ let t4662 = v_outputs.clone(); if !py::contains(&t4662, &t4661)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4688 = { let t4679 = v_d.clone(); Py::Bool({ let t4680 = Py::Int(0i64); if py::gt(&t4679, &t4680)? { true } else { false } }) }; if !t4688.truthy() { t4688 } else { { let t4687 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4687.truthy() { t4687 } else { { let t4686 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4686.truthy() { t4686 } else { { let t4685 = { let t4681 = py::abs(&v_v.clone())?; Py::Bool({ let t4682 = Py::Float(1e-15); if py::lt(&t4681, &t4682)? { true } else { false } }) }; if !t4685.truthy() { t4685 } else { { let t4683 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4684 = Py::Float(1e-15); if py::lt(&t4683, &t4684)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4739(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4741: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4740 = v_inputs.clone(); f_solar_position(py::getitem(&t4740, &Py::str("latitude"))?, py::getitem(&t4740, &Py::str("longitude"))?, py::getitem(&t4740, &Py::str("n"))?, py::getitem(&t4740, &Py::str("clock"))?, py::getitem(&t4740, &Py::str("utc_offset"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4741 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4742 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4771 = (t4742).items()?;
        if t4771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4771[0].clone();
        v_v = t4771[1].clone();
        if ({ let t4743 = v_k.clone(); Py::Bool({ let t4744 = v_outputs.clone(); if !py::contains(&t4744, &t4743)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4770 = { let t4761 = v_d.clone(); Py::Bool({ let t4762 = Py::Int(0i64); if py::gt(&t4761, &t4762)? { true } else { false } }) }; if !t4770.truthy() { t4770 } else { { let t4769 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4769.truthy() { t4769 } else { { let t4768 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4768.truthy() { t4768 } else { { let t4767 = { let t4763 = py::abs(&v_v.clone())?; Py::Bool({ let t4764 = Py::Float(1e-15); if py::lt(&t4763, &t4764)? { true } else { false } }) }; if !t4767.truthy() { t4767 } else { { let t4765 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4766 = Py::Float(1e-15); if py::lt(&t4765, &t4766)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4809(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4811: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4810 = v_inputs.clone(); f_solar_time_hour_angle(py::getitem(&t4810, &Py::str("n"))?, py::getitem(&t4810, &Py::str("clock"))?, py::getitem(&t4810, &Py::str("longitude"))?, py::getitem(&t4810, &Py::str("utc_offset"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4811 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4812 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4841 = (t4812).items()?;
        if t4841.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4841[0].clone();
        v_v = t4841[1].clone();
        if ({ let t4813 = v_k.clone(); Py::Bool({ let t4814 = v_outputs.clone(); if !py::contains(&t4814, &t4813)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4840 = { let t4831 = v_d.clone(); Py::Bool({ let t4832 = Py::Int(0i64); if py::gt(&t4831, &t4832)? { true } else { false } }) }; if !t4840.truthy() { t4840 } else { { let t4839 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4839.truthy() { t4839 } else { { let t4838 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4838.truthy() { t4838 } else { { let t4837 = { let t4833 = py::abs(&v_v.clone())?; Py::Bool({ let t4834 = Py::Float(1e-15); if py::lt(&t4833, &t4834)? { true } else { false } }) }; if !t4837.truthy() { t4837 } else { { let t4835 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4836 = Py::Float(1e-15); if py::lt(&t4835, &t4836)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4865(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4867: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4866 = v_inputs.clone(); f_stack_power(py::getitem(&t4866, &Py::str("I"))?, py::getitem(&t4866, &Py::str("N"))?, py::getitem(&t4866, &Py::str("V_cell"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4867 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4868 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4897 = (t4868).items()?;
        if t4897.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4897[0].clone();
        v_v = t4897[1].clone();
        if ({ let t4869 = v_k.clone(); Py::Bool({ let t4870 = v_outputs.clone(); if !py::contains(&t4870, &t4869)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4896 = { let t4887 = v_d.clone(); Py::Bool({ let t4888 = Py::Int(0i64); if py::gt(&t4887, &t4888)? { true } else { false } }) }; if !t4896.truthy() { t4896 } else { { let t4895 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4895.truthy() { t4895 } else { { let t4894 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4894.truthy() { t4894 } else { { let t4893 = { let t4889 = py::abs(&v_v.clone())?; Py::Bool({ let t4890 = Py::Float(1e-15); if py::lt(&t4889, &t4890)? { true } else { false } }) }; if !t4893.truthy() { t4893 } else { { let t4891 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4892 = Py::Float(1e-15); if py::lt(&t4891, &t4892)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4946(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4948: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4947 = v_inputs.clone(); f_sun_angles(py::getitem(&t4947, &Py::str("latitude"))?, py::getitem(&t4947, &Py::str("declination"))?, py::getitem(&t4947, &Py::str("hour_angle"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4948 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4949 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4978 = (t4949).items()?;
        if t4978.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4978[0].clone();
        v_v = t4978[1].clone();
        if ({ let t4950 = v_k.clone(); Py::Bool({ let t4951 = v_outputs.clone(); if !py::contains(&t4951, &t4950)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4977 = { let t4968 = v_d.clone(); Py::Bool({ let t4969 = Py::Int(0i64); if py::gt(&t4968, &t4969)? { true } else { false } }) }; if !t4977.truthy() { t4977 } else { { let t4976 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4976.truthy() { t4976 } else { { let t4975 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4975.truthy() { t4975 } else { { let t4974 = { let t4970 = py::abs(&v_v.clone())?; Py::Bool({ let t4971 = Py::Float(1e-15); if py::lt(&t4970, &t4971)? { true } else { false } }) }; if !t4974.truthy() { t4974 } else { { let t4972 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4973 = Py::Float(1e-15); if py::lt(&t4972, &t4973)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5017(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5019: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5018 = v_inputs.clone(); f_sunrise_sunset(py::getitem(&t5018, &Py::str("latitude"))?, py::getitem(&t5018, &Py::str("longitude"))?, py::getitem(&t5018, &Py::str("n"))?, py::getitem(&t5018, &Py::str("utc_offset"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5019 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5020 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5049 = (t5020).items()?;
        if t5049.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5049[0].clone();
        v_v = t5049[1].clone();
        if ({ let t5021 = v_k.clone(); Py::Bool({ let t5022 = v_outputs.clone(); if !py::contains(&t5022, &t5021)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5048 = { let t5039 = v_d.clone(); Py::Bool({ let t5040 = Py::Int(0i64); if py::gt(&t5039, &t5040)? { true } else { false } }) }; if !t5048.truthy() { t5048 } else { { let t5047 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5047.truthy() { t5047 } else { { let t5046 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5046.truthy() { t5046 } else { { let t5045 = { let t5041 = py::abs(&v_v.clone())?; Py::Bool({ let t5042 = Py::Float(1e-15); if py::lt(&t5041, &t5042)? { true } else { false } }) }; if !t5045.truthy() { t5045 } else { { let t5043 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5044 = Py::Float(1e-15); if py::lt(&t5043, &t5044)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5137(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5139: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5138 = v_inputs.clone(); f_symmetrical_fault(py::getitem(&t5138, &Py::str("network"))?, py::getitem(&t5138, &Py::str("fault_bus"))?, py::method(&t5138, "get", &[Py::str("Vf"), Py::Float(1.0)])?, py::method(&t5138, "get", &[Py::str("Xf"), Py::Float(0.0)])?, py::method(&t5138, "get", &[Py::str("S_base"), Py::None])?, py::method(&t5138, "get", &[Py::str("V_base"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5139 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5140 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5169 = (t5140).items()?;
        if t5169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5169[0].clone();
        v_v = t5169[1].clone();
        if ({ let t5141 = v_k.clone(); Py::Bool({ let t5142 = v_outputs.clone(); if !py::contains(&t5142, &t5141)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5168 = { let t5159 = v_d.clone(); Py::Bool({ let t5160 = Py::Int(0i64); if py::gt(&t5159, &t5160)? { true } else { false } }) }; if !t5168.truthy() { t5168 } else { { let t5167 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5167.truthy() { t5167 } else { { let t5166 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5166.truthy() { t5166 } else { { let t5165 = { let t5161 = py::abs(&v_v.clone())?; Py::Bool({ let t5162 = Py::Float(1e-15); if py::lt(&t5161, &t5162)? { true } else { false } }) }; if !t5165.truthy() { t5165 } else { { let t5163 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5164 = Py::Float(1e-15); if py::lt(&t5163, &t5164)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_515(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t517: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t516 = v_inputs.clone(); f_cell_temperature_faiman(py::getitem(&t516, &Py::str("poa"))?, py::getitem(&t516, &Py::str("T_air"))?, py::getitem(&t516, &Py::str("wind"))?, py::method(&t516, "get", &[Py::str("u0"), Py::Float(25.0)])?, py::method(&t516, "get", &[Py::str("u1"), Py::Float(6.84)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t517 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t518 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t547 = (t518).items()?;
        if t547.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t547[0].clone();
        v_v = t547[1].clone();
        if ({ let t519 = v_k.clone(); Py::Bool({ let t520 = v_outputs.clone(); if !py::contains(&t520, &t519)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t546 = { let t537 = v_d.clone(); Py::Bool({ let t538 = Py::Int(0i64); if py::gt(&t537, &t538)? { true } else { false } }) }; if !t546.truthy() { t546 } else { { let t545 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t545.truthy() { t545 } else { { let t544 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t544.truthy() { t544 } else { { let t543 = { let t539 = py::abs(&v_v.clone())?; Py::Bool({ let t540 = Py::Float(1e-15); if py::lt(&t539, &t540)? { true } else { false } }) }; if !t543.truthy() { t543 } else { { let t541 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t542 = Py::Float(1e-15); if py::lt(&t541, &t542)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5200(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5202: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5201 = v_inputs.clone(); f_synchronous_generator(py::getitem(&t5201, &Py::str("VL"))?, py::getitem(&t5201, &Py::str("IL"))?, py::getitem(&t5201, &Py::str("pf"))?, py::getitem(&t5201, &Py::str("Xs"))?, py::method(&t5201, "get", &[Py::str("Ra"), Py::Float(0.0)])?, py::method(&t5201, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5202 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5203 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5232 = (t5203).items()?;
        if t5232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5232[0].clone();
        v_v = t5232[1].clone();
        if ({ let t5204 = v_k.clone(); Py::Bool({ let t5205 = v_outputs.clone(); if !py::contains(&t5205, &t5204)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5231 = { let t5222 = v_d.clone(); Py::Bool({ let t5223 = Py::Int(0i64); if py::gt(&t5222, &t5223)? { true } else { false } }) }; if !t5231.truthy() { t5231 } else { { let t5230 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5230.truthy() { t5230 } else { { let t5229 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5229.truthy() { t5229 } else { { let t5228 = { let t5224 = py::abs(&v_v.clone())?; Py::Bool({ let t5225 = Py::Float(1e-15); if py::lt(&t5224, &t5225)? { true } else { false } }) }; if !t5228.truthy() { t5228 } else { { let t5226 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5227 = Py::Float(1e-15); if py::lt(&t5226, &t5227)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5276(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5278: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5277 = v_inputs.clone(); f_machine_speed(py::getitem(&t5277, &Py::str("f"))?, py::getitem(&t5277, &Py::str("poles"))?, py::method(&t5277, "get", &[Py::str("s"), Py::None])?, py::method(&t5277, "get", &[Py::str("n_r"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5278 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5279 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5308 = (t5279).items()?;
        if t5308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5308[0].clone();
        v_v = t5308[1].clone();
        if ({ let t5280 = v_k.clone(); Py::Bool({ let t5281 = v_outputs.clone(); if !py::contains(&t5281, &t5280)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5307 = { let t5298 = v_d.clone(); Py::Bool({ let t5299 = Py::Int(0i64); if py::gt(&t5298, &t5299)? { true } else { false } }) }; if !t5307.truthy() { t5307 } else { { let t5306 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5306.truthy() { t5306 } else { { let t5305 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5305.truthy() { t5305 } else { { let t5304 = { let t5300 = py::abs(&v_v.clone())?; Py::Bool({ let t5301 = Py::Float(1e-15); if py::lt(&t5300, &t5301)? { true } else { false } }) }; if !t5304.truthy() { t5304 } else { { let t5302 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5303 = Py::Float(1e-15); if py::lt(&t5302, &t5303)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5337(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5339: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5338 = v_inputs.clone(); f_three_phase_power(py::getitem(&t5338, &Py::str("VL"))?, py::getitem(&t5338, &Py::str("IL"))?, py::getitem(&t5338, &Py::str("pf"))?, py::getitem(&t5338, &Py::str("connection"))?, py::method(&t5338, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5339 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5340 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5369 = (t5340).items()?;
        if t5369.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5369[0].clone();
        v_v = t5369[1].clone();
        if ({ let t5341 = v_k.clone(); Py::Bool({ let t5342 = v_outputs.clone(); if !py::contains(&t5342, &t5341)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5368 = { let t5359 = v_d.clone(); Py::Bool({ let t5360 = Py::Int(0i64); if py::gt(&t5359, &t5360)? { true } else { false } }) }; if !t5368.truthy() { t5368 } else { { let t5367 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5367.truthy() { t5367 } else { { let t5366 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5366.truthy() { t5366 } else { { let t5365 = { let t5361 = py::abs(&v_v.clone())?; Py::Bool({ let t5362 = Py::Float(1e-15); if py::lt(&t5361, &t5362)? { true } else { false } }) }; if !t5365.truthy() { t5365 } else { { let t5363 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5364 = Py::Float(1e-15); if py::lt(&t5363, &t5364)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5436(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5438: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5437 = v_inputs.clone(); f_transformer_loading(py::getitem(&t5437, &Py::str("sn"))?, py::getitem(&t5437, &Py::str("V_hv"))?, py::getitem(&t5437, &Py::str("V_lv"))?, py::getitem(&t5437, &Py::str("vk"))?, py::getitem(&t5437, &Py::str("vkr"))?, py::getitem(&t5437, &Py::str("P0"))?, py::getitem(&t5437, &Py::str("i0"))?, py::getitem(&t5437, &Py::str("P"))?, py::getitem(&t5437, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5438 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5439 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5468 = (t5439).items()?;
        if t5468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5468[0].clone();
        v_v = t5468[1].clone();
        if ({ let t5440 = v_k.clone(); Py::Bool({ let t5441 = v_outputs.clone(); if !py::contains(&t5441, &t5440)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5467 = { let t5458 = v_d.clone(); Py::Bool({ let t5459 = Py::Int(0i64); if py::gt(&t5458, &t5459)? { true } else { false } }) }; if !t5467.truthy() { t5467 } else { { let t5466 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5466.truthy() { t5466 } else { { let t5465 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5465.truthy() { t5465 } else { { let t5464 = { let t5460 = py::abs(&v_v.clone())?; Py::Bool({ let t5461 = Py::Float(1e-15); if py::lt(&t5460, &t5461)? { true } else { false } }) }; if !t5464.truthy() { t5464 } else { { let t5462 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5463 = Py::Float(1e-15); if py::lt(&t5462, &t5463)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5489(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5491: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5490 = v_inputs.clone(); f_transformer_max_efficiency(py::getitem(&t5490, &Py::str("S"))?, py::getitem(&t5490, &Py::str("P_core"))?, py::getitem(&t5490, &Py::str("P_cu_fl"))?, py::getitem(&t5490, &Py::str("pf"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5491 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5492 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5521 = (t5492).items()?;
        if t5521.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5521[0].clone();
        v_v = t5521[1].clone();
        if ({ let t5493 = v_k.clone(); Py::Bool({ let t5494 = v_outputs.clone(); if !py::contains(&t5494, &t5493)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5520 = { let t5511 = v_d.clone(); Py::Bool({ let t5512 = Py::Int(0i64); if py::gt(&t5511, &t5512)? { true } else { false } }) }; if !t5520.truthy() { t5520 } else { { let t5519 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5519.truthy() { t5519 } else { { let t5518 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5518.truthy() { t5518 } else { { let t5517 = { let t5513 = py::abs(&v_v.clone())?; Py::Bool({ let t5514 = Py::Float(1e-15); if py::lt(&t5513, &t5514)? { true } else { false } }) }; if !t5517.truthy() { t5517 } else { { let t5515 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5516 = Py::Float(1e-15); if py::lt(&t5515, &t5516)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5553(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5555: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5554 = v_inputs.clone(); f_transformer_performance(py::getitem(&t5554, &Py::str("S"))?, py::getitem(&t5554, &Py::str("V1"))?, py::getitem(&t5554, &Py::str("V2"))?, py::getitem(&t5554, &Py::str("Req"))?, py::getitem(&t5554, &Py::str("Xeq"))?, py::getitem(&t5554, &Py::str("Rc"))?, py::getitem(&t5554, &Py::str("Xm"))?, py::getitem(&t5554, &Py::str("load"))?, py::getitem(&t5554, &Py::str("pf"))?, py::method(&t5554, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5555 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5556 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5585 = (t5556).items()?;
        if t5585.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5585[0].clone();
        v_v = t5585[1].clone();
        if ({ let t5557 = v_k.clone(); Py::Bool({ let t5558 = v_outputs.clone(); if !py::contains(&t5558, &t5557)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5584 = { let t5575 = v_d.clone(); Py::Bool({ let t5576 = Py::Int(0i64); if py::gt(&t5575, &t5576)? { true } else { false } }) }; if !t5584.truthy() { t5584 } else { { let t5583 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5583.truthy() { t5583 } else { { let t5582 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5582.truthy() { t5582 } else { { let t5581 = { let t5577 = py::abs(&v_v.clone())?; Py::Bool({ let t5578 = Py::Float(1e-15); if py::lt(&t5577, &t5578)? { true } else { false } }) }; if !t5581.truthy() { t5581 } else { { let t5579 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5580 = Py::Float(1e-15); if py::lt(&t5579, &t5580)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5609(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5611: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5610 = v_inputs.clone(); f_transformer_tests(py::getitem(&t5610, &Py::str("Voc"))?, py::getitem(&t5610, &Py::str("Ioc"))?, py::getitem(&t5610, &Py::str("Poc"))?, py::getitem(&t5610, &Py::str("Vsc"))?, py::getitem(&t5610, &Py::str("Isc"))?, py::getitem(&t5610, &Py::str("Psc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5611 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5612 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5641 = (t5612).items()?;
        if t5641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5641[0].clone();
        v_v = t5641[1].clone();
        if ({ let t5613 = v_k.clone(); Py::Bool({ let t5614 = v_outputs.clone(); if !py::contains(&t5614, &t5613)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5640 = { let t5631 = v_d.clone(); Py::Bool({ let t5632 = Py::Int(0i64); if py::gt(&t5631, &t5632)? { true } else { false } }) }; if !t5640.truthy() { t5640 } else { { let t5639 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5639.truthy() { t5639 } else { { let t5638 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5638.truthy() { t5638 } else { { let t5637 = { let t5633 = py::abs(&v_v.clone())?; Py::Bool({ let t5634 = Py::Float(1e-15); if py::lt(&t5633, &t5634)? { true } else { false } }) }; if !t5637.truthy() { t5637 } else { { let t5635 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5636 = Py::Float(1e-15); if py::lt(&t5635, &t5636)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5669(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5671: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5670 = v_inputs.clone(); f_two_bus_load_flow(py::getitem(&t5670, &Py::str("Vs"))?, py::getitem(&t5670, &Py::str("R"))?, py::getitem(&t5670, &Py::str("X"))?, py::getitem(&t5670, &Py::str("P"))?, py::getitem(&t5670, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5671 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5672 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5701 = (t5672).items()?;
        if t5701.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5701[0].clone();
        v_v = t5701[1].clone();
        if ({ let t5673 = v_k.clone(); Py::Bool({ let t5674 = v_outputs.clone(); if !py::contains(&t5674, &t5673)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5700 = { let t5691 = v_d.clone(); Py::Bool({ let t5692 = Py::Int(0i64); if py::gt(&t5691, &t5692)? { true } else { false } }) }; if !t5700.truthy() { t5700 } else { { let t5699 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5699.truthy() { t5699 } else { { let t5698 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5698.truthy() { t5698 } else { { let t5697 = { let t5693 = py::abs(&v_v.clone())?; Py::Bool({ let t5694 = Py::Float(1e-15); if py::lt(&t5693, &t5694)? { true } else { false } }) }; if !t5697.truthy() { t5697 } else { { let t5695 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5696 = Py::Float(1e-15); if py::lt(&t5695, &t5696)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_569(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t571: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t570 = v_inputs.clone(); f_cell_temperature_noct(py::getitem(&t570, &Py::str("poa"))?, py::getitem(&t570, &Py::str("T_air"))?, py::getitem(&t570, &Py::str("noct"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t571 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t572 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t601 = (t572).items()?;
        if t601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t601[0].clone();
        v_v = t601[1].clone();
        if ({ let t573 = v_k.clone(); Py::Bool({ let t574 = v_outputs.clone(); if !py::contains(&t574, &t573)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t600 = { let t591 = v_d.clone(); Py::Bool({ let t592 = Py::Int(0i64); if py::gt(&t591, &t592)? { true } else { false } }) }; if !t600.truthy() { t600 } else { { let t599 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t599.truthy() { t599 } else { { let t598 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t598.truthy() { t598 } else { { let t597 = { let t593 = py::abs(&v_v.clone())?; Py::Bool({ let t594 = Py::Float(1e-15); if py::lt(&t593, &t594)? { true } else { false } }) }; if !t597.truthy() { t597 } else { { let t595 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t596 = Py::Float(1e-15); if py::lt(&t595, &t596)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5741(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5743: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5742 = v_inputs.clone(); f_weibull_aep(py::getitem(&t5742, &Py::str("speeds"))?, py::getitem(&t5742, &Py::str("powers"))?, py::getitem(&t5742, &Py::str("k"))?, py::getitem(&t5742, &Py::str("c"))?, py::method(&t5742, "get", &[Py::str("hours"), Py::Float(8760.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5743 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5744 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5773 = (t5744).items()?;
        if t5773.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5773[0].clone();
        v_v = t5773[1].clone();
        if ({ let t5745 = v_k.clone(); Py::Bool({ let t5746 = v_outputs.clone(); if !py::contains(&t5746, &t5745)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5772 = { let t5763 = v_d.clone(); Py::Bool({ let t5764 = Py::Int(0i64); if py::gt(&t5763, &t5764)? { true } else { false } }) }; if !t5772.truthy() { t5772 } else { { let t5771 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5771.truthy() { t5771 } else { { let t5770 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5770.truthy() { t5770 } else { { let t5769 = { let t5765 = py::abs(&v_v.clone())?; Py::Bool({ let t5766 = Py::Float(1e-15); if py::lt(&t5765, &t5766)? { true } else { false } }) }; if !t5769.truthy() { t5769 } else { { let t5767 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5768 = Py::Float(1e-15); if py::lt(&t5767, &t5768)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5796(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5798: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5797 = v_inputs.clone(); f_weibull_wind(py::getitem(&t5797, &Py::str("k"))?, py::getitem(&t5797, &Py::str("c"))?, py::getitem(&t5797, &Py::str("rho"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5798 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5799 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5828 = (t5799).items()?;
        if t5828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5828[0].clone();
        v_v = t5828[1].clone();
        if ({ let t5800 = v_k.clone(); Py::Bool({ let t5801 = v_outputs.clone(); if !py::contains(&t5801, &t5800)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5827 = { let t5818 = v_d.clone(); Py::Bool({ let t5819 = Py::Int(0i64); if py::gt(&t5818, &t5819)? { true } else { false } }) }; if !t5827.truthy() { t5827 } else { { let t5826 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5826.truthy() { t5826 } else { { let t5825 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5825.truthy() { t5825 } else { { let t5824 = { let t5820 = py::abs(&v_v.clone())?; Py::Bool({ let t5821 = Py::Float(1e-15); if py::lt(&t5820, &t5821)? { true } else { false } }) }; if !t5824.truthy() { t5824 } else { { let t5822 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5823 = Py::Float(1e-15); if py::lt(&t5822, &t5823)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5852(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5854: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5853 = v_inputs.clone(); f_wind_power(py::getitem(&t5853, &Py::str("rho"))?, py::getitem(&t5853, &Py::str("D"))?, py::getitem(&t5853, &Py::str("v"))?, py::method(&t5853, "get", &[Py::str("cp"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5854 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5855 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5884 = (t5855).items()?;
        if t5884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5884[0].clone();
        v_v = t5884[1].clone();
        if ({ let t5856 = v_k.clone(); Py::Bool({ let t5857 = v_outputs.clone(); if !py::contains(&t5857, &t5856)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5883 = { let t5874 = v_d.clone(); Py::Bool({ let t5875 = Py::Int(0i64); if py::gt(&t5874, &t5875)? { true } else { false } }) }; if !t5883.truthy() { t5883 } else { { let t5882 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5882.truthy() { t5882 } else { { let t5881 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5881.truthy() { t5881 } else { { let t5880 = { let t5876 = py::abs(&v_v.clone())?; Py::Bool({ let t5877 = Py::Float(1e-15); if py::lt(&t5876, &t5877)? { true } else { false } }) }; if !t5880.truthy() { t5880 } else { { let t5878 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5879 = Py::Float(1e-15); if py::lt(&t5878, &t5879)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5912(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5914: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5913 = v_inputs.clone(); f_wind_shear_log(py::getitem(&t5913, &Py::str("v1"))?, py::getitem(&t5913, &Py::str("h1"))?, py::getitem(&t5913, &Py::str("h2"))?, py::getitem(&t5913, &Py::str("z0"))?, py::method(&t5913, "get", &[Py::str("d"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5914 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5915 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5944 = (t5915).items()?;
        if t5944.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5944[0].clone();
        v_v = t5944[1].clone();
        if ({ let t5916 = v_k.clone(); Py::Bool({ let t5917 = v_outputs.clone(); if !py::contains(&t5917, &t5916)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5943 = { let t5934 = v_d.clone(); Py::Bool({ let t5935 = Py::Int(0i64); if py::gt(&t5934, &t5935)? { true } else { false } }) }; if !t5943.truthy() { t5943 } else { { let t5942 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5942.truthy() { t5942 } else { { let t5941 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5941.truthy() { t5941 } else { { let t5940 = { let t5936 = py::abs(&v_v.clone())?; Py::Bool({ let t5937 = Py::Float(1e-15); if py::lt(&t5936, &t5937)? { true } else { false } }) }; if !t5940.truthy() { t5940 } else { { let t5938 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5939 = Py::Float(1e-15); if py::lt(&t5938, &t5939)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5965(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5967: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5966 = v_inputs.clone(); f_wind_shear_power_law(py::getitem(&t5966, &Py::str("v1"))?, py::getitem(&t5966, &Py::str("h1"))?, py::getitem(&t5966, &Py::str("h2"))?, py::method(&t5966, "get", &[Py::str("alpha"), py::truediv(&Py::Int(1i64), &Py::Int(7i64))?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5967 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5968 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5997 = (t5968).items()?;
        if t5997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5997[0].clone();
        v_v = t5997[1].clone();
        if ({ let t5969 = v_k.clone(); Py::Bool({ let t5970 = v_outputs.clone(); if !py::contains(&t5970, &t5969)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5996 = { let t5987 = v_d.clone(); Py::Bool({ let t5988 = Py::Int(0i64); if py::gt(&t5987, &t5988)? { true } else { false } }) }; if !t5996.truthy() { t5996 } else { { let t5995 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5995.truthy() { t5995 } else { { let t5994 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5994.truthy() { t5994 } else { { let t5993 = { let t5989 = py::abs(&v_v.clone())?; Py::Bool({ let t5990 = Py::Float(1e-15); if py::lt(&t5989, &t5990)? { true } else { false } }) }; if !t5993.truthy() { t5993 } else { { let t5991 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5992 = Py::Float(1e-15); if py::lt(&t5991, &t5992)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_624(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t626: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t625 = v_inputs.clone(); f_cell_temperature_sapm(py::getitem(&t625, &Py::str("poa"))?, py::getitem(&t625, &Py::str("T_air"))?, py::getitem(&t625, &Py::str("wind"))?, py::getitem(&t625, &Py::str("mount"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t626 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t627 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t656 = (t627).items()?;
        if t656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t656[0].clone();
        v_v = t656[1].clone();
        if ({ let t628 = v_k.clone(); Py::Bool({ let t629 = v_outputs.clone(); if !py::contains(&t629, &t628)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t655 = { let t646 = v_d.clone(); Py::Bool({ let t647 = Py::Int(0i64); if py::gt(&t646, &t647)? { true } else { false } }) }; if !t655.truthy() { t655 } else { { let t654 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t654.truthy() { t654 } else { { let t653 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t653.truthy() { t653 } else { { let t652 = { let t648 = py::abs(&v_v.clone())?; Py::Bool({ let t649 = Py::Float(1e-15); if py::lt(&t648, &t649)? { true } else { false } }) }; if !t652.truthy() { t652 } else { { let t650 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t651 = Py::Float(1e-15); if py::lt(&t650, &t651)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_675(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t677: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t676 = v_inputs.clone(); f_haurwitz(py::getitem(&t676, &Py::str("zenith"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t677 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t678 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t707 = (t678).items()?;
        if t707.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t707[0].clone();
        v_v = t707[1].clone();
        if ({ let t679 = v_k.clone(); Py::Bool({ let t680 = v_outputs.clone(); if !py::contains(&t680, &t679)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t706 = { let t697 = v_d.clone(); Py::Bool({ let t698 = Py::Int(0i64); if py::gt(&t697, &t698)? { true } else { false } }) }; if !t706.truthy() { t706 } else { { let t705 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t705.truthy() { t705 } else { { let t704 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t704.truthy() { t704 } else { { let t703 = { let t699 = py::abs(&v_v.clone())?; Py::Bool({ let t700 = Py::Float(1e-15); if py::lt(&t699, &t700)? { true } else { false } }) }; if !t703.truthy() { t703 } else { { let t701 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t702 = Py::Float(1e-15); if py::lt(&t701, &t702)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_750(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t752: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t751 = v_inputs.clone(); f_clear_sky_ineichen(py::getitem(&t751, &Py::str("zenith"))?, py::getitem(&t751, &Py::str("linke_turbidity"))?, py::getitem(&t751, &Py::str("altitude"))?, py::getitem(&t751, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t752 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t753 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t782 = (t753).items()?;
        if t782.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t782[0].clone();
        v_v = t782[1].clone();
        if ({ let t754 = v_k.clone(); Py::Bool({ let t755 = v_outputs.clone(); if !py::contains(&t755, &t754)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t781 = { let t772 = v_d.clone(); Py::Bool({ let t773 = Py::Int(0i64); if py::gt(&t772, &t773)? { true } else { false } }) }; if !t781.truthy() { t781 } else { { let t780 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t780.truthy() { t780 } else { { let t779 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t779.truthy() { t779 } else { { let t778 = { let t774 = py::abs(&v_v.clone())?; Py::Bool({ let t775 = Py::Float(1e-15); if py::lt(&t774, &t775)? { true } else { false } }) }; if !t778.truthy() { t778 } else { { let t776 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t777 = Py::Float(1e-15); if py::lt(&t776, &t777)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_816(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t818: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t817 = v_inputs.clone(); f_coulomb_counting(py::getitem(&t817, &Py::str("SOC0"))?, py::getitem(&t817, &Py::str("Q"))?, py::getitem(&t817, &Py::str("currents"))?, py::getitem(&t817, &Py::str("dt"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t818 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t819 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t848 = (t819).items()?;
        if t848.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t848[0].clone();
        v_v = t848[1].clone();
        if ({ let t820 = v_k.clone(); Py::Bool({ let t821 = v_outputs.clone(); if !py::contains(&t821, &t820)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t847 = { let t838 = v_d.clone(); Py::Bool({ let t839 = Py::Int(0i64); if py::gt(&t838, &t839)? { true } else { false } }) }; if !t847.truthy() { t847 } else { { let t846 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t846.truthy() { t846 } else { { let t845 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t845.truthy() { t845 } else { { let t844 = { let t840 = py::abs(&v_v.clone())?; Py::Bool({ let t841 = Py::Float(1e-15); if py::lt(&t840, &t841)? { true } else { false } }) }; if !t844.truthy() { t844 } else { { let t842 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t843 = Py::Float(1e-15); if py::lt(&t842, &t843)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_896(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t898: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t897 = v_inputs.clone(); f_critical_clearing(py::getitem(&t897, &Py::str("Pm"))?, py::getitem(&t897, &Py::str("Pmax_pre"))?, py::getitem(&t897, &Py::str("Pmax_fault"))?, py::getitem(&t897, &Py::str("Pmax_post"))?, py::method(&t897, "get", &[Py::str("H"), Py::None])?, py::method(&t897, "get", &[Py::str("f"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t898 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t899 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t928 = (t899).items()?;
        if t928.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t928[0].clone();
        v_v = t928[1].clone();
        if ({ let t900 = v_k.clone(); Py::Bool({ let t901 = v_outputs.clone(); if !py::contains(&t901, &t900)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t927 = { let t918 = v_d.clone(); Py::Bool({ let t919 = Py::Int(0i64); if py::gt(&t918, &t919)? { true } else { false } }) }; if !t927.truthy() { t927 } else { { let t926 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t926.truthy() { t926 } else { { let t925 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t925.truthy() { t925 } else { { let t924 = { let t920 = py::abs(&v_v.clone())?; Py::Bool({ let t921 = Py::Float(1e-15); if py::lt(&t920, &t921)? { true } else { false } }) }; if !t924.truthy() { t924 } else { { let t922 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t923 = Py::Float(1e-15); if py::lt(&t922, &t923)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_91(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t93: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t92 = v_inputs.clone(); f_air_mass(py::getitem(&t92, &Py::str("zenith"))?, py::method(&t92, "get", &[Py::str("model"), Py::str("kastenyoung1989")])?, py::method(&t92, "get", &[Py::str("pressure"), Py::Float(101325.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t93 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t94 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t123 = (t94).items()?;
        if t123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t123[0].clone();
        v_v = t123[1].clone();
        if ({ let t95 = v_k.clone(); Py::Bool({ let t96 = v_outputs.clone(); if !py::contains(&t96, &t95)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t122 = { let t113 = v_d.clone(); Py::Bool({ let t114 = Py::Int(0i64); if py::gt(&t113, &t114)? { true } else { false } }) }; if !t122.truthy() { t122 } else { { let t121 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t121.truthy() { t121 } else { { let t120 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t120.truthy() { t120 } else { { let t119 = { let t115 = py::abs(&v_v.clone())?; Py::Bool({ let t116 = Py::Float(1e-15); if py::lt(&t115, &t116)? { true } else { false } }) }; if !t119.truthy() { t119 } else { { let t117 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t118 = Py::Float(1e-15); if py::lt(&t117, &t118)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_994(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t996: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t995 = v_inputs.clone(); f_dc_power_flow(py::getitem(&t995, &Py::str("lines"))?, py::getitem(&t995, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t996 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t997 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1026 = (t997).items()?;
        if t1026.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1026[0].clone();
        v_v = t1026[1].clone();
        if ({ let t998 = v_k.clone(); Py::Bool({ let t999 = v_outputs.clone(); if !py::contains(&t999, &t998)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1025 = { let t1016 = v_d.clone(); Py::Bool({ let t1017 = Py::Int(0i64); if py::gt(&t1016, &t1017)? { true } else { false } }) }; if !t1025.truthy() { t1025 } else { { let t1024 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1024.truthy() { t1024 } else { { let t1023 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1023.truthy() { t1023 } else { { let t1022 = { let t1018 = py::abs(&v_v.clone())?; Py::Bool({ let t1019 = Py::Float(1e-15); if py::lt(&t1018, &t1019)? { true } else { false } }) }; if !t1022.truthy() { t1022 } else { { let t1020 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1021 = Py::Float(1e-15); if py::lt(&t1020, &t1021)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
