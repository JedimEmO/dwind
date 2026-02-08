use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use dwind_glass::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::registry::{all_sections, SectionEntry};
use crate::sidebar::scroll_to_section;

fn filter_sections(query: &str) -> Vec<SectionEntry> {
    if query.is_empty() {
        return all_sections();
    }
    let query_lower = query.to_lowercase();
    all_sections()
        .into_iter()
        .filter(|entry| {
            entry.name.to_lowercase().contains(&query_lower)
                || entry.keywords.to_lowercase().contains(&query_lower)
        })
        .collect()
}

fn navigate_to_section(entry: &SectionEntry, search_open: &Mutable<bool>) {
    let target_hash = entry.page.hash();
    let section_id = entry.id.to_string();

    search_open.set(false);

    // Check if already on the right page
    let current_hash = web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default();
    let current_hash_normalized = if current_hash.is_empty() {
        "#/core".to_string()
    } else {
        current_hash
    };

    if current_hash_normalized == target_hash {
        // Same page — scroll directly
        scroll_to_section(&section_id);
    } else {
        // Different page — navigate first, then scroll after delay
        dominator::routing::go_to_url(target_hash);

        // Use setTimeout to wait for the page to render before scrolling
        let cb = Closure::once(move || {
            scroll_to_section(&section_id);
        });
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                100,
            );
        cb.forget();
    }
}

pub fn search_overlay(search_open: Mutable<bool>) -> Dom {
    html!("div", {
        .dwclass!("fixed inset-0 flex items-start justify-center")
        .style("z-index", "1000")
        .style("padding-top", "15vh")
        .style("visibility", "hidden")
        .style("pointer-events", "none")
        .style_signal("visibility", search_open.signal().map(|o| {
            if o { "visible" } else { "hidden" }
        }))
        .style_signal("pointer-events", search_open.signal().map(|o| {
            if o { "auto" } else { "none" }
        }))

        // Backdrop
        .child(html!("div", {
            .dwclass!("absolute inset-0")
            .style("background", "rgba(0, 0, 0, 0.4)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(4px)")
            .event({
                let search_open = search_open.clone();
                move |_: events::Click| {
                    search_open.set(false);
                }
            })
        }))

        // Palette panel — conditionally rendered so claim_focus fires each time
        .child_signal(search_open.signal().map({
            let search_open = search_open.clone();
            move |open| {
                if !open {
                    return None;
                }
                let search_query = Mutable::new(String::new());
                Some(html!("div", {
                    .dwclass!("relative flex flex-col")
                    .style("width", "min(36rem, 90vw)")
                    .style("max-height", "60vh")
                    .style("background", "\
                        linear-gradient(to bottom, rgba(255, 255, 255, 0.08) 0%, transparent 40%), \
                        var(--glass-bg-elevated)")
                    .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
                    .style("border-radius", "var(--glass-border-radius-xl)")
                    .style("box-shadow", "var(--glass-shadow-xl)")
                    .style("overflow", "hidden")
                    .style("z-index", "1001")

                    // Search input
                    .child(html!("div", {
                        .style("padding", "1rem")
                        .style("border-bottom", "1px solid rgba(255, 255, 255, 0.06)")
                        .child(glass_text_input!({
                            .value(search_query.clone())
                            .placeholder("Search components...".to_string())
                            .claim_focus(true)
                        }))
                    }))

                    // Results list
                    .child(html!("div", {
                        .style("overflow-y", "auto")
                        .style("padding", "0.5rem")
                        .child_signal(search_query.signal_cloned().map({
                            let search_open = search_open.clone();
                            move |query| {
                                let results = filter_sections(&query);
                                Some(render_results(results, search_open.clone()))
                            }
                        }))
                    }))
                }))
            }
        }))

        // Escape to close
        .global_event({
            let search_open = search_open.clone();
            move |e: events::KeyDown| {
                if e.key() == "Escape" {
                    search_open.set(false);
                }
            }
        })
    })
}

fn render_results(results: Vec<SectionEntry>, search_open: Mutable<bool>) -> Dom {
    if results.is_empty() {
        return html!("div", {
            .dwclass!("text-sm glass-text-tertiary")
            .style("padding", "1rem")
            .style("text-align", "center")
            .text("No components found")
        });
    }

    html!("div", {
        .dwclass!("flex flex-col gap-0")
        .children(results.into_iter().map(|entry| {
            let hover = Mutable::new(false);
            let name = entry.name;
            let page_name = entry.page.to_string();

            html!("div", {
                .dwclass!("flex items-center justify-between cursor-pointer transition-all")
                .style("padding", "0.625rem 0.75rem")
                .style("border-radius", "var(--glass-border-radius)")
                .style("transition-duration", "var(--glass-transition)")
                .style_signal("background", hover.signal().map(|h| {
                    if h { "rgba(255, 255, 255, 0.08)" } else { "transparent" }
                }))
                .child(html!("span", {
                    .dwclass!("text-sm font-medium glass-text-primary")
                    .text(name)
                }))
                .child(html!("span", {
                    .dwclass!("text-xs glass-text-tertiary")
                    .text(&page_name)
                }))
                .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
                .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))
                .event({
                    let search_open = search_open.clone();
                    move |_: events::Click| {
                        navigate_to_section(&entry, &search_open);
                    }
                })
            })
        }).collect::<Vec<_>>())
    })
}
