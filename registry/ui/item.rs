//! Item: a flex row for media, text and actions, with optional header and footer, grouped into lists.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::separator::separator_variants;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    /// Value of the `data-variant` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
            Self::Muted => "muted",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-item-variant-default",
            Self::Outline => "cn-item-variant-outline",
            Self::Muted => "cn-item-variant-muted",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
    Xs,
}

impl ItemSize {
    /// Value of the `data-size` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Xs => "xs",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-item-size-default",
            Self::Sm => "cn-item-size-sm",
            Self::Xs => "cn-item-size-xs",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    /// Value of the `data-variant` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Icon => "icon",
            Self::Image => "image",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-item-media-variant-default",
            Self::Icon => "cn-item-media-variant-icon",
            Self::Image => "cn-item-media-variant-image",
        }
    }
}

/// Classes for an element that should look like an [`Item`], such as a link.
pub fn item_variants(variant: ItemVariant, size: ItemSize, class: Option<&str>) -> String {
    cn!(
        "cn-item group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors",
        variant.class(),
        size.class(),
        class
    )
}

#[component]
pub fn Item(
    #[props(default)] variant: ItemVariant,
    #[props(default)] size: ItemSize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = item_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "item",
            "data-variant": variant.as_str(),
            "data-size": size.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ItemGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-item-group group/item-group flex w-full flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "item-group",
            role: "list",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ItemSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        separator_variants(None),
        "cn-item-separator",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "item-separator",
            role: "separator",
            "data-orientation": "horizontal",
            "aria-orientation": "horizontal",
            class: class,
            ..attributes,
        }
    }
}

#[component]
pub fn ItemMedia(
    #[props(default)] variant: ItemMediaVariant,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-item-media flex shrink-0 items-center justify-center [&_svg]:pointer-events-none",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "item-media",
            "data-variant": variant.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ItemContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-item-content flex flex-1 flex-col [&+[data-slot=item-content]]:flex-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "item-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ItemTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-item-title line-clamp-1 flex w-fit items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "item-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ItemDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-item-description line-clamp-2 font-normal [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "item-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ItemActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-item-actions flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "item-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ItemHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-item-header flex basis-full items-center justify-between",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "item-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ItemFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-item-footer flex basis-full items-center justify-between",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "item-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}
