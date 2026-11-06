//! Subagent: a line for work the agent handed to a subagent, such as "Ran the research agent".
//! Open it for the tool calls the subagent made and what it found.
//! `Subagent` is the collapsible line and `SubagentStep` is one tool call. Put a `Subagent` in a `Trace`,
//! and the `SubagentStep`s in a `Trace` inside it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::code_block::{CodeBlock, CodeBlockContent, CodeBlockSize};
use super::trace::{
    TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceDescription,
    TraceIcon, TraceLine, TraceStatus, TraceTitle,
};

/// A spinner while the step runs, an alert when it failed, a check otherwise.
fn status_icon(status: TraceStatus) -> Element {
    rsx! {
        svg {
            class: if status == TraceStatus::Running { "animate-spin" },
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            match status {
                TraceStatus::Running => rsx! {
                    path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
                },
                TraceStatus::Error => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    line { x1: "12", x2: "12", y1: "8", y2: "12" }
                    line { x1: "12", x2: "12.01", y1: "16", y2: "16" }
                },
                _ => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    path { d: "m9 12 2 2 4-4" }
                },
            }
        }
    }
}

/// The run. `children` are its details: the task, the model and tools as badges, a `Trace` of
/// `SubagentStep`s and the result. Write `title` as "Running ..." while it runs and "Ran ..." after.
#[component]
pub fn Subagent(
    title: String,
    #[props(default)] status: TraceStatus,
    /// How long it ran, such as "48s".
    duration: Option<String>,
    #[props(default)] default_open: bool,
    children: Element,
) -> Element {
    rsx! {
        TraceCollapsible { default_open: default_open,
            TraceCollapsibleTrigger {
                TraceLine { status: status,
                    TraceIcon { {status_icon(status)} }
                    TraceTitle { "{title}" }
                    if let Some(duration) = duration {
                        TraceDescription { "{duration}" }
                    }
                }
            }
            TraceCollapsibleContent { keep_mounted: true, class: "flex flex-col gap-3", {children} }
        }
    }
}

/// One tool call of the subagent. With `output` it expands to show it.
#[component]
pub fn SubagentStep(
    /// What it ran, such as "Ran grep -rn cacheKey scripts".
    title: String,
    #[props(default)] status: TraceStatus,
    output: Option<String>,
) -> Element {
    let line = rsx! {
        TraceLine { status: status,
            TraceIcon { {status_icon(status)} }
            TraceTitle { "{title}" }
        }
    };

    let Some(output) = output else {
        return line;
    };

    rsx! {
        TraceCollapsible {
            TraceCollapsibleTrigger { {line} }
            TraceCollapsibleContent {
                CodeBlock { size: CodeBlockSize::Sm,
                    CodeBlockContent { wrap: true, class: "max-h-64", code: output }
                }
            }
        }
    }
}
