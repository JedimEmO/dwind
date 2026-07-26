use crate::theme::prelude::*;
use dominator::{clone, events, html, svg, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn toggled(self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }
}

/// A column definition for [`data_table`]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TableColumn {
    /// Stable key identifying the column in sort callbacks
    pub key: String,
    /// Header label
    pub label: String,
    /// Whether the header offers sorting
    pub sortable: bool,
}

impl TableColumn {
    pub fn new(key: impl Into<String>, label: impl Into<String>, sortable: bool) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            sortable,
        }
    }
}

/// A themed data table with sortable column headers.
///
/// Sorting is controlled: clicking a sortable header invokes `on_sort` with
/// the column key and the requested direction, and the current order is
/// reflected back through the `sorted_by` signal (exposed to assistive
/// technology via `aria-sort`). Rows are `(id, cells)` pairs; clicking a row
/// invokes `on_row_click` with its id.
#[component(render_fn = data_table)]
struct DataTable {
    #[signal_vec]
    #[default(vec![])]
    columns: TableColumn,

    #[signal_vec]
    #[default(vec![])]
    rows: (String, Vec<Dom>),

    #[signal]
    #[default(None)]
    sorted_by: Option<(String, SortDirection)>,

    #[default(Box::new(|_, _| {}))]
    on_sort: dyn Fn(String, SortDirection) + 'static,

    #[default(Box::new(|_| {}))]
    on_row_click: dyn Fn(String) + 'static,

    /// Set when row clicks are meaningful; adds hover/cursor affordances
    #[default(false)]
    rows_clickable: bool,
}

pub fn data_table(props: DataTableProps) -> Dom {
    let DataTableProps {
        columns,
        rows,
        sorted_by,
        on_sort,
        on_row_click,
        rows_clickable,
        apply,
    } = props;

    let on_sort = Rc::new(on_sort);
    let on_row_click = Rc::new(on_row_click);
    let sorted_by = sorted_by.broadcast();
    let sorted_state: Mutable<Option<(String, SortDirection)>> = Mutable::new(None);

    html!("div", {
        .future(sorted_by.signal_cloned().for_each(clone!(sorted_state => move |v| {
            sorted_state.set(v);
            async {}
        })))
        .dwclass!("rounded-lg border overflow-x-auto w-full")
        .dwclass!("dwui-border-void-700 is(.light *):dwui-border-void-300")
        .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-100")
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .child(html!("table", {
            .dwclass!("w-full text-sm text-left border-collapse")
            .child(html!("thead", {
                .child(html!("tr", {
                    .dwclass!("border-b dwui-border-void-700 is(.light *):dwui-border-void-300")
                    .children_signal_vec(columns.map(clone!(sorted_by, sorted_state, on_sort => move |column| {
                        let column_sort = sorted_by
                            .signal_cloned()
                            .map(clone!(column => move |sorted| {
                                sorted.and_then(|(key, direction)| {
                                    (key == column.key).then_some(direction)
                                })
                            }))
                            .broadcast();

                        html!("th", {
                            .attr("scope", "col")
                            .attr_signal("aria-sort", column_sort.signal().map(|direction| {
                                direction.map(|direction| match direction {
                                    SortDirection::Ascending => "ascending",
                                    SortDirection::Descending => "descending",
                                })
                            }))
                            .dwclass!("p-0")
                            .apply(clone!(column, on_sort, sorted_state => move |b| {
                                if column.sortable {
                                    b.child(html!("button", {
                                        .attr("type", "button")
                                        .dwclass!("flex flex-row align-items-center gap-1 w-full p-3 cursor-pointer text-left")
                                        .dwclass!("bg-transparent border-none font-bold text-sm")
                                        .dwclass!("dwui-text-on-primary-200 hover:dwui-text-primary-300")
                                        .dwclass!("is(.light *):dwui-text-on-primary-800 is(.light *):hover:dwui-text-primary-700")
                                        .dwclass!("transition-colors dwui-focusable")
                                        .child(html!("span", { .text(&column.label) }))
                                        .child(html!("span", {
                                            .dwclass!("inline-flex align-items-center")
                                            .style_signal("opacity", column_sort.signal().map(|direction| {
                                                if direction.is_some() { "1" } else { "0.4" }
                                            }))
                                            .child(svg!("svg", {
                                            .attr("viewBox", "0 0 10 12")
                                            .attr("width", "10")
                                            .attr("height", "12")
                                            .attr("fill", "none")
                                            .attr("aria-hidden", "true")
                                            .child(svg!("path", {
                                                .attr_signal("d", column_sort.signal().map(|direction| {
                                                    match direction {
                                                        Some(SortDirection::Ascending) => "M5 2 L5 10 M2 5 L5 2 L8 5",
                                                        Some(SortDirection::Descending) => "M5 2 L5 10 M2 7 L5 10 L8 7",
                                                        None => "M5 1 L5 5 M3 3 L5 1 L7 3 M5 7 L5 11 M3 9 L5 11 L7 9",
                                                    }
                                                }))
                                                .attr("stroke", "currentColor")
                                                .attr("stroke-width", "1.2")
                                                .attr("stroke-linecap", "round")
                                                .attr("stroke-linejoin", "round")
                                            }))
                                            }))
                                        }))
                                        .event(clone!(on_sort, column, sorted_state => move |_: events::Click| {
                                            let next = sorted_state
                                                .get_cloned()
                                                .and_then(|(key, direction)| {
                                                    (key == column.key).then(|| direction.toggled())
                                                })
                                                .unwrap_or(SortDirection::Ascending);

                                            (on_sort)(column.key.clone(), next);
                                        }))
                                    }))
                                } else {
                                    b.child(html!("div", {
                                        .dwclass!("p-3 font-bold")
                                        .dwclass!("dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-800")
                                        .text(&column.label)
                                    }))
                                }
                            }))
                        })
                    })))
                }))
            }))
            .child(html!("tbody", {
                .children_signal_vec(rows.map(clone!(on_row_click => move |(row_id, cells)| {
                    html!("tr", {
                        .dwclass!("border-b dwui-border-void-800 is(.light *):dwui-border-void-200 transition-colors")
                        .dwclass!("hover:dwui-bg-void-800 is(.light *):hover:dwui-bg-void-200")
                        .apply_if(rows_clickable, clone!(on_row_click, row_id => move |b| {
                            dwclass!(b, "cursor-pointer")
                                .event(clone!(on_row_click, row_id => move |_: events::Click| {
                                    (on_row_click)(row_id.clone());
                                }))
                        }))
                        .children(cells.into_iter().map(|cell| {
                            html!("td", {
                                .dwclass!("p-3")
                                .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700")
                                .child(cell)
                            })
                        }))
                    })
                })))
            }))
        }))
    })
}
