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
            Self::Default => "cn-empty-media-default",
            Self::Icon => "cn-empty-media-icon",
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
        "cn-empty flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance",
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
        "cn-empty-header flex max-w-sm flex-col items-center",
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
        "cn-empty-media flex shrink-0 items-center justify-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
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
        "cn-empty-title cn-font-heading",
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
        "cn-empty-description text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
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
        "cn-empty-content flex w-full max-w-sm min-w-0 flex-col items-center text-balance",
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
