use crate::input::{GlassSelectOption, GlassSelectValue, ValidationResult};
use crate::prelude::*;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use web_sys::HtmlInputElement;

#[component(render_fn = glass_combobox)]
struct GlassCombobox {
    #[default(Box::new(Mutable::new(None::<String>)))]
    value: dyn GlassSelectValue + 'static,

    #[default(vec![])]
    options: Vec<GlassSelectOption>,

    #[signal]
    #[default("Search...".to_string())]
    placeholder: String,

    #[signal]
    #[default("".to_string())]
    label: String,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default(false)]
    disabled: bool,
}

pub fn glass_combobox(props: GlassComboboxProps) -> Dom {
    let GlassComboboxProps {
        value,
        options,
        placeholder,
        label,
        is_valid,
        disabled,
        apply,
    } = props;

    let disabled = disabled.broadcast();
    let label = label.broadcast();
    let placeholder = placeholder.broadcast();
    let is_valid = is_valid.broadcast();
    let is_valid_bool = is_valid.signal_ref(|v| v.is_valid()).broadcast();
    let value = std::rc::Rc::new(value);
    let selected_signal = value.get_signal().broadcast();

    let is_open = Mutable::new(false);
    let is_hovered = Mutable::new(false);
    let search_text = Mutable::new(String::new());

    let options = std::rc::Rc::new(options);

    // Filtered options signal
    let filtered_options = {
        let options = options.clone();
        search_text.signal_cloned().map(move |query| {
            let query_lower = query.to_lowercase();
            if query_lower.is_empty() {
                options.iter().cloned().collect::<Vec<_>>()
            } else {
                options
                    .iter()
                    .filter(|o| o.label.to_lowercase().contains(&query_lower))
                    .cloned()
                    .collect::<Vec<_>>()
            }
        })
    }
    .broadcast();

    html!("div", {
        .dwclass!("flex flex-col")
        .style("position", "relative")

        // Label
        .child_signal(label.signal_cloned().map({
            let is_open = is_open.clone();
            let selected_signal = selected_signal.clone();
            let search_text = search_text.clone();
            let is_valid_bool = is_valid_bool.clone();
            move |lbl| {
                if lbl.is_empty() {
                    None
                } else {
                    Some(html!("label", {
                        .dwclass!("text-sm transition-all")
                        .style("transition-duration", "var(--glass-transition)")
                        .style("transition-timing-function", "var(--glass-ease)")
                        .style("padding-left", "2px")
                        .style("margin-bottom", "6px")
                        .style_signal("color", {
                            let is_open = is_open.clone();
                            let selected_signal = selected_signal.clone();
                            let search_text = search_text.clone();
                            let is_valid_bool = is_valid_bool.clone();
                            map_ref! {
                                let open = is_open.signal(),
                                let sel = selected_signal.signal_cloned(),
                                let query = search_text.signal_cloned(),
                                let valid = is_valid_bool.signal() => {
                                    if !*valid {
                                        "var(--glass-error)"
                                    } else if *open || sel.is_some() || !query.is_empty() {
                                        "var(--glass-accent)"
                                    } else {
                                        "var(--glass-text-secondary)"
                                    }
                                }
                            }
                        })
                        .text(&lbl)
                    }))
                }
            }
        }))

        // Input wrapper
        .child(html!("div", {
            .dwclass!("flex items-center w-full")
            .style("background", "var(--glass-bg-inset)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"],
                "blur(var(--glass-blur)) saturate(var(--glass-saturation))")
            .style("border-radius", "var(--glass-border-radius)")
            .style("border", "none")
            .style("transition-duration", "var(--glass-transition)")
            .style("transition-timing-function", "var(--glass-ease)")
            .style_signal("box-shadow", {
                let is_open = is_open.clone();
                let is_hovered = is_hovered.clone();
                let disabled = disabled.clone();
                let is_valid_bool = is_valid_bool.clone();
                map_ref! {
                    let open = is_open.signal(),
                    let hovered = is_hovered.signal(),
                    let d = disabled.signal(),
                    let valid = is_valid_bool.signal() => {
                        if *d {
                            "var(--glass-shadow-inset)"
                        } else if !*valid {
                            "var(--glass-shadow-inset), 0 0 0 2px var(--glass-error-muted)"
                        } else if *open {
                            "var(--glass-shadow-inset), 0 0 0 2px var(--glass-accent-muted)"
                        } else if *hovered {
                            "var(--glass-shadow-inset), 0 0 0 1px var(--glass-border-color-hover)"
                        } else {
                            "var(--glass-shadow-inset)"
                        }
                    }
                }
            })
            .style_signal("opacity", disabled.signal().map(|d| if d { "0.5" } else { "1" }))

            .event(clone!(is_hovered => move |_: events::MouseEnter| { is_hovered.set(true); }))
            .event(clone!(is_hovered => move |_: events::MouseLeave| { is_hovered.set(false); }))

            // Actual input
            .child(html!("input" => HtmlInputElement, {
                .dwclass!("w-full px-3 py-3 text-base glass-text-primary bg-transparent")
                .style("outline", "none")
                .style("border", "none")
                .style("background", "transparent")
                .attr_signal("placeholder", placeholder.signal_cloned().map(|ph| {
                    if ph.is_empty() { None } else { Some(ph) }
                }))
                .attr_signal("disabled", disabled.signal().map(|d| if d { Some("disabled") } else { None }))
                .attr("autocomplete", "off")

                .with_node!(element => {
                    // Sync input value from selected value
                    .future({
                        let options = options.clone();
                        selected_signal.signal_cloned().for_each(clone!(element => move |sel| {
                            let display = sel.as_ref()
                                .and_then(|v| options.iter().find(|o| &o.value == v))
                                .map(|o| o.label.clone())
                                .unwrap_or_default();
                            element.set_value(&display);
                            async {}
                        }))
                    })

                    // On typing: update search text and open dropdown
                    .event(clone!(search_text, is_open => move |_: events::Input| {
                        let val = element.value();
                        search_text.set(val);
                        if !is_open.get() {
                            is_open.set(true);
                        }
                    }))
                })

                .event(clone!(is_open => move |_: events::Focus| {
                    is_open.set(true);
                }))
                .event(clone!(is_open => move |_: events::FocusOut| {
                    // Delay close to allow option MouseDown to fire first
                    let is_open = is_open.clone();
                    gloo_timers::callback::Timeout::new(150, move || {
                        is_open.set(false);
                    }).forget();
                }))
            }))

            // Chevron icon
            .child(html!("span", {
                .dwclass!("glass-text-secondary px-3")
                .style("font-size", "10px")
                .style("line-height", "1")
                .style("flex-shrink", "0")
                .style("transition-duration", "var(--glass-transition)")
                .style("transition-timing-function", "var(--glass-ease)")
                .style("transition-property", "transform")
                .style_signal("transform", is_open.signal().map(|o| {
                    if o { "rotate(180deg)" } else { "rotate(0deg)" }
                }))
                .text("\u{25BC}")
            }))
        }))

        // Dropdown panel
        .child(html!("div", {
            .style("position", "absolute")
            .style("top", "100%")
            .style("left", "0")
            .style("right", "0")
            .style("margin-top", "4px")
            .style("z-index", "50")
            .style("background", "var(--glass-bg-elevated)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"],
                "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
            .style("border", "none")
            .style("border-radius", "var(--glass-border-radius)")
            .style("box-shadow", "var(--glass-shadow-lg)")
            .style("max-height", "240px")
            .style("overflow-y", "auto")
            .style("transition-property", "opacity, transform, visibility")
            .style("transition-duration", "var(--glass-transition)")
            .style("transition-timing-function", "var(--glass-ease)")

            .style_signal("opacity", is_open.signal().map(|o| if o { "1" } else { "0" }))
            .style_signal("visibility", is_open.signal().map(|o| if o { "visible" } else { "hidden" }))
            .style_signal("pointer-events", is_open.signal().map(|o| if o { "auto" } else { "none" }))
            .style_signal("transform", is_open.signal().map(|o| {
                if o { "translateY(0)" } else { "translateY(-4px)" }
            }))

            .attr("role", "listbox")

            // Filtered options (rebuilt when search changes)
            .child_signal(filtered_options.signal_cloned().map({
                let value = value.clone();
                let is_open = is_open.clone();
                let selected_signal = selected_signal.clone();
                let search_text = search_text.clone();
                move |filtered| {
                    if filtered.is_empty() {
                        Some(html!("div", {
                            .dwclass!("px-3 py-2 text-sm glass-text-secondary")
                            .text("No results")
                        }))
                    } else {
                        Some(html!("div", {
                            .children(filtered.into_iter().map({
                                let value = value.clone();
                                let is_open = is_open.clone();
                                let selected_signal = selected_signal.clone();
                                let search_text = search_text.clone();
                                move |option| {
                                    let opt_value_for_sel = option.value.clone();
                                    let opt_value_for_bg = option.value.clone();
                                    let opt_value_for_color = option.value.clone();
                                    let opt_value_for_click = option.value.clone();
                                    let value = value.clone();
                                    let is_open = is_open.clone();
                                    let selected_signal = selected_signal.clone();
                                    let search_text = search_text.clone();
                                    let item_hover = Mutable::new(false);

                                    html!("div", {
                                        .dwclass!("px-3 py-2 text-base cursor-pointer transition-all")
                                        .style("transition-duration", "var(--glass-transition-fast)")
                                        .style("transition-timing-function", "var(--glass-ease)")

                                        .attr("role", "option")
                                        .attr_signal("aria-selected", selected_signal.signal_cloned().map(move |sel| {
                                            if sel.as_deref() == Some(&opt_value_for_sel) { "true" } else { "false" }
                                        }))

                                        .style_signal("background", {
                                            let item_hover = item_hover.clone();
                                            let selected_signal = selected_signal.clone();
                                            map_ref! {
                                                let h = item_hover.signal(),
                                                let sel = selected_signal.signal_cloned() => {
                                                    if sel.as_deref() == Some(&opt_value_for_bg) {
                                                        "var(--glass-accent-muted)"
                                                    } else if *h {
                                                        "rgba(255, 255, 255, 0.06)"
                                                    } else {
                                                        "transparent"
                                                    }
                                                }
                                            }
                                        })
                                        .style_signal("color", selected_signal.signal_cloned().map(move |sel| {
                                            if sel.as_deref() == Some(&opt_value_for_color) {
                                                "var(--glass-accent)"
                                            } else {
                                                "var(--glass-text-primary)"
                                            }
                                        }))

                                        .event(clone!(item_hover => move |_: events::MouseEnter| { item_hover.set(true); }))
                                        .event(clone!(item_hover => move |_: events::MouseLeave| { item_hover.set(false); }))
                                        // MouseDown fires before FocusOut so selection works
                                        .event(move |_: events::MouseDown| {
                                            value.set(Some(opt_value_for_click.clone()));
                                            search_text.set(String::new());
                                            is_open.set(false);
                                        })

                                        .text(&option.label)
                                    })
                                }
                            }).collect::<Vec<_>>())
                        }))
                    }
                }
            }))
        }))

        // Escape to close
        .global_event(clone!(is_open, search_text => move |e: events::KeyDown| {
            if e.key() == "Escape" {
                is_open.set(false);
                search_text.set(String::new());
            }
        }))

        // Validation message
        .child_signal(is_valid.signal_cloned().map(|v| {
            match v {
                ValidationResult::Valid => None,
                ValidationResult::Invalid { message } => {
                    Some(html!("div", {
                        .dwclass!("text-sm pt-1 px-3 glass-text-error")
                        .text(&message)
                    }))
                }
            }
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
