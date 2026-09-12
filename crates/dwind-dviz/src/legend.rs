//! The legend: the dependable identity channel for two or more series.
//! A single series renders nothing, since the chart's title already names
//! it and a one-swatch box would only restate it.

use std::rc::Rc;

use dominator::{Dom, clone, events, html};
use dwind_dviz_core::data::Series;
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::SignalVecExt;

use crate::chart::SeriesSlots;
use crate::visibility::SeriesVisibility;

/// An HTML legend for `series`, colored through `slots` (share the same
/// `SeriesSlots` with the chart). Place it above or beside the chart.
pub fn legend<S>(series: S, slots: Rc<SeriesSlots>) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    legend_with(series, slots, None)
}

/// A legend whose items toggle series when `visibility` is given: each
/// item is a button with `aria-pressed`, and hidden series stay listed,
/// dimmed, so they can be brought back.
pub fn legend_with<S>(
    series: S,
    slots: Rc<SeriesSlots>,
    visibility: Option<Rc<SeriesVisibility>>,
) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    let series = series.broadcast();
    let toggleable = visibility.is_some();
    let hidden = match &visibility {
        Some(v) => v.hidden_signal().boxed_local(),
        None => futures_signals::signal::always(Default::default()).boxed_local(),
    }
    .broadcast();
    // Items are keyed by id and diffed, so toggling one never rebuilds the
    // others; a series' label follows its own signal.
    let (ids, driver) = crate::keyed::diffed(
        series
            .signal_ref(|all| {
                if all.len() < 2 {
                    vec![]
                } else {
                    all.iter().map(|s| s.id.clone()).collect::<Vec<_>>()
                }
            })
            .dedupe_cloned(),
    );
    html!("div", {
        .class("dviz-legend")
        .attr("role", if toggleable { "group" } else { "list" })
        .future(driver)
        .child(html!("div", {
            .class("dviz-legend-items")
            .children_signal_vec(ids.signal_vec_cloned().map(move |id| {
                let label = series
                    .signal_ref(clone!(id => move |all| all.iter().find(|s| s.id == id).map(|s| s.label.clone()).unwrap_or_default()))
                    .dedupe_cloned();
                let pressed = hidden
                    .signal_ref(clone!(id => move |h| if h.contains(&id) { "false" } else { "true" }))
                    .dedupe();
                let toggle = visibility.clone();
                let highlight = slots.highlight().clone();
                html!("div", {
                    .class("dviz-legend-item")
                    .attr("role", if toggle.is_some() { "button" } else { "listitem" })
                    .attr("data-series", &id)
                    .event(clone!(highlight, id => move |_: events::PointerEnter| highlight.set_neq(Some(id.clone()))))
                    .event(clone!(highlight => move |_: events::PointerLeave| highlight.set_neq(None)))
                    .apply_if(toggle.is_some(), |b| b
                        .attr("tabindex", "0")
                        .attr_signal("aria-pressed", pressed)
                        .event(clone!(toggle, id => move |_: events::Click| {
                            if let Some(v) = &toggle { v.toggle(&id) }
                        }))
                        .event(clone!(toggle, id => move |e: events::KeyDown| {
                            if matches!(e.key().as_str(), "Enter" | " ") {
                                e.prevent_default();
                                if let Some(v) = &toggle { v.toggle(&id) }
                            }
                        })))
                    .child(html!("span", {
                        .class("dviz-legend-swatch")
                        .attr("aria-hidden", "true")
                        .style("background", slots.color_for(&id))
                    }))
                    .child(html!("span", { .text_signal(label) }))
                })
            }))
        }))
    })
}
