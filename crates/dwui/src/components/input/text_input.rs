use crate::components::input::validation::InputValueWrapper;
use crate::prelude::ValidationResult;
use crate::theme::prelude::*;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{and, not, or, Mutable, Signal, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use web_sys::HtmlInputElement;

pub enum TextInputType {
    Text,
    Password,
}

#[component(render_fn=text_input)]
struct TextInput<
    TValue: InputValueWrapper + 'static = Mutable<String>,
    TOnSubmit: (FnMut() -> ()) + 'static = fn() -> (),
> {
    #[default(Mutable::new("".to_string()))]
    value: TValue,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default("".to_string())]
    label: String,

    #[default(|| {})]
    on_submit: TOnSubmit,

    #[signal]
    #[default(TextInputType::Text)]
    input_type: TextInputType,
}

pub fn text_input(props: impl TextInputPropsTrait + 'static) -> Dom {
    let TextInputProps {
        value,
        is_valid,
        label,
        mut on_submit,
        input_type,
        apply,
    } = props.take();

    let label = label.broadcast();

    let has_label = label.signal_ref(|v| v.len() > 0);

    let has_value = value.value_signal_cloned().map(|v| v.len() > 0);

    let is_focused = Mutable::new(false);
    let parsed_validation_result = Mutable::new(ValidationResult::Valid);

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

    let is_valid = validation_signal.signal_ref(|v| v.is_valid()).broadcast();

    let raise_label = and(
        has_label,
        or(or(is_focused.signal(), has_value), not(is_valid.signal())),
    )
    .broadcast();

    let top_border_margin_signal = map_ref! {
        let raise = raise_label.signal(),
        let label = label.signal_cloned() => {
            if !raise {
                "0px".to_string()
            } else {
                format!("{}px", label.len() as f32  * 9.)
            }
        }
    };

    let bottom_border_margin_signal = map_ref! {
        let is_valid = is_valid.signal(),
        let label = label.signal_cloned() => {
            if *is_valid {
                "0px".to_string()
            } else {
                format!("{}px", label.len() as f32  * 10.)
            }
        }
    };

    html!("div", {
        .dwclass!("grid")
        .children([
            html!("input" => HtmlInputElement, {
                .dwclass!("is(.light *):dwui-bg-void-300 text-base transition-all")
                .dwclass!("p-l-2")
                .dwclass!("grid-col-1 grid-row-1")
                .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-900")
                .dwclass_signal!("h-12", is_valid.signal())
                .dwclass_signal!("h-8", not(is_valid.signal()))
                .attr_signal("value", value.value_signal_cloned())
                .attr_signal("type", input_type.map(|t| {
                    match t {
                        TextInputType::Text => { "text" }
                        TextInputType::Password => { "password" }
                    }
                }))
                .with_node!(element => {
                    .event(clone!(parsed_validation_result => move |_: events::Input| {
                        let result = value.set(element.value());

                        if !result.is_valid() {
                            parsed_validation_result.set(result);
                        }
                    }))
                })
                .event(clone!(is_focused => move |_: events::FocusOut| {
                    is_focused.set(false);
                }))
                .event(clone!(is_focused => move |_: events::Focus| {
                    is_focused.set(true);
                }))
                .event(move |event: events::KeyDown| {
                    if event.key() == "Enter" {
                        on_submit()
                    }
                })
            }),
            html!("label", {
                .dwclass!("grid-col-1 grid-row-1 pointer-events-none transition-all m-l-4")
                .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-900")
                .dwclass_signal!("text-sm", raise_label.signal())
                .dwclass_signal!("text-base", not(raise_label.signal()))
                .style_signal("margin-top", raise_label.signal().map(|v| {
                    if v {
                        "-10px"
                    } else {
                        "12px"
                    }
                }))
                .text_signal(label.signal_cloned())
            }),
            html!("div", {
                .dwclass!("grid-col-1 grid-row-1 pointer-events-none border-l border-r border-b")
                .dwclass_signal!("dwui-border-void-600 is(.light *):dwui-border-void-200", is_valid.signal())
                .dwclass_signal!("dwui-border-error-600 is(.light *):dwui-border-error-700", not(is_valid.signal()))
            })
        ])
        .child(html!("div", {
            .dwclass!("grid-col-1 grid-row-1 pointer-events-none w-2 border-t")
            .dwclass_signal!("dwui-border-void-600 is(.light *):dwui-border-void-200", is_valid.signal())
            .dwclass_signal!("dwui-border-error-500 is(.light *):dwui-border-error-700", not(is_valid.signal()))
        }))
        .child(html!("div", {
            .dwclass!("grid-col-1 grid-row-1 pointer-events-none transition-all border-t")
            .dwclass_signal!("dwui-border-void-600 is(.light *):dwui-border-void-200", is_valid.signal())
            .dwclass_signal!("dwui-border-error-500 is(.light *):dwui-border-error-700", not(is_valid.signal()))
            .style_signal("margin-left", top_border_margin_signal)
        }))
        .child_signal(validation_signal.signal_cloned().map(|validation| {
            match validation {
                ValidationResult::Valid => None,
                ValidationResult::Invalid{ message } => {
                    Some(html!("div", {
                        .dwclass!("grid-col-1 grid-row-2 pointer-events-none transition-all text-sm")
                        .dwclass!("dwui-text-error-500 is(.light *):dwui-text-error-700")
                        .text(&message)
                    }))
                }
            }
        }))
    })
}
