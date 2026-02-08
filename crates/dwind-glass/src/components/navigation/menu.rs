use dominator::{clone, events, html, Dom};
use dwind::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MenuItemVariant {
    Default,
    Danger,
}

pub struct GlassMenuItem {
    pub label: String,
    pub icon: Option<Dom>,
    pub on_click: Box<dyn Fn() + 'static>,
    pub disabled: bool,
    pub variant: MenuItemVariant,
}

pub enum GlassMenuEntry {
    Item(GlassMenuItem),
    Divider,
}

#[component(render_fn = glass_menu)]
struct GlassMenu {
    #[default(None)]
    trigger: Option<Dom>,

    #[default(vec![])]
    items: Vec<GlassMenuEntry>,
}

pub fn glass_menu(props: GlassMenuProps) -> Dom {
    let GlassMenuProps {
        trigger,
        items,
        apply,
    } = props;

    let is_open = Mutable::new(false);
    let just_opened = Mutable::new(false);

    html!("div", {
        .style("position", "relative")
        .dwclass!("inline-flex")

        // Trigger wrapper
        .apply_if(trigger.is_some(), {
            let is_open = is_open.clone();
            let just_opened = just_opened.clone();
            move |b| {
                b.child(html!("div", {
                    .dwclass!("cursor-pointer")
                    .child(trigger.unwrap())
                    .event(clone!(is_open, just_opened => move |_: events::Click| {
                        if !is_open.get() {
                            just_opened.set(true);
                        }
                        is_open.set(!is_open.get());
                    }))
                }))
            }
        })

        // Dropdown panel
        .child(html!("div", {
            .style("position", "absolute")
            .style("top", "100%")
            .style("left", "0")
            .style("margin-top", "4px")
            .style("z-index", "50")
            .style("min-width", "12rem")
            .style("background", "var(--glass-bg-elevated)")
            .style(["backdrop-filter", "-webkit-backdrop-filter"],
                "blur(var(--glass-blur-heavy)) saturate(var(--glass-saturation))")
            .style("border", "none")
            .style("border-radius", "var(--glass-border-radius)")
            .style("box-shadow", "var(--glass-shadow-lg)")
            .style("overflow", "hidden")
            .style("transition-property", "opacity, transform, visibility")
            .style("transition-duration", "var(--glass-transition)")
            .style("transition-timing-function", "var(--glass-ease)")
            .style("padding", "4px")

            .style_signal("opacity", is_open.signal().map(|o| if o { "1" } else { "0" }))
            .style_signal("visibility", is_open.signal().map(|o| if o { "visible" } else { "hidden" }))
            .style_signal("pointer-events", is_open.signal().map(|o| if o { "auto" } else { "none" }))
            .style_signal("transform", is_open.signal().map(|o| {
                if o { "translateY(0)" } else { "translateY(-4px)" }
            }))

            .attr("role", "menu")

            .children(items.into_iter().map({
                let is_open = is_open.clone();
                move |entry| {
                    match entry {
                        GlassMenuEntry::Divider => {
                            html!("div", {
                                .style("height", "1px")
                                .style("margin", "4px 0")
                                .style("background", "var(--glass-border-color)")
                            })
                        }
                        GlassMenuEntry::Item(item) => {
                            let GlassMenuItem { label, icon, on_click, disabled, variant } = item;
                            let item_hover = Mutable::new(false);
                            let is_danger = variant == MenuItemVariant::Danger;
                            let is_open = is_open.clone();

                            html!("div", {
                                .dwclass!("flex items-center gap-3 px-3 py-2 text-sm cursor-pointer transition-all")
                                .style("border-radius", "var(--glass-border-radius-sm)")
                                .style("transition-duration", "var(--glass-transition-fast)")
                                .style("transition-timing-function", "var(--glass-ease)")
                                .attr("role", "menuitem")

                                .style_signal("background", item_hover.signal().map(move |h| {
                                    if h && !disabled {
                                        "rgba(255, 255, 255, 0.06)"
                                    } else {
                                        "transparent"
                                    }
                                }))

                                .style("color", if disabled {
                                    "var(--glass-text-tertiary)"
                                } else if is_danger {
                                    "var(--glass-error)"
                                } else {
                                    "var(--glass-text-primary)"
                                })

                                .apply_if(disabled, |b| {
                                    b.style("opacity", "0.5")
                                     .style("pointer-events", "none")
                                })

                                // Icon
                                .apply_if(icon.is_some(), |b| {
                                    b.child(html!("span", {
                                        .dwclass!("flex items-center justify-center")
                                        .style("min-width", "16px")
                                        .child(icon.unwrap())
                                    }))
                                })

                                // Label
                                .child(html!("span", {
                                    .text(&label)
                                }))

                                .event(clone!(item_hover => move |_: events::MouseEnter| { item_hover.set(true); }))
                                .event(clone!(item_hover => move |_: events::MouseLeave| { item_hover.set(false); }))
                                .event(move |_: events::Click| {
                                    (on_click)();
                                    is_open.set(false);
                                })
                            })
                        }
                    }
                }
            }).collect::<Vec<_>>())
        }))

        // Click outside to close
        .global_event(clone!(is_open, just_opened => move |_: events::Click| {
            if just_opened.get() {
                just_opened.set(false);
            } else if is_open.get() {
                is_open.set(false);
            }
        }))

        // Escape to close
        .global_event(clone!(is_open => move |e: events::KeyDown| {
            if e.key() == "Escape" {
                is_open.set(false);
            }
        }))

        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
    })
}
