//! Live previews and usage snippets for the component pages. A component
//! without an entry here still gets a page with its install command and source.

use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::icons::{ArrowRightIcon, PlusIcon};

pub struct Demo {
    pub preview: fn() -> Element,
    pub usage: &'static str,
}

pub fn demo(name: &str) -> Option<Demo> {
    match name {
        "button" => Some(Demo {
            preview: button_preview,
            usage: r#"use crate::components::ui::button::{Button, ButtonVariant};

rsx! {
    Button { variant: ButtonVariant::Outline, onclick: move |_| save(), "Save" }
}"#,
        }),
        _ => None,
    }
}

fn button_preview() -> Element {
    rsx! {
        Button { "Default" }
        Button { variant: ButtonVariant::Secondary, "Secondary" }
        Button { variant: ButtonVariant::Outline, "Outline" }
        Button { variant: ButtonVariant::Ghost, "Ghost" }
        Button { variant: ButtonVariant::Destructive, "Destructive" }
        Button { variant: ButtonVariant::Link, "Link" }
        Button { size: ButtonSize::Icon, variant: ButtonVariant::Outline, aria_label: "Add", PlusIcon {} }
        Button { variant: ButtonVariant::Outline, "Continue" ArrowRightIcon {} }
    }
}
