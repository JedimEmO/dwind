use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::routing::go_to_url;
use dominator::{events, text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use example_html_highlight_macro::example_html;
use futures_signals::signal::{Mutable, SignalExt};

pub fn getting_started_page() -> Dom {
    html!("div", {
        .dwclass!("w-full m-b-10")
        .child(doc_page_title("Getting started"))

        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("The stack has two layers, and you can adopt them independently: \
                   dwind is the utility styling layer (Tailwind-style classes, checked by rustc), \
                   and dwui is the component library built on top of it.")
        }))

        .child(doc_page_sub_header("Install"))
        .child(html!("pre", {
            .class("font-code")
            .dwclass!("text-sm p-4 m-0 m-t-2 rounded-lg border border-woodsmoke-800 text-woodsmoke-200 overflow-x-auto")
            .style("background", "rgba(2, 2, 3, 0.7)")
            .text("cargo add dwind dwind-macros   # the styling layer\ncargo add dwui                  # optional: the component layer")
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed text-sm m-t-2 m-b-0")
            .text("Both compile to WebAssembly with the DOMINATOR framework; this site is built \
                   with trunk, and the repository's examples directory has a ready-made template.")
        }))

        .child(doc_page_sub_header("The styling layer — dwind"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-0")
            .text("Apply utility classes with dwclass!. Class names resolve to generated Rust \
                   constants, so a typo fails the build:")
        }))
        .child(example_box(dwind_hello(), true))
        .child(code(&DWIND_HELLO_EXAMPLE_HTML_MAP))

        .child(doc_page_sub_header("The component layer — dwui"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-0")
            .text("dwui components are themeable, accessible, and signal-driven. The same \
                   reactive value drives this switch and badge:")
        }))
        .child(example_box(dwui_hello(), false))
        .child(code(&DWUI_HELLO_EXAMPLE_HTML_MAP))

        .child(doc_page_sub_header("Where next"))
        .child(html!("div", {
            .dwclass!("flex @sm:flex-row @<sm:flex-col gap-4 m-t-2")
            .children([
                next_link("dwind showcase →", "#/examples"),
                next_link("dwui gallery →", "#/components"),
                next_link("color system →", "#/docs/colors"),
            ])
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn dwind_hello() -> Dom {
    html!("div", {
        .dwclass!("flex @sm:flex-row @<sm:flex-col gap-4 align-items-center justify-center w-full")
        .dwclass!("rounded-lg border border-woodsmoke-700 bg-woodsmoke-900 p-6")
        .child(html!("div", {
            .dwclass!("text-xl font-extrabold text-woodsmoke-50")
            .text("Hello, dwind!")
        }))
        .child(html!("button", {
            .dwclass!("bg-candlelight-400 hover:bg-candlelight-300 text-woodsmoke-950")
            .dwclass!("rounded-full p-l-4 p-r-4 p-t-1 p-b-1 font-bold border-none cursor-pointer transition-colors")
            .text("Utilities only")
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn dwui_hello() -> Dom {
    let enabled = Mutable::new(true);

    html!("div", {
        .dwclass!("flex flex-col gap-4 w-full p-2")
        .child(switch!({
            .checked_signal(enabled.signal())
            .label("Reactive demo".to_string())
            .on_change({
                let enabled = enabled.clone();
                move |value| enabled.set(value)
            })
        }))
        .child_signal(enabled.signal().map(|on| {
            Some(badge!({
                .variant(if on { BadgeVariant::Primary } else { BadgeVariant::Void })
                .content(Some(text(if on { "signal: true" } else { "signal: false" })))
            }))
        }))
    })
}

fn next_link(label: &str, href: &str) -> Dom {
    let href = href.to_string();

    html!("a", {
        .class("font-code")
        .dwclass!("p-l-4 p-r-4 p-t-2 p-b-2 text-sm rounded-md border border-woodsmoke-800 cursor-pointer transition-colors")
        .dwclass!("text-woodsmoke-300 hover:text-candlelight-300 hover:border-candlelight-700")
        .style("text-decoration", "none")
        .text(label)
        .event(move |_: events::Click| {
            go_to_url(&href);
        })
    })
}
