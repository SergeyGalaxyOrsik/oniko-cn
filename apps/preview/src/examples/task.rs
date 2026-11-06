//! Task examples, ported from shadcn/ui `examples/base/task-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::task::{Task, TaskStep};
use crate::components::ui::trace::{Trace, TraceStatus};

#[component]
pub fn Examples() -> Element {
    rsx! {
        TaskDemo {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

struct TaskSpec {
    title: [&'static str; 2],
    steps: &'static [[&'static str; 2]],
}

const QUESTION: &str = "Add Open Graph metadata to the home page.";
const ANSWER: &str = "Added Open Graph defaults to app/layout.tsx and a title, description and image for the home page in app/page.tsx.";
const TASKS: [TaskSpec; 2] = [
    TaskSpec {
        title: ["Finding the project files", "Found the project files"],
        steps: &[
            [
                "Searching for generateMetadata",
                "Searched for generateMetadata",
            ],
            ["Reading app/layout.tsx", "Read app/layout.tsx"],
            ["Reading app/page.tsx", "Read app/page.tsx"],
        ],
    },
    TaskSpec {
        title: ["Updating the page metadata", "Updated the page metadata"],
        steps: &[
            ["Editing app/layout.tsx", "Edited app/layout.tsx"],
            ["Editing app/page.tsx", "Edited app/page.tsx"],
        ],
    },
];

/// A task with `done` steps finished. The step after them is running, until all are done.
#[component]
fn TaskRow(index: usize, done: usize, streaming: bool) -> Element {
    let task = &TASKS[index];
    let title = task.title[usize::from(done >= task.steps.len())];
    let steps: Vec<(&str, TraceStatus)> = task
        .steps
        .iter()
        .take(done + 1)
        .enumerate()
        .map(|(step, [running, finished])| {
            if step < done {
                (*finished, TraceStatus::Complete)
            } else if streaming {
                (*running, TraceStatus::Running)
            } else {
                (*running, TraceStatus::Error)
            }
        })
        .collect();
    let status = if steps.iter().any(|(_, s)| *s == TraceStatus::Error) {
        TraceStatus::Error
    } else if steps.iter().any(|(_, s)| *s == TraceStatus::Running) {
        TraceStatus::Running
    } else {
        TraceStatus::Complete
    };

    rsx! {
        Task { title, status,
            for (step , step_status) in steps {
                TaskStep { key: "{step}", title: step, status: step_status }
            }
        }
    }
}

#[component]
fn TaskDemo() -> Element {
    let mut tasks = use_signal(|| {
        TASKS
            .iter()
            .enumerate()
            .map(|(index, task)| (index, task.steps.len()))
            .collect::<Vec<_>>()
    });
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
                        if !tasks().is_empty() {
                            Trace {
                                for (index , done) in tasks() {
                                    TaskRow { key: "{index}", index, done, streaming: busy() }
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
                            tasks.set(Vec::new());
                            answer.set(String::new());
                            busy.set(true);
                            spawn(async move {
                                for (index, task) in TASKS.iter().enumerate() {
                                    tasks.write().push((index, 0));
                                    for done in 0..=task.steps.len() {
                                        if let Some(entry) = tasks.write().last_mut() {
                                            entry.1 = done;
                                        }
                                        sleep(900).await;
                                    }
                                }
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
