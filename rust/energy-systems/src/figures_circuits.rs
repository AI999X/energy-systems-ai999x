//! Energy Systems' own live figures, part one: circuits, sequence networks, one-line diagrams and process blocks, each
//! labelled with the values the model computed for the visitor's inputs (curves beside them come from further solves of
//! the model). Written by hand for the Rust build.

use crate::figures::{fnum, nv, pair, panels, raw, rows, Fault};
use ax_engine::figkit::{self as k, eq, num, num_of, q, Canvas, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::Input;

const OHM: &str = "ohm";

/// A wire drawn in the accent colour (a connection the figure is about).
fn accent(c: &mut Canvas, pts: &[P]) {
    let d: String = pts.iter().enumerate().map(|(i, p)| format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1))
        .collect();
    c.raw(&format!("<path class=\"axp-c\" d=\"{}\"/>", d.trim_end()));
}

/// A bus bar (a thick vertical bar) centred at p.
fn bus(c: &mut Canvas, p: P, half: f64) {
    c.raw(&format!("<path class=\"axp-k\" style=\"stroke-width:5\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p.0, p.1 - half,
                   p.0, p.1 + half));
}

fn cells(n: f64) -> String {
    if n == 1.0 { "1 cell".into() } else { format!("{} cells", num(n)) }
}

fn kgh(x: f64) -> String {
    format!("{} kg/h", num(x * 3600.0))
}

// ------------------------------------------------------------------------------------------------ wind and storage
pub fn actuator_disc(s: &Solved) -> Option<Figure> {
    let (a, v) = (nv(s, "a")?, nv(s, "v")?);
    let (cp, ct, vd, vw, p, t) = (s.out("Cp")?, s.out("Ct")?, s.out("v_disc")?, s.out("v_wake")?, s.out("P")?, s.out("T")?);
    let mut c = Canvas::new(160.0);
    let cy = 78.0;
    // the stream tube widens behind the rotor as the air slows (a sketch: the speeds are the model's)
    let r = |x: f64| {
        let t = ((x - 60.0) / 400.0).clamp(0.0, 1.0);
        22.0 + 20.0 * t * t * (3.0 - 2.0 * t)
    };
    let xs: Vec<f64> = (0..=40).map(|i| 60.0 + 10.0 * i as f64).collect();
    let up: Vec<P> = xs.iter().map(|x| (*x, cy - r(*x))).collect();
    let lo: Vec<P> = xs.iter().map(|x| (*x, cy + r(*x))).collect();
    c.wire(&up);
    c.wire(&lo);
    c.raw(&format!("<path class=\"axp-c\" style=\"stroke-width:5\" d=\"M260,{:.1} L260,{:.1}\"/>", cy - r(260.0) - 8.0,
                   cy + r(260.0) + 8.0));
    c.bold(260.0, cy - r(260.0) - 14.0, "middle", "rotor disc");
    c.arrow((66.0, cy), (116.0, cy));
    c.value(64.0, cy - 8.0, "start", &eq("v", v, "m/s"));
    c.arrow((270.0, cy), (316.0, cy));
    c.value(270.0, cy + 18.0, "start", &eq("v_disc", vd, "m/s"));
    c.arrow((400.0, cy), (450.0, cy));
    c.value(510.0, cy - 8.0, "end", &eq("v_wake", vw, "m/s"));
    c.value(260.0, 150.0, "middle", &format!("P = {}, thrust {}", q(p, "W"), q(t, "N")));
    let mut cpp = vec![];
    let mut ctp = vec![];
    for x in k::span(0.02, 0.48, 24, false) {
        if let Some(b) = s.with(&[("a", x)]) {
            if let (Some(y1), Some(y2)) = (num_of(&b, "Cp"), num_of(&b, "Ct")) {
                cpp.push((x, y1));
                ctp.push((x, y2));
            }
        }
    }
    if cpp.len() < 2 {
        return None;
    }
    let chart = k::lines(&[("power coefficient Cp".into(), cpp), ("thrust coefficient Ct".into(), ctp)],
                         "Axial induction factor a", "Coefficient", &format!("Ct = {}", num(ct)), Some((a, cp)),
                         &format!("Cp = {}", num(cp)), false);
    let (body, h) = panels(&[(c.body().to_string(), 160.0), (chart, 340.0)]);
    Some(s.figure("schematic", "Actuator disc: stream tube and power coefficient",
                  &format!("Top: the stream tube through an ideal rotor of induction factor a = {} in a wind of {}, with \
                            the model's speeds at the disc ({}) and in the far wake ({}); the tube outline is a sketch. \
                            Bottom: the model's power and thrust coefficients for a from 0.02 to 0.48, one run per point. \
                            Marked: Cp = {}, Ct = {}, power {}, thrust {}.",
                           num(a), q(v, "m/s"), q(vd, "m/s"), q(vw, "m/s"), num(cp), num(ct), q(p, "W"), q(t, "N")),
                  &body, h))
}

pub fn battery_pack(s: &Solved) -> Option<Figure> {
    let (ns, np, nc) = (s.out("N_series")?, s.out("N_parallel")?, s.out("N_cells")?);
    let (vp, e, qa) = (s.out("V_pack")?, s.out("E_pack_kWh")?, s.out("Q_pack_Ah")?);
    if !(ns >= 1.0 && np >= 1.0 && ns < 1e7 && np < 1e7) {
        return None;
    }
    let (nsi, npi) = (ns.round() as usize, np.round() as usize);
    let cols = nsi.min(14);
    let rws = npi.min(6);
    let (cw, chh, step, rstep) = (22.0, 16.0, 28.0, 30.0);
    let width = step * cols as f64;
    let x0 = 260.0 - width / 2.0;
    let y0 = 52.0;
    let h = y0 + rstep * rws as f64 + 48.0;
    let mut c = Canvas::new(h);
    c.value(260.0, 30.0, "middle", &format!("{} cells in series = {}", num(ns), q(vp, "V")));
    let left = x0 - 12.0;
    let right = x0 + width + 4.0;
    for r in 0..rws {
        let y = y0 + rstep * r as f64;
        let ellipsis_row = npi > rws && r == rws - 2;
        if ellipsis_row {
            c.bold(260.0, y + 13.0, "middle", "...");
            continue;
        }
        c.line((left, y + chh / 2.0), (x0, y + chh / 2.0));
        c.line((x0 + width - step + cw, y + chh / 2.0), (right, y + chh / 2.0));
        for j in 0..cols {
            let x = x0 + step * j as f64;
            if nsi > cols && j == cols - 2 {
                c.bold(x + cw / 2.0, y + 13.0, "middle", "...");
            } else {
                c.raw(&format!("<rect class=\"axp-b2\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{cw}\" height=\"{chh}\"/>\
                                <rect class=\"axp-k\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{cw}\" height=\"{chh}\" rx=\"2\"/>"));
            }
            if j + 1 < cols {
                c.line((x + cw, y + chh / 2.0), (x + step, y + chh / 2.0));
            }
        }
    }
    let ylast = y0 + rstep * (rws as f64 - 1.0) + chh / 2.0;
    c.line((left, y0 + chh / 2.0), (left, ylast));
    c.line((right, y0 + chh / 2.0), (right, ylast));
    c.terminal((left, y0 - 6.0), "");
    c.line((left, y0 - 2.0), (left, y0 + chh / 2.0));
    c.terminal((right, y0 - 6.0), "");
    c.line((right, y0 - 2.0), (right, y0 + chh / 2.0));
    c.value(260.0, h - 24.0, "middle", &format!("{} strings in parallel", num(np)));
    c.value(260.0, h - 8.0, "middle", &format!("{} cells, {} kWh, {} Ah", num(nc), num(e), num(qa)));
    Some(s.figure("schematic", "Battery pack: series strings in parallel",
                  &format!("The pack the model sizes: each box is one cell and a row is one series string ({} cells in \
                            series, {} strings in parallel; long rows and many strings are shortened with dots). Marked: \
                            {} cells in all, nominal pack voltage {}, energy {} kWh, capacity {} Ah.",
                           num(ns), num(np), num(nc), q(vp, "V"), num(e), num(qa)), c.body(), h))
}

pub fn battery_rc(s: &Solved) -> Option<Figure> {
    let (ocv, r0, r1, c1, i) = (nv(s, "OCV")?, nv(s, "R0")?, nv(s, "R1")?, nv(s, "C1")?, nv(s, "I")?);
    let t0 = nv(s, "t").unwrap_or(0.0);
    let (vt, tau) = (s.out("V_terminal")?, s.out("tau")?);
    if !(tau > 0.0) {
        return None;
    }
    let mut c = Canvas::new(170.0);
    let (top, bot) = (60.0, 150.0);
    c.vsource((70.0, top), (70.0, bot), false);
    c.tag(94.0, 104.0, "start", "OCV", &q(ocv, "V"));
    c.line((70.0, top), (110.0, top));
    c.resistor((110.0, top), (190.0, top));
    c.value(150.0, top - 16.0, "middle", &eq("R0", r0, OHM));
    c.line((190.0, top), (220.0, top));
    c.wire(&[(220.0, top), (220.0, 38.0), (240.0, 38.0)]);
    c.resistor((240.0, 38.0), (310.0, 38.0));
    c.wire(&[(310.0, 38.0), (330.0, 38.0), (330.0, top)]);
    c.wire(&[(220.0, top), (220.0, 84.0), (265.0, 84.0)]);
    c.capacitor((265.0, 84.0), (285.0, 84.0));
    c.wire(&[(285.0, 84.0), (330.0, 84.0), (330.0, top)]);
    c.value(275.0, 20.0, "middle", &eq("R1", r1, OHM));
    c.value(275.0, 118.0, "middle", &eq("C1", c1, "F"));
    c.dot((220.0, top));
    c.dot((330.0, top));
    c.line((330.0, top), (430.0, top));
    c.terminal((434.0, top), "");
    c.line((70.0, bot), (430.0, bot));
    c.terminal((434.0, bot), "");
    c.bold(434.0, 102.0, "middle", "V");
    c.value(434.0, 120.0, "middle", &q(vt, "V"));
    let ts = k::span(0.0, 5.0 * tau, 41, false);
    let mut pts = s.curve("t", &ts, "V_terminal", &[]);
    if pts.len() < 2 {
        return None;
    }
    if !pts.iter().any(|p| p.0 == t0) {
        pts.push((t0, vt));
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    }
    let chart = k::lines(&[(format!("V(t) at I = {}", q(i, "A")), pts), ("OCV".into(), vec![(0.0, ocv), (5.0 * tau, ocv)])],
                         "Time since the start of discharge t (s)", "Terminal voltage (V)",
                         &format!("τ = {}", q(tau, "s")), Some((t0, vt)), &eq("V", vt, "V"), false);
    let (body, h) = panels(&[(c.body().to_string(), 170.0), (chart, 340.0)]);
    Some(s.figure("schematic", "Battery RC model: circuit and discharge voltage",
                  &format!("Top: the one-RC battery model with the values given (OCV = {}, R0 = {}, R1 = {}, C1 = {}). \
                            Bottom: the model's terminal voltage during a constant {} discharge, one run per time from 0 \
                            to 5τ, τ = {} as returned. Marked: at t = {} the model returns {}.",
                           q(ocv, "V"), q(r0, OHM), q(r1, OHM), q(c1, "F"), q(i, "A"), q(tau, "s"), q(t0, "s"),
                           q(vt, "V")), &body, h))
}

// ------------------------------------------------------------------------------------------------ faults
pub fn fault(s: &Solved, kind: Fault) -> Option<Figure> {
    let xs = [nv(s, "X1")?, nv(s, "X2")?, nv(s, "X0")?];
    let seq = [s.out("I1")?, s.out("I2")?, s.out("I0")?];
    let phase: Vec<f64> = ["Ia", "Ib", "Ic", "In", "Va", "Vb", "Vc"].iter().map(|n| s.out(n)).collect::<Option<Vec<f64>>>()?;
    let mut c = Canvas::new(190.0);
    let cols = [110.0, 260.0, 410.0];
    let (ft, nb) = (46.0, 150.0);
    let names = ["positive", "negative", "zero"];
    for (j, x) in cols.iter().enumerate() {
        let x = *x;
        if j == 0 {
            c.inductor((x, ft), (x, 100.0));
            c.vsource((x, 104.0), (x, nb), false);
            c.line((x, 100.0), (x, 104.0));
        } else {
            c.inductor((x, ft), (x, nb));
        }
        c.terminal((x, ft), "");
        c.terminal((x, nb), "");
        c.name(x - 9.0, ft + 4.0, "end", &format!("F{}", ["1", "2", "0"][j]));
        c.name(x - 9.0, nb + 4.0, "end", &format!("N{}", ["1", "2", "0"][j]));
        c.bold(x + 14.0, 82.0, "start", names[j]);
        c.value(x + 14.0, 100.0, "start", &format!("X{} = {}", ["1", "2", "0"][j], num(xs[j])));
        c.value(x + 14.0, 116.0, "start", &format!("I{} = {}", ["1", "2", "0"][j], num(seq[j])));
    }
    let (title, how) = match kind {
        Fault::Slg => {
            accent(&mut c, &[(110.0, ft - 4.0), (110.0, 26.0), (225.0, 26.0), (225.0, 172.0), (260.0, 172.0), (260.0, nb + 4.0)]);
            accent(&mut c, &[(260.0, ft - 4.0), (260.0, 26.0), (375.0, 26.0), (375.0, 172.0), (410.0, 172.0), (410.0, nb + 4.0)]);
            accent(&mut c, &[(410.0, ft - 4.0), (410.0, 12.0), (40.0, 12.0), (40.0, 180.0), (110.0, 180.0), (110.0, nb + 4.0)]);
            ("Single line-to-ground fault: sequence networks and currents",
             "the sequence networks connected in series (the same current flows through all three)")
        }
        Fault::Dlg => {
            accent(&mut c, &[(110.0, ft - 4.0), (110.0, 26.0), (410.0, 26.0), (410.0, ft - 4.0)]);
            accent(&mut c, &[(260.0, ft - 4.0), (260.0, 26.0)]);
            accent(&mut c, &[(110.0, nb + 4.0), (110.0, 172.0), (410.0, 172.0), (410.0, nb + 4.0)]);
            accent(&mut c, &[(260.0, nb + 4.0), (260.0, 172.0)]);
            ("Double line-to-ground fault: sequence networks and currents",
             "the sequence networks connected all three in parallel")
        }
        Fault::Ll => {
            accent(&mut c, &[(110.0, ft - 4.0), (110.0, 26.0), (260.0, 26.0), (260.0, ft - 4.0)]);
            accent(&mut c, &[(110.0, nb + 4.0), (110.0, 172.0), (260.0, 172.0), (260.0, nb + 4.0)]);
            ("Line-to-line fault: sequence networks and currents",
             "the positive and negative sequence networks connected in parallel; the zero-sequence network stays open")
        }
    };
    let labels: Vec<String> = ["Ia", "Ib", "Ic", "In", "Va", "Vb", "Vc"].iter().map(|x| x.to_string()).collect();
    let cur: Vec<Option<f64>> = (0..7).map(|i| (i < 4).then_some(phase[i])).collect();
    let vol: Vec<Option<f64>> = (0..7).map(|i| (i >= 4).then_some(phase[i])).collect();
    let chart = k::bar_groups(&labels, &[("fault currents (pu)".into(), cur), ("phase voltages (pu)".into(), vol)], "Phase",
                              "Magnitude (pu)", "");
    let (body, h) = panels(&[(c.body().to_string(), 190.0), (chart, 340.0)]);
    Some(s.figure("schematic", title,
                  &format!("Top: {how}, with the Thevenin reactances given (X1 = {}, X2 = {}, X0 = {} pu) and the model's \
                            sequence currents I1 = {}, I2 = {}, I0 = {} pu. Bottom: the model's fault currents Ia = {}, \
                            Ib = {}, Ic = {}, In = {} pu and phase voltages Va = {}, Vb = {}, Vc = {} pu.",
                           num(xs[0]), num(xs[1]), num(xs[2]), num(seq[0]), num(seq[1]), num(seq[2]), num(phase[0]),
                           num(phase[1]), num(phase[2]), num(phase[3]), num(phase[4]), num(phase[5]), num(phase[6])),
                  &body, h))
}

// ------------------------------------------------------------------------------------------------ hydrogen
pub fn electrolyser(s: &Solved) -> Option<Figure> {
    let (i, n, v) = (nv(s, "I")?, nv(s, "N")?, nv(s, "V_cell")?);
    let (h2, o2, w) = (s.out("H2_kg_s")?, s.out("O2_kg_s")?, s.out("H2O_kg_s")?);
    let (se, hhv, lhv) = (s.out("specific_energy_kWh_per_kg")?, s.out("efficiency_HHV")?, s.out("efficiency_LHV")?);
    let mut c = Canvas::new(210.0);
    c.block(190.0, 76.0, 140.0, 60.0, "electrolyser");
    c.arrow((40.0, 106.0), (188.0, 106.0));
    c.value(40.0, 98.0, "start", &format!("water {}", kgh(w)));
    c.value(260.0, 154.0, "middle", &format!("{}, {}, {} each", cells(n), q(i, "A"), q(v, "V")));
    c.arrow((332.0, 92.0), (470.0, 46.0));
    c.value(500.0, 36.0, "end", &format!("H₂ {}", kgh(h2)));
    c.arrow((332.0, 120.0), (470.0, 166.0));
    c.value(500.0, 184.0, "end", &format!("O₂ {}", kgh(o2)));
    c.value(260.0, 204.0, "middle", &format!("{} kWh per kg of H₂, efficiency {} (HHV), {} (LHV)", num(se), num(hhv),
                                             num(lhv)));
    Some(s.figure("schematic", "Electrolyser: water in, hydrogen and oxygen out",
                  &format!("Mass balance of the water electrolyser as the model returns it: water in {}, hydrogen {} and \
                            oxygen {} out (kg/s times 3600). Marked: {} kWh of electricity per kg of hydrogen and \
                            efficiencies {} on the higher and {} on the lower heating value.",
                           kgh(w), kgh(h2), kgh(o2), num(se), num(hhv), num(lhv)), c.body(), 210.0))
}

pub fn fuel_cell(s: &Solved) -> Option<Figure> {
    let (i, n, v) = (nv(s, "I")?, nv(s, "N")?, nv(s, "V_cell")?);
    let (h2, o2, w) = (s.out("H2_kg_s")?, s.out("O2_kg_s")?, s.out("H2O_kg_s")?);
    let (heat, lhv, hhv) = (s.out("heat_LHV")?, s.out("efficiency_LHV")?, s.out("efficiency_HHV")?);
    let mut c = Canvas::new(210.0);
    c.block(190.0, 76.0, 140.0, 60.0, "fuel cell");
    c.arrow((40.0, 50.0), (188.0, 92.0));
    c.value(40.0, 40.0, "start", &format!("H₂ {}", kgh(h2)));
    c.arrow((40.0, 166.0), (188.0, 122.0));
    c.value(40.0, 186.0, "start", &format!("O₂ {}", kgh(o2)));
    c.arrow((332.0, 106.0), (480.0, 106.0));
    c.value(500.0, 98.0, "end", &format!("water {}", kgh(w)));
    c.arrow((260.0, 74.0), (260.0, 34.0));
    c.value(270.0, 30.0, "start", &format!("heat {}", q(heat, "W")));
    c.value(260.0, 154.0, "middle", &format!("{}, {}, {} each", cells(n), q(i, "A"), q(v, "V")));
    c.value(500.0, 204.0, "end", &format!("efficiency {} (LHV), {} (HHV)", num(lhv), num(hhv)));
    Some(s.figure("schematic", "Fuel cell: hydrogen and oxygen in, water and heat out",
                  &format!("Mass and energy balance of the hydrogen fuel cell as the model returns it: hydrogen {} and \
                            oxygen {} in, water {} out (kg/s times 3600). Marked: heat released {} and efficiencies {} on \
                            the lower and {} on the higher heating value.",
                           kgh(h2), kgh(o2), kgh(w), q(heat, "W"), num(lhv), num(hhv)), c.body(), 210.0))
}

// ------------------------------------------------------------------------------------------------ line geometry
pub fn spacing(s: &Solved, params: bool) -> Option<Figure> {
    let (dab, dbc, dca) = (nv(s, "D_ab")?, nv(s, "D_bc")?, nv(s, "D_ca")?);
    if !(dab > 0.0 && dbc > 0.0 && dca > 0.0) {
        return None;
    }
    // the third conductor from the three spacings (a flat row when one spacing is the sum of the other two)
    let cx = (dab * dab + dca * dca - dbc * dbc) / (2.0 * dab);
    let h2 = dca * dca - cx * cx;
    let scale_all = dab.max(dbc).max(dca);
    if h2 < -1e-9 * scale_all * scale_all {
        return None;
    }
    let cy = h2.max(0.0).sqrt();
    let (xl, xh) = (cx.min(0.0), cx.max(dab));
    let sc = (380.0 / (xh - xl)).min(if cy > 0.0 { 110.0 / cy } else { f64::INFINITY });
    let base = if cy > 0.0 { 150.0 } else { 100.0 };
    let off = 260.0 - (xl + xh) / 2.0 * sc;
    let pt = |x: f64, y: f64| -> P { (off + x * sc, base - y * sc) };
    let (pa, pb, pc) = (pt(0.0, 0.0), pt(dab, 0.0), pt(cx, cy));
    let h = if params { 250.0 } else { 220.0 };
    let mut c = Canvas::new(h);
    let flat = cy * sc < 4.0;
    for (p, q2, name, val) in [(pa, pb, "D_ab", dab), (pb, pc, "D_bc", dbc), (pc, pa, "D_ca", dca)] {
        c.dashed(p, q2);
        let m = ((p.0 + q2.0) / 2.0, (p.1 + q2.1) / 2.0);
        let t = format!("{name} = {}", q(val, "m"));
        if flat {
            let below = name != "D_ca";
            c.name(m.0, if below { m.1 + 24.0 } else { m.1 + 44.0 }, "middle", &t);
        } else {
            // outward from the triangle's centre
            let g = ((pa.0 + pb.0 + pc.0) / 3.0, (pa.1 + pb.1 + pc.1) / 3.0);
            let (dx, dy) = (m.0 - g.0, m.1 - g.1);
            let d = (dx * dx + dy * dy).sqrt().max(1e-9);
            let lp = (m.0 + dx / d * 22.0, m.1 + dy / d * 22.0 + 4.0);
            let anchor = if dx / d > 0.3 { "start" } else if dx / d < -0.3 { "end" } else { "middle" };
            c.name(lp.0, lp.1, anchor, &t);
        }
    }
    for (p, n) in [(pa, "a"), (pb, "b"), (pc, "c")] {
        c.wire_end(p, true);
        c.bold(p.0, p.1 - 16.0, "middle", n);
    }
    let (title, caption);
    if params {
        let (l, x, cc, b, zc) = (s.out("L_mH_per_km")?, s.out("X_ohm_per_km")?, s.out("C_nF_per_km")?,
                                 s.out("B_uS_per_km")?, s.out("Zc")?);
        c.value(260.0, h - 32.0, "middle", &format!("L = {} mH/km, X = {} Ω/km", num(l), num(x)));
        c.value(260.0, h - 12.0, "middle", &format!("C = {} nF/km, B = {} µS/km, Zc = {}", num(cc), num(b), q(zc, OHM)));
        title = "Overhead line geometry and its parameters";
        caption = format!("The three-phase arrangement drawn to scale from the spacings given ({}, {} and {}). Marked: \
                           the model's per-phase inductance {} mH/km, reactance {} Ω/km, capacitance {} nF/km, \
                           susceptance {} µS/km and surge impedance {}.", q(dab, "m"), q(dbc, "m"), q(dca, "m"), num(l),
                          num(x), num(cc), num(b), q(zc, OHM));
    } else {
        let (gmd, ratio) = (s.out("GMD")?, s.out("ratio_to_mean")?);
        c.value(260.0, h - 20.0, "middle", &format!("GMD = {} ({} times the mean spacing)", q(gmd, "m"), num(ratio)));
        title = "Conductor spacing and geometric mean distance";
        caption = format!("Phase conductors drawn to scale from the spacings given, D_ab = {}, D_bc = {}, D_ca = {}{}. \
                           Marked: the model's geometric mean distance GMD = {}, {} of the arithmetic mean spacing.",
                          q(dab, "m"), q(dbc, "m"), q(dca, "m"),
                          if flat { " (a flat row: one spacing is the sum of the other two)" } else { "" }, q(gmd, "m"),
                          num(ratio));
    }
    Some(s.figure("schematic", title, &caption, c.body(), h))
}

// ------------------------------------------------------------------------------------------------ machines
pub fn induction_motor(s: &Solved) -> Option<Figure> {
    let (r1, x1, r2, x2, xm) = (nv(s, "R1")?, nv(s, "X1")?, nv(s, "R2")?, nv(s, "X2")?, nv(s, "Xm")?);
    let (vl, slip) = (nv(s, "VL")?, nv(s, "slip")?);
    let (il, pf, tq, pag, pout, eff) = (s.out("I_line")?, s.out("pf")?, s.out("T")?, s.out("P_ag")?, s.out("P_out")?,
                                        s.out("efficiency_pct")?);
    let mut c = Canvas::new(170.0);
    let (top, bot) = (50.0, 136.0);
    c.terminal((40.0, top), "");
    c.terminal((40.0, bot), "");
    c.bold(40.0, 98.0, "middle", "V");
    c.line((44.0, top), (60.0, top));
    c.resistor((60.0, top), (130.0, top));
    c.inductor((140.0, top), (210.0, top));
    c.line((130.0, top), (140.0, top));
    c.line((210.0, top), (250.0, top));
    c.inductor((230.0, top), (230.0, bot));
    c.dot((230.0, top));
    c.dot((230.0, bot));
    c.inductor((250.0, top), (330.0, top));
    c.line((330.0, top), (420.0, top));
    c.resistor((420.0, top), (420.0, bot));
    c.line((44.0, bot), (420.0, bot));
    c.value(95.0, 30.0, "middle", &eq("R1", r1, OHM));
    c.value(175.0, 30.0, "middle", &eq("X1", x1, OHM));
    c.value(290.0, 30.0, "middle", &eq("X2", x2, OHM));
    c.tag(242.0, 90.0, "start", "Xm", &q(xm, OHM));
    c.name(408.0, 82.0, "end", "R2/s");
    c.value(408.0, 98.0, "end", &eq("R2", r2, OHM));
    c.value(408.0, 114.0, "end", &format!("s = {}", num(slip)));
    c.value(260.0, 160.0, "middle", &format!("line current {}, power factor {}", q(il, "A"), num(pf)));
    let mut xs = k::span(0.02, 1.0, 40, false);
    xs.push(slip);
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let pts = s.curve("slip", &xs, "T", &[]);
    if pts.len() < 2 {
        return None;
    }
    let chart = k::lines(&[("torque against slip, one run per slip".into(), pts)], "Slip s", "Torque T (N·m)",
                         &format!("{} at {}", q(vl, "V"), q(nv(s, "f").unwrap_or(f64::NAN), "Hz")),
                         Some((slip, tq)), &format!("s = {}: {} N·m", num(slip), num(tq)), false);
    let (body, h) = panels(&[(c.body().to_string(), 170.0), (chart, 340.0)]);
    Some(s.figure("circuit", "Induction motor: equivalent circuit and torque against slip",
                  &format!("Top: the per-phase equivalent circuit with the values given (R1 = {}, X1 = {}, R2 = {}, X2 = {}, \
                            Xm = {}), the rotor resistance appearing as R2/s at slip {}. Bottom: the model's torque \
                            against slip from 0.02 to standstill (s = 1), one run per point. Marked: at s = {} the model \
                            returns T = {} N·m, line current {}, power factor {}, air-gap power {}, output {}, efficiency \
                            {} %.", q(r1, OHM), q(x1, OHM), q(r2, OHM), q(x2, OHM), q(xm, OHM), num(slip), num(slip),
                           num(tq), q(il, "A"), num(pf), q(pag, "W"), q(pout, "W"), num(eff)), &body, h))
}

// ------------------------------------------------------------------------------------------------ lines
pub fn line_model(s: &Solved, medium: bool) -> Option<Figure> {
    let (vr, p, pf, r, x) = (nv(s, "Vr")?, nv(s, "P")?, nv(s, "pf")?, nv(s, "R")?, nv(s, "X")?);
    let (vs, d, reg, eff, loss) = (s.out("Vs")?, s.out("delta_deg")?, s.out("regulation_pct")?, s.out("efficiency_pct")?,
                                   s.out("losses")?);
    let cur = if medium { s.out("Is")? } else { s.out("I")? };
    let mut c = Canvas::new(180.0);
    let (top, bot) = (66.0, 136.0);
    c.terminal((40.0, top), "");
    c.terminal((40.0, bot), "");
    c.terminal((480.0, top), "");
    c.terminal((480.0, bot), "");
    c.bold(52.0, 106.0, "start", "Vs");
    c.bold(468.0, 106.0, "end", "Vr");
    c.line((44.0, top), (160.0, top));
    c.resistor((160.0, top), (240.0, top));
    c.line((240.0, top), (270.0, top));
    c.inductor((270.0, top), (350.0, top));
    c.line((350.0, top), (476.0, top));
    c.line((44.0, bot), (476.0, bot));
    c.value(200.0, top - 16.0, "middle", &eq("R", r, OHM));
    c.value(310.0, top - 16.0, "middle", &eq("X", x, OHM));
    let mut caption = String::new();
    if medium {
        let b = nv(s, "B")?;
        for xx in [100.0, 420.0] {
            c.capacitor((xx, top), (xx, bot));
            c.dot((xx, top));
            c.dot((xx, bot));
            if xx < 260.0 {
                c.name(xx + 20.0, 96.0, "start", "B/2");
            } else {
                c.name(xx - 20.0, 96.0, "end", "B/2");
            }
        }
        let (am, aa, bm, ba) = (s.out("A_mag")?, s.out("A_angle_deg")?, s.out("B_mag")?, s.out("B_angle_deg")?);
        c.value(260.0, 34.0, "middle", &format!("A = {}∠{}°, B = {}∠{}° Ω", num(am), num(aa), num(bm), num(ba)));
        caption.push_str(&format!("Nominal-π model of a medium line: series {} + j{}, half the shunt susceptance ({} in \
                                   all) at each end", q(r, OHM), q(x, OHM), q(b, "S")));
        caption.push_str(&format!(", delivering {} at power factor {} and {}. The model returns the sending voltage {} at \
                                   {}°, current {}, losses {}, regulation {} %, efficiency {} %, and the constants A = {}∠{}°, \
                                   B = {}∠{}° Ω.", q(p, "W"), num(pf), q(vr, "V"), q(vs, "V"), num(d), q(cur, "A"),
                                  q(loss, "W"), num(reg), num(eff), num(am), num(aa), num(bm), num(ba)));
    } else {
        caption.push_str(&format!("Short-line model: a series impedance {} + j{} carrying {} at power factor {} to a {} \
                                   receiving end. The model returns the sending voltage {} at {}°, current {}, losses {}, \
                                   regulation {} % and efficiency {} %.", q(r, OHM), q(x, OHM), q(p, "W"), num(pf),
                                  q(vr, "V"), q(vs, "V"), num(d), q(cur, "A"), q(loss, "W"), num(reg), num(eff)));
    }
    c.value(260.0, 16.0, "middle", &format!("I = {}, regulation {} %, efficiency {} %", q(cur, "A"), num(reg), num(eff)));
    c.value(40.0, 162.0, "start", &format!("Vs = {} ∠ {}°", q(vs, "V"), num(d)));
    c.name(500.0, 162.0, "end", &format!("Vr = {}, {}, pf {}", q(vr, "V"), q(p, "W"), num(pf)));
    let kind_title = if medium { "Medium transmission line: nominal-π model" } else { "Short transmission line: series impedance" };
    Some(s.figure("circuit", kind_title, &caption, c.body(), 180.0))
}

// ------------------------------------------------------------------------------------------------ photovoltaics
pub fn single_diode(s: &Solved) -> Option<Figure> {
    let (v, il, i0, rs, rsh, nv) = (nv(s, "V")?, nv(s, "IL")?, nv(s, "I0")?, nv(s, "Rs")?, nv(s, "Rsh")?, nv(s, "nNsVth")?);
    let (i, p) = (s.out("I")?, s.out("P")?);
    let mpp = s.solve_other("energy_single_diode_mpp", &[("IL", Input::Float(il)), ("I0", Input::Float(i0)),
                                                          ("Rs", Input::Float(rs)), ("Rsh", Input::Float(rsh)),
                                                          ("nNsVth", Input::Float(nv))])?;
    let voc = num_of(&mpp, "Voc")?;
    if !(voc > 0.0) {
        return None;
    }
    let mut c = Canvas::new(180.0);
    let (top, bot) = (50.0, 140.0);
    c.isource((70.0, bot), (70.0, top));
    c.tag(94.0, 96.0, "start", "IL", &q(il, "A"));
    c.diode((160.0, top), (160.0, bot), false);
    c.name(176.0, 100.0, "start", "D");
    c.resistor((240.0, top), (240.0, bot));
    c.tag(254.0, 96.0, "start", "Rsh", &q(rsh, OHM));
    c.line((70.0, top), (320.0, top));
    c.resistor((320.0, top), (400.0, top));
    c.value(360.0, top - 18.0, "middle", &eq("Rs", rs, OHM));
    c.line((400.0, top), (456.0, top));
    c.line((70.0, bot), (456.0, bot));
    for x in [160.0, 240.0] {
        c.dot((x, top));
        c.dot((x, bot));
    }
    c.terminal((460.0, top), "");
    c.terminal((460.0, bot), "");
    c.bold(460.0, 100.0, "middle", "V");
    c.value(260.0, 168.0, "middle", &format!("I0 = {}, nNsVth = {}", q(i0, "A"), q(nv, "V")));
    let mut xs = k::span(0.0, voc, 41, false);
    xs.push(v);
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let pts = s.curve("V", &xs, "I", &[]);
    if pts.len() < 2 {
        return None;
    }
    let chart = crate::figures::curves(&[("I(V), one run per voltage".into(), pts)], "Module voltage V (V)",
                                       "Current I (A)", &format!("Voc = {}", q(voc, "V")),
                                       Some(((v, i), format!("{}, {}", q(i, "A"), q(p, "W")))), true)?;
    let (body, h) = panels(&[(c.body().to_string(), 180.0), (chart, 340.0)]);
    Some(s.figure("schematic", "Single-diode PV model: equivalent circuit and current",
                  &format!("Top: the single-diode equivalent circuit (photocurrent source, diode, shunt and series \
                            resistance) with the values given (IL = {}, I0 = {}, Rs = {}, Rsh = {}, nNsVth = {}). Bottom: \
                            the model's current at 41 voltages from 0 to the open-circuit voltage {} (also the model's). \
                            Marked: at V = {} the model returns I = {} and P = {}.",
                           q(il, "A"), q(i0, "A"), q(rs, OHM), q(rsh, OHM), q(nv, "V"), q(voc, "V"), q(v, "V"),
                           q(i, "A"), q(p, "W")), &body, h))
}

// ------------------------------------------------------------------------------------------------ transformers
fn tx_circuit(c: &mut Canvas, rc: &str, xm: &str, req: &str, xeq: &str) {
    let (top, bot) = (50.0, 126.0);
    c.terminal((40.0, top), "");
    c.terminal((40.0, bot), "");
    c.bold(54.0, 94.0, "start", "V1");
    c.line((44.0, top), (230.0, top));
    c.resistor((110.0, top), (110.0, bot));
    c.inductor((180.0, top), (180.0, bot));
    for x in [110.0, 180.0] {
        c.dot((x, top));
        c.dot((x, bot));
    }
    c.tag(122.0, 90.0, "start", "Rc", rc);
    c.tag(192.0, 90.0, "start", "Xm", xm);
    c.resistor((230.0, top), (300.0, top));
    c.line((300.0, top), (320.0, top));
    c.inductor((320.0, top), (390.0, top));
    c.line((390.0, top), (466.0, top));
    c.line((44.0, bot), (466.0, bot));
    c.terminal((470.0, top), "");
    c.terminal((470.0, bot), "");
    c.bold(458.0, 94.0, "end", "V2'");
    c.value(265.0, 30.0, "middle", &format!("Req = {req}"));
    c.value(355.0, 16.0, "middle", &format!("Xeq = {xeq}"));
}

pub fn transformer_tests(s: &Solved) -> Option<Figure> {
    let (rc, xm, req, xeq, zeq) = (s.out("Rc")?, s.out("Xm")?, s.out("Req")?, s.out("Xeq")?, s.out("Zeq")?);
    let (pfo, pfs) = (s.out("pf_oc")?, s.out("pf_sc")?);
    let mut c = Canvas::new(170.0);
    tx_circuit(&mut c, &q(rc, OHM), &q(xm, OHM), &q(req, OHM), &q(xeq, OHM));
    c.value(260.0, 156.0, "middle", &format!("|Zeq| = {}, power factors {} (open) and {} (short)", q(zeq, OHM), num(pfo),
                                             num(pfs)));
    let t = |n: &str, u: &str| nv(s, n).map(|x| q(x, u)).unwrap_or_default();
    Some(s.figure("circuit", "Transformer equivalent circuit from its tests",
                  &format!("Approximate equivalent circuit of the transformer from its two tests (open circuit {}, {}, {}; \
                            short circuit {}, {}, {}), referred to the side the tests were made on. Marked: the model's \
                            core-loss resistance Rc = {} and magnetising reactance Xm = {} (shunt), series Req = {} and \
                            Xeq = {} (|Zeq| = {}); power factors {} and {}.",
                           t("Voc", "V"), t("Ioc", "A"), t("Poc", "W"), t("Vsc", "V"), t("Isc", "A"), t("Psc", "W"),
                           q(rc, OHM), q(xm, OHM), q(req, OHM), q(xeq, OHM), q(zeq, OHM), num(pfo), num(pfs)),
                  c.body(), 170.0))
}

pub fn transformer_performance(s: &Solved) -> Option<Figure> {
    let (rc, xm, req, xeq) = (nv(s, "Rc")?, nv(s, "Xm")?, nv(s, "Req")?, nv(s, "Xeq")?);
    let (sn, load) = (nv(s, "S")?, nv(s, "load")?);
    let (v1, i1, pcu, pco, eff, reg) = (s.out("V1_required")?, s.out("I1")?, s.out("P_cu")?, s.out("P_core")?,
                                        s.out("efficiency_pct")?, s.out("regulation_pct")?);
    let mut c = Canvas::new(170.0);
    tx_circuit(&mut c, &q(rc, OHM), &q(xm, OHM), &q(req, OHM), &q(xeq, OHM));
    c.value(260.0, 156.0, "middle", &format!("V1 = {}, I1 = {}, copper loss {}, core loss {}", q(v1, "V"), q(i1, "A"),
                                             q(pcu, "W"), q(pco, "W")));
    let hi = (load * 1.25).max(1.25);
    let mut xs = k::span(hi / 25.0, hi, 25, false);
    xs.push(load);
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut ep = vec![];
    let mut rp = vec![];
    for x in &xs {
        if let Some(b) = s.with(&[("load", *x)]) {
            if let (Some(e), Some(r)) = (num_of(&b, "efficiency_pct"), num_of(&b, "regulation_pct")) {
                ep.push((*x, e));
                rp.push((*x, r));
            }
        }
    }
    if ep.len() < 2 {
        return None;
    }
    let c1 = k::lines(&[("efficiency (%)".into(), ep)], "", "Efficiency (%)", "", Some((load, eff)),
                      &format!("{} %", num(eff)), false);
    let c2 = k::lines(&[("voltage regulation (%)".into(), rp)], "Load (fraction of rated power)", "Regulation (%)", "",
                      Some((load, reg)), &format!("{} %", num(reg)), false);
    let (body, h) = panels(&[(c.body().to_string(), 170.0), (c1, 340.0), (c2, 340.0)]);
    Some(s.figure("circuit", "Transformer at load: circuit, efficiency and regulation",
                  &format!("The {} transformer referred to its high-voltage side (Req = {}, Xeq = {}, Rc = {}, Xm = {}). \
                            Top: the circuit; at {} of rated load the model returns V1 = {}, I1 = {}, copper loss {}, core \
                            loss {}. Middle and bottom: the model's efficiency and regulation against load, one run per \
                            point. Marked: the load given, {} % and {} %.",
                           q(sn, "VA"), q(req, OHM), q(xeq, OHM), q(rc, OHM), q(xm, OHM), num(load), q(v1, "V"),
                           q(i1, "A"), q(pcu, "W"), q(pco, "W"), num(eff), num(reg)), &body, h))
}

// ------------------------------------------------------------------------------------------------ networks
/// Positions for buses 0..n joined by `edges`: a row when the network is a chain, else round an ellipse.
fn layout(n: usize, edges: &[(usize, usize)], y: f64) -> Vec<P> {
    let mut deg = vec![0usize; n];
    for (a, b) in edges {
        deg[*a] += 1;
        deg[*b] += 1;
    }
    let ends: Vec<usize> = (0..n).filter(|i| deg[*i] == 1).collect();
    let chain = n >= 2 && edges.len() == n - 1 && ends.len() == 2 && deg.iter().all(|d| *d <= 2 && *d >= 1);
    if chain {
        let mut order = vec![ends[0].min(ends[1])];
        let mut prev = usize::MAX;
        while order.len() < n {
            let cur = *order.last().unwrap();
            let next = edges.iter().find_map(|(a, b)| {
                if *a == cur && *b != prev { Some(*b) } else if *b == cur && *a != prev { Some(*a) } else { None }
            });
            match next {
                Some(nx) => {
                    prev = cur;
                    order.push(nx);
                }
                None => break,
            }
        }
        if order.len() == n {
            let mut pos = vec![(0.0, 0.0); n];
            for (k2, i) in order.iter().enumerate() {
                pos[*i] = (80.0 + 360.0 * k2 as f64 / (n - 1) as f64, y);
            }
            return pos;
        }
    }
    (0..n).map(|i| {
        let a = std::f64::consts::PI * (1.0 + 2.0 * i as f64 / n as f64);
        (260.0 + 170.0 * a.cos(), y - 62.0 * a.sin())
    }).collect()
}

/// A flow arrow along the line from a to b (pointing to b), beside it, with its value.
fn flow(c: &mut Canvas, a: P, b: P, text: &str) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let d = (dx * dx + dy * dy).sqrt().max(1e-9);
    let (ux, uy) = (dx / d, dy / d);
    let (nx, ny) = (uy, -ux);
    let (nx, ny) = if ny > 0.0 { (-nx, -ny) } else { (nx, ny) };
    let m = ((a.0 + b.0) / 2.0 + nx * 14.0, (a.1 + b.1) / 2.0 + ny * 14.0);
    let l = (d * 0.3).min(44.0);
    let p0 = (m.0 - ux * l / 2.0, m.1 - uy * l / 2.0);
    let p1 = (m.0 + ux * l / 2.0, m.1 + uy * l / 2.0);
    c.raw(&crate::figures::arrow_svg(p0, p1, "axp-c", "axp-d"));
    c.value(m.0 + nx * 14.0, m.1 + ny * 14.0 + 4.0, "middle", text);
}

struct Net {
    names: Vec<String>,
    edges: Vec<(usize, usize, f64)>,
}

/// Lines "a-b x" (or "a b ..." with the reactance at `xi`), buses numbered from `first`; None when a bus number is
/// not one of `first`..`first + n`.
fn net(text: &str, first: usize, n: usize, xi: usize) -> Option<Net> {
    let mut edges = vec![];
    for r in rows(text) {
        let (a, b, x) = if let Some((a, b)) = pair(&r[0]) {
            (a, b, r.get(1).and_then(|t| fnum(t))?)
        } else {
            (r[0].clone(), r.get(1)?.clone(), r.get(xi).and_then(|t| fnum(t))?)
        };
        let (a, b): (usize, usize) = (a.parse().ok()?, b.parse().ok()?);
        if a < first || b < first || a >= first + n || b >= first + n || a == b {
            return None;
        }
        edges.push((a - first, b - first, x));
    }
    if edges.is_empty() || n > 8 {
        return None;
    }
    Some(Net { names: (first..first + n).map(|i| i.to_string()).collect(), edges })
}

/// Bus labels: the name above the bar, values under it.
fn bus_labels(c: &mut Canvas, p: P, name: &str, vals: &[String]) {
    c.bold(p.0, p.1 - 26.0, "middle", name);
    for (i, v) in vals.iter().enumerate() {
        c.value(p.0, p.1 + 36.0 + 16.0 * i as f64, "middle", v);
    }
}

pub fn dc_power_flow(s: &Solved) -> Option<Figure> {
    let th = s.out_list("theta_deg")?;
    let fl = s.out_list("flows")?;
    let ps = s.out("P_slack")?;
    let pin = s.vec("P").unwrap_or_default();
    let n = th.len();
    let nt = net(&raw(s, "lines")?, 1, n, 2)?;
    if nt.edges.len() != fl.len() || pin.len() + 1 != n {
        return None;
    }
    let mut c = Canvas::new(270.0);
    let pos = layout(n, &nt.edges.iter().map(|e| (e.0, e.1)).collect::<Vec<_>>(), 130.0);
    for (k2, (a, b, x)) in nt.edges.iter().enumerate() {
        c.line(pos[*a], pos[*b]);
        let m = ((pos[*a].0 + pos[*b].0) / 2.0, (pos[*a].1 + pos[*b].1) / 2.0);
        c.name(m.0, m.1 + 22.0, "middle", &format!("x = {}", num(*x)));
        let (from, to) = if fl[k2] >= 0.0 { (pos[*a], pos[*b]) } else { (pos[*b], pos[*a]) };
        flow(&mut c, from, to, &format!("{} pu", num(fl[k2].abs())));
    }
    for i in 0..n {
        bus(&mut c, pos[i], 18.0);
        let pinj = if i == 0 { ps } else { pin[i - 1] };
        bus_labels(&mut c, pos[i], &if i == 0 { format!("bus {} (slack)", nt.names[i]) } else { format!("bus {}", nt.names[i]) },
                   &[format!("θ = {}°", num(th[i])), format!("P = {} pu", num(pinj))]);
    }
    let lines_t: Vec<String> = nt.edges.iter().map(|e| format!("{}-{} {}", nt.names[e.0], nt.names[e.1], num(e.2))).collect();
    Some(s.figure("oneline", "DC power flow: bus angles and line flows",
                  &format!("DC power flow of the {n}-bus network given (lines {}: from-to and series reactance in per \
                            unit; injections {} pu at buses 2 to {n}, bus 1 the slack). The model returns the bus angles \
                            {}°, the line flows {} pu (arrows point the way power flows) and the slack injection {} pu.",
                           lines_t.join(", "), pin.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                           th.iter().map(|x| num(*x)).collect::<Vec<_>>().join("°, "),
                           fl.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), num(ps)), c.body(), 270.0))
}

pub fn newton_load_flow(s: &Solved) -> Option<Figure> {
    let vm = s.out_list("V")?;
    let ang = s.out_list("angle_deg")?;
    let (psl, qsl, ploss) = (s.out("P_slack")?, s.out("Q_slack")?, s.out("P_loss")?);
    let n = vm.len();
    if ang.len() != n {
        return None;
    }
    let lt = raw(s, "lines")?;
    let mut edges = vec![];
    for r in rows(&lt) {
        if r.len() < 4 {
            return None;
        }
        let (a, b): (usize, usize) = (r[0].parse().ok()?, r[1].parse().ok()?);
        let (rr, xx) = (fnum(&r[2])?, fnum(&r[3])?);
        if a < 1 || b < 1 || a > n || b > n || a == b {
            return None;
        }
        edges.push((a - 1, b - 1, rr, xx));
    }
    if edges.is_empty() || n > 8 {
        return None;
    }
    let mut loads: Vec<(usize, f64, f64)> = vec![];
    if let Some(t) = raw(s, "loads") {
        for r in rows(&t) {
            if r.len() >= 3 {
                let b: usize = r[0].parse().ok()?;
                if b >= 1 && b <= n {
                    loads.push((b - 1, fnum(&r[1])?, fnum(&r[2])?));
                }
            }
        }
    }
    let mut gens: Vec<usize> = vec![];
    if let Some(t) = raw(s, "gens") {
        for r in rows(&t) {
            if let Some(b) = r.first().and_then(|x| x.parse::<usize>().ok()) {
                if b >= 1 && b <= n {
                    gens.push(b - 1);
                }
            }
        }
    }
    let mut c = Canvas::new(280.0);
    let pos = layout(n, &edges.iter().map(|e| (e.0, e.1)).collect::<Vec<_>>(), 120.0);
    for (a, b, rr, xx) in &edges {
        c.line(pos[*a], pos[*b]);
        let m = ((pos[*a].0 + pos[*b].0) / 2.0, (pos[*a].1 + pos[*b].1) / 2.0);
        c.name(m.0, m.1 - 8.0, "middle", &format!("z = {} + j{}", num(*rr), num(*xx)));
    }
    for i in 0..n {
        bus(&mut c, pos[i], 18.0);
        let name = if i == 0 { "bus 1 (slack)".to_string() } else if gens.contains(&i) {
            format!("bus {} (generator)", i + 1)
        } else {
            format!("bus {}", i + 1)
        };
        bus_labels(&mut c, pos[i], &name, &[format!("{} pu ∠ {}°", num(vm[i]), num(ang[i]))]);
        if let Some(l) = loads.iter().find(|l| l.0 == i) {
            c.arrow((pos[i].0 + 8.0, pos[i].1 + 10.0), (pos[i].0 + 8.0, pos[i].1 + 26.0));
            c.name(pos[i].0, pos[i].1 + 68.0, "middle", &format!("load {} + j{} pu", num(l.1), num(l.2)));
        }
    }
    c.value(260.0, 250.0, "middle", &format!("slack P = {} pu, Q = {} pu", num(psl), num(qsl)));
    c.value(260.0, 270.0, "middle", &format!("real power loss {} pu", num(ploss)));
    Some(s.figure("oneline", "AC load flow: voltages, angles and slack power",
                  &format!("AC load flow (Newton-Raphson) of the {n}-bus network given (lines {}: from, to, r, x, shunt b in \
                            per unit), bus 1 the slack. The model returns the bus voltages {}, the slack injection P = {}, \
                            Q = {} pu and the real power loss {} pu.", lt.trim(),
                           (0..n).map(|i| format!("{}∠{}°", num(vm[i]), num(ang[i]))).collect::<Vec<_>>().join(", "),
                           num(psl), num(qsl), num(ploss)), c.body(), 280.0))
}

pub fn symmetrical_fault(s: &Solved) -> Option<Figure> {
    let v = s.out_list("V")?;
    let (iff, xth) = (s.out("If")?, s.out("X_th")?);
    let fb = nv(s, "fault_bus")?.round() as usize;
    let n = v.len() + 1;
    let nt = net(&raw(s, "network")?, 0, n, 2)?;
    if fb < 1 || fb >= n {
        return None;
    }
    let mut c = Canvas::new(250.0);
    let pos = layout(n, &nt.edges.iter().map(|e| (e.0, e.1)).collect::<Vec<_>>(), 110.0);
    for (a, b, x) in &nt.edges {
        let (pa, pb) = (pos[*a], pos[*b]);
        let (dx, dy) = (pb.0 - pa.0, pb.1 - pa.1);
        let d = (dx * dx + dy * dy).sqrt().max(1e-9);
        let body = 60f64.min(d * 0.6);
        let s0 = (pa.0 + dx * (0.5 - body / 2.0 / d), pa.1 + dy * (0.5 - body / 2.0 / d));
        let s1 = (pa.0 + dx * (0.5 + body / 2.0 / d), pa.1 + dy * (0.5 + body / 2.0 / d));
        c.line(pa, s0);
        c.inductor(s0, s1);
        c.line(s1, pb);
        let m = ((pa.0 + pb.0) / 2.0, (pa.1 + pb.1) / 2.0);
        c.name(m.0, m.1 - 16.0, "middle", &format!("x = {} pu", num(*x)));
    }
    let vf = nv(s, "Vf").unwrap_or(1.0);
    for i in 0..n {
        bus(&mut c, pos[i], 18.0);
        if i == 0 {
            c.bold(pos[i].0, pos[i].1 - 26.0, "middle", "source (bus 0)");
            c.name(pos[i].0, pos[i].1 + 36.0, "middle", &format!("{} pu behind the network", num(vf)));
        } else if i == fb {
            c.bold(pos[i].0, pos[i].1 - 26.0, "middle", &format!("bus {i}"));
            c.value(pos[i].0 - 10.0, pos[i].1 + 36.0, "end", &format!("V = {} pu", num(v[i - 1])));
        } else {
            bus_labels(&mut c, pos[i], &format!("bus {i}"), &[format!("V = {} pu", num(v[i - 1]))]);
        }
    }
    let p = pos[fb];
    c.raw(&format!("<path class=\"axp-c\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1}\"/>", p.0, p.1 + 18.0,
                   p.0 + 8.0, p.1 + 54.0, p.0 - 6.0, p.1 + 62.0, p.0 + 4.0, p.1 + 84.0));
    c.value(p.0, p.1 + 102.0, "middle", &format!("fault: If = {} pu", num(iff)));
    c.value(260.0, 240.0, "middle", &format!("Thevenin reactance at the fault X_th = {} pu", num(xth)));
    Some(s.figure("oneline", "Three-phase fault: fault current and bus voltages",
                  &format!("Three-phase fault at bus {fb} of the network given ({}: from-to and reactance in per unit; bus 0 \
                            is the source). The model returns the fault current If = {} pu, the Thevenin reactance at the \
                            fault X_th = {} pu and the bus voltages during the fault {} pu.", raw(s, "network")?.trim(),
                           num(iff), num(xth), v.iter().enumerate().map(|(i, x)| format!("V{} = {}", i + 1, num(*x)))
                               .collect::<Vec<_>>().join(", ")), c.body(), 250.0))
}

pub fn two_bus(s: &Solved) -> Option<Figure> {
    let (vs, r, x, p, qq) = (nv(s, "Vs")?, nv(s, "R")?, nv(s, "X")?, nv(s, "P")?, nv(s, "Q")?);
    let (vr, d, i, pl, drop) = (s.out("Vr")?, s.out("delta_deg")?, s.out("I")?, s.out("P_loss")?, s.out("voltage_drop_pct")?);
    let mut c = Canvas::new(150.0);
    bus(&mut c, (90.0, 64.0), 22.0);
    bus(&mut c, (340.0, 64.0), 22.0);
    c.bold(90.0, 30.0, "middle", "sending");
    c.bold(340.0, 30.0, "middle", "receiving");
    c.line((90.0, 64.0), (130.0, 64.0));
    c.resistor((130.0, 64.0), (190.0, 64.0));
    c.line((190.0, 64.0), (230.0, 64.0));
    c.inductor((230.0, 64.0), (300.0, 64.0));
    c.line((300.0, 64.0), (400.0, 64.0));
    c.arrow((400.0, 64.0), (400.0, 100.0));
    c.value(160.0, 48.0, "middle", &eq("R", r, OHM));
    c.value(265.0, 48.0, "middle", &eq("X", x, OHM));
    c.value(90.0, 110.0, "middle", &eq("Vs", vs, "V"));
    c.value(320.0, 110.0, "middle", &format!("Vr = {} ∠ {}°", q(vr, "V"), num(d)));
    c.name(500.0, 82.0, "end", &format!("load {} + j{}", q(p, "W"), q(qq, "var")));
    c.value(260.0, 140.0, "middle", &format!("I = {}, drop {} %, loss {}", q(i, "A"), num(drop), q(pl, "W")));
    // the load grows at the same power factor until the model finds no voltage solution
    let (sym, ratio, x0) = if p != 0.0 || qq == 0.0 { ("P", qq / if p != 0.0 { p } else { 1.0 }, p) } else { ("Q", 0.0, qq) };
    let sign = if x0 < 0.0 { -1.0 } else { 1.0 };
    let set = |v: f64| -> Vec<(&str, f64)> {
        if sym == "P" { vec![("P", v), ("Q", ratio * v)] } else { vec![("Q", v)] }
    };
    let mut lo = 0.0;
    let mut hi = f64::NAN;
    let mut t = x0.abs().max(1.0);
    for _ in 0..60 {
        if s.with(&set(sign * t)).is_some() {
            lo = t;
            t *= 2.0;
        } else {
            hi = t;
            break;
        }
    }
    let top = if hi.is_finite() {
        for _ in 0..12 {
            let m = (lo + hi) / 2.0;
            if s.with(&set(sign * m)).is_some() { lo = m } else { hi = m }
        }
        lo
    } else {
        x0.abs().max(1.0) * 4.0
    };
    if !(top > 0.0) {
        return None;
    }
    let mut pts = vec![];
    for v in k::span(0.0, top, 36, false) {
        if let Some(b) = s.with(&set(sign * v)) {
            if let Some(y) = num_of(&b, "Vr") {
                pts.push((sign * v, y));
            }
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let unit = if sym == "P" { "W" } else { "var" };
    // axes in the prefixes the values need (MW and kV for a transmission line)
    let (fx, px) = if top >= 1e6 { (1e-6, "M") } else if top >= 1e3 { (1e-3, "k") } else { (1.0, "") };
    let vmax = pts.iter().map(|p| p.1).fold(vr, f64::max);
    let (fy, py) = if vmax >= 1e3 { (1e-3, "k") } else { (1.0, "") };
    let pts: Vec<(f64, f64)> = pts.iter().map(|p| (p.0 * fx, p.1 * fy)).collect();
    let xl = if sym == "P" {
        format!("Load real power P ({px}W), Q = {} P", num(ratio))
    } else {
        format!("Load reactive power Q ({px}var), P = 0")
    };
    let chart = crate::figures::curves(&[("receiving voltage as the load grows".into(), pts)], &xl,
                                       &format!("Receiving voltage Vr ({py}V)"),
                                       &format!("largest load drawn {}", q(sign * top, unit)),
                                       Some(((x0 * fx, vr * fy), format!("{} (given)", q(vr, "V")))), false)?;
    let (body, h) = panels(&[(c.body().to_string(), 150.0), (chart, 340.0)]);
    Some(s.figure("oneline", "Two-bus load flow: the line and its voltage curve",
                  &format!("A {} source feeding a load of {} + j{} over a line of R = {}, X = {}. Top: the model returns \
                            Vr = {} at {}°, I = {}, voltage drop {} %, losses {}. Bottom: Vr against the load at the same \
                            power factor, one run per point; loads the model refuses (no voltage solution) are left \
                            out, so the curve stops near the largest load the line can carry ({} drawn).",
                           q(vs, "V"), q(p, "W"), q(qq, "var"), q(r, OHM), q(x, OHM), q(vr, "V"), num(d), q(i, "A"),
                           num(drop), q(pl, "W"), q(sign * top, unit)), &body, h))
}
