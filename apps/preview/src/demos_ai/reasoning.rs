use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::reasoning::{Reasoning, ReasoningContent, ReasoningTrigger};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-col gap-4 rounded-lg border p-6";
const THOUGHTS: &str = "The user wants the build to run on Windows too.\nThe CI matrix only lists ubuntu and macos, so windows-latest has to be added.\nCheck whether the release job depends on a shell script before changing it.";

#[component]
pub fn ReasoningDemo() -> Element {
    let mut streaming = use_signal(|| false);
    let mut open = use_signal(|| false);
    let toggle_label = if streaming() {
        "Stop streaming"
    } else {
        "Start streaming"
    };

    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Reasoning" }
            div { class: BOX,
                Reasoning {
                    default_open: Some(false),
                    duration: Some(7),
                    ReasoningTrigger {}
                    ReasoningContent { "{THOUGHTS}" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Streaming" }
            div { class: BOX,
                div { class: "flex items-center gap-3",
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        onclick: move |_| streaming.set(!streaming()),
                        "{toggle_label}"
                    }
                }
                Reasoning {
                    is_streaming: streaming(),
                    duration: Some(12),
                    ReasoningTrigger {}
                    ReasoningContent { "{THOUGHTS}" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Controlled with custom trigger" }
            div { class: BOX,
                Reasoning {
                    open: Some(open()),
                    on_open_change: move |next: bool| open.set(next),
                    duration: Some(3),
                    ReasoningTrigger {
                        span { "Why this answer?" }
                    }
                    ReasoningContent { "{THOUGHTS}" }
                }
            }
        }
    }
}
