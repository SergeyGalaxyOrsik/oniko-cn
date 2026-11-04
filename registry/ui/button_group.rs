//! Button group: groups related buttons into one joined control, with optional text and separators.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Horizontal => {
                "cn-button-group-orientation-horizontal flex-row [&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none"
            }
            Self::Vertical => {
                "cn-button-group-orientation-vertical flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none"
            }
        }
    }
}

#[component]
pub fn ButtonGroup(
    #[props(default)] orientation: ButtonGroupOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-button-group group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10",
        orientation.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "button-group",
            role: "group",
            "data-orientation": orientation.name(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ButtonGroupText(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-button-group-text flex items-center justify-center whitespace-nowrap [&_svg]:pointer-events-none [&_svg]:shrink-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "button-group-text",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ButtonGroupSeparator(
    #[props(default = ButtonGroupOrientation::Vertical)] orientation: ButtonGroupOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-button-group-separator relative m-0! self-stretch data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-auto data-[orientation=vertical]:h-auto data-[orientation=vertical]:w-px",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "button-group-separator",
            role: "separator",
            "aria-orientation": orientation.name(),
            "data-orientation": orientation.name(),
            class: class,
            ..attributes,
        }
    }
}
