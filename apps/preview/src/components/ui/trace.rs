//! Trace: a compact, optionally expandable list of steps an AI run took, such as a search or a command.
//! Trace, TraceLine, TraceIcon, TraceTitle and TraceDescription render one step.
//! TraceCollapsible, TraceCollapsibleTrigger and TraceCollapsibleContent make a step expandable.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

const TRACE_LINE_BASE: &str = "gap-2 text-sm group/trace-line flex min-w-0 items-center";

const TRACE_COLLAPSIBLE_TRIGGER: &str = "gap-1 rounded-md focus-visible:ring-3 focus-visible:ring-ring/50 group flex w-full min-w-0 items-center text-left outline-none";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TraceStatus {
    #[default]
    Complete,
    Running,
    Error,
    Pending,
}

impl TraceStatus {
    /// Value of the `data-status` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Running => "running",
            Self::Error => "error",
            Self::Pending => "pending",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Complete => "text-muted-foreground",
            Self::Running => "text-foreground",
            Self::Error => "text-destructive",
            Self::Pending => "text-muted-foreground/50",
        }
    }
}

/// Root of a trace. Stacks its steps vertically.
#[component]
pub fn Trace(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 *:has-[>[data-slot=trace-line]]:rounded-md *:has-[>[data-slot=trace-line]]:focus-visible:ring-3 *:has-[>[data-slot=trace-line]]:focus-visible:ring-ring/50 flex w-full flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "trace",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One step of a trace. Its `status` marks it as running, failed or pending.
#[component]
pub fn TraceLine(
    #[props(default)] status: TraceStatus,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        TRACE_LINE_BASE,
        status.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "trace-line",
            "data-status": status.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Decorative icon of a trace step. Hidden from assistive technology.
#[component]
pub fn TraceIcon(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "size-4 [&_svg:not([class*='size-'])]:size-4 [[data-slot=trace-title]~&]:-ml-1 inline-flex items-center justify-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "trace-icon",
            "aria-hidden": "true",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Single-line sentence of a trace step. Truncates, and pulses while the step runs.
#[component]
pub fn TraceTitle(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "min-w-0 truncate group-data-[status=running]/trace-line:animate-pulse",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "trace-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Secondary text of a trace step, such as a count, a duration or a diff stat. Never truncated.
#[component]
pub fn TraceDescription(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "shrink-0 whitespace-nowrap text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "trace-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A trace step that expands to show its details. Starts closed unless `default_open` is set.
#[component]
pub fn TraceCollapsible(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    #[props(default)] default_open: bool,
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
            "data-slot": "trace-collapsible",
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

/// Button that toggles a collapsible trace step. Wrap a `TraceLine` in it.
#[component]
pub fn TraceCollapsibleTrigger(children: Element) -> Element {
    rsx! {
        CollapsibleTrigger {
            "data-slot": "trace-collapsible-trigger",
            class: TRACE_COLLAPSIBLE_TRIGGER,
            {children}
            svg {
                "data-slot": "trace-collapsible-chevron",
                class: "size-4 shrink-0 transition-transform group-data-[state=open]:rotate-180",
                "aria-hidden": "true",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}

/// Details of a collapsible trace step, shown while the step is open.
#[component]
pub fn TraceCollapsibleContent(children: Element) -> Element {
    rsx! {
        CollapsibleContent {
            "data-slot": "trace-collapsible-content",
            class: "duration-200",
            div {
                "data-slot": "trace-collapsible-content-inner",
                class: "mt-2 mb-1 ml-[calc(--spacing(2)-1px)] border-l pr-1 pl-4 leading-relaxed",
                {children}
            }
        }
    }
}
