use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

/// An accessible toggle switch.
///
/// Renders a `role="switch"` button with `aria-checked` state and an
/// associated clickable label. Toggling (mouse, Enter, or Space) invokes
/// `on_change` with the new value; the displayed state follows the
/// `checked` signal, so the consumer stays in control of the value.
#[component(render_fn = switch)]
struct Switch {
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

pub fn switch(props: SwitchProps) -> Dom {
    let SwitchProps {
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

    let switch_id = component_id("switch");

    html!("div", {
        .dwclass!("flex flex-row align-items-center gap-2")
        .future(checked.signal().for_each(clone!(checked_state => move |v| {
            checked_state.set(v);
            async {}
        })))
        .child(html!("button", {
            .attr("type", "button")
            .attr("role", "switch")
            .attr("id", &switch_id)
            .attr_signal("aria-checked", checked.signal().map(|v| if v { "true" } else { "false" }))
            .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
            .dwclass!("w-11 h-6 rounded-full cursor-pointer transition-colors flex-none border-none p-0")
            .dwclass!("disabled:cursor-not-allowed disabled:opacity-60")
            .dwclass!("focus-visible:ring-2 focus-visible:dwui-ring-primary-400 is(.light *):focus-visible:dwui-ring-primary-600")
            .dwclass_signal!("dwui-bg-primary-500 is(.light *):dwui-bg-primary-400", checked.signal())
            .dwclass_signal!("dwui-bg-void-600 is(.light *):dwui-bg-void-300", checked.signal().map(|v| !v))
            .style("outline", "none")
            .style("position", "relative")
            .child(html!("span", {
                .dwclass!("w-5 h-5 rounded-full bg-white block shadow-md")
                .style("position", "absolute")
                .style("top", "2px")
                .style("transition", "left 150ms ease-out")
                .style_signal("left", checked.signal().map(|v| if v { "22px" } else { "2px" }))
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
                .attr("for", &switch_id)
                .dwclass!("cursor-pointer select-none")
                .dwclass!("dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-800")
                .text(label)
            }))
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
