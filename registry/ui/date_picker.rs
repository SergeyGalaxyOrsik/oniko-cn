//! DatePicker: a button that opens a calendar popover and shows the chosen day.
//!
//! Composed from `Popover` and `Calendar`. Pass `today` to mark a day and to choose the month
//! shown when nothing is selected: the component never reads the clock.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::calendar::{Calendar, CalendarDate};
use super::popover::{Popover, PopoverContent, PopoverTrigger};

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

fn format_date(date: CalendarDate) -> String {
    format!(
        "{} {}, {}",
        MONTH_NAMES[usize::from(date.month - 1)],
        date.day,
        date.year
    )
}

/// A date input. Pass `selected` and `on_select` to control it, or `default_selected` to let it keep its own value.
///
/// Extra attributes, such as `class`, go to the trigger button.
#[component]
pub fn DatePicker(
    /// Controlled selection. When omitted, the picker keeps its own value.
    selected: Option<CalendarDate>,
    /// Initial value of an uncontrolled picker.
    default_selected: Option<CalendarDate>,
    on_select: Option<EventHandler<CalendarDate>>,
    /// Month shown when nothing is selected. Defaults to `today`.
    default_month: Option<CalendarDate>,
    /// Today's date, marked in the calendar. Pass the date from your app; the component never reads the clock.
    today: Option<CalendarDate>,
    /// First day that can be picked.
    min: Option<CalendarDate>,
    /// Last day that can be picked.
    max: Option<CalendarDate>,
    #[props(default)] disabled_dates: Vec<CalendarDate>,
    #[props(default = String::from("Pick a date"))] placeholder: String,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let mut internal = use_signal(|| default_selected);
    let mut open = use_signal(|| false);
    let current = selected.or(internal());
    let class = button_variants(
        ButtonVariant::Outline,
        ButtonSize::Default,
        Some(
            cn!(
                "cn-date-picker-trigger w-60 justify-start gap-2 px-3 font-normal",
                take_class(&mut attributes)
            )
            .as_str(),
        ),
    );

    rsx! {
        Popover {
            open: open(),
            on_open_change: move |next: bool| open.set(next),
            PopoverTrigger {
                class: class,
                disabled: disabled,
                "data-empty": current.is_none().then_some("true"),
                CalendarIcon {}
                span { class: "truncate",
                    if let Some(date) = current {
                        "{format_date(date)}"
                    } else {
                        "{placeholder}"
                    }
                }
            }
            PopoverContent {
                Calendar {
                    selected: current,
                    default_month: default_month,
                    today: today,
                    min: min,
                    max: max,
                    disabled_dates: disabled_dates,
                    on_select: move |date: CalendarDate| {
                        internal.set(Some(date));
                        open.set(false);
                        if let Some(handler) = on_select {
                            handler.call(date);
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn CalendarIcon() -> Element {
    rsx! {
        svg {
            class: "pointer-events-none size-4 shrink-0 opacity-50",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "3", y: "4", width: "18", height: "18", rx: "2" }
            path { d: "M16 2v4M8 2v4M3 10h18" }
        }
    }
}
