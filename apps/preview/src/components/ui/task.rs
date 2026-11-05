//! Task: a collapsible step of an AI agent's work, with a title, a list of items and file chips.
//!
//! `Task` holds the open state, `TaskTrigger` shows the title and toggles the task,
//! and `TaskContent` holds the `TaskItem` and `TaskItemFile` parts of the step.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

/// Root of a task. Open by default; the trigger opens and closes it.
#[component]
pub fn Task(
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
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "task",
            class: class,
            ..attributes,
            Collapsible {
                open: open,
                default_open: default_open,
                on_open_change: on_open_change,
                {children}
            }
        }
    }
}

/// The row that shows the task title with a search icon and a chevron. Pass `children` to replace the row.
#[component]
pub fn TaskTrigger(
    title: String,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());
    let label = children.unwrap_or_else(|| {
        rsx! {
            svg {
                "data-slot": "task-trigger-icon",
                class: "size-4 shrink-0",
                "aria-hidden": "true",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "11", cy: "11", r: "8" }
                path { d: "m21 21-4.3-4.3" }
            }
            span { "data-slot": "task-trigger-title", class: "text-sm", {title} }
            svg {
                "data-slot": "task-trigger-chevron",
                class: "size-4 shrink-0 transition-transform group-data-[state=open]:rotate-180",
                "aria-hidden": "true",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    });

    rsx! {
        div {
            "data-slot": "task-trigger",
            class: class,
            ..attributes,
            CollapsibleTrigger {
                class: "group flex w-full cursor-pointer items-center gap-2 text-muted-foreground text-sm transition-colors hover:text-foreground",
                {label}
            }
        }
    }
}

/// The steps of a task. Shown only while the task is open; closed content is unmounted.
#[component]
pub fn TaskContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "task-content",
            class: class,
            ..attributes,
            CollapsibleContent {
                class: "text-popover-foreground outline-none",
                div {
                    "data-slot": "task-content-steps",
                    class: "mt-4 space-y-2 border-muted border-l-2 pl-4",
                    {children}
                }
            }
        }
    }
}

/// One line of text inside a `TaskContent`.
#[component]
pub fn TaskItem(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground text-sm",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "task-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A file chip shown inside a `TaskItem`.
#[component]
pub fn TaskItemFile(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "inline-flex items-center gap-1 rounded-md border bg-secondary px-1.5 py-0.5 text-foreground text-xs",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "task-item-file",
            class: class,
            ..attributes,
            {children}
        }
    }
}
