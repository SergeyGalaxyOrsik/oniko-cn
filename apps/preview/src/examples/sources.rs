//! Sources examples, ported from shadcn/ui `examples/base/sources-demo.tsx`.
//! The AI SDK stream is simulated with signals and timers.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUpRight, Globe};

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceTitle,
};

const QUESTION: &str = "How do I start an Astro project with pnpm?";
const ANSWER: &str = "Run pnpm create astro@latest and follow the prompts. It scaffolds the project and installs the dependencies.";
const SOURCES: [(&str, &str); 4] = [
    (
        "https://docs.astro.build/en/install-and-setup/",
        "Install Astro | docs.astro.build",
    ),
    (
        "https://www.npmjs.com/package/create-astro",
        "create-astro | npmjs.com",
    ),
    (
        "https://docs.astro.build/en/install-and-setup/",
        "Install Astro | docs.astro.build",
    ),
    ("https://pnpm.io/cli/create", "pnpm create | pnpm.io"),
];

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        SourcesDemo {}
    }
}

#[component]
fn SourcesList(sources: Vec<(&'static str, &'static str)>) -> Element {
    rsx! {
        Trace {
            TraceCollapsible { default_open: true,
                TraceCollapsibleTrigger {
                    TraceLine {
                        TraceIcon {
                            Globe {}
                        }
                        TraceTitle { "Used {sources.len()} sources" }
                    }
                }
                TraceCollapsibleContent {
                    Trace {
                        for (url , title) in sources {
                            a {
                                key: "{url}",
                                href: "{url}",
                                target: "_blank",
                                rel: "noreferrer",
                                TraceLine {
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
                }
            }
        }
    }
}

#[component]
fn SourcesDemo() -> Element {
    let mut asked = use_signal(|| true);
    let mut answered = use_signal(|| true);
    let mut received = use_signal(|| SOURCES.len());
    let mut busy = use_signal(|| false);

    let mut sources: Vec<(&'static str, &'static str)> = Vec::new();
    for source in SOURCES.iter().take(received()) {
        if !sources.iter().any(|(url, _)| *url == source.0) {
            sources.push(*source);
        }
    }

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
                if answered() {
                    Message {
                        MessageContent { class: "gap-4",
                            p { "{ANSWER}" }
                            if !sources.is_empty() {
                                SourcesList { sources }
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
                            answered.set(false);
                            received.set(0);
                            busy.set(true);
                            spawn(async move {
                                sleep(200).await;
                                asked.set(true);
                                sleep(600).await;
                                answered.set(true);
                                for _ in 0..SOURCES.len() {
                                    sleep(300).await;
                                    received += 1;
                                }
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
