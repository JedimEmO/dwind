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

    static THEME_CLASS: Lazy<String> = Lazy::new(|| {
        class! {
            .raw(ColorsCssVariables::new(
                DWIND_COLORS.get("charm").unwrap(),
                DWIND_COLORS.get("apple").unwrap()
            ).to_style_sheet_raw())
        }
    });

    static THEME_CLASS_LIGHT: Lazy<String> = Lazy::new(|| {
        class! {
            .raw(
                ColorsCssVariables::new(
                    DWIND_COLORS.get("apple").unwrap(),
                    DWIND_COLORS.get("bunker").unwrap())
                .to_style_sheet_raw())
        }
    });

    html!("div", {
        .class_signal(&*THEME_CLASS, dark_theme.signal())
        .class_signal(&*THEME_CLASS_LIGHT, not(dark_theme.signal()))
        .class_signal("light", not(dark_theme.signal()))
        .dwclass!("m-x-auto")
        .child(html!("div", {
            .dwclass!("bg-picton-blue-950")
            .dwclass!("is(.light *):bg-picton-blue-100")
            .dwclass!("w-52 h-44")
            .dwclass!("flex align-items-center justify-center")
            .child(button!({
                .content(Some(text("Toggle Theme")))
                .click_handler(clone!(dark_theme => move |_| {
                    dark_theme.set(!dark_theme.get());
                }))
            }))
        }))
    })
}
