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

pub fn parsed_to_output_model<'a>(parsed_files: Vec<ParsedCssFile<'a>>) -> Vec<OutputClass<'a>> {
    let mut out_classes: HashMap<CowRcStr<'a>, OutputClass> = HashMap::new();

    for file in parsed_files {
        for block in file.blocks {
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
                    ParsedSelector::Complex(_) => {}
                }
            }
        }
    }

    out_classes.into_iter().map(|v| v.1).collect()
}
