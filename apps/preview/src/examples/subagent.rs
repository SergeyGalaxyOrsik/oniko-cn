//! Subagent examples, ported from shadcn/ui `examples/base/subagent-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{CircleAlert, LoaderCircle};

use super::Example;
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::subagent::{Subagent, SubagentStep};
use crate::components::ui::trace::{Trace, TraceIcon, TraceLine, TraceStatus, TraceTitle};

#[component]
pub fn Examples() -> Element {
    rsx! {
        SubagentDemo {}
        SubagentRunning {}
        SubagentDetails {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

const QUESTION: &str = "Why does image optimization rerun on every deploy?";
const REPORT: &str = "The cache is keyed on the commit SHA, so every deploy misses it and optimizes all 1,240 images again.";
const ANSWER: &str = "Image optimization reruns because its cache is keyed on the commit SHA, which changes on every deploy. Key the cache on a hash of public/images instead, and unchanged images will be skipped.";
const COMMANDS: [(&str, &str); 3] = [
    (
        "grep -rn cacheKey scripts",
        "scripts/optimize-images.ts:14:  const cacheKey = process.env.COMMIT_SHA",
    ),
    (
        "sed -n 13,15p scripts/optimize-images.ts",
        "const cacheDir = \".next/cache/images\"\nconst cacheKey = process.env.COMMIT_SHA\nconst cached = await restoreCache(cacheDir, cacheKey)",
    ),
    ("ls public/images | wc -l", "1240"),
];

/// How far the simulated subagent has got: `shown` tool calls are visible and `done` of them have
/// output. Once `finished`, the report is in and the run is over.
#[derive(Clone, Copy, PartialEq)]
struct Run {
    shown: usize,
    done: usize,
    finished: bool,
}

const COMPLETE: Run = Run {
    shown: COMMANDS.len(),
    done: COMMANDS.len(),
    finished: true,
};

#[component]
fn SubagentRow(run: Run, streaming: bool) -> Element {
    let running = !run.finished && streaming;
    let status = if run.finished {
        TraceStatus::Complete
    } else if running {
        TraceStatus::Running
    } else {
        TraceStatus::Error
    };
    let verb = match status {
        TraceStatus::Running => "Running",
        TraceStatus::Error => "Failed to run",
        _ => "Ran",
    };
    let title = format!("{verb} the research subagent");

    if run.shown == 0 {
        return rsx! {
            TraceLine { status,
                TraceIcon {
                    if running {
                        LoaderCircle { class: "animate-spin" }
                    } else {
                        CircleAlert {}
                    }
                }
                TraceTitle { "{title}" }
            }
        };
    }

    rsx! {
        Subagent { title, status, default_open: true,
            Trace {
                for (index , (command , stdout)) in COMMANDS.into_iter().enumerate().take(run.shown) {
                    if index < run.done {
                        SubagentStep { key: "{command}", title: format!("Ran {command}"), output: stdout }
                    } else {
                        SubagentStep {
                            key: "{command}",
                            status: if running { TraceStatus::Running } else { TraceStatus::Error },
                            title: format!("{} {command}", if running { "Running" } else { "Failed to run" }),
                        }
                    }
                }
            }
            if run.finished {
                p { "{REPORT}" }
            }
        }
    }
}

#[component]
fn SubagentDemo() -> Element {
    let mut run = use_signal(|| Some(COMPLETE));
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
                        if let Some(run) = run() {
                            Trace {
                                SubagentRow { run, streaming: busy() }
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
                            run.set(Some(Run {
                                shown: 0,
                                done: 0,
                                finished: false,
                            }));
                            answer.set(String::new());
                            busy.set(true);
                            spawn(async move {
                                sleep(600).await;
                                for done in 0..COMMANDS.len() {
                                    run.set(Some(Run {
                                        shown: done + 1,
                                        done,
                                        finished: false,
                                    }));
                                    sleep(1000).await;
                                }
                                run.set(Some(COMPLETE));
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
fn SubagentRunning() -> Element {
    rsx! {
        Example { title: "Running", class: "h-auto",
            Trace { class: "max-w-sm",
                Subagent {
                    title: "Running the research agent",
                    status: TraceStatus::Running,
                    default_open: true,
                    Trace {
                        SubagentStep { title: "Ran grep -rn cacheKey scripts" }
                        SubagentStep {
                            title: "Running sed -n 13,15p scripts/optimize-images.ts",
                            status: TraceStatus::Running,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SubagentDetails() -> Element {
    rsx! {
        Example { title: "Details", class: "h-auto",
            Trace { class: "max-w-sm",
                Subagent {
                    title: "Ran the research agent",
                    duration: "48s",
                    default_open: true,
                    div { class: "flex flex-wrap gap-1.5",
                        Badge { variant: BadgeVariant::Secondary, "anthropic/claude-sonnet-4.5" }
                        Badge { variant: BadgeVariant::Outline, class: "font-mono", "runCommand" }
                    }
                    p { "Find out why image optimization reruns on every deploy." }
                    Trace {
                        SubagentStep { title: "Ran grep -rn cacheKey scripts" }
                        SubagentStep { title: "Ran sed -n 13,15p scripts/optimize-images.ts" }
                    }
                    p { "The cache is keyed on the commit SHA." }
                }
            }
        }
    }
}
