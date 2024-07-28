use dominator::routing::go_to_url;
use std::fmt::{Display, Formatter};

pub mod code_widget;
pub mod doc_main;
pub mod doc_pages;
pub mod doc_sidebar;
pub mod example_box;

#[derive(Clone)]
pub struct DocSection {
    pub title: String,
    pub docs: Vec<DocPage>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum DocPage {
    Colors,
    ResponsiveDesign,
    // Flex
    Flex,
    Justify,
    Align,

    // Borders
    Border,
    Rounding,
    Color,
    Style,
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
            DocPage::ResponsiveDesign => go_to_url("#/docs/responsive-design"),
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
        write!(f, "{self:?}")
    }
}

pub fn doc_sections() -> Vec<DocSection> {
    vec![
        DocSection {
            title: "General".to_string(),
            docs: vec![DocPage::Colors, DocPage::ResponsiveDesign],
        },
        DocSection {
            title: "Flex and Grid".to_string(),
            docs: vec![DocPage::Flex, DocPage::Justify, DocPage::Align],
        },
        DocSection {
            title: "Borders".to_string(),
            docs: vec![
                DocPage::Border,
                DocPage::Rounding,
                DocPage::Color,
                DocPage::Style,
            ],
        },
    ]
}
