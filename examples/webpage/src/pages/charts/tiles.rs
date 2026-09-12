//! Stat tiles and sparklines: when the story is one number.

use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use dwind_dviz::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Mutable, SignalExt};

use super::example;

pub fn page() -> Dom {
    let trend: Vec<f64> = vec![
        41.0, 43.0, 42.0, 45.0, 47.0, 46.0, 49.0, 52.0, 51.0, 54.0, 56.0, 58.2,
    ];
    let latency: Vec<f64> = vec![
        210.0, 205.0, 230.0, 228.0, 260.0, 255.0, 240.0, 248.0, 262.0, 270.0, 268.0, 281.0,
    ];

    html!("section", {
        .dwclass!("flex flex-col gap-6")
        .child(example(
            "Stat tiles",
            "Label, compact value, a signed delta colored by direction times whether up is good, and a trend.",
            html!("div", {
                .dwclass!("grid gap-6")
                .style("grid-template-columns", "repeat(auto-fit, minmax(180px, 1fr))")
                .child(dwind_dviz::stat_tile!({
                    .label("Monthly revenue".to_string())
                    .value(4_218_000.0)
                    .format(Some(Box::new(|v| format!("${}", dwind_dviz::core::format::compact(v)))))
                    .delta(Some(0.123))
                    .delta_label("vs last month".to_string())
                    .trend(trend.clone())
                }))
                .child(dwind_dviz::stat_tile!({
                    .label("p95 latency".to_string())
                    .value(281.0)
                    .format(Some(Box::new(|v| format!("{} ms", dwind_dviz::core::format::compact(v)))))
                    .delta(Some(0.048))
                    .delta_label("vs last week".to_string())
                    .up_is_good(false)
                    .trend(latency.clone())
                }))
                .child(dwind_dviz::stat_tile!({
                    .label("Active users".to_string())
                    .value(12_940.0)
                    .delta(Some(-0.021))
                    .delta_label("vs yesterday".to_string())
                }))
                .child(dwind_dviz::stat_tile!({
                    .label("Open incidents".to_string())
                    .value(3.0)
                    .delta(Some(0.0))
                    .delta_label("unchanged".to_string())
                }))
            }),
        ))
        .child(example(
            "Reactive KPI tiles",
            "Values, deltas, and sparklines can each be driven by signals. Press the button to simulate the next reporting snapshot.",
            reactive_tiles(),
        ))
    })
}

fn reactive_tiles() -> Dom {
    let tick = Mutable::new(0u32);
    let value = tick
        .signal()
        .map(|tick| 4_218_000.0 + tick as f64 * 18_500.0);
    let delta = tick.signal().map(|tick| Some(0.123 + tick as f64 * 0.004));
    let trend = tick.signal_ref(|tick| {
        (0..12)
            .map(|i| 41.0 + i as f64 * 1.3 + *tick as f64 * 0.8)
            .collect::<Vec<_>>()
    });

    html!("div", {
        .dwclass!("flex flex-col gap-3")
        .child(html!("button", {
            .class("dviz-zoom-reset")
            .attr("type", "button")
            .text("load next snapshot")
            .event(clone!(tick => move |_: events::Click| {
                tick.set(tick.get().wrapping_add(1));
            }))
        }))
        .child(dwind_dviz::stat_tile!({
            .label("Monthly revenue".to_string())
            .value_signal(value)
            .format(Some(Box::new(|v| format!("${}", dwind_dviz::core::format::compact(v)))))
            .delta_signal(delta)
            .delta_label("vs last month".to_string())
            .trend_signal(trend)
        }))
    })
}
