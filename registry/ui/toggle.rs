//! Toggle: a two-state button that is either on (pressed) or off.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

impl ToggleVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-toggle-variant-default",
            Self::Outline => "cn-toggle-variant-outline",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl ToggleSize {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-toggle-size-default",
            Self::Sm => "cn-toggle-size-sm",
            Self::Lg => "cn-toggle-size-lg",
        }
    }
}

const TOGGLE_BASE: &str = "cn-toggle group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none hover:bg-muted focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";

/// Toggle classes for any element, e.g. a toggle-group item that should look like a toggle.
pub fn toggle_variants(variant: ToggleVariant, size: ToggleSize, class: Option<&str>) -> String {
    cn!(TOGGLE_BASE, variant.class(), size.class(), class)
}

/// A two-state button. Pass `pressed` to control it, or leave it out and use `default_pressed`.
#[component]
pub fn Toggle(
    /// Controlled state. When omitted, the toggle keeps its own state.
    pressed: Option<bool>,
    /// Initial state of an uncontrolled toggle.
    #[props(default)]
    default_pressed: bool,
    #[props(default)] variant: ToggleVariant,
    #[props(default)] size: ToggleSize,
    #[props(default)] disabled: bool,
    on_pressed_change: Option<EventHandler<bool>>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = toggle_variants(variant, size, take_class(&mut attributes).as_deref());
    let mut internal = use_signal(|| default_pressed);
    let is_pressed = pressed.unwrap_or(internal());

    rsx! {
        button {
            "data-slot": "toggle",
            "data-pressed": is_pressed.then_some(""),
            "data-disabled": disabled.then_some(""),
            r#type: "button",
            "aria-pressed": if is_pressed { "true" } else { "false" },
            disabled: disabled,
            class: class,
            onclick: move |event| {
                if pressed.is_none() {
                    internal.set(!is_pressed);
                }
                if let Some(handler) = on_pressed_change {
                    handler.call(!is_pressed);
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
            {children}
        }
    }
}
