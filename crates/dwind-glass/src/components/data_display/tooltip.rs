use crate::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TooltipPosition {
    Top,
    Right,
    Bottom,
    Left,
}

#[component(render_fn = glass_tooltip)]
struct GlassTooltip {
    #[default(None)]
    content: Option<Dom>,

    #[default(None)]
    trigger: Option<Dom>,

    #[signal]
    #[default(TooltipPosition::Top)]
    position: TooltipPosition,
}

pub fn glass_tooltip(props: GlassTooltipProps) -> Dom {
    let GlassTooltipProps {
        content,
        trigger,
        position: _,
        apply,
    } = props;

    let is_visible = Mutable::new(false);

    html!("div", {
        .dwclass!("relative inline-flex")

        .event({
            let is_visible = is_visible.clone();
            move |_: dominator::events::MouseEnter| {
                is_visible.set(true);
            }
        })
        .event({
            let is_visible = is_visible.clone();
            move |_: dominator::events::MouseLeave| {
                is_visible.set(false);
            }
        })

        // Trigger
        .apply_if(trigger.is_some(), |b| b.child(trigger.unwrap()))

        // Tooltip content — always in DOM, fades in/out
        .apply_if(content.is_some(), move |b| {
            b.child(html!("div", {
                .dwclass!("absolute pointer-events-none text-sm glass-text-primary")
                .style("background", "var(--glass-bg-elevated)")
                .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
                .style("border", "none")
                .style("border-radius", "var(--glass-border-radius)")
                .style("box-shadow", "var(--glass-shadow-lg)")
                .style("padding", "6px 12px")
                .style("white-space", "nowrap")
                .style("z-index", "50")
                .style("transition-property", "opacity, transform")
                .style("transition-duration", "var(--glass-transition-fast)")
                .style("transition-timing-function", "var(--glass-ease)")

                // Default to top position
                .style("bottom", "calc(100% + 8px)")
                .style("left", "50%")

                .style_signal("opacity", is_visible.signal().map(|v| if v { "1" } else { "0" }))
                .style_signal("visibility", is_visible.signal().map(|v| if v { "visible" } else { "hidden" }))
                .style_signal("transform", is_visible.signal().map(|v| {
                    if v { "translateX(-50%)" } else { "translateX(-50%) translateY(4px)" }
                }))

                .child(content.unwrap())
            }))
        })

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}

/// Simple text tooltip helper.
pub fn glass_text_tooltip(trigger: Dom, text: &str, position: TooltipPosition) -> Dom {
    let is_visible = Mutable::new(false);

    let text = text.to_string();

    html!("div", {
        .dwclass!("relative inline-flex")

        .event({
            let is_visible = is_visible.clone();
            move |_: dominator::events::MouseEnter| {
                is_visible.set(true);
            }
        })
        .event({
            let is_visible = is_visible.clone();
            move |_: dominator::events::MouseLeave| {
                is_visible.set(false);
            }
        })

        .child(trigger)

        // Tooltip — always in DOM, fades in/out
        .child(html!("div", {
            .dwclass!("absolute pointer-events-none text-sm glass-text-primary")
            .style("background", "var(--glass-bg-elevated)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
            .style("border", "none")
            .style("border-radius", "var(--glass-border-radius)")
            .style("box-shadow", "var(--glass-shadow-lg)")
            .style("padding", "6px 12px")
            .style("white-space", "nowrap")
            .style("z-index", "50")
            .style("transition-property", "opacity, transform")
            .style("transition-duration", "var(--glass-transition-fast)")
            .style("transition-timing-function", "var(--glass-ease)")

            .style_signal("opacity", is_visible.signal().map(|v| if v { "1" } else { "0" }))
            .style_signal("visibility", is_visible.signal().map(|v| if v { "visible" } else { "hidden" }))

            // Position + animated transform
            .apply(move |b| {
                match position {
                    TooltipPosition::Top => b
                        .style("bottom", "calc(100% + 8px)")
                        .style("left", "50%")
                        .style_signal("transform", is_visible.signal().map(|v| {
                            if v { "translateX(-50%)" } else { "translateX(-50%) translateY(4px)" }
                        })),
                    TooltipPosition::Bottom => b
                        .style("top", "calc(100% + 8px)")
                        .style("left", "50%")
                        .style_signal("transform", is_visible.signal().map(|v| {
                            if v { "translateX(-50%)" } else { "translateX(-50%) translateY(-4px)" }
                        })),
                    TooltipPosition::Left => b
                        .style("right", "calc(100% + 8px)")
                        .style("top", "50%")
                        .style_signal("transform", is_visible.signal().map(|v| {
                            if v { "translateY(-50%)" } else { "translateY(-50%) translateX(4px)" }
                        })),
                    TooltipPosition::Right => b
                        .style("left", "calc(100% + 8px)")
                        .style("top", "50%")
                        .style_signal("transform", is_visible.signal().map(|v| {
                            if v { "translateY(-50%)" } else { "translateY(-50%) translateX(-4px)" }
                        })),
                }
            })

            .text(&text)
        }))
    })
}
