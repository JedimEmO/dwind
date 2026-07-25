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

/// Shorthands for the pseudo-*elements*.
///
/// Pseudo-classes need no table — they already pass through verbatim. These are
/// listed because a pseudo-element needs the `::` form, and writing
/// `[&::before]:` for something this common is a lot of punctuation.
///
/// Each entry maps to the selector with a *leading* colon, because the caller
/// joins with `:` and so contributes the other one.
fn pseudo_element_alias(name: &str) -> Option<&'static str> {
    Some(match name {
        "before" => ":before",
        "after" => ":after",
        "placeholder" => ":placeholder",
        "marker" => ":marker",
        "selection" => ":selection",
        "backdrop" => ":backdrop",
        "first-letter" | "first_letter" => ":first-letter",
        "first-line" | "first_line" => ":first-line",
        _ => return None,
    })
}

/// Builds the selector handed to `dominator::pseudo!` from a bracketed variant
/// and any pseudo-class prefixes.
fn build_pseudo_selector(variant: &Option<String>, pseudo_classes: &[String]) -> String {
    let variant = variant.clone().unwrap_or_default();

    if pseudo_classes.is_empty() {
        return variant;
    }

    let pseudo_classes = pseudo_classes
        .iter()
        .map(|name| {
            pseudo_element_alias(name)
                .map(str::to_string)
                .unwrap_or_else(|| name.clone())
        })
        .collect::<Vec<_>>()
        .join(":");

    format!("{variant}:{pseudo_classes}")
}

/// Whether the selector's last compound targets a pseudo-element that has no
/// content of its own, and therefore will not render without one.
///
/// Checks the *last* compound so that `::before:hover` — which is how
/// `[&::before]:hover:…` renders — is still caught.
fn needs_generated_content(selector: &str) -> bool {
    let last_compound = selector
        .rsplit(|c: char| c.is_whitespace() || c == '>' || c == '+' || c == '~')
        .next()
        .unwrap_or(selector);

    last_compound.contains("::before") || last_compound.contains("::after")
}

/// `::before` and `::after` do not render without a `content`. dwind emits an
/// empty one so the utility is enough on its own.
///
/// This lands *before* the class body, and `DomBuilder::raw` appends rather than
/// replaces, so a user's own `content` declaration later in the same class still
/// wins. That is why this needs none of Tailwind's `--tw-content` indirection —
/// Tailwind needs it because its variants are static stylesheet rules with fixed
/// source order.
fn generated_content(selector: &str) -> TokenStream {
    if needs_generated_content(selector) {
        quote! { .raw("content: \"\";") }
    } else {
        quote! {}
    }
}

pub fn render_generate_dwind_class(class_name: String, class: DwindClassSelector) -> TokenStream {
    assert!(
        !class.is_arbitrary(),
        "dwgenerate! cannot name an arbitrary declaration — it has no reusable \
         class to alias. Write the declaration inline with dwclass!, or add a \
         generator macro and use `{class_name}-[value]`."
    );

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

/// Turns `mask-composite:exclude` into `mask-composite: exclude;`.
///
/// Underscores in the *value* become spaces, the way Tailwind handles arbitrary
/// values — a class string is space-separated, so a literal space cannot appear
/// there. The property is left alone, since custom properties such as
/// `--my_var` legitimately contain underscores.
fn normalise_declaration(declaration: &str) -> String {
    let Some((property, value)) = declaration.split_once(':') else {
        panic!(
            "`[{declaration}]` is not a CSS declaration — expected `[property:value]`, \
             for example `[mask-composite:exclude]`. If you meant a variant selector, \
             it needs a class after it: `[{declaration}]:some-class`."
        );
    };

    let property = property.trim();
    let value = value.trim().replace('_', " ");

    if property.is_empty() || value.is_empty() {
        panic!("`[{declaration}]` has an empty property or value");
    }

    format!("{property}: {value};")
}

/// A readable, identifier-safe prefix for the generated class name. Cosmetic —
/// it only ever shows up in devtools.
fn declaration_prefix(declaration: &str) -> String {
    let slug = declaration
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();

    let slug = slug.trim_matches('_').to_string();
    let slug: String = slug.chars().take(40).collect();

    if slug.is_empty() {
        "arbitrary".to_string()
    } else {
        slug
    }
}

pub fn render_dwind_class(
    class: DwindClassSelector,
) -> (TokenStream, Option<BreakpointInfo>, bool) {
    let breakpoint = class.get_breakpoint();

    if let Some(declaration) = &class.arbitrary {
        let css = normalise_declaration(declaration);
        let class_prefix = declaration_prefix(declaration);

        let tokens = if class.pseudo_classes.is_empty() && class.variant.is_none() {
            quote! {
                dominator::class! {
                    # ! [prefix=#class_prefix]
                    .raw(#css)
                }
            }
        } else {
            let pseudo_selector = build_pseudo_selector(&class.variant, &class.pseudo_classes);
            let content = generated_content(&pseudo_selector);

            quote! {
                dominator::class! {
                    # ! [prefix=#class_prefix]
                    .dominator::pseudo!(#pseudo_selector, {
                        #content
                        .raw(#css)
                    })
                }
            }
        };

        return (tokens, breakpoint, true);
    }

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
        let pseudo_selector = build_pseudo_selector(&class.variant, &class.pseudo_classes);
        let content = generated_content(&pseudo_selector);

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
                        #content
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

    // Both a bracketed variant and pseudo-class prefixes have to reach the
    // selector here. Only checking `pseudo_classes` used to drop the variant
    // silently, so `[&::before]:bg-color-[red]` styled the element itself.
    if class.pseudo_classes.is_empty() && class.variant.is_none() {
        let class_prefix = sanitize_class_prefix(&generator_name);

        quote! { dominator::class! {
            # ! [prefix=#class_prefix]
            .raw(#generator_call)
        }}
    } else {
        let pseudo_selector = build_pseudo_selector(&class.variant, &class.pseudo_classes);
        let content = generated_content(&pseudo_selector);
        let class_prefix = sanitize_class_prefix(&generator_classname);

        quote! {
            dominator::class! {
                # ! [prefix=#class_prefix]
                .dominator::pseudo!(#pseudo_selector, {
                    #content
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::grammar::parse_class_string;

    /// Renders the first class of a `dwclass!` string to a token string.
    ///
    /// Assertions below use `contains` rather than whole-token snapshots on
    /// purpose: `TokenStream::to_string()` spacing is not a stable contract and
    /// snapshots would churn on every `quote!` tweak.
    fn render(input: &str) -> String {
        let mut classes = parse_class_string(input).unwrap();

        assert_eq!(classes.len(), 1, "expected exactly one class in {input:?}");

        render_dwind_class(classes.remove(0)).0.to_string()
    }

    #[test]
    fn pseudo_elements_get_generated_content() {
        let rendered = render("[&::before]:opacity-0");

        assert!(rendered.contains("\"::before\""), "{rendered}");
        assert!(rendered.contains("content"), "{rendered}");
    }

    #[test]
    fn pseudo_classes_do_not_get_generated_content() {
        let rendered = render("hover:opacity-0");

        assert!(rendered.contains("\":hover\""), "{rendered}");
        assert!(!rendered.contains("content"), "{rendered}");
    }

    #[test]
    fn content_follows_the_last_compound_not_the_first() {
        // `[&::before]:hover:x` renders as `::before:hover` — the pseudo-element
        // is still the thing being generated, so content is still required.
        let rendered = render("[&::before]:hover:opacity-0");
        assert!(rendered.contains("content"), "{rendered}");

        // A descendant of a pseudo-element cannot exist, but a pseudo-element
        // mentioned in an *earlier* compound must not trigger injection.
        assert!(!needs_generated_content("::before p"));
        assert!(!needs_generated_content("::before > span"));
        assert!(needs_generated_content("::after"));
    }

    #[test]
    fn before_and_after_aliases_expand_to_pseudo_elements() {
        let rendered = render("before:opacity-0");

        assert!(rendered.contains("\"::before\""), "{rendered}");
        assert!(rendered.contains("content"), "{rendered}");

        let rendered = render("after:opacity-0");
        assert!(rendered.contains("\"::after\""), "{rendered}");
    }

    #[test]
    fn placeholder_alias_expands_but_needs_no_content() {
        let rendered = render("placeholder:opacity-0");

        assert!(rendered.contains("\"::placeholder\""), "{rendered}");
        assert!(!rendered.contains("content"), "{rendered}");
    }

    #[test]
    fn generators_keep_their_variant() {
        // Regression: `render_generator` used to consider only `pseudo_classes`,
        // so the bracketed variant was dropped and the style landed on the
        // element itself.
        let rendered = render("[&::before]:padding-[4px]");

        assert!(rendered.contains("pseudo"), "{rendered}");
        assert!(rendered.contains("\"::before\""), "{rendered}");
        assert!(rendered.contains("content"), "{rendered}");
    }

    #[test]
    fn plain_classes_stay_a_bare_reference() {
        let rendered = render("opacity-0");

        assert!(rendered.contains("OPACITY_0"), "{rendered}");
        assert!(!rendered.contains("pseudo"), "{rendered}");
    }

    #[test]
    fn arbitrary_declarations_are_normalised() {
        let rendered = render("[mask-composite:exclude]");

        assert!(
            rendered.contains("\"mask-composite: exclude;\""),
            "{rendered}"
        );
        assert!(!rendered.contains("pseudo"), "{rendered}");
    }

    #[test]
    fn arbitrary_declaration_underscores_become_spaces_in_the_value_only() {
        // A class string is space-separated, so a literal space cannot appear
        // in one; `_` is the Tailwind-compatible stand-in.
        let rendered = render("[transition:opacity_650ms_ease]");
        assert!(
            rendered.contains("\"transition: opacity 650ms ease;\""),
            "{rendered}"
        );

        // Custom properties legitimately contain underscores, so the property
        // side is left alone.
        let rendered = render("[--my_var:red]");
        assert!(rendered.contains("\"--my_var: red;\""), "{rendered}");
    }

    #[test]
    fn arbitrary_declarations_honour_modifiers() {
        let rendered = render("hover:[color:red]");
        assert!(rendered.contains("\":hover\""), "{rendered}");
        assert!(rendered.contains("\"color: red;\""), "{rendered}");

        let rendered = render("[&::after]:[mask-composite:exclude]");
        assert!(rendered.contains("\"::after\""), "{rendered}");
        assert!(rendered.contains("content"), "{rendered}");
    }

    #[test]
    #[should_panic(expected = "is not a CSS declaration")]
    fn a_bracket_group_with_no_colon_is_a_clear_error() {
        // Previously this shape was silently dropped along with everything
        // after it. Now it says what is wrong.
        render("[nonsense]");
    }

    #[test]
    fn child_variants_are_unchanged() {
        let rendered = render("[& > *]:opacity-0");

        assert!(rendered.contains("\" > *\""), "{rendered}");
        assert!(!rendered.contains("content"), "{rendered}");
    }
}
