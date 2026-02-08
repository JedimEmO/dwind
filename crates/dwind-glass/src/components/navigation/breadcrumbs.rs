use crate::prelude::*;
use dominator::{events, html, Dom};
use dwind::prelude::*;
use futures_signals_component_macro::component;

pub struct BreadcrumbItem {
    pub label: String,
    pub on_click: Option<Box<dyn Fn() + 'static>>,
}

#[component(render_fn = glass_breadcrumbs)]
struct GlassBreadcrumbs {
    #[default(vec![])]
    items: Vec<BreadcrumbItem>,
}

pub fn glass_breadcrumbs(props: GlassBreadcrumbsProps) -> Dom {
    let GlassBreadcrumbsProps { items, apply } = props;

    let total = items.len();

    html!("nav", {
        .attr("aria-label", "breadcrumb")
        .child(html!("ol", {
            .dwclass!("flex items-center gap-2 text-sm")
            .style("list-style", "none")
            .style("padding", "0")
            .style("margin", "0")
            .children(items.into_iter().enumerate().flat_map(|(i, item)| {
                let is_last = i == total - 1;
                let mut elements = Vec::new();

                // Separator (except before first item)
                if i > 0 {
                    elements.push(html!("li", {
                        .dwclass!("glass-text-tertiary select-none")
                        .attr("aria-hidden", "true")
                        .text("/")
                    }));
                }

                // Breadcrumb item
                if is_last {
                    elements.push(html!("li", {
                        .dwclass!("glass-text-primary font-medium")
                        .attr("aria-current", "page")
                        .text(&item.label)
                    }));
                } else if let Some(on_click) = item.on_click {
                    elements.push(html!("li", {
                        .child(html!("button", {
                            .dwclass!("glass-text-secondary hover:glass-text-primary cursor-pointer transition-all")
                            .style("background", "none")
                            .style("border", "none")
                            .style("padding", "0")
                            .style("transition-duration", "var(--glass-transition)")
                            .style("text-decoration", "none")
                            .event(move |_: events::Click| {
                                (on_click)();
                            })
                            .text(&item.label)
                        }))
                    }));
                } else {
                    elements.push(html!("li", {
                        .dwclass!("glass-text-secondary")
                        .text(&item.label)
                    }));
                }

                elements
            }).collect::<Vec<_>>())
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
