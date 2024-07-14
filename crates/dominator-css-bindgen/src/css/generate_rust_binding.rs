use std::path::Path;
use cssparser::{Parser, ParserInput};
use crate::codegen::output_model::parsed_to_output_model;
use crate::codegen::render_output_class::render_output_class;
use crate::DCssResult;
use crate::css::parse_css::{ParsedCssFile, take_next_block};

pub fn generate_rust_bindings_from_file(css_file_path: impl AsRef<Path>) -> DCssResult<String> {
    let css_file_content = std::fs::read_to_string(css_file_path)?;

    let mut input = ParserInput::new(css_file_content.as_str());
    let mut parser = Parser::new(&mut input);

    let parsed = parse_css_file(&mut parser).unwrap();
    let output = parsed_to_output_model(vec![parsed]);

    Ok(output.into_iter().map(|v| render_output_class(v).to_string()).collect::<Vec<_>>().join("\n"))
}

pub fn parse_css_file<'a, 'aa>(parser: &mut Parser<'a, 'aa>) -> DCssResult<ParsedCssFile<'a>> {
    let mut out = ParsedCssFile::default();

    while let Some(block) = take_next_block(parser)? {
        out.blocks.push(block);
    }

    Ok(out)
}
