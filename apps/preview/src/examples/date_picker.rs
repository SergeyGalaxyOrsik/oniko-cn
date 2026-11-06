//! Date Picker examples, ported from shadcn/ui `examples/base/date-picker-*.tsx`.
//!
//! `date-picker-natural-language` uses chrono-node upstream. `parse_natural_date` below understands a
//! small subset of it: today, tomorrow, yesterday, `in 2 days`, `3 weeks ago`, `next month`,
//! `next friday` and the explicit formats of `parse_date_text`.

use dioxus::prelude::*;
use lucide_dioxus::{Calendar as CalendarIcon, ChevronDown};

use super::Example;
use crate::components::ui::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::calendar::{
    Calendar, CalendarCaptionLayout, CalendarDate, CalendarLocale, CalendarMode, CalendarSelection,
    DateRange, use_today,
};
use crate::components::ui::date_picker::{DatePicker, format_date};
use crate::components::ui::field::{Field, FieldGroup, FieldLabel};
use crate::components::ui::input::Input;
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButtonSize, InputGroupInput,
    input_group_button_variants,
};
use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverTrigger};

#[component]
pub fn Examples() -> Element {
    rsx! {
        DatePickerDemo {}
        DatePickerBasic {}
        DatePickerSimple {}
        DatePickerWithRange {}
        DatePickerDob {}
        DatePickerInput {}
        DatePickerTime {}
        DatePickerNaturalLanguage {}
        DatePickerRtl {}
    }
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

/// `format(date, "LLL dd, y")`: `Jan 20, 2026`.
fn format_short(date: CalendarDate) -> String {
    let month = CalendarLocale::en_us().months_short[usize::from(date.month - 1)];
    format!("{month} {:02}, {}", date.day, date.year)
}

/// `date.toLocaleDateString()` in en-US: `6/5/2026`.
fn format_numeric(date: CalendarDate) -> String {
    format!("{}/{}/{}", date.month, date.day, date.year)
}

/// `date.toLocaleDateString("en-US", { day: "2-digit", month: "long", year: "numeric" })`: `June 01, 2025`.
fn format_long(date: CalendarDate) -> String {
    let month = CalendarLocale::en_us().months[usize::from(date.month - 1)];
    format!("{month} {:02}, {}", date.day, date.year)
}

fn month_number(name: &str) -> Option<u8> {
    let locale = CalendarLocale::en_us();
    let name = name.to_lowercase();
    (0..12u8)
        .find(|&i| {
            let full = locale.months[usize::from(i)].to_lowercase();
            name == full || name == full[..3] || (full == "september" && name == "sept")
        })
        .map(|i| i + 1)
}

fn valid_date(year: i32, month: u8, day: u8) -> Option<CalendarDate> {
    let date = CalendarDate::new(year, month, day);
    ((1..=12).contains(&month) && day >= 1 && day <= date.days_in_month()).then_some(date)
}

/// Understands `2025-06-01`, `6/1/2025`, `June 1, 2025` and `June 01 2025`, like `new Date(text)` does
/// for those inputs.
fn parse_date_text(text: &str) -> Option<CalendarDate> {
    let text = text.trim();
    let numbers = |separator: char| -> Option<Vec<i32>> {
        text.split(separator)
            .map(|part| part.trim().parse().ok())
            .collect()
    };
    if let Some(parts) = numbers('-')
        && let [year, month, day] = parts[..]
    {
        return valid_date(year, u8::try_from(month).ok()?, u8::try_from(day).ok()?);
    }
    if let Some(parts) = numbers('/')
        && let [month, day, year] = parts[..]
    {
        return valid_date(year, u8::try_from(month).ok()?, u8::try_from(day).ok()?);
    }
    let words: Vec<&str> = text
        .split([' ', ','])
        .filter(|word| !word.is_empty())
        .collect();
    if let [month, day, year] = words[..] {
        return valid_date(year.parse().ok()?, month_number(month)?, day.parse().ok()?);
    }
    None
}

/// Whole days, weeks, months or years from `today`, for phrases like `2 weeks` and `a month`.
fn shift(today: CalendarDate, amount: i64, unit: &str) -> Option<CalendarDate> {
    let unit = unit.trim_end_matches('s');
    Some(match unit {
        "day" => today.add_days(amount),
        "week" => today.add_days(amount * 7),
        "month" => today.add_months(i32::try_from(amount).ok()?),
        "year" => today.add_months(i32::try_from(amount * 12).ok()?),
        _ => return None,
    })
}

fn parse_natural_date(text: &str, today: CalendarDate) -> Option<CalendarDate> {
    let text = text.trim().to_lowercase();
    match text.as_str() {
        "today" | "now" => return Some(today),
        "tomorrow" => return Some(today.add_days(1)),
        "yesterday" => return Some(today.add_days(-1)),
        _ => {}
    }

    let words: Vec<&str> = text.split_whitespace().collect();
    let count = |word: &str| -> Option<i64> {
        match word {
            "a" | "an" | "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            other => other.parse().ok(),
        }
    };
    match words[..] {
        ["in", amount, unit] => return shift(today, count(amount)?, unit),
        [amount, unit, "ago"] => return shift(today, -count(amount)?, unit),
        ["next", unit] if shift(today, 1, unit).is_some() => return shift(today, 1, unit),
        ["last", unit] if shift(today, 1, unit).is_some() => return shift(today, -1, unit),
        _ => {}
    }

    let weekday = |word: &str| {
        CalendarLocale::en_us()
            .weekdays
            .iter()
            .position(|name| name.to_lowercase() == word)
    };
    let (skip_today, name) = match words[..] {
        ["next", name] => (true, name),
        ["this", name] | [name] => (false, name),
        _ => return parse_date_text(&text),
    };
    match weekday(name) {
        Some(target) => (i64::from(skip_today)..8)
            .map(|offset| today.add_days(offset))
            .find(|date| usize::from(date.weekday()) == target),
        None => parse_date_text(&text),
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

#[component]
fn DatePickerDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-96",
            DatePicker {}
        }
    }
}

#[component]
fn DatePickerBasic() -> Element {
    let mut date = use_signal(|| None::<CalendarDate>);
    rsx! {
        Example { title: "Basic", class: "h-96",
            Popover {
                PopoverTrigger {
                    class: button_variants(
                        ButtonVariant::Outline,
                        ButtonSize::Default,
                        Some("w-[212px] justify-between text-left font-normal data-[empty=true]:text-muted-foreground"),
                    ),
                    "data-empty": date().is_none().then_some("true"),
                    if let Some(chosen) = date() {
                        "{format_date(chosen)}"
                    } else {
                        span { "Pick a date" }
                    }
                    span { "data-icon": "inline-end", class: "contents", ChevronDown {} }
                }
                PopoverContent { class: "w-auto p-0", align: PopoverAlign::Start,
                    Calendar {
                        mode: CalendarMode::Single,
                        selected: CalendarSelection::Single(date()),
                        on_select: move |selection| date.set(single(selection)),
                        default_month: date(),
                    }
                }
            }
        }
    }
}

#[component]
fn DatePickerSimple() -> Element {
    let mut date = use_signal(|| None::<CalendarDate>);
    rsx! {
        Example { title: "Simple", class: "h-96",
            Field { class: "mx-auto w-44",
                FieldLabel { r#for: "date-picker-simple", "Date" }
                Popover {
                    PopoverTrigger {
                        id: "date-picker-simple",
                        class: button_variants(
                            ButtonVariant::Outline,
                            ButtonSize::Default,
                            Some("justify-start font-normal"),
                        ),
                        if let Some(chosen) = date() {
                            "{format_date(chosen)}"
                        } else {
                            span { "Pick a date" }
                        }
                    }
                    PopoverContent { class: "w-auto p-0", align: PopoverAlign::Start,
                        Calendar {
                            mode: CalendarMode::Single,
                            selected: CalendarSelection::Single(date()),
                            on_select: move |selection| date.set(single(selection)),
                            default_month: date(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DatePickerWithRange() -> Element {
    let mut date = use_from_today(|today| {
        let from = CalendarDate::new(today.year, 1, 20);
        DateRange {
            from: Some(from),
            to: Some(from.add_days(20)),
        }
    });
    rsx! {
        Example { title: "Range Picker", class: "h-[36rem] md:h-96",
            if let Some(chosen) = date() {
                Field { class: "mx-auto w-60",
                    FieldLabel { r#for: "date-picker-range", "Date Picker Range" }
                    Popover {
                        PopoverTrigger {
                            id: "date-picker-range",
                            class: button_variants(
                                ButtonVariant::Outline,
                                ButtonSize::Default,
                                Some("justify-start px-2.5 font-normal"),
                            ),
                            span { "data-icon": "inline-start", class: "contents", CalendarIcon {} }
                            match (chosen.from, chosen.to) {
                                (Some(from), Some(to)) => rsx! { "{format_short(from)} - {format_short(to)}" },
                                (Some(from), None) => rsx! { "{format_short(from)}" },
                                _ => rsx! {
                                    span { "Pick a date" }
                                },
                            }
                        }
                        PopoverContent { class: "w-auto p-0", align: PopoverAlign::Start,
                            Calendar {
                                mode: CalendarMode::Range,
                                default_month: chosen.from,
                                selected: CalendarSelection::Range(chosen),
                                on_select: move |selection| date.set(Some(range(selection))),
                                number_of_months: 2,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DatePickerDob() -> Element {
    let mut open = use_signal(|| false);
    let mut date = use_signal(|| None::<CalendarDate>);
    rsx! {
        Example { title: "Date of Birth", class: "h-96",
            Field { class: "mx-auto w-44",
                FieldLabel { r#for: "date", "Date of birth" }
                Popover { open: open(), on_open_change: move |next| open.set(next),
                    PopoverTrigger {
                        id: "date",
                        class: button_variants(
                            ButtonVariant::Outline,
                            ButtonSize::Default,
                            Some("justify-start font-normal"),
                        ),
                        if let Some(chosen) = date() {
                            "{format_numeric(chosen)}"
                        } else {
                            "Select date"
                        }
                    }
                    PopoverContent {
                        class: "w-auto overflow-hidden p-0",
                        align: PopoverAlign::Start,
                        Calendar {
                            mode: CalendarMode::Single,
                            selected: CalendarSelection::Single(date()),
                            default_month: date(),
                            caption_layout: CalendarCaptionLayout::Dropdown,
                            on_select: move |selection| {
                                date.set(single(selection));
                                open.set(false);
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DatePickerInput() -> Element {
    let initial = CalendarDate::new(2025, 6, 1);
    let mut open = use_signal(|| false);
    let mut date = use_signal(|| Some(initial));
    let mut month = use_signal(|| Some(initial));
    let mut value = use_signal(|| format_long(initial));
    rsx! {
        Example { title: "Input", class: "h-96",
            Field { class: "mx-auto w-48",
                FieldLabel { r#for: "date-required", "Subscription Date" }
                InputGroup {
                    InputGroupInput {
                        id: "date-required",
                        value: value(),
                        placeholder: "June 01, 2025",
                        oninput: move |event: FormEvent| {
                            value.set(event.value());
                            if let Some(parsed) = parse_date_text(&event.value()) {
                                date.set(Some(parsed));
                                month.set(Some(parsed));
                            }
                        },
                        onkeydown: move |event: KeyboardEvent| {
                            if event.key() == Key::ArrowDown {
                                event.prevent_default();
                                open.set(true);
                            }
                        },
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Popover { open: open(), on_open_change: move |next| open.set(next),
                            PopoverTrigger {
                                id: "date-picker",
                                class: input_group_button_variants(
                                    ButtonVariant::Ghost,
                                    InputGroupButtonSize::IconXs,
                                    None,
                                ),
                                "data-size": "icon-xs",
                                aria_label: "Select date",
                                CalendarIcon {}
                                span { class: "sr-only", "Select date" }
                            }
                            PopoverContent {
                                class: "w-auto overflow-hidden p-0",
                                align: PopoverAlign::End,
                                align_offset: -8.0,
                                side_offset: 10.0,
                                Calendar {
                                    mode: CalendarMode::Single,
                                    selected: CalendarSelection::Single(date()),
                                    month: month(),
                                    on_month_change: move |next| month.set(Some(next)),
                                    on_select: move |selection| {
                                        let chosen = single(selection);
                                        date.set(chosen);
                                        value.set(chosen.map(format_long).unwrap_or_default());
                                        open.set(false);
                                    },
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
fn DatePickerTime() -> Element {
    let mut open = use_signal(|| false);
    let mut date = use_signal(|| None::<CalendarDate>);
    rsx! {
        Example { title: "Time Picker", class: "h-96",
            FieldGroup { class: "mx-auto max-w-xs flex-row",
                Field {
                    FieldLabel { r#for: "date-picker-optional", "Date" }
                    Popover { open: open(), on_open_change: move |next| open.set(next),
                        PopoverTrigger {
                            id: "date-picker-optional",
                            class: button_variants(
                                ButtonVariant::Outline,
                                ButtonSize::Default,
                                Some("w-32 justify-between font-normal"),
                            ),
                            if let Some(chosen) = date() {
                                "{format_date(chosen)}"
                            } else {
                                "Select date"
                            }
                            span { "data-icon": "inline-end", class: "contents", ChevronDown {} }
                        }
                        PopoverContent {
                            class: "w-auto overflow-hidden p-0",
                            align: PopoverAlign::Start,
                            Calendar {
                                mode: CalendarMode::Single,
                                selected: CalendarSelection::Single(date()),
                                caption_layout: CalendarCaptionLayout::Dropdown,
                                default_month: date(),
                                on_select: move |selection| {
                                    date.set(single(selection));
                                    open.set(false);
                                },
                            }
                        }
                    }
                }
                Field { class: "w-32",
                    FieldLabel { r#for: "time-picker-optional", "Time" }
                    Input {
                        r#type: "time",
                        id: "time-picker-optional",
                        step: "1",
                        value: "10:30:00",
                        class: "appearance-none bg-background [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none",
                    }
                }
            }
        }
    }
}

#[component]
fn DatePickerNaturalLanguage() -> Element {
    let today = use_today();
    let mut open = use_signal(|| false);
    let mut value = use_signal(|| String::from("In 2 days"));
    let mut date = use_signal(|| None::<CalendarDate>);
    use_effect(move || {
        if date.peek().is_none()
            && let Some(today) = today()
        {
            date.set(parse_natural_date(&value.peek(), today));
        }
    });
    rsx! {
        Example { title: "Natural Language Picker", class: "h-96",
            Field { class: "mx-auto max-w-xs",
                FieldLabel { r#for: "date-optional", "Schedule Date" }
                InputGroup {
                    InputGroupInput {
                        id: "date-optional",
                        value: value(),
                        placeholder: "Tomorrow or next week",
                        oninput: move |event: FormEvent| {
                            value.set(event.value());
                            if let Some(today) = today()
                                && let Some(parsed) = parse_natural_date(&event.value(), today)
                            {
                                date.set(Some(parsed));
                            }
                        },
                        onkeydown: move |event: KeyboardEvent| {
                            if event.key() == Key::ArrowDown {
                                event.prevent_default();
                                open.set(true);
                            }
                        },
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Popover { open: open(), on_open_change: move |next| open.set(next),
                            PopoverTrigger {
                                id: "date-picker",
                                class: input_group_button_variants(
                                    ButtonVariant::Ghost,
                                    InputGroupButtonSize::IconXs,
                                    None,
                                ),
                                "data-size": "icon-xs",
                                aria_label: "Select date",
                                CalendarIcon {}
                                span { class: "sr-only", "Select date" }
                            }
                            PopoverContent {
                                class: "w-auto overflow-hidden p-0",
                                align: PopoverAlign::End,
                                side_offset: 8.0,
                                Calendar {
                                    mode: CalendarMode::Single,
                                    selected: CalendarSelection::Single(date()),
                                    caption_layout: CalendarCaptionLayout::Dropdown,
                                    default_month: date(),
                                    on_select: move |selection| {
                                        let chosen = single(selection);
                                        date.set(chosen);
                                        value.set(chosen.map(format_long).unwrap_or_default());
                                        open.set(false);
                                    },
                                }
                            }
                        }
                    }
                }
                div { class: "px-1 text-sm text-muted-foreground",
                    "Your post will be published on "
                    span { class: "font-medium", "{date().map(format_long).unwrap_or_default()}" }
                    "."
                }
            }
        }
    }
}

#[component]
fn DatePickerRtl() -> Element {
    let locale = CalendarLocale::ar_sa();
    let mut date = use_signal(|| None::<CalendarDate>);
    rsx! {
        Example { title: "RTL", class: "h-96",
            Popover {
                PopoverTrigger {
                    class: button_variants(
                        ButtonVariant::Outline,
                        ButtonSize::Default,
                        Some("w-[212px] justify-between text-left font-normal data-[empty=true]:text-muted-foreground"),
                    ),
                    dir: "rtl",
                    "data-empty": date().is_none().then_some("true"),
                    if let Some(chosen) = date() {
                        "{chosen.day} {locale.months[usize::from(chosen.month - 1)]} {chosen.year}"
                    } else {
                        span { "اختر تاريخًا" }
                    }
                    span { "data-icon": "inline-end", class: "contents", ChevronDown {} }
                }
                PopoverContent { class: "w-auto p-0", align: PopoverAlign::Start, dir: "rtl",
                    Calendar {
                        mode: CalendarMode::Single,
                        selected: CalendarSelection::Single(date()),
                        on_select: move |selection| date.set(single(selection)),
                        default_month: date(),
                        dir: "rtl",
                        locale: CalendarLocale::ar_sa(),
                    }
                }
            }
        }
    }
}
