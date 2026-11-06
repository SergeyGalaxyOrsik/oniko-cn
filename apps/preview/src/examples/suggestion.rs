//! Suggestion examples, ported from shadcn/ui `examples/base/suggestion-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUp, Plus};

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupTextarea,
};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};

const SUGGESTIONS: [&str; 6] = [
    "Add a dark mode toggle",
    "Fix the failing build",
    "Write tests for the form",
    "Upgrade to Next.js 16",
    "Add a loading state",
    "Find unused dependencies",
];

fn reply_for(prompt: &str) -> &'static str {
    match prompt {
        "Add a dark mode toggle" => {
            "Added a ModeToggle to the site header. It calls setTheme from next-themes."
        }
        "Fix the failing build" => {
            "The build failed on a missing import in app/page.tsx. I added it and the build passes."
        }
        "Write tests for the form" => {
            "Added four tests in signup-form.test.tsx. They cover the empty, invalid and valid states."
        }
        "Upgrade to Next.js 16" => {
            "Ran the codemod and bumped next to 16. Two routes needed async params, so I updated them."
        }
        "Add a loading state" => {
            "Added a loading.tsx to the dashboard route. It shows skeletons while the data loads."
        }
        "Find unused dependencies" => {
            "Three packages are not imported anywhere: lodash, moment and classnames. Should I remove them?"
        }
        _ => "This demo only knows the suggestions. Pick one.",
    }
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        super::Example { title: "Demo", class: "h-auto", SuggestionDemo {} }
    }
}

#[component]
fn SuggestionDemo() -> Element {
    let mut text = use_signal(String::new);
    let mut turn = use_signal(|| None::<(String, &'static str)>);
    let can_send = !text.read().trim().is_empty();

    let mut send = move |prompt: String| {
        let reply = reply_for(&prompt);
        turn.set(Some((prompt, reply)));
        text.set(String::new());
    };

    rsx! {
        div { class: "mx-auto flex w-full max-w-lg flex-col gap-6 py-6 text-sm",
            if let Some((prompt, reply)) = turn() {
                div { class: "flex flex-col gap-4",
                    Message { align: MessageAlign::End,
                        MessageContent {
                            Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                                BubbleContent { "{prompt}" }
                            }
                        }
                    }
                    Message {
                        MessageContent {
                            p { "{reply}" }
                        }
                    }
                }
            }
            div { class: "flex flex-col gap-2",
                div { class: "flex scroll-fade-x scrollbar-none gap-2 overflow-x-auto overscroll-x-contain",
                    for suggestion in SUGGESTIONS {
                        Button {
                            key: "{suggestion}",
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            class: "shrink-0",
                            onclick: move |_| send(suggestion.to_string()),
                            "{suggestion}"
                        }
                    }
                }
                form {
                    onsubmit: move |event| {
                        event.prevent_default();
                        if can_send {
                            send(text.read().trim().to_string());
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
                                        send(text.read().trim().to_string());
                                    }
                                }
                            },
                        }
                        InputGroupAddon { align: InputGroupAlign::BlockEnd,
                            InputGroupButton {
                                size: InputGroupButtonSize::IconSm,
                                variant: ButtonVariant::Outline,
                                class: "rounded-full",
                                aria_label: "Attach files",
                                Plus {}
                            }
                            InputGroupButton {
                                size: InputGroupButtonSize::IconSm,
                                variant: ButtonVariant::Default,
                                class: "ml-auto rounded-full",
                                disabled: !can_send,
                                aria_label: "Send",
                                onclick: move |_| {
                                    if can_send {
                                        send(text.read().trim().to_string());
                                    }
                                },
                                ArrowUp {}
                            }
                        }
                    }
                }
            }
        }
    }
}
