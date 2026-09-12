//! SVG path geometry: lines, areas, bars, arcs and stacks.
//!
//! Everything here takes coordinates already mapped to pixels and returns
//! either an SVG `d` string or plain numbers. Nothing knows about scales or
//! the DOM, so a canvas renderer can consume the same output.

use crate::data::Point;
use std::fmt::Write;

/// A builder for SVG path data. Coordinates are rounded to two decimals to
/// keep the string short; sub-1/100 px is invisible.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Path {
    d: String,
}

fn fmt_num(out: &mut String, v: f64) {
    let r = (v * 100.0).round() / 100.0;
    if r == r.trunc() {
        let _ = write!(out, "{}", r as i64);
    } else {
        let s = format!("{r:.2}");
        let _ = write!(out, "{}", s.trim_end_matches('0').trim_end_matches('.'));
    }
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }

    fn cmd(&mut self, c: char, coords: &[f64]) -> &mut Self {
        self.d.push(c);
        for (i, v) in coords.iter().enumerate() {
            if i > 0 {
                self.d.push(if i % 2 == 0 { ' ' } else { ',' });
            }
            fmt_num(&mut self.d, *v);
        }
        self
    }

    pub fn move_to(&mut self, x: f64, y: f64) -> &mut Self {
        self.cmd('M', &[x, y])
    }

    pub fn line_to(&mut self, x: f64, y: f64) -> &mut Self {
        self.cmd('L', &[x, y])
    }

    pub fn horizontal_to(&mut self, x: f64) -> &mut Self {
        self.d.push('H');
        fmt_num(&mut self.d, x);
        self
    }

    pub fn vertical_to(&mut self, y: f64) -> &mut Self {
        self.d.push('V');
        fmt_num(&mut self.d, y);
        self
    }

    pub fn cubic_to(
        &mut self,
        c1x: f64,
        c1y: f64,
        c2x: f64,
        c2y: f64,
        x: f64,
        y: f64,
    ) -> &mut Self {
        self.cmd('C', &[c1x, c1y, c2x, c2y, x, y])
    }

    /// An SVG elliptical arc with equal radii.
    pub fn arc_to(&mut self, r: f64, large: bool, sweep: bool, x: f64, y: f64) -> &mut Self {
        self.d.push('A');
        fmt_num(&mut self.d, r);
        self.d.push(',');
        fmt_num(&mut self.d, r);
        let _ = write!(self.d, " 0 {} {} ", large as u8, sweep as u8);
        fmt_num(&mut self.d, x);
        self.d.push(',');
        fmt_num(&mut self.d, y);
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.d.push('Z');
        self
    }

    pub fn is_empty(&self) -> bool {
        self.d.is_empty()
    }

    pub fn as_str(&self) -> &str {
        &self.d
    }

    pub fn into_string(self) -> String {
        self.d
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.d)
    }
}

/// How consecutive points are joined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Curve {
    /// Straight segments. The honest default.
    #[default]
    Linear,
    /// Horizontal then vertical: the value holds until the next sample.
    /// Right for counters, states and anything sampled-and-held.
    StepAfter,
    /// Vertical then horizontal.
    StepBefore,
    /// Step at the midpoint between samples.
    StepMiddle,
    /// A cubic spline that never overshoots the data (Fritsch–Carlson
    /// monotone interpolation). Use it only when the underlying quantity is
    /// genuinely smooth.
    MonotoneX,
}

/// Splits `points` into runs of defined points; a NaN in either coordinate
/// breaks the line.
fn defined_runs(points: &[Point]) -> impl Iterator<Item = &[Point]> {
    points
        .split(|p| !p.is_defined())
        .filter(|run| !run.is_empty())
}

/// Path data for a line through `points` (pixel space). Gaps (NaN) produce
/// separate subpaths.
pub fn line(points: &[Point], curve: Curve) -> String {
    let mut path = Path::new();
    for run in defined_runs(points) {
        append_run(&mut path, run, curve);
    }
    path.into_string()
}

fn append_run(path: &mut Path, run: &[Point], curve: Curve) {
    path.move_to(run[0].x, run[0].y);
    match curve {
        Curve::Linear => {
            for p in &run[1..] {
                path.line_to(p.x, p.y);
            }
        }
        Curve::StepAfter => {
            for w in run.windows(2) {
                path.horizontal_to(w[1].x).vertical_to(w[1].y);
            }
        }
        Curve::StepBefore => {
            for w in run.windows(2) {
                path.vertical_to(w[1].y).horizontal_to(w[1].x);
            }
        }
        Curve::StepMiddle => {
            for w in run.windows(2) {
                let mid = (w[0].x + w[1].x) / 2.0;
                path.horizontal_to(mid)
                    .vertical_to(w[1].y)
                    .horizontal_to(w[1].x);
            }
        }
        Curve::MonotoneX => monotone(path, run),
    }
}

/// Fritsch–Carlson tangents, then Hermite → Bézier segments.
fn monotone(path: &mut Path, run: &[Point]) {
    let n = run.len();
    if n < 2 {
        return;
    }
    // Monotone interpolation is only defined for strictly increasing x.
    // Duplicate or reordered samples are valid input for a general line, so
    // degrade to honest straight segments instead of emitting NaN controls.
    if run.windows(2).any(|w| w[1].x <= w[0].x) {
        for p in &run[1..] {
            path.line_to(p.x, p.y);
        }
        return;
    }
    if n == 2 {
        path.line_to(run[1].x, run[1].y);
        return;
    }
    let dx: Vec<f64> = run.windows(2).map(|w| w[1].x - w[0].x).collect();
    let dy: Vec<f64> = run.windows(2).map(|w| w[1].y - w[0].y).collect();
    let slopes: Vec<f64> = dx
        .iter()
        .zip(&dy)
        .map(|(dx, dy)| if *dx == 0.0 { 0.0 } else { dy / dx })
        .collect();

    let mut m = vec![0.0; n];
    m[0] = slopes[0];
    m[n - 1] = slopes[n - 2];
    for i in 1..n - 1 {
        let (s0, s1) = (slopes[i - 1], slopes[i]);
        if s0 * s1 <= 0.0 {
            m[i] = 0.0;
        } else {
            // Weighted harmonic mean keeps the curve monotone.
            let (w0, w1) = (2.0 * dx[i] + dx[i - 1], dx[i] + 2.0 * dx[i - 1]);
            m[i] = (w0 + w1) / (w0 / s0 + w1 / s1);
        }
    }
    for i in 0..n - 1 {
        let h = dx[i];
        let (p0, p1) = (run[i], run[i + 1]);
        path.cubic_to(
            p0.x + h / 3.0,
            p0.y + m[i] * h / 3.0,
            p1.x - h / 3.0,
            p1.y - m[i + 1] * h / 3.0,
            p1.x,
            p1.y,
        );
    }
}

/// Path data for the area between `top` and a flat `baseline_y`. Gaps in
/// `top` produce separate closed subpaths.
pub fn area(top: &[Point], baseline_y: f64, curve: Curve) -> String {
    let mut path = Path::new();
    for run in defined_runs(top) {
        append_run(&mut path, run, curve);
        let last = run[run.len() - 1];
        let first = run[0];
        path.line_to(last.x, baseline_y)
            .line_to(first.x, baseline_y)
            .close();
    }
    path.into_string()
}

/// Path data for the band between `top` and `bottom` (same length, same x
/// values), for stacked areas and confidence bands. Gaps are broken where
/// either side is undefined.
pub fn band(top: &[Point], bottom: &[Point], curve: Curve) -> String {
    let n = top.len().min(bottom.len());
    let mut path = Path::new();
    let mut i = 0;
    while i < n {
        let start = i;
        while i < n && top[i].is_defined() && bottom[i].is_defined() {
            i += 1;
        }
        if i > start {
            append_run(&mut path, &top[start..i], curve);
            let mut back: Vec<Point> = bottom[start..i].to_vec();
            back.reverse();
            path.line_to(back[0].x, back[0].y);
            // Reuse the run appender for the return trip minus its move.
            let mut tail = Path::new();
            append_run(&mut tail, &back, curve);
            let tail = tail.into_string();
            let after_move = tail
                .find(['L', 'H', 'V', 'C'])
                .map(|p| &tail[p..])
                .unwrap_or("");
            path.d.push_str(after_move);
            path.close();
        }
        i += 1;
    }
    path.into_string()
}

/// Which edge of a bar is the data end and gets rounded. The baseline edge
/// stays square.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundedEnd {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

/// Path data for a rectangle with one rounded end. `radius` is clamped so
/// the corners never overlap on a short or thin bar.
pub fn bar(x: f64, y: f64, width: f64, height: f64, radius: f64, end: RoundedEnd) -> String {
    let w = width.max(0.0);
    let h = height.max(0.0);
    let r = radius.max(0.0).min(w / 2.0).min(h / 2.0);
    let mut p = Path::new();
    if r == 0.0 || end == RoundedEnd::None || w == 0.0 || h == 0.0 {
        p.move_to(x, y)
            .horizontal_to(x + w)
            .vertical_to(y + h)
            .horizontal_to(x)
            .close();
        return p.into_string();
    }
    let (x1, y1) = (x + w, y + h);
    match end {
        RoundedEnd::Top => {
            p.move_to(x, y1)
                .vertical_to(y + r)
                .arc_to(r, false, true, x + r, y)
                .horizontal_to(x1 - r)
                .arc_to(r, false, true, x1, y + r)
                .vertical_to(y1)
                .close();
        }
        RoundedEnd::Bottom => {
            p.move_to(x, y)
                .horizontal_to(x1)
                .vertical_to(y1 - r)
                .arc_to(r, false, true, x1 - r, y1)
                .horizontal_to(x + r)
                .arc_to(r, false, true, x, y1 - r)
                .close();
        }
        RoundedEnd::Right => {
            p.move_to(x, y)
                .horizontal_to(x1 - r)
                .arc_to(r, false, true, x1, y + r)
                .vertical_to(y1 - r)
                .arc_to(r, false, true, x1 - r, y1)
                .horizontal_to(x)
                .close();
        }
        RoundedEnd::Left => {
            p.move_to(x1, y)
                .vertical_to(y1)
                .horizontal_to(x + r)
                .arc_to(r, false, true, x, y1 - r)
                .vertical_to(y + r)
                .arc_to(r, false, true, x + r, y)
                .close();
        }
        RoundedEnd::None => unreachable!(),
    }
    p.into_string()
}

/// Path data for an annular sector centred at `(cx, cy)`. Angles are in
/// radians, clockwise from 12 o'clock (SVG convention with y down).
/// `pad` is an angular gap in radians removed from each side, which is how
/// the 2px surface gap between segments is drawn without a stroke.
pub fn arc(cx: f64, cy: f64, inner_r: f64, outer_r: f64, start: f64, end: f64, pad: f64) -> String {
    let (start, end) = if end < start {
        (end, start)
    } else {
        (start, end)
    };
    let pad = pad.max(0.0);
    let (a0, a1) = (start + pad, end - pad);
    if a1 <= a0 || outer_r <= 0.0 {
        return String::new();
    }
    let inner_r = inner_r.clamp(0.0, outer_r);
    let sweep = a1 - a0;
    let large = sweep > std::f64::consts::PI;
    let pt = |r: f64, a: f64| (cx + r * a.sin(), cy - r * a.cos());
    let (ox0, oy0) = pt(outer_r, a0);
    let (ox1, oy1) = pt(outer_r, a1);
    let mut p = Path::new();
    p.move_to(ox0, oy0).arc_to(outer_r, large, true, ox1, oy1);
    if inner_r > 0.0 {
        let (ix0, iy0) = pt(inner_r, a0);
        let (ix1, iy1) = pt(inner_r, a1);
        p.line_to(ix1, iy1).arc_to(inner_r, large, false, ix0, iy0);
    } else {
        p.line_to(cx, cy);
    }
    p.close();
    p.into_string()
}

/// Lays out pie/donut slices from `values`, returning `(start, end)`
/// angle pairs in radians clockwise from 12 o'clock. Non-positive values get
/// an empty slice so indices stay aligned with the input.
pub fn pie(values: &[f64]) -> Vec<(f64, f64)> {
    let total: f64 = values.iter().filter(|v| v.is_finite() && **v > 0.0).sum();
    let mut angle = 0.0;
    values
        .iter()
        .map(|v| {
            let v = if v.is_finite() && *v > 0.0 { *v } else { 0.0 };
            let sweep = if total > 0.0 {
                v / total * std::f64::consts::TAU
            } else {
                0.0
            };
            let s = angle;
            angle += sweep;
            (s, angle)
        })
        .collect()
}

/// How stacked series are offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StackOffset {
    /// Plain cumulative stacking from zero.
    #[default]
    None,
    /// Normalises each x to `0..=1` (100% stacked).
    Expand,
    /// Positive values stack upward from zero, negative values downward.
    Diverging,
}

/// A stacked value: the lower and upper edge of one series at one x.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stacked {
    pub y0: f64,
    pub y1: f64,
}

/// Stacks `series[s][i]` (series-major, all the same length) into lower and
/// upper edges in data units. NaN values contribute zero height.
pub fn stack(series: &[Vec<f64>], offset: StackOffset) -> Vec<Vec<Stacked>> {
    // A live feed can temporarily have different lengths per series.  Stack
    // the union of all columns and treat a missing value like a gap instead
    // of indexing through the first series' length.
    let n = series.iter().map(Vec::len).max().unwrap_or(0);
    let mut out: Vec<Vec<Stacked>> = series
        .iter()
        .map(|_| vec![Stacked { y0: 0.0, y1: 0.0 }; n])
        .collect();
    for i in 0..n {
        let mut pos = 0.0;
        let mut neg = 0.0;
        for (s, values) in series.iter().enumerate() {
            let v = values
                .get(i)
                .copied()
                .filter(|v| v.is_finite())
                .unwrap_or(0.0);
            let (y0, y1) = match offset {
                StackOffset::Diverging if v < 0.0 => {
                    let y0 = neg;
                    neg += v;
                    (y0, neg)
                }
                _ => {
                    let y0 = pos;
                    pos += v;
                    (y0, pos)
                }
            };
            out[s][i] = Stacked { y0, y1 };
        }
        if offset == StackOffset::Expand && pos > 0.0 {
            for column in out.iter_mut() {
                column[i].y0 /= pos;
                column[i].y1 /= pos;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pts(v: &[(f64, f64)]) -> Vec<Point> {
        v.iter().map(|&(x, y)| Point::new(x, y)).collect()
    }

    #[test]
    fn numbers_are_short() {
        let mut p = Path::new();
        p.move_to(1.0, 2.5).line_to(3.333333, 4.0);
        assert_eq!(p.as_str(), "M1,2.5L3.33,4");
    }

    #[test]
    fn line_breaks_on_gaps() {
        let d = line(
            &pts(&[
                (0.0, 0.0),
                (1.0, 1.0),
                (2.0, f64::NAN),
                (3.0, 3.0),
                (4.0, 4.0),
            ]),
            Curve::Linear,
        );
        assert_eq!(d, "M0,0L1,1M3,3L4,4");
    }

    #[test]
    fn steps() {
        let p = pts(&[(0.0, 0.0), (10.0, 5.0)]);
        assert_eq!(line(&p, Curve::StepAfter), "M0,0H10V5");
        assert_eq!(line(&p, Curve::StepBefore), "M0,0V5H10");
        assert_eq!(line(&p, Curve::StepMiddle), "M0,0H5V5H10");
    }

    #[test]
    fn monotone_does_not_overshoot() {
        // A plateau followed by a rise: tangents at the plateau must be zero,
        // so the first control points sit on the plateau line.
        let d = line(
            &pts(&[(0.0, 10.0), (1.0, 10.0), (2.0, 0.0)]),
            Curve::MonotoneX,
        );
        assert!(d.starts_with("M0,10C0.33,10 0.67,10 1,10"), "{d}");
        assert_eq!(
            line(&pts(&[(0.0, 0.0), (1.0, 1.0)]), Curve::MonotoneX),
            "M0,0L1,1"
        );
        assert_eq!(
            line(
                &pts(&[(0.0, 0.0), (0.0, 1.0), (1.0, 2.0)]),
                Curve::MonotoneX
            ),
            "M0,0L0,1L1,2"
        );
    }

    #[test]
    fn area_closes_to_baseline() {
        let d = area(&pts(&[(0.0, 5.0), (10.0, 2.0)]), 20.0, Curve::Linear);
        assert_eq!(d, "M0,5L10,2L10,20L0,20Z");
    }

    #[test]
    fn band_returns_along_bottom() {
        let d = band(
            &pts(&[(0.0, 1.0), (10.0, 2.0)]),
            &pts(&[(0.0, 5.0), (10.0, 6.0)]),
            Curve::Linear,
        );
        assert_eq!(d, "M0,1L10,2L10,6L0,5Z");
    }

    #[test]
    fn bars_round_only_the_data_end() {
        let d = bar(0.0, 0.0, 10.0, 20.0, 4.0, RoundedEnd::Top);
        assert_eq!(d, "M0,20V4A4,4 0 0 1 4,0H6A4,4 0 0 1 10,4V20Z");
        assert_eq!(
            bar(0.0, 0.0, 10.0, 20.0, 0.0, RoundedEnd::Top),
            "M0,0H10V20H0Z"
        );
        // A 2px-tall bar clamps the radius to 1px rather than inverting.
        let short = bar(0.0, 0.0, 10.0, 2.0, 4.0, RoundedEnd::Top);
        assert!(short.contains("A1,1"), "{short}");
        assert_eq!(
            bar(0.0, 0.0, 0.0, 20.0, 4.0, RoundedEnd::Top),
            "M0,0H0V20H0Z"
        );
    }

    #[test]
    fn arcs_and_pies() {
        let slices = pie(&[1.0, 1.0, 2.0]);
        assert_eq!(slices.len(), 3);
        assert!((slices[2].1 - std::f64::consts::TAU).abs() < 1e-12);
        assert!((slices[0].1 - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
        let d = arc(
            50.0,
            50.0,
            20.0,
            40.0,
            0.0,
            std::f64::consts::FRAC_PI_2,
            0.0,
        );
        assert!(
            d.starts_with("M50,10A40,40 0 0 1 90,50L70,50A20,20 0 0 0 50,30Z"),
            "{d}"
        );
        assert_eq!(
            arc(0.0, 0.0, 0.0, 10.0, 0.0, 0.1, 0.2),
            "",
            "padding larger than the slice removes it"
        );
        assert!(pie(&[0.0, -1.0]).iter().all(|(s, e)| s == e));
    }

    #[test]
    fn stacks() {
        let s = stack(&[vec![1.0, 2.0], vec![3.0, f64::NAN]], StackOffset::None);
        assert_eq!(s[0][0], Stacked { y0: 0.0, y1: 1.0 });
        assert_eq!(s[1][0], Stacked { y0: 1.0, y1: 4.0 });
        assert_eq!(s[1][1], Stacked { y0: 2.0, y1: 2.0 });

        let e = stack(&[vec![1.0], vec![3.0]], StackOffset::Expand);
        assert_eq!(e[1][0], Stacked { y0: 0.25, y1: 1.0 });

        let d = stack(&[vec![2.0], vec![-3.0], vec![1.0]], StackOffset::Diverging);
        assert_eq!(d[1][0], Stacked { y0: 0.0, y1: -3.0 });
        assert_eq!(d[2][0], Stacked { y0: 2.0, y1: 3.0 });
    }

    #[test]
    fn stacks_uneven_series_without_panicking() {
        let s = stack(&[vec![1.0, 2.0], vec![3.0]], StackOffset::Diverging);
        assert_eq!(s[0].len(), 2);
        assert_eq!(s[1][0], Stacked { y0: 1.0, y1: 4.0 });
        assert_eq!(s[1][1], Stacked { y0: 2.0, y1: 2.0 });
    }
}
