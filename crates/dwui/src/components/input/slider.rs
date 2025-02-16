use std::rc::Rc;
use crate::mixins::labelled_rect_mixin::labelled_rect_mixin;
use crate::prelude::{InputValueWrapper, ValidationResult};
use crate::theme::prelude::*;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals::signal::{always, Mutable};
use futures_signals_component_macro::component;
use web_sys::HtmlInputElement;

#[component(render_fn=slider)]
struct Slider {
    #[default(Box::new(Mutable::new("".to_string())))]
    value: dyn InputValueWrapper + Send + 'static,

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
    label: String,
}

pub fn slider(props: SliderProps) -> Dom {
    let SliderProps {
        value,
        min,
        max,
        step,
        label,
        apply,
    } = props;

    let value = Rc::new(value);

    let min = min.broadcast();
    let max = max.broadcast();

    html!("div", {
        .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-300 text-base")
        .dwclass!("grid")
        .child(html!("div", {
            .dwclass!("flex flex-row grid-col-1 grid-row-1 w-full align-items-center")
            .child(html!("input" => HtmlInputElement, {
                .dwclass!("h-10 grow")
                .attr("type", "range")
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
                .dwclass!("w-16 text-center flex-none")
                .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-300 text-base")
                .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-900")
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
        .apply(labelled_rect_mixin(label, always(true), always(ValidationResult::Valid)))
        .apply_if(apply.is_some(),|b| b.apply(apply.unwrap()))
    })
}
