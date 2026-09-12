use std::rc::Rc;

use dominator::Dom;
use dwind_dviz_core::data::{CategoryPoint, DataError, Extent, Series};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

use super::with_legend;
use crate::chart::{ChartProps, SeriesSlots, XDomain, YDomain, chart};
use crate::domains::{y_domain_for, y_domain_for_stacked};
use crate::layers::axis::{axis_x, axis_y};
use crate::layers::bars::{BarMode, BarOptions, bars_with};
use crate::layers::grid::grid;
use crate::layers::labels::bar_value_labels;
use crate::visibility::SeriesVisibility;

/// Converts typed category/value series into the numeric representation used
/// by [`bar_chart`], rejecting observations whose category is absent from the
/// supplied domain.
pub fn encode_category_series(
    categories: &[String],
    series: Vec<Series<CategoryPoint>>,
) -> Result<Vec<Series>, DataError> {
    series
        .into_iter()
        .map(|series| series.into_numeric(categories))
        .collect()
}

/// A column chart over categories. Bars grow from zero; a single series
/// takes slot 1 for every bar (identity, not value, drives color).
#[component(render_fn = bar_chart)]
struct BarChart {
    #[signal]
    #[default(vec![])]
    series: Vec<Series>,

    /// Category labels in display order. Use
    /// [`dwind_dviz_core::data::CategoryPoint::to_point`] or
    /// [`Series::into_numeric`] to validate typed category data first.
    #[signal]
    #[default(vec![])]
    categories: Vec<String>,

    #[signal]
    #[default(String::new())]
    label: String,

    #[default(240.0)]
    height: f64,

    #[default(BarMode::Grouped)]
    mode: BarMode,

    #[signal]
    #[default(None)]
    y: Option<Extent<f64>>,

    /// Values on the caps (dropped automatically past 16 bars per series).
    #[default(true)]
    value_labels: bool,

    #[default(true)]
    legend: bool,

    #[default(None)]
    slots: Option<Rc<SeriesSlots>>,

    /// Legend items toggle their series.
    #[default(true)]
    toggle_legend: bool,
}

pub fn bar_chart(props: BarChartProps) -> Dom {
    let BarChartProps {
        series,
        categories,
        label,
        height,
        mode,
        y,
        value_labels,
        legend,
        slots,
        toggle_legend,
        apply,
    } = props;

    let all_series = series.broadcast();
    let visibility = toggle_legend.then(SeriesVisibility::new);
    let series = match &visibility {
        Some(v) => v.filter(all_series.signal_cloned()).boxed_local(),
        None => all_series.signal_cloned().boxed_local(),
    }
    .broadcast();
    let slots = slots.unwrap_or_else(SeriesSlots::new);
    let x_domain = categories.map(XDomain::Band);
    let y_domain = map_ref! {
        let s = series.signal_cloned(),
        let y = y => match (*y, mode) {
            (Some(e), _) => YDomain::Linear(e),
            (None, BarMode::Grouped) => y_domain_for(s, true, 0.0),
            (None, BarMode::Stacked) => y_domain_for_stacked(s),
            (None, BarMode::StackedExpand) => YDomain::Linear(Extent::UNIT),
        }
    };
    let opts = BarOptions {
        mode,
        ..Default::default()
    };

    let chart = chart(
        ChartProps::new()
            .label_signal(label)
            .height(height)
            .slots(Some(slots.clone()))
            .x_domain_signal(x_domain)
            .y_domain_signal(y_domain)
            .layers({
                let mut layers = vec![
                    grid(),
                    axis_x(),
                    axis_y(),
                    bars_with(series.signal_cloned(), opts),
                ];
                if value_labels {
                    layers.push(bar_value_labels(series.signal_cloned(), opts));
                }
                layers
            })
            .apply(move |b| match apply {
                Some(a) => a(b),
                None => b,
            }),
    );
    with_legend(legend, all_series.signal_cloned(), slots, visibility, chart)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_categories_are_checked_before_bar_rendering() {
        let categories = vec!["low".to_string(), "high".to_string()];
        let series = vec![Series::new(
            "priority",
            "Priority",
            vec![CategoryPoint::new("high", 3.0)],
        )];
        let encoded = encode_category_series(&categories, series).unwrap();
        assert_eq!(encoded[0].points[0].x, 1.0);

        let invalid = vec![Series::new(
            "priority",
            "Priority",
            vec![CategoryPoint::new("urgent", 5.0)],
        )];
        assert!(encode_category_series(&categories, invalid).is_err());
    }
}
