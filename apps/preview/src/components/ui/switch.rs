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

const SWITCH_BASE: &str = "data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent shadow-xs focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50";

const THUMB_BASE: &str = "bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform";

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
    let class = cn!(SWITCH_BASE, take_class(&mut attributes).as_deref());
    let mut internal = use_signal(|| default_checked);
    let is_checked = checked.unwrap_or(internal());

    rsx! {
        button {
            "data-slot": "switch",
            "data-size": size.data_size(),
            "data-checked": is_checked.then_some(""),
            "data-unchecked": (!is_checked).then_some(""),
            "data-disabled": disabled.then_some(""),
            r#type: "button",
            role: "switch",
            "aria-checked": if is_checked { "true" } else { "false" },
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
                "data-checked": is_checked.then_some(""),
                "data-unchecked": (!is_checked).then_some(""),
                "data-disabled": disabled.then_some(""),
                class: THUMB_BASE,
            }
        }
    }
}
