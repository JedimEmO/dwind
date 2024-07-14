use crate::DCssResult;
use cssparser::{CowRcStr, ParseError, Parser, Token};

pub enum ParsedSelector<'a> {
    /// Single class selectors are a special case, which we can represent
    /// using a **DOMINATOR** class!
    SingleClass {
        class_name: CowRcStr<'a>,
        /// Remainder of a .:pseudo... selector
        pseudo_classes: Vec<Token<'a>>,
    },
    /// Complex selectors can not be represented as a class, but must rather
    /// result in a generated stylesheet, which must be used in downstream applications
    Complex(Vec<Token<'a>>),
}

pub struct ParsedCssBlock<'a> {
    pub selector: Vec<ParsedSelector<'a>>,
    pub data: Vec<Token<'a>>,
}

#[derive(Default)]
pub struct ParsedCssFile<'a> {
    pub blocks: Vec<ParsedCssBlock<'a>>,
}

pub fn take_next_block<'a, 'b, 'aa>(
    parser: &'b mut Parser<'a, 'aa>,
) -> DCssResult<Option<ParsedCssBlock<'a>>> {
    let selector_groups = take_until_block(parser)?;

    if selector_groups.len() == 0 {
        return Ok(None);
    }

    let selectors = get_selectors(selector_groups)
        .into_iter()
        .map(parse_selector)
        .collect::<Vec<_>>();

    let block = parser.parse_nested_block(take_block_flattened)?;

    Ok(Some(ParsedCssBlock {
        selector: selectors,
        data: block,
    }))
}

fn parse_selector(mut input: Vec<Token>) -> ParsedSelector {
    if let Some(class_name) = try_take_class(&mut input) {
        ParsedSelector::SingleClass {
            class_name,
            pseudo_classes: input,
        }
    } else {
        ParsedSelector::Complex(input)
    }
}

fn try_take_class<'a>(input: &mut Vec<Token<'a>>) -> Option<CowRcStr<'a>> {
    if input.len() < 2 {
        return None;
    }

    if input.iter().fold(0, |a, b| {
        a + match b {
            Token::Delim('.') => 1,
            Token::Delim('#') => 1,
            _ => 0,
        }
    }) > 1
    {
        return None;
    }

    let ret = if let [Token::Delim('.'), Token::Ident(ident)] = &input[0..2] {
        Some(ident.clone())
    } else {
        None
    };

    if ret.is_some() {
        input.remove(0);
        input.remove(0);
    }

    ret
}

/// converts the token vec into a vec of selectors from the group of selectors
fn get_selectors(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let out = tokens
        .split(|token| *token == Token::Delim(','))
        .map(|v| v.to_vec())
        .filter(|v| v.len() > 0)
        .collect();
    out
}

/// Consume tokens until the beginning of a declaration block
fn take_until_block<'a, 'b, 'aa>(parser: &'b mut Parser<'a, 'aa>) -> DCssResult<Vec<Token<'a>>> {
    let mut out = vec![];

    while let Some(token) = take(parser)? {
        if token == Token::CurlyBracketBlock {
            break;
        }

        out.push(token.clone());

        match token {
            Token::ParenthesisBlock => {
                let mut paren_block = parser.parse_nested_block(take_block_flattened)?;
                out.append(&mut paren_block);
                out.push(Token::CloseParenthesis);
            }
            Token::Function(_) => {
                let mut paren_block = parser.parse_nested_block(take_block_flattened)?;
                out.append(&mut paren_block);
                out.push(Token::CloseParenthesis);
            }
            _ => {}
        }
    }

    Ok(out)
}

fn take<'a, 'b, 'aa>(parser: &'b mut Parser<'a, 'aa>) -> DCssResult<Option<Token<'a>>> {
    if parser.is_exhausted() {
        return Ok(None);
    }

    Ok(Some(parser.next()?.clone()))
}

pub fn take_block_flattened<'a, 'aa>(
    parser: &mut Parser<'a, 'aa>,
) -> Result<Vec<Token<'a>>, ParseError<'a, ()>> {
    let mut out = vec![];

    while !parser.is_exhausted() {
        let next = parser.next()?;
        out.push(next.clone());

        if let Some(mut block) = match next {
            Token::CurlyBracketBlock => Some((
                parser.parse_nested_block(take_block_flattened)?,
                Token::CloseCurlyBracket,
            )),
            Token::ParenthesisBlock => Some((
                parser.parse_nested_block(take_block_flattened)?,
                Token::CloseParenthesis,
            )),
            Token::Function(_name) => Some((
                parser.parse_nested_block(take_block_flattened)?,
                Token::CloseParenthesis,
            )),
            Token::SquareBracketBlock => Some((
                parser.parse_nested_block(take_block_flattened)?,
                Token::CloseSquareBracket,
            )),
            _ => None,
        } {
            out.append(&mut block.0);
            out.push(block.1);
        }
    }

    Ok(out)
}
