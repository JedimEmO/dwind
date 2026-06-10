use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;

pub fn doc_page_title(title: &str) -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-1 m-t-8")
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-candlelight-400 text-sm")
            .text("// docs")
        }))
        .child(html!("h1", {
            .class("font-display")
            .dwclass!("text-3xl font-bold text-woodsmoke-50 m-0")
            .text(title)
        }))
    })
}

pub fn doc_page_sub_header(title: &str) -> Dom {
    html!("h2", {
        .class("font-display")
        .dwclass!("text-xl font-bold text-woodsmoke-100 m-t-10 m-b-2")
        .text(title)
    })
}
