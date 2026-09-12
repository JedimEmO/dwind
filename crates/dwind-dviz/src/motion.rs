//! Rust-side tweening for values the browser cannot interpolate well
//! (arc angles, anything with a discontinuous SVG encoding).
//!
//! [`tween`] follows a target signal with an eased interpolation stepped on
//! animation frames. It respects `prefers-reduced-motion` and can be
//! disabled outright for live data.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use futures_signals::signal::{Mutable, Signal, SignalExt};

/// Cubic ease-out: fast start, gentle landing.
pub fn ease_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// True when the viewer asked for reduced motion.
pub fn reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches())
}

/// A tweened signal. Each new `target` starts an interpolation from the
/// current shown value over `duration_ms`; the first target starts from
/// `start(&target)` so a mark can sweep or grow in. When `enabled` is
/// false, or motion is reduced, the shown value simply follows the target.
///
/// Returns the shown signal (`None` until the first target arrives) and
/// the future that drives it; attach the future to the mark's node so the
/// tween stops with it.
pub fn tween<T, S, L, I>(
    target: S,
    duration_ms: f64,
    enabled: bool,
    start: I,
    lerp: L,
) -> (
    impl Signal<Item = Option<T>> + use<T, S, L, I>,
    impl std::future::Future<Output = ()> + use<T, S, L, I>,
)
where
    T: Clone + PartialEq + 'static,
    S: Signal<Item = T> + 'static,
    L: Fn(&T, &T, f64) -> T + 'static,
    I: FnOnce(&T) -> T + 'static,
{
    let shown: Mutable<Option<T>> = Mutable::new(None);
    let generation = Rc::new(Cell::new(0u32));
    let start = Rc::new(RefCell::new(Some(start)));
    let lerp = Rc::new(lerp);
    let animate = enabled && !reduced_motion();
    let out = shown.signal_cloned();
    let driver = target.for_each(move |goal| {
        let from = match shown.get_cloned() {
            Some(v) => v,
            None => match start.borrow_mut().take() {
                Some(s) if animate => s(&goal),
                _ => goal.clone(),
            },
        };
        if !animate || from == goal {
            shown.set(Some(goal));
            return async {}.left_future();
        }
        let my_generation = generation.get() + 1;
        generation.set(my_generation);
        let shown = shown.clone();
        let generation = generation.clone();
        let lerp = lerp.clone();
        run_frames(move |elapsed| {
            if generation.get() != my_generation {
                return false;
            }
            let t = (elapsed / duration_ms).clamp(0.0, 1.0);
            shown.set(Some(lerp(&from, &goal, ease_out(t))));
            t < 1.0
        })
        .right_future()
    });
    (out, driver)
}

/// Calls `step(elapsed_ms)` once per animation frame until it returns
/// false. Frames are approximated with 16ms timeouts, which keeps this
/// executor-agnostic and stops when the owning node is dropped.
async fn run_frames(mut step: impl FnMut(f64) -> bool) {
    let t0 = now_ms();
    loop {
        gloo_timers::future::TimeoutFuture::new(16).await;
        if !step(now_ms() - t0) {
            break;
        }
    }
}

fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

use futures::FutureExt as _;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ease_out_is_monotone_and_bounded() {
        let mut last = 0.0;
        for i in 0..=10 {
            let v = ease_out(i as f64 / 10.0);
            assert!(v >= last && (0.0..=1.0).contains(&v));
            last = v;
        }
        assert_eq!(ease_out(1.0), 1.0);
    }
}
