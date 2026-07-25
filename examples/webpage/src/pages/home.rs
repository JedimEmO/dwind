use crate::fx::{self, kinetic_headline, magnetic, spotlight, spotlight_tilt};
use crate::keyframes::*;
use crate::pages::signal_lab::signal_lab;
use crate::reveal::reveal_on_scroll;
use dominator::routing::go_to_url;
use dominator::{events, text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};

pub fn home_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(hero())
        .child(utility_ticker())
        .child(signal_lab())
        .child(bento_features())
        .child(components_preview())
        .child(final_cta())
    })
}

// ---------------------------------------------------------------------------
// Hero
// ---------------------------------------------------------------------------

fn hero() -> Dom {
    html!("section", {
        .dwclass!("w-full overflow-hidden")
        .dwclass!("relative")
        .child(fx::blueprint_grid(
            "radial-gradient(ellipse 90% 70% at 50% 0%, black 30%, transparent 75%)",
        ))
        .child(html!("div", {
            .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 p-t-20 p-b-10")
            // @md is 1280px in dwind — the hero goes side-by-side from there up.
            .dwclass!("flex @md:flex-row @<md:flex-col gap-12 align-items-center")
            .dwclass!("relative")
            // Left: headline
            .child(html!("div", {
                .dwclass!("flex flex-col gap-6 grow")
                .child(html!("div", {
                    .class("font-code")
                    .dwclass!("flex flex-row align-items-center gap-3 text-sm text-candlelight-400")
                    .style("animation", &format!("{FADE_UP_KEYFRAMES} 400ms ease-out both"))
                    .child(html!("span", {
                        .dwclass!("w-2 h-2 rounded-full bg-candlelight-400 flex-none")
                        .dwclass!("animate-pulse-ring")
                    }))
                    .child(html!("span", { .text("rust → wasm → css · no node toolchain") }))
                }))
                .child(html!("h1", {
                    .class("font-display")
                    .dwclass!("@sm:text-6xl @<sm:text-4xl font-extrabold text-woodsmoke-50 m-0 leading-tight")
                    .dwclass!("tracking-tighter")
                    .children(kinetic_headline("Styling *forged* at compile time."))
                }))
                .child(html!("p", {
                    .dwclass!("text-woodsmoke-300 text-l m-0 leading-relaxed")
                    .style("max-width", "34rem")
                    .style("animation", &format!("{FADE_UP_KEYFRAMES} 900ms 300ms ease-out both"))
                    .text("dwind brings utility-first styling to the DOMINATOR web framework — \
                           every class checked by rustc, every state change driven by signals, \
                           and a complete accessible component library on top.")
                }))
                .child(html!("div", {
                    .dwclass!("flex @sm:flex-row @<sm:flex-col gap-4 align-items-center m-t-2")
                    .style("animation", &format!("{FADE_UP_KEYFRAMES} 900ms 420ms ease-out both"))
                    .child(html!("div", {
                        .dwclass!("w-44")
                        .apply(magnetic(7.0))
                        .child(button!({
                            .content(Some(text("Get started")))
                            .on_click(|_: events::Click| {
                                go_to_url("#/docs/getting-started");
                            })
                        }))
                    }))
                    .child(html!("div", {
                        .dwclass!("w-44")
                        .apply(magnetic(7.0))
                        .child(button!({
                            .button_type(ButtonType::Border)
                            .content(Some(text("Components")))
                            .on_click(|_: events::Click| {
                                go_to_url("#/components");
                            })
                        }))
                    }))
                    .child(html!("code", {
                        .class("font-code")
                        .dwclass!("text-woodsmoke-400 text-sm border border-woodsmoke-800 rounded-md p-l-3 p-r-3 p-t-1 p-b-1 select-all flex-none")
                        .style("background", "rgba(18, 18, 21, 0.6)")
                        .dwclass!("whitespace-nowrap")
                        .text("> cargo add dwind dwui")
                    }))
                }))
            }))
            // Right: code card
            .child(code_card())
        }))
    })
}

fn code_card() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col flex-none w-full")
        .style("max-width", "30rem")
        .style("animation", &format!("{FADE_UP_KEYFRAMES} 900ms 200ms ease-out both"))
        .child(html!("div", {
            .apply(crate::fx::glass)
            .dwclass!("rounded-lg border border-woodsmoke-800 overflow-hidden shadow-2xl")
            .apply(spotlight_tilt(4.0))
            // window chrome
            .child(html!("div", {
                .dwclass!("flex flex-row align-items-center gap-2 p-l-4 p-r-4 h-10 border-b border-woodsmoke-800")
                .children([
                    window_dot("#BD4C4C"),
                    window_dot("#BD9F4C"),
                    window_dot("#61BD4C"),
                ])
                .child(html!("span", {
                    .class("font-code")
                    .dwclass!("text-woodsmoke-500 text-xs m-l-2")
                    .text("button.rs")
                }))
            }))
            // code body
            .child(html!("pre", {
                .class("font-code")
                .dwclass!("text-sm p-6 m-0 leading-relaxed overflow-x-auto")
                .children([
                    code_line(vec![("html!", "mac"), ("(", "pun"), ("\"button\"", "str"), (", {", "pun")]),
                    code_line(vec![("    .", "pun"), ("dwclass!", "mac"), ("(", "pun"), ("\"bg-candlelight-500 hover:bg-candlelight-400\"", "str"), (")", "pun")]),
                    code_line(vec![("    .", "pun"), ("dwclass!", "mac"), ("(", "pun"), ("\"rounded-full h-10 p-x-4 font-bold\"", "str"), (")", "pun")]),
                    code_line(vec![("    .", "pun"), ("text_signal", "fn"), ("(", "pun"), ("label", "var"), (".", "pun"), ("signal_cloned", "fn"), ("())", "pun")]),
                    code_line(vec![("    .", "pun"), ("event", "fn"), ("(|", "pun"), ("_", "var"), (": ", "pun"), ("events::Click", "ty"), ("| { ", "pun"), ("deploy", "fn"), ("() ", "pun"), ("})", "pun")]),
                    code_line(vec![("})", "pun")]),
                ])
            }))
        }))
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-xs m-t-3 flex flex-row gap-4")
            .child(html!("span", {
                .dwclass!("text-apple-400")
                .text("[ok] compiled in 0.42s")
            }))
            .child(html!("span", {
                .dwclass!("text-woodsmoke-500")
                .text("0 errors · 0 warnings · 0 css build steps")
            }))
        }))
    })
}

fn window_dot(color: &str) -> Dom {
    html!("span", {
        .dwclass!("w-3 h-3 rounded-full flex-none")
        .style("background-color", color)
    })
}

pub fn code_line(parts: Vec<(&str, &str)>) -> Dom {
    html!("div", {
        .children(parts.into_iter().map(|(content, kind)| {
            html!("span", {
                .apply(|b| match kind {
                    "mac" => dwclass!(b, "text-candlelight-300"),
                    "str" => dwclass!(b, "text-apple-300"),
                    "fn" => dwclass!(b, "text-picton-blue-300"),
                    "ty" => dwclass!(b, "text-charm-300"),
                    "var" => dwclass!(b, "text-woodsmoke-200"),
                    _ => dwclass!(b, "text-woodsmoke-500"),
                })
                .text(content)
            })
        }))
    })
}

// ---------------------------------------------------------------------------
// Utility ticker
// ---------------------------------------------------------------------------

/// A slow drift of real, compiler-checked class names. Every one of these
/// resolves to a Rust constant at build time.
fn utility_ticker() -> Dom {
    html!("section", {
        .dwclass!("border-t border-b border-woodsmoke-800 w-full p-t-6 p-b-6 m-t-8")
        .style("background", "rgba(10, 10, 12, 0.45)")
        .child(fx::marquee(&[
            "flex flex-col gap-4",
            "@sm:grid-cols-3",
            "hover:bg-candlelight-400",
            "rounded-full",
            "text-picton-blue-300",
            "linear-gradient-45",
            "backdrop-blur-md",
            "@<lg:hidden",
            "animate-pulse",
            "transition-colors",
            "focus:outline-none",
            "shadow-2xl",
            "w-p-50",
            "translate-x-4",
            "@(max-width:200px):hidden",
            "border-woodsmoke-800",
        ]))
    })
}

// ---------------------------------------------------------------------------
// Bento feature grid
// ---------------------------------------------------------------------------

fn bento_features() -> Dom {
    html!("section", {
        .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 p-t-20 w-full")
        .apply(reveal_on_scroll)
        .child(section_header("features", "Everything the compiler can prove"))
        .child(html!("div", {
            .dwclass!("grid grid-cols-6 gap-4 m-t-8")
            .children([
                bento_tile_large(),
                bento_tile_reactive(),
                bento_tile_themes(),
                bento_tile_a11y(),
                bento_tile_responsive(),
            ])
        }))
    })
}

pub fn section_header(kicker: &str, title: &str) -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-2")
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-candlelight-400 text-sm")
            .text(&format!("// {}", kicker))
        }))
        .child(html!("h2", {
            .class("font-display")
            .dwclass!("@sm:text-4xl @<sm:text-2xl font-bold text-woodsmoke-50 m-0")
            .dwclass!("tracking-tight")
            .text(title)
        }))
    })
}

fn bento_tile(span_large: bool, children: Vec<Dom>) -> Dom {
    html!("div", {
        .apply(crate::fx::glass)
        .dwclass!("rounded-lg border border-woodsmoke-800 p-6 flex flex-col gap-3")
        .dwclass!("hover:border-candlelight-700")
        .apply(spotlight_tilt(2.5))
        .apply(move |b| {
            if span_large {
                dwclass!(b, "@md:col-span-4 @<md:col-span-6")
            } else {
                dwclass!(b, "@md:col-span-2 @<md:col-span-6")
            }
        })
        .children(children)
    })
}

fn tile_title(title: &str) -> Dom {
    html!("h3", {
        .class("font-display")
        .dwclass!("text-l font-bold text-woodsmoke-50 m-0")
        .text(title)
    })
}

fn tile_text(content: &str) -> Dom {
    html!("p", {
        .dwclass!("text-sm text-woodsmoke-400 m-0 leading-relaxed")
        .text(content)
    })
}

fn bento_tile_large() -> Dom {
    bento_tile(
        true,
        vec![
            tile_title("Type-checked utility classes"),
            tile_text(
                "Every dwclass! is resolved against generated Rust constants. \
                   A typo in a class name is a compile error, not a silently broken layout. \
                   Refactor styles with the same confidence you refactor code.",
            ),
            html!("pre", {
                .class("font-code")
                .dwclass!("text-sm m-0 m-t-2 p-4 rounded-md border border-woodsmoke-800 overflow-x-auto leading-relaxed")
                .style("background", "rgba(2, 2, 3, 0.7)")
                .children([
                    code_line(vec![(".", "pun"), ("dwclass!", "mac"), ("(", "pun"), ("\"flex gap-4 @sm:flex-row\"", "str"), (")  ", "pun"), ("// ✓ compiles", "pun")]),
                    code_line(vec![(".", "pun"), ("dwclass!", "mac"), ("(", "pun"), ("\"flxe gap-4\"", "str"), (")             ", "pun"), ("// ✗ E0425", "pun")]),
                ])
            }),
        ],
    )
}

fn bento_tile_reactive() -> Dom {
    let enabled = Mutable::new(true);

    bento_tile(
        false,
        vec![
            tile_title("Reactive signals"),
            tile_text("State flows through futures-signals; the DOM patches itself."),
            html!("div", {
                .dwclass!("flex flex-col gap-3 m-t-2")
                .child(switch!({
                    .checked_signal(enabled.signal())
                    .label("Live demo".to_string())
                    .on_change({
                        let enabled = enabled.clone();
                        move |v| enabled.set(v)
                    })
                }))
                .child_signal(enabled.signal().map(|on| {
                    Some(badge!({
                        .variant(if on { BadgeVariant::Primary } else { BadgeVariant::Void })
                        .content(Some(text(if on { "signal: true" } else { "signal: false" })))
                    }))
                }))
            }),
        ],
    )
}

fn bento_tile_themes() -> Dom {
    bento_tile(false, vec![
        tile_title("Theme-native"),
        tile_text("Components read CSS variables — swap palettes at runtime, light and dark included."),
        html!("div", {
            .dwclass!("flex flex-row gap-2 m-t-2")
            .children(["#D5B65F", "#5FB0D5", "#75D55F", "#D55FA8", "#BD4C4C"].into_iter().enumerate().map(|(i, c)| {
                html!("span", {
                    .dwclass!("w-6 h-6 rounded-full border border-woodsmoke-700 transition-all")
                    .dwclass!("hover:scale-125")
                    .style("background-color", c)
                    .style("animation", &format!("{FADE_UP_KEYFRAMES} 600ms {}ms ease-out both", i * 60))
                })
            }))
        }),
    ])
}

fn bento_tile_a11y() -> Dom {
    bento_tile(false, vec![
        tile_title("Accessible by default"),
        tile_text("Focus rings, ARIA wiring, and keyboard support are built into every DWUI component — verified by a headless browser test suite."),
        html!("div", {
            .class("font-code")
            .dwclass!("text-xs text-apple-400 m-t-2")
            .text("[pass] a11y + interaction suite — headless in ci")
        }),
    ])
}

fn bento_tile_responsive() -> Dom {
    bento_tile(false, vec![
        tile_title("Responsive conditionals"),
        tile_text("Breakpoints are class prefixes: @sm:, @md:, @<lg: — and arbitrary media queries when you need them."),
        html!("pre", {
            .class("font-code")
            .dwclass!("text-xs m-0 m-t-2 text-woodsmoke-300")
            .text("@sm:flex-row\n@<sm:flex-col\n@(max-width:200px):hidden")
        }),
    ])
}

// ---------------------------------------------------------------------------
// Component preview
// ---------------------------------------------------------------------------

fn components_preview() -> Dom {
    let progress_value = Mutable::new(64.0f32);

    html!("section", {
        .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 p-t-20 w-full")
        .apply(reveal_on_scroll)
        .child(section_header("components", "A component library that ships with the stack"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 m-t-4 m-b-8")
            .style("max-width", "40rem")
            .text("Buttons, forms, tables, dialogs, date pickers, virtualized lists — 24 themeable building blocks, \
                   each with ARIA semantics and keyboard support baked in.")
        }))
        .child(html!("div", {
            .dwclass!("grid @md:grid-cols-3 @<md:grid-cols-1 gap-4")
            .children([
                preview_card("inputs", vec![
                    text_input!({
                        .label("Project name".to_string())
                    }),
                    slider!({
                        .value(progress_value.clone())
                        .label("Threshold".to_string())
                    }),
                ]),
                preview_card("feedback", vec![
                    alert!({
                        .variant(AlertVariant::Success)
                        .title("Deployed".to_string())
                        .content(Some(text("Build #214 is live.")))
                    }),
                    html!("div", {
                        .dwclass!("flex flex-row gap-4 align-items-center")
                        .child(spinner!({
                            .size(SpinnerSize::Small)
                        }))
                        .child(progress!({
                            .value_signal(progress_value.signal().map(|v| v as f64))
                            .label("Demo progress".to_string())
                        }))
                    }),
                ]),
                preview_card("identity", vec![
                    html!("div", {
                        .dwclass!("flex flex-row gap-3 align-items-center")
                        .child(avatar!({
                            .name("Ada Lovelace".to_string())
                        }))
                        .child(avatar!({
                            .name("Grace Hopper".to_string())
                        }))
                        .child(badge!({
                            .variant(BadgeVariant::Outline)
                            .content(Some(text("v0.9.0")))
                        }))
                    }),
                    breadcrumbs!({
                        .items(vec![
                            ("home".to_string(), "#/".to_string()),
                            ("components".to_string(), "#/components".to_string()),
                            ("avatar".to_string(), "#/components".to_string()),
                        ])
                    }),
                ]),
            ])
        }))
        .child(html!("div", {
            .dwclass!("flex justify-center m-t-8")
            .child(html!("div", {
                .dwclass!("w-64")
                .apply(magnetic(6.0))
                .child(button!({
                    .button_type(ButtonType::Border)
                    .content(Some(text("Browse all components →")))
                    .on_click(|_: events::Click| {
                        go_to_url("#/components");
                    })
                }))
            }))
        }))
    })
}

fn preview_card(label: &str, children: Vec<Dom>) -> Dom {
    html!("div", {
        .apply(crate::fx::glass)
        .dwclass!("rounded-lg border border-woodsmoke-800 p-6 flex flex-col gap-5 hover:border-candlelight-700")
        .apply(spotlight)
        .child(html!("div", {
            .class("font-code")
            .dwclass!("text-xs text-woodsmoke-500")
            .text(&format!("// {}", label))
        }))
        .children(children)
    })
}

// ---------------------------------------------------------------------------
// Final CTA
// ---------------------------------------------------------------------------

fn final_cta() -> Dom {
    html!("section", {
        .dwclass!("w-full p-t-20")
        .dwclass!("relative")
        .apply(reveal_on_scroll)
        .child(html!("div", {
            .dwclass!("m-x-auto max-w-3xl p-l-4 p-r-4 flex flex-col gap-6 align-items-center")
            .child(html!("h2", {
                .class("font-display")
                .dwclass!("@sm:text-5xl @<sm:text-3xl font-extrabold text-woodsmoke-50 m-0 text-center")
                .dwclass!("tracking-tighter")
                .child(text("Build interfaces in "))
                .child(html!("span", {
                    .apply(crate::fx::sheen)
                    .text("Rust")
                }))
                .child(text("."))
            }))
            .child(html!("p", {
                .dwclass!("text-woodsmoke-400 text-center m-0")
                .text("One language from backend to pixel. Start with the docs, or open the component gallery and poke at the live demos.")
            }))
            .child(html!("div", {
                .dwclass!("flex @sm:flex-row @<sm:flex-col gap-4 m-t-2")
                .child(html!("div", {
                    .dwclass!("w-44")
                    .apply(magnetic(7.0))
                    .child(button!({
                        .content(Some(text("Read the docs")))
                        .on_click(|_: events::Click| {
                            go_to_url("#/docs/getting-started");
                        })
                    }))
                }))
                .child(html!("div", {
                    .dwclass!("w-44")
                    .apply(magnetic(7.0))
                    .child(button!({
                        .button_type(ButtonType::Border)
                        .content(Some(text("Open the gallery")))
                        .on_click(|_: events::Click| {
                            go_to_url("#/components");
                        })
                    }))
                }))
            }))
            .child(html!("div", {
                .class("font-code")
                .dwclass!("text-xs text-woodsmoke-600 m-t-4 flex flex-row gap-2 align-items-center")
                .child(html!("kbd", {
                    .dwclass!("border border-woodsmoke-800 rounded-md p-l-2 p-r-2 p-t-1 p-b-1")
                    .text("⌘K")
                }))
                .child(html!("span", { .text("opens the command palette from anywhere") }))
            }))
        }))
    })
}
