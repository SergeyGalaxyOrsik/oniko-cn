//! Reasoning examples, ported from shadcn/ui `examples/base/reasoning-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::CircleCheck;

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::reasoning::Reasoning;
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceDescription,
    TraceIcon, TraceLine, TraceTitle,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        ReasoningDemo {}
        ReasoningDuration {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

const QUESTION: &str = "Why is the checkout deploy so slow?";
const THINKING: &str = "The deploy log has three phases. Install takes 38s and next build takes 52s, both normal for this repo.\n\nImage optimization takes 3m 50s. It reruns on every deploy because its cache key includes the commit hash, so the cache never hits.";
const ANSWER: &str = "Image optimization is the bottleneck. Key its cache on the lockfile and the image directory instead of the commit hash, and the deploy should take under two minutes.";

#[component]
fn ReasoningDemo() -> Element {
    let mut thinking = use_signal(|| Some(THINKING.to_string()));
    let mut thinking_done = use_signal(|| true);
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
                        if let Some(text) = thinking() {
                            Trace {
                                Reasoning {
                                    text,
                                    running: busy() && !thinking_done(),
                                    duration: if busy() { None } else { Some("5s".to_string()) },
                                    default_open: if busy() { None } else { Some(true) },
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
                            thinking.set(None);
                            thinking_done.set(false);
                            answer.set(String::new());
                            busy.set(true);
                            spawn(async move {
                                thinking.set(Some(String::new()));
                                for word in THINKING.split_inclusive(' ') {
                                    sleep(40).await;
                                    if let Some(text) = thinking.write().as_mut() {
                                        text.push_str(word);
                                    }
                                }
                                thinking_done.set(true);
                                sleep(400).await;
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
fn ReasoningDuration() -> Element {
    rsx! {
        Example { title: "Duration", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceCollapsible {
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                CircleCheck {}
                            }
                            TraceTitle { "Thought" }
                            TraceDescription { "12s" }
                        }
                    }
                    TraceCollapsibleContent { class: "whitespace-pre-wrap",
                        "The build log shows install and compile finish in under a minute. Image optimization takes almost four, and it reruns on every deploy because the cache key includes the commit hash."
                    }
                }
            }
        }
    }
}
