use crate::components::ui::suggestion::{Suggestion, Suggestions};
use dioxus::prelude::*;

#[component]
pub fn SuggestionDemo() -> Element {
    let mut picked = use_signal(|| None::<String>);
    let picked_text = picked().unwrap_or_else(|| "nothing yet".to_string());

    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Suggestion" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                Suggestions {
                    Suggestion {
                        suggestion: "Summarize this page",
                        on_click: move |text: String| picked.set(Some(text)),
                    }
                    Suggestion {
                        suggestion: "Explain this error",
                        on_click: move |text: String| picked.set(Some(text)),
                    }
                    Suggestion {
                        suggestion: "Write unit tests",
                        on_click: move |text: String| picked.set(Some(text)),
                    }
                    Suggestion {
                        suggestion: "Refactor for readability",
                        on_click: move |text: String| picked.set(Some(text)),
                    }
                    Suggestion {
                        suggestion: "Add error handling",
                        on_click: move |text: String| picked.set(Some(text)),
                    }
                    Suggestion {
                        suggestion: "Translate to Russian",
                        on_click: move |text: String| picked.set(Some(text)),
                    }
                    Suggestion {
                        suggestion: "Show the diff",
                        on_click: move |text: String| picked.set(Some(text)),
                    }
                    Suggestion {
                        suggestion: "Explain like I am five",
                        on_click: move |text: String| picked.set(Some(text)),
                    }
                }
                Suggestions {
                    Suggestion { suggestion: "Open the docs",
                        span { class: "font-medium", "Docs" }
                        span { class: "text-muted-foreground", " for this API" }
                    }
                    Suggestion { suggestion: "Disabled", disabled: true }
                }
                p { class: "text-sm text-muted-foreground", "Last clicked: {picked_text}" }
            }
        }
    }
}
