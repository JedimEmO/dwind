use crate::codegen::output_model::{OutputClass, OutputPseudoClass};
use cssparser::{ToCss, Token};
use proc_macro2::{Ident, Span};
use quote::__private::TokenStream;
use quote::quote;

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
        .map(|v| v.to_css_string())
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
        .map(|t| t.to_css_string())
        .collect::<Vec<_>>()
        .join("")
}
