//! Checkpoint examples, ported from shadcn/ui `examples/base/checkpoint-*.tsx`.
//!
//! Upstream scripts the thread with `createChat` and `useChat`. Here local state and timers stand in for them.

use dioxus::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::marker::{Marker, MarkerContent, MarkerVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};

/// One scripted turn: what the person says, what the agent answers and the checkpoint it leaves.
struct Turn {
    user: &'static str,
    assistant: &'static str,
    checkpoint: Option<&'static str>,
}

const SCRIPT: [Turn; 2] = [
    Turn {
        user: "Rename the button component to action.",
        assistant: "Renamed Button to Action in components/ui/button.tsx.",
        checkpoint: Some("Button renamed"),
    },
    Turn {
        user: "Update the docs to match.",
        assistant: "Updated the Button page to say Action.",
        checkpoint: None,
    },
];

#[derive(Clone, PartialEq)]
struct ChatMessage {
    id: String,
    is_user: bool,
    text: String,
    checkpoint: Option<&'static str>,
}

fn transcript() -> Vec<ChatMessage> {
    SCRIPT
        .iter()
        .enumerate()
        .flat_map(|(index, turn)| {
            [
                ChatMessage {
                    id: format!("user-{index}"),
                    is_user: true,
                    text: turn.user.to_string(),
                    checkpoint: None,
                },
                ChatMessage {
                    id: format!("assistant-{index}"),
                    is_user: false,
                    text: turn.assistant.to_string(),
                    checkpoint: turn.checkpoint,
                },
            ]
        })
        .collect()
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms})); return true;"
    ))
    .await;
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        super::Example { title: "Demo", class: "h-auto", CheckpointDemo {} }
    }
}

#[component]
fn CheckpointDemo() -> Element {
    let mut messages = use_signal(transcript);
    let mut busy = use_signal(|| false);

    let next_turn = {
        let sent = messages
            .read()
            .iter()
            .filter(|message| message.is_user)
            .count();
        (sent < SCRIPT.len()).then_some(sent)
    };

    let mut restore = move |message_id: String| {
        let index = messages
            .read()
            .iter()
            .position(|message| message.id == message_id);
        if let Some(index) = index {
            messages.write().truncate(index + 1);
        }
    };

    let mut send = move |turn: usize| {
        busy.set(true);
        messages.write().push(ChatMessage {
            id: format!("user-{turn}"),
            is_user: true,
            text: SCRIPT[turn].user.to_string(),
            checkpoint: None,
        });
        spawn(async move {
            sleep(300).await;
            let id = format!("assistant-{turn}");
            messages.write().push(ChatMessage {
                id: id.clone(),
                is_user: false,
                text: String::new(),
                checkpoint: None,
            });
            for word in SCRIPT[turn].assistant.split_inclusive(' ') {
                sleep(20).await;
                if let Some(last) = messages.write().last_mut() {
                    last.text.push_str(word);
                }
            }
            if let Some(last) = messages.write().last_mut() {
                last.checkpoint = SCRIPT[turn].checkpoint;
            }
            busy.set(false);
        });
    };

    let count = messages.read().len();

    rsx! {
        div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
            for (index , message) in messages.read().iter().cloned().enumerate() {
                if message.is_user {
                    Message { key: "{message.id}", align: MessageAlign::End,
                        MessageContent {
                            Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                                BubbleContent { "{message.text}" }
                            }
                        }
                    }
                } else {
                    Message { key: "{message.id}",
                        MessageContent {
                            p { "{message.text}" }
                        }
                    }
                }
                if let Some(label) = message.checkpoint {
                    Marker { key: "{message.id}-checkpoint", variant: MarkerVariant::Separator,
                        MarkerContent { class: "inline-flex items-center gap-2",
                            "{label}"
                            if index < count - 1 {
                                Button {
                                    size: ButtonSize::Xs,
                                    variant: ButtonVariant::Outline,
                                    aria_label: "Restore to {label}",
                                    onclick: {
                                        let id = message.id.clone();
                                        move |_| restore(id.clone())
                                    },
                                    "Restore"
                                }
                            }
                        }
                    }
                }
            }
            if !busy() && let Some(turn) = next_turn {
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "mt-auto self-start",
                    onclick: move |_| send(turn),
                    "Continue"
                }
            }
        }
    }
}
