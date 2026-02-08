use crate::input::ValidationResult;
use crate::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[component(render_fn = glass_checkbox)]
struct GlassCheckbox {
    #[default(Box::new(Mutable::new(false)))]
    checked: dyn GlassToggleValue + 'static,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default(None)]
    label: Option<String>,
}

pub fn glass_checkbox(props: GlassCheckboxProps) -> Dom {
    let GlassCheckboxProps {
        checked,
        is_valid,
        disabled,
        label,
        apply,
    } = props;

    let disabled = disabled.broadcast();
    let is_valid = is_valid.broadcast();
    let is_valid_bool = is_valid.signal_ref(|v| v.is_valid()).broadcast();
    let checked_signal = checked.get_signal().broadcast();
    let checked = std::rc::Rc::new(checked);
    let hover = Mutable::new(false);

    html!("div", {
        .dwclass!("flex flex-col")

        .child(html!("label", {
            .dwclass!("inline-flex items-center gap-3 select-none")
            .style("transition-duration", "var(--glass-transition)")
            .style_signal("opacity", disabled.signal().map(|d| if d { "0.5" } else { "1" }))
            .style_signal("cursor", disabled.signal().map(|d| if d { "not-allowed" } else { "pointer" }))
            .style_signal("pointer-events", disabled.signal().map(|d| if d { "none" } else { "auto" }))

            .attr("role", "checkbox")
            .attr_signal("aria-checked", checked_signal.signal().map(|c| {
                if c { "true" } else { "false" }
            }))
            .attr_signal("aria-disabled", disabled.signal().map(|d| {
                if d { Some("true") } else { None }
            }))

            // Checkbox box
            .child(html!("div", {
                .dwclass!("relative flex items-center justify-center")
                .style("width", "18px")
                .style("height", "18px")
                .style("border-radius", "var(--glass-border-radius-sm)")
                .style("transition-duration", "var(--glass-transition)")
                .style("transition-timing-function", "var(--glass-ease)")
                .style("transition-property", "background, box-shadow")
                .style("flex-shrink", "0")
                .style("border", "none")

                .style_signal("background", checked_signal.signal().map(|c| {
                    if c { "var(--glass-accent)" } else { "var(--glass-bg-inset)" }
                }))
                .style_signal("box-shadow", {
                    let hover = hover.clone();
                    let disabled = disabled.clone();
                    let is_valid_bool = is_valid_bool.clone();
                    map_ref! {
                        let h = hover.signal(),
                        let d = disabled.signal(),
                        let valid = is_valid_bool.signal() => {
                            if !*valid {
                                "var(--glass-shadow-inset), 0 0 0 2px var(--glass-error-muted)"
                            } else if *h && !*d {
                                "var(--glass-shadow-inset), 0 0 0 1px rgba(255, 255, 255, 0.10)"
                            } else {
                                "var(--glass-shadow-inset)"
                            }
                        }
                    }
                })

                .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
                .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))

                // Checkmark indicator
                .child(html!("span", {
                    .style("color", "var(--glass-text-on-accent)")
                    .style("font-size", "14px")
                    .style("font-weight", "bold")
                    .style("line-height", "1")
                    .style("transition-duration", "var(--glass-transition)")
                    .style("transition-timing-function", "var(--glass-ease)")
                    .style("transition-property", "opacity, transform")
                    .style_signal("opacity", checked_signal.signal().map(|c| {
                        if c { "1" } else { "0" }
                    }))
                    .style_signal("transform", checked_signal.signal().map(|c| {
                        if c { "scale(1)" } else { "scale(0.5)" }
                    }))
                    .text("\u{2713}")
                }))
            }))

            // Click to toggle
            .event({
                let checked = checked.clone();
                move |_: events::Click| {
                    checked.toggle();
                }
            })

            // Label text
            .child_signal(label.map(|l| {
                l.map(|text| {
                    html!("span", {
                        .dwclass!("glass-text-primary text-base")
                        .text(&text)
                    })
                })
            }))
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
