use dioxus::prelude::*;

use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::message::{
    Message, MessageAction, MessageActions, MessageBranch, MessageBranchContent, MessageBranchNext,
    MessageBranchPage, MessageBranchPrevious, MessageBranchSelector, MessageContent, MessageFrom,
    MessageResponse, MessageToolbar,
};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-col gap-4 rounded-lg border p-6";

fn action_icon(path: &'static str) -> Element {
    rsx! {
        svg {
            class: "size-4",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: path }
        }
    }
}

#[component]
pub fn MessageDemo() -> Element {
    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Message" }
            div { class: BOX,
                Message { from: MessageFrom::User,
                    MessageContent {
                        MessageResponse { "Can you summarize the release notes for version 2.4?" }
                    }
                }

                Message { from: MessageFrom::Assistant,
                    MessageContent {
                        MessageResponse {
                            "Version 2.4 adds keyboard navigation to the file tree, fixes a crash in the model selector on small screens, and drops support for the legacy theme format."
                        }
                    }
                    MessageToolbar {
                        MessageActions {
                            MessageAction { tooltip: "Copy", label: "Copy message", {action_icon("M8 8h12v12H8z M4 16V4h12")} }
                            MessageAction { tooltip: "Helpful", {action_icon("M7 10v11 M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z")} }
                            MessageAction {
                                tooltip: "Regenerate",
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::IconSm,
                                {action_icon("M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8 M21 3v5h-5 M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16 M8 16H3v5")}
                            }
                        }
                    }
                }

                Message { from: MessageFrom::System,
                    MessageContent {
                        MessageResponse { "Context was trimmed to the last 20 messages." }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Message with alternative branches" }
            div { class: BOX,
                MessageBranch { default_branch: 0,
                    MessageBranchContent {
                        branches: vec![
                            rsx! {
                                Message { from: MessageFrom::Assistant,
                                    MessageContent {
                                        MessageResponse { "Use a HashMap keyed by the user id, then wrap it in an RwLock." }
                                    }
                                }
                            },
                            rsx! {
                                Message { from: MessageFrom::Assistant,
                                    MessageContent {
                                        MessageResponse { "Store the sessions in Redis with a 30 minute TTL and read them on each request." }
                                    }
                                }
                            },
                            rsx! {
                                Message { from: MessageFrom::Assistant,
                                    MessageContent {
                                        MessageResponse { "Keep the sessions in a signed cookie so the server holds no state at all." }
                                    }
                                }
                            },
                        ],
                    }
                    MessageToolbar {
                        MessageBranchSelector {
                            MessageBranchPrevious {}
                            MessageBranchPage {}
                            MessageBranchNext {}
                        }
                        MessageActions {
                            MessageAction { tooltip: "Copy", label: "Copy message", {action_icon("M8 8h12v12H8z M4 16V4h12")} }
                        }
                    }
                }
            }
        }
    }
}
