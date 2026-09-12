//! Scatter, heatmap, and donut.

use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use dwind_dviz::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{always, Mutable};

use super::example;

/// Deterministic pseudo-random in 0..1 (no rand dependency in the gallery).
fn noise(i: u32) -> f64 {
    let x = (i.wrapping_mul(2_654_435_761)) as f64;
    (x.sin() * 43_758.545).fract().abs()
}

pub fn page() -> Dom {
    let scatter: Vec<Series> = (0..3)
        .map(|s| {
            Series::new(
                format!("cohort{s}"),
                format!("Cohort {}", s + 1),
                (0..40)
                    .map(|i| {
                        let k = s * 100 + i;
                        let x = 10.0 + noise(k) * 80.0;
                        Point::new(x, x * (0.4 + 0.3 * s as f64) + noise(k + 7) * 25.0)
                    })
                    .collect(),
            )
        })
        .collect();

    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let hours: Vec<String> = (0..24).map(|h| format!("{h:02}")).collect();
    let heat: Vec<Cell> = (0..7)
        .flat_map(|d| {
            (0..24).map(move |h| {
                let workday = d < 5;
                let peak = if workday {
                    (-(h as f64 - 14.0).powi(2) / 30.0).exp()
                } else {
                    0.2
                };
                Cell::new(h, d, peak * 100.0 + noise((d * 24 + h) as u32) * 15.0)
            })
        })
        .collect();

    html!("section", {
        .dwclass!("flex flex-col gap-6")
        .child(example(
            "Scatter",
            "Any two marks can touch, so the all-pairs palette cap applies: three series at most.",
            dwind_dviz::scatter_chart!({
                .label("Spend versus return for three cohorts".to_string())
                .series(scatter)
            }),
        ))
        .child(example(
            "Heatmap",
            "One hue, light to dark, computed for the current mode; the anchor flips on the dark surface.",
            dwind_dviz::chart!({
                .label("Requests per hour by weekday".to_string())
                .height(220.0)
                .x_domain(XDomain::band(hours.clone()))
                .y_domain(YDomain::band(days))
                .layers(vec![axis_x(), axis_y(), cells(always(heat.clone()), CellOptions::default())])
            }),
        ))
        .child(example(
            "Donut",
            "Part-to-whole at a glance. Nine categories fold to five plus Other.",
            dwind_dviz::donut_chart!({
                .label("Traffic share by source".to_string())
                .series((0..9).map(|i| Series::new(
                    format!("src{i}"),
                    ["Search", "Direct", "Social", "Email", "Referral", "Ads", "Partners", "Push", "Other apps"][i].to_string(),
                    vec![Point::new(0.0, [38.0, 22.0, 14.0, 9.0, 6.0, 4.0, 3.0, 2.0, 2.0][i])],
                )).collect::<Vec<_>>())
            }),
        ))
        .child(example(
            "Reactive distribution",
            "The same chart API accepts a signal of complete series snapshots, so a filter, query, or stream can replace the data without rebuilding the page.",
            reactive_scatter(),
        ))
    })
}

fn reactive_scatter() -> Dom {
    let seed = Mutable::new(0u32);
    let series = seed.signal_ref(|seed| {
        (0..2)
            .map(|s| {
                Series::new(
                    format!("segment{s}"),
                    format!("Segment {}", s + 1),
                    (0..28)
                        .map(|i| {
                            let k = *seed * 97 + s * 100 + i;
                            let x = 10.0 + noise(k) * 80.0;
                            Point::new(x, x * (0.5 + s as f64 * 0.25) + noise(k + 13) * 20.0)
                        })
                        .collect(),
                )
            })
            .collect::<Vec<_>>()
    });

    html!("div", {
        .dwclass!("flex flex-col gap-3")
        .child(html!("button", {
            .class("dviz-zoom-reset")
            .attr("type", "button")
            .text("resample points")
            .event(clone!(seed => move |_: events::Click| {
                seed.set(seed.get().wrapping_add(1));
            }))
        }))
        .child(dwind_dviz::scatter_chart!({
            .label("Reactive spend versus return".to_string())
            .series_signal(series)
        }))
    })
}
