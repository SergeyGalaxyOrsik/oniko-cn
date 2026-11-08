//! Usage snippets for the component pages. A component without one still
//! gets its examples, install command and source.

pub fn usage(name: &str) -> Option<&'static str> {
    match name {
        "button" => Some(
            r#"use crate::components::ui::button::{Button, ButtonVariant};

rsx! {
    Button { variant: ButtonVariant::Outline, onclick: move |_| save(), "Save" }
}"#,
        ),
        _ => None,
    }
}
