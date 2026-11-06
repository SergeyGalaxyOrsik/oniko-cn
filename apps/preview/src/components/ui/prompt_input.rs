//! Prompt Input: the field the person types into to message the agent, an `InputGroup` with a textarea,
//! an attach button and a send button.
//!
//! Built from `InputGroup`. Enter sends the message and Shift+Enter adds a new line. Pass `header` for
//! attachment chips above the text, and `children` for extra controls such as model menus between the
//! attach and send buttons; give the first of them `ml-auto` so they sit next to the send button.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::button::ButtonVariant;
use super::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupTextarea,
};

#[component]
pub fn PromptInput(
    /// The text in the field. The block writes to it as the person types; clear it in `on_submit`.
    text: Signal<String>,
    #[props(default = "Ask the agent to change your code...".to_string())] placeholder: String,
    /// Called with the trimmed text when the person presses Enter or Send. Blank text is not sent.
    on_submit: EventHandler<String>,
    /// Called by the attach button. The button is hidden without it.
    on_attach: Option<EventHandler<()>>,
    /// Content of a `block-start` addon, such as attachment chips.
    header: Option<Element>,
    children: Option<Element>,
    #[props(extends = GlobalAttributes, extends = form)] attributes: Vec<Attribute>,
) -> Element {
    let mut text = text;
    let can_send = !text.read().trim().is_empty();
    let has_children = children.is_some();
    let send = move || {
        let value = text.peek().trim().to_string();
        if !value.is_empty() {
            on_submit.call(value);
        }
    };

    rsx! {
        form {
            onsubmit: move |event: FormEvent| {
                event.prevent_default();
                send();
            },
            ..attributes,
            InputGroup {
                if let Some(header) = header {
                    InputGroupAddon { align: InputGroupAlign::BlockStart, class: "flex-wrap", {header} }
                }
                InputGroupTextarea {
                    value: "{text}",
                    placeholder: placeholder,
                    class: "p-3",
                    oninput: move |event: FormEvent| text.set(event.value()),
                    onkeydown: move |event: KeyboardEvent| {
                        if event.key() == Key::Enter && !event.modifiers().shift() {
                            event.prevent_default();
                            send();
                        }
                    },
                }
                InputGroupAddon { align: InputGroupAlign::BlockEnd,
                    if let Some(attach) = on_attach {
                        InputGroupButton {
                            size: InputGroupButtonSize::IconSm,
                            variant: ButtonVariant::Outline,
                            class: "rounded-full",
                            aria_label: "Attach files",
                            onclick: move |_| attach.call(()),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M5 12h14" }
                                path { d: "M12 5v14" }
                            }
                        }
                    }
                    if let Some(children) = children {
                        {children}
                    }
                    InputGroupButton {
                        size: InputGroupButtonSize::IconSm,
                        variant: ButtonVariant::Default,
                        class: if has_children { "rounded-full" } else { "ml-auto rounded-full" },
                        disabled: !can_send,
                        aria_label: "Send",
                        onclick: move |_| send(),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m5 12 7-7 7 7" }
                            path { d: "M12 19V5" }
                        }
                    }
                }
            }
        }
    }
}
