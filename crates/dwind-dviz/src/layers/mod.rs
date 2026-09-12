//! The standard layers. Each is a function returning a [`crate::chart::Layer`].
//!
//! Chrome layers (`axis`, `grid`) go first so marks paint over them; label
//! and annotation layers go last.
//!
//! The helpers here implement the motion model every mark layer shares:
//!
//! - Marks are persistent, keyed DOM nodes whose geometry arrives through
//!   signals, so a data change patches attributes and CSS can tween them.
//! - A removed key stays in the DOM for [`EXIT_MS`] with the `dviz-leave`
//!   class, so it can fade out; a new key gets `dviz-enter` for its
//!   entrance. A key that returns while leaving simply comes back.
//! - Tooltips are signals too: a mark hovered while its data changes shows
//!   the new value, and a mark removed while hovered clears the tooltip.

pub mod annotations;
pub mod arc;
pub mod area;
pub mod axis;
pub mod bars;
pub mod cells;
pub mod grid;
pub mod interaction;
pub mod labels;
pub mod line;
pub mod points;
pub mod width_tap;

use std::cell::{Cell, RefCell};
use std::future::Future;
use std::hash::Hash;
use std::rc::Rc;

use dominator::{Dom, DomBuilder, clone, events, svg};
use dwind_dviz_core::data::Series;
use dwind_dviz_core::geom::{self, StackOffset, Stacked};
use futures_signals::map_ref;
use futures_signals::signal::{Broadcaster, Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use web_sys::SvgElement;

use crate::chart::{ChartContext, Frame, Tooltip};

/// How long a removed series or mark lingers to play its exit.
pub const EXIT_MS: u32 = 260;

pub(crate) fn round(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

pub(crate) fn px(v: f64) -> String {
    let r = round(v);
    if r == r.trunc() {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

/// A displayed key and whether it is on its way out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Shown<K> {
    pub key: K,
    pub leaving: bool,
}

/// A boxed, thread-local signal.
pub(crate) type LocalSignal<T> = std::pin::Pin<Box<dyn Signal<Item = T>>>;

/// A signal of displayed keys, shared by every node that needs it.
pub(crate) type ShownSignal<K> = Broadcaster<LocalSignal<Vec<Shown<K>>>>;

/// The displayed keys as a minimally diffed list plus their leaving state.
pub(crate) struct Tracked<K: 'static> {
    shown: ShownSignal<K>,
    keys: MutableVec<K>,
}

impl<K: Clone + Eq + 'static> Tracked<K> {
    /// The children to render, with only changed keys added or removed.
    pub fn keys(&self) -> impl SignalVec<Item = K> + use<K> {
        self.keys.signal_vec_cloned()
    }

    /// Whether `key` is on its way out.
    pub fn leaving(&self, key: K) -> impl Signal<Item = bool> + use<K> {
        self.shown
            .signal_ref(move |v| v.iter().find(|s| s.key == key).is_some_and(|s| s.leaving))
            .dedupe()
    }
}

/// A signal that keeps the last `Some` it saw: `None` inputs (a key whose
/// mark is leaving, or not there yet) leave the held value unchanged.
pub(crate) fn held<T, S>(sig: S) -> Broadcaster<LocalSignal<Option<T>>>
where
    T: Clone + 'static,
    S: Signal<Item = Option<T>> + 'static,
{
    let cache: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
    sig.map(move |o| {
        if let Some(v) = o {
            *cache.borrow_mut() = Some(v);
        }
        cache.borrow().clone()
    })
    .boxed_local()
    .broadcast()
}

/// Renders `build` once the held value first exists, giving it a signal
/// that is always present from then on. The exit-tracking driver can run a
/// poll behind the geometry, so a key may briefly precede its value; this
/// waits instead of asserting.
pub(crate) fn when_present<T, F>(held: Broadcaster<LocalSignal<Option<T>>>, build: F) -> Dom
where
    T: Clone + 'static,
    F: FnOnce(Broadcaster<LocalSignal<T>>) -> Dom + 'static,
{
    let present = held.signal_ref(|o| o.is_some()).dedupe();
    let build = RefCell::new(Some(build));
    svg!("g", {
        .child_signal(present.map(move |has| {
            if !has {
                return None;
            }
            let build = build.borrow_mut().take()?;
            let always: LocalSignal<T> = held
                .signal_cloned()
                .filter_map(|o| o)
                .map(|o| o.expect("present"))
                .boxed_local();
            Some(build(always.broadcast()))
        }))
    })
}

/// Turns a signal of current keys into a signal of displayed keys, where a
/// key that disappears lingers for `exit_ms` flagged as leaving. Attach the
/// returned future to the element that owns the keys.
pub(crate) fn with_exit<K, S>(
    keys: S,
    exit_ms: u32,
) -> (Tracked<K>, impl Future<Output = ()> + use<K, S>)
where
    K: Clone + Eq + Hash + 'static,
    S: Signal<Item = Vec<K>> + 'static,
{
    // Each entry carries the generation of its current departure, so a
    // purge scheduled for an earlier departure never removes a key that
    // came back and left again.
    let state: Mutable<Vec<(K, Option<u32>)>> = Mutable::new(Vec::new());
    let generation = Rc::new(Cell::new(0u32));
    let out = state
        .signal_ref(|v| {
            v.iter()
                .map(|(k, leaving)| Shown {
                    key: k.clone(),
                    leaving: leaving.is_some(),
                })
                .collect::<Vec<_>>()
        })
        .boxed_local()
        .broadcast();
    let list: MutableVec<K> = MutableVec::new();
    let sync = {
        let list = list.clone();
        move |entries: &[(K, Option<u32>)]| {
            let keys: Vec<K> = entries.iter().map(|(k, _)| k.clone()).collect();
            crate::keyed::apply(&list, &keys);
        }
    };
    let sync = Rc::new(sync);
    let driver = keys.for_each(clone!(state, generation, sync => move |current| {
        let mut any_leaving = false;
        {
            let mut entries = state.lock_mut();
            let generation_now = generation.get() + 1;
            generation.set(generation_now);
            for (k, leaving) in entries.iter_mut() {
                if current.contains(k) {
                    *leaving = None;
                } else if leaving.is_none() {
                    *leaving = Some(generation_now);
                    any_leaving = true;
                }
            }
            for k in &current {
                if !entries.iter().any(|(e, _)| e == k) {
                    entries.push((k.clone(), None));
                }
            }
            sync(&entries);
        }
        if any_leaving {
            let purge_generation = generation.get();
            schedule(exit_ms, clone!(state, sync => move || {
                let mut entries = state.lock_mut();
                entries.retain(|(_, leaving)| *leaving != Some(purge_generation));
                sync(&entries);
            }));
        }
        async {}
    }));
    (
        Tracked {
            shown: out,
            keys: list,
        },
        driver,
    )
}

/// Runs `f` after `ms` milliseconds in the browser; immediately elsewhere
/// (native tests have no event loop to wait on).
fn schedule(ms: u32, f: impl FnOnce() + 'static) {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(ms).await;
            f();
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = ms;
        f();
    }
}

/// The ids of `series`, in order.
fn ids_of(series: &[Series]) -> Vec<String> {
    // DOM keys must be unique.  Duplicate ids are invalid data, but dropping
    // the later duplicate here prevents duplicate keyed nodes and makes the
    // renderer fail closed instead of corrupting its diff state.
    let mut ids = Vec::with_capacity(series.len());
    for id in series.iter().map(|s| &s.id) {
        if !ids.iter().any(|seen| seen == id) {
            ids.push(id.clone());
        }
    }
    ids
}

/// One `<g data-series=id>` per series, keyed by id, with enter and leave
/// classes. The group's content is rebuilt whenever the frame or the data
/// changes; `render` gets the frame, every series, and the index of its
/// own. For layers whose content is cheap text (labels). Mark layers use
/// [`keyed_marks`] so their nodes persist.
pub(crate) fn keyed_series_groups<S, F>(
    ctx: &Rc<ChartContext>,
    series: S,
    class: &'static str,
    clip: bool,
    render: F,
) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
    F: Fn(&Frame, &[Series], usize, &ChartContext) -> Dom + Clone + 'static,
{
    let series = series.broadcast();
    let ctx = ctx.clone();
    let (shown, driver) = with_exit(
        series.signal_cloned().map(|v| ids_of(&v)).dedupe_cloned(),
        EXIT_MS,
    );
    let clip_url = ctx.clip_url();

    svg!("g", {
        .apply(|b| class.split_whitespace().fold(b, |b, c| b.class(c)))
        .apply_if(clip, |b| b.attr("clip-path", &clip_url))
        .future(driver)
        .children_signal_vec(shown.keys().map(move |id| {
            let render = render.clone();
            let ctx2 = ctx.clone();
            let series = series.clone();
            let wanted = id.clone();
            // Keeps the last content while leaving: the data is gone by then.
            let content = map_ref! {
                let frame = ctx.frame().signal_cloned(),
                let all = series.signal_cloned() => {
                    all.iter()
                        .position(|s| s.id == wanted)
                        .map(|i| render(frame, all, i, &ctx2))
                }
            }
            .filter_map(|d| d);
            svg!("g", {
                .class("dviz-enter")
                .attr("data-series", &id)
                .attr_signal("opacity", ctx.slots().opacity_signal(id.clone()))
                .class_signal("dviz-leave", shown.leaving(id.clone()))
                .child_signal(content)
            })
        }))
    })
}

/// What a mark renderer receives: the series' id and label, and a signal
/// of the mark's own geometry that holds its last value while leaving.
pub(crate) struct MarkInput<M: 'static> {
    pub series_id: String,
    pub series_label: String,
    pub mark: Broadcaster<LocalSignal<M>>,
}

/// Persistent, keyed marks: one node per `(series, key)`, each fed by a
/// signal of its own geometry.
///
/// `geometry` computes every mark of one series from the frame and the
/// data; `render` builds the node for one mark. Nodes get `dviz-enter` on
/// creation and `dviz-leave` while leaving.
pub(crate) fn keyed_marks<S, K, M, G, R>(
    ctx: &Rc<ChartContext>,
    series: S,
    class: &'static str,
    clip: bool,
    geometry: G,
    render: R,
) -> Dom
where
    S: Signal<Item = Vec<Series>> + 'static,
    K: Clone + Eq + Hash + 'static,
    M: Clone + PartialEq + 'static,
    G: Fn(&Frame, &[Series], usize) -> Vec<(K, M)> + Clone + 'static,
    R: Fn(&Rc<ChartContext>, &K, MarkInput<M>) -> Dom + Clone + 'static,
{
    let series = series.broadcast();
    let ctx = ctx.clone();
    let (shown, driver) = with_exit(
        series.signal_cloned().map(|v| ids_of(&v)).dedupe_cloned(),
        EXIT_MS,
    );
    let clip_url = ctx.clip_url();

    svg!("g", {
        .apply(|b| class.split_whitespace().fold(b, |b, c| b.class(c)))
        .apply_if(clip, |b| b.attr("clip-path", &clip_url))
        .future(driver)
        .children_signal_vec(shown.keys().map(move |id| {
            let geometry = geometry.clone();
            let render = render.clone();
            let ctx = ctx.clone();
            let series = series.clone();
            let wanted = id.clone();
            // This series' label and marks, recomputed on frame or data
            // change and held while the series is leaving.
            let marks_opt = held(map_ref! {
                let frame = ctx.frame().signal_cloned(),
                let all = series.signal_cloned() => {
                    all.iter()
                        .position(|s| s.id == wanted)
                        .map(|i| (all[i].label.clone(), geometry(frame, all, i)))
                }
            });
            let ctx2 = ctx.clone();
            let id2 = id.clone();
            let leaving = shown.leaving(id.clone());
            let opacity = ctx.slots().opacity_signal(id.clone());
            svg!("g", {
                .class("dviz-enter")
                .attr("data-series", &id)
                .attr_signal("opacity", opacity)
                .class_signal("dviz-leave", leaving)
                .child(when_present(marks_opt, move |marks| {
                    let (mark_keys, mark_driver) = with_exit(
                        marks
                            .signal_ref(|(_, v)| v.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>())
                            .dedupe_cloned(),
                        EXIT_MS,
                    );
                    svg!("g", {
                        .future(mark_driver)
                        .children_signal_vec(mark_keys.keys().map(move |key| {
                            let k = key.clone();
                            let one = held(
                                marks.signal_ref(move |(_, v)| v.iter().find(|(kk, _)| *kk == k).map(|(_, m)| m.clone())),
                            );
                            let series_label = label_now(&marks);
                            let render = render.clone();
                            let ctx3 = ctx2.clone();
                            let id3 = id2.clone();
                            let key2 = key.clone();
                            svg!("g", {
                                .class("dviz-enter")
                                .class_signal("dviz-leave", mark_keys.leaving(key.clone()))
                                .child(when_present(one, move |mark| {
                                    render(&ctx3, &key2, MarkInput { series_id: id3, series_label, mark })
                                }))
                            })
                        }))
                    })
                }))
            })
        }))
    })
}

/// The series label currently held by a marks broadcaster. Broadcasters
/// own their latest value, so a synchronous read is a single poll.
fn label_now<Sg, V>(marks: &Broadcaster<Sg>) -> String
where
    Sg: Signal<Item = (String, V)> + 'static,
    V: 'static,
{
    let sig = marks.signal_ref(|(l, _)| l.clone());
    futures::pin_mut!(sig);
    let waker = futures::task::noop_waker();
    let mut cx = std::task::Context::from_waker(&waker);
    match sig.as_mut().poll_change(&mut cx) {
        std::task::Poll::Ready(Some(l)) => l,
        _ => String::new(),
    }
}

/// Shows the tooltip from `tip` while the pointer is over the mark or it
/// has keyboard focus. The tooltip follows `tip` while shown, and clears
/// if the mark is removed while shown.
pub(crate) fn hoverable<T>(
    b: DomBuilder<SvgElement>,
    tooltip: Mutable<Option<Tooltip>>,
    tip: T,
) -> DomBuilder<SvgElement>
where
    T: Signal<Item = Tooltip> + 'static,
{
    let hovered = Rc::new(Cell::new(false));
    let latest: Rc<RefCell<Option<Tooltip>>> = Rc::new(RefCell::new(None));
    let show = clone!(tooltip, hovered, latest => move || {
        hovered.set(true);
        if let Some(t) = latest.borrow().clone() {
            tooltip.set(Some(t));
        }
    });
    let hide = clone!(tooltip, hovered => move || {
        hovered.set(false);
        tooltip.set(None);
    });
    b.future(tip.for_each(clone!(hovered, latest, tooltip => move |t| {
        *latest.borrow_mut() = Some(t.clone());
        if hovered.get() {
            tooltip.set(Some(t));
        }
        async {}
    })))
    .event(clone!(show => move |_: events::PointerEnter| show()))
    .event(clone!(hide => move |_: events::PointerLeave| hide()))
    .event(move |_: events::Focus| show())
    .event(clone!(hide => move |_: events::Blur| hide()))
    .after_removed(move |_| {
        if hovered.get() {
            hide();
        }
    })
}

/// Stacks series by point index (all series must share the same x order).
pub(crate) fn stack_series(all: &[Series], offset: StackOffset) -> Vec<Vec<Stacked>> {
    let columns: Vec<Vec<f64>> = all
        .iter()
        .map(|s| s.points.iter().map(|p| p.y).collect())
        .collect();
    geom::stack(&columns, offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::task::noop_waker;
    use std::task::{Context, Poll};

    fn poll<F: Future<Output = ()>>(f: std::pin::Pin<&mut F>) {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        let _ = f.poll(&mut cx);
    }

    fn snapshot(shown: &Tracked<&'static str>) -> Vec<(&'static str, bool)> {
        let sig = shown.shown.signal_cloned();
        futures::pin_mut!(sig);
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        match sig.as_mut().poll_change(&mut cx) {
            Poll::Ready(Some(v)) => v.iter().map(|s| (s.key, s.leaving)).collect(),
            _ => vec![],
        }
    }

    #[test]
    fn with_exit_appends_new_keys_and_purges_removed_ones() {
        let keys = Mutable::new(vec!["a", "b"]);
        let (shown, driver) = with_exit(keys.signal_cloned(), 0);
        futures::pin_mut!(driver);
        poll(driver.as_mut());
        assert_eq!(snapshot(&shown), vec![("a", false), ("b", false)]);
        keys.set(vec!["b", "c"]);
        poll(driver.as_mut());
        // Off-wasm the purge runs immediately, so "a" is already gone.
        assert_eq!(snapshot(&shown), vec![("b", false), ("c", false)]);
    }
}
