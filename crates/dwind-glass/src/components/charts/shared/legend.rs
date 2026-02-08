use crate::prelude::*;
use dominator::{html, Dom};
use dwind::prelude::*;

use crate::components::charts::types::ChartPalette;

/// A legend item: label + color.
#[derive(Clone)]
pub struct LegendItem {
    pub label: String,
    pub color_index: usize,
}

/// Render a horizontal legend below the chart.
pub fn render_legend(items: &[LegendItem], palette: &ChartPalette) -> Dom {
    html!("div", {
        .dwclass!("flex flex-wrap items-center justify-center gap-4 mt-2")
        .children(items.iter().map(|item| {
            let color = palette.color(item.color_index).to_string();
            html!("div", {
                .dwclass!("flex items-center gap-2")
                .child(html!("span", {
                    .style("display", "inline-block")
                    .style("width", "12px")
                    .style("height", "12px")
                    .style("border-radius", "var(--glass-border-radius-sm)")
                    .style("background", &color)
                }))
                .child(html!("span", {
                    .dwclass!("text-xs glass-text-secondary")
                    .text(&item.label)
                }))
            })
        }).collect::<Vec<_>>())
    })
}
