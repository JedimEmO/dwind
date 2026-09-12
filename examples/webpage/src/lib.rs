mod fx;
mod keyframes;
mod pages;
mod palette;
mod reveal;
mod router;

#[macro_use]
extern crate log;

#[macro_use]
extern crate dominator;

#[macro_use]
extern crate dwui;

use crate::fx::magnetic;
use crate::keyframes::*;
use crate::pages::charts::charts_page;
use crate::pages::components_page::components_page;
use crate::pages::docs::doc_main::doc_main_view;
use crate::pages::docs::doc_sidebar::doc_sidebar;
use crate::pages::docs::{doc_sections, DocPage};
use crate::pages::dwind_examples::dwind_examples_page;
use crate::pages::home::home_page;
use crate::palette::Palette;
use crate::router::make_app_router;
use dominator::routing::go_to_url;
use dominator::{body, events, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::theme::prelude::ColorsCssVariables;
use futures_signals::signal::{always, Mutable, SignalExt};
use std::sync::Arc;
use web_sys::window;

#[cfg(not(test))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
async fn main() {
    console_error_panic_hook::set_once();
    wasm_log::init(Default::default());

    dominator::replace_dom(&body().parent_node().unwrap(), &body(), main_view());
}

fn main_view() -> Dom {
    dwind::stylesheet();
    dwui::theme::apply_style_sheet(Some(ColorsCssVariables::new(
        &DWIND_COLORS["candlelight"],
        &DWIND_COLORS["woodsmoke"],
        &DWIND_COLORS["woodsmoke"],
        &DWIND_COLORS["red"],
    )));
    dwind_dviz::theme::apply_style_sheet();

    let palette = palette::global();
    let page = make_app_router().signal().broadcast();

    // How far down the page we are, 0.0 – 1.0. One number, written by a scroll
    // listener and read by exactly one style_signal.
    let scrolled = Mutable::new(0.0f64);

    html!("div", {
        .apply(crate::fx::slim_scrollbar)
        .dwclass!("text-woodsmoke-100 bg-woodsmoke-950")
        .dwclass!("h-full overflow-y-auto overflow-x-hidden")
        .dwclass!("relative")
        // Charts pick up the site's accent and faces through these tokens.
        .style("--dviz-accent", "#D5B65F")
        .style("--dviz-font-display", "'Bricolage Grotesque', 'IBM Plex Sans', sans-serif")
        .style("--dviz-font-mono", "'JetBrains Mono', monospace")
        .apply(palette.shortcuts())
        .with_node!(element => {
            .event(clone!(scrolled => move |_: events::Scroll| {
                let max = (element.scroll_height() - element.client_height()).max(1) as f64;
                scrolled.set_neq((element.scroll_top() as f64 / max).clamp(0.0, 1.0));
            }))
        })
        .child(fx::aurora())
        .child(scroll_progress(scrolled.clone()))
        .child(top_nav(&palette, page.signal()))
        .child(html!("main", {
            .dwclass!("relative")
            .style("z-index", "1")
            .child_signal(page.signal().map(|page| {
                Some(html!("div", {
                    .dwclass!("animate-route-in")
                    .after_inserted(|_| scroll_to_top())
                    .child(match page {
                        DocPage::Home => home_page(),
                        DocPage::DwuiExamples => components_page(),
                        DocPage::Examples => dwind_examples_page(),
                        DocPage::Charts => charts_page(),
                        other => docs_shell(other),
                    })
                }))
            }))
        }))
        .child(footer())
        .child(fx::grain())
        .child(palette.render())
    })
}

fn scroll_to_top() {
    if let Some(win) = window() {
        win.scroll_to_with_x_and_y(0.0, 0.0);
    }
}

/// Reading-progress rail pinned under the header.
fn scroll_progress(scrolled: Mutable<f64>) -> Dom {
    html!("div", {
        .attr("aria-hidden", "true")
        .dwclass!("fixed")
        .style("top", "0")
        .style("left", "0")
        .style("right", "0")
        .style("height", "2px")
        .style("z-index", "60")
        .dwclass!("pointer-events-none")
        .child(html!("div", {
            .style("height", "100%")
            .style("background", "linear-gradient(90deg, #A88735, #D5B65F 45%, #FFF3CF)")
            .style("box-shadow", "0 0 12px rgba(213, 182, 95, 0.6)")
            .style("transform-origin", "left center")
            .dwclass!("will-change-transform")
            .style_signal("transform", scrolled.signal().map(|p| {
                format!("scaleX({:.4})", p.max(0.001))
            }))
            .style("width", "100%")
        }))
    })
}

fn docs_shell(page: DocPage) -> Dom {
    html!("div", {
        .dwclass!("m-x-auto flex max-w-6xl w-full p-t-8 p-l-4 p-r-4")
        .style("min-height", "70vh")
        .child_signal(doc_sidebar(
            doc_sections(),
            move || always(page),
            Arc::new(|v: DocPage| v.goto()),
            move || {
                html!("div", {
                    .dwclass!("m-l-8 m-r-0 w-full")
                    .style("min-width", "0")
                    .child_signal(doc_main_view(always(Some(page))))
                    .child(docs_pager(page))
                })
            },
        ))
    })
}

/// Prev / next between doc pages, in reading order.
fn docs_pager(page: DocPage) -> Dom {
    let order: Vec<DocPage> = doc_sections()
        .into_iter()
        .flat_map(|section| section.docs)
        .collect();

    let index = order.iter().position(|p| *p == page);
    let prev = index
        .and_then(|i| i.checked_sub(1))
        .and_then(|i| order.get(i))
        .copied();
    let next = index.map(|i| i + 1).and_then(|i| order.get(i)).copied();

    html!("div", {
        .dwclass!("flex flex-row justify-between gap-4 m-t-16 p-t-8 border-t border-woodsmoke-800")
        .child(pager_link(prev, "← prev", true))
        .child(pager_link(next, "next →", false))
    })
}

fn pager_link(page: Option<DocPage>, hint: &str, left: bool) -> Dom {
    let Some(page) = page else {
        return html!("div", { .dwclass!("grow") });
    };

    html!("button", {
        .attr("type", "button")
        .apply(crate::fx::glass)
        .dwclass!("flex flex-col gap-1 rounded-lg border border-woodsmoke-800 p-4 grow cursor-pointer")
        .dwclass!("hover:border-candlelight-700 transition-all")
        .apply(fx::spotlight)
        .apply(move |b| if left {
            dwclass!(b, "text-left align-items-start")
        } else {
            dwclass!(b, "text-right align-items-end")
        })
        .style("color", "inherit")
        .dwclass!("font-inherit")
        .child(html!("span", {
            .class("font-code")
            .dwclass!("text-xs text-woodsmoke-500")
            .text(hint)
        }))
        .child(html!("span", {
            .class("font-display")
            .dwclass!("text-l font-bold text-woodsmoke-100")
            .text(&page.to_string())
        }))
        .event(move |_: events::Click| page.goto())
    })
}

fn top_nav(
    palette: &Palette,
    page: impl futures_signals::signal::Signal<Item = DocPage> + 'static,
) -> Dom {
    let page = page.broadcast();
    let palette = palette.clone();

    html!("header", {
        .dwclass!("sticky top-0 z-50 w-full")
        .style("backdrop-filter", "blur(16px) saturate(1.3)")
        .style("background", "rgba(2, 2, 3, 0.62)")
        .dwclass!("border-b border-woodsmoke-800")
        .child(html!("div", {
            .dwclass!("m-x-auto max-w-6xl flex align-items-center justify-between h-16 p-l-4 p-r-4")
            // Wordmark
            .child(html!("a", {
                .attr("href", "#/")
                .class("font-code")
                .dwclass!("flex align-items-center gap-1 text-l font-bold text-woodsmoke-50 cursor-pointer")
                .dwclass!("no-underline")
                .apply(magnetic(5.0))
                .child(html!("span", {
                    .dwclass!("text-candlelight-400")
                    .text("λ")
                }))
                .child(html!("span", { .text("dwind") }))
                .child(html!("span", {
                    .dwclass!("text-candlelight-400")
                    .dwclass!("animate-cursor-blink")
                    .text("_")
                }))
            }))
            // Links
            .child(html!("nav", {
                .attr("aria-label", "Main")
                .class("font-code")
                .dwclass!("flex align-items-center @sm:gap-6 @<sm:gap-3 @sm:text-sm @<sm:text-xs")
                .child(nav_link("components", "#/components", page.signal().map(|p| p == DocPage::DwuiExamples)))
                .child(nav_link("docs", "#/docs/getting-started", page.signal().map(|p| {
                    !matches!(p, DocPage::Home | DocPage::DwuiExamples | DocPage::Examples | DocPage::Charts)
                })))
                .child(nav_link("examples", "#/examples", page.signal().map(|p| p == DocPage::Examples)))
                .child(nav_link("charts", "#/charts", page.signal().map(|p| p == DocPage::Charts)))
                .child(nav_external("github", "https://github.com/JedimEmO/dwind"))
                .child(palette_trigger(&palette))
            }))
        }))
    })
}

/// The ⌘K affordance. Also the only way to discover the palette exists.
fn palette_trigger(palette: &Palette) -> Dom {
    let palette = palette.clone();

    html!("button", {
        .attr("type", "button")
        .attr("aria-label", "Open command palette")
        .class("font-code")
        .dwclass!("flex flex-row align-items-center gap-2 cursor-pointer transition-all")
        .dwclass!("border border-woodsmoke-800 rounded-md p-l-2 p-r-2 p-t-1 p-b-1")
        .dwclass!("text-woodsmoke-400 hover:text-candlelight-300 hover:border-candlelight-700")
        .dwclass!("@<sm:hidden text-xs")
        .style("background", "rgba(18, 18, 21, 0.6)")
        .dwclass!("font-inherit")
        .style("font-size", "0.72rem")
        .child(html!("span", { .text("⌘") }))
        .child(html!("span", { .text("K") }))
        .event(move |_: events::Click| palette.open())
    })
}

fn nav_link(
    label: &str,
    href: &str,
    active: impl futures_signals::signal::Signal<Item = bool> + 'static,
) -> Dom {
    let href = href.to_string();
    let active = active.broadcast();

    html!("a", {
        .attr("href", &href)
        .dwclass!("cursor-pointer transition-colors select-none")
        .dwclass!("text-woodsmoke-300 hover:text-candlelight-300")
        .dwclass!("no-underline")
        .dwclass!("relative")
        .style_signal("color", active.signal().map(|a| {
            if a { Some("#E5CE8F") } else { None }
        }))
        .child(dominator::text(label))
        // an underline that grows in when the route becomes active
        .child(html!("span", {
            .attr("aria-hidden", "true")
            .dwclass!("absolute")
            .style("left", "0")
            .style("right", "0")
            .style("bottom", "-6px")
            .style("height", "1px")
            .style("background", "linear-gradient(90deg, transparent, #D5B65F, transparent)")
            .style("transform-origin", "center")
            .style("transition", "transform 320ms cubic-bezier(0.16, 1, 0.3, 1)")
            .style_signal("transform", active.signal().map(|a| {
                if a { "scaleX(1)" } else { "scaleX(0)" }
            }))
        }))
        .event(move |_: events::Click| {
            go_to_url(&href);
        })
    })
}

fn nav_external(label: &str, url: &str) -> Dom {
    let url = url.to_string();

    html!("a", {
        .attr("href", &url)
        .attr("target", "_blank")
        .attr("rel", "noopener")
        .dwclass!("cursor-pointer transition-colors select-none")
        .dwclass!("text-woodsmoke-300 hover:text-candlelight-300")
        .dwclass!("no-underline")
        .text(label)
        .event(move |_: events::Click| {
            window()
                .unwrap()
                .open_with_url_and_target(&url, "_blank")
                .unwrap();
        })
    })
}

fn footer() -> Dom {
    html!("footer", {
        .dwclass!("border-t border-woodsmoke-800 w-full m-t-20")
        .dwclass!("relative")
        .style("z-index", "1")
        .child(html!("div", {
            .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 p-t-12 p-b-12")
            .dwclass!("flex @sm:flex-row @<sm:flex-col justify-between gap-8")
            .child(html!("div", {
                .dwclass!("flex flex-col gap-2")
                .child(html!("div", {
                    .class("font-code")
                    .dwclass!("text-woodsmoke-50 font-bold")
                    .text("λ dwind_")
                }))
                .child(html!("p", {
                    .dwclass!("text-woodsmoke-400 text-sm m-0")
                    .text("Utility-first styling and UI components for Rust, compiled to WebAssembly.")
                }))
                .child(html!("p", {
                    .class("font-code")
                    .dwclass!("text-woodsmoke-600 text-xs m-0")
                    .text("[ok] rendered by dominator — no virtual dom, styles compiled by rustc")
                }))
            }))
            .child(html!("div", {
                .dwclass!("flex flex-row gap-10")
                .child(footer_column("explore", vec![
                    ("components", "#/components"),
                    ("docs", "#/docs/getting-started"),
                    ("examples", "#/examples"),
                    ("charts", "#/charts"),
                ]))
                .child(footer_column("project", vec![
                    ("github", "https://github.com/JedimEmO/dwind"),
                    ("crates.io", "https://crates.io/crates/dwind"),
                    ("dominator", "https://github.com/Pauan/rust-dominator"),
                ]))
            }))
        }))
    })
}

fn footer_column(title: &str, links: Vec<(&str, &str)>) -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-2")
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-woodsmoke-500 text-xs font-medium")
            .text(&format!("// {}", title))
        }))
        .children(links.into_iter().map(|(label, href)| {
            let external = href.starts_with("http");
            let href = href.to_string();

            html!("a", {
                .attr("href", &href)
                .apply_if(external, |b| b.attr("target", "_blank").attr("rel", "noopener"))
                .dwclass!("text-woodsmoke-300 hover:text-candlelight-300 text-sm transition-colors cursor-pointer")
                .dwclass!("no-underline")
                .text(label)
            })
        }))
    })
}
