//! Input parsing for [`crate::dwkeyframes`].
//!
//! Every CSS fragment is a string literal, deliberately. Accepting bare CSS
//! tokens and reconstructing them with `TokenStream::to_string()` cannot work
//! reliably: Rust's lexer splits `0%` into two tokens, `--sx` into two puncts
//! and `.35` into a float or a dot-plus-int depending on context, and
//! `to_string()` re-inserts whitespace by its own rules. Two quote characters
//! buy exact fidelity.

pub mod codegen;

use syn::parse::{Parse, ParseStream};
use syn::{braced, Attribute, Expr, ExprLit, Ident, Lit, LitStr, Meta, Path, Token};

/// One `selector => declarations` pair inside a keyframes block.
pub struct Stop {
    pub selector: LitStr,
    pub declarations: LitStr,
}

impl Parse for Stop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let selector = input.parse::<LitStr>()?;
        input.parse::<Token![=>]>()?;
        let declarations = input.parse::<LitStr>()?;

        Ok(Self {
            selector,
            declarations,
        })
    }
}

pub enum KeyframesBody {
    /// `name { "from" => "…", "to" => "…" }`
    Stops(Vec<Stop>),
    /// `name = "from { … } to { … }";` — verbatim, for pasting existing CSS.
    Raw(LitStr),
}

pub struct KeyframesEntry {
    /// `#[doc = "…"]` attributes, forwarded to the generated items.
    pub docs: Vec<Attribute>,
    /// `#[name = "spin"]` — pins the exact CSS name, skipping the prefix.
    pub name_override: Option<String>,
    /// `#[animation("1s linear infinite")]` — also mint an `animate-<name>`
    /// utility class with this shorthand.
    pub animation: Option<String>,
    pub ident: Ident,
    pub body: KeyframesBody,
}

pub struct DwKeyframesInput {
    /// `#![prefix = "app"]` — defaults to the consuming crate's name.
    pub prefix: Option<String>,
    /// `#![register_fn = "app_keyframes"]` — emit a fn that eagerly injects all
    /// of them, for callers who want the old unconditional behaviour.
    pub register_fn: Option<Ident>,
    /// `#![path = dwind_base::keyframes]` — where the runtime lives. Defaults to
    /// `dwind::prelude::keyframes`, which is correct for anyone depending on
    /// dwind; the dwind and dwind-base crates themselves override it.
    pub path: Option<Path>,
    pub entries: Vec<KeyframesEntry>,
}

fn name_value_string(attr: &Attribute) -> syn::Result<String> {
    match &attr.meta {
        Meta::NameValue(nv) => match &nv.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => Ok(s.value()),
            other => Err(syn::Error::new_spanned(other, "expected a string literal")),
        },
        other => Err(syn::Error::new_spanned(
            other,
            "expected `name = \"value\"`",
        )),
    }
}

impl Parse for DwKeyframesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut prefix = None;
        let mut register_fn = None;
        let mut path = None;

        for attr in Attribute::parse_inner(input)? {
            let ident = attr.path().get_ident().map(|i| i.to_string());

            match ident.as_deref() {
                Some("prefix") => prefix = Some(name_value_string(&attr)?),
                Some("register_fn") => {
                    let raw = name_value_string(&attr)?;
                    register_fn = Some(Ident::new(&raw, attr.span_of_value()));
                }
                Some("path") => match &attr.meta {
                    Meta::NameValue(nv) => match &nv.value {
                        Expr::Path(p) => path = Some(p.path.clone()),
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(s), ..
                        }) => path = Some(s.parse::<Path>()?),
                        other => {
                            return Err(syn::Error::new_spanned(other, "expected a module path"))
                        }
                    },
                    other => return Err(syn::Error::new_spanned(other, "expected `path = ...`")),
                },
                _ => {
                    return Err(syn::Error::new_spanned(
                        &attr,
                        "unknown dwkeyframes! option; expected `prefix`, `register_fn` or `path`",
                    ))
                }
            }
        }

        let mut entries = vec![];

        while !input.is_empty() {
            entries.push(parse_entry(input)?);
        }

        Ok(Self {
            prefix,
            register_fn,
            path,
            entries,
        })
    }
}

fn parse_entry(input: ParseStream) -> syn::Result<KeyframesEntry> {
    let mut docs = vec![];
    let mut name_override = None;
    let mut animation = None;

    for attr in Attribute::parse_outer(input)? {
        let ident = attr.path().get_ident().map(|i| i.to_string());

        match ident.as_deref() {
            Some("doc") => docs.push(attr),
            Some("name") => name_override = Some(name_value_string(&attr)?),
            Some("animation") => animation = Some(attr.parse_args::<LitStr>()?.value()),
            _ => {
                return Err(syn::Error::new_spanned(
                    &attr,
                    "unknown keyframes attribute; expected `name` or `animation`",
                ))
            }
        }
    }

    let ident = input.parse::<Ident>()?;

    let body = if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        let raw = input.parse::<LitStr>()?;
        input.parse::<Token![;]>()?;

        KeyframesBody::Raw(raw)
    } else {
        let content;
        braced!(content in input);

        let stops = content
            .parse_terminated(Stop::parse, Token![,])?
            .into_iter()
            .collect::<Vec<_>>();

        if stops.is_empty() {
            return Err(syn::Error::new(
                ident.span(),
                "a keyframes block needs at least one stop",
            ));
        }

        KeyframesBody::Stops(stops)
    };

    Ok(KeyframesEntry {
        docs,
        name_override,
        animation,
        ident,
        body,
    })
}

/// `syn::Attribute` has no direct accessor for the span of a name-value's
/// value, and pointing at the whole attribute is good enough for diagnostics.
trait AttrSpan {
    fn span_of_value(&self) -> proc_macro2::Span;
}

impl AttrSpan for Attribute {
    fn span_of_value(&self) -> proc_macro2::Span {
        use syn::spanned::Spanned;

        self.span()
    }
}
