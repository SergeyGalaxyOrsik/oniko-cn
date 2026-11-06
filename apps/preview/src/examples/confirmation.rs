//! Confirmation examples, ported from shadcn/ui `examples/base/confirmation-*.tsx`.
//!
//! Upstream scripts the thread with `createChat` and `useChat`. Here local state and timers stand in for them.

use dioxus::prelude::*;
use lucide_dioxus::{CircleCheck, LoaderCircle, Terminal};

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemSize,
    ItemTitle, ItemVariant,
};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

const USER_MESSAGE: &str = "Clean up the build folder.";
const REASONING: &str = "The build output lives in dist. Deleting it cannot be undone, so I should ask before I run the command.";
const COMMAND: &str = "rm -rf dist";

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms})); return true;"
    ))
    .await;
}

/// The tool part's state, as the AI SDK names them.
#[derive(Clone, Copy, PartialEq)]
enum ToolState {
    ApprovalRequested,
    Denied,
    Running,
    Done,
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        super::Example { title: "Demo", class: "h-auto", ConfirmationDemo {} }
    }
}

#[component]
fn ConfirmationDemo() -> Element {
    let mut sent = use_signal(|| false);
    let mut busy = use_signal(|| false);
    let mut reasoning = use_signal(|| None::<(String, bool)>);
    let mut tool = use_signal(|| None::<ToolState>);
    let mut reply = use_signal(String::new);

    let mut reset = move |_| {
        sent.set(false);
        reasoning.set(None);
        tool.set(None);
        reply.set(String::new());
    };

    let mut send = move |_| {
        sent.set(true);
        busy.set(true);
        spawn(async move {
            sleep(600).await;
            reasoning.set(Some((String::new(), true)));
            for word in REASONING.split_inclusive(' ') {
                sleep(20).await;
                if let Some((text, _)) = reasoning.write().as_mut() {
                    text.push_str(word);
                }
            }
            if let Some((_, streaming)) = reasoning.write().as_mut() {
                *streaming = false;
            }
            sleep(400).await;
            tool.set(Some(ToolState::ApprovalRequested));
            busy.set(false);
        });
    };

    let mut answer = move |approved: bool| {
        busy.set(true);
        spawn(async move {
            if approved {
                tool.set(Some(ToolState::Running));
                sleep(600).await;
                tool.set(Some(ToolState::Done));
            } else {
                tool.set(Some(ToolState::Denied));
            }
            sleep(600).await;
            let text = if approved {
                "Removed the dist folder."
            } else {
                "Okay, I left the dist folder in place."
            };
            for word in text.split_inclusive(' ') {
                sleep(20).await;
                reply.write().push_str(word);
            }
            busy.set(false);
        });
    };

    let has_reply = !reply.read().is_empty();

    rsx! {
        div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
            if sent() {
                Message { align: MessageAlign::End,
                    MessageContent {
                        Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                            BubbleContent { "{USER_MESSAGE}" }
                        }
                    }
                }
                Message {
                    MessageContent { class: "gap-4",
                        if let Some((text, running)) = reasoning() {
                            Trace {
                                ReasoningRow { text, running }
                            }
                        }
                        match tool() {
                            // A pending approval is a prompt, not activity, so it sits outside Trace.
                            Some(ToolState::ApprovalRequested) => rsx! {
                                Item { variant: ItemVariant::Outline, size: ItemSize::Sm,
                                    ItemMedia { variant: ItemMediaVariant::Icon,
                                        Terminal {}
                                    }
                                    ItemContent {
                                        ItemTitle { "Run {COMMAND}?" }
                                        ItemDescription { "The command runs in this project." }
                                    }
                                    ItemActions {
                                        Button {
                                            size: ButtonSize::Sm,
                                            variant: ButtonVariant::Outline,
                                            onclick: move |_| answer(false),
                                            "Deny"
                                        }
                                        Button { size: ButtonSize::Sm, onclick: move |_| answer(true), "Approve" }
                                    }
                                }
                            },
                            Some(state) => rsx! {
                                Trace {
                                    TraceLine { status: if state == ToolState::Running { TraceStatus::Running } else { TraceStatus::Complete },
                                        TraceIcon {
                                            Terminal {}
                                        }
                                        TraceTitle {
                                            match state {
                                                ToolState::Denied => "Denied",
                                                ToolState::Done => "Ran",
                                                _ => "Running",
                                            }
                                            " {COMMAND}"
                                        }
                                    }
                                }
                            },
                            None => rsx! {},
                        }
                        if has_reply {
                            p { "{reply}" }
                        }
                    }
                }
            }
            if !busy() {
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "mt-auto self-start",
                    onclick: move |event| {
                        if sent() { reset(event) } else { send(event) }
                    },
                    if sent() {
                        "Reset"
                    } else {
                        "Send message"
                    }
                }
            }
        }
    }
}

#[component]
fn ReasoningRow(text: String, running: bool) -> Element {
    let mut chosen = use_signal(|| None::<bool>);

    rsx! {
        TraceCollapsible {
            open: chosen().unwrap_or(running),
            on_open_change: move |next| chosen.set(Some(next)),
            TraceCollapsibleTrigger {
                TraceLine { status: if running { TraceStatus::Running } else { TraceStatus::Complete },
                    TraceIcon {
                        if running {
                            LoaderCircle { class: "animate-spin" }
                        } else {
                            CircleCheck {}
                        }
                    }
                    TraceTitle {
                        if running {
                            "Thinking"
                        } else {
                            "Thought"
                        }
                    }
                }
            }
            TraceCollapsibleContent { class: "whitespace-pre-wrap", "{text}" }
        }
    }
}
