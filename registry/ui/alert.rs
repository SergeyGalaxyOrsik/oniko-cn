//! Alert: displays a callout for user attention.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

impl AlertVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-alert-variant-default",
            Self::Destructive => "cn-alert-variant-destructive",
        }
    }
}

#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-alert group/alert relative w-full",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "alert",
            role: "alert",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AlertTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-alert-title [&_a]:underline [&_a]:underline-offset-3 [&_a]:hover:text-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "alert-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AlertDescription(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-alert-description [&_a]:underline [&_a]:underline-offset-3 [&_a]:hover:text-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "alert-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AlertAction(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-alert-action", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "alert-action",
            class: class,
            ..attributes,
            {children}
        }
    }
}
