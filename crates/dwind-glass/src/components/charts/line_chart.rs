use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dominator::{html, svg, Dom};

static CHART_ID: AtomicUsize = AtomicUsize::new(0);
use dwind::prelude::*;
use dwind_graph_data::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

use super::shared::axes::{render_x_axis, render_y_axis};
use super::shared::crosshair::render_crosshair;
use super::shared::grid::render_grid;
use super::shared::interaction::ChartInteraction;
use super::shared::legend::{render_legend, LegendItem};
use super::shared::overlay::render_interaction_overlay;
use super::svg_util;
use super::types::ChartPalette;

// ---------------------------------------------------------------------------
// Internal computed layout — recomputed on every data change.
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct PathDatum {
    line_d: String,
    area_d: Option<String>,
    color: String,
    area_color: String,
}

#[derive(Clone)]
struct ComputedLayout {
    x_scale: LinearScale,
    y_scale: LinearScale,
    data: Rc<Vec<Series>>,
    paths: Vec<PathDatum>,
    legend_items: Vec<LegendItem>,
}

fn compute_layout(
    raw_series: Vec<Series>,
    max_points: usize,
    smooth: bool,
    area: bool,
    palette: &ChartPalette,
    plot_area: &PlotArea,
) -> ComputedLayout {
    let series: Vec<Series> = raw_series
        .into_iter()
        .map(|mut s| {
            if s.data.len() > max_points {
                s.data = lttb(&s.data, max_points);
            }
            s
        })
        .collect();

    // Use raw data range for x so lines always span the full plot width.
    // (.nice() would round to "pretty" boundaries that shift as the window
    // slides, causing visible horizontal jitter.)
    let x_extent = Extent::x_extent_of_series(&series)
        .unwrap_or(Extent::new(0.0, 1.0));
    let y_extent = Extent::y_extent_of_series(&series)
        .unwrap_or(Extent::new(0.0, 1.0))
        .nice();

    let x_scale = LinearScale::new(x_extent);
    let y_scale = LinearScale::new(y_extent);

    let paths = series
        .iter()
        .map(|s| {
            let points: Vec<(f64, f64)> = s
                .data
                .iter()
                .map(|p| {
                    let px = plot_area.x + x_scale.to_pixel(p.x, plot_area.width);
                    let py =
                        plot_area.y + plot_area.height - y_scale.to_pixel(p.y, plot_area.height);
                    (px, py)
                })
                .collect();

            let line_d = if smooth {
                svg_util::points_to_smooth_path_d(&points)
            } else {
                svg_util::points_to_path_d(&points)
            };
            let area_d = if area {
                Some(svg_util::area_path_d(&points, plot_area.y2(), smooth))
            } else {
                None
            };
            let color = palette.color(s.color_index).to_string();
            let area_color = palette.area_color(s.color_index);

            PathDatum {
                line_d,
                area_d,
                color,
                area_color,
            }
        })
        .collect();

    let legend_items = series
        .iter()
        .map(|s| LegendItem {
            label: s.label.clone(),
            color_index: s.color_index,
        })
        .collect();

    ComputedLayout {
        x_scale,
        y_scale,
        data: Rc::new(series),
        paths,
        legend_items,
    }
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component(render_fn = glass_line_chart)]
struct GlassLineChart {
    /// Static data series (used when `live_series` is None).
    #[default(vec![])]
    series: Vec<Series>,

    /// Reactive data source for live/streaming charts. When set, the chart
    /// keeps its SVG structure stable and smoothly transitions paths on updates.
    #[default(None)]
    live_series: Option<Mutable<Vec<Series>>>,

    /// SVG viewBox width.
    #[default(800.0)]
    width: f64,

    /// SVG viewBox height.
    #[default(450.0)]
    height: f64,

    /// Show background grid lines.
    #[default(true)]
    show_grid: bool,

    /// Show legend below the chart.
    #[default(true)]
    show_legend: bool,

    /// X axis label.
    #[default(None)]
    x_label: Option<String>,

    /// Y axis label.
    #[default(None)]
    y_label: Option<String>,

    /// Max points per series before auto-downsampling via LTTB.
    #[default(500)]
    max_points: usize,

    /// Enable interactive features (zoom, pan, brush, crosshair, tooltip).
    #[default(true)]
    interactive: bool,

    /// Use smooth (cubic) line interpolation.
    #[default(false)]
    smooth: bool,

    /// Show area fill under the lines.
    #[default(false)]
    area: bool,

    /// Custom color palette.
    #[default(ChartPalette::glass())]
    palette: ChartPalette,

    /// Click callback: (series_index, point_index).
    #[default(Box::new(|_, _| {}))]
    on_click: dyn Fn(usize, usize) -> () + 'static,
}

pub fn glass_line_chart(props: GlassLineChartProps) -> Dom {
    let GlassLineChartProps {
        series,
        live_series,
        width,
        height,
        show_grid,
        show_legend,
        x_label,
        y_label,
        max_points,
        interactive,
        smooth,
        area,
        palette,
        on_click: _on_click,
        apply,
    } = props;

    let palette = Rc::new(palette);

    let margins = Margins {
        top: 20.0,
        right: 20.0,
        bottom: if x_label.is_some() { 48.0 } else { 40.0 },
        left: if y_label.is_some() { 60.0 } else { 50.0 },
    };
    let plot_area = PlotArea::from_viewbox(width, height, &margins);

    // Determine data source: live mutable or static (wrapped in a Mutable).
    let is_live = live_series.is_some();
    let series_source = live_series.unwrap_or_else(|| Mutable::new(series));

    // Compute initial layout so that the first paint is correct.
    let initial_layout = compute_layout(
        series_source.get_cloned(),
        max_points,
        smooth,
        area,
        &palette,
        &plot_area,
    );
    let initial_series_count = initial_layout.data.len();

    // Central layout state — all chart elements subscribe to this.
    let layout: Mutable<Rc<ComputedLayout>> = Mutable::new(Rc::new(initial_layout));

    let interaction = Rc::new(ChartInteraction::new());

    // Unique clip-path ID for this chart instance (safe with multiple charts on page).
    let clip_id = format!("lc-clip-{}", CHART_ID.fetch_add(1, Ordering::Relaxed));

    // Pre-allocate persistent path element slots.  For live charts the series
    // count is typically fixed; for safety we reserve a few extra slots that
    // stay hidden when unused.
    let path_slots = initial_series_count.max(2);

    // CSS transition duration for the SVG `d` property (only for live charts).
    let transition_css = if is_live {
        "transition: d 0.4s ease;"
    } else {
        ""
    };

    // -----------------------------------------------------------------------
    // Build persistent path elements
    // -----------------------------------------------------------------------
    let mut path_elements: Vec<Dom> = Vec::with_capacity(path_slots * 2);
    for i in 0..path_slots {
        // Area fill path (only if area mode is enabled)
        if area {
            let layout_sig = layout.clone();
            let tc = transition_css;
            path_elements.push(svg!("path", {
                .attr("stroke", "none")
                .attr("pointer-events", "none")
                .attr_signal("style", layout_sig.signal_cloned().map(move |l| {
                    if let Some(pd) = l.paths.get(i) {
                        if let Some(ref area_d) = pd.area_d {
                            format!(
                                "d: path('{}'); fill: {}; {}",
                                area_d, pd.area_color, tc
                            )
                        } else {
                            "visibility: hidden;".to_string()
                        }
                    } else {
                        "visibility: hidden;".to_string()
                    }
                }))
            }));
        }

        // Line path
        let layout_sig = layout.clone();
        let tc = transition_css;
        path_elements.push(svg!("path", {
            .attr("fill", "none")
            .attr("stroke-width", "2")
            .attr("stroke-linecap", "round")
            .attr("stroke-linejoin", "round")
            .attr("pointer-events", "none")
            .attr_signal("style", layout_sig.signal_cloned().map(move |l| {
                if let Some(pd) = l.paths.get(i) {
                    format!(
                        "d: path('{}'); stroke: {}; fill: none; {}",
                        pd.line_d, pd.color, tc
                    )
                } else {
                    "visibility: hidden;".to_string()
                }
            }))
        }));
    }

    // -----------------------------------------------------------------------
    // Build the chart DOM
    // -----------------------------------------------------------------------
    html!("div", {
        .dwclass!("relative w-full")
        .style("aspect-ratio", &format!("{} / {}", width, height))

        // Side-effect: keep `layout` in sync with data changes.
        .future({
            let layout = layout.clone();
            let palette = palette.clone();
            series_source.signal_cloned().for_each(move |series_data| {
                let new = compute_layout(
                    series_data, max_points, smooth, area, &palette, &plot_area,
                );
                layout.set(Rc::new(new));
                async {}
            })
        })

        // SVG element — persistent, never torn down.
        .child(svg!("svg", {
            .attr("viewBox", &format!("0 0 {} {}", width, height))
            .attr("preserveAspectRatio", "xMidYMid meet")
            .attr("width", "100%")
            .attr("height", "100%")

            // ClipPath: constrains data paths to the plot area so smooth curves
            // don't overshoot the axis boundaries.
            .child(svg!("defs", {
                .child(svg!("clipPath", {
                    .attr("id", &clip_id)
                    .child(svg!("rect", {
                        .attr("x", &format!("{:.2}", plot_area.x))
                        .attr("y", &format!("{:.2}", plot_area.y))
                        .attr("width", &format!("{:.2}", plot_area.width))
                        .attr("height", &format!("{:.2}", plot_area.height))
                    }))
                }))
            }))

            // Layer 1: grid + axes (rebuilt on data change; invisible since SVG stays)
            .child_signal({
                let layout = layout.clone();
                let x_label = x_label.clone();
                let y_label = y_label.clone();
                layout.signal_cloned().map(move |l| {
                    Some(svg!("g", {
                        .apply_if(show_grid, {
                            let y_s = l.y_scale.clone();
                            move |b| b.child(render_grid(&plot_area, &y_s, 6))
                        })
                        .child(render_y_axis(&plot_area, &l.y_scale, 6, y_label.as_deref()))
                        .child(render_x_axis(&plot_area, &l.x_scale, 8, x_label.as_deref()))
                    }))
                })
            })

            // Layer 2: persistent data paths with CSS `d` transitions, clipped to plot area
            .child(svg!("g", {
                .attr("clip-path", &format!("url(#{})", clip_id))
                .children(path_elements)
            }))

            // Layer 3: crosshair (persistent — only depends on mouse_pos)
            .apply_if(interactive, {
                let interaction = interaction.clone();
                move |b| b.child(render_crosshair(plot_area, interaction))
            })

            // Layer 4: highlight dot (depends on nearest_point + current data)
            .apply_if(interactive, {
                let interaction = interaction.clone();
                let layout = layout.clone();
                let palette = palette.clone();
                move |b| {
                    b.child(svg!("g", {
                        .child_signal({
                            futures_signals::map_ref! {
                                let nearest = interaction.nearest_point.signal(),
                                let l = layout.signal_cloned()
                                => (*nearest, l.clone())
                            }
                            .map(move |(nearest, l)| {
                                nearest.and_then(|(si, pi)| {
                                    let data = &l.data;
                                    if si < data.len() && pi < data[si].data.len() {
                                        let point = &data[si].data[pi];
                                        let px = plot_area.x + l.x_scale.to_pixel(point.x, plot_area.width);
                                        let py = plot_area.y + plot_area.height
                                            - l.y_scale.to_pixel(point.y, plot_area.height);
                                        let color = palette.color(data[si].color_index).to_string();

                                        Some(svg!("circle", {
                                            .attr("cx", &format!("{:.2}", px))
                                            .attr("cy", &format!("{:.2}", py))
                                            .attr("r", "5")
                                            .attr("fill", &color)
                                            .attr("stroke", "var(--glass-bg)")
                                            .attr("stroke-width", "2")
                                            .attr("pointer-events", "none")
                                        }))
                                    } else {
                                        None
                                    }
                                })
                            })
                        })
                    }))
                }
            })

            // Layer 5: brush selection rect (persistent — driven by brush_rect Mutable)
            .apply_if(interactive, {
                let interaction = interaction.clone();
                move |b| {
                    b.child(svg!("g", {
                        .child_signal(interaction.brush_rect.signal().map(|rect| {
                            rect.map(|(x, y, w, h)| {
                                svg!("rect", {
                                    .attr("x", &format!("{:.2}", x))
                                    .attr("y", &format!("{:.2}", y))
                                    .attr("width", &format!("{:.2}", w))
                                    .attr("height", &format!("{:.2}", h))
                                    .attr("fill", "var(--glass-accent-muted)")
                                    .attr("stroke", "var(--glass-accent)")
                                    .attr("stroke-width", "1")
                                    .attr("opacity", "0.3")
                                    .attr("pointer-events", "none")
                                })
                            })
                        }))
                    }))
                }
            })

            // Layer 6: interaction overlay (rebuilt on data change for fresh scales)
            .apply_if(interactive, {
                let interaction = interaction.clone();
                let layout = layout.clone();
                move |b| {
                    b.child_signal(layout.signal_cloned().map({
                        let interaction = interaction.clone();
                        move |l| {
                            Some(render_interaction_overlay(
                                plot_area,
                                interaction.clone(),
                                l.x_scale.clone(),
                                l.y_scale.clone(),
                                l.data.clone(),
                            ))
                        }
                    }))
                }
            })
        }))

        // Tooltip (HTML overlay — rebuilt on data change for current values)
        .apply_if(interactive, {
            let interaction = interaction.clone();
            let layout = layout.clone();
            let palette = palette.clone();
            move |b| {
                b.child_signal(layout.signal_cloned().map({
                    let interaction = interaction.clone();
                    let palette = palette.clone();
                    move |l| {
                        Some(render_chart_tooltip_reactive(
                            interaction.clone(),
                            l.data.clone(),
                            (*palette).clone(),
                            width,
                            height,
                        ))
                    }
                }))
            }
        })

        // Legend (rebuilt on data change)
        .apply_if(show_legend, {
            let layout = layout.clone();
            let palette = palette.clone();
            move |b| {
                b.child_signal(layout.signal_cloned().map({
                    let palette = palette.clone();
                    move |l| {
                        if l.legend_items.is_empty() {
                            None
                        } else {
                            Some(render_legend(&l.legend_items, &palette))
                        }
                    }
                }))
            }
        })

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}

// ---------------------------------------------------------------------------
// Reactive tooltip (identical logic to chart_tooltip.rs but takes owned data)
// ---------------------------------------------------------------------------

fn render_chart_tooltip_reactive(
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
