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
        "cn-checkbox peer shrink-0 outline-none disabled:pointer-events-none disabled:opacity-50",
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
                    class: "cn-checkbox-indicator grid place-content-center text-current",
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
