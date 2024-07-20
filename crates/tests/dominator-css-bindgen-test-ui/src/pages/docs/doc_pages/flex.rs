use dominator::Dom;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use dwind::prelude::*;
use dwind_macros::{dwclass};
use crate::pages::docs::doc_pages::doc_page::doc_page_title;
use dwind_macros::dwgenerate;
use dwind::background_scratched_generator;

pub fn flex_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Flex"))
        .text("Using flex display for adaptable containers")
        .child(flex_example_1())
        .child(flex_example_1_code())
    })
}
dwgenerate!("background-scratched", "background-scratched-[#2b55a2,#5a6779]");

fn flex_example_1() -> Dom {
    html!("div", {
        .dwclass!("rounded-lg bg-bunker-950 w-full h-32")
        .dwclass!("border border-color-manatee-800")
        .dwclass!("flex align-items-center p-5 m-t-10")
        .child(html!("div", {
            .dwclass!("rounded-lg background-scratched flex gap-4 align-items-center w-full h-p-60")
            .children(
                (0..3).map(|v| {
                    html!("div", {
                        .apply(|b| {
                            match v {
                                0 => dwclass!(b, "w-14 bg-candlelight-900 flex-none"),
                                1 => dwclass!(b, "w-64 bg-candlelight-600 flex-initial"),
                                _ => dwclass!(b, "w-32 bg-candlelight-600 flex-initial"),
                            }
                        })
                        .dwclass!("h-full")
                        .dwclass!("flex align-items-center justify-center")
                        .dwclass!("rounded-lg font-extrabold")
                        .text(v.to_string().as_str())
                    })
                })
            )
        }))
    })
}

fn flex_example_1_code() -> Dom {
    html!("div", {
        .dwclass!("rounded-lg w-full m-t-10")
        .dwclass!("border border-color-manatee-800")
        .child(code(r#"html!("div", {
    .dwclass!("rounded-lg background-scratched flex gap-4 align-items-center w-full h-p-60")
    .children(
        (0..3).map(|v| {
            html!("div", {
                .apply(|b| {
                    match v {
                        0 => dwclass!(b, "w-14 bg-candlelight-900 flex-none"),
                        1 => dwclass!(b, "w-64 bg-candlelight-600 flex-initial"),
                        _ => dwclass!(b, "w-32 bg-candlelight-600 flex-initial"),
                    }
                })
                .dwclass!("h-full")
                .dwclass!("flex align-items-center justify-center")
                .dwclass!("rounded-lg font-extrabold")
                .text(v.to_string().as_str())
            })
        })
    )
})"#))
    })
}

fn code(code: &str) -> Dom {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let sr = syntax_set.find_syntax_by_extension("rs").unwrap();
    let themes = ThemeSet::load_defaults();
    let theme = &themes.themes["base16-ocean.dark"];
    let content = highlighted_html_for_string(code, &syntax_set, sr, &theme).unwrap();

    html!("pre", {
        .child(html!("code", {
            .with_node!(element => {
                .apply(|b| {
                    element.set_inner_html(content.as_str());
                    b
                })
            })
        }))
    })
}