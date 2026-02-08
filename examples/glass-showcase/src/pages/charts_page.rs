use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use dwind_glass::prelude::*;
use dwind_graph_data::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};

use crate::helpers::{section, subsection};

fn rand() -> f64 {
    js_sys::Math::random()
}

fn rand_range(min: f64, max: f64) -> f64 {
    min + rand() * (max - min)
}

const LINE_CHART_CODE: &str = r#"// Live data — chart built once, paths animate via CSS transitions
let series = Mutable::new(initial_series());

// Chart subscribes to the Mutable; SVG stays stable, paths morph smoothly
glass_line_chart!({
    .live_series(Some(series.clone()))
    .interactive(true)
    .smooth(true)
    .area(true)
})

// Push new points every 500ms
.future(async move {
    loop {
        gloo_timers::future::TimeoutFuture::new(500).await;
        series.set(generate_next_frame());
    }
})"#;

const BAR_CHART_CODE: &str = r#"let data = Mutable::new(make_bar_series());

// Randomize on button click
glass_button!({
    .on_click(clone!(data => move |_| {
        data.set(make_random_bar_series());
    }))
})

// Chart re-renders reactively
.child_signal(data.signal_cloned().map(|series| {
    Some(glass_bar_chart!({ .series(series) }))
}))"#;

const PIE_CHART_CODE: &str = r#"let slices = Mutable::new(make_slices());

// Values shift every 2 seconds
.future(async move {
    loop {
        gloo_timers::future::TimeoutFuture::new(2000).await;
        slices.set(make_random_slices());
    }
})"#;

const SPARKLINE_CODE: &str = r#"// Stat card values update in real time
let value = Mutable::new(initial_value);

.future(async move {
    loop {
        gloo_timers::future::TimeoutFuture::new(1500).await;
        value.lock_mut().push_and_shift(new_point);
    }
})"#;

// ---------------------------------------------------------------------------
// Live streaming line chart data
// ---------------------------------------------------------------------------

struct LiveLineData {
    series_a: Vec<DataPoint>,
    series_b: Vec<DataPoint>,
    tick: f64,
    window: usize,
}

impl LiveLineData {
    fn new(window: usize) -> Self {
        let mut data = Self {
            series_a: Vec::new(),
            series_b: Vec::new(),
            tick: 0.0,
            window,
        };
        // Seed with initial points
        for _ in 0..window {
            data.push();
        }
        data
    }

    fn push(&mut self) {
        let t = self.tick;
        let cpu = 35.0 + 25.0 * (t * 0.15).sin() + 10.0 * (t * 0.4).cos() + rand_range(-5.0, 5.0);
        let mem = 55.0 + 15.0 * (t * 0.08).sin() + 8.0 * (t * 0.25).cos() + rand_range(-3.0, 3.0);

        self.series_a.push(DataPoint::new(t, cpu.clamp(5.0, 95.0)));
        self.series_b.push(DataPoint::new(t, mem.clamp(20.0, 95.0)));
        self.tick += 1.0;

        // Trim to window
        if self.series_a.len() > self.window {
            self.series_a.remove(0);
            self.series_b.remove(0);
        }
    }

    fn to_series(&self) -> Vec<Series> {
        vec![
            Series::new("CPU Usage %", self.series_a.clone(), 0),
            Series::new("Memory Usage %", self.series_b.clone(), 1),
        ]
    }
}

impl Clone for LiveLineData {
    fn clone(&self) -> Self {
        Self {
            series_a: self.series_a.clone(),
            series_b: self.series_b.clone(),
            tick: self.tick,
            window: self.window,
        }
    }
}

// ---------------------------------------------------------------------------
// Live sparkline + stat card data
// ---------------------------------------------------------------------------

struct LiveStatData {
    values: Vec<f64>,
    current: f64,
    prev: f64,
    window: usize,
}

impl LiveStatData {
    fn new(base: f64, variance: f64, window: usize) -> Self {
        let mut values = Vec::new();
        let mut v = base;
        for _ in 0..window {
            v += rand_range(-variance, variance);
            v = v.max(base * 0.5);
            values.push(v);
        }
        let current = *values.last().unwrap();
        let prev = values[values.len().saturating_sub(3).max(0)];
        Self {
            values,
            current,
            prev,
            window,
        }
    }

    fn push(&mut self, variance: f64) {
        self.prev = self.current;
        self.current += rand_range(-variance, variance * 1.5);
        self.current = self.current.max(1.0);
        self.values.push(self.current);
        if self.values.len() > self.window {
            self.values.remove(0);
        }
    }

    fn trend_pct(&self) -> f64 {
        if self.prev == 0.0 {
            return 0.0;
        }
        ((self.current - self.prev) / self.prev) * 100.0
    }
}

impl Clone for LiveStatData {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            current: self.current,
            prev: self.prev,
            window: self.window,
        }
    }
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

pub fn charts_page() -> Dom {
    // ---- Live line chart state ----
    // A Mutable<Vec<Series>> that the chart subscribes to via `live_series`.
    // The chart's SVG structure stays stable; only the path `d` attributes
    // transition smoothly via CSS.
    let initial_data = LiveLineData::new(60);
    let live_series: Mutable<Vec<Series>> = Mutable::new(initial_data.to_series());

    // ---- Bar chart state ----
    let bar_series: Mutable<Vec<Series>> = Mutable::new(make_bar_series());
    let bar_categories: Vec<String> = vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()];

    let stacked_series: Mutable<Vec<Series>> = Mutable::new(make_stacked_series());
    let stacked_categories: Vec<String> =
        vec!["Jan".into(), "Feb".into(), "Mar".into(), "Apr".into(), "May".into()];

    // ---- Pie chart state ----
    let pie_slices: Mutable<Vec<PieSlice>> = Mutable::new(make_platform_slices());
    let donut_slices: Mutable<Vec<PieSlice>> = Mutable::new(make_browser_slices());

    // ---- Stat card / sparkline state ----
    let stat_revenue = Mutable::new(LiveStatData::new(42.0, 3.0, 12));
    let stat_users = Mutable::new(LiveStatData::new(1600.0, 80.0, 12));
    let stat_bounce = Mutable::new(LiveStatData::new(38.0, 2.5, 12));

    html!("div", {
        .dwclass!("flex flex-col gap-8")

        // Page header
        .child(html!("div", {
            .dwclass!("mb-4")
            .child(html!("h1", {
                .dwclass!("text-3xl font-bold glass-text-primary mb-2")
                .text("Charts & Visualization")
            }))
            .child(html!("p", {
                .dwclass!("text-base glass-text-secondary")
                .text("Live-updating, SVG-rendered charts. Hover for tooltips, scroll to zoom, drag to pan, shift+drag to brush-select.")
            }))
        }))

        // =====================================================================
        // LINE CHART — live streaming
        // =====================================================================
        .child(section("Line Chart", "section-line-chart", Some(LINE_CHART_CODE), vec![
            subsection("Live server metrics (updates every 500ms)", vec![
                html!("div", {
                    .style("background", "var(--glass-bg-elevated)")
                    .style("border-radius", "var(--glass-border-radius-xl)")
                    .style("padding", "1rem")
                    .style("border", "1px solid var(--glass-border-color)")

                    // Chart built ONCE — SVG stays in DOM, paths animate via CSS `d` transitions.
                    .child(glass_line_chart!({
                        .live_series(Some(live_series.clone()))
                        .show_grid(true)
                        .interactive(true)
                        .smooth(true)
                        .area(true)
                        .x_label(Some("Time (ticks)".to_string()))
                        .y_label(Some("Usage %".to_string()))
                    }))

                    // Push a new data point every 500ms
                    .future(clone!(live_series => async move {
                        let mut data = LiveLineData::new(60);
                        loop {
                            gloo_timers::future::TimeoutFuture::new(500).await;
                            data.push();
                            live_series.set(data.to_series());
                        }
                    }))
                }),
            ]),
        ]))

        // =====================================================================
        // BAR CHART — randomize on click
        // =====================================================================
        .child(section("Bar Chart", "section-bar-chart", Some(BAR_CHART_CODE), vec![
            subsection("Grouped bars — click to randomize", vec![
                html!("div", {
                    .dwclass!("flex flex-col gap-3")

                    .child(html!("div", {
                        .dwclass!("flex gap-2")
                        .child(glass_button!({
                            .content(Some(html!("span", { .text("Shuffle Data") })))
                            .on_click(clone!(bar_series => move |_: events::Click| {
                                bar_series.set(make_bar_series());
                            }))
                        }))
                    }))

                    .child(html!("div", {
                        .style("background", "var(--glass-bg-elevated)")
                        .style("border-radius", "var(--glass-border-radius-xl)")
                        .style("padding", "1rem")
                        .style("border", "1px solid var(--glass-border-color)")

                        .child_signal({
                            let cats = bar_categories.clone();
                            bar_series.signal_cloned().map(move |series| {
                                Some(glass_bar_chart!({
                                    .series(series)
                                    .categories(cats.clone())
                                    .mode(BarMode::Grouped)
                                    .show_grid(true)
                                    .interactive(true)
                                }))
                            })
                        })
                    }))
                }),
            ]),
            subsection("Stacked bars — click to randomize", vec![
                html!("div", {
                    .dwclass!("flex flex-col gap-3")

                    .child(html!("div", {
                        .dwclass!("flex gap-2")
                        .child(glass_button!({
                            .content(Some(html!("span", { .text("Shuffle Data") })))
                            .on_click(clone!(stacked_series => move |_: events::Click| {
                                stacked_series.set(make_stacked_series());
                            }))
                        }))
                    }))

                    .child(html!("div", {
                        .style("background", "var(--glass-bg-elevated)")
                        .style("border-radius", "var(--glass-border-radius-xl)")
                        .style("padding", "1rem")
                        .style("border", "1px solid var(--glass-border-color)")

                        .child_signal({
                            let cats = stacked_categories.clone();
                            stacked_series.signal_cloned().map(move |series| {
                                Some(glass_bar_chart!({
                                    .series(series)
                                    .categories(cats.clone())
                                    .mode(BarMode::Stacked)
                                    .show_grid(true)
                                    .interactive(true)
                                }))
                            })
                        })
                    }))
                }),
            ]),
        ]))

        // =====================================================================
        // PIE & DONUT — auto-shifting values
        // =====================================================================
        .child(section("Pie & Donut", "section-pie-chart", Some(PIE_CHART_CODE), vec![
            subsection("Pie chart — values shift every 2s", vec![
                html!("div", {
                    .dwclass!("flex flex-wrap items-start justify-center gap-8")
                    .style("background", "var(--glass-bg-elevated)")
                    .style("border-radius", "var(--glass-border-radius-xl)")
                    .style("padding", "1.5rem")
                    .style("border", "1px solid var(--glass-border-color)")

                    // Chart built ONCE — slices morph smoothly via CSS `d` transitions.
                    .child(glass_pie_chart!({
                        .live_slices(Some(pie_slices.clone()))
                        .size(260.0)
                        .inner_radius(0.0)
                        .show_labels(true)
                        .show_legend(true)
                        .interactive(true)
                    }))

                    .future(clone!(pie_slices => async move {
                        loop {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            pie_slices.set(make_platform_slices());
                        }
                    }))
                }),
            ]),
            subsection("Donut chart — values shift every 2s", vec![
                html!("div", {
                    .dwclass!("flex flex-wrap items-start justify-center gap-8")
                    .style("background", "var(--glass-bg-elevated)")
                    .style("border-radius", "var(--glass-border-radius-xl)")
                    .style("padding", "1.5rem")
                    .style("border", "1px solid var(--glass-border-color)")

                    // Chart built ONCE — slices morph smoothly via CSS `d` transitions.
                    .child(glass_pie_chart!({
                        .live_slices(Some(donut_slices.clone()))
                        .size(260.0)
                        .inner_radius(0.6)
                        .show_labels(true)
                        .show_legend(true)
                        .interactive(true)
                    }))

                    .future(clone!(donut_slices => async move {
                        loop {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            donut_slices.set(make_browser_slices());
                        }
                    }))
                }),
            ]),
        ]))

        // =====================================================================
        // SPARKLINES — live stat cards
        // =====================================================================
        .child(section("Sparkline", "section-sparkline", Some(SPARKLINE_CODE), vec![
            subsection("Live stat cards (update every 1.5s)", vec![
                html!("div", {
                    .dwclass!("grid gap-4")
                    .style("grid-template-columns", "repeat(auto-fit, minmax(220px, 1fr))")

                    // Revenue card
                    .child_signal(stat_revenue.signal_cloned().map(|data| {
                        let trend = data.trend_pct();
                        let trend_dir = if trend >= 0.0 {
                            StatTrend::Up(trend.abs())
                        } else {
                            StatTrend::Down(trend.abs())
                        };
                        Some(glass_stat_card!({
                            .label("Revenue".to_string())
                            .value(format!("${:.1}k", data.current))
                            .trend(Some(trend_dir))
                            .icon(Some(glass_sparkline!({
                                .data(data.values.clone())
                                .width(80.0)
                                .height(24.0)
                                .area(true)
                            })))
                        }))
                    }))

                    // Users card
                    .child_signal(stat_users.signal_cloned().map(|data| {
                        let trend = data.trend_pct();
                        let trend_dir = if trend >= 0.0 {
                            StatTrend::Up(trend.abs())
                        } else {
                            StatTrend::Down(trend.abs())
                        };
                        Some(glass_stat_card!({
                            .label("Active Users".to_string())
                            .value(format!("{:.0}", data.current))
                            .trend(Some(trend_dir))
                            .icon(Some(glass_sparkline!({
                                .data(data.values.clone())
                                .width(80.0)
                                .height(24.0)
                                .area(true)
                                .color(Some("var(--glass-success)".to_string()))
                            })))
                        }))
                    }))

                    // Bounce rate card
                    .child_signal(stat_bounce.signal_cloned().map(|data| {
                        let trend = data.trend_pct();
                        // For bounce rate, down is good
                        let trend_dir = if trend <= 0.0 {
                            StatTrend::Down(trend.abs())
                        } else {
                            StatTrend::Up(trend.abs())
                        };
                        Some(glass_stat_card!({
                            .label("Bounce Rate".to_string())
                            .value(format!("{:.1}%", data.current))
                            .trend(Some(trend_dir))
                            .icon(Some(glass_sparkline!({
                                .data(data.values.clone())
                                .width(80.0)
                                .height(24.0)
                                .area(true)
                                .color(Some("var(--glass-warning)".to_string()))
                            })))
                        }))
                    }))

                    // Periodic update for all stat cards
                    .future(clone!(stat_revenue, stat_users, stat_bounce => async move {
                        loop {
                            gloo_timers::future::TimeoutFuture::new(1500).await;
                            stat_revenue.lock_mut().push(3.0);
                            stat_users.lock_mut().push(80.0);
                            stat_bounce.lock_mut().push(2.0);
                        }
                    }))
                }),
            ]),
            subsection("Standalone variants", vec![
                html!("div", {
                    .dwclass!("flex flex-wrap items-center gap-6")
                    .style("background", "var(--glass-bg-elevated)")
                    .style("border-radius", "var(--glass-border-radius-xl)")
                    .style("padding", "1rem 1.5rem")
                    .style("border", "1px solid var(--glass-border-color)")

                    .child(html!("div", {
                        .dwclass!("flex flex-col gap-1 items-center")
                        .child(glass_sparkline!({
                            .data(vec![5.0, 8.0, 6.0, 12.0, 9.0, 15.0, 18.0, 14.0, 20.0])
                            .width(120.0)
                            .height(32.0)
                            .area(true)
                        }))
                        .child(html!("span", {
                            .dwclass!("text-xs glass-text-tertiary")
                            .text("Default (accent)")
                        }))
                    }))

                    .child(html!("div", {
                        .dwclass!("flex flex-col gap-1 items-center")
                        .child(glass_sparkline!({
                            .data(vec![20.0, 18.0, 22.0, 16.0, 14.0, 18.0, 12.0, 10.0])
                            .width(120.0)
                            .height(32.0)
                            .area(false)
                            .color(Some("var(--glass-error)".to_string()))
                        }))
                        .child(html!("span", {
                            .dwclass!("text-xs glass-text-tertiary")
                            .text("Line only (error)")
                        }))
                    }))

                    .child(html!("div", {
                        .dwclass!("flex flex-col gap-1 items-center")
                        .child(glass_sparkline!({
                            .data(vec![2.0, 4.0, 3.0, 7.0, 5.0, 8.0, 6.0, 10.0, 9.0, 12.0])
                            .width(120.0)
                            .height(32.0)
                            .area(true)
                            .color(Some("var(--glass-success)".to_string()))
                        }))
                        .child(html!("span", {
                            .dwclass!("text-xs glass-text-tertiary")
                            .text("Area (success)")
                        }))
                    }))
                }),
            ]),
        ]))
    })
}

// ---------------------------------------------------------------------------
// Data generators (randomized each call)
// ---------------------------------------------------------------------------

fn make_bar_series() -> Vec<Series> {
    vec![
        Series::new(
            "2024",
            (0..4)
                .map(|i| DataPoint::new(i as f64, rand_range(30.0, 70.0)))
                .collect(),
            0,
        ),
        Series::new(
            "2025",
            (0..4)
                .map(|i| DataPoint::new(i as f64, rand_range(35.0, 80.0)))
                .collect(),
            1,
        ),
    ]
}

fn make_stacked_series() -> Vec<Series> {
    vec![
        Series::new(
            "Product A",
            (0..5)
                .map(|i| DataPoint::new(i as f64, rand_range(20.0, 50.0)))
                .collect(),
            0,
        ),
        Series::new(
            "Product B",
            (0..5)
                .map(|i| DataPoint::new(i as f64, rand_range(15.0, 35.0)))
                .collect(),
            1,
        ),
        Series::new(
            "Product C",
            (0..5)
                .map(|i| DataPoint::new(i as f64, rand_range(8.0, 25.0)))
                .collect(),
            2,
        ),
    ]
}

fn make_platform_slices() -> Vec<PieSlice> {
    vec![
        PieSlice::new("Desktop", rand_range(35.0, 55.0), 0),
        PieSlice::new("Mobile", rand_range(25.0, 45.0), 1),
        PieSlice::new("Tablet", rand_range(8.0, 20.0), 2),
        PieSlice::new("Other", rand_range(2.0, 8.0), 3),
    ]
}

fn make_browser_slices() -> Vec<PieSlice> {
    vec![
        PieSlice::new("Chrome", rand_range(55.0, 70.0), 0),
        PieSlice::new("Safari", rand_range(14.0, 24.0), 1),
        PieSlice::new("Firefox", rand_range(6.0, 14.0), 2),
        PieSlice::new("Edge", rand_range(3.0, 8.0), 4),
        PieSlice::new("Other", rand_range(1.0, 5.0), 5),
    ]
}
