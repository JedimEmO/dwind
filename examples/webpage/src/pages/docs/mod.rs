use dominator::routing::go_to_url;
use std::fmt::{Display, Formatter};

pub mod code_widget;
pub mod doc_main;
pub mod doc_pages;
pub mod doc_sidebar;
pub mod example_box;
pub mod helper_components;

#[derive(Clone)]
pub struct DocSection {
    pub title: String,
    pub docs: Vec<DocPage>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum DocPage {
    Animation,
    Colors,
    Responsiveness,
    Pseudoclasses,
    // Flex
    Flex,
    Justify,
    Align,

    // Borders
    Border,
    Rounding,
    Color,
    Style,

    // Examples
    Examples,
    DwuiExamples,
}

impl DocPage {
    pub fn goto(&self) {
        match self {
            DocPage::Colors => go_to_url("#/docs/colors"),
            DocPage::Flex => go_to_url("#/docs/flex"),
            DocPage::Justify => {}
            DocPage::Align => {}
            DocPage::Border => {}
            DocPage::Rounding => {}
            DocPage::Color => {}
            DocPage::Style => {}
            DocPage::Responsiveness => go_to_url("#/docs/responsive-design"),
            DocPage::Pseudoclasses => go_to_url("#/docs/pseudoclasses"),
            DocPage::Examples => go_to_url("#/examples"),
            DocPage::DwuiExamples => go_to_url("#/dwui-examples"),
            &DocPage::Animation => go_to_url("#/docs/animation"),
        }
    }
}
impl Default for DocPage {
    fn default() -> Self {
        Self::Colors
    }
}

impl Display for DocPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DocPage::Colors => write!(f, "Colors"),
            DocPage::Flex => write!(f, "Flex"),
            DocPage::Justify => write!(f, "Justify"),
            DocPage::Align => write!(f, "Align"),
            DocPage::Border => write!(f, "Border"),
            DocPage::Rounding => write!(f, "Rounding"),
            DocPage::Color => write!(f, "Color"),
            DocPage::Style => write!(f, "Style"),
            DocPage::Responsiveness => write!(f, "Responsiveness"),
            DocPage::Pseudoclasses => write!(f, "Pseudoclasses"),
            DocPage::Examples => write!(f, "Examples"),
            DocPage::DwuiExamples => write!(f, "DWUI Examples"),
            DocPage::Animation => write!(f, "Animation"),
        }
    }
}

pub fn doc_sections() -> Vec<DocSection> {
    vec![
        DocSection {
            title: "General".to_string(),
            docs: vec![
                DocPage::Animation,
                DocPage::Colors,
                DocPage::Responsiveness,
                DocPage::Pseudoclasses,
            ],
        },
        DocSection {
            title: "Flex and Grid".to_string(),
            docs: vec![DocPage::Flex /*, DocPage::Justify, DocPage::Align*/],
        },
        /*DocSection {
            title: "Borders".to_string(),
            docs: vec![
                DocPage::Border,
                DocPage::Rounding,
                DocPage::Color,
                DocPage::Style,
            ],
        },*/
    ]
}
