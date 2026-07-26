use crate::theme::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;

/// A breadcrumb trail.
///
/// Renders a `<nav aria-label="Breadcrumb">` with one link per
/// `(label, href)` entry; the final entry is marked `aria-current="page"`
/// and rendered as plain text.
#[component(render_fn = breadcrumbs)]
struct Breadcrumbs {
    #[signal_vec]
    #[default(vec![])]
    items: (String, String),
}

pub fn breadcrumbs(props: BreadcrumbsProps) -> Dom {
    let BreadcrumbsProps { items, apply } = props;

    let items = items.to_signal_cloned().broadcast();

    html!("nav", {
        .attr("aria-label", "Breadcrumb")
        .child_signal(items.signal_cloned().map(|items| {
            let count = items.len();

            Some(html!("ol", {
                .dwclass!("flex flex-row flex-wrap align-items-center gap-2 p-0 m-0")
                .style("list-style", "none")
                .children(items.into_iter().enumerate().flat_map(|(index, (label, href))| {
                    let is_last = index + 1 == count;

                    let mut nodes = vec![];

                    if index > 0 {
                        nodes.push(html!("li", {
                            .attr("aria-hidden", "true")
                            .dwclass!("select-none dwui-text-on-primary-500 is(.light *):dwui-text-on-primary-500 text-sm")
                            .text("/")
                        }));
                    }

                    nodes.push(html!("li", {
                        .dwclass!("text-sm")
                        .apply(move |b| {
                            if is_last {
                                b.child(html!("span", {
                                    .attr("aria-current", "page")
                                    .dwclass!("dwui-text-on-primary-100 is(.light *):dwui-text-on-primary-900 font-medium")
                                    .text(&label)
                                }))
                            } else {
                                b.child(html!("a", {
                                    .attr("href", &href)
                                    .dwclass!("dwui-text-on-primary-400 hover:dwui-text-primary-300 is(.light *):dwui-text-on-primary-600 is(.light *):hover:dwui-text-primary-700")
                                    .dwclass!("transition-colors dwui-focusable")
                                    .style("text-decoration", "none")
                                    .text(&label)
                                }))
                            }
                        })
                    }));

                    nodes
                }).collect::<Vec<_>>())
            }))
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
