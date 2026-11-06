//! Chain of Thought examples, ported from shadcn/ui `examples/base/chain-of-thought-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUpRight, Globe};

use super::Example;
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::chain_of_thought::{ChainOfThought, ChainOfThoughtStep};
use crate::components::ui::code_block::{CodeBlock, CodeBlockContent, CodeBlockSize};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::trace::{Trace, TraceIcon, TraceLine, TraceStatus, TraceTitle};

#[component]
pub fn Examples() -> Element {
    rsx! {
        ChainOfThoughtDemo {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    BuildLog,
    Web,
    Code,
}

const QUESTION: &str = "Why is the checkout deploy so slow?";
const BUILD_LOG: &str = "install    41s\ncompile    38s\nimages   3m 52s";
const RESULTS: [(&str, &str); 2] = [
    (
        "Image component | nextjs.org",
        "https://nextjs.org/docs/app/api-reference/components/image",
    ),
    (
        "Build cache | vercel.com",
        "https://vercel.com/docs/deployments/troubleshoot-a-build",
    ),
];
const FILES: [&str; 2] = ["next.config.ts", "scripts/optimize-images.ts"];
const ANSWER: &str = "Image optimization takes almost four minutes because its cache key includes the commit hash. Key it on the image directory instead.";
const STEPS: [Kind; 3] = [Kind::BuildLog, Kind::Web, Kind::Code];

#[component]
fn StepRow(kind: Kind, done: bool, streaming: bool) -> Element {
    let status = if done {
        TraceStatus::Complete
    } else if streaming {
        TraceStatus::Running
    } else {
        TraceStatus::Error
    };
    let verb = |running: &'static str, finished: &'static str, failed: &'static str| match status {
        TraceStatus::Running => running,
        TraceStatus::Error => failed,
        _ => finished,
    };

    match kind {
        Kind::Web => rsx! {
            ChainOfThoughtStep {
                status,
                title: format!(
                    "{} the web for next.js image cache key",
                    verb("Searching", "Searched", "Failed to search"),
                ),
                details: done.then(|| rsx! {
                    Trace {
                        for (title , url) in RESULTS {
                            a { key: "{url}", href: url, target: "_blank", rel: "noreferrer",
                                TraceLine {
                                    TraceIcon {
                                        Globe {}
                                    }
                                    TraceTitle {
                                        "{title}"
                                        span { class: "sr-only", " (opens in a new tab)" }
                                    }
                                    TraceIcon {
                                        ArrowUpRight {}
                                    }
                                }
                            }
                        }
                    }
                }),
            }
        },
        Kind::BuildLog => rsx! {
            ChainOfThoughtStep {
                status,
                default_open: true,
                title: format!(
                    "{} the build log for dpl_8Kx2",
                    verb("Reading", "Read", "Failed to read"),
                ),
                details: done.then(|| rsx! {
                    CodeBlock { size: CodeBlockSize::Sm,
                        CodeBlockContent { wrap: true, class: "max-h-64", code: BUILD_LOG }
                    }
                }),
            }
        },
        Kind::Code => rsx! {
            ChainOfThoughtStep {
                status,
                title: format!(
                    "{} the code for cacheKey",
                    verb("Searching", "Searched", "Failed to search"),
                ),
                details: done.then(|| rsx! {
                    div { class: "flex flex-wrap gap-1.5",
                        for file in FILES {
                            Badge { key: "{file}", variant: BadgeVariant::Secondary, class: "font-mono", "{file}" }
                        }
                    }
                }),
            }
        },
    }
}

#[component]
fn ChainOfThoughtDemo() -> Element {
    let mut steps = use_signal(|| STEPS.iter().map(|&kind| (kind, true)).collect::<Vec<_>>());
    let mut answer = use_signal(|| ANSWER.to_string());
    let mut busy = use_signal(|| false);

    let running = busy() && answer().is_empty();
    let count = steps().len();

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
                        if count > 0 {
                            ChainOfThought {
                                status: if running { TraceStatus::Running } else { TraceStatus::Complete },
                                title: if running { "Researching".to_string() } else { format!("Researched in {count} steps") },
                                for (index , (kind , done)) in steps().into_iter().enumerate() {
                                    StepRow { key: "{index}", kind, done, streaming: busy() }
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
                            steps.set(Vec::new());
                            answer.set(String::new());
                            busy.set(true);
                            spawn(async move {
                                for kind in STEPS {
                                    steps.write().push((kind, false));
                                    sleep(900).await;
                                    if let Some(step) = steps.write().last_mut() {
                                        step.1 = true;
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
