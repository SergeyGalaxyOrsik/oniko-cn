//! Terminal: a shell command the agent runs, with its output as it streams, built from `Trace`
//! and `CodeBlock`. The line opens once the first output arrives while the command runs.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::code_block::{CodeBlock, CodeBlockContent, CodeBlockSize};
use super::trace::{
    TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon, TraceLine,
    TraceStatus, TraceTitle,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TerminalState {
    Running,
    /// The command exited with this code. Any code other than `0` is a failure.
    Exited(i32),
    /// The command never finished, for example because the chat stopped.
    Failed,
}

/// A command as one line of a `Trace`. `default_open` opens or closes the line from the start;
/// leave it unset to open the line while the command runs and has printed output.
#[component]
pub fn TerminalLine(
    command: String,
    output: String,
    state: TerminalState,
    default_open: Option<bool>,
) -> Element {
    let mut chosen = use_signal(|| default_open);
    let running = state == TerminalState::Running;
    let title = match state {
        TerminalState::Running => format!("Running {command}"),
        TerminalState::Exited(0) => format!("Ran {command}"),
        TerminalState::Exited(code) => format!("{command} exited with code {code}"),
        TerminalState::Failed => format!("Failed to run {command}"),
    };
    let status = match state {
        TerminalState::Running => TraceStatus::Running,
        TerminalState::Exited(0) => TraceStatus::Complete,
        _ => TraceStatus::Error,
    };

    rsx! {
        TraceCollapsible {
            open: chosen().unwrap_or(running && !output.is_empty()),
            on_open_change: move |next| chosen.set(Some(next)),
            TraceCollapsibleTrigger {
                TraceLine { status,
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
                    TraceTitle { "{title}" }
                }
            }
            TraceCollapsibleContent {
                CodeBlock { size: CodeBlockSize::Sm,
                    CodeBlockContent { wrap: true, class: "max-h-64", code: output }
                }
            }
        }
    }
}
