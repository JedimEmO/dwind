use crate::mixins::field_mixin::{field_error_row, field_surface_mixin};
use crate::prelude::{InputValueWrapper, ValidationResult};
use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal::{always, Mutable};
use futures_signals_component_macro::component;
use std::rc::Rc;
use web_sys::HtmlInputElement;

#[component(render_fn=slider)]
struct Slider {
    #[default(Box::new(Mutable::new("".to_string())))]
    value: dyn InputValueWrapper + 'static,

    #[signal]
    #[default(0.)]
    min: f32,

    #[signal]
    #[default(100.)]
    max: f32,

    #[signal]
    #[default(1.)]
    step: f32,

    #[signal]
    #[default("".to_string())]
    #[into]
    label: String,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,
}

pub fn slider(props: SliderProps) -> Dom {
    let SliderProps {
        value,
        min,
        max,
        step,
        label,
        disabled,
        is_valid,
        apply,
    } = props;

    let value = Rc::new(value);

    let min = min.broadcast();
    let max = max.broadcast();
    let label = label.broadcast();
    let disabled = disabled.broadcast();
    let is_valid = is_valid.broadcast();

    let slider_id = component_id("slider");

    html!("div", {
        .dwclass!("flex flex-col w-full")
        .child(html!("div", {
            .dwclass!("h-10 flex-none flex flex-row align-items-end gap-2 p-l-3 p-r-3")
            .dwclass_signal!("opacity-60", disabled.signal())
            .apply(field_surface_mixin(
                label.signal_cloned(),
                always(true),
                is_valid.signal_cloned(),
                slider_id.clone(),
            ))
            .child(html!("input" => HtmlInputElement, {
                .attr("id", &slider_id)
                .class("dwui-slider")
                .dwclass!("h-6 grow cursor-pointer dwui-focusable")
                .dwclass_signal!("cursor-not-allowed", disabled.signal())
                .attr("type", "range")
                .style_signal("--dwui-slider-fill", map_ref! {
                    let value = value.value_signal_cloned(),
                    let min = min.signal(),
                    let max = max.signal() => {
                        let value = value.parse::<f32>().unwrap_or(*min);
                        let range = (*max - *min).max(f32::EPSILON);
                        let fraction = ((value - *min) / range).clamp(0.0, 1.0);

                        format!("{}%", fraction * 100.0)
                    }
                })
                .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
                .attr_signal("value", value.value_signal_cloned())
                .attr_signal("min", min.signal().map(|v| v.to_string()))
                .attr_signal("max", max.signal().map(|v| v.to_string()))
                .attr_signal("step", step.map(|v| v.to_string()))
                .with_node!(slider_node => {
                    .future(value.value_signal_cloned().for_each(clone!(slider_node => move |v| {
                        slider_node.set_value(&v);
                        async move {}
                    })))
                    .event(clone!(value => move |_: events::Input| {
                        value.set(slider_node.value());
                    }))
                })
            }))
            .child(html!("input" => HtmlInputElement, {
                .dwclass!("w-16 h-6 text-center flex-none rounded-sm bg-transparent border-none text-base")
                .dwclass!("dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-900")
                .style("outline", "none")
                .attr_signal("aria-label", label.signal_ref(|label| {
                    if label.is_empty() {
                        "value".to_string()
                    } else {
                        format!("{} value", label)
                    }
                }))
                .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
                .attr_signal("min", min.signal().map(|v| v.to_string()))
                .attr_signal("max", max.signal().map(|v| v.to_string()))
                .attr("type", "number")
                .with_node!(element => {
                    .future(value.value_signal_cloned().for_each(clone!(element => move |v| {
                        element.set_value(&v);
                        async move {}
                    })))
                    .event(move |_: events::Input| {
                        value.set(element.value());
                    })
                })
            }))
        }))
        .child(field_error_row(is_valid.signal_cloned(), &slider_id))
        .apply_if(apply.is_some(),|b| b.apply(apply.unwrap()))
    })
}
