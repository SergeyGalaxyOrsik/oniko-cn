//! Reasoning: one line for the agent's thinking, with the thinking itself inside.
//! It reads "Thinking" and opens while `running`, then reads "Thought" and closes when it is done,
//! unless the person opened or closed it. Put one or more in a `Trace`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::trace::{
    TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceDescription,
    TraceIcon, TraceLine, TraceStatus, TraceTitle,
};

#[component]
pub fn Reasoning(
    /// The thinking so far. While it is empty the line has nothing to open.
    text: String,
    /// Whether the model is still thinking.
    #[props(default)]
    running: bool,
    /// How long it took, such as "12s". Shown after the title.
    duration: Option<String>,
    /// Open at first render, instead of following `running`.
    default_open: Option<bool>,
) -> Element {
    let mut chosen = use_signal(|| default_open);

    let line = rsx! {
        TraceLine { status: if running { TraceStatus::Running } else { TraceStatus::Complete },
            TraceIcon {
                svg {
                    class: if running { "animate-spin" },
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    if running {
                        path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
                    } else {
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "m9 12 2 2 4-4" }
                    }
                }
            }
            TraceTitle {
                if running {
                    "Thinking"
                } else {
                    "Thought"
                }
            }
            if let Some(duration) = duration {
                TraceDescription { "{duration}" }
            }
        }
    };

    if text.is_empty() {
        return line;
    }

    rsx! {
        TraceCollapsible {
            open: chosen().unwrap_or(running),
            on_open_change: move |next| chosen.set(Some(next)),
            TraceCollapsibleTrigger { {line} }
            TraceCollapsibleContent { class: "whitespace-pre-wrap", "{text}" }
        }
    }
}
