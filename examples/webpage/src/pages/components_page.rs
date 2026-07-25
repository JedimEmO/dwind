use crate::pages::docs::code_widget::code;
use crate::pages::dwui::example_small::{example_card_modal, EXAMPLE_CARD_MODAL_EXAMPLE_HTML_MAP};
use dominator::{clone, text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;
use dwui::theme::prelude::*;
use example_html_highlight_macro::example_html;
use futures_signals::signal::{Mutable, SignalExt};

pub fn components_page() -> Dom {
    let is_light = Mutable::new(false);

    html!("div", {
        .dwclass!("m-x-auto max-w-6xl p-l-4 p-r-4 w-full m-b-20")
        .child(html!("div", {
            .dwclass!("p-t-10 flex flex-col gap-3")
            .child(breadcrumbs!({
                .items(vec![
                    ("home".to_string(), "#/".to_string()),
                    ("components".to_string(), "#/components".to_string()),
                ])
            }))
            .child(html!("div", {
                .class("font-code")
                .dwclass!("text-candlelight-400 text-sm")
                .text("// dwui component gallery")
            }))
            .child(html!("h1", {
                .class("font-display")
                .dwclass!("@sm:text-5xl @<sm:text-3xl font-extrabold text-woodsmoke-50 m-0")
                .text("Every component, live.")
            }))
            .child(html!("p", {
                .dwclass!("text-woodsmoke-300 m-0")
                .style("max-width", "42rem")
                .text("Accessible, themeable building blocks rendered by this very page. \
                       Tab through them, drag them, toggle the theme — everything below is \
                       compiled Rust running as WebAssembly.")
            }))
        }))

        // Theme toggle
        .child(html!("div", {
            .dwclass!("flex align-items-center gap-4 m-t-6 m-b-4")
            .child(switch!({
                .checked_signal(is_light.signal())
                .label("Light theme preview".to_string())
                .on_change(clone!(is_light => move |checked| {
                    is_light.set(checked);
                }))
            }))
        }))

        // Themed gallery surface
        .child(html!("div", {
            .class_signal("light", is_light.signal())
            .dwclass!("dwui-bg-void-950 is(.light):dwui-bg-void-100 rounded-lg p-4 transition-colors")
            .child(html!("div", {
                .dwclass!("grid @sm:grid-cols-2 @<sm:grid-cols-1 gap-4")
                .children([
                    section_card("Buttons", "Flat, border, and text variants in three sizes — with focus rings, hover, active, and disabled states", buttons_demo(), code(&BUTTONS_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Badges", "Status pills in primary, error, void, and outline variants", badges_demo(), code(&BADGES_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Alerts", "Status callouts with severity-aware ARIA roles; the error alert is dismissible", alerts_demo(), code(&ALERTS_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Text inputs", "Floating labels, password masking, and live validation announced to screen readers", inputs_demo(), code(&INPUTS_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Select & slider", "Labelled form controls bound to reactive signals", select_slider_demo(), code(&SELECT_SLIDER_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Switch & checkbox", "Toggles with role=switch/checkbox semantics and clickable labels", toggles_demo(), code(&TOGGLES_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Progress & spinner", "Determinate, indeterminate, and spinner loading states with aria-valuenow", progress_demo(), code(&PROGRESS_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Avatars & skeletons", "Identity with initials fallback, plus pulsing loading placeholders", avatar_skeleton_demo(), code(&AVATAR_SKELETON_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Tooltip & divider", "Hover or focus the buttons to reveal tooltips", tooltip_divider_demo(), code(&TOOLTIP_DIVIDER_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Breadcrumbs", "Navigation trails with aria-current on the active page", breadcrumbs_demo(), code(&BREADCRUMBS_DEMO_EXAMPLE_HTML_MAP)),
                ])
            }))
            .child(html!("div", {
                .dwclass!("m-t-4 grid @md:grid-cols-2 @<md:grid-cols-1 gap-4")
                .child(section_card("Tabs", "WAI-ARIA tabs with arrow-key navigation and roving tabindex", tabs_demo(), code(&TABS_DEMO_EXAMPLE_HTML_MAP)))
                .child(section_card("Accordion", "Animated disclosure panels with aria-expanded and region landmarks", accordion_demo(), code(&ACCORDION_DEMO_EXAMPLE_HTML_MAP)))
            }))
            .child(html!("div", {
                .dwclass!("m-t-4")
                .child(section_card("Data table", "Sortable columns with aria-sort; sorting is controlled, so your data layer stays in charge. Click the headers.", data_table_demo(), code(&DATA_TABLE_DEMO_EXAMPLE_HTML_MAP)))
            }))
            .child(html!("div", {
                .dwclass!("m-t-4 grid @md:grid-cols-2 @<md:grid-cols-1 gap-4")
                .child(section_card("Virtual scroll", "Windowed rendering — this list has 50,000 rows but only the visible handful exist in the DOM. Scroll to the bottom to trigger infinite loading.", virtual_scroll_demo(), code(&VIRTUAL_SCROLL_DEMO_EXAMPLE_HTML_MAP)))
                .child(section_card("Date picker", "Calendar popup with full keyboard support: arrows move by day and week, PageUp/PageDown by month, Escape closes.", date_picker_demo(), code(&DATE_PICKER_DEMO_EXAMPLE_HTML_MAP)))
            }))
            .child(html!("div", {
                .dwclass!("m-t-4")
                .child(section_card("Modal dialogs", "Focus-managed dialogs with backdrop blur, escape key, and entrance animation", example_card_modal(), code(&EXAMPLE_CARD_MODAL_EXAMPLE_HTML_MAP)))
            }))
        }))

        .child(html!("div", {
            .dwclass!("m-t-10 flex flex-col gap-2")
            .child(html!("div", {
                .class("font-code")
                .dwclass!("text-candlelight-400 text-sm")
                .text("// accessibility")
            }))
            .child(html!("p", {
                .dwclass!("text-woodsmoke-400 m-0 leading-relaxed")
                .style("max-width", "46rem")
                .text("All interactive components are reachable and operable with the keyboard. \
                       Focus rings only appear for keyboard navigation (focus-visible), validation \
                       errors are announced via role=alert and aria-describedby, tab lists implement \
                       the full arrow-key roving tabindex pattern, and dialogs trap announcements \
                       with aria-modal. The whole suite is verified headless in CI.")
            }))
        }))
    })
}

fn section_card(title: &str, description: &str, demo: Dom, source: Dom) -> Dom {
    let title = title.to_string();
    let description = description.to_string();

    card!({
        .scheme(ColorScheme::Void)
        .apply(crate::fx::spotlight)
        .apply(move |b| {
            dwclass!(b, "p-6 flex flex-col gap-4")
            .child(heading!({
                .content(text(&title))
                .text_size(TextSize::Large)
                .level(HeadingLevel::H2)
            }))
            .child(html!("p", {
                .dwclass!("text-sm dwui-text-on-primary-400 is(.light *):dwui-text-on-primary-600 m-0")
                .text(&description)
            }))
            .child(demo)
            .child(source)
        })
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn buttons_demo() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .children([
            button!({
                .content(Some(text("Primary Flat")))
            }),
            button!({
                .button_type(ButtonType::Border)
                .content(Some(text("Primary Border")))
            }),
            button!({
                .button_type(ButtonType::Text)
                .content(Some(text("Text Button")))
            }),
            html!("div", {
                .dwclass!("flex flex-row gap-4 align-items-center")
                .child(html!("div", {
                    .dwclass!("grow")
                    .child(button!({
                        .size(ButtonSize::Small)
                        .content(Some(text("Small")))
                    }))
                }))
                .child(html!("div", {
                    .dwclass!("grow")
                    .child(button!({
                        .size(ButtonSize::Large)
                        .content(Some(text("Large")))
                    }))
                }))
            }),
            button!({
                .disabled(true)
                .content(Some(text("Disabled")))
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn badges_demo() -> Dom {
    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-4 align-items-center")
        .children([
            badge!({
                .content(Some(text("Primary")))
                .variant(BadgeVariant::Primary)
            }),
            badge!({
                .content(Some(text("Error")))
                .variant(BadgeVariant::Error)
            }),
            badge!({
                .content(Some(text("Void")))
                .variant(BadgeVariant::Void)
            }),
            badge!({
                .content(Some(text("Outline")))
                .variant(BadgeVariant::Outline)
            }),
            badge!({
                .content(Some(text("99+")))
                .variant(BadgeVariant::Error)
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn alerts_demo() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-3")
        .children([
            alert!({
                .variant(AlertVariant::Info)
                .title("Heads up".to_string())
                .content(Some(text("A new version of dwui is available.")))
            }),
            alert!({
                .variant(AlertVariant::Success)
                .title("Deployed".to_string())
                .content(Some(text("Build #214 went live without warnings.")))
            }),
            alert!({
                .variant(AlertVariant::Warning)
                .title("Heavy load".to_string())
            }),
            alert!({
                .variant(AlertVariant::Error)
                .title("Build failed".to_string())
                .content(Some(text("Click × to dismiss this alert.")))
                .dismissible(true)
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn inputs_demo() -> Dom {
    let value = Mutable::new("".to_string());

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .children([
            text_input!({
                .value(value.clone())
                .label("Your name".to_string())
            }),
            text_input!({
                .input_type(TextInputType::Password)
                .label("Password".to_string())
            }),
            text_input!({
                .value(value.clone())
                .is_valid_signal(value.signal_ref(|v| {
                    if v.to_lowercase() == "bananas" {
                        ValidationResult::Valid
                    } else {
                        ValidationResult::Invalid { message: "Give me bananas!".to_string() }
                    }
                }))
                .label("Accepts bananas".to_string())
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn select_slider_demo() -> Dom {
    let selected = Mutable::new("a".to_string());
    let amount = Mutable::new(25.0f32);

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .children([
            select!({
                .label("Favourite option".to_string())
                .value(selected.clone())
                .options(vec![
                    ("a".to_string(), "Option A".to_string()),
                    ("b".to_string(), "Option B".to_string()),
                    ("c".to_string(), "Option C".to_string()),
                ])
            }),
            slider!({
                .value(amount.clone())
                .label("Amount".to_string())
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn toggles_demo() -> Dom {
    let notifications = Mutable::new(true);
    let newsletter = Mutable::new(false);
    let terms = Mutable::new(false);

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .children([
            switch!({
                .checked_signal(notifications.signal())
                .label("Enable notifications".to_string())
                .on_change(clone!(notifications => move |v| notifications.set(v)))
            }),
            switch!({
                .checked(false)
                .disabled(true)
                .label("Disabled switch".to_string())
            }),
            checkbox!({
                .checked_signal(newsletter.signal())
                .label("Subscribe to newsletter".to_string())
                .on_change(clone!(newsletter => move |v| newsletter.set(v)))
            }),
            checkbox!({
                .checked_signal(terms.signal())
                .label("Accept the terms".to_string())
                .on_change(clone!(terms => move |v| terms.set(v)))
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn progress_demo() -> Dom {
    let value = Mutable::new(40.0f32);

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .children([
            progress!({
                .value_signal(value.signal().map(|v| v as f64))
                .label("Demo progress".to_string())
            }),
            progress!({
                .indeterminate(true)
                .label("Indeterminate progress".to_string())
            }),
            slider!({
                .value(value.clone())
                .label("Drag to set progress".to_string())
            }),
            html!("div", {
                .dwclass!("flex flex-row gap-4 align-items-center")
                .child(spinner!({
                    .size(SpinnerSize::Small)
                }))
                .child(spinner!({
                    .size(SpinnerSize::Medium)
                }))
                .child(spinner!({
                    .size(SpinnerSize::Large)
                }))
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn avatar_skeleton_demo() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .child(html!("div", {
            .dwclass!("flex flex-row gap-3 align-items-center")
            .child(avatar!({
                .name("Ada Lovelace".to_string())
                .size(AvatarSize::Small)
            }))
            .child(avatar!({
                .name("Grace Hopper".to_string())
            }))
            .child(avatar!({
                .name("Alan Turing".to_string())
                .size(AvatarSize::Large)
            }))
        }))
        .child(html!("div", {
            .dwclass!("flex flex-row gap-3 align-items-center")
            .child(skeleton!({
                .variant(SkeletonVariant::Circle)
            }))
            .child(html!("div", {
                .dwclass!("flex flex-col gap-2 grow")
                .child(skeleton!({
                    .variant(SkeletonVariant::Text)
                }))
                .child(skeleton!({
                    .variant(SkeletonVariant::Text)
                    .apply(|b| dwclass!(b, "w-p-50"))
                }))
            }))
        }))
        .child(skeleton!({
            .variant(SkeletonVariant::Rect)
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn tooltip_divider_demo() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .child(html!("div", {
            .dwclass!("flex flex-row gap-4 justify-center")
            .child(tooltip!({
                .text("Tooltips appear on hover and focus".to_string())
                .position(TooltipPosition::Top)
                .anchor(Some(html!("div", {
                    .dwclass!("w-32")
                    .child(button!({
                        .button_type(ButtonType::Border)
                        .size(ButtonSize::Small)
                        .content(Some(text("Hover me")))
                    }))
                })))
            }))
            .child(tooltip!({
                .text("This one sits below".to_string())
                .position(TooltipPosition::Bottom)
                .anchor(Some(html!("div", {
                    .dwclass!("w-32")
                    .child(button!({
                        .button_type(ButtonType::Border)
                        .size(ButtonSize::Small)
                        .content(Some(text("Or me")))
                    }))
                })))
            }))
        }))
        .child(divider!({
            .label("or".to_string())
        }))
        .child(divider!({}))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn breadcrumbs_demo() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .child(breadcrumbs!({
            .items(vec![
                ("Home".to_string(), "#/".to_string()),
                ("Docs".to_string(), "#/docs/colors".to_string()),
                ("Components".to_string(), "#/components".to_string()),
            ])
        }))
        .child(breadcrumbs!({
            .items(vec![
                ("workspace".to_string(), "#/".to_string()),
                ("crates".to_string(), "#/".to_string()),
                ("dwui".to_string(), "#/".to_string()),
                ("src".to_string(), "#/".to_string()),
            ])
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn tabs_demo() -> Dom {
    let selected = Mutable::new("overview".to_string());

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .child(tab_list!({
            .selected_signal(selected.signal_cloned())
            .aria_label("Tabs demo".to_string())
            .tabs(vec![
                ("overview".to_string(), "Overview".to_string()),
                ("features".to_string(), "Features".to_string()),
                ("pricing".to_string(), "Pricing".to_string()),
            ])
            .on_select(clone!(selected => move |key| {
                selected.set(key);
            }))
        }))
        .child_signal(selected.signal_cloned().map(|key| {
            let content = match key.as_str() {
                "features" => "Reactive signals drive every component, so panels swap instantly without re-rendering the page.",
                "pricing" => "Free and open source under the MIT license, like the rest of the dwind stack.",
                _ => "Use the arrow keys while a tab is focused to move between tabs; Home and End jump to the edges.",
            };

            Some(html!("p", {
                .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700 text-sm m-0")
                .text(content)
            }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn accordion_demo() -> Dom {
    accordion!({
        .initial_open(Some(0))
        .items(vec![
            (
                "What is dwui?".to_string(),
                text("A component library built on dwind utilities and dominator signals — themeable, accessible, and compiled to WebAssembly."),
            ),
            (
                "Is it keyboard accessible?".to_string(),
                text("Yes — every header is a real button with aria-expanded, and panels are labelled region landmarks."),
            ),
            (
                "Can panels animate?".to_string(),
                text("They animate open and closed using the CSS grid-template-rows technique, no JavaScript measurements required."),
            ),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn data_table_demo() -> Dom {
    #[derive(Clone)]
    struct Pioneer {
        id: &'static str,
        name: &'static str,
        born: u32,
        field: &'static str,
    }

    let pioneers = vec![
        Pioneer {
            id: "lovelace",
            name: "Ada Lovelace",
            born: 1815,
            field: "Analytical engines",
        },
        Pioneer {
            id: "hopper",
            name: "Grace Hopper",
            born: 1906,
            field: "Compilers",
        },
        Pioneer {
            id: "turing",
            name: "Alan Turing",
            born: 1912,
            field: "Computability",
        },
        Pioneer {
            id: "hamilton",
            name: "Margaret Hamilton",
            born: 1936,
            field: "Software engineering",
        },
        Pioneer {
            id: "liskov",
            name: "Barbara Liskov",
            born: 1939,
            field: "Abstraction",
        },
    ];

    let sorted: Mutable<Option<(String, SortDirection)>> =
        Mutable::new(Some(("born".to_string(), SortDirection::Ascending)));

    let rows_signal = sorted.signal_cloned().map(move |sort| {
        let mut rows = pioneers.clone();

        if let Some((key, direction)) = sort {
            match key.as_str() {
                "name" => rows.sort_by_key(|p| p.name),
                "born" => rows.sort_by_key(|p| p.born),
                _ => {}
            }

            if direction == SortDirection::Descending {
                rows.reverse();
            }
        }

        rows.into_iter()
            .map(|p| {
                (
                    p.id.to_string(),
                    vec![text(p.name), text(&p.born.to_string()), text(p.field)],
                )
            })
            .collect::<Vec<_>>()
    });

    data_table!({
        .columns(vec![
            TableColumn::new("name", "Name", true),
            TableColumn::new("born", "Born", true),
            TableColumn::new("field", "Field", false),
        ])
        .rows_signal_vec(rows_signal.to_signal_vec())
        .sorted_by_signal(sorted.signal_cloned())
        .on_sort(clone!(sorted => move |key, direction| {
            sorted.set(Some((key, direction)));
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn virtual_scroll_demo() -> Dom {
    let count = Mutable::new(50_000usize);
    let loading = Mutable::new(false);

    virtual_scroll!({
        .item_count_signal(count.signal())
        .item_height(36.0)
        .height(320.0)
        .aria_label("Virtual scroll demo".to_string())
        .loading_signal(loading.signal())
        .render_item(Box::new(|index: usize| {
            html!("div", {
                .dwclass!("flex flex-row align-items-center gap-4 p-l-4 p-r-4 h-full")
                .dwclass!("border-b dwui-border-void-800 is(.light *):dwui-border-void-200")
                .child(html!("span", {
                    .class("font-code")
                    .dwclass!("text-xs dwui-text-primary-400 is(.light *):dwui-text-primary-600 w-16 flex-none")
                    .text(&format!("#{index:05}"))
                }))
                .child(html!("span", {
                    .dwclass!("text-sm dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-700")
                    .text(&format!("Row {index} — rendered on demand"))
                }))
            })
        }))
        .on_reach_end(clone!(count, loading => move || {
            if loading.get() {
                return;
            }

            loading.set(true);

            wasm_bindgen_futures::spawn_local(clone!(count, loading => async move {
                gloo_timers::future::TimeoutFuture::new(600).await;
                count.set(count.get() + 10_000);
                loading.set(false);
            }));
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn date_picker_demo() -> Dom {
    let start = Mutable::new(None::<CalendarDate>);
    let end = Mutable::new(Some(CalendarDate::today()));

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .child(date_picker!({
            .label("Start date".to_string())
            .value_signal(start.signal())
            .on_change(clone!(start => move |date| {
                start.set(Some(date));
            }))
        }))
        .child(date_picker!({
            .label("End date".to_string())
            .value_signal(end.signal())
            .on_change(clone!(end => move |date| {
                end.set(Some(date));
            }))
        }))
        .child(html!("p", {
            .dwclass!("text-sm dwui-text-on-primary-400 is(.light *):dwui-text-on-primary-600 m-0")
            .text_signal(start.signal().map(|v| {
                match v {
                    Some(date) => format!("Selected start: {date}"),
                    None => "No start date selected yet".to_string(),
                }
            }))
        }))
    })
}
