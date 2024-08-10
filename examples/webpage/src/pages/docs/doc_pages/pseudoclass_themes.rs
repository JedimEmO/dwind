use dominator::{Dom, events, text};
use futures_signals::signal::{Mutable, not};
use dwind_macros::dwclass;
use dwind::prelude::*;
use dwui::prelude::*;
use example_html_macro::example_html;
use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::doc_page_title;
use crate::pages::docs::example_box::example_box;

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

    html!("div", {
        .dwclass!("m-x-auto")
        .class_signal("light", not(dark_theme.signal()))
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