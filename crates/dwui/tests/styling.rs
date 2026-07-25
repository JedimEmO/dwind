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

    /// Only ever reached through a `hover:` variant.
    #[animation("1s linear infinite")]
    hover_probe {
        "from" => "opacity: 0.4;",
        "to" => "opacity: 1;",
    }

    /// Only ever reached through a `[&::before]:` variant.
    #[animation("1s linear infinite")]
    before_probe {
        "from" => "opacity: 0.4;",
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

#[wasm_bindgen_test]
async fn modified_animation_utilities_still_register_their_keyframes() {
    // A variant compiles the declaration text into a fresh class and never
    // touches the generated utility, so registration has to hang off the text.
    assert_eq!(count_keyframes("dwuitest-hover-probe"), 0);
    assert_eq!(count_keyframes("dwuitest-before-probe"), 0);

    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .dwclass!("hover:animate-hover-probe")
            .child(html!("span", { .dwclass!("relative [&::before]:animate-before-probe") }))
        }),
    );
    wait_frame().await;

    assert_eq!(
        count_keyframes("dwuitest-hover-probe"),
        1,
        "hover:animate-* did not inject its @keyframes"
    );
    assert_eq!(
        count_keyframes("dwuitest-before-probe"),
        1,
        "[&::before]:animate-* did not inject its @keyframes"
    );
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
async fn explicit_pseudo_content_survives_composition() {
    // Each utility is its own class, so a literal `content: ""` from
    // `before:absolute` would be decided against `before:[content:'x']` by rule
    // order. Routing through --dw-content removes the ordering question.
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-compose")
            .dwclass!("relative before:[content:'x'] before:absolute before:opacity-100")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-compose").unwrap();

    let content = computed_pseudo(&el, "::before", "content");
    assert!(
        content.contains('x'),
        "composed pseudo-element lost its content (got {content:?})"
    );
    assert_eq!(computed_pseudo(&el, "::before", "position"), "absolute");
}

#[wasm_bindgen_test]
async fn arbitrary_values_may_be_non_ascii() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-unicode")
            .dwclass!("relative before:[content:'→'] before:m-r-2")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-unicode").unwrap();

    let content = computed_pseudo(&el, "::before", "content");
    assert!(content.contains('→'), "got {content:?}");
    // The class after the Unicode one must survive too — an unparsed remainder
    // used to discard it silently.
    assert_eq!(computed_pseudo(&el, "::before", "margin-right"), "8px");
}

#[wasm_bindgen_test]
async fn quoted_values_may_contain_brackets() {
    // `[content:'[']` is valid CSS. The parser has to know that a bracket inside
    // a string is content rather than a delimiter.
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-bracket")
            .dwclass!("relative before:[content:'['] before:absolute")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-bracket").unwrap();

    let content = computed_pseudo(&el, "::before", "content");
    assert!(content.contains('['), "got {content:?}");
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

#[wasm_bindgen_test]
async fn pseudo_content_composes_in_either_order() {
    // --dw-content is order-independent by construction, but that is the whole
    // claim, so measure it rather than reason about it.
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .child(html!("div", {
                .attr("id", "order-a")
                .dwclass!("relative before:[content:'a'] before:absolute")
            }))
            .child(html!("div", {
                .attr("id", "order-b")
                .dwclass!("relative before:absolute before:[content:'b']")
            }))
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();

    for (id, want) in [("order-a", 'a'), ("order-b", 'b')] {
        let el = doc.get_element_by_id(id).unwrap();
        let content = computed_pseudo(&el, "::before", "content");

        assert!(content.contains(want), "{id}: got {content:?}");
        assert_eq!(computed_pseudo(&el, "::before", "position"), "absolute");
    }
}

#[wasm_bindgen_test]
async fn content_utilities_reach_the_pseudo_element() {
    // `content-none` has to suppress the generated default, or a utility that
    // only wants to hide a pseudo-element cannot.
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-content-none")
            .dwclass!("relative before:absolute before:content-none")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-content-none").unwrap();

    assert_eq!(computed_pseudo(&el, "::before", "content"), "none");
}

// dwgenerate! must be able to alias any class — a plain utility, and one minted
// by dwkeyframes!. Aliasing a bare class never worked before (the generated
// static tried to hold a `&String` in a `String`), and the AnimationDecl change
// added a second error on the same path.
dwind_macros::dwgenerate!("aliased-flex", "flex");
dwind_macros::dwgenerate!("aliased-anim", "animate-slide-probe");

#[wasm_bindgen_test]
async fn dwgenerate_can_alias_a_plain_class_and_an_animation() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-alias")
            .dwclass!("aliased-flex aliased-anim")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-alias").unwrap();
    let style = web_sys::window()
        .unwrap()
        .get_computed_style(&el)
        .unwrap()
        .unwrap();

    assert_eq!(style.get_property_value("display").unwrap(), "flex");
    // Aliasing the animation must still register its keyframes.
    assert!(
        style
            .get_property_value("animation-name")
            .unwrap()
            .contains("slide-probe"),
        "{:?}",
        style.get_property_value("animation-name")
    );
    assert_eq!(count_keyframes("dwuitest-slide-probe"), 1);
}

// ---------------------------------------------------------------------------
// Selector forms
// ---------------------------------------------------------------------------

// A selector the engine cannot parse is not ignored: `insertRule` throws and
// dominator panics ("selectors are incorrect"), taking the whole app down at
// load. A raw stylesheet would have dropped the rule silently, so moving a
// selector into `dwclass!` makes an unsupported one fatal.
//
// These pin the shapes the example webpage actually applies. CI runs both
// Firefox and Chrome, so an engine that rejects any of them fails the build
// rather than the page.

#[wasm_bindgen_test]
async fn pointer_spotlight_selectors_insert() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .dwclass!("relative isolate [transition:transform 400ms cubic-bezier(0.16, 1, 0.3, 1), border-color 300ms ease]")
            .dwclass!("[&::before]:absolute [&::before]:inset-0 [&::before]:[z-index:-1] \
                [&::before]:[border-radius:inherit] [&::before]:opacity-0 \
                [&::before]:[transition:opacity 320ms ease] [&.hot::before]:opacity-100 \
                [&::before]:[background:radial-gradient(22rem circle at var(--sx, 50%) var(--sy, 50%), rgba(213, 182, 95, 0.13), transparent 62%)]")
            .dwclass!("[&::after]:absolute [&::after]:inset-0 [&::after]:[z-index:-1] \
                [&::after]:[border-radius:inherit] [&::after]:[padding:1px] \
                [&::after]:opacity-0 [&::after]:[transition:opacity 320ms ease] [&.hot::after]:opacity-100 \
                [&::after]:[-webkit-mask:linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0)] \
                [&::after]:[-webkit-mask-composite:xor] \
                [&::after]:[mask:linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0)] \
                [&::after]:[mask-composite:exclude]")
        }),
    );
    wait_frame().await;
}

#[wasm_bindgen_test]
async fn child_and_state_variant_selectors_insert() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            // The scroll-reveal cascade: parent-state plus :nth-child stagger.
            .dwclass!("[& > *]:opacity-0 [& > *]:[transform:translateY(26px)] \
                [& > *]:[transition:opacity 650ms cubic-bezier(0.16, 1, 0.3, 1), transform 650ms cubic-bezier(0.16, 1, 0.3, 1)]")
            .dwclass!("[& > *:nth-child(2)]:delay-75 [& > *:nth-child(5)]:[transition-delay:280ms]")
            .dwclass!("[&.reveal-in > *]:opacity-100")
            // Hover the parent, animate the child — the marquee pause.
            .dwclass!("[&:hover > *]:[animation-play-state:paused]")
        }),
    );
    wait_frame().await;
}

#[wasm_bindgen_test]
async fn scrollbar_styling_uses_standard_properties_only() {
    // `[&::-webkit-scrollbar-thumb:hover]:` panics in Firefox: the engine
    // tolerates the unknown pseudo-element on its own but rejects it with a
    // pseudo-class appended. The standard properties cover both engines.
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .attr("id", "probe-scrollbar")
            .dwclass!("[scrollbar-width:thin] [scrollbar-color:#26262C transparent]")
        }),
    );
    wait_frame().await;

    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.get_element_by_id("probe-scrollbar").unwrap();
    let style = web_sys::window()
        .unwrap()
        .get_computed_style(&el)
        .unwrap()
        .unwrap();

    assert_eq!(style.get_property_value("scrollbar-width").unwrap(), "thin");
}
