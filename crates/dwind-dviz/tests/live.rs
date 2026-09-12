//! Browser tests for the realtime path: frame-coalesced commits, the live
//! indicator, windowed x extents with downsampling, and a frame-time
//! benchmark with eight series of ten thousand points.

#![cfg(target_arch = "wasm32")]

use dominator::{Dom, append_dom, body, html};
use dwind_dviz::prelude::*;
use futures_signals::signal::SignalExt;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Element, HtmlElement};

wasm_bindgen_test_configure!(run_in_browser);

fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

async fn mount(id: &str, width: f64, dom: Dom) -> HtmlElement {
    apply_style_sheet();
    let host = html!("div", {
        .attr("id", id)
        .style("width", format!("{width}px"))
        .child(dom)
    });
    append_dom(&body(), host);
    TimeoutFuture::new(50).await;
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into()
        .unwrap()
}

fn query_all(root: &Element, selector: &str) -> Vec<Element> {
    let list = root.query_selector_all(selector).unwrap();
    (0..list.length())
        .filter_map(|i| list.item(i))
        .map(|n| n.dyn_into().unwrap())
        .collect()
}

async fn next_frame() {
    // Two timeouts comfortably straddle one animation frame in a headless
    // browser that may throttle rendering.
    TimeoutFuture::new(40).await;
}

#[wasm_bindgen_test]
async fn commits_once_per_frame() {
    let source = WindowedSource::new(Retention::Count(1000));
    commit_on_frame(&source);
    for i in 0..100 {
        source.push("a", Point::new(i as f64, 1.0));
    }
    assert!(
        source.series().is_empty(),
        "nothing published synchronously"
    );
    assert!(source.is_dirty());
    next_frame().await;
    assert_eq!(
        source.series()[0].points.len(),
        100,
        "one commit carried every push"
    );
    assert!(!source.is_dirty());
    source.push("a", Point::new(100.0, 1.0));
    next_frame().await;
    assert_eq!(source.series()[0].points.len(), 101);
}

#[wasm_bindgen_test]
async fn live_indicator_toggles_follow() {
    let source = WindowedSource::new(Retention::Span(10.0));
    let host = mount("live", 300.0, live_indicator(&source)).await;
    let button = host.query_selector(".dviz-live").unwrap().unwrap();
    assert_eq!(button.text_content().as_deref(), Some("Live"));
    assert_eq!(button.get_attribute("data-live").as_deref(), Some("true"));
    button.dyn_ref::<HtmlElement>().unwrap().click();
    TimeoutFuture::new(20).await;
    assert!(!source.is_following());
    assert_eq!(button.text_content().as_deref(), Some("Paused"));
    host.remove();
}

#[wasm_bindgen_test]
async fn windowed_chart_follows_the_window_and_downsamples() {
    let source = WindowedSource::new(Retention::Span(1_000.0));
    commit_on_frame(&source);
    let host = mount(
        "window",
        400.0,
        dwind_dviz::line_chart!({
            .label("Window".to_string())
            .x_extent_signal(source.window_signal().map(Some))
            .y(Some(Extent::new(0.0, 10.0)))
            .downsample(true)
            .end_labels(false)
            .legend(false)
            .series_signal(source.series_signal())
        }),
    )
    .await;
    // 5001 points across the last second: far more than 400px can show.
    for i in 0..=5000 {
        source.push("a", Point::new(i as f64 * 0.2, (i % 7) as f64));
    }
    next_frame().await;
    next_frame().await;
    let d = host
        .query_selector("path.dviz-line")
        .unwrap()
        .unwrap()
        .get_attribute("d")
        .unwrap();
    let segments = d.matches('L').count();
    assert!(
        segments < 1000,
        "downsampled to the plot width: {segments} segments"
    );
    assert!(segments > 100, "{segments}");
    let labels: Vec<String> = query_all(&host, ".dviz-axis-x text")
        .iter()
        .map(|e| e.text_content().unwrap())
        .collect();
    assert!(
        labels.first().is_some_and(|l| l == "0") && labels.last().is_some_and(|l| l == "1,000"),
        "window is exactly [0, 1000]: {labels:?}"
    );
    host.remove();
}

#[wasm_bindgen_test]
async fn benchmark_eight_series_ten_thousand_points() {
    let perf = web_sys::window().unwrap().performance().unwrap();
    let source = WindowedSource::new(Retention::Count(10_000));
    commit_on_frame(&source);
    let host = mount(
        "bench",
        800.0,
        dwind_dviz::line_chart!({
            .label("Bench".to_string())
            .x_extent_signal(source.window_signal().map(Some))
            .y(Some(Extent::new(0.0, 100.0)))
            .downsample(true)
            .end_labels(false)
            .height(300.0)
            .series_signal(source.series_signal())
        }),
    )
    .await;
    for s in 0..8 {
        let id = format!("s{s}");
        for i in 0..10_000 {
            source.push(&id, Point::new(i as f64, ((i + s * 100) % 97) as f64));
        }
    }
    next_frame().await;
    next_frame().await;
    assert_eq!(query_all(&host, "path.dviz-line").len(), 8);

    // Steady state: push one sample per series per frame and time the
    // synchronous cost of a commit plus the signal-driven DOM patch.
    let frames = 30;
    let t0 = perf.now();
    for f in 0..frames {
        for s in 0..8 {
            source.push(&format!("s{s}"), Point::new(10_000.0 + f as f64, 50.0));
        }
        let c0 = perf.now();
        source.commit();
        let _ = c0;
    }
    let total = perf.now() - t0;
    let per_frame = total / frames as f64;
    web_sys::console::log_1(&format!("benchmark: 8 series x 10k points, {frames} commits, {per_frame:.2} ms per commit+patch (sync)").into());
    assert!(
        per_frame < 100.0,
        "{per_frame} ms per frame is far too slow"
    );
    host.remove();
}
