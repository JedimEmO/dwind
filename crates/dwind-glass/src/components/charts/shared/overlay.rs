use std::rc::Rc;

use dominator::{clone, events, svg, with_node, Dom};
use dwind_graph_data::prelude::*;
use wasm_bindgen::JsCast;

use super::interaction::{ChartInteraction, InteractionMode};
use crate::components::charts::svg_util;

/// Render the transparent interaction overlay that captures mouse events.
///
/// Handles hover (nearest point search), pan (drag), brush (shift+drag).
pub fn render_interaction_overlay(
    plot_area: PlotArea,
    interaction: Rc<ChartInteraction>,
    x_scale: LinearScale,
    _y_scale: LinearScale,
    data: Rc<Vec<Series>>,
) -> Dom {
    let interaction_move = interaction.clone();
    let interaction_leave = interaction.clone();
    let interaction_down = interaction.clone();
    let interaction_up = interaction.clone();
    let x_scale_move = x_scale.clone();
    let x_scale_down = x_scale.clone();
    let x_scale_up = x_scale.clone();

    svg!("rect" => web_sys::SvgElement, {
        .attr("x", &format!("{:.2}", plot_area.x))
        .attr("y", &format!("{:.2}", plot_area.y))
        .attr("width", &format!("{:.2}", plot_area.width))
        .attr("height", &format!("{:.2}", plot_area.height))
        .attr("fill", "transparent")
        .attr("style", "cursor: crosshair")

        .with_node!(node => {
            .event(clone!(node, interaction_move, data, x_scale_move => move |e: events::MouseMove| {
                let svg_parent = find_parent_svg(&node);
                if let Some(ref svg_el) = svg_parent {
                    if let Some((sx, sy)) = svg_util::client_to_svg(svg_el, e.x() as f64, e.y() as f64) {
                        interaction_move.mouse_pos.set(Some((sx, sy)));

                        // Find nearest data point
                        let domain_x = x_scale_move.from_pixel(sx - plot_area.x, plot_area.width);
                        let nearest = find_nearest_point(&data, domain_x);
                        interaction_move.nearest_point.set(nearest);

                        // Handle pan
                        if let InteractionMode::Pan { domain_at_start, start_x, .. } = interaction_move.mode.get() {
                            let current_domain_x = x_scale_move.from_pixel(sx - plot_area.x, plot_area.width);
                            let start_domain_x = x_scale_move.from_pixel(start_x - plot_area.x, plot_area.width);
                            let dx = current_domain_x - start_domain_x;
                            interaction_move.view_domain_x.set(Some(Extent::new(
                                domain_at_start.min - dx,
                                domain_at_start.max - dx,
                            )));
                        }

                        // Handle brush
                        if let InteractionMode::Brush { start_x, start_y } = interaction_move.mode.get() {
                            interaction_move.brush_rect.set(Some((
                                start_x.min(sx),
                                start_y.min(sy),
                                (sx - start_x).abs(),
                                (sy - start_y).abs(),
                            )));
                        }
                    }
                }
            }))

            .event(clone!(interaction_leave => move |_: events::MouseLeave| {
                interaction_leave.mouse_pos.set(None);
                interaction_leave.nearest_point.set(None);
            }))

            .event(clone!(node, interaction_down, x_scale_down => move |e: events::MouseDown| {
                let svg_parent = find_parent_svg(&node);
                if let Some(ref svg_el) = svg_parent {
                    if let Some((sx, sy)) = svg_util::client_to_svg(svg_el, e.x() as f64, e.y() as f64) {
                        if e.shift_key() {
                            interaction_down.mode.set(InteractionMode::Brush {
                                start_x: sx,
                                start_y: sy,
                            });
                        } else {
                            let current_domain = interaction_down.view_domain_x.get_cloned()
                                .unwrap_or(x_scale_down.domain());
                            interaction_down.mode.set(InteractionMode::Pan {
                                start_x: sx,
                                start_y: sy,
                                domain_at_start: current_domain,
                            });
                        }
                    }
                }
            }))

            // Global mouseup to end interaction
            .global_event(clone!(interaction_up, x_scale_up => move |_: events::MouseUp| {
                let mode = interaction_up.mode.replace(InteractionMode::None);

                if let InteractionMode::Brush { .. } = mode {
                    if let Some((rx, _, rw, _)) = interaction_up.brush_rect.get_cloned() {
                        if rw > 5.0 {
                            let domain_start = x_scale_up.from_pixel(rx - plot_area.x, plot_area.width);
                            let domain_end = x_scale_up.from_pixel(rx + rw - plot_area.x, plot_area.width);
                            interaction_up.view_domain_x.set(Some(Extent::new(
                                domain_start.min(domain_end),
                                domain_start.max(domain_end),
                            )));
                        }
                    }
                    interaction_up.brush_rect.set(None);
                }
            }))
        })
    })
}

/// Find the nearest data point to a given x domain value via binary search.
fn find_nearest_point(data: &[Series], domain_x: f64) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize, f64)> = None;

    for (si, series) in data.iter().enumerate() {
        if series.data.is_empty() {
            continue;
        }

        let idx = match series
            .data
            .binary_search_by(|p| p.x.partial_cmp(&domain_x).unwrap_or(std::cmp::Ordering::Equal))
        {
            Ok(i) => i,
            Err(i) => {
                if i == 0 {
                    0
                } else if i >= series.data.len() {
                    series.data.len() - 1
                } else {
                    let d_left = (series.data[i - 1].x - domain_x).abs();
                    let d_right = (series.data[i].x - domain_x).abs();
                    if d_left < d_right {
                        i - 1
                    } else {
                        i
                    }
                }
            }
        };

        let dist = (series.data[idx].x - domain_x).abs();
        if best.is_none() || dist < best.unwrap().2 {
            best = Some((si, idx, dist));
        }
    }

    best.map(|(si, pi, _)| (si, pi))
}

/// Walk up the DOM tree to find the parent `<svg>` element.
fn find_parent_svg(el: &web_sys::SvgElement) -> Option<web_sys::SvgsvgElement> {
    let mut current: web_sys::Element = el.clone().unchecked_into();
    loop {
        if let Ok(svg) = current.clone().dyn_into::<web_sys::SvgsvgElement>() {
            return Some(svg);
        }
        match current.parent_element() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}
