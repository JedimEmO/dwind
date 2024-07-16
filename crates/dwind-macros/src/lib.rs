mod codegen;
pub(crate) mod grammar;
mod macro_inputs;
use crate::codegen::string_rendering::class_name_to_struct_identifier;
use crate::codegen::{render_classes, render_generate_dwind_class};
use crate::grammar::{parse_class_string, parse_selector};
use macro_inputs::{DwGenerateInput, DwGenerateMapInput, DwindInput};
use proc_macro::TokenStream;
use quote::quote;

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
    let DwindInput {
        self_ident,
        classes,
    } = syn::parse::<DwindInput>(input).unwrap();

    let classes = parse_class_string(classes.value().as_str()).unwrap();
    let classes = render_classes(classes);

    let classes = classes.into_iter().map(|class| {
        quote! {
            .class(#class)
        }
    });

    quote! {
        #self_ident #(#classes)*
    }
    .into()
}

/// Generates a dwind class that can later be used by the 'dwclass!()' macro.
///
/// Using this will create a lazy static in the scope from which the macro is invoked, so it can be used to create
/// styling modules.
///
///
///
/// # Examples
///
/// ```rust,no_run
/// # use dwind_macros::dwclass;
/// # use dominator::html;
/// mod my_custom_theme {
///     use dwind_macros::dwgenerate;
///     macro_rules! padding_generator {
///         ($padding:tt) => {
///             const_format::formatcp!("padding: {};", $padding)
///         };
///     }
///
///     dwgenerate!("nth-2-padding", "nth-child(2):hover:padding-[20px]");
/// }
///
/// use my_custom_theme::*;
///
/// // Now use the generated pseudoclass on an html element
/// html!("div", {
///   .text("hi there")
///   .dwclass!("nth-2-padding")
/// });
/// ```
#[proc_macro]
pub fn dwgenerate(input: TokenStream) -> TokenStream {
    let DwGenerateInput {
        class_definition,
        class_name,
    } = syn::parse(input).unwrap();

    let class_definition = parse_selector(class_definition.value().as_str())
        .map(|v| v.1)
        .unwrap();

    let class_name = class_name_to_struct_identifier(&class_name.value());

    let rendered = render_generate_dwind_class(class_name, class_definition);

    rendered.into()
}

///
/// # Examples
///
/// ```rust,no_run
/// # use dwind_macros::dwgenerate_map;
/// # use dominator::html;
///
/// macro_rules! bg_color_generator {
///    ($color:tt) => {
///     const_format::formatcp!("background-color: {};", $color)
///    };
/// }
///
/// dwgenerate_map!("bg-color-hover", "hover:bg-color-", [
///     ("blue-900", "#aaaafa"),
///     ("blue-800", "#9999da"),
///     ("blue-700", "#8888ca"),
///     ("blue-600", "#7777ba"),
///     ("blue-500", "#6666aa"),
///     ("blue-400", "#55559a"),
///     ("blue-300", "#44448a"),
///     ("blue-200", "#33337a"),
///     ("blue-100", "#22226a"),
///     ("blue-50", "#11115a")
/// ]);
/// ```
#[proc_macro]
pub fn dwgenerate_map(input: TokenStream) -> TokenStream {
    let input: DwGenerateMapInput = syn::parse(input).unwrap();

    let DwGenerateMapInput {
        base_output_ident,
        dwind_class_lit,
        args,
    } = input;

    let output = args.tuples.into_iter().map(|input_tuple| {
        let ident_name = class_name_to_struct_identifier(&format!(
            "{}-{}",
            base_output_ident.value(),
            input_tuple.first.value()
        ));

        let class_literal = format!(
            "{}[{}]",
            dwind_class_lit.value(),
            input_tuple.second.value()
        );

        let class = parse_selector(class_literal.as_str()).unwrap().1;

        render_generate_dwind_class(ident_name, class)
    });

    let out = quote! {
        #(#output)*
    }
    .into();

    out
}
