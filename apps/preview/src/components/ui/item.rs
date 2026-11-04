//! Item: a flex row for media, text and actions, with optional header and footer, grouped into lists.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::separator::{SeparatorOrientation, separator_variants};

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
            Self::Default => "border-transparent",
            Self::Outline => "border-border",
            Self::Muted => "bg-muted/50 border-transparent",
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
            Self::Default => "gap-3.5 px-4 py-3.5",
            Self::Sm => "gap-2.5 px-3 py-2.5",
            Self::Xs => "gap-2 px-2.5 py-2 in-data-[slot=dropdown-menu-content]:p-0",
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
            Self::Default => "bg-transparent",
            Self::Icon => "[&_svg:not([class*='size-'])]:size-4",
            Self::Image => {
                "size-10 overflow-hidden rounded-sm group-data-[size=sm]/item:size-8 group-data-[size=xs]/item:size-6 [&_img]:size-full [&_img]:object-cover"
            }
        }
    }
}

/// Classes for an element that should look like an [`Item`], such as a link.
pub fn item_variants(variant: ItemVariant, size: ItemSize, class: Option<&str>) -> String {
    cn!(
        "[a]:hover:bg-muted rounded-md border text-sm group/item flex flex-wrap items-center",
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
        "gap-4 has-data-[size=sm]:gap-2.5 has-data-[size=xs]:gap-2 group/item-group flex flex-col",
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
        separator_variants(SeparatorOrientation::Horizontal, None),
        "my-2",
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
        "gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start group/item-media flex shrink-0 items-center justify-center",
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
        "gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col",
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
        "gap-2 text-sm leading-snug font-medium underline-offset-4 flex w-fit items-center",
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
        "text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 text-balance",
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
        "gap-2 flex items-center",
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
        "gap-2 flex basis-full items-center justify-between",
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
        "gap-2 flex basis-full items-center justify-between",
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
