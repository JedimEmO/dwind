use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, Dom, EventOptions};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast;

/// An accessible tab list following the WAI-ARIA tabs pattern.
///
/// Renders a `role="tablist"` with one `role="tab"` button per `(key, label)`
/// entry. The selected tab is controlled by the `selected` key signal;
/// activating a tab (click, Enter, or Space) invokes `on_select` with its key.
/// Keyboard support: Left/Right arrows move selection and focus to the
/// previous/next tab (wrapping), Home/End jump to the first/last tab.
///
/// Each tab gets the DOM id `{tablist-id}-tab-{key}`, so tab panels can point
/// back at their tab via `aria-labelledby`.
#[component(render_fn = tab_list)]
struct TabList {
    #[signal_vec]
    #[default(vec![])]
    tabs: (String, String),

    #[signal]
    #[default("".to_string())]
    selected: String,

    #[default(Box::new(|_|{}))]
    on_select: dyn Fn(String) + 'static,

    /// Accessible name for the tab list
    #[signal]
    #[default("Tabs".to_string())]
    aria_label: String,
}

pub fn tab_list(props: TabListProps) -> Dom {
    let TabListProps {
        tabs,
        selected,
        on_select,
        aria_label,
        apply,
    } = props;

    let on_select = Rc::new(on_select);
    let selected = selected.broadcast();
    let tabs = tabs.to_signal_cloned().broadcast();

    let tablist_id = component_id("tablist");

    let tab_dom_id = {
        let tablist_id = tablist_id.clone();
        move |key: &str| format!("{}-tab-{}", tablist_id, key)
    };

    html!("div", {
        .attr("role", "tablist")
        .attr("id", &tablist_id)
        .attr_signal("aria-label", aria_label)
        .dwclass!("flex flex-row gap-1 border-b")
        .dwclass!("dwui-border-void-700 is(.light *):dwui-border-void-300")
        .children_signal_vec(tabs.signal_cloned().map(clone!(selected, tab_dom_id => move |tabs_vec| {
            tabs_vec
                .iter()
                .enumerate()
                .map(|(index, (key, label))| {
                    let key = key.clone();
                    let is_selected = selected
                        .signal_cloned()
                        .map(clone!(key => move |s| s == key))
                        .broadcast();

                    html!("button", {
                        .attr("type", "button")
                        .attr("role", "tab")
                        .attr("id", &tab_dom_id(&key))
                        .attr_signal("aria-selected", is_selected.signal().map(|v| if v { "true" } else { "false" }))
                        .attr_signal("tabindex", is_selected.signal().map(|v| if v { "0" } else { "-1" }))
                        .dwclass!("h-10 p-l-4 p-r-4 cursor-pointer font-medium text-base transition-all bg-transparent border-none")
                        .dwclass!("rounded-t-sm")
                        .dwclass!("focus-visible:ring-2 focus-visible:dwui-ring-primary-400 is(.light *):focus-visible:dwui-ring-primary-600")
                        .dwclass_signal!("dwui-text-on-primary-50 is(.light *):dwui-text-on-primary-950", is_selected.signal())
                        .dwclass_signal!(
                            "dwui-text-on-primary-400 hover:dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-600 is(.light *):hover:dwui-text-on-primary-800",
                            is_selected.signal().map(|v| !v)
                        )
                        .style("outline", "none")
                        .style_signal("box-shadow", is_selected.signal().map(|v| {
                            if v {
                                Some("inset 0 -2px 0 0 var(--dwui-primary-400)")
                            } else {
                                None
                            }
                        }))
                        .text(label)
                        .event(clone!(on_select, key => move |_: events::Click| {
                            (on_select)(key.clone());
                        }))
                        .event_with_options(&EventOptions::preventable(), clone!(on_select, tabs_vec, tab_dom_id => move |e: events::KeyDown| {
                            let count = tabs_vec.len();

                            if count == 0 {
                                return;
                            }

                            let target_index = match e.key().as_str() {
                                "ArrowRight" => (index + 1) % count,
                                "ArrowLeft" => (index + count - 1) % count,
                                "Home" => 0,
                                "End" => count - 1,
                                _ => return,
                            };

                            e.prevent_default();

                            let target_key = tabs_vec[target_index].0.clone();

                            // Move focus along with selection (roving tabindex)
                            if let Some(element) = web_sys::window()
                                .and_then(|w| w.document())
                                .and_then(|d| d.get_element_by_id(&tab_dom_id(&target_key)))
                            {
                                if let Ok(element) = element.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = element.focus();
                                }
                            }

                            (on_select)(target_key);
                        }))
                    })
                })
                .collect::<Vec<_>>()
        })).to_signal_vec())
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
