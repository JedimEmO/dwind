use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::doc_page_title;
use crate::pages::docs::example_box::example_box;
use dominator::{text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use dwui::theme::prelude::ColorsCssVariables;
use example_html_macro::example_html;
use futures_signals::signal::{not, Mutable};
use once_cell::sync::Lazy;
use crate::pages::dwui::example_small::dwui_example_small;

pub fn pseudo_class_themes() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Pseudo Classes"))
        .child(html!("p", {
            .text(r#"We can apply general pseudo classes, such as the :is() class, to any DWIND class.
             This has many use cases, one of which is theming:"#)
        }))
        .child(example_box(pseudo_class_theme(), false))
        .child(code(&PSEUDO_CLASS_THEME_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn pseudo_class_theme() -> Dom {
    let dark_theme = Mutable::new(true);
    let alternate_scheme = Mutable::new(true);

    static SCHEME_CLASS: Lazy<String> = Lazy::new(|| {
        class! {
            .raw(ColorsCssVariables::new(
                DWIND_COLORS.get("charm").unwrap(),
                DWIND_COLORS.get("woodsmoke").unwrap(),
                DWIND_COLORS.get("woodsmoke").unwrap(),
            ).to_style_sheet_raw())
        }
    });

    static SCHEME_CLASS_LIGHT: Lazy<String> = Lazy::new(|| {
        class! {
            .raw(
                ColorsCssVariables::new(
                    DWIND_COLORS.get("apple").unwrap(),
                    DWIND_COLORS.get("bunker").unwrap(),
                    DWIND_COLORS.get("bunker").unwrap())
                .to_style_sheet_raw())
        }
    });

    html!("div", {
        .class_signal(&*SCHEME_CLASS, alternate_scheme.signal())
        .class_signal(&*SCHEME_CLASS_LIGHT, not(alternate_scheme.signal()))
        .class_signal("light", not(dark_theme.signal()))
        .dwclass!("m-x-auto")
        .dwclass!("bg-bunker-950 w-full flex align-items-center flex-col")
        .dwclass!("is(.light):bg-bunker-100")
        .child(html!("div", {
            .dwclass!("w-52 h-44")
            .dwclass!("flex gap-4 align-items-center justify-center")
            .child(button!({
                .content(Some(text("Toggle Theme")))
                .click_handler(clone!(dark_theme => move |_| {
                    dark_theme.set(!dark_theme.get());
                }))
            }))
            .child(button!({
                .content(Some(text("Toggle Scheme")))
                .click_handler(clone!(alternate_scheme => move |_| {
                    alternate_scheme.set(!alternate_scheme.get());
                }))
            }))
        }))
        .child(dwui_example_small())
    })
}
