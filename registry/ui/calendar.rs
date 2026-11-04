//! Calendar: a month grid for picking a single day.
//!
//! The month grid is computed here with plain date arithmetic, so the component needs no
//! date crate and no browser APIs, and renders the same on the web and on the desktop.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const WEEKDAY_NAMES: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// A day in the proleptic Gregorian calendar. Month is 1-12, day is 1-31 for that month.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl CalendarDate {
    pub const fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    fn days_in_month(self) -> u8 {
        match self.month {
            2 if Self::is_leap_year(self.year) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    /// Day of the week, 0 for Sunday.
    fn weekday(self) -> u8 {
        const OFFSETS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let year = self.year - i32::from(self.month < 3);
        let sum = year + year / 4 - year / 100 + year / 400
            + OFFSETS[usize::from(self.month - 1)]
            + i32::from(self.day);
        sum.rem_euclid(7) as u8
    }

    /// The same day `delta` months away. The day is clamped to the length of the target month.
    fn add_months(self, delta: i32) -> Self {
        let total = self.year * 12 + i32::from(self.month - 1) + delta;
        let mut moved = Self {
            year: total.div_euclid(12),
            month: total.rem_euclid(12) as u8 + 1,
            day: 1,
        };
        moved.day = self.day.min(moved.days_in_month());
        moved
    }
}

/// `(year, month)` of a date with the day set to 1, used for the month shown in the grid.
fn month_start(date: CalendarDate) -> CalendarDate {
    CalendarDate { day: 1, ..date }
}

fn is_blocked(
    date: CalendarDate,
    disabled: bool,
    min: Option<CalendarDate>,
    max: Option<CalendarDate>,
    disabled_dates: &[CalendarDate],
) -> bool {
    disabled
        || min.is_some_and(|min| date < min)
        || max.is_some_and(|max| date > max)
        || disabled_dates.contains(&date)
}

#[component]
pub fn Calendar(
    /// The selected day. Changing it from the parent updates the calendar, and clicks update it too.
    selected: Option<CalendarDate>,
    /// Month shown first. Defaults to the selected day's month, then to `today`, then January 1970.
    default_month: Option<CalendarDate>,
    /// Day that gets the today marker (`data-today`). The component never reads the clock.
    today: Option<CalendarDate>,
    /// Days that cannot be picked.
    #[props(default)]
    disabled_dates: Vec<CalendarDate>,
    /// First day that can be picked.
    min: Option<CalendarDate>,
    /// Last day that can be picked.
    max: Option<CalendarDate>,
    #[props(default)] disabled: bool,
    on_select: Option<EventHandler<CalendarDate>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-calendar flex flex-col gap-4 p-3",
        take_class(&mut attributes).as_deref()
    );

    let mut chosen = use_signal(|| selected);
    let mut seen_prop = use_signal(|| selected);
    if *seen_prop.peek() != selected {
        seen_prop.set(selected);
        chosen.set(selected);
    }

    let fallback = CalendarDate::new(1970, 1, 1);
    let mut shown = use_signal(|| {
        month_start(
            default_month
                .or(selected)
                .or(today)
                .unwrap_or(fallback),
        )
    });

    let view = shown();
    let selected_day = chosen();
    let leading = usize::from(view.weekday());
    let days = view.days_in_month();

    rsx! {
        div { "data-slot": "calendar", class: class, ..attributes,
            div { "data-slot": "calendar-nav", class: "cn-calendar-nav flex items-center justify-between",
                button {
                    r#type: "button",
                    "data-slot": "calendar-prev",
                    aria_label: "Previous month",
                    class: button_variants(ButtonVariant::Ghost, ButtonSize::Icon, None),
                    onclick: move |_| shown.set(view.add_months(-1)),
                    ChevronIcon { left: true }
                }
                div {
                    "data-slot": "calendar-caption",
                    class: "cn-calendar-caption-label text-sm font-medium",
                    aria_live: "polite",
                    "{MONTH_NAMES[usize::from(view.month - 1)]} {view.year}"
                }
                button {
                    r#type: "button",
                    "data-slot": "calendar-next",
                    aria_label: "Next month",
                    class: button_variants(ButtonVariant::Ghost, ButtonSize::Icon, None),
                    onclick: move |_| shown.set(view.add_months(1)),
                    ChevronIcon { left: false }
                }
            }
            div { "data-slot": "calendar-grid", class: "grid grid-cols-7 gap-1",
                for name in WEEKDAY_NAMES {
                    div {
                        "data-slot": "calendar-weekday",
                        class: "cn-calendar-weekday flex h-8 items-center justify-center",
                        "aria-hidden": "true",
                        "{name}"
                    }
                }
                for _ in 0..leading {
                    div { "aria-hidden": "true" }
                }
                for number in 1..=days {
                    {
                        let date = CalendarDate { day: number, ..view };
                        let is_selected = selected_day == Some(date);
                        let is_today = today == Some(date);
                        let is_disabled = is_blocked(date, disabled, min, max, &disabled_dates);
                        let class = cn!(
                            "cn-calendar-day inline-flex size-8 items-center justify-center whitespace-nowrap transition-colors outline-none disabled:pointer-events-none disabled:opacity-50",
                            is_selected.then_some("cn-calendar-day-selected"),
                            is_today.then_some("cn-calendar-day-today")
                        );
                        rsx! {
                            button {
                                r#type: "button",
                                "data-slot": "calendar-day",
                                "data-selected": is_selected.then_some("true"),
                                "data-today": is_today.then_some("true"),
                                "aria-pressed": if is_selected { "true" } else { "false" },
                                "aria-current": is_today.then_some("date"),
                                disabled: is_disabled,
                                class: class,
                                onclick: move |_| {
                                    if is_disabled {
                                        return;
                                    }
                                    chosen.set(Some(date));
                                    if let Some(handler) = on_select {
                                        handler.call(date);
                                    }
                                },
                                "{number}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ChevronIcon(left: bool) -> Element {
    let d = if left { "m15 18-6-6 6-6" } else { "m9 18 6-6-6-6" };
    rsx! {
        svg {
            class: "pointer-events-none size-4 shrink-0",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: d }
        }
    }
}
