use dominator::{html, text, Dom};
use dwind::prelude::*;
use dwind_glass::prelude::*;
use futures_signals::signal::{Mutable, SignalExt};

use crate::helpers::section;

const BREADCRUMBS_CODE: &str = r#"glass_breadcrumbs!({
    .items(vec![
        BreadcrumbItem { label: "Home".into(), on_click: Some(Box::new(|| {})) },
        BreadcrumbItem { label: "Components".into(), on_click: Some(Box::new(|| {})) },
        BreadcrumbItem { label: "Current".into(), on_click: None },
    ])
})"#;

const TABS_CODE: &str = r#"glass_tabs!({
    .tabs(vec![
        TabItem {
            label: "Overview".into(),
            content: html!("div", { .text("Tab content here.") }),
        },
        TabItem {
            label: "Settings".into(),
            content: html!("div", { .text("Settings content.") }),
        },
    ])
})"#;

const MENU_CODE: &str = r#"glass_menu!({
    .trigger(Some(glass_button!({
        .content(Some(text("Actions")))
        .variant(ButtonVariant::Glass)
    })))
    .items(vec![
        GlassMenuEntry::Item(GlassMenuItem {
            label: "Edit".into(),
            icon: None,
            on_click: Box::new(|| {}),
            disabled: false,
            variant: MenuItemVariant::Default,
        }),
        GlassMenuEntry::Divider,
        GlassMenuEntry::Item(GlassMenuItem {
            label: "Delete".into(),
            icon: None,
            on_click: Box::new(|| {}),
            disabled: false,
            variant: MenuItemVariant::Danger,
        }),
    ])
})"#;

const SIDEBAR_CODE: &str = r#"let collapsed = Mutable::new(false);

glass_sidebar!({
    .collapsed_signal(collapsed.signal())
    .header(Some(html!("span", { .text("Menu") })))
    .items(vec![
        glass_nav_item!({
            .label("Dashboard".to_string())
            .icon(Some(html!("span", { .text("■") })))
            .active_signal(/* ... */)
            .on_click(Box::new(|| { /* ... */ }))
        }),
    ])
})"#;

pub fn navigation_page() -> Dom {
    let sidebar_collapsed = Mutable::new(false);
    let active_sidebar_item = Mutable::new(0usize);

    html!("div", {
        .dwclass!("flex flex-col gap-8")

        .child(html!("div", {
            .dwclass!("mb-4")
            .child(html!("h1", {
                .dwclass!("text-3xl font-bold glass-text-primary mb-2")
                .text("Navigation Components")
            }))
            .child(html!("p", {
                .dwclass!("text-base glass-text-secondary")
                .text("Components for structuring app navigation and wayfinding.")
            }))
        }))

        // Breadcrumbs
        .child(section("Breadcrumbs", "section-breadcrumbs", Some(BREADCRUMBS_CODE), vec![
            glass_breadcrumbs!({
                .items(vec![
                    BreadcrumbItem { label: "Home".into(), on_click: Some(Box::new(|| {})) },
                    BreadcrumbItem { label: "Components".into(), on_click: Some(Box::new(|| {})) },
                    BreadcrumbItem { label: "Navigation".into(), on_click: None },
                ])
            }),
        ]))

        // Tabs
        .child(section("Tabs", "section-tabs", Some(TABS_CODE), vec![
            glass_tabs!({
                .tabs(vec![
                    TabItem {
                        label: "Overview".into(),
                        content: html!("div", {
                            .dwclass!("glass-text-secondary")
                            .text("Overview tab content. Tabs use an underline indicator for the active state.")
                        }),
                    },
                    TabItem {
                        label: "Settings".into(),
                        content: html!("div", {
                            .dwclass!("glass-text-secondary")
                            .text("Settings tab content. The active tab shows an accent-colored underline.")
                        }),
                    },
                    TabItem {
                        label: "Activity".into(),
                        content: html!("div", {
                            .dwclass!("glass-text-secondary")
                            .text("Activity tab content. Each panel maintains its own content independently.")
                        }),
                    },
                ])
            }),
        ]))

        // Dropdown Menu
        .child(section("Dropdown Menu", "section-dropdown-menu", Some(MENU_CODE), vec![
            glass_menu!({
                .trigger(Some(
                    glass_button!({
                        .content(Some(text("Actions \u{25BC}")))
                        .variant(ButtonVariant::Glass)
                    })
                ))
                .items(vec![
                    GlassMenuEntry::Item(GlassMenuItem {
                        label: "Edit".into(),
                        icon: Some(html!("span", { .text("\u{270F}") })),
                        on_click: Box::new(|| {}),
                        disabled: false,
                        variant: MenuItemVariant::Default,
                    }),
                    GlassMenuEntry::Item(GlassMenuItem {
                        label: "Duplicate".into(),
                        icon: Some(html!("span", { .text("\u{29C9}") })),
                        on_click: Box::new(|| {}),
                        disabled: false,
                        variant: MenuItemVariant::Default,
                    }),
                    GlassMenuEntry::Item(GlassMenuItem {
                        label: "Archive".into(),
                        icon: None,
                        on_click: Box::new(|| {}),
                        disabled: true,
                        variant: MenuItemVariant::Default,
                    }),
                    GlassMenuEntry::Divider,
                    GlassMenuEntry::Item(GlassMenuItem {
                        label: "Delete".into(),
                        icon: Some(html!("span", { .text("\u{2715}") })),
                        on_click: Box::new(|| {}),
                        disabled: false,
                        variant: MenuItemVariant::Danger,
                    }),
                ])
            }),
        ]))

        // Sidebar demo
        .child(section("Sidebar", "section-sidebar", Some(SIDEBAR_CODE), vec![
            html!("div", {
                .dwclass!("flex gap-4")
                .child(glass_button!({
                    .content(Some(text("Toggle Sidebar")))
                    .variant(ButtonVariant::Ghost)
                    .on_click({
                        let sidebar_collapsed = sidebar_collapsed.clone();
                        Box::new(move |_: dominator::events::Click| {
                            sidebar_collapsed.set(!sidebar_collapsed.get());
                        })
                    })
                }))
            }),
            html!("div", {
                .style("height", "300px")
                .dwclass!("flex")
                .style("border-radius", "var(--glass-border-radius-xl)")
                .style("overflow", "hidden")
                .child(glass_sidebar!({
                    .collapsed_signal(sidebar_collapsed.signal())
                    .header(Some(html!("span", {
                        .dwclass!("font-semibold glass-text-primary")
                        .text("Menu")
                    })))
                    .items(vec![
                        make_nav_item("\u{25A3}", "Dashboard", 0, &active_sidebar_item, &sidebar_collapsed),
                        make_nav_item("\u{2197}", "Analytics", 1, &active_sidebar_item, &sidebar_collapsed),
                        make_nav_item("\u{2699}", "Settings", 2, &active_sidebar_item, &sidebar_collapsed),
                        make_nav_item("\u{2753}", "Help", 3, &active_sidebar_item, &sidebar_collapsed),
                    ])
                }))
                .child(html!("div", {
                    .dwclass!("flex-1 flex items-center justify-center glass-text-tertiary")
                    .text("Main content area")
                }))
            }),
        ]))
    })
}

fn make_nav_item(
    icon: &str,
    label: &str,
    index: usize,
    active_item: &Mutable<usize>,
    collapsed: &Mutable<bool>,
) -> Dom {
    let icon_text = icon.to_string();
    glass_nav_item!({
        .label(label.to_string())
        .icon(Some(html!("span", { .text(&icon_text) })))
        .active_signal(active_item.signal().map(move |a| a == index))
        .collapsed_signal(collapsed.signal())
        .on_click({
            let active_item = active_item.clone();
            Box::new(move || { active_item.set(index); })
        })
    })
}
