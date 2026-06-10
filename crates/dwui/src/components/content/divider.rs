use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

/// A horizontal rule with an optional centered label.
#[component(render_fn = divider)]
struct Divider {
    #[signal]
    #[default("".to_string())]
    label: String,
}

pub fn divider(props: DividerProps) -> Dom {
    let DividerProps { label, apply } = props;

    let label = label.broadcast();

    html!("div", {
        .attr("role", "separator")
        .attr("aria-orientation", "horizontal")
        .dwclass!("flex flex-row align-items-center gap-3 w-full")
        .child(html!("div", {
            .dwclass!("grow border-t dwui-border-void-700 is(.light *):dwui-border-void-300")
        }))
        .child_signal(label.signal_cloned().map(|label| {
            if label.is_empty() {
                return None;
            }

            Some(html!("span", {
                .dwclass!("text-sm flex-none select-none")
                .dwclass!("dwui-text-on-primary-400 is(.light *):dwui-text-on-primary-600")
                .text(&label)
            }))
        }))
        .child_signal(label.signal_ref(|label| {
            if label.is_empty() {
                return None;
            }

            Some(html!("div", {
                .dwclass!("grow border-t dwui-border-void-700 is(.light *):dwui-border-void-300")
            }))
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
