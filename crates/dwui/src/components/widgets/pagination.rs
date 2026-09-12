use crate::theme::prelude::*;
use dominator::{clone, events, html, svg, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::rc::Rc;

/// One rendered element of a pagination bar.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PageItem {
    Page(usize),
    Ellipsis,
}

/// Computes the visible page window: always the first and last page,
/// `sibling_count` pages around the current one, and ellipses for any gap
/// wider than a single page.
pub fn page_window(page: usize, total_pages: usize, sibling_count: usize) -> Vec<PageItem> {
    let total = total_pages.max(1);
    let page = page.clamp(1, total);

    let low = page.saturating_sub(sibling_count).max(1);
    let high = (page + sibling_count).min(total);

    let mut out = vec![];

    if low > 1 {
        out.push(PageItem::Page(1));

        if low > 2 {
            out.push(PageItem::Ellipsis);
        }
    }

    out.extend((low..=high).map(PageItem::Page));

    if high < total {
        if high + 1 < total {
            out.push(PageItem::Ellipsis);
        }

        out.push(PageItem::Page(total));
    }

    out
}

/// An accessible pagination bar.
///
/// Renders a `<nav>` of page buttons with previous/next arrows. The current
/// page is controlled through the 1-based `page` signal and marked with
/// `aria-current="page"`; activating any button invokes `on_page_change` with
/// the target page. Plain tab order — every button is a tab stop, per the
/// WAI-ARIA guidance for pagination navigation.
#[component(render_fn = pagination)]
struct Pagination {
    #[signal]
    #[default(1)]
    page: usize,

    #[signal]
    #[default(1)]
    total_pages: usize,

    #[default(Box::new(|_|{}))]
    on_page_change: dyn Fn(usize) + 'static,

    /// Pages shown on each side of the current page
    #[default(1)]
    sibling_count: usize,

    #[signal]
    #[default("Pagination".to_string())]
    aria_label: String,
}

pub fn pagination(props: PaginationProps) -> Dom {
    let PaginationProps {
        page,
        total_pages,
        on_page_change,
        sibling_count,
        aria_label,
        apply,
    } = props;

    let on_page_change = Rc::new(on_page_change);
    let page = page.broadcast();
    let total_pages = total_pages.broadcast();

    // Click handlers need current values, not signals
    let page_state = Mutable::new(1usize);
    let total_state = Mutable::new(1usize);

    let arrow_button = |label: &'static str, path: &'static str, delta: i32| {
        html!("button", {
            .attr("type", "button")
            .attr("aria-label", label)
            .dwclass!("w-8 h-8 flex align-items-center justify-center rounded-md cursor-pointer flex-none")
            .dwclass!("bg-transparent border-none transition-colors dwui-focusable")
            .dwclass!("dwui-text-on-primary-300 hover:dwui-bg-void-800")
            .dwclass!("is(.light *):dwui-text-on-primary-700 is(.light *):hover:dwui-bg-void-200")
            .dwclass!("disabled:opacity-60 disabled:cursor-not-allowed")
            .attr_signal("disabled", map_ref! {
                let page = page.signal(),
                let total = total_pages.signal() => {
                    let at_edge = if delta < 0 { *page <= 1 } else { *page >= (*total).max(1) };

                    if at_edge { Some("disabled") } else { None }
                }
            })
            .child(svg!("svg", {
                .attr("viewBox", "0 0 14 14")
                .attr("width", "14")
                .attr("height", "14")
                .attr("fill", "none")
                .attr("aria-hidden", "true")
                .child(svg!("path", {
                    .attr("d", path)
                    .attr("stroke", "currentColor")
                    .attr("stroke-width", "1.5")
                    .attr("stroke-linecap", "round")
                    .attr("stroke-linejoin", "round")
                }))
            }))
            .event(clone!(on_page_change, page_state, total_state => move |_: events::Click| {
                let current = page_state.get() as i32;
                let total = total_state.get().max(1) as i32;
                let target = (current + delta).clamp(1, total);

                if target != current {
                    (on_page_change)(target as usize);
                }
            }))
        })
    };

    html!("nav", {
        .attr_signal("aria-label", aria_label)
        .future(page.signal().for_each(clone!(page_state => move |v| {
            page_state.set(v);
            async {}
        })))
        .future(total_pages.signal().for_each(clone!(total_state => move |v| {
            total_state.set(v);
            async {}
        })))
        .child(html!("div", {
            .dwclass!("flex flex-row align-items-center gap-1")
            .child(arrow_button("Previous page", "M9 3 L5 7 L9 11", -1))
            .children_signal_vec(map_ref! {
                let page = page.signal(),
                let total = total_pages.signal() =>
                    page_window(*page, *total, sibling_count)
                        .into_iter()
                        .map(|item| (item, *page))
                        .collect::<Vec<_>>()
            }.map(clone!(on_page_change => move |items| {
                items
                    .into_iter()
                    .map(|(item, current)| match item {
                        PageItem::Ellipsis => html!("span", {
                            .attr("aria-hidden", "true")
                            .dwclass!("w-8 h-8 flex align-items-center justify-center select-none text-sm")
                            .dwclass!("dwui-text-on-primary-500")
                            .text("…")
                        }),
                        PageItem::Page(n) => {
                            let is_current = n == current;

                            html!("button", {
                                .attr("type", "button")
                                .attr("aria-label", &format!("Page {}", n))
                                .apply_if(is_current, |b| b.attr("aria-current", "page"))
                                .dwclass!("w-8 h-8 flex align-items-center justify-center rounded-md cursor-pointer text-sm flex-none")
                                .dwclass!("border-none transition-colors dwui-focusable")
                                .apply(move |b| {
                                    if is_current {
                                        dwclass!(b, "dwui-bg-primary-500 dwui-text-on-primary-950 font-bold is(.light *):dwui-bg-primary-400")
                                    } else {
                                        dwclass!(b, "bg-transparent dwui-text-on-primary-300 hover:dwui-bg-void-800 is(.light *):dwui-text-on-primary-700 is(.light *):hover:dwui-bg-void-200")
                                    }
                                })
                                .text(&n.to_string())
                                .event(clone!(on_page_change => move |_: events::Click| {
                                    (on_page_change)(n);
                                }))
                            })
                        }
                    })
                    .collect::<Vec<_>>()
            })).to_signal_vec())
            .child(arrow_button("Next page", "M5 3 L9 7 L5 11", 1))
        }))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}

#[cfg(test)]
mod test {
    use super::{page_window, PageItem};

    fn pages(items: &[PageItem]) -> Vec<Option<usize>> {
        items
            .iter()
            .map(|i| match i {
                PageItem::Page(n) => Some(*n),
                PageItem::Ellipsis => None,
            })
            .collect()
    }

    #[test]
    fn small_totals_render_every_page() {
        assert_eq!(
            pages(&page_window(2, 3, 1)),
            vec![Some(1), Some(2), Some(3)]
        );
    }

    #[test]
    fn middle_pages_get_ellipses_on_both_sides() {
        assert_eq!(
            pages(&page_window(5, 10, 1)),
            vec![
                Some(1),
                None,
                Some(4),
                Some(5),
                Some(6),
                None,
                Some(10)
            ]
        );
    }

    #[test]
    fn edges_do_not_produce_single_page_gaps() {
        // low = 2, so page 1 is adjacent: no ellipsis
        assert_eq!(
            pages(&page_window(3, 10, 1)),
            vec![Some(1), Some(2), Some(3), Some(4), None, Some(10)]
        );
    }

    #[test]
    fn out_of_range_input_is_clamped() {
        assert_eq!(pages(&page_window(99, 3, 1)), vec![Some(1), Some(2), Some(3)]);
        assert_eq!(pages(&page_window(1, 0, 1)), vec![Some(1)]);
    }
}
