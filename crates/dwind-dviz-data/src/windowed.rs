//! A windowed, batched, followable source for live data.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

use dwind_dviz_core::data::{Extent, Point, Series};
use futures::stream::{Stream, StreamExt};
use futures_signals::signal::{Mutable, Signal};

/// How much history a series keeps.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Retention {
    /// The last `n` points.
    Count(usize),
    /// Points whose x is within `span` of the newest x (milliseconds for a
    /// time axis). The window signal is `[newest - span, newest]`.
    Span(f64),
}

struct Buffer {
    label: String,
    points: VecDeque<Point>,
}

#[derive(Default)]
struct Inner {
    order: Vec<String>,
    buffers: Vec<Buffer>,
}

/// Per-series ring buffers that publish as one `Vec<Series>` signal.
///
/// Pushes only touch the buffers; [`commit`](Self::commit) trims to the
/// retention and publishes. Register a notifier with
/// [`set_notifier`](Self::set_notifier) to be told when a commit is due, and
/// schedule it on the next animation frame so a burst of pushes costs one
/// publish per frame.
///
/// While `follow` is on, the window slides with the newest x; turning it
/// off freezes the window where it is (data keeps arriving and trimming),
/// so a reader can inspect a spike without the chart scrolling away.
pub struct WindowedSource {
    retention: Retention,
    inner: RefCell<Inner>,
    published: Mutable<Vec<Series>>,
    window: Mutable<Extent<f64>>,
    follow: Mutable<bool>,
    dirty: Cell<bool>,
    notifier: RefCell<Option<Box<dyn Fn()>>>,
}

impl WindowedSource {
    pub fn new(retention: Retention) -> Rc<Self> {
        Rc::new(Self {
            retention,
            inner: RefCell::new(Inner::default()),
            published: Mutable::new(Vec::new()),
            window: Mutable::new(Extent::new(0.0, 1.0)),
            follow: Mutable::new(true),
            dirty: Cell::new(false),
            notifier: RefCell::new(None),
        })
    }

    pub fn retention(&self) -> Retention {
        self.retention
    }

    /// Declares a series with a display label. Pushing to an unknown id
    /// declares it with the id as its label.
    pub fn add_series(&self, id: impl Into<String>, label: impl Into<String>) {
        let id = id.into();
        let mut inner = self.inner.borrow_mut();
        if let Some(i) = inner.order.iter().position(|s| *s == id) {
            inner.buffers[i].label = label.into();
        } else {
            inner.order.push(id);
            inner.buffers.push(Buffer {
                label: label.into(),
                points: VecDeque::new(),
            });
        }
        self.mark_dirty();
    }

    /// Appends a sample. Points must arrive in x order per series.
    pub fn push(&self, id: &str, point: Point) {
        {
            let mut inner = self.inner.borrow_mut();
            let i = match inner.order.iter().position(|s| s == id) {
                Some(i) => i,
                None => {
                    inner.order.push(id.to_string());
                    inner.buffers.push(Buffer {
                        label: id.to_string(),
                        points: VecDeque::new(),
                    });
                    inner.buffers.len() - 1
                }
            };
            let buf = &mut inner.buffers[i];
            buf.points.push_back(point);
            // Trim eagerly on count so a burst can't balloon memory.
            if let Retention::Count(n) = self.retention {
                while buf.points.len() > n {
                    buf.points.pop_front();
                }
            }
        }
        self.mark_dirty();
    }

    pub fn extend(&self, id: &str, points: impl IntoIterator<Item = Point>) {
        for p in points {
            self.push(id, p);
        }
    }

    pub fn clear(&self) {
        for buf in &mut self.inner.borrow_mut().buffers {
            buf.points.clear();
        }
        self.mark_dirty();
    }

    /// Total points currently buffered.
    pub fn len(&self) -> usize {
        self.inner
            .borrow()
            .buffers
            .iter()
            .map(|b| b.points.len())
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// True when a commit would publish something new.
    pub fn is_dirty(&self) -> bool {
        self.dirty.get()
    }

    fn mark_dirty(&self) {
        if !self.dirty.replace(true) {
            f_call(&self.notifier);
        }
    }

    /// Called once when the source becomes dirty after a commit. The
    /// renderer schedules a commit on the next animation frame from here.
    pub fn set_notifier(&self, f: impl Fn() + 'static) {
        *self.notifier.borrow_mut() = Some(Box::new(f));
        if self.dirty.get() {
            f_call(&self.notifier);
        }
    }

    /// Trims to the retention, publishes the series, and slides the window
    /// if following.
    pub fn commit(&self) {
        let mut inner = self.inner.borrow_mut();
        let newest = inner
            .buffers
            .iter()
            .filter_map(|b| b.points.back())
            .map(|p| p.x)
            .fold(f64::NEG_INFINITY, f64::max);
        if let (Retention::Span(span), true) = (self.retention, newest.is_finite()) {
            let cutoff = newest - span;
            for buf in &mut inner.buffers {
                while buf.points.front().is_some_and(|p| p.x < cutoff) {
                    buf.points.pop_front();
                }
            }
        }
        let series: Vec<Series> = inner
            .order
            .iter()
            .zip(&inner.buffers)
            .map(|(id, b)| {
                Series::new(
                    id.clone(),
                    b.label.clone(),
                    b.points.iter().copied().collect(),
                )
            })
            .collect();
        drop(inner);
        self.dirty.set(false);
        if self.follow.get() {
            self.window.set_neq(self.current_window(&series, newest));
        }
        self.published.set(series);
    }

    fn current_window(&self, series: &[Series], newest: f64) -> Extent<f64> {
        match self.retention {
            Retention::Span(span) if newest.is_finite() => Extent::new(newest - span, newest),
            _ => dwind_dviz_core::data::x_extent_of(series).unwrap_or(Extent::new(0.0, 1.0)),
        }
    }

    /// The published series.
    pub fn series_signal(&self) -> impl Signal<Item = Vec<Series>> + use<> {
        self.published.signal_cloned()
    }

    /// A snapshot of the published series.
    pub fn series(&self) -> Vec<Series> {
        self.published.get_cloned()
    }

    /// The x window to show: sliding while following, frozen while paused.
    pub fn window_signal(&self) -> impl Signal<Item = Extent<f64>> + use<> {
        self.window.signal()
    }

    pub fn window(&self) -> Extent<f64> {
        self.window.get()
    }

    pub fn follow_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.follow.signal()
    }

    pub fn is_following(&self) -> bool {
        self.follow.get()
    }

    /// Resumes following (the window jumps to the newest data) or pauses
    /// (the window freezes where it is).
    pub fn set_follow(&self, follow: bool) {
        self.follow.set_neq(follow);
        if follow {
            let series = self.published.get_cloned();
            let newest = series
                .iter()
                .filter_map(|s| s.points.last())
                .map(|p| p.x)
                .fold(f64::NEG_INFINITY, f64::max);
            self.window.set_neq(self.current_window(&series, newest));
        }
    }

    pub fn toggle_follow(&self) {
        self.set_follow(!self.is_following());
    }
}

fn f_call(notifier: &RefCell<Option<Box<dyn Fn()>>>) {
    if let Some(f) = notifier.borrow().as_ref() {
        f();
    }
}

impl super::DataSource for WindowedSource {
    fn series_signal(&self) -> super::BoxedSeriesSignal {
        Box::pin(self.published.signal_cloned())
    }
}

/// Feeds every `(series id, point)` from `stream` into `source` until the
/// stream ends. Spawn it on your executor (`wasm_bindgen_futures::spawn_local`
/// in the browser). A WebSocket, SSE, or polling loop becomes one stream.
pub async fn drive<S>(source: Rc<WindowedSource>, stream: S)
where
    S: Stream<Item = (String, Point)>,
{
    let mut stream = std::pin::pin!(stream);
    while let Some((id, point)) = stream.next().await {
        source.push(&id, point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn count_retention_trims_on_push() {
        let s = WindowedSource::new(Retention::Count(3));
        for i in 0..5 {
            s.push("a", Point::new(i as f64, 0.0));
        }
        assert_eq!(s.len(), 3);
        s.commit();
        let xs: Vec<f64> = s.series()[0].points.iter().map(|p| p.x).collect();
        assert_eq!(xs, vec![2.0, 3.0, 4.0]);
        assert_eq!(s.window(), Extent::new(2.0, 4.0));
    }

    #[test]
    fn span_retention_trims_on_commit_and_slides_the_window() {
        let s = WindowedSource::new(Retention::Span(10.0));
        s.add_series("a", "Alpha");
        s.extend("a", (0..=20).map(|i| Point::new(i as f64, 1.0)));
        assert_eq!(s.len(), 21, "span trimming waits for commit");
        s.commit();
        let pts = &s.series()[0].points;
        assert_eq!(pts.first().unwrap().x, 10.0);
        assert_eq!(pts.last().unwrap().x, 20.0);
        assert_eq!(s.window(), Extent::new(10.0, 20.0));
        assert_eq!(s.series()[0].label, "Alpha");
    }

    #[test]
    fn pause_freezes_the_window_but_not_the_data() {
        let s = WindowedSource::new(Retention::Span(10.0));
        s.extend("a", (0..=10).map(|i| Point::new(i as f64, 0.0)));
        s.commit();
        s.set_follow(false);
        s.extend("a", (11..=30).map(|i| Point::new(i as f64, 0.0)));
        s.commit();
        assert_eq!(s.window(), Extent::new(0.0, 10.0), "frozen");
        assert_eq!(
            s.series()[0].points.first().unwrap().x,
            20.0,
            "still trimmed"
        );
        s.set_follow(true);
        assert_eq!(s.window(), Extent::new(20.0, 30.0), "jumps to the newest");
    }

    #[test]
    fn notifier_fires_once_per_dirty_period() {
        let s = WindowedSource::new(Retention::Count(100));
        let calls = Rc::new(Cell::new(0));
        s.set_notifier({
            let calls = calls.clone();
            move || calls.set(calls.get() + 1)
        });
        s.push("a", Point::new(0.0, 0.0));
        s.push("a", Point::new(1.0, 0.0));
        s.push("b", Point::new(1.0, 0.0));
        assert_eq!(calls.get(), 1);
        assert!(s.is_dirty());
        s.commit();
        assert!(!s.is_dirty());
        s.push("a", Point::new(2.0, 0.0));
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn late_notifier_is_called_if_already_dirty() {
        let s = WindowedSource::new(Retention::Count(100));
        s.push("a", Point::new(0.0, 0.0));
        let calls = Rc::new(Cell::new(0));
        s.set_notifier({
            let calls = calls.clone();
            move || calls.set(calls.get() + 1)
        });
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn drive_consumes_a_stream() {
        let s = WindowedSource::new(Retention::Count(10));
        let stream = futures::stream::iter(
            (0..5).map(|i| ("x".to_string(), Point::new(i as f64, i as f64))),
        );
        futures::executor::block_on(drive(s.clone(), stream));
        s.commit();
        assert_eq!(s.series()[0].points.len(), 5);
    }
}
