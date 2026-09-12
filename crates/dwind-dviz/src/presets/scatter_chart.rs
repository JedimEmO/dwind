use std::rc::Rc;

use dominator::Dom;
use dwind_dviz_core::data::{Extent, Series};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

use super::with_legend;
use crate::chart::{ChartProps, SeriesSlots, YDomain, chart};
use crate::domains::{XKind, y_domain_for};
use crate::layers::axis::{axis_x, axis_y};
use crate::layers::grid::grid;
use crate::layers::points::points;
use crate::visibility::SeriesVisibility;

/// A scatter plot. Any two marks can touch here, so the palette's all-pairs
/// cap applies: keep it to three series or fold the rest into "Other".
#[component(render_fn = scatter_chart)]
struct ScatterChart {
    #[signal]
    #[default(vec![])]
    series: Vec<Series>,

    #[signal]
    #[default(String::new())]
    label: String,

    #[default(280.0)]
    height: f64,

    #[default(XKind::Linear)]
    x: XKind,

    #[signal]
    #[default(None)]
    y: Option<Extent<f64>>,

    #[default(true)]
    legend: bool,

    #[default(None)]
    slots: Option<Rc<SeriesSlots>>,

    /// Legend items toggle their series.
    #[default(true)]
    toggle_legend: bool,
}

pub fn scatter_chart(props: ScatterChartProps) -> Dom {
    let ScatterChartProps {
        series,
        label,
        height,
        x,
        y,
        legend,
        slots,
        toggle_legend,
        apply,
    } = props;

    let all_series = series.broadcast();
    let visibility = toggle_legend.then(SeriesVisibility::new);
    let series = match &visibility {
        Some(v) => v.filter(all_series.signal_cloned()).boxed_local(),
        None => all_series.signal_cloned().boxed_local(),
    }
    .broadcast();
    let slots = slots.unwrap_or_else(SeriesSlots::new);
    let x_domain = {
        let x = x.clone();
        series.signal_ref(move |s| x.domain_for(s, 0.05))
    };
    let y_domain = map_ref! {
        let s = series.signal_cloned(),
        let y = y => match *y {
            Some(e) => YDomain::Linear(e),
            None => y_domain_for(s, false, 0.05),
        }
    };

    let chart = chart(
        ChartProps::new()
            .label_signal(label)
            .height(height)
            .slots(Some(slots.clone()))
            .x_domain_signal(x_domain)
            .y_domain_signal(y_domain)
            .layers(vec![
                grid(),
                axis_x(),
                axis_y(),
                points(series.signal_cloned()),
            ])
            .apply(move |b| match apply {
                Some(a) => a(b),
                None => b,
            }),
    );
    with_legend(legend, all_series.signal_cloned(), slots, visibility, chart)
}
