use crate::components::input::validation::InputValueWrapper;
use crate::mixins::field_mixin::{field_error_row, field_surface_mixin};
use crate::prelude::ValidationResult;
use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, svg, with_node, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{and, not, or, Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::rc::Rc;
use web_sys::HtmlInputElement;

/// A numeric field sharing the labelled field surface of
/// [`text_input`](crate::components::input::text_input::text_input).
///
/// Renders a native `type="number"` input (spinbutton semantics and arrow-key
/// stepping for free) with themed stepper buttons replacing the browser
/// spinners. The steppers are decorative duplicates of the keyboard
/// interaction, so they are not tabbable.
#[component(render_fn = number_input)]
struct NumberInput {
    #[default(Box::new(Mutable::new(0f64)))]
    value: dyn InputValueWrapper + 'static,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default("".to_string())]
    label: String,

    #[signal]
    #[default(None)]
    min: Option<f64>,

    #[signal]
    #[default(None)]
    max: Option<f64>,

    #[signal]
    #[default(1.0)]
    step: f64,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[default(Box::new(|| {}))]
    on_submit: dyn (FnMut() -> ()) + 'static,
}

pub fn number_input(props: NumberInputProps) -> Dom {
    let NumberInputProps {
        value,
        is_valid,
        label,
        min,
        max,
        step,
        disabled,
        mut on_submit,
        apply,
    } = props;

    let value = Rc::new(value);
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

    let input_id = component_id("number-input");
    let error_id = format!("{}-error", input_id);

    // Shared handle to the input element so the steppers can drive it
    let input_element: Mutable<Option<HtmlInputElement>> = Mutable::new(None);

    let stepper = |direction_up: bool, path: &str| {
        html!("button", {
            .attr("type", "button")
            .attr("tabindex", "-1")
            .attr("aria-hidden", "true")
            .dwclass!("w-5 h-4 flex align-items-center justify-center cursor-pointer")
            .dwclass!("bg-transparent border-none p-0")
            .dwclass!("dwui-text-on-primary-400 hover:dwui-text-on-primary-200")
            .dwclass!("is(.light *):dwui-text-on-primary-600 is(.light *):hover:dwui-text-on-primary-800")
            .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
            .child(svg!("svg", {
                .attr("viewBox", "0 0 10 6")
                .attr("width", "10")
                .attr("height", "6")
                .attr("fill", "none")
                .child(svg!("path", {
                    .attr("d", path)
                    .attr("stroke", "currentColor")
                    .attr("stroke-width", "1.5")
                    .attr("stroke-linecap", "round")
                    .attr("stroke-linejoin", "round")
                }))
            }))
            .event(clone!(input_element, value => move |_: events::Click| {
                let Some(element) = input_element.get_cloned() else { return };

                let attr_f64 = |name: &str| {
                    element
                        .get_attribute(name)
                        .and_then(|v| v.parse::<f64>().ok())
                };

                let step = attr_f64("step").unwrap_or(1.0);
                let current = element.value().parse::<f64>().unwrap_or(0.0);

                let mut next = if direction_up {
                    current + step
                } else {
                    current - step
                };

                if let Some(min) = attr_f64("min") {
                    next = next.max(min);
                }
                if let Some(max) = attr_f64("max") {
                    next = next.min(max);
                }

                element.set_value(&next.to_string());
                value.set(element.value());
            }))
        })
    };

    html!("div", {
        .dwclass!("flex flex-col w-full")
        .child(html!("div", {
            .dwclass!("h-10 flex-none flex flex-row align-items-center")
            .dwclass_signal!("opacity-60", disabled.signal())
            .apply(field_surface_mixin(
                label.signal_cloned(),
                raise_label,
                validation_signal.signal_cloned(),
                input_id.clone(),
            ))
            .child(html!("input" => HtmlInputElement, {
                .attr("id", &input_id)
                .attr("type", "number")
                .attr("inputmode", "decimal")
                .dwclass!("w-full h-full grow bg-transparent border-none text-base p-l-3 p-t-3")
                .dwclass!("dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-900")
                .dwclass_signal!("cursor-not-allowed", disabled.signal())
                .style("outline", "none")
                // Hide the native spinners; the themed steppers replace them.
                // Inline declarations only — no vendor pseudo-element selectors.
                .style("appearance", "textfield")
                .style("-moz-appearance", "textfield")
                .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
                .attr_signal("min", min.map(|v| v.map(|v| v.to_string())))
                .attr_signal("max", max.map(|v| v.map(|v| v.to_string())))
                .attr_signal("step", step.map(|v| v.to_string()))
                .attr_signal("aria-invalid", is_valid.signal().map(|valid| if valid { None } else { Some("true") }))
                .attr_signal("aria-describedby", is_valid.signal().map(clone!(error_id => move |valid| {
                    if valid {
                        None
                    } else {
                        Some(error_id.clone())
                    }
                })))
                .with_node!(element => {
                    .apply(clone!(input_element, element => move |b| {
                        input_element.set(Some(element));
                        b
                    }))
                    .future(value.value_signal_cloned().for_each(clone!(element => move |v| {
                        element.set_value(&v);
                        async move {}
                    })))
                    .event(clone!(parsed_validation_result, value => move |_: events::Input| {
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
            }))
            .child(html!("div", {
                .dwclass!("flex flex-col flex-none justify-center p-r-2 gap-1")
                .child(stepper(true, "M1 4.5 L5 1.5 L9 4.5"))
                .child(stepper(false, "M1 1.5 L5 4.5 L9 1.5"))
            }))
        }))
        .child(field_error_row(validation_signal.signal_cloned(), &input_id))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
