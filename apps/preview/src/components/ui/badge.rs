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
    fn name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Secondary => "secondary",
            Self::Destructive => "destructive",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Link => "link",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "bg-primary text-primary-foreground [a]:hover:bg-primary/80",
            Self::Secondary => "bg-secondary text-secondary-foreground [a]:hover:bg-secondary/80",
            Self::Destructive => {
                "bg-destructive/10 [a]:hover:bg-destructive/20 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 text-destructive dark:bg-destructive/20"
            }
            Self::Outline => {
                "border-border text-foreground [a]:hover:bg-muted [a]:hover:text-muted-foreground"
            }
            Self::Ghost => "hover:bg-muted hover:text-muted-foreground dark:hover:bg-muted/50",
            Self::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

const BADGE_BASE: &str = "h-5 gap-1 rounded-4xl border border-transparent px-2 py-0.5 text-xs font-medium transition-all has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&>svg]:size-3! group/badge inline-flex w-fit shrink-0 items-center justify-center overflow-hidden whitespace-nowrap focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&>svg]:pointer-events-none";

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
            "data-variant": variant.name(),
            class: class,
            ..attributes,
            {children}
        }
    }
}
