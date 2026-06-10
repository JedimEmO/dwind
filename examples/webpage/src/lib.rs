mod pages;
mod reveal;
mod router;

#[macro_use]
extern crate log;

#[macro_use]
extern crate dominator;

#[macro_use]
extern crate dwui;

use crate::pages::components_page::components_page;
use crate::pages::docs::doc_main::doc_main_view;
use crate::pages::docs::doc_sidebar::doc_sidebar;
use crate::pages::docs::{doc_sections, DocPage};
use crate::pages::dwind_examples::dwind_examples_page;
use crate::pages::home::home_page;
use crate::router::make_app_router;
use dominator::routing::go_to_url;
use dominator::{body, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::theme::prelude::ColorsCssVariables;
use futures_signals::signal::{always, SignalExt};
use std::sync::Arc;
use web_sys::window;

const APP_KEYFRAMES: &str = r#"
@keyframes dwind-cursor-blink {
    0%, 49% { opacity: 1; }
    50%, 100% { opacity: 0; }
}

@keyframes dwind-fade-up {
    from {
        opacity: 0;
        transform: translateY(14px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes dwind-glow-drift {
    0%, 100% { transform: translate(0, 0) scale(1); }
    50% { transform: translate(4%, -6%) scale(1.08); }
}

/* scroll-triggered progressive reveal (see reveal.rs) */
.reveal-section > * {
    opacity: 0;
    transform: translateY(26px);
    transition:
        opacity 650ms cubic-bezier(0.16, 1, 0.3, 1),
        transform 650ms cubic-bezier(0.16, 1, 0.3, 1);
}

.reveal-section > *:nth-child(2) { transition-delay: 70ms; }
.reveal-section > *:nth-child(3) { transition-delay: 140ms; }
.reveal-section > *:nth-child(4) { transition-delay: 210ms; }
.reveal-section > *:nth-child(5) { transition-delay: 280ms; }

.reveal-section.reveal-in > * {
    opacity: 1;
    transform: translateY(0);
}
"#;

#[cfg(not(test))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
async fn main() {
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
    dominator::stylesheet_raw(APP_KEYFRAMES);

    html!("div", {
        .dwclass!("text-woodsmoke-100 bg-woodsmoke-950")
        .dwclass!("h-full overflow-y-auto overflow-x-hidden")
        .child(top_nav())
        .child(html!("main", {
            .child_signal(make_app_router().signal().map(|page| {
                Some(match page {
                    DocPage::Home => home_page(),
                    DocPage::DwuiExamples => components_page(),
                    DocPage::Examples => dwind_examples_page(),
                    other => docs_shell(other),
                })
            }))
        }))
        .child(footer())
    })
}

fn docs_shell(page: DocPage) -> Dom {
    html!("div", {
        .dwclass!("m-x-auto flex max-w-5xl w-full p-t-6 p-l-4 p-r-4")
        .style("min-height", "70vh")
        .child_signal(doc_sidebar(
            doc_sections(),
            move || always(page),
            Arc::new(|v: DocPage| v.goto()),
            move || {
                html!("div", {
                    .dwclass!("m-l-4 m-r-0 w-full")
                    .child_signal(doc_main_view(always(Some(page))))
                })
            },
        ))
    })
}

fn top_nav() -> Dom {
    html!("header", {
        .dwclass!("sticky top-0 z-50 w-full")
        .style("backdrop-filter", "blur(12px)")
        .style("background", "rgba(2, 2, 3, 0.72)")
        .dwclass!("border-b border-woodsmoke-800")
        .child(html!("div", {
            .dwclass!("m-x-auto max-w-6xl flex align-items-center justify-between h-14 p-l-4 p-r-4")
            // Wordmark
            .child(html!("a", {
                .attr("href", "#/")
                .class("font-code")
                .dwclass!("flex align-items-center gap-1 text-l font-bold text-woodsmoke-50 cursor-pointer")
                .style("text-decoration", "none")
                .child(html!("span", {
                    .dwclass!("text-candlelight-400")
                    .text("λ")
                }))
                .child(html!("span", { .text("dwind") }))
                .child(html!("span", {
                    .dwclass!("text-candlelight-400")
                    .style("animation", "dwind-cursor-blink 1.2s step-end infinite")
                    .text("_")
                }))
            }))
            // Links
            .child(html!("nav", {
                .attr("aria-label", "Main")
                .class("font-code")
                .dwclass!("flex align-items-center @sm:gap-6 @<sm:gap-3 @sm:text-sm @<sm:text-xs")
                .children([
                    nav_link("components", "#/components", false),
                    nav_link("docs", "#/docs/colors", false),
                    nav_link("examples", "#/examples", false),
                    nav_external("github", "https://github.com/JedimEmO/dwind"),
                ])
            }))
        }))
    })
}

fn nav_link(label: &str, href: &str, emphasized: bool) -> Dom {
    let href = href.to_string();

    html!("a", {
        .attr("href", &href)
        .dwclass!("cursor-pointer transition-colors select-none")
        .dwclass!("text-woodsmoke-300 hover:text-candlelight-300")
        .apply_if(emphasized, |b| dwclass!(b, "text-candlelight-400"))
        .style("text-decoration", "none")
        .text(label)
        .event(move |_: dominator::events::Click| {
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
        .style("text-decoration", "none")
        .text(label)
        .event(move |_: dominator::events::Click| {
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
        .child(html!("div", {
            .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 p-t-10 p-b-10")
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
                    ("docs", "#/docs/colors"),
                    ("examples", "#/examples"),
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
                .style("text-decoration", "none")
                .text(label)
            })
        }))
    })
}
