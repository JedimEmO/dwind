//! Emission for [`crate::dwkeyframes`].
//!
//! The generated `ANIMATE_*` pair is deliberately the same shape that
//! `dominator-css-bindgen` emits for a CSS-file utility — a `&'static str` body
//! plus a `Lazy<String>` class — so `dwclass!` resolves it with no changes to
//! the macro at all. The only addition is the `ensure()` call that injects the
//! `@keyframes` the first time the class is instantiated.

use crate::codegen::string_rendering::class_name_to_struct_identifier;
use crate::keyframes::{DwKeyframesInput, KeyframesBody, KeyframesEntry};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

pub fn render(input: DwKeyframesInput) -> TokenStream {
    let path: Path = input
        .path
        .clone()
        .unwrap_or_else(|| syn::parse_quote!(dwind::prelude::keyframes));

    let items = input
        .entries
        .iter()
        .map(|entry| render_entry(entry, input.prefix.as_deref(), &path))
        .collect::<Vec<_>>();

    let register_fn = input.register_fn.as_ref().map(|fn_ident| {
        let handles = input.entries.iter().map(handle_ident).collect::<Vec<_>>();

        let doc = format!(
            "Eagerly injects the {} `@keyframes` rule(s) declared in this module.",
            handles.len()
        );

        quote! {
            #[doc = #doc]
            pub fn #fn_ident() {
                #( #handles.ensure(); )*
            }
        }
    });

    quote! {
        #( #items )*
        #register_fn
    }
}

/// `fade_up` -> `FADE_UP_KEYFRAMES`
fn handle_ident(entry: &KeyframesEntry) -> Ident {
    Ident::new(
        &format!("{}_KEYFRAMES", entry.ident.to_string().to_uppercase()),
        entry.ident.span(),
    )
}

/// `fade_up` -> `fade-up`, the CSS-facing spelling.
fn kebab(ident: &Ident) -> String {
    ident.to_string().replace('_', "-")
}

/// The `@keyframes` body, as one string.
fn body_literal(entry: &KeyframesEntry) -> String {
    match &entry.body {
        KeyframesBody::Raw(raw) => raw.value(),
        KeyframesBody::Stops(stops) => stops
            .iter()
            .map(|stop| {
                format!(
                    "{} {{ {} }}",
                    stop.selector.value(),
                    stop.declarations.value()
                )
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn render_entry(entry: &KeyframesEntry, prefix: Option<&str>, path: &Path) -> TokenStream {
    let handle = handle_ident(entry);
    let name_ident = Ident::new(&format!("{handle}_NAME"), entry.ident.span());
    let body_ident = Ident::new(&format!("{handle}_BODY"), entry.ident.span());

    // The CSS keyframe name, as an expression that is still a literal after
    // expansion so it can be `concat!`ed into the animation shorthand.
    //
    // `env!` expands in the *consuming* crate, so two crates that both declare
    // `fade_up` get distinct names without having to coordinate.
    let name_expr: TokenStream = match (&entry.name_override, prefix) {
        (Some(exact), _) => quote! { #exact },
        (None, Some(prefix)) => {
            let full = format!("{}-{}", prefix, kebab(&entry.ident));
            quote! { #full }
        }
        (None, None) => {
            let suffix = format!("-{}", kebab(&entry.ident));
            quote! { concat!(env!("CARGO_CRATE_NAME"), #suffix) }
        }
    };

    let body = body_literal(entry);
    let docs = &entry.docs;

    let handle_doc = format!(
        "`@keyframes` handle. `.name()`, `.ensure()` or `{{}}` formatting injects the rule.\n\n\
         ```css\n@keyframes … {{ {body} }}\n```"
    );

    let animation = entry.animation.as_ref().map(|shorthand| {
        let class_name = format!("animate-{}", kebab(&entry.ident));
        let class_ident = Ident::new(
            &class_name_to_struct_identifier(&class_name),
            entry.ident.span(),
        );
        let raw_ident = Ident::new(&format!("{class_ident}_RAW"), entry.ident.span());
        let class_prefix = class_name.replace('-', "_");
        let shorthand = format!(" {shorthand};");

        let class_doc = format!(
            "Utility class.\n\n# Example\n```rust,ignore\nhtml!(\"div\", {{ .dwclass!(\"{class_name}\") }});\n```"
        );

        quote! {
            // An `AnimationDecl` rather than a plain `&str`, so that reading the
            // declaration registers the `@keyframes`. `dwclass!` reads this
            // directly for any modified form — `hover:animate-x`,
            // `[&::before]:animate-x` — which never touches the class below.
            #[doc(hidden)]
            pub static #raw_ident: #path::AnimationDecl =
                #path::AnimationDecl::new(&#handle, concat!("animation: ", #name_expr, #shorthand));

            #[doc = #class_doc]
            pub static #class_ident: once_cell::sync::Lazy<String> =
                once_cell::sync::Lazy::new(|| {
                    dominator::class! {
                        # ! [prefix = #class_prefix]
                        .raw(&* #raw_ident)
                    }
                });
        }
    });

    quote! {
        #[doc(hidden)]
        pub static #name_ident: &str = #name_expr;
        #[doc(hidden)]
        pub static #body_ident: &str = #body;

        #( #docs )*
        #[doc = #handle_doc]
        pub static #handle: #path::Keyframes = #path::Keyframes::new(#name_ident, #body_ident);

        #animation
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::keyframes::DwKeyframesInput;

    fn render_str(input: &str) -> String {
        let parsed: DwKeyframesInput = syn::parse_str(input).expect("failed to parse");

        render(parsed).to_string()
    }

    #[test]
    fn stops_become_one_keyframes_body() {
        let out = render_str(
            r#"
            fade_up {
                "from" => "opacity: 0;",
                "to" => "opacity: 1;",
            }
            "#,
        );

        assert!(
            out.contains(r#""from { opacity: 0; } to { opacity: 1; }""#),
            "{out}"
        );
        assert!(out.contains("FADE_UP_KEYFRAMES"), "{out}");
        // No `#[animation(...)]`, so no utility class is minted.
        assert!(!out.contains("ANIMATE_FADE_UP"), "{out}");
    }

    #[test]
    fn comma_separated_percentage_stops_survive_verbatim() {
        // The reason every fragment is a string literal: `0%` and `-8%` do not
        // round-trip through Rust's lexer.
        let out = render_str(
            r#"
            aurora {
                "0%, 100%" => "transform: translate3d(0, 0, 0) scale(1);",
                "33%" => "transform: translate3d(6%, -8%, 0) scale(1.15);",
            }
            "#,
        );

        assert!(out.contains("0%, 100% {"), "{out}");
        assert!(out.contains("translate3d(6%, -8%, 0) scale(1.15)"), "{out}");
    }

    #[test]
    fn animation_attribute_mints_a_utility_class() {
        let out = render_str(
            r#"
            #[animation("900ms ease-out both")]
            fade_up { "from" => "opacity: 0;" }
            "#,
        );

        assert!(out.contains("ANIMATE_FADE_UP_RAW"), "{out}");
        assert!(out.contains("ANIMATE_FADE_UP :"), "{out}");
        assert!(out.contains(r#"" 900ms ease-out both;""#), "{out}");
        // The declaration is an AnimationDecl, not a `&str`, so that reading it
        // from a variant registers the keyframes.
        assert!(out.contains("AnimationDecl"), "{out}");
    }

    #[test]
    fn names_are_namespaced_by_default_and_pinnable() {
        let out = render_str(r#"fade_up { "from" => "opacity: 0;" }"#);
        assert!(out.contains("CARGO_CRATE_NAME"), "{out}");
        assert!(out.contains(r#""-fade-up""#), "{out}");

        let out = render_str(r#"#![prefix = "app"] fade_up { "from" => "opacity: 0;" }"#);
        assert!(out.contains(r#""app-fade-up""#), "{out}");
        assert!(!out.contains("CARGO_CRATE_NAME"), "{out}");

        // `#[name]` pins the exact CSS name, which is how dwind keeps `spin`.
        let out = render_str(r#"#[name = "spin"] spin { "from" => "opacity: 0;" }"#);
        assert!(out.contains(r#""spin""#), "{out}");
        assert!(!out.contains("CARGO_CRATE_NAME"), "{out}");
    }

    #[test]
    fn register_fn_ensures_every_declared_rule() {
        let out = render_str(
            r#"
            #![register_fn = "app_keyframes"]
            a { "from" => "opacity: 0;" }
            b { "from" => "opacity: 0;" }
            "#,
        );

        assert!(out.contains("fn app_keyframes"), "{out}");
        assert!(out.contains("A_KEYFRAMES . ensure ()"), "{out}");
        assert!(out.contains("B_KEYFRAMES . ensure ()"), "{out}");
    }

    #[test]
    fn raw_bodies_pass_through_untouched() {
        let out = render_str(r#"marquee = "from { left: 0; } to { left: -50%; }";"#);

        assert!(
            out.contains(r#""from { left: 0; } to { left: -50%; }""#),
            "{out}"
        );
    }

    #[test]
    fn an_empty_block_is_rejected() {
        assert!(syn::parse_str::<DwKeyframesInput>("empty { }").is_err());
    }

    #[test]
    fn unknown_options_are_rejected() {
        assert!(syn::parse_str::<DwKeyframesInput>(
            r#"#![nonsense = "x"] a { "from" => "opacity: 0;" }"#
        )
        .is_err());
        assert!(syn::parse_str::<DwKeyframesInput>(
            r#"#[nonsense = "x"] a { "from" => "opacity: 0;" }"#
        )
        .is_err());
    }
}
