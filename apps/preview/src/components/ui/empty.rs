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
                "bg-muted text-foreground flex size-10 shrink-0 items-center justify-center rounded-lg [&_svg:not([class*='size-'])]:size-6"
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
        "gap-4 rounded-lg border-dashed p-12 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance",
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
        "gap-2 flex max-w-sm flex-col items-center",
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
        "mb-2 flex shrink-0 items-center justify-center",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "empty-media",
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
        "text-lg font-medium tracking-tight",
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
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("text-sm/relaxed", take_class(&mut attributes).as_deref());

    rsx! {
        p {
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
        "gap-4 text-sm flex w-full max-w-sm min-w-0 flex-col items-center text-balance",
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
