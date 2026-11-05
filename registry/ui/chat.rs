//! Chat: a conversation with an agent, laid out as a scrolling thread above a composer.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

/// The full-height column that holds a conversation.
#[component]
pub fn Chat(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex h-full min-h-0 w-full flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "chat",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The scrolling thread of turns. Announced to assistive tech as a log.
#[component]
pub fn ChatMessages(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "chat-messages",
            role: "log",
            aria_live: "polite",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The area under the thread where the person writes the next turn.
#[component]
pub fn ChatComposer(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "shrink-0 border-t bg-background p-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "chat-composer",
            class: class,
            ..attributes,
            {children}
        }
    }
}
