use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::doc_page_title;
use crate::pages::docs::example_box::example_box;
use crate::pages::dwui::example_small::dwui_example_small;
use dominator::{text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use dwui::theme::prelude::ColorsCssVariables;
use example_html_highlight_macro::example_html;
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

        // variants
        .child(doc_page_title("Variants"))
        .child(html!("p", {
            .text(r#"We can also apply pseudo classes to specific children of a parent element:"#)
        }))
        .child(example_box(variants(), false))
        .child(code(&VARIANTS_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn variants() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-2")
        // Use the variant to apply style to the second child
        .dwclass!("[& > *]:nth-child(2):bg-candlelight-500")
        .dwclass!("[& > span > div:is(.foo)]:text-bunker-800")
        .dwclass!("[> span]:text-apple-300 [& *]:w-40 [& *]:text-center")
        .dwclass!("[> span]:border [& *:nth-child(odd)]:border-woodsmoke-700")
        .children([
            html!("span", { .text("a")}),
            html!("span", {
                .child(html!("div", {
                    .class("foo")
                    .text("b")
                }))
            }),
            html!("span", { .text("c")}),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn pseudo_class_theme() -> Dom {
    let dark_theme = Mutable::new(true);
    let alternate_scheme = Mutable::new(true);

    static SCHEME_CLASS: Lazy<String> = Lazy::new(|| {
        class! {
            .raw(ColorsCssVariables::new(
                DWIND_COLORS.get("picton-blue").unwrap(),
                DWIND_COLORS.get("woodsmoke").unwrap(),
                DWIND_COLORS.get("woodsmoke").unwrap(),
                DWIND_COLORS.get("red").unwrap(),
            ).to_style_sheet_raw())
        }
    });

    static SCHEME_CLASS_LIGHT: Lazy<String> = Lazy::new(|| {
        class! {
            .raw(
                ColorsCssVariables::new(
                    DWIND_COLORS.get("purple").unwrap(),
                    DWIND_COLORS.get("bunker").unwrap(),
                    DWIND_COLORS.get("bunker").unwrap(),
                    DWIND_COLORS.get("purple").unwrap(),
                )
                .to_style_sheet_raw())
        }
    });

    html!("div", {
        .class_signal(&*SCHEME_CLASS, alternate_scheme.signal())
        .class_signal(&*SCHEME_CLASS_LIGHT, not(alternate_scheme.signal()))
        .class_signal("light", not(dark_theme.signal()))
        .dwclass!("m-x-auto p-b-4")
        .dwclass!("bg-bunker-950 w-full flex align-items-center flex-col")
        .dwclass!("is(.light):bg-bunker-300")
        .child(html!("div", {
            .dwclass!("flex gap-4 align-items-center justify-center @<sm:flex-col @sm:flex-row p-t-4 p-b-4")
            .child(button!({
                .apply(|b| dwclass!(b, "w-64"))
                .content(Some(text("Toggle Theme")))
                .on_click(clone!(dark_theme => move |_| {
                    dark_theme.set(!dark_theme.get());
                }))
            }))
            .child(button!({
                .apply(|b| dwclass!(b, "w-64"))
                .content(Some(text("Toggle Scheme")))
                .on_click(clone!(alternate_scheme => move |_| {
                    alternate_scheme.set(!alternate_scheme.get());
                }))
            }))
        }))
        .child(dwui_example_small())
    })
}
