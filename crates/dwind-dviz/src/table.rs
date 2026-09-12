//! Accessible tabular fallback for charts.
//!
//! A chart is excellent for pattern recognition but a poor primary interface
//! for exact values. [`table_view`] exposes the same series as a compact,
//! keyboard-friendly `dwui` table so applications can place it beside the
//! visual chart or reveal it from an accessibility/details control.

use dominator::{Dom, html};
use dwind_dviz_core::data::Series;
use dwind_dviz_core::format;
use dwind_dviz_core::stats;
use futures_signals::signal::{Signal, SignalExt};

use dwui::prelude::{DataTableProps, TableColumn, data_table};

/// Renders one row per series with its latest finite value and finite extent.
///
/// The source is deliberately just a `Signal<Vec<Series>>`: callers can pass
/// a static signal, a `MutableSource`, or a live `WindowedSource` without
/// changing the accessible representation.
pub fn table_view<S>(series: S) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    let columns = futures_signals::signal::always(vec![
        TableColumn::new("series", "Series", false),
        TableColumn::new("latest", "Latest", false),
        TableColumn::new("minimum", "Minimum", false),
        TableColumn::new("maximum", "Maximum", false),
        TableColumn::new("points", "Points", false),
    ])
    .to_signal_vec();
    let rows = series
        .map(|all| {
            all.into_iter()
                .map(|s| {
                    let values = s.points.iter().filter(|p| p.y.is_finite()).map(|p| p.y);
                    let values: Vec<f64> = values.collect();
                    let extent = stats::extent(values.iter().copied());
                    let latest = values.last().copied();
                    let cell = |value: String| html!("span", { .text(&value) });
                    (
                        s.id,
                        vec![
                            cell(s.label),
                            cell(latest.map(format::compact).unwrap_or_else(|| "—".into())),
                            cell(
                                extent
                                    .map(|e| format::compact(e.min))
                                    .unwrap_or_else(|| "—".into()),
                            ),
                            cell(
                                extent
                                    .map(|e| format::compact(e.max))
                                    .unwrap_or_else(|| "—".into()),
                            ),
                            cell(values.len().to_string()),
                        ],
                    )
                })
                .collect::<Vec<_>>()
        })
        .to_signal_vec();
    data_table(
        DataTableProps::new()
            .columns_signal_vec(columns)
            .rows_signal_vec(rows),
    )
}

/// Renders one row per observation for an exact-value fallback.
///
/// Unlike [`table_view`], this does not aggregate values. It is intended for
/// inspection, export-adjacent workflows, and users who need every plotted
/// observation rather than a series summary.
pub fn point_table_view<S>(series: S) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
{
    let columns = futures_signals::signal::always(vec![
        TableColumn::new("series", "Series", false),
        TableColumn::new("x", "X", false),
        TableColumn::new("y", "Y", false),
    ])
    .to_signal_vec();
    let rows = series
        .map(|all| {
            all.into_iter()
                .flat_map(|s| {
                    s.points.into_iter().enumerate().map(move |(index, point)| {
                        let cell = |value: String| html!("span", { .text(&value) });
                        (
                            format!("{}-{index}", s.id),
                            vec![
                                cell(s.label.clone()),
                                cell(format::compact(point.x)),
                                cell(if point.y.is_finite() {
                                    format::compact(point.y)
                                } else {
                                    "—".into()
                                }),
                            ],
                        )
                    })
                })
                .collect::<Vec<_>>()
        })
        .to_signal_vec();
    data_table(
        DataTableProps::new()
            .columns_signal_vec(columns)
            .rows_signal_vec(rows),
    )
}
