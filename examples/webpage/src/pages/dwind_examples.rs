//! The dwind utility showcase: every artifact on this page is built from
//! dwind utility classes alone — no dwui components anywhere.

use crate::fx;
use crate::pages::docs::code_widget::code;
use crate::pages::docs::example_box::example_box;
use crate::reveal::reveal_on_scroll;
use dominator::Dom;
use dwind::background_scratched_generator;
use dwind::bg_color_generator;
use dwind::prelude::*;
use dwind::width_generator;
use dwind_macros::{dwclass, dwgenerate};
use example_html_highlight_macro::example_html;
use futures_signals::signal::Mutable;

/// Chapter index for the sticky rail. Ids double as anchor targets.
const SECTIONS: &[(&str, &str)] = &[
    ("layout", "Responsive layout"),
    ("color", "Gradients & palettes"),
    ("typography", "Type scale"),
    ("depth", "Glass & depth"),
    ("motion", "Transitions"),
    ("selectors", "Variants"),
    ("composition", "Composition"),
    ("generators", "Generators"),
];

pub fn dwind_examples_page() -> Dom {
    let active = Mutable::new("layout");

    html!("div", {
        .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 w-full m-b-20 flex flex-row gap-10")
        .child(html!("div", {
            .dwclass!("grow")
            .style("min-width", "0")
            .child(examples_body(&active))
        }))
        .child(html!("div", {
            .dwclass!("p-t-20 @<md:hidden")
            .child(fx::spy_rail(SECTIONS, active.clone()))
        }))
    })
}

fn examples_body(active: &Mutable<&'static str>) -> Dom {
    let active = active.clone();

    html!("div", {
        .dwclass!("w-full")
        .child(html!("div", {
            .dwclass!("p-t-10 flex flex-col gap-3")
            .apply(reveal_on_scroll)
            .child(html!("div", {
                .class("font-code")
                .dwclass!("text-candlelight-400 text-sm")
                .text("// dwind — utility showcase")
            }))
            .child(html!("h1", {
                .class("font-display")
                .dwclass!("@sm:text-5xl @<sm:text-3xl font-extrabold text-woodsmoke-50 m-0")
                .text("Built with utilities alone.")
            }))
            .child(html!("p", {
                .dwclass!("text-woodsmoke-300 m-0")
                .style("max-width", "44rem")
                .text("Everything below is composed purely from dwind utility classes — \
                       no components, no custom stylesheets. Each example ships its exact \
                       source; expand \"view source\" to read the dwclass! calls.")
            }))
        }))

        .child(section(
            "layout",
            "Responsive layout",
            "Flexbox utilities with breakpoint prefixes. Drag the handle: below the small \
             breakpoint the bar stacks vertically.",
            example_box(responsive_navbar(), true),
            code(&RESPONSIVE_NAVBAR_EXAMPLE_HTML_MAP),
            &active,
        ))

        .child(section(
            "color",
            "Gradients & palettes",
            "Every palette shade generates gradient-from/to classes; linear-gradient-{angle} \
             composes them at any rotation.",
            example_box(gradient_showcase(), false),
            code(&GRADIENT_SHOWCASE_EXAMPLE_HTML_MAP),
            &active,
        ))

        .child(section(
            "typography",
            "Type scale",
            "Sizes, weights, leading, and font family utilities — from captions to display type.",
            example_box(typography_specimen(), false),
            code(&TYPOGRAPHY_SPECIMEN_EXAMPLE_HTML_MAP),
            &active,
        ))

        .child(section(
            "depth",
            "Glass & depth",
            "Backdrop blur, rings, and layered shadows build glassmorphism without leaving \
             the utility vocabulary.",
            example_box(glass_panel(), false),
            code(&GLASS_PANEL_EXAMPLE_HTML_MAP),
            &active,
        ))

        .child(section(
            "motion",
            "Transitions & transforms",
            "Hover the cards: transition utilities animate scale, rotation, and color in \
             pure CSS. The bottom row shows the built-in keyframe animations.",
            example_box(motion_playground(), false),
            code(&MOTION_PLAYGROUND_EXAMPLE_HTML_MAP),
            &active,
        ))

        .child(section(
            "selectors",
            "Variants & pseudo-classes",
            "Target children with bracket variants — zebra striping, nth-child accents, and \
             hover states on specific descendants, all from the parent element.",
            example_box(variant_zebra_list(), false),
            code(&VARIANT_ZEBRA_LIST_EXAMPLE_HTML_MAP),
            &active,
        ))

        .child(section(
            "composition",
            "Putting it together",
            "A pricing card composed from spacing, borders, gradients, typography, focus \
             rings, and hover transforms.",
            example_box(pricing_card(), false),
            code(&PRICING_CARD_EXAMPLE_HTML_MAP),
            &active,
        ))

        .child(section(
            "generators",
            "Custom generators",
            "When a value isn't in the scale, generate the class: dwgenerate! mints new \
             utilities from parameterized generators at compile time.",
            example_box(generator_demo(), false),
            code(&GENERATOR_DEMO_EXAMPLE_HTML_MAP),
            &active,
        ))
    })
}

/// `kicker` doubles as the section's anchor id and its scroll-spy key.
fn section(
    kicker: &'static str,
    title: &str,
    description: &str,
    preview: Dom,
    source: Dom,
    active: &Mutable<&'static str>,
) -> Dom {
    html!("section", {
        .attr("id", kicker)
        .dwclass!("m-t-14 flex flex-col")
        .apply(reveal_on_scroll)
        .apply(fx::scroll_spy(kicker, active.clone()))
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-candlelight-400 text-xs")
            .text(&format!("// {}", kicker))
        }))
        .child(html!("h2", {
            .class("font-display")
            .dwclass!("text-2xl font-bold text-woodsmoke-50 m-0 m-t-1")
            .text(title)
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-0 m-t-2")
            .style("max-width", "44rem")
            .text(description)
        }))
        .child(preview)
        .child(source)
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn responsive_navbar() -> Dom {
    html!("nav", {
        .dwclass!("w-full rounded-lg border border-woodsmoke-700 bg-woodsmoke-900 p-3")
        .dwclass!("flex @sm:flex-row @<sm:flex-col align-items-center gap-3")
        .child(html!("span", {
            .dwclass!("font-mono font-extrabold text-candlelight-300")
            .text("acme")
        }))
        .child(html!("div", {
            .dwclass!("flex @sm:flex-row @<sm:flex-col align-items-center gap-3 @sm:m-l-auto")
            .children(["products", "pricing", "about"].map(|item| {
                html!("a", {
                    .dwclass!("text-woodsmoke-300 hover:text-woodsmoke-50 text-sm cursor-pointer transition-colors")
                    .text(item)
                })
            }))
            .child(html!("button", {
                .dwclass!("bg-candlelight-400 hover:bg-candlelight-300 text-woodsmoke-950")
                .dwclass!("rounded-full p-l-4 p-r-4 p-t-1 p-b-1 text-sm font-bold")
                .dwclass!("border-none cursor-pointer transition-colors")
                .text("Sign up")
            }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn gradient_showcase() -> Dom {
    html!("div", {
        .dwclass!("grid @sm:grid-cols-3 @<sm:grid-cols-1 gap-4 w-full")
        .child(html!("div", {
            .dwclass!("h-32 rounded-lg linear-gradient-45 gradient-from-charm-500 gradient-to-purple-900")
            .dwclass!("flex align-items-center justify-center font-mono text-sm text-woodsmoke-50")
            .text("charm → purple")
        }))
        .child(html!("div", {
            .dwclass!("h-32 rounded-lg linear-gradient-180 gradient-from-picton-blue-400 gradient-to-bunker-950")
            .dwclass!("flex align-items-center justify-center font-mono text-sm text-woodsmoke-50")
            .text("sky → void")
        }))
        .child(html!("div", {
            .dwclass!("h-32 rounded-lg linear-gradient-135 gradient-from-candlelight-300 gradient-to-red-800")
            .dwclass!("flex align-items-center justify-center font-mono text-sm text-woodsmoke-950")
            .text("ember → rust")
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn typography_specimen() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-3 w-full")
        .child(html!("div", {
            .dwclass!("text-4xl font-extrabold text-woodsmoke-50 leading-tight")
            .text("Display — text-4xl extrabold")
        }))
        .child(html!("div", {
            .dwclass!("text-2xl font-bold text-woodsmoke-100")
            .text("Heading — text-2xl bold")
        }))
        .child(html!("div", {
            .dwclass!("text-base text-woodsmoke-300 leading-relaxed")
            .text("Body — text-base with relaxed leading. The quick brown fox jumps over the lazy dog.")
        }))
        .child(html!("div", {
            .dwclass!("text-sm font-mono text-candlelight-300")
            .text("code — text-sm font-mono")
        }))
        .child(html!("div", {
            .dwclass!("text-xs font-medium text-woodsmoke-500")
            .text("CAPTION — text-xs medium")
        }))
    })
}

dwgenerate!("bg-glass", "bg-color-[#1212158c]");

#[example_html(themes = ["base16-ocean.dark"])]
fn glass_panel() -> Dom {
    html!("div", {
        .dwclass!("w-full rounded-lg overflow-hidden linear-gradient-135 gradient-from-purple-900 gradient-to-picton-blue-900 p-10")
        .child(html!("div", {
            .dwclass!("bg-glass backdrop-blur-md rounded-lg p-6 shadow-2xl")
            .dwclass!("ring-1 ring-woodsmoke-600")
            .dwclass!("flex flex-col gap-2")
            .child(html!("div", {
                .dwclass!("font-bold text-woodsmoke-50")
                .text("Frosted panel")
            }))
            .child(html!("p", {
                .dwclass!("text-sm text-woodsmoke-200 m-0 leading-relaxed")
                .text("bg-glass is a generated translucent background; backdrop-blur-md and a ring complete the effect.")
            }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn motion_playground() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-6 w-full")
        .child(html!("div", {
            .dwclass!("grid @sm:grid-cols-3 @<sm:grid-cols-1 gap-4")
            .child(html!("div", {
                .dwclass!("h-24 rounded-lg bg-woodsmoke-800 border border-woodsmoke-700")
                .dwclass!("flex align-items-center justify-center font-mono text-sm text-woodsmoke-300")
                .dwclass!("transition-transform hover:scale-105 cursor-pointer")
                .text("hover:scale-105")
            }))
            .child(html!("div", {
                .dwclass!("h-24 rounded-lg bg-woodsmoke-800 border border-woodsmoke-700")
                .dwclass!("flex align-items-center justify-center font-mono text-sm text-woodsmoke-300")
                .dwclass!("transition-transform hover:rotate-3 cursor-pointer")
                .text("hover:rotate-3")
            }))
            .child(html!("div", {
                .dwclass!("h-24 rounded-lg bg-woodsmoke-800 border border-woodsmoke-700")
                .dwclass!("flex align-items-center justify-center font-mono text-sm text-woodsmoke-300")
                .dwclass!("transition-all hover:bg-candlelight-400 hover:text-woodsmoke-950 cursor-pointer")
                .text("hover:bg-…")
            }))
        }))
        .child(html!("div", {
            .dwclass!("flex flex-row gap-8 justify-center align-items-center")
            .child(html!("div", { .dwclass!("w-6 h-6 bg-candlelight-400 rounded-md animate-spin") }))
            .child(html!("div", { .dwclass!("w-6 h-6 bg-charm-400 rounded-full animate-ping") }))
            .child(html!("div", { .dwclass!("w-6 h-6 bg-picton-blue-400 rounded-md animate-pulse") }))
            .child(html!("div", { .dwclass!("w-6 h-6 bg-apple-400 rounded-full animate-bounce") }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn variant_zebra_list() -> Dom {
    html!("ul", {
        .dwclass!("w-full rounded-lg overflow-hidden border border-woodsmoke-700 p-0 m-0")
        .dwclass!("[& > *]:p-3 [& > *]:text-sm [& > *]:text-woodsmoke-200")
        .dwclass!("[& > *:nth-child(odd)]:bg-woodsmoke-900 [& > *:nth-child(even)]:bg-woodsmoke-800")
        .dwclass!("[& > *]:nth-child(3):text-candlelight-300")
        .style("list-style", "none")
        .children([
            html!("li", { .text("zebra striping from the parent") }),
            html!("li", { .text("no classes on the children") }),
            html!("li", { .text("nth-child(3) gets the accent") }),
            html!("li", { .text("all via bracket variants") }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn pricing_card() -> Dom {
    html!("div", {
        .dwclass!("w-80 rounded-xl overflow-hidden border border-woodsmoke-700 bg-woodsmoke-900 shadow-2xl")
        .dwclass!("transition-transform hover:scale-105")
        .child(html!("div", {
            .dwclass!("linear-gradient-135 gradient-from-candlelight-400 gradient-to-candlelight-600 p-4")
            .child(html!("div", {
                .dwclass!("font-mono text-xs font-bold text-woodsmoke-950")
                .text("PRO")
            }))
            .child(html!("div", {
                .dwclass!("text-3xl font-extrabold text-woodsmoke-950")
                .text("$19/mo")
            }))
        }))
        .child(html!("ul", {
            .dwclass!("flex flex-col gap-2 p-4 m-0 text-sm text-woodsmoke-300")
            .style("list-style", "none")
            .children([
                "✓ unlimited projects",
                "✓ compile-time styling",
                "✓ no css build pipeline",
            ].map(|item| html!("li", { .text(item) })))
        }))
        .child(html!("div", {
            .dwclass!("p-4 p-t-0")
            .child(html!("button", {
                .dwclass!("w-full h-10 rounded-full border-none cursor-pointer font-bold")
                .dwclass!("bg-candlelight-400 hover:bg-candlelight-300 active:scale-95 text-woodsmoke-950")
                .dwclass!("transition-all focus-visible:ring-2 ring-candlelight-200")
                .text("Choose Pro")
            }))
        }))
    })
}

dwgenerate!("bg-blueprint", "background-scratched-[#1c2e4f,#191919]");
dwgenerate!("w-golden", "width-[61.8%]");

#[example_html(themes = ["base16-ocean.dark"])]
fn generator_demo() -> Dom {
    // dwgenerate!("bg-blueprint", "background-scratched-[#1c2e4f,#191919]");
    // dwgenerate!("w-golden", "width-[61.8%]");
    html!("div", {
        .dwclass!("flex flex-col gap-4 w-full")
        .child(html!("div", {
            .dwclass!("bg-blueprint h-24 rounded-lg flex align-items-center justify-center")
            .dwclass!("font-mono text-sm text-woodsmoke-200")
            .text("bg-blueprint — a generated scratched background")
        }))
        .child(html!("div", {
            .dwclass!("w-golden h-10 rounded-md bg-candlelight-500 flex align-items-center justify-center")
            .dwclass!("font-mono text-xs text-woodsmoke-950")
            .text("w-golden = width-[61.8%]")
        }))
    })
}
