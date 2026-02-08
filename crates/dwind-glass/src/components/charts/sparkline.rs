use dominator::{html, svg, Dom};
use dwind_graph_data::prelude::*;
use futures_signals_component_macro::component;

use super::svg_util;
use super::types::ChartPalette;

#[component(render_fn = glass_sparkline)]
struct GlassSparkline {
    /// Y values for the sparkline.
    #[default(vec![])]
    data: Vec<f64>,

    /// Width of the SVG viewBox.
    #[default(120.0)]
    width: f64,

    /// Height of the SVG viewBox.
    #[default(32.0)]
    height: f64,

    /// Override the stroke color (defaults to accent from palette).
    #[default(None)]
    color: Option<String>,

    /// Show area fill under the line.
    #[default(true)]
    area: bool,
}

pub fn glass_sparkline(props: GlassSparklineProps) -> Dom {
    let GlassSparklineProps {
        data,
        width,
        height,
        color,
        area,
        apply,
    } = props;

    if data.is_empty() {
        return html!("div", {});
    }

    let palette = ChartPalette::glass();
    let stroke_color = color.unwrap_or_else(|| palette.color(0).to_string());

    // Compute scales
    let y_extent = Extent::from_iter(data.iter().copied())
        .unwrap_or(Extent::new(0.0, 1.0))
        .pad(0.1);

    let padding = 2.0;
    let plot_w = width - padding * 2.0;
    let plot_h = height - padding * 2.0;

    let x_step = if data.len() > 1 {
        plot_w / (data.len() - 1) as f64
    } else {
        0.0
    };

    let y_scale = LinearScale::new(y_extent);

    let points: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let x = padding + i as f64 * x_step;
            let y = padding + plot_h - y_scale.to_pixel(v, plot_h);
            (x, y)
        })
        .collect();

    let line_d = svg_util::points_to_smooth_path_d(&points);

    html!("span", {
        .style("display", "inline-block")
        .style("line-height", "0")

        .child(svg!("svg", {
            .attr("viewBox", &format!("0 0 {} {}", width, height))
            .attr("width", &format!("{}", width))
            .attr("height", &format!("{}", height))
            .attr("preserveAspectRatio", "none")

            // Area fill
            .apply_if(area, {
                let area_d = svg_util::area_path_d(&points, height - padding, true);
                let area_color = format!(
                    "color-mix(in srgb, {} 20%, transparent)", stroke_color
                );
                move |b| {
                    b.child(svg!("path", {
                        .attr("d", &area_d)
                        .attr("fill", &area_color)
                        .attr("stroke", "none")
                    }))
                }
            })

            // Line
            .child(svg!("path", {
                .attr("d", &line_d)
                .attr("fill", "none")
                .attr("stroke", &stroke_color)
                .attr("stroke-width", "1.5")
                .attr("stroke-linecap", "round")
                .attr("stroke-linejoin", "round")
            }))
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
