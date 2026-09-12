//! Browser tests for the Phase 3 layers, legend and presets.

#![cfg(target_arch = "wasm32")]

use dominator::{Dom, append_dom, body, html};
use dwind_dviz::prelude::*;
use futures_signals::signal::{Mutable, always};
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

fn cat(id: &str, ys: &[f64]) -> Series {
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
async fn bar_chart_draws_bars_labels_and_legend() {
    let host = mount(
        "bars",
        500.0,
        dwind_dviz::bar_chart!({
            .label("Bars".to_string())
            .categories(vec!["a".to_string(), "b".to_string(), "c".to_string()])
            .series(vec![cat("x", &[1.0, 2.0, 3.0]), cat("y", &[3.0, 2.0, 1.0])])
        }),
    )
    .await;
    assert_eq!(query_all(&host, "path.dviz-bar").len(), 6);
    assert_eq!(
        query_all(&host, ".dviz-labels-bar text").len(),
        6,
        "a value on every cap"
    );
    assert_eq!(query_all(&host, ".dviz-legend-item").len(), 2);
    let first = query_all(&host, "path.dviz-bar")[0]
        .get_attribute("d")
        .unwrap();
    assert!(first.contains('A'), "rounded data end: {first}");
    host.remove();
}

#[wasm_bindgen_test]
async fn single_series_has_no_legend() {
    let host = mount(
        "single",
        500.0,
        dwind_dviz::line_chart!({
            .label("One".to_string())
            .series(vec![cat("only", &[1.0, 2.0])])
        }),
    )
    .await;
    assert!(query_all(&host, ".dviz-legend-item").is_empty());
    assert_eq!(
        query_all(&host, ".dviz-labels-end text").len(),
        1,
        "end label still names the series"
    );
    host.remove();
}

#[wasm_bindgen_test]
async fn stacked_bars_share_a_column_and_label_totals() {
    let host = mount(
        "stacked",
        500.0,
        dwind_dviz::bar_chart!({
            .label("Stacked".to_string())
            .mode(BarMode::Stacked)
            .categories(vec!["a".to_string(), "b".to_string()])
            .series(vec![cat("x", &[1.0, 2.0]), cat("y", &[3.0, 4.0])])
        }),
    )
    .await;
    let labels: Vec<String> = query_all(&host, ".dviz-labels-bar text")
        .iter()
        .map(|e| e.text_content().unwrap())
        .collect();
    assert_eq!(labels, vec!["4", "6"]);
    let bars = query_all(&host, "path.dviz-bar");
    assert_eq!(bars.len(), 4);
    host.remove();
}

#[wasm_bindgen_test]
async fn legend_shares_slots_with_the_chart() {
    let data = Mutable::new(vec![cat("a", &[1.0, 2.0]), cat("b", &[2.0, 1.0])]);
    let host = mount(
        "slots",
        500.0,
        dwind_dviz::line_chart!({
            .label("Slots".to_string())
            .series_signal(data.signal_cloned())
        }),
    )
    .await;
    let swatches = query_all(&host, ".dviz-legend-swatch");
    assert!(
        swatches[1]
            .get_attribute("style")
            .unwrap()
            .contains("--dviz-series-2")
    );
    data.set(vec![cat("b", &[2.0, 1.0])]);
    TimeoutFuture::new(400).await;
    let paths = query_all(&host, "path.dviz-line");
    assert_eq!(paths.len(), 1);
    assert!(
        paths[0]
            .get_attribute("style")
            .unwrap()
            .contains("--dviz-series-2"),
        "survivor keeps its slot"
    );
    assert!(
        query_all(&host, ".dviz-legend-item").is_empty(),
        "legend disappears below two series"
    );
    host.remove();
}

#[wasm_bindgen_test]
async fn area_points_and_annotations_render() {
    let host = mount(
        "misc",
        500.0,
        dwind_dviz::chart!({
            .label("Misc".to_string())
            .x_domain(XDomain::Linear(Extent::new(0.0, 2.0)))
            .y_domain(YDomain::Linear(Extent::new(0.0, 10.0)))
            .layers(vec![
                area(always(vec![cat("a", &[1.0, 5.0, 3.0])])),
                points(always(vec![cat("a", &[1.0, 5.0, 3.0])])),
                reference_y(always(8.0), always("limit".to_string())),
                band_y(always(Extent::new(2.0, 4.0)), always("ok".to_string())),
                reference_x(always(1.0), always("event".to_string())),
            ])
        }),
    )
    .await;
    assert_eq!(query_all(&host, "path.dviz-area").len(), 1);
    assert_eq!(query_all(&host, "circle.dviz-point").len(), 3);
    assert_eq!(query_all(&host, "line.dviz-reference").len(), 2);
    assert_eq!(query_all(&host, "rect.dviz-band").len(), 1);
    let labels: Vec<String> = query_all(&host, ".dviz-annotations text")
        .iter()
        .map(|e| e.text_content().unwrap())
        .collect();
    assert_eq!(labels, vec!["limit", "ok", "event"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn heatmap_cells_and_donut_arcs() {
    let host = mount(
        "cells",
        500.0,
        html!("div", {
            .child(dwind_dviz::chart!({
                .label("Heat".to_string())
                .x_domain(XDomain::band(["a", "b"]))
                .y_domain(YDomain::band(["r1", "r2"]))
                .layers(vec![axis_x(), axis_y(), cells(always(vec![Cell::new(0, 0, 1.0), Cell::new(1, 1, 5.0)]), CellOptions::default())])
            }))
            .child(dwind_dviz::donut_chart!({
                .label("Donut".to_string())
                .series((0..8).map(|i| Series::new(format!("s{i}"), format!("S{i}"), vec![Point::new(0.0, (i + 1) as f64)])).collect::<Vec<_>>())
            }))
        }),
    )
    .await;
    let rects = query_all(&host, "rect.dviz-cell");
    assert_eq!(rects.len(), 2);
    assert_ne!(
        rects[0].get_attribute("fill"),
        rects[1].get_attribute("fill")
    );
    assert_eq!(
        query_all(&host, ".dviz-axis-y text").len(),
        2,
        "band y axis lists categories"
    );
    assert_eq!(
        query_all(&host, "path.dviz-arc").len(),
        6,
        "eight fold to five plus Other"
    );
    assert_eq!(
        query_all(&host, ".dviz-legend-row").len(),
        6,
        "value legend lists every segment"
    );
    assert_eq!(
        query_all(&host, ".dviz-donut-value").len(),
        1,
        "centre total"
    );
    host.remove();
}

#[wasm_bindgen_test]
async fn stat_tile_shows_value_delta_and_trend() {
    let host = mount(
        "tile",
        300.0,
        dwind_dviz::stat_tile!({
            .label("Latency".to_string())
            .value(12_900.0)
            .delta(Some(0.05))
            .up_is_good(false)
            .trend(vec![1.0, 2.0, 3.0])
        }),
    )
    .await;
    assert_eq!(
        host.query_selector(".dviz-stat-value")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("12.9K")
    );
    let delta = host.query_selector(".dviz-stat-delta").unwrap().unwrap();
    assert_eq!(
        delta.get_attribute("data-tone").as_deref(),
        Some("bad"),
        "up is bad for latency"
    );
    assert!(delta.text_content().unwrap().contains("5%"));
    assert_eq!(query_all(&host, "path.dviz-sparkline").len(), 1);
    assert_eq!(query_all(&host, "circle.dviz-sparkline-end").len(), 1);
    host.remove();
}
