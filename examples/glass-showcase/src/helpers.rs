use dominator::{html, Dom};
use dwind::prelude::*;
use dwind_glass::prelude::*;

use crate::code_block::code_example;

pub fn section(title: &str, id: &str, code: Option<&'static str>, children: Vec<Dom>) -> Dom {
    html!("div", {
        .attr("id", id)
        .dwclass!("flex flex-col gap-4 mb-8")
        .style("scroll-margin-top", "76px")
        .child(html!("h2", {
            .dwclass!("text-xl font-bold glass-text-primary")
            .text(title)
        }))
        .children(children)
        .apply_if(code.is_some(), |b| {
            b.child(code_example(code.unwrap()))
        })
    })
}

pub fn subsection(title: &str, children: Vec<Dom>) -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-3")
        .child(html!("h3", {
            .dwclass!("text-base font-medium glass-text-secondary")
            .text(title)
        }))
        .children(children)
    })
}
