use crate::theme::layers;
use crate::theme::prelude::*;
use dominator::{clone, events, html, svg, with_node, Dom, EventOptions};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast;

/// An accessible dropdown menu following the WAI-ARIA menu-button pattern.
///
/// Renders a trigger button (`aria-haspopup="menu"`, `aria-expanded`) that
/// opens a `role="menu"` panel of `(key, label, disabled)` items. Keyboard
/// support: Enter/Space/ArrowDown open the menu and focus its first enabled
/// item; inside, ArrowUp/ArrowDown move focus (wrapping, skipping disabled
/// items), Home/End jump, Escape closes and refocuses the trigger.
/// Activating an item invokes `on_select` with its key and closes the menu.
#[component(render_fn = dropdown_menu)]
struct DropdownMenu {
    #[signal_vec]
    #[default(vec![])]
    items: (String, String, bool),

    /// Trigger button text
    #[signal]
    #[default("Menu".to_string())]
    #[into]
    label: String,

    #[default(Box::new(|_|{}))]
    on_select: dyn Fn(String) + 'static,

    #[signal]
    #[default(false)]
    disabled: bool,
}

pub fn dropdown_menu(props: DropdownMenuProps) -> Dom {
    let DropdownMenuProps {
        items,
        label,
        on_select,
        disabled,
        apply,
    } = props;

    let on_select = Rc::new(on_select);
    let disabled = disabled.broadcast();
    let items = items.to_signal_cloned().broadcast();

    let open = Mutable::new(false);

    // focus_first needs the current entries, not a signal
    let items_state: Mutable<Vec<(String, String, bool)>> = Mutable::new(vec![]);

    let menu_id = component_id_pair();
    let (trigger_id, panel_id) = menu_id;

    let focus_element = |id: &str| {
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(id))
        {
            if let Ok(element) = element.dyn_into::<web_sys::HtmlElement>() {
                let _ = element.focus();
            }
        }
    };

    let item_dom_id = {
        let panel_id = panel_id.clone();
        move |key: &str| format!("{}-item-{}", panel_id, key)
    };

    // Focus the first enabled item of the currently rendered menu
    let focus_first = Rc::new({
        let items_state = items_state.clone();
        let item_dom_id = item_dom_id.clone();

        move || {
            let entries = items_state.get_cloned();

            if let Some((key, _, _)) = entries.iter().find(|(_, _, disabled)| !disabled) {
                focus_element(&item_dom_id(key));
            }
        }
    });

    html!("span", {
        .dwclass!("inline-block")
        .style("position", "relative")
        .future(items.signal_cloned().for_each(clone!(items_state => move |v| {
            items_state.set(v);
            async {}
        })))
        .global_event(clone!(open, trigger_id => move |e: events::KeyDown| {
            if open.get() && e.key() == "Escape" {
                open.set(false);
                focus_element(&trigger_id);
            }
        }))
        .with_node!(wrapper => {
            .global_event(clone!(open => move |e: events::Click| {
                if !open.get() {
                    return;
                }

                let outside = e
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                    .map(|node| !wrapper.contains(Some(&node)))
                    .unwrap_or(true);

                if outside {
                    open.set(false);
                }
            }))
        })
        // Trigger
        .child(html!("button", {
            .attr("type", "button")
            .attr("id", &trigger_id)
            .attr("aria-haspopup", "menu")
            .attr("aria-controls", &panel_id)
            .attr_signal("aria-expanded", open.signal().map(|v| if v { "true" } else { "false" }))
            .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
            .dwclass!("h-10 p-l-3 p-r-3 rounded-md cursor-pointer font-medium text-base transition-colors dwui-focusable")
            .dwclass!("flex flex-row align-items-center gap-2")
            .dwclass!("border dwui-border-void-600 bg-transparent")
            .dwclass!("dwui-text-on-primary-200 hover:dwui-bg-void-800")
            .dwclass!("is(.light *):dwui-border-void-300 is(.light *):dwui-text-on-primary-800 is(.light *):hover:dwui-bg-void-200")
            .dwclass!("disabled:opacity-60 disabled:cursor-not-allowed")
            .child(html!("span", { .text_signal(label) }))
            .child(svg!("svg", {
                .attr("viewBox", "0 0 12 12")
                .attr("width", "12")
                .attr("height", "12")
                .attr("fill", "none")
                .attr("aria-hidden", "true")
                .child(svg!("path", {
                    .attr("d", "M2 4 L6 8 L10 4")
                    .attr("stroke", "currentColor")
                    .attr("stroke-width", "1.5")
                    .attr("stroke-linecap", "round")
                    .attr("stroke-linejoin", "round")
                }))
            }))
            .event(clone!(open => move |_: events::Click| {
                open.set(!open.get());
            }))
            .event_with_options(&EventOptions::preventable(), clone!(open, focus_first => move |e: events::KeyDown| {
                if e.key() != "ArrowDown" {
                    return;
                }

                e.prevent_default();
                open.set(true);

                // Let the panel become visible, then move focus into it
                let focus_first = focus_first.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(0).await;
                    (focus_first)();
                });
            }))
        }))
        // Menu panel
        .child(html!("div", {
            .attr("id", &panel_id)
            .attr("role", "menu")
            .attr("aria-labelledby", &trigger_id)
            .visible_signal(open.signal())
            .dwclass!("rounded-lg border shadow-lg p-1 flex flex-col")
            .dwclass!("dwui-bg-void-800 dwui-border-void-700")
            .dwclass!("is(.light *):dwui-bg-void-50 is(.light *):dwui-border-void-200")
            .style("position", "absolute")
            .style("top", "calc(100% + 0.375rem)")
            .style("left", "0")
            .style("z-index", layers::OVERLAY)
            .style("min-width", "100%")
            .style("width", "max-content")
            .style("animation", "dwui-modal-in 150ms ease-out")
            .children_signal_vec(items.signal_cloned().map(clone!(open, on_select, item_dom_id, trigger_id => move |entries| {
                entries
                    .iter()
                    .enumerate()
                    .map(|(index, (key, item_label, item_disabled))| {
                        let key = key.clone();
                        let item_disabled = *item_disabled;

                        html!("button", {
                            .attr("type", "button")
                            .attr("role", "menuitem")
                            .attr("id", &item_dom_id(&key))
                            .attr("tabindex", "-1")
                            .apply_if(item_disabled, |b| {
                                b.attr("disabled", "disabled")
                                    .attr("aria-disabled", "true")
                            })
                            .dwclass!("h-8 p-l-3 p-r-3 rounded-md cursor-pointer text-sm text-left transition-colors dwui-focusable")
                            .dwclass!("bg-transparent border-none w-full")
                            .dwclass!("dwui-text-on-primary-200 hover:dwui-bg-void-700")
                            .dwclass!("is(.light *):dwui-text-on-primary-800 is(.light *):hover:dwui-bg-void-200")
                            .dwclass!("disabled:opacity-60 disabled:cursor-not-allowed")
                            .text(item_label)
                            .event(clone!(open, on_select, key, trigger_id => move |e: events::Click| {
                                e.stop_propagation();

                                if item_disabled {
                                    return;
                                }

                                (on_select)(key.clone());
                                open.set(false);
                                focus_element(&trigger_id);
                            }))
                            .event_with_options(&EventOptions::preventable(), clone!(entries, item_dom_id => move |e: events::KeyDown| {
                                let count = entries.len();

                                if count == 0 {
                                    return;
                                }

                                let next_enabled = |mut index: usize, step: i32| -> Option<usize> {
                                    for _ in 0..count {
                                        index = (index as i32 + step).rem_euclid(count as i32) as usize;

                                        if !entries[index].2 {
                                            return Some(index);
                                        }
                                    }

                                    None
                                };

                                let target = match e.key().as_str() {
                                    "ArrowDown" => next_enabled(index, 1),
                                    "ArrowUp" => next_enabled(index, -1),
                                    "Home" => entries.iter().position(|(_, _, d)| !d),
                                    "End" => entries.iter().rposition(|(_, _, d)| !d),
                                    _ => return,
                                };

                                e.prevent_default();

                                if let Some(target) = target {
                                    focus_element(&item_dom_id(&entries[target].0));
                                }
                            }))
                        })
                    })
                    .collect::<Vec<_>>()
            })).to_signal_vec())
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}

fn component_id_pair() -> (String, String) {
    let base = crate::utils::component_id("menu");

    (format!("{}-trigger", base), base)
}
