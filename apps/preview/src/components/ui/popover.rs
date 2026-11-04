//! Popover: a floating panel that opens from a trigger and closes on outside click or `Escape`.
//!
//! The panel is positioned under the trigger with CSS and dismissed by a transparent full-screen
//! layer. No `dioxus-primitives` crate is used: it is not published on crates.io, so focus
//! placement and flipping at viewport edges are not handled.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

#[derive(Clone, Copy)]
struct PopoverContext {
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl PopoverContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn set_open(mut self, open: bool) {
        if self.controlled.peek().is_none() {
            self.internal.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
#[component]
pub fn Popover(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("relative inline-block", take_class(&mut attributes));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let context = use_context_provider(|| PopoverContext {
        internal,
        controlled,
        on_open_change,
    });

    rsx! {
        div {
            "data-slot": "popover",
            class: class,
            onkeydown: move |event| {
                if event.key() == Key::Escape && context.is_open() {
                    context.set_open(false);
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PopoverTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let context = use_context::<PopoverContext>();
    let open = context.is_open();

    rsx! {
        button {
            "data-slot": "popover-trigger",
            "data-state": if open { "open" } else { "closed" },
            r#type: "button",
            aria_haspopup: "dialog",
            aria_expanded: open,
            class: class,
            onclick: move |_| context.set_open(!open),
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PopoverContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-4 rounded-md p-4 text-sm shadow-md ring-1 duration-100 absolute top-full left-0 z-50 mt-2 w-auto outline-none",
        take_class(&mut attributes)
    );
    let context = use_context::<PopoverContext>();

    if !context.is_open() {
        return rsx! {};
    }
    rsx! {
        div {
            "data-slot": "popover-overlay",
            class: "fixed inset-0 z-40",
            onclick: move |_| context.set_open(false),
        }
        div {
            "data-slot": "popover-content",
            "data-state": "open",
            role: "dialog",
            class: class,
            ..attributes,
            {children}
        }
    }
}
