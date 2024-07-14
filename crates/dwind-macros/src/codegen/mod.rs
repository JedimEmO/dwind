pub mod string_rendering;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::codegen::string_rendering::{class_name_to_raw_identifier, class_name_to_struct_identifier, sanitize_class_prefix};
use crate::grammar::DwindClassSelector;

pub fn render_classes(classes: Vec<DwindClassSelector>) -> Vec<proc_macro2::TokenStream> {
    classes.into_iter().map(|class| {
        render_dwind_class(class)
    }).collect::<Vec<proc_macro2::TokenStream>>()
}

pub fn render_generate_dwind_class(class_name: String, class: DwindClassSelector) -> TokenStream {
    let ident = Ident::new(class_name_to_struct_identifier(&class_name).as_str(), Span::call_site());
    let rendered_class = render_dwind_class(class);

    quote! {
        pub static #ident: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
            #rendered_class
        });
    }
}

pub fn render_dwind_class(class: DwindClassSelector) -> TokenStream {
    if class.is_generator() {
        return render_generator(class);
    }

    if class.pseudo_classes.is_empty() {
        let class_ident = Ident::new(&class.class_name.to_uppercase(), Span::call_site());

        quote! { &* #class_ident }
    } else {
        let pseudo_selector = format!(":{}", class.pseudo_classes.join(":"));
        let class_raw_ident = Ident::new(&class_name_to_raw_identifier(&class.class_name), Span::call_site());
        let class_name = class.class_name;
        let class_prefix = sanitize_class_prefix(&class_name);

        quote! {
            dominator::class! {
                # ! [prefix=#class_prefix]

                .dominator::pseudo!(#pseudo_selector, {
                    .raw(&* #class_raw_ident)
                })
            }
        }
    }
}

pub fn render_generator(class: DwindClassSelector) -> proc_macro2::TokenStream {
    assert!(class.is_generator(), "class {class:?} must be a generator");

    let generator_name = format!("{}generator", class.class_name).to_lowercase();
    let generator_classname = format!("{}{}", class.class_name, class.generator_params.join(""));
    let generator_params = class.generator_params;

    if class.pseudo_classes.is_empty() {
        let class_ident = Ident::new(&generator_name, Span::call_site());
        let class_prefix = sanitize_class_prefix(&generator_name);

        quote! { dominator::class! {
            # ! [prefix=#class_prefix]
            .raw(#class_ident! ( #(#generator_params),*))
        }}
    } else {
        let pseudo_selector = format!(":{}", class.pseudo_classes.join(":"));
        let class_ident = Ident::new(&generator_name, Span::call_site());
        let class_prefix = sanitize_class_prefix(&generator_classname);

        quote! {
            dominator::class! {
                # ! [prefix=#class_prefix]
                .dominator::pseudo!(#pseudo_selector, {
                    .raw( #class_ident!( #(#generator_params),*))
                })
            }
        }
    }
}

