use std::rc::Rc;

use crate::prelude::*;
use dominator::{clone, events, html, svg, Dom};
use dwind::prelude::*;
use dwind_graph_data::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

use super::shared::axes::{render_band_x_axis, render_y_axis};
use super::shared::grid::render_grid;
use super::shared::legend::{render_legend, LegendItem};
use super::types::{BarMode, BarOrientation, ChartPalette};

#[component(render_fn = glass_bar_chart)]
struct GlassBarChart {
    /// Data series. For grouped bars, each series becomes a group.
    /// The x values should be category indices (0, 1, 2...).
    #[default(vec![])]
    series: Vec<Series>,

    /// Category labels for the x axis.
    #[default(vec![])]
    categories: Vec<String>,

    /// SVG viewBox width.
    #[default(800.0)]
    width: f64,

    /// SVG viewBox height.
    #[default(450.0)]
    height: f64,

    /// Bar orientation.
    #[default(BarOrientation::Vertical)]
    orientation: BarOrientation,

    /// Grouped or stacked bar layout.
    #[default(BarMode::Grouped)]
    mode: BarMode,

    /// Show background grid lines.
    #[default(true)]
    show_grid: bool,

    /// Show legend below the chart.
    #[default(true)]
    show_legend: bool,

    /// Enable hover highlighting.
    #[default(true)]
    interactive: bool,

    /// Custom color palette.
    #[default(ChartPalette::glass())]
    palette: ChartPalette,

    /// Click callback: (series_index, category_index).
    #[default(Box::new(|_, _| {}))]
    on_click: dyn Fn(usize, usize) -> () + 'static,
}

pub fn glass_bar_chart(props: GlassBarChartProps) -> Dom {
    let GlassBarChartProps {
        series,
        categories,
        width,
        height,
        orientation: _orientation,
        mode,
        show_grid,
        show_legend,
        interactive,
        palette,
        on_click,
        apply,
    } = props;

    let on_click = Rc::new(on_click);

    let margins = Margins {
        top: 20.0,
        right: 20.0,
        bottom: 40.0,
        left: 50.0,
    };
    let plot_area = PlotArea::from_viewbox(width, height, &margins);

    let band_scale = BandScale::new(categories.clone(), 0.2);

    // Compute y extent
    let y_max = match mode {
        BarMode::Stacked => {
            let stacked = compute_stack(&series);
            stacked_y_max(&stacked)
        }
        BarMode::Grouped => series
            .iter()
            .flat_map(|s| s.data.iter().map(|p| p.y))
            .fold(0.0_f64, f64::max),
    };

    let y_extent = Extent::new(0.0, y_max).nice();
    let y_scale = LinearScale::new(y_extent);

    // Hover state: (series_index, category_index)
    let hover: Mutable<Option<(usize, usize)>> = Mutable::new(None);

    // Build bars
    let bars: Vec<Dom> = match mode {
        BarMode::Grouped => render_grouped_bars(
            &series,
            &plot_area,
            &band_scale,
            &y_scale,
            &palette,
            &hover,
            interactive,
            on_click.clone(),
        ),
        BarMode::Stacked => render_stacked_bars(
            &series,
            &plot_area,
            &band_scale,
            &y_scale,
            &palette,
            &hover,
            interactive,
            on_click.clone(),
        ),
    };

    let legend_items: Vec<LegendItem> = series
        .iter()
        .map(|s| LegendItem {
            label: s.label.clone(),
            color_index: s.color_index,
        })
        .collect();

    html!("div", {
        .dwclass!("relative w-full")
        .style("aspect-ratio", &format!("{} / {}", width, height))

        .child(svg!("svg", {
            .attr("viewBox", &format!("0 0 {} {}", width, height))
            .attr("preserveAspectRatio", "xMidYMid meet")
            .attr("width", "100%")
            .attr("height", "100%")

            // Grid
            .apply_if(show_grid, {
                let y_scale_grid = y_scale.clone();
                move |b| b.child(render_grid(&plot_area, &y_scale_grid, 6))
            })

            // Axes
            .child(render_y_axis(&plot_area, &y_scale, 6, None))
            .child(render_band_x_axis(&plot_area, &band_scale, None))

            // Bars
            .children(bars)
        }))

        // Tooltip on hover
        .apply_if(interactive, {
            let hover_tt = hover.clone();
            let series_tt = series.clone();
            let palette_tt = palette.clone();
            move |b| {
                b.child(html!("div", {
                    .style("position", "absolute")
                    .style("top", "0")
                    .style("left", "0")
                    .style("width", "100%")
                    .style("height", "100%")
                    .style("pointer-events", "none")

                    .child_signal(hover_tt.signal_cloned().map(move |hovered| {
                        hovered.and_then(|(si, ci)| {
                            if si < series_tt.len() && ci < series_tt[si].data.len() {
                                let point = &series_tt[si].data[ci];
                                let label = &series_tt[si].label;
                                let color = palette_tt.color(series_tt[si].color_index).to_string();

                                Some(html!("div", {
                                    .style("position", "absolute")
                                    .style("top", "8px")
                                    .style("right", "8px")
                                    .style("pointer-events", "none")
                                    .style("z-index", "10")
                                    .style("background", "var(--glass-bg-elevated)")
                                    .style(["backdrop-filter", "-webkit-backdrop-filter"],
                                        "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
                                    .style("border", "1px solid var(--glass-border-color)")
                                    .style("border-radius", "var(--glass-border-radius)")
                                    .style("box-shadow", "var(--glass-shadow)")
                                    .style("padding", "6px 10px")

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
                                            .text(label)
                                        }))
                                    }))
                                    .child(html!("div", {
                                        .dwclass!("text-sm font-semibold glass-text-primary")
                                        .text(&format!("{:.1}", point.y))
                                    }))
                                }))
                            } else {
                                None
                            }
                        })
                    }))
                }))
            }
        })

        .apply_if(show_legend && !legend_items.is_empty(), {
            move |b| b.child(render_legend(&legend_items, &palette))
        })

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}

fn render_grouped_bars(
    series: &[Series],
    plot_area: &PlotArea,
    band_scale: &BandScale,
    y_scale: &LinearScale,
    palette: &ChartPalette,
    hover: &Mutable<Option<(usize, usize)>>,
    interactive: bool,
    on_click: Rc<Box<dyn Fn(usize, usize) -> () + 'static>>,
) -> Vec<Dom> {
    let n_series = series.len().max(1);
    let bandwidth = band_scale.bandwidth(plot_area.width);
    let bar_width = bandwidth / n_series as f64;
    let gap = bar_width * 0.1;
    let bar_w = bar_width - gap;

    let mut bars = Vec::new();

    for (si, s) in series.iter().enumerate() {
        for (ci, point) in s.data.iter().enumerate() {
            let band_start = plot_area.x + band_scale.band_start(ci, plot_area.width);
            let bar_x = band_start + si as f64 * bar_width + gap * 0.5;
            let bar_h = y_scale.to_pixel(point.y, plot_area.height);
            let bar_y = plot_area.y + plot_area.height - bar_h;

            let color = palette.color(s.color_index).to_string();
            let highlight = palette.highlight_color(s.color_index).to_string();

            let hover_clone = hover.clone();
            let hover_enter = hover.clone();
            let hover_leave = hover.clone();
            let on_click = on_click.clone();

            bars.push(svg!("rect", {
                .attr("x", &format!("{:.2}", bar_x))
                .attr("y", &format!("{:.2}", bar_y))
                .attr("width", &format!("{:.2}", bar_w.max(1.0)))
                .attr("height", &format!("{:.2}", bar_h.max(0.0)))
                .attr("rx", "3")

                .attr_signal("fill", hover_clone.signal_cloned().map(move |h| {
                    if h == Some((si, ci)) {
                        highlight.clone()
                    } else {
                        color.clone()
                    }
                }))

                .apply_if(interactive, move |b| {
                    b.event(clone!(hover_enter => move |_: events::MouseEnter| {
                        hover_enter.set(Some((si, ci)));
                    }))
                    .event(clone!(hover_leave => move |_: events::MouseLeave| {
                        hover_leave.set(None);
                    }))
                    .event(move |_: events::Click| {
                        on_click(si, ci);
                    })
                    .attr("style", "cursor: pointer")
                })
            }));
        }
    }

    bars
}

fn render_stacked_bars(
    series: &[Series],
    plot_area: &PlotArea,
    band_scale: &BandScale,
    y_scale: &LinearScale,
    palette: &ChartPalette,
    hover: &Mutable<Option<(usize, usize)>>,
    interactive: bool,
    on_click: Rc<Box<dyn Fn(usize, usize) -> () + 'static>>,
) -> Vec<Dom> {
    let stacked = compute_stack(series);
    let bandwidth = band_scale.bandwidth(plot_area.width);
    let gap = bandwidth * 0.1;
    let bar_w = bandwidth - gap;

    let mut bars = Vec::new();

    for (si, ss) in stacked.iter().enumerate() {
        for (ci, &(_x, y_base, y_top)) in ss.data.iter().enumerate() {
            let band_start = plot_area.x + band_scale.band_start(ci, plot_area.width);
            let bar_x = band_start + gap * 0.5;
            let px_base = y_scale.to_pixel(y_base, plot_area.height);
            let px_top = y_scale.to_pixel(y_top, plot_area.height);
            let bar_h = px_top - px_base;
            let bar_y = plot_area.y + plot_area.height - px_top;

            let color = palette.color(ss.color_index).to_string();
            let highlight = palette.highlight_color(ss.color_index).to_string();

            let hover_clone = hover.clone();
            let hover_enter = hover.clone();
            let hover_leave = hover.clone();
            let on_click = on_click.clone();

            bars.push(svg!("rect", {
                .attr("x", &format!("{:.2}", bar_x))
                .attr("y", &format!("{:.2}", bar_y))
                .attr("width", &format!("{:.2}", bar_w.max(1.0)))
                .attr("height", &format!("{:.2}", bar_h.max(0.0)))
                .attr("rx", "2")

                .attr_signal("fill", hover_clone.signal_cloned().map(move |h| {
                    if h == Some((si, ci)) {
                        highlight.clone()
                    } else {
                        color.clone()
                    }
                }))

                .apply_if(interactive, move |b| {
                    b.event(clone!(hover_enter => move |_: events::MouseEnter| {
                        hover_enter.set(Some((si, ci)));
                    }))
                    .event(clone!(hover_leave => move |_: events::MouseLeave| {
                        hover_leave.set(None);
                    }))
                    .event(move |_: events::Click| {
                        on_click(si, ci);
                    })
                    .attr("style", "cursor: pointer")
                })
            }));
        }
    }

    bars
}
