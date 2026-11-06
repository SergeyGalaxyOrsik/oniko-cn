//! Context: how much of the model's context window the conversation has used.
//! A hover card on an outline button that shows the share used and a progress bar, with optional rows
//! for the breakdown, such as input and output tokens.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};
use super::progress::Progress;

/// Shortens a token count: `980` stays, `8420` becomes `8.4k`.
pub fn format_tokens(value: u32) -> String {
    if value < 1000 {
        return value.to_string();
    }
    let thousands = format!("{:.1}", f64::from(value) / 1000.0);
    format!("{}k", thousands.trim_end_matches(".0"))
}

fn as_button((attributes, children): (Vec<Attribute>, Element)) -> Element {
    rsx! {
        button { r#type: "button", ..attributes, {children} }
    }
}

/// `used` of `limit` tokens. `rows` are label and token pairs listed under the progress bar.
#[component]
pub fn Context(used: u32, limit: u32, #[props(default)] rows: Vec<(String, u32)>) -> Element {
    let percent = if limit == 0 {
        0.0
    } else {
        (f64::from(used) / f64::from(limit) * 100.0).round()
    };

    rsx! {
        HoverCard {
            HoverCardTrigger {
                delay: 10,
                close_delay: 100,
                render: as_button,
                class: button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
                "{percent}% context"
            }
            HoverCardContent { class: "flex w-64 flex-col gap-3",
                div { class: "flex items-center justify-between gap-3 text-sm",
                    span { class: "font-medium", "Context" }
                    span { class: "text-muted-foreground", "{format_tokens(used)} / {format_tokens(limit)}" }
                }
                Progress { value: percent, aria_label: "Context used" }
                if !rows.is_empty() {
                    div { class: "flex flex-col gap-1 text-sm",
                        for (label , value) in rows {
                            div { key: "{label}", class: "flex justify-between gap-3",
                                span { class: "text-muted-foreground", "{label}" }
                                span { "{format_tokens(value)}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
