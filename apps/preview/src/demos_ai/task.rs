use crate::components::ui::task::{Task, TaskContent, TaskItem, TaskItemFile, TaskTrigger};
use dioxus::prelude::*;

#[component]
pub fn TaskDemo() -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Task" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                Task {
                    TaskTrigger { title: "Searching the codebase" }
                    TaskContent {
                        TaskItem { "Searching for 'session' in src/" }
                        TaskItem {
                            "Found 3 matches in"
                            TaskItemFile { "src/auth/session.rs" }
                        }
                    }
                }
                Task { default_open: false,
                    TaskTrigger { title: "Reading configuration" }
                    TaskContent {
                        TaskItem {
                            "Read"
                            TaskItemFile { "Cargo.toml" }
                            TaskItemFile { "Dioxus.toml" }
                        }
                    }
                }
                Task {
                    TaskTrigger { title: "Custom trigger",
                        span { class: "text-sm font-medium", "Custom trigger row" }
                    }
                    TaskContent {
                        TaskItem { "Children replace the default icon and title." }
                    }
                }
                Task {
                    open: open(),
                    on_open_change: move |value: bool| open.set(value),
                    TaskTrigger { title: "Editing files (controlled)" }
                    TaskContent {
                        TaskItem {
                            "Updated"
                            TaskItemFile { "src/auth/session.rs" }
                        }
                    }
                }
            }
        }
    }
}
