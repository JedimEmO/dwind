use crate::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CardVariant {
    /// Standard elevated glass card.
    Default,
    /// More prominent elevation with heavier blur.
    Elevated,
    /// Recessed/inset appearance.
    Inset,
}

#[component(render_fn = glass_card)]
struct GlassCard {
    #[signal]
    #[default(None)]
    content: Option<Dom>,

    #[signal]
    #[default(None)]
    header: Option<Dom>,

    #[signal]
    #[default(None)]
    footer: Option<Dom>,

    #[signal]
    #[default(CardVariant::Default)]
    variant: CardVariant,
}

pub fn glass_card(props: GlassCardProps) -> Dom {
    let GlassCardProps {
        content,
        header,
        footer,
        variant,
        apply,
    } = props;

    let variant_val = variant.broadcast();
    let hover = Mutable::new(false);

    html!("div", {
        .dwclass!("w-full glass-text-primary")
        .style("overflow", "hidden")

        // Glass surface: gradient simulates overhead light hitting glass
        .style_signal("background", variant_val.signal().map(|v| match v {
            CardVariant::Default => "\
                linear-gradient(to bottom, rgba(255, 255, 255, 0.06) 0%, transparent 50%), \
                var(--glass-bg-elevated)",
            CardVariant::Elevated => "\
                linear-gradient(to bottom, rgba(255, 255, 255, 0.09) 0%, transparent 40%), \
                var(--glass-bg-elevated)",
            CardVariant::Inset => "var(--glass-bg-inset)",
        }))
        .style_signal(["backdrop-filter", "-webkit-backdrop-filter"], variant_val.signal().map(|v| {
            match v {
                CardVariant::Elevated => "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))",
                CardVariant::Default => "blur(var(--glass-blur)) saturate(var(--glass-saturation))",
                CardVariant::Inset => "blur(var(--glass-blur)) saturate(var(--glass-saturation))",
            }
        }))
        .style("border", "none")
        .style("border-radius", "var(--glass-border-radius-xl)")
        .style("transition-property", "box-shadow")
        .style("transition-duration", "var(--glass-transition)")
        .style("transition-timing-function", "var(--glass-ease)")
        .style_signal("box-shadow", {
            let variant_val = variant_val.clone();
            let hover = hover.clone();
            map_ref! {
                let v = variant_val.signal(),
                let h = hover.signal() => {
                    match (v, h) {
                        (CardVariant::Default, false) => "var(--glass-shadow)",
                        (CardVariant::Default, true) => "var(--glass-shadow-lg), 0 0 0 1px rgba(255, 255, 255, 0.08)",
                        (CardVariant::Elevated, false) => "var(--glass-shadow-lg)",
                        (CardVariant::Elevated, true) => "var(--glass-shadow-xl), 0 0 0 1px rgba(255, 255, 255, 0.08)",
                        (CardVariant::Inset, _) => "var(--glass-shadow-inset)",
                    }
                }
            }
        })

        // Header (optional) — slightly brighter glass tint for visual hierarchy
        .child_signal(header.map(|h| {
            h.map(|header_dom| {
                html!("div", {
                    .dwclass!("px-6 py-4")
                    .style("background", "rgba(255, 255, 255, 0.04)")
                    .style("border-bottom", "1px solid rgba(255, 255, 255, 0.06)")
                    .child(header_dom)
                })
            })
        }))

        // Body content
        .child_signal(content.map(|c| {
            c.map(|content_dom| {
                html!("div", {
                    .dwclass!("p-6")
                    .child(content_dom)
                })
            })
        }))

        // Footer (optional)
        .child_signal(footer.map(|f| {
            f.map(|footer_dom| {
                html!("div", {
                    .dwclass!("px-6 py-4")
                    .style("border-top", "var(--glass-border-width) solid var(--glass-border-color)")
                    .child(footer_dom)
                })
            })
        }))

        .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
        .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
