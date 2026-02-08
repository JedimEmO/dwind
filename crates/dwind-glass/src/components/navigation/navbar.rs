use crate::mixins::glass_surface::{glass_surface_mixin, GlassSurfaceLevel};
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[component(render_fn = glass_navbar)]
struct GlassNavbar {
    #[signal]
    #[default(None)]
    brand: Option<Dom>,

    #[signal_vec]
    #[default(vec![])]
    items: Dom,

    #[signal]
    #[default(None)]
    trailing: Option<Dom>,

    #[signal]
    #[default(true)]
    sticky: bool,
}

pub fn glass_navbar(props: GlassNavbarProps) -> Dom {
    let GlassNavbarProps {
        brand,
        items,
        trailing,
        sticky,
        apply,
    } = props;

    let sticky = sticky.broadcast();

    html!("nav", {
        .dwclass!("w-full flex items-center px-6 h-16")
        .apply(glass_surface_mixin(GlassSurfaceLevel::Base))
        .style("border-bottom", "var(--glass-border-width) solid var(--glass-border-color)")
        .style("z-index", "40")

        .style_signal("position", sticky.signal().map(|s| if s { "sticky" } else { "relative" }))
        .style_signal("top", sticky.signal().map(|s| if s { "0" } else { "auto" }))

        // Brand
        .child_signal(brand.map(|b| {
            b.map(|brand_dom| {
                html!("div", {
                    .dwclass!("flex items-center mr-6")
                    .child(brand_dom)
                })
            })
        }))

        // Items
        .child(html!("div", {
            .dwclass!("flex items-center gap-2 flex-1")
            .children_signal_vec(items)
        }))

        // Trailing
        .child_signal(trailing.map(|t| {
            t.map(|trailing_dom| {
                html!("div", {
                    .dwclass!("flex items-center ml-6")
                    .child(trailing_dom)
                })
            })
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
