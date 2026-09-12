//! Browser tests for hover, keyboard access, legend toggling and zoom.

#![cfg(target_arch = "wasm32")]

use dominator::{Dom, append_dom, body, html};
use dwind_dviz::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{
    Element, HtmlElement, KeyboardEvent, KeyboardEventInit, PointerEvent, PointerEventInit,
};

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

/// The tooltip element persists; visibility is an attribute.
fn tooltip_visible(host: &HtmlElement) -> bool {
    query_all(host, ".dviz-tooltip")
        .iter()
        .any(|t| t.get_attribute("data-visible").as_deref() == Some("true"))
}

fn one(root: &Element, selector: &str) -> Element {
    root.query_selector(selector).unwrap().unwrap()
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

fn pointer(el: &Element, kind: &str, client_x: f64, client_y: f64) {
    let init = PointerEventInit::new();
    init.set_client_x(client_x as i32);
    init.set_client_y(client_y as i32);
    init.set_bubbles(true);
    init.set_pointer_id(1);
    let ev = PointerEvent::new_with_event_init_dict(kind, &init).unwrap();
    el.dispatch_event(&ev).unwrap();
}

fn key(el: &Element, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_key(key);
    init.set_bubbles(true);
    let ev = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    el.dispatch_event(&ev).unwrap();
}

#[wasm_bindgen_test]
async fn crosshair_snaps_and_shows_a_tooltip_for_every_series() {
    let host = mount(
        "cross",
        400.0,
        dwind_dviz::line_chart!({
            .label("Cross".to_string())
            .series(vec![cat("a", &[1.0, 2.0, 3.0, 4.0]), cat("b", &[4.0, 3.0, 2.0, 1.0])])
        }),
    )
    .await;
    let overlay = one(&host, ".dviz-overlay");
    let rect = overlay.get_bounding_client_rect();
    // Near the right end: snaps to x = 3.
    pointer(
        &overlay,
        "pointermove",
        rect.right() - 2.0,
        rect.top() + 10.0,
    );
    TimeoutFuture::new(20).await;
    let tip = one(&host, ".dviz-tooltip");
    assert_eq!(
        one(&tip, ".dviz-tooltip-title").text_content().as_deref(),
        Some("3")
    );
    let values: Vec<String> = query_all(&tip, ".dviz-tooltip-value")
        .iter()
        .map(|e| e.text_content().unwrap())
        .collect();
    assert_eq!(values, vec!["4", "1"]);
    assert_eq!(query_all(&host, ".dviz-crosshair-dot").len(), 2);
    assert!(query_all(&host, "line.dviz-crosshair").len() == 1);

    pointer(&overlay, "pointerleave", 0.0, 0.0);
    TimeoutFuture::new(20).await;
    assert!(!tooltip_visible(&host), "tooltip clears on leave");
    host.remove();
}

#[wasm_bindgen_test]
async fn keyboard_steps_through_points() {
    let host = mount(
        "keys",
        400.0,
        dwind_dviz::line_chart!({
            .label("Keys".to_string())
            .series(vec![cat("a", &[10.0, 20.0, 30.0])])
        }),
    )
    .await;
    let overlay = one(&host, ".dviz-overlay");
    assert_eq!(overlay.get_attribute("tabindex").as_deref(), Some("0"));
    overlay
        .dyn_ref::<web_sys::SvgElement>()
        .unwrap()
        .focus()
        .unwrap();
    TimeoutFuture::new(20).await;
    let title = || one(&host, ".dviz-tooltip-title").text_content().unwrap();
    assert_eq!(title(), "2", "focus lands on the last point");
    key(&overlay, "ArrowLeft");
    TimeoutFuture::new(20).await;
    assert_eq!(title(), "1");
    key(&overlay, "Home");
    TimeoutFuture::new(20).await;
    assert_eq!(title(), "0");
    key(&overlay, "Escape");
    TimeoutFuture::new(20).await;
    assert!(!tooltip_visible(&host));
    host.remove();
}

#[wasm_bindgen_test]
async fn legend_toggles_series_and_survivors_keep_colors() {
    let host = mount(
        "toggle",
        400.0,
        dwind_dviz::bar_chart!({
            .label("Toggle".to_string())
            .categories(vec!["a".to_string(), "b".to_string()])
            .series(vec![cat("x", &[1.0, 2.0]), cat("y", &[2.0, 1.0])])
        }),
    )
    .await;
    let items = query_all(&host, ".dviz-legend-item");
    assert_eq!(items[0].get_attribute("role").as_deref(), Some("button"));
    assert_eq!(
        items[0].get_attribute("aria-pressed").as_deref(),
        Some("true")
    );
    items[0].dyn_ref::<HtmlElement>().unwrap().click();
    TimeoutFuture::new(400).await;
    let items = query_all(&host, ".dviz-legend-item");
    assert_eq!(items.len(), 2, "hidden series stays listed");
    assert_eq!(
        items[0].get_attribute("aria-pressed").as_deref(),
        Some("false")
    );
    let bars = query_all(&host, "path.dviz-bar");
    assert_eq!(bars.len(), 2, "only y's bars remain");
    let slots = SeriesSlots::new();
    let y_color = slots.color_for("y");
    assert!(bars[0].get_attribute("style").unwrap().contains(&y_color));
    items[0].dyn_ref::<HtmlElement>().unwrap().click();
    TimeoutFuture::new(20).await;
    assert_eq!(query_all(&host, "path.dviz-bar").len(), 4);
    host.remove();
}

#[wasm_bindgen_test]
async fn marks_show_tooltips_on_hover_and_focus() {
    let host = mount(
        "marks",
        400.0,
        html!("div", {
            .child(dwind_dviz::bar_chart!({
                .label("Bars".to_string())
                .categories(vec!["jan".to_string()])
                .series(vec![cat("x", &[1234.0])])
            }))
            .child(dwind_dviz::scatter_chart!({
                .label("Dots".to_string())
                .series(vec![cat("d", &[5.0])])
            }))
        }),
    )
    .await;
    let bar = one(&host, "path.dviz-bar");
    pointer(&bar, "pointerenter", 0.0, 0.0);
    TimeoutFuture::new(20).await;
    let tip = one(&host, ".dviz-tooltip");
    assert_eq!(
        one(&tip, ".dviz-tooltip-title").text_content().as_deref(),
        Some("jan")
    );
    assert_eq!(
        one(&tip, ".dviz-tooltip-value").text_content().as_deref(),
        Some("1,234")
    );
    pointer(&bar, "pointerleave", 0.0, 0.0);
    TimeoutFuture::new(20).await;
    assert!(!tooltip_visible(&host));

    let hit = one(&host, "circle.dviz-hit");
    assert!(
        hit.get_attribute("r").unwrap().parse::<f64>().unwrap() >= 12.0,
        "24px hit target"
    );
    hit.dyn_ref::<web_sys::SvgElement>()
        .unwrap()
        .focus()
        .unwrap();
    TimeoutFuture::new(20).await;
    assert!(tooltip_visible(&host), "focus shows the tooltip");
    host.remove();
}

#[wasm_bindgen_test]
async fn brush_zooms_and_reset_restores() {
    let host = mount(
        "zoom",
        400.0,
        dwind_dviz::line_chart!({
            .label("Zoom".to_string())
            .zoomable(true)
            .series(vec![cat("a", &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0])])
        }),
    )
    .await;
    let labels = |host: &HtmlElement| -> Vec<String> {
        query_all(host, ".dviz-axis-x .dviz-tick:not(.dviz-leave) text")
            .iter()
            .map(|e| e.text_content().unwrap())
            .collect()
    };
    let before = labels(&host);
    assert_eq!(before.last().map(String::as_str), Some("10"));
    assert!(host.query_selector(".dviz-zoom-reset").unwrap().is_none());

    let overlay = one(&host, ".dviz-overlay");
    let rect = overlay.get_bounding_client_rect();
    let y = rect.top() + 10.0;
    pointer(&overlay, "pointerdown", rect.left() + 10.0, y);
    pointer(&overlay, "pointermove", rect.left() + rect.width() / 2.0, y);
    TimeoutFuture::new(20).await;
    assert_eq!(
        query_all(&host, "rect.dviz-brush").len(),
        1,
        "selection drawn while dragging"
    );
    pointer(&overlay, "pointerup", rect.left() + rect.width() / 2.0, y);
    TimeoutFuture::new(50).await;
    let after = labels(&host);
    assert_ne!(after, before);
    assert!(
        after.last().unwrap().parse::<f64>().unwrap() <= 6.0,
        "{after:?}"
    );
    assert!(query_all(&host, "rect.dviz-brush").is_empty());

    let reset = one(&host, ".dviz-zoom-reset");
    reset.dyn_ref::<HtmlElement>().unwrap().click();
    TimeoutFuture::new(50).await;
    assert_eq!(labels(&host), before);
    host.remove();
}
