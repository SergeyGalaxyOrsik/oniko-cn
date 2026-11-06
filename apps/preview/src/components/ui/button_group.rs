//! Button group: groups related buttons into one joined control, with optional text and separators.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::separator::{SeparatorOrientation, separator_variants};

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
                "[&>[data-slot]:not(:has(~[data-slot]))]:rounded-r-md! *:data-slot:rounded-r-none [&>[data-slot]~[data-slot]]:rounded-l-none [&>[data-slot]~[data-slot]]:border-l-0"
            }
            Self::Vertical => {
                "[&>[data-slot]:not(:has(~[data-slot]))]:rounded-b-md! flex-col *:data-slot:rounded-b-none [&>[data-slot]~[data-slot]]:rounded-t-none [&>[data-slot]~[data-slot]]:border-t-0"
            }
        }
    }
}

const BUTTON_GROUP_BASE: &str = "has-[>[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-r-md flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1";

/// Button group classes for any element.
pub fn button_group_variants(orientation: ButtonGroupOrientation, class: Option<&str>) -> String {
    cn!(BUTTON_GROUP_BASE, orientation.class(), class)
}

/// `orientation` is optional because upstream only sets `data-orientation` when it is passed.
#[component]
pub fn ButtonGroup(
    orientation: Option<ButtonGroupOrientation>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = button_group_variants(
        orientation.unwrap_or_default(),
        take_class(&mut attributes).as_deref(),
    );

    rsx! {
        div {
            role: "group",
            "data-slot": "button-group",
            "data-orientation": orientation.map(ButtonGroupOrientation::name),
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
        "bg-muted gap-2 rounded-md border px-2.5 text-sm font-medium shadow-xs [&_svg:not([class*='size-'])]:size-4 flex items-center [&_svg]:pointer-events-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            class: class,
            "data-slot": "button-group-text",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ButtonGroupSeparator(
    #[props(default = SeparatorOrientation::Vertical)] orientation: SeparatorOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = separator_variants(Some(&cn!(
        "bg-input relative self-stretch data-horizontal:mx-px data-horizontal:w-auto data-vertical:my-px data-vertical:h-auto",
        take_class(&mut attributes).as_deref()
    )));

    rsx! {
        div {
            "data-slot": "button-group-separator",
            role: "separator",
            "data-orientation": orientation.as_str(),
            "aria-orientation": (orientation == SeparatorOrientation::Vertical).then_some("vertical"),
            class: class,
            ..attributes,
        }
    }
}
