use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

pub struct TabItem {
    pub label: String,
    pub content: Dom,
}

#[component(render_fn = glass_tabs)]
struct GlassTabs {
    #[default(vec![])]
    tabs: Vec<TabItem>,

    #[default(Box::new(Mutable::new(0usize)))]
    active_index: dyn GlassTabIndex + 'static,
}

pub trait GlassTabIndex {
    fn get_signal(&self) -> futures_signals::signal::LocalBoxSignal<'static, usize>;
    fn set(&self, index: usize);
}

impl GlassTabIndex for Mutable<usize> {
    fn get_signal(&self) -> futures_signals::signal::LocalBoxSignal<'static, usize> {
        self.signal().boxed_local()
    }

    fn set(&self, index: usize) {
        Mutable::set(self, index);
    }
}

pub fn glass_tabs(props: GlassTabsProps) -> Dom {
    let GlassTabsProps {
        tabs,
        active_index,
        apply,
    } = props;

    let active_index = std::rc::Rc::new(active_index);
    let active_signal = active_index.get_signal().broadcast();

    // Build tab headers
    let tab_labels: Vec<String> = tabs.iter().map(|t| t.label.clone()).collect();
    let tab_contents: Vec<Dom> = tabs.into_iter().map(|t| t.content).collect();

    html!("div", {
        .dwclass!("flex flex-col")

        // Tab list
        .child(html!("div", {
            .dwclass!("flex")
            .attr("role", "tablist")
            .style("border-bottom", "var(--glass-border-width) solid var(--glass-border-color)")

            .children(tab_labels.into_iter().enumerate().map({
                let active_index = active_index.clone();
                let active_signal = active_signal.clone();
                move |(i, label)| {
                let active_signal = active_signal.clone();
                let active_index = active_index.clone();
                let hover = Mutable::new(false);
                html!("button", {
                    .dwclass!("flex-1 px-4 py-2.5 text-sm font-medium cursor-pointer transition-all")
                    .style("border", "none")
                    .style("background", "transparent")
                    .style("border-bottom", "2px solid transparent")
                    .style("margin-bottom", "calc(-1 * var(--glass-border-width))")
                    .style("transition-duration", "var(--glass-transition)")
                    .style("transition-timing-function", "var(--glass-ease)")
                    .attr("role", "tab")
                    .attr_signal("aria-selected", active_signal.signal().map(move |a| {
                        if a == i { "true" } else { "false" }
                    }))

                    .style_signal("border-bottom-color", {
                        let active_signal = active_signal.clone();
                        let hover = hover.clone();
                        map_ref! {
                            let a = active_signal.signal(),
                            let h = hover.signal() => {
                                if *a == i {
                                    "var(--glass-accent)"
                                } else if *h {
                                    "var(--glass-border-color-hover)"
                                } else {
                                    "transparent"
                                }
                            }
                        }
                    })
                    .style_signal("color", {
                        let active_signal = active_signal.clone();
                        let hover = hover.clone();
                        map_ref! {
                            let a = active_signal.signal(),
                            let h = hover.signal() => {
                                if *a == i {
                                    "var(--glass-accent)"
                                } else if *h {
                                    "var(--glass-text-primary)"
                                } else {
                                    "var(--glass-text-secondary)"
                                }
                            }
                        }
                    })

                    .event(clone!(hover => move |_: events::MouseEnter| { hover.set(true); }))
                    .event(clone!(hover => move |_: events::MouseLeave| { hover.set(false); }))
                    .text(&label)
                    .event(move |_: events::Click| {
                        active_index.set(i);
                    })
                })
            }}).collect::<Vec<_>>())
        }))

        // Tab panels
        .child(html!("div", {
            .dwclass!("pt-4")
            .attr("role", "tabpanel")
            .children(tab_contents.into_iter().enumerate().map(|(i, content)| {
                let active_signal = active_signal.clone();
                html!("div", {
                    .style_signal("display", active_signal.signal().map(move |a| {
                        if a == i { "block" } else { "none" }
                    }))
                    .child(content)
                })
            }).collect::<Vec<_>>())
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
