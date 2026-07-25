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
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text("Animation utility classes — toggle each one to start and stop it.")
        }))
        .child(doc_page_sub_header("Spinning"))
        .child(example_box(animation_examples(), false))
        .child(code(&ANIMATION_EXAMPLES_EXAMPLE_HTML_MAP))

        .child(doc_page_sub_header("Your own keyframes"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-0")
            .text("A utility class is a single declaration block, so a @keyframes can never be one. \
                   Declare them with dwkeyframes! instead: it emits the at-rule and, when you give it \
                   an #[animation(...)], a matching animate-* class that rustc still checks. The rule \
                   is injected the first time something uses it, and never twice.")
        }))
        .child(html!("pre", {
            .class("font-code")
            .dwclass!("text-sm p-4 m-0 m-t-4 rounded-lg border border-woodsmoke-800 text-woodsmoke-200 overflow-x-auto")
            .dwclass!("[background:rgba(2, 2, 3, 0.7)]")
            .text(r#"dwkeyframes! {
    #[animation("900ms cubic-bezier(0.16, 1, 0.3, 1) both")]
    fade_up {
        "from" => "opacity: 0; transform: translateY(14px);",
        "to"   => "opacity: 1; transform: translateY(0);",
    }
}

html!("div", { .dwclass!("animate-fade-up") })"#)
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-t-4 m-b-0")
            .text("Every CSS fragment is a string literal, because Rust's lexer splits 0%, --sx and .35 \
                   in ways that do not survive a round trip through the token stream.")
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-t-4 m-b-0")
            .text("When the shorthand has to be built at runtime — a per-item delay, say — use the handle \
                   directly. Formatting it registers the rule, so the animation can never point at a \
                   keyframe that was never injected:")
        }))
        .child(html!("pre", {
            .class("font-code")
            .dwclass!("text-sm p-4 m-0 m-t-2 rounded-lg border border-woodsmoke-800 text-woodsmoke-200 overflow-x-auto")
            .dwclass!("[background:rgba(2, 2, 3, 0.7)]")
            .text(".style(\"animation\", &format!(\"{FADE_UP_KEYFRAMES} 600ms {}ms ease-out both\", i * 60))")
        }))

        .child(doc_page_sub_header("Reduced motion"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-0")
            .text("Motion preferences are just another media query, so the @(( )) conditional covers them \
                   with no extra machinery:")
        }))
        .child(html!("pre", {
            .class("font-code")
            .dwclass!("text-sm p-4 m-0 m-t-2 rounded-lg border border-woodsmoke-800 text-woodsmoke-200 overflow-x-auto")
            .dwclass!("[background:rgba(2, 2, 3, 0.7)]")
            .text("dwclass!(\"animate-spin @((prefers-reduced-motion: reduce)):animate-none\")")
        }))
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
                            .text("Toggle bounce")
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
