use std::rc::Rc;

use dominator::{Dom, html};
use dwind_dviz_core::data::{Extent, Series};
use dwind_dviz_core::geom::Curve;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

use super::line_chart::{interaction_opts, zoom_domain};
use super::{Zoom, with_legend, zoom_reset};
use crate::chart::{ChartProps, SeriesSlots, YDomain, chart};
use crate::domains::{XKind, y_domain_for, y_domain_for_stacked};
use crate::layers::area::{AreaOptions, area_with};
use crate::layers::axis::{axis_x, axis_y};
use crate::layers::grid::grid;
use crate::layers::interaction::crosshair_with;
use crate::visibility::SeriesVisibility;
use futures_signals::map_ref;

/// An area chart. Areas always grow from zero; `stacked` tiles the series.
#[component(render_fn = area_chart)]
struct AreaChart {
    #[signal]
    #[default(vec![])]
    series: Vec<Series>,

    #[signal]
    #[default(String::new())]
    label: String,

    #[default(240.0)]
    height: f64,

    #[default(XKind::Linear)]
    x: XKind,

    #[signal]
    #[default(None)]
    y: Option<Extent<f64>>,

    #[default(false)]
    stacked: bool,

    #[default(Curve::Linear)]
    curve: Curve,

    #[default(true)]
    legend: bool,

    #[default(None)]
    slots: Option<Rc<SeriesSlots>>,

    /// Legend items toggle their series.
    #[default(true)]
    toggle_legend: bool,

    /// Drag on the plot to zoom the x axis; double-click or the reset
    /// button restores it.
    #[default(false)]
    zoomable: bool,
}

pub fn area_chart(props: AreaChartProps) -> Dom {
    let AreaChartProps {
        series,
        label,
        height,
        x,
        y,
        stacked,
        curve,
        legend,
        slots,
        toggle_legend,
        zoomable,
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
    let zoom: Zoom = Zoom::new(None);
    let x_domain = {
        let x = x.clone();
        map_ref! {
            let s = series.signal_cloned(),
            let z = zoom.signal() => zoom_domain(x.domain_for(s, 0.0), *z)
        }
    };
    let y_domain = map_ref! {
        let s = series.signal_cloned(),
        let y = y => match *y {
            Some(e) => YDomain::Linear(e),
            None if stacked => y_domain_for_stacked(s),
            None => y_domain_for(s, true, 0.0),
        }
    };
    let opts = AreaOptions {
        curve,
        ..if stacked {
            AreaOptions::stacked()
        } else {
            AreaOptions::default()
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
                area_with(series.signal_cloned(), opts),
                crosshair_with(series.signal_cloned(), interaction_opts(zoomable, &zoom)),
            ])
            .apply(move |b| match apply {
                Some(a) => a(b),
                None => b,
            }),
    );
    let figure = html!("div", {
        .class("dviz-figure")
        .child(zoom_reset(&zoom))
        .child(chart)
    });
    with_legend(
        legend,
        all_series.signal_cloned(),
        slots,
        visibility,
        figure,
    )
}
