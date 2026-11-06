//! Chat: the thread of a conversation, a list of `Message` turns inside a `MessageScroller`.
//!
//! Built from `MessageScroller`, `Message` and `Bubble`. The person's turn is the scroll anchor, so a
//! new question settles near the top and the reply streams in under it. `auto_scroll` follows the
//! reply while the person stays at the live edge. Put a prompt input under it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use super::message::{Message, MessageAlign, MessageContent};
use super::message_scroller::{
    MessageScroller, MessageScrollerButton, MessageScrollerContent, MessageScrollerItem,
    MessageScrollerProvider, MessageScrollerViewport,
};

/// The scrolling thread. Wrap each turn in `ChatUserTurn` or `ChatAgentTurn`.
#[component]
pub fn Chat(
    #[props(default = true)] auto_scroll: bool,
    /// Classes for the content column, such as `gap-6 px-8`.
    content_class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        MessageScrollerProvider { auto_scroll,
            MessageScroller {
                MessageScrollerViewport {
                    MessageScrollerContent { class: content_class, {children} }
                }
                MessageScrollerButton {}
            }
        }
    }
}

/// The person's turn, in a muted bubble on the end side. It is the scroll anchor.
#[component]
pub fn ChatUserTurn(id: String, children: Element) -> Element {
    rsx! {
        MessageScrollerItem { message_id: id, scroll_anchor: true,
            Message { align: MessageAlign::End,
                MessageContent {
                    Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                        BubbleContent { {children} }
                    }
                }
            }
        }
    }
}

/// The agent's turn: plain content on the start side.
#[component]
pub fn ChatAgentTurn(id: String, class: Option<String>, children: Element) -> Element {
    rsx! {
        MessageScrollerItem { message_id: id,
            Message {
                MessageContent { class, {children} }
            }
        }
    }
}
