//! Input OTP: a row of single-character slots for one-time codes and PIN-like input.
//!
//! Compose an `InputOTP` with one or more `InputOTPGroup`s of `InputOTPSlot`s, optionally split by `InputOTPSeparator`.
//! The slots only draw the value; a single transparent `input` covers the whole component and receives the typing,
//! so the keyboard, IME, paste and one-time-code autofill work the same on web and desktop.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

/// Which characters the input accepts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OtpPattern {
    #[default]
    Any,
    Digits,
    DigitsAndChars,
}

impl OtpPattern {
    pub fn accepts(self, c: char) -> bool {
        match self {
            Self::Any => true,
            Self::Digits => c.is_ascii_digit(),
            Self::DigitsAndChars => c.is_ascii_alphanumeric(),
        }
    }
}

#[derive(Clone, Copy)]
struct OtpContext {
    value: Signal<String>,
    focused: Signal<bool>,
    max_length: Signal<usize>,
    disabled: Signal<bool>,
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// The slot that shows the caret while the input is focused: the next empty one, or the last one when full.
fn active_index(len: usize, max_length: usize) -> usize {
    len.min(max_length.saturating_sub(1))
}

/// The one-time-code field. Pass `value` to control it, or leave it out and use `default_value`.
#[component]
pub fn InputOTP(
    /// Number of characters the field holds.
    #[props(default = 6)]
    max_length: usize,
    #[props(default)] pattern: OtpPattern,
    /// Controlled value. Update it from `on_value_change`.
    value: Option<String>,
    /// Initial value of an uncontrolled field.
    #[props(default)]
    default_value: String,
    #[props(default)] disabled: bool,
    /// Name of the underlying input, so the code is included in a form submission.
    name: Option<String>,
    /// Accessible name of the input. Defaults to "One-time code".
    aria_label: Option<String>,
    on_value_change: Option<EventHandler<String>>,
    /// Called once when the value reaches `max_length` characters.
    on_complete: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 relative flex items-center has-disabled:opacity-50",
        take_class(&mut attributes)
    );

    let mut current = use_signal(|| value.clone().unwrap_or(default_value));
    if let Some(controlled) = &value
        && *current.peek() != *controlled
    {
        current.set(controlled.clone());
    }
    let controlled = value.is_some();
    let mut focused = use_signal(|| false);
    let limit = use_prop(max_length);
    let disabled_signal = use_prop(disabled);
    use_context_provider(|| OtpContext {
        value: current,
        focused,
        max_length: limit,
        disabled: disabled_signal,
    });

    let inputmode = if pattern == OtpPattern::Digits {
        "numeric"
    } else {
        "text"
    };

    rsx! {
        div {
            "data-slot": "input-otp",
            "data-disabled": disabled.then_some("true"),
            class: class,
            ..attributes,
            {children}
            // Last in the DOM so it paints above the slots. It is transparent: the slots draw the characters.
            input {
                "data-slot": "input-otp-input",
                class: "absolute inset-0 size-full cursor-text opacity-0 outline-none disabled:cursor-not-allowed",
                r#type: "text",
                name: name,
                inputmode: inputmode,
                autocomplete: "one-time-code",
                maxlength: "{max_length}",
                "aria-label": aria_label.unwrap_or_else(|| "One-time code".to_string()),
                value: current(),
                disabled: disabled,
                onfocus: move |_| focused.set(true),
                onblur: move |_| focused.set(false),
                onkeydown: move |event: KeyboardEvent| {
                    if let Key::Character(text) = event.key()
                        && !text.chars().all(|c| pattern.accepts(c))
                    {
                        event.prevent_default();
                    }
                },
                oninput: move |event: FormEvent| {
                    let limit = limit();
                    let next: String = event
                        .value()
                        .chars()
                        .filter(|c| pattern.accepts(*c))
                        .take(limit)
                        .collect();
                    if next == *current.peek() {
                        return;
                    }
                    let was_full = current.peek().chars().count() >= limit;
                    let is_full = next.chars().count() >= limit;
                    if !controlled {
                        current.set(next.clone());
                    }
                    if let Some(handler) = on_value_change {
                        handler.call(next.clone());
                    }
                    if is_full
                        && !was_full
                        && let Some(handler) = on_complete
                    {
                        handler.call(next);
                    }
                },
            }
        }
    }
}

#[component]
pub fn InputOTPGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "has-aria-invalid:ring-destructive/20 dark:has-aria-invalid:ring-destructive/40 has-aria-invalid:border-destructive rounded-md has-aria-invalid:ring-3 flex items-center",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "input-otp-group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One character cell. `index` is its position in the whole field, starting at 0.
#[component]
pub fn InputOTPSlot(
    index: usize,
    /// Marks the slot as invalid (`aria-invalid`), which the styles draw in the destructive color.
    #[props(default)]
    invalid: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<OtpContext>();
    let mut attributes = attributes;
    let class = cn!(
        "dark:bg-input/30 border-input data-[active=true]:border-ring data-[active=true]:ring-ring/50 data-[active=true]:aria-invalid:ring-destructive/20 dark:data-[active=true]:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[active=true]:aria-invalid:border-destructive size-9 border-y border-r text-sm shadow-xs transition-all outline-none first:rounded-l-md first:border-l last:rounded-r-md data-[active=true]:ring-3 flex items-center justify-center",
        take_class(&mut attributes)
    );

    let text = (ctx.value)();
    let character = text.chars().nth(index);
    let disabled = (ctx.disabled)();
    let active = (ctx.focused)()
        && !disabled
        && index == active_index(text.chars().count(), (ctx.max_length)());

    rsx! {
        div {
            "data-slot": "input-otp-slot",
            "data-active": active.then_some("true"),
            "data-disabled": disabled.then_some("true"),
            "aria-invalid": invalid.then_some("true"),
            class: class,
            ..attributes,
            {character.map(|c| c.to_string())}
            if active && character.is_none() {
                div { "data-slot": "input-otp-caret", class: "animate-caret-blink bg-foreground h-4 w-px duration-1000" }
            }
        }
    }
}

/// A visual divider between two groups.
#[component]
pub fn InputOTPSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "[&_svg:not([class*='size-'])]:size-4 flex items-center",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "input-otp-separator",
            role: "separator",
            class: class,
            ..attributes,
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M5 12h14" }
            }
        }
    }
}
