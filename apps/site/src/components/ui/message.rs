//! Message: one turn in a conversation, with an avatar, a header, content and a footer.
//! Parts are `MessageGroup`, `Message` (`align`: start or end), `MessageAvatar`,
//! `MessageContent`, `MessageHeader` and `MessageFooter`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

/// Which side of the conversation a message sits on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageAlign {
    #[default]
    Start,
    End,
}

impl MessageAlign {
    /// Value of the `data-align` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

#[component]
pub fn MessageGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-1.5 flex min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "message-group", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn Message(
    #[props(default)] align: MessageAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-xs/relaxed gap-1.5 group/message relative flex w-full min-w-0 data-[align=end]:flex-row-reverse",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "message",
            "data-align": align.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MessageAvatar(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "min-w-8 group-has-data-[slot=message-footer]/message:-translate-y-8 flex w-fit shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "message-avatar", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn MessageContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 group-data-[align=end]/message:*:data-slot:self-end flex w-full min-w-0 flex-col wrap-break-word",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "message-content", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn MessageHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-[0.625rem] font-medium text-muted-foreground px-2.5 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "message-header", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn MessageFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-[0.625rem] font-medium text-muted-foreground px-2.5 group-has-data-[variant=ghost]/message:px-0 flex max-w-full min-w-0 items-center group-data-[align=end]/message:justify-end",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "message-footer", class: class, ..attributes, {children} }
    }
}
