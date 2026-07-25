use dwind_base::media_queries::Breakpoint;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::is_alphanumeric;
use nom::combinator::opt;
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, terminated};
use nom::IResult;

use crate::codegen::BreakpointInfo;

#[derive(Eq, PartialEq, Debug, Default)]
pub struct DwindClassSelector {
    pub class_name: String,
    pub pseudo_classes: Vec<String>,
    pub conditionals: Vec<String>,
    pub generator_params: Vec<String>,
    /// Variants are the first pseudo selector, bracketed with []
    /// [& > *]:nth-child(2):bg-red-500
    pub variant: Option<String>,
    /// An arbitrary CSS declaration, written in place of a class name:
    /// `[mask-composite:exclude]`, `[--sx:50%]`.
    ///
    /// The escape hatch for properties with no utility. Unambiguous against
    /// the variant syntax because a variant's `]` is always followed by `:`.
    pub arbitrary: Option<String>,
}

impl DwindClassSelector {
    pub fn is_generator(&self) -> bool {
        !self.generator_params.is_empty()
    }

    pub fn is_arbitrary(&self) -> bool {
        self.arbitrary.is_some()
    }

    pub fn get_breakpoint(&self) -> Option<BreakpointInfo> {
        let breakpoints = self
            .conditionals
            .iter()
            .filter_map(|v| {
                let (v, modifier, is_media_query) = if v.starts_with("@<") {
                    let mut v = v.clone();
                    v.remove(1);
                    (v, Some("<".to_string()), false)
                } else {
                    let is_mq = v.trim().starts_with("@(") && v.trim().ends_with(")");

                    (v.clone(), None, is_mq)
                };

                if is_media_query {
                    Some(BreakpointInfo {
                        breakpoint: Breakpoint::MediaQuery(v[2..v.len() - 1].to_string()),
                        modifier,
                        is_media_query: true,
                    })
                } else {
                    if let Ok(bp) = Breakpoint::try_from(v.as_str()) {
                        Some(BreakpointInfo {
                            breakpoint: bp,
                            modifier,
                            is_media_query: false,
                        })
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        if breakpoints.len() > 1 {
            panic!("only one breakpoint allowed");
        }

        breakpoints.first().cloned()
    }
}

pub fn parse_class_string(input: &str) -> Result<Vec<DwindClassSelector>, ()> {
    let (rest, classes) = selectors(input).unwrap();

    // `many0` stops at the first thing it cannot parse and reports success with
    // the remainder untouched. Ignoring that remainder meant a single malformed
    // class silently discarded itself *and every class after it* — the exact
    // failure mode dwclass! exists to prevent. Refuse instead.
    if !rest.trim().is_empty() {
        panic!(
            "dwclass!: could not parse {rest:?} in {input:?}.\n\
             Classes are separated by whitespace. An arbitrary declaration needs \
             a property and a value in square brackets, like `[mask-composite:exclude]`."
        );
    }

    Ok(classes
        .into_iter()
        .map(|(variant, prefixes, body, generator_params)| {
            let pseudo_classes: Vec<String> = prefixes
                .clone()
                .into_iter()
                .filter(|v| !v.contains('@'))
                .map(|v| v.to_string())
                .collect();
            let conditionals = prefixes
                .clone()
                .into_iter()
                .filter(|v| v.contains('@'))
                .map(|v| v.to_string())
                .collect();
            let generator_params = generator_params
                .or(Some(vec![]))
                .unwrap()
                .into_iter()
                .map(|v| v.to_string())
                .collect();

            let (class_name, arbitrary) = match body {
                ClassBody::Name(name) => (name.to_string().replace('-', "_"), None),
                ClassBody::Arbitrary(decl) => (String::new(), Some(decl)),
            };

            DwindClassSelector {
                class_name,
                pseudo_classes,
                conditionals,
                generator_params,
                variant,
                arbitrary,
            }
        })
        .collect())
}

/// Any run of whitespace between classes. Not `tag(" ")`, so that a class string
/// broken over several source lines parses the same as a single-spaced one.
fn whitespace(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while(|c: char| c.is_whitespace())(input)
}

/// What sits in the class-name position: either a utility name, or an arbitrary
/// declaration written inline.
#[derive(Debug)]
pub enum ClassBody<'a> {
    Name(&'a str),
    Arbitrary(String),
}

/// Tried in the class-name position, *after* `variant_selector` and any
/// pseudo-class prefixes have been consumed. That ordering is what makes the
/// two bracket syntaxes unambiguous: a variant's `]` is always followed by `:`,
/// so anything still bracketed at this point is a declaration.
fn class_body(input: &str) -> IResult<&str, ClassBody<'_>> {
    alt((
        |v| arbitrary_declaration(v).map(|(rest, decl)| (rest, ClassBody::Arbitrary(decl))),
        |v| css_identifier(v).map(|(rest, name)| (rest, ClassBody::Name(name))),
    ))(input)
}

/// One parsed selector: `(variant, prefixes, class body, generator params)`.
type ParsedSelector<'a> = (
    Option<String>,
    Vec<String>,
    ClassBody<'a>,
    Option<Vec<&'a str>>,
);

fn selectors(input: &str) -> IResult<&str, Vec<ParsedSelector<'_>>> {
    let prefixes = many0(pseudo_selector);
    let parser = nom::sequence::delimited(
        whitespace,
        nom::sequence::tuple((
            variant_selector,
            prefixes,
            class_body,
            opt(generator_parameters),
        )),
        whitespace,
    );
    many0(parser)(input)
}

pub fn parse_selector(input: &str) -> IResult<&str, DwindClassSelector> {
    let (input, variant) = variant_selector(input)?;
    let (input, prefixes) = many0(pseudo_selector)(input)?;
    let (input, body) = class_body(input)?;

    let generator_params = if let Ok((_input, generator_params)) = generator_parameters(input) {
        generator_params
            .into_iter()
            .map(|v| v.to_string())
            .collect()
    } else {
        vec![]
    };

    let pseudo_classes: Vec<String> = prefixes
        .clone()
        .into_iter()
        .filter(|v| !v.contains('@'))
        .map(|v| v.to_string())
        .collect();

    let conditionals = prefixes
        .clone()
        .into_iter()
        .filter(|v| v.contains('@'))
        .map(|v| v.to_string())
        .collect();

    let (class_name, arbitrary) = match body {
        ClassBody::Name(name) => (name.to_string().replace('-', "_"), None),
        ClassBody::Arbitrary(decl) => (String::new(), Some(decl)),
    };

    Ok((
        input,
        DwindClassSelector {
            class_name,
            pseudo_classes,
            conditionals,
            generator_params: generator_params
                .into_iter()
                .map(|v| v.to_string())
                .collect(),
            variant,
            arbitrary,
        },
    ))
}

fn is_extended_alphanumeric(chars: Vec<char>) -> impl Fn(char) -> bool {
    move |c| is_alphanumeric(c as u8) || chars.contains(&c)
}

fn css_identifier(input: &str) -> IResult<&str, &str> {
    let parser = take_while1(is_extended_alphanumeric(vec!['_', '-', '.', '/']));

    parser(input)
}

fn color(input: &str) -> IResult<&str, &str> {
    let parser = take_while1(is_extended_alphanumeric(vec![
        '#', '%', '_', '-', '@', '(', ')', '.', '/',
    ]));

    parser(input)
}

fn generator_parameter_value(input: &str) -> IResult<&str, &str> {
    color(input)
}

fn generator_parameters(input: &str) -> IResult<&str, Vec<&str>> {
    let inner_parser = separated_list1(tag(","), generator_parameter_value);
    let mut parser = delimited(tag("["), inner_parser, tag("]"));

    parser(input)
}

const CHARS_EXT: [char; 13] = [
    '_', '-', '@', ',', '<', '>', '*', ' ', '.', ' ', ':', '#', '&',
];

/// Any character that is not structural to the bracket grammar.
///
/// A CSS value can contain essentially anything — `→` in a `content`, a `°` in a
/// gradient angle, a `字` in a font stack. So this is a deny-list of the four
/// delimiters the parser needs to track, not an allow-list of what CSS is
/// permitted. An allow-list here also silently truncated at any non-ASCII byte,
/// because `nom`'s `is_alphanumeric` takes a `u8`.
fn is_declaration_char(c: char) -> bool {
    !matches!(c, '[' | ']' | '(' | ')')
}

fn declaration_body<'a>(input: &'a str) -> IResult<&'a str, String> {
    many0(alt((
        bracketed("(", ")", declaration_body),
        |v: &'a str| take_while1(is_declaration_char)(v).map(move |v| (v.0, v.1.to_string())),
    )))(input)
    .map(|r| (r.0, r.1.join("")))
}

/// `[mask-composite:exclude]`, `[--sx:50%]`,
/// `[grid-template-columns:repeat(2,minmax(0,1fr))]`.
fn arbitrary_declaration(input: &str) -> IResult<&str, String> {
    delimited(tag("["), declaration_body, tag("]"))(input)
}

fn bracketed<'a>(
    bracket: &'a str,
    bracket_end: &'a str,
    body_parser: impl Fn(&str) -> IResult<&str, String> + Clone + 'a,
) -> impl Fn(&str) -> IResult<&str, String> + 'a {
    move |v| {
        delimited(tag(bracket), body_parser.clone(), tag(bracket_end))(v)
            .map(|v| (v.0, format!("{}{}{}", bracket, v.1, bracket_end)))
    }
}

fn recursive_selector<'a>(input: &'a str) -> IResult<&'a str, String> {
    many0(alt((
        bracketed("(", ")", recursive_selector),
        bracketed("[", "]", recursive_selector),
        |v: &'a str| {
            take_while1(is_extended_alphanumeric(CHARS_EXT.to_vec()))(v)
                .map(move |v| (v.0, v.1.to_string()))
        },
    )))(input)
    .map(|r| (r.0, r.1.join("")))
}

fn variant_selector(input: &str) -> IResult<&str, Option<String>> {
    opt(terminated(
        bracketed("[", "]", recursive_selector),
        tag(":"),
    ))(input)
    .map(|r| {
        (
            r.0,
            r.1.map(|variant| {
                let variant = variant[1..variant.len() - 1].to_string().to_string();

                if variant.starts_with("&") {
                    variant[1..].to_string().to_string()
                } else {
                    variant.to_string()
                }
            }),
        )
    })
}

fn pseudo_selector(input: &str) -> IResult<&str, String> {
    let chars = ['_', '-', '@', ',', '<', '>', '*', '.'];

    let name_parser = many0(take_while1(is_extended_alphanumeric(chars.to_vec())));

    let bracketed_parser = alt((
        bracketed("(", ")", recursive_selector),
        bracketed("[", "]", recursive_selector),
    ));

    let mut parser = terminated(
        nom::sequence::tuple((name_parser, many0(bracketed_parser))),
        tag(":"),
    );

    parser(input).map(|r| (r.0, r.1 .0.join("").to_string() + &r.1 .1.join("")))
}

#[cfg(test)]
mod test {
    use crate::grammar::{
        css_identifier, generator_parameters, parse_class_string, pseudo_selector, selectors,
        DwindClassSelector,
    };

    #[test]
    fn media_query_parser() {
        let v =
            selectors("foo @((max-width: 500px) and (max-height: 500px)):bar @is[white]:bg-[5px] ")
                .unwrap();
        assert_eq!(v.1.len(), 3);
        assert_eq!(v.1[1].1[0], "@((max-width: 500px) and (max-height: 500px))")
    }

    #[test]
    fn verify_selectors_parser() {
        let v = selectors("foo @sm:bar @is[white]:bg-[5px] ").unwrap();
        assert_eq!(v.1.len(), 3);
        let v = selectors("@sm:@is[dark]:@is[selected]:foo @<sm:bar").unwrap();
        assert_eq!(v.1.len(), 2);
    }

    #[test]
    fn verify_conditionals_parser() {
        assert_eq!(
            parse_class_string("@sm:@is[dark]:padding-5").unwrap(),
            vec![DwindClassSelector {
                class_name: "padding_5".to_string(),
                pseudo_classes: vec![],
                conditionals: vec!["@sm".to_string(), "@is[dark]".to_string()],
                generator_params: vec![],
                variant: None,
                ..Default::default()
            }]
        );
    }
    #[test]
    fn verify_parser() {
        assert_eq!(
            parse_class_string("padding-[5px]").unwrap(),
            vec![DwindClassSelector {
                class_name: "padding_".to_string(),
                pseudo_classes: vec![],
                conditionals: vec![],
                generator_params: vec!["5px".to_string()],
                variant: None,
                ..Default::default()
            }]
        );

        assert_eq!(
            parse_class_string("foo/0.25").unwrap(),
            vec![DwindClassSelector {
                class_name: "foo/0.25".to_string(),
                pseudo_classes: vec![],
                conditionals: vec![],
                generator_params: vec![],
                variant: None,
                ..Default::default()
            }]
        );

        assert_eq!(
            parse_class_string("foo[1/2]").unwrap(),
            vec![DwindClassSelector {
                class_name: "foo".to_string(),
                pseudo_classes: vec![],
                conditionals: vec![],
                generator_params: vec!["1/2".to_string()],
                variant: None,
                ..Default::default()
            }]
        );
    }

    #[test]
    fn verify_generator_parser() {
        assert_eq!(
            generator_parameters("[foobar-test]").unwrap().1,
            vec!["foobar-test".to_string()]
        );
        assert_eq!(
            generator_parameters("[a%,5px,42]").unwrap().1,
            vec!["a%", "5px", "42"]
        );
    }

    #[test]
    fn verify_class_list_parser() {
        let classes = parse_class_string("hover:foo bar nth-child(1):baz").unwrap();
        let class_names = classes
            .into_iter()
            .map(|v| v.class_name)
            .collect::<Vec<_>>();

        assert_eq!(class_names, ["foo", "bar", "baz"])
    }

    #[test]
    fn verify_pseudo_selector() {
        assert_eq!(pseudo_selector("foo:").unwrap().1, "foo".to_string());
        assert_eq!(pseudo_selector("foo(1):").unwrap().1, "foo(1)".to_string());
        assert_eq!(
            pseudo_selector("is(:not(:nth-child(1)) *):").unwrap().1,
            "is(:not(:nth-child(1)) *)".to_string()
        );
    }

    #[test]
    fn verify_css_identifier() {
        assert_eq!(css_identifier("foo").unwrap().1, "foo".to_string());
        assert_eq!(css_identifier("foo-bar").unwrap().1, "foo-bar".to_string());
        assert_eq!(css_identifier("foo_baz").unwrap().1, "foo_baz".to_string());
        assert_eq!(css_identifier("foo/0.2").unwrap().1, "foo/0.2".to_string());
    }

    #[test]
    fn verify_child_selector_parser() {
        let parsed = parse_class_string("a [& > *]:is(p):b c").unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[1].variant, Some(" > *".to_string()));

        let parsed = parse_class_string("[& > *:is(span):hover]:is(p):b").unwrap();
        assert_eq!(parsed[0].variant, Some(" > *:is(span):hover".to_string()));
    }

    // -----------------------------------------------------------------------
    // Regression locks.
    //
    // These pin the shapes that appear in the docs and in the example app, so
    // that changes to the bracket handling cannot quietly alter what an
    // existing `dwclass!` string means.
    // -----------------------------------------------------------------------

    #[test]
    fn pins_documented_variant_forms() {
        // From the Pseudoclasses docs page.
        let parsed = parse_class_string("[& > *]:nth-child(2):bg-candlelight-500").unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].variant, Some(" > *".to_string()));
        assert_eq!(parsed[0].pseudo_classes, vec!["nth-child(2)".to_string()]);
        assert_eq!(parsed[0].class_name, "bg_candlelight_500");

        // A variant with no leading `&`.
        let parsed = parse_class_string("[> span]:text-apple-300").unwrap();
        assert_eq!(parsed[0].variant, Some("> span".to_string()));
        assert_eq!(parsed[0].class_name, "text_apple_300");
    }

    #[test]
    fn pins_pseudo_element_variants() {
        // Already supported today: `CHARS_EXT` includes `:`, so a `::`-prefixed
        // variant parses and reaches `dominator::pseudo!` unchanged.
        let parsed = parse_class_string("[&::before]:opacity-0").unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].variant, Some("::before".to_string()));
        assert_eq!(parsed[0].class_name, "opacity_0");

        // Parent-state driven child selector, as used by the scroll reveal.
        let parsed = parse_class_string("[&.reveal-in > *]:opacity-100").unwrap();
        assert_eq!(parsed[0].variant, Some(".reveal-in > *".to_string()));
    }

    #[test]
    fn pins_breakpoint_forms() {
        let parsed = parse_class_string("@sm:flex-row").unwrap();
        assert_eq!(parsed[0].conditionals, vec!["@sm".to_string()]);

        let parsed = parse_class_string("@<sm:flex-col").unwrap();
        assert_eq!(parsed[0].conditionals, vec!["@<sm".to_string()]);

        // Arbitrary media queries, including motion preferences.
        let parsed = parse_class_string("@((max-width: 700px)):hidden").unwrap();
        assert_eq!(
            parsed[0].conditionals,
            vec!["@((max-width: 700px))".to_string()]
        );

        let parsed =
            parse_class_string("@((prefers-reduced-motion: reduce)):animate-none").unwrap();
        assert_eq!(
            parsed[0].conditionals,
            vec!["@((prefers-reduced-motion: reduce))".to_string()]
        );
        assert_eq!(parsed[0].class_name, "animate_none");
    }

    // -----------------------------------------------------------------------
    // Arbitrary declarations
    // -----------------------------------------------------------------------

    #[test]
    fn arbitrary_declaration_is_distinct_from_a_variant() {
        // A variant's `]` is always followed by `:`. Without one, the bracket
        // group is a declaration. This is the whole disambiguation rule.
        let parsed = parse_class_string("[mask-composite:exclude]").unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(
            parsed[0].arbitrary,
            Some("mask-composite:exclude".to_string())
        );
        assert_eq!(parsed[0].class_name, "");
        assert_eq!(parsed[0].variant, None);
        assert!(parsed[0].is_arbitrary());

        // ...and the variant reading is untouched.
        let parsed = parse_class_string("[& > *]:opacity-0").unwrap();
        assert_eq!(parsed[0].variant, Some(" > *".to_string()));
        assert_eq!(parsed[0].arbitrary, None);
        assert_eq!(parsed[0].class_name, "opacity_0");
    }

    #[test]
    fn arbitrary_declarations_cover_the_awkward_characters() {
        // Custom properties, percentages, and a leading double dash.
        let parsed = parse_class_string("[--sx:50%]").unwrap();
        assert_eq!(parsed[0].arbitrary, Some("--sx:50%".to_string()));

        // Nested parens and commas.
        let parsed = parse_class_string("[grid-template-columns:repeat(2,minmax(0,1fr))]").unwrap();
        assert_eq!(
            parsed[0].arbitrary,
            Some("grid-template-columns:repeat(2,minmax(0,1fr))".to_string())
        );

        // Quotes.
        let parsed = parse_class_string("[content:\"x\"]").unwrap();
        assert_eq!(parsed[0].arbitrary, Some("content:\"x\"".to_string()));
    }

    #[test]
    fn arbitrary_declarations_compose_with_modifiers() {
        let parsed = parse_class_string("hover:[color:red]").unwrap();
        assert_eq!(parsed[0].pseudo_classes, vec!["hover".to_string()]);
        assert_eq!(parsed[0].arbitrary, Some("color:red".to_string()));

        let parsed = parse_class_string("[&::before]:[mask-composite:exclude]").unwrap();
        assert_eq!(parsed[0].variant, Some("::before".to_string()));
        assert_eq!(
            parsed[0].arbitrary,
            Some("mask-composite:exclude".to_string())
        );

        let parsed = parse_class_string("@sm:[color:red]").unwrap();
        assert_eq!(parsed[0].conditionals, vec!["@sm".to_string()]);
        assert_eq!(parsed[0].arbitrary, Some("color:red".to_string()));
    }

    #[test]
    fn arbitrary_declarations_no_longer_truncate_the_class_list() {
        // Before the escape hatch existed this yielded ONE class: the bracket
        // group failed every parser, `many0` stopped, and `bar` was discarded
        // along with it. Silent truncation, no diagnostic.
        let parsed = parse_class_string("foo [mask-composite:exclude] bar").unwrap();

        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].class_name, "foo");
        assert_eq!(
            parsed[1].arbitrary,
            Some("mask-composite:exclude".to_string())
        );
        assert_eq!(parsed[2].class_name, "bar");
    }

    #[test]
    fn unparseable_input_is_rejected_rather_than_dropped() {
        // `many0` succeeds with an untouched remainder, so anything the grammar
        // cannot handle used to discard itself *and every class after it*.
        let err = std::panic::catch_unwind(|| parse_class_string("foo ((bad)) bar"));
        assert!(err.is_err(), "malformed input should not parse silently");
    }

    #[test]
    fn classes_may_be_separated_by_any_whitespace() {
        // Multi-line `dwclass!` strings and double spaces used to hit the silent
        // truncation path above.
        let parsed = parse_class_string("foo  bar\n  baz\tqux").unwrap();
        let names = parsed.into_iter().map(|v| v.class_name).collect::<Vec<_>>();

        assert_eq!(names, ["foo", "bar", "baz", "qux"]);
    }

    #[test]
    fn arbitrary_declarations_accept_any_css_value() {
        // Non-ASCII: the old allow-list truncated at the first multi-byte char,
        // because `nom`'s `is_alphanumeric` takes a `u8`.
        let parsed = parse_class_string("before:[content:'→'] before:m-r-2").unwrap();
        assert_eq!(parsed.len(), 2, "{parsed:?}");
        assert_eq!(parsed[0].arbitrary, Some("content:'→'".to_string()));
        assert_eq!(parsed[1].class_name, "m_r_2");

        // Underscores are not touched — rewriting them would corrupt this.
        let parsed = parse_class_string("[color:var(--brand_color)]").unwrap();
        assert_eq!(
            parsed[0].arbitrary,
            Some("color:var(--brand_color)".to_string())
        );

        // Real spaces work inside the brackets.
        let parsed = parse_class_string("[transition:opacity 650ms ease] flex").unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(
            parsed[0].arbitrary,
            Some("transition:opacity 650ms ease".to_string())
        );
        assert_eq!(parsed[1].class_name, "flex");
    }

    #[test]
    fn pins_generator_forms() {
        let parsed = parse_class_string("padding-[20px]").unwrap();
        assert_eq!(parsed[0].class_name, "padding_");
        assert_eq!(parsed[0].generator_params, vec!["20px".to_string()]);
        assert!(parsed[0].is_generator());
    }
}
