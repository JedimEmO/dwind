use dominator::{svg, Dom};
use dwind_graph_data::prelude::*;

/// Render horizontal grid lines within the plot area.
pub fn render_grid(plot_area: &PlotArea, y_scale: &LinearScale, tick_count: usize) -> Dom {
    let y_extent = y_scale.domain();
    let ticks = generate_ticks(&y_extent, tick_count);

    svg!("g", {
        .children(ticks.into_iter().map(|tick| {
            let y = plot_area.y + plot_area.height - y_scale.to_pixel(tick.value, plot_area.height);
            svg!("line", {
                .class("glass-chart-grid")
                .attr("x1", &format!("{:.2}", plot_area.x))
                .attr("y1", &format!("{:.2}", y))
                .attr("x2", &format!("{:.2}", plot_area.x2()))
                .attr("y2", &format!("{:.2}", y))
            })
        }).collect::<Vec<_>>())
    })
}

/// Render vertical grid lines within the plot area.
pub fn render_vertical_grid(plot_area: &PlotArea, x_scale: &LinearScale, tick_count: usize) -> Dom {
    let x_extent = x_scale.domain();
    let ticks = generate_ticks(&x_extent, tick_count);

    svg!("g", {
        .children(ticks.into_iter().map(|tick| {
            let x = plot_area.x + x_scale.to_pixel(tick.value, plot_area.width);
            svg!("line", {
                .class("glass-chart-grid")
                .attr("x1", &format!("{:.2}", x))
                .attr("y1", &format!("{:.2}", plot_area.y))
                .attr("x2", &format!("{:.2}", x))
                .attr("y2", &format!("{:.2}", plot_area.y2()))
            })
        }).collect::<Vec<_>>())
    })
}
