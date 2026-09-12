use std::rc::Rc;

use dominator::{Dom, clone, events, html};
use dwind_dviz_core::data::{Point, Series};
use dwind_dviz_core::format;
use dwind_dviz_core::layout::Margins;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;

use crate::chart::{ChartProps, SeriesSlots, chart};
use crate::layers::arc::{ArcOptions, arcs};
use crate::visibility::SeriesVisibility;

/// Segments beyond this fold into "Other": past six, adjacent slices blur.
pub const MAX_SEGMENTS: usize = 6;

/// A donut for part-to-whole at a glance, with the total in the centre and
/// a value legend beside it. Each series contributes its first point's
/// value; smaller series beyond the sixth fold into "Other". For comparing
/// close values, use a bar chart.
#[component(render_fn = donut_chart)]
struct DonutChart {
    #[signal]
    #[default(vec![])]
    series: Vec<Series>,

    #[signal]
    #[default(String::new())]
    label: String,

    #[default(220.0)]
    size: f64,

    /// Label under the centre total.
    #[default("Total".to_string())]
    center_label: String,

    #[default(None)]
    slots: Option<Rc<SeriesSlots>>,
}

/// Keeps the largest `MAX_SEGMENTS - 1` series and folds the rest into an
/// "Other" series when there are more than `MAX_SEGMENTS`.
pub fn fold_other(series: &[Series]) -> Vec<Series> {
    if series.len() <= MAX_SEGMENTS {
        return series.to_vec();
    }
    let mut sorted: Vec<&Series> = series.iter().collect();
    let value = |s: &Series| s.points.first().map_or(0.0, |p| p.y);
    sorted.sort_by(|a, b| {
        value(b)
            .partial_cmp(&value(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let (keep, rest) = sorted.split_at(MAX_SEGMENTS - 1);
    let other: f64 = rest.iter().map(|s| value(s)).sum();
    let mut out: Vec<Series> = keep.iter().map(|s| (*s).clone()).collect();
    out.push(Series::new("other", "Other", vec![Point::new(0.0, other)]));
    out
}

pub fn donut_chart(props: DonutChartProps) -> Dom {
    let DonutChartProps {
        series,
        label,
        size,
        center_label,
        slots,
        apply,
    } = props;

    let all_series = series.map(|s| fold_other(&s)).broadcast();
    let visibility = SeriesVisibility::new();
    let series = visibility.filter(all_series.signal_cloned()).broadcast();
    let slots = slots.unwrap_or_else(SeriesSlots::new);

    html!("div", {
        .class(["dviz-figure", "dviz-figure-row"])
        .child(html!("div", {
            .style("width", format!("{size}px"))
            .style("flex", "none")
            .child(chart(ChartProps::new()
                .label_signal(label)
                .height(size)
                .margins(Some(Margins::uniform(4.0)))
                .slots(Some(slots.clone()))
                .layers(vec![arcs(series.signal_cloned(), ArcOptions { center_label, ..ArcOptions::default() })])))
        }))
        .child(value_legend(all_series.signal_cloned(), series.signal_cloned(), slots, visibility))
        .apply(move |b| match apply {
            Some(a) => a(b),
            None => b,
        })
    })
}

/// A legend with the value and share of each segment. Rows are keyed by
/// series and diffed, so a toggle updates one row's state and every row's
/// share without rebuilding anything.
fn value_legend<A, V>(
    all: A,
    visible: V,
    slots: Rc<SeriesSlots>,
    visibility: Rc<SeriesVisibility>,
) -> Dom
where
    A: futures_signals::signal::Signal<Item = Vec<Series>> + 'static,
    V: futures_signals::signal::Signal<Item = Vec<Series>> + 'static,
{
    let all = all.broadcast();
    let total = visible
        .map(|v| {
            v.iter()
                .filter_map(|s| s.points.first())
                .map(|p| p.y)
                .filter(|v| v.is_finite() && *v > 0.0)
                .sum::<f64>()
        })
        .broadcast();
    let hidden = visibility.hidden_signal().broadcast();
    let (ids, driver) = crate::keyed::diffed(
        all.signal_ref(|v| v.iter().map(|s| s.id.clone()).collect::<Vec<_>>())
            .dedupe_cloned(),
    );
    html!("div", {
        .class("dviz-legend-table")
        .attr("role", "group")
        .future(driver)
        .children_signal_vec(ids.signal_vec_cloned().map(move |id| {
            let highlight = slots.highlight().clone();
            let visibility = visibility.clone();
            let value = all
                .signal_ref(clone!(id => move |v| v.iter().find(|s| s.id == id).and_then(|s| s.points.first()).map_or(0.0, |p| p.y)))
                .dedupe()
                .broadcast();
            let label = all
                .signal_ref(clone!(id => move |v| v.iter().find(|s| s.id == id).map(|s| s.label.clone()).unwrap_or_default()))
                .dedupe_cloned();
            let is_hidden = hidden.signal_ref(clone!(id => move |h| h.contains(&id))).dedupe().broadcast();
            let share = map_ref! {
                let v = value.signal(),
                let t = total.signal(),
                let h = is_hidden.signal() => if *t > 0.0 && !*h { *v / *t } else { 0.0 }
            };
            html!("div", {
                .class("dviz-legend-row")
                .attr("role", "button")
                .attr("tabindex", "0")
                .attr("data-series", &id)
                .attr_signal("aria-pressed", is_hidden.signal().map(|h| if h { "false" } else { "true" }))
                .event(clone!(highlight, id => move |_: events::PointerEnter| highlight.set_neq(Some(id.clone()))))
                .event(clone!(highlight => move |_: events::PointerLeave| highlight.set_neq(None)))
                .event(clone!(visibility, id => move |_: events::Click| visibility.toggle(&id)))
                .event(clone!(visibility, id => move |e: events::KeyDown| {
                    if matches!(e.key().as_str(), "Enter" | " ") {
                        e.prevent_default();
                        visibility.toggle(&id);
                    }
                }))
                .child(html!("span", {
                    .class("dviz-legend-swatch")
                    .attr("aria-hidden", "true")
                    .style("background", slots.color_for(&id))
                }))
                .child(html!("span", { .class("dviz-legend-name") .text_signal(label) }))
                .child(html!("span", { .class("dviz-legend-value") .text_signal(value.signal().map(format::compact)) }))
                .child(html!("span", { .class("dviz-legend-share") .text_signal(share.map(|s| format::percent(s, 1))) }))
            })
        }))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_past_six() {
        let all: Vec<Series> = (0..9)
            .map(|i| {
                Series::new(
                    format!("s{i}"),
                    format!("S{i}"),
                    vec![Point::new(0.0, i as f64)],
                )
            })
            .collect();
        let folded = fold_other(&all);
        assert_eq!(folded.len(), MAX_SEGMENTS);
        assert_eq!(folded.last().unwrap().id, "other");
        assert_eq!(folded.last().unwrap().points[0].y, 0.0 + 1.0 + 2.0 + 3.0);
        assert_eq!(fold_other(&all[..6]).len(), 6);
    }
}
