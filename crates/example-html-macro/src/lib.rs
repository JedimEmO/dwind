use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, ExprArray, ExprLit, Lit, Token};
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

struct ExampleHtmlArgs {
    themes: Option<Vec<String>>,
}

impl Parse for ExampleHtmlArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut themes = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "themes" => {
                    let mut out_themes = vec![];
                    input.parse::<Token![=]>()?;
                    let themes_ = input.parse::<ExprArray>()?;

                    for theme in themes_.elems {
                        match theme {
                            Expr::Lit(ExprLit { lit, .. }) => match lit {
                                Lit::Str(str) => out_themes.push(str.value()),
                                _ => {
                                    return Err(syn::Error::new(
                                        Span::call_site(),
                                        "expected a string literal",
                                    ))
                                }
                            },
                            _ => {
                                return Err(syn::Error::new(
                                    Span::call_site(),
                                    "expected a string literal",
                                ))
                            }
                        }
                    }

                    themes = Some(out_themes);
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unexpected argument: {}", ident),
                    ))
                }
            }
        }

        Ok(ExampleHtmlArgs { themes })
    }
}

#[proc_macro_attribute]
pub fn example_html(args: TokenStream, token_stream: TokenStream) -> TokenStream {
    let args = syn::parse::<ExampleHtmlArgs>(args).unwrap();

    let themes = args
        .themes
        .or(Some(vec!["base16-ocean.dark".to_string()]))
        .unwrap();

    let fn_ = syn::parse::<syn::ItemFn>(token_stream.clone()).unwrap();

    let sig = fn_.sig.clone();
    let fn_name = quote! {#sig}.to_string();
    let source_code = fn_
        .block
        .span()
        .source_text()
        .or(Some("".to_string()))
        .expect("did not find function block source text");

    let rendered_themes = render_themes(
        fn_.sig.ident.to_string(),
        format!("{fn_name} {source_code}"),
        themes,
    );
    let original_tokens: proc_macro2::TokenStream = token_stream.into();

    let out: TokenStream = quote! {
        #rendered_themes
        #original_tokens
    }
    .into();

    out
}

fn render_themes(fn_name: String, code: String, themes: Vec<String>) -> proc_macro2::TokenStream {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let sr = syntax_set.find_syntax_by_extension("rs").unwrap();
    let theme_set = ThemeSet::load_defaults();

    let mut rendered_themes = vec![];

    for theme_name in themes {
        let mut theme = theme_set.themes[&theme_name].clone();

        theme.settings.background = Some(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });

        let rendered_theme = highlighted_html_for_string(&code, &syntax_set, sr, &theme)
            .unwrap()
            .replace("style=\"background-color:#000000;\"", "");

        rendered_themes.push(quote! {( #theme_name.to_string(), #rendered_theme.to_string())});
    }

    let fn_example_ident = Ident::new(
        format!("{}_EXAMPLE_HTML_MAP", fn_name)
            .to_uppercase()
            .as_str(),
        Span::call_site(),
    );

    quote! {
        pub static #fn_example_ident: once_cell::sync::Lazy<std::collections::BTreeMap<String, String>> = once_cell::sync::Lazy::new(|| {
             std::collections::BTreeMap::from([
                #(#rendered_themes),*
            ])
        });
    }
}
