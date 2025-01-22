use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use crate::pages::docs::helper_components::table::example_table;
use dominator::Dom;
use dwind::prelude::media_queries::{breakpoint_active_signal, Breakpoint};
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;
use futures_signals::signal::SignalExt;

pub fn responsive_design() -> Dom {
    html!("div", {
        .child(doc_page_title("Responsive Design"))
        .text("Any modern web app must look good on on both small mobile devices, as well as on enormous desktop monitors.")
        .child(doc_page_sub_header("Breakpoints"))
        .text("To chose a breakpoint, we can use the @constraint syntax. The following breakpoints are available:")
        .child(example_table(["Usage".to_string(), "Description".to_string(), "Size".to_string()], [
            ["dwclass!(\"my-cls\")".to_string(), "very small screens".to_string(), "< 640px".to_string()],
            ["dwclass!(\"@sm:my-cls\")".to_string(), "small screens".to_string(), ">= 640px".to_string()],
            ["dwclass!(\"@md:my-cls\")".to_string(), "medium screens".to_string(), ">= 1280px".to_string()],
            ["dwclass!(\"@lg:my-cls\")".to_string(), "large screens".to_string(), ">= 1920px".to_string()],
            ["dwclass!(\"@xl:my-cls\")".to_string(), "large screens".to_string(), ">= 2560px".to_string()],
            [
                "dwclass!(\"@((max-width: 500px)):bg-candlelight-400)\")".to_string(),
                "Custom media query".to_string(),
                "-".to_string(),
            ],
        ]))
        .child(doc_page_sub_header("Responsive Example"))
        .child(example_box(breakpoint_example(), false))
        .child(code(&BREAKPOINT_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn breakpoint_example() -> Dom {
    html!("div", {
        .dwclass!("w-full @<sm:flex-col @sm:flex-row flex text-woodsmoke-950")
        .child(html!("div", {
            .dwclass!("bg-picton-blue-400")
            .dwclass!("@sm:w-40 h-40 flex-initial @((max-width: 700px)):bg-candlelight-400")
            .dwclass!("flex align-items-center justify-center")
            .text_signal(breakpoint_active_signal(Breakpoint::Small).map(|active| {
                if active {
                    "Horizontal"
                } else {
                    "Vertical"
                }
            }))
        }))
        .child(html!("div", {
            .dwclass!("w-full h-40 flex-initial bg-charm-400")
        }))
    })
}
