use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;

/// The masthead every documentation page opens with.
pub fn doc_page_title(title: &str) -> Dom {
    html!("header", {
        .dwclass!("flex flex-col gap-3 m-t-2 m-b-2 p-b-6 border-b border-woodsmoke-800")
        .child(html!("div", {
            .class("font-code")
            .dwclass!("flex flex-row align-items-center gap-2 text-xs text-woodsmoke-500")
            .child(html!("span", { .text("docs") }))
            .child(html!("span", { .dwclass!("text-woodsmoke-700") .text("/") }))
            .child(html!("span", {
                .dwclass!("text-candlelight-400")
                .text(&title.to_lowercase())
            }))
        }))
        .child(html!("h1", {
            .class("font-display")
            .dwclass!("@sm:text-5xl @<sm:text-3xl font-extrabold text-woodsmoke-50 m-0")
            .style("letter-spacing", "-0.03em")
            .text(title)
        }))
    })
}

/// A section heading inside a page. The leading rule gives the long utility
/// reference pages a scannable rhythm.
pub fn doc_page_sub_header(title: &str) -> Dom {
    html!("h2", {
        .class("font-display")
        .dwclass!("flex flex-row align-items-center gap-3 text-xl font-bold text-woodsmoke-100 m-t-12 m-b-2")
        .child(html!("span", {
            .attr("aria-hidden", "true")
            .dwclass!("w-1 h-6 rounded-full flex-none")
            .style("background", "linear-gradient(180deg, #D5B65F, rgba(213, 182, 95, 0))")
        }))
        .child(dominator::text(title))
    })
}
