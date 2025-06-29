use dominator::{clone, events, text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use futures_signals::signal::Mutable;

pub fn dwui_example_small() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(html!("div", {
            .dwclass!("flex justify-center align-items-center gap-4 @sm:flex-row @<sm:flex-col")
            .child(example_card_border_buttons())
            .child(example_card_input())
            .child(example_card_modal())
        }))
    })
}

pub fn example_card_border_buttons() -> Dom {
    card!({
        .scheme(ColorScheme::Void)
        .apply(|b| {
            dwclass!(b, "p-4 w-64 h-96 flex-initial flex flex-col gap-4")
            .children([
                button!({
                    .content(Some(text("Primary Flat")))
                }),
                button!({
                    .disabled(true)
                    .content(Some(text("Primary Flat Disabled")))
                }),
                button!({
                    .button_type(ButtonType::Border)
                    .content(Some(text("Primary Border")))
                }),
                button!({
                    .button_type(ButtonType::Border)
                    .disabled(true)
                    .content(Some(text("Primary Border Disabled")))
                })
            ])
        })
    })
}

pub fn example_card_input() -> Dom {
    // let value = Mutable::new("Some string value".to_string());
    let value = Mutable::new("".to_string());
    let f32_value = Mutable::new(25.);

    card!({
        .scheme(ColorScheme::Void)
        .apply(move |b| {
            dwclass!(b, "p-4 w-64 h-96 flex-initial flex flex-col gap-4")
            .children([
                text_input!({
                    .claim_focus(true)
                    .value(value.clone())
                    .label("Hi there".to_string())
                }),
                text_input!({
                    .value(value.clone())
                    .is_valid(ValidationResult::Invalid { message: "Always!!".to_string() })
                    .label("Always invalid".to_string())
                }),
                text_input!({
                    .value(value.clone())
                    .is_valid_signal(value.signal_ref(|v| {
                        if v.to_lowercase() == "bananas" {
                            ValidationResult::Valid
                        } else {
                            ValidationResult::Invalid { message: "Give me bananas!".to_string() }
                        }
                    }))
                    .label("Accepts bananas".to_string())
                }),
                text_input!({
                    .input_type(TextInputType::Password)
                    .label("Password".to_string())
                }),
                slider!({
                    .value(f32_value)
                    .label("Some slider".to_string())
                }),
                select!({
                    .label("Some dropdown".to_string())
                    .value(value.clone())
                    .options(vec![
                        ("a".to_string(), "Option A".to_string()),
                        ("b".to_string(), "Option B".to_string()),
                        ("c".to_string(), "Option C".to_string()),
                    ])
                })
            ])
        })
    })
}

pub fn example_card_modal() -> Dom {
    let show_modal = Mutable::new(false);
    let show_modal_small = Mutable::new(false);

    card!({
        .scheme(ColorScheme::Void)
        .apply(clone!(show_modal, show_modal_small => move |b| {
            dwclass!(b, "p-4 w-64 h-96 flex-initial flex flex-col gap-4")
            .children([
                heading!({
                    .content(text("Modal Example"))
                    .text_size(TextSize::Large)
                }),
                button!({
                    .content(Some(text("Open Large Modal")))
                    .on_click(clone!(show_modal => move |_: events::Click| {
                        show_modal.set(true);
                    }))
                }),
                button!({
                    .button_type(ButtonType::Border)
                    .content(Some(text("Open Small Modal")))
                    .on_click(clone!(show_modal_small => move |_: events::Click| {
                        show_modal_small.set(true);
                    }))
                }),
                modal!({
                    .open_signal(show_modal.signal())
                    .size(ModalSize::Large)
                    .on_close(clone!(show_modal => move || {
                        show_modal.set(false);
                    }))
                    .content(Some(html!("div", {
                        .dwclass!("flex flex-col gap-4")
                        .children([
                            heading!({
                                .content(text("Large Modal"))
                                .text_size(TextSize::ExtraLarge)
                            }),
                            html!("p", {
                                .text("This is a large modal dialog (900px wide). You can close it by clicking the X button, clicking outside, or pressing Escape.")
                            }),
                            button!({
                                .content(Some(text("Close Modal")))
                                .on_click(clone!(show_modal => move |_: events::Click| {
                                    show_modal.set(false);
                                }))
                            })
                        ])
                    })))
                }),
                modal!({
                    .open_signal(show_modal_small.signal())
                    .size(ModalSize::Small)
                    .on_close(clone!(show_modal_small => move || {
                        show_modal_small.set(false);
                    }))
                    .content(Some(html!("div", {
                        .dwclass!("flex flex-col gap-4")
                        .children([
                            heading!({
                                .content(text("Small Modal"))
                                .text_size(TextSize::ExtraLarge)
                            }),
                            html!("p", {
                                .text("This is a small modal dialog (24rem wide).")
                            }),
                            button!({
                                .content(Some(text("Close Modal")))
                                .on_click(clone!(show_modal_small => move |_: events::Click| {
                                    show_modal_small.set(false);
                                }))
                            })
                        ])
                    })))
                })
            ])
        }))
    })
}
