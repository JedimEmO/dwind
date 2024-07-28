use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::Dom;
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
        .child(example_box(breakpoint_table(), false))
        .child(doc_page_sub_header("Responsive Example"))
        .child(example_box(breakpoint_example(), false))
        .child(code(&BREAKPOINT_EXAMPLE_EXAMPLE_HTML_MAP))
    })
}

#[rustfmt::skip]
#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn breakpoint_example() -> Dom {
html!("div", {
    .dwclass!("flex w-full flex-col @sm:flex-row text-woodsmoke-950 font-extrabold")
    .child(html!("div", {
        .dwclass!("@sm:w-40 h-40 flex-initial bg-picton-blue-400 flex align-items-center justify-center")
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
    html!("table", {
        .dwclass!("text-woodsmoke-500 font-mono")
        .children([
            html!("tr", {
                .children([
                    html!("th", {
                        .text("Selector")
                    }),
                    html!("th", {
                        .text("Description")
                    }),
                    html!("th", {
                        .text("Size")
                    })
                ])
            }),
            html!("tr", {
                .children([
                    html!("td", {
                        .text("default")
                    }),
                    html!("td", {
                        .text("Very small screens")
                    }),
                    html!("td", {
                        .text("< 640")
                    })
                ])
            }),
            html!("tr", {
                .children([
                    html!("td", {
                        .text("@sm")
                    }),
                    html!("td", {
                        .text("Small screens")
                    }),
                    html!("td", {
                        .text("< 1280")
                    })
                ])
            }),
            html!("tr", {
                .children([
                    html!("td", {
                        .text("@md")
                    }),
                    html!("td", {
                        .text("Medium screens")
                    }),
                    html!("td", {
                        .text("< 1920")
                    })
                ])
            }),
            html!("tr", {
                .children([
                    html!("td", {
                        .text("@lg")
                    }),
                    html!("td", {
                        .text("Large screens")
                    }),
                    html!("td", {
                        .text("< 2560")
                    })
                ])
            }),
        ])
    })
}
