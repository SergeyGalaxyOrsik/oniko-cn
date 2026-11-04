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
            Self::Default => "bg-primary text-primary-foreground hover:bg-primary/80",
            Self::Outline => {
                "border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground shadow-xs"
            }
            Self::Secondary => {
                "bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground"
            }
            Self::Ghost => {
                "hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground"
            }
            Self::Destructive => {
                "bg-destructive/10 hover:bg-destructive/20 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/20 text-destructive focus-visible:border-destructive/40 dark:hover:bg-destructive/30"
            }
            Self::Link => "text-primary underline-offset-4 hover:underline",
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
            Self::Default => {
                "h-9 gap-1.5 px-2.5 in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
            }
            Self::Xs => {
                "h-6 gap-1 rounded-[min(var(--radius-md),8px)] px-2 text-xs in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3"
            }
            Self::Sm => {
                "h-8 gap-1 rounded-[min(var(--radius-md),10px)] px-2.5 in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5"
            }
            Self::Lg => {
                "h-10 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
            }
            Self::Icon => "size-9",
            Self::IconXs => {
                "size-6 rounded-[min(var(--radius-md),8px)] in-data-[slot=button-group]:rounded-md [&_svg:not([class*='size-'])]:size-3"
            }
            Self::IconSm => {
                "size-8 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-md"
            }
            Self::IconLg => "size-10",
        }
    }
}

const BUTTON_BASE: &str = "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-md border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";

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
