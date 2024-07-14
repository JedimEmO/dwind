pub(crate) mod grammar;
mod codegen;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::codegen::{render_classes, render_generate_dwind_class};
use crate::codegen::string_rendering::class_name_to_struct_identifier;
use crate::grammar::{DwindClassSelector, parse_class_string, parse_selector};

/// Use dwind-macros macros on your DOMINATOR component
///
/// Basic usage:
///
/// # Example
/// ```rust,no_run
/// # use dominator::class;
/// # const SOME_CLASS: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {class! { .raw("")}});
/// use dwind_macros::dwclass;
/// dominator::html!("div", {
///     .dwclass!("some_class")
/// });
/// ```
#[proc_macro]
pub fn dwclass(input: TokenStream) -> TokenStream {
    let DwindInput { self_ident, classes } = syn::parse::<DwindInput>(input).unwrap();

    let classes = parse_class_string(classes.value().as_str()).unwrap();
    let classes = render_classes(classes);

    let classes = classes.into_iter().map(|class| {
        quote! {
            .class(#class)
        }
    });

    quote! {
        #self_ident #(#classes)*
    }.into()
}

#[proc_macro]
pub fn dwgenerate(input: TokenStream) -> TokenStream {
    let DwGenerateInput { class_definition, class_name } = syn::parse(input).unwrap();

    let class_definition = parse_selector(class_definition.value().as_str()).map(|v| v.1).unwrap();
    let class_name = class_name_to_struct_identifier(&class_name.value());

    let rendered = render_generate_dwind_class(class_name, class_definition);

    rendered.into()
}

struct DwindInput {
    self_ident: Ident,
    classes: LitStr,
}

impl Parse for DwindInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let self_ident = input.parse::<Ident>()?;
        let _sep = input.parse::<Token![,]>()?;
        let classes = input.parse::<LitStr>()?;

        Ok(DwindInput {
            self_ident,
            classes,
        })
    }
}

struct DwGenerateInput {
    class_name: LitStr,
    class_definition: LitStr,
}

impl Parse for DwGenerateInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let class_name = input.parse::<LitStr>()?;
        let _sep = input.parse::<Token![,]>()?;
        let classes = input.parse::<LitStr>()?;

        Ok(DwGenerateInput {
            class_name,
            class_definition: classes,
        })
    }
}