//! Separator: a thin line that divides content horizontally or vertically.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    /// Value of the `data-orientation` and `aria-orientation` attributes.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Horizontal => "h-px w-full",
            Self::Vertical => "h-full w-px",
        }
    }
}

/// Classes for a separator element, for components that render their own.
pub fn separator_variants(orientation: SeparatorOrientation, class: Option<&str>) -> String {
    cn!("bg-border shrink-0", orientation.class(), class)
}

#[component]
pub fn Separator(
    #[props(default)] orientation: SeparatorOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = separator_variants(orientation, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "separator",
            role: "separator",
            "data-orientation": orientation.as_str(),
            "aria-orientation": orientation.as_str(),
            class: class,
            ..attributes,
        }
    }
}
