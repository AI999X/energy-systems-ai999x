//! Energy Systems: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p energy-systems-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&energy_systems_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "energy_battery_pack_sizing", "inputs": {"V_target": "48 V", "E_target": "10 kWh", "V_cell": "3.2 V", "Q_cell": "360000 C"}}));
    println!();
    show(json!({"key": "energy_stack_electrical_power", "inputs": {"I": "200 A", "N": "100", "V_cell": "0.7 V"}}));
    println!();
    show(json!({"key": "energy_capacity_factor", "inputs": {"P_rated": "2 MW", "E": "5256 MWh"}}));
    println!();
    show(json!({"key": "energy_hydrogen_tank", "inputs": {"V": "1 m^3", "p": "700 bar", "T": "288.15 K"}}));
    println!();
    show(json!({"key": "energy_induction_max_torque", "inputs": {"VL": "460 V", "f": "60 Hz", "poles": "4", "R1": "0 ohm", "X1": "1 ohm", "R2": "0.2 ohm", "X2": "1 ohm", "Xm": "1e+12 ohm", "connection": "star"}}));
    println!();
    show(json!({"key": "energy_pvwatts_losses", "inputs": {"soiling": "0", "shading": "0", "snow": "0", "mismatch": "0", "wiring": "0", "connections": "0", "lid": "0", "nameplate_rating": "0", "age": "0", "availability": "0.1"}}));
    println!();
    show(json!({"key": "energy_newton_load_flow", "inputs": {"lines": "1 2 0.0 0.5 0.0", "loads": "2 0.0 0.0"}}));
    println!();
    show(json!({"key": "energy_angle_of_incidence", "inputs": {"tilt": "0", "surface_azimuth": "180", "zenith": "60", "azimuth": "100"}}));
    println!();
    show(json!({"key": "energy_three_phase_power", "inputs": {"VL": "400 V", "IL": "50 A", "pf": "0.8", "connection": "star"}}));
    println!();
    show(json!({"key": "energy_transformer_max_efficiency", "inputs": {"S": "100 kVA", "P_core": "500 W", "P_cu_fl": "2000 W", "pf": "1"}}));
    println!();
    show(json!({"key": "energy_actuator_disc", "inputs": {"a": "0.25", "rho": "1.2 kg/m^3", "D": "10 m", "v": "8 m/s"}}));
    println!();
    show(json!({"key": "energy_battery_rc_discharge", "inputs": {"OCV": "4 V", "R0": "10 mohm", "R1": "20 mohm", "C1": "1000 F", "I": "10 A", "t": "0 s"}}));
    println!();
    show(json!({"key": "energy_butler_volmer", "inputs": {"j0": "10", "alpha_a": "0.5", "alpha_c": "0.5", "T": "298.15 K", "eta": "100 mV"}}));
    println!();
    show(json!({"key": "energy_hydro_power", "inputs": {"Q": "10 m^3/s", "H": "50 m", "eta": "0.9"}}));
    println!();
    show(json!({"key": "energy_hydrogen_isentropic_compression", "inputs": {"p1": "30 bar", "T1": "300 K", "p2": "100 bar"}}));
}
