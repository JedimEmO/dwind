pub mod string_rendering;

use crate::codegen::string_rendering::{
    class_name_to_raw_identifier, class_name_to_struct_identifier, sanitize_class_prefix,
};
use crate::grammar::DwindClassSelector;
use dwind_base::media_queries::Breakpoint;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn render_classes(
    classes: Vec<DwindClassSelector>,
) -> Vec<(TokenStream, Option<BreakpointInfo>, bool)> {
    classes
        .into_iter()
        .map(render_dwind_class)
        .collect::<Vec<_>>()
}

pub fn render_generate_dwind_class(class_name: String, class: DwindClassSelector) -> TokenStream {
    let ident = Ident::new(
        class_name_to_struct_identifier(&class_name).as_str(),
        Span::call_site(),
    );
    let raw_ident = Ident::new(
        class_name_to_raw_identifier(&class_name).as_str(),
        Span::call_site(),
    );
    let raw_inner_ident = Ident::new(
        class_name_to_raw_identifier(&class.class_name).as_str(),
        Span::call_site(),
    );

    let raw = if class.is_generator() {
        let generator_call = render_generator_call(&class);

        quote! { #generator_call }
    } else {
        quote! { #raw_inner_ident }
    };

    let doc_str = format!("generator call: `{}`", raw);

    let rendered_class = render_dwind_class(class).0;

    quote! {
        #[doc(hidden)]
        pub static #raw_ident: &str = #raw;
        #[doc = #doc_str]
        pub static #ident: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
            #rendered_class
        });
    }
}

pub fn render_dwind_class(
    class: DwindClassSelector,
) -> (TokenStream, Option<BreakpointInfo>, bool) {
    let breakpoint = class.get_breakpoint();

    if class.is_generator() {
        return (render_generator(class), breakpoint, true);
    }

    if class.pseudo_classes.is_empty() && class.variant.is_none() {
        let class_ident = Ident::new(
            &class_name_to_struct_identifier(&class.class_name).to_uppercase(),
            Span::call_site(),
        );

        (quote! { &* #class_ident }, breakpoint, false)
    } else {
        let pseudo_selector = if class.pseudo_classes.is_empty() {
            class.variant.unwrap_or("".to_string())
        } else {
            format!(
                "{}:{}",
                class.variant.clone().unwrap_or("".to_string()),
                class.pseudo_classes.join(":")
            )
        };

        let class_raw_ident = Ident::new(
            &class_name_to_raw_identifier(&class.class_name),
            Span::call_site(),
        );
        let class_name = class.class_name;
        let class_prefix = sanitize_class_prefix(&class_name);

        (
            quote! {
                dominator::class! {
                    # ! [prefix=#class_prefix]
                    .dominator::pseudo!(#pseudo_selector, {
                        .raw(&* #class_raw_ident)
                    })
                }
            },
            breakpoint,
            true,
        )
    }
}

pub fn render_generator(class: DwindClassSelector) -> TokenStream {
    assert!(class.is_generator(), "class {class:?} must be a generator");

    let generator_name = format!("{}generator", class.class_name).to_lowercase();
    let generator_classname = format!("{}{}", class.class_name, class.generator_params.join(""));
    let generator_call = render_generator_call(&class);

    if class.pseudo_classes.is_empty() {
        let class_prefix = sanitize_class_prefix(&generator_name);

        quote! { dominator::class! {
            # ! [prefix=#class_prefix]
            .raw(#generator_call)
        }}
    } else {
        let pseudo_selector = format!(":{}", class.pseudo_classes.join(":"));
        let class_prefix = sanitize_class_prefix(&generator_classname);

        quote! {
            dominator::class! {
                # ! [prefix=#class_prefix]
                .dominator::pseudo!(#pseudo_selector, {
                    .raw( #generator_call )
                })
            }
        }
    }
}

fn render_generator_call(class: &DwindClassSelector) -> TokenStream {
    let generator_name = format!("{}generator", class.class_name).to_lowercase();
    let generator_params = class.generator_params.clone();

    let generator_ident = Ident::new(&generator_name, Span::call_site());

    quote! {
        #generator_ident!( #(#generator_params),*)
    }
}

#[derive(Clone)]
pub struct BreakpointInfo {
    pub breakpoint: Breakpoint,
    pub modifier: Option<String>,
    pub is_media_query: bool,
}
