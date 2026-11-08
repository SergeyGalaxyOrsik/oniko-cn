//! Input OTP: a row of single-character slots for one-time codes and PIN-like input.
//!
//! Compose an `InputOTP` with one or more `InputOTPGroup`s of `InputOTPSlot`s, optionally split by `InputOTPSeparator`.
//! The slots only draw the value; a single transparent `input` covers the whole component and receives the typing,
//! so the keyboard, IME, paste and one-time-code autofill work the same on web and desktop.
//! As in shadcn/ui, HTML attributes and `class` go to that `input`; use `container_class` for the wrapping element.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

/// Which characters the input accepts. Stands in for the `REGEXP_ONLY_*` patterns of `input-otp`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OtpPattern {
    #[default]
    Any,
    Digits,
    Chars,
    DigitsAndChars,
}

impl OtpPattern {
    pub fn accepts(self, c: char) -> bool {
        match self {
            Self::Any => true,
            Self::Digits => c.is_ascii_digit(),
            Self::Chars => c.is_ascii_alphabetic(),
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

/// The one-time-code field. Pass `value` to control it, or leave it out to let the field keep the text.
#[component]
pub fn InputOTP(
    /// Number of characters the field holds.
    #[props(default = 6)]
    max_length: usize,
    #[props(default)] pattern: OtpPattern,
    /// Controlled value. Update it from `onchange`.
    value: Option<String>,
    /// Initial value of an uncontrolled field.
    #[props(default)]
    default_value: String,
    #[props(default)] disabled: bool,
    /// Value of the `inputmode` attribute. Defaults to "numeric", like `input-otp`.
    #[props(default = "numeric".to_string())]
    inputmode: String,
    /// Classes for the wrapping element. `class` goes to the `input`.
    container_class: Option<String>,
    onchange: Option<EventHandler<String>>,
    /// Called once when the value reaches `max_length` characters.
    oncomplete: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let input_class = cn!(
        "disabled:cursor-not-allowed",
        take_class(&mut attributes).as_deref()
    );
    let container_class = cn!(
        "gap-2 relative flex items-center has-disabled:opacity-50",
        container_class.as_deref()
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

    rsx! {
        div { class: container_class,
            {children}
            // Last in the DOM so it paints above the slots. It is transparent: the slots draw the characters.
            input {
                "data-slot": "input-otp",
                class: cn!(
                    "absolute inset-0 size-full cursor-text opacity-0 outline-none",
                    input_class.as_str()
                ),
                r#type: "text",
                inputmode: inputmode,
                autocomplete: "one-time-code",
                spellcheck: false,
                maxlength: "{max_length}",
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
                    if let Some(handler) = onchange {
                        handler.call(next.clone());
                    }
                    if is_full
                        && !was_full
                        && let Some(handler) = oncomplete
                    {
                        handler.call(next);
                    }
                },
                ..attributes,
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
        "has-aria-invalid:ring-destructive/20 dark:has-aria-invalid:ring-destructive/40 has-aria-invalid:border-destructive rounded-md has-aria-invalid:ring-2 flex items-center",
        take_class(&mut attributes).as_deref()
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
/// Set `"aria-invalid": "true"` to draw it in the destructive color.
#[component]
pub fn InputOTPSlot(
    index: usize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<OtpContext>();
    let mut attributes = attributes;
    let class = cn!(
        "bg-input/20 dark:bg-input/30 border-input data-[active=true]:border-ring data-[active=true]:ring-ring/30 data-[active=true]:aria-invalid:ring-destructive/20 dark:data-[active=true]:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[active=true]:aria-invalid:border-destructive size-7 border-y border-r text-xs/relaxed transition-all outline-none first:rounded-l-md first:border-l last:rounded-r-md data-[active=true]:ring-2 relative flex items-center justify-center data-[active=true]:z-10",
        take_class(&mut attributes).as_deref()
    );

    let text = (ctx.value)();
    let character = text.chars().nth(index);
    let active = (ctx.focused)()
        && !(ctx.disabled)()
        && index == active_index(text.chars().count(), (ctx.max_length)());
    let has_fake_caret = active && character.is_none();

    rsx! {
        div {
            "data-slot": "input-otp-slot",
            "data-active": if active { "true" } else { "false" },
            class: class,
            ..attributes,
            {character.map(|c| c.to_string())}
            if has_fake_caret {
                div { class: "pointer-events-none absolute inset-0 flex items-center justify-center",
                    div { class: "animate-caret-blink bg-foreground h-4 w-px duration-1000" }
                }
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
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "input-otp-separator",
            class: class,
            role: "separator",
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
