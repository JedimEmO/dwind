use dominator::{text, Dom};
use dwui::prelude::*;

pub fn doc_page_title(title: &str) -> Dom {
    heading!({
        .content(text(title))
        .text_size(TextSize::ExtraLarge)
    })
}
