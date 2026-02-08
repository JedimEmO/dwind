use crate::prelude::*;
use dominator::{events, html, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    Full,
}

#[component(render_fn = glass_modal)]
struct GlassModal {
    #[signal]
    #[default(None)]
    content: Option<Dom>,

    #[signal]
    #[default(false)]
    open: bool,

    #[default(Box::new(|| {}))]
    on_close: dyn Fn() -> () + 'static,

    #[signal]
    #[default(ModalSize::Medium)]
    size: ModalSize,

    #[signal]
    #[default(true)]
    close_on_backdrop_click: bool,

    #[signal]
    #[default(None)]
    title: Option<String>,
}

pub fn glass_modal(props: GlassModalProps) -> Dom {
    let GlassModalProps {
        content,
        open,
        on_close,
        size,
        close_on_backdrop_click: _,
        title,
        apply,
    } = props;

    let open = open.broadcast();
    let on_close = std::rc::Rc::new(on_close);

    html!("div", {
        // Overlay container — always in DOM, visibility controlled by CSS
        .dwclass!("fixed inset-0 flex items-center justify-center")
        .style("z-index", "1000")
        .style_signal("pointer-events", open.signal().map(|o| {
            if o { "auto" } else { "none" }
        }))
        .style_signal("opacity", open.signal().map(|o| {
            if o { "1" } else { "0" }
        }))
        .style("transition", "opacity var(--glass-transition) var(--glass-ease)")

        // Backdrop
        .child(html!("div", {
            .dwclass!("absolute inset-0")
            .style("background", "rgba(0, 0, 0, 0.4)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(4px)")
            .event({
                let on_close = on_close.clone();
                move |_: events::Click| {
                    (on_close)();
                }
            })
        }))

        // Modal panel — always in DOM so title/content signals are consumed once inside it
        .child(html!("div", {
            .dwclass!("relative flex flex-col glass-text-primary")
            .style("background", "\
                linear-gradient(to bottom, rgba(255, 255, 255, 0.08) 0%, transparent 40%), \
                var(--glass-bg-elevated)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
            .style("border", "none")
            .style("border-radius", "var(--glass-border-radius-xl)")
            .style("box-shadow", "var(--glass-shadow-xl)")
            .style("max-height", "85vh")
            .style("overflow", "hidden")
            .style_signal("width", size.map(|s| match s {
                ModalSize::Small => "24rem",
                ModalSize::Medium => "36rem",
                ModalSize::Large => "56rem",
                ModalSize::Full => "90vw",
            }))
            .style("z-index", "1001")
            .style("transition", "transform var(--glass-transition) var(--glass-ease)")
            .style_signal("transform", open.signal().map(|o| {
                if o { "scale(1)" } else { "scale(0.97)" }
            }))

            .attr("role", "dialog")
            .attr("aria-modal", "true")

            // Title header inside the panel
            .child_signal(title.map({
                let on_close = on_close.clone();
                move |t| {
                    t.map(|title_text| {
                        html!("div", {
                            .dwclass!("flex items-center justify-between px-6 py-4")
                            .style("background", "rgba(255, 255, 255, 0.04)")
                            .style("border-bottom", "1px solid rgba(255, 255, 255, 0.06)")
                            .child(html!("h2", {
                                .dwclass!("text-lg font-semibold glass-text-primary")
                                .text(&title_text)
                            }))
                            .child(html!("button", {
                                .dwclass!("cursor-pointer glass-text-secondary hover:glass-text-primary transition-all")
                                .style("background", "none")
                                .style("border", "none")
                                .style("font-size", "1.25rem")
                                .style("line-height", "1")
                                .style("padding", "4px")
                                .text("\u{2715}")
                                .event({
                                    let on_close = on_close.clone();
                                    move |_: events::Click| {
                                        (on_close)();
                                    }
                                })
                            }))
                        })
                    })
                }
            }))

            // Content body inside the panel
            .child(html!("div", {
                .dwclass!("p-6")
                .style("overflow-y", "auto")
                .child_signal(content)
            }))
        }))

        // Escape key to close
        .global_event({
            let on_close = on_close.clone();
            move |e: events::KeyDown| {
                if e.key() == "Escape" {
                    (on_close)();
                }
            }
        })

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
