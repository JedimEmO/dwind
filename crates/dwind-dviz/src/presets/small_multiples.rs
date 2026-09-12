use std::rc::Rc;

use dominator::{Dom, html};
use dwind_dviz_core::data::{Extent, Series};
use dwind_dviz_core::geom::Curve;
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::SignalVecExt;

use super::line_chart::{LineChartProps, line_chart};
use crate::chart::SeriesSlots;
use crate::chart::YDomain;
use crate::domains::{XKind, y_domain_for};

#[derive(Debug, Clone, PartialEq)]
pub struct SmallMultiplesOptions {
    pub columns: usize,
    pub height: f64,
    pub x: XKind,
    pub curve: Curve,
    /// Share one y domain across every panel (the union), so panels are
    /// comparable. Off, each panel scales to itself.
    pub shared_y: bool,
}

impl Default for SmallMultiplesOptions {
    fn default() -> Self {
        Self {
            columns: 3,
            height: 140.0,
            x: XKind::Linear,
            curve: Curve::Linear,
            shared_y: true,
        }
    }
}

/// One small line chart per series in a grid, each titled by its series
/// label. The answer to "too many series for one plot" and to two measures
/// of different scale (never a dual axis).
pub fn small_multiples<S>(series: S, opts: SmallMultiplesOptions) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    let series = series.broadcast();
    let slots = SeriesSlots::new();
    let (panels, driver) = crate::keyed::diffed(
        series.signal_ref(|all| all.iter().map(|s| s.id.clone()).collect::<Vec<_>>()),
    );
    html!("div", {
        .class("dviz-multiples")
        .style("grid-template-columns", format!("repeat({}, minmax(0, 1fr))", opts.columns.max(1)))
        .future(driver)
        .children_signal_vec(panels.signal_vec_cloned().map(move |id| {
            let series = series.clone();
            let opts = opts.clone();
            let slots = slots.clone();
            panel_signal(id, series.signal_cloned(), opts, slots)
        }))
    })
}

fn panel_signal<S>(
    id: String,
    series: S,
    opts: SmallMultiplesOptions,
    slots: Rc<SeriesSlots>,
) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    let series = series.broadcast();
    let title = series
        .signal_ref({
            let id = id.clone();
            move |all| {
                all.iter()
                    .find(|s| s.id == id)
                    .map(|s| s.label.clone())
                    .unwrap_or_default()
            }
        })
        .broadcast();
    let one = series
        .signal_ref(move |all| {
            all.iter()
                .find(|s| s.id == id)
                .cloned()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .boxed_local();
    let shared_y = if opts.shared_y {
        series
            .signal_ref(|all| match y_domain_for(all, false, 0.05) {
                YDomain::Linear(e) => Some(e),
                _ => Some(Extent::UNIT),
            })
            .boxed_local()
    } else {
        futures_signals::signal::always(None).boxed_local()
    };
    html!("div", {
        .class("dviz-multiple")
        .child(html!("div", {
            .class("dviz-multiple-title")
            .text_signal(title.signal_cloned())
        }))
        .child(line_chart(LineChartProps::new()
            .label_signal(title.signal_cloned())
            .height(opts.height)
            .x(opts.x)
            .curve(opts.curve)
            .y_signal(shared_y)
            .legend(false)
            .end_labels(false)
            .slots(Some(slots))
            .series_signal(one)))
    })
}
