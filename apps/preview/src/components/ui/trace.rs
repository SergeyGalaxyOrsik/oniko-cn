//! Trace: a compact, optionally expandable list of steps an AI run took, such as a search or a command.
//! Trace, TraceLine, TraceIcon, TraceTitle and TraceDescription render one step.
//! TraceCollapsible, TraceCollapsibleTrigger and TraceCollapsibleContent make a step expandable.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TraceStatus {
    #[default]
    Complete,
    Running,
    Error,
    Pending,
}

impl TraceStatus {
    /// Value of the `data-status` attribute, or `None` for a finished step.
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            Self::Complete => None,
            Self::Running => Some("running"),
            Self::Error => Some("error"),
            Self::Pending => Some("pending"),
        }
    }
}

/// `data-slot` and `class` for a part that is built from another component, which cannot take them as literal attributes.
/// Callers' own attributes come after these, so they can override them.
fn slot_and_class(slot: &'static str, class: String) -> [Attribute; 2] {
    [
        Attribute::new("data-slot", slot, None, false),
        Attribute::new("class", class, None, false),
    ]
}

/// Root of a trace. Stacks its steps vertically.
#[component]
pub fn Trace(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 *:has-[>[data-slot=trace-line]]:rounded-md *:has-[>[data-slot=trace-line]]:focus-visible:ring-3 *:has-[>[data-slot=trace-line]]:focus-visible:ring-ring/50 flex w-full min-w-0 flex-col *:has-[>[data-slot=trace-line]]:flex *:has-[>[data-slot=trace-line]]:w-fit *:has-[>[data-slot=trace-line]]:max-w-full *:has-[>[data-slot=trace-line]]:min-w-0 *:has-[>[data-slot=trace-line]]:text-left *:has-[>[data-slot=trace-line]]:outline-none",
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
        "gap-2 text-sm flex w-fit max-w-full min-w-0 items-center text-muted-foreground transition-colors in-[a,button]:cursor-pointer in-[a:hover,button:hover]:text-foreground data-[status=error]:text-destructive data-[status=error]:in-[a:hover,button:hover]:text-destructive data-[status=pending]:opacity-60 data-[status=running]:*:data-[slot=trace-title]:shimmer",
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
        "size-4 [&_svg:not([class*='size-'])]:size-4 [[data-slot=trace-title]~&]:-ml-1 flex shrink-0 items-center justify-center [&_svg]:pointer-events-none",
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

/// Single-line sentence of a trace step. Truncates, and shimmers while the step runs.
#[component]
pub fn TraceTitle(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "min-w-0 truncate motion-reduce:shimmer-none",
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
        "shrink-0 text-muted-foreground/70 tabular-nums",
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
        "flex w-full min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    attributes.splice(0..0, slot_and_class("trace-collapsible", class));

    rsx! {
        Collapsible {
            open: open,
            default_open: default_open,
            on_open_change: on_open_change,
            attributes: attributes,
            {children}
        }
    }
}

/// Button that toggles a collapsible trace step. Wrap a `TraceLine` in it.
#[component]
pub fn TraceCollapsibleTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-1 rounded-md focus-visible:ring-3 focus-visible:ring-ring/50 group/trace-collapsible-trigger flex w-fit max-w-full min-w-0 cursor-pointer items-center text-left text-muted-foreground transition-colors outline-none hover:text-foreground has-[[data-status=error]]:text-destructive",
        take_class(&mut attributes).as_deref()
    );

    attributes.splice(0..0, slot_and_class("trace-collapsible-trigger", class));

    rsx! {
        CollapsibleTrigger {
            attributes: attributes,
            {children}
            TraceIcon { class: "transition-transform group-aria-expanded/trace-collapsible-trigger:rotate-90",
                svg {
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "m9 18 6-6-6-6" }
                }
            }
        }
    }
}

/// Details of a collapsible trace step, shown while the step is open.
/// `class` goes on the inner wrapper, like upstream; every other attribute goes on the panel.
#[component]
pub fn TraceCollapsibleContent(
    /// Keep the content in the DOM while it is closed, so nested state survives.
    #[props(default)]
    keep_mounted: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mt-2 mb-1 ml-[calc(--spacing(2)-1px)] border-l pr-1 pl-4 leading-relaxed min-w-0 wrap-break-word text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    attributes.splice(
        0..0,
        slot_and_class(
            "trace-collapsible-content",
            "duration-200 h-(--collapsible-panel-height) min-w-0 overflow-hidden transition-[height] data-ending-style:h-0 data-starting-style:h-0".to_string(),
        ),
    );

    rsx! {
        CollapsibleContent {
            keep_mounted: keep_mounted,
            attributes: attributes,
            div {
                "data-slot": "trace-collapsible-content-inner",
                class: class,
                {children}
            }
        }
    }
}
