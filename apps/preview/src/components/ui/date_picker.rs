//! DatePicker: the shadcn/ui date picker demo as one component, a `Popover` whose trigger is an outline
//! `Button` and whose content is a single-mode `Calendar`.
//!
//! The other date pickers of the docs (range, date of birth, input, time) are compositions of the same
//! parts, shown in the preview app.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::calendar::{Calendar, CalendarDate, CalendarLocale, CalendarMode, CalendarSelection};
use super::popover::{Popover, PopoverAlign, PopoverContent, PopoverTrigger};

/// Formats a date like date-fns' `PPP`: `June 5th, 2026`.
pub fn format_date(date: CalendarDate) -> String {
    let month = CalendarLocale::en_us().months[usize::from(date.month - 1)];
    let suffix = match (date.day % 10, date.day % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{month} {}{suffix}, {}", date.day, date.year)
}

/// A button that opens a calendar and shows the chosen day. Pass `selected` and `on_select` to control it,
/// or `default_selected` to let it keep its own value.
#[component]
pub fn DatePicker(
    /// Controlled selection. When omitted, the picker keeps its own value.
    selected: Option<CalendarDate>,
    /// Initial value of an uncontrolled picker.
    default_selected: Option<CalendarDate>,
    on_select: Option<EventHandler<Option<CalendarDate>>>,
    #[props(default = String::from("Pick a date"))] placeholder: String,
) -> Element {
    let mut internal = use_signal(|| default_selected);
    let date = selected.or(internal());

    rsx! {
        Popover {
            PopoverTrigger {
                class: button_variants(
                    ButtonVariant::Outline,
                    ButtonSize::Default,
                    Some("w-[212px] justify-between text-left font-normal data-[empty=true]:text-muted-foreground"),
                ),
                "data-empty": date.is_none().then_some("true"),
                if let Some(date) = date {
                    "{format_date(date)}"
                } else {
                    span { "{placeholder}" }
                }
                span { "data-icon": "inline-end", class: "contents",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m6 9 6 6 6-6" }
                    }
                }
            }
            PopoverContent { class: "w-auto p-0", align: PopoverAlign::Start,
                Calendar {
                    mode: CalendarMode::Single,
                    selected: CalendarSelection::Single(date),
                    default_month: date,
                    on_select: move |selection: CalendarSelection| {
                        let next = match selection {
                            CalendarSelection::Single(next) => next,
                            _ => None,
                        };
                        internal.set(next);
                        if let Some(handler) = on_select {
                            handler.call(next);
                        }
                    },
                }
            }
        }
    }
}
