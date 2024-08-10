use crate::codegen::output_model::parsed_to_output_model;
use crate::codegen::render_output_class::{render_output_class, render_output_style_sheets};
use crate::css::parse_css::{take_next_block, ParsedCssFile};
use crate::DCssResult;
use cssparser::{Parser, ParserInput};
use std::path::Path;
use crate::codegen::render_variables::{css_ident_to_rust, render_variable_definitions};

pub fn generate_rust_bindings_from_file(css_file_path: impl AsRef<Path>) -> DCssResult<String> {
    let css_path = css_file_path.as_ref();
    let css_module_name = css_ident_to_rust(css_path.file_stem().unwrap().to_str().unwrap());
    let css_file_content = std::fs::read_to_string(css_path)?;

    let mut input = ParserInput::new(css_file_content.as_str());
    let mut parser = Parser::new(&mut input);

    let parsed = parse_css_file(&mut parser).unwrap();
    let referenced_variables = parsed.referenced_variable_names();

    let (output, output_style_sheets) = parsed_to_output_model(vec![parsed]);

    let mut out_items =
        vec![
            render_output_style_sheets(output_style_sheets, &css_module_name)
                .to_string(),
        ];

    out_items.extend(
        output
            .into_iter()
            .map(|v| render_output_class(v).to_string())
            .collect::<Vec<_>>(),
    );

    out_items.push(render_variable_definitions(&css_module_name, referenced_variables));

    Ok(out_items.join("\n"))
}

pub fn parse_css_file<'a, 'aa>(parser: &mut Parser<'a, 'aa>) -> DCssResult<ParsedCssFile<'a>> {
    let mut out = ParsedCssFile::default();

    while let Some(block) = take_next_block(parser)? {
        out.blocks.push(block);
    }

    Ok(out)
}
