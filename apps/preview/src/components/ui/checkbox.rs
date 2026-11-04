//! Checkbox: a control that toggles between checked and unchecked.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

fn state(checked: bool) -> &'static str {
    if checked { "checked" } else { "unchecked" }
}

/// A two-state checkbox. Pass `checked` to control it, or leave it out and use `default_checked`.
#[component]
pub fn Checkbox(
    /// Controlled state. When omitted, the checkbox keeps its own state.
    checked: Option<bool>,
    /// Initial state of an uncontrolled checkbox.
    #[props(default)]
    default_checked: bool,
    #[props(default)] disabled: bool,
    on_checked_change: Option<EventHandler<bool>>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border shadow-xs transition-shadow group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer shrink-0 outline-none disabled:pointer-events-none disabled:opacity-50",
        take_class(&mut attributes)
    );
    let mut internal = use_signal(|| default_checked);
    let is_checked = checked.unwrap_or(internal());

    rsx! {
        button {
            "data-slot": "checkbox",
            "data-state": state(is_checked),
            "data-checked": is_checked.then_some("true"),
            "data-disabled": disabled.then_some("true"),
            r#type: "button",
            role: "checkbox",
            aria_checked: is_checked,
            disabled: disabled,
            class: class,
            onclick: move |event| {
                if checked.is_none() {
                    internal.set(!is_checked);
                }
                if let Some(handler) = on_checked_change {
                    handler.call(!is_checked);
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            ..attributes,
            if is_checked {
                span {
                    "data-slot": "checkbox-indicator",
                    class: "[&>svg]:size-3.5 grid place-content-center text-current",
                    svg {
                        "data-slot": "checkbox-icon",
                        class: "pointer-events-none",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M20 6 9 17l-5-5" }
                    }
                }
            }
        }
    }
}
