use crate::mixins::field_mixin::{field_error_row, field_surface_mixin};
use crate::prelude::{InputValueWrapper, ValidationResult};
use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{events, html, svg, with_node, Dom};
use dwind::prelude::*;
use futures_signals::signal::{always, Mutable, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use web_sys::HtmlSelectElement;

#[component(render_fn = select)]
struct Select {
    #[default(Box::new(Mutable::new("".to_string())))]
    value: dyn InputValueWrapper + 'static,

    #[signal_vec]
    #[default(vec![])]
    options: (String, String),

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

pub fn select(props: SelectProps) -> Dom {
    let SelectProps {
        value,
        options,
        label,
        disabled,
        is_valid,
        apply,
    } = props;
    let value_signal = value.value_signal_cloned().broadcast();

    let is_valid = is_valid.broadcast();
    let disabled = disabled.broadcast();

    let select_id = component_id("select");
    let error_id = format!("{}-error", select_id);

    let is_valid_bool = is_valid
        .signal_ref(|validation| validation.is_valid())
        .broadcast();

    html!("div", {
        .dwclass!("flex flex-col w-full")
        .child(html!("div", {
            .dwclass!("h-10 flex-none")
            .dwclass_signal!("opacity-60", disabled.signal())
            .apply(field_surface_mixin(
                label,
                always(true),
                is_valid.signal_cloned(),
                select_id.clone(),
            ))
            .child(html!("select" => HtmlSelectElement, {
                .attr("id", &select_id)
                .class("dwui-select")
                .dwclass!("w-full h-full bg-transparent border-none text-base p-l-3 p-r-8 p-t-3 cursor-pointer")
                .dwclass!("dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-900")
                .dwclass_signal!("cursor-not-allowed", disabled.signal())
                .style("outline", "none")
                // The themed chevron beside this select replaces the native one
                .style("appearance", "none")
                .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
                .attr_signal("aria-invalid", is_valid_bool.signal().map(|valid| if valid { None } else { Some("true") }))
                .attr_signal("aria-describedby", is_valid_bool.signal().map({
                    let error_id = error_id.clone();
                    move |valid| {
                        if valid {
                            None
                        } else {
                            Some(error_id.clone())
                        }
                    }
                }))
                .children_signal_vec(options.map(move |(key, value)| {
                    html!("option", {
                        .attr("value", &key)
                        .attr_signal("selected", value_signal.signal_cloned().map(move |v|{
                            if  key == v {
                                Some("selected")
                            } else {
                                None
                            }
                        }))
                        .text(&value)
                    })
                }))
                .with_node!(node => {
                    .event(move |_: events::Change| {
                        value.set(node.value());
                    })
                })
            }))
            .child(html!("span", {
                .attr("aria-hidden", "true")
                .dwclass!("absolute pointer-events-none flex align-items-center")
                .dwclass!("dwui-text-on-primary-400 is(.light *):dwui-text-on-primary-600")
                .style("right", "0.75rem")
                .style("top", "50%")
                .style("transform", "translateY(-50%)")
                .child(svg!("svg", {
                    .attr("viewBox", "0 0 12 12")
                    .attr("width", "12")
                    .attr("height", "12")
                    .attr("fill", "none")
                    .child(svg!("path", {
                        .attr("d", "M2 4 L6 8 L10 4")
                        .attr("stroke", "currentColor")
                        .attr("stroke-width", "1.5")
                        .attr("stroke-linecap", "round")
                        .attr("stroke-linejoin", "round")
                    }))
                }))
            }))
        }))
        .child(field_error_row(is_valid.signal_cloned(), &select_id))
        .apply_if(apply.is_some(),|b| b.apply(apply.unwrap()))
    })
}
