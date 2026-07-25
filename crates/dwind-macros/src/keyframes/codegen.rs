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
        let handles = input
            .entries
            .iter()
            .map(|entry| handle_ident(entry))
            .collect::<Vec<_>>();

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
            #[doc(hidden)]
            pub static #raw_ident: &str = concat!("animation: ", #name_expr, #shorthand);

            #[doc = #class_doc]
            pub static #class_ident: once_cell::sync::Lazy<String> =
                once_cell::sync::Lazy::new(|| {
                    #handle.ensure();
                    dominator::class! {
                        # ! [prefix = #class_prefix]
                        .raw(#raw_ident)
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
