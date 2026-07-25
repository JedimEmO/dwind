use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, Dom, EventOptions};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RadioGroupDirection {
    Horizontal,
    Vertical,
}

/// An accessible radio group following the WAI-ARIA radio pattern.
///
/// Renders a `role="radiogroup"` with one `role="radio"` button per
/// `(key, label)` entry. Selection is controlled by the `value` key signal;
/// picking an option (click, Space, or arrow keys — selection follows focus)
/// invokes `on_change` with its key. Roving tabindex: the checked option (or
/// the first, when nothing is checked) is the group's single tab stop.
#[component(render_fn = radio_group)]
struct RadioGroup {
    #[signal_vec]
    #[default(vec![])]
    options: (String, String),

    #[signal]
    #[default("".to_string())]
    value: String,

    #[default(Box::new(|_|{}))]
    on_change: dyn Fn(String) + 'static,

    /// Visible group label
    #[signal]
    #[default("".to_string())]
    label: String,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[default(RadioGroupDirection::Vertical)]
    direction: RadioGroupDirection,
}

pub fn radio_group(props: RadioGroupProps) -> Dom {
    let RadioGroupProps {
        options,
        value,
        on_change,
        label,
        disabled,
        direction,
        apply,
    } = props;

    let on_change = Rc::new(on_change);
    let value = value.broadcast();
    let disabled = disabled.broadcast();
    let label = label.broadcast();
    let options = options.to_signal_cloned().broadcast();

    let group_id = component_id("radio-group");
    let label_id = format!("{}-label", group_id);

    let radio_dom_id = {
        let group_id = group_id.clone();
        move |key: &str| format!("{}-radio-{}", group_id, key)
    };

    html!("div", {
        .attr("role", "radiogroup")
        .attr("id", &group_id)
        .attr("aria-labelledby", &label_id)
        .dwclass!("flex flex-col gap-2")
        .child(html!("div", {
            .attr("id", &label_id)
            .dwclass!("text-sm font-medium")
            .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700")
            .visible_signal(label.signal_ref(|v| !v.is_empty()))
            .text_signal(label.signal_cloned())
        }))
        .child(html!("div", {
            .apply(move |b| match direction {
                RadioGroupDirection::Horizontal => dwclass!(b, "flex flex-row flex-wrap gap-4"),
                RadioGroupDirection::Vertical => dwclass!(b, "flex flex-col gap-2"),
            })
            .children_signal_vec(options.signal_cloned().map(clone!(value, disabled, on_change, radio_dom_id => move |options_vec| {
                options_vec
                    .iter()
                    .enumerate()
                    .map(|(index, (key, option_label))| {
                        let key = key.clone();

                        let is_checked = value
                            .signal_cloned()
                            .map(clone!(key => move |v| v == key))
                            .broadcast();

                        // Roving tabindex: the checked option is the tab stop;
                        // with nothing checked, the first option is.
                        let tab_stop = value
                            .signal_cloned()
                            .map(clone!(key, options_vec => move |v| {
                                if options_vec.iter().any(|(k, _)| *k == v) {
                                    v == key
                                } else {
                                    index == 0
                                }
                            }));

                        let radio_id = radio_dom_id(&key);

                        html!("div", {
                            .dwclass!("flex flex-row align-items-center gap-2")
                            .child(html!("button", {
                                .attr("type", "button")
                                .attr("role", "radio")
                                .attr("id", &radio_id)
                                .attr_signal("aria-checked", is_checked.signal().map(|v| if v { "true" } else { "false" }))
                                .attr_signal("tabindex", tab_stop.map(|v| if v { "0" } else { "-1" }))
                                .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
                                .dwclass!("w-5 h-5 rounded-full cursor-pointer transition-colors flex-none p-0")
                                .dwclass!("flex align-items-center justify-center bg-transparent")
                                .dwclass!("border dwui-border-void-500 is(.light *):dwui-border-void-300")
                                .dwclass!("disabled:cursor-not-allowed disabled:opacity-60")
                                .dwclass!("dwui-focusable")
                                .dwclass_signal!("dwui-border-primary-500 is(.light *):dwui-border-primary-400", is_checked.signal())
                                .child(html!("span", {
                                    .dwclass!("w-2 h-2 rounded-full flex-none")
                                    .dwclass!("dwui-bg-primary-500 is(.light *):dwui-bg-primary-400")
                                    .style("transition", "opacity 100ms ease-out")
                                    .style_signal("opacity", is_checked.signal().map(|v| if v { "1" } else { "0" }))
                                }))
                                .event(clone!(on_change, key => move |_: events::Click| {
                                    (on_change)(key.clone());
                                }))
                                .event_with_options(&EventOptions::preventable(), clone!(on_change, options_vec, radio_dom_id => move |e: events::KeyDown| {
                                    let count = options_vec.len();

                                    if count == 0 {
                                        return;
                                    }

                                    let target_index = match e.key().as_str() {
                                        "ArrowRight" | "ArrowDown" => (index + 1) % count,
                                        "ArrowLeft" | "ArrowUp" => (index + count - 1) % count,
                                        "Home" => 0,
                                        "End" => count - 1,
                                        _ => return,
                                    };

                                    e.prevent_default();

                                    let target_key = options_vec[target_index].0.clone();

                                    // Selection follows focus (roving tabindex)
                                    if let Some(element) = web_sys::window()
                                        .and_then(|w| w.document())
                                        .and_then(|d| d.get_element_by_id(&radio_dom_id(&target_key)))
                                    {
                                        if let Ok(element) = element.dyn_into::<web_sys::HtmlElement>() {
                                            let _ = element.focus();
                                        }
                                    }

                                    (on_change)(target_key);
                                }))
                            }))
                            .child(html!("label", {
                                .attr("for", &radio_id)
                                .dwclass!("cursor-pointer select-none")
                                .dwclass!("dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-800")
                                .text(option_label)
                            }))
                        })
                    })
                    .collect::<Vec<_>>()
            })).to_signal_vec())
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
