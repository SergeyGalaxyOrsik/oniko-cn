//! Confirmation: asks for a yes or no before a tool runs, as an `Item` with Deny and Approve buttons.
//!
//! Built from `Item` and `Button`. Once the person answers, replace it with a `Trace` line.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::button::{Button, ButtonSize, ButtonVariant};
use super::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemSize,
    ItemTitle, ItemVariant,
};

#[component]
pub fn Confirmation(
    title: String,
    description: Option<String>,
    /// Shown in the media slot, for example a terminal icon.
    icon: Option<Element>,
    on_deny: Option<EventHandler<()>>,
    on_approve: Option<EventHandler<()>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Item { variant: ItemVariant::Outline, size: ItemSize::Sm, attributes,
            if let Some(icon) = icon {
                ItemMedia { variant: ItemMediaVariant::Icon, {icon} }
            }
            ItemContent {
                ItemTitle { "{title}" }
                if let Some(description) = description {
                    ItemDescription { "{description}" }
                }
            }
            ItemActions {
                Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        if let Some(handler) = on_deny {
                            handler.call(());
                        }
                    },
                    "Deny"
                }
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        if let Some(handler) = on_approve {
                            handler.call(());
                        }
                    },
                    "Approve"
                }
            }
        }
    }
}
