//! Textarea examples, ported from shadcn/ui `examples/base/textarea-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::Button;
use crate::components::ui::field::{Field, FieldDescription, FieldLabel};
use crate::components::ui::textarea::Textarea;

#[component]
pub fn Examples() -> Element {
    rsx! {
        TextareaDemo {}
        TextareaField {}
        TextareaDisabled {}
        TextareaInvalid {}
        TextareaButton {}
        TextareaRtl {}
    }
}

#[component]
fn TextareaDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "*:max-w-xs",
            Textarea { placeholder: "Type your message here." }
        }
    }
}

#[component]
fn TextareaField() -> Element {
    rsx! {
        Example { title: "Field", class: "*:max-w-xs",
            Field {
                FieldLabel { r#for: "textarea-message", "Message" }
                FieldDescription { "Enter your message below." }
                Textarea { id: "textarea-message", placeholder: "Type your message here." }
            }
        }
    }
}

#[component]
fn TextareaDisabled() -> Element {
    rsx! {
        Example { title: "Disabled", class: "*:max-w-xs",
            Field { "data-disabled": "true",
                FieldLabel { r#for: "textarea-disabled", "Message" }
                Textarea {
                    id: "textarea-disabled",
                    placeholder: "Type your message here.",
                    disabled: true,
                }
            }
        }
    }
}

#[component]
fn TextareaInvalid() -> Element {
    rsx! {
        Example { title: "Invalid", class: "*:max-w-xs",
            Field { "data-invalid": "true",
                FieldLabel { r#for: "textarea-invalid", "Message" }
                Textarea {
                    id: "textarea-invalid",
                    placeholder: "Type your message here.",
                    "aria-invalid": "true",
                }
                FieldDescription { "Please enter a valid message." }
            }
        }
    }
}

#[component]
fn TextareaButton() -> Element {
    rsx! {
        Example { title: "Button", class: "*:max-w-xs",
            div { class: "grid w-full gap-2",
                Textarea { placeholder: "Type your message here." }
                Button { "Send message" }
            }
        }
    }
}

#[component]
fn TextareaRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Field { class: "w-full max-w-xs", dir: "rtl",
                FieldLabel { r#for: "feedback", dir: "rtl", "التعليقات" }
                Textarea {
                    id: "feedback",
                    placeholder: "تعليقاتك تساعدنا على التحسين...",
                    dir: "rtl",
                    rows: "4",
                }
                FieldDescription { dir: "rtl", "شاركنا أفكارك حول خدمتنا." }
            }
        }
    }
}
