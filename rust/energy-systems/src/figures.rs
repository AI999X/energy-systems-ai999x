//! Energy Systems' own live figures (ax_engine::figkit): every capability's published figure kinds, drawn for the
//! visitor's inputs from what the model computed (circuits and one-line diagrams labelled with the solved values,
//! sequence networks of a fault, power-angle areas, sun paths, irradiance on a tilted plane, ...). Written by hand for
//! the Rust build. A figure the inputs make impossible is left out, never drawn wrong.
//!
//! This file holds the dispatcher and the small chart kit the figures share (axes, areas, arrows, legends, panels);
//! the drawings are in figures_circuits.rs (circuits, networks, process blocks) and figures_charts.rs (charts).

use crate::figures_charts as ch;
use crate::figures_circuits as ci;
use ax_engine::figkit::{num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "energy_actuator_disc" => vec![ci::actuator_disc(s)],
        "energy_battery_pack_sizing" => vec![ci::battery_pack(s)],
        "energy_battery_rc_discharge" => vec![ci::battery_rc(s)],
        "energy_slg_fault" => vec![ci::fault(s, Fault::Slg)],
        "energy_dlg_fault" => vec![ci::fault(s, Fault::Dlg)],
        "energy_ll_fault" => vec![ci::fault(s, Fault::Ll)],
        "energy_electrolyser" => vec![ci::electrolyser(s)],
        "energy_fuel_cell_stack" => vec![ci::fuel_cell(s)],
        "energy_geometric_mean_distance" => vec![ci::spacing(s, false)],
        "energy_line_parameters" => vec![ci::spacing(s, true)],
        "energy_induction_motor" => vec![ci::induction_motor(s)],
        "energy_medium_line" => vec![ci::line_model(s, true)],
        "energy_short_line" => vec![ci::line_model(s, false)],
        "energy_single_diode_current" => vec![ci::single_diode(s)],
        "energy_transformer_performance" => vec![ci::transformer_performance(s)],
        "energy_transformer_tests" => vec![ci::transformer_tests(s)],
        "energy_dc_power_flow" => vec![ci::dc_power_flow(s)],
        "energy_newton_load_flow" => vec![ci::newton_load_flow(s)],
        "energy_symmetrical_fault" => vec![ci::symmetrical_fault(s)],
        "energy_two_bus_load_flow" => vec![ci::two_bus(s)],
        _ => ch::live(s),
    };
    v.into_iter().flatten().collect()
}

/// The three unbalanced faults (their sequence networks are connected differently).
#[derive(Clone, Copy, PartialEq)]
pub enum Fault {
    Slg,
    Dlg,
    Ll,
}

// ------------------------------------------------------------------------------------------------ inputs as given
/// An input as the visitor gave it, as text (a list or table written out).
pub fn raw(s: &Solved, sym: &str) -> Option<String> {
    Some(match &s.given.iter().find(|g| g.0 == sym)?.1 {
        Input::Text(t) => t.clone(),
        Input::Int(i) => i.to_string(),
        Input::Float(x) => format!("{x}"),
        Input::Json(v) => v.to_string(),
    })
}

/// A table given as text: rows separated by ";" (or by "," when there is no ";"), cells by spaces; a JSON array of
/// arrays (or of strings) is read the same way.
pub fn rows(t: &str) -> Vec<Vec<String>> {
    let t = t.trim();
    if t.starts_with('[') {
        if let Ok(serde_json::Value::Array(a)) = serde_json::from_str::<serde_json::Value>(t) {
            return a.iter().map(|r| match r {
                serde_json::Value::Array(c) => c.iter().map(|x| match x {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                }).collect(),
                serde_json::Value::String(s) => s.split_whitespace().map(|x| x.to_string()).collect(),
                other => vec![other.to_string()],
            }).filter(|r: &Vec<String>| !r.is_empty()).collect();
        }
    }
    let sep = if t.contains(';') { ';' } else { ',' };
    t.split(sep).map(|r| r.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>())
        .filter(|r| !r.is_empty()).collect()
}

/// "1-2" -> ("1", "2").
pub fn pair(tok: &str) -> Option<(String, String)> {
    let i = tok.char_indices().skip(1).find(|c| c.1 == '-')?.0;
    let (a, b) = (tok[..i].trim(), tok[i + 1..].trim());
    (!a.is_empty() && !b.is_empty()).then(|| (a.to_string(), b.to_string()))
}

/// A number input, also when the procedure reads it as a whole number (a count, a day of the year).
pub fn nv(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| match s.val(sym)? {
        ax_engine::Val::Num(x) if x.is_finite() => Some(x),
        ax_engine::Val::Int(i) => Some(i as f64),
        _ => None,
    })
}

pub fn fnum(t: &str) -> Option<f64> {
    t.trim().parse::<f64>().ok().filter(|x| x.is_finite())
}

// ------------------------------------------------------------------------------------------------ the chart kit
pub const X0: f64 = 96.0;
pub const X1: f64 = 504.0;
pub const Y0: f64 = 44.0;
pub const Y1: f64 = 278.0;

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// A text kept inside a canvas `h` tall (widths as the figure check estimates them).
pub fn put(out: &mut String, x: f64, y: f64, anchor: &str, class: &str, t: &str, h: f64) {
    if t.is_empty() {
        return;
    }
    let w = t.chars().count() as f64 * if class == "axp-mt" { 9.2 } else { 7.8 };
    let left = match anchor {
        "middle" => x - w / 2.0,
        "end" => x - w,
        _ => x,
    };
    let shift = if left < 3.0 { 3.0 - left } else if left + w > 517.0 { 517.0 - (left + w) } else { 0.0 };
    let y = y.clamp(13.0, h - 4.0);
    out.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>",
                          x + shift, esc(t)));
}

pub fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 12 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}

#[derive(Clone, Copy)]
pub struct Ax {
    pub lo: f64,
    pub hi: f64,
    a: f64,
    b: f64,
    log: bool,
}

impl Ax {
    pub fn new(lo: f64, hi: f64, a: f64, b: f64, pad: bool, log: bool) -> Ax {
        let (mut lo, mut hi) = (lo, hi);
        let log = log && lo > 0.0 && hi > lo;
        if log {
            if pad {
                lo /= 1.2;
                hi *= 1.2;
            }
        } else {
            if !(hi > lo) || !(hi - lo).is_finite() {
                let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.1 };
                lo -= d;
                hi += d;
            }
            if pad {
                let d = (hi - lo) * 0.06;
                lo -= d;
                hi += d;
            }
        }
        Ax { lo, hi, a, b, log }
    }
    pub fn px(&self, v: f64) -> f64 {
        let f = if self.log { (v.log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.a + f * (self.b - self.a)
    }
    pub fn ticks(&self) -> Vec<f64> {
        if self.log {
            let (e0, e1) = (self.lo.log10().ceil() as i32, self.hi.log10().floor() as i32);
            let v: Vec<f64> = (e0..=e1).map(|e| 10f64.powi(e)).collect();
            return if v.is_empty() { vec![(self.lo * self.hi).sqrt()] } else { v };
        }
        nice_ticks(self.lo, self.hi, 5)
    }
}

/// A chart in the generic 520 x 340 frame, drawn in data coordinates.
pub struct Plot {
    pub x: Ax,
    pub y: Ax,
    pub s: String,
    clip: bool,
}

pub const CL: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
pub const BAR: [&str; 4] = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];
pub const SH: [&str; 4] = ["axp-s", "axp-s2", "axp-s3", "axp-s4"];
pub const DOT: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];

fn finite(v: &[f64]) -> Vec<f64> {
    v.iter().copied().filter(|x| x.is_finite()).collect()
}

pub fn range(v: &[f64]) -> Option<(f64, f64)> {
    let v = finite(v);
    if v.is_empty() {
        return None;
    }
    Some(v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x))))
}

impl Plot {
    /// Axes over these ranges (padded a little when asked).
    pub fn new(xr: (f64, f64), yr: (f64, f64), pad_x: bool, pad_y: bool) -> Plot {
        Plot { x: Ax::new(xr.0, xr.1, X0, X1, pad_x, false), y: Ax::new(yr.0, yr.1, Y1, Y0, pad_y, false),
               s: String::new(), clip: false }
    }
    pub fn log_x(mut self) -> Plot {
        self.x = Ax::new(self.x.lo, self.x.hi, X0, X1, false, true);
        self
    }
    /// Equal scales on both axes (a phasor diagram), covering these points.
    pub fn equal(xs: &[f64], ys: &[f64]) -> Option<Plot> {
        let (xl, xh) = range(xs)?;
        let (yl, yh) = range(ys)?;
        let span = (xh - xl).max(yh - yl).max(1e-300);
        let (cx, cy) = ((xl + xh) / 2.0, (yl + yh) / 2.0);
        let half_y = span * 0.58;
        let half_x = half_y * (X1 - X0) / (Y1 - Y0);
        let mut p = Plot::new((cx - half_x, cx + half_x), (cy - half_y, cy + half_y), false, false);
        p.x = Ax::new(cx - half_x, cx + half_x, X0, X1, false, false);
        p.y = Ax::new(cy - half_y, cy + half_y, Y1, Y0, false, false);
        Some(p)
    }
    pub fn px(&self, p: (f64, f64)) -> (f64, f64) {
        (self.x.px(p.0), self.y.px(p.1))
    }
    /// Grid, ticks, frame, axis names and the header line.
    pub fn frame(&mut self, xlab: &str, ylab: &str, header: &str) {
        let mut o = String::new();
        for t in self.x.ticks() {
            let x = self.x.px(t);
            o.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{Y0}\" x2=\"{x:.1}\" y2=\"{Y1}\"/>\
                                 <text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                                Y1 + 20.0, esc(&num(t))));
        }
        for t in self.y.ticks() {
            let y = self.y.px(t);
            o.push_str(&format!("<line class=\"axp-g\" x1=\"{X0}\" y1=\"{y:.1}\" x2=\"{X1}\" y2=\"{y:.1}\"/>\
                                 <text class=\"axp-t\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                X0 - 6.0, y + 5.0, esc(&num(t))));
        }
        o.push_str(&format!("<rect class=\"axp-f\" x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/>", X1 - X0, Y1 - Y0));
        if !xlab.is_empty() {
            o.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>",
                                (X0 + X1) / 2.0, esc(xlab)));
        }
        if !ylab.is_empty() {
            o.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" \
                                 text-anchor=\"middle\">{}</text>", (Y0 + Y1) / 2.0, esc(ylab)));
        }
        // the frame goes under what is drawn later
        self.s = o + &self.s;
        if !header.is_empty() {
            put(&mut self.s, X0, 20.0, "start", "axp-mt", header, 340.0);
        }
    }
    fn open(&mut self) {
        if !self.clip {
            self.s.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/>\
                                      </clipPath>", X1 - X0, Y1 - Y0));
            self.clip = true;
        }
    }
    fn clipped(&mut self, inner: &str) {
        self.open();
        self.s.push_str(&format!("<g clip-path=\"url(#CLIPID)\">{inner}</g>"));
    }
    fn d_of(&self, pts: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut pen = false;
        for p in pts {
            let (x, y) = self.px(*p);
            if x.is_finite() && y.is_finite() {
                d.push_str(&format!("{}{x:.1},{y:.1} ", if pen { "L" } else { "M" }));
                pen = true;
            } else {
                pen = false;
            }
        }
        d.trim_end().to_string()
    }
    /// A curve (a NaN point breaks it).
    pub fn path(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.d_of(pts);
        if !d.is_empty() {
            self.clipped(&format!("<path class=\"{class}\" d=\"{d}\"/>"));
        }
    }
    pub fn dashed(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.d_of(pts);
        if !d.is_empty() {
            self.clipped(&format!("<path class=\"{class}\" stroke-dasharray=\"6 4\" d=\"{d}\"/>"));
        }
    }
    /// The region between two curves (`bottom` given in the same order as `top`).
    pub fn area(&mut self, top: &[(f64, f64)], bottom: &[(f64, f64)], class: &str) {
        let mut all: Vec<(f64, f64)> = top.to_vec();
        all.extend(bottom.iter().rev());
        let pts: Vec<(f64, f64)> = all.iter().map(|p| self.px(*p)).filter(|p| p.0.is_finite() && p.1.is_finite()).collect();
        if pts.len() < 3 {
            return;
        }
        let d: String = pts.iter().enumerate().map(|(i, p)| format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1))
            .collect();
        self.clipped(&format!("<path class=\"{class}\" d=\"{}Z\"/>", d));
    }
    pub fn dot(&mut self, p: (f64, f64), class: &str, r: f64) {
        let (x, y) = self.px(p);
        if x.is_finite() && y.is_finite() {
            self.clipped(&format!("<circle class=\"{class}\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{r}\"/>"));
        }
    }
    /// The visitor's point, marked and labelled inside the plot.
    pub fn marker(&mut self, p: (f64, f64), label: &str) {
        let (px, py) = self.px(p);
        if !(px.is_finite() && py.is_finite()) {
            return;
        }
        self.s.push_str(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>"));
        let w = 9.2 * label.chars().count() as f64;
        let (tx, anchor) = if px + 10.0 + w <= X1 { (px + 10.0, "start") } else if px - 10.0 - w >= X0 {
            (px - 10.0, "end")
        } else {
            (((X0 + X1) / 2.0).clamp(X0 + w / 2.0, X1 - w / 2.0), "middle")
        };
        let ty = if py < Y0 + 30.0 { py + 22.0 } else { py - 10.0 };
        put(&mut self.s, tx, ty, anchor, "axp-mt", label, 340.0);
    }
    /// A text at a point of the plot (data coordinates), offset in pixels.
    pub fn label(&mut self, p: (f64, f64), dx: f64, dy: f64, anchor: &str, class: &str, t: &str) {
        let (x, y) = self.px(p);
        if x.is_finite() && y.is_finite() {
            put(&mut self.s, x + dx, (y + dy).clamp(Y0 + 12.0, Y1 + 14.0), anchor, class, t, 340.0);
        }
    }
    /// An arrow between two points (data coordinates), its head filled with `head`.
    pub fn arrow(&mut self, a: (f64, f64), b: (f64, f64), class: &str, head: &str) {
        let (pa, pb) = (self.px(a), self.px(b));
        let s = arrow_svg(pa, pb, class, head);
        if !s.is_empty() {
            self.clipped(&s);
        }
    }
    /// Legend swatches at the top right (bars for areas and bars, a short line for curves).
    pub fn legend(&mut self, items: &[(String, &str)]) {
        legend(&mut self.s, items, 28.0);
    }
    pub fn body(self) -> String {
        self.s
    }
}

/// An arrow from pa to pb in pixels: a line in `class` and a filled head in `head`.
pub fn arrow_svg(pa: (f64, f64), pb: (f64, f64), class: &str, head: &str) -> String {
    let (dx, dy) = (pb.0 - pa.0, pb.1 - pa.1);
    let d = (dx * dx + dy * dy).sqrt();
    if !(d > 1.0) || !d.is_finite() {
        return String::new();
    }
    let (ux, uy) = (dx / d, dy / d);
    let (nx, ny) = (-uy, ux);
    let hl = 10f64.min(d * 0.5);
    let b = (pb.0 - ux * hl, pb.1 - uy * hl);
    format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/><path class=\"{head}\" d=\"M{:.1},{:.1} L{:.1},{:.1} \
             L{:.1},{:.1} Z\"/>", pa.0, pa.1, b.0, b.1, pb.0, pb.1, b.0 + nx * 5.0, b.1 + ny * 5.0, b.0 - nx * 5.0,
            b.1 - ny * 5.0)
}

/// Legend entries right-aligned at the top of a chart: a swatch (a bar class: area or bar; a curve class: a line) and
/// the name; a second row when they do not fit in one.
pub fn legend(out: &mut String, items: &[(String, &str)], y: f64) {
    let mut rows: Vec<Vec<(String, &str)>> = vec![vec![]];
    let mut used = 0.0;
    for (n, class) in items.iter().filter(|x| !x.0.is_empty()).take(6) {
        let w = 7.4 * n.chars().count() as f64 + 30.0;
        if used + w > X1 - X0 && !rows.last().map(|r| r.is_empty()).unwrap_or(true) {
            rows.push(vec![]);
            used = 0.0;
        }
        used += w;
        rows.last_mut().unwrap().push((n.clone(), class));
    }
    for (r, row) in rows.iter().enumerate() {
        let mut x = X1;
        let yy = y + 14.0 * r as f64 - 14.0 * (rows.len() - 1) as f64;
        for (n, class) in row.iter().rev() {
            let w = 7.4 * n.chars().count() as f64 + 22.0;
            x -= w;
            if class.starts_with("axp-c") {
                out.push_str(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", x, yy + 5.0, x + 12.0,
                                      yy + 5.0));
            } else {
                out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{yy:.1}\" width=\"12\" height=\"10\"/>"));
            }
            out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\">{}</text>", x + 16.0, yy + 9.0, esc(n)));
            x -= 8.0;
        }
    }
}

/// Curves on linear axes that keep every point (and zero when `with_zero`): a legend when named, the visitor's
/// point marked.
pub fn curves(series: &[(String, Vec<(f64, f64)>)], xlab: &str, ylab: &str, header: &str,
              marker: Option<((f64, f64), String)>, with_zero: bool) -> Option<String> {
    let mut xs: Vec<f64> = series.iter().flat_map(|s| s.1.iter().map(|p| p.0)).collect();
    let mut ys: Vec<f64> = series.iter().flat_map(|s| s.1.iter().map(|p| p.1)).collect();
    if let Some((m, _)) = &marker {
        xs.push(m.0);
        ys.push(m.1);
    }
    if with_zero {
        ys.push(0.0);
    }
    let mut p = Plot::new(range(&xs)?, range(&ys)?, false, true);
    for (i, (_, pts)) in series.iter().enumerate() {
        p.path(pts, CL[i % 4]);
    }
    if let Some((m, t)) = &marker {
        p.marker(*m, t);
    }
    p.frame(xlab, ylab, header);
    let names: Vec<(String, &str)> = series.iter().enumerate().map(|(i, s)| (s.0.clone(), CL[i % 4])).collect();
    p.legend(&names);
    Some(p.body())
}

/// Panels drawn one under another (a schematic over a chart): the body and its height.
pub fn panels(parts: &[(String, f64)]) -> (String, f64) {
    let mut out = String::new();
    let mut y = 0.0;
    for (i, (b, h)) in parts.iter().enumerate() {
        out.push_str(&format!("<g transform=\"translate(0,{y})\">{}</g>", b.replace("CLIPID", &format!("CLIPID{i}"))));
        y += h;
    }
    (out, y)
}
