use std::f64::consts::PI;
use std::rc::Rc;

use dominator::{clone, events, html, svg, Dom};
use dwind::prelude::*;
use dwind_graph_data::prelude::PieSlice;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

use super::shared::legend::{render_legend, LegendItem};
use super::svg_util;
use super::types::ChartPalette;

// ---------------------------------------------------------------------------
// Internal computed layout — recomputed on every data change.
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct SliceDatum {
    path_d: String,
    color: String,
    highlight: String,
    label: String,
    pct: f64,
    mid_angle: f64,
    label_x: f64,
    label_y: f64,
}

#[derive(Clone)]
struct ComputedPieLayout {
    slices: Vec<SliceDatum>,
    total: f64,
    legend_items: Vec<LegendItem>,
}

fn compute_pie_layout(
    raw_slices: &[PieSlice],
    cx: f64,
    cy: f64,
    outer_r: f64,
    inner_r: f64,
    palette: &ChartPalette,
) -> ComputedPieLayout {
    let total: f64 = raw_slices.iter().map(|s| s.value).sum();
    if total == 0.0 {
        return ComputedPieLayout {
            slices: vec![],
            total: 0.0,
            legend_items: vec![],
        };
    }

    let label_r = (outer_r + inner_r) / 2.0;
    let mut angle = 0.0_f64;
    let mut slices = Vec::with_capacity(raw_slices.len());

    for slice in raw_slices {
        let sweep = (slice.value / total) * 2.0 * PI;
        let start = angle;
        let end = angle + sweep;
        let mid = (start + end) / 2.0;

        let path_d = svg_util::arc_path_split(cx, cy, outer_r, inner_r, start, end);
        let color = palette.color(slice.color_index).to_string();
        let highlight = palette.highlight_color(slice.color_index).to_string();
        let pct = (slice.value / total) * 100.0;

        let mid_offset = mid - PI / 2.0;
        let lx = cx + label_r * mid_offset.cos();
        let ly = cy + label_r * mid_offset.sin();

        slices.push(SliceDatum {
            path_d,
            color,
            highlight,
            label: slice.label.clone(),
            pct,
            mid_angle: mid,
            label_x: lx,
            label_y: ly,
        });

        angle = end;
    }

    let legend_items = raw_slices
        .iter()
        .map(|s| LegendItem {
            label: s.label.clone(),
            color_index: s.color_index,
        })
        .collect();

    ComputedPieLayout {
        slices,
        total,
        legend_items,
    }
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component(render_fn = glass_pie_chart)]
struct GlassPieChart {
    /// Static pie slices (used when `live_slices` is None).
    #[default(vec![])]
    slices: Vec<PieSlice>,

    /// Reactive data source for live charts. When set, the chart keeps its SVG
    /// structure stable and smoothly transitions slice arcs on updates.
    #[default(None)]
    live_slices: Option<Mutable<Vec<PieSlice>>>,

    /// Chart size (width and height of the SVG viewBox).
    #[default(300.0)]
    size: f64,

    /// Inner radius ratio: 0.0 = full pie, 0.5 = donut, etc.
    #[default(0.0)]
    inner_radius: f64,

    /// Show percentage labels on slices.
    #[default(true)]
    show_labels: bool,

    /// Show legend below the chart.
    #[default(true)]
    show_legend: bool,

    /// Enable hover/click interaction.
    #[default(true)]
    interactive: bool,

    /// Custom color palette.
    #[default(ChartPalette::glass())]
    palette: ChartPalette,

    /// Click callback: slice_index.
    #[default(Box::new(|_| {}))]
    on_click: dyn Fn(usize) -> () + 'static,
}

pub fn glass_pie_chart(props: GlassPieChartProps) -> Dom {
    let GlassPieChartProps {
        slices,
        live_slices,
        size,
        inner_radius,
        show_labels,
        show_legend,
        interactive,
        palette,
        on_click,
        apply,
    } = props;

    let on_click = Rc::new(on_click);
    let palette = Rc::new(palette);

    let cx = size / 2.0;
    let cy = size / 2.0;
    let outer_r = size / 2.0 - 10.0;
    let inner_r = outer_r * inner_radius.clamp(0.0, 0.95);

    // Determine data source
    let is_live = live_slices.is_some();
    let slices_source = live_slices.unwrap_or_else(|| Mutable::new(slices));

    // Compute initial layout
    let initial_layout = compute_pie_layout(
        &slices_source.get_cloned(),
        cx, cy, outer_r, inner_r, &palette,
    );
    let initial_slice_count = initial_layout.slices.len();
    if initial_layout.total == 0.0 && !is_live {
        return html!("div", {});
    }

    let pie_layout: Mutable<Rc<ComputedPieLayout>> = Mutable::new(Rc::new(initial_layout));
    let hover: Mutable<Option<usize>> = Mutable::new(None);

    // Pre-allocate slice path slots
    let slot_count = initial_slice_count.max(2);

    let transition_css = if is_live {
        "transition: d 0.6s ease-in-out, transform 0.15s ease-out;"
    } else {
        "transition: transform 0.15s ease-out;"
    };

    // -----------------------------------------------------------------------
    // Build persistent slice path elements
    // -----------------------------------------------------------------------
    let slice_paths: Vec<Dom> = (0..slot_count)
        .map(|i| {
            let pie_layout = pie_layout.clone();
            let hover = hover.clone();
            let hover_enter = hover.clone();
            let hover_leave = hover.clone();
            let on_click = on_click.clone();
            let tc = transition_css;

            svg!("path", {
                .attr("stroke", "var(--glass-bg)")
                .attr("stroke-width", "2")

                // Combined style: d (animated) + fill (instant) + transform (animated)
                .attr_signal("style", {
                    futures_signals::map_ref! {
                        let h = hover.signal(),
                        let l = pie_layout.signal_cloned()
                        => (*h, l.clone())
                    }
                    .map(move |(hovered, l)| {
                        if let Some(sd) = l.slices.get(i) {
                            let fill = if hovered == Some(i) {
                                &sd.highlight
                            } else {
                                &sd.color
                            };
                            let transform = if hovered == Some(i) {
                                let mid = sd.mid_angle - PI / 2.0;
                                let dx = 4.0 * mid.cos();
                                let dy = 4.0 * mid.sin();
                                format!("translate({:.2}px, {:.2}px)", dx, dy)
                            } else {
                                "translate(0, 0)".to_string()
                            };
                            format!(
                                "d: path('{}'); fill: {}; cursor: pointer; transform: {}; {}",
                                sd.path_d, fill, transform, tc
                            )
                        } else {
                            "visibility: hidden;".to_string()
                        }
                    })
                })

                .apply_if(interactive, move |b| {
                    b.event(clone!(hover_enter => move |_: events::MouseEnter| {
                        hover_enter.set(Some(i));
                    }))
                    .event(clone!(hover_leave => move |_: events::MouseLeave| {
                        hover_leave.set(None);
                    }))
                    .event(move |_: events::Click| {
                        on_click(i);
                    })
                })
            })
        })
        .collect();

    // -----------------------------------------------------------------------
    // Build the chart DOM
    // -----------------------------------------------------------------------
    html!("div", {
        .dwclass!("relative inline-block")

        // Keep layout in sync with data changes
        .future({
            let pie_layout = pie_layout.clone();
            let palette = palette.clone();
            slices_source.signal_cloned().for_each(move |new_slices| {
                let new = compute_pie_layout(&new_slices, cx, cy, outer_r, inner_r, &palette);
                pie_layout.set(Rc::new(new));
                async {}
            })
        })

        // SVG — persistent
        .child(svg!("svg", {
            .attr("viewBox", &format!("0 0 {} {}", size, size))
            .attr("width", &format!("{}", size))
            .attr("height", &format!("{}", size))

            // Slice paths (persistent, CSS d transitions)
            .children(slice_paths)

            // Percentage labels (rebuilt on data change — just text)
            .apply_if(show_labels, {
                let pie_layout = pie_layout.clone();
                move |b| {
                    b.child_signal(pie_layout.signal_cloned().map(move |l| {
                        Some(svg!("g", {
                            .attr("pointer-events", "none")
                            .children(l.slices.iter().filter_map(|sd| {
                                if sd.pct < 3.0 {
                                    return None;
                                }
                                Some(svg!("text", {
                                    .attr("x", &format!("{:.2}", sd.label_x))
                                    .attr("y", &format!("{:.2}", sd.label_y))
                                    .attr("text-anchor", "middle")
                                    .attr("dominant-baseline", "middle")
                                    .attr("fill", "var(--glass-text-on-accent)")
                                    .attr("font-size", "11")
                                    .attr("font-weight", "600")
                                    .attr("pointer-events", "none")
                                    .text(&format!("{:.0}%", sd.pct))
                                }))
                            }).collect::<Vec<_>>())
                        }))
                    }))
                }
            })
        }))

        // Tooltip (HTML overlay)
        .apply_if(interactive, {
            let hover = hover.clone();
            let pie_layout = pie_layout.clone();
            move |b| {
                b.child(html!("div", {
                    .style("position", "absolute")
                    .style("top", "0")
                    .style("left", "0")
                    .style("width", "100%")
                    .style("height", "100%")
                    .style("pointer-events", "none")

                    .child_signal({
                        futures_signals::map_ref! {
                            let h = hover.signal(),
                            let l = pie_layout.signal_cloned()
                            => (*h, l.clone())
                        }
                        .map(move |(hovered, l)| {
                            hovered.and_then(|i| {
                                l.slices.get(i).map(|sd| {
                                    html!("div", {
                                        .style("position", "absolute")
                                        .style("top", "8px")
                                        .style("left", "50%")
                                        .style("transform", "translateX(-50%)")
                                        .style("pointer-events", "none")
                                        .style("z-index", "10")
                                        .style("background", "var(--glass-bg-elevated)")
                                        .style(["backdrop-filter", "-webkit-backdrop-filter"],
                                            "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
                                        .style("border", "1px solid var(--glass-border-color)")
                                        .style("border-radius", "var(--glass-border-radius)")
                                        .style("box-shadow", "var(--glass-shadow)")
                                        .style("padding", "6px 10px")
                                        .style("white-space", "nowrap")

                                        .child(html!("div", {
                                            .dwclass!("flex items-center gap-2 text-xs")
                                            .child(html!("span", {
                                                .style("display", "inline-block")
                                                .style("width", "8px")
                                                .style("height", "8px")
                                                .style("border-radius", "50%")
                                                .style("background", &sd.color)
                                            }))
                                            .child(html!("span", {
                                                .style("color", "var(--glass-text-secondary)")
                                                .text(&sd.label)
                                            }))
                                        }))
                                        .child(html!("div", {
                                            .dwclass!("text-sm font-semibold")
                                            .style("color", "var(--glass-text-primary)")
                                            .text(&format!("{:.1}%", sd.pct))
                                        }))
                                    })
                                })
                            })
                        })
                    })
                }))
            }
        })

        // Legend (rebuilt on data change)
        .apply_if(show_legend, {
            let pie_layout = pie_layout.clone();
            let palette = palette.clone();
            move |b| {
                b.child_signal(pie_layout.signal_cloned().map({
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
