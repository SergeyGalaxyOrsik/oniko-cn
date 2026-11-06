//! Context examples, ported from shadcn/ui `examples/base/context-demo.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};
use crate::components::ui::progress::Progress;

const INPUT: u32 = 8420;
const OUTPUT: u32 = 1260;
const REASONING: u32 = 980;
const LIMIT: u32 = 128000;

fn format_tokens(value: u32) -> String {
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

#[component]
pub fn Examples() -> Element {
    rsx! {
        ContextDemo {}
    }
}

#[component]
fn ContextDemo() -> Element {
    let used = INPUT + OUTPUT + REASONING;
    let percent = (f64::from(used) / f64::from(LIMIT) * 100.0).round();
    let rows = [
        ("Input", INPUT),
        ("Output", OUTPUT),
        ("Reasoning", REASONING),
    ];

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex justify-center py-12",
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
                            span { class: "text-muted-foreground", "{format_tokens(used)} / {format_tokens(LIMIT)}" }
                        }
                        Progress { value: percent, aria_label: "Context used" }
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
}
