//! Every Energy Systems capability's own live figures (src/figures*.rs): its worked example gives each published
//! figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text outside its
//! canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use energy_systems_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/energy-systems-ai999x/figures, except the
/// generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("energy_actuator_disc", &["schematic"]),
    ("energy_battery_pack_sizing", &["schematic"]),
    ("energy_battery_rc_discharge", &["schematic"]),
    ("energy_coulomb_counting", &["chart"]),
    ("energy_critical_clearing", &["diagram"]),
    ("energy_dc_power_flow", &["oneline"]),
    ("energy_dlg_fault", &["schematic"]),
    ("energy_droop_sharing", &["diagram"]),
    ("energy_economic_dispatch", &["bars", "diagram"]),
    ("energy_electrolyser", &["schematic"]),
    ("energy_equation_of_time", &["chart"]),
    ("energy_fuel_cell_stack", &["schematic"]),
    ("energy_geometric_mean_distance", &["schematic"]),
    ("energy_hydrogen_tank", &["curve-2"]),
    ("energy_induction_motor", &["circuit"]),
    ("energy_line_parameters", &["schematic"]),
    ("energy_ll_fault", &["schematic"]),
    ("energy_medium_line", &["circuit"]),
    ("energy_newton_load_flow", &["oneline"]),
    ("energy_poa_haydavies", &["chart"]),
    ("energy_poa_isotropic", &["chart"]),
    ("energy_poa_klucher", &["chart"]),
    ("energy_poa_perez", &["chart"]),
    ("energy_poa_reindl", &["chart"]),
    ("energy_power_curve_output", &["curve-2", "xy"]),
    ("energy_pvwatts_inverter", &["decision"]),
    ("energy_pvwatts_losses", &["chart"]),
    ("energy_short_line", &["circuit"]),
    ("energy_single_diode_current", &["schematic"]),
    ("energy_single_diode_mpp", &["curve-2"]),
    ("energy_slg_fault", &["schematic"]),
    ("energy_solar_position", &["sky"]),
    ("energy_sun_angles", &["chart"]),
    ("energy_sunrise_sunset", &["chart"]),
    ("energy_symmetrical_fault", &["oneline"]),
    ("energy_synchronous_generator", &["phasor"]),
    ("energy_three_phase_power", &["phasor"]),
    ("energy_transformer_performance", &["circuit"]),
    ("energy_transformer_tests", &["circuit"]),
    ("energy_two_bus_load_flow", &["oneline"]),
    ("energy_weibull_wind", &["distribution"]),
    ("energy_wind_shear_log", &["profile"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities, or whose drawing depends on one input
/// only (a second, different example; an input not in the worked example is added).
const OTHER: &[(&str, &str, &str)] = &[
    ("energy_coulomb_counting", "currents", "[10, 15, -5, 8]"),
    ("energy_dc_power_flow", "P", "0.5, -1.5"),
    ("energy_newton_load_flow", "loads", "2 0.5 0.2"),
    ("energy_symmetrical_fault", "network", "0-1 0.25, 1-2 0.35"),
    ("energy_economic_dispatch", "demand", "700"),
    ("energy_pvwatts_losses", "soiling", "0.05"),
    ("energy_equation_of_time", "n", "140"),
    ("energy_sunrise_sunset", "latitude", "30"),
    ("energy_solar_position", "latitude", "20"),
    ("energy_sun_angles", "latitude", "25"),
    ("energy_power_curve_output", "v", "9 m/s"),
    ("energy_droop_sharing", "R", "[0.04, 0.06]"),
    ("energy_slg_fault", "X0", "0.3"),
    ("energy_dlg_fault", "X0", "0.3"),
    ("energy_ll_fault", "X2", "0.3"),
    ("energy_battery_pack_sizing", "E_target", "7.2e+07 J"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c=[1, 2]).
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else if cs[i] == '[' {
            let s0 = i;
            let mut depth = 0;
            while i < cs.len() {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    if !f.slug.contains("-live-") {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    }
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind} (got {:?})", figs.iter().map(|f| f.kind.clone()).collect::<Vec<_>>())),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:#?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        // only the model's own drawings: a kind the generic sweeps cover is checked by the audit
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let own: Vec<&str> = kinds.iter().copied()
            .filter(|kd| base.iter().any(|f| f.kind == *kd && !f.slug.contains("-live-"))).collect();
        if own.is_empty() {
            continue;
        }
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            match g.iter_mut().find(|e| e.0 == *sym) {
                Some(e) => e.1 = Input::Text(val.to_string()),
                None => g.push((sym.to_string(), Input::Text(val.to_string()))),
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = own.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a mechanical power above the transfer limit is refused by the model: no figure at all
    let p = pack();
    let given: Vec<(String, Input)> = [("Pm", "3"), ("Pmax_pre", "2"), ("Pmax_fault", "0"), ("Pmax_post", "2")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "energy_critical_clearing", &given, None);
    assert!(!a.solved());
    assert!(figures(p, "energy_critical_clearing", &given, None, None).is_empty());
    // a network naming a bus the answer does not have draws no one-line diagram
    let given: Vec<(String, Input)> = [("lines", "1-2 0.1, 2-3 0.2"), ("P", "0, -1")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let figs = figures(p, "energy_dc_power_flow", &given, None, None);
    assert!(figs.iter().all(|f| !f.svg.contains("NaN")));
}
