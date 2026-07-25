use crate::components::input::calendar_date::CalendarDate;
use crate::mixins::labelled_rect_mixin::labelled_rect_mixin;
use crate::prelude::ValidationResult;
use crate::theme::prelude::*;
use crate::utils::component_id;
use dominator::{clone, events, html, svg, Dom, EventOptions};
use dwind::prelude::*;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals_component_macro::component;
use std::rc::Rc;

/// An accessible date picker.
///
/// A labelled field button opens a calendar popup (`role="dialog"`
/// containing a `role="grid"` month view). Full keyboard support: arrows
/// move by day/week, PageUp/PageDown by month, Enter/Space select,
/// Escape closes. The selected date is controlled through the `value`
/// signal; picking a day invokes `on_change`.
#[component(render_fn = date_picker)]
struct DatePicker {
    #[signal]
    #[default(None)]
    value: Option<CalendarDate>,

    #[signal]
    #[default("".to_string())]
    label: String,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[default(Box::new(|_| {}))]
    on_change: dyn Fn(CalendarDate) + 'static,
}

pub fn date_picker(props: DatePickerProps) -> Dom {
    let DatePickerProps {
        value,
        label,
        disabled,
        on_change,
        apply,
    } = props;

    let value = value.broadcast();
    let label = label.broadcast();
    let disabled = disabled.broadcast();
    let on_change = Rc::new(on_change);

    let open = Mutable::new(false);
    let value_state: Mutable<Option<CalendarDate>> = Mutable::new(None);
    // (year, month) shown in the calendar
    let view = Mutable::new((CalendarDate::today().year, CalendarDate::today().month));
    let focused_day = Mutable::new(CalendarDate::today());

    let field_id = component_id("date-picker");

    let open_picker = Rc::new(clone!(open, value_state, view, focused_day => move || {
        let initial = value_state.get().unwrap_or_else(CalendarDate::today);

        view.set((initial.year, initial.month));
        focused_day.set(initial);
        open.set(true);
    }));

    let has_value = value.signal_ref(|v| v.is_some()).broadcast();

    html!("div", {
        .dwclass!("grid")
        .future(value.signal().for_each(clone!(value_state => move |v| {
            value_state.set(v);
            async {}
        })))
        // Field button
        .child(html!("button", {
            .attr("type", "button")
            .attr("id", &field_id)
            .attr("aria-haspopup", "dialog")
            .attr_signal("aria-expanded", open.signal().map(|v| if v { "true" } else { "false" }))
            .attr_signal("disabled", disabled.signal().map(|v| if v { Some("disabled") } else { None }))
            .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-300 text-base h-10 p-l-2 p-r-2")
            .dwclass!("grid-col-1 grid-row-1 cursor-pointer rounded-t-sm transition-all border-none text-left")
            .dwclass!("dwui-text-on-primary-300 is(.light *):dwui-text-on-primary-900")
            .dwclass!("disabled:cursor-not-allowed disabled:opacity-60")
            .dwclass!("flex flex-row align-items-center justify-between gap-2")
            .dwclass!("dwui-focusable")
            .child(html!("span", {
                .text_signal(value.signal().map(|v| {
                    v.map(|v| v.to_string()).unwrap_or_default()
                }))
            }))
            .child(svg!("svg", {
                .attr("viewBox", "0 0 16 16")
                .attr("width", "16")
                .attr("height", "16")
                .attr("fill", "none")
                .attr("aria-hidden", "true")
                .child(svg!("rect", {
                    .attr("x", "2")
                    .attr("y", "3")
                    .attr("width", "12")
                    .attr("height", "11")
                    .attr("rx", "1.5")
                    .attr("stroke", "currentColor")
                    .attr("stroke-width", "1.2")
                }))
                .child(svg!("path", {
                    .attr("d", "M2 6.5 H14 M5.5 1.5 V4 M10.5 1.5 V4")
                    .attr("stroke", "currentColor")
                    .attr("stroke-width", "1.2")
                    .attr("stroke-linecap", "round")
                }))
            }))
            .event(clone!(open, open_picker => move |_: events::Click| {
                if open.get() {
                    open.set(false);
                } else {
                    (open_picker)();
                }
            }))
        }))
        // Popup
        .child(html!("div", {
            .dwclass!("grid-col-1 grid-row-1")
            .style("position", "relative")
            .style("pointer-events", "none")
            .child_signal(open.signal().map(clone!(view, focused_day, value_state, on_change, open, field_id => move |is_open| {
                if !is_open {
                    return None;
                }

                Some(html!("div", {
                    // invisible backdrop: clicking outside closes the popup
                    .child(html!("div", {
                        .style("position", "fixed")
                        .style("inset", "0")
                        .style("z-index", "40")
                        .style("pointer-events", "auto")
                        .event(clone!(open => move |_: events::Click| {
                            open.set(false);
                        }))
                    }))
                    .child(calendar_popup(
                        &field_id,
                        view.clone(),
                        focused_day.clone(),
                        value_state.clone(),
                        on_change.clone(),
                        open.clone(),
                    ))
                }))
            })))
        }))
        .apply(labelled_rect_mixin(
            label.signal_cloned(),
            map_ref! {
                let has_value = has_value.signal(),
                let open = open.signal() => *has_value || *open
            },
            futures_signals::signal::always(ValidationResult::Valid),
            field_id.clone(),
        ))
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
    })
}

fn calendar_popup(
    field_id: &str,
    view: Mutable<(i32, u32)>,
    focused_day: Mutable<CalendarDate>,
    value_state: Mutable<Option<CalendarDate>>,
    on_change: Rc<Box<dyn Fn(CalendarDate) + 'static>>,
    open: Mutable<bool>,
) -> Dom {
    let picker_id = field_id.to_string();
    let today = CalendarDate::today();

    let pick = Rc::new(clone!(on_change, open => move |date: CalendarDate| {
        (on_change)(date);
        open.set(false);
    }));

    let move_focus = Rc::new(clone!(focused_day, view => move |target: CalendarDate| {
        view.set((target.year, target.month));
        focused_day.set(target);
    }));

    html!("div", {
        .attr("role", "dialog")
        .attr("aria-label", "Choose date")
        .dwclass!("rounded-lg border shadow-2xl p-4 w-72")
        .dwclass!("dwui-bg-void-900 dwui-border-void-700")
        .dwclass!("is(.light *):dwui-bg-void-100 is(.light *):dwui-border-void-300")
        .style("position", "absolute")
        .style("top", "calc(100% + 6px)")
        .style("left", "0")
        .style("z-index", "50")
        .style("pointer-events", "auto")
        .style("animation", "dwui-modal-in 150ms ease-out")
        .global_event(clone!(open => move |e: events::KeyDown| {
            if e.key() == "Escape" {
                open.set(false);
            }
        }))
        // Month header
        .child(html!("div", {
            .dwclass!("flex flex-row align-items-center justify-between m-b-2")
            .child(month_nav_button("Previous month", "M9 3 L5 7 L9 11", clone!(view => move || {
                let (y, m) = view.get();
                view.set(CalendarDate::prev_month(y, m));
            })))
            .child(html!("div", {
                .attr("aria-live", "polite")
                .dwclass!("font-bold text-sm")
                .dwclass!("dwui-text-on-primary-100 is(.light *):dwui-text-on-primary-900")
                .text_signal(view.signal().map(|(y, m)| {
                    format!("{} {}", CalendarDate::month_name(m), y)
                }))
            }))
            .child(month_nav_button("Next month", "M5 3 L9 7 L5 11", clone!(view => move || {
                let (y, m) = view.get();
                view.set(CalendarDate::next_month(y, m));
            })))
        }))
        // Weekday header
        .child(html!("div", {
            .dwclass!("grid grid-cols-7 m-b-1")
            .children(["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"].map(|day| {
                html!("div", {
                    .attr("aria-hidden", "true")
                    .class("font-code")
                    .dwclass!("text-xs text-center p-1 select-none")
                    .dwclass!("dwui-text-on-primary-500 is(.light *):dwui-text-on-primary-600")
                    .text(day)
                })
            }))
        }))
        // Day grid: 6 fixed weeks starting at the Monday of the week of the 1st
        .child(html!("div", {
            .attr("role", "grid")
            .attr("aria-label", "Calendar")
            .children_signal_vec(view.signal().map(clone!(picker_id, focused_day, value_state, pick, move_focus => move |(year, month)| {
                let first = CalendarDate::new(year, month, 1);
                let grid_start = first.add_days(-(first.day_of_week() as i32));

                (0..6).map(|week| {
                    html!("div", {
                        .attr("role", "row")
                        .dwclass!("grid grid-cols-7")
                        .children((0..7).map(|weekday| {
                            let date = grid_start.add_days(week * 7 + weekday);
                            let in_month = date.month == month;

                            day_cell(
                                &picker_id,
                                date,
                                in_month,
                                today,
                                focused_day.clone(),
                                value_state.clone(),
                                pick.clone(),
                                move_focus.clone(),
                            )
                        }).collect::<Vec<_>>())
                    })
                }).collect::<Vec<_>>()
            })).to_signal_vec())
        }))
    })
}

fn month_nav_button(label: &str, path: &str, on_click: impl Fn() + 'static) -> Dom {
    html!("button", {
        .attr("type", "button")
        .attr("aria-label", label)
        .dwclass!("w-8 h-8 flex align-items-center justify-center rounded-md cursor-pointer")
        .dwclass!("bg-transparent border-none transition-colors")
        .dwclass!("dwui-text-on-primary-300 hover:dwui-bg-void-800 hover:dwui-text-on-primary-50")
        .dwclass!("is(.light *):dwui-text-on-primary-700 is(.light *):hover:dwui-bg-void-200")
        .dwclass!("dwui-focusable")
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
        .event(move |_: events::Click| {
            on_click();
        })
    })
}

#[allow(clippy::too_many_arguments)]
fn day_cell(
    picker_id: &str,
    date: CalendarDate,
    in_month: bool,
    today: CalendarDate,
    focused_day: Mutable<CalendarDate>,
    value_state: Mutable<Option<CalendarDate>>,
    pick: Rc<impl Fn(CalendarDate) + 'static>,
    move_focus: Rc<impl Fn(CalendarDate) + 'static>,
) -> Dom {
    let is_selected = value_state
        .signal()
        .map(move |v| v == Some(date))
        .broadcast();

    let is_focused = focused_day.signal().map(move |v| v == date).broadcast();

    html!("div", {
        .attr("role", "gridcell")
        .child(html!("button", {
            .attr("type", "button")
            .attr("id", &format!("{}-day-{}", picker_id, date))
            .attr("aria-label", &date.to_string())
            .attr_signal("aria-selected", is_selected.signal().map(|v| if v { "true" } else { "false" }))
            .attr_signal("tabindex", is_focused.signal().map(|v| if v { "0" } else { "-1" }))
            .focused_signal(is_focused.signal())
            .dwclass!("w-9 h-9 flex align-items-center justify-center rounded-md cursor-pointer text-sm")
            .dwclass!("bg-transparent border-none transition-colors")
            .dwclass!("dwui-focusable")
            .apply(move |b| {
                if in_month {
                    dwclass!(b, "dwui-text-on-primary-200 is(.light *):dwui-text-on-primary-800")
                } else {
                    dwclass!(b, "dwui-text-on-primary-600 is(.light *):dwui-text-on-primary-500")
                }
            })
            .dwclass!("hover:dwui-bg-void-700 is(.light *):hover:dwui-bg-void-200")
            .dwclass_signal!("dwui-bg-primary-600 dwui-text-on-primary-50 font-bold is(.light *):dwui-bg-primary-400 is(.light *):dwui-text-on-primary-950", is_selected.signal())
            .apply_if(date == today, |b| {
                dwclass!(b, "dwui-border-primary-500 border")
            })
            .text(&date.day.to_string())
            .event(clone!(pick => move |_: events::Click| {
                (pick)(date);
            }))
            .event_with_options(&EventOptions::preventable(), clone!(move_focus => move |e: events::KeyDown| {
                let target = match e.key().as_str() {
                    "ArrowLeft" => date.add_days(-1),
                    "ArrowRight" => date.add_days(1),
                    "ArrowUp" => date.add_days(-7),
                    "ArrowDown" => date.add_days(7),
                    "PageUp" => date.add_months(-1),
                    "PageDown" => date.add_months(1),
                    "Home" => date.add_days(-(date.day_of_week() as i32)),
                    "End" => date.add_days(6 - date.day_of_week() as i32),
                    _ => return,
                };

                e.prevent_default();
                (move_focus)(target);
            }))
        }))
    })
}
