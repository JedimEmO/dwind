use crate::input::{InputValueWrapper, ValidationResult};
use crate::prelude::*;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use web_sys::HtmlInputElement;

pub enum TextInputType {
    Text,
    Password,
}

#[component(render_fn = glass_text_input)]
struct GlassTextInput {
    #[default(Box::new(Mutable::new("".to_string())))]
    value: dyn InputValueWrapper + 'static,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default("".to_string())]
    label: String,

    #[signal]
    #[default("".to_string())]
    placeholder: String,

    #[default(Box::new(|| {}))]
    on_submit: dyn (FnMut() -> ()) + 'static,

    #[signal]
    #[default(TextInputType::Text)]
    input_type: TextInputType,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[default(false)]
    claim_focus: bool,
}

pub fn glass_text_input(props: GlassTextInputProps) -> Dom {
    let GlassTextInputProps {
        value,
        is_valid,
        label,
        placeholder,
        mut on_submit,
        input_type,
        disabled,
        claim_focus,
        apply,
    } = props;

    let label = label.broadcast();
    let placeholder = placeholder.broadcast();
    let is_focused = Mutable::new(false);
    let is_hovered = Mutable::new(false);
    let parsed_validation_result = Mutable::new(ValidationResult::Valid);

    let internal_value = value.value_signal_cloned().map(|v| !v.is_empty()).broadcast();

    let validation_signal = map_ref! {
        let parse_result = parsed_validation_result.signal_cloned(),
        let external_result = is_valid => {
            if !parse_result.is_valid() {
                parse_result.clone()
            } else if !external_result.is_valid() {
                external_result.clone()
            } else {
                ValidationResult::Valid
            }
        }
    }
    .broadcast();

    let is_valid_signal = validation_signal
        .signal_ref(|v| v.is_valid())
        .broadcast();

    // Label is raised when: has label text AND (focused OR has value OR invalid)
    let raise_label = map_ref! {
        let has_label = label.signal_ref(|v| !v.is_empty()),
        let focused = is_focused.signal(),
        let has_val = internal_value.signal(),
        let valid = is_valid_signal.signal() => {
            *has_label && (*focused || *has_val || !*valid)
        }
    }
    .broadcast();

    html!("div", {
        .dwclass!("flex flex-col")

        // Label — sits above the input when visible
        .child_signal(label.signal_cloned().map({
            let raise_label = raise_label.clone();
            let is_valid_signal = is_valid_signal.clone();
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
                            let is_valid_signal = is_valid_signal.clone();
                            map_ref! {
                                let raised = raise_label.signal(),
                                let valid = is_valid_signal.signal() => {
                                    if !*valid {
                                        "var(--glass-error)"
                                    } else if *raised {
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

        // Input element
        .child(html!("input" => HtmlInputElement, {
            .dwclass!("w-full px-3 py-3 text-base glass-text-primary bg-transparent transition-all")
            .style("outline", "none")
            .style("background", "var(--glass-bg-inset)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur)) saturate(var(--glass-saturation))")
            .style("border-radius", "var(--glass-border-radius)")
            .style("border", "none")
            .style("transition-duration", "var(--glass-transition)")
            .style("transition-timing-function", "var(--glass-ease)")
            .style_signal("box-shadow", {
                let is_valid = is_valid_signal.clone();
                let is_focused = is_focused.clone();
                let is_hovered = is_hovered.clone();
                map_ref! {
                    let valid = is_valid.signal(),
                    let focused = is_focused.signal(),
                    let hovered = is_hovered.signal() => {
                        if !*valid {
                            "var(--glass-shadow-inset), 0 0 0 2px var(--glass-error-muted)"
                        } else if *focused {
                            "var(--glass-shadow-inset), 0 0 0 2px var(--glass-accent-muted)"
                        } else if *hovered {
                            "var(--glass-shadow-inset), 0 0 0 1px var(--glass-border-color-hover)"
                        } else {
                            "var(--glass-shadow-inset)"
                        }
                    }
                }
            })
            .attr_signal("placeholder", placeholder.signal_cloned().map(|ph| {
                if ph.is_empty() { None } else { Some(ph) }
            }))
            .attr_signal("disabled", disabled.map(|d| if d { Some("disabled") } else { None }))
            .attr_signal("type", input_type.map(|t| match t {
                TextInputType::Text => "text",
                TextInputType::Password => "password",
            }))
            .focused(claim_focus)
            .with_node!(element => {
                .future(value.value_signal_cloned().for_each(clone!(element => move |v| {
                    element.set_value(&v);
                    async move {}
                })))
                .event(clone!(parsed_validation_result => move |_: events::Input| {
                    let result = value.set(element.value());
                    if !result.is_valid() {
                        parsed_validation_result.set(result);
                    } else {
                        parsed_validation_result.set(ValidationResult::Valid);
                    }
                }))
            })
            .event(clone!(is_focused => move |_: events::Focus| {
                is_focused.set(true);
            }))
            .event(clone!(is_focused => move |_: events::FocusOut| {
                is_focused.set(false);
            }))
            .event(clone!(is_hovered => move |_: events::MouseEnter| {
                is_hovered.set(true);
            }))
            .event(clone!(is_hovered => move |_: events::MouseLeave| {
                is_hovered.set(false);
            }))
            .event(move |event: events::KeyDown| {
                if event.key() == "Enter" {
                    on_submit()
                }
            })
        }))

        // Validation message
        .child_signal(validation_signal.signal_cloned().map(|v| {
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
