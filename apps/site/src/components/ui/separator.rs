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
}

const SEPARATOR_BASE: &str = "shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch";

/// Classes for a separator element, for components that render their own.
pub fn separator_variants(class: Option<&str>) -> String {
    cn!(SEPARATOR_BASE, class)
}

#[component]
pub fn Separator(
    #[props(default)] orientation: SeparatorOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = separator_variants(take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "separator",
            role: "separator",
            "data-orientation": orientation.as_str(),
            "aria-orientation": (orientation == SeparatorOrientation::Vertical).then_some("vertical"),
            class: class,
            ..attributes,
        }
    }
}
