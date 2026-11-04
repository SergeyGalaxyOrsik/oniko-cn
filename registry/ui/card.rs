//! Card: displays a container with a header, content and footer.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardSize {
    #[default]
    Default,
    Sm,
}

impl CardSize {
    /// Value of the `data-size` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

#[component]
pub fn Card(
    #[props(default)] size: CardSize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-card group/card flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "card",
            "data-size": size.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto]",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "card-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-card-title", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "card-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardDescription(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-card-description", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "card-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Places its content in the top right corner of `CardHeader`.
#[component]
pub fn CardAction(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "card-action",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-card-content", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "card-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-card-footer flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "card-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}
