//! Lines and areas: presets, annotations, and small multiples.

use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use dwind_dviz::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{always, Mutable};
use jiff::{Timestamp, ToSpan};

use super::example;

fn wave(id: &str, label: &str, n: usize, base: f64, amp: f64, period: f64, slope: f64) -> Series {
    Series::new(
        id,
        label,
        (0..=n)
            .map(|i| {
                let x = i as f64;
                Point::new(x, base + amp * (x / period).sin() + x * slope)
            })
            .collect(),
    )
}

pub fn page() -> Dom {
    html!("section", {
        .dwclass!("flex flex-col gap-6")
        .child(example(
            "Line chart preset",
            "Legend for two or more series, series names at the line ends, colors bound to the series id.",
            dwind_dviz::line_chart!({
                .label("Revenue, cost and margin over 24 periods, in thousands".to_string())
                .include_zero(true)
                .series(vec![
                    wave("revenue", "Revenue", 24, 40.0, 30.0, 4.0, 1.5),
                    wave("cost", "Cost", 24, 30.0, 10.0, 3.0, 0.8),
                    wave("margin", "Margin", 24, 12.0, 8.0, 5.0, 0.4),
                ])
            }),
        ))
        .child(example(
            "Time axis, monotone curve, gap, and annotations",
            "A reference line for the SLO, a shaded target band, and an event marker at a deploy.",
            time_chart(),
        ))
        .child(example(
            "Stacked area",
            "Series tile from zero; the y domain follows the column totals.",
            dwind_dviz::area_chart!({
                .label("Requests per second by region, stacked".to_string())
                .stacked(true)
                .curve(Curve::MonotoneX)
                .series(vec![
                    wave("eu", "EU", 30, 120.0, 40.0, 6.0, 1.0),
                    wave("us", "US", 30, 200.0, 60.0, 8.0, 0.5),
                    wave("apac", "APAC", 30, 80.0, 30.0, 5.0, 2.0),
                ])
            }),
        ))
        .child(example(
            "Small multiples",
            "Six series that would tangle on one plot, each on its own panel with a shared y domain.",
            small_multiples(
                always((0..6).map(|i| wave(&format!("s{i}"), &format!("Service {}", i + 1), 40, 50.0 + i as f64 * 10.0, 20.0, 4.0 + i as f64, 0.3)).collect()),
                SmallMultiplesOptions::default(),
            ),
        ))
        .child(example(
            "Reactive line data",
            "A signal can replace every series at once while the chart preserves identity, scales, labels, and transitions.",
            reactive_lines(),
        ))
    })
}

fn reactive_lines() -> Dom {
    let phase = Mutable::new(0u32);
    let series = phase.signal_ref(|phase| {
        let p = *phase as f64;
        vec![
            wave("revenue", "Revenue", 24, 40.0 + p * 2.0, 30.0, 4.0, 1.5),
            wave("cost", "Cost", 24, 30.0 + p, 10.0, 3.0, 0.8),
        ]
    });

    html!("div", {
        .dwclass!("flex flex-col gap-3")
        .child(html!("button", {
            .class("dviz-zoom-reset")
            .attr("type", "button")
            .text("advance quarter")
            .event(clone!(phase => move |_: events::Click| {
                phase.set(phase.get().wrapping_add(1));
            }))
        }))
        .child(dwind_dviz::line_chart!({
            .label("Reactive revenue and cost".to_string())
            .series_signal(series)
        }))
    })
}

fn time_chart() -> Dom {
    let start: Timestamp = "2026-09-11T06:00:00Z".parse().unwrap();
    let points: Vec<Point> = (0..=72)
        .map(|i| {
            let t = start.checked_add((i * 10).minutes()).unwrap();
            let y = if (30..34).contains(&i) {
                f64::NAN
            } else {
                200.0 + 80.0 * ((i as f64) / 9.0).sin() + 15.0 * ((i as f64) / 2.0).cos()
            };
            TimePoint::new(t, y).to_point()
        })
        .collect();
    let end = start.checked_add(720.minutes()).unwrap();
    let series = vec![Series::new("latency", "p95 latency", points)];
    let deploy = start.checked_add(400.minutes()).unwrap().as_millisecond() as f64;

    dwind_dviz::chart!({
        .label("p95 latency in milliseconds over twelve hours, with the 250ms SLO".to_string())
        .x_domain(XDomain::time(Extent::new(start, end)))
        .y_domain(YDomain::Linear(Extent::new(0.0, 320.0)))
        .height(240.0)
        .layers(vec![
            grid(),
            band_y(always(Extent::new(150.0, 220.0)), always("target".to_string())),
            axis_x(),
            axis_y(),
            line_with(always(series), LineOptions { curve: Curve::MonotoneX, ..Default::default() }),
            reference_y(always(250.0), always("SLO 250ms".to_string())),
            reference_x(always(deploy), always("deploy".to_string())),
        ])
    })
}
