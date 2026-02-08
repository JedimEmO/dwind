use dominator::{events, html, text, Dom};
use dwind::prelude::*;
use dwind_glass::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};

pub fn code_example(code: &'static str) -> Dom {
    let show = Mutable::new(false);

    html!("div", {
        .dwclass!("flex flex-col gap-2")

        .child({
            let show = show.clone();
            html!("div", {
                .child(glass_button!({
                    .content_signal(show.signal().map(|s| {
                        Some(text(if s { "Hide Code" } else { "View Code" }))
                    }))
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Small)
                    .on_click(Box::new(move |_: events::Click| {
                        show.set(!show.get());
                    }))
                }))
            })
        })

        .child_signal(show.signal().map(move |visible| {
            if visible {
                Some(html!("div", {
                    .style("background", "var(--glass-bg-inset)")
                    .style("border-radius", "var(--glass-border-radius)")
                    .style("box-shadow", "var(--glass-shadow-inset)")
                    .style("padding", "1rem")
                    .style("overflow-x", "auto")
                    .child(html!("pre", {
                        .style("margin", "0")
                        .child(html!("code", {
                            .dwclass!("font-mono text-sm")
                            .style("line-height", "1.6")
                            .style("color", "var(--glass-text-secondary)")
                            .text(code)
                        }))
                    }))
                }))
            } else {
                None
            }
        }))
    })
}
