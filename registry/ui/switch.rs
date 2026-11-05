//! Switch: a control that toggles a setting on and off.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwitchSize {
    #[default]
    Default,
    Sm,
}

impl SwitchSize {
    pub fn data_size(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

fn state(checked: bool) -> &'static str {
    if checked { "checked" } else { "unchecked" }
}

/// A two-state switch. Pass `checked` to control it, or leave it out and use `default_checked`.
#[component]
pub fn Switch(
    /// Controlled state. When omitted, the switch keeps its own state.
    checked: Option<bool>,
    /// Initial state of an uncontrolled switch.
    #[props(default)]
    default_checked: bool,
    #[props(default)] size: SwitchSize,
    #[props(default)] disabled: bool,
    on_checked_change: Option<EventHandler<bool>>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-switch group/switch peer inline-flex items-center outline-none disabled:pointer-events-none disabled:opacity-50",
        take_class(&mut attributes).as_deref()
    );
    let mut internal = use_signal(|| default_checked);
    let is_checked = checked.unwrap_or(internal());

    rsx! {
        button {
            "data-slot": "switch",
            "data-size": size.data_size(),
            "data-state": state(is_checked),
            "data-checked": is_checked.then_some(""),
            "data-unchecked": (!is_checked).then_some(""),
            "data-disabled": disabled.then_some("true"),
            r#type: "button",
            role: "switch",
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
            span {
                "data-slot": "switch-thumb",
                "data-state": state(is_checked),
                "data-checked": is_checked.then_some(""),
                "data-unchecked": (!is_checked).then_some(""),
                class: "cn-switch-thumb pointer-events-none block",
            }
        }
    }
}
