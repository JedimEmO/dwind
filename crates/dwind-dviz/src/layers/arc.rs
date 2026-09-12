//! Donut segments: part-to-whole at a glance, six segments at most. For
//! comparing close values, use bars.
//!
//! Segments are rounded stroked arcs on one ring, with a centre figure
//! that shows the total, or the hovered segment's value and share. Angles
//! are tweened in Rust (the browser's path interpolation cannot cross the
//! large-arc flag cleanly), and the ring sweeps in on first render.

use std::rc::Rc;

use dominator::{Dom, clone, events, svg};
use dwind_dviz_core::data::Series;
use dwind_dviz_core::format;
use dwind_dviz_core::geom::{self, Path};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};

use super::{MarkInput, hoverable, keyed_marks, px};
use crate::chart::{ChartContext, Frame, Layer, Tooltip, TooltipRow, layer};
use crate::motion::tween;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcOptions {
    /// Inner radius as a fraction of the outer; the ring is the rest.
    pub inner_ratio: f64,
    /// Gap between segments in px, measured on the ring's centreline.
    pub gap: f64,
    /// Round the segment ends.
    pub rounded: bool,
    /// Show the total in the centre, and the hovered segment's value and
    /// share while hovering. Replaces the tooltip.
    pub center: bool,
    /// Label under the centre figure when nothing is hovered.
    pub center_label: &'static str,
}

impl Default for ArcOptions {
    fn default() -> Self {
        Self {
            inner_ratio: 0.72,
            gap: 3.0,
            rounded: true,
            center: true,
            center_label: "Total",
        }
    }
}

/// One segment on the ring, in angles (radians clockwise from 12) and px.
#[derive(Debug, Clone, PartialEq)]
pub struct ArcMark {
    pub cx: f64,
    pub cy: f64,
    /// Centreline radius.
    pub r: f64,
    pub thickness: f64,
    pub a0: f64,
    pub a1: f64,
    pub value: f64,
    pub share: f64,
    pub label: String,
}

impl ArcMark {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        let l = |x: f64, y: f64| x + (y - x) * t;
        Self {
            cx: l(a.cx, b.cx),
            cy: l(a.cy, b.cy),
            r: l(a.r, b.r),
            thickness: l(a.thickness, b.thickness),
            a0: l(a.a0, b.a0),
            a1: l(a.a1, b.a1),
            value: l(a.value, b.value),
            share: l(a.share, b.share),
            label: b.label.clone(),
        }
    }

    /// The centreline path, shortened at both ends by the gap and, when
    /// rounded, by the cap radius so caps never overlap a neighbour. A
    /// segment too short for that becomes a dot.
    pub fn path(&self, opts: ArcOptions) -> String {
        if self.r <= 0.0 {
            return String::new();
        }
        let cap = if opts.rounded {
            self.thickness / 2.0
        } else {
            0.0
        };
        let pad = (opts.gap / 2.0 + cap) / self.r;
        let (mut s, mut e) = (self.a0 + pad, self.a1 - pad);
        if e <= s {
            let mid = (self.a0 + self.a1) / 2.0;
            s = mid - 1e-4;
            e = mid + 1e-4;
        }
        let pt = |a: f64| (self.cx + self.r * a.sin(), self.cy - self.r * a.cos());
        let (x0, y0) = pt(s);
        let (x1, y1) = pt(e);
        let mut p = Path::new();
        p.move_to(x0, y0)
            .arc_to(self.r, e - s > std::f64::consts::PI, true, x1, y1);
        p.into_string()
    }
}

/// The segment for series `si` among `all`, from each series' first value.
pub fn arc_geometry(f: &Frame, all: &[Series], si: usize, opts: ArcOptions) -> ArcMark {
    let (cx, cy) = f.plot.center();
    let outer = (f.plot.width.min(f.plot.height) / 2.0).max(0.0);
    let inner = outer * opts.inner_ratio.clamp(0.0, 0.95);
    let values: Vec<f64> = all
        .iter()
        .map(|s| s.points.first().map_or(0.0, |p| p.y))
        .collect();
    let slices = geom::pie(&values);
    let total: f64 = values.iter().filter(|v| v.is_finite() && **v > 0.0).sum();
    let (a0, a1) = slices[si];
    ArcMark {
        cx,
        cy,
        r: (inner + outer) / 2.0,
        thickness: outer - inner,
        a0,
        a1,
        value: values[si],
        share: if total > 0.0 { values[si] / total } else { 0.0 },
        label: all[si].label.clone(),
    }
}

/// Segments for each series' first point value, centred in the plot.
pub fn arcs<S>(series: S, opts: ArcOptions) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    layer(move |ctx| {
        let hovered: Mutable<Option<String>> = Mutable::new(None);
        let series = series.broadcast();
        let ring = keyed_marks(
            ctx,
            series.signal_cloned(),
            "dviz-arcs",
            false,
            move |frame, all, si| vec![((), arc_geometry(frame, all, si, opts))],
            {
                let hovered = hovered.clone();
                move |ctx: &Rc<ChartContext>, _: &(), input: MarkInput<ArcMark>| {
                    segment(ctx, input, opts, &hovered)
                }
            },
        );
        let center = opts
            .center
            .then(|| center_figure(ctx, series.signal_cloned(), hovered.clone(), opts));
        svg!("g", {
            .child(ring)
            .apply_if(center.is_some(), |b| b.child(center.unwrap()))
        })
    })
}

fn segment(
    ctx: &Rc<ChartContext>,
    input: MarkInput<ArcMark>,
    opts: ArcOptions,
    hovered: &Mutable<Option<String>>,
) -> Dom {
    let color = ctx.color_for(&input.series_id);
    let id = input.series_id.clone();
    // Sweep in from 12 o'clock on first render, then tween angle changes.
    let (shown, driver) = tween(
        input.mark.signal_cloned(),
        650.0,
        ctx.motion_enabled(),
        |goal: &ArcMark| ArcMark {
            a0: 0.0,
            a1: 0.0,
            ..goal.clone()
        },
        ArcMark::lerp,
    );
    let shown = shown.broadcast();
    let tip = shown.signal_ref({
        let color = color.clone();
        move |m| {
            let m = m.clone().unwrap_or_else(|| ArcMark {
                cx: 0.0,
                cy: 0.0,
                r: 0.0,
                thickness: 0.0,
                a0: 0.0,
                a1: 0.0,
                value: 0.0,
                share: 0.0,
                label: String::new(),
            });
            let mid = (m.a0 + m.a1) / 2.0;
            Tooltip {
                x: m.cx + m.r * mid.sin(),
                y: m.cy - m.r * mid.cos(),
                title: m.label.clone(),
                rows: vec![
                    TooltipRow {
                        color: Some(color.clone()),
                        label: "value".into(),
                        value: format::compact(m.value),
                    },
                    TooltipRow {
                        color: None,
                        label: "share".into(),
                        value: format::percent(m.share, 1),
                    },
                ],
            }
        }
    });
    let tooltip = ctx.tooltip().clone();
    let is_hovered = hovered
        .signal_ref(clone!(id => move |h| h.as_deref() == Some(id.as_str())))
        .dedupe();
    svg!("path", {
        .class("dviz-arc")
        .future(driver)
        .attr("tabindex", "0")
        .attr("data-series", &input.series_id)
        .attr("fill", "none")
        .apply_if(opts.rounded, |b| b.attr("stroke-linecap", "round"))
        .attr("style", &format!("stroke: {color}; color: {color}"))
        .attr_signal("stroke-width", shown.signal_ref(|m| m.as_ref().map_or("0".into(), |m| px(m.thickness))))
        .attr_signal("d", shown.signal_ref(move |m| m.as_ref().map_or(String::new(), |m| m.path(opts))))
        .attr_signal("data-hovered", is_hovered.map(|h| if h { "true" } else { "false" }))
        .attr_signal("transform-origin", shown.signal_ref(|m| m.as_ref().map_or("0 0".into(), |m| format!("{} {}", px(m.cx), px(m.cy)))))
        .event(clone!(hovered, id => move |_: events::PointerEnter| hovered.set_neq(Some(id.clone()))))
        .event(clone!(hovered => move |_: events::PointerLeave| hovered.set_neq(None)))
        .event(clone!(hovered, id => move |_: events::Focus| hovered.set_neq(Some(id.clone()))))
        .event(clone!(hovered => move |_: events::Blur| hovered.set_neq(None)))
        .apply_if(!opts.center, |b| hoverable(b, tooltip, tip))
    })
}

fn center_figure<S>(
    ctx: &Rc<ChartContext>,
    series: S,
    hovered: Mutable<Option<String>>,
    opts: ArcOptions,
) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    let frame = ctx.frame();
    let series = series.broadcast();
    let content = map_ref! {
        let all = series.signal_cloned(),
        let h = hovered.signal_cloned() => {
            let total: f64 = all.iter().filter_map(|s| s.points.first()).map(|p| p.y).filter(|v| v.is_finite() && *v > 0.0).sum();
            match h.as_ref().and_then(|id| all.iter().find(|s| s.id == *id)) {
                Some(s) => {
                    let v = s.points.first().map_or(0.0, |p| p.y);
                    let share = if total > 0.0 { v / total } else { 0.0 };
                    (format::compact(v), format!("{} · {}", s.label, format::percent(share, 1)))
                }
                None => (format::compact(total), opts.center_label.to_string()),
            }
        }
    }
    .broadcast();
    let cx = || frame.signal_ref(|f| px(f.plot.center().0));
    let cy = || frame.signal_ref(|f| px(f.plot.center().1));
    let size = frame.signal_ref(|f| {
        let outer = f.plot.width.min(f.plot.height) / 2.0;
        format!("{}px", (outer * 0.34).clamp(14.0, 40.0))
    });
    svg!("g", {
        .class("dviz-donut-center")
        .attr("aria-hidden", "true")
        .child(svg!("text", {
            .class("dviz-donut-value")
            .attr("text-anchor", "middle")
            .attr("dy", "0.1em")
            .attr_signal("x", cx())
            .attr_signal("y", cy())
            .attr_signal("font-size", size)
            .text_signal(content.signal_ref(|(v, _)| v.clone()))
        }))
        .child(svg!("text", {
            .class("dviz-donut-label")
            .attr("text-anchor", "middle")
            .attr("dy", "1.9em")
            .attr_signal("x", cx())
            .attr_signal("y", cy())
            .text_signal(content.signal_ref(|(_, l)| l.clone()))
        }))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_segments_become_dots_and_paths_stay_valid() {
        let m = ArcMark {
            cx: 100.0,
            cy: 100.0,
            r: 40.0,
            thickness: 12.0,
            a0: 0.0,
            a1: 0.05,
            value: 1.0,
            share: 0.01,
            label: "x".into(),
        };
        let d = m.path(ArcOptions::default());
        assert!(d.starts_with('M') && d.contains('A'), "{d}");
        let wide = ArcMark {
            a1: 4.0,
            ..m.clone()
        };
        assert!(
            wide.path(ArcOptions::default()).contains(" 0 1 1 "),
            "large arc flag past half a turn"
        );
        let none = ArcMark { r: 0.0, ..m };
        assert!(none.path(ArcOptions::default()).is_empty());
    }
}
