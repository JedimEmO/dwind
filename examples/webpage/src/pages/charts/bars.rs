//! Bars: grouped, stacked, and 100% stacked.

use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use dwind_dviz::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::Mutable;

use super::example;

fn cat(id: &str, label: &str, ys: &[f64]) -> Series {
    Series::new(
        id,
        label,
        ys.iter()
            .enumerate()
            .map(|(i, y)| Point::new(i as f64, *y))
            .collect(),
    )
}

const MONTHS: [&str; 6] = ["Apr", "May", "Jun", "Jul", "Aug", "Sep"];

pub fn page() -> Dom {
    html!("section", {
        .dwclass!("flex flex-col gap-6")
        .child(example(
            "Single series",
            "One series, one color: every bar takes slot 1. No legend box, the title names it. Values on the caps.",
            dwind_dviz::bar_chart!({
                .label("Signups per month".to_string())
                .categories(MONTHS.map(String::from).to_vec())
                .series(vec![cat("signups", "Signups", &[1240.0, 1580.0, 1320.0, 1910.0, 2260.0, 2040.0])])
            }),
        ))
        .child(example(
            "Grouped with a negative value",
            "Bars grow from zero in both directions; the rounded end is always the data end.",
            dwind_dviz::bar_chart!({
                .label("Net change by month for two products".to_string())
                .categories(MONTHS.map(String::from).to_vec())
                .series(vec![
                    cat("a", "Product A", &[12.0, 18.0, -6.0, 22.0, 15.0, 9.0]),
                    cat("b", "Product B", &[8.0, -4.0, 14.0, 10.0, 19.0, 21.0]),
                ])
            }),
        ))
        .child(example(
            "Stacked",
            "Segments separated by a 2px surface gap; only the outer segment is rounded; totals on the caps.",
            dwind_dviz::bar_chart!({
                .label("Tickets by priority per month, stacked".to_string())
                .mode(BarMode::Stacked)
                .categories(MONTHS.map(String::from).to_vec())
                .series(vec![
                    cat("low", "Low", &[30.0, 42.0, 38.0, 51.0, 47.0, 40.0]),
                    cat("med", "Medium", &[18.0, 20.0, 25.0, 22.0, 30.0, 28.0]),
                    cat("high", "High", &[6.0, 4.0, 9.0, 7.0, 5.0, 11.0]),
                ])
            }),
        ))
        .child(example(
            "100% stacked",
            "Each column normalised to one; the axis reads as a share.",
            dwind_dviz::bar_chart!({
                .label("Share of traffic by device per month".to_string())
                .mode(BarMode::StackedExpand)
                .value_labels(false)
                .categories(MONTHS.map(String::from).to_vec())
                .series(vec![
                    cat("mobile", "Mobile", &[52.0, 55.0, 58.0, 61.0, 63.0, 66.0]),
                    cat("desktop", "Desktop", &[40.0, 38.0, 35.0, 32.0, 30.0, 28.0]),
                    cat("tablet", "Tablet", &[8.0, 7.0, 7.0, 7.0, 7.0, 6.0]),
                ])
            }),
        ))
        .child(example(
            "Reactive bars",
            "The chart keeps its SVG nodes and patches the series signal when the sample changes. Static data is still the simplest option; signals make live dashboards just as direct.",
            reactive_bars(),
        ))
    })
}

fn reactive_bars() -> Dom {
    let phase = Mutable::new(0u32);
    let series = phase.signal_ref(|phase| {
        let p = *phase as f64;
        vec![
            cat(
                "signups",
                "Signups",
                &(0..6)
                    .map(|i| 1_400.0 + (i as f64 * 0.9 + p).sin() * 420.0 + i as f64 * 110.0)
                    .collect::<Vec<_>>(),
            ),
            cat(
                "activations",
                "Activations",
                &(0..6)
                    .map(|i| 900.0 + (i as f64 * 0.7 + p * 1.2).cos() * 260.0 + i as f64 * 80.0)
                    .collect::<Vec<_>>(),
            ),
        ]
    });

    html!("div", {
        .dwclass!("flex flex-col gap-3")
        .child(html!("button", {
            .class("dviz-zoom-reset")
            .attr("type", "button")
            .text("refresh sample")
            .event(clone!(phase => move |_: events::Click| {
                phase.set(phase.get().wrapping_add(1));
            }))
        }))
        .child(dwind_dviz::bar_chart!({
            .label("Reactive signups and activations by month".to_string())
            .categories(MONTHS.map(String::from).to_vec())
            .series_signal(series)
        }))
    })
}
