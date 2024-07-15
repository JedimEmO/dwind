use nom::bytes::complete::{tag, take_while1};
use nom::IResult;
use nom::character::{is_alphanumeric};
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, terminated};

#[derive(Eq, PartialEq, Debug, Default)]
pub struct DwindClassSelector {
    pub class_name: String,
    pub pseudo_classes: Vec<String>,
    pub generator_params: Vec<String>,
}

impl DwindClassSelector {
    pub fn is_generator(&self) -> bool {
        self.generator_params.len() > 0
    }
}

pub fn parse_class_string(input: &str) -> Result<Vec<DwindClassSelector>, ()> {
    let classes_parts = input.split(" ");

    let classes: Result<_, _> = classes_parts.map(|class_part| {
        parse_selector(class_part).map(|v| v.1)
    }).collect();

    Ok(classes.unwrap())
}

pub fn parse_selector(input: &str) -> IResult<&str, DwindClassSelector> {
    let (input, pseudo_classes) = many0(pseudo_selector)(input)?;
    let (input, identifier) = css_identifier(input)?;

    let generator_params = if let Ok((_input, generator_params)) = generator_parameters(input) {
        generator_params.into_iter().map(|v| v.to_string()).collect()
    } else {
        vec![]
    };

    Ok((input, DwindClassSelector {
        class_name: identifier.to_string().replace("-", "_"),
        pseudo_classes: pseudo_classes.into_iter().map(|v| v.to_string()).collect(),
        generator_params: generator_params.into_iter().map(|v| v.to_string()).collect(),
    }))
}

fn is_extended_alphanumeric(chars: Vec<char>) -> impl Fn(char) -> bool {
    move |c| {
        is_alphanumeric(c as u8) || chars.contains(&c)
    }
}

fn css_identifier(input: &str) -> IResult<&str, &str> {
    let parser = take_while1(is_extended_alphanumeric(vec!['_', '-']));

    parser(input)
}

fn color(input: &str) -> IResult<&str, &str> {
    let parser = take_while1(is_extended_alphanumeric(vec!['#', '%', '_', '-']));

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

fn pseudo_selector(input: &str) -> IResult<&str, &str> {
    let pseudo_parser = take_while1(is_extended_alphanumeric(vec!['_', '-', '(', ')']));
    let mut parser = terminated(pseudo_parser, tag(":"));

    parser(input)
}

#[cfg(test)]
mod test {
    use crate::grammar::{css_identifier, DwindClassSelector, generator_parameters, parse_class_string, pseudo_selector};

    #[test]
    fn verify_parser() {
        assert_eq!(
            parse_class_string("padding-[5px]").unwrap(),
            vec![
                DwindClassSelector {
                    class_name: "padding_".to_string(),
                    pseudo_classes: vec![],
                    generator_params: vec!["5px".to_string()],
                }
            ]
        );
    }
    #[test]
    fn verify_generator_parser() {
        assert_eq!(generator_parameters("[foobar-test]").unwrap().1, vec!["foobar-test".to_string()]);
        assert_eq!(generator_parameters("[a%,5px,42]").unwrap().1, vec!["a%", "5px", "42"]);
    }

    #[test]
    fn verify_class_list_parser() {
        let classes = parse_class_string("hover:foo bar nth-child(1):baz").unwrap();
        let class_names = classes.into_iter().map(|v| v.class_name).collect::<Vec<_>>();

        assert_eq!(class_names, ["foo", "bar", "baz"])
    }

    #[test]
    fn verify_pseudo_selector() {
        assert_eq!(pseudo_selector("foo:").unwrap().1, "foo".to_string());
        assert_eq!(pseudo_selector("foo(1):").unwrap().1, "foo(1)".to_string());
    }

    #[test]
    fn verify_css_identifier() {
        assert_eq!(css_identifier("foo").unwrap().1, "foo".to_string());
        assert_eq!(css_identifier("foo-bar").unwrap().1, "foo-bar".to_string());
        assert_eq!(css_identifier("foo_baz").unwrap().1, "foo_baz".to_string());
    }
}