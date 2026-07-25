//! Pointer- and scroll-reactive effects, built the DOMINATOR way.
//!
//! Every effect in here is a plain `Mutable` feeding a `style_signal`. There is
//! no animation library, no `requestAnimationFrame` loop and no virtual DOM
//! diff: an event writes a number, the signal writes one CSS property on one
//! node, and the compositor does the rest.

use crate::keyframes::*;
use dominator::{events, html, Dom, DomBuilder};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::signal::{Mutable, Signal, SignalExt};
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

/// Wires pointer tracking into a builder, and paints the two pseudo-elements
/// that follow the cursor.
///
/// The glow (`::before`) and the lit border edge (`::after`) are parked at
/// `--sx` / `--sy`, which the handlers below write on every pointer move. Both
/// used to be raw CSS: `content: ""` is now emitted automatically for
/// `::before` / `::after` variants, and the mask/blend declarations that have no
/// utility go through the arbitrary-declaration escape hatch.
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
    .apply(|b| dwclass!(b, "relative isolate [transition:transform 400ms cubic-bezier(0.16, 1, 0.3, 1), border-color 300ms ease]"))
    // The glow.
    .apply(|b| dwclass!(b, "\
        [&::before]:absolute [&::before]:inset-0 [&::before]:[z-index:-1] \
        [&::before]:[border-radius:inherit] [&::before]:opacity-0 \
        [&::before]:[transition:opacity 320ms ease] [&.hot::before]:opacity-100 \
        [&::before]:[background:radial-gradient(22rem circle at var(--sx, 50%) var(--sy, 50%), rgba(213, 182, 95, 0.13), transparent 62%)]"))
    // A one-pixel gradient ring, cut out of a solid fill with a mask so only the
    // border shows. Four co-dependent declarations, none with a utility.
    .apply(|b| dwclass!(b, "\
        [&::after]:absolute [&::after]:inset-0 [&::after]:[z-index:-1] \
        [&::after]:[border-radius:inherit] [&::after]:[padding:1px] \
        [&::after]:opacity-0 [&::after]:[transition:opacity 320ms ease] [&.hot::after]:opacity-100 \
        [&::after]:[background:radial-gradient(16rem circle at var(--sx, 50%) var(--sy, 50%), rgba(213, 182, 95, 0.55), transparent 55%)] \
        [&::after]:[-webkit-mask:linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0)] \
        [&::after]:[-webkit-mask-composite:xor] \
        [&::after]:[mask:linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0)] \
        [&::after]:[mask-composite:exclude]"))
    // The pseudo-element rules key off this class rather than a data attribute,
    // because a class is what a `[&.hot::before]:` variant can select.
    .class_signal("hot", hot.signal())
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

/// Whether the reader has asked for less motion.
///
/// A CSS `@media` block can shorten a transition, but it cannot stop a
/// `style_signal` from writing a transform in the first place. The effects below
/// are driven from Rust, so the preference has to be read in Rust too — as a
/// signal, so it also tracks a change made while the page is open.
fn prefers_reduced_motion() -> impl Signal<Item = bool> {
    dominator::media_query("(prefers-reduced-motion: reduce)")
}

/// The 3D tilt transform for a card, or the resting one.
///
/// Split out from the signal wiring so the reduced-motion rule is a plain
/// assertion rather than something only a browser can check.
fn tilt_transform(x: f64, y: f64, hot: bool, still: bool, strength: f64) -> String {
    if !hot || still {
        return "perspective(1100px) rotateX(0deg) rotateY(0deg) translateZ(0)".to_string();
    }

    format!(
        "perspective(1100px) rotateX({:.2}deg) rotateY({:.2}deg) translateZ(6px)",
        (0.5 - y) * strength * 2.0,
        (x - 0.5) * strength * 2.0,
    )
}

/// The magnetic offset for a control, or the resting one.
fn magnetic_transform(x: f64, y: f64, hot: bool, still: bool, pull: f64) -> String {
    if !hot || still {
        return "translate3d(0, 0, 0)".to_string();
    }

    format!(
        "translate3d({:.2}px, {:.2}px, 0)",
        (x - 0.5) * pull * 2.0,
        (y - 0.5) * pull * 2.0,
    )
}

/// A spotlight card that also tips towards the cursor in 3D.
///
/// `strength` is the maximum rotation in degrees. The tilt is suppressed
/// entirely under `prefers-reduced-motion`.
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
                let hot = hot.signal(),
                let still = prefers_reduced_motion() =>
                    move { tilt_transform(*x, *y, *hot, *still, strength) }
            },
        )
    }
}

/// A control that drifts toward the cursor while hovered — the classic
/// "magnetic button", in about twenty lines of signal plumbing.
///
/// Suppressed under `prefers-reduced-motion`, for the same reason as
/// [`spotlight_tilt`]: this is a `style_signal` write, so no `@media` block can
/// stop it.
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
                let hot = hot.signal(),
                let still = prefers_reduced_motion() =>
                    move { magnetic_transform(*x, *y, *hot, *still, pull) }
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
        .dwclass!("fixed inset-0 z-0 pointer-events-none overflow-hidden")
        .child(aurora_blob(
            "radial-gradient(circle, rgba(213, 182, 95, 0.30) 0%, rgba(213, 182, 95, 0.08) 40%, transparent 70%)",
            "-22%", "54%", "66rem",
            &format!("{AURORA_A_KEYFRAMES} 22s ease-in-out infinite"),
        ))
        .child(aurora_blob(
            "radial-gradient(circle, rgba(95, 176, 213, 0.16) 0%, transparent 68%)",
            "38%", "-18%", "52rem",
            &format!("{AURORA_B_KEYFRAMES} 28s ease-in-out infinite"),
        ))
        .child(aurora_blob(
            "radial-gradient(circle, rgba(213, 95, 168, 0.10) 0%, transparent 70%)",
            "74%", "62%", "46rem",
            &format!("{AURORA_A_KEYFRAMES} 34s ease-in-out infinite reverse"),
        ))
    })
}

fn aurora_blob(background: &str, top: &str, left: &str, size: &str, animation: &str) -> Dom {
    html!("div", {
        .dwclass!("absolute will-change-transform [filter:blur(20px)]")
        .style("top", top)
        .style("left", left)
        .style("width", size)
        .style("height", size)
        .style("background", background)
        .style("animation", animation)
    })
}

/// Fixed film-grain overlay. Costs one node for the whole document.
///
/// The texture is an inline `feTurbulence` SVG. That data URI stays a plain
/// `.style()` — it is a one-off asset, not a reusable utility value.
pub fn grain() -> Dom {
    html!("div", {
        .attr("aria-hidden", "true")
        .dwclass!("fixed inset-0 pointer-events-none opacity-20 mix-blend-overlay [z-index:9998]")
        .style("background-image", "url(\"data:image/svg+xml,%3Csvg xmlns=\'http://www.w3.org/2000/svg\' width=\'140\' height=\'140\'%3E%3Cfilter id=\'n\'%3E%3CfeTurbulence type=\'fractalNoise\' baseFrequency=\'0.85\' numOctaves=\'3\' stitchTiles=\'stitch\'/%3E%3CfeColorMatrix type=\'saturate\' values=\'0\'/%3E%3C/filter%3E%3Crect width=\'140\' height=\'140\' filter=\'url(%23n)\' opacity=\'0.5\'/%3E%3C/svg%3E\")")
    })
}

/// Faint engineering grid, masked to fade out towards the bottom.
pub fn blueprint_grid(mask: &str) -> Dom {
    html!("div", {
        .attr("aria-hidden", "true")
        .dwclass!("absolute inset-0 pointer-events-none \
            [background-image:linear-gradient(rgba(125, 125, 135, 0.06) 1px, transparent 1px), linear-gradient(90deg, rgba(125, 125, 135, 0.06) 1px, transparent 1px)] \
            [background-size:48px 48px]")
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
        .dwclass!("sticky")
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
                .dwclass!("no-underline")
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

/// Gold gradient text with a sheen that sweeps across it.
///
/// `background-clip: text` needs three co-dependent declarations and has no
/// utility, so it goes through the escape hatch — but it is still one
/// compile-checked mixin rather than a stylesheet rule.
pub fn sheen(builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dwclass!(
        builder,
        "animate-sheen \
         [background-image:linear-gradient(100deg, #F0E2B6 0%, #D5B65F 18%, #FFF8E2 30%, #D5B65F 42%, #A88735 60%, #D5B65F 100%)] \
         [background-size:200% auto] \
         [-webkit-background-clip:text] [background-clip:text] [color:transparent]"
    )
}

/// One headline word, animated in on its own delay.
pub fn word(content: &str, index: usize, accent: bool) -> Dom {
    html!("span", {
        .dwclass!("inline-block [padding-right:0.26em]")
        .apply_if(accent, sheen)
        // Formatting the handle registers the @keyframes, so a computed delay
        // can never reference a rule that was not injected.
        .style("animation", &format!(
            "{WORD_IN_KEYFRAMES} 900ms {}ms cubic-bezier(0.16, 1, 0.3, 1) both",
            90 * index as u32,
        ))
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
            .dwclass!("text-xs text-woodsmoke-400 flex-none whitespace-nowrap")
            .dwclass!("border border-woodsmoke-800 rounded-full p-l-4 p-r-4 p-t-2 p-b-2 m-r-3")
            .dwclass!("[background:rgba(18, 18, 21, 0.55)]")
            .text(label)
        })
    };

    html!("div", {
        .attr("aria-hidden", "true")
        .dwclass!("w-full overflow-hidden \
            [mask-image:linear-gradient(90deg, transparent, #000 12%, #000 88%, transparent)] \
            [-webkit-mask-image:linear-gradient(90deg, transparent, #000 12%, #000 88%, transparent)]")
        // Hover on the parent, effect on the child — the classic case that no
        // element-level class can express, and a plain child variant here.
        .dwclass!("[&:hover > *]:[animation-play-state:paused]")
        .child(html!("div", {
            .dwclass!("flex animate-marquee [width:max-content]")
            .children(items.iter().chain(items.iter()).map(|i| chip(i)))
        }))
    })
}

// ---------------------------------------------------------------------------
// Surfaces
// ---------------------------------------------------------------------------

/// The translucent panel used by cards, the header and the docs sidebar.
///
/// Three co-dependent declarations that only make sense together — a component
/// surface rather than a utility, so it lives here as one named mixin.
pub fn glass(builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dwclass!(
        builder,
        "[background:linear-gradient(160deg, rgba(28, 28, 33, 0.72) 0%, rgba(14, 14, 17, 0.62) 100%)] \
         [backdrop-filter:blur(14px) saturate(1.2)] \
         [box-shadow:inset 0 1px 0 0 rgba(255, 255, 255, 0.045)]"
    )
}

/// Thin scrollbars that match the surface they sit on.
///
/// The standard `scrollbar-width` / `scrollbar-color` properties only — no
/// `::-webkit-scrollbar` pseudo-elements. Two reasons:
///
/// 1. They are no longer needed. Firefox has supported the standard properties
///    since 64 and Chrome since 121.
/// 2. `::-webkit-scrollbar-thumb:hover` **crashes Firefox**. A selector that the
///    browser cannot parse makes `insertRule` throw, and dominator panics rather
///    than skipping it (`dom.rs:1691`), so the whole app dies on load. A raw
///    stylesheet would have ignored the rule silently — moving these selectors
///    into `dwclass!` is what made an unsupported selector fatal.
pub fn slim_scrollbar(builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dwclass!(
        builder,
        "[scrollbar-width:thin] [scrollbar-color:#26262C transparent]"
    )
}

#[cfg(test)]
mod test {
    use super::*;

    // `prefers-reduced-motion` is honoured in CSS for animations and
    // transitions, but these two effects are `style_signal` writes that no
    // `@media` block can reach — so the rule lives in Rust and is asserted here.

    #[test]
    fn tilt_is_suppressed_under_reduced_motion() {
        let moving = tilt_transform(0.2, 0.25, true, false, 4.0);
        let still = tilt_transform(0.2, 0.25, true, true, 4.0);
        let resting = tilt_transform(0.2, 0.25, false, false, 4.0);

        assert!(moving.contains("rotateX(2.00deg)"), "{moving}");
        assert_eq!(still, resting);
        assert!(still.contains("rotateX(0deg)"), "{still}");
        assert!(still.contains("rotateY(0deg)"), "{still}");
    }

    #[test]
    fn magnetic_drift_is_suppressed_under_reduced_motion() {
        let moving = magnetic_transform(1.0, 1.0, true, false, 7.0);
        let still = magnetic_transform(1.0, 1.0, true, true, 7.0);
        let resting = magnetic_transform(1.0, 1.0, false, false, 7.0);

        assert!(
            moving.contains("translate3d(7.00px, 7.00px, 0)"),
            "{moving}"
        );
        assert_eq!(still, "translate3d(0, 0, 0)");
        assert_eq!(still, resting);
    }
}
