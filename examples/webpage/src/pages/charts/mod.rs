//! The dwind-dviz chart gallery: every chart, static and live, on the dark
//! surface and the light one. Also the renderer's visual test bed.
//!
//! Charts inherit the site's look through the `--dviz-accent` and
//! `--dviz-font-*` tokens set on the app root (see `main_view`); the light
//! preview switch below wraps the examples in `.light`, which is the same
//! ancestor class dwui and dwind-dviz key their light tokens on.

pub mod bars;
pub mod distributions;
pub mod interaction;
pub mod lines;
pub mod live;
pub mod tiles;

use crate::fx;
use crate::reveal::reveal_on_scroll;
use dominator::{clone, Dom};
use dwind::prelude::*;
use dwind_dviz::prelude::Mode;
use dwind_macros::dwclass;
use dwui::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};

/// Chapter index for the sticky rail. Ids double as anchor targets.
const SECTIONS: &[(&str, &str)] = &[
    ("live", "Live data"),
    ("interaction", "Interaction"),
    ("tiles", "Stat tiles"),
    ("lines", "Lines & areas"),
    ("bars", "Bars"),
    ("distributions", "Distributions"),
];

pub fn charts_page() -> Dom {
    let active = Mutable::new("live");
    let is_light = Mutable::new(false);

    html!("div", {
        .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 w-full m-b-20 flex flex-row gap-10")
        .child(html!("div", {
            .dwclass!("grow")
            .style("min-width", "0")
            .child(header(&is_light))
            // The `.light` class sits on this wrapper: `is(.light *)` variants
            // and the `--dviz-*` light tokens match descendants of it.
            .child(html!("div", {
                .class_signal("light", is_light.signal())
                .style_signal("--dviz-accent", is_light.signal().map(|l| {
                    if l { "#997E35" } else { "#D5B65F" }
                }))
                .child(html!("div", {
                    .dwclass!("flex flex-col rounded-lg transition-colors")
                    .dwclass!("is(.light *):bg-woodsmoke-50 is(.light *):p-6 is(.light *):m-t-4")
                    .child(section("live", "Live data", "A windowed source fed by an in-browser generator, committed once per frame and drawn downsampled to the plot width. Eight series, sixty seconds, no jitter.", live::page(), &active))
                    .child(section("interaction", "Interaction", "Crosshair and tooltip, keyboard navigation, legend toggling and brush-to-zoom. Every mark is reachable with Tab.", interaction::page(), &active))
                    .child(section("tiles", "Stat tiles", "When the story is one number: label, compact figure, a delta colored by direction, and a trend.", tiles::page(), &active))
                    .child(section("lines", "Lines & areas", "Presets for the common shapes, annotation layers for the rest, and small multiples when one plot would tangle.", lines::page(), &active))
                    .child(section("bars", "Bars", "Single, grouped, stacked and 100% stacked. Bars grow from zero, the rounded end is always the data end.", bars::page(), &active))
                    .child(section("distributions", "Distributions", "Scatter, heatmap and donut. Color is validated for color-vision deficiency in both modes; nine slices fold to five plus Other.", distributions::page(), &active))
                }))
            }))
        }))
        .child(html!("div", {
            .dwclass!("p-t-20 @<md:hidden")
            .child(fx::spy_rail(SECTIONS, active.clone()))
        }))
    })
}

fn header(is_light: &Mutable<bool>) -> Dom {
    html!("div", {
        .dwclass!("p-t-10 flex flex-col gap-3")
        .apply(reveal_on_scroll)
        .child(breadcrumbs!({
            .items(vec![
                ("home".to_string(), "#/".to_string()),
                ("charts".to_string(), "#/charts".to_string()),
            ])
        }))
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-candlelight-400 text-sm")
            .text("// dwind-dviz — chart gallery")
        }))
        .child(html!("h1", {
            .class("font-display")
            .dwclass!("@sm:text-5xl @<sm:text-3xl font-extrabold text-woodsmoke-50 m-0")
            .text("Charts that belong on the page.")
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 m-0")
            .style("max-width", "44rem")
            .text("dwind-dviz draws with the same tokens, faces and motion as the rest of \
                   a dwind app. Signals in, SVG out: no virtual DOM, no chart runtime, \
                   and a palette that passes colorblind checks on both surfaces.")
        }))
        .child(html!("div", {
            .dwclass!("flex align-items-center gap-4 m-t-3")
            .child(switch!({
                .checked_signal(is_light.signal())
                .label("Light surface preview".to_string())
                .on_change(clone!(is_light => move |checked| {
                    is_light.set(checked);
                    dwind_dviz::theme::set_mode(if checked { Mode::Light } else { Mode::Dark });
                }))
            }))
        }))
    })
}

/// `kicker` doubles as the section's anchor id and its scroll-spy key.
fn section(
    kicker: &'static str,
    title: &str,
    description: &str,
    content: Dom,
    active: &Mutable<&'static str>,
) -> Dom {
    html!("section", {
        .attr("id", kicker)
        .dwclass!("m-t-14 flex flex-col")
        .apply(reveal_on_scroll)
        .apply(fx::scroll_spy(kicker, active.clone()))
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-candlelight-400 is(.light *):text-candlelight-700 text-xs")
            .text(&format!("// {}", kicker))
        }))
        .child(html!("h2", {
            .class("font-display")
            .dwclass!("text-2xl font-bold m-0 m-t-1 text-woodsmoke-50 is(.light *):text-woodsmoke-950")
            .text(title)
        }))
        .child(html!("p", {
            .dwclass!("leading-relaxed m-0 m-t-2 text-woodsmoke-400 is(.light *):text-woodsmoke-600")
            .style("max-width", "44rem")
            .text(description)
        }))
        .child(html!("div", {
            .dwclass!("m-t-6")
            .child(content)
        }))
    })
}

/// One example: a display-face title, a line of prose, and the chart in a
/// glass preview frame with the site's mono header bar.
pub fn example(title: &str, description: &str, content: Dom) -> Dom {
    html!("div", {
        .dwclass!("flex flex-col")
        .child(html!("h3", {
            .class("font-display")
            .dwclass!("text-l font-bold m-0 text-woodsmoke-50 is(.light *):text-woodsmoke-950")
            .text(title)
        }))
        .child(html!("p", {
            .dwclass!("text-sm leading-relaxed m-0 m-t-1 text-woodsmoke-400 is(.light *):text-woodsmoke-600")
            .style("max-width", "44rem")
            .text(description)
        }))
        .child(preview(content))
    })
}

/// The preview frame: `example_box` from the docs, with light variants so
/// the surface switch above carries through.
fn preview(content: Dom) -> Dom {
    html!("div", {
        .apply(fx::glass)
        .apply(fx::spotlight)
        .dwclass!("m-t-4 rounded-lg border overflow-hidden w-full")
        .dwclass!("border-woodsmoke-800 is(.light *):border-woodsmoke-200")
        .dwclass!("is(.light *):[background:linear-gradient(160deg, rgba(255, 255, 255, 0.82) 0%, rgba(247, 247, 249, 0.7) 100%)]")
        .child(html!("div", {
            .dwclass!("flex flex-row justify-between align-items-center h-8 p-l-4 p-r-4 border-b")
            .dwclass!("border-woodsmoke-800 is(.light *):border-woodsmoke-200")
            .dwclass!("[background:rgba(18, 18, 21, 0.7)] is(.light *):[background:rgba(255, 255, 255, 0.7)]")
            .child(html!("span", {
                .class("font-code")
                .dwclass!("text-xs select-none text-woodsmoke-500")
                .text("// preview")
            }))
            .child(html!("span", {
                .class("font-code")
                .dwclass!("text-xs select-none text-woodsmoke-600 is(.light *):text-woodsmoke-400")
                .text("svg · signals")
            }))
        }))
        .child(html!("div", {
            .dwclass!("p-5 [background:rgba(2, 2, 3, 0.5)] is(.light *):[background:rgba(251, 251, 251, 0.55)]")
            .child(content)
        }))
    })
}
