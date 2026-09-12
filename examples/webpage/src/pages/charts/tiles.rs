//! Stat tiles and sparklines: when the story is one number.

use dominator::{html, Dom};
use dwind::prelude::*;
use dwind_dviz::prelude::*;
use dwind_macros::dwclass;

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
    })
}
