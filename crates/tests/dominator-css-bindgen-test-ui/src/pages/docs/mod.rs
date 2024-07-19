use std::fmt::{Display, Formatter};

pub mod doc_main;
pub mod doc_sidebar;
pub mod doc_pages;

#[derive(Clone)]
pub struct DocSection {
    pub title: String,
    pub docs: Vec<DocPage>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum DocPage {
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

impl Display for DocPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn doc_sections() -> Vec<DocSection> {
    vec![
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
