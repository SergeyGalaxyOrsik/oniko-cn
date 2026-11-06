//! Inline Citation: a source that sits in the sentence it supports and opens on hover.
//! A hover card on a secondary button that shows the title, an optional description and a link.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};

fn hostname(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    let host = rest.split(['/', '?', '#']).next().unwrap_or(rest);
    host.strip_prefix("www.").unwrap_or(host)
}

fn as_button((attributes, children): (Vec<Attribute>, Element)) -> Element {
    rsx! {
        button { r#type: "button", ..attributes, {children} }
    }
}

/// A button with the hostname of `url`. Hovering or focusing it shows `title` and a link to the source.
#[component]
pub fn InlineCitation(url: String, title: String, description: Option<String>) -> Element {
    let host = hostname(&url).to_string();

    rsx! {
        HoverCard {
            HoverCardTrigger {
                delay: 10,
                close_delay: 100,
                render: as_button,
                class: button_variants(ButtonVariant::Secondary, ButtonSize::Xs, None),
                "{host}"
            }
            HoverCardContent { class: "flex w-64 flex-col gap-1",
                div { class: "font-medium", "{title}" }
                if let Some(description) = description {
                    p { class: "text-muted-foreground", "{description}" }
                }
                a {
                    href: "{url}",
                    target: "_blank",
                    rel: "noreferrer",
                    class: "text-xs text-muted-foreground underline-offset-4 hover:underline",
                    "{host}"
                    span { class: "sr-only", " (opens in a new tab)" }
                }
            }
        }
    }
}
