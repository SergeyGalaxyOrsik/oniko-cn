//! Queue: a list of pending and completed tasks or messages, grouped into collapsible sections.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn Queue(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-2 rounded-xl border border-border bg-background px-3 pt-2 pb-2 shadow-xs",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "queue",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A group of queue items that can be collapsed under its header.
#[component]
pub fn QueueSection(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default = true)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "queue-section",
            class: class,
            ..attributes,
            Collapsible {
                open: open,
                default_open: default_open,
                on_open_change: on_open_change,
                class: "flex flex-col gap-2",
                {children}
            }
        }
    }
}

/// Header button that expands and collapses the enclosing `QueueSection`.
/// It takes no extra attributes, because `CollapsibleTrigger` cannot receive them.
#[component]
pub fn QueueSectionTrigger(children: Element) -> Element {
    rsx! {
        CollapsibleTrigger {
            "data-slot": "queue-section-trigger",
            class: "group flex w-full items-center justify-between rounded-md bg-muted/40 px-3 py-2 text-left text-sm font-medium text-muted-foreground transition-colors hover:bg-muted",
            {children}
        }
    }
}

/// Label of a section header: a chevron, an optional icon, and the count with its label.
#[component]
pub fn QueueSectionLabel(
    count: Option<usize>,
    label: String,
    icon: Option<Element>,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center gap-2",
        take_class(&mut attributes).as_deref()
    );
    let text = match count {
        Some(count) => format!("{count} {label}"),
        None => label,
    };

    rsx! {
        span {
            "data-slot": "queue-section-label",
            class: class,
            ..attributes,
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "size-4 transition-transform group-data-[state=closed]:-rotate-90",
                "aria-hidden": "true",
                path { d: "m6 9 6 6 6-6" }
            }
            {icon}
            span { {text} }
        }
    }
}

/// Content of a `QueueSection`, shown only while the section is open.
#[component]
pub fn QueueSectionContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));

    rsx! {
        CollapsibleContent {
            class: "flex flex-col",
            div {
                "data-slot": "queue-section-content",
                class: class,
                ..attributes,
                {children}
            }
        }
    }
}

/// Scrollable list of queue items, capped at a fixed height.
#[component]
pub fn QueueList(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mt-2 -mb-1 max-h-40 overflow-y-auto pr-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "queue-list",
            class: class,
            ..attributes,
            ul {
                {children}
            }
        }
    }
}

#[component]
pub fn QueueItem(
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "group flex flex-col gap-1 rounded-md px-3 py-1 text-sm transition-colors hover:bg-muted",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        li {
            "data-slot": "queue-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Dot in front of an item. A completed item gets a muted dot.
#[component]
pub fn QueueItemIndicator(
    #[props(default)] completed: bool,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let tone = if completed {
        "border-muted-foreground/20 bg-muted-foreground/10"
    } else {
        "border-muted-foreground/50"
    };
    let class = cn!(
        "mt-0.5 inline-block size-2.5 rounded-full border",
        tone,
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "queue-item-indicator",
            "data-completed": completed.then_some("true"),
            class: class,
            ..attributes,
        }
    }
}

/// Title of an item. A completed item is struck through.
#[component]
pub fn QueueItemContent(
    #[props(default)] completed: bool,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let tone = if completed {
        "text-muted-foreground/50 line-through"
    } else {
        "text-muted-foreground"
    };
    let class = cn!(
        "line-clamp-1 grow break-words",
        tone,
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "queue-item-content",
            "data-completed": completed.then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Secondary text under an item title, aligned with the title.
#[component]
pub fn QueueItemDescription(
    #[props(default)] completed: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let tone = if completed {
        "text-muted-foreground/40 line-through"
    } else {
        "text-muted-foreground"
    };
    let class = cn!("ml-6 text-xs", tone, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "queue-item-description",
            "data-completed": completed.then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn QueueItemActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("flex gap-1", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "queue-item-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Icon button shown on the right of an item. It appears on row hover.
#[component]
pub fn QueueItemAction(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let extra = cn!(
        "size-auto rounded p-1 text-muted-foreground opacity-0 transition-opacity hover:bg-muted-foreground/10 hover:text-foreground group-hover:opacity-100",
        take_class(&mut attributes).as_deref()
    );
    let class = button_variants(ButtonVariant::Ghost, ButtonSize::Icon, Some(extra.as_str()));

    rsx! {
        button {
            "data-slot": "queue-item-action",
            r#type: "button",
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
pub fn QueueItemAttachment(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mt-1 flex flex-wrap gap-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "queue-item-attachment",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Small square thumbnail of an attached image. Decorative by default, so `alt` is empty.
#[component]
pub fn QueueItemImage(
    #[props(extends = GlobalAttributes, extends = img)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "h-8 w-8 rounded border object-cover",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        img {
            "data-slot": "queue-item-image",
            alt: "",
            class: class,
            ..attributes,
        }
    }
}

/// Chip for an attached file, with a paperclip icon and a truncated name.
#[component]
pub fn QueueItemFile(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center gap-1 rounded border bg-muted px-2 py-1 text-xs",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "queue-item-file",
            class: class,
            ..attributes,
            svg {
                view_box: "0 0 24 24",
                width: "12",
                height: "12",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" }
            }
            span { class: "max-w-[100px] truncate", {children} }
        }
    }
}
