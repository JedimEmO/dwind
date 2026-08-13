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
                .label("Light theme preview")
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
                    section_card("Text inputs", "Floating labels, password masking, disabled state, and live validation announced to screen readers", inputs_demo(), code(&INPUTS_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Textarea & number", "Multi-line and numeric fields sharing the same labelled field surface", text_area_number_demo(), code(&TEXT_AREA_NUMBER_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Select & slider", "Labelled form controls bound to reactive signals", select_slider_demo(), code(&SELECT_SLIDER_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Switch & checkbox", "Toggles with role=switch/checkbox semantics and clickable labels", toggles_demo(), code(&TOGGLES_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Radio group", "WAI-ARIA radios with roving tabindex — selection follows arrow-key focus", radio_group_demo(), code(&RADIO_GROUP_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Dropdown menu", "A menu button with full keyboard navigation; disabled items are skipped", dropdown_menu_demo(), code(&DROPDOWN_MENU_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Pagination", "Windowed page numbers with ellipses and aria-current on the active page", pagination_demo(), code(&PAGINATION_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Toasts", "Push notifications into a fixed aria-live host — timed toasts dismiss themselves", toasts_demo(), code(&TOASTS_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("Cards & headings", "Card schemes and padding scales, and semantic h1-h6 headings", cards_headings_demo(), code(&CARDS_HEADINGS_DEMO_EXAMPLE_HTML_MAP)),
                    section_card("List", "A selectable list with keyboard activation and aria-current", list_demo(), code(&LIST_DEMO_EXAMPLE_HTML_MAP)),
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
                .dwclass!("m-t-4 grid @md:grid-cols-2 @<md:grid-cols-1 gap-4")
                .child(section_card("Modal dialogs", "Focus-trapped dialogs with backdrop scrim, escape key, and entrance animation", example_card_modal(), code(&EXAMPLE_CARD_MODAL_EXAMPLE_HTML_MAP)))
                .child(section_card("Drawer", "A modal side panel sliding in from either edge, sharing the modal's focus trap", drawer_demo(), code(&DRAWER_DEMO_EXAMPLE_HTML_MAP)))
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
        .padding(CardPadding::Large)
        .apply(crate::fx::spotlight)
        .content(html!("div", {
            .dwclass!("flex flex-col gap-4")
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
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn buttons_demo() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .children([
            button!({
                .content(text("Primary Flat"))
            }),
            button!({
                .button_type(ButtonType::Border)
                .content(text("Primary Border"))
            }),
            button!({
                .button_type(ButtonType::Text)
                .content(text("Text Button"))
            }),
            html!("div", {
                .dwclass!("flex flex-row gap-4 align-items-center")
                .child(html!("div", {
                    .dwclass!("grow")
                    .child(button!({
                        .size(ButtonSize::Small)
                        .content(text("Small"))
                    }))
                }))
                .child(html!("div", {
                    .dwclass!("grow")
                    .child(button!({
                        .size(ButtonSize::Large)
                        .content(text("Large"))
                    }))
                }))
            }),
            button!({
                .disabled(true)
                .content(text("Disabled"))
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
                .content(text("Primary"))
                .variant(BadgeVariant::Primary)
            }),
            badge!({
                .content(text("Error"))
                .variant(BadgeVariant::Error)
            }),
            badge!({
                .content(text("Void"))
                .variant(BadgeVariant::Void)
            }),
            badge!({
                .content(text("Outline"))
                .variant(BadgeVariant::Outline)
            }),
            badge!({
                .content(text("99+"))
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
                .title("Heads up")
                .content(text("A new version of dwui is available."))
            }),
            alert!({
                .variant(AlertVariant::Success)
                .title("Deployed")
                .content(text("Build #214 went live without warnings."))
            }),
            alert!({
                .variant(AlertVariant::Warning)
                .title("Heavy load")
            }),
            alert!({
                .variant(AlertVariant::Error)
                .title("Build failed")
                .content(text("Use the dismiss button to close this alert."))
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
                .label("Your name")
            }),
            text_input!({
                .input_type(TextInputType::Password)
                .label("Password")
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
                .label("Accepts bananas")
            }),
            text_input!({
                .value(Mutable::new("Read only".to_string()))
                .disabled(true)
                .label("Disabled")
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn text_area_number_demo() -> Dom {
    let bio = Mutable::new("".to_string());
    let amount = Mutable::new(3.0f64);

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .children([
            text_area!({
                .value(bio.clone())
                .label("Short bio")
                .rows(3u32)
            }),
            number_input!({
                .value(amount.clone())
                .label("Quantity")
                .min(0.0)
                .max(10.0)
                .step(1.0)
            }),
        ])
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn radio_group_demo() -> Dom {
    let flavor = Mutable::new("butterscotch".to_string());
    let channel = Mutable::new("stable".to_string());

    html!("div", {
        .dwclass!("flex flex-col gap-6")
        .child(radio_group!({
            .label("Flavor")
            .value_signal(flavor.signal_cloned())
            .options(vec![
                ("almond".to_string(), "Almond".to_string()),
                ("butterscotch".to_string(), "Butterscotch".to_string()),
                ("cinnamon".to_string(), "Cinnamon".to_string()),
            ])
            .on_change(clone!(flavor => move |key| flavor.set(key)))
        }))
        .child(radio_group!({
            .label("Release channel")
            .direction(RadioGroupDirection::Horizontal)
            .value_signal(channel.signal_cloned())
            .options(vec![
                ("stable".to_string(), "Stable".to_string()),
                ("beta".to_string(), "Beta".to_string()),
                ("nightly".to_string(), "Nightly".to_string()),
            ])
            .on_change(clone!(channel => move |key| channel.set(key)))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn dropdown_menu_demo() -> Dom {
    let last_action: Mutable<Option<String>> = Mutable::new(None);

    html!("div", {
        .dwclass!("flex flex-col gap-4 align-items-start")
        .child(dropdown_menu!({
            .label("Actions")
            .items(vec![
                ("copy".to_string(), "Copy link".to_string(), false),
                ("rename".to_string(), "Rename".to_string(), false),
                ("archive".to_string(), "Archive".to_string(), true),
                ("delete".to_string(), "Delete".to_string(), false),
            ])
            .on_select(clone!(last_action => move |key| {
                last_action.set(Some(key));
            }))
        }))
        .child(html!("p", {
            .dwclass!("text-sm dwui-text-on-primary-400 is(.light *):dwui-text-on-primary-600 m-0")
            .text_signal(last_action.signal_cloned().map(|action| {
                match action {
                    Some(key) => format!("Selected: {key}"),
                    None => "Nothing selected yet".to_string(),
                }
            }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn pagination_demo() -> Dom {
    let page = Mutable::new(1usize);

    html!("div", {
        .dwclass!("flex flex-col gap-4 align-items-start")
        .child(pagination!({
            .page_signal(page.signal())
            .total_pages(12usize)
            .on_page_change(clone!(page => move |p| page.set(p)))
        }))
        .child(html!("p", {
            .dwclass!("text-sm dwui-text-on-primary-400 is(.light *):dwui-text-on-primary-600 m-0")
            .text_signal(page.signal().map(|p| format!("Viewing page {p} of 12")))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn toasts_demo() -> Dom {
    let toaster = Toaster::default();

    html!("div", {
        .dwclass!("flex flex-row flex-wrap gap-4")
        // The host is position: fixed, so toasts appear at the viewport corner
        .child(toasts!({
            .toaster(toaster.clone())
            .position(ToastPosition::BottomRight)
        }))
        .child(button!({
            .button_type(ButtonType::Border)
            .size(ButtonSize::Small)
            .content(text("Success toast"))
            .on_click(clone!(toaster => move |_| {
                toaster.success("Saved to workspace");
            }))
        }))
        .child(button!({
            .button_type(ButtonType::Border)
            .size(ButtonSize::Small)
            .content(text("Sticky error"))
            .on_click(clone!(toaster => move |_| {
                toaster.push(ToastOptions {
                    title: "Build failed".to_string(),
                    message: Some("Sticky until dismissed".to_string()),
                    variant: ToastVariant::Error,
                    duration_ms: None,
                    ..Default::default()
                });
            }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn cards_headings_demo() -> Dom {
    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .child(card!({
            .scheme(ColorScheme::Primary)
            .padding(CardPadding::Small)
            .content(html!("div", {
                .child(heading!({
                    .content(text("Primary card"))
                    .text_size(TextSize::Base)
                    .level(HeadingLevel::H3)
                }))
                .child(html!("p", {
                    .dwclass!("text-sm m-0")
                    .text("Small padding, primary scheme.")
                }))
            }))
        }))
        .child(card!({
            .content(html!("div", {
                .child(heading!({
                    .content(text("Void card"))
                    .text_size(TextSize::Large)
                    .level(HeadingLevel::H3)
                }))
                .child(html!("p", {
                    .dwclass!("text-sm m-0")
                    .text("Default medium padding; headings render bare h1-h6 tags.")
                }))
            }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn list_demo() -> Dom {
    let selected = Mutable::new(Some(1usize));

    html!("div", {
        .dwclass!("flex flex-col gap-4")
        .child(list!({
            .selected_index_signal(selected.signal())
            .items(vec![
                text("Getting started"),
                text("Theming"),
                text("Components"),
                text("Recipes"),
            ])
            .item_click_handler(clone!(selected => move |index| {
                selected.set(Some(index));
            }))
        }))
    })
}

#[example_html(themes = ["base16-ocean.dark"])]
fn drawer_demo() -> Dom {
    let open = Mutable::new(false);
    let side = Mutable::new(DrawerSide::Right);

    html!("div", {
        .dwclass!("flex flex-row gap-4")
        .child(button!({
            .button_type(ButtonType::Border)
            .content(text("Open right drawer"))
            .on_click(clone!(open, side => move |_| {
                side.set(DrawerSide::Right);
                open.set(true);
            }))
        }))
        .child(button!({
            .button_type(ButtonType::Border)
            .content(text("Open left drawer"))
            .on_click(clone!(open, side => move |_| {
                side.set(DrawerSide::Left);
                open.set(true);
            }))
        }))
        .child(drawer!({
            .open_signal(open.signal())
            .side_signal(side.signal())
            .aria_label("Demo drawer")
            .on_close(clone!(open => move || open.set(false)))
            .content(html!("div", {
                .dwclass!("flex flex-col gap-3 m-t-8")
                .child(heading!({
                    .content(text("Drawer"))
                    .text_size(TextSize::Large)
                    .level(HeadingLevel::H2)
                }))
                .child(html!("p", {
                    .dwclass!("text-sm m-0")
                    .text("Escape closes it, Tab stays trapped inside, and the scrim click dismisses.")
                }))
            }))
        }))
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
                .label("Favourite option")
                .value(selected.clone())
                .options(vec![
                    ("a".to_string(), "Option A".to_string()),
                    ("b".to_string(), "Option B".to_string()),
                    ("c".to_string(), "Option C".to_string()),
                ])
            }),
            slider!({
                .value(amount.clone())
                .label("Amount")
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
                .label("Enable notifications")
                .on_change(clone!(notifications => move |v| notifications.set(v)))
            }),
            switch!({
                .checked(false)
                .disabled(true)
                .label("Disabled switch")
            }),
            checkbox!({
                .checked_signal(newsletter.signal())
                .label("Subscribe to newsletter")
                .on_change(clone!(newsletter => move |v| newsletter.set(v)))
            }),
            checkbox!({
                .checked_signal(terms.signal())
                .label("Accept the terms")
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
                .label("Demo progress")
            }),
            progress!({
                .indeterminate(true)
                .label("Indeterminate progress")
            }),
            slider!({
                .value(value.clone())
                .label("Drag to set progress")
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
                .name("Ada Lovelace")
                .size(AvatarSize::Small)
            }))
            .child(avatar!({
                .name("Grace Hopper")
            }))
            .child(avatar!({
                .name("Alan Turing")
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
                .text("Tooltips appear on hover and focus")
                .position(TooltipPosition::Top)
                .anchor(html!("div", {
                    .dwclass!("w-32")
                    .child(button!({
                        .button_type(ButtonType::Border)
                        .size(ButtonSize::Small)
                        .content(text("Hover me"))
                    }))
                }))
            }))
            .child(tooltip!({
                .text("This one sits below")
                .position(TooltipPosition::Bottom)
                .anchor(html!("div", {
                    .dwclass!("w-32")
                    .child(button!({
                        .button_type(ButtonType::Border)
                        .size(ButtonSize::Small)
                        .content(text("Or me"))
                    }))
                }))
            }))
        }))
        .child(divider!({
            .label("or")
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
            .aria_label("Tabs demo")
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
        .initial_open(0)
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
        .aria_label("Virtual scroll demo")
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
            .label("Start date")
            .value_signal(start.signal())
            .on_change(clone!(start => move |date| {
                start.set(Some(date));
            }))
        }))
        .child(date_picker!({
            .label("End date")
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
