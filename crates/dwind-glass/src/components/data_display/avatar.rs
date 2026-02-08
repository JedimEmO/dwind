use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
}

#[component(render_fn = glass_avatar)]
struct GlassAvatar {
    #[signal]
    #[default(None)]
    src: Option<String>,

    #[signal]
    #[default("?".to_string())]
    fallback: String,

    #[signal]
    #[default(AvatarSize::Medium)]
    size: AvatarSize,
}

pub fn glass_avatar(props: GlassAvatarProps) -> Dom {
    let GlassAvatarProps {
        src,
        fallback,
        size,
        apply,
    } = props;

    let size = size.broadcast();

    html!("div", {
        .dwclass!("inline-flex items-center justify-center overflow-hidden select-none")
        .style("border-radius", "var(--glass-border-radius-full)")
        .style("border", "none")
        .style("box-shadow", "inset 0 0 0 1px rgba(255, 255, 255, 0.06)")
        .style("background", "var(--glass-accent-muted)")
        .style("flex-shrink", "0")

        .style_signal("width", size.signal().map(|s| match s {
            AvatarSize::Small => "32px",
            AvatarSize::Medium => "40px",
            AvatarSize::Large => "56px",
        }))
        .style_signal("height", size.signal().map(|s| match s {
            AvatarSize::Small => "32px",
            AvatarSize::Medium => "40px",
            AvatarSize::Large => "56px",
        }))
        .style_signal("font-size", size.signal().map(|s| match s {
            AvatarSize::Small => "0.75rem",
            AvatarSize::Medium => "0.875rem",
            AvatarSize::Large => "1.125rem",
        }))

        .child_signal(futures_signals::map_ref! {
            let src = src,
            let fallback = fallback => {
                if let Some(url) = src {
                    Some(html!("img", {
                        .dwclass!("w-full h-full")
                        .style("object-fit", "cover")
                        .attr("src", url)
                        .attr("alt", "")
                    }))
                } else {
                    Some(html!("span", {
                        .dwclass!("font-semibold")
                        .style("color", "var(--glass-accent)")
                        .text(fallback)
                    }))
                }
            }
        })

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
