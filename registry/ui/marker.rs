//! Marker: a compact inline status, note or action row, with an optional icon, border or centered separator label.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MarkerVariant {
    #[default]
    Default,
    Border,
    Separator,
}

impl MarkerVariant {
    /// Value of the `data-variant` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Border => "border",
            Self::Separator => "separator",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-marker-variant-default",
            Self::Border => "cn-marker-variant-border",
            Self::Separator => "cn-marker-variant-separator",
        }
    }
}

/// Marker classes for any element, e.g. a link that should look like a marker.
pub fn marker_variants(variant: MarkerVariant, class: Option<&str>) -> String {
    cn!(
        "cn-marker group/marker relative flex w-full items-center",
        variant.class(),
        class
    )
}

#[component]
pub fn Marker(
    #[props(default)] variant: MarkerVariant,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = marker_variants(variant, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "marker",
            "data-variant": variant.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MarkerIcon(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-marker-icon shrink-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "marker-icon",
            "aria-hidden": "true",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MarkerContent(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-marker-content min-w-0 wrap-break-word",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "marker-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}
