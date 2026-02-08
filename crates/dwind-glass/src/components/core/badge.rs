use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BadgeVariant {
    Default,
    Success,
    Warning,
    Error,
    Info,
    Accent,
}

#[component(render_fn = glass_badge)]
struct GlassBadge {
    #[signal]
    #[default(None)]
    content: Option<Dom>,

    #[signal]
    #[default(BadgeVariant::Default)]
    variant: BadgeVariant,
}

pub fn glass_badge(props: GlassBadgeProps) -> Dom {
    let GlassBadgeProps {
        content,
        variant,
        apply,
    } = props;

    let variant = variant.broadcast();

    html!("span", {
        .dwclass!("inline-flex items-center h-6 px-2 text-xs font-medium select-none")
        .style("border-radius", "var(--glass-border-radius-full)")
        .style("transition-duration", "var(--glass-transition)")

        .style_signal("background-color", variant.signal().map(|v| match v {
            BadgeVariant::Default => "var(--glass-bg)",
            BadgeVariant::Success => "var(--glass-success-muted)",
            BadgeVariant::Warning => "var(--glass-warning-muted)",
            BadgeVariant::Error => "var(--glass-error-muted)",
            BadgeVariant::Info => "var(--glass-info-muted)",
            BadgeVariant::Accent => "var(--glass-accent-muted)",
        }))
        .style_signal("color", variant.signal().map(|v| match v {
            BadgeVariant::Default => "var(--glass-text-secondary)",
            BadgeVariant::Success => "var(--glass-success)",
            BadgeVariant::Warning => "var(--glass-warning)",
            BadgeVariant::Error => "var(--glass-error)",
            BadgeVariant::Info => "var(--glass-info)",
            BadgeVariant::Accent => "var(--glass-accent)",
        }))
        .style("border", "none")

        .child_signal(content)

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
