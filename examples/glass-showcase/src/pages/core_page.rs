use dominator::{events, html, text, Dom};
use dwind::prelude::*;
use dwind_glass::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use dwind_glass::input::{GlassSelectOption, ValidationResult};

use crate::helpers::{section, subsection};

const BUTTONS_CODE: &str = r#"// Variants
glass_button!({
    .content(Some(text("Filled")))
    .variant(ButtonVariant::Filled)
    .on_click(Box::new(|_: events::Click| { /* ... */ }))
})

// Sizes
glass_button!({
    .content(Some(text("Small")))
    .size(ButtonSize::Small)
})

// States
glass_button!({
    .content(Some(text("Loading")))
    .loading(true)
})"#;

const CARDS_CODE: &str = r#"glass_card!({
    .variant(CardVariant::Elevated)
    .content(Some(html!("div", {
        .child(html!("span", { .text("Title") }))
        .child(html!("p", { .text("Card content here.") }))
    })))
})"#;

const BADGES_CODE: &str = r#"glass_badge!({
    .content(Some(text("Success")))
    .variant(BadgeVariant::Success)
})"#;

const TEXT_INPUT_CODE: &str = r#"let value = Mutable::new("".to_string());

glass_text_input!({
    .value(value.clone())
    .label("Email".to_string())
    .placeholder("you@example.com".to_string())
    .is_valid_signal(value.signal_cloned().map(|v| {
        if v.contains('@') { ValidationResult::Valid }
        else { ValidationResult::Invalid {
            message: "Must be a valid email".to_string()
        }}
    }))
})"#;

const TOGGLE_CODE: &str = r#"let state = Mutable::new(false);

glass_toggle!({
    .checked(state)
    .label(Some("Enable notifications".to_string()))
})"#;

const MODAL_CODE: &str = r#"let open = Mutable::new(false);

glass_modal!({
    .open_signal(open.signal())
    .on_close({
        let open = open.clone();
        Box::new(move || { open.set(false); })
    })
    .title(Some("Dialog Title".to_string()))
    .size(ModalSize::Medium)
    .content(Some(html!("p", { .text("Modal content") })))
})"#;

const CHECKBOX_CODE: &str = r#"let checked = Mutable::new(false);

glass_checkbox!({
    .checked(checked)
    .label(Some("Accept terms".to_string()))
    .is_valid_signal(checked.signal().map(|c| {
        if c { ValidationResult::Valid }
        else { ValidationResult::Invalid {
            message: "Required".to_string()
        }}
    }))
})"#;

const RADIO_CODE: &str = r#"let value = Mutable::new(None::<String>);

glass_radio_group!({
    .value(value)
    .options(vec![
        GlassSelectOption { label: "Light".into(), value: "light".into() },
        GlassSelectOption { label: "Dark".into(), value: "dark".into() },
    ])
})"#;

const SELECT_CODE: &str = r#"let value = Mutable::new(None::<String>);

glass_select!({
    .value(value)
    .label("Framework".to_string())
    .placeholder("Choose one...".to_string())
    .options(vec![
        GlassSelectOption { label: "Dominator".into(), value: "dominator".into() },
        GlassSelectOption { label: "Leptos".into(), value: "leptos".into() },
    ])
})"#;

const COMBOBOX_CODE: &str = r#"let value = Mutable::new(None::<String>);

glass_combobox!({
    .value(value)
    .label("Country".to_string())
    .placeholder("Search countries...".to_string())
    .options(vec![
        GlassSelectOption { label: "United States".into(), value: "us".into() },
        GlassSelectOption { label: "Canada".into(), value: "ca".into() },
    ])
})"#;

pub fn core_page() -> Dom {
    let modal_open = Mutable::new(false);
    let toggle_state = Mutable::new(false);
    let input_value = Mutable::new("".to_string());
    let checkbox_a = Mutable::new(false);
    let checkbox_b = Mutable::new(true);
    let radio_value = Mutable::new(None::<String>);
    let select_value = Mutable::new(None::<String>);
    let combobox_value = Mutable::new(None::<String>);

    // Validation demo state
    let email_value = Mutable::new("".to_string());
    let required_checkbox = Mutable::new(false);
    let validated_select = Mutable::new(None::<String>);
    let validated_combobox = Mutable::new(None::<String>);
    let validated_radio = Mutable::new(None::<String>);

    html!("div", {
        .dwclass!("flex flex-col gap-8")

        // Page title
        .child(html!("div", {
            .dwclass!("mb-4")
            .child(html!("h1", {
                .dwclass!("text-3xl font-bold glass-text-primary mb-2")
                .text("Core Components")
            }))
            .child(html!("p", {
                .dwclass!("text-base glass-text-secondary")
                .text("The foundational building blocks of the glass design system.")
            }))
        }))

        // Buttons
        .child(section("Buttons", "section-buttons", Some(BUTTONS_CODE), vec![
            subsection("Variants", vec![
                html!("div", {
                    .dwclass!("flex flex-wrap items-center gap-3")
                    .child(glass_button!({
                        .content(Some(text("Glass")))
                        .variant(ButtonVariant::Glass)
                    }))
                    .child(glass_button!({
                        .content(Some(text("Filled")))
                        .variant(ButtonVariant::Filled)
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
            ]),
            subsection("Sizes", vec![
                html!("div", {
                    .dwclass!("flex flex-wrap items-center gap-3")
                    .child(glass_button!({
                        .content(Some(text("Small")))
                        .size(ButtonSize::Small)
                    }))
                    .child(glass_button!({
                        .content(Some(text("Medium")))
                        .size(ButtonSize::Medium)
                    }))
                    .child(glass_button!({
                        .content(Some(text("Large")))
                        .size(ButtonSize::Large)
                    }))
                }),
            ]),
            subsection("States", vec![
                html!("div", {
                    .dwclass!("flex flex-wrap items-center gap-3")
                    .child(glass_button!({
                        .content(Some(text("Disabled")))
                        .disabled(true)
                    }))
                    .child(glass_button!({
                        .content(Some(text("Loading")))
                        .loading(true)
                    }))
                }),
            ]),
        ]))

        // Cards
        .child(section("Cards", "section-cards", Some(CARDS_CODE), vec![
            html!("div", {
                .dwclass!("grid gap-4")
                .style("grid-template-columns", "repeat(auto-fit, minmax(280px, 1fr))")
                .child(glass_card!({
                    .variant(CardVariant::Default)
                    .content(Some(html!("div", {
                        .dwclass!("flex flex-col gap-2")
                        .child(html!("span", {
                            .dwclass!("font-semibold glass-text-primary")
                            .text("Default Card")
                        }))
                        .child(html!("p", {
                            .dwclass!("glass-text-secondary")
                            .text("Standard glass card with subtle frosted backdrop.")
                        }))
                    })))
                }))
                .child(glass_card!({
                    .variant(CardVariant::Elevated)
                    .content(Some(html!("div", {
                        .dwclass!("flex flex-col gap-2")
                        .child(html!("span", {
                            .dwclass!("font-semibold glass-text-primary")
                            .text("Elevated Card")
                        }))
                        .child(html!("p", {
                            .dwclass!("glass-text-secondary")
                            .text("More prominent glass with heavier blur and stronger shadow.")
                        }))
                    })))
                }))
                .child(glass_card!({
                    .variant(CardVariant::Inset)
                    .content(Some(html!("div", {
                        .dwclass!("flex flex-col gap-2")
                        .child(html!("span", {
                            .dwclass!("font-semibold glass-text-primary")
                            .text("Inset Card")
                        }))
                        .child(html!("p", {
                            .dwclass!("glass-text-secondary")
                            .text("Recessed appearance for secondary or nested content areas.")
                        }))
                    })))
                }))
            }),
        ]))

        // Badges
        .child(section("Badges", "section-badges", Some(BADGES_CODE), vec![
            html!("div", {
                .dwclass!("flex flex-wrap items-center gap-3")
                .child(glass_badge!({
                    .content(Some(text("Default")))
                    .variant(BadgeVariant::Default)
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
                .child(glass_badge!({
                    .content(Some(text("Accent")))
                    .variant(BadgeVariant::Accent)
                }))
            }),
        ]))

        // Text Input
        .child(section("Text Input", "section-text-input", Some(TEXT_INPUT_CODE), vec![
            subsection("Basic", vec![
                html!("div", {
                    .style("max-width", "400px")
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_text_input!({
                        .value(input_value.clone())
                        .label("Username".to_string())
                        .placeholder("Enter your username".to_string())
                    }))
                    .child(glass_text_input!({
                        .label("Password".to_string())
                        .placeholder("Enter password".to_string())
                        .input_type(TextInputType::Password)
                    }))
                    .child(glass_text_input!({
                        .label("Disabled".to_string())
                        .disabled(true)
                    }))
                }),
            ]),
            subsection("Validation", vec![
                html!("div", {
                    .style("max-width", "400px")
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_text_input!({
                        .value(email_value.clone())
                        .label("Email (required)".to_string())
                        .placeholder("you@example.com".to_string())
                        .is_valid_signal(email_value.signal_cloned().map(|v| {
                            if v.is_empty() {
                                ValidationResult::Invalid { message: "Email is required".to_string() }
                            } else if !v.contains('@') {
                                ValidationResult::Invalid { message: "Must be a valid email address".to_string() }
                            } else {
                                ValidationResult::Valid
                            }
                        }))
                    }))
                }),
            ]),
        ]))

        // Toggle
        .child(section("Toggle", "section-toggle", Some(TOGGLE_CODE), vec![
            html!("div", {
                .dwclass!("flex flex-col gap-4")
                .child(glass_toggle!({
                    .checked(toggle_state.clone())
                    .label(Some("Enable notifications".to_string()))
                }))
                .child(glass_toggle!({
                    .label(Some("Disabled toggle".to_string()))
                    .disabled(true)
                }))
            }),
        ]))

        // Modal
        .child(section("Modal", "section-modal", Some(MODAL_CODE), vec![
            html!("div", {
                .dwclass!("flex gap-3")
                .child(glass_button!({
                    .content(Some(text("Open Modal")))
                    .variant(ButtonVariant::Glass)
                    .on_click({
                        let modal_open = modal_open.clone();
                        Box::new(move |_: events::Click| {
                            modal_open.set(true);
                        })
                    })
                }))
            }),
            glass_modal!({
                .open_signal(modal_open.signal())
                .on_close({
                    let modal_open = modal_open.clone();
                    Box::new(move || {
                        modal_open.set(false);
                    })
                })
                .title(Some("Glass Modal".to_string()))
                .size(ModalSize::Medium)
                .content(Some(html!("div", {
                    .dwclass!("flex flex-col gap-4")
                    .child(html!("p", {
                        .dwclass!("glass-text-secondary")
                        .text("This is a glassmorphic modal with frosted backdrop blur. The modal surface uses elevated glass styling with a heavier blur effect.")
                    }))
                    .child(html!("div", {
                        .dwclass!("flex gap-3 justify-end")
                        .child(glass_button!({
                            .content(Some(text("Cancel")))
                            .variant(ButtonVariant::Ghost)
                            .on_click({
                                let modal_open = modal_open.clone();
                                Box::new(move |_: events::Click| {
                                    modal_open.set(false);
                                })
                            })
                        }))
                        .child(glass_button!({
                            .content(Some(text("Confirm")))
                            .variant(ButtonVariant::Filled)
                            .on_click({
                                let modal_open = modal_open.clone();
                                Box::new(move |_: events::Click| {
                                    modal_open.set(false);
                                })
                            })
                        }))
                    }))
                })))
            }),
        ]))

        // Checkbox
        .child(section("Checkbox", "section-checkbox", Some(CHECKBOX_CODE), vec![
            subsection("Basic", vec![
                html!("div", {
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_checkbox!({
                        .checked(checkbox_a.clone())
                        .label(Some("Accept terms and conditions".to_string()))
                    }))
                    .child(glass_checkbox!({
                        .checked(checkbox_b.clone())
                        .label(Some("Subscribe to newsletter".to_string()))
                    }))
                    .child(glass_checkbox!({
                        .label(Some("Disabled checkbox".to_string()))
                        .disabled(true)
                    }))
                }),
            ]),
            subsection("Validation", vec![
                html!("div", {
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_checkbox!({
                        .checked(required_checkbox.clone())
                        .label(Some("I accept the terms of service".to_string()))
                        .is_valid_signal(required_checkbox.signal().map(|checked| {
                            if checked {
                                ValidationResult::Valid
                            } else {
                                ValidationResult::Invalid { message: "You must accept the terms".to_string() }
                            }
                        }))
                    }))
                }),
            ]),
        ]))

        // Radio Group
        .child(section("Radio Group", "section-radio-group", Some(RADIO_CODE), vec![
            subsection("Basic", vec![
                html!("div", {
                    .style("max-width", "400px")
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_radio_group!({
                        .value(radio_value.clone())
                        .options(vec![
                            GlassSelectOption { label: "Light".to_string(), value: "light".to_string() },
                            GlassSelectOption { label: "Dark".to_string(), value: "dark".to_string() },
                            GlassSelectOption { label: "System".to_string(), value: "system".to_string() },
                        ])
                    }))
                }),
            ]),
            subsection("Validation", vec![
                html!("div", {
                    .style("max-width", "400px")
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_radio_group!({
                        .value(validated_radio.clone())
                        .options(vec![
                            GlassSelectOption { label: "Small".to_string(), value: "sm".to_string() },
                            GlassSelectOption { label: "Medium".to_string(), value: "md".to_string() },
                            GlassSelectOption { label: "Large".to_string(), value: "lg".to_string() },
                        ])
                        .is_valid_signal(validated_radio.signal_cloned().map(|v| {
                            if v.is_some() {
                                ValidationResult::Valid
                            } else {
                                ValidationResult::Invalid { message: "Please select a size".to_string() }
                            }
                        }))
                    }))
                }),
            ]),
        ]))

        // Select
        .child(section("Select", "section-select", Some(SELECT_CODE), vec![
            subsection("Basic", vec![
                html!("div", {
                    .style("max-width", "400px")
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_select!({
                        .value(select_value.clone())
                        .label("Favorite framework".to_string())
                        .placeholder("Choose one...".to_string())
                        .options(vec![
                            GlassSelectOption { label: "Dominator".to_string(), value: "dominator".to_string() },
                            GlassSelectOption { label: "Leptos".to_string(), value: "leptos".to_string() },
                            GlassSelectOption { label: "Yew".to_string(), value: "yew".to_string() },
                            GlassSelectOption { label: "Dioxus".to_string(), value: "dioxus".to_string() },
                            GlassSelectOption { label: "Sycamore".to_string(), value: "sycamore".to_string() },
                        ])
                    }))
                }),
            ]),
            subsection("Validation", vec![
                html!("div", {
                    .style("max-width", "400px")
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_select!({
                        .value(validated_select.clone())
                        .label("Required selection".to_string())
                        .placeholder("Pick a framework...".to_string())
                        .options(vec![
                            GlassSelectOption { label: "Dominator".to_string(), value: "dominator".to_string() },
                            GlassSelectOption { label: "Leptos".to_string(), value: "leptos".to_string() },
                            GlassSelectOption { label: "Yew".to_string(), value: "yew".to_string() },
                        ])
                        .is_valid_signal(validated_select.signal_cloned().map(|v| {
                            if v.is_some() {
                                ValidationResult::Valid
                            } else {
                                ValidationResult::Invalid { message: "Please select a framework".to_string() }
                            }
                        }))
                    }))
                }),
            ]),
        ]))

        // Combobox
        .child(section("Combobox", "section-combobox", Some(COMBOBOX_CODE), vec![
            subsection("Basic", vec![
                html!("div", {
                    .style("max-width", "400px")
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_combobox!({
                        .value(combobox_value.clone())
                        .label("Country".to_string())
                        .placeholder("Search countries...".to_string())
                        .options(vec![
                            GlassSelectOption { label: "United States".to_string(), value: "us".to_string() },
                            GlassSelectOption { label: "United Kingdom".to_string(), value: "uk".to_string() },
                            GlassSelectOption { label: "Canada".to_string(), value: "ca".to_string() },
                            GlassSelectOption { label: "Germany".to_string(), value: "de".to_string() },
                            GlassSelectOption { label: "France".to_string(), value: "fr".to_string() },
                            GlassSelectOption { label: "Japan".to_string(), value: "jp".to_string() },
                            GlassSelectOption { label: "Australia".to_string(), value: "au".to_string() },
                            GlassSelectOption { label: "Brazil".to_string(), value: "br".to_string() },
                        ])
                    }))
                }),
            ]),
            subsection("Validation", vec![
                html!("div", {
                    .style("max-width", "400px")
                    .dwclass!("flex flex-col gap-4")
                    .child(glass_combobox!({
                        .value(validated_combobox.clone())
                        .label("Country (required)".to_string())
                        .placeholder("Search countries...".to_string())
                        .options(vec![
                            GlassSelectOption { label: "United States".to_string(), value: "us".to_string() },
                            GlassSelectOption { label: "United Kingdom".to_string(), value: "uk".to_string() },
                            GlassSelectOption { label: "Canada".to_string(), value: "ca".to_string() },
                        ])
                        .is_valid_signal(validated_combobox.signal_cloned().map(|v| {
                            if v.is_some() {
                                ValidationResult::Valid
                            } else {
                                ValidationResult::Invalid { message: "Please select a country".to_string() }
                            }
                        }))
                    }))
                }),
            ]),
        ]))
    })
}
