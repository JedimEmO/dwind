use crate::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonVariant {
    /// Accent-colored filled button.
    Filled,
    /// Transparent frosted glass with border.
    Glass,
    /// No background, text only.
    Ghost,
    /// Error-colored for destructive actions.
    Danger,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component(render_fn = glass_button)]
struct GlassButton {
    #[signal]
    #[default(None)]
    content: Option<Dom>,

    #[default(Box::new(|_: events::Click| {}))]
    on_click: dyn Fn(events::Click) -> () + 'static,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default(false)]
    loading: bool,

    #[signal]
    #[default(ButtonVariant::Glass)]
    variant: ButtonVariant,

    #[signal]
    #[default(ButtonSize::Medium)]
    size: ButtonSize,
}

pub fn glass_button(props: GlassButtonProps) -> Dom {
    let GlassButtonProps {
        content,
        on_click,
        disabled,
        loading,
        variant,
        size,
        apply,
    } = props;

    let variant = variant.broadcast();
    let size = size.broadcast();
    let disabled = disabled.broadcast();
    let loading = loading.broadcast();
    let hover = Mutable::new(false);
    let active = Mutable::new(false);

    html!("button", {
        // Base styles all buttons share
        .dwclass!("inline-flex items-center justify-center font-semibold cursor-pointer transition-all select-none")
        .dwclass!("glass-text-primary")
        .dwclass!("disabled:opacity-50 disabled:pointer-events-none disabled:cursor-not-allowed")

        // Size classes
        .dwclass_signal!("h-8 px-3 text-sm gap-1", size.signal().map(|s| s == ButtonSize::Small))
        .dwclass_signal!("h-10 px-4 text-base gap-2", size.signal().map(|s| s == ButtonSize::Medium))
        .dwclass_signal!("h-12 px-6 text-lg gap-2", size.signal().map(|s| s == ButtonSize::Large))

        // Border radius based on size
        .style_signal("border-radius", size.signal().map(|s| match s {
            ButtonSize::Small => "var(--glass-border-radius-sm)",
            ButtonSize::Medium => "var(--glass-border-radius)",
            ButtonSize::Large => "var(--glass-border-radius-lg)",
        }))

        // Variant-specific surfaces
        .style_signal("background", {
            let variant = variant.clone();
            let hover = hover.clone();
            let disabled = disabled.clone();
            map_ref! {
                let v = variant.signal(),
                let h = hover.signal(),
                let d = disabled.signal() => {
                    if *d {
                        match v {
                            ButtonVariant::Glass => "var(--glass-bg)",
                            ButtonVariant::Ghost => "transparent",
                            ButtonVariant::Filled => "var(--glass-accent)",
                            ButtonVariant::Danger => "var(--glass-error)",
                        }
                    } else if *h {
                        match v {
                            ButtonVariant::Glass => "rgba(255, 255, 255, 0.14)",
                            ButtonVariant::Ghost => "var(--glass-bg)",
                            ButtonVariant::Filled => "var(--glass-accent-hover)",
                            ButtonVariant::Danger => "rgba(248, 113, 113, 0.85)",
                        }
                    } else {
                        match v {
                            ButtonVariant::Glass => "var(--glass-bg)",
                            ButtonVariant::Ghost => "transparent",
                            ButtonVariant::Filled => "var(--glass-accent)",
                            ButtonVariant::Danger => "var(--glass-error)",
                        }
                    }
                }
            }
        })
        .style_signal(["backdrop-filter", "-webkit-backdrop-filter"], variant.signal().map(|v| match v {
            ButtonVariant::Glass => "blur(var(--glass-blur)) saturate(var(--glass-saturation))",
            _ => "none",
        }))
        .style("border", "none")
        .style_signal("box-shadow", variant.signal().map(|v| match v {
            ButtonVariant::Glass => "var(--glass-shadow-sm)",
            _ => "none",
        }))
        .style_signal("color", variant.signal().map(|v| match v {
            ButtonVariant::Filled | ButtonVariant::Danger => "var(--glass-text-on-accent)",
            _ => "var(--glass-text-primary)",
        }))

        // Active press transform
        .style_signal("transform", active.signal().map(|a| {
            if a { "scale(0.98)" } else { "scale(1)" }
        }))

        // Transitions
        .style("transition-duration", "var(--glass-transition)")
        .style("transition-timing-function", "var(--glass-ease)")

        // Focus ring for keyboard navigation
        .dwclass!("glass-focus-ring")

        // Disabled + loading
        .attr_signal("disabled", {
            let disabled = disabled.clone();
            let loading = loading.clone();
            futures_signals::map_ref! {
                let d = disabled.signal(),
                let l = loading.signal() => {
                    if *d || *l { Some("disabled") } else { None }
                }
            }
        })
        .attr_signal("aria-disabled", {
            let disabled = disabled.clone();
            let loading = loading.clone();
            futures_signals::map_ref! {
                let d = disabled.signal(),
                let l = loading.signal() => {
                    if *d || *l { Some("true") } else { None }
                }
            }
        })
        .attr_signal("aria-busy", loading.signal().map(|l| if l { Some("true") } else { None }))

        // Show spinner when loading, content otherwise
        .child_signal(loading.signal().map(|l| {
            if l {
                Some(html!("span", {
                    .dwclass!("inline-block w-4 h-4 border-2 rounded-full animate-spin")
                    .style("border-color", "var(--glass-text-primary)")
                    .style("border-top-color", "transparent")
                }))
            } else {
                None
            }
        }))
        .child_signal(content)

        .event(move |e: events::Click| {
            (on_click)(e);
        })
        .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
        .event(clone!(hover, active => move |_: events::MouseLeave| { hover.set(false); active.set(false); }))
        .event(clone!(active => move |_: events::MouseDown| { active.set(true); }))
        .event(clone!(active => move |_: events::MouseUp| { active.set(false); }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
