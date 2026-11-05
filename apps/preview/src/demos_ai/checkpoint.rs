use dioxus::prelude::*;

use crate::components::ui::button::ButtonVariant;
use crate::components::ui::checkpoint::{Checkpoint, CheckpointIcon, CheckpointTrigger};

#[component]
pub fn CheckpointDemo() -> Element {
    let mut restored = use_signal(|| None::<&'static str>);

    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Checkpoint" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                p { class: "text-sm", "Refactored the parser into three functions." }
                Checkpoint {
                    CheckpointIcon {}
                    CheckpointTrigger {
                        tooltip: "Restore the conversation to this point",
                        onclick: move |_| restored.set(Some("Restored to the first checkpoint.")),
                        "Restore"
                    }
                }
                p { class: "text-sm", "Added tests for the empty-input case." }
                Checkpoint {
                    CheckpointIcon { "v2" }
                    CheckpointTrigger {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| restored.set(Some("Restored to the second checkpoint.")),
                        "Restore"
                    }
                }
                if let Some(message) = restored() {
                    p { class: "text-xs text-muted-foreground", "{message}" }
                }
            }
        }
    }
}
