//! Chain of Thought: a summary line that expands into the steps an agent took, such as "Researched in 3 steps".
//! Built from `Trace`: `ChainOfThought` is the summary and `ChainOfThoughtStep` is one step in it.
//! While `status` is `Running` the summary opens by itself, until the person opens or closes it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
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

/// The summary and its steps. `children` are the steps, each a `ChainOfThoughtStep` or any `Trace` part.
#[component]
pub fn ChainOfThought(
    /// What happened, such as "Researching" while it runs and "Researched in 3 steps" after.
    title: String,
    #[props(default)] status: TraceStatus,
    /// Open at first render. Otherwise the summary is open only while `status` is `Running`.
    #[props(default)]
    default_open: bool,
    children: Element,
) -> Element {
    let mut chosen = use_signal(|| default_open.then_some(true));

    rsx! {
        Trace {
            TraceCollapsible {
                open: chosen().unwrap_or(status == TraceStatus::Running),
                on_open_change: move |next| chosen.set(Some(next)),
                TraceCollapsibleTrigger {
                    TraceLine { status: status,
                        TraceIcon { {status_icon(status)} }
                        TraceTitle { "{title}" }
                    }
                }
                TraceCollapsibleContent { keep_mounted: true,
                    Trace { {children} }
                }
            }
        }
    }
}

/// One step. With `details` it expands to show them, such as search results or command output.
#[component]
pub fn ChainOfThoughtStep(
    title: String,
    #[props(default)] status: TraceStatus,
    #[props(default)] default_open: bool,
    details: Option<Element>,
) -> Element {
    let line = rsx! {
        TraceLine { status: status,
            TraceIcon { {status_icon(status)} }
            TraceTitle { "{title}" }
        }
    };

    let Some(details) = details else {
        return line;
    };

    rsx! {
        TraceCollapsible { default_open: default_open,
            TraceCollapsibleTrigger { {line} }
            TraceCollapsibleContent { {details} }
        }
    }
}
