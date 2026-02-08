use std::rc::Rc;

use dominator::{svg, Dom};
use dwind_graph_data::prelude::PlotArea;
use futures_signals::signal::SignalExt;

use super::interaction::ChartInteraction;

/// Render crosshair lines that follow the mouse position within the plot area.
pub fn render_crosshair(plot_area: PlotArea, interaction: Rc<ChartInteraction>) -> Dom {
    let interaction2 = interaction.clone();

    svg!("g", {
        // Vertical crosshair line
        .child_signal(interaction.mouse_pos.signal().map(move |pos| {
            pos.and_then(|(mx, _)| {
                if mx >= plot_area.x && mx <= plot_area.x2() {
                    Some(svg!("line", {
                        .attr("x1", &format!("{:.2}", mx))
                        .attr("y1", &format!("{:.2}", plot_area.y))
                        .attr("x2", &format!("{:.2}", mx))
                        .attr("y2", &format!("{:.2}", plot_area.y2()))
                        .attr("stroke", "rgba(255, 255, 255, 0.25)")
                        .attr("stroke-width", "1")
                        .attr("stroke-dasharray", "4 4")
                        .attr("pointer-events", "none")
                    }))
                } else {
                    None
                }
            })
        }))

        // Horizontal crosshair line
        .child_signal(interaction2.mouse_pos.signal().map(move |pos| {
            pos.and_then(|(_, my)| {
                if my >= plot_area.y && my <= plot_area.y2() {
                    Some(svg!("line", {
                        .attr("x1", &format!("{:.2}", plot_area.x))
                        .attr("y1", &format!("{:.2}", my))
                        .attr("x2", &format!("{:.2}", plot_area.x2()))
                        .attr("y2", &format!("{:.2}", my))
                        .attr("stroke", "rgba(255, 255, 255, 0.25)")
                        .attr("stroke-width", "1")
                        .attr("stroke-dasharray", "4 4")
                        .attr("pointer-events", "none")
                    }))
                } else {
                    None
                }
            })
        }))
    })
}
