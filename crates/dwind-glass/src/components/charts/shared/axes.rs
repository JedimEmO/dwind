use dominator::{svg, Dom};
use dwind_graph_data::prelude::*;

/// Render a Y axis (left side) with ticks and labels.
pub fn render_y_axis(
    plot_area: &PlotArea,
    y_scale: &LinearScale,
    tick_count: usize,
    label: Option<&str>,
) -> Dom {
    let y_extent = y_scale.domain();
    let ticks = generate_ticks(&y_extent, tick_count);

    svg!("g", {
        // Axis line
        .child(svg!("line", {
            .class("glass-chart-axis")
            .attr("x1", &format!("{:.2}", plot_area.x))
            .attr("y1", &format!("{:.2}", plot_area.y))
            .attr("x2", &format!("{:.2}", plot_area.x))
            .attr("y2", &format!("{:.2}", plot_area.y2()))
        }))

        // Ticks and labels
        .children(ticks.into_iter().map(|tick| {
            let y = plot_area.y + plot_area.height - y_scale.to_pixel(tick.value, plot_area.height);
            svg!("g", {
                // Tick mark
                .child(svg!("line", {
                    .class("glass-chart-axis")
                    .attr("x1", &format!("{:.2}", plot_area.x - 4.0))
                    .attr("y1", &format!("{:.2}", y))
                    .attr("x2", &format!("{:.2}", plot_area.x))
                    .attr("y2", &format!("{:.2}", y))
                }))
                // Label
                .child(svg!("text", {
                    .class("glass-chart-tick")
                    .attr("x", &format!("{:.2}", plot_area.x - 8.0))
                    .attr("y", &format!("{:.2}", y))
                    .attr("text-anchor", "end")
                    .attr("dominant-baseline", "middle")
                    .text(&tick.label)
                }))
            })
        }).collect::<Vec<_>>())

        // Axis label (rotated)
        .apply_if(label.is_some(), |b| {
            let label = label.unwrap();
            let mid_y = plot_area.y + plot_area.height / 2.0;
            b.child(svg!("text", {
                .class("glass-chart-label")
                .attr("x", "0")
                .attr("y", "0")
                .attr("text-anchor", "middle")
                .attr("dominant-baseline", "middle")
                .attr("transform", &format!(
                    "translate({:.2}, {:.2}) rotate(-90)",
                    plot_area.x - 40.0, mid_y
                ))
                .text(label)
            }))
        })
    })
}

/// Render an X axis (bottom) with ticks and labels.
pub fn render_x_axis(
    plot_area: &PlotArea,
    x_scale: &LinearScale,
    tick_count: usize,
    label: Option<&str>,
) -> Dom {
    let x_extent = x_scale.domain();
    let ticks = generate_ticks(&x_extent, tick_count);

    svg!("g", {
        // Axis line
        .child(svg!("line", {
            .class("glass-chart-axis")
            .attr("x1", &format!("{:.2}", plot_area.x))
            .attr("y1", &format!("{:.2}", plot_area.y2()))
            .attr("x2", &format!("{:.2}", plot_area.x2()))
            .attr("y2", &format!("{:.2}", plot_area.y2()))
        }))

        // Ticks and labels
        .children(ticks.into_iter().map(|tick| {
            let x = plot_area.x + x_scale.to_pixel(tick.value, plot_area.width);
            svg!("g", {
                // Tick mark
                .child(svg!("line", {
                    .class("glass-chart-axis")
                    .attr("x1", &format!("{:.2}", x))
                    .attr("y1", &format!("{:.2}", plot_area.y2()))
                    .attr("x2", &format!("{:.2}", x))
                    .attr("y2", &format!("{:.2}", plot_area.y2() + 4.0))
                }))
                // Label
                .child(svg!("text", {
                    .class("glass-chart-tick")
                    .attr("x", &format!("{:.2}", x))
                    .attr("y", &format!("{:.2}", plot_area.y2() + 16.0))
                    .attr("text-anchor", "middle")
                    .text(&tick.label)
                }))
            })
        }).collect::<Vec<_>>())

        // Axis label
        .apply_if(label.is_some(), |b| {
            let label = label.unwrap();
            let mid_x = plot_area.x + plot_area.width / 2.0;
            b.child(svg!("text", {
                .class("glass-chart-label")
                .attr("x", &format!("{:.2}", mid_x))
                .attr("y", &format!("{:.2}", plot_area.y2() + 32.0))
                .attr("text-anchor", "middle")
                .text(label)
            }))
        })
    })
}

/// Render a categorical X axis using a BandScale.
pub fn render_band_x_axis(
    plot_area: &PlotArea,
    band_scale: &BandScale,
    label: Option<&str>,
) -> Dom {
    svg!("g", {
        // Axis line
        .child(svg!("line", {
            .class("glass-chart-axis")
            .attr("x1", &format!("{:.2}", plot_area.x))
            .attr("y1", &format!("{:.2}", plot_area.y2()))
            .attr("x2", &format!("{:.2}", plot_area.x2()))
            .attr("y2", &format!("{:.2}", plot_area.y2()))
        }))

        // Category labels
        .children(band_scale.categories().iter().enumerate().map(|(i, cat)| {
            let x = plot_area.x + band_scale.to_pixel(i, plot_area.width);
            svg!("g", {
                .child(svg!("text", {
                    .class("glass-chart-tick")
                    .attr("x", &format!("{:.2}", x))
                    .attr("y", &format!("{:.2}", plot_area.y2() + 16.0))
                    .attr("text-anchor", "middle")
                    .text(cat)
                }))
            })
        }).collect::<Vec<_>>())

        // Axis label
        .apply_if(label.is_some(), |b| {
            let label = label.unwrap();
            let mid_x = plot_area.x + plot_area.width / 2.0;
            b.child(svg!("text", {
                .class("glass-chart-label")
                .attr("x", &format!("{:.2}", mid_x))
                .attr("y", &format!("{:.2}", plot_area.y2() + 32.0))
                .attr("text-anchor", "middle")
                .text(label)
            }))
        })
    })
}
