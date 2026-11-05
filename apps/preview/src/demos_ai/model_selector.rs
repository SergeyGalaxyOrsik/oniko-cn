use dioxus::prelude::*;

use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::model_selector::{
    ModelSelector, ModelSelectorContent, ModelSelectorEmpty, ModelSelectorGroup,
    ModelSelectorInput, ModelSelectorItem, ModelSelectorList, ModelSelectorLogo,
    ModelSelectorLogoGroup, ModelSelectorName, ModelSelectorSeparator, ModelSelectorShortcut,
    ModelSelectorTrigger,
};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-wrap items-center gap-3 rounded-lg border p-6";

/// A small colored dot standing in for a provider logo.
fn logo(color: &str) -> String {
    format!(
        "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 8 8'><circle cx='4' cy='4' r='4' fill='{color}'/></svg>"
    )
}

#[component]
pub fn ModelSelectorDemo() -> Element {
    let mut model = use_signal(|| "gpt-5".to_string());
    let current = model();
    let current_name = match current.as_str() {
        "claude-sonnet-4" => "Claude Sonnet 4",
        "gemini-2.5-pro" => "Gemini 2.5 Pro",
        "llama-4-maverick" => "Llama 4 Maverick",
        _ => "GPT-5",
    };

    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Model Selector" }
            div { class: BOX,
                ModelSelector {
                    ModelSelectorTrigger {
                        ModelSelectorLogo { src: logo("#10a37f"), alt: "OpenAI" }
                        ModelSelectorName { "{current_name}" }
                    }
                    ModelSelectorContent {
                        title: "Choose a model",
                        ModelSelectorInput { placeholder: "Search models..." }
                        ModelSelectorList {
                            ModelSelectorEmpty { "No models match your search." }
                            ModelSelectorGroup { heading: "OpenAI",
                                ModelSelectorItem {
                                    value: "gpt-5",
                                    keywords: vec!["openai".to_string(), "reasoning".to_string()],
                                    selected: current == "gpt-5",
                                    on_select: move |value: String| model.set(value),
                                    ModelSelectorLogo { src: logo("#10a37f"), alt: "" }
                                    ModelSelectorName { "GPT-5" }
                                    ModelSelectorShortcut { "⌘1" }
                                }
                                ModelSelectorItem {
                                    value: "gpt-5-mini",
                                    keywords: vec!["openai".to_string(), "fast".to_string()],
                                    selected: current == "gpt-5-mini",
                                    on_select: move |value: String| model.set(value),
                                    ModelSelectorLogo { src: logo("#10a37f"), alt: "" }
                                    ModelSelectorName { "GPT-5 mini" }
                                }
                            }
                            ModelSelectorSeparator {}
                            ModelSelectorGroup { heading: "Anthropic",
                                ModelSelectorItem {
                                    value: "claude-sonnet-4",
                                    keywords: vec!["anthropic".to_string(), "claude".to_string()],
                                    selected: current == "claude-sonnet-4",
                                    on_select: move |value: String| model.set(value),
                                    ModelSelectorLogo { src: logo("#d97757"), alt: "" }
                                    ModelSelectorName { "Claude Sonnet 4" }
                                    ModelSelectorShortcut { "⌘2" }
                                }
                                ModelSelectorItem {
                                    value: "claude-opus-4",
                                    keywords: vec!["anthropic".to_string(), "claude".to_string()],
                                    disabled: true,
                                    on_select: move |value: String| model.set(value),
                                    ModelSelectorLogo { src: logo("#d97757"), alt: "" }
                                    ModelSelectorName { "Claude Opus 4 (no access)" }
                                }
                            }
                            ModelSelectorSeparator {}
                            ModelSelectorGroup { heading: "Google",
                                ModelSelectorItem {
                                    value: "gemini-2.5-pro",
                                    keywords: vec!["google".to_string(), "gemini".to_string()],
                                    selected: current == "gemini-2.5-pro",
                                    on_select: move |value: String| model.set(value),
                                    ModelSelectorLogo { src: logo("#4285f4"), alt: "" }
                                    ModelSelectorName { "Gemini 2.5 Pro" }
                                }
                            }
                            ModelSelectorGroup { heading: "Open weights",
                                ModelSelectorItem {
                                    value: "llama-4-maverick",
                                    keywords: vec!["meta".to_string(), "llama".to_string()],
                                    selected: current == "llama-4-maverick",
                                    on_select: move |value: String| model.set(value),
                                    ModelSelectorLogoGroup {
                                        ModelSelectorLogo { src: logo("#0467df"), alt: "" }
                                        ModelSelectorLogo { src: logo("#7c3aed"), alt: "" }
                                    }
                                    ModelSelectorName { "Llama 4 Maverick" }
                                }
                            }
                        }
                    }
                }

                ModelSelector {
                    ModelSelectorTrigger {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        ModelSelectorLogo { src: logo("#10a37f"), alt: "" }
                        ModelSelectorName { "Small ghost trigger" }
                    }
                    ModelSelectorContent {
                        ModelSelectorInput {}
                        ModelSelectorList {
                            ModelSelectorItem {
                                value: "gpt-5-nano",
                                ModelSelectorName { "GPT-5 nano" }
                            }
                        }
                    }
                }
            }
        }
    }
}
