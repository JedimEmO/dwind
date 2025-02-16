use crate::components::input::validation::InputValueWrapper;
use crate::mixins::labelled_rect_mixin::labelled_rect_mixin;
use crate::prelude::ValidationResult;
use crate::theme::prelude::*;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{and, not, or, Mutable, SignalExt};
use futures_signals_component_macro::component;
use web_sys::HtmlInputElement;

pub enum TextInputType {
    Text,
    Password,
}

#[component(render_fn=text_input)]
struct TextInput {
    #[default(Box::new(Mutable::new("".to_string())))]
    value: dyn InputValueWrapper + Send + 'static,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default("".to_string())]
    label: String,

    #[default(Box::new(|| {}))]
    on_submit: dyn (FnMut() -> ()) + Send + 'static,

    #[signal]
    #[default(TextInputType::Text)]
    input_type: TextInputType,

    #[default(false)]
    claim_focus: bool
}

pub fn text_input(props: TextInputProps) -> Dom {
    let TextInputProps {
        value,
        is_valid,
        label,
        mut on_submit,
        input_type,
        claim_focus, apply,
    } = props;

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

    let is_valid = validation_signal
        .signal_ref(|validation| validation.is_valid())
        .broadcast();

    let raise_label = and(
        has_label,
        or(or(is_focused.signal(), has_value), not(is_valid.signal())),
    );

    html!("div", {
        .dwclass!("grid")
        .children([
            html!("input" => HtmlInputElement, {
                .dwclass!("text-base transition-all")
                .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-300 text-base")
                .dwclass!("p-l-2")
                .dwclass!("grid-col-1 grid-row-1")
                .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-900")
                .dwclass_signal!("h-10", is_valid.signal())
                .dwclass_signal!("h-6", not(is_valid.signal()))
                .focused(claim_focus)
                .attr_signal("type", input_type.map(|t| {
                    match t {
                        TextInputType::Text => { "text" }
                        TextInputType::Password => { "password" }
                    }
                }))
                .with_node!(element => {
                    .future(value.value_signal_cloned().for_each(clone!(element => move |v| {
                        element.set_value(&v);
                        async move {}
                    })))
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
        ])

        .apply(labelled_rect_mixin(label.signal_cloned(), raise_label, validation_signal.signal_cloned()))
        .apply_if(apply.is_some(),|b| b.apply(apply.unwrap()))
    })
}
