//! Checkbox: a control that toggles between checked and unchecked, or shows a mixed state.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const CHECKBOX_BASE: &str = "cn-checkbox peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50";

const INDICATOR_BASE: &str =
    "cn-checkbox-indicator grid place-content-center text-current transition-none";

/// A two-state checkbox. Pass `checked` to control it, or leave it out and use `default_checked`.
#[component]
pub fn Checkbox(
    /// Controlled state. When omitted, the checkbox keeps its own state.
    checked: Option<bool>,
    /// Initial state of an uncontrolled checkbox.
    #[props(default)]
    default_checked: bool,
    /// Shows the mixed state. The next click checks the box.
    #[props(default)]
    indeterminate: bool,
    #[props(default)] disabled: bool,
    on_checked_change: Option<EventHandler<bool>>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(CHECKBOX_BASE, take_class(&mut attributes).as_deref());
    let mut internal = use_signal(|| default_checked);
    let is_checked = checked.unwrap_or(internal());
    let aria_checked = if indeterminate {
        "mixed"
    } else if is_checked {
        "true"
    } else {
        "false"
    };

    rsx! {
        button {
            "data-slot": "checkbox",
            "data-checked": (is_checked && !indeterminate).then_some(""),
            "data-unchecked": (!is_checked && !indeterminate).then_some(""),
            "data-indeterminate": indeterminate.then_some(""),
            "data-disabled": disabled.then_some(""),
            r#type: "button",
            role: "checkbox",
            "aria-checked": aria_checked,
            disabled: disabled,
            class: class,
            onclick: move |event| {
                let next = !is_checked;
                if checked.is_none() {
                    internal.set(next);
                }
                if let Some(handler) = on_checked_change {
                    handler.call(next);
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
            if is_checked || indeterminate {
                span {
                    "data-slot": "checkbox-indicator",
                    "data-checked": (is_checked && !indeterminate).then_some(""),
                    "data-indeterminate": indeterminate.then_some(""),
                    class: INDICATOR_BASE,
                    svg {
                        width: "24",
                        height: "24",
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
