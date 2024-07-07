use std::io::{BufWriter, Write};
use std::path::Path;
use cssparser::{ParseError, Parser, ParserInput, ToCss, Token};
use nom::AsBytes;
use crate::{DCssError, DCssResult};

pub fn generate_rust_bindings_from_file(css_file_path: impl AsRef<Path>) -> DCssResult<String> {
    let css_file_content = std::fs::read_to_string(css_file_path)?;

    let mut input = ParserInput::new(css_file_content.as_str());
    let mut parser = Parser::new(&mut input);

    let parsed = parse_css_file(&mut parser).unwrap();

    let mut buf = BufWriter::new(vec![]);

    parsed.write_rust_code(&mut buf)?;
    let output = String::from_utf8(buf.into_inner().unwrap()).unwrap();
    Ok(output)
}

pub enum ParsedCssBlockData<'a> {
    SingleClass(Vec<Token<'a>>),
    Other(Vec<Token<'a>>),
}

pub struct ParsedCssBlock<'a> {
    pub selector: Vec<Token<'a>>,
    pub data: ParsedCssBlockData<'a>,
}

impl<'a> ParsedCssBlock<'a> {
    pub fn write_rust_code<W: Write>(&self, writer: &mut W) -> DCssResult<()> {
        match &self.data {
            ParsedCssBlockData::SingleClass(data) => {
                let Token::Ident(ident) = &self.selector[1] else {
                    panic!()
                };

                writer.write(format!("pub static {}: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {{\n",ident.to_uppercase()).as_bytes())?;
                writer.write("dominator::class! {\n".as_bytes())?;
                writer.write("\t.raw(r#\"\n".as_bytes())?;

                for token in data {
                    writer.write(token.to_css_string().as_bytes())?;
                }

                writer.write("\n\"#)}});\n".as_bytes())?;
            }
            ParsedCssBlockData::Other(_) => {}
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct ParsedCssFile<'a> {
    pub blocks: Vec<ParsedCssBlock<'a>>,
}

impl<'a> ParsedCssFile<'a> {
    pub fn write_rust_code<W: Write>(&self, writer: &mut W) -> DCssResult<()> {
        for block in self.blocks.iter() {
            block.write_rust_code(writer)?;
        }

        Ok(())
    }
}

pub fn parse_css_file<'a, 'aa>(parser: &mut Parser<'a, 'aa>) -> DCssResult<ParsedCssFile<'a>> {
    let mut out = ParsedCssFile::default();
    let mut collected = vec![];

    while !parser.is_exhausted() {
        let next = parser.next()?;
        collected.push(next.clone());

        if is_single_class_rule(&collected) {
            let block = parser.parse_nested_block(parse_block_flattened)?;

            out.blocks.push(ParsedCssBlock { selector: collected.clone(), data: ParsedCssBlockData::SingleClass(block) });
            collected.clear();
        } else {
            if let Token::CurlyBracketBlock = next {
                let block = parse_block_flattened(parser)?;

                out.blocks.push(ParsedCssBlock { selector: collected.clone(), data: ParsedCssBlockData::Other(block) });
                collected.clear();
            }
        }
    }

    Ok(out)
}

fn parse_block_flattened<'a, 'aa>(parser: &mut Parser<'a, 'aa>) -> Result<Vec<Token<'a>>, ParseError<'a, ()>> {
    let mut out = vec![];

    while !parser.is_exhausted() {
        let next = parser.next()?;
        out.push(next.clone());

        if let Some(mut block) = match next {
            Token::CurlyBracketBlock => Some((parser.parse_nested_block(parse_block_flattened)?, Token::CloseCurlyBracket)),
            Token::ParenthesisBlock => Some((parser.parse_nested_block(parse_block_flattened)?, Token::CloseParenthesis)),
            Token::SquareBracketBlock => Some((parser.parse_nested_block(parse_block_flattened)?, Token::CloseSquareBracket)),
            _ => None
        } {
            out.append(&mut block.0);
            out.push(block.1);
        }
    }

    Ok(out)
}

pub fn is_single_class_rule(tokens: &Vec<Token>) -> bool {
    if tokens.len() != 3 {
        return false;
    }

    match &tokens[0] {
        Token::Delim('.') => {
            match &tokens[1] {
                Token::Ident(_ident) => {
                    match &tokens[2] {
                        Token::CurlyBracketBlock => true,
                        _ => false
                    }
                }
                _ => false
            }
        }
        _ => false
    }
}
