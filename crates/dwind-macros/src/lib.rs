mod codegen;
pub(crate) mod grammar;
mod keyframes;
mod macro_inputs;
use crate::codegen::string_rendering::class_name_to_struct_identifier;
use crate::codegen::{render_classes, render_generate_dwind_class};
use crate::grammar::{parse_class_string, parse_selector};
use crate::macro_inputs::DwindInputSignal;
use dwind_base::media_queries::Breakpoint;
use macro_inputs::{DwGenerateInput, DwGenerateMapInput, DwindInput};
use proc_macro::TokenStream;
use quote::quote;
use std::rc::Rc;

/// Use dwind-macros macros on your DOMINATOR component
///
/// Basic usage:
///
/// # Example
/// ```rust,no_run
/// # use dominator::class;
/// # const SOME_CLASS: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {class! { .raw("")}});
/// # const SOME_OTHER_CLASS_RAW: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {class! { .raw("")}});
/// use dwind_macros::dwclass;
/// dominator::html!("div", {
///     .dwclass!("some_class [>*]:hover:some-other-class")
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

    let (generator_classes, normal_classes): (Vec<_>, Vec<_>) = classes
        .into_iter()
        .map(|class| match class {
            (tokens, breakpoint, false) => (None, Some((tokens, breakpoint))),
            (tokens, breakpoint, true) => (Some((tokens, breakpoint)), None),
        })
        .unzip();

    let v = Rc::new(Some(""));

    v.as_ref().as_ref().map(|_v| {});

    let classes = normal_classes.into_iter().filter_map(|v| v).map(|(class, breakpoint)| {
        if let Some(breakpoint) = breakpoint {
            let bp = breakpoint.breakpoint;

            if breakpoint.is_media_query {
                let Breakpoint::MediaQuery(mq) = bp  else { unreachable!() };

                quote! {
                    .class_signal(#class, dominator::media_query(#mq))
                }
            } else {
                let bp_value = match bp {
                    Breakpoint::VerySmall => quote! { dwind::prelude::media_queries::Breakpoint::VerySmall },
                    Breakpoint::Small => quote! { dwind::prelude::media_queries::Breakpoint::Small },
                    Breakpoint::Medium => quote! { dwind::prelude::media_queries::Breakpoint::Medium },
                    Breakpoint::Large => quote! { dwind::prelude::media_queries::Breakpoint::Large },
                    Breakpoint::VeryLarge => quote! { dwind::prelude::media_queries::Breakpoint::VeryLarge },
                    _ => { panic!("media query breakpoint not supported here")}
                };

                if let Some(_modifier) = breakpoint.modifier {
                    quote! {
                        .class_signal(#class, dwind::prelude::media_queries::breakpoint_less_than_signal(#bp_value))
                    }
                    } else {
                        quote! {
                        .class_signal(#class, dwind::prelude::media_queries::breakpoint_active_signal(#bp_value))
                    }
                }
            }
        } else {
            quote! {
                .class(#class)
            }
        }
    });

    let gen_classes = generator_classes.into_iter().filter_map(|v| v).map(|(class_tokens, breakpoint)| {
        if let Some(breakpoint) = breakpoint {
            let bp = breakpoint.breakpoint;

            let apply = if breakpoint.is_media_query {
                let Breakpoint::MediaQuery(mq) = bp  else { unreachable!() };

                quote! {
                    .class_signal(&*foobar, dominator::media_query(#mq))
                }
            } else {
                let bp_value = match bp {
                    Breakpoint::VerySmall => quote! { dwind::prelude::media_queries::Breakpoint::VerySmall },
                    Breakpoint::Small => quote! { dwind::prelude::media_queries::Breakpoint::Small },
                    Breakpoint::Medium => quote! { dwind::prelude::media_queries::Breakpoint::Medium },
                    Breakpoint::Large => quote! { dwind::prelude::media_queries::Breakpoint::Large },
                    Breakpoint::VeryLarge => quote! { dwind::prelude::media_queries::Breakpoint::VeryLarge },
                    _ => { panic!("media query breakpoint not supported here")}
                };

                if let Some(_modifier) = breakpoint.modifier {
                    quote! {
                        .class_signal(&*foobar, dwind::prelude::media_queries::breakpoint_less_than_signal(#bp_value))
                    }
                } else {
                    quote! {
                        .class_signal(&*foobar, dwind::prelude::media_queries::breakpoint_active_signal(#bp_value))
                    }
                }
            };

            quote! {
               let #self_ident = {
                    #[doc(hidden)]
                    pub static foobar: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
                        #class_tokens
                    });

                    #self_ident . #apply
                };
            }
        } else {
            quote! {
               let #self_ident = {
                    #[doc(hidden)]
                    pub static foobar: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
                        #class_tokens
                    });

                    #self_ident . class(&*foobar)
                };
            }
        }
    });

    quote! {
        {
            #(#gen_classes)*
            #self_ident #(#classes)*
        }
    }
    .into()
}

/// Use dwind-macros macros on your DOMINATOR component
///
/// Basic usage:
///
/// # Example
/// ```rust,no_run
/// # use dominator::class;
/// # use futures_signals::signal::always;
/// # static SOME_CLASS: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {class! { .raw("")}});
/// use dwind_macros::{dwclass, dwclass_signal};
/// dominator::html!("div", {
///     .dwclass_signal!("some_class", always(true))
/// });
/// ```
#[proc_macro]
pub fn dwclass_signal(input: TokenStream) -> TokenStream {
    let DwindInputSignal {
        input: DwindInput {
            self_ident,
            classes,
        },
        signal,
    } = syn::parse::<DwindInputSignal>(input).unwrap();

    let classes = parse_class_string(classes.value().as_str()).unwrap();
    let classes = render_classes(classes);

    let (generator_classes, normal_classes): (Vec<_>, Vec<_>) = classes
        .into_iter()
        .map(|class| match class {
            (tokens, breakpoint, false) => (None, Some((tokens, breakpoint))),
            (tokens, breakpoint, true) => (Some((tokens, breakpoint)), None),
        })
        .unzip();

    let classes = normal_classes
        .into_iter()
        .filter_map(|v| v)
        .map(|(class, _breakpoint)| {
            quote! {
                .class_signal(#class, #signal)
            }
        });

    let gen_classes = generator_classes.into_iter().filter_map(|v| v).map(|(class_tokens, _breakpoint)| {
        quote! {
           let #self_ident = {
                #[doc(hidden)]
                pub static foobar: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
                    #class_tokens
                });

                #self_ident . class_signal(&*foobar, #signal)
            };
        }
    });

    quote! {
        {
            #(#gen_classes)*
            #self_ident #(#classes)*
        }
    }
    .into()
}

/// Declares `@keyframes` rules and, optionally, the `animate-*` utility class
/// that drives them.
///
/// dwind utility classes are single declaration blocks, so a `@keyframes` can
/// never be one. This macro is the way to declare animations without dropping
/// to a raw CSS string: it emits the at-rule *and* a compile-time-checked class,
/// and injects the rule lazily the first time either is used.
///
/// # Example
///
/// ```rust,ignore
/// use dwind_macros::{dwclass, dwkeyframes};
///
/// dwkeyframes! {
///     /// Rises and fades in.
///     #[animation("900ms cubic-bezier(0.16, 1, 0.3, 1) both")]
///     fade_up {
///         "from" => "opacity: 0; transform: translateY(14px);",
///         "to"   => "opacity: 1; transform: translateY(0);",
///     }
///
///     aurora {
///         "0%, 100%" => "transform: translate3d(0, 0, 0) scale(1);",
///         "50%"      => "transform: translate3d(6%, -8%, 0) scale(1.15);",
///     }
/// }
///
/// dominator::html!("div", {
///     // the utility, minted by `#[animation(...)]`
///     .dwclass!("animate-fade-up")
///     // or compose the shorthand yourself; `Display` injects the rule
///     .style("animation", &format!("{AURORA_KEYFRAMES} 22s ease-in-out infinite"))
/// });
/// ```
///
/// Every CSS fragment is a string literal. Bare CSS tokens cannot survive Rust's
/// lexer intact — `0%`, `--sx` and `.35` all tokenise in ways that do not
/// round-trip.
///
/// # Options
///
/// Inner attributes configure the whole block:
///
/// - `#![prefix = "app"]` — namespace for the generated CSS names. Defaults to
///   the consuming crate's name, so two crates declaring `fade_up` do not
///   collide.
/// - `#![register_fn = "app_keyframes"]` — also emit a function that injects
///   every rule eagerly, for callers who want the rules present regardless of
///   which classes get instantiated.
/// - `#![path = dwind_base::keyframes]` — where the runtime lives. Defaults to
///   `dwind::prelude::keyframes`.
///
/// Outer attributes configure one entry:
///
/// - `#[name = "spin"]` — pin the exact CSS name, skipping the prefix. Use this
///   when an existing stylesheet already references the name.
/// - `#[animation("1s linear infinite")]` — mint an `animate-<name>` utility
///   with this shorthand. Omit it if you only want the handle.
///
/// # Injection is lazy, but not conditional
///
/// A rule reaches the document the first time anything reads its declaration or
/// its name: the generated class, a modified form of it
/// (`hover:animate-fade-up`, `[&::before]:animate-fade-up`), or the handle's
/// [`Display`](std::fmt::Display). That covers every path `dwclass!` can take,
/// because the generated `*_RAW` value is an `AnimationDecl` whose `Deref`
/// registers rather than a plain `&str`.
///
/// Use `#![register_fn]` if you want the rules present regardless of which
/// classes get instantiated — dwind and dwui both do, to keep their existing
/// eager behaviour.
#[proc_macro]
pub fn dwkeyframes(input: TokenStream) -> TokenStream {
    let input = match syn::parse::<keyframes::DwKeyframesInput>(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    keyframes::codegen::render(input).into()
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
        // create the full name of the new class
        let ident_name = class_name_to_struct_identifier(&format!(
            "{}-{}",
            base_output_ident.value(),
            input_tuple.first.value()
        ));

        // create generator dwind string literal
        let class_literal = format!(
            "{}[{}]",
            dwind_class_lit.value(),
            input_tuple.second.value()
        );

        // parse the generator string into a class selector
        let class = parse_selector(class_literal.as_str()).unwrap().1;

        render_generate_dwind_class(ident_name, class)
    });

    quote! {
        #(#output)*
    }
    .into()
}
