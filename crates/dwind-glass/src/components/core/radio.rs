use crate::input::{GlassSelectOption, GlassSelectValue, ValidationResult};
use crate::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[component(render_fn = glass_radio_group)]
struct GlassRadioGroup {
    #[default(Box::new(Mutable::new(None::<String>)))]
    value: dyn GlassSelectValue + 'static,

    #[default(vec![])]
    options: Vec<GlassSelectOption>,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default(false)]
    disabled: bool,
}

pub fn glass_radio_group(props: GlassRadioGroupProps) -> Dom {
    let GlassRadioGroupProps {
        value,
        options,
        is_valid,
        disabled,
        apply,
    } = props;

    let disabled = disabled.broadcast();
    let is_valid = is_valid.broadcast();
    let is_valid_bool = is_valid.signal_ref(|v| v.is_valid()).broadcast();
    let value = std::rc::Rc::new(value);
    let selected_signal = value.get_signal().broadcast();

    html!("div", {
        .dwclass!("flex flex-col gap-2")
        .attr("role", "radiogroup")
        .style("padding", "8px")
        .style("border-radius", "var(--glass-border-radius)")
        .style("transition-duration", "var(--glass-transition)")
        .style("transition-timing-function", "var(--glass-ease)")
        .style("transition-property", "box-shadow")
        .style_signal("box-shadow", is_valid_bool.signal().map(|valid| {
            if !valid {
                "0 0 0 2px var(--glass-error-muted)"
            } else {
                "none"
            }
        }))
        .style_signal("opacity", disabled.signal().map(|d| if d { "0.5" } else { "1" }))
        .style_signal("pointer-events", disabled.signal().map(|d| if d { "none" } else { "auto" }))

        .children(options.into_iter().map({
            let value = value.clone();
            let selected_signal = selected_signal.clone();
            let disabled = disabled.clone();
            move |option| {
                let opt_value_for_check = option.value.clone();
                let opt_value_for_dot = option.value.clone();
                let opt_value_for_dot2 = option.value.clone();
                let opt_value_for_click = option.value.clone();
                let value = value.clone();
                let selected_signal = selected_signal.clone();
                let disabled = disabled.clone();
                let hover = Mutable::new(false);

                html!("label", {
                    .dwclass!("inline-flex items-center gap-3 select-none cursor-pointer")
                    .style("transition-duration", "var(--glass-transition)")

                    .attr("role", "radio")
                    .attr_signal("aria-checked", selected_signal.signal_cloned().map(move |sel| {
                        if sel.as_deref() == Some(&opt_value_for_check) { "true" } else { "false" }
                    }))

                    // Radio circle
                    .child(html!("div", {
                        .dwclass!("relative flex items-center justify-center")
                        .style("width", "18px")
                        .style("height", "18px")
                        .style("border-radius", "var(--glass-border-radius-full)")
                        .style("transition-duration", "var(--glass-transition)")
                        .style("transition-timing-function", "var(--glass-ease)")
                        .style("transition-property", "background, box-shadow")
                        .style("flex-shrink", "0")
                        .style("background", "var(--glass-bg-inset)")
                        .style("border", "none")
                        .style_signal("box-shadow", {
                            let hover = hover.clone();
                            let disabled = disabled.clone();
                            map_ref! {
                                let h = hover.signal(),
                                let d = disabled.signal() => {
                                    if *h && !*d {
                                        "var(--glass-shadow-inset), 0 0 0 1px rgba(255, 255, 255, 0.10)"
                                    } else {
                                        "var(--glass-shadow-inset)"
                                    }
                                }
                            }
                        })

                        .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
                        .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))

                        // Inner dot
                        .child(html!("div", {
                            .style("width", "10px")
                            .style("height", "10px")
                            .style("border-radius", "var(--glass-border-radius-full)")
                            .style("background", "var(--glass-accent)")
                            .style("transition-duration", "var(--glass-transition)")
                            .style("transition-timing-function", "var(--glass-ease)")
                            .style("transition-property", "transform, opacity")
                            .style_signal("transform", selected_signal.signal_cloned().map(move |sel| {
                                if sel.as_deref() == Some(&opt_value_for_dot) { "scale(1)" } else { "scale(0)" }
                            }))
                            .style_signal("opacity", selected_signal.signal_cloned().map(move |sel| {
                                if sel.as_deref() == Some(&opt_value_for_dot2) { "1" } else { "0" }
                            }))
                        }))
                    }))

                    // Label text
                    .child(html!("span", {
                        .dwclass!("glass-text-primary text-base")
                        .text(&option.label)
                    }))

                    .event(move |_: events::Click| {
                        value.set(Some(opt_value_for_click.clone()));
                    })
                })
            }
        }).collect::<Vec<_>>())

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
