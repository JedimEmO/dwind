use std::rc::Rc;

use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, Signal, SignalExt};

use crate::registry::{sections_for_page, SectionEntry};
use crate::router::Page;

pub fn scroll_to_section(id: &str) {
    if let Some(element) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
    {
        let options = web_sys::ScrollIntoViewOptions::new();
        options.set_behavior(web_sys::ScrollBehavior::Smooth);
        element.scroll_into_view_with_scroll_into_view_options(&options);
    }
}

pub fn sidebar_view(current_page: impl Signal<Item = Page> + 'static) -> Dom {
    let active_section: Rc<Mutable<Option<String>>> = Rc::new(Mutable::new(None));

    html!("aside", {
        .style("position", "sticky")
        .style("top", "60px")
        .style("height", "calc(100vh - 60px)")
        .style("width", "13rem")
        .style("flex-shrink", "0")
        .style("overflow-y", "auto")
        .style("padding", "1.5rem 0.75rem")
        .style("border-right", "1px solid var(--glass-border-color)")

        .child(html!("div", {
            .dwclass!("text-xs font-semibold mb-3")
            .style("color", "var(--glass-text-tertiary)")
            .style("text-transform", "uppercase")
            .style("letter-spacing", "0.05em")
            .style("padding", "0 0.5rem")
            .text("On this page")
        }))

        .child_signal(current_page.map({
            let active_section = active_section.clone();
            move |page| {
                let sections = sections_for_page(page);
                active_section.set(None);
                Some(render_section_links(sections, active_section.clone()))
            }
        }))
    })
}

fn render_section_links(sections: Vec<SectionEntry>, active_section: Rc<Mutable<Option<String>>>) -> Dom {
    html!("nav", {
        .dwclass!("flex flex-col gap-0")
        .children(sections.into_iter().map(|entry| {
            let id = entry.id.to_string();
            let active_section = active_section.clone();
            let hover = Mutable::new(false);

            html!("a", {
                .dwclass!("text-sm cursor-pointer transition-all")
                .style("border-radius", "var(--glass-border-radius-sm)")
                .style("text-decoration", "none")
                .style("display", "block")
                .style("padding", "0.375rem 0.5rem")
                .style("transition-duration", "var(--glass-transition)")
                .style_signal("color", {
                    let id = id.clone();
                    active_section.signal_cloned().map(move |active| {
                        if active.as_deref() == Some(id.as_str()) {
                            "var(--glass-accent)"
                        } else {
                            "var(--glass-text-secondary)"
                        }
                    })
                })
                .style_signal("background", {
                    let id = id.clone();
                    let hover = hover.clone();
                    futures_signals::map_ref! {
                        let active = active_section.signal_cloned(),
                        let hovered = hover.signal() => {
                            if active.as_deref() == Some(id.as_str()) {
                                "var(--glass-accent-muted)"
                            } else if *hovered {
                                "rgba(255, 255, 255, 0.05)"
                            } else {
                                "transparent"
                            }
                        }
                    }
                })
                .text(entry.name)
                .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
                .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))
                .event({
                    let id = id.clone();
                    let active_section = active_section.clone();
                    move |_: events::Click| {
                        active_section.set(Some(id.clone()));
                        scroll_to_section(&id);
                    }
                })
            })
        }).collect::<Vec<_>>())
    })
}
