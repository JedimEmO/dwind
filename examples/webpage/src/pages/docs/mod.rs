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
    Home,
    GettingStarted,
    Animation,
    Colors,
    Responsiveness,
    Pseudoclasses,
    // Layout
    Flex,
    Grid,
    Position,
    Spacing,
    Sizing,

    // Style
    Typography,
    Borders,
    Shadows,
    Filters,
    Transforms,
    Interactivity,

    // Showcases
    Examples,
    DwuiExamples,
}

impl DocPage {
    pub fn goto(&self) {
        match self {
            DocPage::Home => go_to_url("#/"),
            DocPage::GettingStarted => go_to_url("#/docs/getting-started"),
            DocPage::Colors => go_to_url("#/docs/colors"),
            DocPage::Flex => go_to_url("#/docs/flex"),
            DocPage::Grid => go_to_url("#/docs/grid"),
            DocPage::Position => go_to_url("#/docs/position"),
            DocPage::Spacing => go_to_url("#/docs/spacing"),
            DocPage::Sizing => go_to_url("#/docs/sizing"),
            DocPage::Typography => go_to_url("#/docs/typography"),
            DocPage::Borders => go_to_url("#/docs/borders"),
            DocPage::Shadows => go_to_url("#/docs/shadows"),
            DocPage::Filters => go_to_url("#/docs/filters"),
            DocPage::Transforms => go_to_url("#/docs/transforms"),
            DocPage::Interactivity => go_to_url("#/docs/interactivity"),
            DocPage::Responsiveness => go_to_url("#/docs/responsive-design"),
            DocPage::Pseudoclasses => go_to_url("#/docs/pseudoclasses"),
            DocPage::Examples => go_to_url("#/examples"),
            DocPage::DwuiExamples => go_to_url("#/components"),
            &DocPage::Animation => go_to_url("#/docs/animation"),
        }
    }
}
impl Default for DocPage {
    fn default() -> Self {
        Self::Home
    }
}

impl Display for DocPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DocPage::Home => write!(f, "Home"),
            DocPage::GettingStarted => write!(f, "Getting Started"),
            DocPage::Colors => write!(f, "Colors"),
            DocPage::Flex => write!(f, "Flex"),
            DocPage::Grid => write!(f, "Grid"),
            DocPage::Position => write!(f, "Position"),
            DocPage::Spacing => write!(f, "Spacing"),
            DocPage::Sizing => write!(f, "Sizing"),
            DocPage::Typography => write!(f, "Typography"),
            DocPage::Borders => write!(f, "Borders"),
            DocPage::Shadows => write!(f, "Shadows"),
            DocPage::Filters => write!(f, "Filters"),
            DocPage::Transforms => write!(f, "Transforms"),
            DocPage::Interactivity => write!(f, "Interactivity"),
            DocPage::Responsiveness => write!(f, "Responsiveness"),
            DocPage::Pseudoclasses => write!(f, "Pseudoclasses"),
            DocPage::Examples => write!(f, "Examples"),
            DocPage::DwuiExamples => write!(f, "Components"),
            DocPage::Animation => write!(f, "Animation"),
        }
    }
}

pub fn doc_sections() -> Vec<DocSection> {
    vec![
        DocSection {
            title: "General".to_string(),
            docs: vec![
                DocPage::GettingStarted,
                DocPage::Colors,
                DocPage::Responsiveness,
                DocPage::Pseudoclasses,
            ],
        },
        DocSection {
            title: "Layout".to_string(),
            docs: vec![
                DocPage::Flex,
                DocPage::Grid,
                DocPage::Position,
                DocPage::Spacing,
                DocPage::Sizing,
            ],
        },
        DocSection {
            title: "Style".to_string(),
            docs: vec![
                DocPage::Typography,
                DocPage::Borders,
                DocPage::Shadows,
                DocPage::Filters,
                DocPage::Transforms,
                DocPage::Animation,
                DocPage::Interactivity,
            ],
        },
    ]
}
