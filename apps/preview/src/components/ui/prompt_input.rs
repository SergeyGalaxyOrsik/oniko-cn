//! Prompt input: a message box with a textarea, header and footer rows, and a submit button.
//!
//! Enter sends the message and Shift+Enter adds a line break. While a reply is being
//! submitted or streamed, the submit button stops it instead. Attachments, speech input,
//! drag and drop and pasted files are not included: they need browser-only APIs or a file
//! pipeline the app owns.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::input_group::{InputGroup, InputGroupAddon, InputGroupAlign};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PromptInputMessage {
    pub text: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PromptInputStatus {
    #[default]
    Ready,
    Submitted,
    Streaming,
    Error,
}

impl PromptInputStatus {
    /// Value of the `data-status` attribute on the submit button.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Submitted => "submitted",
            Self::Streaming => "streaming",
            Self::Error => "error",
        }
    }

    /// A reply is in flight, so the submit button stops it instead of sending.
    fn is_busy(self) -> bool {
        matches!(self, Self::Submitted | Self::Streaming)
    }
}

#[derive(Clone, Copy)]
struct PromptInputContext {
    text: Signal<String>,
    textarea: Signal<Option<Rc<MountedData>>>,
    on_submit: Option<EventHandler<PromptInputMessage>>,
}

impl PromptInputContext {
    /// Sends the current text and clears the field. Blank text is ignored.
    fn submit(mut self) {
        let text = (self.text)();
        if text.trim().is_empty() {
            return;
        }
        self.text.set(String::new());
        if let Some(handler) = self.on_submit {
            handler.call(PromptInputMessage { text });
        }
    }
}

/// Moves focus to the textarea, as a click on the header or footer background does.
fn focus_textarea(context: PromptInputContext) {
    if let Some(input) = context.textarea.peek().clone() {
        spawn(async move {
            _ = input.set_focus(true).await;
        });
    }
}

#[component]
pub fn PromptInput(
    onsubmit: Option<EventHandler<PromptInputMessage>>,
    #[props(extends = GlobalAttributes, extends = form)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let text = use_signal(String::new);
    let textarea = use_signal(|| None::<Rc<MountedData>>);
    let context = use_context_provider(|| PromptInputContext {
        text,
        textarea,
        on_submit: onsubmit,
    });

    let mut attributes = attributes;
    let class = cn!("w-full", take_class(&mut attributes).as_deref());

    rsx! {
        form {
            "data-slot": "prompt-input",
            class: class,
            onsubmit: move |event: FormEvent| {
                event.prevent_default();
                context.submit();
            },
            ..attributes,
            InputGroup {
                class: "flex-wrap overflow-hidden rounded-xl border border-input bg-background shadow-xs focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50",
                {children}
            }
        }
    }
}

#[component]
pub fn PromptInputBody(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("contents", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "prompt-input-body",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PromptInputHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<PromptInputContext>();
    let mut attributes = attributes;
    let class = cn!("contents", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "prompt-input-header",
            class: class,
            onclick: move |_| focus_textarea(context),
            ..attributes,
            InputGroupAddon { align: InputGroupAlign::BlockStart, {children} }
        }
    }
}

#[component]
pub fn PromptInputFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<PromptInputContext>();
    let mut attributes = attributes;
    let class = cn!("contents", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "prompt-input-footer",
            class: class,
            onclick: move |_| focus_textarea(context),
            ..attributes,
            InputGroupAddon { align: InputGroupAlign::BlockEnd, {children} }
        }
    }
}

#[component]
pub fn PromptInputTools(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex min-w-0 items-center gap-1",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "prompt-input-tools",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The message field. Enter sends, Shift+Enter inserts a line break.
#[component]
pub fn PromptInputTextarea(
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = textarea)] attributes: Vec<Attribute>,
) -> Element {
    let context = use_context::<PromptInputContext>();
    let mut text = context.text;
    let mut textarea = context.textarea;
    let value = (context.text)();
    let mut attributes = attributes;
    let class = cn!(
        "rounded-none border-0 bg-transparent py-2 shadow-none ring-0 focus-visible:ring-0 aria-invalid:ring-0 dark:bg-transparent flex-1 min-w-0 resize-none field-sizing-content outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        textarea {
            "data-slot": "prompt-input-textarea",
            name: "message",
            rows: "1",
            placeholder: "What would you like to know?",
            class: class,
            value: "{value}",
            onmounted: move |event| textarea.set(Some(event.data())),
            oninput: move |event: FormEvent| text.set(event.value()),
            onkeydown: move |event: KeyboardEvent| {
                if let Some(handler) = onkeydown {
                    handler.call(event.clone());
                }
                if event.key() == Key::Enter && !event.modifiers().shift() && !event.is_composing() {
                    event.prevent_default();
                    context.submit();
                }
            },
            ..attributes,
        }
    }
}

/// A small ghost button for the footer tools row. Does not submit the form.
#[component]
pub fn PromptInputButton(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::Sm)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "prompt-input-button",
            r#type: "button",
            class: class,
            onclick: move |event: MouseEvent| {
                // Keeps the footer's click handler from moving focus to the textarea.
                event.stop_propagation();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// The submit button. Its icon follows `status`. While the status is `Submitted` or
/// `Streaming`, a click calls `onstop` instead of submitting.
#[component]
pub fn PromptInputSubmit(
    #[props(default)] status: PromptInputStatus,
    onstop: Option<EventHandler<()>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let variant = match status {
        PromptInputStatus::Error => ButtonVariant::Destructive,
        _ => ButtonVariant::Default,
    };
    let extra = cn!(
        "ml-auto rounded-full",
        take_class(&mut attributes).as_deref()
    );
    let class = button_variants(variant, ButtonSize::IconSm, Some(extra.as_str()));
    let label = if status.is_busy() {
        "Stop generating"
    } else {
        "Send message"
    };
    let icon = match status {
        PromptInputStatus::Ready => svg_icon(
            "size-4",
            rsx! {
                path { d: "m5 12 7-7 7 7" }
                path { d: "M12 19V5" }
            },
        ),
        PromptInputStatus::Submitted => svg_icon(
            "size-4 animate-spin",
            rsx! {
                path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
            },
        ),
        PromptInputStatus::Streaming => svg_icon(
            "size-4",
            rsx! {
                rect {
                    x: "6",
                    y: "6",
                    width: "12",
                    height: "12",
                    rx: "2",
                    fill: "currentColor",
                    stroke: "none",
                }
            },
        ),
        PromptInputStatus::Error => svg_icon(
            "size-4",
            rsx! {
                path { d: "M18 6 6 18" }
                path { d: "m6 6 12 12" }
            },
        ),
    };

    rsx! {
        button {
            "data-slot": "prompt-input-submit",
            "data-status": status.as_str(),
            r#type: "submit",
            aria_label: label,
            class: class,
            onclick: move |event: MouseEvent| {
                if status.is_busy() {
                    event.prevent_default();
                    if let Some(handler) = onstop {
                        handler.call(());
                    }
                }
            },
            ..attributes,
            {icon}
        }
    }
}

fn svg_icon(class: &'static str, shapes: Element) -> Element {
    rsx! {
        svg {
            "data-slot": "prompt-input-icon",
            "aria-hidden": "true",
            class: class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            {shapes}
        }
    }
}
