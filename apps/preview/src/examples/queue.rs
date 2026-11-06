//! Queue examples, ported from shadcn/ui `examples/base/queue-demo.tsx` and `queue-descriptions.tsx`.
//! The AI SDK stream is simulated with signals and timers.

use dioxus::prelude::*;
use lucide_dioxus::{Circle, CircleCheck, CircleDot, ListChecks, ListTodo};

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

const QUESTION: &str = "Add dark mode to the settings page.";
const ANSWER: &str = "Dark mode is in. The toggle is under Settings, Appearance.";
const TODOS: [&str; 4] = [
    "Add a dark variant to the theme tokens",
    "Read the theme from a cookie",
    "Add a toggle to the settings page",
    "Check contrast on the charts",
];

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Pending,
    InProgress,
    Completed,
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        QueueDemo {}
        QueueDescriptions {}
    }
}

/// The todo list as the agent last wrote it, with `current` the index of the todo in progress.
fn todos_at(current: usize) -> Vec<(&'static str, Status)> {
    TODOS
        .iter()
        .enumerate()
        .map(|(index, content)| {
            let status = match index.cmp(&current) {
                std::cmp::Ordering::Less => Status::Completed,
                std::cmp::Ordering::Equal => Status::InProgress,
                std::cmp::Ordering::Greater => Status::Pending,
            };
            (*content, status)
        })
        .collect()
}

#[component]
fn TodoLine(content: &'static str, status: Status, streaming: bool) -> Element {
    rsx! {
        TraceLine { status: if streaming && status == Status::InProgress { TraceStatus::Running } else { TraceStatus::Complete },
            TraceIcon {
                match status {
                    Status::Completed => rsx! {
                        CircleCheck {}
                    },
                    Status::InProgress => rsx! {
                        CircleDot {}
                    },
                    Status::Pending => rsx! {
                        Circle {}
                    },
                }
            }
            TraceTitle { "{content}" }
        }
    }
}

#[component]
fn TodoQueue(todos: Vec<(&'static str, Status)>, streaming: bool) -> Element {
    let pending: Vec<_> = todos
        .iter()
        .filter(|todo| todo.1 != Status::Completed)
        .cloned()
        .collect();
    let completed: Vec<_> = todos
        .iter()
        .filter(|todo| todo.1 == Status::Completed)
        .cloned()
        .collect();

    rsx! {
        Trace {
            if !pending.is_empty() {
                TraceCollapsible { default_open: true,
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                ListTodo {}
                            }
                            TraceTitle { "{pending.len()} to do" }
                        }
                    }
                    TraceCollapsibleContent {
                        Trace {
                            for (content , status) in pending {
                                TodoLine { key: "{content}", content, status, streaming }
                            }
                        }
                    }
                }
            }
            if !completed.is_empty() {
                TraceCollapsible { default_open: true,
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                ListChecks {}
                            }
                            TraceTitle { "{completed.len()} completed" }
                        }
                    }
                    TraceCollapsibleContent {
                        Trace {
                            for (content , status) in completed {
                                TodoLine { key: "{content}", content, status, streaming }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QueueDemo() -> Element {
    let mut asked = use_signal(|| true);
    // None before the agent has written its first list, TODOS.len() when every todo is done.
    let mut step = use_signal(|| Some(TODOS.len()));
    let mut answered = use_signal(|| true);
    let mut busy = use_signal(|| false);

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                if asked() {
                    Message { align: MessageAlign::End,
                        MessageContent {
                            Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                                BubbleContent { "{QUESTION}" }
                            }
                        }
                    }
                }
                if step().is_some() || answered() {
                    Message {
                        MessageContent { class: "gap-4",
                            if let Some(current) = step() {
                                TodoQueue { todos: todos_at(current), streaming: busy() }
                            }
                            if answered() {
                                p { "{ANSWER}" }
                            }
                        }
                    }
                }
                if !busy() {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "mt-auto self-start",
                        onclick: move |_| {
                            asked.set(false);
                            step.set(None);
                            answered.set(false);
                            busy.set(true);
                            spawn(async move {
                                sleep(200).await;
                                asked.set(true);
                                for current in 0..=TODOS.len() {
                                    sleep(if current == 0 { 600 } else { 1400 }).await;
                                    step.set(Some(current));
                                }
                                sleep(300).await;
                                answered.set(true);
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

struct Todo {
    title: &'static str,
    description: Option<&'static str>,
    done: bool,
}

const DESCRIBED: [Todo; 5] = [
    Todo {
        title: "Write the migration guide",
        description: Some("Cover the renamed props and the new data attributes."),
        done: true,
    },
    Todo {
        title: "Add a changeset",
        description: None,
        done: true,
    },
    Todo {
        title: "Fix the focus ring on the select trigger",
        description: Some("It is clipped by overflow-hidden on the field."),
        done: false,
    },
    Todo {
        title: "Update the registry snapshots",
        description: None,
        done: false,
    },
    Todo {
        title: "Add tests for the combobox",
        description: Some("Typing, clearing and keyboard selection."),
        done: false,
    },
];

#[component]
fn TodoRow(title: &'static str, description: Option<&'static str>, done: bool) -> Element {
    let line = rsx! {
        TraceLine {
            TraceIcon {
                if done {
                    CircleCheck {}
                } else {
                    Circle {}
                }
            }
            TraceTitle { "{title}" }
        }
    };

    match description {
        None => line,
        Some(description) => rsx! {
            TraceCollapsible {
                TraceCollapsibleTrigger { {line} }
                TraceCollapsibleContent { "{description}" }
            }
        },
    }
}

#[component]
fn QueueDescriptions() -> Element {
    let pending: Vec<&Todo> = DESCRIBED.iter().filter(|todo| !todo.done).collect();
    let completed: Vec<&Todo> = DESCRIBED.iter().filter(|todo| todo.done).collect();

    rsx! {
        Example { title: "Descriptions", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceCollapsible { default_open: true,
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                ListTodo {}
                            }
                            TraceTitle { "{pending.len()} to do" }
                        }
                    }
                    TraceCollapsibleContent {
                        Trace {
                            for todo in pending {
                                TodoRow {
                                    key: "{todo.title}",
                                    title: todo.title,
                                    description: todo.description,
                                    done: todo.done,
                                }
                            }
                        }
                    }
                }
                TraceCollapsible {
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                ListChecks {}
                            }
                            TraceTitle { "{completed.len()} completed" }
                        }
                    }
                    TraceCollapsibleContent {
                        Trace {
                            for todo in completed {
                                TodoRow {
                                    key: "{todo.title}",
                                    title: todo.title,
                                    description: todo.description,
                                    done: todo.done,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
