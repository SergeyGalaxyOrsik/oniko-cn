//! Subagent: a trace line for work the agent handed to a subagent. Its header shows the title and status,
//! and expands to show the task, the subagent's own steps, the tools and model it used, and its result.
//!
//! `Subagent` holds the open state, `SubagentHeader` is the clickable line, and `SubagentContent` holds
//! the details. `SubagentTask` and `SubagentTool` fill the details.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::badge::{Badge, BadgeVariant};
use super::trace::{
    TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceDescription,
    TraceIcon, TraceLine, TraceStatus, TraceTitle,
};

fn robot_icon() -> Element {
    rsx! {
        svg {
            "data-slot": "subagent-icon",
            class: "size-4 shrink-0",
            "aria-hidden": "true",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 8V4H8" }
            rect { width: "16", height: "12", x: "4", y: "8", rx: "2" }
            path { d: "M2 14h2" }
            path { d: "M20 14h2" }
            path { d: "M15 13v2" }
            path { d: "M9 13v2" }
        }
    }
}

/// Root of a subagent line. Closed by default; the header opens and closes it.
#[component]
pub fn Subagent(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex w-full flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "subagent",
            class: class,
            ..attributes,
            TraceCollapsible {
                open: open,
                default_open: default_open,
                on_open_change: on_open_change,
                {children}
            }
        }
    }
}

/// The clickable line of a subagent. `icon` defaults to a robot; `description` is optional,
/// for example a duration.
#[component]
pub fn SubagentHeader(
    #[props(default)] status: TraceStatus,
    icon: Option<Element>,
    title: Element,
    description: Option<Element>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());
    let icon = icon.unwrap_or_else(robot_icon);

    rsx! {
        div {
            "data-slot": "subagent-header",
            class: class,
            ..attributes,
            TraceCollapsibleTrigger {
                TraceLine { status: status,
                    TraceIcon { {icon} }
                    TraceTitle { {title} }
                    if let Some(description) = description {
                        TraceDescription { {description} }
                    }
                }
            }
        }
    }
}

/// The details of a subagent, shown while it is open. Renders nothing while closed.
#[component]
pub fn SubagentContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "empty:hidden ml-2 mt-2 flex flex-col gap-3 border-l pl-4 text-sm",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "subagent-content",
            class: class,
            ..attributes,
            TraceCollapsibleContent {
                {children}
            }
        }
    }
}

/// The task the subagent was given. Put it at the top of `SubagentContent`.
#[component]
pub fn SubagentTask(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "subagent-task",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A tool or model the subagent used, shown as an outline badge.
#[component]
pub fn SubagentTool(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-xs font-normal",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "subagent-tool",
            class: "contents",
            ..attributes,
            Badge { variant: BadgeVariant::Outline, class: class, {children} }
        }
    }
}
