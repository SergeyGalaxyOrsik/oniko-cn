//! Tool: one tool call of an AI agent, from input to output, built from `Trace` and `CodeBlock`.
//! `ToolState` maps an AI SDK tool part to the status and title of its line. A pending approval is
//! not a line: show it as an `Item` outside the `Trace`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::code_block::{CodeBlock, CodeBlockContent, CodeBlockSize};
use super::trace::{
    TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon, TraceLine,
    TraceStatus, TraceTitle,
};

#[derive(Clone, PartialEq)]
pub enum ToolState {
    /// Input streaming or available. Running while the chat streams, failed after it stops.
    Input,
    /// Output available, with the output.
    Output(String),
    /// Output error, with the error text.
    Error(String),
    Denied,
}

#[component]
fn TerminalIcon() -> Element {
    rsx! {
        TraceIcon {
            svg {
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 19h8" }
                path { d: "m4 17 6-6-6-6" }
            }
        }
    }
}

/// A tool call as one line of a `Trace`. A call with output expands to show it.
#[component]
pub fn ToolLine(
    command: String,
    state: ToolState,
    #[props(default)] streaming: bool,
    #[props(default)] default_open: bool,
) -> Element {
    match &state {
        ToolState::Denied => rsx! {
            TraceLine {
                TerminalIcon {}
                TraceTitle { "Denied {command}" }
            }
        },
        ToolState::Output(text) | ToolState::Error(text) => {
            let failed = matches!(state, ToolState::Error(_));
            let verb = if failed { "Failed to run" } else { "Ran" };
            rsx! {
                TraceCollapsible { default_open,
                    TraceCollapsibleTrigger {
                        TraceLine { status: if failed { TraceStatus::Error } else { TraceStatus::Complete },
                            TerminalIcon {}
                            TraceTitle { "{verb} {command}" }
                        }
                    }
                    TraceCollapsibleContent {
                        CodeBlock { size: CodeBlockSize::Sm,
                            CodeBlockContent { wrap: true, class: "max-h-64", code: text.clone() }
                        }
                    }
                }
            }
        }
        ToolState::Input => {
            let verb = if streaming { "Running" } else { "Failed to run" };
            rsx! {
                TraceLine { status: if streaming { TraceStatus::Running } else { TraceStatus::Error },
                    TerminalIcon {}
                    TraceTitle { "{verb} {command}" }
                }
            }
        }
    }
}
