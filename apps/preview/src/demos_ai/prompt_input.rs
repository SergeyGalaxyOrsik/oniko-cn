use dioxus::prelude::*;

use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::prompt_input::{
    PromptInput, PromptInputBody, PromptInputButton, PromptInputFooter, PromptInputHeader,
    PromptInputMessage, PromptInputStatus, PromptInputSubmit, PromptInputTextarea,
    PromptInputTools,
};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-col gap-4 rounded-lg border p-6";

#[component]
pub fn PromptInputDemo() -> Element {
    let mut status = use_signal(PromptInputStatus::default);
    let mut last_message = use_signal(String::new);
    let message = last_message();

    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Prompt input" }
            div { class: BOX,
                PromptInput {
                    onsubmit: move |submitted: PromptInputMessage| {
                        last_message.set(submitted.text);
                        status.set(PromptInputStatus::Streaming);
                    },
                    PromptInputHeader {
                        span { class: "text-xs text-muted-foreground", "Replying in the Dioxus thread" }
                    }
                    PromptInputBody {
                        PromptInputTextarea {}
                    }
                    PromptInputFooter {
                        PromptInputTools {
                            PromptInputButton { "Search" }
                            PromptInputButton {
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Sm,
                                "Think"
                            }
                        }
                        PromptInputSubmit {
                            status: status(),
                            onstop: move |_| status.set(PromptInputStatus::Ready),
                        }
                    }
                }
                if !message.is_empty() {
                    p { class: "text-sm text-muted-foreground", "Last message: {message}" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Submit button states" }
            div { class: "flex flex-wrap items-center gap-6 rounded-lg border p-6",
                for (label, state) in [
                    ("Ready", PromptInputStatus::Ready),
                    ("Submitted", PromptInputStatus::Submitted),
                    ("Streaming", PromptInputStatus::Streaming),
                    ("Error", PromptInputStatus::Error),
                ] {
                    div { class: "flex flex-col items-center gap-2",
                        PromptInputSubmit { status: state }
                        span { class: "text-xs text-muted-foreground", "{label}" }
                    }
                }
            }
        }
    }
}
