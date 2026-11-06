//! Calendar: a date picker grid modelled on react-day-picker, which shadcn/ui's Calendar wraps.
//!
//! Supports `single`, `multiple` and `range` selection, one or more months, a month and year dropdown caption,
//! week numbers, disabled, hidden and custom-modified days, outside days, a `today` marker, locales, right to
//! left layout and keyboard navigation in the grid (arrows, `Home`, `End`, `PageUp`, `PageDown`, with `Shift`
//! for years). The grid is computed here with plain date arithmetic, so the component needs no date crate and
//! no browser APIs. Today's date is read once with `document::eval`, or taken from the `today` prop.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// A day in the proleptic Gregorian calendar. Month is 1-12, day is 1-31 for that month.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn days_in_month(self) -> u8 {
        match self.month {
            2 if Self::is_leap_year(self.year) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    /// Days since 1970-01-01.
    fn to_days(self) -> i64 {
        let year = i64::from(self.year) - i64::from(self.month <= 2);
        let era = year.div_euclid(400);
        let year_of_era = year.rem_euclid(400);
        let month = i64::from(self.month);
        let shifted = if month > 2 { month - 3 } else { month + 9 };
        let day_of_year = (153 * shifted + 2) / 5 + i64::from(self.day) - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    fn from_days(days: i64) -> Self {
        let shifted = days + 719_468;
        let era = shifted.div_euclid(146_097);
        let day_of_era = shifted.rem_euclid(146_097);
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_shifted = (5 * day_of_year + 2) / 153;
        let day = (day_of_year - (153 * month_shifted + 2) / 5 + 1) as u8;
        let month = if month_shifted < 10 {
            month_shifted + 3
        } else {
            month_shifted - 9
        } as u8;
        let year = year_of_era + era * 400 + i64::from(month <= 2);
        Self {
            year: year as i32,
            month,
            day,
        }
    }

    /// Day of the week, 0 for Sunday.
    pub fn weekday(self) -> u8 {
        (self.to_days() + 4).rem_euclid(7) as u8
    }

    pub fn add_days(self, delta: i64) -> Self {
        Self::from_days(self.to_days() + delta)
    }

    /// The same day `delta` months away. The day is clamped to the length of the target month.
    pub fn add_months(self, delta: i32) -> Self {
        let total = self.year * 12 + i32::from(self.month - 1) + delta;
        let mut moved = Self {
            year: total.div_euclid(12),
            month: total.rem_euclid(12) as u8 + 1,
            day: 1,
        };
        moved.day = self.day.min(moved.days_in_month());
        moved
    }

    pub fn month_start(self) -> Self {
        Self { day: 1, ..self }
    }

    fn month_end(self) -> Self {
        Self {
            day: self.days_in_month(),
            ..self
        }
    }

    fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// `M/D/YYYY`, what `toLocaleDateString` gives in `en-US`.
    fn short(self) -> String {
        format!("{}/{}/{}", self.month, self.day, self.year)
    }

    fn start_of_week(self, week_starts_on: u8) -> Self {
        let offset = (self.weekday() + 7 - week_starts_on) % 7;
        self.add_days(-i64::from(offset))
    }

    /// Week of the year where week 1 contains January 1, like date-fns with `firstWeekContainsDate: 1`.
    fn week_number(self, week_starts_on: u8) -> u32 {
        let start = self.start_of_week(week_starts_on);
        let next_first = Self::new(self.year + 1, 1, 1).start_of_week(week_starts_on);
        if start >= next_first {
            return 1;
        }
        let first = Self::new(self.year, 1, 1).start_of_week(week_starts_on);
        ((start.to_days() - first.to_days()) / 7 + 1) as u32
    }
}

fn ordinal(day: u8) -> &'static str {
    match (day % 10, day % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    }
}

/// An inclusive span of days. Either end may be open.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateRange {
    pub from: Option<CalendarDate>,
    pub to: Option<CalendarDate>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarMode {
    #[default]
    Single,
    Multiple,
    Range,
}

impl CalendarMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
            Self::Range => "range",
        }
    }

    fn empty_selection(self) -> CalendarSelection {
        match self {
            Self::Single => CalendarSelection::Single(None),
            Self::Multiple => CalendarSelection::Multiple(Vec::new()),
            Self::Range => CalendarSelection::Range(DateRange::default()),
        }
    }
}

/// The selected days. The variant must match the calendar's `mode`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CalendarSelection {
    Single(Option<CalendarDate>),
    Multiple(Vec<CalendarDate>),
    Range(DateRange),
}

impl CalendarSelection {
    fn first(&self) -> Option<CalendarDate> {
        match self {
            Self::Single(date) => *date,
            Self::Multiple(dates) => dates.first().copied(),
            Self::Range(range) => range.from.or(range.to),
        }
    }
}

/// Adds `date` to a range the way react-day-picker does: the first click starts it, the second completes it,
/// and a click on the start of a lone day clears it.
fn add_to_range(date: CalendarDate, range: DateRange) -> DateRange {
    let clear = DateRange::default();
    match (range.from, range.to) {
        (None, _) => DateRange {
            from: Some(date),
            to: Some(date),
        },
        (Some(from), None) => {
            if from == date {
                clear
            } else if date < from {
                DateRange {
                    from: Some(date),
                    to: Some(from),
                }
            } else {
                DateRange {
                    from: Some(from),
                    to: Some(date),
                }
            }
        }
        (Some(from), Some(to)) => {
            if from == date && to == date {
                clear
            } else if from == date {
                DateRange {
                    from: Some(from),
                    to: Some(date),
                }
            } else if to == date {
                DateRange {
                    from: Some(date),
                    to: Some(date),
                }
            } else if date < from {
                DateRange {
                    from: Some(date),
                    to: Some(to),
                }
            } else {
                DateRange {
                    from: Some(from),
                    to: Some(date),
                }
            }
        }
    }
}

/// Picks which days a flag such as `disabled` or `hidden` applies to.
#[derive(Clone, PartialEq)]
pub enum CalendarMatcher {
    /// Every day.
    All,
    Date(CalendarDate),
    Between(DateRange),
    Before(CalendarDate),
    After(CalendarDate),
    /// Days of the week, 0 for Sunday.
    DayOfWeek(Vec<u8>),
    Custom(Callback<CalendarDate, bool>),
}

impl CalendarMatcher {
    fn matches(&self, date: CalendarDate) -> bool {
        match self {
            Self::All => true,
            Self::Date(other) => *other == date,
            Self::Between(range) => {
                range.from.is_none_or(|from| date >= from) && range.to.is_none_or(|to| date <= to)
            }
            Self::Before(other) => date < *other,
            Self::After(other) => date > *other,
            Self::DayOfWeek(days) => days.contains(&date.weekday()),
            Self::Custom(callback) => callback.call(date),
        }
    }
}

fn any_matches(matchers: &[CalendarMatcher], date: CalendarDate) -> bool {
    matchers.iter().any(|matcher| matcher.matches(date))
}

/// A custom day flag. Matching days get `class_name` on their cell and are listed in `CalendarModifiers::custom`.
#[derive(Clone, PartialEq)]
pub struct CalendarModifier {
    pub name: String,
    pub matchers: Vec<CalendarMatcher>,
    pub class_name: String,
}

/// How the month and year are shown in the caption.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarCaptionLayout {
    /// Plain text.
    #[default]
    Label,
    /// Month and year dropdowns.
    Dropdown,
    DropdownMonths,
    DropdownYears,
}

impl CalendarCaptionLayout {
    fn month_dropdown(self) -> bool {
        matches!(self, Self::Dropdown | Self::DropdownMonths)
    }

    fn year_dropdown(self) -> bool {
        matches!(self, Self::Dropdown | Self::DropdownYears)
    }
}

/// Names used in the caption, weekday header and labels.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarLocale {
    pub code: &'static str,
    pub months: [&'static str; 12],
    pub months_short: [&'static str; 12],
    pub weekdays: [&'static str; 7],
    /// Header cell text, starting with Sunday.
    pub weekdays_short: [&'static str; 7],
    /// 0 for Sunday.
    pub week_starts_on: u8,
}

impl CalendarLocale {
    pub fn en_us() -> Self {
        Self {
            code: "en-US",
            months: [
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
            ],
            months_short: [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ],
            weekdays: [
                "Sunday",
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
            ],
            weekdays_short: ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"],
            week_starts_on: 0,
        }
    }

    pub fn ar_sa() -> Self {
        Self {
            code: "ar-SA",
            months: [
                "يناير",
                "فبراير",
                "مارس",
                "أبريل",
                "مايو",
                "يونيو",
                "يوليو",
                "أغسطس",
                "سبتمبر",
                "أكتوبر",
                "نوفمبر",
                "ديسمبر",
            ],
            months_short: [
                "يناير",
                "فبراير",
                "مارس",
                "أبريل",
                "مايو",
                "يونيو",
                "يوليو",
                "أغسطس",
                "سبتمبر",
                "أكتوبر",
                "نوفمبر",
                "ديسمبر",
            ],
            weekdays: [
                "الأحد",
                "الاثنين",
                "الثلاثاء",
                "الأربعاء",
                "الخميس",
                "الجمعة",
                "السبت",
            ],
            weekdays_short: ["ح", "ن", "ث", "ر", "خ", "ج", "س"],
            week_starts_on: 6,
        }
    }

    pub fn he() -> Self {
        Self {
            code: "he",
            months: [
                "ינואר",
                "פברואר",
                "מרץ",
                "אפריל",
                "מאי",
                "יוני",
                "יולי",
                "אוגוסט",
                "ספטמבר",
                "אוקטובר",
                "נובמבר",
                "דצמבר",
            ],
            months_short: [
                "ינו׳",
                "פבר׳",
                "מרץ",
                "אפר׳",
                "מאי",
                "יוני",
                "יולי",
                "אוג׳",
                "ספט׳",
                "אוק׳",
                "נוב׳",
                "דצמ׳",
            ],
            weekdays: [
                "יום ראשון",
                "יום שני",
                "יום שלישי",
                "יום רביעי",
                "יום חמישי",
                "יום שישי",
                "יום שבת",
            ],
            weekdays_short: ["א׳", "ב׳", "ג׳", "ד׳", "ה׳", "ו׳", "ש׳"],
            week_starts_on: 0,
        }
    }
}

impl Default for CalendarLocale {
    fn default() -> Self {
        Self::en_us()
    }
}

/// A day in the grid. `outside` is true for days that belong to a neighbouring month.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarDay {
    pub date: CalendarDate,
    pub outside: bool,
}

/// The flags of a day, like react-day-picker's `modifiers`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CalendarModifiers {
    pub selected: bool,
    pub disabled: bool,
    pub hidden: bool,
    pub outside: bool,
    pub today: bool,
    pub focused: bool,
    pub range_start: bool,
    pub range_middle: bool,
    pub range_end: bool,
    /// Names of the matching custom modifiers.
    pub custom: Vec<String>,
}

/// What a custom `day_button` receives. Render a `CalendarDayButton` with it.
#[derive(Clone, PartialEq)]
pub struct CalendarDayProps {
    pub day: CalendarDay,
    pub modifiers: CalendarModifiers,
    /// The formatted day of the month.
    pub children: Element,
}

/// Reads today's date from the browser or webview clock once, on mount. `None` until it is known.
pub fn use_today() -> Signal<Option<CalendarDate>> {
    let mut today = use_signal(|| None);
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                "const d = new Date(); dioxus.send([d.getFullYear(), d.getMonth() + 1, d.getDate()]);",
            );
            if let Ok((year, month, day)) = eval.recv::<(i32, u8, u8)>().await {
                today.set(Some(CalendarDate::new(year, month, day)));
            }
        });
    });
    today
}

async fn next_frame() {
    let mut done = document::eval(
        "requestAnimationFrame(() => requestAnimationFrame(() => dioxus.send(true)));",
    );
    let _ = done.recv::<bool>().await;
}

#[derive(Clone, Copy)]
struct CalendarContext {
    id: u64,
    mode: Signal<CalendarMode>,
    selection: Signal<CalendarSelection>,
    internal_selection: Signal<CalendarSelection>,
    controlled_selection: Signal<Option<CalendarSelection>>,
    on_select: Option<EventHandler<CalendarSelection>>,
    first_month: Signal<CalendarDate>,
    internal_month: Signal<Option<CalendarDate>>,
    controlled_month: Signal<Option<CalendarDate>>,
    on_month_change: Option<EventHandler<CalendarDate>>,
    number_of_months: Signal<usize>,
    nav_start: Signal<Option<CalendarDate>>,
    nav_end: Signal<Option<CalendarDate>>,
    focused: Signal<Option<CalendarDate>>,
    focus_target: Signal<Option<CalendarDate>>,
    focus_request: Signal<Option<(CalendarDate, u64)>>,
    today: Signal<Option<CalendarDate>>,
    disabled: Signal<Vec<CalendarMatcher>>,
    hidden: Signal<Vec<CalendarMatcher>>,
    modifiers: Signal<Vec<CalendarModifier>>,
    show_outside_days: Signal<bool>,
    show_week_number: Signal<bool>,
    fixed_weeks: Signal<bool>,
    rtl: Signal<bool>,
    locale: Signal<CalendarLocale>,
    caption_layout: Signal<CalendarCaptionLayout>,
    button_variant: Signal<ButtonVariant>,
    day_button: Signal<Option<Callback<CalendarDayProps, Element>>>,
}

impl CalendarContext {
    fn last_month(self) -> CalendarDate {
        (self.first_month)().add_months((self.number_of_months)() as i32 - 1)
    }

    fn is_displayed(self, date: CalendarDate) -> bool {
        let month = date.month_start();
        month >= (self.first_month)() && month <= self.last_month()
    }

    fn go_to_month(self, target: CalendarDate) {
        let count = *self.number_of_months.peek() as i32;
        let mut first = target.month_start();
        if let Some(end) = *self.nav_end.peek() {
            let latest = end.month_start().add_months(1 - count);
            if first > latest {
                first = latest;
            }
        }
        if let Some(start) = *self.nav_start.peek()
            && first < start.month_start()
        {
            first = start.month_start();
        }
        if first == *self.first_month.peek() {
            return;
        }
        if self.controlled_month.peek().is_none() {
            let mut internal = self.internal_month;
            internal.set(Some(first));
        }
        if let Some(handler) = self.on_month_change {
            handler.call(first);
        }
    }

    fn is_blocked(self, date: CalendarDate) -> bool {
        any_matches(&self.disabled.peek(), date)
    }

    fn modifiers_of(self, date: CalendarDate, outside: bool) -> CalendarModifiers {
        let displayed_elsewhere = outside
            && (self.number_of_months)() > 1
            && self.is_displayed(date);
        let hidden = (outside && (!(self.show_outside_days)() || displayed_elsewhere))
            || any_matches(&self.hidden.read(), date);
        let selection = self.selection.read();
        let (selected, range_start, range_middle, range_end) = match &*selection {
            CalendarSelection::Single(chosen) => (*chosen == Some(date), false, false, false),
            CalendarSelection::Multiple(chosen) => (chosen.contains(&date), false, false, false),
            CalendarSelection::Range(range) => {
                let start = range.from == Some(date);
                let end = range.to == Some(date) || (range.to.is_none() && start);
                let middle = matches!((range.from, range.to), (Some(from), Some(to)) if date > from && date < to);
                (start || end || middle, start, middle, end)
            }
        };
        CalendarModifiers {
            selected,
            disabled: any_matches(&self.disabled.read(), date),
            hidden,
            outside,
            today: (self.today)() == Some(date),
            focused: !outside && (self.focused)() == Some(date),
            range_start,
            range_middle,
            range_end,
            custom: self
                .modifiers
                .read()
                .iter()
                .filter(|modifier| any_matches(&modifier.matchers, date))
                .map(|modifier| modifier.name.clone())
                .collect(),
        }
    }

    fn click_day(self, date: CalendarDate) {
        let mut focused = self.focused;
        focused.set(Some(date));
        if !self.is_displayed(date) {
            self.go_to_month(date);
        }
        let current = self.selection.peek().clone();
        let next = match (&current, *self.mode.peek()) {
            (CalendarSelection::Single(chosen), _) => {
                CalendarSelection::Single((*chosen != Some(date)).then_some(date))
            }
            (CalendarSelection::Multiple(chosen), _) => {
                let mut next = chosen.clone();
                if let Some(index) = next.iter().position(|candidate| *candidate == date) {
                    next.remove(index);
                } else {
                    next.push(date);
                }
                CalendarSelection::Multiple(next)
            }
            (CalendarSelection::Range(range), _) => {
                CalendarSelection::Range(add_to_range(date, *range))
            }
        };
        if self.controlled_selection.peek().is_none() {
            let mut internal = self.internal_selection;
            internal.set(next.clone());
        }
        if let Some(handler) = self.on_select {
            handler.call(next);
        }
    }

    /// Moves keyboard focus to the first enabled day reached by repeating `step` from `from`.
    fn move_focus(self, from: CalendarDate, step: impl Fn(CalendarDate) -> CalendarDate) {
        let start = (*self.nav_start.peek()).map(CalendarDate::month_start);
        let end = (*self.nav_end.peek()).map(CalendarDate::month_end);
        let mut candidate = step(from);
        for _ in 0..400 {
            if start.is_some_and(|start| candidate < start) || end.is_some_and(|end| candidate > end)
            {
                return;
            }
            if !self.is_blocked(candidate) {
                break;
            }
            candidate = step(candidate);
        }
        if self.is_blocked(candidate) {
            return;
        }
        let count = *self.number_of_months.peek() as i32;
        let month = candidate.month_start();
        if month < *self.first_month.peek() {
            self.go_to_month(month);
        } else if month > self.last_month() {
            self.go_to_month(month.add_months(1 - count));
        }
        let mut focused = self.focused;
        focused.set(Some(candidate));
        let mut request = self.focus_request;
        let nonce = request.peek().map_or(0, |(_, nonce)| nonce) + 1;
        request.set(Some((candidate, nonce)));
    }
}

fn is_rtl(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.name == "dir"
            && matches!(&attribute.value, dioxus::core::AttributeValue::Text(text) if text == "rtl")
    })
}

fn month_weeks(month: CalendarDate, week_starts_on: u8, fixed: bool) -> Vec<Vec<CalendarDate>> {
    let first = month.start_of_week(week_starts_on);
    let offset = (month.weekday() + 7 - week_starts_on) % 7;
    let weeks = if fixed {
        6
    } else {
        (usize::from(offset) + usize::from(month.days_in_month())).div_ceil(7)
    };
    (0..weeks)
        .map(|week| {
            (0..7)
                .map(|day| first.add_days((week * 7 + day) as i64))
                .collect()
        })
        .collect()
}

/// A date picker. `mode` decides what `selected` and `on_select` carry. Pass `selected` to control the
/// selection, or `default_selected` to let the calendar keep its own.
///
/// Extra attributes, such as `class` and `dir`, go to the root element.
#[component]
pub fn Calendar(
    #[props(default)] mode: CalendarMode,
    /// Controlled selection. When omitted, the calendar keeps its own.
    selected: Option<CalendarSelection>,
    /// Initial selection of an uncontrolled calendar.
    default_selected: Option<CalendarSelection>,
    on_select: Option<EventHandler<CalendarSelection>>,
    /// Controlled first month shown.
    month: Option<CalendarDate>,
    /// Month shown first. Defaults to the selected day's month, then to today's.
    default_month: Option<CalendarDate>,
    on_month_change: Option<EventHandler<CalendarDate>>,
    #[props(default = 1)] number_of_months: usize,
    #[props(default)] caption_layout: CalendarCaptionLayout,
    #[props(default = true)] show_outside_days: bool,
    #[props(default)] show_week_number: bool,
    /// Always show six weeks per month.
    #[props(default)]
    fixed_weeks: bool,
    /// Overrides the clock for the `today` marker.
    today: Option<CalendarDate>,
    /// First month that can be shown. The dropdown caption defaults to 100 years before today.
    start_month: Option<CalendarDate>,
    /// Last month that can be shown. The dropdown caption defaults to 100 years after today.
    end_month: Option<CalendarDate>,
    #[props(default)] disabled: Vec<CalendarMatcher>,
    #[props(default)] hidden: Vec<CalendarMatcher>,
    #[props(default)] modifiers: Vec<CalendarModifier>,
    #[props(default = ButtonVariant::Ghost)] button_variant: ButtonVariant,
    #[props(default)] locale: CalendarLocale,
    /// Replaces the day button. Render a `CalendarDayButton` with the props it gets.
    day_button: Option<Callback<CalendarDayProps, Element>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let detected = use_today();
    let today_now = today.or(detected());
    let rtl = is_rtl(&attributes);

    let internal_selection = use_signal(|| {
        default_selected
            .clone()
            .or_else(|| selected.clone())
            .unwrap_or_else(|| mode.empty_selection())
    });
    let controlled_selection = use_prop(selected.clone());
    let effective_selection = selected
        .clone()
        .unwrap_or_else(|| internal_selection.read().clone());
    let selection = use_prop(effective_selection.clone());

    let mut internal_month = use_signal(|| {
        month
            .or(default_month)
            .or_else(|| effective_selection.first())
            .map(CalendarDate::month_start)
    });
    let controlled_month = use_prop(month.map(CalendarDate::month_start));
    use_effect(move || {
        if internal_month.peek().is_none()
            && let Some(today) = today_now
        {
            internal_month.set(Some(today.month_start()));
        }
    });
    let ready = month.is_some() || internal_month().is_some();
    let first = month
        .map(CalendarDate::month_start)
        .or(internal_month())
        .unwrap_or(CalendarDate::new(1970, 1, 1));

    let dropdown = caption_layout != CalendarCaptionLayout::Label;
    let reference = today_now.unwrap_or(first);
    let nav_start = start_month.or_else(|| {
        dropdown.then(|| CalendarDate::new(reference.year - 100, 1, 1))
    });
    let nav_end = end_month.or_else(|| {
        dropdown.then(|| CalendarDate::new(reference.year + 100, 12, 1))
    });

    let first_month = use_prop(first);
    let number_of_months = use_prop(number_of_months.max(1));
    let focused = use_signal(|| None);
    let focus_target = use_signal(|| None);
    let focus_request = use_signal(|| None);
    let locale_signal = use_prop(locale.clone());
    let mode_prop = use_prop(mode);
    let nav_start_prop = use_prop(nav_start);
    let nav_end_prop = use_prop(nav_end);
    let today_prop = use_prop(today_now);
    let disabled_prop = use_prop(disabled);
    let hidden_prop = use_prop(hidden);
    let modifiers_prop = use_prop(modifiers);
    let show_outside_days_prop = use_prop(show_outside_days);
    let show_week_number_prop = use_prop(show_week_number);
    let fixed_weeks_prop = use_prop(fixed_weeks);
    let rtl_prop = use_prop(rtl);
    let caption_layout_prop = use_prop(caption_layout);
    let button_variant_prop = use_prop(button_variant);
    let day_button_prop = use_prop(day_button);
    let context = use_context_provider(|| CalendarContext {
        id,
        mode: mode_prop,
        selection,
        internal_selection,
        controlled_selection,
        on_select,
        first_month,
        internal_month,
        controlled_month,
        on_month_change,
        number_of_months,
        nav_start: nav_start_prop,
        nav_end: nav_end_prop,
        focused,
        focus_target,
        focus_request,
        today: today_prop,
        disabled: disabled_prop,
        hidden: hidden_prop,
        modifiers: modifiers_prop,
        show_outside_days: show_outside_days_prop,
        show_week_number: show_week_number_prop,
        fixed_weeks: fixed_weeks_prop,
        rtl: rtl_prop,
        locale: locale_signal,
        caption_layout: caption_layout_prop,
        button_variant: button_variant_prop,
        day_button: day_button_prop,
    });

    let mut target_signal = focus_target;
    let target = {
        let interior = |date: &CalendarDate| context.is_displayed(*date);
        let focused_day = focused().filter(interior);
        let selected_day = match &*selection.read() {
            CalendarSelection::Single(date) => date.iter().copied().find(interior),
            CalendarSelection::Multiple(dates) => dates.iter().copied().find(interior),
            CalendarSelection::Range(range) => [range.from, range.to]
                .into_iter()
                .flatten()
                .find(interior),
        };
        focused_day
            .or(selected_day)
            .or(today_now.filter(interior))
            .or_else(|| {
                (0..i64::from(first.days_in_month()))
                    .map(|offset| first.add_days(offset))
                    .find(|date| !context.is_blocked(*date))
            })
    };
    if *target_signal.peek() != target {
        target_signal.set(target);
    }

    use_effect(move || {
        if let Some((date, _)) = focus_request() {
            let script = format!(
                "document.querySelector('#calendar-{id} td[data-day=\"{}\"]:not([data-outside]) button')?.focus();",
                date.iso()
            );
            spawn(async move {
                next_frame().await;
                let _ = document::eval(&script).await;
            });
        }
    });

    let mut attributes = attributes;
    let class = cn!(
        "w-fit rdp-root",
        "cn-calendar group/calendar bg-background in-data-[slot=card-content]:bg-transparent in-data-[slot=popover-content]:bg-transparent",
        r"rtl:**:[.rdp-button\_next>svg]:rotate-180",
        r"rtl:**:[.rdp-button\_previous>svg]:rotate-180",
        take_class(&mut attributes).as_deref()
    );
    let count = number_of_months();
    let nav_button_class = |extra: &str| {
        button_variants(
            button_variant,
            ButtonSize::Default,
            Some(&format!(
                "size-(--cell-size) p-0 select-none aria-disabled:opacity-50 {extra}"
            )),
        )
    };
    let previous_disabled = nav_start.is_some_and(|start| first <= start.month_start());
    let next_disabled = nav_end.is_some_and(|end| context.last_month() >= end.month_start());

    rsx! {
        div {
            id: "calendar-{id}",
            "data-slot": "calendar",
            "data-mode": mode.as_str(),
            "data-multiple-months": (count > 1).then_some("true"),
            "data-week-numbers": show_week_number.then_some("true"),
            lang: locale.code,
            class: class,
            style: (!ready).then_some("visibility:hidden"),
            ..attributes,
            div { class: "relative flex flex-col gap-4 md:flex-row rdp-months",
                nav { class: "absolute inset-x-0 top-0 flex w-full items-center justify-between gap-1 rdp-nav",
                    button {
                        r#type: "button",
                        aria_label: "Go to the Previous Month",
                        "aria-disabled": previous_disabled.then_some("true"),
                        disabled: previous_disabled,
                        class: nav_button_class("rdp-button_previous"),
                        onclick: move |_| context.go_to_month(first.add_months(-1)),
                        CalendarChevron { orientation: ChevronOrientation::Left }
                    }
                    button {
                        r#type: "button",
                        aria_label: "Go to the Next Month",
                        "aria-disabled": next_disabled.then_some("true"),
                        disabled: next_disabled,
                        class: nav_button_class("rdp-button_next"),
                        onclick: move |_| context.go_to_month(first.add_months(1)),
                        CalendarChevron { orientation: ChevronOrientation::Right }
                    }
                }
                for index in 0..count {
                    CalendarMonthView { key: "{index}", index }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChevronOrientation {
    Left,
    Right,
    Down,
}

#[component]
fn CalendarChevron(orientation: ChevronOrientation) -> Element {
    let (class, path) = match orientation {
        ChevronOrientation::Left => ("cn-rtl-flip size-4 rdp-chevron", "m15 18-6-6 6-6"),
        ChevronOrientation::Right => ("cn-rtl-flip size-4 rdp-chevron", "m9 18 6-6-6-6"),
        ChevronOrientation::Down => ("size-4 rdp-chevron", "m6 9 6 6 6-6"),
    };
    rsx! {
        svg {
            class: class,
            "aria-hidden": "true",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: path }
        }
    }
}

#[component]
fn CalendarMonthView(index: usize) -> Element {
    let context = use_context::<CalendarContext>();
    let first = (context.first_month)();
    let month = first.add_months(index as i32);
    let locale = (context.locale)();
    let layout = (context.caption_layout)();
    let weeks = month_weeks(month, locale.week_starts_on, (context.fixed_weeks)());
    let show_week_number = (context.show_week_number)();
    let weekday_order: Vec<usize> = (0..7)
        .map(|offset| (usize::from(locale.week_starts_on) + offset) % 7)
        .collect();
    let caption = format!("{} {}", locale.months[usize::from(month.month - 1)], month.year);
    let nav_start = (context.nav_start)().map(CalendarDate::month_start);
    let nav_end = (context.nav_end)().map(CalendarDate::month_start);
    let month_label = locale.months_short[usize::from(month.month - 1)];
    let caption_label_class = "font-medium select-none cn-calendar-caption-label flex items-center gap-1 rounded-(--cell-radius) text-sm [&>svg]:size-3.5 [&>svg]:text-muted-foreground rdp-caption_label";

    let show_month = move |target_month: u8| {
        context.go_to_month(
            CalendarDate::new(month.year, target_month, 1).add_months(-(index as i32)),
        );
    };
    let show_year = move |target_year: i32| {
        context.go_to_month(
            CalendarDate::new(target_year, month.month, 1).add_months(-(index as i32)),
        );
    };
    let year_from = nav_start.map_or(month.year - 100, |start| start.year);
    let year_to = nav_end.map_or(month.year + 100, |end| end.year);

    rsx! {
        div { class: "flex w-full flex-col gap-4 rdp-month",
            div { class: "flex h-(--cell-size) w-full items-center justify-center px-(--cell-size) rdp-month_caption",
                if layout == CalendarCaptionLayout::Label {
                    span {
                        role: "status",
                        aria_live: "polite",
                        class: "font-medium select-none cn-calendar-caption text-sm rdp-caption_label",
                        "{caption}"
                    }
                } else {
                    div { class: "flex h-(--cell-size) w-full items-center justify-center gap-1.5 text-sm font-medium rdp-dropdowns",
                        if layout.month_dropdown() {
                            span { class: "cn-calendar-dropdown-root relative rounded-(--cell-radius) rdp-dropdown_root",
                                select {
                                    aria_label: "Choose the Month",
                                    class: "absolute inset-0 bg-popover opacity-0 rdp-dropdown",
                                    onchange: move |event| {
                                        if let Ok(value) = event.value().parse::<u8>() {
                                            show_month(value);
                                        }
                                    },
                                    for number in 1..=12u8 {
                                        option {
                                            key: "{number}",
                                            value: "{number}",
                                            selected: number == month.month,
                                            disabled: nav_start.is_some_and(|start| CalendarDate::new(month.year, number, 1) < start)
                                                || nav_end.is_some_and(|end| CalendarDate::new(month.year, number, 1) > end),
                                            "{locale.months_short[usize::from(number - 1)]}"
                                        }
                                    }
                                }
                                span { "aria-hidden": "true", class: caption_label_class,
                                    "{month_label}"
                                    CalendarChevron { orientation: ChevronOrientation::Down }
                                }
                            }
                        } else {
                            span {
                                role: "status",
                                aria_live: "polite",
                                class: "font-medium select-none cn-calendar-caption-label flex items-center gap-1 rounded-(--cell-radius) text-sm rdp-caption_label",
                                "{locale.months[usize::from(month.month - 1)]}"
                            }
                        }
                        if layout.year_dropdown() {
                            span { class: "cn-calendar-dropdown-root relative rounded-(--cell-radius) rdp-dropdown_root",
                                select {
                                    aria_label: "Choose the Year",
                                    class: "absolute inset-0 bg-popover opacity-0 rdp-dropdown",
                                    onchange: move |event| {
                                        if let Ok(value) = event.value().parse::<i32>() {
                                            show_year(value);
                                        }
                                    },
                                    for year in year_from..=year_to {
                                        option {
                                            key: "{year}",
                                            value: "{year}",
                                            selected: year == month.year,
                                            "{year}"
                                        }
                                    }
                                }
                                span { "aria-hidden": "true", class: caption_label_class,
                                    "{month.year}"
                                    CalendarChevron { orientation: ChevronOrientation::Down }
                                }
                            }
                        } else {
                            span {
                                role: "status",
                                aria_live: "polite",
                                class: "font-medium select-none cn-calendar-caption-label flex items-center gap-1 rounded-(--cell-radius) text-sm rdp-caption_label",
                                "{month.year}"
                            }
                        }
                    }
                }
            }
            table {
                role: "grid",
                aria_multiselectable: (*context.mode.read() != CalendarMode::Single).then_some("true"),
                aria_label: "{caption}",
                class: "w-full border-collapse rdp-month_grid",
                thead { "aria-hidden": "true",
                    tr { class: "flex rdp-weekdays",
                        if show_week_number {
                            th {
                                scope: "col",
                                aria_label: "Week Number",
                                class: "w-(--cell-size) select-none rdp-week_number_header",
                                "#"
                            }
                        }
                        for day in weekday_order {
                            th {
                                key: "{day}",
                                scope: "col",
                                aria_label: locale.weekdays[day],
                                class: "flex-1 rounded-(--cell-radius) text-[0.8rem] font-normal text-muted-foreground select-none rdp-weekday",
                                "{locale.weekdays_short[day]}"
                            }
                        }
                    }
                }
                tbody {
                    for week in weeks {
                        tr {
                            key: "{week[0].iso()}",
                            class: "mt-2 flex w-full rdp-week",
                            if show_week_number {
                                td {
                                    role: "rowheader",
                                    aria_label: "Week {week[0].week_number(locale.week_starts_on)}",
                                    class: "text-[0.8rem] text-muted-foreground select-none rdp-week_number",
                                    div { class: "flex size-(--cell-size) items-center justify-center text-center",
                                        "{week[0].week_number(locale.week_starts_on)}"
                                    }
                                }
                            }
                            for date in week {
                                CalendarDayCell { key: "{date.iso()}", date, month }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarDayCell(date: CalendarDate, month: CalendarDate) -> Element {
    let context = use_context::<CalendarContext>();
    let outside = date.month != month.month || date.year != month.year;
    let modifiers = context.modifiers_of(date, outside);
    let week_numbers = (context.show_week_number)();
    let custom = context.modifiers.read().clone();

    let mut classes = vec![
        "group/day relative aspect-square h-full w-full rounded-(--cell-radius) p-0 text-center select-none [&:last-child[data-selected=true]_button]:rounded-r-(--cell-radius)",
        if week_numbers {
            "[&:nth-child(2)[data-selected=true]_button]:rounded-l-(--cell-radius)"
        } else {
            "[&:first-child[data-selected=true]_button]:rounded-l-(--cell-radius)"
        },
        "rdp-day",
    ];
    let flags = [
        (
            modifiers.outside,
            "text-muted-foreground aria-selected:text-muted-foreground rdp-outside",
        ),
        (
            modifiers.disabled,
            "text-muted-foreground opacity-50 rdp-disabled",
        ),
        (modifiers.hidden, "invisible rdp-hidden"),
        (
            modifiers.today,
            "rounded-(--cell-radius) bg-muted text-foreground data-[selected=true]:rounded-none rdp-today",
        ),
        (modifiers.selected, "rdp-selected"),
        (
            modifiers.range_start,
            "relative isolate z-0 rounded-l-(--cell-radius) bg-muted after:absolute after:inset-y-0 after:right-0 after:w-4 after:bg-muted rdp-range_start",
        ),
        (
            modifiers.range_middle,
            "rounded-none rdp-range_middle",
        ),
        (
            modifiers.range_end,
            "relative isolate z-0 rounded-r-(--cell-radius) bg-muted after:absolute after:inset-y-0 after:left-0 after:w-4 after:bg-muted rdp-range_end",
        ),
    ];
    classes.extend(flags.iter().filter(|(on, _)| *on).map(|(_, class)| *class));
    classes.extend(
        custom
            .iter()
            .filter(|modifier| modifiers.custom.contains(&modifier.name))
            .map(|modifier| modifier.class_name.as_str()),
    );
    let class = classes.join(" ");

    let mut attributes = vec![
        Attribute::new("data-day", date.iso(), None, false),
        Attribute::new(
            "data-month",
            format!("{:04}-{:02}", month.year, month.month),
            None,
            false,
        ),
    ];
    let boolean_flags = [
        ("data-selected", modifiers.selected),
        ("data-disabled", modifiers.disabled),
        ("data-hidden", modifiers.hidden),
        ("data-outside", modifiers.outside),
        ("data-focused", modifiers.focused),
        ("data-today", modifiers.today),
    ];
    for (name, on) in boolean_flags {
        if on {
            attributes.push(Attribute::new(name, "true", None, false));
        }
    }

    if modifiers.hidden {
        return rsx! {
            td { role: "gridcell", class: class, ..attributes }
        };
    }

    let day = CalendarDay { date, outside };
    let children = rsx! {
        "{date.day}"
    };
    let button = match (context.day_button)() {
        Some(render) => render.call(CalendarDayProps {
            day,
            modifiers: modifiers.clone(),
            children,
        }),
        None => rsx! {
            CalendarDayButton { day, modifiers: modifiers.clone(), {children} }
        },
    };

    rsx! {
        td {
            role: "gridcell",
            aria_selected: modifiers.selected.then_some("true"),
            class: class,
            ..attributes,
            {button}
        }
    }
}

/// The button of one day. It selects the day, takes part in keyboard navigation and carries the range
/// and selection state the styles read.
#[component]
pub fn CalendarDayButton(
    day: CalendarDay,
    modifiers: CalendarModifiers,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<CalendarContext>();
    let date = day.date;
    let locale = (context.locale)();
    let rtl = (context.rtl)();
    let mut attributes = attributes;
    let tabbable = !day.outside && (context.focus_target)() == Some(date);
    let class = cn!(
        button_variants(ButtonVariant::Ghost, ButtonSize::Icon, None),
        "cn-calendar-day-button relative isolate z-10 flex aspect-square size-auto w-full min-w-(--cell-size) flex-col gap-1 border-0 leading-none font-normal group-data-[focused=true]/day:relative group-data-[focused=true]/day:z-10 group-data-[focused=true]/day:border-ring group-data-[focused=true]/day:ring-[3px] group-data-[focused=true]/day:ring-ring/50 data-[range-end=true]:rounded-(--cell-radius) data-[range-end=true]:rounded-r-(--cell-radius) data-[range-end=true]:bg-primary data-[range-end=true]:text-primary-foreground data-[range-middle=true]:rounded-none data-[range-middle=true]:bg-muted data-[range-middle=true]:text-foreground data-[range-start=true]:rounded-(--cell-radius) data-[range-start=true]:rounded-l-(--cell-radius) data-[range-start=true]:bg-primary data-[range-start=true]:text-primary-foreground data-[selected-single=true]:bg-primary data-[selected-single=true]:text-primary-foreground dark:hover:text-foreground [&>span]:text-xs [&>span]:opacity-70",
        "rdp-day",
        take_class(&mut attributes).as_deref(),
        "rdp-day_button"
    );
    let selected_single = modifiers.selected
        && !modifiers.range_start
        && !modifiers.range_end
        && !modifiers.range_middle;
    let label = format!(
        "{}{}, {} {}{}, {}{}",
        if modifiers.today { "Today, " } else { "" },
        locale.weekdays[usize::from(date.weekday())],
        locale.months[usize::from(date.month - 1)],
        date.day,
        ordinal(date.day),
        date.year,
        if modifiers.selected { ", selected" } else { "" }
    );

    rsx! {
        button {
            r#type: "button",
            "data-slot": "button",
            "data-day": date.short(),
            "data-selected-single": if selected_single { "true" } else { "false" },
            "data-range-start": if modifiers.range_start { "true" } else { "false" },
            "data-range-end": if modifiers.range_end { "true" } else { "false" },
            "data-range-middle": if modifiers.range_middle { "true" } else { "false" },
            tabindex: if tabbable { "0" } else { "-1" },
            aria_label: label,
            disabled: modifiers.disabled,
            class: class,
            onclick: move |_| context.click_day(date),
            onfocus: move |_| {
                let mut focused = context.focused;
                focused.set(Some(date));
            },
            onblur: move |_| {
                let mut focused = context.focused;
                if *focused.peek() == Some(date) {
                    focused.set(None);
                }
            },
            onkeydown: move |event| {
                let key = event.key();
                let shift = event.modifiers().shift();
                let forward = if rtl { -1 } else { 1 };
                let step: Option<Box<dyn Fn(CalendarDate) -> CalendarDate>> = match key {
                    Key::ArrowLeft => Some(Box::new(move |d| d.add_days(-forward))),
                    Key::ArrowRight => Some(Box::new(move |d| d.add_days(forward))),
                    Key::ArrowUp => Some(Box::new(|d| d.add_days(-7))),
                    Key::ArrowDown => Some(Box::new(|d| d.add_days(7))),
                    Key::PageUp => {
                        let months = if shift { -12 } else { -1 };
                        Some(Box::new(move |d| d.add_months(months)))
                    }
                    Key::PageDown => {
                        let months = if shift { 12 } else { 1 };
                        Some(Box::new(move |d| d.add_months(months)))
                    }
                    Key::Home => {
                        let start = locale.week_starts_on;
                        Some(Box::new(move |d| d.start_of_week(start)))
                    }
                    Key::End => {
                        let start = locale.week_starts_on;
                        Some(Box::new(move |d| d.start_of_week(start).add_days(6)))
                    }
                    _ => None,
                };
                if let Some(step) = step {
                    event.prevent_default();
                    context.move_focus(date, step);
                }
            },
            ..attributes,
            {children}
        }
    }
}
