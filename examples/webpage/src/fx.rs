//! Pointer- and scroll-reactive effects, built the DOMINATOR way.
//!
//! Every effect in here is a plain `Mutable` feeding a `style_signal`. There is
//! no animation library, no `requestAnimationFrame` loop and no virtual DOM
//! diff: an event writes a number, the signal writes one CSS property on one
//! node, and the compositor does the rest.

use dominator::{events, html, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlElement;

/// Normalised pointer position inside an element, plus whether it is hovered.
#[derive(Clone, Default)]
struct PointerState {
    /// 0.0 – 1.0 across the element's width / height.
    pos: Mutable<(f64, f64)>,
    hot: Mutable<bool>,
}

impl PointerState {
    fn new() -> Self {
        Self {
            pos: Mutable::new((0.5, 0.5)),
            hot: Mutable::new(false),
        }
    }
}

/// Reads the pointer position relative to `element` and normalises it.
fn normalised(element: &HtmlElement, client_x: f64, client_y: f64) -> (f64, f64) {
    let rect = element.get_bounding_client_rect();
    let (w, h) = (rect.width().max(1.0), rect.height().max(1.0));

    (
        ((client_x - rect.left()) / w).clamp(0.0, 1.0),
        ((client_y - rect.top()) / h).clamp(0.0, 1.0),
    )
}

/// Wires pointer tracking into a builder and exposes the state.
///
/// `--sx` / `--sy` are written on every pointer move; the `.dw-spot` rules in
/// [`crate::styles`] park a radial gradient and a lit border edge there.
fn track_pointer(
    builder: DomBuilder<HtmlElement>,
    state: &PointerState,
) -> DomBuilder<HtmlElement> {
    let (pos, hot) = (state.pos.clone(), state.hot.clone());

    dominator::with_node!(builder, element => {
        .event({
            let pos = pos.clone();
            let element = element.clone();
            move |e: events::PointerMove| {
                pos.set_neq(normalised(&element, e.mouse_x() as f64, e.mouse_y() as f64));
            }
        })
        .event({
            let hot = hot.clone();
            move |_: events::PointerEnter| hot.set_neq(true)
        })
        .event({
            let hot = hot.clone();
            let pos = pos.clone();
            move |_: events::PointerLeave| {
                hot.set_neq(false);
                pos.set_neq((0.5, 0.5));
            }
        })
    })
    .class("dw-spot")
    .attr_signal(
        "data-hot",
        hot.signal().map(|h| Some(if h { "1" } else { "0" })),
    )
    .style_signal(
        "--sx",
        pos.signal().map(|(x, _)| format!("{:.2}%", x * 100.0)),
    )
    .style_signal(
        "--sy",
        pos.signal().map(|(_, y)| format!("{:.2}%", y * 100.0)),
    )
}

/// A card that lights up under the cursor.
///
/// ```ignore
/// html!("div", { .apply(spotlight) .child(...) })
/// ```
pub fn spotlight(builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    track_pointer(builder, &PointerState::new())
}

/// A spotlight card that also tips towards the cursor in 3D.
///
/// `strength` is the maximum rotation in degrees.
pub fn spotlight_tilt(
    strength: f64,
) -> impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    move |builder| {
        let state = PointerState::new();
        let (pos, hot) = (state.pos.clone(), state.hot.clone());

        track_pointer(builder, &state).style_signal(
            "transform",
            futures_signals::map_ref! {
                let (x, y) = pos.signal(),
                let hot = hot.signal() => move {
                    if *hot {
                        format!(
                            "perspective(1100px) rotateX({:.2}deg) rotateY({:.2}deg) translateZ(6px)",
                            (0.5 - *y) * strength * 2.0,
                            (*x - 0.5) * strength * 2.0,
                        )
                    } else {
                        "perspective(1100px) rotateX(0deg) rotateY(0deg) translateZ(0)".to_string()
                    }
                }
            },
        )
    }
}

/// A control that drifts toward the cursor while hovered — the classic
/// "magnetic button", in about twenty lines of signal plumbing.
pub fn magnetic(pull: f64) -> impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    move |builder| {
        let pos = Mutable::new((0.5f64, 0.5f64));
        let hot = Mutable::new(false);

        dominator::with_node!(builder, element => {
            .event({
                let pos = pos.clone();
                let element = element.clone();
                move |e: events::PointerMove| {
                    pos.set_neq(normalised(&element, e.mouse_x() as f64, e.mouse_y() as f64));
                }
            })
            .event({
                let hot = hot.clone();
                move |_: events::PointerEnter| hot.set_neq(true)
            })
            .event({
                let hot = hot.clone();
                let pos = pos.clone();
                move |_: events::PointerLeave| {
                    hot.set_neq(false);
                    pos.set_neq((0.5, 0.5));
                }
            })
        })
        .style(
            "transition",
            "transform 320ms cubic-bezier(0.16, 1, 0.3, 1)",
        )
        .style_signal(
            "transform",
            futures_signals::map_ref! {
                let (x, y) = pos.signal(),
                let hot = hot.signal() => move {
                    if *hot {
                        format!(
                            "translate3d({:.2}px, {:.2}px, 0)",
                            (*x - 0.5) * pull * 2.0,
                            (*y - 0.5) * pull * 2.0,
                        )
                    } else {
                        "translate3d(0, 0, 0)".to_string()
                    }
                }
            },
        )
    }
}

// ---------------------------------------------------------------------------
// Ambient background layers
// ---------------------------------------------------------------------------

/// Slowly drifting colour field behind the whole app.
pub fn aurora() -> Dom {
    html!("div", {
        .attr("aria-hidden", "true")
        .style("position", "fixed")
        .style("inset", "0")
        .style("z-index", "0")
        .style("pointer-events", "none")
        .style("overflow", "hidden")
        .child(aurora_blob(
            "radial-gradient(circle, rgba(213, 182, 95, 0.30) 0%, rgba(213, 182, 95, 0.08) 40%, transparent 70%)",
            "-22%", "54%", "66rem", "dwind-aurora-a 22s ease-in-out infinite",
        ))
        .child(aurora_blob(
            "radial-gradient(circle, rgba(95, 176, 213, 0.16) 0%, transparent 68%)",
            "38%", "-18%", "52rem", "dwind-aurora-b 28s ease-in-out infinite",
        ))
        .child(aurora_blob(
            "radial-gradient(circle, rgba(213, 95, 168, 0.10) 0%, transparent 70%)",
            "74%", "62%", "46rem", "dwind-aurora-a 34s ease-in-out infinite reverse",
        ))
    })
}

fn aurora_blob(background: &str, top: &str, left: &str, size: &str, animation: &str) -> Dom {
    html!("div", {
        .style("position", "absolute")
        .style("top", top)
        .style("left", left)
        .style("width", size)
        .style("height", size)
        .style("background", background)
        .style("filter", "blur(20px)")
        .style("will-change", "transform")
        .style("animation", animation)
    })
}

/// Fixed film-grain overlay. Costs one node for the whole document.
pub fn grain() -> Dom {
    html!("div", {
        .attr("aria-hidden", "true")
        .class("dw-grain")
    })
}

/// Faint engineering grid, masked to fade out towards the bottom.
pub fn blueprint_grid(mask: &str) -> Dom {
    html!("div", {
        .attr("aria-hidden", "true")
        .style("position", "absolute")
        .style("inset", "0")
        .style("pointer-events", "none")
        .style("background-image", "linear-gradient(rgba(125, 125, 135, 0.06) 1px, transparent 1px), linear-gradient(90deg, rgba(125, 125, 135, 0.06) 1px, transparent 1px)")
        .style("background-size", "48px 48px")
        .style("mask-image", mask)
        .style("-webkit-mask-image", mask)
    })
}

// ---------------------------------------------------------------------------
// Scroll spy
// ---------------------------------------------------------------------------

/// Reports into `active` whenever this element occupies the reading band —
/// the middle slice of the viewport. Pair with [`spy_rail`].
pub fn scroll_spy(
    id: &'static str,
    active: Mutable<&'static str>,
) -> impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    move |builder| {
        let active = active.clone();

        builder.after_inserted(move |element| {
            let callback = wasm_bindgen::prelude::Closure::<dyn FnMut(web_sys::js_sys::Array)>::new(
                move |entries: web_sys::js_sys::Array| {
                    for entry in entries.iter() {
                        let entry: web_sys::IntersectionObserverEntry =
                            wasm_bindgen::JsCast::unchecked_into(entry);

                        if entry.is_intersecting() {
                            active.set_neq(id);
                        }
                    }
                },
            );

            let options = web_sys::IntersectionObserverInit::new();
            options.set_root_margin("-25% 0px -60% 0px");

            if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
                wasm_bindgen::JsCast::unchecked_ref(callback.as_ref()),
                &options,
            ) {
                observer.observe(element.as_ref());
                // both live for the page lifetime
                std::mem::forget(observer);
                callback.forget();
            }
        })
    }
}

/// A sticky chapter index that highlights whatever [`scroll_spy`] last saw.
pub fn spy_rail(
    sections: &'static [(&'static str, &'static str)],
    active: Mutable<&'static str>,
) -> Dom {
    html!("nav", {
        .attr("aria-label", "On this page")
        .dwclass!("flex flex-col gap-1 flex-none w-44 @<md:hidden")
        .style("position", "sticky")
        .style("top", "6rem")
        .style("align-self", "flex-start")
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-xs text-woodsmoke-600 m-b-2")
            .text("// on this page")
        }))
        .children(sections.iter().map(move |(id, label)| {
            let active = active.clone();
            let id = *id;

            html!("a", {
                .attr("href", &format!("#{id}"))
                .dwclass!("flex flex-row align-items-center gap-2 text-sm cursor-pointer transition-colors")
                .dwclass!("text-woodsmoke-500 hover:text-candlelight-300 p-t-1 p-b-1")
                .style("text-decoration", "none")
                .style_signal("color", active.signal().map(move |a| {
                    if a == id { Some("#E5CE8F") } else { None }
                }))
                .child(html!("span", {
                    .attr("aria-hidden", "true")
                    .dwclass!("h-1 rounded-full flex-none transition-all")
                    .style("background", "currentColor")
                    .style_signal("width", active.signal().map(move |a| {
                        if a == id { "1.25rem" } else { "0.5rem" }
                    }))
                }))
                .child(dominator::text(label))
            })
        }))
    })
}

// ---------------------------------------------------------------------------
// Kinetic type
// ---------------------------------------------------------------------------

/// One headline word, animated in on its own delay.
pub fn word(content: &str, index: usize, accent: bool) -> Dom {
    html!("span", {
        .class("dw-word")
        .apply_if(accent, |b| b.class("dw-sheen"))
        .style("animation-delay", &format!("{}ms", 90 * index as u32))
        .style("padding-right", "0.26em")
        .text(content)
    })
}

/// Splits a headline into words and staggers them in. Words wrapped in `*` are
/// rendered with the animated gold sheen.
pub fn kinetic_headline(headline: &str) -> Vec<Dom> {
    headline
        .split_whitespace()
        .enumerate()
        .map(|(i, w)| {
            if let Some(stripped) = w.strip_prefix('*').and_then(|w| w.strip_suffix('*')) {
                word(stripped, i, true)
            } else {
                word(w, i, false)
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Marquee
// ---------------------------------------------------------------------------

/// An infinite horizontal ticker. The track is rendered twice so the
/// `-50%` keyframe loops seamlessly.
pub fn marquee(items: &[&str]) -> Dom {
    let chip = |label: &str| {
        html!("span", {
            .class("font-code")
            .dwclass!("text-xs text-woodsmoke-400 flex-none")
            .dwclass!("border border-woodsmoke-800 rounded-full p-l-4 p-r-4 p-t-2 p-b-2 m-r-3")
            .style("background", "rgba(18, 18, 21, 0.55)")
            .style("white-space", "nowrap")
            .text(label)
        })
    };

    html!("div", {
        .attr("aria-hidden", "true")
        .class("dw-marquee")
        .dwclass!("w-full overflow-hidden")
        .child(html!("div", {
            .class("dw-marquee-track")
            .children(items.iter().chain(items.iter()).map(|i| chip(i)))
        }))
    })
}
