//! Textarea: a multi-line text field for forms and longer user input.
//!
//! Pass `value` and `oninput` to control it, or leave `value` out and let the browser keep the text.
//! Set `placeholder`, `rows`, `disabled`, `aria-invalid` and other HTML attributes directly on the component.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const TEXTAREA_BASE: &str = "cn-textarea flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";

/// Textarea classes for any element, e.g. the control inside an input group.
pub fn textarea_variants(class: Option<&str>) -> String {
    cn!(TEXTAREA_BASE, class)
}

#[component]
pub fn Textarea(
    onclick: Option<EventHandler<MouseEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = textarea)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = textarea_variants(take_class(&mut attributes).as_deref());

    rsx! {
        textarea {
            "data-slot": "textarea",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            oninput: move |event| {
                if let Some(handler) = oninput {
                    handler.call(event);
                }
            },
            onchange: move |event| {
                if let Some(handler) = onchange {
                    handler.call(event);
                }
            },
            onfocus: move |event| {
                if let Some(handler) = onfocus {
                    handler.call(event);
                }
            },
            onblur: move |event| {
                if let Some(handler) = onblur {
                    handler.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            ..attributes,
        }
    }
}
