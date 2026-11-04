//! Collapsible: an interactive region that shows and hides a block of content.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
struct CollapsibleContext {
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    disabled: Signal<bool>,
    on_open_change: Option<EventHandler<bool>>,
    id: usize,
}

impl CollapsibleContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn toggle(mut self) {
        let next = !self.is_open();
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

fn state(open: bool) -> &'static str {
    if open { "open" } else { "closed" }
}

#[component]
pub fn Collapsible(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    #[props(default)] disabled: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let disabled = use_prop(disabled);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let context = use_context_provider(|| CollapsibleContext {
        internal,
        controlled,
        disabled,
        on_open_change,
        id,
    });

    let open = context.is_open();
    rsx! {
        div {
            "data-slot": "collapsible",
            "data-state": state(open),
            "data-disabled": disabled().then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let context = use_context::<CollapsibleContext>();
    let open = context.is_open();
    let disabled = (context.disabled)();
    let id = context.id;

    rsx! {
        button {
            "data-slot": "collapsible-trigger",
            "data-state": state(open),
            r#type: "button",
            id: "collapsible-trigger-{id}",
            aria_controls: "collapsible-content-{id}",
            aria_expanded: open,
            disabled: disabled,
            class: class,
            onclick: move |_| {
                if !disabled {
                    context.toggle();
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CollapsibleContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let context = use_context::<CollapsibleContext>();
    let open = context.is_open();
    let id = context.id;

    // Closed content is unmounted, so it cannot be reached by keyboard or read by assistive tech.
    if !open {
        return rsx! {};
    }
    rsx! {
        div {
            "data-slot": "collapsible-content",
            "data-state": state(open),
            id: "collapsible-content-{id}",
            aria_labelledby: "collapsible-trigger-{id}",
            class: class,
            ..attributes,
            {children}
        }
    }
}
