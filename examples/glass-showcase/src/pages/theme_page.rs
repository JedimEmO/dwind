use dominator::{events, html, text, Dom};
use dwind::prelude::*;
use dwind_glass::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::JsCast;

use crate::helpers::section;

const THEME_SELECTOR_CODE: &str = r#"// Define a custom theme
let mut theme = GlassTheme::dark();
theme.accent = "rgba(20, 184, 166, 1)".into();
theme.accent_hover = "rgba(45, 212, 191, 1)".into();
theme.accent_muted = "rgba(20, 184, 166, 0.3)".into();

// Apply it globally
apply_glass_theme(Some(theme));"#;

const PREVIEW_CODE: &str = r#"// All components automatically pick up theme changes
// via CSS custom properties (--glass-*)

glass_button!({
    .content(Some(text("Themed Button")))
    .variant(ButtonVariant::Filled)
})
// The button's accent color comes from --glass-accent,
// which is set by apply_glass_theme()."#;

struct PresetTheme {
    name: &'static str,
    description: &'static str,
    accent_swatch: &'static str,
    theme: GlassTheme,
    background: &'static str,
}

fn preset_themes() -> Vec<PresetTheme> {
    vec![
        PresetTheme {
            name: "Midnight",
            description: "Deep indigo on dark purple",
            accent_swatch: "#6366f1",
            theme: GlassTheme::dark(),
            background: "linear-gradient(135deg, #080614 0%, #1a1540 50%, #12101e 100%)",
        },
        PresetTheme {
            name: "Ember",
            description: "Warm amber on charcoal",
            accent_swatch: "#f59e0b",
            theme: {
                let mut t = GlassTheme::dark();
                t.accent = "rgba(245, 158, 11, 1)".into();
                t.accent_hover = "rgba(252, 191, 73, 1)".into();
                t.accent_muted = "rgba(245, 158, 11, 0.3)".into();
                t
            },
            background: "linear-gradient(135deg, #0d0804 0%, #1f1008 50%, #120c06 100%)",
        },
        PresetTheme {
            name: "Ocean",
            description: "Cool teal on deep sea",
            accent_swatch: "#14b8a6",
            theme: {
                let mut t = GlassTheme::dark();
                t.accent = "rgba(20, 184, 166, 1)".into();
                t.accent_hover = "rgba(45, 212, 191, 1)".into();
                t.accent_muted = "rgba(20, 184, 166, 0.3)".into();
                t
            },
            background: "linear-gradient(135deg, #020e0e 0%, #0a1f1f 50%, #061414 100%)",
        },
    ]
}

fn apply_theme_to_root(theme: &GlassTheme) {
    let raw = theme.to_style_sheet_raw();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let root = document.document_element().unwrap();
    let style = root.unchecked_ref::<web_sys::HtmlElement>().style();

    for declaration in raw.split(';') {
        let declaration = declaration.trim();
        if declaration.is_empty() {
            continue;
        }
        if let Some((prop, val)) = declaration.split_once(':') {
            let _ = style.set_property(prop.trim(), val.trim());
        }
    }
}

fn set_body_background(gradient: &str) {
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        let _ = body.style().set_property("background", gradient);
        let _ = body.style().set_property("background-attachment", "fixed");
    }
}

pub fn theme_page() -> Dom {
    let active_theme = Mutable::new(0usize);

    let themes = preset_themes();

    html!("div", {
        .dwclass!("flex flex-col gap-8")

        .child(html!("div", {
            .dwclass!("mb-4")
            .child(html!("h1", {
                .dwclass!("text-3xl font-bold glass-text-primary mb-2")
                .text("Theme System")
            }))
            .child(html!("p", {
                .dwclass!("text-base glass-text-secondary")
                .text("Switch between preset themes. Every component across all pages updates in real time.")
            }))
        }))

        // Theme selector
        .child(section("Theme Selector", "section-theme-selector", Some(THEME_SELECTOR_CODE), vec![
            html!("div", {
                .dwclass!("grid gap-4")
                .style("grid-template-columns", "repeat(auto-fit, minmax(200px, 1fr))")
                .children(themes.into_iter().enumerate().map(|(i, preset)| {
                    let active_theme = active_theme.clone();
                    let theme = preset.theme;
                    let background = preset.background;

                    html!("div", {
                        .style("cursor", "pointer")
                        .style("transition-duration", "var(--glass-transition)")
                        .style("transition-property", "outline, outline-offset")
                        .style_signal("outline", active_theme.signal().map(move |a| {
                            if a == i {
                                "2px solid var(--glass-accent)"
                            } else {
                                "2px solid transparent"
                            }
                        }))
                        .style("outline-offset", "3px")
                        .style("border-radius", "var(--glass-border-radius-xl)")
                        .event(move |_: events::Click| {
                            apply_theme_to_root(&theme);
                            set_body_background(background);
                            active_theme.set(i);
                        })
                        .child(glass_card!({
                            .content(Some(html!("div", {
                                .dwclass!("flex flex-col gap-3")
                                .child(html!("div", {
                                    .dwclass!("flex items-center gap-3")
                                    .child(html!("div", {
                                        .style("width", "20px")
                                        .style("height", "20px")
                                        .style("border-radius", "var(--glass-border-radius-full)")
                                        .style("background", preset.accent_swatch)
                                        .style("flex-shrink", "0")
                                    }))
                                    .child(html!("span", {
                                        .dwclass!("font-semibold glass-text-primary")
                                        .text(preset.name)
                                    }))
                                }))
                                .child(html!("p", {
                                    .dwclass!("text-sm glass-text-secondary")
                                    .text(preset.description)
                                }))
                            })))
                        }))
                    })
                }).collect::<Vec<_>>())
            }),
        ]))

        // Preview section
        .child(section("Preview", "section-preview", Some(PREVIEW_CODE), vec![
            html!("div", {
                .dwclass!("flex flex-wrap items-center gap-3")
                .child(glass_button!({
                    .content(Some(text("Filled")))
                    .variant(ButtonVariant::Filled)
                }))
                .child(glass_button!({
                    .content(Some(text("Glass")))
                    .variant(ButtonVariant::Glass)
                }))
                .child(glass_button!({
                    .content(Some(text("Ghost")))
                    .variant(ButtonVariant::Ghost)
                }))
                .child(glass_button!({
                    .content(Some(text("Danger")))
                    .variant(ButtonVariant::Danger)
                }))
            }),

            html!("div", {
                .dwclass!("flex flex-wrap items-center gap-3")
                .child(glass_badge!({
                    .content(Some(text("Accent")))
                    .variant(BadgeVariant::Accent)
                }))
                .child(glass_badge!({
                    .content(Some(text("Success")))
                    .variant(BadgeVariant::Success)
                }))
                .child(glass_badge!({
                    .content(Some(text("Warning")))
                    .variant(BadgeVariant::Warning)
                }))
                .child(glass_badge!({
                    .content(Some(text("Error")))
                    .variant(BadgeVariant::Error)
                }))
                .child(glass_badge!({
                    .content(Some(text("Info")))
                    .variant(BadgeVariant::Info)
                }))
            }),

            glass_card!({
                .content(Some(html!("p", {
                    .dwclass!("glass-text-secondary")
                    .text("This card and all components across every page update when you switch themes. Navigate to other pages to see the effect.")
                })))
            }),

            html!("div", {
                .style("max-width", "400px")
                .child(glass_text_input!({
                    .label("Sample input".to_string())
                    .placeholder("Themed input field".to_string())
                }))
            }),

            glass_toggle!({
                .label(Some("Toggle with theme accent".to_string()))
            }),
        ]))
    })
}
