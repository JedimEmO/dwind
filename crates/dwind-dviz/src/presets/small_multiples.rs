use std::rc::Rc;

use dominator::{Dom, html};
use dwind_dviz_core::data::{Extent, Series};
use dwind_dviz_core::geom::Curve;
use futures_signals::signal::{Signal, SignalExt};

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
    let slots = SeriesSlots::new();
    html!("div", {
        .class("dviz-multiples")
        .style("grid-template-columns", format!("repeat({}, minmax(0, 1fr))", opts.columns.max(1)))
        .children_signal_vec(series.map(move |all| {
            let shared = opts.shared_y.then(|| match y_domain_for(&all, false, 0.05) {
                YDomain::Linear(e) => e,
                _ => Extent::UNIT,
            });
            all.into_iter()
                .map(|s| panel(s, shared, &opts, slots.clone()))
                .collect::<Vec<_>>()
        }).to_signal_vec())
    })
}

fn panel(
    s: Series,
    shared: Option<Extent<f64>>,
    opts: &SmallMultiplesOptions,
    slots: Rc<SeriesSlots>,
) -> Dom {
    let title = s.label.clone();
    html!("div", {
        .class("dviz-multiple")
        .child(html!("div", {
            .class("dviz-multiple-title")
            .text(&title)
        }))
        .child(line_chart(LineChartProps::new()
            .label(title.clone())
            .height(opts.height)
            .x(opts.x.clone())
            .curve(opts.curve)
            .y(shared)
            .legend(false)
            .end_labels(false)
            .slots(Some(slots))
            .series(vec![s])))
    })
}
