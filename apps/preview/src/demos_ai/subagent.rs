use crate::components::ui::subagent::{
    Subagent, SubagentContent, SubagentHeader, SubagentTask, SubagentTool,
};
use crate::components::ui::trace::TraceStatus;
use dioxus::prelude::*;

#[component]
pub fn SubagentDemo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Subagent" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                Subagent { default_open: true,
                    SubagentHeader {
                        status: TraceStatus::Running,
                        title: rsx! { "Explore the auth module" },
                        description: rsx! { "4 steps" },
                    }
                    SubagentContent {
                        SubagentTask { "Find where session tokens are validated." }
                        div { class: "flex flex-wrap gap-1",
                            SubagentTool { "grep" }
                            SubagentTool { "read_file" }
                            SubagentTool { "claude-sonnet" }
                        }
                    }
                }
                Subagent {
                    SubagentHeader {
                        status: TraceStatus::Complete,
                        title: rsx! { "Summarize the changelog" },
                        description: rsx! { "12.4s" },
                    }
                    SubagentContent {
                        SubagentTask { "Read CHANGELOG.md and list the breaking changes." }
                        SubagentTool { "read_file" }
                        p { "Two breaking changes: the session cookie name and the token TTL." }
                    }
                }
                Subagent {
                    SubagentHeader {
                        status: TraceStatus::Error,
                        title: rsx! { "Run the migration check" },
                    }
                    SubagentContent {
                        SubagentTask { "Apply pending migrations to the staging copy." }
                        p { class: "text-destructive", "Connection refused on port 5432." }
                    }
                }
                Subagent {
                    SubagentHeader {
                        status: TraceStatus::Pending,
                        title: rsx! { "Review the pull request" },
                        icon: rsx! {
                            span { class: "text-xs font-medium", "PR" }
                        },
                    }
                }
                ControlledSubagent {}
            }
        }
    }
}

#[component]
fn ControlledSubagent() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Subagent {
            open: open(),
            on_open_change: move |value: bool| open.set(value),
            SubagentHeader {
                title: rsx! { "Review the diff (controlled)" },
                description: rsx! { if open() { "open" } else { "closed" } },
            }
            SubagentContent {
                SubagentTask { "Check the diff for missing error handling." }
            }
        }
    }
}
