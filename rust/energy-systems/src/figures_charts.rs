//! Energy Systems' own live figures, part two: charts (power-angle areas, dispatch, sun paths, irradiance on a tilted
//! plane, the Weibull distribution, wind profiles, phasor diagrams, ...). Every curve is a set of further solves of the
//! model (ax_engine::figkit::Solved), never a formula written here.

use crate::figures::{esc, legend, nv, put, range, rows, Plot, CL, DOT, X0, X1, Y1};
use ax_engine::figkit::{self as k, eq, num, num_of, q, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};
use serde_json::json;

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "energy_coulomb_counting" => vec![coulomb(s)],
        "energy_critical_clearing" => vec![equal_area(s)],
        "energy_droop_sharing" => vec![droop(s)],
        "energy_economic_dispatch" => vec![dispatch_bars(s), dispatch_lambda(s)],
        "energy_equation_of_time" => vec![analemma(s)],
        "energy_hydrogen_tank" => vec![hydrogen(s)],
        "energy_poa_haydavies" | "energy_poa_isotropic" | "energy_poa_klucher" | "energy_poa_perez"
        | "energy_poa_reindl" => vec![poa(s)],
        "energy_power_curve_output" => vec![power_curve(s), power_curve_data(s)],
        "energy_pvwatts_losses" => vec![losses(s)],
        "energy_single_diode_mpp" => vec![iv_pv(s)],
        "energy_solar_position" => vec![sky(s)],
        "energy_sun_angles" => vec![sun_path(s)],
        "energy_sunrise_sunset" => vec![daylight(s)],
        "energy_synchronous_generator" => vec![generator_phasor(s)],
        "energy_three_phase_power" => vec![power_triangle(s)],
        "energy_weibull_wind" => vec![weibull(s)],
        "energy_wind_shear_log" => vec![shear(s)],
        _ => vec![],
    }
}

fn sorted(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v.dedup();
    v
}

fn list(a: &Answer, s: &str) -> Option<Vec<f64>> {
    k::list_of(a, s)
}

// ------------------------------------------------------------------------------------------------ storage
fn coulomb(s: &Solved) -> Option<Figure> {
    let cur = s.vec("currents")?;
    let (soc0, dt) = (nv(s, "SOC0")?, nv(s, "dt")?);
    let (soc, smin, qah) = (s.out("SOC")?, s.out("SOC_min")?, s.out("charge_out_Ah")?);
    if cur.is_empty() || cur.len() > 60 || !(dt > 0.0) {
        return None;
    }
    let hrs = dt / 3600.0;
    let mut pts = vec![(0.0, soc0)];
    for n in 1..=cur.len() {
        let a = s.with_input(&[("currents", Input::Json(json!(cur[..n].to_vec())))])?;
        pts.push((n as f64 * hrs, num_of(&a, "SOC")?));
    }
    let tend = cur.len() as f64 * hrs;
    let top = k::lines(&[("state of charge after each interval".into(), pts)], "", "State of charge",
                       &format!("lowest {}", num(smin)), Some((tend, soc)), &num(soc), false);
    let mut step = vec![];
    for (i, c) in cur.iter().enumerate() {
        step.push((i as f64 * hrs, *c));
        step.push(((i + 1) as f64 * hrs, *c));
    }
    let ys: Vec<f64> = cur.iter().copied().chain([0.0]).collect();
    let mut pb = Plot::new((0.0, tend), range(&ys)?, false, true);
    pb.dashed(&[(0.0, 0.0), (tend, 0.0)], "axp-c4");
    pb.path(&step, "axp-c2");
    pb.frame("Time t (h)", "Current I (A)", "");
    pb.legend(&[("current (+ discharge, - charge)".into(), "axp-c2")]);
    let bottom = pb.body();
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("chart", "Coulomb counting: state of charge through a current profile",
                  &format!("A battery starting at state of charge {} through the profile given: {} intervals of {} at \
                            currents {} A (bottom). Top: the model's state of charge after each interval, each point one \
                            run on the profile up to that interval (constant current between points). Marked: final \
                            state of charge {}, lowest {}, net charge out {} Ah.",
                           num(soc0), cur.len(), q(dt, "s"), cur.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                           num(soc), num(smin), num(qah)), &body, h))
}

// ------------------------------------------------------------------------------------------------ power systems
/// The power-angle curve of a transfer limit `amp`, traced by the model: for each mechanical power on the curve its
/// initial angle (rising side) and its largest angle (falling side), as (angle in degrees, power).
fn power_angle(s: &Solved, amp: f64) -> Vec<(f64, f64)> {
    let mut up = vec![];
    let mut down = vec![];
    let fr: Vec<f64> = (1..=18).map(|i| i as f64 / 19.0).chain([0.975, 0.995]).collect();
    for f in fr {
        let p = amp * f;
        if let Some(a) = s.with(&[("Pm", p), ("Pmax_pre", amp), ("Pmax_post", amp), ("Pmax_fault", 0.0)]) {
            if let (Some(d0), Some(dm)) = (num_of(&a, "delta0_deg"), num_of(&a, "delta_max_deg")) {
                up.push((d0, p));
                down.push((dm, p));
            }
        }
    }
    up.extend(down.into_iter().rev());
    up
}

fn interp(c: &[(f64, f64)], x: f64) -> Option<f64> {
    c.windows(2).find(|w| (w[0].0 <= x && x <= w[1].0) || (w[1].0 <= x && x <= w[0].0)).map(|w| {
        if (w[1].0 - w[0].0).abs() < 1e-12 { w[0].1 } else { w[0].1 + (w[1].1 - w[0].1) * (x - w[0].0) / (w[1].0 - w[0].0) }
    })
}

fn between(c: &[(f64, f64)], a: f64, b: f64) -> Vec<(f64, f64)> {
    let mut v = vec![];
    if let Some(y) = interp(c, a) {
        v.push((a, y));
    }
    v.extend(c.iter().filter(|p| p.0 > a && p.0 < b));
    if let Some(y) = interp(c, b) {
        v.push((b, y));
    }
    v
}

fn equal_area(s: &Solved) -> Option<Figure> {
    let (pm, pre, fault, post) = (nv(s, "Pm")?, nv(s, "Pmax_pre")?, nv(s, "Pmax_fault")?, nv(s, "Pmax_post")?);
    let (d0, dm, dc) = (s.out("delta0_deg")?, s.out("delta_max_deg")?, s.out("delta_cr_deg")?);
    let cpre = power_angle(s, pre);
    let cpost = if (post - pre).abs() > 1e-12 { power_angle(s, post) } else { cpre.clone() };
    let cfault = if fault > 0.0 { power_angle(s, fault) } else { vec![(0.0, 0.0), (180.0, 0.0)] };
    if cpre.len() < 6 || cpost.len() < 6 || cfault.len() < 2 {
        return None;
    }
    let ymax = pre.max(post).max(pm) * 1.1;
    let mut p = Plot::new((0.0, 180.0), (0.0, ymax), false, false);
    // A1: accelerating (Pm above the fault curve, from δ0 to δcr); A2: decelerating (post curve above Pm, δcr to δmax)
    let f1 = between(&cfault, d0, dc);
    p.area(&[(d0, pm), (dc, pm)], &f1, "axp-s3");
    let p2 = between(&cpost, dc, dm);
    p.area(&p2, &[(dc, pm), (dm, pm)], "axp-s2");
    p.path(&cpre, "axp-c");
    p.dashed(&cfault, "axp-c4");
    p.path(&cpost, "axp-c2");
    p.dashed(&[(0.0, pm), (180.0, pm)], "axp-c4");
    p.label((178.0, pm), 0.0, -6.0, "end", "axp-lt", &format!("Pm = {}", num(pm)));
    for (i, (d, t)) in [(d0, "δ0"), (dc, "δcr"), (dm, "δmax")].iter().enumerate() {
        p.dashed(&[(*d, 0.0), (*d, ymax)], "axp-c4");
        let (dx, anchor) = if *d > 120.0 { (-4.0, "end") } else { (4.0, "start") };
        p.label((*d, 0.0), dx, if i == 1 { -24.0 } else { -8.0 }, anchor, "axp-vt", &format!("{t} = {}°", num(*d)));
    }
    p.frame("Rotor angle δ (°)", "Electrical power P (pu)", "");
    p.legend(&[("A1".into(), "axp-s3"), ("A2".into(), "axp-s2"), ("before".into(), "axp-c"), ("during".into(), "axp-c4"),
               ("after".into(), "axp-c2")]);
    Some(s.chart("diagram", "Equal-area criterion: critical clearing angle",
                 &format!("Equal-area criterion for mechanical power Pm = {} pu and transfer limits {} (before), {} (during) \
                           and {} pu (after the fault). Each power-angle curve is traced by the model: for a mechanical \
                           power on that curve, its initial angle and its largest angle. The angles marked are the \
                           model's: δ0 = {}°, the critical clearing angle δcr = {}° and the largest angle δmax = {}°; \
                           clearing at δcr makes the accelerating area A1 equal to the decelerating area A2.",
                          num(pm), num(pre), num(fault), num(post), num(d0), num(dc), num(dm)), &p.body()))
}

fn droop(s: &Solved) -> Option<Figure> {
    let (dp, f0) = (nv(s, "dP")?, nv(s, "f0")?);
    let r = s.vec("R")?;
    let rat = s.vec("ratings")?;
    let (f, gen) = (s.out("f")?, s.out_list("dP_gen")?);
    if gen.is_empty() || gen.len() > 4 {
        return None;
    }
    let hi = if dp != 0.0 { 2.0 * dp } else { 100.0 };
    let mut series: Vec<(String, Vec<(f64, f64)>)> = (0..gen.len()).map(|i| {
        (format!("unit {} ({} MW)", i + 1, rat.get(i).map(|x| num(*x)).unwrap_or_default()), vec![])
    }).collect();
    for x in k::span(0.0, hi, 21, false) {
        if let Some(a) = s.with(&[("dP", x)]) {
            if let (Some(ff), Some(g)) = (num_of(&a, "f"), list(&a, "dP_gen")) {
                for (i, ser) in series.iter_mut().enumerate() {
                    if let Some(v) = g.get(i) {
                        ser.1.push((*v, ff));
                    }
                }
            }
        }
    }
    series.retain(|x| x.1.len() >= 2);
    if series.is_empty() {
        return None;
    }
    let body = k::lines(&series, "Extra output of each unit ΔP (MW)", "System frequency f (Hz)",
                        &format!("shares {} MW", gen.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")),
                        Some((gen[0], f)), &format!("f = {} Hz", num(f)), false);
    Some(s.chart("diagram", "Droop sharing between generators",
                 &format!("{} generators on droop control (droops {} on ratings {} MW, nominal {}) taking up a load \
                           increase stepped from 0 to {} MW: each line joins the model's frequency and that unit's share \
                           for every step. Marked: the {} MW increase given settles at {} Hz with shares {} MW.",
                          gen.len(), r.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                          rat.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), q(f0, "Hz"), num(hi), num(dp),
                          num(f), gen.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")), &body))
}

/// The cost rows (a, b, c, Pmin, Pmax) as given, when every row has five numbers.
fn cost_rows(s: &Solved) -> Option<Vec<Vec<f64>>> {
    let t = crate::figures::raw(s, "costs")?;
    let r: Vec<Vec<f64>> = rows(&t).iter().map(|r| r.iter().filter_map(|x| crate::figures::fnum(x)).collect()).collect();
    (!r.is_empty() && r.iter().all(|x| x.len() == 5)).then_some(r)
}

fn dispatch_bars(s: &Solved) -> Option<Figure> {
    let pv = s.out_list("P")?;
    let (lam, cost) = (s.out("lambda_")?, s.out("cost")?);
    let lim = cost_rows(s).filter(|r| r.len() == pv.len());
    let n = pv.len();
    if n == 0 || n > 12 {
        return None;
    }
    let mut top = pv.iter().cloned().fold(0.0, f64::max);
    if let Some(l) = &lim {
        top = top.max(l.iter().map(|r| r[4]).fold(0.0, f64::max));
    }
    let mut p = Plot::new((0.4, n as f64 + 0.6), (0.0, top * 1.12), false, false);
    for (i, v) in pv.iter().enumerate() {
        let x = i as f64 + 1.0;
        p.area(&[(x - 0.3, *v), (x + 0.3, *v)], &[(x - 0.3, 0.0), (x + 0.3, 0.0)], "axp-b");
        p.label((x, *v), 0.0, -6.0, "middle", "axp-vt", &format!("{} MW", num(*v)));
        if let Some(l) = &lim {
            for y in [l[i][3], l[i][4]] {
                p.dashed(&[(x - 0.42, y), (x + 0.42, y)], "axp-c4");
            }
        }
    }
    let cats: Vec<(f64, String)> = (0..n).map(|i| (i as f64 + 1.0, format!("unit {}", i + 1))).collect();
    p.frame("", "Output P (MW)", &format!("λ = {} per MWh", num(lam)));
    cat_axis(&mut p, &cats, "Generator");
    p.legend(&[("dispatched output".into(), "axp-b"), ("limits".into(), "axp-c4")]);
    Some(s.chart("bars", "Economic dispatch: generators sharing a load",
                 &format!("Economic dispatch of {n} generators with quadratic cost curves meeting the demand given: the \
                           model returns outputs {} MW (bars{}) at a common incremental cost λ = {} per MWh, total cost {} \
                           per hour.", pv.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                          if lim.is_some() { ", dashed lines the limits given" } else { "" }, num(lam), num(cost)),
                 &p.body()))
}

/// Category names under the plot instead of number ticks (the number ticks of the x axis are removed first).
fn cat_axis(p: &mut Plot, cats: &[(f64, String)], xlab: &str) {
    // drop the numeric x tick labels the frame wrote (they sit at y = Y1 + 20)
    let tag = format!("y=\"{}\" text-anchor=\"middle\"", Y1 + 20.0);
    let mut out = String::with_capacity(p.s.len());
    let mut rest = p.s.as_str();
    while let Some(i) = rest.find("<text class=\"axp-t\"") {
        let end = rest[i..].find("</text>").map(|j| i + j + 7).unwrap_or(rest.len());
        out.push_str(&rest[..i]);
        if !rest[i..end].contains(&tag) {
            out.push_str(&rest[i..end]);
        }
        rest = &rest[end..];
    }
    out.push_str(rest);
    // and the vertical grid lines with them
    p.s = out;
    let many = cats.len() > 6;
    for (x, t) in cats {
        let px = p.x.px(*x);
        if many {
            p.s.push_str(&format!("<text class=\"axp-lt\" transform=\"translate({px:.1},{}) rotate(-35)\" \
                                   text-anchor=\"end\">{}</text>", Y1 + 14.0, esc(t)));
        } else {
            put(&mut p.s, px, Y1 + 20.0, "middle", "axp-lt", t, 340.0);
        }
    }
    if !many && !xlab.is_empty() {
        p.s.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>",
                              (X0 + X1) / 2.0, esc(xlab)));
    }
}

fn dispatch_lambda(s: &Solved) -> Option<Figure> {
    let pv = s.out_list("P")?;
    let lam = s.out("lambda_")?;
    let dem = nv(s, "demand")?;
    let n = pv.len();
    if n == 0 || n > 4 {
        return None;
    }
    let (lo, hi) = match cost_rows(s).filter(|r| r.len() == n) {
        Some(r) => (r.iter().map(|x| x[3]).sum::<f64>(), r.iter().map(|x| x[4]).sum::<f64>()),
        None => (dem * 0.6, dem * 1.4),
    };
    if !(hi > lo) {
        return None;
    }
    let mut series: Vec<Vec<(f64, f64)>> = vec![vec![]; n];
    for d in k::span(lo + (hi - lo) * 1e-4, hi - (hi - lo) * 1e-4, 25, false) {
        if let Some(a) = s.with(&[("demand", d)]) {
            if let (Some(l), Some(ps)) = (num_of(&a, "lambda_"), list(&a, "P")) {
                for i in 0..n.min(ps.len()) {
                    series[i].push((ps[i], l));
                }
            }
        }
    }
    let xs: Vec<f64> = series.iter().flatten().map(|p| p.0).chain(pv.iter().copied()).collect();
    let ys: Vec<f64> = series.iter().flatten().map(|p| p.1).chain([lam]).collect();
    let ((xl, xh), (yl, yh)) = (range(&xs)?, range(&ys)?);
    if series.iter().all(|x| x.len() < 2) {
        return None;
    }
    let mut p = Plot::new((xl, xh), (yl.min(lam), yh.max(lam)), true, true);
    p.dashed(&[(p.x.lo, lam), (p.x.hi, lam)], "axp-c4");
    for (i, ser) in series.iter().enumerate() {
        p.path(ser, CL[i % 3]);
    }
    for (i, v) in pv.iter().enumerate() {
        p.dashed(&[(*v, p.y.lo), (*v, lam)], CL[i % 3]);
        p.dot((*v, lam), DOT[i % 3], 5.0);
        p.label((*v, lam), 0.0, -12.0, "middle", "axp-vt", &format!("{} MW", num(*v)));
    }
    p.label((p.x.hi, lam), -6.0, 18.0, "end", "axp-lt", &format!("λ = {} per MWh", num(lam)));
    p.frame("Unit output P (MW)", "Incremental cost (per MWh)", "");
    let names: Vec<(String, &str)> = (0..n).map(|i| (format!("unit {}", i + 1), CL[i % 3])).collect();
    p.legend(&names);
    Some(s.chart("diagram", "Economic dispatch: equal incremental cost",
                 &format!("Economic dispatch of {n} units meeting {} MW. Each line joins the model's common incremental \
                           cost and that unit's output for demands from {} to {} MW (one run each; flat and upright \
                           pieces are a unit held at a limit), so it traces the unit's incremental cost. The model's \
                           λ = {} per MWh cuts every line at that unit's dispatched output. Marked: outputs {} MW.",
                          num(dem), num(lo), num(hi), num(lam), pv.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")),
                 &p.body()))
}

// ------------------------------------------------------------------------------------------------ the sun
fn decl(s: &Solved, n: f64) -> Option<f64> {
    s.solve_other("energy_solar_declination", &[("n", Input::Int(n as i64))]).and_then(|a| num_of(&a, "declination_deg"))
}

fn analemma(s: &Solved) -> Option<Figure> {
    let n0 = nv(s, "n")?;
    let e0 = s.out("eot_min")?;
    let mut pts = vec![];
    for d in (1..=361).step_by(5) {
        let d = d as f64;
        if let (Some(a), Some(de)) = (s.with_input(&[("n", Input::Int(d as i64))]), decl(s, d)) {
            if let Some(e) = num_of(&a, "eot_min") {
                pts.push((e, de));
            }
        }
    }
    if pts.len() < 10 {
        return None;
    }
    pts.push(pts[0]);
    let d0 = decl(s, n0.round())?;
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let mut p = Plot::new(range(&xs)?, range(&ys)?, true, true);
    p.path(&pts, "axp-c");
    for (d, name) in [(1.0, "1 Jan"), (91.0, "1 Apr"), (182.0, "1 Jul"), (274.0, "1 Oct")] {
        if let (Some(a), Some(de)) = (s.with_input(&[("n", Input::Int(d as i64))]), decl(s, d)) {
            if let Some(e) = num_of(&a, "eot_min") {
                p.dot((e, de), "axp-d3", 4.0);
                p.label((e, de), 8.0, -6.0, "start", "axp-lt", name);
            }
        }
    }
    p.marker((e0, d0), &format!("day {}: {} min", num(n0), num(e0)));
    p.frame("Equation of time (min), Sun fast to the right", "Solar declination (°)", "");
    p.legend(&[("one point every 5 days".into(), "axp-c")]);
    Some(s.chart("chart", "The analemma: equation of time against declination",
                 &format!("The analemma: the model's equation of time (across, minutes) against its solar declination \
                           (up) for every fifth day of the year, where the Sun stands at clock noon through the year. \
                           Marked: day {}, equation of time {} min at declination {}°.", num(n0), num(e0), num(d0)),
                 &p.body()))
}

fn sun_path(s: &Solved) -> Option<Figure> {
    let (lat, dec0, ha0) = (nv(s, "latitude")?, nv(s, "declination")?, nv(s, "hour_angle")?);
    let (el0, az0, ze0) = (s.out("elevation_deg")?, s.out("azimuth_deg")?, s.out("zenith_deg")?);
    let mut days: Vec<(String, f64)> = vec![("June, δ 23.44°".into(), 23.44), ("equinox, δ 0°".into(), 0.0),
                                            ("December, δ -23.44°".into(), -23.44)];
    if days.iter().all(|d| (d.1 - dec0).abs() > 0.5) {
        days.push((format!("given, δ {}°", num(dec0)), dec0));
    }
    let mut series = vec![];
    for (name, d) in &days {
        let mut pts = vec![];
        for h in k::span(-180.0, 180.0, 49, false) {
            match s.with(&[("declination", *d), ("hour_angle", h)]) {
                Some(a) => match (num_of(&a, "azimuth_deg"), num_of(&a, "elevation_deg")) {
                    (Some(az), Some(el)) if el >= 0.0 => pts.push((az, el)),
                    _ => pts.push((f64::NAN, f64::NAN)),
                },
                None => pts.push((f64::NAN, f64::NAN)),
            }
        }
        if pts.iter().filter(|p| p.0.is_finite()).count() >= 2 {
            series.push((name.clone(), pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let body = k::lines(&series, "Azimuth (°, 180 = south)", "Elevation (°)", &format!("latitude {}°", num(lat)),
                        Some((az0, el0)), &format!("{}°", num(el0)), false);
    Some(s.chart("chart", "Sun-path chart: elevation against azimuth",
                 &format!("Sun-path chart at latitude {}°: elevation against azimuth on the solstices and the equinox{}, \
                           each point a run of the model for an hour angle every 7.5° (half an hour), only while the Sun \
                           is up. Marked: declination {}° at hour angle {}°, elevation {}°, azimuth {}°, zenith {}°.",
                          num(lat), if days.len() > 3 { " and at the declination given" } else { "" }, num(dec0), num(ha0),
                          num(el0), num(az0), num(ze0)), &body))
}

fn daylight(s: &Solved) -> Option<Figure> {
    let (lat, n0) = (nv(s, "latitude")?, nv(s, "n")?);
    let (rise0, set0, len0) = (s.out("sunrise_h")?, s.out("sunset_h")?, s.out("day_length_h")?);
    let other = if (lat - 52.0).abs() < 2.0 { 0.0 } else { 52.0 };
    let run = |la: f64| -> Vec<(f64, Option<(f64, f64)>)> {
        (1..=365).step_by(7).map(|d| {
            let d = d as f64;
            let v = s.with_input(&[("latitude", Input::Float(la)), ("n", Input::Int(d as i64))])
                .and_then(|a| Some((num_of(&a, "sunrise_h")?, num_of(&a, "sunset_h")?)));
            (d, v)
        }).collect()
    };
    let mine = run(lat);
    let theirs = run(other);
    let ok: Vec<(f64, (f64, f64))> = mine.iter().filter_map(|(d, v)| v.map(|v| (*d, v))).collect();
    if ok.len() < 4 {
        return None;
    }
    let mut p = Plot::new((1.0, 365.0), (0.0, 24.0), false, false);
    let rise: Vec<(f64, f64)> = mine.iter().map(|(d, v)| (*d, v.map(|x| x.0).unwrap_or(f64::NAN))).collect();
    let set: Vec<(f64, f64)> = mine.iter().map(|(d, v)| (*d, v.map(|x| x.1).unwrap_or(f64::NAN))).collect();
    p.area(&ok.iter().map(|x| (x.0, x.1 .1)).collect::<Vec<_>>(), &ok.iter().map(|x| (x.0, x.1 .0)).collect::<Vec<_>>(),
           "axp-s");
    p.path(&rise, "axp-c");
    p.path(&set, "axp-c");
    let orise: Vec<(f64, f64)> = theirs.iter().map(|(d, v)| (*d, v.map(|x| x.0).unwrap_or(f64::NAN))).collect();
    let oset: Vec<(f64, f64)> = theirs.iter().map(|(d, v)| (*d, v.map(|x| x.1).unwrap_or(f64::NAN))).collect();
    p.dashed(&orise, "axp-c3");
    p.dashed(&oset, "axp-c3");
    p.marker((n0, rise0), &format!("day {}: sunrise {} h ({} h of day)", num(n0), num(rise0), num(len0)));
    p.frame("Day of the year n", "Clock time (h)", "");
    p.legend(&[(format!("daylight at {}° (given)", num(lat)), "axp-s"), (format!("sunrise and sunset, {}°", num(other)), "axp-c3")]);
    Some(s.chart("chart", "Sunrise, sunset and day length through the year",
                 &format!("Sunrise and sunset in clock hours through the year at the longitude and time zone given, at the \
                           latitude given, {}° (shaded daylight), and at {}° (dashed), one run of the model per week; \
                           days the model refuses (no sunrise or no sunset) are left out. The wobble of both edges is the \
                           equation of time. Marked: day {}: sunrise {} h, sunset {} h, {} h of daylight.",
                          num(lat), num(other), num(n0), num(rise0), num(set0), num(len0)), &p.body()))
}

fn sky(s: &Solved) -> Option<Figure> {
    let (el0, az0) = (s.out("elevation_deg")?, s.out("azimuth_deg")?);
    let (lat, lon) = (nv(s, "latitude")?, nv(s, "longitude")?);
    let clock0 = nv(s, "clock")?;
    let h = 380.0;
    let (cx, cy, rr) = (260.0, 200.0, 140.0);
    let at = |az: f64, el: f64| -> (f64, f64) {
        let r = rr * (90.0 - el) / 90.0;
        let a = az.to_radians();
        (cx + r * a.sin(), cy - r * a.cos())
    };
    let mut o = String::new();
    o.push_str(&format!("<circle class=\"axp-f\" cx=\"{cx}\" cy=\"{cy}\" r=\"{rr}\"/>"));
    for e in [30.0, 60.0] {
        o.push_str(&format!("<circle class=\"axp-g\" fill=\"none\" cx=\"{cx}\" cy=\"{cy}\" r=\"{:.1}\"/>", rr * (90.0 - e) / 90.0));
    }
    for e in [0.0, 30.0, 60.0] {
        put(&mut o, cx + 4.0, cy - rr * (90.0 - e) / 90.0 + 14.0, "start", "axp-t", &format!("{}°", num(e)), h);
    }
    o.push_str(&format!("<line class=\"axp-g\" x1=\"{}\" y1=\"{cy}\" x2=\"{}\" y2=\"{cy}\"/><line class=\"axp-g\" x1=\"{cx}\" \
                         y1=\"{}\" x2=\"{cx}\" y2=\"{}\"/>", cx - rr, cx + rr, cy - rr, cy + rr));
    for (t, x, y) in [("N", cx, cy - rr - 8.0), ("S", cx, cy + rr + 18.0), ("E", cx + rr + 12.0, cy + 5.0),
                      ("W", cx - rr - 12.0, cy + 5.0)] {
        put(&mut o, x, y, "middle", "axp-bt", t, h);
    }
    let days = [(172.0, "21 June"), (81.0, "22 March"), (355.0, "21 December")];
    let mut tops = vec![];
    let mut drawn = 0;
    for (k2, (d, name)) in days.iter().enumerate() {
        let mut d_path = String::new();
        let mut pen = false;
        let mut top = f64::NEG_INFINITY;
        let mut dots = vec![];
        for i in 0..=72 {
            let t = i as f64 / 3.0;
            let a = s.with_input(&[("n", Input::Int(*d as i64)), ("clock", Input::Float(t))]);
            match a.and_then(|a| Some((num_of(&a, "azimuth_deg")?, num_of(&a, "elevation_deg")?))) {
                Some((az, el)) if el >= 0.0 => {
                    let (x, y) = at(az, el);
                    d_path.push_str(&format!("{}{x:.1},{y:.1} ", if pen { "L" } else { "M" }));
                    pen = true;
                    top = top.max(el);
                    if i % 6 == 0 {
                        dots.push((x, y, t));
                    }
                }
                _ => pen = false,
            }
        }
        if d_path.is_empty() {
            continue;
        }
        drawn += 1;
        o.push_str(&format!("<path class=\"{}\" d=\"{}\"/>", CL[k2], d_path.trim_end()));
        for (x, y, t) in &dots {
            o.push_str(&format!("<circle class=\"{}\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3\"/>", DOT[k2]));
            if k2 == 0 {
                put(&mut o, x + 6.0, y - 5.0, "start", "axp-lt", &format!("{} h", num(*t)), h);
            }
        }
        tops.push(format!("{name} {}°", num(top)));
    }
    if drawn == 0 {
        return None;
    }
    let (mx, my) = at(az0, el0.max(0.0));
    if el0 >= 0.0 {
        o.push_str(&format!("<circle class=\"axp-m\" cx=\"{mx:.1}\" cy=\"{my:.1}\" r=\"6\"/>"));
        put(&mut o, mx + 10.0, my + 18.0, "start", "axp-mt", &format!("{}°", num(el0)), h);
    }
    legend(&mut o, &days.iter().enumerate().map(|(i, d)| (d.1.to_string(), CL[i])).collect::<Vec<_>>(), 18.0);
    put(&mut o, 16.0, h - 8.0, "start", "axp-lt", "centre: overhead, rim: horizon", h);
    Some(s.figure("sky", "The Sun's path across the sky: polar sky chart",
                  &format!("The Sun's path across the sky at latitude {}°, longitude {}° on three days: azimuth around the \
                            rim (N at the top) and elevation inward (rings at 0°, 30° and 60°), computed by the model every \
                            20 minutes of clock time while the Sun is up; dots every 2 h. Highest computed elevations: {}. \
                            Marked: day {} at {} h, elevation {}°, azimuth {}°.", num(lat), num(lon), tops.join(", "),
                           num(nv(s, "n").unwrap_or(f64::NAN)), num(clock0), num(el0), num(az0)), &o, h))
}

// ------------------------------------------------------------------------------------------------ solar irradiance
fn poa(s: &Solved) -> Option<Figure> {
    let tilt = nv(s, "tilt")?;
    let (g0, b0, d0, r0, aoi) = (s.out("poa_global")?, s.out("poa_direct")?, s.out("poa_sky_diffuse")?,
                                 s.out("poa_ground_diffuse")?, s.out("aoi_deg")?);
    let mut rowsv = vec![];
    for t in sorted(k::span(0.0, 90.0, 19, false).into_iter().chain([tilt]).collect()) {
        if let Some(a) = s.with(&[("tilt", t)]) {
            if let (Some(g), Some(b), Some(d), Some(r)) = (num_of(&a, "poa_global"), num_of(&a, "poa_direct"),
                                                           num_of(&a, "poa_sky_diffuse"), num_of(&a, "poa_ground_diffuse")) {
                rowsv.push((t, b, d, r, g));
            }
        }
    }
    if rowsv.len() < 3 {
        return None;
    }
    let top = rowsv.iter().map(|x| x.4.max(x.1 + x.2 + x.3)).fold(0.0, f64::max).max(g0);
    let lo = rowsv.iter().map(|x| x.1.min(0.0)).fold(0.0, f64::min);
    let mut p = Plot::new((rowsv[0].0.min(0.0), rowsv[rowsv.len() - 1].0.max(90.0)), (lo, top * 1.1), false, false);
    let zero: Vec<(f64, f64)> = rowsv.iter().map(|x| (x.0, 0.0)).collect();
    let c1: Vec<(f64, f64)> = rowsv.iter().map(|x| (x.0, x.1)).collect();
    let c2: Vec<(f64, f64)> = rowsv.iter().map(|x| (x.0, x.1 + x.2)).collect();
    let c3: Vec<(f64, f64)> = rowsv.iter().map(|x| (x.0, x.1 + x.2 + x.3)).collect();
    p.area(&c1, &zero, "axp-s");
    p.area(&c2, &c1, "axp-s2");
    p.area(&c3, &c2, "axp-s3");
    p.path(&rowsv.iter().map(|x| (x.0, x.4)).collect::<Vec<_>>(), "axp-c4");
    p.marker((tilt, g0), &format!("{} W/m²", num(g0)));
    p.frame("Panel tilt from horizontal (°)", "Plane-of-array irradiance (W/m²)", "");
    p.legend(&[("direct".into(), "axp-s"), ("sky diffuse".into(), "axp-s2"), ("ground-reflected".into(), "axp-s3"),
               ("global (returned)".into(), "axp-c4")]);
    let (dni, ghi, dhi) = (nv(s, "dni").unwrap_or(f64::NAN), nv(s, "ghi").unwrap_or(f64::NAN), nv(s, "dhi").unwrap_or(f64::NAN));
    Some(s.chart("chart", "Plane-of-array irradiance by component against tilt",
                 &format!("Irradiance on a tilted panel split into its parts by the model for the sky given (DNI {}, GHI \
                           {}, DHI {} W/m²) at tilts from 0 to 90°, one run per tilt, stacked: direct, sky diffuse and \
                           ground-reflected, the line their returned total. Marked: tilt {}°, direct {}, sky {}, ground \
                           {}, global {} W/m² (angle of incidence {}°).", num(dni), num(ghi), num(dhi), num(tilt), num(b0),
                          num(d0), num(r0), num(g0), num(aoi)), &p.body()))
}

fn losses(s: &Solved) -> Option<Figure> {
    const L: [(&str, &str); 10] = [("soiling", "soiling"), ("shading", "shading"), ("snow", "snow"), ("mismatch", "mismatch"),
                                   ("wiring", "wiring"), ("connections", "connections"), ("lid", "light-induced"),
                                   ("nameplate_rating", "nameplate"), ("age", "age"), ("availability", "availability")];
    let pf = s.out("performance_factor")?;
    let total = s.out("total_loss_pct")?;
    let mut f = vec![];
    for kk in 0..=10 {
        let set: Vec<(&str, Input)> = L[kk..].iter().map(|x| (x.0, Input::Float(0.0))).collect();
        let a = if kk == 10 { None } else { s.with_input(&set) };
        let v = if kk == 10 { Some(pf) } else { a.and_then(|a| num_of(&a, "performance_factor")) };
        f.push(v?);
    }
    let lo = f.iter().cloned().fold(1.0, f64::min);
    let mut p = Plot::new((0.4, 11.6), (lo - (1.0 - lo) * 0.15 - 0.005, 1.0 + (1.0 - lo) * 0.08 + 0.002), false, false);
    for j in 1..=10 {
        let x = j as f64;
        let (a, b) = (f[j - 1], f[j]);
        p.area(&[(x - 0.36, a), (x + 0.36, a)], &[(x - 0.36, b), (x + 0.36, b)], "axp-b3");
        if j < 10 {
            p.dashed(&[(x + 0.36, b), (x + 0.64, b)], "axp-c4");
        }
        if (a - b).abs() > 1e-12 {
            p.label((x, b), 0.0, 14.0, "middle", "axp-vt", &format!("-{}%", num(((a - b) * 100.0 * 100.0).round() / 100.0)));
        }
    }
    p.area(&[(10.64, pf), (11.36, pf)], &[(10.64, p.y.lo), (11.36, p.y.lo)], "axp-b");
    p.label((11.0, pf), 0.0, -6.0, "middle", "axp-vt", &num(pf));
    let cats: Vec<(f64, String)> = L.iter().enumerate().map(|(i, x)| (i as f64 + 1.0, x.1.to_string()))
        .chain([(11.0, "result".to_string())]).collect();
    p.frame("", "Remaining fraction of DC output", &format!("total loss {} %", num(total)));
    cat_axis(&mut p, &cats, "");
    Some(s.chart("chart", "PV system losses: from nameplate to delivered",
                 &format!("System losses of a PV array (PVWatts loss model) for the losses given (defaults where none is \
                           given): each step adds one loss to those before it and the model returns the remaining \
                           performance factor; a step's drop (printed) is the difference of two returned factors. The \
                           losses multiply, so the total {} % is less than their sum. Marked: performance factor {}.",
                          num(total), num(pf)), &p.body()))
}

fn iv_pv(s: &Solved) -> Option<Figure> {
    let (il, i0, rs, rsh, nv) = (nv(s, "IL")?, nv(s, "I0")?, nv(s, "Rs")?, nv(s, "Rsh")?, nv(s, "nNsVth")?);
    let (isc, voc, imp, vmp, pmp, ff) = (s.out("Isc")?, s.out("Voc")?, s.out("Imp")?, s.out("Vmp")?, s.out("Pmp")?, s.out("FF")?);
    if !(voc > 0.0) {
        return None;
    }
    let mut iv = vec![];
    let mut pv = vec![];
    for v in sorted(k::span(0.0, voc, 41, false).into_iter().chain([vmp]).collect()) {
        let a = s.solve_other("energy_single_diode_current", &[("V", Input::Float(v)), ("IL", Input::Float(il)),
                                                               ("I0", Input::Float(i0)), ("Rs", Input::Float(rs)),
                                                               ("Rsh", Input::Float(rsh)), ("nNsVth", Input::Float(nv))]);
        if let Some(a) = a {
            if let (Some(i), Some(pw)) = (num_of(&a, "I"), num_of(&a, "P")) {
                iv.push((v, i));
                pv.push((v, pw));
            }
        }
    }
    if iv.len() < 3 {
        return None;
    }
    let top = crate::figures::curves(&[("I(V)".into(), iv)], "", "Current I (A)",
                                     &format!("Isc = {}, Voc = {}", q(isc, "A"), q(voc, "V")),
                                     Some(((vmp, imp), format!("MPP {}, {}", q(vmp, "V"), q(imp, "A")))), true)?;
    let bottom = crate::figures::curves(&[("P = V·I".into(), pv)], "Module voltage V (V)", "Power P (W)",
                                        &format!("fill factor {}", num(ff)),
                                        Some(((vmp, pmp), format!("Pmp = {}", q(pmp, "W")))), true)?;
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("curve-2", "PV module: I-V and P-V curves with the maximum power point",
                  &format!("Current-voltage (top) and power-voltage (bottom) curves of a PV module, single-diode model with \
                            the parameters given (IL = {}, I0 = {}, Rs = {}, Rsh = {}, nNsVth = {}); each point is the \
                            model's current and power at one of 41 voltages from 0 to Voc. Marked: the model's maximum \
                            power point Vmp = {}, Imp = {}, Pmp = {}, Isc = {}, Voc = {}, fill factor {}.",
                           q(il, "A"), q(i0, "A"), q(rs, "ohm"), q(rsh, "ohm"), q(nv, "V"), q(vmp, "V"), q(imp, "A"),
                           q(pmp, "W"), q(isc, "A"), q(voc, "V"), num(ff)), &body, h))
}

// ------------------------------------------------------------------------------------------------ hydrogen
fn hydrogen(s: &Solved) -> Option<Figure> {
    let (v, p) = (nv(s, "V")?, nv(s, "p")?);
    let m = s.out("mass")?;
    if !(v > 0.0 && p > 0.0) {
        return None;
    }
    let xs = k::span(v * 0.5, v * 1.5, 31, false);
    let fam = s.family("V", &xs, "mass", "p", &[p, p * 0.5, p * 2.0], "Pa");
    if fam.is_empty() {
        return None;
    }
    let body = k::lines(&fam, "Internal tank volume V (m³)", "Stored hydrogen mass (kg)", &format!("mass = {} kg", num(m)),
                        Some((v, m)), &format!("{} kg", num(m)), false);
    Some(s.chart("curve-2", "Hydrogen stored in a tank at pressure (real gas): mass against tank volume",
                 &format!("Stored hydrogen mass against the internal tank volume from {} to {} m³, one curve for each \
                           pressure (the one given, {}, half and twice it), 31 points per curve, each computed by the \
                           model (real gas); marked: the inputs given, V = {} m³, mass = {} kg.",
                          num(v * 0.5), num(v * 1.5), q(p, "Pa"), num(v), num(m)), &body))
}

// ------------------------------------------------------------------------------------------------ wind
fn power_curve(s: &Solved) -> Option<Figure> {
    let v = nv(s, "v")?;
    let pw = s.out("P")?;
    if !(v > 0.0) {
        return None;
    }
    let xs = sorted(k::span(v / 10.0, v * 10.0, 41, true).into_iter().chain([v]).collect());
    let pts = s.curve("v", &xs, "P", &[]);
    if pts.len() < 2 {
        return None;
    }
    let body = k::lines_y(&[("P".into(), pts)], "Wind speed at hub height v (m/s)", "Output P (W)", &eq("P", pw, "W"),
                          Some((v, pw)), &q(pw, "W"), true, true);
    Some(s.chart("curve-2", "Turbine output from its power curve: output against wind speed",
                 &format!("Turbine output against the wind speed at hub height from {} to {} m/s on a logarithmic axis, \
                           41 points each computed by the model from the power curve given; marked: v = {} m/s, P = {}.",
                          num(v / 10.0), num(v * 10.0), num(v), q(pw, "W")), &body))
}

fn power_curve_data(s: &Solved) -> Option<Figure> {
    let sp = s.vec("speeds")?;
    let pw = s.vec("powers")?;
    let v = nv(s, "v")?;
    let pk = s.out("P_kW")?;
    if sp.len() != pw.len() || sp.is_empty() || sp.len() > 200 {
        return None;
    }
    let xs: Vec<f64> = sp.iter().copied().chain([v, 0.0]).collect();
    let ys: Vec<f64> = pw.iter().copied().chain([pk, 0.0]).collect();
    let mut p = Plot::new(range(&xs)?, range(&ys)?, true, true);
    for (x, y) in sp.iter().zip(pw.iter()) {
        p.dot((*x, *y), "axp-d", 4.5);
    }
    p.marker((v, pk), &format!("{} kW", num(pk)));
    p.frame("Power-curve wind speeds (m/s)", "Power-curve outputs (kW)", &format!("v = {} m/s", num(v)));
    p.legend(&[("data (input)".into(), "axp-b"), ("returned".into(), "axp-b2")]);
    Some(s.chart("xy", "Turbine output from its power curve: the data and what was returned",
                 &format!("The power-curve outputs against the power-curve wind speeds as given ({} points), and the output \
                           the model returns at v = {} m/s: P_kW = {}, P = {}.", sp.len(), num(v), num(pk),
                          q(s.out("P").unwrap_or(f64::NAN), "W")), &p.body()))
}

fn weibull(s: &Solved) -> Option<Figure> {
    let (kk, c) = (nv(s, "k")?, nv(s, "c")?);
    let rho = nv(s, "rho").unwrap_or(1.225);
    let (vm, vp, ve, pd, epf) = (s.out("v_mean")?, s.out("v_most_probable")?, s.out("v_max_energy")?, s.out("power_density")?,
                                 s.out("energy_pattern_factor")?);
    if !(c > 0.0 && kk > 0.0) {
        return None;
    }
    let top = (ve * 1.6).max(c * 2.5);
    let n = 40;
    let hw = top / n as f64 * 0.5;
    // the share of time near v: the model's mean output for a unit triangular power curve around v, per m/s
    let share = |v: f64, peak: f64| -> Option<f64> {
        let a = s.solve_other("energy_weibull_aep", &[("speeds", Input::Json(json!([v - hw, v, v + hw]))),
                                                      ("powers", Input::Json(json!([0.0, peak, 0.0]))),
                                                      ("k", Input::Float(kk)), ("c", Input::Float(c))])?;
        Some(num_of(&a, "mean_power_kW")? / hw / peak)
    };
    let mut dens = vec![];
    let mut energy = vec![];
    for i in 1..=n {
        let v = top * i as f64 / n as f64;
        if let Some(f) = share(v, 1.0) {
            dens.push((v, f));
            let w = s.solve_other("energy_wind_power", &[("rho", Input::Float(rho)), ("D", Input::Float(1.0)),
                                                         ("v", Input::Float(v))])
                .and_then(|a| num_of(&a, "power_density"));
            if let Some(w) = w {
                energy.push((v, f * w));
            }
        }
    }
    if dens.len() < 5 {
        return None;
    }
    let dmax = dens.iter().map(|x| x.1).fold(0.0, f64::max);
    let emax = energy.iter().map(|x| x.1).fold(0.0, f64::max);
    if !(dmax > 0.0) {
        return None;
    }
    let energy: Vec<(f64, f64)> = if emax > 0.0 { energy.iter().map(|x| (x.0, x.1 / emax * dmax)).collect() } else { vec![] };
    let mut p = Plot::new((0.0, top), (0.0, dmax * 1.28), false, false);
    let zero: Vec<(f64, f64)> = dens.iter().map(|x| (x.0, 0.0)).collect();
    p.area(&dens, &zero, "axp-s");
    p.path(&dens, "axp-c");
    p.dashed(&energy, "axp-c2");
    for (i, (v, t, cl)) in [(vp, "most probable", "axp-c3"), (vm, "mean", "axp-c4"), (ve, "most energy", "axp-c2")].iter()
        .enumerate() {
        p.dashed(&[(*v, 0.0), (*v, dmax * 1.28)], cl);
        let (dx, anchor) = if i == 0 { (-4.0, "end") } else { (4.0, "start") };
        p.label((*v, dmax * 1.28), dx, 16.0 + 16.0 * (i.min(1)) as f64, anchor, "axp-vt", &format!("{t} {} m/s", num(*v)));
    }
    p.frame("Wind speed v (m/s)", "Probability density (s/m)", "");
    p.legend(&[(format!("Weibull density, k = {}, c = {} m/s", num(kk), num(c)), "axp-s"),
               ("energy share, scaled".into(), "axp-c2")]);
    Some(s.chart("distribution", "Weibull wind distribution with its key speeds",
                 &format!("Weibull wind-speed distribution with shape k = {} and scale c = {} m/s. The density is the \
                           model's: at each of {} speeds, its mean output for a narrow unit power curve centred there \
                           (the share of time near that speed); the dashed energy-share curve weights it by the model's \
                           wind power density at that speed (scaled to the same peak). The marked speeds are the model's: \
                           most probable {} m/s, mean {} m/s and the speed carrying the most energy {} m/s. It also returns \
                           the mean power density {} W/m² and the energy pattern factor {}.",
                          num(kk), num(c), dens.len(), num(vp), num(vm), num(ve), num(pd), num(epf)), &p.body()))
}

fn shear(s: &Solved) -> Option<Figure> {
    let (v1, h1, h2) = (nv(s, "v1")?, nv(s, "h1")?, nv(s, "h2")?);
    let v2 = s.out("v2")?;
    let hmax = h1.max(h2) * 1.5;
    let mut logp = vec![];
    let mut powp = vec![];
    for i in 1..=9 {
        let h = hmax * i as f64 / 9.0;
        if let Some(a) = s.with(&[("h2", h)]) {
            if let Some(v) = num_of(&a, "v2") {
                logp.push((h, v));
            }
        }
        let a = s.solve_other("energy_wind_shear_power_law", &[("v1", Input::Float(v1)), ("h1", Input::Float(h1)),
                                                               ("h2", Input::Float(h)), ("alpha", Input::Float(1.0 / 7.0))]);
        if let Some(v) = a.and_then(|a| num_of(&a, "v2")) {
            powp.push((h, v));
        }
    }
    if logp.len() < 2 {
        return None;
    }
    let vmax = logp.iter().chain(powp.iter()).map(|x| x.1).fold(v2.max(v1), f64::max);
    let mut p = Plot::new((0.0, vmax * 1.12), (0.0, hmax * 1.06), false, false);
    let gap = hmax * 0.014;
    for (h, v) in &logp {
        p.arrow((0.0, h + gap), (*v, h + gap), "axp-c", "axp-d");
    }
    for (h, v) in &powp {
        p.arrow((0.0, h - gap), (*v, h - gap), "axp-c3", "axp-d3");
    }
    p.dot((v1, h1), "axp-d4", 5.0);
    p.label((v1, h1), 8.0, 16.0, "start", "axp-lt", "measured");
    p.marker((v2, h2), &format!("{} m/s at {}", num(v2), q(h2, "m")));
    p.frame("Wind speed v (m/s)", "Height above ground h (m)", "");
    p.legend(&[("logarithmic law".into(), "axp-c"), ("power law, exponent 1/7".into(), "axp-c3")]);
    let at = |pts: &[(f64, f64)]| pts.last().map(|x| format!("{} m/s at {}", num(x.1), q(x.0, "m"))).unwrap_or_default();
    Some(s.chart("profile", "Wind speed with height: the shear profile as arrows",
                 &format!("Wind speed at nine heights extrapolated from {} m/s measured at {}, drawn as arrows whose length \
                           is the speed: the logarithmic law with the roughness given (this capability) and, for \
                           comparison, the power law with exponent 1/7, each computed by the model. At {} the model \
                           returns {} m/s; at the top: log law {}, power law {}.", num(v1), q(h1, "m"), q(h2, "m"),
                          num(v2), at(&logp), at(&powp)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ phasors
fn arc(p: &Plot, from_deg: f64, to_deg: f64, rpx: f64) -> String {
    let (ox, oy) = p.px((0.0, 0.0));
    let pt = |a: f64| (ox + rpx * a.to_radians().cos(), oy - rpx * a.to_radians().sin());
    let (a, b) = (pt(from_deg), pt(to_deg));
    let sweep = if to_deg > from_deg { 0 } else { 1 };
    format!("<path class=\"axp-c4\" d=\"M{:.1},{:.1} A{rpx},{rpx} 0 0 {sweep} {:.1},{:.1}\"/>", a.0, a.1, b.0, b.1)
}

fn generator_phasor(s: &Solved) -> Option<Figure> {
    let (il, pf) = (nv(s, "IL")?, nv(s, "pf")?);
    let (e, d, reg, pw, qv) = (s.out("E_phase")?, s.out("delta_deg")?, s.out("regulation_pct")?, s.out("P")?, s.out("Q")?);
    let el = s.out("E_line")?;
    // the terminal phase voltage as the returned E and regulation give it
    let v = e / (1.0 + reg / 100.0);
    if !(v > 0.0 && il > 0.0 && pf > 0.0 && pf <= 1.0) {
        return None;
    }
    let phi = pf.acos().to_degrees() * if qv < 0.0 { -1.0 } else { 1.0 };
    let et = (e * d.to_radians().cos(), e * d.to_radians().sin());
    let scale = 0.6 * v / il;
    let it = (il * scale * (-phi).to_radians().cos(), il * scale * (-phi).to_radians().sin());
    let mut p = Plot::equal(&[0.0, v, et.0, it.0], &[0.0, 0.0, et.1, it.1])?;
    p.arrow((0.0, 0.0), it, "axp-c4", "axp-d4");
    p.arrow((0.0, 0.0), (v, 0.0), "axp-c", "axp-d");
    p.arrow((v, 0.0), et, "axp-c3", "axp-d3");
    p.arrow((0.0, 0.0), et, "axp-c2", "axp-d2");
    let a1 = arc(&p, 0.0, d, 46.0);
    p.s.push_str(&a1);
    let mid = (d / 2.0).to_radians();
    let (ox, oy) = p.px((0.0, 0.0));
    put(&mut p.s, ox + 56.0 * mid.cos() + 4.0, oy - 56.0 * mid.sin() + 5.0, "start", "axp-vt", &format!("δ = {}°", num(d)), 340.0);
    if phi.abs() > 0.5 {
        let a2 = arc(&p, -phi, 0.0, 30.0);
        p.s.push_str(&a2);
    }
    p.label(it, 6.0, 16.0, "start", "axp-lt", &format!("I, φ = {}°", num(phi.abs())));
    p.label(et, 8.0, -6.0, "start", "axp-mt", &format!("E = {}", q(e, "V")));
    p.label((v * 0.55, 0.0), 0.0, -8.0, "middle", "axp-vt", &format!("V = {}", q(v, "V")));
    p.frame("Real part (V)", "Imaginary part (V)", "");
    let ra = nv(s, "Ra").unwrap_or(0.0);
    p.legend(&[("V (phase)".into(), "axp-c"), ("E (internal)".into(), "axp-c2"),
               (if ra > 0.0 { "(Ra + jXs)·I".into() } else { "jXs·I".to_string() }, "axp-c3"),
               (format!("I ({} V per A)", num(scale)), "axp-c4")]);
    Some(s.chart("phasor", "Synchronous generator phasor diagram",
                 &format!("Phasor diagram of the synchronous generator per phase: the terminal phase voltage V = {} \
                           (the reference, as the model's E and regulation give it), {} at power factor {} ({}), the \
                           current drawn at {} V per A. The model returns E = {} per phase ({} line) at load angle δ = {}°, \
                           regulation {} %, P = {}, Q = {}; the third side joins the tip of V to the tip of E.",
                          q(v, "V"), q(il, "A"), num(pf), if qv < 0.0 { "leading" } else { "lagging" }, num(scale),
                          q(e, "V"), q(el, "V"), num(d), num(reg), q(pw, "W"), q(qv, "var")), &p.body()))
}

fn power_triangle(s: &Solved) -> Option<Figure> {
    let (pw, qv, sa) = (s.out("P")?, s.out("Q")?, s.out("S")?);
    let (vph, iph) = (s.out("V_phase")?, s.out("I_phase")?);
    if !(sa > 0.0) {
        return None;
    }
    let (f, u) = if sa >= 1e6 { (1e-6, "M") } else if sa >= 1e3 { (1e-3, "k") } else { (1.0, "") };
    let (x, y) = (pw * f, qv * f);
    let mut p = Plot::equal(&[0.0, x], &[0.0, y])?;
    p.area(&[(0.0, 0.0), (x, y)], &[(x, 0.0)], "axp-s");
    p.arrow((0.0, 0.0), (x, 0.0), "axp-c", "axp-d");
    p.arrow((x, 0.0), (x, y), "axp-c3", "axp-d3");
    p.arrow((0.0, 0.0), (x, y), "axp-c2", "axp-d2");
    let phi = qv.atan2(pw).to_degrees();
    if phi.abs() > 0.5 {
        let a1 = arc(&p, 0.0, phi, 46.0);
        p.s.push_str(&a1);
    }
    let (ox, oy) = p.px((0.0, 0.0));
    let mid = (phi / 2.0).to_radians();
    put(&mut p.s, ox + 58.0 * mid.cos(), oy - 58.0 * mid.sin() + 5.0, "start", "axp-vt", &format!("φ = {}°", num(phi)), 340.0);
    p.label((x / 2.0, 0.0), 0.0, if y >= 0.0 { 18.0 } else { -8.0 }, "middle", "axp-vt", &format!("P = {}", q(pw, "W")));
    p.label((x, y / 2.0), 8.0, 4.0, "start", "axp-vt", &format!("Q = {}", q(qv, "var")));
    p.label((x / 2.0, y / 2.0), -10.0, if y >= 0.0 { -10.0 } else { 18.0 }, "end", "axp-vt", &format!("S = {}", q(sa, "VA")));
    p.frame(&format!("Real power P ({u}W)"), &format!("Reactive power Q ({u}var)"), "");
    Some(s.chart("phasor", "Three-phase power triangle",
                 &format!("Power triangle of the three-phase load: the model returns P = {}, Q = {} and S = {} (phase \
                           voltage {}, phase current {}); φ = {}° is the angle of the returned P and Q.", q(pw, "W"),
                          q(qv, "var"), q(sa, "VA"), q(vph, "V"), q(iph, "A"), num(phi)), &p.body()))
}
