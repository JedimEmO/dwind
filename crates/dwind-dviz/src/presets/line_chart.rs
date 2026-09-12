use std::rc::Rc;

use crate::layers::width_tap::width_tap;
use dominator::{Dom, html};
use dwind_dviz_core::data::{Extent, Series};
use dwind_dviz_core::geom::Curve;
use dwind_dviz_data::Strategy;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

use super::{Zoom, with_legend, zoom_reset};
use crate::chart::{ChartProps, SeriesSlots, YDomain, chart};
use crate::domains::{XKind, y_domain_for};
use crate::layers::axis::{axis_x, axis_y};
use crate::layers::grid::grid;
use crate::layers::interaction::{CrosshairOptions, crosshair_with};
use crate::layers::labels::line_end_labels;
use crate::layers::line::{LineOptions, Wash, line_with};
use crate::visibility::SeriesVisibility;
use futures_signals::map_ref;

/// Applies a brushed range to the x domain of a zoomable chart.
pub(crate) fn zoom_domain(
    base: crate::chart::XDomain,
    zoom: Option<Extent<f64>>,
) -> crate::chart::XDomain {
    use crate::chart::XDomain;
    use dwind_dviz_core::jiff::Timestamp;
    match (base, zoom) {
        (XDomain::Linear(_), Some(z)) => XDomain::Linear(z),
        (XDomain::Time { tz, .. }, Some(z)) => {
            let ts = |ms: f64| {
                Timestamp::from_millisecond(ms.round() as i64).unwrap_or(Timestamp::UNIX_EPOCH)
            };
            XDomain::Time {
                extent: Extent::new(ts(z.min), ts(z.max)),
                tz,
            }
        }
        (base, _) => base,
    }
}

/// Crosshair options for a preset: brushing sets the zoom when enabled.
pub(crate) fn interaction_opts(zoomable: bool, zoom: &Zoom) -> CrosshairOptions {
    if zoomable {
        let z = zoom.clone();
        CrosshairOptions::brush(move |e| z.set(Some(e)))
    } else {
        CrosshairOptions {
            crosshair: true,
            ..Default::default()
        }
    }
}

/// Series over which direct end labels are dropped in favour of the legend.
pub const MAX_END_LABELLED: usize = 4;

/// A line chart: grid, axes, one line per series, a legend for two or more
/// series and series names at the line ends for up to four.
#[component(render_fn = line_chart)]
struct LineChart {
    #[signal]
    #[default(vec![])]
    series: Vec<Series>,

    /// Accessible name: what is plotted and what the axes measure.
    #[default(String::new())]
    label: String,

    #[default(240.0)]
    height: f64,

    #[default(XKind::Linear)]
    x: XKind,

    /// Fix the y domain (a live chart should, so the axis does not jitter).
    #[default(None)]
    y: Option<Extent<f64>>,

    #[default(false)]
    include_zero: bool,

    #[default(Curve::Linear)]
    curve: Curve,

    /// A faint wash under the line when there is exactly one series.
    #[default(true)]
    wash_single: bool,

    #[default(true)]
    legend: bool,

    #[default(true)]
    end_labels: bool,

    #[default(None)]
    slots: Option<Rc<SeriesSlots>>,

    /// Legend items toggle their series.
    #[default(true)]
    toggle_legend: bool,

    /// Drag on the plot to zoom the x axis; double-click or the reset
    /// button restores it.
    #[default(false)]
    zoomable: bool,

    /// Override the x extent derived from the data (a live window from
    /// `WindowedSource::window_signal`). In the x kind's units.
    #[signal]
    #[default(None)]
    x_extent: Option<Extent<f64>>,

    /// Reduce each series to what the plot width can show (min/max
    /// buckets), so a long history stays cheap to draw.
    #[default(false)]
    downsample: bool,

    /// Mark the series as live: a pulsing halo on the newest point.
    #[default(false)]
    live: bool,
}

pub fn line_chart(props: LineChartProps) -> Dom {
    let LineChartProps {
        series,
        label,
        height,
        x,
        y,
        include_zero,
        curve,
        wash_single,
        legend,
        end_labels,
        slots,
        toggle_legend,
        zoomable,
        x_extent,
        downsample,
        live,
        apply,
    } = props;

    let all_series = series.broadcast();
    let visibility = toggle_legend.then(SeriesVisibility::new);
    let visible = match &visibility {
        Some(v) => v.filter(all_series.signal_cloned()).boxed_local(),
        None => all_series.signal_cloned().boxed_local(),
    }
    .broadcast();
    let width = Mutable::new(1.0f64);
    let series = if downsample {
        dwind_dviz_data::downsample(visible.signal_cloned(), width.signal(), Strategy::MinMax)
            .boxed_local()
    } else {
        visible.signal_cloned().boxed_local()
    }
    .broadcast();
    let slots = slots.unwrap_or_else(SeriesSlots::new);
    let zoom: Zoom = Zoom::new(None);
    let x_domain = {
        let x = x.clone();
        map_ref! {
            let s = series.signal_cloned(),
            let z = zoom.signal(),
            let fixed = x_extent => {
                let base = match fixed {
                    Some(e) => zoom_domain(x.domain_for(&[], 0.0), Some(*e)),
                    None => x.domain_for(s, 0.0),
                };
                zoom_domain(base, *z)
            }
        }
    };
    let y_domain = series.signal_ref(move |s| match y {
        Some(e) => YDomain::Linear(e),
        None => y_domain_for(s, include_zero, 0.0),
    });
    let labelled = series.signal_ref(|s| {
        if s.len() <= MAX_END_LABELLED {
            s.clone()
        } else {
            vec![]
        }
    });

    let chart = chart(
        ChartProps::new()
            .label(label)
            .height(height)
            .slots(Some(slots.clone()))
            .x_domain_signal(x_domain)
            .y_domain_signal(y_domain)
            .layers({
                let mut layers = vec![
                    grid(),
                    axis_x(),
                    axis_y(),
                    line_with(
                        series.signal_cloned(),
                        LineOptions {
                            curve,
                            wash: if wash_single {
                                Wash::WhenSingle
                            } else {
                                Wash::Never
                            },
                            live,
                            ..Default::default()
                        },
                    ),
                ];
                if end_labels {
                    layers.push(line_end_labels(labelled));
                }
                layers.push(crosshair_with(
                    series.signal_cloned(),
                    interaction_opts(zoomable, &zoom),
                ));
                if downsample {
                    layers.push(width_tap(width.clone()));
                }
                layers
            })
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
