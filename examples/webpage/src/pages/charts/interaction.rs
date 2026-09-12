//! Interaction: hover, keyboard, legend toggling, and brush-to-zoom.

use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use dwind_dviz::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::Mutable;
use jiff::{Timestamp, ToSpan};

use super::example;

pub fn page() -> Dom {
    let start: Timestamp = "2026-09-11T00:00:00Z".parse().unwrap();
    let mk = |id: &str, label: &str, base: f64, amp: f64, period: f64| {
        Series::new(
            id,
            label,
            (0..=288)
                .map(|i| {
                    let t = start.checked_add((i * 5).minutes()).unwrap();
                    let x = i as f64;
                    TimePoint::new(t, base + amp * (x / period).sin() + 6.0 * (x / 3.0).cos())
                        .to_point()
                })
                .collect(),
        )
    };

    html!("section", {
        .dwclass!("flex flex-col gap-6")
        .child(example(
            "Crosshair, tooltip, keyboard, legend toggling, brush to zoom",
            "Hover for the nearest sample across series. Tab to the plot and use the arrow keys. Click a legend item to hide its series; survivors keep their colors. Drag to zoom the time axis, then reset.",
            dwind_dviz::line_chart!({
                .label("Requests per second for three services over one day".to_string())
                .x(XKind::time_utc())
                .zoomable(true)
                .curve(Curve::MonotoneX)
                .height(280.0)
                .series(vec![
                    mk("api", "API", 120.0, 40.0, 30.0),
                    mk("web", "Web", 80.0, 25.0, 22.0),
                    mk("jobs", "Jobs", 30.0, 12.0, 15.0),
                ])
            }),
        ))
        .child(example(
            "Per-mark tooltips",
            "Bars, points, cells and slices show their own tooltip on hover and on keyboard focus; the hit target on a point is 24px.",
            dwind_dviz::scatter_chart!({
                .label("Latency versus payload size".to_string())
                .height(240.0)
                .series(vec![Series::new(
                    "req",
                    "Requests",
                    (0..30).map(|i| Point::new(i as f64 * 3.3, 20.0 + i as f64 * 1.7 + ((i * 7) % 11) as f64)).collect(),
                )])
            }),
        ))
        .child(example(
            "Reactive interaction data",
            "Interaction state and data state are independent: the crosshair, keyboard focus, legend, and brush continue to work while the series signal changes.",
            reactive_interaction(),
        ))
    })
}

fn reactive_interaction() -> Dom {
    let phase = Mutable::new(0u32);
    let series = phase.signal_ref(|phase| {
        let p = *phase as f64;
        vec![
            mk_series("api", "API", 120.0, 40.0, 30.0, p),
            mk_series("web", "Web", 80.0, 25.0, 22.0, p + 1.0),
        ]
    });

    html!("div", {
        .dwclass!("flex flex-col gap-3")
        .child(html!("button", {
            .class("dviz-zoom-reset")
            .attr("type", "button")
            .text("append a new sample")
            .event(clone!(phase => move |_: events::Click| {
                phase.set(phase.get().wrapping_add(1));
            }))
        }))
        .child(dwind_dviz::line_chart!({
            .label("Reactive requests per second".to_string())
            .x(XKind::time_utc())
            .series_signal(series)
        }))
    })
}

fn mk_series(id: &str, label: &str, base: f64, amp: f64, period: f64, phase: f64) -> Series {
    let start: Timestamp = "2026-09-11T00:00:00Z".parse().unwrap();
    Series::new(
        id,
        label,
        (0..=48)
            .map(|i| {
                let t = start.checked_add((i * 30).minutes()).unwrap();
                let x = i as f64;
                TimePoint::new(t, base + amp * ((x + phase) / period).sin()).to_point()
            })
            .collect(),
    )
}
