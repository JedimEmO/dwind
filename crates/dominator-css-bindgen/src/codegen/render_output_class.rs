use crate::codegen::output_model::{OutputClass, OutputPseudoClass, OutputStyleSheet};
use cssparser::{ToCss, Token};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn render_output_style_sheets(
    output_style_sheets: Vec<OutputStyleSheet>,
    module_name: &str,
) -> TokenStream {
    let stylesheets = output_style_sheets
        .into_iter()
        .map(render_output_style_sheet)
        .collect::<Vec<_>>();

    let stylesheet_fn_indent = Ident::new(
        format!("apply_{}_stylesheet", module_name).as_str(),
        Span::call_site(),
    );

    quote! {
        pub fn #stylesheet_fn_indent() {
            #(#stylesheets)*
        }
    }
}

fn render_output_style_sheet(output_style_sheet: OutputStyleSheet) -> TokenStream {
    let selectors = output_style_sheet
        .selectors
        .into_iter()
        .map(|selector| {
            selector
                .into_iter()
                .map(|t| t.to_css_string().trim_matches(&['\n', '\t']).to_string())
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string()
        })
        .collect::<Vec<_>>();

    let body = render_token_body(output_style_sheet.body);

    let (moz, non_moz): (Vec<_>, Vec<_>) = selectors
        .into_iter()
        .map(|v| {
            if v.to_lowercase().contains("moz") {
                (Some(v), None)
            } else {
                (None, Some(v))
            }
        })
        .unzip();

    let moz = moz.into_iter().filter_map(|v| v).collect::<Vec<_>>();
    let non_moz = non_moz.into_iter().filter_map(|v| v).collect::<Vec<_>>();

    let moz = if !moz.is_empty() {
        quote! {
            if web_sys::window().unwrap().navigator().user_agent().unwrap().to_lowercase().contains("firefox") {
                dominator::stylesheet!([#(#moz),*], {
                    .raw(#body)
                });
            }
        }
    } else {
        quote! {}
    };

    let non_moz = if !non_moz.is_empty() {
        quote! {
            dominator::stylesheet!([#(#non_moz),*], {
                .raw(#body)
            });
        }
    } else {
        quote! {}
    };

    quote! {
        #moz
        #non_moz
    }
}

pub fn render_output_class(output_class: OutputClass) -> TokenStream {
    let name = output_class
        .identity
        .to_string()
        .replace("-", "_")
        .to_uppercase();

    let name_lower = name.to_lowercase();
    let ident = Ident::new(name.as_str(), Span::call_site());
    let ident_raw = Ident::new(format!("{name}_RAW").as_str(), Span::call_site());
    let body = render_token_body(output_class.main_class_body);
    let body = body.to_string();

    let example = format!(
        "html!(\"div\", {{ .dwclass!(\"{}\") }});",
        name.to_lowercase()
    );

    let doc_comment = format!(
        r#"Generated from css file. Class content:
`{}`
# Example
```rust,ignore
{}
```
"#,
        body.replace(";", ";\n"),
        example
    );
    let pseudo_classes = output_class
        .pseudo_classes
        .into_iter()
        .map(render_pseudo_class);

    quote! {
        #[doc(hidden)]
        pub static #ident_raw: &str = #body;
        #[doc = #doc_comment ]
        pub static #ident: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
            dominator::class! {
                # ! [prefix=#name_lower]
                .raw(#ident_raw)
                #(#pseudo_classes)*
            }
        });
    }
}

fn render_pseudo_class(pseudo_class: OutputPseudoClass) -> TokenStream {
    let pseudo_selector = pseudo_class
        .selector
        .into_iter()
        .map(|v| v.to_css_string().trim_matches(&['\n', '\t']).to_string())
        .collect::<Vec<_>>()
        .join("");
    let body = render_token_body(pseudo_class.body);
    let body = body.to_string();

    quote! {
        .dominator::pseudo!(#pseudo_selector, {
            .raw(#body)
        })
    }
}

fn render_token_body(tokens: Vec<Token>) -> String {
    tokens
        .into_iter()
        .map(|t| t.to_css_string().trim_matches(&['\n', '\t']).to_string())
        .collect::<Vec<_>>()
        .join("")
        .split(";")
        .map(|v| v.trim().to_string())
        .collect::<Vec<_>>()
        .join(";")
}
