use crate::components::input::validation::InputValueWrapper;
use crate::mixins::field_mixin::{field_error_row, field_surface_mixin};
use crate::prelude::ValidationResult;
use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{and, not, or, Mutable, SignalExt};
use futures_signals_component_macro::component;
use web_sys::HtmlTextAreaElement;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TextAreaResize {
    None,
    Vertical,
    Both,
}

/// A multi-line text field sharing the labelled field surface of
/// [`text_input`](crate::components::input::text_input::text_input).
#[component(render_fn = text_area)]
struct TextArea {
    #[default(Box::new(Mutable::new("".to_string())))]
    value: dyn InputValueWrapper + 'static,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default("".to_string())]
    #[into]
    label: String,

    #[signal]
    #[default(4)]
    rows: u32,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[default(TextAreaResize::Vertical)]
    resize: TextAreaResize,
}

pub fn text_area(props: TextAreaProps) -> Dom {
    let TextAreaProps {
        value,
        is_valid,
        label,
        rows,
        disabled,
        resize,
        apply,
    } = props;

    let label = label.broadcast();
    let disabled = disabled.broadcast();

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

    let input_id = component_id("text-area");
    let error_id = format!("{}-error", input_id);

    html!("div", {
        .dwclass!("flex flex-col w-full")
        .child(html!("div", {
            .dwclass!("flex-none")
            .dwclass_signal!("opacity-60", disabled.signal())
            .style("--dwui-field-label-top", "1.25rem")
            .apply(field_surface_mixin(
                label.signal_cloned(),
                raise_label,
                validation_signal.signal_cloned(),
                input_id.clone(),
            ))
            .child(html!("textarea" => HtmlTextAreaElement, {
                .attr("id", &input_id)
                .dwclass!("w-full bg-transparent border-none text-base p-l-3 p-r-3 p-t-5 p-b-2 block")
                .dwclass!("dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-900")
                .dwclass_signal!("cursor-not-allowed", disabled.signal())
                .style("outline", "none")
                .style("font-family", "inherit")
                .style("resize", match resize {
                    TextAreaResize::None => "none",
                    TextAreaResize::Vertical => "vertical",
                    TextAreaResize::Both => "both",
                })
                .attr_signal("rows", rows.map(|v| v.to_string()))
                .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
                .attr_signal("aria-invalid", is_valid.signal().map(|valid| if valid { None } else { Some("true") }))
                .attr_signal("aria-describedby", is_valid.signal().map(clone!(error_id => move |valid| {
                    if valid {
                        None
                    } else {
                        Some(error_id.clone())
                    }
                })))
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
            }))
        }))
        .child(field_error_row(validation_signal.signal_cloned(), &input_id))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
