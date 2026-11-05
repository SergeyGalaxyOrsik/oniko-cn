//! Reasoning: a collapsible panel that shows a model's thinking. It opens while the model streams
//! and closes about a second after streaming ends, once. The content is plain text, not markdown.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

const AUTO_CLOSE_SCRIPT: &str = "await new Promise((resolve) => setTimeout(resolve, 1000))";

#[derive(Clone, Copy)]
struct ReasoningContext {
    is_streaming: Signal<bool>,
    controlled: Signal<Option<bool>>,
    internal: Signal<bool>,
    duration: Signal<Option<u32>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl ReasoningContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn set_open(mut self, next: bool) {
        if self.controlled.peek().is_none() {
            self.internal.set(next);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(next);
        }
    }
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn thinking_label(is_streaming: bool, duration: Option<u32>) -> String {
    if is_streaming || duration == Some(0) {
        "Thinking...".to_string()
    } else if let Some(seconds) = duration {
        format!("Thought for {seconds} seconds")
    } else {
        "Thought for a few seconds".to_string()
    }
}

/// Root of the panel. Opens by itself while `is_streaming` is set, unless `default_open` is `false`.
#[component]
pub fn Reasoning(
    /// Whether the model is still producing the thinking text.
    #[props(default)]
    is_streaming: bool,
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled. Defaults to `is_streaming`.
    default_open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
    /// Seconds spent thinking, shown in the trigger once streaming has ended.
    duration: Option<u32>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("mb-4 w-full", take_class(&mut attributes).as_deref());

    let streaming = use_prop(is_streaming);
    let explicitly_closed = use_prop(default_open == Some(false));
    let controlled = use_prop(open);
    let duration = use_prop(duration);
    let internal = use_signal(|| default_open.unwrap_or(is_streaming));
    let mut ever_streamed = use_signal(|| is_streaming);
    let mut has_auto_closed = use_signal(|| false);

    let ctx = use_context_provider(|| ReasoningContext {
        is_streaming: streaming,
        controlled,
        internal,
        duration,
        on_open_change,
    });

    use_effect(move || {
        if streaming() && !*ever_streamed.peek() {
            ever_streamed.set(true);
        }
    });

    use_effect(move || {
        if streaming() && !ctx.is_open() && !explicitly_closed() {
            ctx.set_open(true);
        }
    });

    use_effect(move || {
        if streaming() || !ever_streamed() || !ctx.is_open() || has_auto_closed() {
            return;
        }
        spawn(async move {
            let _ = document::eval(AUTO_CLOSE_SCRIPT).await;
            // Streaming may have restarted, or the panel may have been closed, while waiting.
            if *streaming.peek() || *has_auto_closed.peek() || !ctx.is_open() {
                return;
            }
            has_auto_closed.set(true);
            ctx.set_open(false);
        });
    });

    rsx! {
        div {
            "data-slot": "reasoning",
            class: class,
            ..attributes,
            Collapsible {
                open: Some(ctx.is_open()),
                on_open_change: move |next: bool| ctx.set_open(next),
                {children}
            }
        }
    }
}

/// The clickable header. Shows a brain icon, the thinking label and a chevron.
/// `children` replaces that default content.
#[component]
pub fn ReasoningTrigger(
    children: Option<Element>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<ReasoningContext>();
    let mut attributes = attributes;
    let class = cn!("flex w-full", take_class(&mut attributes).as_deref());
    let open = ctx.is_open();
    let label = thinking_label((ctx.is_streaming)(), (ctx.duration)());
    let chevron_class = if open {
        "size-4 shrink-0 rotate-180 transition-transform"
    } else {
        "size-4 shrink-0 transition-transform"
    };

    let content = children.unwrap_or_else(|| {
        rsx! {
            svg {
                "data-slot": "reasoning-trigger-icon",
                "aria-hidden": "true",
                class: "size-4 shrink-0",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 18V5" }
                path { d: "M15 13a4.17 4.17 0 0 1-3-4 4.17 4.17 0 0 1-3 4" }
                path { d: "M17.598 6.5A3 3 0 1 0 12 5a3 3 0 1 0-5.598 1.5" }
                path { d: "M17.997 5.125a4 4 0 0 1 2.526 5.77" }
                path { d: "M18 18a4 4 0 0 0 2-7.464" }
                path { d: "M19.967 17.483A4 4 0 1 1 12 18a4 4 0 1 1-7.967-.517" }
                path { d: "M6 18a4 4 0 0 1-2-7.464" }
                path { d: "M6.003 5.125a4 4 0 0 0-2.526 5.77" }
            }
            span { "data-slot": "reasoning-trigger-label", class: "flex-1", {label} }
            svg {
                "data-slot": "reasoning-trigger-chevron",
                "aria-hidden": "true",
                class: chevron_class,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    });

    rsx! {
        div {
            "data-slot": "reasoning-trigger",
            class: class,
            ..attributes,
            CollapsibleTrigger {
                class: "flex w-full items-center gap-2 text-sm text-muted-foreground transition-colors hover:text-foreground",
                {content}
            }
        }
    }
}

/// The panel body. Rendered only while open; the text is shown as given.
#[component]
pub fn ReasoningContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "reasoning-content",
            class: class,
            ..attributes,
            CollapsibleContent {
                class: "mt-4 text-sm text-muted-foreground outline-none whitespace-pre-wrap",
                {children}
            }
        }
    }
}
