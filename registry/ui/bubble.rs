//! Bubble: a chat message bubble with variants, alignment and reactions.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

impl BubbleVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-bubble-variant-default",
            Self::Secondary => "cn-bubble-variant-secondary",
            Self::Muted => "cn-bubble-variant-muted",
            Self::Tinted => "cn-bubble-variant-tinted",
            Self::Outline => "cn-bubble-variant-outline",
            Self::Ghost => "cn-bubble-variant-ghost",
            Self::Destructive => "cn-bubble-variant-destructive",
        }
    }

    /// Value of the `data-variant` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Secondary => "secondary",
            Self::Muted => "muted",
            Self::Tinted => "tinted",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Destructive => "destructive",
        }
    }
}

/// Which side of the conversation a bubble sits on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleAlign {
    #[default]
    Start,
    End,
}

impl BubbleAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }

    fn reactions_class(self) -> &'static str {
        match self {
            Self::Start => "cn-bubble-reactions-align-start",
            Self::End => "cn-bubble-reactions-align-end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleReactionsSide {
    Top,
    #[default]
    Bottom,
}

impl BubbleReactionsSide {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Top => "cn-bubble-reactions-side-top",
            Self::Bottom => "cn-bubble-reactions-side-bottom",
        }
    }
}

#[component]
pub fn BubbleGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-bubble-group flex min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "bubble-group", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn Bubble(
    #[props(default)] variant: BubbleVariant,
    #[props(default)] align: BubbleAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-bubble group/bubble relative flex w-fit min-w-0 flex-col",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "bubble",
            "data-variant": variant.as_str(),
            "data-align": align.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

const BUBBLE_CONTENT_BASE: &str = "cn-bubble-content w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors";

/// Bubble content classes for any element, e.g. a custom clickable container.
pub fn bubble_content_variants(class: Option<&str>) -> String {
    cn!(BUBBLE_CONTENT_BASE, class)
}

#[component]
pub fn BubbleContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = bubble_content_variants(take_class(&mut attributes).as_deref());

    rsx! {
        div { "data-slot": "bubble-content", class: class, ..attributes, {children} }
    }
}

/// A bubble body that is a `<button>`, for clickable messages.
#[component]
pub fn BubbleContentButton(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = bubble_content_variants(take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "bubble-content",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// A bubble body that is an `<a>`, for messages that link somewhere.
#[component]
pub fn BubbleContentLink(
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = bubble_content_variants(take_class(&mut attributes).as_deref());

    rsx! {
        a {
            "data-slot": "bubble-content",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn BubbleReactions(
    #[props(default)] side: BubbleReactionsSide,
    #[props(default = BubbleAlign::End)] align: BubbleAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-bubble-reactions absolute z-10 flex w-fit items-center justify-center",
        side.class(),
        align.reactions_class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "bubble-reactions",
            "data-align": align.as_str(),
            "data-side": side.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}
