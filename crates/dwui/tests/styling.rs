//! Browser tests for the styling layer: `dwkeyframes!` injection, pseudo-element
//! variants, and arbitrary declarations.
//!
//! These live in dwui rather than dwind because dwui is the crate already wired
//! for `wasm-pack test`, and because the last test here guards dwui's own
//! keyframe migration.
//!
//! Run with: `wasm-pack test --headless --firefox crates/dwui`

#![cfg(target_arch = "wasm32")]

use dominator::html;
use dwind::prelude::*;
use dwind_macros::{dwclass, dwkeyframes};
use wasm_bindgen_test::*;
use web_sys::js_sys;
use web_sys::wasm_bindgen::JsCast;

wasm_bindgen_test_configure!(run_in_browser);

dwkeyframes! {
    #![prefix = "dwuitest"]

    #[animation("1s linear infinite")]
    slide_probe {
        "from" => "transform: translateX(0);",
        "to" => "transform: translateX(10px);",
    }

    /// Declared but never used, to prove injection is lazy.
    unused_probe {
        "from" => "opacity: 0;",
        "to" => "opacity: 1;",
    }
}

struct TestContainer {
    element: web_sys::Element,
}

impl TestContainer {
    fn new() -> Self {
        let doc = web_sys::window().unwrap().document().unwrap();
        let el = doc.create_element("div").unwrap();
        el.set_attribute(
            "style",
            "position:absolute;left:0;top:0;width:800px;height:600px",
        )
        .unwrap();
        doc.body().unwrap().append_child(&el).unwrap();
        Self { element: el }
    }

    fn dom_element(&self) -> web_sys::HtmlElement {
        self.element.clone().dyn_into().unwrap()
    }
}

impl Drop for TestContainer {
    fn drop(&mut self) {
        self.element.remove();
    }
}

async fn wait_frame() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .request_animation_frame(&resolve)
            .unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}

/// Counts `@keyframes` rules with this name across every stylesheet in the
/// document. More than one means the registry failed to deduplicate.
fn count_keyframes(name: &str) -> usize {
    let doc = web_sys::window().unwrap().document().unwrap();
    let sheets = doc.style_sheets();
    let mut found = 0;

    for i in 0..sheets.length() {
        let Some(sheet) = sheets.item(i) else {
            continue;
        };
        let Ok(sheet) = sheet.dyn_into::<web_sys::CssStyleSheet>() else {
            continue;
        };
        // A cross-origin sheet throws on access; skip it.
        let Ok(rules) = sheet.css_rules() else {
            continue;
        };

        for r in 0..rules.length() {
            let Some(rule) = rules.item(r) else { continue };

            if let Ok(keyframes) = rule.dyn_into::<web_sys::CssKeyframesRule>() {
                if keyframes.name() == name {
                    found += 1;
                }
            }
        }
    }

    found
}

fn computed_pseudo(element: &web_sys::Element, pseudo: &str, property: &str) -> String {
    web_sys::window()
        .unwrap()
        .get_computed_style_with_pseudo_elt(element, pseudo)
        .unwrap()
        .unwrap()
        .get_property_value(property)
        .unwrap()
}

// ---------------------------------------------------------------------------
// dwkeyframes!
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn keyframes_are_injected_lazily_and_only_once() {
    // Nothing has touched `unused_probe`, so its rule must not be in the
    // document. This is the property the old eager `&str` blob could not offer.
    assert_eq!(
        count_keyframes("dwuitest-unused-probe"),
        0,
        "an unused keyframe was injected"
    );

    let tc = TestContainer::new();

    // Instantiate the class twice: the registry must inject exactly one rule.
    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .dwclass!("animate-slide-probe")
            .child(html!("span", { .dwclass!("animate-slide-probe") }))
        }),
    );
    wait_frame().await;

    assert_eq!(
        count_keyframes("dwuitest-slide-probe"),
        1,
        "expected exactly one @keyframes rule after two instantiations"
    );
}

#[wasm_bindgen_test]
async fn keyframe_handles_register_when_formatted() {
    // `Display` registers, so a hand-composed shorthand cannot reference a rule
    // that was never injected.
    let shorthand = format!("{UNUSED_PROBE_KEYFRAMES} 1s linear");

    assert!(
        shorthand.starts_with("dwuitest-unused-probe"),
        "{shorthand}"
    );
    assert_eq!(count_keyframes("dwuitest-unused-probe"), 1);
}

#[wasm_bindgen_test]
async fn dwui_keyframes_survived_the_migration() {
    dwui::theme::apply_style_sheet(None);
    // Idempotent: calling twice must not duplicate the rules.
    dwui::theme::apply_style_sheet(None);

    for name in [
        "dwui-modal-in",
        "dwui-fade-in",
        "dwui-progress-indeterminate",
        "dwui-spin",
        "dwui-skeleton-pulse",
    ] {
        assert_eq!(count_keyframes(name), 1, "{name} should be injected once");
    }
}

// ---------------------------------------------------------------------------
// Pseudo-element variants
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn pseudo_element_variants_materialise_without_a_content_utility() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-before")
            .dwclass!("relative [&::before]:absolute [&::before]:opacity-100")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-before").unwrap();

    // Without a `content`, a ::before never generates a box, so this is the
    // property that decides whether the utility is usable on its own.
    let content = computed_pseudo(&el, "::before", "content");
    assert!(
        content != "none",
        "::before had no generated content (got {content:?})"
    );
    assert_eq!(computed_pseudo(&el, "::before", "position"), "absolute");
}

#[wasm_bindgen_test]
async fn before_shorthand_matches_the_bracket_form() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-alias")
            .dwclass!("relative before:absolute")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-alias").unwrap();

    assert!(computed_pseudo(&el, "::before", "content") != "none");
    assert_eq!(computed_pseudo(&el, "::before", "position"), "absolute");
}

#[wasm_bindgen_test]
async fn pseudo_classes_do_not_gain_content() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-hover")
            .dwclass!("hover:opacity-50")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-hover").unwrap();

    // The element itself is unaffected; only `::before`/`::after` get content.
    assert_eq!(computed_pseudo(&el, "::before", "content"), "none");
}

// ---------------------------------------------------------------------------
// Arbitrary declarations
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn arbitrary_declarations_reach_the_element() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-arbitrary")
            .dwclass!("[mix-blend-mode:overlay] [--sx:42%] [letter-spacing:0.5px]")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-arbitrary").unwrap();
    let style = web_sys::window()
        .unwrap()
        .get_computed_style(&el)
        .unwrap()
        .unwrap();

    assert_eq!(
        style.get_property_value("mix-blend-mode").unwrap(),
        "overlay"
    );
    assert_eq!(style.get_property_value("letter-spacing").unwrap(), "0.5px");
    // Custom properties round-trip too, which is what the pointer-tracking
    // effects in the example app need.
    assert_eq!(style.get_property_value("--sx").unwrap().trim(), "42%");
}

#[wasm_bindgen_test]
async fn arbitrary_declarations_compose_with_pseudo_elements() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-arb-before")
            .dwclass!("relative [&::before]:[mix-blend-mode:screen]")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-arb-before").unwrap();

    assert!(computed_pseudo(&el, "::before", "content") != "none");
    assert_eq!(computed_pseudo(&el, "::before", "mix-blend-mode"), "screen");
}

// ---------------------------------------------------------------------------
// New utilities
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn newly_added_utilities_apply() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-utils")
            .dwclass!("absolute inset-0 isolate no-underline whitespace-nowrap will-change-transform delay-150 tracking-tight")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-utils").unwrap();
    let style = web_sys::window()
        .unwrap()
        .get_computed_style(&el)
        .unwrap()
        .unwrap();

    assert_eq!(style.get_property_value("isolation").unwrap(), "isolate");
    assert_eq!(style.get_property_value("white-space").unwrap(), "nowrap");
    assert_eq!(
        style.get_property_value("transition-delay").unwrap(),
        "0.15s"
    );
    assert_eq!(style.get_property_value("top").unwrap(), "0px");
    assert_eq!(
        style.get_property_value("text-decoration-line").unwrap(),
        "none"
    );
}
