//! Checkpoint: a point in the conversation you can return to, drawn as a separator marker with a Restore button.
//!
//! Built from `Marker` and `Button`. Render it after the turn it marks. Without `on_restore` the button is hidden.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::button::{Button, ButtonSize, ButtonVariant};
use super::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn Checkpoint(
    label: String,
    on_restore: Option<EventHandler<()>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Marker { variant: MarkerVariant::Separator, attributes,
            MarkerContent { class: "inline-flex items-center gap-2",
                "{label}"
                if let Some(restore) = on_restore {
                    Button {
                        size: ButtonSize::Xs,
                        variant: ButtonVariant::Outline,
                        aria_label: "Restore to {label}",
                        onclick: move |_| restore.call(()),
                        "Restore"
                    }
                }
            }
        }
    }
}
