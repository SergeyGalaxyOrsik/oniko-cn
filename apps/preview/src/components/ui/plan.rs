//! Plan: the steps the agent decides on before it starts, and how far it has got.
//! `Plan` is the collapsible line with a title and a progress `description` such as "2 of 5 done".
//! `PlanStep` is one step. Put a `Plan` in a `Trace`, and the `PlanStep`s in a `Trace` inside it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::trace::{
    TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceDescription,
    TraceIcon, TraceLine, TraceStatus, TraceTitle,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlanStepStatus {
    Done,
    Running,
    #[default]
    Pending,
}

/// The plan. `children` go inside it: an optional description, then a `Trace` of `PlanStep`s.
#[component]
pub fn Plan(
    title: String,
    /// Progress, such as "2 of 5 done".
    description: Option<String>,
    /// Whether the agent is still working through the plan.
    #[props(default)]
    running: bool,
    #[props(default = true)] default_open: bool,
    children: Element,
) -> Element {
    rsx! {
        TraceCollapsible { default_open: default_open,
            TraceCollapsibleTrigger {
                TraceLine { status: if running { TraceStatus::Running } else { TraceStatus::Complete },
                    TraceIcon {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "6", cy: "19", r: "3" }
                            path { d: "M9 19h8.5a3.5 3.5 0 0 0 0-7h-11a3.5 3.5 0 0 1 0-7H15" }
                            circle { cx: "18", cy: "5", r: "3" }
                        }
                    }
                    TraceTitle { "{title}" }
                    if let Some(description) = description {
                        TraceDescription { "{description}" }
                    }
                }
            }
            TraceCollapsibleContent { {children} }
        }
    }
}

/// One step of the plan. A running step only shimmers while the plan is `running`.
#[component]
pub fn PlanStep(
    title: String,
    #[props(default)] status: PlanStepStatus,
    #[props(default)] running: bool,
) -> Element {
    let line_status = match status {
        PlanStepStatus::Pending => TraceStatus::Pending,
        PlanStepStatus::Running if running => TraceStatus::Running,
        _ => TraceStatus::Complete,
    };

    rsx! {
        TraceLine { status: line_status,
            TraceIcon {
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "12", cy: "12", r: "10" }
                    match status {
                        PlanStepStatus::Done => rsx! {
                            path { d: "m9 12 2 2 4-4" }
                        },
                        PlanStepStatus::Running => rsx! {
                            circle { cx: "12", cy: "12", r: "1" }
                        },
                        PlanStepStatus::Pending => rsx! {},
                    }
                }
            }
            TraceTitle { "{title}" }
        }
    }
}
