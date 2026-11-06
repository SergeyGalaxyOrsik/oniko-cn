//! Calendar examples, ported from shadcn/ui `examples/base/calendar-*.tsx`.
//!
//! `calendar-hijri` is not ported: it needs react-day-picker's Persian calendar system.

use dioxus::prelude::*;
use lucide_dioxus::Clock2;

use super::Example;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::calendar::{
    Calendar, CalendarCaptionLayout, CalendarDate, CalendarDayButton, CalendarDayProps,
    CalendarLocale, CalendarMatcher, CalendarMode, CalendarModifier, CalendarSelection, DateRange,
    use_today,
};
use crate::components::ui::card::{Card, CardContent, CardFooter, CardSize};
use crate::components::ui::field::{Field, FieldGroup, FieldLabel};
use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupInput};

#[component]
pub fn Examples() -> Element {
    rsx! {
        CalendarDemo {}
        CalendarBasic {}
        CalendarRange {}
        CalendarCaption {}
        CalendarWithPresets {}
        CalendarWithTime {}
        CalendarBookedDates {}
        CalendarCustomDays {}
        CalendarWeekNumbers {}
        CalendarRtl {}
    }
}

/// A signal that starts as `init(today)` once the clock has been read, like `useState(new Date())`.
fn use_from_today<T: Clone + 'static>(
    init: impl Fn(CalendarDate) -> T + 'static,
) -> Signal<Option<T>> {
    let today = use_today();
    let mut state = use_signal(|| None);
    use_effect(move || {
        if state.peek().is_none()
            && let Some(today) = today()
        {
            state.set(Some(init(today)));
        }
    });
    state
}

fn single(selection: CalendarSelection) -> Option<CalendarDate> {
    match selection {
        CalendarSelection::Single(date) => date,
        _ => None,
    }
}

fn range(selection: CalendarSelection) -> DateRange {
    match selection {
        CalendarSelection::Range(range) => range,
        _ => DateRange::default(),
    }
}

#[component]
fn CalendarDemo() -> Element {
    let mut date = use_from_today(Some);
    rsx! {
        Example { title: "Demo", class: "h-96",
            if let Some(chosen) = date() {
                Calendar {
                    mode: CalendarMode::Single,
                    selected: CalendarSelection::Single(chosen),
                    on_select: move |selection| date.set(Some(single(selection))),
                    class: "rounded-lg border",
                    caption_layout: CalendarCaptionLayout::Dropdown,
                }
            }
        }
    }
}

#[component]
fn CalendarBasic() -> Element {
    rsx! {
        Example { title: "Basic", class: "h-96",
            Calendar { mode: CalendarMode::Single, class: "rounded-lg border" }
        }
    }
}

#[component]
fn CalendarRange() -> Element {
    let mut selected = use_from_today(|today| {
        let from = CalendarDate::new(today.year, 1, 12);
        DateRange {
            from: Some(from),
            to: Some(from.add_days(30)),
        }
    });
    rsx! {
        Example { title: "Range Calendar", class: "h-[36rem] md:h-96",
            if let Some(chosen) = selected() {
                Calendar {
                    mode: CalendarMode::Range,
                    default_month: chosen.from,
                    selected: CalendarSelection::Range(chosen),
                    on_select: move |selection| selected.set(Some(range(selection))),
                    number_of_months: 2,
                    class: "rounded-lg border",
                }
            }
        }
    }
}

#[component]
fn CalendarCaption() -> Element {
    rsx! {
        Example { title: "Month and Year Selector", class: "h-96",
            Calendar {
                mode: CalendarMode::Single,
                caption_layout: CalendarCaptionLayout::Dropdown,
                class: "rounded-lg border",
            }
        }
    }
}

#[component]
fn CalendarWithPresets() -> Element {
    let today = use_today();
    let mut date = use_from_today(|today| Some(CalendarDate::new(today.year, 2, 12)));
    let mut current_month = use_from_today(CalendarDate::month_start);
    let presets = [
        ("Today", 0),
        ("Tomorrow", 1),
        ("In 3 days", 3),
        ("In a week", 7),
        ("In 2 weeks", 14),
    ];
    rsx! {
        Example { title: "Presets", class: "h-[650px]",
            if let (Some(chosen), Some(month)) = (date(), current_month()) {
                Card { class: "mx-auto w-fit max-w-[300px]", size: CardSize::Sm,
                    CardContent {
                        Calendar {
                            mode: CalendarMode::Single,
                            selected: CalendarSelection::Single(chosen),
                            on_select: move |selection| date.set(Some(single(selection))),
                            month,
                            on_month_change: move |next| current_month.set(Some(next)),
                            fixed_weeks: true,
                            class: "p-0 [--cell-size:--spacing(9.5)]",
                        }
                    }
                    CardFooter { class: "flex flex-wrap gap-2 border-t",
                        for (label , offset) in presets {
                            Button {
                                key: "{offset}",
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Sm,
                                class: "flex-1",
                                onclick: move |_| {
                                    if let Some(today) = today() {
                                        let next = today.add_days(offset);
                                        date.set(Some(Some(next)));
                                        current_month.set(Some(next.month_start()));
                                    }
                                },
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarWithTime() -> Element {
    let mut date = use_from_today(|today| Some(CalendarDate::new(today.year, today.month, 12)));
    let time_input = "appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none";
    rsx! {
        Example { title: "Date and Time Picker", class: "h-[600px]",
            if let Some(chosen) = date() {
                Card { size: CardSize::Sm, class: "mx-auto w-fit",
                    CardContent {
                        Calendar {
                            mode: CalendarMode::Single,
                            selected: CalendarSelection::Single(chosen),
                            on_select: move |selection| date.set(Some(single(selection))),
                            class: "p-0",
                        }
                    }
                    CardFooter { class: "border-t bg-card",
                        FieldGroup {
                            Field {
                                FieldLabel { r#for: "time-from", "Start Time" }
                                InputGroup {
                                    InputGroupInput {
                                        id: "time-from",
                                        r#type: "time",
                                        step: "1",
                                        value: "10:30:00",
                                        class: time_input,
                                    }
                                    InputGroupAddon {
                                        Clock2 { size: 16, class: "text-muted-foreground" }
                                    }
                                }
                            }
                            Field {
                                FieldLabel { r#for: "time-to", "End Time" }
                                InputGroup {
                                    InputGroupInput {
                                        id: "time-to",
                                        r#type: "time",
                                        step: "1",
                                        value: "12:30:00",
                                        class: time_input,
                                    }
                                    InputGroupAddon {
                                        Clock2 { size: 16, class: "text-muted-foreground" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarBookedDates() -> Element {
    let mut date = use_from_today(|today| Some(CalendarDate::new(today.year, 1, 6)));
    let today = use_today();
    rsx! {
        Example { title: "Booked dates", class: "h-96",
            if let (Some(chosen), Some(today)) = (date(), today()) {
                {
                    let booked: Vec<CalendarMatcher> = (0..15)
                        .map(|offset| {
                            CalendarMatcher::Date(CalendarDate::new(today.year, 1, 12).add_days(offset))
                        })
                        .collect();
                    rsx! {
                        Card { class: "mx-auto w-fit p-0",
                            CardContent { class: "p-0",
                                Calendar {
                                    mode: CalendarMode::Single,
                                    default_month: chosen,
                                    selected: CalendarSelection::Single(chosen),
                                    on_select: move |selection| date.set(Some(single(selection))),
                                    disabled: booked.clone(),
                                    modifiers: vec![
                                        CalendarModifier {
                                            name: "booked".to_string(),
                                            matchers: booked,
                                            class_name: "[&>button]:line-through opacity-100".to_string(),
                                        },
                                    ],
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarCustomDays() -> Element {
    let mut selected = use_from_today(|today| {
        let from = CalendarDate::new(today.year, 12, 8);
        DateRange {
            from: Some(from),
            to: Some(from.add_days(10)),
        }
    });
    let mut locale = CalendarLocale::en_us();
    locale.months_short = locale.months;
    let day_button = use_callback(|props: CalendarDayProps| {
        let weekend = matches!(props.day.date.weekday(), 0 | 6);
        let outside = props.modifiers.outside;
        rsx! {
            CalendarDayButton { day: props.day, modifiers: props.modifiers,
                {props.children}
                if !outside {
                    span {
                        if weekend {
                            "$120"
                        } else {
                            "$100"
                        }
                    }
                }
            }
        }
    });
    rsx! {
        Example { title: "Custom Cell Size",
            if let Some(chosen) = selected() {
                Card { class: "mx-auto w-fit p-0",
                    CardContent { class: "p-0",
                        Calendar {
                            mode: CalendarMode::Range,
                            default_month: chosen.from,
                            selected: CalendarSelection::Range(chosen),
                            on_select: move |selection| selected.set(Some(range(selection))),
                            number_of_months: 1,
                            caption_layout: CalendarCaptionLayout::Dropdown,
                            class: "[--cell-size:--spacing(10)] md:[--cell-size:--spacing(12)]",
                            locale,
                            day_button,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarWeekNumbers() -> Element {
    let mut date = use_from_today(|today| Some(CalendarDate::new(today.year, 1, 12)));
    rsx! {
        Example { title: "Week Numbers", class: "h-96",
            if let Some(chosen) = date() {
                Card { class: "mx-auto w-fit p-0",
                    CardContent { class: "p-0",
                        Calendar {
                            mode: CalendarMode::Single,
                            default_month: chosen,
                            selected: CalendarSelection::Single(chosen),
                            on_select: move |selection| date.set(Some(single(selection))),
                            show_week_number: true,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarRtl() -> Element {
    let mut date = use_from_today(Some);
    rsx! {
        Example { title: "RTL", class: "h-96",
            if let Some(chosen) = date() {
                Calendar {
                    mode: CalendarMode::Single,
                    selected: CalendarSelection::Single(chosen),
                    on_select: move |selection| date.set(Some(single(selection))),
                    class: "rounded-lg border [--cell-size:--spacing(9)]",
                    caption_layout: CalendarCaptionLayout::Dropdown,
                    dir: "rtl",
                    locale: CalendarLocale::ar_sa(),
                }
            }
        }
    }
}
