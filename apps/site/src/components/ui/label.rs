//! Label: a caption for a form control. Use `r#for` to point it at the control's `id`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const LABEL_BASE: &str = "gap-2 text-xs/relaxed leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed";

/// Label classes for any element, e.g. a styled caption outside of [`Label`].
pub fn label_variants(class: Option<&str>) -> String {
    cn!(LABEL_BASE, class)
}

#[component]
pub fn Label(
    #[props(extends = GlobalAttributes, extends = label)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = label_variants(take_class(&mut attributes).as_deref());

    rsx! {
        label {
            "data-slot": "label",
            class: class,
            ..attributes,
            {children}
        }
    }
}
