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
            Self::Default => "",
            Self::Border => "border-b border-border pb-2",
            Self::Separator => {
                "before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
            }
        }
    }
}

/// Marker classes for any element, e.g. a link that should look like a marker.
pub fn marker_variants(variant: MarkerVariant, class: Option<&str>) -> String {
    cn!(
        "gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center",
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
        "size-4 [&_svg:not([class*='size-'])]:size-4 shrink-0",
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
        "group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word",
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
