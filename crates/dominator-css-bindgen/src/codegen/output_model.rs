use crate::css::parse_css::{ParsedCssFile, ParsedSelector};
use cssparser::{CowRcStr, Token};
use std::collections::HashMap;

/// Represent a simple output class and its pseudo classes
pub struct OutputClass<'a> {
    pub identity: CowRcStr<'a>,

    /// main class body
    pub main_class_body: Vec<Token<'a>>,

    pub pseudo_classes: Vec<OutputPseudoClass<'a>>,
}

pub struct OutputPseudoClass<'a> {
    /// pseudo selector, i.e. ':hover:nth-child(1):active'
    pub selector: Vec<Token<'a>>,
    pub body: Vec<Token<'a>>,
}

pub struct OutputStyleSheet<'a> {
    pub selectors: Vec<Vec<Token<'a>>>,
    pub body: Vec<Token<'a>>
}

pub fn parsed_to_output_model<'a>(parsed_files: Vec<ParsedCssFile<'a>>) -> (Vec<OutputClass<'a>>, Vec<OutputStyleSheet<'a>>){
    let mut out_classes: HashMap<CowRcStr<'a>, OutputClass> = HashMap::new();
    let mut out_style_sheets = vec![];

    for file in parsed_files {
        for block in file.blocks {
            let mut style_sheet_selectors = vec![];

            for selector in block.selector {
                match selector {
                    ParsedSelector::SingleClass {
                        class_name,
                        pseudo_classes,
                    } => {
                        let out_class =
                            out_classes
                                .entry(class_name.clone())
                                .or_insert(OutputClass {
                                    identity: class_name,
                                    main_class_body: vec![],
                                    pseudo_classes: vec![],
                                });

                        if pseudo_classes.is_empty() {
                            out_class.main_class_body = block.data.clone();
                        } else {
                            out_class.pseudo_classes.push(OutputPseudoClass {
                                selector: pseudo_classes,
                                body: block.data.clone(),
                            });
                        }
                    }
                    ParsedSelector::Complex(complex) => {
                        style_sheet_selectors.push(complex);
                    }
                }
            }

            if !style_sheet_selectors.is_empty() {
                out_style_sheets.push(OutputStyleSheet {
                    selectors: style_sheet_selectors,
                    body: block.data,
                })
            }
        }
    }

    (
        out_classes.into_iter().map(|v| v.1).collect(),
        out_style_sheets
    )
}
