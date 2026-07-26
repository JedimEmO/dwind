use dominator::{clone, events, text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use example_html_highlight_macro::example_html;
use futures_signals::signal::Mutable;

#[example_html(themes = ["base16-ocean.dark"])]
pub fn example_card_modal() -> Dom {
    let show_modal = Mutable::new(false);
    let show_modal_small = Mutable::new(false);

    card!({
        .scheme(ColorScheme::Void)
        .apply(clone!(show_modal, show_modal_small => move |b| {
            dwclass!(b, "w-64 flex-initial flex flex-col gap-4")
            .children([
                heading!({
                    .content(text("Modal Example"))
                    .text_size(TextSize::Large)
                    .level(HeadingLevel::H2)
                }),
                button!({
                    .apply(|b| dwclass!(b, "w-full"))
                    .content(Some(text("Open Large Modal")))
                    .on_click(clone!(show_modal => move |_: events::Click| {
                        show_modal.set(true);
                    }))
                }),
                button!({
                    .apply(|b| dwclass!(b, "w-full"))
                    .button_type(ButtonType::Border)
                    .content(Some(text("Open Small Modal")))
                    .on_click(clone!(show_modal_small => move |_: events::Click| {
                        show_modal_small.set(true);
                    }))
                }),
                modal!({
                    .open_signal(show_modal.signal())
                    .size(ModalSize::Large)
                    .aria_label("Large modal example".to_string())
                    .on_close(clone!(show_modal => move || {
                        show_modal.set(false);
                    }))
                    .content(Some(html!("div", {
                        .dwclass!("flex flex-col gap-4")
                        .children([
                            heading!({
                                .content(text("Large Modal"))
                                .text_size(TextSize::ExtraLarge)
                                .level(HeadingLevel::H2)
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
                    .aria_label("Small modal example".to_string())
                    .on_close(clone!(show_modal_small => move || {
                        show_modal_small.set(false);
                    }))
                    .content(Some(html!("div", {
                        .dwclass!("flex flex-col gap-4")
                        .children([
                            heading!({
                                .content(text("Small Modal"))
                                .text_size(TextSize::ExtraLarge)
                                .level(HeadingLevel::H2)
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
