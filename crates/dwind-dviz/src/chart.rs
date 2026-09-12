//! The chart container: size, domains, scales, and the layer contract.
//!
//! [`chart`] renders a block element that observes its own size, computes a
//! [`Frame`] (plot rectangle plus resolved x and y scales) whenever the size
//! or a domain changes, and hands every layer a [`ChartContext`] to read that
//! frame from. Layers never measure the DOM themselves.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use dominator::{Dom, clone, html, svg};
use dwind_dviz_core::data::Extent;
use dwind_dviz_core::format;
use dwind_dviz_core::jiff::Timestamp;
use dwind_dviz_core::jiff::tz::TimeZone;
use dwind_dviz_core::layout::{Margins, Rect, estimate_max_text_width};
use dwind_dviz_core::scale::{BandScale, ContinuousScale, LinearScale, LogScale, TimeScale};
use dwind_dviz_core::ticks::TimeTicks;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt, always};
use futures_signals::signal_vec::SignalVecExt;
use futures_signals_component_macro::component;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlElement, ResizeObserver, ResizeObserverEntry};

/// Axis label font size in px; margins are reserved from it.
pub const AXIS_FONT_SIZE: f64 = 11.0;

/// What the x axis measures. Resolved to an [`XScale`] once a size is known.
#[derive(Debug, Clone, PartialEq)]
pub enum XDomain {
    Linear(Extent<f64>),
    /// Timestamps; ticks snap to calendar boundaries in `tz`.
    Time {
        extent: Extent<Timestamp>,
        tz: TimeZone,
    },
    /// Categories in display order. Points on a band axis carry the
    /// category *index* as their x.
    Band(Vec<String>),
}

impl XDomain {
    pub fn time(extent: Extent<Timestamp>) -> Self {
        Self::Time {
            extent,
            tz: TimeZone::UTC,
        }
    }

    pub fn band(categories: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Band(categories.into_iter().map(Into::into).collect())
    }
}

/// What the y axis measures.
#[derive(Debug, Clone, PartialEq)]
pub enum YDomain {
    Linear(Extent<f64>),
    /// Strictly positive extent; non-positive values become gaps.
    Log(Extent<f64>),
    /// Categories top to bottom, for heatmaps. Values carry the index.
    Band(Vec<String>),
}

impl YDomain {
    pub fn band(categories: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Band(categories.into_iter().map(Into::into).collect())
    }
}

/// A resolved tick: pixel position along the axis and its label.
#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    pub position: f64,
    pub label: String,
}

/// The x scale with its pixel range applied.
#[derive(Debug, Clone, PartialEq)]
pub enum XScale {
    Linear(LinearScale),
    Time(TimeScale),
    Band(BandScale),
}

impl XScale {
    /// Pixel x for a data x. For a band scale the value is the category
    /// index and maps to the band centre. Non-finite in, NaN out.
    pub fn map(&self, x: f64) -> f64 {
        if !x.is_finite() {
            return f64::NAN;
        }
        match self {
            XScale::Linear(s) => s.map(x),
            XScale::Time(s) => s.map_millis(x),
            XScale::Band(s) => s
                .map_index(x.round() as usize)
                .map(|p| p + s.bandwidth() / 2.0)
                .unwrap_or(f64::NAN),
        }
    }

    /// Data x nearest to pixel `px`, for hover. A band scale returns the
    /// category index.
    pub fn invert(&self, px: f64) -> f64 {
        match self {
            XScale::Linear(s) => s.invert(px),
            XScale::Time(s) => s.invert(px).as_millisecond() as f64,
            XScale::Band(s) => s
                .invert(px)
                .and_then(|c| s.index_of(c))
                .map_or(f64::NAN, |i| i as f64),
        }
    }

    /// A tooltip title for a data x: the category, a full timestamp, or the
    /// number.
    pub fn describe(&self, x: f64) -> String {
        match self {
            XScale::Linear(_) => format::compact(x),
            XScale::Time(s) => Timestamp::from_millisecond(x.round() as i64)
                .map(|t| format::datetime(t, &s.tz))
                .unwrap_or_default(),
            XScale::Band(s) => s
                .categories()
                .get(x.round() as usize)
                .cloned()
                .unwrap_or_default(),
        }
    }

    pub fn ticks(&self, count: usize) -> Vec<Tick> {
        match self {
            XScale::Linear(s) => {
                let step = s.tick_step(count);
                s.ticks(count)
                    .into_iter()
                    .map(|v| Tick {
                        position: s.map(v),
                        label: format::tick(v, step),
                    })
                    .collect()
            }
            XScale::Time(s) => {
                let TimeTicks { interval, ticks } = s.time_ticks(count);
                let labels = format::time_ticks(&ticks, interval, &s.tz);
                ticks
                    .into_iter()
                    .zip(labels)
                    .map(|(t, label)| Tick {
                        position: s.map(t),
                        label,
                    })
                    .collect()
            }
            XScale::Band(s) => {
                // Show every category, or every k-th when they would collide.
                let n = s.len();
                let stride = (n as f64 / count.max(1) as f64).ceil().max(1.0) as usize;
                s.categories()
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % stride == 0)
                    .map(|(i, c)| Tick {
                        position: s.map_index(i).unwrap() + s.bandwidth() / 2.0,
                        label: c.clone(),
                    })
                    .collect()
            }
        }
    }
}

/// The y scale with its pixel range applied.
#[derive(Debug, Clone, PartialEq)]
pub enum YScale {
    Linear(LinearScale),
    Log(LogScale),
    Band(BandScale),
}

impl YScale {
    /// Pixel y for a data y. Non-finite (or non-positive on a log scale)
    /// yields NaN, which every mark treats as a gap.
    pub fn map(&self, y: f64) -> f64 {
        if !y.is_finite() {
            return f64::NAN;
        }
        match self {
            YScale::Linear(s) => s.map(y),
            YScale::Log(s) => {
                if y > 0.0 {
                    s.map(y)
                } else {
                    f64::NAN
                }
            }
            YScale::Band(s) => s
                .map_index(y.round() as usize)
                .map(|p| p + s.bandwidth() / 2.0)
                .unwrap_or(f64::NAN),
        }
    }

    pub fn invert(&self, px: f64) -> f64 {
        match self {
            YScale::Linear(s) => s.invert(px),
            YScale::Log(s) => s.invert(px),
            YScale::Band(s) => s
                .invert(px)
                .and_then(|c| s.index_of(c))
                .map_or(f64::NAN, |i| i as f64),
        }
    }

    pub fn domain(&self) -> Extent<f64> {
        match self {
            YScale::Linear(s) => s.domain,
            YScale::Log(s) => s.domain,
            YScale::Band(s) => Extent::new(0.0, s.len().saturating_sub(1) as f64),
        }
    }

    /// The band scale, when the axis is categorical.
    pub fn band(&self) -> Option<&BandScale> {
        match self {
            YScale::Band(s) => Some(s),
            _ => None,
        }
    }

    pub fn ticks(&self, count: usize) -> Vec<Tick> {
        match self {
            YScale::Linear(s) => {
                let step = s.tick_step(count);
                s.ticks(count)
                    .into_iter()
                    .map(|v| Tick {
                        position: s.map(v),
                        label: format::tick(v, step),
                    })
                    .collect()
            }
            YScale::Log(s) => s
                .ticks(count)
                .into_iter()
                .map(|v| Tick {
                    position: s.map(v),
                    label: format::si(v, 3),
                })
                .collect(),
            YScale::Band(s) => s
                .categories()
                .iter()
                .enumerate()
                .map(|(i, c)| Tick {
                    position: s.map_index(i).unwrap() + s.bandwidth() / 2.0,
                    label: c.clone(),
                })
                .collect(),
        }
    }

    /// Pixel y of the baseline bars grow from: zero when it is inside the
    /// domain, otherwise the nearer edge.
    pub fn baseline(&self) -> f64 {
        match self {
            YScale::Linear(s) => s.clamped().map(0.0),
            YScale::Log(s) => s.range.0,
            YScale::Band(s) => s.range.1,
        }
    }
}

/// Everything a layer needs to place marks: the outer size, the plot
/// rectangle, the two scales and the tick counts the container chose.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub outer: Rect,
    pub plot: Rect,
    pub x: XScale,
    pub y: YScale,
    pub x_tick_count: usize,
    pub y_tick_count: usize,
}

impl Frame {
    /// A zero-size frame for the instant before the container is measured.
    pub fn empty() -> Self {
        Self::compute(
            Rect::from_size(0.0, 0.0),
            &XDomain::Linear(Extent::UNIT),
            &YDomain::Linear(Extent::UNIT),
            true,
            None,
        )
    }

    pub fn x_ticks(&self) -> Vec<Tick> {
        self.x.ticks(self.x_tick_count)
    }

    /// Shrinks the plot from the right by `extra` px (when `apply`) and
    /// re-ranges the x scale.
    pub fn with_extra_right(mut self, extra: f64, apply: bool) -> Self {
        if apply && extra > 0.0 {
            self.plot.width = (self.plot.width - extra).max(0.0);
            self.x = resolve_x_range(self.x, self.plot.x_range());
        }
        self
    }

    pub fn y_ticks(&self) -> Vec<Tick> {
        self.y.ticks(self.y_tick_count)
    }

    /// Lays out the plot inside `outer`: reserves margins for the axes
    /// (sized from the widest y label) and ranges both scales.
    pub fn compute(
        outer: Rect,
        x: &XDomain,
        y: &YDomain,
        nice_y: bool,
        margins: Option<Margins>,
    ) -> Self {
        let y_tick_count = ((outer.height / 40.0).floor() as usize).clamp(2, 10);
        let x_tick_count = ((outer.width / 80.0).floor() as usize).clamp(2, 12);

        // Range the y scale on a provisional plot to learn its labels.
        let mut y_scale = resolve_y(y, (0.0, 1.0), nice_y);
        let margins = margins.unwrap_or_else(|| {
            let labels = y_scale.ticks(y_tick_count);
            let widest =
                estimate_max_text_width(labels.iter().map(|t| t.label.as_str()), AXIS_FONT_SIZE);
            Margins::for_axes(widest, AXIS_FONT_SIZE)
        });
        let plot = outer.inset(margins);
        set_y_range(&mut y_scale, plot.y_range());
        let x_scale = resolve_x(x, plot.x_range());

        Self {
            outer,
            plot,
            x: x_scale,
            y: y_scale,
            x_tick_count,
            y_tick_count,
        }
    }
}

fn resolve_y(y: &YDomain, range: (f64, f64), nice: bool) -> YScale {
    match y {
        YDomain::Linear(e) => {
            let e = if e.is_degenerate() { e.pad(0.1) } else { *e };
            let s = LinearScale::new(e, range);
            YScale::Linear(if nice { s.nice(5) } else { s })
        }
        YDomain::Band(categories) => {
            // The y range runs (bottom, top); categories read top to bottom,
            // so the band scale gets (top, bottom) and index 0 sits at the top.
            YScale::Band(
                BandScale::new(categories.iter().cloned(), (range.1, range.0))
                    .with_padding(0.08, 0.04),
            )
        }
        YDomain::Log(e) => {
            let e = Extent::new(e.min.max(f64::MIN_POSITIVE), e.max.max(f64::MIN_POSITIVE));
            let e = if e.is_degenerate() {
                Extent::new(e.min / 10.0, e.max * 10.0)
            } else {
                e
            };
            let s = LogScale::new(e, range);
            YScale::Log(if nice { s.nice() } else { s })
        }
    }
}

fn set_y_range(y: &mut YScale, range: (f64, f64)) {
    match y {
        YScale::Linear(s) => s.range = range,
        YScale::Log(s) => s.range = range,
        YScale::Band(s) => s.range = (range.1, range.0),
    }
}

fn resolve_x_range(x: XScale, range: (f64, f64)) -> XScale {
    match x {
        XScale::Linear(mut s) => {
            s.range = range;
            XScale::Linear(s)
        }
        XScale::Time(mut s) => {
            s.range = range;
            XScale::Time(s)
        }
        XScale::Band(mut s) => {
            s.range = range;
            XScale::Band(s)
        }
    }
}

fn resolve_x(x: &XDomain, range: (f64, f64)) -> XScale {
    match x {
        XDomain::Linear(e) => {
            let e = if e.is_degenerate() { e.pad(0.1) } else { *e };
            XScale::Linear(LinearScale::new(e, range))
        }
        XDomain::Time { extent, tz } => {
            let extent = if extent.span_ms() > 0.0 {
                *extent
            } else {
                let ms = extent.min.as_millisecond();
                Extent::new(
                    Timestamp::from_millisecond(ms - 30_000).unwrap_or(extent.min),
                    Timestamp::from_millisecond(ms + 30_000).unwrap_or(extent.max),
                )
            };
            XScale::Time(TimeScale::new(extent, range).in_zone(tz.clone()))
        }
        XDomain::Band(categories) => {
            XScale::Band(BandScale::new(categories.iter().cloned(), range))
        }
    }
}

/// Categorical slot assignment by series id.
///
/// Slots are handed out on first sight and never reassigned: color follows
/// the entity, so filtering out a series does not repaint the survivors.
/// Share one `SeriesSlots` between a chart and its legend, or between the
/// charts of a dashboard, so the same id is the same color everywhere.
#[derive(Default)]
pub struct SeriesSlots {
    ids: RefCell<Vec<String>>,
    highlight: Mutable<Option<String>>,
}

impl SeriesSlots {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    /// The series id being pointed at in a legend (or `None`). Mark layers
    /// dim every other series while it is set.
    pub fn highlight(&self) -> &Mutable<Option<String>> {
        &self.highlight
    }

    /// Opacity for `id` given the current highlight: 1 when nothing or this
    /// series is highlighted, dimmed otherwise.
    pub fn opacity_signal(
        &self,
        id: String,
    ) -> impl futures_signals::signal::Signal<Item = String> + use<> {
        self.highlight.signal_ref(move |h| match h {
            Some(other) if *other != id => "0.25".to_string(),
            _ => "1".to_string(),
        })
    }

    /// The slot for `id`, assigning the next free one if it is new. The
    /// ninth and later ids get [`crate::theme::SERIES_SLOTS`] and draw as
    /// "Other".
    pub fn slot_for(&self, id: &str) -> usize {
        let mut ids = self.ids.borrow_mut();
        if let Some(i) = ids.iter().position(|s| s == id) {
            return i;
        }
        ids.push(id.to_string());
        ids.len() - 1
    }

    /// The CSS color for a series id: `var(--dviz-series-N)`.
    pub fn color_for(&self, id: &str) -> String {
        crate::theme::series_var(self.slot_for(id))
    }
}

/// One line of a tooltip: an optional swatch color, a label, a value.
#[derive(Debug, Clone, PartialEq)]
pub struct TooltipRow {
    pub color: Option<String>,
    pub label: String,
    pub value: String,
}

/// What the chart's tooltip shows and where it anchors (outer px coords).
/// Set by interaction and mark layers; rendered by the container.
#[derive(Debug, Clone, PartialEq)]
pub struct Tooltip {
    pub x: f64,
    pub y: f64,
    pub title: String,
    pub rows: Vec<TooltipRow>,
}

/// Shared state every layer of one chart reads.
pub struct ChartContext {
    frame: Mutable<Frame>,
    slots: Rc<SeriesSlots>,
    tooltip: Mutable<Option<Tooltip>>,
    hover_x: Mutable<Option<f64>>,
    clip_id: String,
    vivid: Cell<bool>,
    live: Cell<bool>,
}

static CHART_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl ChartContext {
    pub fn new() -> Self {
        Self::with_slots(SeriesSlots::new())
    }

    pub fn with_slots(slots: Rc<SeriesSlots>) -> Self {
        let n = CHART_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            frame: Mutable::new(Frame::empty()),
            slots,
            tooltip: Mutable::new(None),
            hover_x: Mutable::new(None),
            clip_id: format!("dviz-clip-{n}"),
            vivid: Cell::new(crate::theme::vivid()),
            live: Cell::new(false),
        }
    }

    /// Whether Rust-side tweens should run: the vivid style is on and the
    /// chart is not live.
    pub fn motion_enabled(&self) -> bool {
        self.vivid.get() && !self.live.get()
    }

    pub(crate) fn configure(&self, vivid: bool, live: bool) {
        self.vivid.set(vivid);
        self.live.set(live);
    }

    /// The id of this chart's plot-area `<clipPath>`.
    pub fn clip_id(&self) -> &str {
        &self.clip_id
    }

    /// `url(#…)` for a `clip-path` attribute: marks must never paint outside
    /// the plot, which matters as soon as a domain is zoomed or fixed.
    pub fn clip_url(&self) -> String {
        format!("url(#{})", self.clip_id)
    }

    /// The id of the vertical fade gradient for a color slot.
    pub fn gradient_id(&self, slot: usize) -> String {
        format!("{}-grad-{slot}", self.clip_id)
    }

    /// `url(#…)` of the fade gradient for a series id: the vivid wash fill.
    pub fn gradient_for(&self, id: &str) -> String {
        format!(
            "url(#{})",
            self.gradient_id(self.slot_for(id).min(crate::theme::SERIES_SLOTS))
        )
    }

    /// The tooltip state. Layers set it on hover and focus and clear it on
    /// leave and blur; the container draws it.
    pub fn tooltip(&self) -> &Mutable<Option<Tooltip>> {
        &self.tooltip
    }

    /// The hovered data x, when a crosshair layer is tracking the pointer.
    /// Other layers may highlight against it.
    pub fn hover_x(&self) -> &Mutable<Option<f64>> {
        &self.hover_x
    }

    pub fn slots(&self) -> &Rc<SeriesSlots> {
        &self.slots
    }

    /// The current frame as a signal source. Layers derive their
    /// attributes from `frame().signal_ref(..)` / `signal_cloned()`.
    pub fn frame(&self) -> ReadOnlyMutable<Frame> {
        self.frame.read_only()
    }

    /// Replaces the frame. The container does this; layers should not.
    pub fn set_frame(&self, frame: Frame) {
        self.frame.set_neq(frame);
    }

    /// See [`SeriesSlots::slot_for`].
    pub fn slot_for(&self, id: &str) -> usize {
        self.slots.slot_for(id)
    }

    /// See [`SeriesSlots::color_for`].
    pub fn color_for(&self, id: &str) -> String {
        self.slots.color_for(id)
    }
}

impl Default for ChartContext {
    fn default() -> Self {
        Self::new()
    }
}

/// A layer draws into the plot given the shared context. Layers are
/// rendered once and keep themselves updated through signals.
pub type Layer = Box<dyn FnOnce(&Rc<ChartContext>) -> Dom>;

/// Wraps a closure as a [`Layer`].
pub fn layer(f: impl FnOnce(&Rc<ChartContext>) -> Dom + 'static) -> Layer {
    Box::new(f)
}

/// The chart container.
///
/// Renders a block `div.dviz-chart` of the given height and full width,
/// with an `svg[role=img]` inside sized to it. `x_domain` and `y_domain`
/// are signals so a live chart re-ranges as its data grows. `label` is the
/// accessible name and is required for a chart to be meaningful to a screen
/// reader; put the series names and what the axes measure in it.
#[component(render_fn = chart)]
struct Chart {
    #[signal]
    #[default(XDomain::Linear(Extent::UNIT))]
    x_domain: XDomain,

    #[signal]
    #[default(YDomain::Linear(Extent::UNIT))]
    y_domain: YDomain,

    /// Round the y domain out to tick boundaries.
    #[default(true)]
    nice_y: bool,

    /// Container height in px. The x-axis band is inside it.
    #[default(240.0)]
    height: f64,

    /// Override the automatic axis margins.
    #[default(None)]
    margins: Option<Margins>,

    /// Extra room on the right, in px, for end labels that hang past the
    /// last point. Ignored when `margins` is set.
    #[signal]
    #[default(0.0)]
    extra_right: f64,

    /// Accessible name (`aria-label` and `<title>`).
    #[signal]
    #[default(String::new())]
    label: String,

    #[default(vec![])]
    layers: Vec<Layer>,

    /// Share color slots with a legend or other charts.
    #[default(None)]
    slots: Option<Rc<SeriesSlots>>,

    /// Receives the context after it is created; for building layers that
    /// live outside the chart (a legend beside it, a table view).
    #[default(None)]
    on_context: Option<Box<dyn FnOnce(Rc<ChartContext>)>>,

    /// The vivid dressing (gradients, glow, entrance motion, plot surface).
    /// Defaults to the global [`crate::theme::vivid`] setting.
    #[default(crate::theme::vivid())]
    vivid: bool,

    /// The data streams in: tweens are off so marks never lag the data,
    /// and ticks jump instead of sliding.
    #[default(false)]
    live: bool,
}

pub fn chart(props: ChartProps) -> Dom {
    let ChartProps {
        x_domain,
        y_domain,
        nice_y,
        height,
        margins,
        extra_right,
        label,
        layers,
        slots,
        on_context,
        vivid,
        live,
        apply,
    } = props;

    let ctx = Rc::new(ChartContext::with_slots(
        slots.unwrap_or_else(SeriesSlots::new),
    ));
    ctx.configure(vivid, live);
    if let Some(cb) = on_context {
        cb(ctx.clone());
    }
    let size = Mutable::new((0.0f64, height));
    let label = label.broadcast();

    let frames = map_ref! {
        let size = size.signal(),
        let x = x_domain,
        let y = y_domain,
        let extra_right = extra_right =>
        Frame::compute(Rect::from_size(size.0, size.1), x, y, nice_y, margins).with_extra_right(*extra_right, margins.is_none())
    };

    // The observer and its callback must outlive the insertion callback, and
    // dominator drops removal callbacks without running them at the root
    // (`DomHandle` leaks them), so the guard rides inside the frame future:
    // that future lives exactly as long as the element does.
    let observer = ObserverGuard::default();
    let slot = observer.0.clone();

    let view_box = ctx
        .frame()
        .signal_ref(|f| format!("0 0 {} {}", f.outer.width.max(1.0), f.outer.height.max(1.0)));

    html!("div", {
        .class("dviz-chart")
        .apply_if(vivid, |b| b.class("dviz-vivid"))
        .apply_if(live, |b| b.class("dviz-live-chart"))
        .style("height", format!("{height}px"))
        .future({
            let ctx = ctx.clone();
            async move {
                let _keep_alive = observer;
                frames
                    .for_each(move |frame| {
                        ctx.set_frame(frame);
                        async {}
                    })
                    .await
            }
        })
        .after_inserted(clone!(size => move |el: HtmlElement| {
            let rect = el.get_bounding_client_rect();
            size.set_neq((rect.width(), rect.height()));

            let cb = Closure::<dyn FnMut(js_sys::Array)>::new(clone!(size => move |entries: js_sys::Array| {
                if let Some(entry) = entries.get(0).dyn_ref::<ResizeObserverEntry>() {
                    let r = entry.content_rect();
                    size.set_neq((r.width(), r.height()));
                }
            }));
            if let Ok(obs) = ResizeObserver::new(cb.as_ref().unchecked_ref()) {
                obs.observe(&el);
                *slot.borrow_mut() = Some((obs, cb));
            }
        }))
        .child(svg!("svg", {
            .attr("width", "100%")
            .attr("height", "100%")
            .attr("role", "img")
            .attr_signal("viewBox", view_box)
            .attr_signal("aria-label", label.signal_cloned())
            .child(svg!("title", { .text_signal(label.signal_cloned()) }))
            .child(svg!("defs", {
                .child(svg!("clipPath", {
                    .attr("id", ctx.clip_id())
                    .child(svg!("rect", {
                        .attr_signal("x", ctx.frame().signal_ref(|f| format!("{}", f.plot.x)))
                        .attr_signal("y", ctx.frame().signal_ref(|f| format!("{}", f.plot.y - 1.0)))
                        .attr_signal("width", ctx.frame().signal_ref(|f| format!("{}", f.plot.width.max(0.0))))
                        .attr_signal("height", ctx.frame().signal_ref(|f| format!("{}", (f.plot.height + 2.0).max(0.0))))
                    }))
                }))
                .children((0..=crate::theme::SERIES_SLOTS).map(|slot| {
                    // A vertical fade of each slot color for washes: strong
                    // at the line, gone at the baseline.
                    let color = crate::theme::series_var(slot);
                    svg!("linearGradient", {
                        .attr("id", &ctx.gradient_id(slot))
                        .attr("x1", "0").attr("y1", "0").attr("x2", "0").attr("y2", "1")
                        .child(svg!("stop", {
                            .attr("offset", "0")
                            .attr("style", &format!("stop-color: {color}; stop-opacity: 0.42"))
                        }))
                        .child(svg!("stop", {
                            .attr("offset", "1")
                            .attr("style", &format!("stop-color: {color}; stop-opacity: 0.02"))
                        }))
                    })
                }))
            }))
            .child(svg!("rect", {
                .class("dviz-plot-bg")
                .attr("rx", "6")
                .attr_signal("x", ctx.frame().signal_ref(|f| format!("{}", f.plot.x)))
                .attr_signal("y", ctx.frame().signal_ref(|f| format!("{}", f.plot.y)))
                .attr_signal("width", ctx.frame().signal_ref(|f| format!("{}", f.plot.width.max(0.0))))
                .attr_signal("height", ctx.frame().signal_ref(|f| format!("{}", f.plot.height.max(0.0))))
            }))
            .children(layers.into_iter().map(|l| l(&ctx)))
        }))
        .child(tooltip_host(&ctx))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}

/// The floating tooltip: one persistent element whose position, content
/// and visibility follow the tooltip signal, so moving the pointer never
/// recreates it (and never restarts its entrance). Anchored at the
/// tooltip's point, offset 12px, and flipped toward the centre when the
/// point is near an edge so it never leaves the container.
fn tooltip_host(ctx: &Rc<ChartContext>) -> Dom {
    let state = map_ref! {
        let tip = ctx.tooltip().signal_cloned(),
        let frame = ctx.frame().signal_cloned() => {
            tip.as_ref().map(|t| placed(t, frame.outer.width, frame.outer.height))
        }
    }
    .broadcast();
    // Content and placement are held while hidden so the fade-out shows
    // the last tooltip rather than an empty box.
    let last = held_last(state.signal_cloned());
    let visible = state.signal_ref(|s| s.is_some()).dedupe().broadcast();
    let rows = last
        .signal_ref(|s| s.as_ref().map(|p| p.tip.rows.clone()).unwrap_or_default())
        .dedupe_cloned()
        .to_signal_vec()
        .map(|r| {
            html!("div", {
                .class("dviz-tooltip-row")
                .apply_if(r.color.is_some(), |b| b.child(html!("span", {
                    .class("dviz-tooltip-swatch")
                    .style("background", r.color.clone().unwrap())
                })))
                .child(html!("span", { .class("dviz-tooltip-label") .text(&r.label) }))
                .child(html!("span", { .class("dviz-tooltip-value") .text(&r.value) }))
            })
        });
    html!("div", {
        .class("dviz-tooltip-host")
        .attr("aria-live", "polite")
        .child(html!("div", {
            .class("dviz-tooltip")
            .attr("role", "status")
            .attr_signal("data-visible", visible.signal().map(|v| if v { "true" } else { "false" }))
            .style_signal("left", last.signal_ref(|s| s.as_ref().map_or("auto".to_string(), |p| p.left.clone())))
            .style_signal("right", last.signal_ref(|s| s.as_ref().map_or("auto".to_string(), |p| p.right.clone())))
            .style_signal("top", last.signal_ref(|s| s.as_ref().map_or("auto".to_string(), |p| p.top.clone())))
            .style_signal("bottom", last.signal_ref(|s| s.as_ref().map_or("auto".to_string(), |p| p.bottom.clone())))
            .child(html!("div", {
                .class("dviz-tooltip-title")
                .text_signal(last.signal_ref(|s| s.as_ref().map(|p| p.tip.title.clone()).unwrap_or_default()))
            }))
            .children_signal_vec(rows)
        }))
    })
}

#[derive(Clone, PartialEq)]
struct PlacedTooltip {
    tip: Tooltip,
    left: String,
    right: String,
    top: String,
    bottom: String,
}

fn placed(t: &Tooltip, width: f64, height: f64) -> PlacedTooltip {
    let flip_x = t.x > width / 2.0;
    let flip_y = t.y > height * 0.66;
    PlacedTooltip {
        tip: t.clone(),
        left: if flip_x {
            "auto".into()
        } else {
            format!("{}px", t.x + 12.0)
        },
        right: if flip_x {
            format!("{}px", width - t.x + 12.0)
        } else {
            "auto".into()
        },
        top: if flip_y {
            "auto".into()
        } else {
            format!("{}px", t.y + 12.0)
        },
        bottom: if flip_y {
            format!("{}px", height - t.y + 12.0)
        } else {
            "auto".into()
        },
    }
}

/// Holds the last `Some` of an optional signal.
fn held_last<T: Clone + 'static>(
    sig: impl futures_signals::signal::Signal<Item = Option<T>> + 'static,
) -> futures_signals::signal::Broadcaster<impl futures_signals::signal::Signal<Item = Option<T>>> {
    let cache: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
    sig.map(move |o| {
        if let Some(v) = o {
            *cache.borrow_mut() = Some(v);
        }
        cache.borrow().clone()
    })
    .broadcast()
}

/// A live `ResizeObserver` and the closure it calls back into.
type Observation = (ResizeObserver, Closure<dyn FnMut(js_sys::Array)>);

/// Disconnects the observer when dropped.
#[derive(Default)]
struct ObserverGuard(Rc<RefCell<Option<Observation>>>);

impl Drop for ObserverGuard {
    fn drop(&mut self) {
        if let Some((obs, _cb)) = self.0.borrow_mut().take() {
            obs.disconnect();
        }
    }
}

/// A static domain signal, for charts whose data does not change.
pub fn fixed<T: Clone + 'static>(v: T) -> impl futures_signals::signal::Signal<Item = T> {
    always(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_follow_identity() {
        let ctx = ChartContext::new();
        assert_eq!(ctx.slot_for("a"), 0);
        assert_eq!(ctx.slot_for("b"), 1);
        assert_eq!(ctx.slot_for("a"), 0, "re-asking never reassigns");
        assert_eq!(ctx.color_for("b"), "var(--dviz-series-2)");
        for i in 0..7 {
            ctx.slot_for(&format!("s{i}"));
        }
        assert_eq!(ctx.color_for("ninth"), "var(--dviz-ink-muted)");
    }

    #[test]
    fn frame_reserves_axis_margins_and_ranges_scales() {
        let f = Frame::compute(
            Rect::from_size(400.0, 200.0),
            &XDomain::Linear(Extent::new(0.0, 10.0)),
            &YDomain::Linear(Extent::new(0.0, 97.0)),
            true,
            None,
        );
        assert!(f.plot.x > 20.0, "left margin fits '100': {}", f.plot.x);
        assert!(f.plot.bottom() < 200.0 - 15.0, "bottom margin fits labels");
        assert_eq!(f.y.domain(), Extent::new(0.0, 100.0), "niced");
        assert_eq!(f.y.map(0.0), f.plot.bottom());
        assert_eq!(f.y.map(100.0), f.plot.y);
        assert_eq!(f.x.map(0.0), f.plot.x);
        assert_eq!(f.x.map(10.0), f.plot.right());
        assert!(f.y.map(f64::NAN).is_nan());
        assert!(!f.y_ticks().is_empty());
        assert!(
            f.y_ticks()
                .iter()
                .all(|t| t.position >= f.plot.y && t.position <= f.plot.bottom())
        );
    }

    #[test]
    fn band_x_maps_index_to_centre() {
        let f = Frame::compute(
            Rect::from_size(400.0, 200.0),
            &XDomain::band(["a", "b"]),
            &YDomain::Linear(Extent::new(0.0, 1.0)),
            false,
            Some(Margins::uniform(0.0)),
        );
        let XScale::Band(b) = &f.x else { panic!() };
        assert_eq!(f.x.map(1.0), b.map("b").unwrap() + b.bandwidth() / 2.0);
        assert!(f.x.map(5.0).is_nan());
        assert_eq!(f.x_ticks().len(), 2);
        assert_eq!(f.x.invert(f.x.map(1.0)), 1.0);
    }

    #[test]
    fn log_y_gaps_non_positive_and_labels_si() {
        let f = Frame::compute(
            Rect::from_size(400.0, 300.0),
            &XDomain::Linear(Extent::UNIT),
            &YDomain::Log(Extent::new(3.0, 8000.0)),
            true,
            None,
        );
        assert!(f.y.map(0.0).is_nan());
        assert_eq!(f.y.domain(), Extent::new(1.0, 10000.0));
        assert!(
            f.y_ticks().iter().any(|t| t.label == "1k"),
            "{:?}",
            f.y_ticks()
        );
    }

    #[test]
    fn time_x_ticks_are_calendar_aligned() {
        let a: Timestamp = "2026-01-01T00:07:00Z".parse().unwrap();
        let b: Timestamp = "2026-01-01T02:00:00Z".parse().unwrap();
        let f = Frame::compute(
            Rect::from_size(800.0, 200.0),
            &XDomain::time(Extent::new(a, b)),
            &YDomain::Linear(Extent::UNIT),
            true,
            None,
        );
        let ticks = f.x_ticks();
        assert!(ticks.len() >= 4, "{ticks:?}");
        assert!(ticks.iter().any(|t| t.label == "01:00"), "{ticks:?}");
        assert_eq!(f.x.map(b.as_millisecond() as f64), f.plot.right());
    }

    #[test]
    fn degenerate_domains_open_up() {
        let f = Frame::compute(
            Rect::from_size(100.0, 100.0),
            &XDomain::Linear(Extent::new(5.0, 5.0)),
            &YDomain::Linear(Extent::new(3.0, 3.0)),
            true,
            None,
        );
        assert!(!f.y.domain().is_degenerate());
        assert!(f.x.map(5.0).is_finite());
        let e = Frame::empty();
        assert_eq!(e.plot.width, 0.0);
        assert!(e.y_ticks().len() <= 3);
    }
}

#[cfg(test)]
mod band_y_tests {
    use super::*;

    #[test]
    fn band_y_reads_top_to_bottom_and_fills_the_plot() {
        let f = Frame::compute(
            Rect::from_size(200.0, 220.0),
            &XDomain::band(["a"]),
            &YDomain::band(["r1", "r2"]),
            false,
            Some(Margins::new(0.0, 0.0, 20.0, 0.0)),
        );
        let b = f.y.band().unwrap();
        assert!(f.y.map(0.0) < f.y.map(1.0), "first category is at the top");
        assert!(b.map_index(0).unwrap() >= f.plot.y);
        assert!(b.map_index(1).unwrap() + b.bandwidth() <= f.plot.bottom() + 1e-9);
        let ticks = f.y_ticks();
        assert_eq!(ticks[0].label, "r1");
        assert!(ticks[0].position < ticks[1].position);
        assert_eq!(f.y.invert(f.y.map(1.0)), 1.0);
    }
}
