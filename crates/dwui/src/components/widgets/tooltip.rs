use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

/// A hover/focus tooltip.
///
/// Wraps the `anchor` content; the tooltip text appears on mouse hover or
/// keyboard focus (focus-within), is exposed via `role="tooltip"`, and is
/// linked from the wrapper with `aria-describedby`.
#[component(render_fn = tooltip)]
struct Tooltip {
    #[signal]
    #[default(None)]
    anchor: Option<Dom>,

    #[signal]
    #[default("".to_string())]
    text: String,

    #[signal]
    #[default(TooltipPosition::Top)]
    position: TooltipPosition,
}

pub fn tooltip(props: TooltipProps) -> Dom {
    let TooltipProps {
        anchor,
        text,
        position,
        apply,
    } = props;

    let position = position.broadcast();
    let visible = Mutable::new(false);

    let tooltip_id = component_id("tooltip");

    html!("span", {
        .attr("aria-describedby", &tooltip_id)
        .dwclass!("inline-block")
        .style("position", "relative")
        .event(clone!(visible => move |_: events::MouseEnter| {
            visible.set(true);
        }))
        .event(clone!(visible => move |_: events::MouseLeave| {
            visible.set(false);
        }))
        .event(clone!(visible => move |_: events::FocusIn| {
            visible.set(true);
        }))
        .event(clone!(visible => move |_: events::FocusOut| {
            visible.set(false);
        }))
        .child_signal(anchor)
        .child(html!("span", {
            .attr("id", &tooltip_id)
            .attr("role", "tooltip")
            .dwclass!("text-sm rounded-md p-l-2 p-r-2 p-t-1 p-b-1 shadow-lg pointer-events-none")
            .dwclass!("dwui-bg-void-700 dwui-text-on-primary-50")
            .dwclass!("is(.light *):dwui-bg-void-800 is(.light *):dwui-text-on-primary-50")
            .style("position", "absolute")
            .style("z-index", crate::theme::layers::TOOLTIP)
            .style("width", "max-content")
            .style("max-width", "16rem")
            .style("transition", "opacity 100ms ease-out")
            .style_signal("opacity", visible.signal().map(|v| if v { "1" } else { "0" }))
            .style_signal("top", position.signal().map(|p| match p {
                TooltipPosition::Top => Some("auto"),
                TooltipPosition::Bottom => Some("calc(100% + 0.375rem)"),
                TooltipPosition::Left | TooltipPosition::Right => Some("50%"),
            }))
            .style_signal("bottom", position.signal().map(|p| match p {
                TooltipPosition::Top => Some("calc(100% + 0.375rem)"),
                _ => None,
            }))
            .style_signal("left", position.signal().map(|p| match p {
                TooltipPosition::Top | TooltipPosition::Bottom => Some("50%"),
                TooltipPosition::Right => Some("calc(100% + 0.375rem)"),
                TooltipPosition::Left => None,
            }))
            .style_signal("right", position.signal().map(|p| match p {
                TooltipPosition::Left => Some("calc(100% + 0.375rem)"),
                _ => None,
            }))
            .style_signal("transform", position.signal().map(|p| match p {
                TooltipPosition::Top | TooltipPosition::Bottom => "translateX(-50%)",
                TooltipPosition::Left | TooltipPosition::Right => "translateY(-50%)",
            }))
            .text_signal(text)
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
