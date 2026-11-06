//! Model Selector examples, ported from shadcn/ui `examples/base/model-selector-demo.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUp, ChevronDown, Plus};

use super::Example;
use crate::components::ui::button::ButtonVariant;
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuLabel,
    DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuTrigger,
};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupTextarea, input_group_button_variants,
};

struct Model {
    id: &'static str,
    name: &'static str,
}

struct Provider {
    name: &'static str,
    models: &'static [Model],
}

const PROVIDERS: &[Provider] = &[
    Provider {
        name: "OpenAI",
        models: &[
            Model {
                id: "gpt-5",
                name: "GPT-5",
            },
            Model {
                id: "gpt-5-mini",
                name: "GPT-5 mini",
            },
        ],
    },
    Provider {
        name: "Anthropic",
        models: &[
            Model {
                id: "claude-sonnet-4-5",
                name: "Claude Sonnet 4.5",
            },
            Model {
                id: "claude-haiku-4-5",
                name: "Claude Haiku 4.5",
            },
        ],
    },
];

const EFFORTS: &[Model] = &[
    Model {
        id: "low",
        name: "Low",
    },
    Model {
        id: "medium",
        name: "Medium",
    },
    Model {
        id: "high",
        name: "High",
    },
];

#[component]
pub fn Examples() -> Element {
    rsx! {
        ModelSelectorDemo {}
    }
}

#[component]
fn ModelSelectorDemo() -> Element {
    let mut text = use_signal(String::new);
    let mut model = use_signal(|| "gpt-5".to_string());
    let mut effort = use_signal(|| "medium".to_string());
    let current_model = PROVIDERS
        .iter()
        .flat_map(|provider| provider.models)
        .find(|item| item.id == model())
        .map_or("GPT-5", |item| item.name);
    let current_effort = EFFORTS
        .iter()
        .find(|item| item.id == effort())
        .map_or("Medium", |item| item.name);
    let can_send = !text().trim().is_empty();

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "mx-auto w-full max-w-lg py-6",
                form {
                    onsubmit: move |event| {
                        event.prevent_default();
                        if can_send {
                            text.set(String::new());
                        }
                    },
                    InputGroup {
                        InputGroupTextarea {
                            value: "{text}",
                            placeholder: "Ask the agent to change your code...",
                            class: "p-3",
                            oninput: move |event: FormEvent| text.set(event.value()),
                            onkeydown: move |event: KeyboardEvent| {
                                if event.key() == Key::Enter && !event.modifiers().shift() {
                                    event.prevent_default();
                                    if can_send {
                                        text.set(String::new());
                                    }
                                }
                            },
                        }
                        InputGroupAddon { align: InputGroupAlign::BlockEnd,
                            InputGroupButton {
                                r#type: "submit",
                                size: InputGroupButtonSize::IconSm,
                                variant: ButtonVariant::Outline,
                                class: "rounded-full",
                                disabled: can_send,
                                aria_label: "Attach",
                                Plus { size: 16 }
                            }
                            DropdownMenu {
                                DropdownMenuTrigger {
                                    class: input_group_button_variants(
                                        ButtonVariant::Outline,
                                        InputGroupButtonSize::Sm,
                                        Some("ml-auto"),
                                    ),
                                    "data-size": "sm",
                                    aria_label: "Model: {current_model}",
                                    "{current_model}"
                                    ChevronDown { size: 16 }
                                }
                                DropdownMenuContent { align: DropdownMenuAlign::Start, class: "w-48",
                                    DropdownMenuRadioGroup {
                                        value: model(),
                                        on_value_change: move |value| model.set(value),
                                        for (index , provider) in PROVIDERS.iter().enumerate() {
                                            Fragment { key: "{provider.name}",
                                                if index > 0 {
                                                    DropdownMenuSeparator {}
                                                }
                                                DropdownMenuGroup {
                                                    DropdownMenuLabel { "{provider.name}" }
                                                    for item in provider.models {
                                                        DropdownMenuRadioItem {
                                                            key: "{item.id}",
                                                            value: item.id,
                                                            close_on_click: true,
                                                            "{item.name}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            DropdownMenu {
                                DropdownMenuTrigger {
                                    class: input_group_button_variants(
                                        ButtonVariant::Outline,
                                        InputGroupButtonSize::Sm,
                                        None,
                                    ),
                                    "data-size": "sm",
                                    aria_label: "Effort: {current_effort}",
                                    span { class: "text-muted-foreground", "Effort" }
                                    "{current_effort}"
                                    ChevronDown { size: 16 }
                                }
                                DropdownMenuContent { align: DropdownMenuAlign::Start, class: "w-36",
                                    DropdownMenuGroup {
                                        DropdownMenuLabel { "Effort" }
                                        DropdownMenuRadioGroup {
                                            value: effort(),
                                            on_value_change: move |value| effort.set(value),
                                            for item in EFFORTS {
                                                DropdownMenuRadioItem {
                                                    key: "{item.id}",
                                                    value: item.id,
                                                    close_on_click: true,
                                                    "{item.name}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            InputGroupButton {
                                r#type: "submit",
                                size: InputGroupButtonSize::IconSm,
                                variant: ButtonVariant::Default,
                                class: "rounded-full",
                                disabled: !can_send,
                                aria_label: "Send",
                                ArrowUp { size: 16 }
                            }
                        }
                    }
                }
            }
        }
    }
}
