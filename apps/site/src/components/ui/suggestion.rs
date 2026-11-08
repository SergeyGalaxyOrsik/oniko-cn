//! Suggestion: a row of buttons that send a prompt in one click, scrolling sideways when they do not fit.
//!
//! Built from `Button`. Put the row above the prompt input.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn Suggestions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex scroll-fade-x scrollbar-none gap-2 overflow-x-auto overscroll-x-contain",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { class: class, ..attributes, {children} }
    }
}

/// A button whose label is the prompt. `on_click` receives the label.
#[component]
pub fn Suggestion(
    suggestion: String,
    on_click: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("shrink-0", take_class(&mut attributes).as_deref());
    let prompt = suggestion.clone();

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            class: class,
            attributes,
            onclick: move |_| {
                if let Some(handler) = on_click {
                    handler.call(prompt.clone());
                }
            },
            "{suggestion}"
        }
    }
}
