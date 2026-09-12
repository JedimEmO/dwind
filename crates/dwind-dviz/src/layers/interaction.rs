//! Pointer and keyboard interaction over the plot: a crosshair that snaps
//! to the nearest x, a tooltip listing every series there, arrow-key
//! navigation, and an optional brush that reports a selected x range.
//!
//! The tooltip enhances, never gates: every value is also on an axis, a
//! label, or in the table view.

use std::cell::RefCell;
use std::rc::Rc;

use dominator::{Dom, clone, events, svg, with_node};
use dwind_dviz_core::data::{Extent, Series};
use dwind_dviz_core::format;
use futures_signals::map_ref;
use futures_signals::signal::Signal;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use wasm_bindgen::JsCast;

use super::px;
use crate::chart::{ChartContext, Layer, Tooltip, TooltipRow, layer};

/// Called with the brushed x range in data units when a drag ends.
pub type BrushHandler = Rc<dyn Fn(Extent<f64>)>;
pub type HoverHandler = Rc<dyn Fn(Option<Snap>)>;
pub type SelectionHandler = Rc<dyn Fn(Snap)>;

#[derive(Clone, Default)]
pub struct CrosshairOptions {
    /// Draw the vertical line and the ring markers.
    pub crosshair: bool,
    /// Report drags wider than a few pixels as an x range (zoom).
    pub on_brush: Option<BrushHandler>,
    /// Receives the current snapped point, or `None` when hover clears.
    pub on_hover: Option<HoverHandler>,
    /// Receives the current snapped point when the plot is clicked.
    pub on_select: Option<SelectionHandler>,
    /// Format a y value for the tooltip. Defaults to the compact figure.
    pub format_y: Option<Rc<dyn Fn(f64) -> String>>,
}

impl CrosshairOptions {
    pub fn brush(handler: impl Fn(Extent<f64>) + 'static) -> Self {
        Self {
            crosshair: true,
            on_brush: Some(Rc::new(handler)),
            on_hover: None,
            on_select: None,
            format_y: None,
        }
    }

    pub fn hover(handler: impl Fn(Option<Snap>) + 'static) -> Self {
        Self {
            crosshair: true,
            on_hover: Some(Rc::new(handler)),
            ..Default::default()
        }
    }

    pub fn select(handler: impl Fn(Snap) + 'static) -> Self {
        Self {
            crosshair: true,
            on_select: Some(Rc::new(handler)),
            ..Default::default()
        }
    }
}

/// Crosshair and tooltip for line and area charts, plus keyboard access.
pub fn crosshair<S>(series: S) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    crosshair_with(
        series,
        CrosshairOptions {
            crosshair: true,
            ..Default::default()
        },
    )
}

/// The snapped hover: the data x and, per series, the nearest point.
#[derive(Debug, Clone, PartialEq)]
pub struct Snap {
    pub x: f64,
    /// `(series index, point index)` for every series with a defined point
    /// near `x`.
    pub hits: Vec<(usize, usize)>,
}

/// Finds, for each series, the point nearest to data `x` (by x), and snaps
/// `x` to the closest of those. Assumes points are sorted by x.
pub fn snap(all: &[Series], x: f64) -> Option<Snap> {
    let mut best: Option<(f64, f64)> = None; // (distance, x)
    let mut hits = Vec::new();
    for (si, s) in all.iter().enumerate() {
        let pts = &s.points;
        if pts.is_empty() {
            continue;
        }
        let i = pts.partition_point(|p| p.x < x);
        let candidates = [i.checked_sub(1), Some(i)]
            .into_iter()
            .flatten()
            .filter(|&k| k < pts.len() && pts[k].is_defined());
        if let Some(k) = candidates.min_by(|&a, &b| {
            (pts[a].x - x)
                .abs()
                .partial_cmp(&(pts[b].x - x).abs())
                .unwrap()
        }) {
            let d = (pts[k].x - x).abs();
            if best.is_none_or(|(bd, _)| d < bd) {
                best = Some((d, pts[k].x));
            }
            hits.push((si, k));
        }
    }
    let (_, sx) = best?;
    // Keep only hits at the snapped x (within a hair), so one tooltip row
    // per series that actually has a sample there.
    let hits = hits
        .into_iter()
        .filter(|&(si, k)| (all[si].points[k].x - sx).abs() <= f64::EPSILON.max(sx.abs() * 1e-9))
        .collect();
    Some(Snap { x: sx, hits })
}

pub fn crosshair_with<S>(series: S, opts: CrosshairOptions) -> Layer
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    layer(move |ctx| {
        // A synchronous mirror of the series for the event handlers.
        let data: Mutable<Vec<Series>> = Mutable::new(Vec::new());
        let feed = series.for_each(clone!(data => move |v| {
            data.set(v);
            async {}
        }));
        let series = data;
        let ctx = ctx.clone();
        let frame = ctx.frame();
        let snap_state: Mutable<Option<Snap>> = Mutable::new(None);
        let drag: Mutable<Option<(f64, f64)>> = Mutable::new(None); // plot-local px
        let format_y = opts
            .format_y
            .clone()
            .unwrap_or_else(|| Rc::new(format::compact));
        let on_hover = opts.on_hover.clone();
        let on_select = opts.on_select.clone();
        // The current point index for keyboard stepping (into series 0).
        let key_index: Rc<RefCell<Option<usize>>> = Rc::new(RefCell::new(None));

        // Applies a hover at data x: snaps, sets the crosshair and tooltip.
        let hover_at = {
            let ctx = ctx.clone();
            let series = series.clone();
            let snap_state = snap_state.clone();
            let format_y = format_y.clone();
            let on_hover = on_hover.clone();
            Rc::new(move |x: f64| {
                let f = ctx.frame().get_cloned();
                let all = series.lock_ref();
                let Some(sn) = snap(&all, x) else {
                    snap_state.set(None);
                    ctx.hover_x().set(None);
                    ctx.tooltip().set(None);
                    if let Some(handler) = on_hover.as_ref() {
                        handler(None);
                    }
                    return;
                };
                let px_x = f.x.map(sn.x);
                if px_x < f.plot.x - 0.5 || px_x > f.plot.right() + 0.5 {
                    snap_state.set(None);
                    ctx.hover_x().set(None);
                    ctx.tooltip().set(None);
                    if let Some(handler) = on_hover.as_ref() {
                        handler(None);
                    }
                    return;
                }
                let rows: Vec<TooltipRow> = sn
                    .hits
                    .iter()
                    .map(|&(si, k)| TooltipRow {
                        color: Some(ctx.color_for(&all[si].id)),
                        label: all[si].label.clone(),
                        value: format_y(all[si].points[k].y),
                    })
                    .collect();
                let y_anchor = sn
                    .hits
                    .iter()
                    .map(|&(si, k)| f.y.map(all[si].points[k].y))
                    .filter(|v| v.is_finite())
                    .fold(f64::INFINITY, f64::min);
                ctx.tooltip().set(Some(Tooltip {
                    x: px_x,
                    y: if y_anchor.is_finite() {
                        y_anchor
                    } else {
                        f.plot.y
                    },
                    title: f.x.describe(sn.x),
                    rows,
                }));
                ctx.hover_x().set(Some(sn.x));
                snap_state.set(Some(sn.clone()));
                if let Some(handler) = on_hover.as_ref() {
                    handler(Some(sn));
                }
            })
        };
        let clear = {
            let ctx = ctx.clone();
            let snap_state = snap_state.clone();
            let on_hover = on_hover.clone();
            Rc::new(move || {
                snap_state.set(None);
                ctx.hover_x().set(None);
                ctx.tooltip().set(None);
                if let Some(handler) = on_hover.as_ref() {
                    handler(None);
                }
            })
        };

        let markers = crosshair_markers(&ctx, &series, &snap_state);
        let brush_rect = map_ref! {
            let f = frame.signal_cloned(),
            let d = drag.signal() => {
                d.map(|(a, b)| {
                    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
                    svg!("rect", {
                        .class("dviz-brush")
                        .attr("x", &px(f.plot.x + lo))
                        .attr("y", &px(f.plot.y))
                        .attr("width", &px(hi - lo))
                        .attr("height", &px(f.plot.height))
                    })
                })
            }
        };

        let on_brush = opts.on_brush.clone();
        let show_crosshair = opts.crosshair;

        let clip = ctx.clip_url();
        svg!("g", {
            .class("dviz-interaction")
            .future(feed)
            .apply_if(show_crosshair, |b| b.child(svg!("g", {
                .attr("clip-path", &clip)
                .child(markers)
            })))
            .child_signal(brush_rect)
            .child(svg!("rect", {
                .class("dviz-overlay")
                .attr("tabindex", "0")
                .attr("role", "group")
                .attr("aria-label", "Chart data. Use the arrow keys to move between points.")
                .attr_signal("x", frame.signal_ref(|f| px(f.plot.x)))
                .attr_signal("y", frame.signal_ref(|f| px(f.plot.y)))
                .attr_signal("width", frame.signal_ref(|f| px(f.plot.width.max(0.0))))
                .attr_signal("height", frame.signal_ref(|f| px(f.plot.height.max(0.0))))
                .with_node!(el => {
                    .event(clone!(el, hover_at, ctx, drag => move |e: events::PointerMove| {
                        let rect = el.get_bounding_client_rect();
                        let local = e.x() as f64 - rect.left();
                        let f = ctx.frame().get_cloned();
                        if drag.get().is_some() {
                            drag.set(drag.get().map(|(a, _)| (a, local.clamp(0.0, f.plot.width))));
                        }
                        hover_at(f.x.invert(f.plot.x + local));
                    }))
                    .event(clone!(el, drag, on_brush => move |e: events::PointerDown| {
                        if on_brush.is_none() {
                            return;
                        }
                        let rect = el.get_bounding_client_rect();
                        let local = e.x() as f64 - rect.left();
                        drag.set(Some((local, local)));
                        let _ = el.set_pointer_capture(e.pointer_id());
                    }))
                    .event(clone!(drag, on_brush, ctx => move |_: events::PointerUp| {
                        if let (Some((a, b)), Some(handler)) = (drag.get(), on_brush.as_ref()) {
                            drag.set(None);
                            if (a - b).abs() >= 4.0 {
                                let f = ctx.frame().get_cloned();
                                let (lo, hi) = if a < b { (a, b) } else { (b, a) };
                                handler(Extent::new(f.x.invert(f.plot.x + lo), f.x.invert(f.plot.x + hi)));
                            }
                        }
                    }))
                })
                .event(clone!(clear, drag => move |_: events::PointerLeave| {
                    if drag.get().is_none() {
                        clear();
                    }
                }))
                .event(clone!(drag, clear => move |_: events::PointerCancel| {
                    drag.set(None);
                    clear();
                }))
                .event(clone!(snap_state, on_select => move |_: events::Click| {
                    if let (Some(selection), Some(handler)) =
                        (snap_state.get_cloned(), on_select.as_ref())
                    {
                        handler(selection);
                    }
                }))
                .event(clone!(hover_at, series, key_index => move |_: events::Focus| {
                    let all = series.lock_ref();
                    if let Some(s) = all.first() {
                        let last = s.points.iter().rposition(|p| p.is_defined());
                        *key_index.borrow_mut() = last;
                        if let Some(i) = last {
                            hover_at(s.points[i].x);
                        }
                    }
                }))
                .event(clone!(clear, key_index => move |_: events::Blur| {
                    *key_index.borrow_mut() = None;
                    clear();
                }))
                .event(clone!(hover_at, clear, series, key_index => move |e: events::KeyDown| {
                    let all = series.lock_ref();
                    let Some(s) = all.first() else { return };
                    let n = s.points.len();
                    if n == 0 {
                        return;
                    }
                    let current = key_index.borrow().unwrap_or(n - 1);
                    let next = match e.key().as_str() {
                        "ArrowLeft" | "ArrowDown" => Some(current.saturating_sub(1)),
                        "ArrowRight" | "ArrowUp" => Some((current + 1).min(n - 1)),
                        "Home" => Some(0),
                        "End" => Some(n - 1),
                        "Escape" => {
                            clear();
                            None
                        }
                        _ => return,
                    };
                    e.prevent_default();
                    if let Some(i) = next {
                        *key_index.borrow_mut() = Some(i);
                        hover_at(s.points[i].x);
                    }
                }))
            }))
        })
    })
}

/// The crosshair line and one dot per series, as persistent nodes whose
/// positions follow the snap state. Nothing is rebuilt per pointer move.
fn crosshair_markers(
    ctx: &Rc<ChartContext>,
    series: &Mutable<Vec<Series>>,
    snap_state: &Mutable<Option<Snap>>,
) -> Dom {
    let frame = ctx.frame();
    let ctx = ctx.clone();
    let series = series.clone();
    let snap_state = snap_state.clone();
    let x_px = map_ref! {
        let f = frame.signal_cloned(),
        let sn = snap_state.signal_cloned() => sn.as_ref().map(|sn| f.x.map(sn.x))
    }
    .broadcast();
    let visible = x_px
        .signal_ref(|x| x.is_some_and(|x| x.is_finite()))
        .dedupe();
    let (ids, ids_driver) = crate::keyed::diffed(
        series
            .signal_ref(|v| v.iter().map(|s| s.id.clone()).collect::<Vec<_>>())
            .dedupe_cloned(),
    );
    svg!("g", {
        .class("dviz-crosshair-group")
        .future(ids_driver)
        .attr_signal("opacity", visible.map(|v| if v { "1" } else { "0" }))
        .child(svg!("line", {
            .class("dviz-crosshair")
            .attr_signal("x1", x_px.signal_ref(|x| px(x.unwrap_or(0.0))))
            .attr_signal("x2", x_px.signal_ref(|x| px(x.unwrap_or(0.0))))
            .attr_signal("y1", frame.signal_ref(|f| px(f.plot.y)))
            .attr_signal("y2", frame.signal_ref(|f| px(f.plot.bottom())))
        }))
        .children_signal_vec(ids.signal_vec_cloned().map(move |id| {
            let c = ctx.color_for(&id);
            let wanted = id.clone();
            let pos = map_ref! {
                let f = frame.signal_cloned(),
                let sn = snap_state.signal_cloned(),
                let all = series.signal_cloned() => {
                    sn.as_ref().and_then(|sn| {
                        let si = all.iter().position(|s| s.id == wanted)?;
                        let (_, k) = sn.hits.iter().find(|(s, _)| *s == si)?;
                        let y = f.y.map(all[si].points[*k].y);
                        y.is_finite().then(|| (f.x.map(sn.x), y))
                    })
                }
            }
            .broadcast();
            svg!("circle", {
                .class("dviz-crosshair-dot")
                .attr("r", "4")
                .attr("style", &format!("fill: {c}; color: {c}"))
                .attr_signal("opacity", pos.signal_ref(|p| if p.is_some() { "1" } else { "0" }))
                .attr_signal("cx", pos.signal_ref(|p| px(p.map_or(0.0, |p| p.0))))
                .attr_signal("cy", pos.signal_ref(|p| px(p.map_or(0.0, |p| p.1))))
            })
        }))
    })
}

/// Silence the unused-import lint on non-wasm builds where `JsCast` is
/// only needed by the pointer-capture call.
#[allow(dead_code)]
fn _uses_jscast(v: &wasm_bindgen::JsValue) -> bool {
    v.dyn_ref::<web_sys::Element>().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dwind_dviz_core::data::Point;

    fn s(id: &str, pts: &[(f64, f64)]) -> Series {
        Series::new(id, id, pts.iter().map(|&(x, y)| Point::new(x, y)).collect())
    }

    #[test]
    fn snaps_to_the_nearest_sample_across_series() {
        let all = vec![
            s("a", &[(0.0, 1.0), (10.0, 2.0), (20.0, 3.0)]),
            s("b", &[(0.0, 5.0), (10.0, f64::NAN), (20.0, 7.0)]),
        ];
        let sn = snap(&all, 9.0).unwrap();
        assert_eq!(sn.x, 10.0);
        assert_eq!(sn.hits, vec![(0, 1)], "b has a gap at 10 and drops out");
        let sn = snap(&all, 100.0).unwrap();
        assert_eq!(sn.x, 20.0);
        assert_eq!(sn.hits.len(), 2);
        assert!(snap(&[], 1.0).is_none());
    }
}
