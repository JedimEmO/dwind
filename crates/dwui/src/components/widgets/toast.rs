use crate::theme::layers;
use crate::theme::prelude::*;
use dominator::{clone, events, html, svg, Dom};
use dwind::prelude::*;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use futures_signals_component_macro::component;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ToastVariant {
    Info,
    Success,
    Warning,
    Error,
}

/// Options for a single toast pushed through a [`Toaster`].
pub struct ToastOptions {
    pub title: String,
    pub message: Option<String>,
    pub variant: ToastVariant,
    /// Auto-dismiss delay; `None` keeps the toast until dismissed
    pub duration_ms: Option<u32>,
    pub dismissible: bool,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            message: None,
            variant: ToastVariant::Info,
            duration_ms: Some(4000),
            dismissible: true,
        }
    }
}

pub struct ToastEntry {
    pub id: u64,
    pub options: ToastOptions,
}

/// Handle for pushing toasts into a [`toasts`] host.
///
/// Clone it freely; all clones share the same queue. Mount exactly one
/// `toasts!` host per handle.
#[derive(Clone, Default)]
pub struct Toaster {
    entries: Rc<MutableVec<Rc<ToastEntry>>>,
}

impl Toaster {
    pub fn push(&self, options: ToastOptions) -> u64 {
        static TOAST_ID: AtomicU64 = AtomicU64::new(0);

        let id = TOAST_ID.fetch_add(1, Ordering::Relaxed);

        self.entries
            .lock_mut()
            .push_cloned(Rc::new(ToastEntry { id, options }));

        id
    }

    pub fn dismiss(&self, id: u64) {
        self.entries.lock_mut().retain(|entry| entry.id != id);
    }

    pub fn clear(&self) {
        self.entries.lock_mut().clear();
    }

    pub fn info(&self, title: impl Into<String>) -> u64 {
        self.push(ToastOptions {
            title: title.into(),
            ..Default::default()
        })
    }

    pub fn success(&self, title: impl Into<String>) -> u64 {
        self.push(ToastOptions {
            title: title.into(),
            variant: ToastVariant::Success,
            ..Default::default()
        })
    }

    pub fn warning(&self, title: impl Into<String>) -> u64 {
        self.push(ToastOptions {
            title: title.into(),
            variant: ToastVariant::Warning,
            ..Default::default()
        })
    }

    pub fn error(&self, title: impl Into<String>) -> u64 {
        self.push(ToastOptions {
            title: title.into(),
            variant: ToastVariant::Error,
            ..Default::default()
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    BottomCenter,
}

/// The fixed-position host that renders a [`Toaster`]'s queue.
///
/// Mount once (e.g. at the app root). The container is an `aria-live="polite"`
/// region layered above modals; Error toasts additionally announce assertively
/// via `role="alert"`. Toasts with a `duration_ms` dismiss themselves — the
/// timer is cancelled if the host is dropped first.
#[component(render_fn = toasts)]
struct Toasts {
    #[default(Toaster::default())]
    toaster: Toaster,

    #[default(ToastPosition::BottomRight)]
    position: ToastPosition,
}

pub fn toasts(props: ToastsProps) -> Dom {
    let ToastsProps {
        toaster,
        position,
        apply,
    } = props;

    // Portalled to the body so a transformed ancestor can never capture the
    // fixed positioning (see utils::body_portal).
    crate::utils::body_portal(html!("div", {
        .attr("aria-live", "polite")
        .attr("role", "status")
        .dwclass!("flex flex-col gap-2")
        .style("position", "fixed")
        .style("z-index", layers::TOAST)
        .style("pointer-events", "none")
        .apply(move |b| {
            let b = match position {
                ToastPosition::TopLeft | ToastPosition::TopRight => b.style("top", "1rem"),
                _ => b.style("bottom", "1rem"),
            };

            match position {
                ToastPosition::TopLeft | ToastPosition::BottomLeft => b.style("left", "1rem"),
                ToastPosition::TopRight | ToastPosition::BottomRight => b.style("right", "1rem"),
                ToastPosition::BottomCenter => {
                    b.style("left", "50%").style("transform", "translateX(-50%)")
                }
            }
        })
        .children_signal_vec(toaster.entries.signal_vec_cloned().map(clone!(toaster => move |entry| {
            render_toast(&toaster, entry)
        })))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    }))
}

fn render_toast(toaster: &Toaster, entry: Rc<ToastEntry>) -> Dom {
    let toaster = toaster.clone();
    let id = entry.id;
    let options = &entry.options;

    html!("div", {
        .apply_if(options.variant == ToastVariant::Error, |b| b.attr("role", "alert"))
        .dwclass!("flex flex-row gap-3 p-3 rounded-lg border-l-2 shadow-lg w-80")
        .dwclass!("dwui-bg-void-800 dwui-text-on-primary-100")
        .dwclass!("is(.light *):dwui-bg-void-100 is(.light *):dwui-text-on-primary-900")
        .apply(|b| match options.variant {
            ToastVariant::Info => dwclass!(b, "dwui-border-primary-500"),
            ToastVariant::Success => dwclass!(b, "dwui-border-success-500"),
            ToastVariant::Warning => dwclass!(b, "dwui-border-warning-500"),
            ToastVariant::Error => dwclass!(b, "dwui-border-error-500"),
        })
        .style("pointer-events", "auto")
        .style("max-width", "90vw")
        .style("animation", "dwui-toast-in 200ms ease-out")
        .apply_if(options.duration_ms.is_some(), |b| {
            let duration = options.duration_ms.unwrap();

            b.future(clone!(toaster => async move {
                gloo_timers::future::TimeoutFuture::new(duration).await;
                toaster.dismiss(id);
            }))
        })
        .child(html!("div", {
            .dwclass!("flex flex-col gap-1 grow")
            .child(html!("div", {
                .dwclass!("font-bold text-sm")
                .text(&options.title)
            }))
            .apply_if(options.message.is_some(), |b| {
                b.child(html!("div", {
                    .dwclass!("text-sm")
                    .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700")
                    .text(options.message.as_deref().unwrap_or_default())
                }))
            })
        }))
        .apply_if(options.dismissible, |b| {
            b.child(html!("button", {
                .attr("type", "button")
                .attr("aria-label", "Dismiss")
                .dwclass!("flex-none w-6 h-6 flex align-items-center justify-center rounded-full cursor-pointer")
                .dwclass!("bg-transparent border-none dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700")
                .dwclass!("hover:dwui-bg-void-700 is(.light *):hover:dwui-bg-void-200 transition-colors")
                .dwclass!("dwui-focusable")
                .child(svg!("svg", {
                    .attr("viewBox", "0 0 12 12")
                    .attr("width", "12")
                    .attr("height", "12")
                    .attr("fill", "none")
                    .attr("aria-hidden", "true")
                    .child(svg!("path", {
                        .attr("d", "M2 2 L10 10 M10 2 L2 10")
                        .attr("stroke", "currentColor")
                        .attr("stroke-width", "1.5")
                        .attr("stroke-linecap", "round")
                    }))
                }))
                .event(clone!(toaster => move |_: events::Click| {
                    toaster.dismiss(id);
                }))
            }))
        })
    })
}
