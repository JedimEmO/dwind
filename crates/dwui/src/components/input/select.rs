use crate::mixins::labelled_rect_mixin::labelled_rect_mixin;
use crate::prelude::{InputValueWrapper, ValidationResult};
use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::signal::{always, Mutable, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use web_sys::HtmlSelectElement;

#[component(render_function=select)]
struct Select {
    #[default(Box::new(Mutable::new("".to_string())))]
    value: dyn InputValueWrapper + 'static,

    #[signal_vec]
    #[default(vec![])]
    options: (String, String),

    #[signal]
    #[default("".to_string())]
    label: String,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,
}

pub fn select(props: SelectProps) -> Dom {
    let SelectProps {
        value,
        options,
        label,
        is_valid,
        apply,
    } = props;
    let value_signal = value.value_signal_cloned().broadcast();

    let is_valid = is_valid.broadcast();

    let select_id = component_id("select");
    let error_id = format!("{}-error", select_id);

    let is_valid_bool = is_valid
        .signal_ref(|validation| validation.is_valid())
        .broadcast();

    html!("div", {
        .dwclass!("grid h-10")
        .children([
            html!("select" => HtmlSelectElement, {
                .attr("id", &select_id)
                .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-300 is(.light *):dwui-text-on-primary-800 text-base h-10 p-l-2")
                .dwclass!("dwui-text-on-primary-300")
                .dwclass!("grid-row-1 grid-col-1 cursor-pointer rounded-t-sm transition-all")
                .dwclass!("dwui-focusable")
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
            })
        ])
        .apply(labelled_rect_mixin(label, always(true), is_valid.signal_cloned(), select_id))
        .apply_if(apply.is_some(),|b| b.apply(apply.unwrap()))
    })
}
