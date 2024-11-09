use dominator::{events, html, with_node, Dom};
use futures_signals::signal::{always, Mutable, SignalExt};
use futures_signals_component_macro::component;
use crate::prelude::{InputValueWrapper, ValidationResult};
use dwind::prelude::*;
use crate::mixins::labelled_rect_mixin::labelled_rect_mixin;
use futures_signals::signal_vec::SignalVecExt;
use web_sys::HtmlSelectElement;
use crate::theme::prelude::*;

#[component(render_function=select)]
struct Select<TValue: InputValueWrapper + 'static = Mutable<String>> {
    #[default(Mutable::new("".to_string()))]
    value: TValue,

    #[signal_vec]
    #[default(vec![])]
    options: (String, String),

    #[signal]
    #[default("".to_string())]
    label: String,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult
}

pub fn select(props: impl SelectPropsTrait + 'static) -> Dom {
    let SelectProps { value, options, label, is_valid, apply } = props.take();
    let value_signal = value.value_signal_cloned().broadcast();

    html!("div", {
        .dwclass!("grid h-12")
        .children([
            html!("select" => HtmlSelectElement, {
                .dwclass!("dwui-bg-void-950 is(.light *):dwui-bg-void-300 text-base h-12")
                .dwclass!("grid-row-1 grid-col-1")
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
        .apply(labelled_rect_mixin(label, always(true), is_valid))
    })
}