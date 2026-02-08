#[macro_use]
extern crate dwind_macros;

#[macro_use]
extern crate dwind_glass;

mod code_block;
mod helpers;
mod pages;
mod registry;
mod router;
mod search;
mod sidebar;

use dominator::{events, html, routing, Dom};
use dwind::prelude::*;
use dwind::prelude::media_queries::{breakpoint_active_signal, Breakpoint};
use dwind_glass::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use router::{resolve_route, Page};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub async fn main() {
    wasm_log::init(wasm_log::Config::default());

    dwind::stylesheet();
    apply_glass_theme(None);

    dominator::append_dom(&dominator::body(), main_view());

    // Focus the page so keyboard shortcuts work without clicking first.
    // The body needs tabindex to be focusable, and we defer via setTimeout
    // to let the browser finish layout after WASM init.
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        let _ = body.set_attribute("tabindex", "-1");
        let cb = wasm_bindgen::closure::Closure::once(move || {
            let _ = body.focus();
        });
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                200,
            );
        cb.forget();
    }
}

fn main_view() -> Dom {
    let current_page = routing::url()
        .signal_ref(|url| {
            let hash = url
                .split('#')
                .nth(1)
                .map(|h| format!("#{}", h))
                .unwrap_or_default();
            resolve_route(&hash)
        })
        .broadcast();

    let search_open = Mutable::new(false);
    let mobile_menu_open = Mutable::new(false);

    html!("div", {
        .dwclass!("font-sans min-h-screen flex flex-col")
        .style("color", "var(--glass-text-primary)")

        // Navbar
        .child(navbar_view(current_page.clone(), search_open.clone(), mobile_menu_open.clone()))

        // Mobile sidebar drawer
        .child(mobile_sidebar_drawer(current_page.clone(), mobile_menu_open.clone()))

        // Search overlay (always in DOM, visibility toggled)
        .child(search::search_overlay(search_open.clone()))

        // Body: sidebar + content
        .child(html!("div", {
            .dwclass!("flex flex-1")
            .style("padding-top", "60px")

            // Sidebar (desktop only, >= 1280px)
            .child_signal(
                breakpoint_active_signal(Breakpoint::Medium).map({
                    let current_page = current_page.clone();
                    move |is_desktop| {
                        if is_desktop {
                            Some(sidebar::sidebar_view(current_page.signal_cloned()))
                        } else {
                            None
                        }
                    }
                })
            )

            // Main content
            .child(html!("main", {
                .dwclass!("flex-1 p-6")
                .style("max-width", "64rem")
                .style("min-width", "0")
                .child_signal(current_page.signal_cloned().map(|page| {
                    Some(match page {
                        Page::Core => pages::core_page::core_page(),
                        Page::Navigation => pages::navigation_page::navigation_page(),
                        Page::Data => pages::data_page::data_page(),
                        Page::Theme => pages::theme_page::theme_page(),
                    })
                }))
            }))
        }))

        // Global "s" key to open search (non-passive so we can preventDefault)
        .future({
            let search_open = search_open.clone();
            async move {
                let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                    if e.key() == "s" && !search_open.get() {
                        let is_input = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.active_element())
                            .map(|el| {
                                let tag = el.tag_name().to_uppercase();
                                tag == "INPUT" || tag == "TEXTAREA" || tag == "SELECT"
                            })
                            .unwrap_or(false);

                        if !is_input {
                            e.prevent_default();
                            search_open.set(true);
                        }
                    }
                }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

                web_sys::window().unwrap()
                    .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
                    .unwrap();
                cb.forget();
            }
        })
    })
}

fn navbar_view(
    current_page: futures_signals::signal::Broadcaster<impl futures_signals::signal::Signal<Item = Page> + 'static>,
    search_open: Mutable<bool>,
    mobile_menu_open: Mutable<bool>,
) -> Dom {
    html!("nav", {
        .dwclass!("fixed inset-x-0 top-0 flex items-center justify-between px-6")
        .style("height", "60px")
        .style("z-index", "100")
        .style("background", "\
            linear-gradient(to bottom, rgba(255, 255, 255, 0.06) 0%, transparent 100%), \
            rgba(10, 10, 20, 0.75)")
        .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
        .style("border-bottom", "1px solid var(--glass-border-color)")

        // Left side: hamburger (mobile) + brand + page links
        .child(html!("div", {
            .dwclass!("flex items-center gap-4")

            // Hamburger button (hidden on desktop)
            .child(html!("button", {
                .dwclass!("cursor-pointer")
                .style("background", "none")
                .style("border", "none")
                .style("padding", "4px")
                .style("color", "var(--glass-text-secondary)")
                .style_signal("display", breakpoint_active_signal(Breakpoint::Medium).map(|desktop| {
                    if desktop { "none" } else { "flex" }
                }))
                .event({
                    let mobile_menu_open = mobile_menu_open.clone();
                    move |_: events::Click| {
                        mobile_menu_open.set(!mobile_menu_open.get());
                    }
                })
                .dwclass!("text-xl")
                .text("\u{2630}")
            }))

            .child(html!("span", {
                .dwclass!("text-lg font-bold")
                .style("color", "var(--glass-accent)")
                .text("dwind-glass")
            }))

            // Page links (hidden on mobile)
            .child(html!("div", {
                .dwclass!("items-center gap-1")
                .style("display", "none")
                .style_signal("display", breakpoint_active_signal(Breakpoint::Medium).map(|desktop| {
                    if desktop { "flex" } else { "none" }
                }))
                .children(Page::all().into_iter().map(|page| {
                    let hash = page.hash().to_string();
                    let label = page.to_string();
                    let current_page = current_page.clone();
                    html!("a", {
                        .dwclass!("px-3 py-1 text-sm cursor-pointer transition-all")
                        .style("border-radius", "var(--glass-border-radius)")
                        .style("text-decoration", "none")
                        .style("transition-duration", "var(--glass-transition)")
                        .style_signal("background", current_page.signal_ref(move |p| {
                            if *p == page { "var(--glass-accent-muted)" } else { "transparent" }
                        }))
                        .style_signal("color", current_page.signal_ref(move |p| {
                            if *p == page { "var(--glass-accent)" } else { "var(--glass-text-secondary)" }
                        }))
                        .attr("href", &hash)
                        .text(&label)
                    })
                }).collect::<Vec<_>>())
            }))
        }))

        // Search hint
        .child(html!("div", {
            .dwclass!("flex items-center gap-2 cursor-pointer transition-all")
            .style("transition-duration", "var(--glass-transition)")
            .event({
                let search_open = search_open.clone();
                move |_: events::Click| {
                    search_open.set(true);
                }
            })
            .child(html!("span", {
                .dwclass!("text-sm glass-text-tertiary")
                .text("Search")
            }))
            .child(html!("kbd", {
                .dwclass!("text-xs glass-text-tertiary font-mono")
                .style("background", "rgba(255, 255, 255, 0.08)")
                .style("border", "1px solid rgba(255, 255, 255, 0.1)")
                .style("border-radius", "4px")
                .style("padding", "2px 6px")
                .text("S")
            }))
        }))
    })
}

fn mobile_sidebar_drawer(
    current_page: futures_signals::signal::Broadcaster<impl futures_signals::signal::Signal<Item = Page> + 'static>,
    mobile_menu_open: Mutable<bool>,
) -> Dom {
    html!("div", {
        .dwclass!("fixed inset-0")
        .style("z-index", "99")
        .style("visibility", "hidden")
        .style("pointer-events", "none")
        .style_signal("visibility", mobile_menu_open.signal().map(|o| {
            if o { "visible" } else { "hidden" }
        }))
        .style_signal("pointer-events", mobile_menu_open.signal().map(|o| {
            if o { "auto" } else { "none" }
        }))

        // Backdrop
        .child(html!("div", {
            .dwclass!("absolute inset-0")
            .style("background", "rgba(0, 0, 0, 0.4)")
            .style("transition", "opacity 150ms ease")
            .style("opacity", "0")
            .style_signal("opacity", mobile_menu_open.signal().map(|o| {
                if o { "1" } else { "0" }
            }))
            .event({
                let mobile_menu_open = mobile_menu_open.clone();
                move |_: events::Click| {
                    mobile_menu_open.set(false);
                }
            })
        }))

        // Drawer panel
        .child(html!("aside", {
            .style("position", "absolute")
            .style("top", "0")
            .style("left", "0")
            .style("bottom", "0")
            .style("width", "16rem")
            .style("padding-top", "60px")
            .style("overflow-y", "auto")
            .style("background", "\
                linear-gradient(to bottom, rgba(255, 255, 255, 0.04) 0%, transparent 100%), \
                rgba(10, 10, 20, 0.95)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur-heavy))")
            .style("border-right", "1px solid var(--glass-border-color)")
            .style("transition", "transform 200ms var(--glass-ease)")
            .style("transform", "translateX(-100%)")
            .style_signal("transform", mobile_menu_open.signal().map(|o| {
                if o { "translateX(0)" } else { "translateX(-100%)" }
            }))

            // Page links
            .child(html!("div", {
                .dwclass!("flex flex-col gap-1")
                .style("padding", "1rem 0.75rem")
                .style("border-bottom", "1px solid var(--glass-border-color)")
                .children(Page::all().into_iter().map(|page| {
                    let hash = page.hash().to_string();
                    let label = page.to_string();
                    let current_page = current_page.clone();
                    let mobile_menu_open = mobile_menu_open.clone();
                    html!("a", {
                        .dwclass!("px-3 py-2 text-sm cursor-pointer transition-all")
                        .style("border-radius", "var(--glass-border-radius)")
                        .style("text-decoration", "none")
                        .style("display", "block")
                        .style("transition-duration", "var(--glass-transition)")
                        .style_signal("background", current_page.signal_ref(move |p| {
                            if *p == page { "var(--glass-accent-muted)" } else { "transparent" }
                        }))
                        .style_signal("color", current_page.signal_ref(move |p| {
                            if *p == page { "var(--glass-accent)" } else { "var(--glass-text-secondary)" }
                        }))
                        .attr("href", &hash)
                        .text(&label)
                        .event(move |_: events::Click| {
                            mobile_menu_open.set(false);
                        })
                    })
                }).collect::<Vec<_>>())
            }))

            // Section links for current page
            .child(html!("div", {
                .style("padding", "1rem 0.75rem")
                .child(html!("div", {
                    .dwclass!("text-xs font-semibold mb-3")
                    .style("color", "var(--glass-text-tertiary)")
                    .style("text-transform", "uppercase")
                    .style("letter-spacing", "0.05em")
                    .style("padding", "0 0.5rem")
                    .text("On this page")
                }))
                .child_signal(current_page.signal_cloned().map({
                    let mobile_menu_open = mobile_menu_open.clone();
                    move |page| {
                        let sections = crate::registry::sections_for_page(page);
                        Some(html!("nav", {
                            .dwclass!("flex flex-col gap-0")
                            .children(sections.into_iter().map(|entry| {
                                let mobile_menu_open = mobile_menu_open.clone();
                                html!("a", {
                                    .dwclass!("text-sm cursor-pointer transition-all")
                                    .style("border-radius", "var(--glass-border-radius-sm)")
                                    .style("text-decoration", "none")
                                    .style("display", "block")
                                    .style("padding", "0.375rem 0.5rem")
                                    .style("color", "var(--glass-text-secondary)")
                                    .style("transition-duration", "var(--glass-transition)")
                                    .text(entry.name)
                                    .event(move |_: events::Click| {
                                        mobile_menu_open.set(false);
                                        sidebar::scroll_to_section(entry.id);
                                    })
                                })
                            }).collect::<Vec<_>>())
                        }))
                    }
                }))
            }))
        }))
    })
}
