use crate::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[derive(Clone, PartialEq)]
pub enum StatTrend {
    Up(f64),
    Down(f64),
    Neutral,
}

#[component(render_fn = glass_stat_card)]
struct GlassStatCard {
    #[signal]
    #[default("".to_string())]
    label: String,

    #[signal]
    #[default("".to_string())]
    value: String,

    #[signal]
    #[default(None)]
    trend: Option<StatTrend>,

    #[signal]
    #[default(None)]
    icon: Option<Dom>,
}

pub fn glass_stat_card(props: GlassStatCardProps) -> Dom {
    let GlassStatCardProps {
        label,
        value,
        trend,
        icon,
        apply,
    } = props;

    let hover = Mutable::new(false);

    html!("div", {
        .dwclass!("flex flex-col gap-2")
        .style("background", "var(--glass-bg-elevated)")
        .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur)) saturate(var(--glass-saturation))")
        .style("border", "none")
        .style("border-radius", "var(--glass-border-radius-xl)")
        .style("transition-property", "box-shadow")
        .style("transition-duration", "var(--glass-transition)")
        .style("transition-timing-function", "var(--glass-ease)")
        .style_signal("box-shadow", hover.signal().map(|h| {
            if h {
                "var(--glass-shadow-lg), 0 0 0 1px rgba(255, 255, 255, 0.08)"
            } else {
                "var(--glass-shadow)"
            }
        }))
        .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
        .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))
        .style("padding", "1.5rem")

        // Header: label + icon
        .child(html!("div", {
            .dwclass!("flex items-center justify-between")
            .child_signal(label.map(|l| {
                Some(html!("span", {
                    .dwclass!("text-sm glass-text-secondary")
                    .text(&l)
                }))
            }))
            .child_signal(icon.map(|i| i))
        }))

        // Value
        .child_signal(value.map(|v| {
            Some(html!("div", {
                .dwclass!("text-2xl font-bold glass-text-primary")
                .text(&v)
            }))
        }))

        // Trend
        .child_signal(trend.map(|t| {
            t.map(|trend| {
                let (symbol, color, pct) = match &trend {
                    StatTrend::Up(p) => ("\u{2191}", "var(--glass-success)", *p),
                    StatTrend::Down(p) => ("\u{2193}", "var(--glass-error)", *p),
                    StatTrend::Neutral => ("\u{2192}", "var(--glass-text-tertiary)", 0.0),
                };
                html!("div", {
                    .dwclass!("flex items-center gap-1 text-sm")
                    .style("color", color)
                    .child(html!("span", {
                        .text(symbol)
                    }))
                    .child(html!("span", {
                        .text(&format!("{:.1}%", pct))
                    }))
                })
            })
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
