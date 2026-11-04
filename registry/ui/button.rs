//! Button: displays a button, or anything that should look like one.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Secondary,
    Ghost,
    Destructive,
    Link,
}

impl ButtonVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-button-variant-default",
            Self::Outline => "cn-button-variant-outline",
            Self::Secondary => "cn-button-variant-secondary",
            Self::Ghost => "cn-button-variant-ghost",
            Self::Destructive => "cn-button-variant-destructive",
            Self::Link => "cn-button-variant-link",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Xs,
    Sm,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-button-size-default",
            Self::Xs => "cn-button-size-xs",
            Self::Sm => "cn-button-size-sm",
            Self::Lg => "cn-button-size-lg",
            Self::Icon => "cn-button-size-icon",
            Self::IconXs => "cn-button-size-icon-xs",
            Self::IconSm => "cn-button-size-icon-sm",
            Self::IconLg => "cn-button-size-icon-lg",
        }
    }
}

const BUTTON_BASE: &str = "cn-button group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";

/// Button classes for any element, e.g. a link that should look like a button.
pub fn button_variants(variant: ButtonVariant, size: ButtonSize, class: Option<&str>) -> String {
    cn!(BUTTON_BASE, variant.class(), size.class(), class)
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "button",
            class: class,
            onclick: move |event| {
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
