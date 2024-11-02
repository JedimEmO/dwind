use dominator::{clone, events, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{and, not, or, Mutable, Signal, SignalExt};
use futures_signals_component_macro::component;
use crate::components::input::validation::InputValueWrapper;
use dwind::prelude::*;
use crate::theme::prelude::*;

#[component(render_fn=text_input)]
struct TextInput<TValue: InputValueWrapper + 'static = Mutable<String>> {
    #[default(Mutable::new("".to_string()))]
    value: TValue,

    #[signal]
    #[default("".to_string())]
    label: String
}

pub fn text_input(props: impl TextInputPropsTrait + 'static) -> Dom {
    let TextInputProps { value, label, apply } = props.take();

    let label = label.broadcast();

    let has_label = label.signal_ref(|v| {
        v.len() > 0
    });

    let has_value = value.value_signal_cloned().map(|v| {
        v.len() > 0
    });

    let is_focused = Mutable::new(false);

    let raise_label = and(has_label, or(is_focused.signal(), has_value)).broadcast();

    let top_border_margin_signal = map_ref! {
        let raise = raise_label.signal(),
        let label = label.signal_cloned() => {
            if !raise {
                "0px".to_string()
            } else {
                format!("{}px", label.len() as f32  * 8.)
            }
        }
    };

    html!("div", {
        .dwclass!("grid")
        .children([
            html!("input", {
                .dwclass!("h-12 p-l-2")
                .dwclass!("grid-col-1 grid-row-1")
                .attr_signal("value", value.value_signal_cloned())
                .event(clone!(is_focused => move |_: events::FocusOut| {
                    is_focused.set(false);
                }))
                .event(clone!(is_focused => move |_: events::Focus| {
                    is_focused.set(true);
                }))
            }),
            html!("label", {
                .dwclass!("grid-col-1 grid-row-1 pointer-events-none transition-all m-l-4 dwui-text-on-primary-300")
                .dwclass_signal!("text-xs", raise_label.signal())
                .dwclass_signal!("text-sm", not(raise_label.signal()))
                .style_signal("margin-top", raise_label.signal().map(|v| {
                    if v {
                        "-8px"
                    } else {
                        "12px"
                    }
                }))
                .text_signal(label.signal_cloned())
            }),
            html!("div", {
                .dwclass!("grid-col-1 grid-row-1 pointer-events-none")
                .dwclass!("border-l border-r border-b border-dwui-void-600")
            })
        ])
        .child(html!("div", {
            .dwclass!("grid-col-1 grid-row-1 pointer-events-none w-2")
            .dwclass!("border-t border-dwui-void-600")
        }))
        .child(html!("div", {
            .dwclass!("grid-col-1 grid-row-1 pointer-events-none transition-all")
            .dwclass!("border-t border-dwui-void-600")
            .style_signal("margin-left", top_border_margin_signal)
        }))
    })
}