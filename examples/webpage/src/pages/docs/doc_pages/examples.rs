use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::routing::go_to_url;
use dominator::{events, text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;

pub fn examples_page() -> Dom {
    html!("div", {
        .dwclass!("w-full m-b-10")
        .child(doc_page_title("Welcome to dwind"))

        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("dwind brings Tailwind-style utility classes to the DOMINATOR web framework — \
                   generated at compile time, checked by rustc, with zero runtime overhead.")
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-0")
            .text("This site is itself built with dwind and dwui, compiled to WebAssembly. \
                   Everything you see — including this sentence — is a Rust expression.")
        }))

        .child(doc_page_sub_header("How to use this app"))
        .child(html!("ul", {
            .dwclass!("flex flex-col gap-2 text-woodsmoke-300 m-0 p-l-4")
            .children([
                how_to_item("Navigate the sidebar to explore dwind's styling features"),
                how_to_item("Open the component gallery to play with the DWUI library"),
                how_to_item("Expand \"view source\" under any example to read its implementation"),
                how_to_item("Drag the amber handles (or resize the window) to test responsiveness"),
            ])
        }))

        .child(doc_page_sub_header("Key features"))
        .child(html!("div", {
            .dwclass!("grid grid-cols-1 @md:grid-cols-2 gap-4 m-t-2")
            .children([
                feature_tile("zero-runtime", "Compiled away", "All CSS is generated at build time by rustc and macro expansion. Nothing is parsed or injected at runtime."),
                feature_tile("type-safety", "Type-checked classes", "An invalid class name is a compile error (E0425), not a silently broken layout."),
                feature_tile("responsive", "Responsive conditionals", "Breakpoint prefixes like @sm: and @<md:, plus arbitrary media queries when you need them."),
                feature_tile("theming", "Theme-native", "Design tokens flow through CSS variables — swap palettes and light/dark at runtime."),
            ])
        }))

        .child(doc_page_sub_header("Quick example"))
        .child(example_box(
            html!("div", {
                .dwclass!("flex flex-col gap-4 p-6 rounded-lg border border-woodsmoke-800 w-full")
                .style("background", "rgba(18, 18, 21, 0.7)")
                .child(html!("div", {
                    .dwclass!("flex justify-between align-items-center gap-4")
                    .child(html!("h3", {
                        .class("font-display")
                        .dwclass!("text-xl font-bold text-woodsmoke-50 m-0")
                        .text("Hello dwind!")
                    }))
                    .child(html!("div", {
                        .dwclass!("w-40")
                        .child(button!({
                            .size(ButtonSize::Small)
                            .content(Some(text("Get started")))
                            .on_click(|_: events::Click| {
                                go_to_url("#/docs/colors");
                            })
                        }))
                    }))
                }))
                .child(html!("p", {
                    .dwclass!("text-woodsmoke-300 m-0 leading-relaxed")
                    .text("This card demonstrates dwind utility classes — flexbox layout, spacing, \
                           borders, and typography — plus a live dwui button, all compiled from Rust.")
                }))
            }),
            true
        ))

        .child(doc_page_sub_header("Explore more"))
        .child(html!("div", {
            .dwclass!("flex @sm:flex-row @<sm:flex-col gap-4 m-t-2")
            .children([
                explore_link("colors →", "#/docs/colors"),
                explore_link("responsive design →", "#/docs/responsive-design"),
                explore_link("component gallery →", "#/components"),
            ])
        }))
    })
}

fn how_to_item(content: &str) -> Dom {
    html!("li", {
        .dwclass!("leading-relaxed")
        .text(content)
    })
}

fn feature_tile(kicker: &str, title: &str, description: &str) -> Dom {
    html!("div", {
        .dwclass!("p-6 rounded-lg border border-woodsmoke-800 flex flex-col gap-2 transition-all hover:border-candlelight-700")
        .style("background", "rgba(18, 18, 21, 0.55)")
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-xs text-candlelight-400")
            .text(&format!("// {}", kicker))
        }))
        .child(html!("h4", {
            .class("font-display")
            .dwclass!("font-bold text-woodsmoke-50 m-0")
            .text(title)
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 text-sm m-0 leading-relaxed")
            .text(description)
        }))
    })
}

fn explore_link(label: &str, href: &str) -> Dom {
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
