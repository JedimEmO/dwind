use crate::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[component(render_fn = glass_toggle)]
struct GlassToggle {
    #[default(Box::new(Mutable::new(false)))]
    checked: dyn GlassToggleValue + 'static,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default(None)]
    label: Option<String>,
}

pub fn glass_toggle(props: GlassToggleProps) -> Dom {
    let GlassToggleProps {
        checked,
        disabled,
        label,
        apply,
    } = props;

    let disabled = disabled.broadcast();
    let checked_signal = checked.get_signal().broadcast();
    let checked = std::rc::Rc::new(checked);
    let hover = Mutable::new(false);

    html!("label", {
        .dwclass!("inline-flex items-center gap-3 select-none")
        .style("transition-duration", "var(--glass-transition)")
        .style_signal("opacity", disabled.signal().map(|d| if d { "0.5" } else { "1" }))
        .style_signal("cursor", disabled.signal().map(|d| if d { "not-allowed" } else { "pointer" }))
        .style_signal("pointer-events", disabled.signal().map(|d| if d { "none" } else { "auto" }))

        .attr("role", "switch")
        .attr_signal("aria-checked", checked_signal.signal().map(|c| {
            if c { "true" } else { "false" }
        }))
        .attr_signal("aria-disabled", disabled.signal().map(|d| {
            if d { Some("true") } else { None }
        }))

        // Track
        .child(html!("div", {
            .dwclass!("relative")
            .style("width", "44px")
            .style("height", "24px")
            .style("border-radius", "var(--glass-border-radius-full)")
            .style("transition-duration", "var(--glass-transition)")
            .style("transition-timing-function", "var(--glass-ease)")
            .style("transition-property", "background, box-shadow")

            .style_signal("background", checked_signal.signal().map(|c| {
                if c { "var(--glass-accent)" } else { "var(--glass-bg-inset)" }
            }))
            .style("border", "none")
            .style_signal("box-shadow", {
                let hover = hover.clone();
                let disabled = disabled.clone();
                map_ref! {
                    let h = hover.signal(),
                    let d = disabled.signal() => {
                        if *h && !*d {
                            "var(--glass-shadow-inset), 0 0 0 1px rgba(255, 255, 255, 0.10)"
                        } else {
                            "var(--glass-shadow-inset)"
                        }
                    }
                }
            })

            .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
            .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))

            // Thumb
            .child(html!("div", {
                .style("position", "absolute")
                .style("top", "2px")
                .style("width", "18px")
                .style("height", "18px")
                .style("border-radius", "var(--glass-border-radius-full)")
                .style("background", "var(--glass-text-primary)")
                .style("transition-duration", "var(--glass-transition)")
                .style("transition-timing-function", "var(--glass-ease)")
                .style("transition-property", "transform")
                .style("box-shadow", "var(--glass-shadow-sm)")

                .style_signal("transform", checked_signal.signal().map(|c| {
                    if c { "translateX(22px)" } else { "translateX(2px)" }
                }))
            }))

            .event(move |_: events::Click| {
                checked.toggle();
            })
        }))

        // Label text
        .child_signal(label.map(|l| {
            l.map(|text| {
                html!("span", {
                    .dwclass!("glass-text-primary text-base")
                    .text(&text)
                })
            })
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
