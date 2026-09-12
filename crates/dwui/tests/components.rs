//! Browser-based component tests.
//!
//! Run with: `wasm-pack test --headless --firefox crates/dwui`

#![cfg(target_arch = "wasm32")]

#[macro_use]
extern crate dwui;

use dominator::{clone, html, text};
use dwui::prelude::*;
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use wasm_bindgen_test::*;
use web_sys::js_sys;
use web_sys::wasm_bindgen::JsCast;

wasm_bindgen_test_configure!(run_in_browser);

/// Isolated test container. Removed from DOM on drop.
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

    fn query_all(&self, selector: &str) -> web_sys::NodeList {
        self.element.query_selector_all(selector).unwrap()
    }

    fn query(&self, selector: &str) -> Option<web_sys::Element> {
        self.element.query_selector(selector).unwrap()
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

async fn wait_frames(n: usize) {
    for _ in 0..n {
        wait_frame().await;
    }
}

fn click(element: &web_sys::Element) {
    element
        .clone()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .click();
}

/// Query the whole document — used for overlays that portal to `body`.
fn doc_query(selector: &str) -> Option<web_sys::Element> {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector(selector)
        .unwrap()
}

#[wasm_bindgen_test]
async fn button_is_a_real_button_and_handles_clicks() {
    let tc = TestContainer::new();
    let clicked = Mutable::new(0);

    dominator::append_dom(
        &tc.dom_element(),
        button!({
            .content(Some(text("Click me")))
            .on_click(clone!(clicked => move |_| {
                clicked.set(clicked.get() + 1);
            }))
        }),
    );
    wait_frame().await;

    let button = tc.query("button").unwrap();
    assert_eq!(button.get_attribute("type").as_deref(), Some("button"));
    assert_eq!(button.text_content().unwrap(), "Click me");

    click(&button);
    wait_frame().await;

    assert_eq!(clicked.get(), 1);
}

#[wasm_bindgen_test]
async fn button_disabled_signal_sets_disabled_attribute() {
    let tc = TestContainer::new();
    let disabled = Mutable::new(false);

    dominator::append_dom(
        &tc.dom_element(),
        button!({
            .content(Some(text("Disabled?")))
            .disabled_signal(disabled.signal())
        }),
    );
    wait_frame().await;

    let button = tc.query("button").unwrap();
    assert!(!button.has_attribute("disabled"));

    disabled.set(true);
    wait_frame().await;

    assert!(button.has_attribute("disabled"));
}

#[wasm_bindgen_test]
async fn text_input_associates_label_with_input() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        text_input!({
            .label("Username".to_string())
        }),
    );
    wait_frame().await;

    let input = tc.query("input").unwrap();
    let label = tc.query("label").unwrap();

    let input_id = input.get_attribute("id").unwrap();
    assert!(!input_id.is_empty());
    assert_eq!(label.get_attribute("for").unwrap(), input_id);
    assert_eq!(label.text_content().unwrap(), "Username");
}

#[wasm_bindgen_test]
async fn text_input_exposes_validation_to_assistive_technology() {
    let tc = TestContainer::new();
    let validity = Mutable::new(ValidationResult::Valid);

    dominator::append_dom(
        &tc.dom_element(),
        text_input!({
            .label("Email".to_string())
            .is_valid_signal(validity.signal_cloned())
        }),
    );
    wait_frame().await;

    let input = tc.query("input").unwrap();
    assert!(!input.has_attribute("aria-invalid"));
    assert!(tc.query("[role=alert]").is_none());

    validity.set(ValidationResult::Invalid {
        message: "Email is required".to_string(),
    });
    wait_frames(2).await;

    assert_eq!(input.get_attribute("aria-invalid").as_deref(), Some("true"));

    let alert = tc.query("[role=alert]").unwrap();
    assert_eq!(alert.text_content().unwrap(), "Email is required");
    assert_eq!(
        input.get_attribute("aria-describedby").unwrap(),
        alert.get_attribute("id").unwrap()
    );
}

#[wasm_bindgen_test]
async fn text_input_writes_back_to_the_value_wrapper() {
    let tc = TestContainer::new();
    let value = Mutable::new("hello".to_string());

    dominator::append_dom(
        &tc.dom_element(),
        text_input!({
            .value(value.clone())
            .label("Greeting".to_string())
        }),
    );
    wait_frames(2).await;

    let input: web_sys::HtmlInputElement = tc.query("input").unwrap().dyn_into().unwrap();
    assert_eq!(input.value(), "hello");

    value.set("world".to_string());
    wait_frames(2).await;

    assert_eq!(input.value(), "world");
}

#[wasm_bindgen_test]
async fn select_associates_label_and_renders_options() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        select!({
            .label("Fruit".to_string())
            .options(vec![
                ("a".to_string(), "Apple".to_string()),
                ("b".to_string(), "Banana".to_string()),
            ])
        }),
    );
    wait_frames(2).await;

    let select = tc.query("select").unwrap();
    let label = tc.query("label").unwrap();

    assert_eq!(
        label.get_attribute("for").unwrap(),
        select.get_attribute("id").unwrap()
    );
    assert_eq!(tc.query_all("option").length(), 2);
}

#[wasm_bindgen_test]
async fn slider_labels_both_inputs() {
    let tc = TestContainer::new();
    let value = Mutable::new(25.0f32);

    dominator::append_dom(
        &tc.dom_element(),
        slider!({
            .value(value.clone())
            .label("Volume".to_string())
        }),
    );
    wait_frames(2).await;

    let range = tc.query("input[type=range]").unwrap();
    let label = tc.query("label").unwrap();
    let number = tc.query("input[type=number]").unwrap();

    assert_eq!(
        label.get_attribute("for").unwrap(),
        range.get_attribute("id").unwrap()
    );
    assert_eq!(
        number.get_attribute("aria-label").as_deref(),
        Some("Volume value")
    );
}

#[wasm_bindgen_test]
async fn switch_exposes_and_toggles_state() {
    let tc = TestContainer::new();
    let checked = Mutable::new(false);

    dominator::append_dom(
        &tc.dom_element(),
        switch!({
            .checked_signal(checked.signal())
            .label("Notifications".to_string())
            .on_change(clone!(checked => move |value| {
                checked.set(value);
            }))
        }),
    );
    wait_frame().await;

    let switch = tc.query("[role=switch]").unwrap();
    assert_eq!(
        switch.get_attribute("aria-checked").as_deref(),
        Some("false")
    );

    // The label is associated and clickable
    let label = tc.query("label").unwrap();
    assert_eq!(
        label.get_attribute("for").unwrap(),
        switch.get_attribute("id").unwrap()
    );

    click(&switch);
    wait_frame().await;

    assert!(checked.get());
    assert_eq!(
        switch.get_attribute("aria-checked").as_deref(),
        Some("true")
    );
}

#[wasm_bindgen_test]
async fn checkbox_exposes_and_toggles_state() {
    let tc = TestContainer::new();
    let checked = Mutable::new(true);

    dominator::append_dom(
        &tc.dom_element(),
        checkbox!({
            .checked_signal(checked.signal())
            .label("Accept terms".to_string())
            .on_change(clone!(checked => move |value| {
                checked.set(value);
            }))
        }),
    );
    wait_frame().await;

    let checkbox = tc.query("[role=checkbox]").unwrap();
    assert_eq!(
        checkbox.get_attribute("aria-checked").as_deref(),
        Some("true")
    );

    click(&checkbox);
    wait_frame().await;

    assert!(!checked.get());
    assert_eq!(
        checkbox.get_attribute("aria-checked").as_deref(),
        Some("false")
    );
}

#[wasm_bindgen_test]
async fn progress_exposes_value_to_assistive_technology() {
    let tc = TestContainer::new();
    let value = Mutable::new(30.0);

    dominator::append_dom(
        &tc.dom_element(),
        progress!({
            .value_signal(value.signal())
            .label("Upload progress".to_string())
        }),
    );
    wait_frame().await;

    let bar = tc.query("[role=progressbar]").unwrap();
    assert_eq!(bar.get_attribute("aria-valuemin").as_deref(), Some("0"));
    assert_eq!(bar.get_attribute("aria-valuemax").as_deref(), Some("100"));
    assert_eq!(bar.get_attribute("aria-valuenow").as_deref(), Some("30"));
    assert_eq!(
        bar.get_attribute("aria-label").as_deref(),
        Some("Upload progress")
    );

    value.set(150.0);
    wait_frame().await;

    // Values are clamped to the valid range
    assert_eq!(bar.get_attribute("aria-valuenow").as_deref(), Some("100"));
}

#[wasm_bindgen_test]
async fn progress_indeterminate_omits_valuenow() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        progress!({
            .indeterminate(true)
        }),
    );
    wait_frame().await;

    let bar = tc.query("[role=progressbar]").unwrap();
    assert!(!bar.has_attribute("aria-valuenow"));
}

#[wasm_bindgen_test]
async fn modal_has_dialog_semantics_and_closes() {
    use discard::Discard;

    let tc = TestContainer::new();
    let open = Mutable::new(true);

    let handle = dominator::append_dom(
        &tc.dom_element(),
        modal!({
            .open_signal(open.signal())
            .aria_label("Example dialog".to_string())
            .on_close(clone!(open => move || {
                open.set(false);
            }))
            .content(Some(text("Dialog body")))
        }),
    );
    wait_frame().await;

    // The modal portals to the body, so query the document
    let dialog = doc_query("[role=dialog]").unwrap();
    assert_eq!(dialog.get_attribute("aria-modal").as_deref(), Some("true"));
    assert_eq!(
        dialog.get_attribute("aria-label").as_deref(),
        Some("Example dialog")
    );

    let close_button = doc_query("button[aria-label='Close dialog']").unwrap();
    click(&close_button);
    wait_frame().await;

    assert!(!open.get());

    handle.discard();
    wait_frame().await;
    assert!(
        doc_query("[role=dialog]").is_none(),
        "the portal should clean up"
    );
}

#[wasm_bindgen_test]
async fn tab_list_follows_the_aria_tabs_pattern() {
    let tc = TestContainer::new();
    let selected = Mutable::new("one".to_string());

    dominator::append_dom(
        &tc.dom_element(),
        tab_list!({
            .selected_signal(selected.signal_cloned())
            .aria_label("Example tabs".to_string())
            .tabs(vec![
                ("one".to_string(), "One".to_string()),
                ("two".to_string(), "Two".to_string()),
            ])
            .on_select(clone!(selected => move |key| {
                selected.set(key);
            }))
        }),
    );
    wait_frames(2).await;

    let tablist = tc.query("[role=tablist]").unwrap();
    assert_eq!(
        tablist.get_attribute("aria-label").as_deref(),
        Some("Example tabs")
    );

    let tabs = tc.query_all("[role=tab]");
    assert_eq!(tabs.length(), 2);

    let first: web_sys::Element = tabs.get(0).unwrap().dyn_into().unwrap();
    let second: web_sys::Element = tabs.get(1).unwrap().dyn_into().unwrap();

    assert_eq!(
        first.get_attribute("aria-selected").as_deref(),
        Some("true")
    );
    assert_eq!(first.get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(
        second.get_attribute("aria-selected").as_deref(),
        Some("false")
    );
    assert_eq!(second.get_attribute("tabindex").as_deref(), Some("-1"));

    click(&second);
    wait_frames(2).await;

    assert_eq!(selected.get_cloned(), "two");
    assert_eq!(
        second.get_attribute("aria-selected").as_deref(),
        Some("true")
    );
    assert_eq!(second.get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(
        first.get_attribute("aria-selected").as_deref(),
        Some("false")
    );
    assert_eq!(first.get_attribute("tabindex").as_deref(), Some("-1"));
}

#[wasm_bindgen_test]
async fn heading_renders_semantic_levels() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        heading!({
            .content(text("Section title"))
            .level(HeadingLevel::H2)
        }),
    );
    wait_frame().await;

    let h2 = tc.query("h2").unwrap();
    assert_eq!(h2.text_content().unwrap(), "Section title");
    assert!(tc.query("h1").is_none());
}

#[wasm_bindgen_test]
async fn badge_renders_content() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        badge!({
            .content(Some(text("New")))
            .variant(BadgeVariant::Primary)
        }),
    );
    wait_frame().await;

    let badge = tc.query("span").unwrap();
    assert_eq!(badge.text_content().unwrap(), "New");
}

#[wasm_bindgen_test]
async fn alert_roles_match_severity() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        alert!({
            .variant(AlertVariant::Error)
            .title("Something failed".to_string())
        }),
    );
    dominator::append_dom(
        &tc.dom_element(),
        alert!({
            .variant(AlertVariant::Success)
            .title("All good".to_string())
        }),
    );
    wait_frame().await;

    assert_eq!(tc.query_all("[role=alert]").length(), 1);
    assert_eq!(tc.query_all("[role=status]").length(), 1);
}

#[wasm_bindgen_test]
async fn alert_dismiss_hides_and_notifies() {
    let tc = TestContainer::new();
    let dismissed = Mutable::new(false);

    dominator::append_dom(
        &tc.dom_element(),
        alert!({
            .variant(AlertVariant::Info)
            .title("Heads up".to_string())
            .dismissible(true)
            .on_dismiss(clone!(dismissed => move || {
                dismissed.set(true);
            }))
        }),
    );
    wait_frame().await;

    let dismiss_button = tc.query("button[aria-label=Dismiss]").unwrap();
    click(&dismiss_button);
    wait_frame().await;

    assert!(dismissed.get());

    let alert_el = tc.query("[role=status]").unwrap();
    let display = web_sys::window()
        .unwrap()
        .get_computed_style(&alert_el)
        .unwrap()
        .unwrap()
        .get_property_value("display")
        .unwrap();
    assert_eq!(display, "none");
}

#[wasm_bindgen_test]
async fn accordion_expands_and_collapses_exclusively() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        accordion!({
            .items(vec![
                ("First".to_string(), text("first content")),
                ("Second".to_string(), text("second content")),
            ])
        }),
    );
    wait_frames(2).await;

    let headers = tc.query_all("button[aria-expanded]");
    assert_eq!(headers.length(), 2);

    let first: web_sys::Element = headers.get(0).unwrap().dyn_into().unwrap();
    let second: web_sys::Element = headers.get(1).unwrap().dyn_into().unwrap();

    assert_eq!(
        first.get_attribute("aria-expanded").as_deref(),
        Some("false")
    );

    // headers control their panels
    let panel_id = first.get_attribute("aria-controls").unwrap();
    let panel = tc.query(&format!("#{}", panel_id)).unwrap();
    assert_eq!(panel.get_attribute("role").as_deref(), Some("region"));
    assert_eq!(
        panel.get_attribute("aria-labelledby").unwrap(),
        first.get_attribute("id").unwrap()
    );

    click(&first);
    wait_frame().await;
    assert_eq!(
        first.get_attribute("aria-expanded").as_deref(),
        Some("true")
    );

    // opening the second closes the first (exclusive mode)
    click(&second);
    wait_frame().await;
    assert_eq!(
        first.get_attribute("aria-expanded").as_deref(),
        Some("false")
    );
    assert_eq!(
        second.get_attribute("aria-expanded").as_deref(),
        Some("true")
    );

    // clicking an open header closes it
    click(&second);
    wait_frame().await;
    assert_eq!(
        second.get_attribute("aria-expanded").as_deref(),
        Some("false")
    );
}

#[wasm_bindgen_test]
async fn tooltip_is_linked_and_hidden_by_default() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        tooltip!({
            .anchor(Some(text("hover me")))
            .text("More information".to_string())
        }),
    );
    wait_frame().await;

    let tip = tc.query("[role=tooltip]").unwrap();
    let tip_id = tip.get_attribute("id").unwrap();

    let wrapper = tc.query("[aria-describedby]").unwrap();
    assert_eq!(wrapper.get_attribute("aria-describedby").unwrap(), tip_id);

    let tip_el: web_sys::HtmlElement = tip.dyn_into().unwrap();
    assert_eq!(tip_el.style().get_property_value("opacity").unwrap(), "0");
}

#[wasm_bindgen_test]
async fn avatar_falls_back_to_initials() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        avatar!({
            .name("Ada Lovelace".to_string())
        }),
    );
    wait_frames(2).await;

    assert!(tc.query("img").is_none());
    let root = tc.query("[role=img]").unwrap();
    assert_eq!(
        root.get_attribute("aria-label").as_deref(),
        Some("Ada Lovelace")
    );
    assert_eq!(root.text_content().unwrap(), "AL");
}

#[wasm_bindgen_test]
async fn avatar_renders_image_when_src_given() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        avatar!({
            .src(Some("data:image/gif;base64,R0lGODlhAQABAAAAACw=".to_string()))
            .name("Grace Hopper".to_string())
        }),
    );
    wait_frames(2).await;

    let img = tc.query("img").unwrap();
    assert_eq!(img.get_attribute("alt").as_deref(), Some("Grace Hopper"));
}

#[wasm_bindgen_test]
async fn breadcrumbs_mark_the_current_page() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        breadcrumbs!({
            .items(vec![
                ("Home".to_string(), "#/".to_string()),
                ("Docs".to_string(), "#/docs".to_string()),
                ("Colors".to_string(), "#/docs/colors".to_string()),
            ])
        }),
    );
    wait_frames(2).await;

    let nav = tc.query("nav[aria-label=Breadcrumb]").unwrap();
    assert!(nav.query_selector("ol").unwrap().is_some());

    // two links + one current page
    assert_eq!(tc.query_all("a").length(), 2);
    let current = tc.query("[aria-current=page]").unwrap();
    assert_eq!(current.text_content().unwrap(), "Colors");
}

#[wasm_bindgen_test]
async fn skeleton_is_hidden_from_assistive_technology() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        skeleton!({
            .variant(SkeletonVariant::Circle)
        }),
    );
    wait_frame().await;

    let el = tc.query("div[aria-hidden=true]").unwrap();
    assert!(el
        .get_attribute("style")
        .unwrap()
        .contains("dwui-skeleton-pulse"));
}

#[wasm_bindgen_test]
async fn spinner_announces_loading() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        spinner!({
            .label("Crunching numbers".to_string())
        }),
    );
    wait_frame().await;

    let el = tc.query("[role=status]").unwrap();
    assert_eq!(
        el.get_attribute("aria-label").as_deref(),
        Some("Crunching numbers")
    );
}

#[wasm_bindgen_test]
async fn divider_is_a_separator() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        divider!({
            .label("or".to_string())
        }),
    );
    wait_frame().await;

    let el = tc.query("[role=separator]").unwrap();
    assert_eq!(el.text_content().unwrap(), "or");
}

#[wasm_bindgen_test]
async fn button_sizes_set_height_classes() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        button!({
            .size(ButtonSize::Small)
            .content(Some(text("Small")))
        }),
    );
    wait_frame().await;

    let button = tc.query("button").unwrap();
    let height = web_sys::window()
        .unwrap()
        .get_computed_style(&button)
        .unwrap()
        .unwrap()
        .get_property_value("height")
        .unwrap();
    // h-8 == 2rem == 32px
    assert_eq!(height, "32px");
}

#[wasm_bindgen_test]
async fn data_table_sorts_through_headers() {
    let tc = TestContainer::new();
    let sorted: Mutable<Option<(String, SortDirection)>> = Mutable::new(None);
    let last_sort: Mutable<Option<(String, SortDirection)>> = Mutable::new(None);

    dominator::append_dom(
        &tc.dom_element(),
        data_table!({
            .columns(vec![
                TableColumn::new("name", "Name", true),
                TableColumn::new("role", "Role", false),
            ])
            .rows(vec![
                ("1".to_string(), vec![text("Ada"), text("Engineer")]),
                ("2".to_string(), vec![text("Grace"), text("Admiral")]),
            ])
            .sorted_by_signal(sorted.signal_cloned())
            .on_sort(clone!(sorted, last_sort => move |key, direction| {
                last_sort.set(Some((key.clone(), direction)));
                sorted.set(Some((key, direction)));
            }))
        }),
    );
    wait_frames(2).await;

    assert_eq!(tc.query_all("tbody tr").length(), 2);

    let name_header = tc.query("th button").unwrap();
    let name_th = tc.query("th").unwrap();
    assert!(!name_th.has_attribute("aria-sort"));

    click(&name_header);
    wait_frames(2).await;

    assert_eq!(
        last_sort.get_cloned(),
        Some(("name".to_string(), SortDirection::Ascending))
    );
    assert_eq!(
        name_th.get_attribute("aria-sort").as_deref(),
        Some("ascending")
    );

    click(&name_header);
    wait_frames(2).await;

    assert_eq!(
        last_sort.get_cloned(),
        Some(("name".to_string(), SortDirection::Descending))
    );
    assert_eq!(
        name_th.get_attribute("aria-sort").as_deref(),
        Some("descending")
    );

    // the non-sortable column renders no button
    assert_eq!(tc.query_all("th button").length(), 1);
}

#[wasm_bindgen_test]
async fn data_table_row_clicks_report_ids() {
    let tc = TestContainer::new();
    let clicked = Mutable::new(None::<String>);

    dominator::append_dom(
        &tc.dom_element(),
        data_table!({
            .columns(vec![TableColumn::new("name", "Name", false)])
            .rows(vec![
                ("alpha".to_string(), vec![text("Ada")]),
                ("beta".to_string(), vec![text("Grace")]),
            ])
            .rows_clickable(true)
            .on_row_click(clone!(clicked => move |id| {
                clicked.set(Some(id));
            }))
        }),
    );
    wait_frames(2).await;

    let rows = tc.query_all("tbody tr");
    let second: web_sys::Element = rows.get(1).unwrap().dyn_into().unwrap();
    click(&second);
    wait_frame().await;

    assert_eq!(clicked.get_cloned().as_deref(), Some("beta"));
}

#[wasm_bindgen_test]
async fn virtual_scroll_renders_only_a_window() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        virtual_scroll!({
            .item_count(10_000)
            .item_height(30.0)
            .height(300.0)
            .render_item(Box::new(|index: usize| {
                html!("div", {
                    .text(&format!("row {}", index))
                })
            }))
        }),
    );
    wait_frames(2).await;

    let rendered = tc.query_all("[role=listitem]").length();
    assert!(rendered > 0, "some rows render");
    assert!(
        rendered < 50,
        "windowing keeps row count small, got {}",
        rendered
    );

    let first = tc.query("[role=listitem]").unwrap();
    assert_eq!(
        first.get_attribute("aria-setsize").as_deref(),
        Some("10000")
    );
    assert_eq!(first.get_attribute("aria-posinset").as_deref(), Some("1"));

    // scroll deep into the list; the window must follow
    let container: web_sys::Element = tc.query("[role=list]").unwrap();
    container.set_scroll_top(60_000);
    wait_frames(3).await;

    let first_index: usize = tc
        .query("[data-vs-index]")
        .unwrap()
        .get_attribute("data-vs-index")
        .unwrap()
        .parse()
        .unwrap();

    assert!(
        first_index > 1_000,
        "window moved with scroll, got {}",
        first_index
    );
    assert!(tc.query_all("[role=listitem]").length() < 50);
}

#[wasm_bindgen_test]
async fn virtual_scroll_requests_more_at_the_end() {
    let tc = TestContainer::new();
    let requested = Mutable::new(0usize);

    dominator::append_dom(
        &tc.dom_element(),
        virtual_scroll!({
            .item_count(100)
            .item_height(30.0)
            .height(300.0)
            .render_item(Box::new(|index: usize| {
                html!("div", {
                    .text(&format!("row {}", index))
                })
            }))
            .on_reach_end(clone!(requested => move || {
                requested.set(requested.get() + 1);
            }))
        }),
    );
    wait_frames(2).await;

    let container: web_sys::Element = tc.query("[role=list]").unwrap();
    container.set_scroll_top(100 * 30);
    wait_frames(3).await;

    assert_eq!(requested.get(), 1, "on_reach_end fires once per item count");

    // scrolling again without new items must not re-fire
    container.set_scroll_top(99 * 30);
    wait_frames(3).await;
    assert_eq!(requested.get(), 1);
}

#[wasm_bindgen_test]
async fn date_picker_opens_a_grid_and_selects_a_day() {
    let tc = TestContainer::new();
    let value = Mutable::new(Some(CalendarDate::new(2026, 6, 10)));

    dominator::append_dom(
        &tc.dom_element(),
        date_picker!({
            .label("Start date".to_string())
            .value_signal(value.signal())
            .on_change(clone!(value => move |date| {
                value.set(Some(date));
            }))
        }),
    );
    wait_frames(2).await;

    // closed by default
    assert!(tc.query("[role=dialog]").is_none());

    let field = tc.query("button[aria-haspopup=dialog]").unwrap();
    assert_eq!(field.text_content().unwrap(), "2026-06-10");

    click(&field);
    wait_frames(2).await;

    let dialog = tc.query("[role=dialog]").unwrap();
    assert!(dialog.query_selector("[role=grid]").unwrap().is_some());
    assert_eq!(tc.query_all("[role=row]").length(), 6);

    // the current value is marked selected
    let selected = tc
        .query("[role=gridcell] button[aria-selected=true]")
        .unwrap();
    assert_eq!(
        selected.get_attribute("aria-label").as_deref(),
        Some("2026-06-10")
    );

    // pick another day
    let target = tc.query("button[aria-label='2026-06-15']").unwrap();
    click(&target);
    wait_frames(2).await;

    assert_eq!(value.get(), Some(CalendarDate::new(2026, 6, 15)));
    assert!(
        tc.query("[role=dialog]").is_none(),
        "picker closes after selection"
    );
    assert_eq!(field.text_content().unwrap(), "2026-06-15");
}

#[wasm_bindgen_test]
async fn date_picker_navigates_months() {
    let tc = TestContainer::new();
    let value = Mutable::new(Some(CalendarDate::new(2026, 1, 15)));

    dominator::append_dom(
        &tc.dom_element(),
        date_picker!({
            .label("Date".to_string())
            .value_signal(value.signal())
        }),
    );
    wait_frames(2).await;

    click(&tc.query("button[aria-haspopup=dialog]").unwrap());
    wait_frames(2).await;

    let month_label = tc.query("[aria-live=polite]").unwrap();
    assert_eq!(month_label.text_content().unwrap(), "January 2026");

    // previous month wraps the year
    click(&tc.query("button[aria-label='Previous month']").unwrap());
    wait_frames(2).await;
    assert_eq!(month_label.text_content().unwrap(), "December 2025");

    click(&tc.query("button[aria-label='Next month']").unwrap());
    wait_frames(2).await;
    assert_eq!(month_label.text_content().unwrap(), "January 2026");
}

// ---------------------------------------------------------------------------
// Focus standard
// ---------------------------------------------------------------------------

/// Asserts the element carries the `dwui-focusable` marker class, i.e. it
/// participates in the outline-based `:focus-visible` ring standard.
fn assert_focusable(element: &web_sys::Element) {
    let class = element.get_attribute("class").unwrap_or_default();

    assert!(
        class
            .split_whitespace()
            .any(|c| c.starts_with("dwui_focusable_")),
        "expected a dwui-focusable marker class, got: {class:?}"
    );
}

/// Every style-rule selector across all document stylesheets.
fn stylesheet_selectors() -> Vec<String> {
    let doc = web_sys::window().unwrap().document().unwrap();
    let sheets = doc.style_sheets();
    let mut out = vec![];

    for i in 0..sheets.length() {
        let Some(sheet) = sheets.item(i) else {
            continue;
        };
        let Ok(sheet) = sheet.dyn_into::<web_sys::CssStyleSheet>() else {
            continue;
        };
        let Ok(rules) = sheet.css_rules() else {
            continue;
        };

        for r in 0..rules.length() {
            let Some(rule) = rules.item(r) else { continue };

            if let Ok(style_rule) = rule.dyn_into::<web_sys::CssStyleRule>() {
                out.push(style_rule.selector_text());
            }
        }
    }

    out
}

#[wasm_bindgen_test]
async fn focus_standard_replaces_the_global_reset() {
    dwui::theme::apply_style_sheet(None);

    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        button!({ .content(Some(text("Focus me"))) }),
    );
    wait_frame().await;

    assert_focusable(&tc.query("button").unwrap());

    let selectors = stylesheet_selectors();

    // The old blanket reset removed every native focus indicator; it must be gone.
    assert!(
        !selectors.iter().any(|s| s == "*:focus"),
        "the global *:focus reset is still injected"
    );

    // The marker class carries the outline-based focus-visible ring.
    assert!(
        selectors
            .iter()
            .any(|s| s.contains("dwui_focusable") && s.contains(":focus-visible")),
        "no :focus-visible rule found for dwui-focusable"
    );
}

#[wasm_bindgen_test]
async fn interactive_controls_carry_the_focusable_marker() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .child(checkbox!({ .label("Check".to_string()) }))
            .child(switch!({ .label("Toggle".to_string()) }))
        }),
    );
    wait_frame().await;

    assert_focusable(&tc.query("[role=checkbox]").unwrap());
    assert_focusable(&tc.query("[role=switch]").unwrap());
}

// ---------------------------------------------------------------------------
// Field system
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn field_reserves_error_space_and_keeps_height() {
    let tc = TestContainer::new();
    let validity = Mutable::new(ValidationResult::Valid);

    dominator::append_dom(
        &tc.dom_element(),
        text_input!({
            .label("Email".to_string())
            .is_valid_signal(validity.signal_cloned())
        }),
    );
    wait_frames(2).await;

    let input = tc.query("input").unwrap();
    let input_id = input.get_attribute("id").unwrap();

    // The message row exists (and reserves space) before any error appears.
    let error_row = tc.query(&format!("[id='{}-error']", input_id)).unwrap();
    assert_eq!(error_row.text_content().unwrap(), "");

    let surface_height = input.parent_element().unwrap().client_height();

    validity.set(ValidationResult::Invalid {
        message: "Nope".to_string(),
    });
    wait_frames(2).await;

    // Turning invalid must not change the field surface height.
    assert_eq!(
        input.parent_element().unwrap().client_height(),
        surface_height,
        "field surface changed height when it became invalid"
    );
    assert_eq!(error_row.text_content().unwrap(), "Nope");
}

#[wasm_bindgen_test]
async fn text_input_and_select_support_disabled() {
    let tc = TestContainer::new();
    let disabled = Mutable::new(false);

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .child(text_input!({
                .label("Name".to_string())
                .disabled_signal(disabled.signal())
            }))
            .child(select!({
                .label("Fruit".to_string())
                .disabled_signal(disabled.signal())
                .options(vec![("a".to_string(), "Apple".to_string())])
            }))
            .child(slider!({
                .label("Volume".to_string())
                .disabled_signal(disabled.signal())
            }))
        }),
    );
    wait_frames(2).await;

    assert!(!tc.query("input").unwrap().has_attribute("disabled"));
    assert!(!tc.query("select").unwrap().has_attribute("disabled"));

    disabled.set(true);
    wait_frame().await;

    assert!(tc.query("input").unwrap().has_attribute("disabled"));
    assert!(tc.query("select").unwrap().has_attribute("disabled"));
    assert!(tc
        .query("input[type=range]")
        .unwrap()
        .has_attribute("disabled"));
}

// ---------------------------------------------------------------------------
// text_area!
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn text_area_syncs_value_and_reports_invalid() {
    let tc = TestContainer::new();
    let value = Mutable::new("hello".to_string());
    let validity = Mutable::new(ValidationResult::Valid);

    dominator::append_dom(
        &tc.dom_element(),
        text_area!({
            .value(value.clone())
            .label("Bio".to_string())
            .rows(6u32)
            .is_valid_signal(validity.signal_cloned())
        }),
    );
    wait_frames(2).await;

    let textarea: web_sys::HtmlTextAreaElement = tc.query("textarea").unwrap().dyn_into().unwrap();

    assert_eq!(textarea.value(), "hello");
    assert_eq!(textarea.get_attribute("rows").as_deref(), Some("6"));

    let label = tc.query("label").unwrap();
    assert_eq!(
        label.get_attribute("for").unwrap(),
        textarea.get_attribute("id").unwrap()
    );

    value.set("world".to_string());
    wait_frames(2).await;
    assert_eq!(textarea.value(), "world");

    validity.set(ValidationResult::Invalid {
        message: "Too short".to_string(),
    });
    wait_frames(2).await;

    assert_eq!(
        textarea.get_attribute("aria-invalid").as_deref(),
        Some("true")
    );
    let alert = tc.query("[role=alert]").unwrap();
    assert_eq!(alert.text_content().unwrap(), "Too short");
}

// ---------------------------------------------------------------------------
// number_input!
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn number_input_steps_and_clamps() {
    let tc = TestContainer::new();
    let value = Mutable::new(5.0f64);

    dominator::append_dom(
        &tc.dom_element(),
        number_input!({
            .value(value.clone())
            .label("Amount".to_string())
            .min(Some(0.0))
            .max(Some(6.0))
            .step(2.0)
        }),
    );
    wait_frames(2).await;

    let input = tc.query("input[type=number]").unwrap();
    assert_eq!(input.get_attribute("min").as_deref(), Some("0"));
    assert_eq!(input.get_attribute("max").as_deref(), Some("6"));
    assert_eq!(input.get_attribute("step").as_deref(), Some("2"));
    assert_eq!(input.get_attribute("inputmode").as_deref(), Some("decimal"));

    // The steppers are decorative: not tabbable, hidden from AT.
    let steppers = tc.query_all("button[tabindex='-1'][aria-hidden=true]");
    assert_eq!(steppers.length(), 2);

    let up: web_sys::Element = steppers.item(0).unwrap().dyn_into().unwrap();
    let down: web_sys::Element = steppers.item(1).unwrap().dyn_into().unwrap();

    click(&up);
    wait_frame().await;
    // 5 + 2 clamps to max 6
    assert_eq!(value.get(), 6.0);

    click(&down);
    wait_frame().await;
    assert_eq!(value.get(), 4.0);
}

// ---------------------------------------------------------------------------
// radio_group!
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn radio_group_selection_and_roving_tabindex() {
    let tc = TestContainer::new();
    let value = Mutable::new("b".to_string());

    dominator::append_dom(
        &tc.dom_element(),
        radio_group!({
            .label("Flavor".to_string())
            .value_signal(value.signal_cloned())
            .options(vec![
                ("a".to_string(), "Almond".to_string()),
                ("b".to_string(), "Butterscotch".to_string()),
                ("c".to_string(), "Cinnamon".to_string()),
            ])
            .on_change(clone!(value => move |key| {
                value.set(key);
            }))
        }),
    );
    wait_frames(2).await;

    let group = tc.query("[role=radiogroup]").unwrap();
    let label_id = group.get_attribute("aria-labelledby").unwrap();
    assert_eq!(
        tc.query(&format!("[id='{}']", label_id))
            .unwrap()
            .text_content()
            .unwrap(),
        "Flavor"
    );

    let radios = tc.query_all("[role=radio]");
    assert_eq!(radios.length(), 3);

    let radio = |i: u32| -> web_sys::Element { radios.item(i).unwrap().dyn_into().unwrap() };

    // Roving tabindex: only the checked option is a tab stop
    assert_eq!(radio(0).get_attribute("tabindex").as_deref(), Some("-1"));
    assert_eq!(radio(1).get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(
        radio(1).get_attribute("aria-checked").as_deref(),
        Some("true")
    );

    click(&radio(2));
    wait_frames(2).await;

    assert_eq!(value.get_cloned(), "c");
    assert_eq!(
        radio(2).get_attribute("aria-checked").as_deref(),
        Some("true")
    );
    assert_eq!(radio(2).get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(radio(1).get_attribute("tabindex").as_deref(), Some("-1"));

    // ArrowDown moves selection (selection follows focus)
    let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &{
        let dict = web_sys::KeyboardEventInit::new();
        dict.set_key("ArrowDown");
        dict.set_bubbles(true);
        dict.set_cancelable(true);
        dict
    })
    .unwrap();
    radio(2).dispatch_event(&event).unwrap();
    wait_frames(2).await;

    assert_eq!(
        value.get_cloned(),
        "a",
        "ArrowDown should wrap to the first option"
    );
}

// ---------------------------------------------------------------------------
// Control chrome
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn slider_chrome_tracks_fill_percentage() {
    dwui::theme::apply_style_sheet(None);

    let tc = TestContainer::new();
    let value = Mutable::new(25.0f32);

    dominator::append_dom(
        &tc.dom_element(),
        slider!({
            .value(value.clone())
            .label("Volume".to_string())
            .min(0.0f32)
            .max(100.0f32)
        }),
    );
    wait_frames(2).await;

    let range = tc.query("input[type=range]").unwrap();
    assert!(range.class_list().contains("dwui-slider"));

    let fill = |el: &web_sys::Element| {
        el.dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .style()
            .get_property_value("--dwui-slider-fill")
            .unwrap()
    };

    assert_eq!(fill(&range).trim(), "25%");

    value.set(50.0);
    wait_frames(2).await;
    assert_eq!(fill(&range).trim(), "50%");
}

#[wasm_bindgen_test]
async fn checkbox_and_switch_use_the_on_accent_token() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        html!("div", {
            .child(checkbox!({ .checked(true) .label("Check".to_string()) }))
            .child(switch!({ .checked(true) .label("Toggle".to_string()) }))
        }),
    );
    wait_frames(2).await;

    // Checkmark stroke is themed, not hard-coded white
    let check_path = tc.query("[role=checkbox] path").unwrap();
    assert!(check_path
        .get_attribute("style")
        .unwrap()
        .contains("--dwui-on-accent"));
    assert!(!check_path.has_attribute("stroke"));

    // The switch knob is themed and moves by transform
    let knob = tc.query("[role=switch] span").unwrap();
    assert!(!knob.class_list().contains("bg-white"));

    let transform = knob
        .dyn_ref::<web_sys::HtmlElement>()
        .unwrap()
        .style()
        .get_property_value("transform")
        .unwrap();
    assert!(transform.contains("translateX"), "got {transform:?}");
}

// ---------------------------------------------------------------------------
// Surfaces
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn heading_renders_without_a_wrapper() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        heading!({
            .content(text("Bare"))
            .level(HeadingLevel::H3)
        }),
    );
    wait_frame().await;

    // The heading is the component's root element, not nested in a div.
    let h3 = tc.query("h3").unwrap();
    assert_eq!(
        h3.parent_element()
            .unwrap()
            .get_attribute("style")
            .as_deref()
            .map(|s| s.contains("width:800px")),
        Some(true),
        "expected the heading to be a direct child of the test container"
    );
}

#[wasm_bindgen_test]
async fn card_defaults_to_padded_content() {
    dwui::theme::apply_style_sheet(None);

    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        card!({
            .content(text("padded"))
        }),
    );
    wait_frames(2).await;

    let card = tc.dom_element().first_element_child().unwrap();
    let style = web_sys::window()
        .unwrap()
        .get_computed_style(&card)
        .unwrap()
        .unwrap();

    assert_eq!(style.get_property_value("padding-left").unwrap(), "16px");
}

#[wasm_bindgen_test]
async fn modal_traps_tab_focus() {
    use discard::Discard;

    let tc = TestContainer::new();

    let handle = dominator::append_dom(
        &tc.dom_element(),
        modal!({
            .open(true)
            .aria_label("Trap test".to_string())
            .content(Some(html!("div", {
                .child(html!("button", { .attr("id", "trap-inner") .text("Inner") }))
            })))
        }),
    );
    wait_frames(2).await;

    let dialog = doc_query("[role=dialog]").unwrap();
    let inner: web_sys::HtmlElement = doc_query("[id=trap-inner]").unwrap().dyn_into().unwrap();

    // Focus the last focusable element (the inner button), then Tab: the trap
    // must wrap focus around to the dialog's first focusable (the close
    // button) instead of leaving the dialog.
    inner.focus().unwrap();

    let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &{
        let dict = web_sys::KeyboardEventInit::new();
        dict.set_key("Tab");
        dict.set_bubbles(true);
        dict.set_cancelable(true);
        dict
    })
    .unwrap();
    dialog.dispatch_event(&event).unwrap();
    wait_frame().await;

    let active = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .active_element()
        .unwrap();

    assert_eq!(
        active.get_attribute("aria-label").as_deref(),
        Some("Close dialog"),
        "Tab from the last focusable should wrap to the close button"
    );

    handle.discard();
}

#[wasm_bindgen_test]
async fn alert_dismiss_button_is_an_icon_with_a_label() {
    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        alert!({
            .title("Closable".to_string())
            .dismissible(true)
        }),
    );
    wait_frame().await;

    let dismiss = tc.query("button[aria-label=Dismiss]").unwrap();
    assert!(dismiss.query_selector("svg").unwrap().is_some());
    assert!(!dismiss.text_content().unwrap().contains('×'));
}

// ---------------------------------------------------------------------------
// pagination!
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn pagination_renders_window_and_aria_current() {
    let tc = TestContainer::new();
    let page = Mutable::new(5usize);

    dominator::append_dom(
        &tc.dom_element(),
        pagination!({
            .page_signal(page.signal())
            .total_pages(10usize)
            .on_page_change(clone!(page => move |p| {
                page.set(p);
            }))
        }),
    );
    wait_frames(2).await;

    // 1 … 4 5 6 … 10 plus prev/next arrows
    assert_eq!(tc.query_all("button").length(), 7);
    assert_eq!(tc.query_all("span[aria-hidden=true]").length(), 2);

    let current = tc.query("[aria-current=page]").unwrap();
    assert_eq!(current.text_content().unwrap(), "5");

    click(&tc.query("button[aria-label='Next page']").unwrap());
    wait_frames(2).await;
    assert_eq!(page.get(), 6);

    click(&tc.query("button[aria-label='Page 10']").unwrap());
    wait_frames(2).await;
    assert_eq!(page.get(), 10);

    // At the last page the next arrow is disabled
    assert!(tc
        .query("button[aria-label='Next page']")
        .unwrap()
        .has_attribute("disabled"));
}

// ---------------------------------------------------------------------------
// dropdown_menu! / popover!
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn dropdown_menu_opens_selects_and_closes() {
    let tc = TestContainer::new();
    let selected: Mutable<Option<String>> = Mutable::new(None);

    dominator::append_dom(
        &tc.dom_element(),
        dropdown_menu!({
            .label("Actions".to_string())
            .items(vec![
                ("copy".to_string(), "Copy".to_string(), false),
                ("paste".to_string(), "Paste".to_string(), true),
                ("delete".to_string(), "Delete".to_string(), false),
            ])
            .on_select(clone!(selected => move |key| {
                selected.set(Some(key));
            }))
        }),
    );
    wait_frames(2).await;

    let trigger = tc.query("button[aria-haspopup=menu]").unwrap();
    assert_eq!(
        trigger.get_attribute("aria-expanded").as_deref(),
        Some("false")
    );

    let menu = tc.query("[role=menu]").unwrap();

    click(&trigger);
    wait_frames(2).await;

    assert_eq!(
        trigger.get_attribute("aria-expanded").as_deref(),
        Some("true")
    );
    assert_eq!(tc.query_all("[role=menuitem]").length(), 3);

    let disabled_item = tc.query("[role=menuitem][aria-disabled=true]").unwrap();
    assert_eq!(disabled_item.text_content().unwrap(), "Paste");

    click(
        &tc.query(&format!(
            "[id='{}-item-delete']",
            menu.get_attribute("id").unwrap()
        ))
        .unwrap(),
    );
    wait_frames(2).await;

    assert_eq!(selected.get_cloned().as_deref(), Some("delete"));
    assert_eq!(
        trigger.get_attribute("aria-expanded").as_deref(),
        Some("false")
    );
}

#[wasm_bindgen_test]
async fn popover_opens_and_closes_on_escape() {
    let tc = TestContainer::new();
    let open = Mutable::new(false);

    dominator::append_dom(
        &tc.dom_element(),
        popover!({
            .open_signal(open.signal())
            .anchor(Some(html!("button", { .attr("id", "pop-trigger") .text("Open") })))
            .content(Some(text("Popover body")))
            .on_close(clone!(open => move || {
                open.set(false);
            }))
        }),
    );
    wait_frames(2).await;

    open.set(true);
    wait_frames(2).await;

    let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &{
        let dict = web_sys::KeyboardEventInit::new();
        dict.set_key("Escape");
        dict.set_bubbles(true);
        dict
    })
    .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .dispatch_event(&event)
        .unwrap();
    wait_frames(2).await;

    assert!(!open.get(), "Escape should have closed the popover");
}

// ---------------------------------------------------------------------------
// drawer!
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn drawer_opens_from_a_side_and_closes() {
    use discard::Discard;

    let tc = TestContainer::new();
    let open = Mutable::new(true);

    let handle = dominator::append_dom(
        &tc.dom_element(),
        drawer!({
            .open_signal(open.signal())
            .side(DrawerSide::Left)
            .aria_label("Settings".to_string())
            .content(Some(text("Drawer body")))
            .on_close(clone!(open => move || {
                open.set(false);
            }))
        }),
    );
    wait_frames(2).await;

    let dialog = doc_query("[role=dialog]").unwrap();
    assert_eq!(
        dialog.get_attribute("aria-label").as_deref(),
        Some("Settings")
    );
    assert_eq!(dialog.get_attribute("aria-modal").as_deref(), Some("true"));

    let style = dialog.get_attribute("style").unwrap_or_default();
    assert!(
        style.contains("left: 0"),
        "left drawer should pin to the left edge, got {style:?}"
    );

    click(&doc_query("button[aria-label='Close drawer']").unwrap());
    wait_frames(2).await;

    assert!(!open.get());

    handle.discard();
}

// ---------------------------------------------------------------------------
// toasts! / Toaster
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn toaster_pushes_auto_dismisses_and_manually_dismisses() {
    use discard::Discard;

    let tc = TestContainer::new();
    let toaster = Toaster::default();

    let handle = dominator::append_dom(
        &tc.dom_element(),
        toasts!({
            .toaster(toaster.clone())
        }),
    );
    wait_frame().await;

    let host = doc_query("[aria-live=polite]").unwrap();
    assert_eq!(host.child_element_count(), 0);

    // Auto-dismissing toast
    toaster.push(ToastOptions {
        title: "Saved".to_string(),
        variant: ToastVariant::Success,
        duration_ms: Some(80),
        ..Default::default()
    });

    // Sticky error toast
    let sticky = toaster.push(ToastOptions {
        title: "Broken".to_string(),
        variant: ToastVariant::Error,
        duration_ms: None,
        ..Default::default()
    });

    wait_frame().await;
    assert_eq!(host.child_element_count(), 2);
    assert!(
        doc_query("[role=alert]").is_some(),
        "error toasts announce assertively"
    );

    gloo_timers::future::TimeoutFuture::new(300).await;
    assert_eq!(
        host.child_element_count(),
        1,
        "the timed toast should have auto-dismissed"
    );

    // Dismiss the sticky one via its button
    click(
        &host
            .query_selector("button[aria-label=Dismiss]")
            .unwrap()
            .unwrap(),
    );
    wait_frame().await;
    assert_eq!(host.child_element_count(), 0);

    // And the handle API can clear directly
    toaster.push(ToastOptions::default());
    wait_frame().await;
    assert_eq!(host.child_element_count(), 1);
    toaster.dismiss(sticky + 1);
    toaster.clear();
    wait_frame().await;
    assert_eq!(host.child_element_count(), 0);

    handle.discard();
}

#[wasm_bindgen_test]
async fn select_popup_is_theme_colored() {
    dwui::theme::apply_style_sheet(None);

    let tc = TestContainer::new();

    dominator::append_dom(
        &tc.dom_element(),
        select!({
            .label("Fruit".to_string())
            .options(vec![("a".to_string(), "Apple".to_string())])
        }),
    );
    wait_frames(2).await;

    let select = tc.query("select").unwrap();
    assert!(select.class_list().contains("dwui-select"));

    // The popup follows the element's color-scheme, and engines that paint
    // options themselves honor the explicit option colors.
    let style = web_sys::window()
        .unwrap()
        .get_computed_style(&select)
        .unwrap()
        .unwrap();
    assert_eq!(style.get_property_value("color-scheme").unwrap(), "dark");

    let option = tc.query("option").unwrap();
    let option_style = web_sys::window()
        .unwrap()
        .get_computed_style(&option)
        .unwrap()
        .unwrap();
    assert!(
        !option_style
            .get_property_value("background-color")
            .unwrap()
            .is_empty(),
        "option should have an explicit background"
    );
}
