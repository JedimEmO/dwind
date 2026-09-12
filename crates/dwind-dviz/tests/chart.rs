//! Browser tests for the chart container and the standard layers.

#![cfg(target_arch = "wasm32")]

use dominator::{Dom, append_dom, body, html};
use dwind_dviz::prelude::*;
use dwind_dviz_core::ticks::linear_ticks;
use futures_signals::signal::{Mutable, always};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Element, HtmlElement};

wasm_bindgen_test_configure!(run_in_browser);

fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

/// Mounts `dom` inside a fixed-width host so the ResizeObserver has a real
/// size to report, and waits for the observer to fire.
async fn mount(id: &str, width: f64, dom: Dom) -> HtmlElement {
    apply_style_sheet();
    let host = html!("div", {
        .attr("id", id)
        .style("width", format!("{width}px"))
        .child(dom)
    });
    append_dom(&body(), host);
    // ResizeObserver delivers after layout; two frames is plenty.
    TimeoutFuture::new(50).await;
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into()
        .unwrap()
}

fn unmount(host: &HtmlElement) {
    host.remove();
}

fn query_all(root: &Element, selector: &str) -> Vec<Element> {
    let list = root.query_selector_all(selector).unwrap();
    (0..list.length())
        .filter_map(|i| list.item(i))
        .map(|n| n.dyn_into().unwrap())
        .collect()
}

fn series(id: &str, ys: &[f64]) -> Series {
    Series::new(
        id,
        id,
        ys.iter()
            .enumerate()
            .map(|(i, y)| Point::new(i as f64, *y))
            .collect(),
    )
}

#[wasm_bindgen_test]
async fn chart_is_an_accessible_image_sized_to_its_host() {
    let host = mount(
        "accessible",
        400.0,
        dwind_dviz::chart!({
            .label("Test chart".to_string())
            .height(200.0)
            .x_domain(XDomain::Linear(Extent::new(0.0, 10.0)))
            .y_domain(YDomain::Linear(Extent::new(0.0, 100.0)))
            .layers(vec![grid(), axis_x(), axis_y()])
        }),
    )
    .await;

    let svg = host.query_selector("svg").unwrap().unwrap();
    assert_eq!(svg.get_attribute("role").as_deref(), Some("img"));
    assert_eq!(
        svg.get_attribute("aria-label").as_deref(),
        Some("Test chart")
    );
    assert_eq!(
        svg.query_selector("title")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("Test chart")
    );
    assert_eq!(
        svg.query_selector("desc")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("Interactive data visualization. Use the keyboard to inspect available marks.")
    );
    assert_eq!(
        svg.get_attribute("viewBox").as_deref(),
        Some("0 0 400 200"),
        "viewBox follows the host size"
    );
    unmount(&host);
}

#[wasm_bindgen_test]
async fn chart_without_a_label_has_a_nonempty_accessible_name() {
    let host = mount(
        "fallback-label",
        320.0,
        dwind_dviz::chart!({
            .height(160.0)
            .layers(vec![])
        }),
    )
    .await;
    let svg = host.query_selector("svg").unwrap().unwrap();
    assert_eq!(
        svg.get_attribute("aria-label").as_deref(),
        Some("Data visualization")
    );
    host.remove();
}

#[wasm_bindgen_test]
async fn axis_ticks_match_core() {
    let host = mount(
        "ticks",
        400.0,
        dwind_dviz::chart!({
            .label("Ticks".to_string())
            .height(240.0)
            .x_domain(XDomain::Linear(Extent::new(0.0, 10.0)))
            .y_domain(YDomain::Linear(Extent::new(0.0, 100.0)))
            .layers(vec![grid(), axis_x(), axis_y()])
        }),
    )
    .await;

    let y_labels: Vec<String> = query_all(&host, ".dviz-axis-y text")
        .iter()
        .map(|e| e.text_content().unwrap())
        .collect();
    let expected: Vec<String> = linear_ticks(0.0, 100.0, 6)
        .iter()
        .map(|v| format!("{v}"))
        .collect();
    assert_eq!(
        y_labels, expected,
        "240px tall → 6 ticks requested from core"
    );

    let x_labels: Vec<String> = query_all(&host, ".dviz-axis-x text")
        .iter()
        .map(|e| e.text_content().unwrap())
        .collect();
    assert!(x_labels.first().is_some_and(|l| l == "0"), "{x_labels:?}");
    assert!(x_labels.last().is_some_and(|l| l == "10"), "{x_labels:?}");

    // One gridline per y tick minus the baseline the axis draws.
    assert_eq!(
        query_all(&host, ".dviz-grid line").len(),
        y_labels.len() - 1
    );
    unmount(&host);
}

#[wasm_bindgen_test]
async fn lines_are_one_path_per_series_and_keep_their_slot() {
    let data = Mutable::new(vec![
        series("a", &[1.0, 2.0, 3.0]),
        series("b", &[3.0, 2.0, 1.0]),
    ]);
    let host = mount(
        "lines",
        400.0,
        dwind_dviz::chart!({
            .label("Lines".to_string())
            .x_domain(XDomain::Linear(Extent::new(0.0, 2.0)))
            .y_domain(YDomain::Linear(Extent::new(0.0, 3.0)))
            .layers(vec![line(data.signal_cloned())])
        }),
    )
    .await;

    let paths = query_all(&host, "path.dviz-line");
    assert_eq!(paths.len(), 2);
    assert_eq!(paths[0].get_attribute("data-series").as_deref(), Some("a"));
    let slots = SeriesSlots::new();
    let a_color = slots.color_for("a");
    let b_color = slots.color_for("b");
    assert!(paths[0].get_attribute("style").unwrap().contains(&a_color));
    assert!(paths[1].get_attribute("style").unwrap().contains(&b_color));
    let d = paths[0].get_attribute("d").unwrap();
    assert!(d.starts_with('M') && d.matches('L').count() == 2, "{d}");

    // Removing the first series must not repaint the survivor. The removed
    // series lingers for its exit first, flagged as leaving.
    data.set(vec![series("b", &[3.0, 2.0, 1.0])]);
    TimeoutFuture::new(20).await;
    assert_eq!(
        query_all(&host, ".dviz-leave path.dviz-line").len(),
        1,
        "a is on its way out"
    );
    TimeoutFuture::new(400).await;
    let paths = query_all(&host, "path.dviz-line");
    assert_eq!(paths.len(), 1, "a is gone after the exit");
    assert_eq!(paths[0].get_attribute("data-series").as_deref(), Some("b"));
    assert!(paths[0].get_attribute("style").unwrap().contains(&b_color));

    // Updating data patches the `d` attribute of the existing node.
    let before = paths[0].get_attribute("d").unwrap();
    data.set(vec![series("b", &[0.0, 0.0, 0.0])]);
    TimeoutFuture::new(20).await;
    let after = query_all(&host, "path.dviz-line")[0]
        .get_attribute("d")
        .unwrap();
    assert_ne!(before, after);
    unmount(&host);
}

#[wasm_bindgen_test]
async fn resize_reranges_the_frame() {
    let host = mount(
        "resize",
        400.0,
        dwind_dviz::chart!({
            .label("Resize".to_string())
            .height(200.0)
            .layers(vec![axis_x()])
        }),
    )
    .await;
    host.style().set_property("width", "600px").unwrap();
    TimeoutFuture::new(100).await;
    let svg = host.query_selector("svg").unwrap().unwrap();
    assert_eq!(svg.get_attribute("viewBox").as_deref(), Some("0 0 600 200"));
    unmount(&host);
}

#[wasm_bindgen_test]
async fn gaps_break_the_path() {
    let pts = vec![
        Point::new(0.0, 1.0),
        Point::new(1.0, f64::NAN),
        Point::new(2.0, 1.0),
    ];
    let host = mount(
        "gap",
        400.0,
        dwind_dviz::chart!({
            .label("Gap".to_string())
            .x_domain(XDomain::Linear(Extent::new(0.0, 2.0)))
            .y_domain(YDomain::Linear(Extent::new(0.0, 2.0)))
            .layers(vec![line(always(vec![Series::new("g", "g", pts)]))])
        }),
    )
    .await;
    let d = query_all(&host, "path.dviz-line")[0]
        .get_attribute("d")
        .unwrap();
    assert_eq!(d.matches('M').count(), 2, "{d}");
    unmount(&host);
}

#[wasm_bindgen_test]
async fn marks_are_clipped_to_the_plot() {
    let host = mount(
        "clip",
        400.0,
        dwind_dviz::chart!({
            .label("Clip".to_string())
            .x_domain(XDomain::Linear(Extent::new(0.0, 1.0)))
            .y_domain(YDomain::Linear(Extent::new(0.0, 1.0)))
            .layers(vec![line(always(vec![series("a", &[0.0, 5.0, 0.0])]))])
        }),
    )
    .await;
    let lines = host.query_selector("path.dviz-line").unwrap().unwrap();
    let clip = lines.get_attribute("clip-path").unwrap();
    assert!(clip.starts_with("url(#dviz-clip-"), "{clip}");
    let id = clip.trim_start_matches("url(#").trim_end_matches(')');
    let rect = host
        .query_selector(&format!("#{id} rect"))
        .unwrap()
        .unwrap();
    let width: f64 = rect.get_attribute("width").unwrap().parse().unwrap();
    let svg = host.query_selector("svg").unwrap().unwrap();
    assert!(
        width > 300.0 && width < 400.0,
        "clip follows the plot, not the svg: {width} of {:?}",
        svg.get_attribute("viewBox")
    );
    assert_eq!(
        query_all(&host, "circle.dviz-end-marker").len(),
        1,
        "one end marker per line"
    );
    unmount(&host);
}
