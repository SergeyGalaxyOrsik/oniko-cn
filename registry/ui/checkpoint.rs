//! Checkpoint: a marker after a message in a conversation, with a restore action that can be returned to.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::separator::Separator;

/// A horizontal row that marks a point in the conversation, drawn as a line with content before it.
#[component]
pub fn Checkpoint(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center gap-0.5 overflow-hidden text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "checkpoint",
            class: class,
            ..attributes,
            {children}
            Separator {
                class: "flex-1 w-auto",
            }
        }
    }
}

/// The bookmark icon shown by default. Pass children to replace it.
#[component]
pub fn CheckpointIcon(
    #[props(extends = GlobalAttributes, extends = svg)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("size-4 shrink-0", take_class(&mut attributes).as_deref());

    if let Some(children) = children {
        return rsx! {
            span {
                "data-slot": "checkpoint-icon",
                class: class,
                ..attributes,
                {children}
            }
        };
    }

    rsx! {
        svg {
            "data-slot": "checkpoint-icon",
            class: class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            ..attributes,
            path { d: "m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" }
        }
    }
}

/// A small ghost button for the checkpoint row, e.g. "Restore". `tooltip` is shown on hover.
#[component]
pub fn CheckpointTrigger(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::Sm)] size: ButtonSize,
    tooltip: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "checkpoint-trigger",
            r#type: "button",
            title: tooltip.as_deref(),
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
