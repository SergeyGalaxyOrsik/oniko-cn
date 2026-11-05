//! MessageScroller: a chat transcript viewport that keeps the latest message in view,
//! follows streamed output while the reader is at the bottom, and jumps to anchored turns.
//!
//! Scroll behavior runs in a small script attached to the viewport with `document::eval`,
//! so the same code works in the browser and in the desktop webview. The viewport state
//! lives on its `data-*` attributes, which the script reads on every change.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Where the viewport sits when it first opens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageScrollerPosition {
    Start,
    #[default]
    End,
    /// The last `scroll_anchor` item, or the end when there is no anchor or it already fits.
    LastAnchor,
}

impl MessageScrollerPosition {
    fn name(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::LastAnchor => "last-anchor",
        }
    }
}

/// Which edge a `MessageScrollerButton` scrolls to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageScrollerTarget {
    #[default]
    End,
    Start,
}

impl MessageScrollerTarget {
    fn name(self) -> &'static str {
        match self {
            Self::End => "end",
            Self::Start => "start",
        }
    }

    fn action(self) -> &'static str {
        match self {
            Self::End => "el.dataset.following = \"true\"; el.scrollTop = el.scrollHeight;",
            Self::Start => "el.dataset.following = \"false\"; el.scrollTop = 0;",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MessageScrollerConfig {
    auto_scroll: bool,
    default_position: MessageScrollerPosition,
    peek: u32,
}

#[derive(Clone, Copy)]
struct MessageScrollerContext {
    viewport_id: Signal<String>,
    config: Signal<MessageScrollerConfig>,
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// Quotes a value as a JavaScript string literal.
fn js_string(value: &str) -> String {
    format!("{value:?}")
}

const VIEWPORT_SCRIPT: &str = r#"(() => {
  const el = document.getElementById("__ID__");
  if (!el) return;
  __ACTION__
})();"#;

const INSTALL_SCRIPT: &str = r#"(() => {
  const el = document.getElementById("__ID__");
  if (!el || el.dataset.scrollerReady === "true") return;
  el.dataset.scrollerReady = "true";
  el.dataset.following = "true";

  const peek = () => Number(el.dataset.scrollPeek || 0);
  const atEnd = () => el.scrollHeight - el.scrollTop - el.clientHeight < 24;
  const offsetOf = (node) =>
    node.getBoundingClientRect().top - el.getBoundingClientRect().top + el.scrollTop;
  const toEnd = () => {
    el.dataset.following = "true";
    el.scrollTop = el.scrollHeight;
  };
  const toAnchor = (node) => {
    el.dataset.following = "false";
    el.scrollTop = Math.max(0, offsetOf(node) - peek());
  };

  el.addEventListener("scroll", () => {
    el.dataset.following = atEnd() ? "true" : "false";
  }, { passive: true });

  new MutationObserver((mutations) => {
    let anchor = null;
    for (const mutation of mutations) {
      for (const node of mutation.addedNodes) {
        if (node.nodeType === 1 && node.dataset && node.dataset.scrollAnchor === "true") {
          anchor = node;
        }
      }
    }
    if (anchor) {
      toAnchor(anchor);
    } else if (el.dataset.autoScroll === "true" && el.dataset.following === "true") {
      el.scrollTop = el.scrollHeight;
    }
  }).observe(el, { childList: true, subtree: true, characterData: true });

  const position = el.dataset.defaultPosition;
  if (position === "start") {
    el.dataset.following = "false";
    el.scrollTop = 0;
  } else if (position === "last-anchor") {
    const anchors = el.querySelectorAll('[data-scroll-anchor="true"]');
    const last = anchors[anchors.length - 1];
    if (last && last.getBoundingClientRect().height > el.clientHeight) {
      toAnchor(last);
    } else {
      toEnd();
    }
  } else {
    toEnd();
  }
})();"#;

fn run_on_viewport(viewport_id: &str, action: &str) {
    let script = VIEWPORT_SCRIPT
        .replace("__ID__", viewport_id)
        .replace("__ACTION__", action);
    let _ = document::eval(&script);
}

/// Scroll controls for a `MessageScroller`, for use from application code.
#[derive(Clone, Copy)]
pub struct MessageScrollerHandle {
    context: MessageScrollerContext,
}

impl MessageScrollerHandle {
    /// Scrolls to the newest message and resumes following streamed output.
    pub fn scroll_to_end(self) {
        let id = self.context.viewport_id.read().clone();
        run_on_viewport(&id, MessageScrollerTarget::End.action());
    }

    /// Scrolls to the first message and stops following streamed output.
    pub fn scroll_to_start(self) {
        let id = self.context.viewport_id.read().clone();
        run_on_viewport(&id, MessageScrollerTarget::Start.action());
    }

    /// Scrolls to the item with this `message_id`, with the configured peek above it.
    pub fn scroll_to_message(self, message_id: &str) {
        let id = self.context.viewport_id.read().clone();
        let action = format!(
            "const target = Array.from(el.querySelectorAll(\"[data-message-id]\")).find((node) => node.dataset.messageId === {}); \
             if (target) {{ el.dataset.following = \"false\"; \
             el.scrollTop = Math.max(0, target.getBoundingClientRect().top - el.getBoundingClientRect().top + el.scrollTop - Number(el.dataset.scrollPeek || 0)); }}",
            js_string(message_id)
        );
        run_on_viewport(&id, &action);
    }
}

/// Returns scroll controls for the nearest `MessageScrollerProvider`.
pub fn use_message_scroller() -> MessageScrollerHandle {
    MessageScrollerHandle {
        context: use_context::<MessageScrollerContext>(),
    }
}

/// Holds the scroll settings and shares them with the parts through context.
#[component]
pub fn MessageScrollerProvider(
    #[props(default)] auto_scroll: bool,
    #[props(default)] default_scroll_position: MessageScrollerPosition,
    #[props(default)] scroll_previous_item_peek: u32,
    children: Element,
) -> Element {
    let viewport_id = use_signal(|| {
        format!(
            "message-scroller-{}",
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        )
    });
    let config = use_prop(MessageScrollerConfig {
        auto_scroll,
        default_position: default_scroll_position,
        peek: scroll_previous_item_peek,
    });
    use_context_provider(|| MessageScrollerContext {
        viewport_id,
        config,
    });

    rsx! {
        {children}
    }
}

/// The styled frame. Fills its parent, so place it in a height-constrained container.
#[component]
pub fn MessageScroller(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "relative flex h-full min-h-0 w-full flex-col",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "message-scroller",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The scrollable element. Keeps its scroll position when the content changes.
#[component]
pub fn MessageScrollerViewport(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<MessageScrollerContext>();
    let viewport_id = context.viewport_id.read().clone();
    let config = *context.config.read();
    let class = cn!(
        "min-h-0 flex-1 overflow-y-auto overscroll-contain",
        take_class(&mut attributes)
    );
    // The scroll script finds the viewport by this id, so a caller id must not replace it.
    attributes.retain(|attribute| attribute.name != "id");
    let auto_scroll = config.auto_scroll.to_string();
    let peek = config.peek.to_string();

    let install_id = viewport_id.clone();
    use_effect(move || {
        let script = INSTALL_SCRIPT.replace("__ID__", &install_id);
        let _ = document::eval(&script);
    });

    rsx! {
        div {
            "data-slot": "message-scroller-viewport",
            "data-auto-scroll": auto_scroll,
            "data-scroll-peek": peek,
            "data-default-position": config.default_position.name(),
            class: class,
            id: viewport_id,
            ..attributes,
            {children}
        }
    }
}

/// Holds the transcript rows and announces new ones to screen readers.
#[component]
pub fn MessageScrollerContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-message-scroller-content flex flex-col px-4 py-6",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "message-scroller-content",
            role: "log",
            aria_live: "polite",
            aria_relevant: "additions text",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Wraps one transcript row. `message_id` lets the row be found by `scroll_to_message`,
/// and `scroll_anchor` marks a turn that new content should move to the top of the viewport.
#[component]
pub fn MessageScrollerItem(
    #[props(default)] message_id: Option<String>,
    #[props(default)] scroll_anchor: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("flex w-full flex-col", take_class(&mut attributes));
    let anchor = scroll_anchor.to_string();

    rsx! {
        div {
            "data-slot": "message-scroller-item",
            "data-message-id": message_id,
            "data-scroll-anchor": anchor,
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A floating button that scrolls to the start or the end of the transcript.
#[component]
pub fn MessageScrollerButton(
    #[props(default)] target: MessageScrollerTarget,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<MessageScrollerContext>();
    let class = cn!(
        button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
        "absolute bottom-4 left-1/2 z-10 -translate-x-1/2 rounded-full",
        take_class(&mut attributes)
    );

    rsx! {
        button {
            "data-slot": "message-scroller-button",
            "data-target": target.name(),
            "type": "button",
            class: class,
            onclick: move |event| {
                let id = context.viewport_id.read().clone();
                run_on_viewport(&id, target.action());
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
