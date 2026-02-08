use dominator::{html, text, Dom};
use dwind::prelude::*;
use dwind_glass::prelude::*;
use futures_signals::signal::Mutable;

use crate::helpers::section;

const AVATAR_CODE: &str = r#"glass_avatar!({
    .size(AvatarSize::Large)
    .fallback("JD".to_string())
})

// With image
glass_avatar!({
    .size(AvatarSize::Medium)
    .src(Some("https://example.com/photo.jpg".to_string()))
    .fallback("JD".to_string())
})"#;

const STAT_CARDS_CODE: &str = r#"glass_stat_card!({
    .label("Revenue".to_string())
    .value("$48,290".to_string())
    .trend(Some(StatTrend::Up(12.5)))
})"#;

const PROGRESS_CODE: &str = r#"let progress = Mutable::new(0.65);

glass_progress_bar!({
    .value_signal(progress.signal())
    .label(Some("Upload progress".to_string()))
    .show_percentage(true)
    .variant(ProgressVariant::Accent)
})"#;

const TABLE_CODE: &str = r#"glass_table!({
    .columns(vec!["Name".into(), "Role".into(), "Status".into()])
    .rows(vec![
        vec![text("Alice"), text("Engineer"), text("Active")],
        vec![text("Bob"), text("Designer"), text("Away")],
    ])
    .striped(true)
})"#;

const TOOLTIP_CODE: &str = r#"glass_text_tooltip(
    glass_button!({
        .content(Some(text("Hover me")))
        .variant(ButtonVariant::Ghost)
    }),
    "Tooltip text",
    TooltipPosition::Top,
)"#;

pub fn data_page() -> Dom {
    let progress = Mutable::new(0.65);

    html!("div", {
        .dwclass!("flex flex-col gap-8")

        .child(html!("div", {
            .dwclass!("mb-4")
            .child(html!("h1", {
                .dwclass!("text-3xl font-bold glass-text-primary mb-2")
                .text("Data Display Components")
            }))
            .child(html!("p", {
                .dwclass!("text-base glass-text-secondary")
                .text("Components for presenting data, metrics, and visual information.")
            }))
        }))

        // Avatars
        .child(section("Avatar", "section-avatar", Some(AVATAR_CODE), vec![
            html!("div", {
                .dwclass!("flex items-center gap-4")
                .child(glass_avatar!({
                    .size(AvatarSize::Small)
                    .fallback("JD".to_string())
                }))
                .child(glass_avatar!({
                    .size(AvatarSize::Medium)
                    .fallback("AB".to_string())
                }))
                .child(glass_avatar!({
                    .size(AvatarSize::Large)
                    .fallback("XY".to_string())
                }))
            }),
        ]))

        // Stat Cards
        .child(section("Stat Cards", "section-stat-cards", Some(STAT_CARDS_CODE), vec![
            html!("div", {
                .dwclass!("grid gap-4")
                .style("grid-template-columns", "repeat(auto-fit, minmax(200px, 1fr))")
                .child(glass_stat_card!({
                    .label("Revenue".to_string())
                    .value("$48,290".to_string())
                    .trend(Some(StatTrend::Up(12.5)))
                }))
                .child(glass_stat_card!({
                    .label("Users".to_string())
                    .value("2,847".to_string())
                    .trend(Some(StatTrend::Up(4.2)))
                }))
                .child(glass_stat_card!({
                    .label("Bounce Rate".to_string())
                    .value("23.1%".to_string())
                    .trend(Some(StatTrend::Down(2.1)))
                }))
                .child(glass_stat_card!({
                    .label("Avg. Session".to_string())
                    .value("4m 32s".to_string())
                    .trend(Some(StatTrend::Neutral))
                }))
            }),
        ]))

        // Progress Bars
        .child(section("Progress Bar", "section-progress-bar", Some(PROGRESS_CODE), vec![
            html!("div", {
                .style("max-width", "500px")
                .dwclass!("flex flex-col gap-4")
                .child(glass_progress_bar!({
                    .value_signal(progress.signal())
                    .label(Some("Upload progress".to_string()))
                    .show_percentage(true)
                    .variant(ProgressVariant::Accent)
                }))
                .child(glass_progress_bar!({
                    .value(0.85)
                    .label(Some("Storage used".to_string()))
                    .show_percentage(true)
                    .variant(ProgressVariant::Success)
                }))
                .child(glass_progress_bar!({
                    .value(0.45)
                    .label(Some("CPU usage".to_string()))
                    .show_percentage(true)
                    .variant(ProgressVariant::Warning)
                }))
                .child(glass_progress_bar!({
                    .value(0.92)
                    .label(Some("Disk almost full".to_string()))
                    .show_percentage(true)
                    .variant(ProgressVariant::Error)
                }))
            }),
        ]))

        // Table
        .child(section("Table", "section-table", Some(TABLE_CODE), vec![
            glass_table!({
                .columns(vec![
                    "Name".into(),
                    "Role".into(),
                    "Status".into(),
                    "Last Active".into(),
                ])
                .rows(vec![
                    vec![text("Alice Chen"), text("Engineer"), text("Active"), text("2 min ago")],
                    vec![text("Bob Smith"), text("Designer"), text("Away"), text("1 hour ago")],
                    vec![text("Carol Wu"), text("Manager"), text("Active"), text("5 min ago")],
                    vec![text("David Kim"), text("Engineer"), text("Offline"), text("2 days ago")],
                ])
                .striped(true)
            }),
        ]))

        // Tooltip
        .child(section("Tooltip", "section-tooltip", Some(TOOLTIP_CODE), vec![
            html!("div", {
                .dwclass!("flex gap-4")
                .child(glass_text_tooltip(
                    glass_button!({
                        .content(Some(text("Hover me (top)")))
                        .variant(ButtonVariant::Ghost)
                    }),
                    "Tooltip on top",
                    TooltipPosition::Top,
                ))
                .child(glass_text_tooltip(
                    glass_button!({
                        .content(Some(text("Hover me (bottom)")))
                        .variant(ButtonVariant::Ghost)
                    }),
                    "Tooltip on bottom",
                    TooltipPosition::Bottom,
                ))
            }),
        ]))
    })
}
