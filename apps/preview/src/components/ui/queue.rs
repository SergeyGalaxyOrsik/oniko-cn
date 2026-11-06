//! Queue: the agent's todo list, split into To do and Completed sections. The icon on each todo shows its
//! status, and a todo with a description opens to show it. Built with `Trace`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TodoStatus {
    #[default]
    Pending,
    InProgress,
    Completed,
}

#[derive(Clone, PartialEq)]
pub struct Todo {
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
}

fn icon(paths: Element) -> Element {
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
                {paths}
            }
        }
    }
}

fn status_icon(status: TodoStatus) -> Element {
    match status {
        TodoStatus::Pending => icon(rsx! {
            circle { cx: "12", cy: "12", r: "10" }
        }),
        TodoStatus::InProgress => icon(rsx! {
            circle { cx: "12", cy: "12", r: "10" }
            circle { cx: "12", cy: "12", r: "1" }
        }),
        TodoStatus::Completed => icon(rsx! {
            circle { cx: "12", cy: "12", r: "10" }
            path { d: "m9 12 2 2 4-4" }
        }),
    }
}

#[component]
fn TodoRow(todo: Todo, streaming: bool) -> Element {
    let status = if streaming && todo.status == TodoStatus::InProgress {
        TraceStatus::Running
    } else {
        TraceStatus::Complete
    };
    let line = rsx! {
        TraceLine { status,
            {status_icon(todo.status)}
            TraceTitle { "{todo.title}" }
        }
    };

    match todo.description {
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
fn Section(label: String, todos: Vec<Todo>, streaming: bool, to_do: bool) -> Element {
    if todos.is_empty() {
        return rsx! {};
    }

    rsx! {
        TraceCollapsible { default_open: true,
            TraceCollapsibleTrigger {
                TraceLine {
                    {icon(if to_do {
                        rsx! {
                            rect { x: "3", y: "5", width: "6", height: "6", rx: "1" }
                            path { d: "m3 17 2 2 4-4" }
                            path { d: "M13 6h8" }
                            path { d: "M13 12h8" }
                            path { d: "M13 18h8" }
                        }
                    } else {
                        rsx! {
                            path { d: "m3 17 2 2 4-4" }
                            path { d: "m3 7 2 2 4-4" }
                            path { d: "M13 6h8" }
                            path { d: "M13 12h8" }
                            path { d: "M13 18h8" }
                        }
                    })}
                    TraceTitle { "{todos.len()} {label}" }
                }
            }
            TraceCollapsibleContent {
                Trace {
                    for todo in todos {
                        TodoRow { key: "{todo.title}", todo, streaming }
                    }
                }
            }
        }
    }
}

/// Sorts `todos` into the two sections. Set `streaming` while the agent is still working, so the todo in
/// progress shimmers.
#[component]
pub fn Queue(todos: Vec<Todo>, #[props(default)] streaming: bool) -> Element {
    let (completed, pending): (Vec<Todo>, Vec<Todo>) = todos
        .into_iter()
        .partition(|todo| todo.status == TodoStatus::Completed);

    rsx! {
        Trace {
            Section { label: "to do", todos: pending, streaming, to_do: true }
            Section { label: "completed", todos: completed, streaming, to_do: false }
        }
    }
}
