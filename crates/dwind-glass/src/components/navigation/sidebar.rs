use crate::mixins::glass_surface::{glass_surface_mixin, GlassSurfaceLevel};
use crate::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[component(render_fn = glass_sidebar)]
struct GlassSidebar {
    #[signal_vec]
    #[default(vec![])]
    items: Dom,

    #[signal]
    #[default(None)]
    header: Option<Dom>,

    #[signal]
    #[default(None)]
    footer: Option<Dom>,

    #[signal]
    #[default(false)]
    collapsed: bool,
}

pub fn glass_sidebar(props: GlassSidebarProps) -> Dom {
    let GlassSidebarProps {
        items,
        header,
        footer,
        collapsed,
        apply,
    } = props;

    let collapsed = collapsed.broadcast();

    html!("aside", {
        .dwclass!("h-full flex flex-col glass-text-primary")
        .apply(glass_surface_mixin(GlassSurfaceLevel::Base))
        .style("border-right", "var(--glass-border-width) solid var(--glass-border-color)")
        .style("transition-duration", "var(--glass-transition-slow)")
        .style("transition-timing-function", "var(--glass-ease)")
        .style("transition-property", "width")
        .style("overflow", "hidden")

        .style_signal("width", collapsed.signal().map(|c| {
            if c { "4rem" } else { "16rem" }
        }))

        // Header
        .child_signal(header.map(|h| {
            h.map(|header_dom| {
                html!("div", {
                    .dwclass!("p-4")
                    .style("border-bottom", "var(--glass-border-width) solid var(--glass-border-color)")
                    .child(header_dom)
                })
            })
        }))

        // Items
        .child(html!("nav", {
            .dwclass!("flex-1 flex flex-col gap-1 p-2 overflow-y-auto")
            .style("white-space", "nowrap")
            .children_signal_vec(items)
        }))

        // Footer
        .child_signal(footer.map(|f| {
            f.map(|footer_dom| {
                html!("div", {
                    .dwclass!("p-4")
                    .style("border-top", "var(--glass-border-width) solid var(--glass-border-color)")
                    .child(footer_dom)
                })
            })
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
