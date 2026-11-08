//! Empty: displays an empty state with optional media, title, description and actions.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

impl EmptyMediaVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "bg-transparent",
            Self::Icon => {
                "bg-muted text-foreground flex size-8 shrink-0 items-center justify-center rounded-md [&_svg:not([class*='size-'])]:size-4"
            }
        }
    }

    /// Value of the `data-variant` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Icon => "icon",
        }
    }
}

#[component]
pub fn Empty(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-4 rounded-xl border-dashed p-6 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "empty",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn EmptyHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-1 flex max-w-sm flex-col items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "empty-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn EmptyMedia(
    #[props(default)] variant: EmptyMediaVariant,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mb-2 flex shrink-0 items-center justify-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "empty-icon",
            "data-variant": variant.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn EmptyTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sm font-medium tracking-tight font-heading",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "empty-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn EmptyDescription(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-xs/relaxed text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "empty-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn EmptyContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 text-xs/relaxed flex w-full max-w-sm min-w-0 flex-col items-center text-balance",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "empty-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}
