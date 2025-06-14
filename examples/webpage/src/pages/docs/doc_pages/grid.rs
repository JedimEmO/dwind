use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::Dom;
use dwind::background_scratched_generator;
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwind_macros::dwgenerate;
use example_html_highlight_macro::example_html;

pub fn grid_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Grid"))
        .text("Using CSS Grid for powerful layout systems")
        .child(doc_page_sub_header("Basic Grid"))
        .child(html!("p", {
            .text("Create responsive grid layouts with specified column counts")
        }))
        .child(example_box(grid_example_basic(), true))
        .child(code(&GRID_EXAMPLE_BASIC_EXAMPLE_HTML_MAP))
        .child(doc_page_sub_header("Column Spanning"))
        .child(html!("p", {
            .text("Span elements across multiple columns")
        }))
        .child(example_box(grid_example_col_span(), true))
        .child(code(&GRID_EXAMPLE_COL_SPAN_EXAMPLE_HTML_MAP))
        .child(doc_page_sub_header("Row Spanning"))
        .child(html!("p", {
            .text("Span elements across multiple rows")
        }))
        .child(example_box(grid_example_row_span(), true))
        .child(code(&GRID_EXAMPLE_ROW_SPAN_EXAMPLE_HTML_MAP))
        .child(doc_page_sub_header("Responsive Grid"))
        .child(html!("p", {
            .text("Adapt grid layouts based on container size")
        }))
        .child(example_box(grid_example_responsive(), true))
        .child(code(&GRID_EXAMPLE_RESPONSIVE_EXAMPLE_HTML_MAP))
    })
}

dwgenerate!(
    "background-scratched",
    "background-scratched-[#1c2e4f,#191919]"
);

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn grid_example_basic() -> Dom {
    html!("div", {
        .dwclass!("w-full rounded-lg background-scratched p-4")
        .child(html!("div", {
            .dwclass!("w-full grid grid-cols-3")
            .children(
                (1..=9).map(|i| {
                    html!("div", {
                        .dwclass!("bg-apple-700 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                        .text(&i.to_string())
                    })
                })
            )
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn grid_example_col_span() -> Dom {
    html!("div", {
        .dwclass!("w-full rounded-lg background-scratched p-4")
        .child(html!("div", {
            .dwclass!("w-full grid grid-cols-4")
            .children([
                html!("div", {
                    .dwclass!("bg-charm-700 rounded-lg flex align-items-center justify-center h-20 font-bold col-span-2 m-2")
                    .text("Span 2")
                }),
                html!("div", {
                    .dwclass!("bg-charm-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("1")
                }),
                html!("div", {
                    .dwclass!("bg-charm-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("2")
                }),
                html!("div", {
                    .dwclass!("bg-charm-700 rounded-lg flex align-items-center justify-center h-20 font-bold col-span-3 m-2")
                    .text("Span 3")
                }),
                html!("div", {
                    .dwclass!("bg-charm-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("3")
                }),
                html!("div", {
                    .dwclass!("bg-charm-700 rounded-lg flex align-items-center justify-center h-20 font-bold col-span-full m-2")
                    .text("Full Width")
                }),
            ])
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn grid_example_row_span() -> Dom {
    html!("div", {
        .dwclass!("w-full rounded-lg background-scratched p-4")
        .child(html!("div", {
            .dwclass!("w-full grid grid-cols-3")
            .children([
                html!("div", {
                    .dwclass!("bg-candlelight-700 rounded-lg flex align-items-center justify-center font-bold row-span-2 m-2")
                    .text("Span 2 Rows")
                }),
                html!("div", {
                    .dwclass!("bg-candlelight-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("1")
                }),
                html!("div", {
                    .dwclass!("bg-candlelight-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("2")
                }),
                html!("div", {
                    .dwclass!("bg-candlelight-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("3")
                }),
                html!("div", {
                    .dwclass!("bg-candlelight-700 rounded-lg flex align-items-center justify-center font-bold row-span-2 m-2")
                    .text("Span 2 Rows")
                }),
                html!("div", {
                    .dwclass!("bg-candlelight-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("4")
                }),
                html!("div", {
                    .dwclass!("bg-candlelight-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("5")
                }),
                html!("div", {
                    .dwclass!("bg-candlelight-600 rounded-lg flex align-items-center justify-center h-20 font-bold m-2")
                    .text("6")
                }),
            ])
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn grid_example_responsive() -> Dom {
    html!("div", {
        .dwclass!("w-full rounded-lg background-scratched p-4")
        .child(html!("div", {
            .dwclass!("w-full grid grid-cols-2 @md:grid-cols-3 @lg:grid-cols-4")
            .children(
                (1..=8).map(|i| {
                    html!("div", {
                        .dwclass!("bg-apple-600 rounded-lg flex align-items-center justify-center h-16 @md:h-20 font-bold m-2")
                        .text(&i.to_string())
                    })
                })
            )
        }))
    })
}
