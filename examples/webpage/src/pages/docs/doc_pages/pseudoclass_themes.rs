use crate::pages::docs::code_widget::code;
use crate::pages::docs::doc_pages::doc_page::doc_page_title;
use crate::pages::docs::example_box::example_box;
use dominator::{clone, events, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use example_html_highlight_macro::example_html;
use futures_signals::signal::Mutable;

pub fn pseudo_class_themes() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Pseudo Classes"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text(r#"We can apply general pseudo classes, such as the :is() class, to any DWIND class.
             This has many use cases, one of which is theming: every is(.light) variant below
             activates when a parent element carries the light class."#)
        }))
        .child(example_box(pseudo_class_theme(), false))
        .child(code(&PSEUDO_CLASS_THEME_EXAMPLE_HTML_MAP))

        // variants
        .child(doc_page_title("Variants"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text(r#"We can also apply pseudo classes to specific children of a parent element:"#)
        }))
        .child(example_box(variants(), false))
        .child(code(&VARIANTS_EXAMPLE_HTML_MAP))

        // pseudo-elements
        .child(doc_page_title("Pseudo Elements"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text(r#"::before and ::after are variants like any other. dwind adds the content: "" they
             need in order to render, so a utility is enough on its own. It goes through a
             --dw-content custom property, so composing several before: utilities keeps whichever
             content you wrote rather than the last one winning."#)
        }))
        .child(example_box(pseudo_elements(), false))
        .child(code(&PSEUDO_ELEMENTS_EXAMPLE_HTML_MAP))

        // arbitrary declarations
        .child(doc_page_title("Arbitrary Declarations"))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-300 leading-relaxed m-t-4 m-b-2")
            .text(r#"When a property has no utility — a vendor-prefixed mask, a custom property, a
             one-off gradient — write the declaration inline in square brackets. It is unambiguous
             against the variant syntax because a variant's ] is always followed by a colon."#)
        }))
        .child(html!("p", {
            .dwclass!("text-woodsmoke-400 leading-relaxed m-0 m-b-2")
            .text("Values pass through untouched — spaces are fine inside the brackets, and so is \
                   any character, so [content:'→'] works. Modifiers go first: hover:[color:red], \
                   not [color:red]:hover.")
        }))
        .child(example_box(arbitrary_declarations(), false))
        .child(code(&ARBITRARY_DECLARATIONS_EXAMPLE_HTML_MAP))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn pseudo_elements() -> Dom {
    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-6 justify-center w-full p-4")
        // A decorative corner notch, drawn entirely by ::before.
        .child(html!("div", {
            .dwclass!("relative rounded-lg border border-woodsmoke-700 p-6 text-woodsmoke-200")
            .dwclass!("[&::before]:absolute [&::before]:[top:-6px] [&::before]:[left:-6px]")
            .dwclass!("[&::before]:w-4 [&::before]:h-4 [&::before]:rounded-full")
            .dwclass!("[&::before]:[background:#D5B65F]")
            .text("[&::before] corner dot")
        }))
        // The `before:` shorthand, with an explicit content override.
        .child(html!("div", {
            .dwclass!("relative rounded-lg border border-woodsmoke-700 p-6 text-woodsmoke-200")
            .dwclass!("before:[content:'→'] before:m-r-2 before:text-candlelight-400")
            .text("before: shorthand")
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn arbitrary_declarations() -> Dom {
    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-6 justify-center w-full p-4")
        .child(html!("div", {
            .dwclass!("rounded-lg p-6 text-woodsmoke-950 font-bold")
            .dwclass!("[background:conic-gradient(from 210deg, #D5B65F, #5FB0D5, #D5B65F)]")
            .text("conic-gradient")
        }))
        .child(html!("div", {
            .dwclass!("rounded-lg border border-woodsmoke-700 p-6 text-woodsmoke-200")
            .dwclass!("[writing-mode:vertical-rl] [letter-spacing:0.2em]")
            .text("vertical-rl")
        }))
        .child(html!("div", {
            .dwclass!("rounded-lg border border-woodsmoke-700 p-6")
            .dwclass!("[--accent:#75D55F] [color:var(--accent)] [box-shadow:0 0 0 1px var(--accent)]")
            .text("--accent custom property")
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn pseudo_class_theme() -> Dom {
    let light = Mutable::new(false);

    html!("div", {
        // the `light` class on this parent flips every is(.light) variant below
        .class_signal("light", light.signal())
        .dwclass!("w-full rounded-lg p-6 transition-colors")
        .dwclass!("bg-woodsmoke-950 is(.light):bg-woodsmoke-100")
        .dwclass!("flex flex-col gap-4 align-items-center")
        .child(html!("button", {
            .dwclass!("rounded-full p-l-4 p-r-4 p-t-1 p-b-1 font-bold border-none cursor-pointer transition-colors")
            .dwclass!("bg-candlelight-400 hover:bg-candlelight-300 text-woodsmoke-950")
            .text("Toggle theme")
            .event(clone!(light => move |_: events::Click| {
                light.set(!light.get());
            }))
        }))
        .child(html!("div", {
            .dwclass!("rounded-lg border p-4 w-full transition-colors")
            .dwclass!("border-woodsmoke-700 is(.light *):border-woodsmoke-300")
            .dwclass!("bg-woodsmoke-900 is(.light *):bg-woodsmoke-50")
            .child(html!("div", {
                .dwclass!("font-bold transition-colors")
                .dwclass!("text-woodsmoke-50 is(.light *):text-woodsmoke-950")
                .text("Theme-aware card")
            }))
            .child(html!("p", {
                .dwclass!("text-sm m-0 leading-relaxed transition-colors")
                .dwclass!("text-woodsmoke-300 is(.light *):text-woodsmoke-600")
                .text("Every class pair here is a plain utility plus its is(.light) variant — no JavaScript theme logic, just CSS selectors.")
            }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark", "base16-ocean.light"])]
fn variants() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col justify-center align-items-center gap-2 w-full")
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
