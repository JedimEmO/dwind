use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[component(render_fn = glass_nav_item)]
struct GlassNavItem {
    #[default("".to_string())]
    label: String,

    #[default(None)]
    icon: Option<Dom>,

    #[default(None)]
    badge: Option<Dom>,

    #[signal]
    #[default(false)]
    active: bool,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[default(Box::new(|| {}))]
    on_click: dyn Fn() -> () + 'static,

    #[signal]
    #[default(false)]
    collapsed: bool,
}

pub fn glass_nav_item(props: GlassNavItemProps) -> Dom {
    let GlassNavItemProps {
        label,
        icon,
        badge,
        active,
        disabled,
        on_click,
        collapsed,
        apply,
    } = props;

    let active = active.broadcast();
    let disabled = disabled.broadcast();
    let collapsed = collapsed.broadcast();
    let hover = Mutable::new(false);

    html!("div", {
        .dwclass!("flex items-center cursor-pointer transition-all select-none")
        .style("border-radius", "var(--glass-border-radius)")
        .style("transition-duration", "var(--glass-transition)")
        .style("transition-timing-function", "var(--glass-ease)")
        .style("transition-property", "background, color, padding, gap, opacity")
        .style("overflow", "hidden")
        .style("white-space", "nowrap")
        .attr("role", "menuitem")

        // Padding: equal when collapsed to center icon, normal when expanded
        .style_signal("padding", collapsed.signal().map(|c| {
            if c { "8px" } else { "8px 12px" }
        }))

        .style_signal("gap", collapsed.signal().map(|c| {
            if c { "0px" } else { "12px" }
        }))

        .style_signal("justify-content", collapsed.signal().map(|c| {
            if c { "center" } else { "flex-start" }
        }))

        // Background
        .style_signal("background", {
            let active = active.clone();
            let hover = hover.clone();
            let disabled = disabled.clone();
            map_ref! {
                let a = active.signal(),
                let h = hover.signal(),
                let d = disabled.signal() => {
                    if *d {
                        "transparent"
                    } else if *a {
                        "var(--glass-accent-muted)"
                    } else if *h {
                        "rgba(255, 255, 255, 0.06)"
                    } else {
                        "transparent"
                    }
                }
            }
        })

        // Text color
        .style_signal("color", {
            let active = active.clone();
            let hover = hover.clone();
            let disabled = disabled.clone();
            map_ref! {
                let a = active.signal(),
                let h = hover.signal(),
                let d = disabled.signal() => {
                    if *d {
                        "var(--glass-text-tertiary)"
                    } else if *a {
                        "var(--glass-accent)"
                    } else if *h {
                        "var(--glass-text-primary)"
                    } else {
                        "var(--glass-text-secondary)"
                    }
                }
            }
        })

        // Disabled state
        .style_signal("opacity", disabled.signal().map(|d| if d { "0.5" } else { "1" }))
        .style_signal("pointer-events", disabled.signal().map(|d| if d { "none" } else { "auto" }))

        // Icon (always visible, even when collapsed)
        .apply_if(icon.is_some(), |b| {
            b.child(html!("span", {
                .dwclass!("flex items-center justify-center")
                .style("min-width", "20px")
                .style("font-size", "16px")
                .style("flex-shrink", "0")
                .child(icon.unwrap())
            }))
        })

        // Label (fades out when collapsed)
        .child(html!("span", {
            .dwclass!("text-sm font-medium")
            .style("transition-property", "opacity, max-width")
            .style("transition-duration", "var(--glass-transition)")
            .style("transition-timing-function", "var(--glass-ease)")
            .style("overflow", "hidden")
            .style_signal("opacity", collapsed.signal().map(|c| if c { "0" } else { "1" }))
            .style_signal("max-width", collapsed.signal().map(|c| if c { "0px" } else { "200px" }))
            .text(&label)
        }))

        // Badge (only visible when expanded)
        .apply_if(badge.is_some(), {
            let collapsed = collapsed.clone();
            move |b| {
                b.child(html!("span", {
                    .dwclass!("ml-auto")
                    .style("transition-property", "opacity, max-width")
                    .style("transition-duration", "var(--glass-transition)")
                    .style("transition-timing-function", "var(--glass-ease)")
                    .style("overflow", "hidden")
                    .style("flex-shrink", "0")
                    .style_signal("opacity", collapsed.signal().map(|c| if c { "0" } else { "1" }))
                    .style_signal("max-width", collapsed.signal().map(|c| if c { "0px" } else { "100px" }))
                    .child(badge.unwrap())
                }))
            }
        })

        // Events
        .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
        .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))
        .event(move |_: events::Click| {
            (on_click)();
        })

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
