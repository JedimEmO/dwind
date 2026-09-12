//! Live data: a windowed source fed by an in-browser generator, committed
//! once per frame, drawn downsampled to the plot width.

use std::rc::Rc;

use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use dwind_dviz::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Mutable, SignalExt};
use gloo_timers::callback::Interval;
use jiff::Timestamp;
use wasm_bindgen::JsCast;

use super::example;

const SERIES: [(&str, &str); 8] = [
    ("api", "API"),
    ("web", "Web"),
    ("jobs", "Jobs"),
    ("db", "Database"),
    ("cache", "Cache"),
    ("queue", "Queue"),
    ("auth", "Auth"),
    ("search", "Search"),
];

fn now_ms() -> f64 {
    Timestamp::now().as_millisecond() as f64
}

/// A cheap deterministic wobble so every series looks alive but different.
fn sample(i: usize, t: f64) -> f64 {
    let k = i as f64;
    60.0 + 15.0 * k
        + 25.0 * ((t / (900.0 + 200.0 * k)) + k).sin()
        + 8.0 * (t / 130.0 + k * 3.0).cos()
}

pub fn page() -> Dom {
    // Sixty seconds of history, sliding.
    let source = WindowedSource::new(Retention::Span(60_000.0));
    for (id, label) in SERIES {
        source.add_series(id, label);
    }
    // Seed the window so the chart is full from the first frame.
    let t0 = now_ms();
    for i in (0..1200).rev() {
        let t = t0 - i as f64 * 50.0;
        for (k, (id, _)) in SERIES.iter().enumerate() {
            source.push(id, Point::new(t, sample(k, t)));
        }
    }
    commit_on_frame(&source);

    let rate_hz = Mutable::new(20u32);
    let buffered = Mutable::new(source.len());
    let ticker: Rc<std::cell::RefCell<Option<Interval>>> = Rc::new(std::cell::RefCell::new(None));
    let start = {
        let source = source.clone();
        let buffered = buffered.clone();
        let ticker = ticker.clone();
        move |hz: u32| {
            let interval = Interval::new(
                1000 / hz.max(1),
                clone!(source, buffered => move || {
                    let t = now_ms();
                    for (k, (id, _)) in SERIES.iter().enumerate() {
                        source.push(id, Point::new(t, sample(k, t)));
                    }
                    buffered.set_neq(source.len());
                }),
            );
            *ticker.borrow_mut() = Some(interval);
        }
    };
    start(20);

    html!("section", {
        .dwclass!("flex flex-col gap-6")
        .child(example(
            "Eight series, sixty-second window, one commit per frame",
            "Samples arrive on a timer and are batched into one publish per animation frame. The y domain is fixed so the axis never jitters. Pause to freeze the window while data keeps arriving; the history stays trimmed to the window.",
            html!("div", {
                .dwclass!("flex flex-col gap-3")
                .child(html!("div", {
                    .class("font-code")
                    .dwclass!("flex align-items-center gap-4 text-xs flex-wrap")
                    .child(live_indicator(&source))
                    .child(html!("span", {
                        .dwclass!("text-woodsmoke-500")
                        .text_signal(buffered.signal().map(|n| format!("{} points buffered", dwind_dviz::core::format::thousands(n as f64, 0))))
                    }))
                    .child(html!("label", {
                        .dwclass!("flex align-items-center gap-2 text-woodsmoke-400 is(.light *):text-woodsmoke-600")
                        .text("rate")
                        .child(html!("select", {
                            .class("font-code")
                            .dwclass!("text-xs rounded-md p-l-2 p-r-2 p-t-1 p-b-1 cursor-pointer font-inherit")
                            .dwclass!("border border-woodsmoke-800 bg-woodsmoke-900 text-woodsmoke-200")
                            .dwclass!("is(.light *):border-woodsmoke-200 is(.light *):bg-woodsmoke-50 is(.light *):text-woodsmoke-800")
                            .dwclass!("hover:border-candlelight-700 transition-colors")
                            .children([1u32, 5, 20, 60].map(|hz| html!("option", {
                                .attr("value", &hz.to_string())
                                .apply_if(hz == 20, |b| b.attr("selected", "selected"))
                                .text(&format!("{hz} Hz"))
                            })))
                            .with_node!(el => {
                                .event(clone!(rate_hz, start => move |_: events::Change| {
                                    let v: u32 = el
                                        .dyn_ref::<web_sys::HtmlSelectElement>()
                                        .map(|s| s.value())
                                        .and_then(|v| v.parse().ok())
                                        .unwrap_or(20);
                                    rate_hz.set(v);
                                    start(v);
                                }))
                            })
                        }))
                    }))
                    .child(html!("button", {
                        .class("dviz-zoom-reset")
                        .attr("type", "button")
                        .text("burst 10k samples")
                        .event(clone!(source, buffered => move |_: events::Click| {
                            let t = now_ms();
                            for i in 0..1250 {
                                let tt = t - (1250 - i) as f64 * 4.0;
                                for (k, (id, _)) in SERIES.iter().enumerate() {
                                    source.push(id, Point::new(tt, sample(k, tt) + 40.0 * ((i as f64) / 60.0).sin()));
                                }
                            }
                            buffered.set_neq(source.len());
                        }))
                    }))
                }))
                .child(dwind_dviz::line_chart!({
                    .label("Requests per second for eight services, last sixty seconds".to_string())
                    .x(XKind::time_utc())
                    .x_extent_signal(source.window_signal().map(Some))
                    .y(Some(Extent::new(0.0, 220.0)))
                    .downsample(true)
                    .live(true)
                    .end_labels(false)
                    .height(300.0)
                    .series_signal(source.series_signal())
                }))
            }),
        ))
    })
}
