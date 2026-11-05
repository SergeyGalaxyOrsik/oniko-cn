//! Suggestion: a row of pill buttons that offer follow-up prompts, scrolling sideways when they do not fit.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

/// A horizontal row of `Suggestion` buttons that scrolls sideways when it overflows.
#[component]
pub fn Suggestions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex w-full flex-nowrap items-center gap-2 overflow-x-auto whitespace-nowrap [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "suggestions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A pill button that offers `suggestion`. `on_click` receives the suggestion text.
/// Pass `children` to replace the text shown in the pill.
#[component]
pub fn Suggestion(
    suggestion: String,
    on_click: Option<EventHandler<String>>,
    children: Option<Element>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let extra = cn!(
        "cursor-pointer rounded-full px-4 whitespace-nowrap",
        take_class(&mut attributes).as_deref()
    );
    let class = button_variants(ButtonVariant::Outline, ButtonSize::Sm, Some(extra.as_str()));
    let content = match children {
        Some(children) => children,
        None => rsx! { "{suggestion}" },
    };

    rsx! {
        button {
            "data-slot": "suggestion",
            r#type: "button",
            class: class,
            onclick: move |_| {
                if let Some(handler) = on_click {
                    handler.call(suggestion.clone());
                }
            },
            ..attributes,
            {content}
        }
    }
}
