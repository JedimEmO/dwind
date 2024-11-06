use std::env::var;
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
    pub variant: Option<String>
}

impl DwindClassSelector {
    pub fn is_generator(&self) -> bool {
        !self.generator_params.is_empty()
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
    let (_, classes) = selectors(input).unwrap();

    Ok(classes
        .into_iter()
        .map(|(variant, prefixes, class_name, generator_params)| {
            let mut pseudo_classes: Vec<String> = prefixes
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

            DwindClassSelector {
                class_name: class_name.to_string().replace('-', "_"),
                pseudo_classes,
                conditionals,
                generator_params,
                variant,
            }
        })
        .collect())
}

fn selectors(input: &str) -> IResult<&str, Vec<(Option<String>, Vec<String>, &str, Option<Vec<&str>>)>> {
    let prefixes = many0(pseudo_selector);
    let parser = terminated(
        nom::sequence::tuple((variant_selector, prefixes, css_identifier, opt(generator_parameters))),
        opt(tag(" ")),
    );
    many0(parser)(input)
}

pub fn parse_selector(input: &str) -> IResult<&str, DwindClassSelector> {
    let (input, variant) = variant_selector(input)?;
    let (input, mut prefixes) = many0(pseudo_selector)(input)?;
    let (input, identifier) = css_identifier(input)?;

    let generator_params = if let Ok((_input, generator_params)) = generator_parameters(input) {
        generator_params
            .into_iter()
            .map(|v| v.to_string())
            .collect()
    } else {
        vec![]
    };

    let mut pseudo_classes: Vec<String> = prefixes
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

    Ok((
        input,
        DwindClassSelector {
            class_name: identifier.to_string().replace('-', "_"),
            pseudo_classes,
            conditionals,
            generator_params: generator_params
                .into_iter()
                .map(|v| v.to_string())
                .collect(),
            variant,
        },
    ))
}

fn is_extended_alphanumeric(chars: Vec<char>) -> impl Fn(char) -> bool {
    move |c| is_alphanumeric(c as u8) || chars.contains(&c)
}

fn css_identifier(input: &str) -> IResult<&str, &str> {
    let parser = take_while1(is_extended_alphanumeric(vec!['_', '-']));

    parser(input)
}

fn color(input: &str) -> IResult<&str, &str> {
    let parser = take_while1(is_extended_alphanumeric(vec![
        '#', '%', '_', '-', '@', '(', ')',
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

const CHARS_EXT: [char; 13] = ['_', '-', '@', ',', '<', '>', '*', ' ', '.', ' ', ':', '#', '&' ];

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
        tag(":")
    ))(input).map(|r| {
        (r.0, r.1.map(|variant| {
            let variant = variant[1..variant.len() -1].to_string().trim().to_string();

            if variant.starts_with("&") {
                variant[1..].to_string().trim().to_string()
            } else {
                variant.to_string()
            }
        }))
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
    }

    #[test]
    fn verify_child_selector_parser() {
        let parsed = parse_class_string("a [& > *]:is(p):b c").unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[1].variant, Some("> *".to_string()));

        let parsed = parse_class_string("[& > *:is(span):hover]:is(p):b").unwrap();
        assert_eq!(parsed[0].variant, Some("> *:is(span):hover".to_string()));
    }
}
