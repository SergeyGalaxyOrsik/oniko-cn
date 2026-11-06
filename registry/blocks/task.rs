//! Task: a piece of work the agent is doing, with each step appearing under it as the agent takes it.
//! `Task` is the collapsible line and `TaskStep` is one step. Put one or more tasks in a `Trace`.
//! While `status` is `Running` the task opens by itself, until the person opens or closes it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

/// The task. Write `title` in the present tense while it runs and in the past tense once it is done.
#[component]
pub fn Task(
    title: String,
    #[props(default)] status: TraceStatus,
    /// Open at first render. Otherwise the task is open only while `status` is `Running`.
    #[props(default)]
    default_open: bool,
    children: Element,
) -> Element {
    let mut chosen = use_signal(|| default_open.then_some(true));

    rsx! {
        TraceCollapsible {
            open: chosen().unwrap_or(status == TraceStatus::Running),
            on_open_change: move |next| chosen.set(Some(next)),
            TraceCollapsibleTrigger {
                TraceLine { status: status,
                    TraceIcon {
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
                    TraceTitle { "{title}" }
                }
            }
            TraceCollapsibleContent { keep_mounted: true,
                Trace { {children} }
            }
        }
    }
}

/// One step of a task.
#[component]
pub fn TaskStep(title: String, #[props(default)] status: TraceStatus) -> Element {
    rsx! {
        TraceLine { status: status,
            TraceTitle { "{title}" }
        }
    }
}
