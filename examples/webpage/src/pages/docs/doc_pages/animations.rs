use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::{dwclass, dwclass_signal};
use dwui::prelude::*;
use example_html_highlight_macro::example_html;
use futures_signals::signal::Mutable;

pub fn animation_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Animations"))
        .text("Animation utility classes")
        .child(doc_page_sub_header("Spinning"))
        .child(example_box(animation_examples(), false))
        .child(code(&ANIMATION_EXAMPLES_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn animation_examples() -> Dom {
    let is_spinning = Mutable::new(true);
    let is_pinging = Mutable::new(true);
    let is_pulsing = Mutable::new(true);
    let is_bouncing = Mutable::new(true);

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .children([
            html!("div", {
                .dwclass!("flex gap-4 align-items-center")
                .children([
                    button!({
                        .apply(|b| {
                            dwclass!(b, "px-4")
                        })
                        .on_click(clone!(is_spinning => move |_| {
                            is_spinning.set(!is_spinning.get());
                        }))
                        .content(html!("span", {
                            .text("Toggle spinning")
                        }).into())
                    }),
                    html!("div", {
                        .dwclass!("w-8 h-8 bg-red-500")
                        .dwclass_signal!("animate-spin", is_spinning.signal())
                        .dwclass_signal!("bg-red-200", is_spinning.signal())
                    })
                ])
            }),
            html!("div", {
                .dwclass!("flex gap-4 align-items-center")
                .children([
                    button!({
                        .apply(|b| {
                            dwclass!(b, "px-4")
                        })
                        .on_click(clone!(is_pinging => move |_| {
                            is_pinging.set(!is_pinging.get());
                        }))
                        .content(html!("span", {
                            .text("Toggle ping")
                        }).into())
                    }),
                    html!("div", {
                        .dwclass!("w-8 h-8 bg-red-500")
                        .dwclass_signal!("animate-ping", is_pinging.signal())
                        .dwclass_signal!("bg-red-200", is_pinging.signal())
                    })
                ])
            }),
            html!("div", {
                .dwclass!("flex gap-4 align-items-center")
                .children([
                    button!({
                        .apply(|b| {
                            dwclass!(b, "px-4")
                        })
                        .on_click(clone!(is_pulsing => move |_| {
                            is_pulsing.set(!is_pulsing.get());
                        }))
                        .content(html!("span", {
                            .text("Toggle pulse")
                        }).into())
                    }),
                    html!("div", {
                        .dwclass!("w-8 h-8 bg-red-500")
                        .dwclass_signal!("animate-pulse", is_pulsing.signal())
                        .dwclass_signal!("bg-red-200", is_pulsing.signal())
                    })
                ])
            }),
            html!("div", {
                .dwclass!("flex gap-4 align-items-center")
                .children([
                    button!({
                        .apply(|b| {
                            dwclass!(b, "px-4")
                        })
                        .on_click(clone!(is_bouncing => move |_| {
                            is_bouncing.set(!is_bouncing.get());
                        }))
                        .content(html!("span", {
                            .text("Toggle pulse")
                        }).into())
                    }),
                    html!("div", {
                        .dwclass!("w-8 h-8 bg-red-500")
                        .dwclass_signal!("animate-bounce", is_bouncing.signal())
                        .dwclass_signal!("bg-red-200", is_bouncing.signal())
                    })
                ])
            })
        ])
    })
}
