//! RadioGroup: a set of options where exactly one can be checked.
//!
//! Each item is a native button with `role="radio"`, so `Space` and `Enter` check it. Arrow keys move
//! focus to the next or previous enabled item and check it, wrapping at both ends. Only the checked item
//! is a tab stop; while nothing is checked, the first enabled item is. Give an item an `id` and point a
//! `<label for>` at it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const GROUP_BASE: &str = "grid gap-3 w-full";

const ITEM_BASE: &str = "border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50";

const INDICATOR_BASE: &str = "flex size-4 items-center justify-center";

const INDICATOR_ICON_BASE: &str = "bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full";

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn move_focus(group_id: usize, key: &'static str) {
    spawn(async move {
        let script = format!(
            r#"
            const group = document.getElementById('radio-group-{group_id}');
            const radios = group ? [...group.querySelectorAll('[role="radio"]:not(:disabled)')] : [];
            if (radios.length) {{
                const rtl = getComputedStyle(group).direction === 'rtl';
                const forward = ['ArrowDown', rtl ? 'ArrowLeft' : 'ArrowRight'].includes('{key}');
                const current = radios.indexOf(document.activeElement);
                const step = forward ? 1 : -1;
                const next = radios[(current + step + radios.length) % radios.length];
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
    internal: Signal<Option<String>>,
    controlled: Signal<Option<String>>,
    disabled: Signal<bool>,
    items: Signal<Vec<(String, bool)>>,
    on_value_change: Option<EventHandler<String>>,
}

impl RadioGroupContext {
    fn current(&self) -> Option<String> {
        self.controlled
            .read()
            .clone()
            .or_else(|| self.internal.read().clone())
    }

    fn select(&self, value: &str) {
        if self.current().as_deref() == Some(value) {
            return;
        }
        if self.controlled.peek().is_none() {
            let mut internal = self.internal;
            internal.set(Some(value.to_string()));
        }
        if let Some(handler) = self.on_value_change {
            handler.call(value.to_string());
        }
    }

    fn is_tab_stop(&self, value: &str) -> bool {
        let items = self.items.read();
        let checked_is_enabled = self.current().is_some_and(|current| {
            items
                .iter()
                .any(|(item, disabled)| *item == current && !disabled)
        });
        if checked_is_enabled {
            return self.current().as_deref() == Some(value);
        }
        items
            .iter()
            .find(|(_, disabled)| !disabled)
            .is_some_and(|(first, _)| first == value)
    }
}

/// Owns the selected value. Pass `value` to control it, or leave it out and use `default_value`.
#[component]
pub fn RadioGroup(
    /// Controlled value. When set, `on_value_change` is the only way to change it.
    value: Option<String>,
    /// Value selected at first render when `value` is not controlled.
    default_value: Option<String>,
    on_value_change: Option<EventHandler<String>>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_value);
    let controlled = use_prop(value);
    let disabled = use_prop(disabled);
    let items = use_signal(Vec::new);
    use_context_provider(|| RadioGroupContext {
        id,
        internal,
        controlled,
        disabled,
        items,
        on_value_change,
    });

    let mut attributes = attributes;
    let class = cn!(GROUP_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            id: "radio-group-{id}",
            "data-slot": "radio-group",
            "data-disabled": disabled().then_some(""),
            role: "radiogroup",
            "aria-disabled": disabled().then_some("true"),
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
    let disabled = disabled || (group.disabled)();

    let mut items = group.items;
    let registered = value.clone();
    use_effect(move || {
        let mut list = items.write();
        match list.iter_mut().find(|(item, _)| *item == registered) {
            Some(entry) => entry.1 = disabled,
            None => list.push((registered.clone(), disabled)),
        }
    });
    let leaving = value.clone();
    use_drop(move || items.write().retain(|(item, _)| *item != leaving));

    let mut attributes = attributes;
    let class = cn!(ITEM_BASE, take_class(&mut attributes).as_deref());
    let checked = group.current().as_deref() == Some(value.as_str());

    rsx! {
        button {
            "data-slot": "radio-group-item",
            "data-checked": checked.then_some(""),
            "data-unchecked": (!checked).then_some(""),
            "data-disabled": disabled.then_some(""),
            r#type: "button",
            role: "radio",
            "aria-checked": if checked { "true" } else { "false" },
            tabindex: if group.is_tab_stop(&value) { "0" } else { "-1" },
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
                let key = match event.key() {
                    Key::ArrowDown => Some("ArrowDown"),
                    Key::ArrowRight => Some("ArrowRight"),
                    Key::ArrowUp => Some("ArrowUp"),
                    Key::ArrowLeft => Some("ArrowLeft"),
                    _ => None,
                };
                if let Some(key) = key {
                    event.prevent_default();
                    move_focus(group.id, key);
                }
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            ..attributes,
            if checked {
                span {
                    "data-slot": "radio-group-indicator",
                    "data-checked": "",
                    class: INDICATOR_BASE,
                    span { class: INDICATOR_ICON_BASE }
                }
            }
        }
    }
}
