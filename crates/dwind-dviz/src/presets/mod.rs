//! Presets: the 90% case in one call. Each is a thin wrapper over the
//! layer API that derives the domains from the data, adds the standard
//! chrome, and applies the rules (legend for two or more series, selective
//! labels, zero baselines for bars and areas).

mod area_chart;
mod bar_chart;
mod donut_chart;
mod line_chart;
mod scatter_chart;
mod small_multiples;
mod sparkline;
mod stat_tile;

pub use area_chart::*;
pub use bar_chart::*;
pub use donut_chart::*;
pub use line_chart::*;
pub use scatter_chart::*;
pub use small_multiples::*;
pub use sparkline::*;
pub use stat_tile::*;

use std::rc::Rc;

use dominator::{Dom, html};
use dwind_dviz_core::data::Series;
use futures_signals::signal::Signal;

use crate::chart::SeriesSlots;
use crate::legend::legend_with;
use crate::visibility::SeriesVisibility;

/// A legend above a chart, in one column. With `visibility`, legend items
/// toggle their series.
pub(crate) fn with_legend<S>(
    show: bool,
    series: S,
    slots: Rc<SeriesSlots>,
    visibility: Option<Rc<SeriesVisibility>>,
    chart: Dom,
) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    html!("div", {
        .class("dviz-figure")
        .apply_if(show, |b| b.child(legend_with(series, slots, visibility)))
        .child(chart)
    })
}

/// Shared zoom state for brushable charts: the selected x extent, or none.
pub type Zoom = futures_signals::signal::Mutable<Option<dwind_dviz_core::data::Extent<f64>>>;

/// A small "Reset zoom" control, shown only while zoomed.
pub(crate) fn zoom_reset(zoom: &Zoom) -> Dom {
    use futures_signals::signal::SignalExt;
    let z = zoom.clone();
    html!("div", {
        .child_signal(zoom.signal_ref(|z| z.is_some()).dedupe().map(move |zoomed| {
            let z = z.clone();
            zoomed.then(|| html!("button", {
                .class("dviz-zoom-reset")
                .attr("type", "button")
                .text("Reset zoom")
                .event(move |_: dominator::events::Click| z.set(None))
            }))
        }))
    })
}
