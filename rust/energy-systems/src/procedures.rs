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
    "energy_hydrogen_isentropic_compression",
    "energy_hydrogen_isothermal_compression",
    "energy_hydrogen_tank",
    "energy_iam_ashrae",
    "energy_iam_physical",
    "energy_induction_max_torque",
    "energy_induction_motor",
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
        "energy_cell_temperature_faiman" => {
            let r = f_cell_temperature_faiman(match v.get("poa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wind") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(25.0)) })().ok()? } }, match v.get("u1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(6.84)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_323(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_cell_temperature_noct" => {
            let r = f_cell_temperature_noct(match v.get("poa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("noct") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_377(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_cell_temperature_sapm" => {
            let r = f_cell_temperature_sapm(match v.get("poa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wind") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mount") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_432(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_clear_sky_haurwitz" => {
            let r = f_haurwitz(match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_clear_sky_ineichen" => {
            let r = f_clear_sky_ineichen(match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("linke_turbidity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("altitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_558(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_coulomb_counting" => {
            let r = f_coulomb_counting(match v.get("SOC0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("currents") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_624(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_critical_clearing" => {
            let r = f_critical_clearing(match v.get("Pm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pmax_pre") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pmax_fault") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pmax_post") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("f") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_704(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_dc_power_flow" => {
            let r = f_dc_power_flow(match v.get("lines") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_802(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_desoto_parameters" => {
            let r = f_desoto_parameters(match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha_sc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EgRef") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.121)) })().ok()? } }, match v.get("dEgdT") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(py::neg(&Py::Float(0.0002677))?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_862(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_disc_dni" => {
            let r = f_disc(match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pressure") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(101325.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_936(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_dlg_fault" => {
            let r = s_969(match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Xf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1007(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_droop_sharing" => {
            let r = f_droop_sharing(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ratings") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1078(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_economic_dispatch" => {
            let r = f_economic_dispatch(match v.get("costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1180(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_electrolyser" => {
            let r = f_electrolyser(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("faradaic") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1239(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_equation_of_time" => {
            let r = s_1272(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("spencer")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1291(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_erbs_decomposition" => {
            let r = f_erbs(match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1353(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_extraterrestrial_irradiance" => {
            let r = f_extraterrestrial_irradiance(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("spencer")) })().ok()? } }, match v.get("solar_constant") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(c_SOLAR_CONSTANT()?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1409(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_fuel_cell_stack" => {
            let r = f_fuel_cell_stack(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("utilization") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1467(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_geometric_mean_distance" => {
            let r = f_gmd(match v.get("D_ab") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_bc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_ca") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1525(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hub_air_density" => {
            let r = f_hub_air_density(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hub") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("barometric")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1586(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hydrogen_isentropic_compression" => {
            let r = f_hydrogen_isentropic_compression(match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1687(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hydrogen_isothermal_compression" => {
            let r = f_hydrogen_isothermal_compression(match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1780(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_hydrogen_tank" => {
            let r = f_hydrogen_tank(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1874(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_iam_ashrae" => {
            let r = f_iam_ashrae(match v.get("aoi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.05)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1928(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_iam_physical" => {
            let r = f_iam_physical(match v.get("aoi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.526)) })().ok()? } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(4.0)) })().ok()? } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.002)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1985(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_induction_max_torque" => {
            let r = f_induction_max_torque(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("connection") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2055(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_induction_motor" => {
            let r = f_induction_motor(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("slip") { Some(x) => py::from_val(x), None => { return None } }, match v.get("connection") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_rot") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2124(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_line_parameters" => {
            let r = f_line_parameters(match v.get("D_ab") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_bc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_ca") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gmr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n_bundle") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("spacing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2202(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_ll_fault" => {
            let r = s_2235(match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Xf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2273(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_long_line" => {
            let r = f_long_line(match v.get("Vr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2346(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_medium_line" => {
            let r = f_medium_line(match v.get("Vr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_newton_load_flow" => {
            let r = f_newton_load_flow(match v.get("lines") { Some(x) => py::from_val(x), None => { return None } }, match v.get("loads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gens") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2619(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_parallel_transformers" => {
            let r = f_parallel_transformers(match v.get("S1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vk1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vkr1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vk2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vkr2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_lv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2677(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_per_unit_change_of_base" => {
            let r = f_per_unit_change_of_base(match v.get("R_pu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X_pu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S_old") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_old") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S_new") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_new") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2732(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_per_unit_impedance" => {
            let r = f_per_unit_impedance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S_base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_base") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2792(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_peukert" => {
            let r = f_peukert(match v.get("currents") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2865(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_haydavies" => {
            let r = s_2898(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2950(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_isotropic" => {
            let r = s_2983(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3035(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_klucher" => {
            let r = s_3068(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3120(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_perez" => {
            let r = f_poa_perez(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3214(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_poa_reindl" => {
            let r = s_3247(match v.get("tilt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zenith") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ghi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3299(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_power_angle" => {
            let r = f_power_angle(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3351(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_power_curve_density_corrected" => {
            let r = f_power_curve_density_corrected(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("powers") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3438(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_power_curve_output" => {
            let r = f_power_curve_output(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("powers") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3513(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pv_array_output" => {
            let r = f__pv_array_check(match v.get("poa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("noct") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pdc0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3577(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pv_module_at_conditions" => {
            let r = f_pv_module_at_conditions(match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha_sc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EgRef") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.121)) })().ok()? } }, match v.get("dEgdT") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(py::neg(&Py::Float(0.0002677))?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3683(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pvwatts_dc" => {
            let r = f_pvwatts_dc(match v.get("G_eff") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pdc0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3745(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pvwatts_inverter" => {
            let r = f_pvwatts_inverter(match v.get("pdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pdc0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta_nom") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.96)) })().ok()? } }, match v.get("eta_ref") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.9637)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3804(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_pvwatts_losses" => {
            let r = f_pvwatts_losses({ let mut d = vec![]; for k in ["soiling", "shading", "snow", "mismatch", "wiring", "connections", "lid", "nameplate_rating", "age", "availability"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3855(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_short_line" => {
            let r = f_short_line(match v.get("Vr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3917(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_single_diode_current" => {
            let r = f_single_diode_current(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nNsVth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4005(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_single_diode_mpp" => {
            let r = f_single_diode_mpp(match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nNsVth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4107(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_single_diode_voltage" => {
            let r = f_single_diode_voltage(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nNsVth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4198(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_slg_fault" => {
            let r = s_4231(match v.get("X1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Xf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4269(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_solar_declination" => {
            let r = s_4302(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("cooper")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4321(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_solar_position" => {
            let r = f_solar_position(match v.get("latitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("longitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("clock") { Some(x) => py::from_val(x), None => { return None } }, match v.get("utc_offset") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4403(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_solar_time_hour_angle" => {
            let r = f_solar_time_hour_angle(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("clock") { Some(x) => py::from_val(x), None => { return None } }, match v.get("longitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("utc_offset") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_stack_electrical_power" => {
            let r = f_stack_power(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_cell") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4529(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_sun_angles" => {
            let r = f_sun_angles(match v.get("latitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("declination") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hour_angle") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4610(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_sunrise_sunset" => {
            let r = f_sunrise_sunset(match v.get("latitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("longitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("utc_offset") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4681(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_symmetrical_fault" => {
            let r = f_symmetrical_fault(match v.get("network") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fault_bus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("Xf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("S_base") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V_base") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4801(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_synchronous_generator" => {
            let r = f_synchronous_generator(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4864(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_synchronous_speed" => {
            let r = f_machine_speed(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4940(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_three_phase_power" => {
            let r = f_three_phase_power(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("connection") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5001(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_transformer_loading" => {
            let r = f_transformer_loading(match v.get("sn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_hv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_lv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vkr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5100(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_transformer_max_efficiency" => {
            let r = f_transformer_max_efficiency(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_core") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_cu_fl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5153(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_transformer_performance" => {
            let r = f_transformer_performance(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Req") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xeq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("load") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("lagging")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5217(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_transformer_tests" => {
            let r = f_transformer_tests(match v.get("Voc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ioc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Poc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vsc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Isc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Psc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5273(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_two_bus_load_flow" => {
            let r = f_two_bus_load_flow(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5333(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_weibull_aep" => {
            let r = f_weibull_aep(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("powers") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hours") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(8760.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_weibull_wind" => {
            let r = f_weibull_wind(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5460(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_wind_power" => {
            let r = f_wind_power(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5516(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_wind_shear_log" => {
            let r = f_wind_shear_log(match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5576(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "energy_wind_shear_power_law" => {
            let r = f_wind_shear_power_law(match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(py::truediv(&Py::Int(1i64), &Py::Int(7i64))?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5629(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    if ({ let t3184 = v_model.clone(); Py::Bool({ let t3185 = Py::str("kastenyoung1989"); if py::eq(&t3184, &t3185) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &py::add(&v_c.clone(), &py::mul(&Py::Float(0.50572), &py::pow(&py::add(&Py::Float(6.07995), &py::sub(&Py::Int(90i64), &v_z.clone())?)?, &py::neg(&Py::Float(1.6364))?)?)?)?)?);
    }
    if ({ let t3186 = v_model.clone(); Py::Bool({ let t3187 = Py::str("kasten1966"); if py::eq(&t3186, &t3187) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &py::add(&v_c.clone(), &py::mul(&Py::Float(0.15), &py::pow(&py::sub(&Py::Float(93.885), &v_z.clone())?, &py::neg(&Py::Float(1.253))?)?)?)?)?);
    }
    if ({ let t3188 = v_model.clone(); Py::Bool({ let t3189 = Py::str("simple"); if py::eq(&t3188, &t3189) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &v_c.clone())?);
    }
    if ({ let t3190 = v_model.clone(); Py::Bool({ let t3191 = Py::str("pickering2002"); if py::eq(&t3190, &t3191) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &py::math1("sin", &py::math1("radians", &py::add(&py::sub(&Py::Int(90i64), &v_z.clone())?, &py::truediv(&Py::Float(244.0), &py::add(&Py::Int(165i64), &py::mul(&Py::Float(47.0), &py::pow(&py::sub(&Py::Int(90i64), &v_z.clone())?, &Py::Float(1.1))?)?)?)?)?)?)?)?);
    }
    if ({ let t3192 = v_model.clone(); Py::Bool({ let t3193 = Py::str("youngirvine1967"); if py::eq(&t3192, &t3193) { true } else { false } }) }).truthy() {
        v_s = py::truediv(&Py::Float(1.0), &v_c.clone())?;
        return Ok(py::mul(&v_s.clone(), &py::sub(&Py::Int(1i64), &py::mul(&Py::Float(0.0012), &py::sub(&py::mul(&v_s.clone(), &v_s.clone())?, &Py::Int(1i64))?)?)?)?);
    }
    if ({ let t3194 = v_model.clone(); Py::Bool({ let t3195 = Py::str("young1994"); if py::eq(&t3194, &t3195) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::add(&py::add(&py::mul(&Py::Float(1.002432), &py::pow(&v_c.clone(), &Py::Int(2i64))?)?, &py::mul(&Py::Float(0.148386), &v_c.clone())?)?, &Py::Float(0.0096467))?, &py::add(&py::add(&py::add(&py::pow(&v_c.clone(), &Py::Int(3i64))?, &py::mul(&Py::Float(0.149864), &py::pow(&v_c.clone(), &Py::Int(2i64))?)?)?, &py::mul(&Py::Float(0.0102963), &v_c.clone())?)?, &Py::Float(0.000303978))?)?);
    }
    if ({ let t3196 = v_model.clone(); Py::Bool({ let t3197 = Py::str("gueymard1993"); if py::eq(&t3196, &t3197) { true } else { false } }) }).truthy() {
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
    let t4160 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t4160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t4160[0].clone();
    v_fb = t4160[1].clone();
    if ({ let t4161 = v_fa.clone(); Py::Bool({ let t4162 = Py::Int(0i64); if py::eq(&t4161, &t4162) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t4163 = v_fb.clone(); Py::Bool({ let t4164 = Py::Int(0i64); if py::eq(&t4163, &t4164) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t4165 = { let t4167 = v_fa.clone(); Py::Bool({ let t4168 = Py::Int(0i64); if py::gt(&t4167, &t4168)? { true } else { false } }) }; Py::Bool({ let t4166 = { let t4169 = v_fb.clone(); Py::Bool({ let t4170 = Py::Int(0i64); if py::gt(&t4169, &t4170)? { true } else { false } }) }; if py::ne(&t4165, &t4166) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t4171 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t4171;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t4176 = { let t4172 = v_m.clone(); Py::Bool({ let t4173 = v_a.clone(); if py::eq(&t4172, &t4173) { true } else { false } }) }; if t4176.truthy() { t4176 } else { { let t4174 = v_m.clone(); Py::Bool({ let t4175 = v_b.clone(); if py::eq(&t4174, &t4175) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t4177 = v_fm.clone(); Py::Bool({ let t4178 = Py::Int(0i64); if py::eq(&t4177, &t4178) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t4179 = { let t4181 = v_fm.clone(); Py::Bool({ let t4182 = Py::Int(0i64); if py::gt(&t4181, &t4182)? { true } else { false } }) }; Py::Bool({ let t4180 = { let t4183 = v_fa.clone(); Py::Bool({ let t4184 = Py::Int(0i64); if py::gt(&t4183, &t4184)? { true } else { false } }) }; if py::eq(&t4179, &t4180) { true } else { false } }) }).truthy() {
            let t4185 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t4185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4185[0].clone();
            v_fa = t4185[1].clone();
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
    if ({ let t2178 = v_n.clone(); Py::Bool({ let t2179 = Py::Int(1i64); if py::eq(&t2178, &t2179) { true } else { false } }) }).truthy() {
        return Ok(v_r_or_gmr.clone());
    }
    if ({ let t2180 = v_n.clone(); Py::Bool({ let t2181 = Py::Int(2i64); if py::eq(&t2180, &t2181) { true } else { false } }) }).truthy() {
        return Ok(py::math1("sqrt", &py::mul(&v_r_or_gmr.clone(), &v_d.clone())?)?);
    }
    if ({ let t2182 = v_n.clone(); Py::Bool({ let t2183 = Py::Int(3i64); if py::eq(&t2182, &t2183) { true } else { false } }) }).truthy() {
        return Ok(py::pow(&py::mul(&py::mul(&v_r_or_gmr.clone(), &v_d.clone())?, &v_d.clone())?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?);
    }
    return Ok(py::mul(&py::pow(&Py::Int(2i64), &Py::Float(0.125))?, &py::pow(&py::mul(&v_r_or_gmr.clone(), &py::pow(&v_d.clone(), &Py::Int(3i64))?)?, &Py::Float(0.25))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bus_index(mut v_name: Py, mut v_n: Py) -> R {
    let _ = f__need({ let t4717 = ax_engine::pyre::module_call("fullmatch", &Py::str("\\d+"), &[Py::str(&py::to_str(&v_name.clone()))], 0)?; Py::Bool({ let t4718 = Py::None; if !py::is(&t4717, &t4718) { true } else { false } }) }, Py::str(&["bus names must be whole numbers 1, 2, 3, ... (got ".to_string(), py::format(&Py::str(&py::repr(&v_name.clone())), "")?, ")".to_string()].concat()))?;
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
    let _ = f__need({ let t5366 = py::len(&v_speeds.clone())?; Py::Bool({ let t5367 = py::len(&v_powers.clone())?; if py::eq(&t5366, &t5367) { true } else { false } }) }, Py::str("give one power for every wind speed of the curve"))?;
    let _ = f__need({ let t5368 = py::len(&v_speeds.clone())?; Py::Bool({ let t5369 = Py::Int(2i64); if py::ge(&t5368, &t5369)? { true } else { false } }) }, Py::str("the power curve needs at least two points"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5370 in py::iter(&(py::zip(&[v_speeds.clone(), py::slice(&v_speeds.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5371 = (t5370).items()?;
if t5371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5371[0].clone();
v_b = t5371[1].clone();
out.push({ let t5372 = v_b.clone(); Py::Bool({ let t5373 = v_a.clone(); if py::gt(&t5372, &t5373)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the wind speeds of the curve must increase strictly"))?;
    let _ = f__need({ let t5379 = { let t5374 = py::getitem(&v_speeds.clone(), &Py::Int(0i64))?; Py::Bool({ let t5375 = Py::Int(0i64); if py::ge(&t5374, &t5375)? { true } else { false } }) }; if !t5379.truthy() { t5379 } else { Py::Bool(({ let mut out = vec![]; for t5376 in py::iter(&(v_powers.clone()))? { let mut v_p: Py; 
v_p = t5376;
out.push({ let t5377 = v_p.clone(); Py::Bool({ let t5378 = Py::Int(0i64); if py::ge(&t5377, &t5378)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("speeds and powers must not be negative"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_day(mut v_n: Py) -> R {
    let _ = f__need({ let t4646 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4646.truthy() { t4646 } else { { let t4643 = Py::Int(1i64); Py::Bool({ let t4644 = v_n.clone(); if py::le(&t4643, &t4644)? { { let t4645 = Py::Int(366i64); if py::le(&t4644, &t4645)? { true } else { false } } } else { false } }) } } }, Py::str("the day of the year n must be a whole number from 1 to 366"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_im(mut v_VL: Py, mut v_f: Py, mut v_poles: Py, mut v_R1: Py, mut v_X1: Py, mut v_R2: Py, mut v_X2: Py, mut v_Xm: Py) -> R {
    let _ = f__pos(Py::str("the line voltage and frequency"), Py::tuple(vec![v_VL.clone(), v_f.clone()]))?;
    let _ = f__need({ let t2098 = Py::Bool(matches!(v_poles.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2098.truthy() { t2098 } else { { let t2097 = { let t2093 = v_poles.clone(); Py::Bool({ let t2094 = Py::Int(2i64); if py::ge(&t2093, &t2094)? { true } else { false } }) }; if !t2097.truthy() { t2097 } else { { let t2095 = py::modulo(&v_poles.clone(), &Py::Int(2i64))?; Py::Bool({ let t2096 = Py::Int(0i64); if py::eq(&t2095, &t2096) { true } else { false } }) } } } } }, Py::str("the number of poles must be an even whole number (2, 4, 6, ...)"))?;
    let _ = f__nonneg(Py::str("the stator resistance and reactance"), Py::tuple(vec![v_R1.clone(), v_X1.clone()]))?;
    let _ = f__pos(Py::str("the rotor resistance and reactance and the magnetizing reactance"), Py::tuple(vec![v_R2.clone(), v_X2.clone(), v_Xm.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_irr(mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py) -> R {
    let _ = f__nonneg(Py::str("the irradiances"), Py::tuple(vec![v_dni.clone(), v_ghi.clone(), v_dhi.clone()]))?;
    let _ = f__need({ let t3268 = v_dhi.clone(); Py::Bool({ let t3269 = py::add(&v_ghi.clone(), &Py::Float(1e-09))?; if py::le(&t3268, &t3269)? { true } else { false } }) }, Py::str("the diffuse horizontal irradiance cannot exceed the global horizontal irradiance"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_place(mut v_latitude: Py, mut v_longitude: Py, mut v_utc_offset: Py) -> R {
    if ({ let t4647 = v_latitude.clone(); Py::Bool({ let t4648 = Py::None; if !py::is(&t4647, &t4648) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4652 = Py::Bool(matches!(v_latitude.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4652.truthy() { t4652 } else { { let t4649 = py::neg(&Py::Int(90i64))?; Py::Bool({ let t4650 = v_latitude.clone(); if py::lt(&t4649, &t4650)? { { let t4651 = Py::Int(90i64); if py::lt(&t4650, &t4651)? { true } else { false } } } else { false } }) } } }, Py::str("the latitude must lie between -90 and 90 degrees"))?;
    }
    if ({ let t4653 = v_longitude.clone(); Py::Bool({ let t4654 = Py::None; if !py::is(&t4653, &t4654) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4658 = Py::Bool(matches!(v_longitude.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4658.truthy() { t4658 } else { { let t4655 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t4656 = v_longitude.clone(); if py::le(&t4655, &t4656)? { { let t4657 = Py::Int(180i64); if py::le(&t4656, &t4657)? { true } else { false } } } else { false } }) } } }, Py::str("the longitude must lie between -180 and 180 degrees (east positive)"))?;
    }
    if ({ let t4659 = v_utc_offset.clone(); Py::Bool({ let t4660 = Py::None; if !py::is(&t4659, &t4660) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4664 = Py::Bool(matches!(v_utc_offset.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4664.truthy() { t4664 } else { { let t4661 = py::neg(&Py::Int(12i64))?; Py::Bool({ let t4662 = v_utc_offset.clone(); if py::le(&t4661, &t4662)? { { let t4663 = Py::Int(14i64); if py::le(&t4662, &t4663)? { true } else { false } } } else { false } }) } } }, Py::str("the UTC offset must lie between -12 and +14 hours"))?;
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
    let _ = f__need({ let t3251 = Py::Bool(matches!(v_tilt.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3251.truthy() { t3251 } else { { let t3248 = Py::Int(0i64); Py::Bool({ let t3249 = v_tilt.clone(); if py::le(&t3248, &t3249)? { { let t3250 = Py::Int(180i64); if py::le(&t3249, &t3250)? { true } else { false } } } else { false } }) } } }, Py::str("the tilt must lie between 0 and 180 degrees"))?;
    let _ = f__need({ let t3255 = Py::Bool(matches!(v_saz.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3255.truthy() { t3255 } else { { let t3252 = Py::Int(0i64); Py::Bool({ let t3253 = v_saz.clone(); if py::le(&t3252, &t3253)? { { let t3254 = Py::Int(360i64); if py::le(&t3253, &t3254)? { true } else { false } } } else { false } }) } } }, Py::str("the surface azimuth must lie between 0 and 360 degrees (clockwise from north)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_temp(mut v_name: Py, mut v_T: Py) -> R {
    let _ = f__need({ let t3724 = Py::Bool(matches!(v_T.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3724.truthy() { t3724 } else { { let t3721 = Py::Int(150i64); Py::Bool({ let t3722 = v_T.clone(); if py::le(&t3721, &t3722)? { { let t3723 = Py::Int(400i64); if py::le(&t3722, &t3723)? { true } else { false } } } else { false } }) } } }, Py::str(&["the ".to_string(), py::format(&v_name.clone(), "")?, " must lie between 150 K and 400 K".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_weibull(mut v_k: Py, mut v_c: Py) -> R {
    let _ = f__need({ let t5441 = Py::Bool(matches!(v_k.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5441.truthy() { t5441 } else { { let t5438 = Py::Float(1.0); Py::Bool({ let t5439 = v_k.clone(); if py::lt(&t5438, &t5439)? { { let t5440 = Py::Float(5.0); if py::le(&t5439, &t5440)? { true } else { false } } } else { false } }) } } }, Py::str("the Weibull shape k must lie in (1, 5]"))?;
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
    let _ = f__need({ let t3615 = Py::Int(200i64); Py::Bool({ let t3616 = v_T_cell.clone(); if py::le(&t3615, &t3616)? { { let t3617 = Py::Int(400i64); if py::le(&t3616, &t3617)? { true } else { false } } } else { false } }) }, Py::str("the cell temperature must lie between 200 K and 400 K"))?;
    let _ = f__pos(Py::str("a_ref, IL_ref, I0_ref, Rsh_ref and EgRef"), Py::tuple(vec![v_a_ref.clone(), v_IL_ref.clone(), v_I0_ref.clone(), v_Rsh_ref.clone(), v_EgRef.clone()]))?;
    let _ = f__nonneg(Py::str("Rs"), Py::tuple(vec![v_Rs.clone()]))?;
    let t3623 = (Py::tuple(vec![py::add(&Py::Int(25i64), &Py::Float(273.15))?, py::add(&f__degC(v_T_cell.clone())?, &Py::Float(273.15))?])).items()?;
    if t3623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Tref = t3623[0].clone();
    v_Tc = t3623[1].clone();
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
    for t4714 in py::iter(&(v_network.clone()))? {
        v_e = t4714;
        let _ = f__need({ let t4715 = py::len(&v_e.clone())?; Py::Bool({ let t4716 = Py::Int(3i64); if py::eq(&t4715, &t4716) { true } else { false } }) }, Py::str("each branch is 'i-j x' with its reactance in per unit; node 0 is the reference"))?;
        let t4719 = (Py::tuple(vec![f__bus_index(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::None)?, f__bus_index(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::None)?, py::float(&py::getitem(&v_e.clone(), &Py::Int(2i64))?)?])).items()?;
        if t4719.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_i = t4719[0].clone();
        v_j = t4719[1].clone();
        v_x = t4719[2].clone();
        let _ = f__need({ let t4720 = v_i.clone(); Py::Bool({ let t4721 = v_j.clone(); if py::ne(&t4720, &t4721) { true } else { false } }) }, Py::str("a branch must join two different nodes"))?;
        let _ = f__need({ let t4722 = v_x.clone(); Py::Bool({ let t4723 = Py::Int(0i64); if py::gt(&t4722, &t4723)? { true } else { false } }) }, Py::str("branch reactances must be positive"))?;
        let _ = py::method(&v_edges.clone(), "append", &[Py::tuple(vec![v_i.clone(), v_j.clone(), v_x.clone()])])?;
    }
    v_nb = py::minmax(&[{ let mut out = vec![]; for t4724 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t4725 = (t4724).items()?;
if t4725.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t4725[0].clone();
v_j = t4725[1].clone();
v__ = t4725[2].clone();
out.push(py::minmax(&[v_i.clone(), v_j.clone()], true)?); } Py::list(out) }], true)?;
    let _ = f__need({ let t4726 = py::sub(&py::set(&{ let mut out = vec![]; for t4728 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t4729 = (t4728).items()?;
if t4729.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t4729[0].clone();
v_j = t4729[1].clone();
v__ = t4729[2].clone();
for t4730 in py::iter(&(Py::tuple(vec![v_i.clone(), v_j.clone()])))? { let mut v_k: Py; 
v_k = t4730;
out.push(v_k.clone()); }} Py::list(out) })?, &py::set(&Py::list(vec![Py::Int(0i64)]))?)?; Py::Bool({ let t4727 = py::set(&Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nb.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?; if py::eq(&t4726, &t4727) { true } else { false } }) }, Py::str("buses must be numbered 1 to n without gaps"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4731 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t4732 = (t4731).items()?;
if t4732.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t4732[0].clone();
v_j = t4732[1].clone();
v__ = t4732[2].clone();
out.push({ let t4733 = Py::Int(0i64); Py::Bool({ let t4734 = Py::tuple(vec![v_i.clone(), v_j.clone()]); if py::contains(&t4734, &t4733)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())), Py::str("at least one branch must connect to the reference node 0 (a source)"))?;
    return Ok(Py::tuple(vec![v_edges.clone(), v_nb.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_v: Py) -> R {
    let _ = f__need({ let t4919 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4919.truthy() { t4919 } else { { let t4918 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4918.truthy() { t4918 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
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
    v_M = { let mut out = vec![]; for t5062 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t5063 = (t5062).items()?;
if t5063.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t5063[0].clone();
v_row = t5063[1].clone();
out.push(py::add(&Py::list((py::map(&{ let v_A = v_A.clone(); let v_b = v_b.clone(); let v_i = v_i.clone(); let v_n = v_n.clone(); let v_row = v_row.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_v.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &v_row.clone())?).items()?), &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_scale = { let t5066 = { let t = { let mut out = vec![]; for t5064 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t5064;
for t5065 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t5065;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(1.0) } else { py::minmax(&[t], true)? } }; if t5066.truthy() { t5066 } else { Py::Float(1.0) } };
    for t5067 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t5067;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        let _ = f__need({ let t5068 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t5069 = py::mul(&Py::Float(1e-13), &v_scale.clone())?; if py::gt(&t5068, &t5069)? { true } else { false } }) }, Py::str("the system is singular: the network does not determine every unknown"))?;
        let t5070 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t5070.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t5070[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t5070[1].clone())?;
        for t5071 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t5071;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t5072 = v_f.clone(); Py::Bool({ let t5073 = Py::Int(0i64); if py::ne(&t5072, &t5073) { true } else { false } }) }).truthy() {
                for t5074 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t5074;
                    let t5075 = py::getitem(&v_M.clone(), &v_r.clone())?; let t5076 = v_k.clone();
                    py::setitem(&t5075, &t5076, py::sub(&py::getitem(&t5075, &t5076)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t5077 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t5077;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::sum(&{ let mut out = vec![]; for t5078 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5078;
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
    for t1669 in py::iter(&(py::zip(&[c__L0_N()?, c__L0_V()?])?))? {
        let t1670 = (t1669).items()?;
        if t1670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1670[0].clone();
        v_v = t1670[1].clone();
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
    let t1857 = Py::Float(0.0);
    v_ar = t1857.clone();
    v_ard = t1857.clone();
    v_art = t1857.clone();
    for t1858 in py::iter(&(py::zip(&[c__LP_N()?, c__LP_T()?, c__LP_D()?, c__LP_L()?])?))? {
        let t1859 = (t1858).items()?;
        if t1859.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1859[0].clone();
        v_tt = t1859[1].clone();
        v_dd = t1859[2].clone();
        v_l = t1859[3].clone();
        v_e = (if (v_l.clone()).truthy() { py::math1("exp", &py::neg(&py::pow(&v_d.clone(), &v_l.clone())?)?)? } else { Py::Float(1.0) });
        v_term = py::mul(&py::mul(&py::mul(&v_n.clone(), &py::pow(&v_d.clone(), &v_dd.clone())?)?, &py::pow(&v_t.clone(), &v_tt.clone())?)?, &v_e.clone())?;
        v_ar = py::iadd(&v_ar, &v_term.clone())?;
        v_ard = py::iadd(&v_ard, &py::truediv(&py::mul(&v_term.clone(), &py::sub(&v_dd.clone(), &py::mul(&v_l.clone(), &py::pow(&v_d.clone(), &v_l.clone())?)?)?)?, &v_d.clone())?)?;
        v_art = py::iadd(&v_art, &py::truediv(&py::mul(&v_term.clone(), &v_tt.clone())?, &v_t.clone())?)?;
    }
    for t1860 in py::iter(&(py::zip(&[c__LG_N()?, c__LG_T()?, c__LG_D()?, c__LG_ETA()?, c__LG_EPS()?, c__LG_BETA()?, c__LG_GAM()?])?))? {
        let t1861 = (t1860).items()?;
        if t1861.len() != 7 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1861[0].clone();
        v_tt = t1861[1].clone();
        v_dd = t1861[2].clone();
        v_eta = t1861[3].clone();
        v_eps = t1861[4].clone();
        v_beta = t1861[5].clone();
        v_gam = t1861[6].clone();
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
    let _ = f__need({ let t1818 = Py::Float(1000.0); Py::Bool({ let t1819 = v_p.clone(); if py::le(&t1818, &t1819)? { { let t1820 = Py::Float(150000000.0); if py::le(&t1819, &t1820)? { true } else { false } } } else { false } }) }, Py::str("the pressure must lie between 1 kPa and 1500 bar"))?;
    let _ = f__need({ let t1821 = Py::Int(200i64); Py::Bool({ let t1822 = v_T.clone(); if py::le(&t1821, &t1822)? { { let t1823 = Py::Int(700i64); if py::le(&t1822, &t1823)? { true } else { false } } } else { false } }) }, Py::str("the temperature must lie between 200 K and 700 K (gaseous hydrogen)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__h2_p(mut v_rho: Py, mut v_T: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t1856 = (Py::tuple(vec![py::truediv(&v_rho.clone(), &c__H2_RHOC()?)?, py::truediv(&c__H2_TC()?, &v_T.clone())?])).items()?;
    if t1856.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1856[0].clone();
    v_t = t1856[1].clone();
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
    let t1667 = (Py::tuple(vec![py::truediv(&v_rm.clone(), &c__H2_RHOC()?)?, py::truediv(&c__H2_TC()?, &v_T.clone())?])).items()?;
    if t1667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1667[0].clone();
    v_t = t1667[1].clone();
    let t1668 = (f__h2_ar(v_d.clone(), v_t.clone())?).items()?;
    if t1668.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ar = t1668[0].clone();
    v_ard = t1668[1].clone();
    v_art = t1668[2].clone();
    let t1671 = (f__h2_a0(v_d.clone(), v_t.clone())?).items()?;
    if t1671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a0 = t1671[0].clone();
    v_a0t = t1671[1].clone();
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
    let t1824 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t1824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t1824[0].clone();
    v_fb = t1824[1].clone();
    if ({ let t1825 = v_fa.clone(); Py::Bool({ let t1826 = Py::Int(0i64); if py::eq(&t1825, &t1826) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t1827 = v_fb.clone(); Py::Bool({ let t1828 = Py::Int(0i64); if py::eq(&t1827, &t1828) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t1829 = { let t1831 = v_fa.clone(); Py::Bool({ let t1832 = Py::Int(0i64); if py::gt(&t1831, &t1832)? { true } else { false } }) }; Py::Bool({ let t1830 = { let t1833 = v_fb.clone(); Py::Bool({ let t1834 = Py::Int(0i64); if py::gt(&t1833, &t1834)? { true } else { false } }) }; if py::ne(&t1829, &t1830) { true } else { false } }) }, Py::str("no root in the interval"))?;
    v_side = Py::Int(0i64);
    for t1835 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t1835;
        v_c = py::truediv(&py::sub(&py::mul(&v_a.clone(), &v_fb.clone())?, &py::mul(&v_b.clone(), &v_fa.clone())?)?, &py::sub(&v_fb.clone(), &v_fa.clone())?)?;
        if (py::not(&{ let t1836 = py::minmax(&[v_a.clone(), v_b.clone()], false)?; Py::Bool({ let t1837 = v_c.clone(); if py::lt(&t1836, &t1837)? { { let t1838 = py::minmax(&[v_a.clone(), v_b.clone()], true)?; if py::lt(&t1837, &t1838)? { true } else { false } } } else { false } }) })).truthy() {
            v_c = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        }
        v_fc = py::call(&v_f.clone(), &[v_c.clone()])?;
        if ({ let t1843 = { let t1839 = v_fc.clone(); Py::Bool({ let t1840 = Py::Int(0i64); if py::eq(&t1839, &t1840) { true } else { false } }) }; if t1843.truthy() { t1843 } else { { let t1841 = py::abs(&py::sub(&v_b.clone(), &v_a.clone())?)?; Py::Bool({ let t1842 = py::mul(&Py::Float(4e-16), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?; if py::le(&t1841, &t1842)? { true } else { false } }) } } }).truthy() {
            return Ok(v_c.clone());
        }
        if ({ let t1844 = { let t1846 = v_fc.clone(); Py::Bool({ let t1847 = Py::Int(0i64); if py::gt(&t1846, &t1847)? { true } else { false } }) }; Py::Bool({ let t1845 = { let t1848 = v_fb.clone(); Py::Bool({ let t1849 = Py::Int(0i64); if py::gt(&t1848, &t1849)? { true } else { false } }) }; if py::eq(&t1844, &t1845) { true } else { false } }) }).truthy() {
            let t1850 = (Py::tuple(vec![v_c.clone(), v_fc.clone()])).items()?;
            if t1850.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_b = t1850[0].clone();
            v_fb = t1850[1].clone();
            if ({ let t1851 = v_side.clone(); Py::Bool({ let t1852 = py::neg(&Py::Int(1i64))?; if py::eq(&t1851, &t1852) { true } else { false } }) }).truthy() {
                v_fa = py::mul(&v_fa, &Py::Float(0.5))?;
            }
            v_side = py::neg(&Py::Int(1i64))?;
        } else {
            let t1853 = (Py::tuple(vec![v_c.clone(), v_fc.clone()])).items()?;
            if t1853.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t1853[0].clone();
            v_fa = t1853[1].clone();
            if ({ let t1854 = v_side.clone(); Py::Bool({ let t1855 = Py::Int(1i64); if py::eq(&t1854, &t1855) { true } else { false } }) }).truthy() {
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
    return Ok((if ({ let t2107 = v_connection.clone(); Py::Bool({ let t2108 = Py::str("star"); if py::eq(&t2107, &t2108) { true } else { false } }) }).truthy() { py::truediv(&v_VL.clone(), &c_SQ3()?)? } else { v_VL.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__interp(mut v_x: Py, mut v_xs: Py, mut v_ys: Py) -> R {
    let mut v_i: Py = Py::None;
    if ({ let t3494 = { let t3490 = v_x.clone(); Py::Bool({ let t3491 = py::getitem(&v_xs.clone(), &Py::Int(0i64))?; if py::lt(&t3490, &t3491)? { true } else { false } }) }; if t3494.truthy() { t3494 } else { { let t3492 = v_x.clone(); Py::Bool({ let t3493 = py::getitem(&v_xs.clone(), &py::neg(&Py::Int(1i64))?)?; if py::gt(&t3492, &t3493)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    for t3495 in py::range_iter(&[(py::sub(&py::len(&v_xs.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t3495;
        if ({ let t3496 = py::getitem(&v_xs.clone(), &v_i.clone())?; Py::Bool({ let t3497 = v_x.clone(); if py::le(&t3496, &t3497)? { { let t3498 = py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?; if py::le(&t3497, &t3498)? { true } else { false } } } else { false } }) }).truthy() {
            if ({ let t3499 = v_x.clone(); Py::Bool({ let t3500 = py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?; if py::eq(&t3499, &t3500) { true } else { false } }) }).truthy() {
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
    let t2398 = (f__load_current(v_Vr.clone(), v_P.clone(), v_pf.clone(), v_pf_type.clone())?).items()?;
    if t2398.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t2398[0].clone();
    v_I = t2398[1].clone();
    v_Ic = t2398[2].clone();
    v_vr = py::truediv(&v_Vr.clone(), &c_SQ3()?)?;
    v_vs = py::add(&py::mul(&v_A.clone(), &v_vr.clone())?, &py::mul(&v_B.clone(), &v_Ic.clone())?)?;
    v_is_ = py::add(&py::mul(&v_C.clone(), &v_vr.clone())?, &py::mul(&v_D.clone(), &v_Ic.clone())?)?;
    v_Ss = py::mul(&py::mul(&Py::Int(3i64), &v_vs.clone())?, &py::method(&v_is_.clone(), "conjugate", &[])?)?;
    v_Vs = py::mul(&c_SQ3()?, &f__habs(v_vs.clone())?)?;
    v_Ps = (if ({ let t2399 = v_R.clone(); Py::Bool({ let t2400 = Py::Int(0i64); if py::eq(&t2399, &t2400) { true } else { false } }) }).truthy() { v_P.clone() } else { py::real(&v_Ss.clone())? });
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
    for t5554 in py::iter(&(v_vals.clone()))? {
        v_v = t5554;
        let _ = f__need({ let t5558 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5558.truthy() { t5558 } else { { let t5557 = py::math1("isfinite", &v_v.clone())?; if !t5557.truthy() { t5557 } else { { let t5555 = v_v.clone(); Py::Bool({ let t5556 = Py::Int(0i64); if py::ge(&t5555, &t5556)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t5617 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5618 = (t5617).items()?;
if t5618.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5618[0].clone();
v_v = t5618[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pf_angle(mut v_pf: Py, mut v_pf_type: Py) -> R {
    let mut v_phi: Py = Py::None;
    let _ = f__need({ let t5202 = Py::Bool(matches!(v_pf.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5202.truthy() { t5202 } else { { let t5199 = Py::Int(0i64); Py::Bool({ let t5200 = v_pf.clone(); if py::lt(&t5199, &t5200)? { { let t5201 = Py::Int(1i64); if py::le(&t5200, &t5201)? { true } else { false } } } else { false } }) } } }, Py::str("the power factor must lie in (0, 1]"))?;
    v_phi = py::math1("acos", &v_pf.clone())?;
    return Ok((if ({ let t5203 = v_pf_type.clone(); Py::Bool({ let t5204 = Py::str("leading"); if py::ne(&t5203, &t5204) { true } else { false } }) }).truthy() { v_phi.clone() } else { py::neg(&v_phi.clone())? }));
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
    let _ = f__need({ let t3259 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3259.truthy() { t3259 } else { { let t3256 = Py::Int(0i64); Py::Bool({ let t3257 = v_zenith.clone(); if py::le(&t3256, &t3257)? { { let t3258 = Py::Int(90i64); if py::lt(&t3257, &t3258)? { true } else { false } } } else { false } }) } } }, Py::str("the sun must be above the horizon (zenith below 90 degrees)"))?;
    let _ = f__need({ let t3260 = Py::Int(0i64); Py::Bool({ let t3261 = v_azimuth.clone(); if py::le(&t3260, &t3261)? { { let t3262 = Py::Int(360i64); if py::le(&t3261, &t3262)? { true } else { false } } } else { false } }) }, Py::str("the solar azimuth must lie between 0 and 360 degrees"))?;
    let _ = f__check_irr(v_dni.clone(), v_ghi.clone(), v_dhi.clone())?;
    let _ = f__need({ let t3270 = Py::Int(0i64); Py::Bool({ let t3271 = v_albedo.clone(); if py::le(&t3270, &t3271)? { { let t3272 = Py::Int(1i64); if py::le(&t3271, &t3272)? { true } else { false } } } else { false } }) }, Py::str("the albedo must lie between 0 and 1"))?;
    v_p = f__projection(v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone())?;
    v_cos_tt = py::minmax(&[v_p.clone(), Py::Float(0.0)], true)?;
    v_svf = py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &f__cosd(v_tilt.clone())?)?)?;
    if ({ let t3273 = v_model.clone(); Py::Bool({ let t3274 = Py::str("isotropic"); if py::eq(&t3273, &t3274) { true } else { false } }) }).truthy() {
        v_sky = py::mul(&py::mul(&v_dhi.clone(), &py::add(&Py::Int(1i64), &f__cosd(v_tilt.clone())?)?)?, &Py::Float(0.5))?;
    } else {
        if ({ let t3275 = v_model.clone(); Py::Bool({ let t3276 = Py::str("klucher"); if py::eq(&t3275, &t3276) { true } else { false } }) }).truthy() {
            v_F = (if ({ let t3277 = v_ghi.clone(); Py::Bool({ let t3278 = Py::Int(0i64); if py::gt(&t3277, &t3278)? { true } else { false } }) }).truthy() { py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_dhi.clone(), &v_ghi.clone())?, &Py::Int(2i64))?)? } else { Py::Float(0.0) });
            v_sky = py::mul(&py::mul(&py::mul(&v_dhi.clone(), &v_svf.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_F.clone(), &py::pow(&f__sind(py::mul(&Py::Float(0.5), &v_tilt.clone())?)?, &Py::Int(3i64))?)?)?)?, &py::add(&Py::Int(1i64), &py::mul(&py::mul(&v_F.clone(), &py::pow(&v_cos_tt.clone(), &Py::Int(2i64))?)?, &py::pow(&f__sind(v_zenith.clone())?, &Py::Int(3i64))?)?)?)?;
        } else {
            let _ = f__check_day(v_n.clone())?;
            v_dni_extra = f__g_on_spencer(v_n.clone())?;
            v_Rb = py::truediv(&v_cos_tt.clone(), &py::minmax(&[f__cosd(v_zenith.clone())?, Py::Float(0.01745)], true)?)?;
            v_AI = py::truediv(&v_dni.clone(), &v_dni_extra.clone())?;
            if ({ let t3283 = v_model.clone(); Py::Bool({ let t3284 = Py::str("haydavies"); if py::eq(&t3283, &t3284) { true } else { false } }) }).truthy() {
                v_sky = py::add(&py::minmax(&[py::mul(&py::mul(&v_dhi.clone(), &py::sub(&Py::Int(1i64), &v_AI.clone())?)?, &v_svf.clone())?, Py::Float(0.0)], true)?, &py::minmax(&[py::mul(&v_dhi.clone(), &py::mul(&v_AI.clone(), &v_Rb.clone())?)?, Py::Float(0.0)], true)?)?;
            } else {
                v_HB = py::minmax(&[py::mul(&v_dni.clone(), &f__cosd(v_zenith.clone())?)?, Py::Float(0.0)], true)?;
                v_h = py::mul(&py::math1("sqrt", &(if ({ let t3285 = v_ghi.clone(); Py::Bool({ let t3286 = Py::Int(0i64); if py::ne(&t3285, &t3286) { true } else { false } }) }).truthy() { py::truediv(&v_HB.clone(), &v_ghi.clone())? } else { Py::Float(0.0) }))?, &py::pow(&f__sind(py::truediv(&v_tilt.clone(), &Py::Int(2i64))?)?, &Py::Int(3i64))?)?;
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
    for t5609 in py::iter(&(v_vals.clone()))? {
        v_v = t5609;
        let _ = f__need({ let t5613 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5613.truthy() { t5613 } else { { let t5612 = py::math1("isfinite", &v_v.clone())?; if !t5612.truthy() { t5612 } else { { let t5610 = v_v.clone(); Py::Bool({ let t5611 = Py::Int(0i64); if py::gt(&t5610, &t5611)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
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
    if ({ let t5622 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5622.truthy() { t5622 } else { { let t5621 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5621.truthy() { t5621 } else { { let t5619 = v_x.clone(); Py::Bool({ let t5620 = Py::None; if py::is(&t5619, &t5620) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Complex(..)))).truthy() {
        return Ok(py::complex(&[f__r(py::real(&v_x.clone())?)?, f__r(py::imag(&v_x.clone())?)?])?);
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5623 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5623;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5626 = { let t5624 = v_x.clone(); Py::Bool({ let t5625 = Py::Int(0i64); if py::eq(&t5624, &t5625) { true } else { false } }) }; if t5626.truthy() { t5626 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t5627 = v_x.clone(); Py::Bool({ let t5628 = Py::Int(0i64); if py::eq(&t5627, &t5628) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t5635 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5635.truthy() { t5635 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5639 = py::not(&{ let t5636 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5636.truthy() { t5636 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5639.truthy() { t5639 } else { { let t5637 = py::len(&v_a.clone())?; Py::Bool({ let t5638 = py::len(&v_b.clone())?; if py::ne(&t5637, &t5638) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t5640 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5641 = (t5640).items()?;
if t5641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5641[0].clone();
v_y = t5641[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t5642 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t5642.truthy() { t5642 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t5643 = v_a.clone(); Py::Bool({ let t5644 = v_b.clone(); if py::eq(&t5643, &t5644) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t5646: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5645 = (Py::tuple(vec![py::complex(&[v_a.clone()])?, py::complex(&[v_b.clone()])?])).items()?;
            if t5645.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5645[0].clone();
            v_b = t5645[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5646 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5647 = v_d.clone(); Py::Bool({ let t5648 = Py::Int(0i64); if py::eq(&t5647, &t5648) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t5649 = v_s.clone(); Py::Bool({ let t5650 = Py::Float(1e-300); if py::gt(&t5649, &t5650)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
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
    let t4048 = (Py::tuple(vec![py::neg(&Py::Float(1.0))?, py::add(&py::add(&v_IL.clone(), &v_I0.clone())?, &Py::Float(1.0))?])).items()?;
    if t4048.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4048[0].clone();
    v_hi = t4048[1].clone();
    while ({ let t4051 = f__sd_g(v_lo.clone(), v_V.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?; Py::Bool({ let t4052 = Py::Int(0i64); if py::lt(&t4051, &t4052)? { true } else { false } }) }).truthy() {
        v_lo = py::mul(&v_lo, &Py::Int(2i64))?;
        let _ = f__need({ let t4049 = v_lo.clone(); Py::Bool({ let t4050 = py::neg(&Py::Float(1000000000000.0))?; if py::gt(&t4049, &t4050)? { true } else { false } }) }, Py::str("no solution for the current at this voltage"))?;
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
    if ({ let t4154 = v_x.clone(); Py::Bool({ let t4155 = Py::Int(700i64); if py::gt(&t4154, &t4155)? { true } else { false } }) }).truthy() {
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
    while ({ let t4152 = f__sd_g(v_I.clone(), v_hi.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?; Py::Bool({ let t4153 = Py::Int(0i64); if py::gt(&t4152, &t4153)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Int(2i64))?;
        let _ = f__need({ let t4150 = v_hi.clone(); Py::Bool({ let t4151 = Py::Float(1000000000.0); if py::lt(&t4150, &t4151)? { true } else { false } }) }, Py::str("no solution for the voltage at this current"))?;
    }
    v_lo = py::neg(&Py::Float(1.0))?;
    while ({ let t4158 = f__sd_g(v_I.clone(), v_lo.clone(), v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_a.clone())?; Py::Bool({ let t4159 = Py::Int(0i64); if py::lt(&t4158, &t4159)? { true } else { false } }) }).truthy() {
        v_lo = py::mul(&v_lo, &Py::Int(2i64))?;
        let _ = f__need({ let t4156 = v_lo.clone(); Py::Bool({ let t4157 = py::neg(&Py::Float(1000000000000.0))?; if py::gt(&t4156, &t4157)? { true } else { false } }) }, Py::str("no solution for the voltage at this current"))?;
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
    let _ = f__need({ let t4242 = Py::Float(0.5); Py::Bool({ let t4243 = v_Vf.clone(); if py::le(&t4242, &t4243)? { { let t4244 = Py::Float(1.5); if py::le(&t4243, &t4244)? { true } else { false } } } else { false } }) }, Py::str("the prefault voltage must lie between 0.5 and 1.5 pu"))?;
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
    return Ok((if ({ let t4267 = py::abs(&v_x.clone())?; Py::Bool({ let t4268 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_scale.clone(), Py::Float(1e-300)], true)?)?; if py::le(&t4267, &t4268)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
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
    let t4586 = (Py::tuple(vec![py::math1("radians", &v_latitude.clone())?, py::math1("radians", &v_ha_deg.clone())?])).items()?;
    if t4586.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lat = t4586[0].clone();
    v_h = t4586[1].clone();
    v_cz = py::add(&py::mul(&py::mul(&py::math1("cos", &v_decl_rad.clone())?, &py::math1("cos", &v_lat.clone())?)?, &py::math1("cos", &v_h.clone())?)?, &py::mul(&py::math1("sin", &v_decl_rad.clone())?, &py::math1("sin", &v_lat.clone())?)?)?;
    v_z = py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), v_cz.clone()], false)?], true)?)?;
    v_numer = py::sub(&py::mul(&py::math1("cos", &v_z.clone())?, &py::math1("sin", &v_lat.clone())?)?, &py::math1("sin", &v_decl_rad.clone())?)?;
    v_denom = py::mul(&py::math1("sin", &v_z.clone())?, &py::math1("cos", &v_lat.clone())?)?;
    v_ca = (if ({ let t4587 = py::abs(&v_denom.clone())?; Py::Bool({ let t4588 = Py::Float(1e-08); if py::le(&t4587, &t4588)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&v_numer.clone(), &v_denom.clone())? });
    if ({ let t4589 = py::abs(&py::sub(&v_ca.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t4590 = Py::Float(1e-08); if py::le(&t4589, &t4590)? { true } else { false } }) }).truthy() {
        v_ca = Py::Float(1.0);
    } else {
        if ({ let t4591 = py::abs(&py::add(&v_ca.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t4592 = Py::Float(1e-08); if py::le(&t4591, &t4592)? { true } else { false } }) }).truthy() {
            v_ca = py::neg(&Py::Float(1.0))?;
        }
    }
    v_sgn = py::sub(&{ let t4593 = v_ha_deg.clone(); Py::Bool({ let t4594 = Py::Int(0i64); if py::gt(&t4593, &t4594)? { true } else { false } }) }, &{ let t4595 = v_ha_deg.clone(); Py::Bool({ let t4596 = Py::Int(0i64); if py::lt(&t4595, &t4596)? { true } else { false } }) })?;
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
    for t5059 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t5059;
        v_f = py::sub(&py::getitem(&py::call(&v_v1_of.clone(), &[v_v2.clone()])?, &Py::Int(0i64))?, &Py::Int(1i64))?;
        if ({ let t5060 = py::abs(&v_f.clone())?; Py::Bool({ let t5061 = Py::Float(1e-15); if py::lt(&t5060, &t5061)? { true } else { false } }) }).truthy() {
            break;
        }
        v_h = Py::Float(1e-07);
        v_fr = py::truediv(&py::sub(&py::sub(&py::getitem(&py::call(&v_v1_of.clone(), &[py::add(&v_v2.clone(), &v_h.clone())?])?, &Py::Int(0i64))?, &Py::Int(1i64))?, &v_f.clone())?, &v_h.clone())?;
        v_fi = py::truediv(&py::sub(&py::sub(&py::getitem(&py::call(&v_v1_of.clone(), &[py::add(&v_v2.clone(), &py::mul(&Py::Complex(0.0, 1.0), &v_h.clone())?)?])?, &Py::Int(0i64))?, &Py::Int(1i64))?, &v_f.clone())?, &v_h.clone())?;
        v_dx = f__gauss_solve(Py::list(vec![Py::list(vec![py::real(&v_fr.clone())?, py::real(&v_fi.clone())?]), Py::list(vec![py::imag(&v_fr.clone())?, py::imag(&v_fi.clone())?])]), Py::list(vec![py::neg(&py::real(&v_f.clone())?)?, py::neg(&py::imag(&v_f.clone())?)?]))?;
        v_v2 = py::iadd(&v_v2, &py::complex(&[py::getitem(&v_dx.clone(), &Py::Int(0i64))?, py::getitem(&v_dx.clone(), &Py::Int(1i64))?])?)?;
        let _ = f__need({ let t5079 = Py::Float(0.3); Py::Bool({ let t5080 = py::abs(&v_v2.clone())?; if py::lt(&t5079, &t5080)? { { let t5081 = Py::Int(2i64); if py::lt(&t5080, &t5081)? { true } else { false } } } else { false } }) }, Py::str("the transformer cannot supply this load (no load-flow solution)"))?;
    }
    let _ = f__need({ let t5082 = py::abs(&py::sub(&py::getitem(&py::call(&v_v1_of.clone(), &[v_v2.clone()])?, &Py::Int(0i64))?, &Py::Int(1i64))?)?; Py::Bool({ let t5083 = Py::Float(1e-12); if py::lt(&t5082, &t5083)? { true } else { false } }) }, Py::str("the transformer load flow did not converge"))?;
    let t5084 = (py::call(&v_v1_of.clone(), &[v_v2.clone()])?).items()?;
    if t5084.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t5084[0].clone();
    v_i1 = t5084[1].clone();
    v_i2 = t5084[2].clone();
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
    let t419 = (py::getitem(&c__SAPM_MOUNTS()?, &v_mount.clone())?).items()?;
    if t419.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t419[0].clone();
    v_b = t419[1].clone();
    v_dT = t419[2].clone();
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
    let _ = f__need({ let t519 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t519.truthy() { t519 } else { { let t516 = Py::Int(0i64); Py::Bool({ let t517 = v_zenith.clone(); if py::le(&t516, &t517)? { { let t518 = Py::Int(90i64); if py::lt(&t517, &t518)? { true } else { false } } } else { false } }) } } }, Py::str("the sun must be above the horizon (apparent zenith below 90 degrees)"))?;
    let _ = f__need({ let t520 = Py::Int(1i64); Py::Bool({ let t521 = v_linke_turbidity.clone(); if py::le(&t520, &t521)? { { let t522 = Py::Int(10i64); if py::le(&t521, &t522)? { true } else { false } } } else { false } }) }, Py::str("the Linke turbidity must lie between 1 and 10"))?;
    let _ = f__need({ let t523 = py::neg(&Py::Int(500i64))?; Py::Bool({ let t524 = v_altitude.clone(); if py::le(&t523, &t524)? { { let t525 = Py::Int(9000i64); if py::le(&t524, &t525)? { true } else { false } } } else { false } }) }, Py::str("the altitude must lie between -500 m and 9000 m"))?;
    let _ = f__check_day(v_n.clone())?;
    v_am = py::truediv(&py::mul(&f__airmass_rel(v_zenith.clone(), Py::str("kastenyoung1989"))?, &f__alt2pres(v_altitude.clone())?)?, &Py::Float(101325.0))?;
    v_extra = f__g_on_spencer(v_n.clone())?;
    v_cz = py::minmax(&[f__cosd(v_zenith.clone())?, Py::Float(0.0)], true)?;
    v_tl = v_linke_turbidity.clone();
    let t544 = (Py::tuple(vec![py::math1("exp", &py::truediv(&py::neg(&v_altitude.clone())?, &Py::Float(8000.0))?)?, py::math1("exp", &py::truediv(&py::neg(&v_altitude.clone())?, &Py::Float(1250.0))?)?])).items()?;
    if t544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fh1 = t544[0].clone();
    v_fh2 = t544[1].clone();
    let t545 = (Py::tuple(vec![py::add(&py::mul(&Py::Float(5.09e-05), &v_altitude.clone())?, &Py::Float(0.868))?, py::add(&py::mul(&Py::Float(3.92e-05), &v_altitude.clone())?, &Py::Float(0.0387))?])).items()?;
    if t545.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cg1 = t545[0].clone();
    v_cg2 = t545[1].clone();
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
    let _ = f__need({ let t594 = Py::Bool(matches!(v_SOC0.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t594.truthy() { t594 } else { { let t591 = Py::Int(0i64); Py::Bool({ let t592 = v_SOC0.clone(); if py::le(&t591, &t592)? { { let t593 = Py::Int(1i64); if py::le(&t592, &t593)? { true } else { false } } } else { false } }) } } }, Py::str("the initial state of charge must lie between 0 and 1 (or 0 and 100%)"))?;
    let _ = f__pos(Py::str("the capacity and the time step"), Py::tuple(vec![v_Q.clone(), v_dt.clone()]))?;
    let _ = f__need({ let t600 = py::len(&v_currents.clone())?; Py::Bool({ let t601 = Py::Int(1i64); if py::ge(&t600, &t601)? { true } else { false } }) }, Py::str("give at least one current"))?;
    let t602 = (Py::tuple(vec![v_SOC0.clone(), v_SOC0.clone(), v_SOC0.clone(), Py::Float(0.0)])).items()?;
    if t602.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_soc = t602[0].clone();
    v_lo = t602[1].clone();
    v_hi = t602[2].clone();
    v_net = t602[3].clone();
    for t603 in py::iter(&(v_currents.clone()))? {
        v_i = t603;
        let _ = f__fin(Py::str("each current"), v_i.clone())?;
        v_net = py::iadd(&v_net, &py::mul(&v_i.clone(), &v_dt.clone())?)?;
        v_soc = py::sub(&v_SOC0.clone(), &py::truediv(&v_net.clone(), &v_Q.clone())?)?;
        let t606 = (Py::tuple(vec![py::minmax(&[v_lo.clone(), v_soc.clone()], false)?, py::minmax(&[v_hi.clone(), v_soc.clone()], true)?])).items()?;
        if t606.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t606[0].clone();
        v_hi = t606[1].clone();
    }
    let _ = f__need({ let t611 = { let t607 = v_lo.clone(); Py::Bool({ let t608 = py::neg(&Py::Float(1e-12))?; if py::ge(&t607, &t608)? { true } else { false } }) }; if !t611.truthy() { t611 } else { { let t609 = v_hi.clone(); Py::Bool({ let t610 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t609, &t610)? { true } else { false } }) } } }, Py::str("the state of charge leaves the range 0 to 100% (more charge than the capacity)"))?;
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
    let _ = f__need({ let t671 = { let t667 = v_Pm.clone(); Py::Bool({ let t668 = v_Pmax_pre.clone(); if py::lt(&t667, &t668)? { true } else { false } }) }; if !t671.truthy() { t671 } else { { let t669 = v_Pm.clone(); Py::Bool({ let t670 = v_Pmax_post.clone(); if py::lt(&t669, &t670)? { true } else { false } }) } } }, Py::str("the mechanical power must be below the pre- and post-fault transfer limits"))?;
    let _ = f__need({ let t672 = v_Pmax_fault.clone(); Py::Bool({ let t673 = v_Pm.clone(); if py::lt(&t672, &t673)? { true } else { false } }) }, Py::str("the during-fault transfer limit must be below the mechanical power (otherwise the rotor does not accelerate)"))?;
    v_d0 = py::math1("asin", &py::truediv(&v_Pm.clone(), &v_Pmax_pre.clone())?)?;
    v_dm = py::sub(&Py::Float(std::f64::consts::PI), &py::math1("asin", &py::truediv(&v_Pm.clone(), &v_Pmax_post.clone())?)?)?;
    let _ = f__need({ let t674 = v_dm.clone(); Py::Bool({ let t675 = v_d0.clone(); if py::gt(&t674, &t675)? { true } else { false } }) }, Py::str("the post-fault network cannot hold the rotor"))?;
    v_num = py::sub(&py::add(&py::mul(&v_Pm.clone(), &py::sub(&v_dm.clone(), &v_d0.clone())?)?, &py::mul(&v_Pmax_post.clone(), &py::math1("cos", &v_dm.clone())?)?)?, &py::mul(&v_Pmax_fault.clone(), &py::math1("cos", &v_d0.clone())?)?)?;
    v_cc = py::truediv(&v_num.clone(), &py::sub(&v_Pmax_post.clone(), &v_Pmax_fault.clone())?)?;
    let _ = f__need({ let t676 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t677 = v_cc.clone(); if py::lt(&t676, &t677)? { { let t678 = Py::Int(1i64); if py::lt(&t677, &t678)? { true } else { false } } } else { false } }) }, Py::str("no clearing angle keeps the system stable (the decelerating area is too small)"))?;
    v_dc = py::math1("acos", &v_cc.clone())?;
    v_out = Py::dict(vec![(Py::str("delta0_deg"), py::math1("degrees", &v_d0.clone())?), (Py::str("delta_max_deg"), py::math1("degrees", &v_dm.clone())?), (Py::str("delta_cr_deg"), py::math1("degrees", &v_dc.clone())?)]);
    if ({ let t683 = { let t679 = v_H.clone(); Py::Bool({ let t680 = Py::None; if !py::is(&t679, &t680) { true } else { false } }) }; if t683.truthy() { t683 } else { { let t681 = v_f.clone(); Py::Bool({ let t682 = Py::None; if !py::is(&t681, &t682) { true } else { false } }) } } }).truthy() {
        let _ = f__need({ let t688 = { let t684 = v_H.clone(); Py::Bool({ let t685 = Py::None; if !py::is(&t684, &t685) { true } else { false } }) }; if !t688.truthy() { t688 } else { { let t686 = v_f.clone(); Py::Bool({ let t687 = Py::None; if !py::is(&t686, &t687) { true } else { false } }) } } }, Py::str("the clearing time needs both H and f"))?;
        let _ = f__pos(Py::str("H and f"), Py::tuple(vec![v_H.clone(), v_f.clone()]))?;
        let _ = f__need({ let t689 = v_Pmax_fault.clone(); Py::Bool({ let t690 = Py::Int(0i64); if py::eq(&t689, &t690) { true } else { false } }) }, Py::str("a closed-form clearing time exists only for a fault with zero transfer (Pmax_fault = 0)"))?;
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
    let _ = f__need({ let t737 = py::len(&v_lines.clone())?; Py::Bool({ let t738 = Py::Int(1i64); if py::ge(&t737, &t738)? { true } else { false } }) }, Py::str("give at least one line"))?;
    v_edges = Py::list(vec![]);
    for t739 in py::iter(&(v_lines.clone()))? {
        v_e = t739;
        let _ = f__need({ let t740 = py::len(&v_e.clone())?; Py::Bool({ let t741 = Py::Int(3i64); if py::eq(&t740, &t741) { true } else { false } }) }, Py::str("each line is 'i-j x' with its reactance in per unit"))?;
        let t744 = (Py::tuple(vec![f__bus_index(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::None)?, f__bus_index(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::None)?, py::float(&py::getitem(&v_e.clone(), &Py::Int(2i64))?)?])).items()?;
        if t744.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_i = t744[0].clone();
        v_j = t744[1].clone();
        v_x = t744[2].clone();
        let _ = f__need({ let t745 = v_i.clone(); Py::Bool({ let t746 = v_j.clone(); if py::ne(&t745, &t746) { true } else { false } }) }, Py::str("a line must join two different buses"))?;
        let _ = f__need({ let t747 = v_x.clone(); Py::Bool({ let t748 = Py::Int(0i64); if py::gt(&t747, &t748)? { true } else { false } }) }, Py::str("line reactances must be positive"))?;
        let _ = py::method(&v_edges.clone(), "append", &[Py::tuple(vec![v_i.clone(), v_j.clone(), v_x.clone()])])?;
    }
    v_nb = py::minmax(&[{ let mut out = vec![]; for t749 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t750 = (t749).items()?;
if t750.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t750[0].clone();
v_j = t750[1].clone();
v__ = t750[2].clone();
out.push(py::minmax(&[v_i.clone(), v_j.clone()], true)?); } Py::list(out) }], true)?;
    let _ = f__need({ let t751 = py::set(&Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nb.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?; Py::Bool({ let t752 = py::set(&{ let mut out = vec![]; for t753 in py::iter(&(v_edges.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t754 = (t753).items()?;
if t754.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t754[0].clone();
v_j = t754[1].clone();
v__ = t754[2].clone();
for t755 in py::iter(&(Py::tuple(vec![v_i.clone(), v_j.clone()])))? { let mut v_b: Py; 
v_b = t755;
out.push(v_b.clone()); }} Py::list(out) })?; if py::eq(&t751, &t752) { true } else { false } }) }, Py::str("buses must be numbered 1 to n without gaps, every bus on a line"))?;
    let _ = f__need({ let t756 = py::len(&v_P.clone())?; Py::Bool({ let t757 = py::sub(&v_nb.clone(), &Py::Int(1i64))?; if py::eq(&t756, &t757) { true } else { false } }) }, Py::str(&["give the net injections of buses 2 to ".to_string(), py::format(&v_nb.clone(), "")?, " (".to_string(), py::format(&py::sub(&v_nb.clone(), &Py::Int(1i64))?, "")?, " values); bus 1 is the slack".to_string()].concat()))?;
    v_B = { let mut out = vec![]; for t758 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v__: Py; 
v__ = t758;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?); } Py::list(out) };
    for t759 in py::iter(&(v_edges.clone()))? {
        let t768 = (t759).items()?;
        if t768.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_i = t768[0].clone();
        v_j = t768[1].clone();
        v_x = t768[2].clone();
        v_b = py::truediv(&Py::Float(1.0), &v_x.clone())?;
        let t760 = py::getitem(&v_B.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t761 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t760, &t761, py::iadd(&py::getitem(&t760, &t761)?, &v_b.clone())?)?;
        let t762 = py::getitem(&v_B.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t763 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
        py::setitem(&t762, &t763, py::iadd(&py::getitem(&t762, &t763)?, &v_b.clone())?)?;
        let t764 = py::getitem(&v_B.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t765 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
        py::setitem(&t764, &t765, py::sub(&py::getitem(&t764, &t765)?, &v_b.clone())?)?;
        let t766 = py::getitem(&v_B.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t767 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t766, &t767, py::sub(&py::getitem(&t766, &t767)?, &v_b.clone())?)?;
    }
    v_th = f__gauss_solve({ let mut out = vec![]; for t786 in py::iter(&(py::slice(&v_B.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_row: Py; 
v_row = t786;
out.push(py::slice(&v_row.clone(), Some((Py::Int(1i64)).i()?), None, None)?); } Py::list(out) }, Py::list((v_P.clone()).items()?))?;
    v_theta = py::add(&Py::list(vec![Py::Float(0.0)]), &v_th.clone())?;
    v_flows = { let mut out = vec![]; for t787 in py::iter(&(v_edges.clone()))? { let mut v_i: Py; let mut v_j: Py; let mut v_x: Py; 
let t788 = (t787).items()?;
if t788.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t788[0].clone();
v_j = t788[1].clone();
v_x = t788[2].clone();
out.push(py::truediv(&py::sub(&py::getitem(&v_theta.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_theta.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?)?, &v_x.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("theta_deg"), { let mut out = vec![]; for t801 in py::iter(&(v_theta.clone()))? { let mut v_t: Py; 
v_t = t801;
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
    let t849 = (f__desoto(v_G.clone(), v_T_cell.clone(), v_alpha_sc.clone(), v_a_ref.clone(), v_IL_ref.clone(), v_I0_ref.clone(), v_Rsh_ref.clone(), v_Rs.clone(), v_EgRef.clone(), v_dEgdT.clone())?).items()?;
    if t849.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_IL = t849[0].clone();
    v_I0 = t849[1].clone();
    v_Rs_ = t849[2].clone();
    v_Rsh = t849[3].clone();
    v_a = t849[4].clone();
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
    let _ = f__need({ let t902 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t902.truthy() { t902 } else { { let t899 = Py::Int(0i64); Py::Bool({ let t900 = v_zenith.clone(); if py::le(&t899, &t900)? { { let t901 = Py::Int(85i64); if py::le(&t900, &t901)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must lie between 0 and 85 degrees"))?;
    let _ = f__pos(Py::str("the global horizontal irradiance and the pressure"), Py::tuple(vec![v_ghi.clone(), v_pressure.clone()]))?;
    v_b = f__day_angle(v_n.clone(), Py::Int(1i64))?;
    v_I0 = py::mul(&Py::Float(1370.0), &py::add(&py::add(&py::add(&py::add(&Py::Float(1.00011), &py::mul(&Py::Float(0.034221), &py::math1("cos", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.00128), &py::math1("sin", &v_b.clone())?)?)?, &py::mul(&Py::Float(0.000719), &py::math1("cos", &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?, &py::mul(&Py::Float(7.7e-05), &py::math1("sin", &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?)?;
    v_kt = py::minmax(&[py::minmax(&[py::truediv(&v_ghi.clone(), &py::mul(&v_I0.clone(), &py::minmax(&[f__cosd(v_zenith.clone())?, Py::Float(0.065)], true)?)?)?, Py::Float(0.0)], true)?, Py::Float(1.0)], false)?;
    v_am = py::minmax(&[py::truediv(&py::mul(&f__airmass_rel(v_zenith.clone(), Py::str("kasten1966"))?, &v_pressure.clone())?, &Py::Float(101325.0))?, Py::Float(12.0)], false)?;
    if ({ let t922 = v_kt.clone(); Py::Bool({ let t923 = Py::Float(0.6); if py::le(&t922, &t923)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t1044 = { let t1040 = py::len(&v_R.clone())?; Py::Bool({ let t1041 = py::len(&v_ratings.clone())?; if py::eq(&t1040, &t1041) { true } else { false } }) }; if !t1044.truthy() { t1044 } else { { let t1042 = py::len(&v_R.clone())?; Py::Bool({ let t1043 = Py::Int(1i64); if py::ge(&t1042, &t1043)? { true } else { false } }) } } }, Py::str("give one droop and one rating per generator"))?;
    for t1045 in py::iter(&(py::zip(&[v_R.clone(), v_ratings.clone()])?))? {
        let t1051 = (t1045).items()?;
        if t1051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t1051[0].clone();
        v_s = t1051[1].clone();
        let _ = f__need({ let t1050 = { let t1046 = v_r.clone(); Py::Bool({ let t1047 = Py::Int(0i64); if py::gt(&t1046, &t1047)? { true } else { false } }) }; if !t1050.truthy() { t1050 } else { { let t1048 = v_s.clone(); Py::Bool({ let t1049 = Py::Int(0i64); if py::gt(&t1048, &t1049)? { true } else { false } }) } } }, Py::str("droops and ratings must be positive"))?;
    }
    let _ = f__pos(Py::str("the frequency"), Py::tuple(vec![v_f0.clone()]))?;
    let _ = f__nonneg(Py::str("the load damping D"), Py::tuple(vec![v_D.clone()]))?;
    v_beta = py::add(&py::sum(&{ let mut out = vec![]; for t1062 in py::iter(&(py::zip(&[v_R.clone(), v_ratings.clone()])?))? { let mut v_r: Py; let mut v_s: Py; 
let t1063 = (t1062).items()?;
if t1063.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1063[0].clone();
v_s = t1063[1].clone();
out.push(py::truediv(&v_s.clone(), &py::mul(&v_r.clone(), &v_f0.clone())?)?); } Py::list(out) }, None)?, &v_D.clone())?;
    v_df = py::truediv(&py::neg(&v_dP.clone())?, &v_beta.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("df"), v_df.clone()), (Py::str("f"), py::add(&v_f0.clone(), &v_df.clone())?), (Py::str("dP_gen"), { let mut out = vec![]; for t1076 in py::iter(&(py::zip(&[v_R.clone(), v_ratings.clone()])?))? { let mut v_r: Py; let mut v_s: Py; 
let t1077 = (t1076).items()?;
if t1077.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1077[0].clone();
v_s = t1077[1].clone();
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
    let _ = f__need({ let t1111 = py::len(&v_costs.clone())?; Py::Bool({ let t1112 = Py::Int(1i64); if py::ge(&t1111, &t1112)? { true } else { false } }) }, Py::str("give at least one generator"))?;
    v_units = Py::list(vec![]);
    for t1113 in py::iter(&(v_costs.clone()))? {
        v_row = t1113;
        let _ = f__need({ let t1114 = py::len(&v_row.clone())?; Py::Bool({ let t1115 = Py::Int(5i64); if py::eq(&t1114, &t1115) { true } else { false } }) }, Py::str("each generator row is: a, b, c, Pmin, Pmax (cost a + b P + c P^2 in $/h, P in MW)"))?;
        let t1116 = (v_row.clone()).items()?;
        if t1116.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1116[0].clone();
        v_b = t1116[1].clone();
        v_c = t1116[2].clone();
        v_lo = t1116[3].clone();
        v_hi = t1116[4].clone();
        let _ = f__need({ let t1117 = v_c.clone(); Py::Bool({ let t1118 = Py::Int(0i64); if py::gt(&t1117, &t1118)? { true } else { false } }) }, Py::str("the quadratic cost coefficient c must be positive"))?;
        let _ = f__need({ let t1119 = Py::Int(0i64); Py::Bool({ let t1120 = v_lo.clone(); if py::le(&t1119, &t1120)? { { let t1121 = v_hi.clone(); if py::lt(&t1120, &t1121)? { true } else { false } } } else { false } }) }, Py::str("each generator needs 0 <= Pmin < Pmax"))?;
        let _ = py::method(&v_units.clone(), "append", &[Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_lo.clone(), v_hi.clone()])])?;
    }
    let _ = f__need({ let t1122 = py::sum(&{ let mut out = vec![]; for t1125 in py::iter(&(v_units.clone()))? { let mut v_u: Py; 
v_u = t1125;
out.push(py::getitem(&v_u.clone(), &Py::Int(3i64))?); } Py::list(out) }, None)?; Py::Bool({ let t1123 = v_demand.clone(); if py::le(&t1122, &t1123)? { { let t1124 = py::sum(&{ let mut out = vec![]; for t1126 in py::iter(&(v_units.clone()))? { let mut v_u: Py; 
v_u = t1126;
out.push(py::getitem(&v_u.clone(), &Py::Int(4i64))?); } Py::list(out) }, None)?; if py::le(&t1123, &t1124)? { true } else { false } } } else { false } }) }, Py::str("the demand must lie between the total minimum and total maximum output"))?;
    v_P = { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_costs = v_costs.clone(); let v_demand = v_demand.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_row = v_row.clone(); let v_units = v_units.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_lam: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t1127 in py::iter(&(v_units.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_lo: Py; 
let t1128 = (t1127).items()?;
if t1128.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1128[0].clone();
v_b = t1128[1].clone();
v_c = t1128[2].clone();
v_lo = t1128[3].clone();
v_hi = t1128[4].clone();
out.push(py::minmax(&[py::minmax(&[py::truediv(&py::sub(&v_lam.clone(), &v_b.clone())?, &py::mul(&Py::Int(2i64), &v_c.clone())?)?, v_lo.clone()], true)?, v_hi.clone()], false)?); } Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_lo_l = py::sub(&py::minmax(&[{ let mut out = vec![]; for t1129 in py::iter(&(v_units.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_lo: Py; 
let t1130 = (t1129).items()?;
if t1130.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1130[0].clone();
v_b = t1130[1].clone();
v_c = t1130[2].clone();
v_lo = t1130[3].clone();
v_hi = t1130[4].clone();
out.push(py::add(&v_b.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_lo.clone())?)?); } Py::list(out) }], false)?, &Py::Int(1i64))?;
    v_hi_l = py::add(&py::minmax(&[{ let mut out = vec![]; for t1131 in py::iter(&(v_units.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_lo: Py; 
let t1132 = (t1131).items()?;
if t1132.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1132[0].clone();
v_b = t1132[1].clone();
v_c = t1132[2].clone();
v_lo = t1132[3].clone();
v_hi = t1132[4].clone();
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
    v_free = { let mut out = vec![]; for t1159 in py::iter(&(py::enumerate(&v_units.clone(), 0)?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_k: Py; let mut v_lo: Py; 
let t1160 = (t1159).items()?;
if t1160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1160[0].clone();
let t1161 = (t1160[1].clone()).items()?;
if t1161.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1161[0].clone();
v_b = t1161[1].clone();
v_c = t1161[2].clone();
v_lo = t1161[3].clone();
v_hi = t1161[4].clone();
if ({ let t1162 = v_lo.clone(); Py::Bool({ let t1163 = py::getitem(&v_Ps.clone(), &v_k.clone())?; if py::lt(&t1162, &t1163)? { { let t1164 = v_hi.clone(); if py::lt(&t1163, &t1164)? { true } else { false } } } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    let _ = f__need(Py::Bool((v_free.clone()).truthy()), Py::str("every unit is at a limit: the incremental cost is not unique"))?;
    v_k = py::getitem(&v_free.clone(), &Py::Int(0i64))?;
    v_lam = py::add(&py::getitem(&py::getitem(&v_units.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_units.clone(), &v_k.clone())?, &Py::Int(2i64))?)?, &py::getitem(&v_Ps.clone(), &v_k.clone())?)?)?;
    v_cost = py::sum(&{ let mut out = vec![]; for t1165 in py::iter(&(py::zip(&[v_units.clone(), v_Ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; let mut v_hi: Py; let mut v_lo: Py; let mut v_p: Py; 
let t1166 = (t1165).items()?;
if t1166.len() != 2 { return Err(PyErr::Value("unpack".into())); }
let t1167 = (t1166[0].clone()).items()?;
if t1167.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_a = t1167[0].clone();
v_b = t1167[1].clone();
v_c = t1167[2].clone();
v_lo = t1167[3].clone();
v_hi = t1167[4].clone();
v_p = t1166[1].clone();
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
    let _ = f__need({ let t1220 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1220.truthy() { t1220 } else { { let t1218 = v_N.clone(); Py::Bool({ let t1219 = Py::Int(1i64); if py::ge(&t1218, &t1219)? { true } else { false } }) } } }, Py::str("the number of cells must be a whole number >= 1"))?;
    let _ = f__need({ let t1221 = Py::Float(1.23); Py::Bool({ let t1222 = v_V_cell.clone(); if py::le(&t1221, &t1222)? { { let t1223 = Py::Float(3.0); if py::le(&t1222, &t1223)? { true } else { false } } } else { false } }) }, Py::str("an electrolysis cell operates between the reversible voltage 1.23 V and about 3 V"))?;
    let _ = f__need({ let t1224 = Py::Int(0i64); Py::Bool({ let t1225 = v_faradaic.clone(); if py::lt(&t1224, &t1225)? { { let t1226 = Py::Int(1i64); if py::le(&t1225, &t1226)? { true } else { false } } } else { false } }) }, Py::str("the Faradaic efficiency must lie in (0, 1]"))?;
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
    v_e = (if ({ let t1277 = v_method.clone(); Py::Bool({ let t1278 = Py::str("spencer"); if py::eq(&t1277, &t1278) { true } else { false } }) }).truthy() { f__eot_spencer(v_n.clone())? } else { f__eot_pvcdrom(v_n.clone())? });
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
    let _ = f__need({ let t1331 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1331.truthy() { t1331 } else { { let t1328 = Py::Int(0i64); Py::Bool({ let t1329 = v_zenith.clone(); if py::le(&t1328, &t1329)? { { let t1330 = Py::Int(85i64); if py::le(&t1329, &t1330)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must lie between 0 and 85 degrees"))?;
    let _ = f__pos(Py::str("the global horizontal irradiance"), Py::tuple(vec![v_ghi.clone()]))?;
    v_kt = py::truediv(&v_ghi.clone(), &py::mul(&f__g_on_spencer(v_n.clone())?, &py::minmax(&[f__cosd(v_zenith.clone())?, Py::Float(0.065)], true)?)?)?;
    v_kt = py::minmax(&[py::minmax(&[v_kt.clone(), Py::Float(0.0)], true)?, Py::Float(1.0)], false)?;
    if ({ let t1337 = v_kt.clone(); Py::Bool({ let t1338 = Py::Float(0.22); if py::le(&t1337, &t1338)? { true } else { false } }) }).truthy() {
        v_df = py::sub(&Py::Int(1i64), &py::mul(&Py::Float(0.09), &v_kt.clone())?)?;
    } else {
        if ({ let t1339 = v_kt.clone(); Py::Bool({ let t1340 = Py::Float(0.8); if py::le(&t1339, &t1340)? { true } else { false } }) }).truthy() {
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
    if ({ let t1395 = v_method.clone(); Py::Bool({ let t1396 = Py::str("asce"); if py::eq(&t1395, &t1396) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t1449 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1449.truthy() { t1449 } else { { let t1447 = v_N.clone(); Py::Bool({ let t1448 = Py::Int(1i64); if py::ge(&t1447, &t1448)? { true } else { false } }) } } }, Py::str("the number of cells must be a whole number >= 1"))?;
    let _ = f__need({ let t1450 = v_V_cell.clone(); Py::Bool({ let t1451 = Py::Float(1.25); if py::lt(&t1450, &t1451)? { true } else { false } }) }, Py::str("a hydrogen fuel cell's voltage must stay below the thermoneutral (LHV) voltage of about 1.25 V"))?;
    let _ = f__need({ let t1452 = Py::Int(0i64); Py::Bool({ let t1453 = v_utilization.clone(); if py::lt(&t1452, &t1453)? { { let t1454 = Py::Int(1i64); if py::le(&t1453, &t1454)? { true } else { false } } } else { false } }) }, Py::str("the fuel utilization must lie in (0, 1]"))?;
    v_n_h2 = py::truediv(&py::mul(&v_N.clone(), &v_I.clone())?, &py::mul(&Py::Int(2i64), &c_F_FARADAY()?)?)?;
    v_P = py::mul(&py::mul(&v_N.clone(), &v_I.clone())?, &v_V_cell.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("H2_kg_s"), py::mul(&v_n_h2.clone(), &c_M_H2()?)?), (Py::str("H2_g_h"), py::mul(&py::mul(&v_n_h2.clone(), &c_M_H2()?)?, &Py::Float(3600000.0))?), (Py::str("H2_fed_kg_s"), py::truediv(&py::mul(&v_n_h2.clone(), &c_M_H2()?)?, &v_utilization.clone())?), (Py::str("O2_kg_s"), py::mul(&py::truediv(&v_n_h2.clone(), &Py::Int(2i64))?, &c_M_O2()?)?), (Py::str("H2O_kg_s"), py::mul(&v_n_h2.clone(), &c_M_H2O()?)?), (Py::str("efficiency_LHV"), py::truediv(&v_P.clone(), &py::mul(&v_n_h2.clone(), &c_LHV_H2()?)?)?), (Py::str("efficiency_HHV"), py::truediv(&v_P.clone(), &py::mul(&v_n_h2.clone(), &c_HHV_H2()?)?)?), (Py::str("heat_LHV"), py::sub(&py::mul(&v_n_h2.clone(), &c_LHV_H2()?)?, &v_P.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gmd(mut v_D_ab: Py, mut v_D_bc: Py, mut v_D_ca: Py) -> R {
    let _ = f__pos(Py::str("the phase spacings"), Py::tuple(vec![v_D_ab.clone(), v_D_bc.clone(), v_D_ca.clone()]))?;
    let _ = f__need({ let t1512 = { let t1505 = py::add(&v_D_ab.clone(), &v_D_bc.clone())?; Py::Bool({ let t1506 = v_D_ca.clone(); if py::ge(&t1505, &t1506)? { true } else { false } }) }; if !t1512.truthy() { t1512 } else { { let t1511 = { let t1507 = py::add(&v_D_bc.clone(), &v_D_ca.clone())?; Py::Bool({ let t1508 = v_D_ab.clone(); if py::ge(&t1507, &t1508)? { true } else { false } }) }; if !t1511.truthy() { t1511 } else { { let t1509 = py::add(&v_D_ca.clone(), &v_D_ab.clone())?; Py::Bool({ let t1510 = v_D_bc.clone(); if py::ge(&t1509, &t1510)? { true } else { false } }) } } } } }, Py::str("the three spacings must form a triangle (or a line)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("GMD"), py::pow(&py::mul(&py::mul(&v_D_ab.clone(), &v_D_bc.clone())?, &v_D_ca.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?), (Py::str("ratio_to_mean"), py::truediv(&py::pow(&py::mul(&py::mul(&v_D_ab.clone(), &v_D_bc.clone())?, &v_D_ca.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &py::truediv(&py::add(&py::add(&v_D_ab.clone(), &v_D_bc.clone())?, &v_D_ca.clone())?, &Py::Int(3i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_haurwitz(mut v_zenith: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let _ = f__need({ let t468 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t468.truthy() { t468 } else { { let t465 = Py::Int(0i64); Py::Bool({ let t466 = v_zenith.clone(); if py::le(&t465, &t466)? { { let t467 = Py::Int(180i64); if py::le(&t466, &t467)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must lie between 0 and 180 degrees"))?;
    v_c = f__cosd(v_zenith.clone())?;
    v_g = (if ({ let t469 = v_c.clone(); Py::Bool({ let t470 = Py::Int(0i64); if py::gt(&t469, &t470)? { true } else { false } }) }).truthy() { py::mul(&py::mul(&Py::Float(1098.0), &v_c.clone())?, &py::math1("exp", &py::truediv(&py::neg(&Py::Float(0.059))?, &v_c.clone())?)?)? } else { Py::Float(0.0) });
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
    let _ = f__need({ let t1568 = v_T_hub.clone(); Py::Bool({ let t1569 = Py::Int(150i64); if py::gt(&t1568, &t1569)? { true } else { false } }) }, Py::str("the temperature at hub height must stay above 150 K"))?;
    v_p_hpa = py::sub(&py::truediv(&v_p.clone(), &Py::Int(100i64))?, &py::truediv(&py::mul(&py::sub(&v_hub.clone(), &v_h_p.clone())?, &Py::Int(1i64))?, &Py::Int(8i64))?)?;
    let _ = f__need({ let t1570 = v_p_hpa.clone(); Py::Bool({ let t1571 = Py::Int(0i64); if py::gt(&t1570, &t1571)? { true } else { false } }) }, Py::str("the pressure at hub height must stay positive"))?;
    v_rho = (if ({ let t1572 = v_model.clone(); Py::Bool({ let t1573 = Py::str("barometric"); if py::eq(&t1572, &t1573) { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&py::mul(&v_p_hpa.clone(), &Py::Float(1.225))?, &Py::Float(288.15))?, &Py::Int(100i64))?, &py::mul(&Py::Int(101330i64), &v_T_hub.clone())?)? } else { py::truediv(&py::mul(&v_p_hpa.clone(), &Py::Int(100i64))?, &py::mul(&Py::Float(287.058), &v_T_hub.clone())?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("T_hub"), v_T_hub.clone()), (Py::str("p_hub"), py::mul(&v_p_hpa.clone(), &Py::Int(100i64))?), (Py::str("rho"), v_rho.clone())]))?);
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
    let _ = f__need({ let t1625 = v_p2.clone(); Py::Bool({ let t1626 = v_p1.clone(); if py::gt(&t1625, &t1626)? { true } else { false } }) }, Py::str("the final pressure must exceed the initial pressure"))?;
    let _ = f__need({ let t1627 = v_p2.clone(); Py::Bool({ let t1628 = Py::Float(150000000.0); if py::le(&t1627, &t1628)? { true } else { false } }) }, Py::str("the final pressure must not exceed 1500 bar"))?;
    let t1672 = (f__h2_s_h(v_p1.clone(), v_T1.clone())?).items()?;
    if t1672.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t1672[0].clone();
    v_h1 = t1672[1].clone();
    v_T2 = f__illinois({ let v_T1 = v_T1.clone(); let v_h1 = v_h1.clone(); let v_p1 = v_p1.clone(); let v_p2 = v_p2.clone(); let v_s1 = v_s1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::getitem(&f__h2_s_h(v_p2.clone(), v_T.clone())?, &Py::Int(0i64))?, &v_s1.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_T1.clone(), py::mul(&Py::Int(20i64), &v_T1.clone())?, Py::Int(200i64))?;
    let _ = f__need({ let t1673 = v_T2.clone(); Py::Bool({ let t1674 = Py::Int(1000i64); if py::le(&t1673, &t1674)? { true } else { false } }) }, Py::str("the outlet temperature leaves the range of the equation of state (1000 K)"))?;
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
    let _ = f__need({ let t1726 = v_p2.clone(); Py::Bool({ let t1727 = v_p1.clone(); if py::gt(&t1726, &t1727)? { true } else { false } }) }, Py::str("the final pressure must exceed the initial pressure"))?;
    v_t = py::truediv(&c__H2_TC()?, &v_T.clone())?;
    v_g = Py::list(vec![]);
    for t1728 in py::iter(&(Py::tuple(vec![v_p1.clone(), v_p2.clone()])))? {
        v_p = t1728;
        v_rm = f__h2_rho(v_p.clone(), v_T.clone())?;
        v_d = py::truediv(&v_rm.clone(), &c__H2_RHOC()?)?;
        let t1767 = (f__h2_ar(v_d.clone(), v_t.clone())?).items()?;
        if t1767.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_ar = t1767[0].clone();
        v_ard = t1767[1].clone();
        v__ = t1767[2].clone();
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
    let _ = f__need({ let t1910 = Py::Bool(matches!(v_aoi.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1910.truthy() { t1910 } else { { let t1907 = Py::Int(0i64); Py::Bool({ let t1908 = v_aoi.clone(); if py::le(&t1907, &t1908)? { { let t1909 = Py::Int(180i64); if py::le(&t1908, &t1909)? { true } else { false } } } else { false } }) } } }, Py::str("the angle of incidence must lie between 0 and 180 degrees"))?;
    let _ = f__need({ let t1911 = Py::Int(0i64); Py::Bool({ let t1912 = v_b0.clone(); if py::lt(&t1911, &t1912)? { { let t1913 = Py::Int(1i64); if py::lt(&t1912, &t1913)? { true } else { false } } } else { false } }) }, Py::str("b0 must lie between 0 and 1"))?;
    if ({ let t1914 = v_aoi.clone(); Py::Bool({ let t1915 = Py::Int(90i64); if py::ge(&t1914, &t1915)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t1964 = Py::Bool(matches!(v_aoi.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1964.truthy() { t1964 } else { { let t1961 = Py::Int(0i64); Py::Bool({ let t1962 = v_aoi.clone(); if py::le(&t1961, &t1962)? { { let t1963 = Py::Int(90i64); if py::lt(&t1962, &t1963)? { true } else { false } } } else { false } }) } } }, Py::str("the angle of incidence must lie between 0 and 90 degrees"))?;
    let _ = f__need({ let t1965 = Py::Float(1.0); Py::Bool({ let t1966 = v_n.clone(); if py::lt(&t1965, &t1966)? { { let t1967 = Py::Float(3.0); if py::lt(&t1966, &t1967)? { true } else { false } } } else { false } }) }, Py::str("the refractive index must lie between 1 and 3"))?;
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
    let t2036 = (Py::tuple(vec![py::complex(&[v_R1.clone(), v_X1.clone()])?, py::complex(&[Py::Int(0i64), v_Xm.clone()])?])).items()?;
    if t2036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z1 = t2036[0].clone();
    v_Zm = t2036[1].clone();
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
    let _ = f__need({ let t2104 = Py::Int(0i64); Py::Bool({ let t2105 = v_slip.clone(); if py::lt(&t2104, &t2105)? { { let t2106 = Py::Int(1i64); if py::le(&t2105, &t2106)? { true } else { false } } } else { false } }) }, Py::str("the slip must lie in (0, 1] (motoring)"))?;
    let _ = f__nonneg(Py::str("the rotational loss"), Py::tuple(vec![v_P_rot.clone()]))?;
    v_ns = py::truediv(&py::mul(&Py::Int(120i64), &v_f.clone())?, &v_poles.clone())?;
    v_ws = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_poles.clone())?;
    v_V = f__im_phase(v_VL.clone(), v_connection.clone())?;
    let t2109 = (Py::tuple(vec![py::complex(&[v_R1.clone(), v_X1.clone()])?, py::complex(&[Py::Int(0i64), v_Xm.clone()])?, py::complex(&[py::truediv(&v_R2.clone(), &v_slip.clone())?, v_X2.clone()])?])).items()?;
    if t2109.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Z1 = t2109[0].clone();
    v_Zm = t2109[1].clone();
    v_Z2 = t2109[2].clone();
    v_Zin = py::add(&v_Z1.clone(), &py::truediv(&py::mul(&v_Zm.clone(), &v_Z2.clone())?, &py::add(&v_Zm.clone(), &v_Z2.clone())?)?)?;
    v_I1 = py::truediv(&v_V.clone(), &v_Zin.clone())?;
    v_E = py::sub(&v_V.clone(), &py::mul(&v_I1.clone(), &v_Z1.clone())?)?;
    v_I2 = py::truediv(&v_E.clone(), &v_Z2.clone())?;
    v_Pin = py::mul(&Py::Int(3i64), &py::real(&py::mul(&v_V.clone(), &py::method(&v_I1.clone(), "conjugate", &[])?)?)?)?;
    v_Pag = py::truediv(&py::mul(&py::mul(&Py::Int(3i64), &py::pow(&py::abs(&v_I2.clone())?, &Py::Int(2i64))?)?, &v_R2.clone())?, &v_slip.clone())?;
    v_Pconv = py::mul(&py::sub(&Py::Int(1i64), &v_slip.clone())?, &v_Pag.clone())?;
    v_Pout = py::sub(&v_Pconv.clone(), &v_P_rot.clone())?;
    v_IL = (if ({ let t2110 = v_connection.clone(); Py::Bool({ let t2111 = Py::str("star"); if py::eq(&t2110, &t2111) { true } else { false } }) }).truthy() { py::abs(&v_I1.clone())? } else { py::mul(&c_SQ3()?, &py::abs(&v_I1.clone())?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("I_line"), v_IL.clone()), (Py::str("I2"), py::abs(&v_I2.clone())?), (Py::str("pf"), py::math1("cos", &py::neg(&py::cmath("phase", &[v_I1.clone()])?)?)?), (Py::str("P_in"), v_Pin.clone()), (Py::str("P_ag"), v_Pag.clone()), (Py::str("P_conv"), v_Pconv.clone()), (Py::str("T"), py::truediv(&v_Pag.clone(), &v_ws.clone())?), (Py::str("P_out"), v_Pout.clone()), (Py::str("efficiency_pct"), py::truediv(&py::mul(&Py::Int(100i64), &v_Pout.clone())?, &v_Pin.clone())?)]))?);
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
    let _ = f__need({ let t2165 = Py::Bool(matches!(v_n_bundle.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2165.truthy() { t2165 } else { { let t2162 = Py::Int(1i64); Py::Bool({ let t2163 = v_n_bundle.clone(); if py::le(&t2162, &t2163)? { { let t2164 = Py::Int(4i64); if py::le(&t2163, &t2164)? { true } else { false } } } else { false } }) } } }, Py::str("the bundle has 1 to 4 subconductors"))?;
    v_g = (if ({ let t2166 = v_gmr.clone(); Py::Bool({ let t2167 = Py::None; if py::is(&t2166, &t2167) { true } else { false } }) }).truthy() { py::mul(&v_r.clone(), &py::math1("exp", &py::neg(&Py::Float(0.25))?)?)? } else { v_gmr.clone() });
    let _ = f__pos(Py::str("the geometric mean radius"), Py::tuple(vec![v_g.clone()]))?;
    let _ = f__need({ let t2168 = v_g.clone(); Py::Bool({ let t2169 = v_r.clone(); if py::le(&t2168, &t2169)? { true } else { false } }) }, Py::str("the geometric mean radius cannot exceed the conductor radius"))?;
    if ({ let t2170 = v_n_bundle.clone(); Py::Bool({ let t2171 = Py::Int(1i64); if py::gt(&t2170, &t2171)? { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2176 = { let t2172 = v_spacing.clone(); Py::Bool({ let t2173 = Py::None; if !py::is(&t2172, &t2173) { true } else { false } }) }; if !t2176.truthy() { t2176 } else { { let t2174 = v_spacing.clone(); Py::Bool({ let t2175 = py::mul(&Py::Int(2i64), &v_r.clone())?; if py::gt(&t2174, &t2175)? { true } else { false } }) } } }, Py::str("a bundle needs a subconductor spacing larger than the conductor diameter"))?;
    }
    v_d = { let t2177 = v_spacing.clone(); if t2177.truthy() { t2177 } else { Py::Float(0.0) } };
    v_gmd = py::pow(&py::mul(&py::mul(&v_D_ab.clone(), &v_D_bc.clone())?, &v_D_ca.clone())?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    let t2184 = (Py::tuple(vec![f__bundle_equiv(v_g.clone(), v_n_bundle.clone(), v_d.clone())?, f__bundle_equiv(v_r.clone(), v_n_bundle.clone(), v_d.clone())?])).items()?;
    if t2184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gmr_b = t2184[0].clone();
    v_r_b = t2184[1].clone();
    let _ = f__need({ let t2185 = v_gmd.clone(); Py::Bool({ let t2186 = py::mul(&Py::Int(10i64), &v_r_b.clone())?; if py::gt(&t2185, &t2186)? { true } else { false } }) }, Py::str("the phase spacing must be much larger than the conductor (bundle) size"))?;
    v_L = py::mul(&py::truediv(&c_MU0()?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, &py::math1("log", &py::truediv(&v_gmd.clone(), &v_gmr_b.clone())?)?)?;
    v_C = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &c_EPS0()?)?, &py::math1("log", &py::truediv(&v_gmd.clone(), &v_r_b.clone())?)?)?;
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("GMR"), v_gmr_b.clone()), (Py::str("L_mH_per_km"), py::mul(&v_L.clone(), &Py::Float(1000000.0))?), (Py::str("C_nF_per_km"), py::mul(&v_C.clone(), &Py::Float(1000000000000.0))?), (Py::str("X_ohm_per_km"), py::mul(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Float(1000.0))?), (Py::str("B_uS_per_km"), py::mul(&py::mul(&v_w.clone(), &v_C.clone())?, &Py::Float(1000000000.0))?), (Py::str("Zc"), py::math1("sqrt", &py::truediv(&v_L.clone(), &v_C.clone())?)?)])?;
    if ({ let t2187 = v_V.clone(); Py::Bool({ let t2188 = Py::None; if !py::is(&t2187, &t2188) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t2316 = py::add(&v_R.clone(), &v_X.clone())?; Py::Bool({ let t2317 = Py::Int(0i64); if py::gt(&t2316, &t2317)? { true } else { false } }) }, Py::str("the series impedance must not be zero"))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t2318 = (Py::tuple(vec![ax_engine::pymp::call("mpc", &[v_R.clone(), v_X.clone()], &[])?, ax_engine::pymp::call("mpc", &[Py::Int(0i64), v_B.clone()], &[])?])).items()?;
        if t2318.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_z = t2318[0].clone();
        v_y = t2318[1].clone();
        v_g = ax_engine::pymp::call("sqrt", &[py::mul(&v_z.clone(), &v_y.clone())?], &[])?;
        if ({ let t2319 = py::real(&v_g.clone())?; Py::Bool({ let t2320 = Py::Int(0i64); if py::lt(&t2319, &t2320)? { true } else { false } }) }).truthy() {
            v_g = py::neg(&v_g.clone())?;
        }
        v_zc = ax_engine::pymp::call("sqrt", &[py::truediv(&v_z.clone(), &v_y.clone())?], &[])?;
        if ({ let t2321 = py::real(&v_zc.clone())?; Py::Bool({ let t2322 = Py::Int(0i64); if py::lt(&t2321, &t2322)? { true } else { false } }) }).truthy() {
            v_zc = py::neg(&v_zc.clone())?;
        }
        let t2323 = (Py::tuple(vec![py::complex(&[v_g.clone()])?, py::complex(&[v_zc.clone()])?])).items()?;
        if t2323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_gl = t2323[0].clone();
        v_Zc = t2323[1].clone();
        let t2324 = (Py::tuple(vec![py::complex(&[ax_engine::pymp::call("cosh", &[v_g.clone()], &[])?])?, py::complex(&[py::mul(&v_zc.clone(), &ax_engine::pymp::call("sinh", &[v_g.clone()], &[])?)?])?, py::complex(&[py::truediv(&ax_engine::pymp::call("sinh", &[v_g.clone()], &[])?, &v_zc.clone())?])?])).items()?;
        if t2324.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_A = t2324[0].clone();
        v_Bc = t2324[1].clone();
        v_C = t2324[2].clone();
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
    let _ = f__need({ let t4910 = Py::Bool(matches!(v_poles.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4910.truthy() { t4910 } else { { let t4909 = { let t4902 = v_poles.clone(); Py::Bool({ let t4903 = Py::Int(2i64); if py::ge(&t4902, &t4903)? { true } else { false } }) }; if !t4909.truthy() { t4909 } else { { let t4908 = { let t4904 = py::modulo(&v_poles.clone(), &Py::Int(2i64))?; Py::Bool({ let t4905 = Py::Int(0i64); if py::eq(&t4904, &t4905) { true } else { false } }) }; if !t4908.truthy() { t4908 } else { { let t4906 = v_poles.clone(); Py::Bool({ let t4907 = Py::Int(200i64); if py::le(&t4906, &t4907)? { true } else { false } }) } } } } } } }, Py::str("the number of poles must be an even whole number from 2 to 200"))?;
    let _ = f__need({ let t4915 = { let t4911 = v_s.clone(); Py::Bool({ let t4912 = Py::None; if py::is(&t4911, &t4912) { true } else { false } }) }; if t4915.truthy() { t4915 } else { { let t4913 = v_n_r.clone(); Py::Bool({ let t4914 = Py::None; if py::is(&t4913, &t4914) { true } else { false } }) } } }, Py::str("give the slip or the rotor speed, not both"))?;
    v_ns = py::truediv(&py::mul(&Py::Float(120.0), &v_f.clone())?, &v_poles.clone())?;
    v_ws = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_poles.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("n_sync_rpm"), v_ns.clone()), (Py::str("omega_sync"), v_ws.clone())])?;
    if ({ let t4916 = v_s.clone(); Py::Bool({ let t4917 = Py::None; if !py::is(&t4916, &t4917) { true } else { false } }) }).truthy() {
        let _ = f__fin(Py::str("the slip"), v_s.clone())?;
        let _ = f__need({ let t4920 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t4921 = v_s.clone(); if py::lt(&t4920, &t4921)? { { let t4922 = Py::Int(2i64); if py::lt(&t4921, &t4922)? { true } else { false } } } else { false } }) }, Py::str("the slip must lie between -1 and 2"))?;
        let _ = { let t4923 = v_out.clone(); py::setitem(&t4923, &Py::str("n_rotor_rpm"), py::mul(&v_ns.clone(), &py::sub(&Py::Int(1i64), &v_s.clone())?)?)?; py::setitem(&t4923, &Py::str("slip"), v_s.clone())?; py::setitem(&t4923, &Py::str("slip_frequency"), py::mul(&v_s.clone(), &v_f.clone())?)?; Py::None };
    }
    if ({ let t4924 = v_n_r.clone(); Py::Bool({ let t4925 = Py::None; if !py::is(&t4924, &t4925) { true } else { false } }) }).truthy() {
        let _ = f__fin(Py::str("the rotor speed"), v_n_r.clone())?;
        v_nr = py::truediv(&py::mul(&v_n_r.clone(), &Py::Int(60i64))?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
        v_sl = py::truediv(&py::sub(&v_ns.clone(), &v_nr.clone())?, &v_ns.clone())?;
        let _ = { let t4926 = v_out.clone(); py::setitem(&t4926, &Py::str("n_rotor_rpm"), v_nr.clone())?; py::setitem(&t4926, &Py::str("slip"), v_sl.clone())?; py::setitem(&t4926, &Py::str("slip_frequency"), py::mul(&v_sl.clone(), &v_f.clone())?)?; Py::None };
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
    let _ = f__need({ let t2389 = py::add(&v_R.clone(), &v_X.clone())?; Py::Bool({ let t2390 = Py::Int(0i64); if py::gt(&t2389, &t2390)? { true } else { false } }) }, Py::str("the series impedance must not be zero"))?;
    let t2391 = (Py::tuple(vec![py::complex(&[v_R.clone(), v_X.clone()])?, py::complex(&[Py::Float(0.0), v_B.clone()])?])).items()?;
    if t2391.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z = t2391[0].clone();
    v_Y = t2391[1].clone();
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
    let _ = f__need({ let t2446 = Py::Float(0.8); Py::Bool({ let t2447 = v_V1.clone(); if py::le(&t2446, &t2447)? { { let t2448 = Py::Float(1.2); if py::le(&t2447, &t2448)? { true } else { false } } } else { false } }) }, Py::str("the slack voltage must lie between 0.8 and 1.2 pu"))?;
    v_br = Py::list(vec![]);
    for t2449 in py::iter(&(v_lines.clone()))? {
        v_row = t2449;
        let _ = f__need({ let t2450 = py::len(&v_row.clone())?; Py::Bool({ let t2451 = Py::Int(5i64); if py::eq(&t2450, &t2451) { true } else { false } }) }, Py::str("each line row is: from bus, to bus, r, x, total shunt b (per unit)"))?;
        let t2452 = (Py::tuple(vec![py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, py::int(&py::getitem(&v_row.clone(), &Py::Int(1i64))?)?])).items()?;
        if t2452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2452[0].clone();
        v_j = t2452[1].clone();
        let _ = f__need({ let t2466 = { let t2453 = py::getitem(&v_row.clone(), &Py::Int(0i64))?; Py::Bool({ let t2454 = v_i.clone(); if py::eq(&t2453, &t2454) { true } else { false } }) }; if !t2466.truthy() { t2466 } else { { let t2465 = { let t2455 = py::getitem(&v_row.clone(), &Py::Int(1i64))?; Py::Bool({ let t2456 = v_j.clone(); if py::eq(&t2455, &t2456) { true } else { false } }) }; if !t2465.truthy() { t2465 } else { { let t2464 = { let t2457 = v_i.clone(); Py::Bool({ let t2458 = v_j.clone(); if py::ne(&t2457, &t2458) { true } else { false } }) }; if !t2464.truthy() { t2464 } else { { let t2463 = { let t2459 = v_i.clone(); Py::Bool({ let t2460 = Py::Int(1i64); if py::ge(&t2459, &t2460)? { true } else { false } }) }; if !t2463.truthy() { t2463 } else { { let t2461 = v_j.clone(); Py::Bool({ let t2462 = Py::Int(1i64); if py::ge(&t2461, &t2462)? { true } else { false } }) } } } } } } } } }, Py::str("line buses must be different whole numbers from 1"))?;
        let t2467 = (Py::tuple(vec![py::getitem(&v_row.clone(), &Py::Int(2i64))?, py::getitem(&v_row.clone(), &Py::Int(3i64))?, py::getitem(&v_row.clone(), &Py::Int(4i64))?])).items()?;
        if t2467.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_r = t2467[0].clone();
        v_x = t2467[1].clone();
        v_b = t2467[2].clone();
        let _ = f__need({ let t2475 = { let t2468 = v_r.clone(); Py::Bool({ let t2469 = Py::Int(0i64); if py::ge(&t2468, &t2469)? { true } else { false } }) }; if !t2475.truthy() { t2475 } else { { let t2474 = { let t2470 = v_x.clone(); Py::Bool({ let t2471 = Py::Int(0i64); if py::gt(&t2470, &t2471)? { true } else { false } }) }; if !t2474.truthy() { t2474 } else { { let t2472 = v_b.clone(); Py::Bool({ let t2473 = Py::Int(0i64); if py::ge(&t2472, &t2473)? { true } else { false } }) } } } } }, Py::str("line r and b must not be negative and x must be positive"))?;
        let _ = py::method(&v_br.clone(), "append", &[Py::tuple(vec![v_i.clone(), v_j.clone(), py::complex(&[v_r.clone(), v_x.clone()])?, v_b.clone()])])?;
    }
    v_nb = py::minmax(&[{ let mut out = vec![]; for t2476 in py::iter(&(v_br.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t2477 = (t2476).items()?;
if t2477.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_i = t2477[0].clone();
v_j = t2477[1].clone();
v__ = t2477[2].clone();
v__ = t2477[3].clone();
out.push(py::minmax(&[v_i.clone(), v_j.clone()], true)?); } Py::list(out) }], true)?;
    let _ = f__need({ let t2478 = py::set(&{ let mut out = vec![]; for t2480 in py::iter(&(v_br.clone()))? { let mut v__: Py; let mut v_i: Py; let mut v_j: Py; 
let t2481 = (t2480).items()?;
if t2481.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_i = t2481[0].clone();
v_j = t2481[1].clone();
v__ = t2481[2].clone();
v__ = t2481[3].clone();
for t2482 in py::iter(&(Py::tuple(vec![v_i.clone(), v_j.clone()])))? { let mut v_k: Py; 
v_k = t2482;
out.push(v_k.clone()); }} Py::list(out) })?; Py::Bool({ let t2479 = py::set(&Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nb.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?; if py::eq(&t2478, &t2479) { true } else { false } }) }, Py::str("buses must be numbered 1 to n without gaps"))?;
    let t2483 = (Py::tuple(vec![py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?, py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?])).items()?;
    if t2483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Pd = t2483[0].clone();
    v_Qd = t2483[1].clone();
    for t2484 in py::iter(&(v_loads.clone()))? {
        v_row = t2484;
        let _ = f__need({ let t2493 = { let t2485 = py::len(&v_row.clone())?; Py::Bool({ let t2486 = Py::Int(3i64); if py::eq(&t2485, &t2486) { true } else { false } }) }; if !t2493.truthy() { t2493 } else { { let t2492 = { let t2487 = py::getitem(&v_row.clone(), &Py::Int(0i64))?; Py::Bool({ let t2488 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::eq(&t2487, &t2488) { true } else { false } }) }; if !t2492.truthy() { t2492 } else { { let t2489 = Py::Int(2i64); Py::Bool({ let t2490 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::le(&t2489, &t2490)? { { let t2491 = v_nb.clone(); if py::le(&t2490, &t2491)? { true } else { false } } } else { false } }) } } } } }, Py::str("each load row is: bus (2..n), P, Q in per unit"))?;
        let t2494 = v_Pd.clone(); let t2495 = py::sub(&py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?;
        py::setitem(&t2494, &t2495, py::iadd(&py::getitem(&t2494, &t2495)?, &py::getitem(&v_row.clone(), &Py::Int(1i64))?)?)?;
        let t2496 = v_Qd.clone(); let t2497 = py::sub(&py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?;
        py::setitem(&t2496, &t2497, py::iadd(&py::getitem(&t2496, &t2497)?, &py::getitem(&v_row.clone(), &Py::Int(2i64))?)?)?;
    }
    let t2498 = (Py::tuple(vec![Py::list(vec![]), py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?, Py::dict(vec![])])).items()?;
    if t2498.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_pv = t2498[0].clone();
    v_Pg = t2498[1].clone();
    v_Vset = t2498[2].clone();
    for t2500 in py::iter(&({ let t2499 = v_gens.clone(); if t2499.truthy() { t2499 } else { Py::list(vec![]) } }))? {
        v_row = t2500;
        let _ = f__need({ let t2509 = { let t2501 = py::len(&v_row.clone())?; Py::Bool({ let t2502 = Py::Int(3i64); if py::eq(&t2501, &t2502) { true } else { false } }) }; if !t2509.truthy() { t2509 } else { { let t2508 = { let t2503 = py::getitem(&v_row.clone(), &Py::Int(0i64))?; Py::Bool({ let t2504 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::eq(&t2503, &t2504) { true } else { false } }) }; if !t2508.truthy() { t2508 } else { { let t2505 = Py::Int(2i64); Py::Bool({ let t2506 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::le(&t2505, &t2506)? { { let t2507 = v_nb.clone(); if py::le(&t2506, &t2507)? { true } else { false } } } else { false } }) } } } } }, Py::str("each generator row is: bus (2..n), P, V (per unit)"))?;
        v_k = py::sub(&py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?;
        let _ = f__need({ let t2510 = v_k.clone(); Py::Bool({ let t2511 = v_Vset.clone(); if !py::contains(&t2511, &t2510)? { true } else { false } }) }, Py::str("one generator row per bus"))?;
        let _ = f__need({ let t2512 = Py::Float(0.8); Py::Bool({ let t2513 = py::getitem(&v_row.clone(), &Py::Int(2i64))?; if py::le(&t2512, &t2513)? { { let t2514 = Py::Float(1.2); if py::le(&t2513, &t2514)? { true } else { false } } } else { false } }) }, Py::str("generator voltages must lie between 0.8 and 1.2 pu"))?;
        let t2515 = v_Pg.clone(); let t2516 = v_k.clone();
        py::setitem(&t2515, &t2516, py::iadd(&py::getitem(&t2515, &t2516)?, &py::getitem(&v_row.clone(), &Py::Int(1i64))?)?)?;
        py::setitem(&v_Vset.clone(), &v_k.clone(), py::getitem(&v_row.clone(), &Py::Int(2i64))?)?;
    }
    v_Y = { let mut out = vec![]; for t2517 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v__: Py; 
v__ = t2517;
out.push(py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_nb.clone())?); } Py::list(out) };
    for t2518 in py::iter(&(v_br.clone()))? {
        let t2527 = (t2518).items()?;
        if t2527.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2527[0].clone();
        v_j = t2527[1].clone();
        v_z = t2527[2].clone();
        v_b = t2527[3].clone();
        v_y = py::truediv(&Py::Int(1i64), &v_z.clone())?;
        let t2519 = py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t2520 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t2519, &t2520, py::iadd(&py::getitem(&t2519, &t2520)?, &py::add(&v_y.clone(), &py::mul(&Py::Complex(0.0, 0.5), &v_b.clone())?)?)?)?;
        let t2521 = py::getitem(&v_Y.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t2522 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
        py::setitem(&t2521, &t2522, py::iadd(&py::getitem(&t2521, &t2522)?, &py::add(&v_y.clone(), &py::mul(&Py::Complex(0.0, 0.5), &v_b.clone())?)?)?)?;
        let t2523 = py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t2524 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
        py::setitem(&t2523, &t2524, py::sub(&py::getitem(&t2523, &t2524)?, &v_y.clone())?)?;
        let t2525 = py::getitem(&v_Y.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t2526 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t2525, &t2526, py::sub(&py::getitem(&t2525, &t2526)?, &v_y.clone())?)?;
    }
    v_V = py::mul(&Py::list(vec![Py::Float(1.0)]), &v_nb.clone())?;
    py::setitem(&v_V.clone(), &Py::Int(0i64), v_V1.clone())?;
    for t2528 in py::iter(&(py::method(&v_Vset.clone(), "items", &[])?))? {
        let t2529 = (t2528).items()?;
        if t2529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2529[0].clone();
        v_v = t2529[1].clone();
        py::setitem(&v_V.clone(), &v_k.clone(), v_v.clone())?;
    }
    v_th = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_nb.clone())?;
    v_pq = { let mut out = vec![]; for t2530 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2530;
if ({ let t2531 = v_k.clone(); Py::Bool({ let t2532 = v_Vset.clone(); if !py::contains(&t2532, &t2531)? { true } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    v_pvb = { let mut out = vec![]; for t2533 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2533;
if ({ let t2534 = v_k.clone(); Py::Bool({ let t2535 = v_Vset.clone(); if py::contains(&t2535, &t2534)? { true } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    v_nsl = Py::list((Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (v_nb.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
    v_Psp = { let mut out = vec![]; for t2536 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2536;
out.push(py::sub(&py::getitem(&v_Pg.clone(), &v_k.clone())?, &py::getitem(&v_Pd.clone(), &v_k.clone())?)?); } Py::list(out) };
    v_Qsp = { let mut out = vec![]; for t2537 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2537;
out.push(py::neg(&py::getitem(&v_Qd.clone(), &v_k.clone())?)?); } Py::list(out) };
    v_calc = { let v_Pd = v_Pd.clone(); let v_Pg = v_Pg.clone(); let v_Psp = v_Psp.clone(); let v_Qd = v_Qd.clone(); let v_Qsp = v_Qsp.clone(); let v_V = v_V.clone(); let v_V1 = v_V1.clone(); let v_Vset = v_Vset.clone(); let v_Y = v_Y.clone(); let v_b = v_b.clone(); let v_br = v_br.clone(); let v_calc = v_calc.clone(); let v_gens = v_gens.clone(); let v_i = v_i.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_lines = v_lines.clone(); let v_loads = v_loads.clone(); let v_nb = v_nb.clone(); let v_nsl = v_nsl.clone(); let v_pq = v_pq.clone(); let v_pv = v_pv.clone(); let v_pvb = v_pvb.clone(); let v_r = v_r.clone(); let v_row = v_row.clone(); let v_th = v_th.clone(); let v_v = v_v.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_S: Py = Py::None;
            let mut v_Vc: Py = Py::None;
            v_Vc = { let mut out = vec![]; for t2538 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2538;
out.push(py::cmath("rect", &[py::getitem(&v_V.clone(), &v_k.clone())?, py::getitem(&v_th.clone(), &v_k.clone())?])?); } Py::list(out) };
            v_S = { let mut out = vec![]; for t2539 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t2539;
out.push(py::mul(&py::getitem(&v_Vc.clone(), &v_k.clone())?, &py::method(&py::sum(&{ let mut out = vec![]; for t2540 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_m: Py; 
v_m = t2540;
out.push(py::mul(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?, &py::getitem(&v_Vc.clone(), &v_m.clone())?)?); } Py::list(out) }, None)?, "conjugate", &[])?)?); } Py::list(out) };
            return Ok(Py::tuple(vec![v_Vc.clone(), v_S.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let mut t2541 = false;
    for t2542 in py::range_iter(&[(Py::Int(40i64)).i()?]) {
        v_it = t2542;
        let t2543 = (py::call(&v_calc.clone(), &[])?).items()?;
        if t2543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Vc = t2543[0].clone();
        v_S = t2543[1].clone();
        v_mis = py::add(&{ let mut out = vec![]; for t2544 in py::iter(&(v_nsl.clone()))? { let mut v_k: Py; 
v_k = t2544;
out.push(py::sub(&py::getitem(&v_Psp.clone(), &v_k.clone())?, &py::real(&py::getitem(&v_S.clone(), &v_k.clone())?)?)?); } Py::list(out) }, &{ let mut out = vec![]; for t2545 in py::iter(&(v_pq.clone()))? { let mut v_k: Py; 
v_k = t2545;
out.push(py::sub(&py::getitem(&v_Qsp.clone(), &v_k.clone())?, &py::imag(&py::getitem(&v_S.clone(), &v_k.clone())?)?)?); } Py::list(out) })?;
        if ({ let t2546 = py::minmax(&[{ let mut out = vec![]; for t2548 in py::iter(&(v_mis.clone()))? { let mut v_m: Py; 
v_m = t2548;
out.push(py::abs(&v_m.clone())?); } Py::list(out) }], true)?; Py::Bool({ let t2547 = Py::Float(1e-13); if py::lt(&t2546, &t2547)? { true } else { false } }) }).truthy() {
            { t2541 = true; break; }
        }
        let t2549 = (Py::tuple(vec![py::len(&v_nsl.clone())?, py::len(&v_pq.clone())?])).items()?;
        if t2549.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n1 = t2549[0].clone();
        v_n2 = t2549[1].clone();
        v_J = { let mut out = vec![]; for t2550 in py::range_iter(&[(py::add(&v_n1.clone(), &v_n2.clone())?).i()?]) { let mut v__: Py; 
v__ = t2550;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n1.clone(), &v_n2.clone())?)?); } Py::list(out) };
        for t2551 in py::iter(&(py::enumerate(&v_nsl.clone(), 0)?))? {
            let t2560 = (t2551).items()?;
            if t2560.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2560[0].clone();
            v_k = t2560[1].clone();
            for t2552 in py::iter(&(py::enumerate(&v_nsl.clone(), 0)?))? {
                let t2555 = (t2552).items()?;
                if t2555.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_c = t2555[0].clone();
                v_m = t2555[1].clone();
                if ({ let t2553 = v_k.clone(); Py::Bool({ let t2554 = v_m.clone(); if py::eq(&t2553, &t2554) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_J.clone(), &v_a.clone())?, &v_c.clone(), py::sub(&py::neg(&py::imag(&py::getitem(&v_S.clone(), &v_k.clone())?)?)?, &py::mul(&py::pow(&py::getitem(&v_V.clone(), &v_k.clone())?, &Py::Int(2i64))?, &py::imag(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_k.clone())?)?)?)?)?;
                } else {
                    v_Ykm = py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?;
                    v_t = py::sub(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&v_th.clone(), &v_m.clone())?)?;
                    py::setitem(&py::getitem(&v_J.clone(), &v_a.clone())?, &v_c.clone(), py::mul(&py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::getitem(&v_V.clone(), &v_m.clone())?)?, &py::sub(&py::mul(&py::real(&v_Ykm.clone())?, &py::math1("sin", &v_t.clone())?)?, &py::mul(&py::imag(&v_Ykm.clone())?, &py::math1("cos", &v_t.clone())?)?)?)?)?;
                }
            }
            for t2556 in py::iter(&(py::enumerate(&v_pq.clone(), 0)?))? {
                let t2559 = (t2556).items()?;
                if t2559.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_c = t2559[0].clone();
                v_m = t2559[1].clone();
                if ({ let t2557 = v_k.clone(); Py::Bool({ let t2558 = v_m.clone(); if py::eq(&t2557, &t2558) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_J.clone(), &v_a.clone())?, &py::add(&v_n1.clone(), &v_c.clone())?, py::add(&py::truediv(&py::real(&py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::getitem(&v_V.clone(), &v_k.clone())?)?, &py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::real(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_k.clone())?)?)?)?)?;
                } else {
                    v_Ykm = py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?;
                    v_t = py::sub(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&v_th.clone(), &v_m.clone())?)?;
                    py::setitem(&py::getitem(&v_J.clone(), &v_a.clone())?, &py::add(&v_n1.clone(), &v_c.clone())?, py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::add(&py::mul(&py::real(&v_Ykm.clone())?, &py::math1("cos", &v_t.clone())?)?, &py::mul(&py::imag(&v_Ykm.clone())?, &py::math1("sin", &v_t.clone())?)?)?)?)?;
                }
            }
        }
        for t2561 in py::iter(&(py::enumerate(&v_pq.clone(), 0)?))? {
            let t2570 = (t2561).items()?;
            if t2570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2570[0].clone();
            v_k = t2570[1].clone();
            for t2562 in py::iter(&(py::enumerate(&v_nsl.clone(), 0)?))? {
                let t2565 = (t2562).items()?;
                if t2565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_c = t2565[0].clone();
                v_m = t2565[1].clone();
                if ({ let t2563 = v_k.clone(); Py::Bool({ let t2564 = v_m.clone(); if py::eq(&t2563, &t2564) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_J.clone(), &py::add(&v_n1.clone(), &v_a.clone())?)?, &v_c.clone(), py::sub(&py::real(&py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::mul(&py::pow(&py::getitem(&v_V.clone(), &v_k.clone())?, &Py::Int(2i64))?, &py::real(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_k.clone())?)?)?)?)?;
                } else {
                    v_Ykm = py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?;
                    v_t = py::sub(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&v_th.clone(), &v_m.clone())?)?;
                    py::setitem(&py::getitem(&v_J.clone(), &py::add(&v_n1.clone(), &v_a.clone())?)?, &v_c.clone(), py::mul(&py::mul(&py::neg(&py::getitem(&v_V.clone(), &v_k.clone())?)?, &py::getitem(&v_V.clone(), &v_m.clone())?)?, &py::add(&py::mul(&py::real(&v_Ykm.clone())?, &py::math1("cos", &v_t.clone())?)?, &py::mul(&py::imag(&v_Ykm.clone())?, &py::math1("sin", &v_t.clone())?)?)?)?)?;
                }
            }
            for t2566 in py::iter(&(py::enumerate(&v_pq.clone(), 0)?))? {
                let t2569 = (t2566).items()?;
                if t2569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_c = t2569[0].clone();
                v_m = t2569[1].clone();
                if ({ let t2567 = v_k.clone(); Py::Bool({ let t2568 = v_m.clone(); if py::eq(&t2567, &t2568) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_J.clone(), &py::add(&v_n1.clone(), &v_a.clone())?)?, &py::add(&v_n1.clone(), &v_c.clone())?, py::sub(&py::truediv(&py::imag(&py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::getitem(&v_V.clone(), &v_k.clone())?)?, &py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::imag(&py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_k.clone())?)?)?)?)?;
                } else {
                    v_Ykm = py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_m.clone())?;
                    v_t = py::sub(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&v_th.clone(), &v_m.clone())?)?;
                    py::setitem(&py::getitem(&v_J.clone(), &py::add(&v_n1.clone(), &v_a.clone())?)?, &py::add(&v_n1.clone(), &v_c.clone())?, py::mul(&py::getitem(&v_V.clone(), &v_k.clone())?, &py::sub(&py::mul(&py::real(&v_Ykm.clone())?, &py::math1("sin", &v_t.clone())?)?, &py::mul(&py::imag(&v_Ykm.clone())?, &py::math1("cos", &v_t.clone())?)?)?)?)?;
                }
            }
        }
        v_dx = f__gauss_solve(v_J.clone(), v_mis.clone())?;
        for t2588 in py::iter(&(py::enumerate(&v_nsl.clone(), 0)?))? {
            let t2591 = (t2588).items()?;
            if t2591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2591[0].clone();
            v_k = t2591[1].clone();
            let t2589 = v_th.clone(); let t2590 = v_k.clone();
            py::setitem(&t2589, &t2590, py::iadd(&py::getitem(&t2589, &t2590)?, &py::getitem(&v_dx.clone(), &v_a.clone())?)?)?;
        }
        for t2592 in py::iter(&(py::enumerate(&v_pq.clone(), 0)?))? {
            let t2595 = (t2592).items()?;
            if t2595.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t2595[0].clone();
            v_m = t2595[1].clone();
            let t2593 = v_V.clone(); let t2594 = v_m.clone();
            py::setitem(&t2593, &t2594, py::iadd(&py::getitem(&t2593, &t2594)?, &py::getitem(&v_dx.clone(), &py::add(&v_n1.clone(), &v_c.clone())?)?)?)?;
        }
        let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2596 in py::iter(&(v_V.clone()))? { let mut v_v: Py; 
v_v = t2596;
out.push({ let t2597 = Py::Float(0.3); Py::Bool({ let t2598 = v_v.clone(); if py::lt(&t2597, &t2598)? { { let t2599 = Py::Int(2i64); if py::lt(&t2598, &t2599)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the load flow diverged (the network cannot carry this loading)"))?;
    }
    if !t2541 {
        return Err(py::value_error(py::to_str(&Py::str("the load flow did not converge in 40 iterations"))));
    }
    let t2600 = (py::call(&v_calc.clone(), &[])?).items()?;
    if t2600.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Vc = t2600[0].clone();
    v_S = t2600[1].clone();
    let _ = f__need({ let t2601 = py::minmax(&[{ let mut out = vec![]; for t2603 in py::iter(&(v_nsl.clone()))? { let mut v_k: Py; 
v_k = t2603;
out.push(py::abs(&py::sub(&py::getitem(&v_Psp.clone(), &v_k.clone())?, &py::real(&py::getitem(&v_S.clone(), &v_k.clone())?)?)?)?); } Py::list(out) }], true)?; Py::Bool({ let t2602 = Py::Float(1e-10); if py::lt(&t2601, &t2602)? { true } else { false } }) }, Py::str("the load flow did not converge"))?;
    v_loss = py::sum(&{ let mut out = vec![]; for t2604 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t2604;
out.push(py::real(&v_s.clone())?); } Py::list(out) }, None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), Py::list((v_V.clone()).items()?)), (Py::str("angle_deg"), { let mut out = vec![]; for t2617 in py::iter(&(v_th.clone()))? { let mut v_t: Py; 
v_t = t2617;
out.push(py::math1("degrees", &v_t.clone())?); } Py::list(out) }), (Py::str("P_slack"), py::real(&py::getitem(&v_S.clone(), &Py::Int(0i64))?)?), (Py::str("Q_slack"), py::imag(&py::getitem(&v_S.clone(), &Py::Int(0i64))?)?), (Py::str("P_loss"), v_loss.clone()), (Py::str("Q_gen"), { let mut out = vec![]; for t2618 in py::iter(&(v_pvb.clone()))? { let mut v_k: Py; 
v_k = t2618;
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
    for t2657 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_vk1.clone(), v_vkr1.clone()]), Py::tuple(vec![v_vk2.clone(), v_vkr2.clone()])])))? {
        let t2662 = (t2657).items()?;
        if t2662.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_vk = t2662[0].clone();
        v_vkr = t2662[1].clone();
        let _ = f__need({ let t2658 = Py::Int(0i64); Py::Bool({ let t2659 = v_vkr.clone(); if py::le(&t2658, &t2659)? { { let t2660 = v_vk.clone(); if py::lt(&t2659, &t2660)? { { let t2661 = Py::Float(0.5); if py::lt(&t2660, &t2661)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("need 0 <= vkr < vk below 50% for each transformer"))?;
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
    let t2663 = (Py::tuple(vec![py::call(&v_zb.clone(), &[v_vk1.clone(), v_vkr1.clone(), v_S1.clone()])?, py::call(&v_zb.clone(), &[v_vk2.clone(), v_vkr2.clone(), v_S2.clone()])?])).items()?;
    if t2663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z1 = t2663[0].clone();
    v_Z2 = t2663[1].clone();
    v_SL = py::complex(&[v_P.clone(), v_Q.clone()])?;
    let t2664 = (Py::tuple(vec![py::mul(&v_SL.clone(), &py::method(&py::truediv(&v_Z2.clone(), &py::add(&v_Z1.clone(), &v_Z2.clone())?)?, "conjugate", &[])?)?, py::mul(&v_SL.clone(), &py::method(&py::truediv(&v_Z1.clone(), &py::add(&v_Z1.clone(), &v_Z2.clone())?)?, "conjugate", &[])?)?])).items()?;
    if t2664.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t2664[0].clone();
    v_s2 = t2664[1].clone();
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
    let _ = f__need({ let t2774 = { let t2770 = v_R.clone(); Py::Bool({ let t2771 = Py::Int(0i64); if py::gt(&t2770, &t2771)? { true } else { false } }) }; if t2774.truthy() { t2774 } else { { let t2772 = v_X.clone(); Py::Bool({ let t2773 = Py::Int(0i64); if py::gt(&t2772, &t2773)? { true } else { false } }) } } }, Py::str("the impedance must not be zero"))?;
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
    let _ = f__need({ let t2829 = { let t2825 = py::len(&v_currents.clone())?; Py::Bool({ let t2826 = py::len(&v_times.clone())?; if py::eq(&t2825, &t2826) { true } else { false } }) }; if !t2829.truthy() { t2829 } else { { let t2827 = py::len(&v_currents.clone())?; Py::Bool({ let t2828 = Py::Int(2i64); if py::ge(&t2827, &t2828)? { true } else { false } }) } } }, Py::str("give at least two discharge tests (a current and its run time each)"))?;
    for t2830 in py::iter(&(py::zip(&[v_currents.clone(), v_times.clone()])?))? {
        let t2836 = (t2830).items()?;
        if t2836.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t2836[0].clone();
        v_t = t2836[1].clone();
        let _ = f__need({ let t2835 = { let t2831 = v_c.clone(); Py::Bool({ let t2832 = Py::Int(0i64); if py::gt(&t2831, &t2832)? { true } else { false } }) }; if !t2835.truthy() { t2835 } else { { let t2833 = v_t.clone(); Py::Bool({ let t2834 = Py::Int(0i64); if py::gt(&t2833, &t2834)? { true } else { false } }) } } }, Py::str("test currents and run times must be positive"))?;
    }
    let _ = f__need({ let t2837 = py::len(&py::set(&v_currents.clone())?)?; Py::Bool({ let t2838 = py::len(&v_currents.clone())?; if py::eq(&t2837, &t2838) { true } else { false } }) }, Py::str("the test currents must be different"))?;
    let _ = f__pos(Py::str("the new discharge current"), Py::tuple(vec![v_I.clone()]))?;
    let t2846 = (Py::tuple(vec![{ let mut out = vec![]; for t2844 in py::iter(&(v_currents.clone()))? { let mut v_c: Py; 
v_c = t2844;
out.push(py::math1("log", &v_c.clone())?); } Py::list(out) }, { let mut out = vec![]; for t2845 in py::iter(&(v_times.clone()))? { let mut v_t: Py; 
v_t = t2845;
out.push(py::math1("log", &v_t.clone())?); } Py::list(out) }])).items()?;
    if t2846.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t2846[0].clone();
    v_ys = t2846[1].clone();
    v_n = py::len(&v_xs.clone())?;
    let t2847 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2847.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2847[0].clone();
    v_my = t2847[1].clone();
    v_b = py::truediv(&py::sum(&{ let mut out = vec![]; for t2848 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2849 = (t2848).items()?;
if t2849.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2849[0].clone();
v_y = t2849[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &py::sum(&{ let mut out = vec![]; for t2850 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2850;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?)?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    v_k = py::neg(&v_b.clone())?;
    let _ = f__need({ let t2851 = v_k.clone(); Py::Bool({ let t2852 = Py::Int(0i64); if py::gt(&t2851, &t2852)? { true } else { false } }) }, Py::str("the run time must fall as the current rises (Peukert exponent k > 0)"))?;
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
    let _ = f__need({ let t3164 = Py::Bool(matches!(v_zenith.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3164.truthy() { t3164 } else { { let t3161 = Py::Int(0i64); Py::Bool({ let t3162 = v_zenith.clone(); if py::le(&t3161, &t3162)? { { let t3163 = Py::Int(87i64); if py::lt(&t3162, &t3163)? { true } else { false } } } else { false } }) } } }, Py::str("the zenith angle must lie between 0 and 87 degrees"))?;
    let _ = f__need({ let t3165 = Py::Int(0i64); Py::Bool({ let t3166 = v_azimuth.clone(); if py::le(&t3165, &t3166)? { { let t3167 = Py::Int(360i64); if py::le(&t3166, &t3167)? { true } else { false } } } else { false } }) }, Py::str("the solar azimuth must lie between 0 and 360 degrees"))?;
    let _ = f__check_irr(v_dni.clone(), v_ghi.clone(), v_dhi.clone())?;
    let _ = f__need({ let t3175 = v_dhi.clone(); Py::Bool({ let t3176 = Py::Int(0i64); if py::gt(&t3175, &t3176)? { true } else { false } }) }, Py::str("the Perez model needs a positive diffuse irradiance"))?;
    let _ = f__need({ let t3177 = Py::Int(0i64); Py::Bool({ let t3178 = v_albedo.clone(); if py::le(&t3177, &t3178)? { { let t3179 = Py::Int(1i64); if py::le(&t3178, &t3179)? { true } else { false } } } else { false } }) }, Py::str("the albedo must lie between 0 and 1"))?;
    let _ = f__check_day(v_n.clone())?;
    v_am = f__airmass_rel(v_zenith.clone(), Py::str("kastenyoung1989"))?;
    v_z = py::math1("radians", &v_zenith.clone())?;
    v_delta = py::truediv(&py::mul(&v_dhi.clone(), &v_am.clone())?, &f__g_on_spencer(v_n.clone())?)?;
    v_eps = py::truediv(&py::add(&py::truediv(&py::add(&v_dhi.clone(), &v_dni.clone())?, &v_dhi.clone())?, &py::mul(&Py::Float(1.041), &py::pow(&v_z.clone(), &Py::Int(3i64))?)?)?, &py::add(&Py::Int(1i64), &py::mul(&Py::Float(1.041), &py::pow(&v_z.clone(), &Py::Int(3i64))?)?)?)?;
    v_b = py::minmax(&[{ let mut out = vec![]; for t3198 in py::iter(&(py::enumerate(&c__PEREZ_EDGES()?, 0)?))? { let mut v_e: Py; let mut v_i: Py; 
let t3199 = (t3198).items()?;
if t3199.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t3199[0].clone();
v_e = t3199[1].clone();
if ({ let t3200 = v_eps.clone(); Py::Bool({ let t3201 = v_e.clone(); if py::ge(&t3200, &t3201)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); }} Py::list(out) }], true)?;
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
    let _ = f__need({ let t3337 = py::abs(&v_P.clone())?; Py::Bool({ let t3338 = v_Pmax.clone(); if py::lt(&t3337, &t3338)? { true } else { false } }) }, Py::str("the power exceeds the steady-state limit V1 V2/X"))?;
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
    let _ = f__need({ let t3403 = Py::Float(0.5); Py::Bool({ let t3404 = v_rho.clone(); if py::lt(&t3403, &t3404)? { { let t3405 = Py::Float(1.6); if py::lt(&t3404, &t3405)? { true } else { false } } } else { false } }) }, Py::str("the air density must lie between 0.5 and 1.6 kg/m^3"))?;
    v_corr = Py::list(vec![]);
    for t3406 in py::iter(&(v_speeds.clone()))? {
        v_s = t3406;
        v_e = (if ({ let t3407 = v_s.clone(); Py::Bool({ let t3408 = Py::Float(7.5); if py::le(&t3407, &t3408)? { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &Py::Int(3i64))? } else { (if ({ let t3409 = v_s.clone(); Py::Bool({ let t3410 = Py::Float(12.5); if py::ge(&t3409, &t3410)? { true } else { false } }) }).truthy() { py::truediv(&Py::Int(2i64), &Py::Int(3i64))? } else { py::add(&py::truediv(&Py::Int(1i64), &Py::Int(3i64))?, &py::truediv(&py::mul(&py::sub(&v_s.clone(), &Py::Float(7.5))?, &py::sub(&py::truediv(&Py::Int(2i64), &Py::Int(3i64))?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?, &py::sub(&Py::Float(12.5), &Py::Float(7.5))?)?)? }) });
        let _ = py::method(&v_corr.clone(), "append", &[py::mul(&py::pow(&py::truediv(&Py::Float(1.225), &v_rho.clone())?, &v_e.clone())?, &v_s.clone())?])?;
    }
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3411 in py::iter(&(py::zip(&[v_corr.clone(), py::slice(&v_corr.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3412 = (t3411).items()?;
if t3412.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3412[0].clone();
v_b = t3412[1].clone();
out.push({ let t3413 = v_b.clone(); Py::Bool({ let t3414 = v_a.clone(); if py::gt(&t3413, &t3414)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the density-corrected curve is not increasing; the correction does not apply"))?;
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
    return Ok(f__out(Py::dict(vec![(Py::str("T_cell"), py::getitem(&v_t.clone(), &Py::str("T_cell"))?), (Py::str("T_cell_C"), py::getitem(&v_t.clone(), &Py::str("T_cell_C"))?), (Py::str("pdc"), py::getitem(&v_p.clone(), &Py::str("pdc"))?), (Py::str("derate"), (if ({ let t3575 = v_poa.clone(); Py::Bool({ let t3576 = Py::Int(0i64); if py::gt(&t3575, &t3576)? { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_p.clone(), &Py::str("pdc"))?, &py::mul(&py::mul(&v_poa.clone(), &Py::Float(0.001))?, &v_pdc0.clone())?)? } else { Py::Float(1.0) }))]))?);
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
    let t3665 = ({ let mut v: Vec<Py> = vec![]; v.extend((v_p.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "IL") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument IL".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "I0") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument I0".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "Rs") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument Rs".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "Rsh") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument Rsh".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("_sd_mpp() missing argument a".into())) } } }; if v.len() > 5 { return Err(PyErr::Type("_sd_mpp() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_sd_mpp() got an unexpected keyword argument".into())) } f__sd_mpp(a0, a1, a2, a3, a4)? }).items()?;
    if t3665.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Isc = t3665[0].clone();
    v_Voc = t3665[1].clone();
    v_Imp = t3665[2].clone();
    v_Vmp = t3665[3].clone();
    let _ = f__need({ let t3670 = { let t3666 = v_Vmp.clone(); Py::Bool({ let t3667 = Py::Int(0i64); if py::gt(&t3666, &t3667)? { true } else { false } }) }; if !t3670.truthy() { t3670 } else { { let t3668 = v_Imp.clone(); Py::Bool({ let t3669 = Py::Int(0i64); if py::gt(&t3668, &t3669)? { true } else { false } }) } } }, Py::str("the maximum power point is not in the first quadrant for these parameters"))?;
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
    let _ = f__need({ let t3730 = py::neg(&Py::Float(0.02))?; Py::Bool({ let t3731 = v_gamma.clone(); if py::le(&t3730, &t3731)? { { let t3732 = Py::Float(0.01); if py::le(&t3731, &t3732)? { true } else { false } } } else { false } }) }, Py::str("the temperature coefficient gamma must lie between -0.02 and 0.01 per K (e.g. -0.004 or -0.4%)"))?;
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
    let _ = f__need({ let t3789 = { let t3783 = Py::Float(0.5); Py::Bool({ let t3784 = v_eta_nom.clone(); if py::lt(&t3783, &t3784)? { { let t3785 = Py::Int(1i64); if py::le(&t3784, &t3785)? { true } else { false } } } else { false } }) }; if !t3789.truthy() { t3789 } else { { let t3786 = Py::Float(0.5); Py::Bool({ let t3787 = v_eta_ref.clone(); if py::lt(&t3786, &t3787)? { { let t3788 = Py::Int(1i64); if py::le(&t3787, &t3788)? { true } else { false } } } else { false } }) } } }, Py::str("the efficiencies must lie between 0.5 and 1"))?;
    v_z = py::truediv(&v_pdc.clone(), &v_pdc0.clone())?;
    v_eta = py::mul(&py::truediv(&v_eta_nom.clone(), &v_eta_ref.clone())?, &py::add(&py::sub(&py::mul(&py::neg(&Py::Float(0.0162))?, &v_z.clone())?, &py::truediv(&Py::Float(0.0059), &v_z.clone())?)?, &Py::Float(0.9858))?)?;
    v_pac = py::minmax(&[Py::Float(0.0), py::minmax(&[py::mul(&v_eta_nom.clone(), &v_pdc0.clone())?, py::mul(&v_eta.clone(), &v_pdc.clone())?], false)?], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("pac"), v_pac.clone()), (Py::str("efficiency"), py::truediv(&v_pac.clone(), &v_pdc.clone())?), (Py::str("clipped"), { let t3802 = v_pac.clone(); Py::Bool({ let t3803 = py::mul(&v_eta_nom.clone(), &v_pdc0.clone())?; if py::eq(&t3802, &t3803) { true } else { false } }) })]))?);
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
    for t3837 in py::iter(&(c__PVW_LOSSES()?))? {
        let t3842 = (t3837).items()?;
        if t3842.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t3842[0].clone();
        v_default = t3842[1].clone();
        v_v = py::method(&v_kw.clone(), "get", &[v_name.clone(), py::truediv(&v_default.clone(), &Py::Float(100.0))?])?;
        let _ = f__need({ let t3841 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3841.truthy() { t3841 } else { { let t3838 = Py::Int(0i64); Py::Bool({ let t3839 = v_v.clone(); if py::le(&t3838, &t3839)? { { let t3840 = Py::Int(1i64); if py::lt(&t3839, &t3840)? { true } else { false } } } else { false } }) } } }, Py::str(&["the ".to_string(), py::format(&v_name.clone(), "")?, " loss must lie between 0 and 100%".to_string()].concat()))?;
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
    let t3904 = (f__load_current(v_Vr.clone(), v_P.clone(), v_pf.clone(), v_pf_type.clone())?).items()?;
    if t3904.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t3904[0].clone();
    v_I = t3904[1].clone();
    v_Ic = t3904[2].clone();
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
    let t4089 = (f__sd_mpp(v_IL.clone(), v_I0.clone(), v_Rs.clone(), v_Rsh.clone(), v_nNsVth.clone())?).items()?;
    if t4089.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Isc = t4089[0].clone();
    v_Voc = t4089[1].clone();
    v_Imp = t4089[2].clone();
    v_Vmp = t4089[3].clone();
    let _ = f__need({ let t4094 = { let t4090 = v_Vmp.clone(); Py::Bool({ let t4091 = Py::Int(0i64); if py::gt(&t4090, &t4091)? { true } else { false } }) }; if !t4094.truthy() { t4094 } else { { let t4092 = v_Imp.clone(); Py::Bool({ let t4093 = Py::Int(0i64); if py::gt(&t4092, &t4093)? { true } else { false } }) } } }, Py::str("the maximum power point is not in the first quadrant for these parameters"))?;
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
    v_d = (if ({ let t4307 = v_method.clone(); Py::Bool({ let t4308 = Py::str("cooper"); if py::eq(&t4307, &t4308) { true } else { false } }) }).truthy() { f__decl_cooper(v_n.clone())? } else { f__decl_spencer(v_n.clone())? });
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
    let _ = f__need({ let t4376 = Py::Int(0i64); Py::Bool({ let t4377 = v_clock.clone(); if py::le(&t4376, &t4377)? { { let t4378 = Py::Int(24i64); if py::lt(&t4377, &t4378)? { true } else { false } } } else { false } }) }, Py::str("the clock time must be from 0 to 24 hours (decimal hours or hh:mm)"))?;
    v_d = f__decl_spencer(v_n.clone())?;
    v_eot = f__eot_spencer(v_n.clone())?;
    v_ha = py::add(&py::add(&py::mul(&Py::Float(15.0), &py::sub(&py::sub(&v_clock.clone(), &v_utc_offset.clone())?, &Py::Float(12.0))?)?, &v_longitude.clone())?, &py::truediv(&v_eot.clone(), &Py::Float(4.0))?)?;
    let t4390 = (f__sun_angles(v_latitude.clone(), v_d.clone(), v_ha.clone())?).items()?;
    if t4390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t4390[0].clone();
    v_az = t4390[1].clone();
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
    let _ = f__need({ let t4458 = Py::Int(0i64); Py::Bool({ let t4459 = v_clock.clone(); if py::le(&t4458, &t4459)? { { let t4460 = Py::Int(24i64); if py::lt(&t4459, &t4460)? { true } else { false } } } else { false } }) }, Py::str("the clock time must be from 0 to 24 hours (decimal hours or hh:mm)"))?;
    v_eot = f__eot_spencer(v_n.clone())?;
    v_ha = py::add(&py::add(&py::mul(&Py::Float(15.0), &py::sub(&py::sub(&v_clock.clone(), &v_utc_offset.clone())?, &Py::Float(12.0))?)?, &v_longitude.clone())?, &py::truediv(&v_eot.clone(), &Py::Float(4.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("eot_min"), v_eot.clone()), (Py::str("hour_angle_deg"), v_ha.clone()), (Py::str("solar_time_h"), py::add(&Py::Float(12.0), &py::truediv(&v_ha.clone(), &Py::Float(15.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stack_power(mut v_I: Py, mut v_N: Py, mut v_V_cell: Py) -> R {
    let _ = f__pos(Py::str("the current and the cell voltage"), Py::tuple(vec![v_I.clone(), v_V_cell.clone()]))?;
    let _ = f__need({ let t4514 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4514.truthy() { t4514 } else { { let t4511 = Py::Int(1i64); Py::Bool({ let t4512 = v_N.clone(); if py::le(&t4511, &t4512)? { { let t4513 = Py::Int(100000i64); if py::le(&t4512, &t4513)? { true } else { false } } } else { false } }) } } }, Py::str("the number of cells must be a whole number from 1 to 100,000"))?;
    let _ = f__need({ let t4515 = v_V_cell.clone(); Py::Bool({ let t4516 = Py::Int(5i64); if py::le(&t4515, &t4516)? { true } else { false } }) }, Py::str("the cell voltage must be at most 5 V"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::mul(&py::mul(&v_N.clone(), &v_I.clone())?, &v_V_cell.clone())?), (Py::str("P_kW"), py::truediv(&py::mul(&py::mul(&v_N.clone(), &v_I.clone())?, &v_V_cell.clone())?, &Py::Float(1000.0))?), (Py::str("V_stack"), py::mul(&v_N.clone(), &v_V_cell.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sun_angles(mut v_latitude: Py, mut v_declination: Py, mut v_hour_angle: Py) -> R {
    let mut v_az: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__check_place(v_latitude.clone(), Py::None, Py::None)?;
    let _ = f__need({ let t4580 = py::neg(&Py::Float(23.5))?; Py::Bool({ let t4581 = v_declination.clone(); if py::le(&t4580, &t4581)? { { let t4582 = Py::Float(23.5); if py::le(&t4581, &t4582)? { true } else { false } } } else { false } }) }, Py::str("the declination must lie between -23.5 and 23.5 degrees"))?;
    let _ = f__need({ let t4583 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t4584 = v_hour_angle.clone(); if py::le(&t4583, &t4584)? { { let t4585 = Py::Int(180i64); if py::le(&t4584, &t4585)? { true } else { false } } } else { false } }) }, Py::str("the hour angle must lie between -180 and 180 degrees"))?;
    let t4597 = (f__sun_angles(v_latitude.clone(), py::math1("radians", &v_declination.clone())?, v_hour_angle.clone())?).items()?;
    if t4597.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t4597[0].clone();
    v_az = t4597[1].clone();
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
    let _ = f__need({ let t4665 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t4666 = v_x.clone(); if py::lt(&t4665, &t4666)? { { let t4667 = Py::Int(1i64); if py::lt(&t4666, &t4667)? { true } else { false } } } else { false } }) }, Py::str("the sun does not rise or does not set on this day at this latitude (polar day or night)"))?;
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
    let t4668 = (Py::tuple(vec![py::call(&v_to_h.clone(), &[py::neg(&v_ws.clone())?])?, py::call(&v_to_h.clone(), &[v_ws.clone()])?, py::call(&v_to_h.clone(), &[Py::Float(0.0)])?])).items()?;
    if t4668.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_rise = t4668[0].clone();
    v_sset = t4668[1].clone();
    v_transit = t4668[2].clone();
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
    let t4735 = (f__fault_network(v_network.clone())?).items()?;
    if t4735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_edges = t4735[0].clone();
    v_nb = t4735[1].clone();
    let _ = f__need({ let t4739 = Py::Bool(matches!(v_fault_bus.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4739.truthy() { t4739 } else { { let t4736 = Py::Int(1i64); Py::Bool({ let t4737 = v_fault_bus.clone(); if py::le(&t4736, &t4737)? { { let t4738 = v_nb.clone(); if py::le(&t4737, &t4738)? { true } else { false } } } else { false } }) } } }, Py::str(&["the faulted bus must be one of 1..".to_string(), py::format(&v_nb.clone(), "")?].concat()))?;
    let _ = f__need({ let t4740 = Py::Float(0.5); Py::Bool({ let t4741 = v_Vf.clone(); if py::le(&t4740, &t4741)? { { let t4742 = Py::Float(1.5); if py::le(&t4741, &t4742)? { true } else { false } } } else { false } }) }, Py::str("the prefault voltage must lie between 0.5 and 1.5 pu"))?;
    let _ = f__nonneg(Py::str("the fault reactance"), Py::tuple(vec![v_Xf.clone()]))?;
    v_Y = { let mut out = vec![]; for t4748 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v__: Py; 
v__ = t4748;
out.push(py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_nb.clone())?); } Py::list(out) };
    for t4749 in py::iter(&(v_edges.clone()))? {
        let t4759 = (t4749).items()?;
        if t4759.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_i = t4759[0].clone();
        v_j = t4759[1].clone();
        v_x = t4759[2].clone();
        v_y = py::truediv(&Py::Int(1i64), &py::complex(&[Py::Int(0i64), v_x.clone()])?)?;
        if (v_i.clone()).truthy() {
            let t4750 = py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t4751 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t4750, &t4751, py::iadd(&py::getitem(&t4750, &t4751)?, &v_y.clone())?)?;
        }
        if (v_j.clone()).truthy() {
            let t4752 = py::getitem(&v_Y.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t4753 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
            py::setitem(&t4752, &t4753, py::iadd(&py::getitem(&t4752, &t4753)?, &v_y.clone())?)?;
        }
        if ({ let t4754 = v_i.clone(); if !t4754.truthy() { t4754 } else { v_j.clone() } }).truthy() {
            let t4755 = py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t4756 = py::sub(&v_j.clone(), &Py::Int(1i64))?;
            py::setitem(&t4755, &t4756, py::sub(&py::getitem(&t4755, &t4756)?, &v_y.clone())?)?;
            let t4757 = py::getitem(&v_Y.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; let t4758 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t4757, &t4758, py::sub(&py::getitem(&t4757, &t4758)?, &v_y.clone())?)?;
        }
    }
    v_e = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_nb.clone())?;
    py::setitem(&v_e.clone(), &py::sub(&v_fault_bus.clone(), &Py::Int(1i64))?, py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?)?;
    v_z = f__gauss_solve(v_Y.clone(), v_e.clone())?;
    v_zff = py::getitem(&v_z.clone(), &py::sub(&v_fault_bus.clone(), &Py::Int(1i64))?)?;
    v_If = py::truediv(&v_Vf.clone(), &py::add(&v_zff.clone(), &py::complex(&[Py::Int(0i64), v_Xf.clone()])?)?)?;
    v_Vd = { let mut out = vec![]; for t4777 in py::range_iter(&[(v_nb.clone()).i()?]) { let mut v_k: Py; 
v_k = t4777;
out.push(py::abs(&py::sub(&v_Vf.clone(), &py::mul(&py::getitem(&v_z.clone(), &v_k.clone())?, &v_If.clone())?)?)?); } Py::list(out) };
    v_out = Py::dict(vec![(Py::str("If"), py::abs(&v_If.clone())?), (Py::str("X_th"), py::imag(&v_zff.clone())?), (Py::str("V"), v_Vd.clone())]);
    if ({ let t4782 = { let t4778 = v_S_base.clone(); Py::Bool({ let t4779 = Py::None; if !py::is(&t4778, &t4779) { true } else { false } }) }; if !t4782.truthy() { t4782 } else { { let t4780 = v_V_base.clone(); Py::Bool({ let t4781 = Py::None; if !py::is(&t4780, &t4781) { true } else { false } }) } } }).truthy() {
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
    return Ok(f__out(Py::dict(vec![(Py::str("E_line"), py::mul(&c_SQ3()?, &py::abs(&v_E.clone())?)?), (Py::str("E_phase"), py::abs(&v_E.clone())?), (Py::str("delta_deg"), py::math1("degrees", &py::cmath("phase", &[v_E.clone()])?)?), (Py::str("regulation_pct"), py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&py::abs(&v_E.clone())?, &v_V.clone())?)?, &v_V.clone())?), (Py::str("P"), py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_V.clone())?, &v_IL.clone())?, &v_pf.clone())?), (Py::str("Q"), (if ({ let t4862 = v_pf.clone(); Py::Bool({ let t4863 = Py::Int(1i64); if py::eq(&t4862, &t4863) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_V.clone())?, &v_IL.clone())?, &py::math1("sin", &v_phi.clone())?)? }))]))?);
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
    let t4986 = ((if ({ let t4984 = v_connection.clone(); Py::Bool({ let t4985 = Py::str("star"); if py::eq(&t4984, &t4985) { true } else { false } }) }).truthy() { Py::tuple(vec![py::truediv(&v_VL.clone(), &c_SQ3()?)?, v_IL.clone()]) } else { Py::tuple(vec![v_VL.clone(), py::truediv(&v_IL.clone(), &c_SQ3()?)?]) })).items()?;
    if t4986.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vph = t4986[0].clone();
    v_iph = t4986[1].clone();
    v_Q = py::mul(&v_S.clone(), &py::math1("sin", &v_phi.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::mul(&v_S.clone(), &v_pf.clone())?), (Py::str("Q"), (if ({ let t4999 = v_pf.clone(); Py::Bool({ let t5000 = Py::Int(1i64); if py::eq(&t4999, &t5000) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_Q.clone() })), (Py::str("S"), v_S.clone()), (Py::str("V_phase"), v_vph.clone()), (Py::str("I_phase"), v_iph.clone())]))?);
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
    let _ = f__need({ let t5039 = v_V_hv.clone(); Py::Bool({ let t5040 = v_V_lv.clone(); if py::gt(&t5039, &t5040)? { true } else { false } }) }, Py::str("the high-voltage side must have the higher rated voltage"))?;
    let _ = f__need({ let t5041 = Py::Int(0i64); Py::Bool({ let t5042 = v_vkr.clone(); if py::lt(&t5041, &t5042)? { { let t5043 = v_vk.clone(); if py::lt(&t5042, &t5043)? { { let t5044 = Py::Float(0.5); if py::lt(&t5043, &t5044)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("need 0 < vkr < vk (short-circuit voltages as fractions or percentages, vk below 50%)"))?;
    let _ = f__nonneg(Py::str("the no-load loss and current"), Py::tuple(vec![v_P0.clone(), v_i0.clone()]))?;
    let _ = f__need({ let t5057 = { let t5050 = py::truediv(&v_P0.clone(), &v_sn.clone())?; Py::Bool({ let t5051 = v_i0.clone(); if py::le(&t5050, &t5051)? { true } else { false } }) }; if t5057.truthy() { t5057 } else { { let t5056 = { let t5052 = v_i0.clone(); Py::Bool({ let t5053 = Py::Int(0i64); if py::eq(&t5052, &t5053) { true } else { false } }) }; if !t5056.truthy() { t5056 } else { { let t5054 = v_P0.clone(); Py::Bool({ let t5055 = Py::Int(0i64); if py::eq(&t5054, &t5055) { true } else { false } }) } } } } }, Py::str("the no-load current must cover the no-load loss (i0 >= P0/sn)"))?;
    let _ = f__nonneg(Py::str("the load real power"), Py::tuple(vec![v_P.clone()]))?;
    let t5058 = (Py::tuple(vec![v_vkr.clone(), v_vk.clone()])).items()?;
    if t5058.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rk = t5058[0].clone();
    v_zk = t5058[1].clone();
    v_Z = py::complex(&[v_rk.clone(), py::math1("sqrt", &py::sub(&py::mul(&v_zk.clone(), &v_zk.clone())?, &py::mul(&v_rk.clone(), &v_rk.clone())?)?)?])?;
    v_gm = py::truediv(&v_P0.clone(), &v_sn.clone())?;
    v_Ym = py::complex(&[v_gm.clone(), py::neg(&py::math1("sqrt", &py::minmax(&[py::sub(&py::mul(&v_i0.clone(), &v_i0.clone())?, &py::mul(&v_gm.clone(), &v_gm.clone())?)?, Py::Float(0.0)], true)?)?)?])?;
    let t5085 = (f__trafo_t_solve(v_Z.clone(), v_Ym.clone(), py::truediv(&py::complex(&[v_P.clone(), v_Q.clone()])?, &v_sn.clone())?)?).items()?;
    if t5085.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t5085[0].clone();
    v_i1 = t5085[1].clone();
    v_i2 = t5085[2].clone();
    v_S1 = py::mul(&py::mul(&Py::Int(1i64), &py::method(&v_i1.clone(), "conjugate", &[])?)?, &v_sn.clone())?;
    v_loss = py::sub(&v_S1.clone(), &py::complex(&[v_P.clone(), v_Q.clone()])?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_lv_actual"), py::mul(&py::abs(&v_v2.clone())?, &v_V_lv.clone())?), (Py::str("V_lv_pu"), py::abs(&v_v2.clone())?), (Py::str("delta_deg"), py::math1("degrees", &py::cmath("phase", &[v_v2.clone()])?)?), (Py::str("P_loss"), py::real(&v_loss.clone())?), (Py::str("Q_loss"), py::imag(&v_loss.clone())?), (Py::str("efficiency_pct"), (if ({ let t5098 = py::real(&v_S1.clone())?; Py::Bool({ let t5099 = Py::Int(0i64); if py::gt(&t5098, &t5099)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&Py::Int(100i64), &v_P.clone())?, &py::real(&v_S1.clone())?)? } else { Py::Float(0.0) })), (Py::str("loading_pct"), py::mul(&Py::Int(100i64), &py::minmax(&[py::abs(&v_i1.clone())?, py::abs(&v_i2.clone())?], true)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformer_max_efficiency(mut v_S: Py, mut v_P_core: Py, mut v_P_cu_fl: Py, mut v_pf: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the rating and the losses"), Py::tuple(vec![v_S.clone(), v_P_core.clone(), v_P_cu_fl.clone()]))?;
    let _ = f__need({ let t5138 = Py::Int(0i64); Py::Bool({ let t5139 = v_pf.clone(); if py::lt(&t5138, &t5139)? { { let t5140 = Py::Int(1i64); if py::le(&t5139, &t5140)? { true } else { false } } } else { false } }) }, Py::str("the power factor must lie in (0, 1]"))?;
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
    let _ = f__need({ let t5196 = Py::Int(0i64); Py::Bool({ let t5197 = v_load.clone(); if py::lt(&t5196, &t5197)? { { let t5198 = Py::Int(2i64); if py::le(&t5197, &t5198)? { true } else { false } } } else { false } }) }, Py::str("the load must lie between 0 and 2 times the rating (a fraction, or a percentage)"))?;
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
    let _ = f__need({ let t5255 = v_Poc.clone(); Py::Bool({ let t5256 = py::mul(&v_Voc.clone(), &v_Ioc.clone())?; if py::lt(&t5255, &t5256)? { true } else { false } }) }, Py::str("the open-circuit power must be below V I (power factor below 1)"))?;
    let _ = f__need({ let t5257 = v_Psc.clone(); Py::Bool({ let t5258 = py::mul(&v_Vsc.clone(), &v_Isc.clone())?; if py::lt(&t5257, &t5258)? { true } else { false } }) }, Py::str("the short-circuit power must be below V I (power factor below 1)"))?;
    let t5259 = (Py::tuple(vec![py::truediv(&v_Poc.clone(), &py::pow(&v_Voc.clone(), &Py::Int(2i64))?)?, py::truediv(&v_Ioc.clone(), &v_Voc.clone())?])).items()?;
    if t5259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_G = t5259[0].clone();
    v_Y = t5259[1].clone();
    v_Bm = py::math1("sqrt", &py::sub(&py::mul(&v_Y.clone(), &v_Y.clone())?, &py::mul(&v_G.clone(), &v_G.clone())?)?)?;
    let t5260 = (Py::tuple(vec![py::truediv(&v_Vsc.clone(), &v_Isc.clone())?, py::truediv(&v_Psc.clone(), &py::pow(&v_Isc.clone(), &Py::Int(2i64))?)?])).items()?;
    if t5260.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z = t5260[0].clone();
    v_R = t5260[1].clone();
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
    let _ = f__need({ let t5316 = py::add(&v_R.clone(), &v_X.clone())?; Py::Bool({ let t5317 = Py::Int(0i64); if py::gt(&t5316, &t5317)? { true } else { false } }) }, Py::str("the line impedance must not be zero"))?;
    let _ = f__nonneg(Py::str("the load real power"), Py::tuple(vec![v_P.clone()]))?;
    v_a = py::sub(&py::mul(&Py::Int(2i64), &py::add(&py::mul(&v_P.clone(), &v_R.clone())?, &py::mul(&v_Q.clone(), &v_X.clone())?)?)?, &py::mul(&v_Vs.clone(), &v_Vs.clone())?)?;
    v_disc = py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &py::add(&py::mul(&v_P.clone(), &v_P.clone())?, &py::mul(&v_Q.clone(), &v_Q.clone())?)?)?, &py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_X.clone(), &v_X.clone())?)?)?)?;
    let _ = f__need({ let t5318 = v_disc.clone(); Py::Bool({ let t5319 = Py::Int(0i64); if py::ge(&t5318, &t5319)? { true } else { false } }) }, Py::str("the load exceeds what the line can deliver: the load flow has no solution"))?;
    v_V2 = py::truediv(&py::add(&py::neg(&v_a.clone())?, &py::math1("sqrt", &v_disc.clone())?)?, &Py::Int(2i64))?;
    v_Vr = py::math1("sqrt", &v_V2.clone())?;
    let t5320 = (Py::tuple(vec![py::add(&v_Vr.clone(), &py::truediv(&py::add(&py::mul(&v_P.clone(), &v_R.clone())?, &py::mul(&v_Q.clone(), &v_X.clone())?)?, &v_Vr.clone())?)?, py::truediv(&py::sub(&py::mul(&v_P.clone(), &v_X.clone())?, &py::mul(&v_Q.clone(), &v_R.clone())?)?, &v_Vr.clone())?])).items()?;
    if t5320.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re = t5320[0].clone();
    v_im = t5320[1].clone();
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
    let t4245 = (Py::tuple(vec![py::complex(&[Py::Int(0i64), v_X1.clone()])?, py::complex(&[Py::Int(0i64), v_X2.clone()])?, py::complex(&[Py::Int(0i64), v_X0.clone()])?, py::complex(&[v_Rf.clone(), v_Xf.clone()])?])).items()?;
    if t4245.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Z1 = t4245[0].clone();
    v_Z2 = t4245[1].clone();
    v_Z0 = t4245[2].clone();
    v_Zf = t4245[3].clone();
    if ({ let t4246 = v_kind.clone(); Py::Bool({ let t4247 = Py::str("slg"); if py::eq(&t4246, &t4247) { true } else { false } }) }).truthy() {
        v_I1 = py::truediv(&v_Vf.clone(), &py::add(&py::add(&py::add(&v_Z1.clone(), &v_Z2.clone())?, &v_Z0.clone())?, &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?;
        let t4248 = (Py::tuple(vec![v_I1.clone(), v_I1.clone()])).items()?;
        if t4248.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_I2 = t4248[0].clone();
        v_I0 = t4248[1].clone();
    } else {
        if ({ let t4249 = v_kind.clone(); Py::Bool({ let t4250 = Py::str("ll"); if py::eq(&t4249, &t4250) { true } else { false } }) }).truthy() {
            v_I1 = py::truediv(&v_Vf.clone(), &py::add(&py::add(&v_Z1.clone(), &v_Z2.clone())?, &v_Zf.clone())?)?;
            let t4251 = (Py::tuple(vec![py::neg(&v_I1.clone())?, Py::Complex(0.0, 0.0)])).items()?;
            if t4251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_I2 = t4251[0].clone();
            v_I0 = t4251[1].clone();
        } else {
            v_Zp = py::truediv(&py::mul(&v_Z2.clone(), &py::add(&v_Z0.clone(), &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?, &py::add(&py::add(&v_Z2.clone(), &v_Z0.clone())?, &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?;
            v_I1 = py::truediv(&v_Vf.clone(), &py::add(&v_Z1.clone(), &v_Zp.clone())?)?;
            v_I2 = py::truediv(&py::mul(&py::neg(&v_I1.clone())?, &py::add(&v_Z0.clone(), &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?, &py::add(&py::add(&v_Z2.clone(), &v_Z0.clone())?, &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?;
            v_I0 = py::truediv(&py::mul(&py::neg(&v_I1.clone())?, &v_Z2.clone())?, &py::add(&py::add(&v_Z2.clone(), &v_Z0.clone())?, &py::mul(&Py::Int(3i64), &v_Zf.clone())?)?)?;
        }
    }
    let t4252 = (Py::tuple(vec![py::sub(&v_Vf.clone(), &py::mul(&v_Z1.clone(), &v_I1.clone())?)?, py::mul(&py::neg(&v_Z2.clone())?, &v_I2.clone())?, py::mul(&py::neg(&v_Z0.clone())?, &v_I0.clone())?])).items()?;
    if t4252.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V1 = t4252[0].clone();
    v_V2 = t4252[1].clone();
    v_V0 = t4252[2].clone();
    let t4253 = (f__phase_from_seq(v_I0.clone(), v_I1.clone(), v_I2.clone())?).items()?;
    if t4253.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Ia = t4253[0].clone();
    v_Ib = t4253[1].clone();
    v_Ic = t4253[2].clone();
    let t4254 = (f__phase_from_seq(v_V0.clone(), v_V1.clone(), v_V2.clone())?).items()?;
    if t4254.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Va = t4254[0].clone();
    v_Vb = t4254[1].clone();
    v_Vc = t4254[2].clone();
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
    for t5389 in py::range_iter(&[(py::sub(&py::len(&v_speeds.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t5389;
        let t5390 = (Py::tuple(vec![py::getitem(&v_speeds.clone(), &v_i.clone())?, py::getitem(&v_speeds.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])).items()?;
        if t5390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t5390[0].clone();
        v_b = t5390[1].clone();
        v_slope = py::truediv(&py::sub(&py::getitem(&v_powers.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_powers.clone(), &v_i.clone())?)?, &py::sub(&v_b.clone(), &v_a.clone())?)?;
        v_base = py::sub(&py::getitem(&v_powers.clone(), &v_i.clone())?, &py::mul(&v_slope.clone(), &v_a.clone())?)?;
        v_pa = py::sub(&py::call(&v_F.clone(), &[v_b.clone()])?, &py::call(&v_F.clone(), &[v_a.clone()])?)?;
        v_mv = py::mul(&py::mul(&v_c.clone(), &py::math1("gamma", &v_s.clone())?)?, &ax_engine::pymp::call("gammainc", &[v_s.clone(), py::pow(&py::truediv(&v_a.clone(), &v_c.clone())?, &v_k.clone())?, py::pow(&py::truediv(&v_b.clone(), &v_c.clone())?, &v_k.clone())?], &[(String::from("regularized"), Py::Bool(true))])?)?;
        v_total = py::iadd(&v_total, &py::add(&py::mul(&v_base.clone(), &v_pa.clone())?, &py::mul(&v_slope.clone(), &v_mv.clone())?)?)?;
    }
    v_pm = py::float(&v_total.clone())?;
    v_pr = py::minmax(&[v_powers.clone()], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("mean_power_kW"), v_pm.clone()), (Py::str("AEP_MWh"), py::truediv(&py::mul(&v_pm.clone(), &v_hours.clone())?, &Py::Float(1000.0))?), (Py::str("capacity_factor"), (if ({ let t5403 = v_pr.clone(); Py::Bool({ let t5404 = Py::Int(0i64); if py::gt(&t5403, &t5404)? { true } else { false } }) }).truthy() { py::truediv(&v_pm.clone(), &v_pr.clone())? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_wind(mut v_k: Py, mut v_c: Py, mut v_rho: Py) -> R {
    let mut v_g1: Py = Py::None;
    let mut v_g3: Py = Py::None;
    let _ = f__check_weibull(v_k.clone(), v_c.clone())?;
    let _ = f__pos(Py::str("the air density"), Py::tuple(vec![v_rho.clone()]))?;
    let t5447 = (Py::tuple(vec![py::math1("gamma", &py::add(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_k.clone())?)?)?, py::math1("gamma", &py::add(&Py::Int(1i64), &py::truediv(&Py::Int(3i64), &v_k.clone())?)?)?])).items()?;
    if t5447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g1 = t5447[0].clone();
    v_g3 = t5447[1].clone();
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
    if ({ let t5498 = v_cp.clone(); Py::Bool({ let t5499 = Py::None; if !py::is(&t5498, &t5499) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t5500 = Py::Int(0i64); Py::Bool({ let t5501 = v_cp.clone(); if py::lt(&t5500, &t5501)? { { let t5502 = py::add(&c_BETZ()?, &Py::Float(1e-12))?; if py::le(&t5501, &t5502)? { true } else { false } } } else { false } }) }, Py::str("the power coefficient must lie between 0 and the Betz limit 16/27 = 0.593"))?;
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
    let _ = f__need({ let t5563 = { let t5559 = py::sub(&v_h1.clone(), &py::mul(&Py::Float(0.7), &v_d.clone())?)?; Py::Bool({ let t5560 = v_z0.clone(); if py::gt(&t5559, &t5560)? { true } else { false } }) }; if !t5563.truthy() { t5563 } else { { let t5561 = py::sub(&v_h2.clone(), &py::mul(&Py::Float(0.7), &v_d.clone())?)?; Py::Bool({ let t5562 = v_z0.clone(); if py::gt(&t5561, &t5562)? { true } else { false } }) } } }, Py::str("both heights must exceed the displacement height 0.7 d plus the roughness length"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("v2"), py::truediv(&py::mul(&v_v1.clone(), &py::math1("log", &py::truediv(&py::sub(&v_h2.clone(), &py::mul(&Py::Float(0.7), &v_d.clone())?)?, &v_z0.clone())?)?)?, &py::math1("log", &py::truediv(&py::sub(&v_h1.clone(), &py::mul(&Py::Float(0.7), &v_d.clone())?)?, &v_z0.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_shear_power_law(mut v_v1: Py, mut v_h1: Py, mut v_h2: Py, mut v_alpha: Py) -> R {
    let _ = f__pos(Py::str("the wind speed and the heights"), Py::tuple(vec![v_v1.clone(), v_h1.clone(), v_h2.clone()]))?;
    let _ = f__need({ let t5614 = Py::Int(0i64); Py::Bool({ let t5615 = v_alpha.clone(); if py::le(&t5614, &t5615)? { { let t5616 = Py::Int(1i64); if py::le(&t5615, &t5616)? { true } else { false } } } else { false } }) }, Py::str("the shear exponent alpha must lie between 0 and 1"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("v2"), py::mul(&v_v1.clone(), &py::pow(&py::truediv(&v_h2.clone(), &v_h1.clone())?, &v_alpha.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1007(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1009: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1008 = v_inputs.clone(); s_969(py::getitem(&t1008, &Py::str("X1"))?, py::getitem(&t1008, &Py::str("X2"))?, py::getitem(&t1008, &Py::str("X0"))?, py::method(&t1008, "get", &[Py::str("Rf"), Py::Float(0.0)])?, py::method(&t1008, "get", &[Py::str("Xf"), Py::Float(0.0)])?, py::method(&t1008, "get", &[Py::str("Vf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1009 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1010 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1039 = (t1010).items()?;
        if t1039.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1039[0].clone();
        v_v = t1039[1].clone();
        if ({ let t1011 = v_k.clone(); Py::Bool({ let t1012 = v_outputs.clone(); if !py::contains(&t1012, &t1011)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1038 = { let t1029 = v_d.clone(); Py::Bool({ let t1030 = Py::Int(0i64); if py::gt(&t1029, &t1030)? { true } else { false } }) }; if !t1038.truthy() { t1038 } else { { let t1037 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1037.truthy() { t1037 } else { { let t1036 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1036.truthy() { t1036 } else { { let t1035 = { let t1031 = py::abs(&v_v.clone())?; Py::Bool({ let t1032 = Py::Float(1e-15); if py::lt(&t1031, &t1032)? { true } else { false } }) }; if !t1035.truthy() { t1035 } else { { let t1033 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1034 = Py::Float(1e-15); if py::lt(&t1033, &t1034)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1078(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1080: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1079 = v_inputs.clone(); f_droop_sharing(py::getitem(&t1079, &Py::str("R"))?, py::getitem(&t1079, &Py::str("ratings"))?, py::getitem(&t1079, &Py::str("dP"))?, py::getitem(&t1079, &Py::str("f0"))?, py::method(&t1079, "get", &[Py::str("D"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1080 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1081 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1110 = (t1081).items()?;
        if t1110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1110[0].clone();
        v_v = t1110[1].clone();
        if ({ let t1082 = v_k.clone(); Py::Bool({ let t1083 = v_outputs.clone(); if !py::contains(&t1083, &t1082)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1109 = { let t1100 = v_d.clone(); Py::Bool({ let t1101 = Py::Int(0i64); if py::gt(&t1100, &t1101)? { true } else { false } }) }; if !t1109.truthy() { t1109 } else { { let t1108 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1108.truthy() { t1108 } else { { let t1107 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1107.truthy() { t1107 } else { { let t1106 = { let t1102 = py::abs(&v_v.clone())?; Py::Bool({ let t1103 = Py::Float(1e-15); if py::lt(&t1102, &t1103)? { true } else { false } }) }; if !t1106.truthy() { t1106 } else { { let t1104 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1105 = Py::Float(1e-15); if py::lt(&t1104, &t1105)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1180(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1182: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1181 = v_inputs.clone(); f_economic_dispatch(py::getitem(&t1181, &Py::str("costs"))?, py::getitem(&t1181, &Py::str("demand"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1182 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1183 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1212 = (t1183).items()?;
        if t1212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1212[0].clone();
        v_v = t1212[1].clone();
        if ({ let t1184 = v_k.clone(); Py::Bool({ let t1185 = v_outputs.clone(); if !py::contains(&t1185, &t1184)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1211 = { let t1202 = v_d.clone(); Py::Bool({ let t1203 = Py::Int(0i64); if py::gt(&t1202, &t1203)? { true } else { false } }) }; if !t1211.truthy() { t1211 } else { { let t1210 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1210.truthy() { t1210 } else { { let t1209 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1209.truthy() { t1209 } else { { let t1208 = { let t1204 = py::abs(&v_v.clone())?; Py::Bool({ let t1205 = Py::Float(1e-15); if py::lt(&t1204, &t1205)? { true } else { false } }) }; if !t1208.truthy() { t1208 } else { { let t1206 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1207 = Py::Float(1e-15); if py::lt(&t1206, &t1207)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1239(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1241: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1240 = v_inputs.clone(); f_electrolyser(py::getitem(&t1240, &Py::str("I"))?, py::getitem(&t1240, &Py::str("N"))?, py::getitem(&t1240, &Py::str("V_cell"))?, py::method(&t1240, "get", &[Py::str("faradaic"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1241 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1242 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1271 = (t1242).items()?;
        if t1271.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1271[0].clone();
        v_v = t1271[1].clone();
        if ({ let t1243 = v_k.clone(); Py::Bool({ let t1244 = v_outputs.clone(); if !py::contains(&t1244, &t1243)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1270 = { let t1261 = v_d.clone(); Py::Bool({ let t1262 = Py::Int(0i64); if py::gt(&t1261, &t1262)? { true } else { false } }) }; if !t1270.truthy() { t1270 } else { { let t1269 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1269.truthy() { t1269 } else { { let t1268 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1268.truthy() { t1268 } else { { let t1267 = { let t1263 = py::abs(&v_v.clone())?; Py::Bool({ let t1264 = Py::Float(1e-15); if py::lt(&t1263, &t1264)? { true } else { false } }) }; if !t1267.truthy() { t1267 } else { { let t1265 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1266 = Py::Float(1e-15); if py::lt(&t1265, &t1266)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1272(mut v_n: Py, mut v_method: Py) -> R {
    return Ok(f_equation_of_time(v_n.clone(), v_method.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1291(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1293: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1292 = v_inputs.clone(); s_1272(py::getitem(&t1292, &Py::str("n"))?, py::method(&t1292, "get", &[Py::str("method"), Py::str("spencer")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1293 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1294 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1323 = (t1294).items()?;
        if t1323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1323[0].clone();
        v_v = t1323[1].clone();
        if ({ let t1295 = v_k.clone(); Py::Bool({ let t1296 = v_outputs.clone(); if !py::contains(&t1296, &t1295)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1322 = { let t1313 = v_d.clone(); Py::Bool({ let t1314 = Py::Int(0i64); if py::gt(&t1313, &t1314)? { true } else { false } }) }; if !t1322.truthy() { t1322 } else { { let t1321 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1321.truthy() { t1321 } else { { let t1320 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1320.truthy() { t1320 } else { { let t1319 = { let t1315 = py::abs(&v_v.clone())?; Py::Bool({ let t1316 = Py::Float(1e-15); if py::lt(&t1315, &t1316)? { true } else { false } }) }; if !t1319.truthy() { t1319 } else { { let t1317 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1318 = Py::Float(1e-15); if py::lt(&t1317, &t1318)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1353(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1355: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1354 = v_inputs.clone(); f_erbs(py::getitem(&t1354, &Py::str("ghi"))?, py::getitem(&t1354, &Py::str("zenith"))?, py::getitem(&t1354, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1355 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1356 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1385 = (t1356).items()?;
        if t1385.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1385[0].clone();
        v_v = t1385[1].clone();
        if ({ let t1357 = v_k.clone(); Py::Bool({ let t1358 = v_outputs.clone(); if !py::contains(&t1358, &t1357)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1384 = { let t1375 = v_d.clone(); Py::Bool({ let t1376 = Py::Int(0i64); if py::gt(&t1375, &t1376)? { true } else { false } }) }; if !t1384.truthy() { t1384 } else { { let t1383 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1383.truthy() { t1383 } else { { let t1382 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1382.truthy() { t1382 } else { { let t1381 = { let t1377 = py::abs(&v_v.clone())?; Py::Bool({ let t1378 = Py::Float(1e-15); if py::lt(&t1377, &t1378)? { true } else { false } }) }; if !t1381.truthy() { t1381 } else { { let t1379 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1380 = Py::Float(1e-15); if py::lt(&t1379, &t1380)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1409(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1411: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1410 = v_inputs.clone(); f_extraterrestrial_irradiance(py::getitem(&t1410, &Py::str("n"))?, py::method(&t1410, "get", &[Py::str("method"), Py::str("spencer")])?, py::method(&t1410, "get", &[Py::str("solar_constant"), c_SOLAR_CONSTANT()?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1411 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1412 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1441 = (t1412).items()?;
        if t1441.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1441[0].clone();
        v_v = t1441[1].clone();
        if ({ let t1413 = v_k.clone(); Py::Bool({ let t1414 = v_outputs.clone(); if !py::contains(&t1414, &t1413)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1440 = { let t1431 = v_d.clone(); Py::Bool({ let t1432 = Py::Int(0i64); if py::gt(&t1431, &t1432)? { true } else { false } }) }; if !t1440.truthy() { t1440 } else { { let t1439 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1439.truthy() { t1439 } else { { let t1438 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1438.truthy() { t1438 } else { { let t1437 = { let t1433 = py::abs(&v_v.clone())?; Py::Bool({ let t1434 = Py::Float(1e-15); if py::lt(&t1433, &t1434)? { true } else { false } }) }; if !t1437.truthy() { t1437 } else { { let t1435 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1436 = Py::Float(1e-15); if py::lt(&t1435, &t1436)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1467(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1469: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1468 = v_inputs.clone(); f_fuel_cell_stack(py::getitem(&t1468, &Py::str("I"))?, py::getitem(&t1468, &Py::str("N"))?, py::getitem(&t1468, &Py::str("V_cell"))?, py::method(&t1468, "get", &[Py::str("utilization"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1469 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1470 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1499 = (t1470).items()?;
        if t1499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1499[0].clone();
        v_v = t1499[1].clone();
        if ({ let t1471 = v_k.clone(); Py::Bool({ let t1472 = v_outputs.clone(); if !py::contains(&t1472, &t1471)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1498 = { let t1489 = v_d.clone(); Py::Bool({ let t1490 = Py::Int(0i64); if py::gt(&t1489, &t1490)? { true } else { false } }) }; if !t1498.truthy() { t1498 } else { { let t1497 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1497.truthy() { t1497 } else { { let t1496 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1496.truthy() { t1496 } else { { let t1495 = { let t1491 = py::abs(&v_v.clone())?; Py::Bool({ let t1492 = Py::Float(1e-15); if py::lt(&t1491, &t1492)? { true } else { false } }) }; if !t1495.truthy() { t1495 } else { { let t1493 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1494 = Py::Float(1e-15); if py::lt(&t1493, &t1494)? { true } else { false } }) } } } } } } } } }).truthy() {
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
fn s_1525(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1527: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1526 = v_inputs.clone(); f_gmd(py::getitem(&t1526, &Py::str("D_ab"))?, py::getitem(&t1526, &Py::str("D_bc"))?, py::getitem(&t1526, &Py::str("D_ca"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1527 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1528 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1557 = (t1528).items()?;
        if t1557.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1557[0].clone();
        v_v = t1557[1].clone();
        if ({ let t1529 = v_k.clone(); Py::Bool({ let t1530 = v_outputs.clone(); if !py::contains(&t1530, &t1529)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1556 = { let t1547 = v_d.clone(); Py::Bool({ let t1548 = Py::Int(0i64); if py::gt(&t1547, &t1548)? { true } else { false } }) }; if !t1556.truthy() { t1556 } else { { let t1555 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1555.truthy() { t1555 } else { { let t1554 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1554.truthy() { t1554 } else { { let t1553 = { let t1549 = py::abs(&v_v.clone())?; Py::Bool({ let t1550 = Py::Float(1e-15); if py::lt(&t1549, &t1550)? { true } else { false } }) }; if !t1553.truthy() { t1553 } else { { let t1551 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1552 = Py::Float(1e-15); if py::lt(&t1551, &t1552)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1586(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1588: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1587 = v_inputs.clone(); f_hub_air_density(py::getitem(&t1587, &Py::str("p"))?, py::getitem(&t1587, &Py::str("h_p"))?, py::getitem(&t1587, &Py::str("T"))?, py::getitem(&t1587, &Py::str("h_T"))?, py::getitem(&t1587, &Py::str("hub"))?, py::method(&t1587, "get", &[Py::str("model"), Py::str("barometric")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1588 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1589 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1618 = (t1589).items()?;
        if t1618.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1618[0].clone();
        v_v = t1618[1].clone();
        if ({ let t1590 = v_k.clone(); Py::Bool({ let t1591 = v_outputs.clone(); if !py::contains(&t1591, &t1590)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1617 = { let t1608 = v_d.clone(); Py::Bool({ let t1609 = Py::Int(0i64); if py::gt(&t1608, &t1609)? { true } else { false } }) }; if !t1617.truthy() { t1617 } else { { let t1616 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1616.truthy() { t1616 } else { { let t1615 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1615.truthy() { t1615 } else { { let t1614 = { let t1610 = py::abs(&v_v.clone())?; Py::Bool({ let t1611 = Py::Float(1e-15); if py::lt(&t1610, &t1611)? { true } else { false } }) }; if !t1614.truthy() { t1614 } else { { let t1612 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1613 = Py::Float(1e-15); if py::lt(&t1612, &t1613)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1687(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1689: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1688 = v_inputs.clone(); f_hydrogen_isentropic_compression(py::getitem(&t1688, &Py::str("p1"))?, py::getitem(&t1688, &Py::str("T1"))?, py::getitem(&t1688, &Py::str("p2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1689 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1690 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1719 = (t1690).items()?;
        if t1719.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1719[0].clone();
        v_v = t1719[1].clone();
        if ({ let t1691 = v_k.clone(); Py::Bool({ let t1692 = v_outputs.clone(); if !py::contains(&t1692, &t1691)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1718 = { let t1709 = v_d.clone(); Py::Bool({ let t1710 = Py::Int(0i64); if py::gt(&t1709, &t1710)? { true } else { false } }) }; if !t1718.truthy() { t1718 } else { { let t1717 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1717.truthy() { t1717 } else { { let t1716 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1716.truthy() { t1716 } else { { let t1715 = { let t1711 = py::abs(&v_v.clone())?; Py::Bool({ let t1712 = Py::Float(1e-15); if py::lt(&t1711, &t1712)? { true } else { false } }) }; if !t1715.truthy() { t1715 } else { { let t1713 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1714 = Py::Float(1e-15); if py::lt(&t1713, &t1714)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1780(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1782: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1781 = v_inputs.clone(); f_hydrogen_isothermal_compression(py::getitem(&t1781, &Py::str("p1"))?, py::getitem(&t1781, &Py::str("p2"))?, py::getitem(&t1781, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1782 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1783 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1812 = (t1783).items()?;
        if t1812.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1812[0].clone();
        v_v = t1812[1].clone();
        if ({ let t1784 = v_k.clone(); Py::Bool({ let t1785 = v_outputs.clone(); if !py::contains(&t1785, &t1784)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1811 = { let t1802 = v_d.clone(); Py::Bool({ let t1803 = Py::Int(0i64); if py::gt(&t1802, &t1803)? { true } else { false } }) }; if !t1811.truthy() { t1811 } else { { let t1810 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1810.truthy() { t1810 } else { { let t1809 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1809.truthy() { t1809 } else { { let t1808 = { let t1804 = py::abs(&v_v.clone())?; Py::Bool({ let t1805 = Py::Float(1e-15); if py::lt(&t1804, &t1805)? { true } else { false } }) }; if !t1808.truthy() { t1808 } else { { let t1806 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1807 = Py::Float(1e-15); if py::lt(&t1806, &t1807)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1874(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1876: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1875 = v_inputs.clone(); f_hydrogen_tank(py::getitem(&t1875, &Py::str("V"))?, py::getitem(&t1875, &Py::str("p"))?, py::getitem(&t1875, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1876 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1877 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1906 = (t1877).items()?;
        if t1906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1906[0].clone();
        v_v = t1906[1].clone();
        if ({ let t1878 = v_k.clone(); Py::Bool({ let t1879 = v_outputs.clone(); if !py::contains(&t1879, &t1878)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1905 = { let t1896 = v_d.clone(); Py::Bool({ let t1897 = Py::Int(0i64); if py::gt(&t1896, &t1897)? { true } else { false } }) }; if !t1905.truthy() { t1905 } else { { let t1904 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1904.truthy() { t1904 } else { { let t1903 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1903.truthy() { t1903 } else { { let t1902 = { let t1898 = py::abs(&v_v.clone())?; Py::Bool({ let t1899 = Py::Float(1e-15); if py::lt(&t1898, &t1899)? { true } else { false } }) }; if !t1902.truthy() { t1902 } else { { let t1900 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1901 = Py::Float(1e-15); if py::lt(&t1900, &t1901)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1928(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1930: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1929 = v_inputs.clone(); f_iam_ashrae(py::getitem(&t1929, &Py::str("aoi"))?, py::method(&t1929, "get", &[Py::str("b0"), Py::Float(0.05)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1930 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1931 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1960 = (t1931).items()?;
        if t1960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1960[0].clone();
        v_v = t1960[1].clone();
        if ({ let t1932 = v_k.clone(); Py::Bool({ let t1933 = v_outputs.clone(); if !py::contains(&t1933, &t1932)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t1959 = { let t1950 = v_d.clone(); Py::Bool({ let t1951 = Py::Int(0i64); if py::gt(&t1950, &t1951)? { true } else { false } }) }; if !t1959.truthy() { t1959 } else { { let t1958 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1958.truthy() { t1958 } else { { let t1957 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t1957.truthy() { t1957 } else { { let t1956 = { let t1952 = py::abs(&v_v.clone())?; Py::Bool({ let t1953 = Py::Float(1e-15); if py::lt(&t1952, &t1953)? { true } else { false } }) }; if !t1956.truthy() { t1956 } else { { let t1954 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t1955 = Py::Float(1e-15); if py::lt(&t1954, &t1955)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1985(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1987: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1986 = v_inputs.clone(); f_iam_physical(py::getitem(&t1986, &Py::str("aoi"))?, py::method(&t1986, "get", &[Py::str("n"), Py::Float(1.526)])?, py::method(&t1986, "get", &[Py::str("K"), Py::Float(4.0)])?, py::method(&t1986, "get", &[Py::str("L"), Py::Float(0.002)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1987 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1988 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2017 = (t1988).items()?;
        if t2017.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2017[0].clone();
        v_v = t2017[1].clone();
        if ({ let t1989 = v_k.clone(); Py::Bool({ let t1990 = v_outputs.clone(); if !py::contains(&t1990, &t1989)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2016 = { let t2007 = v_d.clone(); Py::Bool({ let t2008 = Py::Int(0i64); if py::gt(&t2007, &t2008)? { true } else { false } }) }; if !t2016.truthy() { t2016 } else { { let t2015 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2015.truthy() { t2015 } else { { let t2014 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2014.truthy() { t2014 } else { { let t2013 = { let t2009 = py::abs(&v_v.clone())?; Py::Bool({ let t2010 = Py::Float(1e-15); if py::lt(&t2009, &t2010)? { true } else { false } }) }; if !t2013.truthy() { t2013 } else { { let t2011 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2012 = Py::Float(1e-15); if py::lt(&t2011, &t2012)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2055(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2057: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2056 = v_inputs.clone(); f_induction_max_torque(py::getitem(&t2056, &Py::str("VL"))?, py::getitem(&t2056, &Py::str("f"))?, py::getitem(&t2056, &Py::str("poles"))?, py::getitem(&t2056, &Py::str("R1"))?, py::getitem(&t2056, &Py::str("X1"))?, py::getitem(&t2056, &Py::str("R2"))?, py::getitem(&t2056, &Py::str("X2"))?, py::getitem(&t2056, &Py::str("Xm"))?, py::getitem(&t2056, &Py::str("connection"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2057 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2058 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2087 = (t2058).items()?;
        if t2087.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2087[0].clone();
        v_v = t2087[1].clone();
        if ({ let t2059 = v_k.clone(); Py::Bool({ let t2060 = v_outputs.clone(); if !py::contains(&t2060, &t2059)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2086 = { let t2077 = v_d.clone(); Py::Bool({ let t2078 = Py::Int(0i64); if py::gt(&t2077, &t2078)? { true } else { false } }) }; if !t2086.truthy() { t2086 } else { { let t2085 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2085.truthy() { t2085 } else { { let t2084 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2084.truthy() { t2084 } else { { let t2083 = { let t2079 = py::abs(&v_v.clone())?; Py::Bool({ let t2080 = Py::Float(1e-15); if py::lt(&t2079, &t2080)? { true } else { false } }) }; if !t2083.truthy() { t2083 } else { { let t2081 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2082 = Py::Float(1e-15); if py::lt(&t2081, &t2082)? { true } else { false } }) } } } } } } } } }).truthy() {
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
fn s_2124(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2126: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2125 = v_inputs.clone(); f_induction_motor(py::getitem(&t2125, &Py::str("VL"))?, py::getitem(&t2125, &Py::str("f"))?, py::getitem(&t2125, &Py::str("poles"))?, py::getitem(&t2125, &Py::str("R1"))?, py::getitem(&t2125, &Py::str("X1"))?, py::getitem(&t2125, &Py::str("R2"))?, py::getitem(&t2125, &Py::str("X2"))?, py::getitem(&t2125, &Py::str("Xm"))?, py::getitem(&t2125, &Py::str("slip"))?, py::getitem(&t2125, &Py::str("connection"))?, py::method(&t2125, "get", &[Py::str("P_rot"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2126 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2127 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2156 = (t2127).items()?;
        if t2156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2156[0].clone();
        v_v = t2156[1].clone();
        if ({ let t2128 = v_k.clone(); Py::Bool({ let t2129 = v_outputs.clone(); if !py::contains(&t2129, &t2128)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2155 = { let t2146 = v_d.clone(); Py::Bool({ let t2147 = Py::Int(0i64); if py::gt(&t2146, &t2147)? { true } else { false } }) }; if !t2155.truthy() { t2155 } else { { let t2154 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2154.truthy() { t2154 } else { { let t2153 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2153.truthy() { t2153 } else { { let t2152 = { let t2148 = py::abs(&v_v.clone())?; Py::Bool({ let t2149 = Py::Float(1e-15); if py::lt(&t2148, &t2149)? { true } else { false } }) }; if !t2152.truthy() { t2152 } else { { let t2150 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2151 = Py::Float(1e-15); if py::lt(&t2150, &t2151)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2202(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2204: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2203 = v_inputs.clone(); f_line_parameters(py::getitem(&t2203, &Py::str("D_ab"))?, py::getitem(&t2203, &Py::str("D_bc"))?, py::getitem(&t2203, &Py::str("D_ca"))?, py::getitem(&t2203, &Py::str("r"))?, py::getitem(&t2203, &Py::str("f"))?, py::method(&t2203, "get", &[Py::str("gmr"), Py::None])?, py::method(&t2203, "get", &[Py::str("n_bundle"), Py::Int(1i64)])?, py::method(&t2203, "get", &[Py::str("spacing"), Py::None])?, py::method(&t2203, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2204 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2205 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2234 = (t2205).items()?;
        if t2234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2234[0].clone();
        v_v = t2234[1].clone();
        if ({ let t2206 = v_k.clone(); Py::Bool({ let t2207 = v_outputs.clone(); if !py::contains(&t2207, &t2206)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2233 = { let t2224 = v_d.clone(); Py::Bool({ let t2225 = Py::Int(0i64); if py::gt(&t2224, &t2225)? { true } else { false } }) }; if !t2233.truthy() { t2233 } else { { let t2232 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2232.truthy() { t2232 } else { { let t2231 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2231.truthy() { t2231 } else { { let t2230 = { let t2226 = py::abs(&v_v.clone())?; Py::Bool({ let t2227 = Py::Float(1e-15); if py::lt(&t2226, &t2227)? { true } else { false } }) }; if !t2230.truthy() { t2230 } else { { let t2228 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2229 = Py::Float(1e-15); if py::lt(&t2228, &t2229)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2235(mut v_X1: Py, mut v_X2: Py, mut v_X0: Py, mut v_Rf: Py, mut v_Xf: Py, mut v_Vf: Py) -> R {
    return Ok(f_unsym_fault(Py::str("ll"), v_X1.clone(), v_X2.clone(), v_X0.clone(), v_Rf.clone(), v_Xf.clone(), v_Vf.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2273(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2275: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2274 = v_inputs.clone(); s_2235(py::getitem(&t2274, &Py::str("X1"))?, py::getitem(&t2274, &Py::str("X2"))?, py::getitem(&t2274, &Py::str("X0"))?, py::method(&t2274, "get", &[Py::str("Rf"), Py::Float(0.0)])?, py::method(&t2274, "get", &[Py::str("Xf"), Py::Float(0.0)])?, py::method(&t2274, "get", &[Py::str("Vf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2275 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2276 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2305 = (t2276).items()?;
        if t2305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2305[0].clone();
        v_v = t2305[1].clone();
        if ({ let t2277 = v_k.clone(); Py::Bool({ let t2278 = v_outputs.clone(); if !py::contains(&t2278, &t2277)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2304 = { let t2295 = v_d.clone(); Py::Bool({ let t2296 = Py::Int(0i64); if py::gt(&t2295, &t2296)? { true } else { false } }) }; if !t2304.truthy() { t2304 } else { { let t2303 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2303.truthy() { t2303 } else { { let t2302 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2302.truthy() { t2302 } else { { let t2301 = { let t2297 = py::abs(&v_v.clone())?; Py::Bool({ let t2298 = Py::Float(1e-15); if py::lt(&t2297, &t2298)? { true } else { false } }) }; if !t2301.truthy() { t2301 } else { { let t2299 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2300 = Py::Float(1e-15); if py::lt(&t2299, &t2300)? { true } else { false } }) } } } } } } } } }).truthy() {
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
fn s_2346(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2348: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2347 = v_inputs.clone(); f_long_line(py::getitem(&t2347, &Py::str("Vr"))?, py::getitem(&t2347, &Py::str("P"))?, py::getitem(&t2347, &Py::str("pf"))?, py::getitem(&t2347, &Py::str("R"))?, py::getitem(&t2347, &Py::str("X"))?, py::getitem(&t2347, &Py::str("B"))?, py::method(&t2347, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2348 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2349 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2378 = (t2349).items()?;
        if t2378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2378[0].clone();
        v_v = t2378[1].clone();
        if ({ let t2350 = v_k.clone(); Py::Bool({ let t2351 = v_outputs.clone(); if !py::contains(&t2351, &t2350)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2377 = { let t2368 = v_d.clone(); Py::Bool({ let t2369 = Py::Int(0i64); if py::gt(&t2368, &t2369)? { true } else { false } }) }; if !t2377.truthy() { t2377 } else { { let t2376 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2376.truthy() { t2376 } else { { let t2375 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2375.truthy() { t2375 } else { { let t2374 = { let t2370 = py::abs(&v_v.clone())?; Py::Bool({ let t2371 = Py::Float(1e-15); if py::lt(&t2370, &t2371)? { true } else { false } }) }; if !t2374.truthy() { t2374 } else { { let t2372 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2373 = Py::Float(1e-15); if py::lt(&t2372, &t2373)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2413(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2414 = v_inputs.clone(); f_medium_line(py::getitem(&t2414, &Py::str("Vr"))?, py::getitem(&t2414, &Py::str("P"))?, py::getitem(&t2414, &Py::str("pf"))?, py::getitem(&t2414, &Py::str("R"))?, py::getitem(&t2414, &Py::str("X"))?, py::getitem(&t2414, &Py::str("B"))?, py::method(&t2414, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2415 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2416 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2445 = (t2416).items()?;
        if t2445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2445[0].clone();
        v_v = t2445[1].clone();
        if ({ let t2417 = v_k.clone(); Py::Bool({ let t2418 = v_outputs.clone(); if !py::contains(&t2418, &t2417)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2444 = { let t2435 = v_d.clone(); Py::Bool({ let t2436 = Py::Int(0i64); if py::gt(&t2435, &t2436)? { true } else { false } }) }; if !t2444.truthy() { t2444 } else { { let t2443 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2443.truthy() { t2443 } else { { let t2442 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2442.truthy() { t2442 } else { { let t2441 = { let t2437 = py::abs(&v_v.clone())?; Py::Bool({ let t2438 = Py::Float(1e-15); if py::lt(&t2437, &t2438)? { true } else { false } }) }; if !t2441.truthy() { t2441 } else { { let t2439 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2440 = Py::Float(1e-15); if py::lt(&t2439, &t2440)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2619(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2621: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2620 = v_inputs.clone(); f_newton_load_flow(py::getitem(&t2620, &Py::str("lines"))?, py::getitem(&t2620, &Py::str("loads"))?, py::method(&t2620, "get", &[Py::str("gens"), Py::None])?, py::method(&t2620, "get", &[Py::str("V1"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2621 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2622 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2651 = (t2622).items()?;
        if t2651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2651[0].clone();
        v_v = t2651[1].clone();
        if ({ let t2623 = v_k.clone(); Py::Bool({ let t2624 = v_outputs.clone(); if !py::contains(&t2624, &t2623)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2650 = { let t2641 = v_d.clone(); Py::Bool({ let t2642 = Py::Int(0i64); if py::gt(&t2641, &t2642)? { true } else { false } }) }; if !t2650.truthy() { t2650 } else { { let t2649 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2649.truthy() { t2649 } else { { let t2648 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2648.truthy() { t2648 } else { { let t2647 = { let t2643 = py::abs(&v_v.clone())?; Py::Bool({ let t2644 = Py::Float(1e-15); if py::lt(&t2643, &t2644)? { true } else { false } }) }; if !t2647.truthy() { t2647 } else { { let t2645 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2646 = Py::Float(1e-15); if py::lt(&t2645, &t2646)? { true } else { false } }) } } } } } } } } }).truthy() {
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
fn s_2677(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2679: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2678 = v_inputs.clone(); f_parallel_transformers(py::getitem(&t2678, &Py::str("S1"))?, py::getitem(&t2678, &Py::str("vk1"))?, py::getitem(&t2678, &Py::str("vkr1"))?, py::getitem(&t2678, &Py::str("S2"))?, py::getitem(&t2678, &Py::str("vk2"))?, py::getitem(&t2678, &Py::str("vkr2"))?, py::getitem(&t2678, &Py::str("V_lv"))?, py::getitem(&t2678, &Py::str("P"))?, py::getitem(&t2678, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2679 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2680 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2709 = (t2680).items()?;
        if t2709.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2709[0].clone();
        v_v = t2709[1].clone();
        if ({ let t2681 = v_k.clone(); Py::Bool({ let t2682 = v_outputs.clone(); if !py::contains(&t2682, &t2681)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2708 = { let t2699 = v_d.clone(); Py::Bool({ let t2700 = Py::Int(0i64); if py::gt(&t2699, &t2700)? { true } else { false } }) }; if !t2708.truthy() { t2708 } else { { let t2707 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2707.truthy() { t2707 } else { { let t2706 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2706.truthy() { t2706 } else { { let t2705 = { let t2701 = py::abs(&v_v.clone())?; Py::Bool({ let t2702 = Py::Float(1e-15); if py::lt(&t2701, &t2702)? { true } else { false } }) }; if !t2705.truthy() { t2705 } else { { let t2703 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2704 = Py::Float(1e-15); if py::lt(&t2703, &t2704)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2732(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2734: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2733 = v_inputs.clone(); f_per_unit_change_of_base(py::getitem(&t2733, &Py::str("R_pu"))?, py::getitem(&t2733, &Py::str("X_pu"))?, py::getitem(&t2733, &Py::str("S_old"))?, py::getitem(&t2733, &Py::str("V_old"))?, py::getitem(&t2733, &Py::str("S_new"))?, py::getitem(&t2733, &Py::str("V_new"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2734 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2735 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2764 = (t2735).items()?;
        if t2764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2764[0].clone();
        v_v = t2764[1].clone();
        if ({ let t2736 = v_k.clone(); Py::Bool({ let t2737 = v_outputs.clone(); if !py::contains(&t2737, &t2736)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2763 = { let t2754 = v_d.clone(); Py::Bool({ let t2755 = Py::Int(0i64); if py::gt(&t2754, &t2755)? { true } else { false } }) }; if !t2763.truthy() { t2763 } else { { let t2762 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2762.truthy() { t2762 } else { { let t2761 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2761.truthy() { t2761 } else { { let t2760 = { let t2756 = py::abs(&v_v.clone())?; Py::Bool({ let t2757 = Py::Float(1e-15); if py::lt(&t2756, &t2757)? { true } else { false } }) }; if !t2760.truthy() { t2760 } else { { let t2758 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2759 = Py::Float(1e-15); if py::lt(&t2758, &t2759)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2792(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2794: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2793 = v_inputs.clone(); f_per_unit_impedance(py::getitem(&t2793, &Py::str("R"))?, py::getitem(&t2793, &Py::str("X"))?, py::getitem(&t2793, &Py::str("S_base"))?, py::getitem(&t2793, &Py::str("V_base"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2794 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2795 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2824 = (t2795).items()?;
        if t2824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2824[0].clone();
        v_v = t2824[1].clone();
        if ({ let t2796 = v_k.clone(); Py::Bool({ let t2797 = v_outputs.clone(); if !py::contains(&t2797, &t2796)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2823 = { let t2814 = v_d.clone(); Py::Bool({ let t2815 = Py::Int(0i64); if py::gt(&t2814, &t2815)? { true } else { false } }) }; if !t2823.truthy() { t2823 } else { { let t2822 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2822.truthy() { t2822 } else { { let t2821 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2821.truthy() { t2821 } else { { let t2820 = { let t2816 = py::abs(&v_v.clone())?; Py::Bool({ let t2817 = Py::Float(1e-15); if py::lt(&t2816, &t2817)? { true } else { false } }) }; if !t2820.truthy() { t2820 } else { { let t2818 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2819 = Py::Float(1e-15); if py::lt(&t2818, &t2819)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2865(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2867: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2866 = v_inputs.clone(); f_peukert(py::getitem(&t2866, &Py::str("currents"))?, py::getitem(&t2866, &Py::str("times"))?, py::getitem(&t2866, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2867 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2868 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2897 = (t2868).items()?;
        if t2897.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2897[0].clone();
        v_v = t2897[1].clone();
        if ({ let t2869 = v_k.clone(); Py::Bool({ let t2870 = v_outputs.clone(); if !py::contains(&t2870, &t2869)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2896 = { let t2887 = v_d.clone(); Py::Bool({ let t2888 = Py::Int(0i64); if py::gt(&t2887, &t2888)? { true } else { false } }) }; if !t2896.truthy() { t2896 } else { { let t2895 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2895.truthy() { t2895 } else { { let t2894 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2894.truthy() { t2894 } else { { let t2893 = { let t2889 = py::abs(&v_v.clone())?; Py::Bool({ let t2890 = Py::Float(1e-15); if py::lt(&t2889, &t2890)? { true } else { false } }) }; if !t2893.truthy() { t2893 } else { { let t2891 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2892 = Py::Float(1e-15); if py::lt(&t2891, &t2892)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2898(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py, mut v_n: Py) -> R {
    return Ok(f__poa(Py::str("haydavies"), v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone(), v_dni.clone(), v_ghi.clone(), v_dhi.clone(), v_albedo.clone(), v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2950(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2952: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2951 = v_inputs.clone(); s_2898(py::getitem(&t2951, &Py::str("tilt"))?, py::getitem(&t2951, &Py::str("surface_azimuth"))?, py::getitem(&t2951, &Py::str("zenith"))?, py::getitem(&t2951, &Py::str("azimuth"))?, py::getitem(&t2951, &Py::str("dni"))?, py::getitem(&t2951, &Py::str("ghi"))?, py::getitem(&t2951, &Py::str("dhi"))?, py::getitem(&t2951, &Py::str("albedo"))?, py::getitem(&t2951, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2952 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2953 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2982 = (t2953).items()?;
        if t2982.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2982[0].clone();
        v_v = t2982[1].clone();
        if ({ let t2954 = v_k.clone(); Py::Bool({ let t2955 = v_outputs.clone(); if !py::contains(&t2955, &t2954)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t2981 = { let t2972 = v_d.clone(); Py::Bool({ let t2973 = Py::Int(0i64); if py::gt(&t2972, &t2973)? { true } else { false } }) }; if !t2981.truthy() { t2981 } else { { let t2980 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2980.truthy() { t2980 } else { { let t2979 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2979.truthy() { t2979 } else { { let t2978 = { let t2974 = py::abs(&v_v.clone())?; Py::Bool({ let t2975 = Py::Float(1e-15); if py::lt(&t2974, &t2975)? { true } else { false } }) }; if !t2978.truthy() { t2978 } else { { let t2976 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t2977 = Py::Float(1e-15); if py::lt(&t2976, &t2977)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2983(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py) -> R {
    return Ok(f__poa(Py::str("isotropic"), v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone(), v_dni.clone(), v_ghi.clone(), v_dhi.clone(), v_albedo.clone(), Py::None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3035(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3037: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3036 = v_inputs.clone(); s_2983(py::getitem(&t3036, &Py::str("tilt"))?, py::getitem(&t3036, &Py::str("surface_azimuth"))?, py::getitem(&t3036, &Py::str("zenith"))?, py::getitem(&t3036, &Py::str("azimuth"))?, py::getitem(&t3036, &Py::str("dni"))?, py::getitem(&t3036, &Py::str("ghi"))?, py::getitem(&t3036, &Py::str("dhi"))?, py::getitem(&t3036, &Py::str("albedo"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3037 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3038 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3067 = (t3038).items()?;
        if t3067.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3067[0].clone();
        v_v = t3067[1].clone();
        if ({ let t3039 = v_k.clone(); Py::Bool({ let t3040 = v_outputs.clone(); if !py::contains(&t3040, &t3039)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3066 = { let t3057 = v_d.clone(); Py::Bool({ let t3058 = Py::Int(0i64); if py::gt(&t3057, &t3058)? { true } else { false } }) }; if !t3066.truthy() { t3066 } else { { let t3065 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3065.truthy() { t3065 } else { { let t3064 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3064.truthy() { t3064 } else { { let t3063 = { let t3059 = py::abs(&v_v.clone())?; Py::Bool({ let t3060 = Py::Float(1e-15); if py::lt(&t3059, &t3060)? { true } else { false } }) }; if !t3063.truthy() { t3063 } else { { let t3061 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3062 = Py::Float(1e-15); if py::lt(&t3061, &t3062)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3068(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py) -> R {
    return Ok(f__poa(Py::str("klucher"), v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone(), v_dni.clone(), v_ghi.clone(), v_dhi.clone(), v_albedo.clone(), Py::None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3120(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3122: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3121 = v_inputs.clone(); s_3068(py::getitem(&t3121, &Py::str("tilt"))?, py::getitem(&t3121, &Py::str("surface_azimuth"))?, py::getitem(&t3121, &Py::str("zenith"))?, py::getitem(&t3121, &Py::str("azimuth"))?, py::getitem(&t3121, &Py::str("dni"))?, py::getitem(&t3121, &Py::str("ghi"))?, py::getitem(&t3121, &Py::str("dhi"))?, py::getitem(&t3121, &Py::str("albedo"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3122 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3123 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3152 = (t3123).items()?;
        if t3152.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3152[0].clone();
        v_v = t3152[1].clone();
        if ({ let t3124 = v_k.clone(); Py::Bool({ let t3125 = v_outputs.clone(); if !py::contains(&t3125, &t3124)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3151 = { let t3142 = v_d.clone(); Py::Bool({ let t3143 = Py::Int(0i64); if py::gt(&t3142, &t3143)? { true } else { false } }) }; if !t3151.truthy() { t3151 } else { { let t3150 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3150.truthy() { t3150 } else { { let t3149 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3149.truthy() { t3149 } else { { let t3148 = { let t3144 = py::abs(&v_v.clone())?; Py::Bool({ let t3145 = Py::Float(1e-15); if py::lt(&t3144, &t3145)? { true } else { false } }) }; if !t3148.truthy() { t3148 } else { { let t3146 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3147 = Py::Float(1e-15); if py::lt(&t3146, &t3147)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3214(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3216: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3215 = v_inputs.clone(); f_poa_perez(py::getitem(&t3215, &Py::str("tilt"))?, py::getitem(&t3215, &Py::str("surface_azimuth"))?, py::getitem(&t3215, &Py::str("zenith"))?, py::getitem(&t3215, &Py::str("azimuth"))?, py::getitem(&t3215, &Py::str("dni"))?, py::getitem(&t3215, &Py::str("ghi"))?, py::getitem(&t3215, &Py::str("dhi"))?, py::getitem(&t3215, &Py::str("albedo"))?, py::getitem(&t3215, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3216 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3217 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3246 = (t3217).items()?;
        if t3246.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3246[0].clone();
        v_v = t3246[1].clone();
        if ({ let t3218 = v_k.clone(); Py::Bool({ let t3219 = v_outputs.clone(); if !py::contains(&t3219, &t3218)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3245 = { let t3236 = v_d.clone(); Py::Bool({ let t3237 = Py::Int(0i64); if py::gt(&t3236, &t3237)? { true } else { false } }) }; if !t3245.truthy() { t3245 } else { { let t3244 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3244.truthy() { t3244 } else { { let t3243 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3243.truthy() { t3243 } else { { let t3242 = { let t3238 = py::abs(&v_v.clone())?; Py::Bool({ let t3239 = Py::Float(1e-15); if py::lt(&t3238, &t3239)? { true } else { false } }) }; if !t3242.truthy() { t3242 } else { { let t3240 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3241 = Py::Float(1e-15); if py::lt(&t3240, &t3241)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_323(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t325: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t324 = v_inputs.clone(); f_cell_temperature_faiman(py::getitem(&t324, &Py::str("poa"))?, py::getitem(&t324, &Py::str("T_air"))?, py::getitem(&t324, &Py::str("wind"))?, py::method(&t324, "get", &[Py::str("u0"), Py::Float(25.0)])?, py::method(&t324, "get", &[Py::str("u1"), Py::Float(6.84)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t325 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t326 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t355 = (t326).items()?;
        if t355.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t355[0].clone();
        v_v = t355[1].clone();
        if ({ let t327 = v_k.clone(); Py::Bool({ let t328 = v_outputs.clone(); if !py::contains(&t328, &t327)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t354 = { let t345 = v_d.clone(); Py::Bool({ let t346 = Py::Int(0i64); if py::gt(&t345, &t346)? { true } else { false } }) }; if !t354.truthy() { t354 } else { { let t353 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t353.truthy() { t353 } else { { let t352 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t352.truthy() { t352 } else { { let t351 = { let t347 = py::abs(&v_v.clone())?; Py::Bool({ let t348 = Py::Float(1e-15); if py::lt(&t347, &t348)? { true } else { false } }) }; if !t351.truthy() { t351 } else { { let t349 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t350 = Py::Float(1e-15); if py::lt(&t349, &t350)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3247(mut v_tilt: Py, mut v_surface_azimuth: Py, mut v_zenith: Py, mut v_azimuth: Py, mut v_dni: Py, mut v_ghi: Py, mut v_dhi: Py, mut v_albedo: Py, mut v_n: Py) -> R {
    return Ok(f__poa(Py::str("reindl"), v_tilt.clone(), v_surface_azimuth.clone(), v_zenith.clone(), v_azimuth.clone(), v_dni.clone(), v_ghi.clone(), v_dhi.clone(), v_albedo.clone(), v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3299(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3301: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3300 = v_inputs.clone(); s_3247(py::getitem(&t3300, &Py::str("tilt"))?, py::getitem(&t3300, &Py::str("surface_azimuth"))?, py::getitem(&t3300, &Py::str("zenith"))?, py::getitem(&t3300, &Py::str("azimuth"))?, py::getitem(&t3300, &Py::str("dni"))?, py::getitem(&t3300, &Py::str("ghi"))?, py::getitem(&t3300, &Py::str("dhi"))?, py::getitem(&t3300, &Py::str("albedo"))?, py::getitem(&t3300, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3301 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3302 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3331 = (t3302).items()?;
        if t3331.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3331[0].clone();
        v_v = t3331[1].clone();
        if ({ let t3303 = v_k.clone(); Py::Bool({ let t3304 = v_outputs.clone(); if !py::contains(&t3304, &t3303)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3330 = { let t3321 = v_d.clone(); Py::Bool({ let t3322 = Py::Int(0i64); if py::gt(&t3321, &t3322)? { true } else { false } }) }; if !t3330.truthy() { t3330 } else { { let t3329 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3329.truthy() { t3329 } else { { let t3328 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3328.truthy() { t3328 } else { { let t3327 = { let t3323 = py::abs(&v_v.clone())?; Py::Bool({ let t3324 = Py::Float(1e-15); if py::lt(&t3323, &t3324)? { true } else { false } }) }; if !t3327.truthy() { t3327 } else { { let t3325 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3326 = Py::Float(1e-15); if py::lt(&t3325, &t3326)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3351(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3353: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3352 = v_inputs.clone(); f_power_angle(py::getitem(&t3352, &Py::str("V1"))?, py::getitem(&t3352, &Py::str("V2"))?, py::getitem(&t3352, &Py::str("X"))?, py::getitem(&t3352, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3353 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3354 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3383 = (t3354).items()?;
        if t3383.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3383[0].clone();
        v_v = t3383[1].clone();
        if ({ let t3355 = v_k.clone(); Py::Bool({ let t3356 = v_outputs.clone(); if !py::contains(&t3356, &t3355)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3382 = { let t3373 = v_d.clone(); Py::Bool({ let t3374 = Py::Int(0i64); if py::gt(&t3373, &t3374)? { true } else { false } }) }; if !t3382.truthy() { t3382 } else { { let t3381 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3381.truthy() { t3381 } else { { let t3380 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3380.truthy() { t3380 } else { { let t3379 = { let t3375 = py::abs(&v_v.clone())?; Py::Bool({ let t3376 = Py::Float(1e-15); if py::lt(&t3375, &t3376)? { true } else { false } }) }; if !t3379.truthy() { t3379 } else { { let t3377 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3378 = Py::Float(1e-15); if py::lt(&t3377, &t3378)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3438(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3440: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3439 = v_inputs.clone(); f_power_curve_density_corrected(py::getitem(&t3439, &Py::str("speeds"))?, py::getitem(&t3439, &Py::str("powers"))?, py::getitem(&t3439, &Py::str("v"))?, py::getitem(&t3439, &Py::str("rho"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3440 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3441 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3470 = (t3441).items()?;
        if t3470.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3470[0].clone();
        v_v = t3470[1].clone();
        if ({ let t3442 = v_k.clone(); Py::Bool({ let t3443 = v_outputs.clone(); if !py::contains(&t3443, &t3442)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3469 = { let t3460 = v_d.clone(); Py::Bool({ let t3461 = Py::Int(0i64); if py::gt(&t3460, &t3461)? { true } else { false } }) }; if !t3469.truthy() { t3469 } else { { let t3468 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3468.truthy() { t3468 } else { { let t3467 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3467.truthy() { t3467 } else { { let t3466 = { let t3462 = py::abs(&v_v.clone())?; Py::Bool({ let t3463 = Py::Float(1e-15); if py::lt(&t3462, &t3463)? { true } else { false } }) }; if !t3466.truthy() { t3466 } else { { let t3464 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3465 = Py::Float(1e-15); if py::lt(&t3464, &t3465)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3513(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3515: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3514 = v_inputs.clone(); f_power_curve_output(py::getitem(&t3514, &Py::str("speeds"))?, py::getitem(&t3514, &Py::str("powers"))?, py::getitem(&t3514, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3515 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3516 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3545 = (t3516).items()?;
        if t3545.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3545[0].clone();
        v_v = t3545[1].clone();
        if ({ let t3517 = v_k.clone(); Py::Bool({ let t3518 = v_outputs.clone(); if !py::contains(&t3518, &t3517)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3544 = { let t3535 = v_d.clone(); Py::Bool({ let t3536 = Py::Int(0i64); if py::gt(&t3535, &t3536)? { true } else { false } }) }; if !t3544.truthy() { t3544 } else { { let t3543 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3543.truthy() { t3543 } else { { let t3542 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3542.truthy() { t3542 } else { { let t3541 = { let t3537 = py::abs(&v_v.clone())?; Py::Bool({ let t3538 = Py::Float(1e-15); if py::lt(&t3537, &t3538)? { true } else { false } }) }; if !t3541.truthy() { t3541 } else { { let t3539 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3540 = Py::Float(1e-15); if py::lt(&t3539, &t3540)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3577(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3579: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3578 = v_inputs.clone(); f__pv_array_check(py::getitem(&t3578, &Py::str("poa"))?, py::getitem(&t3578, &Py::str("T_air"))?, py::getitem(&t3578, &Py::str("noct"))?, py::getitem(&t3578, &Py::str("pdc0"))?, py::getitem(&t3578, &Py::str("gamma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3579 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3580 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3609 = (t3580).items()?;
        if t3609.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3609[0].clone();
        v_v = t3609[1].clone();
        if ({ let t3581 = v_k.clone(); Py::Bool({ let t3582 = v_outputs.clone(); if !py::contains(&t3582, &t3581)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3608 = { let t3599 = v_d.clone(); Py::Bool({ let t3600 = Py::Int(0i64); if py::gt(&t3599, &t3600)? { true } else { false } }) }; if !t3608.truthy() { t3608 } else { { let t3607 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3607.truthy() { t3607 } else { { let t3606 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3606.truthy() { t3606 } else { { let t3605 = { let t3601 = py::abs(&v_v.clone())?; Py::Bool({ let t3602 = Py::Float(1e-15); if py::lt(&t3601, &t3602)? { true } else { false } }) }; if !t3605.truthy() { t3605 } else { { let t3603 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3604 = Py::Float(1e-15); if py::lt(&t3603, &t3604)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3683(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3685: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3684 = v_inputs.clone(); f_pv_module_at_conditions(py::getitem(&t3684, &Py::str("G"))?, py::getitem(&t3684, &Py::str("T_cell"))?, py::getitem(&t3684, &Py::str("alpha_sc"))?, py::getitem(&t3684, &Py::str("a_ref"))?, py::getitem(&t3684, &Py::str("IL_ref"))?, py::getitem(&t3684, &Py::str("I0_ref"))?, py::getitem(&t3684, &Py::str("Rsh_ref"))?, py::getitem(&t3684, &Py::str("Rs"))?, py::method(&t3684, "get", &[Py::str("EgRef"), Py::Float(1.121)])?, py::method(&t3684, "get", &[Py::str("dEgdT"), py::neg(&Py::Float(0.0002677))?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3685 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3686 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3715 = (t3686).items()?;
        if t3715.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3715[0].clone();
        v_v = t3715[1].clone();
        if ({ let t3687 = v_k.clone(); Py::Bool({ let t3688 = v_outputs.clone(); if !py::contains(&t3688, &t3687)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3714 = { let t3705 = v_d.clone(); Py::Bool({ let t3706 = Py::Int(0i64); if py::gt(&t3705, &t3706)? { true } else { false } }) }; if !t3714.truthy() { t3714 } else { { let t3713 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3713.truthy() { t3713 } else { { let t3712 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3712.truthy() { t3712 } else { { let t3711 = { let t3707 = py::abs(&v_v.clone())?; Py::Bool({ let t3708 = Py::Float(1e-15); if py::lt(&t3707, &t3708)? { true } else { false } }) }; if !t3711.truthy() { t3711 } else { { let t3709 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3710 = Py::Float(1e-15); if py::lt(&t3709, &t3710)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3745(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3747: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3746 = v_inputs.clone(); f_pvwatts_dc(py::getitem(&t3746, &Py::str("G_eff"))?, py::getitem(&t3746, &Py::str("T_cell"))?, py::getitem(&t3746, &Py::str("pdc0"))?, py::getitem(&t3746, &Py::str("gamma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3747 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3748 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3777 = (t3748).items()?;
        if t3777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3777[0].clone();
        v_v = t3777[1].clone();
        if ({ let t3749 = v_k.clone(); Py::Bool({ let t3750 = v_outputs.clone(); if !py::contains(&t3750, &t3749)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3776 = { let t3767 = v_d.clone(); Py::Bool({ let t3768 = Py::Int(0i64); if py::gt(&t3767, &t3768)? { true } else { false } }) }; if !t3776.truthy() { t3776 } else { { let t3775 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3775.truthy() { t3775 } else { { let t3774 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3774.truthy() { t3774 } else { { let t3773 = { let t3769 = py::abs(&v_v.clone())?; Py::Bool({ let t3770 = Py::Float(1e-15); if py::lt(&t3769, &t3770)? { true } else { false } }) }; if !t3773.truthy() { t3773 } else { { let t3771 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3772 = Py::Float(1e-15); if py::lt(&t3771, &t3772)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_377(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t379: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t378 = v_inputs.clone(); f_cell_temperature_noct(py::getitem(&t378, &Py::str("poa"))?, py::getitem(&t378, &Py::str("T_air"))?, py::getitem(&t378, &Py::str("noct"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t379 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t380 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t409 = (t380).items()?;
        if t409.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t409[0].clone();
        v_v = t409[1].clone();
        if ({ let t381 = v_k.clone(); Py::Bool({ let t382 = v_outputs.clone(); if !py::contains(&t382, &t381)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t408 = { let t399 = v_d.clone(); Py::Bool({ let t400 = Py::Int(0i64); if py::gt(&t399, &t400)? { true } else { false } }) }; if !t408.truthy() { t408 } else { { let t407 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t407.truthy() { t407 } else { { let t406 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t406.truthy() { t406 } else { { let t405 = { let t401 = py::abs(&v_v.clone())?; Py::Bool({ let t402 = Py::Float(1e-15); if py::lt(&t401, &t402)? { true } else { false } }) }; if !t405.truthy() { t405 } else { { let t403 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t404 = Py::Float(1e-15); if py::lt(&t403, &t404)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3804(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3806: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3805 = v_inputs.clone(); f_pvwatts_inverter(py::getitem(&t3805, &Py::str("pdc"))?, py::getitem(&t3805, &Py::str("pdc0"))?, py::method(&t3805, "get", &[Py::str("eta_nom"), Py::Float(0.96)])?, py::method(&t3805, "get", &[Py::str("eta_ref"), Py::Float(0.9637)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3806 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3807 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3836 = (t3807).items()?;
        if t3836.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3836[0].clone();
        v_v = t3836[1].clone();
        if ({ let t3808 = v_k.clone(); Py::Bool({ let t3809 = v_outputs.clone(); if !py::contains(&t3809, &t3808)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3835 = { let t3826 = v_d.clone(); Py::Bool({ let t3827 = Py::Int(0i64); if py::gt(&t3826, &t3827)? { true } else { false } }) }; if !t3835.truthy() { t3835 } else { { let t3834 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3834.truthy() { t3834 } else { { let t3833 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3833.truthy() { t3833 } else { { let t3832 = { let t3828 = py::abs(&v_v.clone())?; Py::Bool({ let t3829 = Py::Float(1e-15); if py::lt(&t3828, &t3829)? { true } else { false } }) }; if !t3832.truthy() { t3832 } else { { let t3830 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3831 = Py::Float(1e-15); if py::lt(&t3830, &t3831)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3855(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3857: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = f_pvwatts_losses(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3857 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3858 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3887 = (t3858).items()?;
        if t3887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3887[0].clone();
        v_v = t3887[1].clone();
        if ({ let t3859 = v_k.clone(); Py::Bool({ let t3860 = v_outputs.clone(); if !py::contains(&t3860, &t3859)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3886 = { let t3877 = v_d.clone(); Py::Bool({ let t3878 = Py::Int(0i64); if py::gt(&t3877, &t3878)? { true } else { false } }) }; if !t3886.truthy() { t3886 } else { { let t3885 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3885.truthy() { t3885 } else { { let t3884 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3884.truthy() { t3884 } else { { let t3883 = { let t3879 = py::abs(&v_v.clone())?; Py::Bool({ let t3880 = Py::Float(1e-15); if py::lt(&t3879, &t3880)? { true } else { false } }) }; if !t3883.truthy() { t3883 } else { { let t3881 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3882 = Py::Float(1e-15); if py::lt(&t3881, &t3882)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3917(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3919: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3918 = v_inputs.clone(); f_short_line(py::getitem(&t3918, &Py::str("Vr"))?, py::getitem(&t3918, &Py::str("P"))?, py::getitem(&t3918, &Py::str("pf"))?, py::getitem(&t3918, &Py::str("R"))?, py::getitem(&t3918, &Py::str("X"))?, py::method(&t3918, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3919 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3920 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3949 = (t3920).items()?;
        if t3949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3949[0].clone();
        v_v = t3949[1].clone();
        if ({ let t3921 = v_k.clone(); Py::Bool({ let t3922 = v_outputs.clone(); if !py::contains(&t3922, &t3921)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t3948 = { let t3939 = v_d.clone(); Py::Bool({ let t3940 = Py::Int(0i64); if py::gt(&t3939, &t3940)? { true } else { false } }) }; if !t3948.truthy() { t3948 } else { { let t3947 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3947.truthy() { t3947 } else { { let t3946 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3946.truthy() { t3946 } else { { let t3945 = { let t3941 = py::abs(&v_v.clone())?; Py::Bool({ let t3942 = Py::Float(1e-15); if py::lt(&t3941, &t3942)? { true } else { false } }) }; if !t3945.truthy() { t3945 } else { { let t3943 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t3944 = Py::Float(1e-15); if py::lt(&t3943, &t3944)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4005(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4007: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4006 = v_inputs.clone(); f_single_diode_current(py::getitem(&t4006, &Py::str("V"))?, py::getitem(&t4006, &Py::str("IL"))?, py::getitem(&t4006, &Py::str("I0"))?, py::getitem(&t4006, &Py::str("Rs"))?, py::getitem(&t4006, &Py::str("Rsh"))?, py::getitem(&t4006, &Py::str("nNsVth"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4007 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4008 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4037 = (t4008).items()?;
        if t4037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4037[0].clone();
        v_v = t4037[1].clone();
        if ({ let t4009 = v_k.clone(); Py::Bool({ let t4010 = v_outputs.clone(); if !py::contains(&t4010, &t4009)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4036 = { let t4027 = v_d.clone(); Py::Bool({ let t4028 = Py::Int(0i64); if py::gt(&t4027, &t4028)? { true } else { false } }) }; if !t4036.truthy() { t4036 } else { { let t4035 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4035.truthy() { t4035 } else { { let t4034 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4034.truthy() { t4034 } else { { let t4033 = { let t4029 = py::abs(&v_v.clone())?; Py::Bool({ let t4030 = Py::Float(1e-15); if py::lt(&t4029, &t4030)? { true } else { false } }) }; if !t4033.truthy() { t4033 } else { { let t4031 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4032 = Py::Float(1e-15); if py::lt(&t4031, &t4032)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4107(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4109: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4108 = v_inputs.clone(); f_single_diode_mpp(py::getitem(&t4108, &Py::str("IL"))?, py::getitem(&t4108, &Py::str("I0"))?, py::getitem(&t4108, &Py::str("Rs"))?, py::getitem(&t4108, &Py::str("Rsh"))?, py::getitem(&t4108, &Py::str("nNsVth"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4109 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4110 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4139 = (t4110).items()?;
        if t4139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4139[0].clone();
        v_v = t4139[1].clone();
        if ({ let t4111 = v_k.clone(); Py::Bool({ let t4112 = v_outputs.clone(); if !py::contains(&t4112, &t4111)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4138 = { let t4129 = v_d.clone(); Py::Bool({ let t4130 = Py::Int(0i64); if py::gt(&t4129, &t4130)? { true } else { false } }) }; if !t4138.truthy() { t4138 } else { { let t4137 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4137.truthy() { t4137 } else { { let t4136 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4136.truthy() { t4136 } else { { let t4135 = { let t4131 = py::abs(&v_v.clone())?; Py::Bool({ let t4132 = Py::Float(1e-15); if py::lt(&t4131, &t4132)? { true } else { false } }) }; if !t4135.truthy() { t4135 } else { { let t4133 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4134 = Py::Float(1e-15); if py::lt(&t4133, &t4134)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4198(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4200: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4199 = v_inputs.clone(); f_single_diode_voltage(py::getitem(&t4199, &Py::str("I"))?, py::getitem(&t4199, &Py::str("IL"))?, py::getitem(&t4199, &Py::str("I0"))?, py::getitem(&t4199, &Py::str("Rs"))?, py::getitem(&t4199, &Py::str("Rsh"))?, py::getitem(&t4199, &Py::str("nNsVth"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4200 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4201 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4230 = (t4201).items()?;
        if t4230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4230[0].clone();
        v_v = t4230[1].clone();
        if ({ let t4202 = v_k.clone(); Py::Bool({ let t4203 = v_outputs.clone(); if !py::contains(&t4203, &t4202)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4229 = { let t4220 = v_d.clone(); Py::Bool({ let t4221 = Py::Int(0i64); if py::gt(&t4220, &t4221)? { true } else { false } }) }; if !t4229.truthy() { t4229 } else { { let t4228 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4228.truthy() { t4228 } else { { let t4227 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4227.truthy() { t4227 } else { { let t4226 = { let t4222 = py::abs(&v_v.clone())?; Py::Bool({ let t4223 = Py::Float(1e-15); if py::lt(&t4222, &t4223)? { true } else { false } }) }; if !t4226.truthy() { t4226 } else { { let t4224 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4225 = Py::Float(1e-15); if py::lt(&t4224, &t4225)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4231(mut v_X1: Py, mut v_X2: Py, mut v_X0: Py, mut v_Rf: Py, mut v_Xf: Py, mut v_Vf: Py) -> R {
    return Ok(f_unsym_fault(Py::str("slg"), v_X1.clone(), v_X2.clone(), v_X0.clone(), v_Rf.clone(), v_Xf.clone(), v_Vf.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4269(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4271: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4270 = v_inputs.clone(); s_4231(py::getitem(&t4270, &Py::str("X1"))?, py::getitem(&t4270, &Py::str("X2"))?, py::getitem(&t4270, &Py::str("X0"))?, py::method(&t4270, "get", &[Py::str("Rf"), Py::Float(0.0)])?, py::method(&t4270, "get", &[Py::str("Xf"), Py::Float(0.0)])?, py::method(&t4270, "get", &[Py::str("Vf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4271 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4272 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4301 = (t4272).items()?;
        if t4301.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4301[0].clone();
        v_v = t4301[1].clone();
        if ({ let t4273 = v_k.clone(); Py::Bool({ let t4274 = v_outputs.clone(); if !py::contains(&t4274, &t4273)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4300 = { let t4291 = v_d.clone(); Py::Bool({ let t4292 = Py::Int(0i64); if py::gt(&t4291, &t4292)? { true } else { false } }) }; if !t4300.truthy() { t4300 } else { { let t4299 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4299.truthy() { t4299 } else { { let t4298 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4298.truthy() { t4298 } else { { let t4297 = { let t4293 = py::abs(&v_v.clone())?; Py::Bool({ let t4294 = Py::Float(1e-15); if py::lt(&t4293, &t4294)? { true } else { false } }) }; if !t4297.truthy() { t4297 } else { { let t4295 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4296 = Py::Float(1e-15); if py::lt(&t4295, &t4296)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4302(mut v_n: Py, mut v_method: Py) -> R {
    return Ok(f_solar_declination(v_n.clone(), v_method.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_432(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t434: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t433 = v_inputs.clone(); f_cell_temperature_sapm(py::getitem(&t433, &Py::str("poa"))?, py::getitem(&t433, &Py::str("T_air"))?, py::getitem(&t433, &Py::str("wind"))?, py::getitem(&t433, &Py::str("mount"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t434 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t464 = (t435).items()?;
        if t464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t464[0].clone();
        v_v = t464[1].clone();
        if ({ let t436 = v_k.clone(); Py::Bool({ let t437 = v_outputs.clone(); if !py::contains(&t437, &t436)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t463 = { let t454 = v_d.clone(); Py::Bool({ let t455 = Py::Int(0i64); if py::gt(&t454, &t455)? { true } else { false } }) }; if !t463.truthy() { t463 } else { { let t462 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t462.truthy() { t462 } else { { let t461 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t461.truthy() { t461 } else { { let t460 = { let t456 = py::abs(&v_v.clone())?; Py::Bool({ let t457 = Py::Float(1e-15); if py::lt(&t456, &t457)? { true } else { false } }) }; if !t460.truthy() { t460 } else { { let t458 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t459 = Py::Float(1e-15); if py::lt(&t458, &t459)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4321(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4323: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4322 = v_inputs.clone(); s_4302(py::getitem(&t4322, &Py::str("n"))?, py::method(&t4322, "get", &[Py::str("method"), Py::str("cooper")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4323 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4324 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4353 = (t4324).items()?;
        if t4353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4353[0].clone();
        v_v = t4353[1].clone();
        if ({ let t4325 = v_k.clone(); Py::Bool({ let t4326 = v_outputs.clone(); if !py::contains(&t4326, &t4325)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4352 = { let t4343 = v_d.clone(); Py::Bool({ let t4344 = Py::Int(0i64); if py::gt(&t4343, &t4344)? { true } else { false } }) }; if !t4352.truthy() { t4352 } else { { let t4351 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4351.truthy() { t4351 } else { { let t4350 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4350.truthy() { t4350 } else { { let t4349 = { let t4345 = py::abs(&v_v.clone())?; Py::Bool({ let t4346 = Py::Float(1e-15); if py::lt(&t4345, &t4346)? { true } else { false } }) }; if !t4349.truthy() { t4349 } else { { let t4347 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4348 = Py::Float(1e-15); if py::lt(&t4347, &t4348)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4403(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4405: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4404 = v_inputs.clone(); f_solar_position(py::getitem(&t4404, &Py::str("latitude"))?, py::getitem(&t4404, &Py::str("longitude"))?, py::getitem(&t4404, &Py::str("n"))?, py::getitem(&t4404, &Py::str("clock"))?, py::getitem(&t4404, &Py::str("utc_offset"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4405 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4406 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4435 = (t4406).items()?;
        if t4435.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4435[0].clone();
        v_v = t4435[1].clone();
        if ({ let t4407 = v_k.clone(); Py::Bool({ let t4408 = v_outputs.clone(); if !py::contains(&t4408, &t4407)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4434 = { let t4425 = v_d.clone(); Py::Bool({ let t4426 = Py::Int(0i64); if py::gt(&t4425, &t4426)? { true } else { false } }) }; if !t4434.truthy() { t4434 } else { { let t4433 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4433.truthy() { t4433 } else { { let t4432 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4432.truthy() { t4432 } else { { let t4431 = { let t4427 = py::abs(&v_v.clone())?; Py::Bool({ let t4428 = Py::Float(1e-15); if py::lt(&t4427, &t4428)? { true } else { false } }) }; if !t4431.truthy() { t4431 } else { { let t4429 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4430 = Py::Float(1e-15); if py::lt(&t4429, &t4430)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4475: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4474 = v_inputs.clone(); f_solar_time_hour_angle(py::getitem(&t4474, &Py::str("n"))?, py::getitem(&t4474, &Py::str("clock"))?, py::getitem(&t4474, &Py::str("longitude"))?, py::getitem(&t4474, &Py::str("utc_offset"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4475 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4476 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4505 = (t4476).items()?;
        if t4505.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4505[0].clone();
        v_v = t4505[1].clone();
        if ({ let t4477 = v_k.clone(); Py::Bool({ let t4478 = v_outputs.clone(); if !py::contains(&t4478, &t4477)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4504 = { let t4495 = v_d.clone(); Py::Bool({ let t4496 = Py::Int(0i64); if py::gt(&t4495, &t4496)? { true } else { false } }) }; if !t4504.truthy() { t4504 } else { { let t4503 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4503.truthy() { t4503 } else { { let t4502 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4502.truthy() { t4502 } else { { let t4501 = { let t4497 = py::abs(&v_v.clone())?; Py::Bool({ let t4498 = Py::Float(1e-15); if py::lt(&t4497, &t4498)? { true } else { false } }) }; if !t4501.truthy() { t4501 } else { { let t4499 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4500 = Py::Float(1e-15); if py::lt(&t4499, &t4500)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4529(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4531: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4530 = v_inputs.clone(); f_stack_power(py::getitem(&t4530, &Py::str("I"))?, py::getitem(&t4530, &Py::str("N"))?, py::getitem(&t4530, &Py::str("V_cell"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4531 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4532 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4561 = (t4532).items()?;
        if t4561.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4561[0].clone();
        v_v = t4561[1].clone();
        if ({ let t4533 = v_k.clone(); Py::Bool({ let t4534 = v_outputs.clone(); if !py::contains(&t4534, &t4533)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4560 = { let t4551 = v_d.clone(); Py::Bool({ let t4552 = Py::Int(0i64); if py::gt(&t4551, &t4552)? { true } else { false } }) }; if !t4560.truthy() { t4560 } else { { let t4559 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4559.truthy() { t4559 } else { { let t4558 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4558.truthy() { t4558 } else { { let t4557 = { let t4553 = py::abs(&v_v.clone())?; Py::Bool({ let t4554 = Py::Float(1e-15); if py::lt(&t4553, &t4554)? { true } else { false } }) }; if !t4557.truthy() { t4557 } else { { let t4555 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4556 = Py::Float(1e-15); if py::lt(&t4555, &t4556)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4610(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4612: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4611 = v_inputs.clone(); f_sun_angles(py::getitem(&t4611, &Py::str("latitude"))?, py::getitem(&t4611, &Py::str("declination"))?, py::getitem(&t4611, &Py::str("hour_angle"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4612 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4613 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4642 = (t4613).items()?;
        if t4642.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4642[0].clone();
        v_v = t4642[1].clone();
        if ({ let t4614 = v_k.clone(); Py::Bool({ let t4615 = v_outputs.clone(); if !py::contains(&t4615, &t4614)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4641 = { let t4632 = v_d.clone(); Py::Bool({ let t4633 = Py::Int(0i64); if py::gt(&t4632, &t4633)? { true } else { false } }) }; if !t4641.truthy() { t4641 } else { { let t4640 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4640.truthy() { t4640 } else { { let t4639 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4639.truthy() { t4639 } else { { let t4638 = { let t4634 = py::abs(&v_v.clone())?; Py::Bool({ let t4635 = Py::Float(1e-15); if py::lt(&t4634, &t4635)? { true } else { false } }) }; if !t4638.truthy() { t4638 } else { { let t4636 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4637 = Py::Float(1e-15); if py::lt(&t4636, &t4637)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4681(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4683: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4682 = v_inputs.clone(); f_sunrise_sunset(py::getitem(&t4682, &Py::str("latitude"))?, py::getitem(&t4682, &Py::str("longitude"))?, py::getitem(&t4682, &Py::str("n"))?, py::getitem(&t4682, &Py::str("utc_offset"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4683 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4684 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4713 = (t4684).items()?;
        if t4713.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4713[0].clone();
        v_v = t4713[1].clone();
        if ({ let t4685 = v_k.clone(); Py::Bool({ let t4686 = v_outputs.clone(); if !py::contains(&t4686, &t4685)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4712 = { let t4703 = v_d.clone(); Py::Bool({ let t4704 = Py::Int(0i64); if py::gt(&t4703, &t4704)? { true } else { false } }) }; if !t4712.truthy() { t4712 } else { { let t4711 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4711.truthy() { t4711 } else { { let t4710 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4710.truthy() { t4710 } else { { let t4709 = { let t4705 = py::abs(&v_v.clone())?; Py::Bool({ let t4706 = Py::Float(1e-15); if py::lt(&t4705, &t4706)? { true } else { false } }) }; if !t4709.truthy() { t4709 } else { { let t4707 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4708 = Py::Float(1e-15); if py::lt(&t4707, &t4708)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4801(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4803: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4802 = v_inputs.clone(); f_symmetrical_fault(py::getitem(&t4802, &Py::str("network"))?, py::getitem(&t4802, &Py::str("fault_bus"))?, py::method(&t4802, "get", &[Py::str("Vf"), Py::Float(1.0)])?, py::method(&t4802, "get", &[Py::str("Xf"), Py::Float(0.0)])?, py::method(&t4802, "get", &[Py::str("S_base"), Py::None])?, py::method(&t4802, "get", &[Py::str("V_base"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4803 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4804 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4833 = (t4804).items()?;
        if t4833.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4833[0].clone();
        v_v = t4833[1].clone();
        if ({ let t4805 = v_k.clone(); Py::Bool({ let t4806 = v_outputs.clone(); if !py::contains(&t4806, &t4805)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4832 = { let t4823 = v_d.clone(); Py::Bool({ let t4824 = Py::Int(0i64); if py::gt(&t4823, &t4824)? { true } else { false } }) }; if !t4832.truthy() { t4832 } else { { let t4831 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4831.truthy() { t4831 } else { { let t4830 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4830.truthy() { t4830 } else { { let t4829 = { let t4825 = py::abs(&v_v.clone())?; Py::Bool({ let t4826 = Py::Float(1e-15); if py::lt(&t4825, &t4826)? { true } else { false } }) }; if !t4829.truthy() { t4829 } else { { let t4827 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4828 = Py::Float(1e-15); if py::lt(&t4827, &t4828)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t485: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t484 = v_inputs.clone(); f_haurwitz(py::getitem(&t484, &Py::str("zenith"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t485 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t486 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t515 = (t486).items()?;
        if t515.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t515[0].clone();
        v_v = t515[1].clone();
        if ({ let t487 = v_k.clone(); Py::Bool({ let t488 = v_outputs.clone(); if !py::contains(&t488, &t487)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t514 = { let t505 = v_d.clone(); Py::Bool({ let t506 = Py::Int(0i64); if py::gt(&t505, &t506)? { true } else { false } }) }; if !t514.truthy() { t514 } else { { let t513 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t513.truthy() { t513 } else { { let t512 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t512.truthy() { t512 } else { { let t511 = { let t507 = py::abs(&v_v.clone())?; Py::Bool({ let t508 = Py::Float(1e-15); if py::lt(&t507, &t508)? { true } else { false } }) }; if !t511.truthy() { t511 } else { { let t509 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t510 = Py::Float(1e-15); if py::lt(&t509, &t510)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4864(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4866: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4865 = v_inputs.clone(); f_synchronous_generator(py::getitem(&t4865, &Py::str("VL"))?, py::getitem(&t4865, &Py::str("IL"))?, py::getitem(&t4865, &Py::str("pf"))?, py::getitem(&t4865, &Py::str("Xs"))?, py::method(&t4865, "get", &[Py::str("Ra"), Py::Float(0.0)])?, py::method(&t4865, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4866 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4867 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4896 = (t4867).items()?;
        if t4896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4896[0].clone();
        v_v = t4896[1].clone();
        if ({ let t4868 = v_k.clone(); Py::Bool({ let t4869 = v_outputs.clone(); if !py::contains(&t4869, &t4868)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4895 = { let t4886 = v_d.clone(); Py::Bool({ let t4887 = Py::Int(0i64); if py::gt(&t4886, &t4887)? { true } else { false } }) }; if !t4895.truthy() { t4895 } else { { let t4894 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4894.truthy() { t4894 } else { { let t4893 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4893.truthy() { t4893 } else { { let t4892 = { let t4888 = py::abs(&v_v.clone())?; Py::Bool({ let t4889 = Py::Float(1e-15); if py::lt(&t4888, &t4889)? { true } else { false } }) }; if !t4892.truthy() { t4892 } else { { let t4890 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4891 = Py::Float(1e-15); if py::lt(&t4890, &t4891)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4940(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4942: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4941 = v_inputs.clone(); f_machine_speed(py::getitem(&t4941, &Py::str("f"))?, py::getitem(&t4941, &Py::str("poles"))?, py::method(&t4941, "get", &[Py::str("s"), Py::None])?, py::method(&t4941, "get", &[Py::str("n_r"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4942 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4943 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4972 = (t4943).items()?;
        if t4972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4972[0].clone();
        v_v = t4972[1].clone();
        if ({ let t4944 = v_k.clone(); Py::Bool({ let t4945 = v_outputs.clone(); if !py::contains(&t4945, &t4944)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t4971 = { let t4962 = v_d.clone(); Py::Bool({ let t4963 = Py::Int(0i64); if py::gt(&t4962, &t4963)? { true } else { false } }) }; if !t4971.truthy() { t4971 } else { { let t4970 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4970.truthy() { t4970 } else { { let t4969 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4969.truthy() { t4969 } else { { let t4968 = { let t4964 = py::abs(&v_v.clone())?; Py::Bool({ let t4965 = Py::Float(1e-15); if py::lt(&t4964, &t4965)? { true } else { false } }) }; if !t4968.truthy() { t4968 } else { { let t4966 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t4967 = Py::Float(1e-15); if py::lt(&t4966, &t4967)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5001(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5003: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5002 = v_inputs.clone(); f_three_phase_power(py::getitem(&t5002, &Py::str("VL"))?, py::getitem(&t5002, &Py::str("IL"))?, py::getitem(&t5002, &Py::str("pf"))?, py::getitem(&t5002, &Py::str("connection"))?, py::method(&t5002, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5003 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5004 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5033 = (t5004).items()?;
        if t5033.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5033[0].clone();
        v_v = t5033[1].clone();
        if ({ let t5005 = v_k.clone(); Py::Bool({ let t5006 = v_outputs.clone(); if !py::contains(&t5006, &t5005)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5032 = { let t5023 = v_d.clone(); Py::Bool({ let t5024 = Py::Int(0i64); if py::gt(&t5023, &t5024)? { true } else { false } }) }; if !t5032.truthy() { t5032 } else { { let t5031 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5031.truthy() { t5031 } else { { let t5030 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5030.truthy() { t5030 } else { { let t5029 = { let t5025 = py::abs(&v_v.clone())?; Py::Bool({ let t5026 = Py::Float(1e-15); if py::lt(&t5025, &t5026)? { true } else { false } }) }; if !t5029.truthy() { t5029 } else { { let t5027 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5028 = Py::Float(1e-15); if py::lt(&t5027, &t5028)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5100(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5102: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5101 = v_inputs.clone(); f_transformer_loading(py::getitem(&t5101, &Py::str("sn"))?, py::getitem(&t5101, &Py::str("V_hv"))?, py::getitem(&t5101, &Py::str("V_lv"))?, py::getitem(&t5101, &Py::str("vk"))?, py::getitem(&t5101, &Py::str("vkr"))?, py::getitem(&t5101, &Py::str("P0"))?, py::getitem(&t5101, &Py::str("i0"))?, py::getitem(&t5101, &Py::str("P"))?, py::getitem(&t5101, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5102 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5103 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5132 = (t5103).items()?;
        if t5132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5132[0].clone();
        v_v = t5132[1].clone();
        if ({ let t5104 = v_k.clone(); Py::Bool({ let t5105 = v_outputs.clone(); if !py::contains(&t5105, &t5104)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5131 = { let t5122 = v_d.clone(); Py::Bool({ let t5123 = Py::Int(0i64); if py::gt(&t5122, &t5123)? { true } else { false } }) }; if !t5131.truthy() { t5131 } else { { let t5130 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5130.truthy() { t5130 } else { { let t5129 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5129.truthy() { t5129 } else { { let t5128 = { let t5124 = py::abs(&v_v.clone())?; Py::Bool({ let t5125 = Py::Float(1e-15); if py::lt(&t5124, &t5125)? { true } else { false } }) }; if !t5128.truthy() { t5128 } else { { let t5126 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5127 = Py::Float(1e-15); if py::lt(&t5126, &t5127)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5153(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5155: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5154 = v_inputs.clone(); f_transformer_max_efficiency(py::getitem(&t5154, &Py::str("S"))?, py::getitem(&t5154, &Py::str("P_core"))?, py::getitem(&t5154, &Py::str("P_cu_fl"))?, py::getitem(&t5154, &Py::str("pf"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5155 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5156 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5185 = (t5156).items()?;
        if t5185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5185[0].clone();
        v_v = t5185[1].clone();
        if ({ let t5157 = v_k.clone(); Py::Bool({ let t5158 = v_outputs.clone(); if !py::contains(&t5158, &t5157)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5184 = { let t5175 = v_d.clone(); Py::Bool({ let t5176 = Py::Int(0i64); if py::gt(&t5175, &t5176)? { true } else { false } }) }; if !t5184.truthy() { t5184 } else { { let t5183 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5183.truthy() { t5183 } else { { let t5182 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5182.truthy() { t5182 } else { { let t5181 = { let t5177 = py::abs(&v_v.clone())?; Py::Bool({ let t5178 = Py::Float(1e-15); if py::lt(&t5177, &t5178)? { true } else { false } }) }; if !t5181.truthy() { t5181 } else { { let t5179 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5180 = Py::Float(1e-15); if py::lt(&t5179, &t5180)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5217(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5219: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5218 = v_inputs.clone(); f_transformer_performance(py::getitem(&t5218, &Py::str("S"))?, py::getitem(&t5218, &Py::str("V1"))?, py::getitem(&t5218, &Py::str("V2"))?, py::getitem(&t5218, &Py::str("Req"))?, py::getitem(&t5218, &Py::str("Xeq"))?, py::getitem(&t5218, &Py::str("Rc"))?, py::getitem(&t5218, &Py::str("Xm"))?, py::getitem(&t5218, &Py::str("load"))?, py::getitem(&t5218, &Py::str("pf"))?, py::method(&t5218, "get", &[Py::str("pf_type"), Py::str("lagging")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5219 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5220 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5249 = (t5220).items()?;
        if t5249.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5249[0].clone();
        v_v = t5249[1].clone();
        if ({ let t5221 = v_k.clone(); Py::Bool({ let t5222 = v_outputs.clone(); if !py::contains(&t5222, &t5221)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5248 = { let t5239 = v_d.clone(); Py::Bool({ let t5240 = Py::Int(0i64); if py::gt(&t5239, &t5240)? { true } else { false } }) }; if !t5248.truthy() { t5248 } else { { let t5247 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5247.truthy() { t5247 } else { { let t5246 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5246.truthy() { t5246 } else { { let t5245 = { let t5241 = py::abs(&v_v.clone())?; Py::Bool({ let t5242 = Py::Float(1e-15); if py::lt(&t5241, &t5242)? { true } else { false } }) }; if !t5245.truthy() { t5245 } else { { let t5243 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5244 = Py::Float(1e-15); if py::lt(&t5243, &t5244)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5273(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5275: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5274 = v_inputs.clone(); f_transformer_tests(py::getitem(&t5274, &Py::str("Voc"))?, py::getitem(&t5274, &Py::str("Ioc"))?, py::getitem(&t5274, &Py::str("Poc"))?, py::getitem(&t5274, &Py::str("Vsc"))?, py::getitem(&t5274, &Py::str("Isc"))?, py::getitem(&t5274, &Py::str("Psc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5275 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5276 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5305 = (t5276).items()?;
        if t5305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5305[0].clone();
        v_v = t5305[1].clone();
        if ({ let t5277 = v_k.clone(); Py::Bool({ let t5278 = v_outputs.clone(); if !py::contains(&t5278, &t5277)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5304 = { let t5295 = v_d.clone(); Py::Bool({ let t5296 = Py::Int(0i64); if py::gt(&t5295, &t5296)? { true } else { false } }) }; if !t5304.truthy() { t5304 } else { { let t5303 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5303.truthy() { t5303 } else { { let t5302 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5302.truthy() { t5302 } else { { let t5301 = { let t5297 = py::abs(&v_v.clone())?; Py::Bool({ let t5298 = Py::Float(1e-15); if py::lt(&t5297, &t5298)? { true } else { false } }) }; if !t5301.truthy() { t5301 } else { { let t5299 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5300 = Py::Float(1e-15); if py::lt(&t5299, &t5300)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5333(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5335: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5334 = v_inputs.clone(); f_two_bus_load_flow(py::getitem(&t5334, &Py::str("Vs"))?, py::getitem(&t5334, &Py::str("R"))?, py::getitem(&t5334, &Py::str("X"))?, py::getitem(&t5334, &Py::str("P"))?, py::getitem(&t5334, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5335 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5336 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5365 = (t5336).items()?;
        if t5365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5365[0].clone();
        v_v = t5365[1].clone();
        if ({ let t5337 = v_k.clone(); Py::Bool({ let t5338 = v_outputs.clone(); if !py::contains(&t5338, &t5337)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5364 = { let t5355 = v_d.clone(); Py::Bool({ let t5356 = Py::Int(0i64); if py::gt(&t5355, &t5356)? { true } else { false } }) }; if !t5364.truthy() { t5364 } else { { let t5363 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5363.truthy() { t5363 } else { { let t5362 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5362.truthy() { t5362 } else { { let t5361 = { let t5357 = py::abs(&v_v.clone())?; Py::Bool({ let t5358 = Py::Float(1e-15); if py::lt(&t5357, &t5358)? { true } else { false } }) }; if !t5361.truthy() { t5361 } else { { let t5359 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5360 = Py::Float(1e-15); if py::lt(&t5359, &t5360)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5407: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5406 = v_inputs.clone(); f_weibull_aep(py::getitem(&t5406, &Py::str("speeds"))?, py::getitem(&t5406, &Py::str("powers"))?, py::getitem(&t5406, &Py::str("k"))?, py::getitem(&t5406, &Py::str("c"))?, py::method(&t5406, "get", &[Py::str("hours"), Py::Float(8760.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5407 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5408 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5437 = (t5408).items()?;
        if t5437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5437[0].clone();
        v_v = t5437[1].clone();
        if ({ let t5409 = v_k.clone(); Py::Bool({ let t5410 = v_outputs.clone(); if !py::contains(&t5410, &t5409)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5436 = { let t5427 = v_d.clone(); Py::Bool({ let t5428 = Py::Int(0i64); if py::gt(&t5427, &t5428)? { true } else { false } }) }; if !t5436.truthy() { t5436 } else { { let t5435 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5435.truthy() { t5435 } else { { let t5434 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5434.truthy() { t5434 } else { { let t5433 = { let t5429 = py::abs(&v_v.clone())?; Py::Bool({ let t5430 = Py::Float(1e-15); if py::lt(&t5429, &t5430)? { true } else { false } }) }; if !t5433.truthy() { t5433 } else { { let t5431 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5432 = Py::Float(1e-15); if py::lt(&t5431, &t5432)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5460(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5462: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5461 = v_inputs.clone(); f_weibull_wind(py::getitem(&t5461, &Py::str("k"))?, py::getitem(&t5461, &Py::str("c"))?, py::getitem(&t5461, &Py::str("rho"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5462 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5463 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5492 = (t5463).items()?;
        if t5492.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5492[0].clone();
        v_v = t5492[1].clone();
        if ({ let t5464 = v_k.clone(); Py::Bool({ let t5465 = v_outputs.clone(); if !py::contains(&t5465, &t5464)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5491 = { let t5482 = v_d.clone(); Py::Bool({ let t5483 = Py::Int(0i64); if py::gt(&t5482, &t5483)? { true } else { false } }) }; if !t5491.truthy() { t5491 } else { { let t5490 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5490.truthy() { t5490 } else { { let t5489 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5489.truthy() { t5489 } else { { let t5488 = { let t5484 = py::abs(&v_v.clone())?; Py::Bool({ let t5485 = Py::Float(1e-15); if py::lt(&t5484, &t5485)? { true } else { false } }) }; if !t5488.truthy() { t5488 } else { { let t5486 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5487 = Py::Float(1e-15); if py::lt(&t5486, &t5487)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5516(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5518: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5517 = v_inputs.clone(); f_wind_power(py::getitem(&t5517, &Py::str("rho"))?, py::getitem(&t5517, &Py::str("D"))?, py::getitem(&t5517, &Py::str("v"))?, py::method(&t5517, "get", &[Py::str("cp"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5518 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5519 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5548 = (t5519).items()?;
        if t5548.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5548[0].clone();
        v_v = t5548[1].clone();
        if ({ let t5520 = v_k.clone(); Py::Bool({ let t5521 = v_outputs.clone(); if !py::contains(&t5521, &t5520)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5547 = { let t5538 = v_d.clone(); Py::Bool({ let t5539 = Py::Int(0i64); if py::gt(&t5538, &t5539)? { true } else { false } }) }; if !t5547.truthy() { t5547 } else { { let t5546 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5546.truthy() { t5546 } else { { let t5545 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5545.truthy() { t5545 } else { { let t5544 = { let t5540 = py::abs(&v_v.clone())?; Py::Bool({ let t5541 = Py::Float(1e-15); if py::lt(&t5540, &t5541)? { true } else { false } }) }; if !t5544.truthy() { t5544 } else { { let t5542 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5543 = Py::Float(1e-15); if py::lt(&t5542, &t5543)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5576(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5578: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5577 = v_inputs.clone(); f_wind_shear_log(py::getitem(&t5577, &Py::str("v1"))?, py::getitem(&t5577, &Py::str("h1"))?, py::getitem(&t5577, &Py::str("h2"))?, py::getitem(&t5577, &Py::str("z0"))?, py::method(&t5577, "get", &[Py::str("d"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5578 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5579 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5608 = (t5579).items()?;
        if t5608.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5608[0].clone();
        v_v = t5608[1].clone();
        if ({ let t5580 = v_k.clone(); Py::Bool({ let t5581 = v_outputs.clone(); if !py::contains(&t5581, &t5580)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5607 = { let t5598 = v_d.clone(); Py::Bool({ let t5599 = Py::Int(0i64); if py::gt(&t5598, &t5599)? { true } else { false } }) }; if !t5607.truthy() { t5607 } else { { let t5606 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5606.truthy() { t5606 } else { { let t5605 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5605.truthy() { t5605 } else { { let t5604 = { let t5600 = py::abs(&v_v.clone())?; Py::Bool({ let t5601 = Py::Float(1e-15); if py::lt(&t5600, &t5601)? { true } else { false } }) }; if !t5604.truthy() { t5604 } else { { let t5602 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5603 = Py::Float(1e-15); if py::lt(&t5602, &t5603)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_558(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t560: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t559 = v_inputs.clone(); f_clear_sky_ineichen(py::getitem(&t559, &Py::str("zenith"))?, py::getitem(&t559, &Py::str("linke_turbidity"))?, py::getitem(&t559, &Py::str("altitude"))?, py::getitem(&t559, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t560 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t561 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t590 = (t561).items()?;
        if t590.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t590[0].clone();
        v_v = t590[1].clone();
        if ({ let t562 = v_k.clone(); Py::Bool({ let t563 = v_outputs.clone(); if !py::contains(&t563, &t562)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t589 = { let t580 = v_d.clone(); Py::Bool({ let t581 = Py::Int(0i64); if py::gt(&t580, &t581)? { true } else { false } }) }; if !t589.truthy() { t589 } else { { let t588 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t588.truthy() { t588 } else { { let t587 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t587.truthy() { t587 } else { { let t586 = { let t582 = py::abs(&v_v.clone())?; Py::Bool({ let t583 = Py::Float(1e-15); if py::lt(&t582, &t583)? { true } else { false } }) }; if !t586.truthy() { t586 } else { { let t584 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t585 = Py::Float(1e-15); if py::lt(&t584, &t585)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5629(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5631: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5630 = v_inputs.clone(); f_wind_shear_power_law(py::getitem(&t5630, &Py::str("v1"))?, py::getitem(&t5630, &Py::str("h1"))?, py::getitem(&t5630, &Py::str("h2"))?, py::method(&t5630, "get", &[Py::str("alpha"), py::truediv(&Py::Int(1i64), &Py::Int(7i64))?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5631 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5632 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5661 = (t5632).items()?;
        if t5661.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5661[0].clone();
        v_v = t5661[1].clone();
        if ({ let t5633 = v_k.clone(); Py::Bool({ let t5634 = v_outputs.clone(); if !py::contains(&t5634, &t5633)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t5660 = { let t5651 = v_d.clone(); Py::Bool({ let t5652 = Py::Int(0i64); if py::gt(&t5651, &t5652)? { true } else { false } }) }; if !t5660.truthy() { t5660 } else { { let t5659 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5659.truthy() { t5659 } else { { let t5658 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5658.truthy() { t5658 } else { { let t5657 = { let t5653 = py::abs(&v_v.clone())?; Py::Bool({ let t5654 = Py::Float(1e-15); if py::lt(&t5653, &t5654)? { true } else { false } }) }; if !t5657.truthy() { t5657 } else { { let t5655 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t5656 = Py::Float(1e-15); if py::lt(&t5655, &t5656)? { true } else { false } }) } } } } } } } } }).truthy() {
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
            v_want = { let t625 = v_inputs.clone(); f_coulomb_counting(py::getitem(&t625, &Py::str("SOC0"))?, py::getitem(&t625, &Py::str("Q"))?, py::getitem(&t625, &Py::str("currents"))?, py::getitem(&t625, &Py::str("dt"))?)? };
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
fn s_704(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t706: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t705 = v_inputs.clone(); f_critical_clearing(py::getitem(&t705, &Py::str("Pm"))?, py::getitem(&t705, &Py::str("Pmax_pre"))?, py::getitem(&t705, &Py::str("Pmax_fault"))?, py::getitem(&t705, &Py::str("Pmax_post"))?, py::method(&t705, "get", &[Py::str("H"), Py::None])?, py::method(&t705, "get", &[Py::str("f"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t706 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t707 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t736 = (t707).items()?;
        if t736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t736[0].clone();
        v_v = t736[1].clone();
        if ({ let t708 = v_k.clone(); Py::Bool({ let t709 = v_outputs.clone(); if !py::contains(&t709, &t708)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t735 = { let t726 = v_d.clone(); Py::Bool({ let t727 = Py::Int(0i64); if py::gt(&t726, &t727)? { true } else { false } }) }; if !t735.truthy() { t735 } else { { let t734 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t734.truthy() { t734 } else { { let t733 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t733.truthy() { t733 } else { { let t732 = { let t728 = py::abs(&v_v.clone())?; Py::Bool({ let t729 = Py::Float(1e-15); if py::lt(&t728, &t729)? { true } else { false } }) }; if !t732.truthy() { t732 } else { { let t730 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t731 = Py::Float(1e-15); if py::lt(&t730, &t731)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_802(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t804: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t803 = v_inputs.clone(); f_dc_power_flow(py::getitem(&t803, &Py::str("lines"))?, py::getitem(&t803, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t804 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t805 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t834 = (t805).items()?;
        if t834.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t834[0].clone();
        v_v = t834[1].clone();
        if ({ let t806 = v_k.clone(); Py::Bool({ let t807 = v_outputs.clone(); if !py::contains(&t807, &t806)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t833 = { let t824 = v_d.clone(); Py::Bool({ let t825 = Py::Int(0i64); if py::gt(&t824, &t825)? { true } else { false } }) }; if !t833.truthy() { t833 } else { { let t832 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t832.truthy() { t832 } else { { let t831 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t831.truthy() { t831 } else { { let t830 = { let t826 = py::abs(&v_v.clone())?; Py::Bool({ let t827 = Py::Float(1e-15); if py::lt(&t826, &t827)? { true } else { false } }) }; if !t830.truthy() { t830 } else { { let t828 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t829 = Py::Float(1e-15); if py::lt(&t828, &t829)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_862(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t864: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t863 = v_inputs.clone(); f_desoto_parameters(py::getitem(&t863, &Py::str("G"))?, py::getitem(&t863, &Py::str("T_cell"))?, py::getitem(&t863, &Py::str("alpha_sc"))?, py::getitem(&t863, &Py::str("a_ref"))?, py::getitem(&t863, &Py::str("IL_ref"))?, py::getitem(&t863, &Py::str("I0_ref"))?, py::getitem(&t863, &Py::str("Rsh_ref"))?, py::getitem(&t863, &Py::str("Rs"))?, py::method(&t863, "get", &[Py::str("EgRef"), Py::Float(1.121)])?, py::method(&t863, "get", &[Py::str("dEgdT"), py::neg(&Py::Float(0.0002677))?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t864 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t865 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t894 = (t865).items()?;
        if t894.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t894[0].clone();
        v_v = t894[1].clone();
        if ({ let t866 = v_k.clone(); Py::Bool({ let t867 = v_outputs.clone(); if !py::contains(&t867, &t866)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t893 = { let t884 = v_d.clone(); Py::Bool({ let t885 = Py::Int(0i64); if py::gt(&t884, &t885)? { true } else { false } }) }; if !t893.truthy() { t893 } else { { let t892 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t892.truthy() { t892 } else { { let t891 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t891.truthy() { t891 } else { { let t890 = { let t886 = py::abs(&v_v.clone())?; Py::Bool({ let t887 = Py::Float(1e-15); if py::lt(&t886, &t887)? { true } else { false } }) }; if !t890.truthy() { t890 } else { { let t888 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t889 = Py::Float(1e-15); if py::lt(&t888, &t889)? { true } else { false } }) } } } } } } } } }).truthy() {
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
fn s_936(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t938: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t937 = v_inputs.clone(); f_disc(py::getitem(&t937, &Py::str("ghi"))?, py::getitem(&t937, &Py::str("zenith"))?, py::getitem(&t937, &Py::str("n"))?, py::method(&t937, "get", &[Py::str("pressure"), Py::Float(101325.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t938 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t939 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t968 = (t939).items()?;
        if t968.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t968[0].clone();
        v_v = t968[1].clone();
        if ({ let t940 = v_k.clone(); Py::Bool({ let t941 = v_outputs.clone(); if !py::contains(&t941, &t940)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_d = f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?;
        if ({ let t967 = { let t958 = v_d.clone(); Py::Bool({ let t959 = Py::Int(0i64); if py::gt(&t958, &t959)? { true } else { false } }) }; if !t967.truthy() { t967 } else { { let t966 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t966.truthy() { t966 } else { { let t965 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t965.truthy() { t965 } else { { let t964 = { let t960 = py::abs(&v_v.clone())?; Py::Bool({ let t961 = Py::Float(1e-15); if py::lt(&t960, &t961)? { true } else { false } }) }; if !t964.truthy() { t964 } else { { let t962 = py::abs(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t963 = Py::Float(1e-15); if py::lt(&t962, &t963)? { true } else { false } }) } } } } } } } } }).truthy() {
            v_d = Py::Float(0.0);
        }
        v_worst = py::minmax(&[v_worst.clone(), v_d.clone()], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_969(mut v_X1: Py, mut v_X2: Py, mut v_X0: Py, mut v_Rf: Py, mut v_Xf: Py, mut v_Vf: Py) -> R {
    return Ok(f_unsym_fault(Py::str("dlg"), v_X1.clone(), v_X2.clone(), v_X0.clone(), v_Rf.clone(), v_Xf.clone(), v_Vf.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
