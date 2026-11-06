//! Prompt Input examples, ported from shadcn/ui `examples/base/prompt-input-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUp, ChevronDown, FileText, Plus, X};

use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentMedia,
    AttachmentSize, AttachmentTitle,
};
use crate::components::ui::button::ButtonVariant;
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuLabel,
    DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuTrigger,
};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupTextarea, input_group_button_variants,
};

struct Provider {
    name: &'static str,
    models: [(&'static str, &'static str); 2],
}

const PROVIDERS: [Provider; 2] = [
    Provider {
        name: "OpenAI",
        models: [("gpt-5", "GPT-5"), ("gpt-5-mini", "GPT-5 mini")],
    },
    Provider {
        name: "Anthropic",
        models: [
            ("claude-sonnet-4-5", "Claude Sonnet 4.5"),
            ("claude-haiku-4-5", "Claude Haiku 4.5"),
        ],
    },
];

const EFFORTS: [(&str, &str); 3] = [("low", "Low"), ("medium", "Medium"), ("high", "High")];

const FILE_INPUT_ID: &str = "prompt-input-demo-files";

fn model_name(id: &str) -> &'static str {
    PROVIDERS
        .iter()
        .flat_map(|provider| provider.models)
        .find(|(model, _)| *model == id)
        .map_or(PROVIDERS[0].models[0].1, |(_, name)| name)
}

fn effort_name(id: &str) -> &'static str {
    EFFORTS
        .iter()
        .find(|(effort, _)| *effort == id)
        .map_or(EFFORTS[1].1, |(_, name)| name)
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        super::Example { title: "Demo", class: "h-auto", PromptInputDemo {} }
    }
}

#[component]
fn PromptInputDemo() -> Element {
    let mut text = use_signal(String::new);
    let mut files = use_signal(Vec::<(usize, String)>::new);
    let mut next_id = use_signal(|| 0_usize);
    let mut model = use_signal(|| PROVIDERS[0].models[0].0.to_string());
    let mut effort = use_signal(|| EFFORTS[1].0.to_string());
    let can_send = !text.read().trim().is_empty();

    let mut submit = move || {
        if text.read().trim().is_empty() {
            return;
        }
        text.set(String::new());
        files.write().clear();
    };

    rsx! {
        div { class: "mx-auto w-full max-w-lg py-6",
            form {
                onsubmit: move |event| {
                    event.prevent_default();
                    submit();
                },
                input {
                    id: FILE_INPUT_ID,
                    r#type: "file",
                    multiple: true,
                    class: "sr-only",
                    tabindex: "-1",
                    onchange: move |event| {
                        for file in event.files() {
                            let id = next_id();
                            next_id.set(id + 1);
                            files.write().push((id, file.name()));
                        }
                        spawn(async move {
                            let _ = document::eval(
                                    &format!("document.getElementById('{FILE_INPUT_ID}').value = ''"),
                                )
                                .await;
                        });
                    },
                }
                InputGroup {
                    if !files.read().is_empty() {
                        InputGroupAddon { align: InputGroupAlign::BlockStart, class: "flex-wrap",
                            for (id , name) in files.read().iter().cloned() {
                                Attachment { key: "{id}", size: AttachmentSize::Sm,
                                    AttachmentMedia {
                                        FileText {}
                                    }
                                    AttachmentContent {
                                        AttachmentTitle { "{name}" }
                                    }
                                    AttachmentActions {
                                        AttachmentAction {
                                            aria_label: "Remove {name}",
                                            onclick: move |_| files.write().retain(|(file, _)| *file != id),
                                            X {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    InputGroupTextarea {
                        value: "{text}",
                        placeholder: "Ask the agent to change your code...",
                        class: "p-3",
                        oninput: move |event: FormEvent| text.set(event.value()),
                        onkeydown: move |event: KeyboardEvent| {
                            if event.key() == Key::Enter && !event.modifiers().shift() {
                                event.prevent_default();
                                submit();
                            }
                        },
                    }
                    InputGroupAddon { align: InputGroupAlign::BlockEnd,
                        InputGroupButton {
                            size: InputGroupButtonSize::IconSm,
                            variant: ButtonVariant::Outline,
                            class: "rounded-full",
                            aria_label: "Attach files",
                            onclick: move |_| {
                                spawn(async move {
                                    let _ = document::eval(
                                            &format!("document.getElementById('{FILE_INPUT_ID}').click()"),
                                        )
                                        .await;
                                });
                            },
                            Plus {}
                        }
                        DropdownMenu {
                            DropdownMenuTrigger {
                                aria_label: "Model: {model_name(&model.read())}",
                                class: input_group_button_variants(
                                    ButtonVariant::Outline,
                                    InputGroupButtonSize::Sm,
                                    Some("ml-auto"),
                                ),
                                "{model_name(&model.read())}"
                                ChevronDown {}
                            }
                            DropdownMenuContent { align: DropdownMenuAlign::Start, class: "w-48",
                                DropdownMenuRadioGroup {
                                    value: model(),
                                    on_value_change: move |value| model.set(value),
                                    for (index , provider) in PROVIDERS.iter().enumerate() {
                                        if index > 0 {
                                            DropdownMenuSeparator {}
                                        }
                                        DropdownMenuGroup {
                                            DropdownMenuLabel { "{provider.name}" }
                                            for (id , name) in provider.models {
                                                DropdownMenuRadioItem {
                                                    key: "{id}",
                                                    value: id,
                                                    close_on_click: true,
                                                    "{name}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        DropdownMenu {
                            DropdownMenuTrigger {
                                aria_label: "Effort: {effort_name(&effort.read())}",
                                class: input_group_button_variants(
                                    ButtonVariant::Outline,
                                    InputGroupButtonSize::Sm,
                                    None,
                                ),
                                span { class: "text-muted-foreground", "Effort" }
                                "{effort_name(&effort.read())}"
                                ChevronDown {}
                            }
                            DropdownMenuContent { align: DropdownMenuAlign::Start, class: "w-36",
                                DropdownMenuGroup {
                                    DropdownMenuLabel { "Effort" }
                                    DropdownMenuRadioGroup {
                                        value: effort(),
                                        on_value_change: move |value| effort.set(value),
                                        for (id , name) in EFFORTS {
                                            DropdownMenuRadioItem {
                                                key: "{id}",
                                                value: id,
                                                close_on_click: true,
                                                "{name}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        InputGroupButton {
                            size: InputGroupButtonSize::IconSm,
                            variant: ButtonVariant::Default,
                            class: "rounded-full",
                            disabled: !can_send,
                            aria_label: "Send",
                            onclick: move |_| submit(),
                            ArrowUp {}
                        }
                    }
                }
            }
        }
    }
}
