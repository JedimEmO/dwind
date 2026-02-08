use crate::input::{GlassSelectOption, GlassSelectValue, ValidationResult};
use crate::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[component(render_fn = glass_select)]
struct GlassSelect {
    #[default(Box::new(Mutable::new(None::<String>)))]
    value: dyn GlassSelectValue + 'static,

    #[default(vec![])]
    options: Vec<GlassSelectOption>,

    #[signal]
    #[default("Select...".to_string())]
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

pub fn glass_select(props: GlassSelectProps) -> Dom {
    let GlassSelectProps {
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
    let just_opened = Mutable::new(false);

    // Clone options for display text lookup
    let options_for_display: Vec<GlassSelectOption> = options.iter().cloned().collect();

    html!("div", {
        .dwclass!("flex flex-col")
        .style("position", "relative")

        // Label
        .child_signal(label.signal_cloned().map({
            let is_open = is_open.clone();
            let selected_signal = selected_signal.clone();
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
                            let is_valid_bool = is_valid_bool.clone();
                            map_ref! {
                                let open = is_open.signal(),
                                let selected = selected_signal.signal_cloned(),
                                let valid = is_valid_bool.signal() => {
                                    if !*valid {
                                        "var(--glass-error)"
                                    } else if *open || selected.is_some() {
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

        // Trigger
        .child(html!("div", {
            .dwclass!("w-full px-3 py-3 text-base glass-text-primary transition-all cursor-pointer")
            .dwclass!("flex items-center justify-between")
            .style("outline", "none")
            .style("background", "var(--glass-bg-inset)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"],
                "blur(var(--glass-blur)) saturate(var(--glass-saturation))")
            .style("border-radius", "var(--glass-border-radius)")
            .style("border", "none")
            .style("transition-duration", "var(--glass-transition)")
            .style("transition-timing-function", "var(--glass-ease)")
            .style("user-select", "none")

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
            .style_signal("pointer-events", disabled.signal().map(|d| if d { "none" } else { "auto" }))
            .style_signal("cursor", disabled.signal().map(|d| if d { "not-allowed" } else { "pointer" }))

            .attr("tabindex", "0")
            .attr("aria-haspopup", "listbox")
            .attr_signal("aria-expanded", is_open.signal().map(|o| if o { "true" } else { "false" }))

            // Display text
            .child(html!("span", {
                .style("flex", "1")
                .style("min-width", "0")
                .style("overflow", "hidden")
                .style("text-overflow", "ellipsis")
                .style("white-space", "nowrap")
                .style_signal("color", selected_signal.signal_cloned().map(|sel| {
                    if sel.is_some() { "var(--glass-text-primary)" } else { "var(--glass-text-tertiary)" }
                }))
                .text_signal({
                    let options_for_display = options_for_display.clone();
                    let placeholder = placeholder.clone();
                    map_ref! {
                        let sel = selected_signal.signal_cloned(),
                        let ph = placeholder.signal_cloned() => {
                            if let Some(ref val) = *sel {
                                options_for_display.iter()
                                    .find(|o| &o.value == val)
                                    .map(|o| o.label.clone())
                                    .unwrap_or(val.clone())
                            } else {
                                ph.clone()
                            }
                        }
                    }
                })
            }))

            // Chevron
            .child(html!("span", {
                .dwclass!("glass-text-secondary")
                .style("transition-duration", "var(--glass-transition)")
                .style("transition-timing-function", "var(--glass-ease)")
                .style("transition-property", "transform")
                .style("font-size", "10px")
                .style("line-height", "1")
                .style_signal("transform", is_open.signal().map(|o| {
                    if o { "rotate(180deg)" } else { "rotate(0deg)" }
                }))
                .text("\u{25BC}")
            }))

            .event(clone!(is_hovered => move |_: events::MouseEnter| { is_hovered.set(true); }))
            .event(clone!(is_hovered => move |_: events::MouseLeave| { is_hovered.set(false); }))
            .event(clone!(is_open, just_opened => move |_: events::Click| {
                if !is_open.get() {
                    just_opened.set(true);
                }
                is_open.set(!is_open.get());
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

            .children(options.into_iter().map({
                let value = value.clone();
                let is_open = is_open.clone();
                let selected_signal = selected_signal.clone();
                move |option| {
                    let opt_value_for_sel = option.value.clone();
                    let opt_value_for_bg = option.value.clone();
                    let opt_value_for_color = option.value.clone();
                    let opt_value_for_click = option.value.clone();
                    let value = value.clone();
                    let is_open = is_open.clone();
                    let selected_signal = selected_signal.clone();
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
                        .event(move |_: events::Click| {
                            value.set(Some(opt_value_for_click.clone()));
                            is_open.set(false);
                        })

                        .text(&option.label)
                    })
                }
            }).collect::<Vec<_>>())
        }))

        // Click outside to close
        .global_event(clone!(is_open, just_opened => move |_: events::Click| {
            if just_opened.get() {
                just_opened.set(false);
            } else if is_open.get() {
                is_open.set(false);
            }
        }))

        // Escape to close
        .global_event(clone!(is_open => move |e: events::KeyDown| {
            if e.key() == "Escape" {
                is_open.set(false);
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
