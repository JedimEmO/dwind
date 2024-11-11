use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::{HashMap, HashSet};

pub fn render_variable_definitions(module_name: &str, variable_names: Vec<String>) -> String {
    let variable_names = variable_names
        .into_iter()
        .filter_map(|name| {
            if !name.starts_with("--") {
                return None;
            }

            Some(name)
        })
        .collect::<Vec<_>>();

    render_struct_of_variable_names(module_name, variable_names).to_string()
}

fn render_struct_of_variable_names(module_name: &str, variable_names: Vec<String>) -> TokenStream {
    let rustified_variable_names = variable_names
        .iter()
        .filter_map(|name| name.strip_prefix("--").map(|v| v.to_string()))
        .collect::<Vec<String>>();

    let pairs = variable_names
        .iter()
        .cloned()
        .zip(rustified_variable_names.iter().cloned())
        .collect::<Vec<_>>();

    let mut rustified_variable_names: HashMap<String, String> = Default::default();

    for pair in pairs.iter() {
        rustified_variable_names.insert(pair.0.clone(), pair.1.clone());
    }

    let (variable_names, rustified_variable_names): (Vec<String>, Vec<String>) =
        rustified_variable_names.into_iter().unzip();

    let css_variable_names = rustified_variable_names.iter().map(|name| {
        let rust_name = Ident::new(css_ident_to_rust(&name).as_str(), Span::call_site());
        quote! {
            pub #rust_name: String,
        }
    });

    let raw_string_exprs = variable_names
        .iter()
        .zip(rustified_variable_names.iter())
        .map(|(raw_name, struct_field_name)| {
            let field_ident = Ident::new(
                css_ident_to_rust(struct_field_name).as_str(),
                Span::call_site(),
            );

            quote! {
                out.push(format!("{}: {};", #raw_name, self. #field_ident));
            }
        })
        .collect::<Vec<_>>();

    let struct_ident = Ident::new(
        format!("{}CssVariables", module_name).to_camel().as_str(),
        Span::call_site(),
    );

    quote! {
        pub struct #struct_ident {
            #(#css_variable_names)*
        }

        impl #struct_ident {
            pub fn to_style_sheet_raw(self) -> String {
                let mut out: Vec<String> = vec![];

                #(#raw_string_exprs)*

                out.join("\n")
            }
        }
    }
}

pub fn css_ident_to_rust(ident: &str) -> String {
    ident.replace("-", "_")
}
