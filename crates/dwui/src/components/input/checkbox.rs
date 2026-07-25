use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, svg, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

/// An accessible checkbox.
///
/// Renders a `role="checkbox"` button with `aria-checked` state, an animated
/// checkmark, and an associated clickable label. Toggling (mouse, Enter, or
/// Space) invokes `on_change` with the new value; the displayed state follows
/// the `checked` signal, so the consumer stays in control of the value.
#[component(render_fn = checkbox)]
struct Checkbox {
    #[signal]
    #[default(false)]
    checked: bool,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default("".to_string())]
    label: String,

    #[default(Box::new(|_| {}))]
    on_change: dyn Fn(bool) + 'static,
}

pub fn checkbox(props: CheckboxProps) -> Dom {
    let CheckboxProps {
        checked,
        disabled,
        label,
        on_change,
        apply,
    } = props;

    let checked = checked.broadcast();
    let disabled = disabled.broadcast();
    let label = label.broadcast();

    let checked_state = Mutable::new(false);

    let checkbox_id = component_id("checkbox");

    html!("div", {
        .dwclass!("flex flex-row align-items-center gap-2")
        .future(checked.signal().for_each(clone!(checked_state => move |v| {
            checked_state.set(v);
            async {}
        })))
        .child(html!("button", {
            .attr("type", "button")
            .attr("role", "checkbox")
            .attr("id", &checkbox_id)
            .attr_signal("aria-checked", checked.signal().map(|v| if v { "true" } else { "false" }))
            .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
            .dwclass!("w-5 h-5 rounded-sm cursor-pointer transition-colors flex-none p-0")
            .dwclass!("flex align-items-center justify-center")
            .dwclass!("border dwui-border-void-500 is(.light *):dwui-border-void-300")
            .dwclass!("disabled:cursor-not-allowed disabled:opacity-60")
            .dwclass!("dwui-focusable")
            .dwclass_signal!("dwui-bg-primary-500 dwui-border-primary-500 is(.light *):dwui-bg-primary-400 is(.light *):dwui-border-primary-400", checked.signal())
            .dwclass_signal!("bg-transparent", checked.signal().map(|v| !v))
            .child(html!("span", {
                .dwclass!("flex align-items-center justify-center")
                .style("transition", "opacity 100ms ease-out")
                .style_signal("opacity", checked.signal().map(|v| if v { "1" } else { "0" }))
                .child(svg!("svg", {
                    .attr("viewBox", "0 0 12 12")
                    .attr("width", "12")
                    .attr("height", "12")
                    .attr("fill", "none")
                    .attr("aria-hidden", "true")
                    .child(svg!("path", {
                        .attr("d", "M2 6.5 L4.5 9 L10 3")
                        .attr("stroke", "white")
                        .attr("stroke-width", "2")
                        .attr("stroke-linecap", "round")
                        .attr("stroke-linejoin", "round")
                    }))
                }))
            }))
            .event(clone!(checked_state => move |_: events::Click| {
                (on_change)(!checked_state.get());
            }))
        }))
        .child_signal(label.signal_ref(move |label| {
            if label.is_empty() {
                return None;
            }

            Some(html!("label", {
                .attr("for", &checkbox_id)
                .dwclass!("cursor-pointer select-none")
                .dwclass!("dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-800")
                .text(label)
            }))
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
