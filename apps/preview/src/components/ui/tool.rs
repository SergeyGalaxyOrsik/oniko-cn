//! Tool: shows one tool call of an AI agent, with its status, input and output in a collapsible panel.
//!
//! `Tool` holds the open state, `ToolHeader` shows the tool name and a status badge, and `ToolContent`
//! holds `ToolInput` and `ToolOutput`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::badge::{Badge, BadgeVariant};
use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

/// Lifecycle of a tool call. Mirrors the AI SDK tool part states.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToolState {
    #[default]
    InputStreaming,
    InputAvailable,
    ApprovalRequested,
    ApprovalResponded,
    OutputAvailable,
    OutputError,
    OutputDenied,
}

impl ToolState {
    /// Value of the `data-state` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InputStreaming => "input-streaming",
            Self::InputAvailable => "input-available",
            Self::ApprovalRequested => "approval-requested",
            Self::ApprovalResponded => "approval-responded",
            Self::OutputAvailable => "output-available",
            Self::OutputError => "output-error",
            Self::OutputDenied => "output-denied",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::InputStreaming => "Pending",
            Self::InputAvailable => "Running",
            Self::ApprovalRequested => "Awaiting Approval",
            Self::ApprovalResponded => "Responded",
            Self::OutputAvailable => "Completed",
            Self::OutputError => "Error",
            Self::OutputDenied => "Denied",
        }
    }

    fn badge_variant(self) -> BadgeVariant {
        match self {
            Self::OutputError => BadgeVariant::Destructive,
            Self::OutputDenied => BadgeVariant::Outline,
            _ => BadgeVariant::Secondary,
        }
    }

    /// SVG path data of the status icon.
    fn icon_paths(self) -> &'static [&'static str] {
        match self {
            Self::InputStreaming | Self::InputAvailable => {
                &["M12 2a10 10 0 1 0 0 20 10 10 0 1 0 0-20z", "M12 6v6l4 2"]
            }
            Self::ApprovalRequested | Self::ApprovalResponded => {
                &["M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"]
            }
            Self::OutputAvailable => &["M20 6 9 17l-5-5"],
            Self::OutputError => &[
                "M12 2a10 10 0 1 0 0 20 10 10 0 1 0 0-20z",
                "M12 8v4",
                "M12 16h.01",
            ],
            Self::OutputDenied => &["M18 6 6 18", "M6 6l12 12"],
        }
    }
}

/// Root of one tool call. Closed by default; the header toggles it.
#[component]
pub fn Tool(
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
        "mb-4 w-full overflow-hidden rounded-md border border-border",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "tool",
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

/// The toggle row: tool name, status badge and chevron. Defaults to `title`, then `tool_name`, then "Tool".
#[component]
pub fn ToolHeader(
    #[props(default)] state: ToolState,
    title: Option<String>,
    tool_name: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex w-full items-center justify-between gap-4 p-3",
        take_class(&mut attributes).as_deref()
    );
    let name = title.or(tool_name).unwrap_or_else(|| "Tool".to_string());
    let icon_paths = state.icon_paths();

    rsx! {
        div {
            "data-slot": "tool-header",
            "data-state": state.as_str(),
            class: class,
            ..attributes,
            CollapsibleTrigger {
                class: "group flex min-w-0 flex-1 items-center gap-2 text-left text-sm transition-colors hover:text-foreground",
                svg {
                    "data-slot": "tool-icon",
                    class: "size-4 shrink-0 text-muted-foreground",
                    "aria-hidden": "true",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" }
                }
                span { "data-slot": "tool-title", class: "min-w-0 truncate font-medium", {name} }
                Badge {
                    variant: state.badge_variant(),
                    class: "gap-1 text-xs",
                    svg {
                        "data-slot": "tool-status-icon",
                        class: "size-3 shrink-0",
                        "aria-hidden": "true",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        for d in icon_paths {
                            path { d: *d }
                        }
                    }
                    {state.label()}
                }
                svg {
                    "data-slot": "tool-chevron",
                    class: "size-4 shrink-0 text-muted-foreground transition-transform group-data-[state=open]:rotate-180",
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
        }
    }
}

/// The expandable part of a tool call. Shown only while `Tool` is open.
#[component]
pub fn ToolContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "tool-content",
            class: class,
            ..attributes,
            CollapsibleContent {
                class: "space-y-4 border-t border-border p-4",
                {children}
            }
        }
    }
}

/// The tool's input, shown as preformatted text. The caller serializes it, e.g. with `serde_json`.
#[component]
pub fn ToolInput(
    input: String,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "space-y-2 overflow-hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "tool-input",
            class: class,
            ..attributes,
            h4 {
                "data-slot": "tool-label",
                class: "text-xs font-medium tracking-wide text-muted-foreground uppercase",
                "Parameters"
            }
            pre {
                "data-slot": "tool-input-code",
                class: "overflow-x-auto rounded-md bg-muted p-3 font-mono text-xs whitespace-pre-wrap break-all",
                {input}
            }
        }
    }
}

/// The tool's result or error. Renders nothing when both are empty.
#[component]
pub fn ToolOutput(
    output: Option<String>,
    error_text: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let output = output.filter(|text| !text.is_empty());
    let error_text = error_text.filter(|text| !text.is_empty());
    if output.is_none() && error_text.is_none() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "space-y-2 overflow-hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "tool-output",
            class: class,
            ..attributes,
            if let Some(text) = output {
                h4 {
                    "data-slot": "tool-label",
                    class: "text-xs font-medium tracking-wide text-muted-foreground uppercase",
                    "Result"
                }
                pre {
                    "data-slot": "tool-output-code",
                    class: "overflow-x-auto rounded-md bg-muted p-3 font-mono text-xs whitespace-pre-wrap break-all",
                    {text}
                }
            }
            if let Some(text) = error_text {
                h4 {
                    "data-slot": "tool-label",
                    class: "text-xs font-medium tracking-wide text-destructive uppercase",
                    "Error"
                }
                pre {
                    "data-slot": "tool-error",
                    class: "overflow-x-auto rounded-md bg-destructive/10 p-3 font-mono text-xs whitespace-pre-wrap break-all text-destructive",
                    {text}
                }
            }
        }
    }
}
