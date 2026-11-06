//! Plan examples, ported from shadcn/ui `examples/base/plan-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::plan::{Plan, PlanStep, PlanStepStatus};
use crate::components::ui::trace::Trace;

#[component]
pub fn Examples() -> Element {
    rsx! {
        PlanDemo {}
        PlanDescription {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

const QUESTION: &str = "Migrate the dialog components to Base UI.";
const ANSWER: &str = "Migrated Dialog and AlertDialog. The type checker passes.";
const STEPS: [&str; 4] = [
    "Install @base-ui/react",
    "Replace the Radix imports",
    "Rename asChild to render",
    "Run the type checker",
];

fn step_status(index: usize, current: usize) -> PlanStepStatus {
    match index {
        i if i < current => PlanStepStatus::Done,
        i if i == current => PlanStepStatus::Running,
        _ => PlanStepStatus::Pending,
    }
}

#[component]
fn PlanDemo() -> Element {
    let mut current = use_signal(|| Some(STEPS.len()));
    let mut answer = use_signal(|| ANSWER.to_string());
    let mut busy = use_signal(|| false);

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                Message { align: MessageAlign::End,
                    MessageContent {
                        Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                            BubbleContent { "{QUESTION}" }
                        }
                    }
                }
                Message {
                    MessageContent { class: "gap-4",
                        if let Some(current) = current() {
                            Trace {
                                Plan {
                                    title: "Plan: migrate to Base UI",
                                    description: format!("{} of {} done", current.min(STEPS.len()), STEPS.len()),
                                    running: busy() && current < STEPS.len(),
                                    Trace {
                                        for (index , title) in STEPS.into_iter().enumerate() {
                                            PlanStep {
                                                key: "{title}",
                                                title,
                                                status: step_status(index, current),
                                                running: busy(),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !answer().is_empty() {
                            p { "{answer}" }
                        }
                    }
                }
                if !busy() {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "mt-auto self-start",
                        onclick: move |_| {
                            current.set(None);
                            answer.set(String::new());
                            busy.set(true);
                            spawn(async move {
                                for index in 0..STEPS.len() {
                                    current.set(Some(index));
                                    sleep(1200).await;
                                }
                                current.set(Some(STEPS.len()));
                                answer.set(ANSWER.to_string());
                                busy.set(false);
                            });
                        },
                        "Replay"
                    }
                }
            }
        }
    }
}

#[component]
fn PlanDescription() -> Element {
    let steps = [
        "Add a dark variant to the theme tokens",
        "Read the theme from a cookie on the server",
        "Add a toggle to the settings page",
        "Check contrast on the charts",
    ];

    rsx! {
        Example { title: "Details", class: "h-auto",
            Trace { class: "max-w-sm",
                Plan {
                    title: "Plan: add dark mode",
                    description: format!("1 of {} done", steps.len()),
                    div { class: "flex flex-col gap-3",
                        p {
                            "Store the theme in a cookie so the server renders the right one and the page does not flash on load."
                        }
                        Trace {
                            for (index , step) in steps.into_iter().enumerate() {
                                PlanStep {
                                    key: "{step}",
                                    title: step,
                                    status: if index == 0 { PlanStepStatus::Done } else { PlanStepStatus::Pending },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
