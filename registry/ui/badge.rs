//! Badge: displays a small status label, or anything that should look like one.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

impl BadgeVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-badge-variant-default",
            Self::Secondary => "cn-badge-variant-secondary",
            Self::Destructive => "cn-badge-variant-destructive",
            Self::Outline => "cn-badge-variant-outline",
            Self::Ghost => "cn-badge-variant-ghost",
            Self::Link => "cn-badge-variant-link",
        }
    }
}

const BADGE_BASE: &str = "cn-badge group/badge inline-flex w-fit shrink-0 items-center justify-center overflow-hidden whitespace-nowrap focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&>svg]:pointer-events-none";

/// Badge classes for any element, e.g. a link that should look like a badge.
pub fn badge_variants(variant: BadgeVariant, class: Option<&str>) -> String {
    cn!(BADGE_BASE, variant.class(), class)
}

#[component]
pub fn Badge(
    #[props(default)] variant: BadgeVariant,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = badge_variants(variant, take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "badge",
            class: class,
            ..attributes,
            {children}
        }
    }
}
