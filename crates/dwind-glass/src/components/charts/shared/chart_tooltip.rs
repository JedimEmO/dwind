use std::rc::Rc;

use dominator::{html, Dom};
use dwind::prelude::*;
use dwind_graph_data::prelude::Series;

use super::interaction::ChartInteraction;
use crate::components::charts::types::ChartPalette;

/// Render an HTML tooltip that appears near the cursor showing the nearest data point.
///
/// This is an HTML element positioned absolutely over the chart container.
pub fn render_chart_tooltip(
    interaction: Rc<ChartInteraction>,
    data: Rc<Vec<Series>>,
    palette: ChartPalette,
    vb_width: f64,
    vb_height: f64,
) -> Dom {
    html!("div", {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style("width", "100%")
        .style("height", "100%")
        .style("pointer-events", "none")
        .style("overflow", "visible")

        .child_signal({
            let interaction2 = interaction.clone();
            futures_signals::map_ref! {
                let pos = interaction.mouse_pos.signal(),
                let nearest = interaction2.nearest_point.signal() => move {
                    match (pos, nearest) {
                        (Some((mx, my)), Some((series_idx, point_idx))) => {
                            let si = *series_idx;
                            let pi = *point_idx;
                            if si < data.len() && pi < data[si].data.len() {
                                let point = &data[si].data[pi];
                                let series_label = &data[si].label;
                                let color = palette.color(data[si].color_index).to_string();

                                let left_pct = (mx / vb_width) * 100.0;
                                let top_pct = (my / vb_height) * 100.0;

                                Some(html!("div", {
                                    .style("position", "absolute")
                                    .style("left", &format!("{}%", left_pct))
                                    .style("top", &format!("{}%", top_pct))
                                    .style("transform", "translate(-50%, -120%)")
                                    .style("pointer-events", "none")
                                    .style("z-index", "10")
                                    .style("background", "var(--glass-bg-elevated)")
                                    .style(["backdrop-filter", "-webkit-backdrop-filter"],
                                        "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
                                    .style("border", "1px solid var(--glass-border-color)")
                                    .style("border-radius", "var(--glass-border-radius)")
                                    .style("box-shadow", "var(--glass-shadow-lg)")
                                    .style("padding", "6px 10px")
                                    .style("white-space", "nowrap")

                                    .child(html!("div", {
                                        .dwclass!("flex items-center gap-2 text-xs")
                                        .child(html!("span", {
                                            .style("display", "inline-block")
                                            .style("width", "8px")
                                            .style("height", "8px")
                                            .style("border-radius", "50%")
                                            .style("background", &color)
                                        }))
                                        .child(html!("span", {
                                            .style("color", "var(--glass-text-secondary)")
                                            .text(series_label)
                                        }))
                                    }))
                                    .child(html!("div", {
                                        .dwclass!("text-sm font-semibold")
                                        .style("color", "var(--glass-text-primary)")
                                        .text(&format!("x: {:.2}  y: {:.2}", point.x, point.y))
                                    }))
                                }))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
            }
        })
    })
}
