use dominator::{text, Dom};
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
        }))
    })
}

fn example_card() -> Dom {
    card!({
        .scheme(ColorScheme::Primary)
        .apply(|b| {
            dwclass!(b, "w-64 h-64 flex-initial")
        })
    })
}

fn example_card_buttons() -> Dom {
    card!({
        .scheme(ColorScheme::Void)
        .apply(|b| {
            dwclass!(b, "p-4 w-64 h-64 flex-initial flex flex-col gap-4")
            .children([
                html!("h1", {
                    .dwclass!("font-extrabold")
                    .text("Flat Buttons")
                }),
                button!({
                    .content(Some(text("Primary Flat")))
                }),
                button!({
                    .disabled(true)
                    .content(Some(text("Primary Flat Disabled")))
                })
            ])
        })
    })
}

fn example_card_border_buttons() -> Dom {
    card!({
        .scheme(ColorScheme::Void)
        .apply(|b| {
            dwclass!(b, "p-4 w-64 h-64 flex-initial flex flex-col gap-4")
            .children([
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

fn example_card_input() -> Dom {
    // let value = Mutable::new("Some string value".to_string());
    let value = Mutable::new("".to_string());

    card!({
        .scheme(ColorScheme::Void)
        .apply(move |b| {
            dwclass!(b, "p-4 w-64 h-64 flex-initial flex flex-col gap-4")
                .children([
                    text_input!({
                        .value(value.clone())
                        .label("Hi there".to_string())
                    }),
                    text_input!({
                        .value(value.clone())
                        .is_valid(ValidationResult::Invalid { message: "Always!!".to_string() })
                        .label("Always invalid".to_string())
                    }),
                    text_input!({
                        .input_type(TextInputType::Password)
                        .label("Password".to_string())
                    }),
            ])
        })
    })
}
