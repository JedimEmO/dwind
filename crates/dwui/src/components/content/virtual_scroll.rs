use crate::components::widgets::spinner::{spinner, SpinnerProps, SpinnerSize};
use crate::theme::prelude::*;
use dominator::{clone, events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::rc::Rc;

/// How many extra rows to render above and below the visible window
const OVERSCAN: usize = 5;

/// How many rows from the end before `on_reach_end` fires
const REACH_END_THRESHOLD: usize = 10;

/// A windowed (virtualized) list with infinite-loading support.
///
/// Only the rows visible in the scroll viewport (plus a small overscan) are
/// rendered, so lists of tens of thousands of items stay cheap. Items are
/// rendered on demand through `render_item(index)`. When scrolling brings
/// the window near the end of the list, `on_reach_end` fires once per
/// item-count so consumers can append more rows (infinite scroll); the
/// `loading` signal shows a spinner row and is exposed as `aria-busy`.
///
/// Rows must have a fixed pixel height (`item_height`).
#[component(render_fn = virtual_scroll)]
struct VirtualScroll {
    #[signal]
    #[default(0)]
    item_count: usize,

    /// Fixed pixel height of every row
    #[default(40.0)]
    item_height: f64,

    /// Pixel height of the scroll viewport
    #[default(400.0)]
    height: f64,

    #[required]
    render_item: dyn Fn(usize) -> Dom + 'static,

    #[default(Box::new(|| {}))]
    on_reach_end: dyn Fn() + 'static,

    #[signal]
    #[default(false)]
    loading: bool,

    /// Accessible name for the list
    #[signal]
    #[default("List".to_string())]
    #[into]
    aria_label: String,
}

pub fn virtual_scroll(props: VirtualScrollProps) -> Dom {
    let VirtualScrollProps {
        item_count,
        item_height,
        height,
        render_item,
        on_reach_end,
        loading,
        aria_label,
        apply,
    } = props;

    let render_item = Rc::new(render_item);
    let on_reach_end = Rc::new(on_reach_end);

    let item_count = item_count.broadcast();
    let loading = loading.broadcast();
    let count_state = Mutable::new(0usize);
    let scroll_top = Mutable::new(0.0f64);
    // last item count for which on_reach_end already fired
    let last_requested_at = Mutable::new(None::<usize>);

    let visible_range = map_ref! {
        let scroll_top = scroll_top.signal(),
        let count = item_count.signal() => {
            let first = (scroll_top / item_height).floor() as usize;
            let visible = (height / item_height).ceil() as usize + 1;

            let start = first.saturating_sub(OVERSCAN);
            let end = (first + visible + OVERSCAN).min(*count);

            (start, end)
        }
    }
    .dedupe()
    .broadcast();

    html!("div", {
        .future(item_count.signal().for_each(clone!(count_state => move |v| {
            count_state.set(v);
            async {}
        })))
        .attr("role", "list")
        .attr_signal("aria-label", aria_label)
        .attr_signal("aria-busy", loading.signal().map(|v| if v { Some("true") } else { None }))
        .dwclass!("overflow-y-auto w-full rounded-lg border")
        .dwclass!("dwui-border-void-700 is(.light *):dwui-border-void-300")
        .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-100")
        .style("height", format!("{height}px"))
        .with_node!(element => {
            .event(clone!(scroll_top, count_state, last_requested_at, on_reach_end => move |_: events::Scroll| {
                let top = element.scroll_top() as f64;
                scroll_top.set_neq(top);

                let count = count_state.get();
                let last_visible = ((top + height) / item_height).ceil() as usize;

                if count > 0
                    && last_visible + REACH_END_THRESHOLD >= count
                    && last_requested_at.get() != Some(count)
                {
                    last_requested_at.set(Some(count));
                    (on_reach_end)();
                }
            }))
        })
        // total-height sizer keeps the scrollbar honest
        .child(html!("div", {
            .style("position", "relative")
            .style_signal("height", map_ref! {
                let count = item_count.signal(),
                let loading = loading.signal() => {
                    let spinner_row = if *loading { 1.0 } else { 0.0 };
                    format!("{}px", (*count as f64 + spinner_row) * item_height)
                }
            })
            // windowed rows
            .children_signal_vec(
                map_ref! {
                    let range = visible_range.signal(),
                    let count = item_count.signal() => (*range, *count)
                }
                .map(clone!(render_item => move |((start, end), count)| {
                    (start..end).map(|index| {
                        html!("div", {
                            .attr("role", "listitem")
                            .attr("aria-setsize", &count.to_string())
                            .attr("aria-posinset", &(index + 1).to_string())
                            .attr("data-vs-index", &index.to_string())
                            .style("position", "absolute")
                            .style("top", format!("{}px", index as f64 * item_height))
                            .style("left", "0")
                            .style("right", "0")
                            .style("height", format!("{item_height}px"))
                            .child((render_item)(index))
                        })
                    }).collect::<Vec<_>>()
                }))
                .to_signal_vec()
            )
            // loading row pinned after the last item
            .child_signal(map_ref! {
                let loading = loading.signal(),
                let count = item_count.signal() => {
                    if *loading {
                        Some((*count as f64) * item_height)
                    } else {
                        None
                    }
                }
            }.map(|top| {
                top.map(|top| {
                    html!("div", {
                        .style("position", "absolute")
                        .style("top", format!("{top}px"))
                        .style("left", "0")
                        .style("right", "0")
                        .dwclass!("flex justify-center align-items-center p-2")
                        .child(spinner(
                            SpinnerProps::new()
                                .size(SpinnerSize::Small)
                                .label("Loading more"),
                        ))
                    })
                })
            }))
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}
