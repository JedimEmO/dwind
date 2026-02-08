use crate::prelude::*;
use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[component(render_fn = glass_table)]
struct GlassTable {
    #[default(vec![])]
    columns: Vec<String>,

    #[default(vec![])]
    rows: Vec<Vec<Dom>>,

    #[signal]
    #[default(false)]
    striped: bool,
}

pub fn glass_table(props: GlassTableProps) -> Dom {
    let GlassTableProps {
        columns,
        rows,
        striped,
        apply,
    } = props;

    let striped = striped.broadcast();

    html!("div", {
        .style("background", "var(--glass-bg-elevated)")
        .style(["backdrop-filter", "-webkit-backdrop-filter"], "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
        .style("border", "none")
        .style("border-radius", "var(--glass-border-radius-xl)")
        .style("box-shadow", "var(--glass-shadow)")
        .style("overflow", "hidden")

        .child(html!("table", {
            .dwclass!("w-full")
            .style("border-collapse", "collapse")

            // Header
            .child(html!("thead", {
                .child(html!("tr", {
                    .style("border-bottom", "var(--glass-border-width) solid var(--glass-border-color)")
                    .children(columns.into_iter().map(|col| {
                        html!("th", {
                            .dwclass!("px-4 py-3 text-left text-sm font-semibold glass-text-secondary")
                            .text(&col)
                        })
                    }).collect::<Vec<_>>())
                }))
            }))

            // Body
            .child(html!("tbody", {
                .children(rows.into_iter().enumerate().map(|(i, row)| {
                    let striped = striped.clone();
                    let row_hover = Mutable::new(false);
                    html!("tr", {
                        .style("border-bottom", "var(--glass-border-width) solid var(--glass-border-color)")
                        .style("transition-duration", "var(--glass-transition)")
                        .style("transition-property", "background")
                        .event(clone!(row_hover => move |_: events::MouseEnter| { row_hover.set(true); }))
                        .event(clone!(row_hover => move |_: events::MouseLeave| { row_hover.set(false); }))
                        .style_signal("background", {
                            let striped = striped.clone();
                            let row_hover = row_hover.clone();
                            map_ref! {
                                let s = striped.signal(),
                                let h = row_hover.signal() => {
                                    if *h {
                                        "rgba(255, 255, 255, 0.06)"
                                    } else if *s && i % 2 == 1 {
                                        "rgba(255, 255, 255, 0.03)"
                                    } else {
                                        "transparent"
                                    }
                                }
                            }
                        })
                        .children(row.into_iter().map(|cell| {
                            html!("td", {
                                .dwclass!("px-4 py-3 text-base glass-text-primary")
                                .child(cell)
                            })
                        }).collect::<Vec<_>>())
                    })
                }).collect::<Vec<_>>())
            }))
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
