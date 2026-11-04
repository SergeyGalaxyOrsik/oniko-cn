//! HoverCard: a floating card that opens when the pointer rests on a trigger or the trigger is focused.
//!
//! The card opens after `open_delay` ms and closes after `close_delay` ms, so the pointer can move
//! from the trigger into the card. Delays run a `setTimeout` through `document::eval`, because
//! `tokio::time` is not available in the browser. The card is positioned with CSS and does not flip
//! at viewport edges. No `dioxus-primitives` crate is used: it is not published on crates.io.
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HoverCardSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl HoverCardSide {
    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HoverCardAlign {
    Start,
    #[default]
    Center,
    End,
}

impl HoverCardAlign {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

fn placement(side: HoverCardSide, align: HoverCardAlign) -> &'static str {
    use HoverCardAlign::{Center, End, Start};
    use HoverCardSide::{Bottom, Left, Right, Top};
    match (side, align) {
        (Top, Start) => "bottom-full left-0 mb-2",
        (Top, Center) => "bottom-full left-1/2 mb-2 -translate-x-1/2",
        (Top, End) => "bottom-full right-0 mb-2",
        (Bottom, Start) => "top-full left-0 mt-2",
        (Bottom, Center) => "top-full left-1/2 mt-2 -translate-x-1/2",
        (Bottom, End) => "top-full right-0 mt-2",
        (Left, Start) => "right-full top-0 mr-2",
        (Left, Center) => "right-full top-1/2 mr-2 -translate-y-1/2",
        (Left, End) => "right-full bottom-0 mr-2",
        (Right, Start) => "left-full top-0 ml-2",
        (Right, Center) => "left-full top-1/2 ml-2 -translate-y-1/2",
        (Right, End) => "left-full bottom-0 ml-2",
    }
}

#[derive(Clone, Copy)]
struct HoverCardContext {
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    open_delay: Signal<u32>,
    close_delay: Signal<u32>,
    /// Bumped on every request, so a pending timer that lost the race does nothing.
    pending: Signal<u64>,
}

impl HoverCardContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn commit(mut self, open: bool) {
        if self.is_open() == open {
            return;
        }
        if self.controlled.peek().is_none() {
            self.internal.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }

    fn request(mut self, open: bool) {
        let ticket = *self.pending.peek() + 1;
        self.pending.set(ticket);
        let delay = if open {
            *self.open_delay.peek()
        } else {
            *self.close_delay.peek()
        };
        if delay == 0 {
            self.commit(open);
            return;
        }
        spawn(async move {
            let _ = document::eval(&format!(
                "await new Promise((resolve) => setTimeout(resolve, {delay}))"
            ))
            .await;
            if *self.pending.peek() == ticket {
                self.commit(open);
            }
        });
    }

    fn cancel(mut self) {
        let ticket = *self.pending.peek() + 1;
        self.pending.set(ticket);
    }
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
#[component]
pub fn HoverCard(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    /// Milliseconds the pointer or focus must stay on the trigger before the card opens.
    #[props(default = 700)]
    open_delay: u32,
    /// Milliseconds after the pointer leaves the trigger and the card before the card closes.
    #[props(default = 300)]
    close_delay: u32,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("relative inline-block", take_class(&mut attributes));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let open_delay_signal = use_prop(open_delay);
    let close_delay_signal = use_prop(close_delay);
    let pending = use_signal(|| 0_u64);
    let context = use_context_provider(|| HoverCardContext {
        internal,
        controlled,
        on_open_change,
        open_delay: open_delay_signal,
        close_delay: close_delay_signal,
        pending,
    });

    rsx! {
        div {
            "data-slot": "hover-card",
            class: class,
            onkeydown: move |event| {
                if event.key() == Key::Escape {
                    context.cancel();
                    context.commit(false);
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// The element that opens the card on hover or focus. Rendered as a focusable `span`.
#[component]
pub fn HoverCardTrigger(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let context = use_context::<HoverCardContext>();
    let open = context.is_open();

    rsx! {
        span {
            "data-slot": "hover-card-trigger",
            "data-state": if open { "open" } else { "closed" },
            tabindex: "0",
            class: class,
            onpointerenter: move |_| context.request(true),
            onpointerleave: move |_| context.request(false),
            onfocus: move |_| context.request(true),
            onblur: move |_| context.request(false),
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn HoverCardContent(
    #[props(default)] side: HoverCardSide,
    #[props(default)] align: HoverCardAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-hover-card-content absolute z-50 outline-none",
        placement(side, align),
        take_class(&mut attributes)
    );
    let context = use_context::<HoverCardContext>();

    if !context.is_open() {
        return rsx! {};
    }
    rsx! {
        div {
            "data-slot": "hover-card-content",
            "data-state": "open",
            "data-open": "",
            "data-side": side.as_str(),
            "data-align": align.as_str(),
            class: class,
            onpointerenter: move |_| context.cancel(),
            onpointerleave: move |_| context.request(false),
            onfocus: move |_| context.cancel(),
            ..attributes,
            {children}
        }
    }
}
