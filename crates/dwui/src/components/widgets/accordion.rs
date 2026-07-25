use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, svg, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;

/// An accessible accordion / disclosure list.
///
/// Each `(title, content)` entry renders a header button with
/// `aria-expanded` and `aria-controls` pointing at its content region
/// (`role="region"`, `aria-labelledby` back at the header). Panels animate
/// open/closed with the CSS grid-rows technique. By default only one panel
/// is open at a time; set `allow_multiple` to keep panels independent.
#[component(render_fn = accordion)]
struct Accordion {
    #[signal_vec]
    #[default(vec![])]
    items: (String, Dom),

    #[default(false)]
    allow_multiple: bool,

    /// Index of the panel that starts open
    #[default(None)]
    initial_open: Option<usize>,
}

pub fn accordion(props: AccordionProps) -> Dom {
    let AccordionProps {
        items,
        allow_multiple,
        initial_open,
        apply,
    } = props;

    let open_items: Mutable<Vec<usize>> =
        Mutable::new(initial_open.map(|v| vec![v]).unwrap_or_default());

    html!("div", {
        .dwclass!("flex flex-col w-full")
        .dwclass!("border dwui-border-void-700 is(.light *):dwui-border-void-300 rounded-md overflow-hidden")
        .children_signal_vec(items.enumerate().map(clone!(open_items => move |(index, (title, content))| {
            let header_id = component_id("accordion-header");
            let panel_id = component_id("accordion-panel");

            let is_open = open_items
                .signal_cloned()
                .map(clone!(index => move |open| {
                    index.get().map(|i| open.contains(&i)).unwrap_or(false)
                }))
                .broadcast();

            html!("div", {
                .dwclass!("flex flex-col")
                .dwclass!("border-b dwui-border-void-700 is(.light *):dwui-border-void-300")
                .child(html!("button", {
                    .attr("type", "button")
                    .attr("id", &header_id)
                    .attr("aria-controls", &panel_id)
                    .attr_signal("aria-expanded", is_open.signal().map(|v| if v { "true" } else { "false" }))
                    .dwclass!("flex flex-row align-items-center justify-between gap-2 p-4 w-full cursor-pointer")
                    .dwclass!("bg-transparent border-none text-base font-medium text-left")
                    .dwclass!("dwui-text-on-primary-100 is(.light *):dwui-text-on-primary-900")
                    .dwclass!("hover:dwui-bg-void-800 is(.light *):hover:dwui-bg-void-200 transition-colors")
                    .dwclass!("dwui-focusable")
                    .child(html!("span", { .text(&title) }))
                    .child(html!("span", {
                        .dwclass!("flex-none inline-flex align-items-center")
                        .style("transition", "transform 200ms ease-out")
                        .style_signal("transform", is_open.signal().map(|v| {
                            if v { "rotate(180deg)" } else { "rotate(0deg)" }
                        }))
                        .child(svg!("svg", {
                            .attr("viewBox", "0 0 12 12")
                            .attr("width", "12")
                            .attr("height", "12")
                            .attr("fill", "none")
                            .attr("aria-hidden", "true")
                            .child(svg!("path", {
                                .attr("d", "M2 4 L6 8 L10 4")
                                .attr("stroke", "currentColor")
                                .attr("stroke-width", "1.5")
                                .attr("stroke-linecap", "round")
                                .attr("stroke-linejoin", "round")
                            }))
                        }))
                    }))
                    .event(clone!(open_items, index => move |_: events::Click| {
                        let Some(i) = index.get() else { return };

                        let mut open = open_items.get_cloned();

                        if let Some(pos) = open.iter().position(|v| *v == i) {
                            open.remove(pos);
                        } else if allow_multiple {
                            open.push(i);
                        } else {
                            open = vec![i];
                        }

                        open_items.set(open);
                    }))
                }))
                .child(html!("div", {
                    .style("display", "grid")
                    .style("transition", "grid-template-rows 200ms ease-out")
                    .style_signal("grid-template-rows", is_open.signal().map(|v| {
                        if v { "1fr" } else { "0fr" }
                    }))
                    .child(html!("div", {
                        .attr("id", &panel_id)
                        .attr("role", "region")
                        .attr("aria-labelledby", &header_id)
                        .attr_signal("aria-hidden", is_open.signal().map(|v| if v { None } else { Some("true") }))
                        .dwclass!("overflow-hidden")
                        .child(html!("div", {
                            .dwclass!("p-4 p-t-0 text-base")
                            .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700")
                            .child(content)
                        }))
                    }))
                }))
            })
        })))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
