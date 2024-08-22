use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::{media_query, Dom};
use dwind::prelude::media_queries::{breakpoint_active_signal, Breakpoint};
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_macro::example_html;
use futures_signals::signal::SignalExt;

pub fn responsive_design() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Responsive Design"))
        .text("Any modern web app must look good on on both small mobile devices, as well as on enormous desktop monitors.")
        .child(doc_page_sub_header("Breakpoints"))
        .text("To chose a breakpoint, we can use the @constraint syntax. The following breakpoints are available:")
        .child(breakpoint_table())
        .child(doc_page_sub_header("Responsive Example"))
        .child(example_box(breakpoint_example(), false))
        .child(code(&BREAKPOINT_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn breakpoint_example() -> Dom {
    html!("div", {
        .dwclass!("w-full @<sm:flex-col @sm:flex-row flex text-woodsmoke-950")
        .child(html!("div", {
            .dwclass!("@sm:w-40 h-40 flex-initial bg-picton-blue-400 flex align-items-center justify-center @((max-width: 700px)):bg-candlelight-400")
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

fn breakpoint_table() -> Dom {
    let breakpoints = vec![
        ("dwclass!(\"my-cls\")", "very small screens", "< 640px"),
        ("dwclass!(\"@sm:my-cls\")", "small screens", ">= 640px"),
        ("dwclass!(\"@md:my-cls\")", "medium screens", ">= 1280px"),
        ("dwclass!(\"@lg:my-cls\")", "large screens", ">= 1920px"),
        ("dwclass!(\"@xl:my-cls\")", "large screens", ">= 2560px"),
        ("dwclass!(\"@((max-width: 500px)):bg-candlelight-400)\")", "Custom media query", "-")
    ];

    html!("table", {
        .dwclass!("m-t-10 text-woodsmoke-400 divide-y border-collapse border-woodsmoke-900 w-full text-left text-sm")
        .child(html!("tr", {
            .children([
                html!("th", {
                    .dwclass!("p-b-2")
                    .text("Usage")
                }),
                html!("th", {
                    .dwclass!("p-b-2")
                    .text("Description")
                }),
                html!("th", {
                    .dwclass!("p-b-2")
                    .text("Size")
                })
            ])
        }))
        .children(breakpoints.into_iter().map(|(selector, description, size)| {
            html!("tr", {
                .dwclass!("border-woodsmoke-900")
                .children([
                    html!("td", {
                        .dwclass!("text-picton-blue-400 font-bold font-mono")
                        .dwclass!("p-t-2 p-b-2")
                        .text(selector)
                    }),
                    html!("td", {
                        .dwclass!("p-t-2 p-b-2")
                        .text(description)
                    }),
                    html!("td", {
                        .dwclass!("p-t-2 p-b-2")
                        .text(size)
                    })
                ])
            })
        }))
    })
}
