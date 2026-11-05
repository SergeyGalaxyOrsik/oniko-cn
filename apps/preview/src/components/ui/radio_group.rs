//! RadioGroup: a set of options where exactly one can be checked.
//!
//! Each item is a native button with `role="radio"`, so `Space` and `Enter` check it. Arrow keys move
//! focus to the next or previous enabled item and check it. Only the checked item is a tab stop; while
//! nothing is checked, every item is one. Give an item an `id` and point a `<label for>` at it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn move_focus(group_id: usize, step: i32) {
    spawn(async move {
        let script = format!(
            r#"
            const group = document.getElementById('radio-group-{group_id}');
            const radios = group ? [...group.querySelectorAll('[role="radio"]:not(:disabled)')] : [];
            if (radios.length) {{
                const current = radios.indexOf(document.activeElement);
                const next = radios[(current + {step} + radios.length) % radios.length];
                next.focus();
                next.click();
            }}
            "#
        );
        let _ = document::eval(&script).await;
    });
}

#[derive(Clone, Copy)]
struct RadioGroupContext {
    id: usize,
    internal: Signal<String>,
    controlled: Signal<Option<String>>,
    disabled: Signal<bool>,
    on_value_change: Option<EventHandler<String>>,
}

impl RadioGroupContext {
    fn current(&self) -> String {
        self.controlled
            .read()
            .clone()
            .unwrap_or_else(|| self.internal.read().clone())
    }

    fn select(&self, value: &str) {
        if self.current() == value {
            return;
        }
        if self.controlled.peek().is_none() {
            let mut internal = self.internal;
            internal.set(value.to_string());
        }
        if let Some(handler) = self.on_value_change {
            handler.call(value.to_string());
        }
    }
}

/// Owns the selected value. Pass `value` to control it, or leave it out and use `default_value`.
#[component]
pub fn RadioGroup(
    /// Controlled value. When set, `on_value_change` is the only way to change it.
    value: Option<String>,
    /// Value selected at first render when `value` is not controlled.
    #[props(default)]
    default_value: String,
    on_value_change: Option<EventHandler<String>>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_value);
    let controlled = use_prop(value);
    let disabled = use_prop(disabled);
    use_context_provider(|| RadioGroupContext {
        id,
        internal,
        controlled,
        disabled,
        on_value_change,
    });

    let mut attributes = attributes;
    let class = cn!(
        "grid gap-3 gap-2 outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            id: "radio-group-{id}",
            "data-slot": "radio-group",
            "data-disabled": disabled().then_some("true"),
            role: "radiogroup",
            aria_disabled: disabled(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One option of a `RadioGroup`. It is checked when `value` equals the group's value.
#[component]
pub fn RadioGroupItem(
    value: String,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let group = use_context::<RadioGroupContext>();
    let mut attributes = attributes;
    let class = cn!(
        "border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary relative aspect-square shrink-0 border outline-none disabled:pointer-events-none disabled:opacity-50",
        take_class(&mut attributes).as_deref()
    );

    let current = group.current();
    let checked = current == value;
    let disabled = disabled || (group.disabled)();
    let tabindex = if checked || current.is_empty() {
        "0"
    } else {
        "-1"
    };

    rsx! {
        button {
            "data-slot": "radio-group-item",
            "data-state": if checked { "checked" } else { "unchecked" },
            "data-checked": checked.then_some("true"),
            "data-disabled": disabled.then_some("true"),
            r#type: "button",
            role: "radio",
            aria_checked: checked,
            tabindex: tabindex,
            disabled: disabled,
            class: class,
            onclick: move |event| {
                if disabled {
                    return;
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                group.select(&value);
            },
            onkeydown: move |event| {
                let step = match event.key() {
                    Key::ArrowDown | Key::ArrowRight => 1,
                    Key::ArrowUp | Key::ArrowLeft => -1,
                    _ => 0,
                };
                if step != 0 {
                    event.prevent_default();
                    move_focus(group.id, step);
                }
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            ..attributes,
            if checked {
                span {
                    "data-slot": "radio-group-indicator",
                    class: "flex size-4 items-center justify-center",
                    span {
                        "data-slot": "radio-group-indicator-icon",
                        class: "bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full",
                    }
                }
            }
        }
    }
}
