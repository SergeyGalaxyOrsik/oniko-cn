//! Tooltip: a short label that opens when the pointer rests on a trigger or the trigger is focused.
//!
//! `TooltipProvider` sets the open delay for the tooltips inside it. A `Tooltip` opens after its delay and
//! closes at once on pointer leave, blur or `Escape`. The delay runs a `setTimeout` through `document::eval`,
//! because `tokio::time` is not available in the browser. The trigger points at the content with
//! `aria-describedby` while the tooltip is open, and the content has `role="tooltip"`. The content is
//! positioned with CSS and does not flip at viewport edges. No `dioxus-primitives` crate is used: it is not
//! published on crates.io.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static TOOLTIP_IDS: AtomicU64 = AtomicU64::new(0);

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

#[derive(Clone, Copy)]
struct TooltipProviderContext {
    delay: Signal<u32>,
}

/// Sets the open delay for the tooltips inside it. Renders its children without a DOM element.
#[component]
pub fn TooltipProvider(
    /// Milliseconds the pointer or focus must stay on a trigger before its tooltip opens.
    #[props(default)]
    delay_duration: u32,
    children: Element,
) -> Element {
    let delay = use_prop(delay_duration);
    use_context_provider(|| TooltipProviderContext { delay });

    children
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
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
pub enum TooltipAlign {
    Start,
    #[default]
    Center,
    End,
}

impl TooltipAlign {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

fn placement(side: TooltipSide, align: TooltipAlign) -> &'static str {
    use TooltipAlign::{Center, End, Start};
    use TooltipSide::{Bottom, Left, Right, Top};
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

/// Places the arrow on the edge of the content that faces the trigger. Left and right sides always
/// center the arrow vertically, so `align` only moves the arrow on top and bottom sides.
fn arrow_placement(side: TooltipSide, align: TooltipAlign) -> &'static str {
    use TooltipAlign::{Center, End, Start};
    use TooltipSide::{Bottom, Left, Right, Top};
    match (side, align) {
        (Top, Start) => "top-full left-3",
        (Top, Center) => "top-full left-1/2 -translate-x-1/2",
        (Top, End) => "top-full right-3",
        (Bottom, Start) => "top-0 left-3",
        (Bottom, Center) => "top-0 left-1/2 -translate-x-1/2",
        (Bottom, End) => "top-0 right-3",
        (Left, Start | Center | End) => "left-full top-1/2",
        (Right, Start | Center | End) => "left-0 top-1/2",
    }
}

#[derive(Clone, Copy)]
struct TooltipContext {
    id: u64,
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    open_delay: Signal<u32>,
    /// Bumped on every request, so a pending timer that lost the race does nothing.
    pending: Signal<u64>,
}

impl TooltipContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn content_id(self) -> String {
        format!("tooltip-content-{}", self.id)
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

    fn cancel(mut self) {
        let ticket = *self.pending.peek() + 1;
        self.pending.set(ticket);
    }

    fn show(mut self) {
        let ticket = *self.pending.peek() + 1;
        self.pending.set(ticket);
        let delay = *self.open_delay.peek();
        if delay == 0 {
            self.commit(true);
            return;
        }
        spawn(async move {
            let _ = document::eval(&format!(
                "await new Promise((resolve) => setTimeout(resolve, {delay}))"
            ))
            .await;
            if *self.pending.peek() == ticket {
                self.commit(true);
            }
        });
    }

    fn hide(self) {
        self.cancel();
        self.commit(false);
    }
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
/// `open_delay` overrides the delay of the enclosing `TooltipProvider`.
#[component]
pub fn Tooltip(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    /// Milliseconds before the tooltip opens after the pointer or focus arrives on the trigger.
    open_delay: Option<u32>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "relative inline-block",
        take_class(&mut attributes).as_deref()
    );
    let provider_delay =
        try_use_context::<TooltipProviderContext>().map(|provider| (provider.delay)());
    let delay = open_delay.or(provider_delay).unwrap_or(0);
    let id = use_hook(|| TOOLTIP_IDS.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let open_delay_signal = use_prop(delay);
    let pending = use_signal(|| 0_u64);
    let context = use_context_provider(|| TooltipContext {
        id,
        internal,
        controlled,
        on_open_change,
        open_delay: open_delay_signal,
        pending,
    });

    rsx! {
        div {
            "data-slot": "tooltip",
            class: class,
            onkeydown: move |event| {
                if event.key() == Key::Escape {
                    context.hide();
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// The element that opens the tooltip on hover or focus. Rendered as a focusable `span`.
#[component]
pub fn TooltipTrigger(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());
    let context = use_context::<TooltipContext>();
    let open = context.is_open();
    let described_by = open.then(|| context.content_id());

    rsx! {
        span {
            "data-slot": "tooltip-trigger",
            "data-state": if open { "open" } else { "closed" },
            tabindex: "0",
            "aria-describedby": described_by,
            class: class,
            onpointerenter: move |_| context.show(),
            onpointerleave: move |_| context.hide(),
            onfocus: move |_| context.show(),
            onblur: move |_| context.hide(),
            ..attributes,
            {children}
        }
    }
}

/// The floating label with its arrow. Renders nothing while the tooltip is closed. It ignores the pointer,
/// so it never blocks the trigger.
#[component]
pub fn TooltipContent(
    #[props(default)] side: TooltipSide,
    #[props(default)] align: TooltipAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<TooltipContext>();
    if !context.is_open() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "cn-tooltip-content absolute z-50 pointer-events-none w-max max-w-xs bg-foreground text-background",
        placement(side, align),
        take_class(&mut attributes).as_deref()
    );
    let arrow_class = cn!(
        "cn-tooltip-arrow absolute z-50 bg-foreground fill-foreground",
        arrow_placement(side, align)
    );

    rsx! {
        div {
            "data-slot": "tooltip-content",
            "data-state": "open",
            "data-open": "",
            "data-side": side.as_str(),
            "data-align": align.as_str(),
            id: context.content_id(),
            role: "tooltip",
            class: class,
            ..attributes,
            {children}
            span {
                "data-slot": "tooltip-arrow",
                "data-side": side.as_str(),
                class: arrow_class,
            }
        }
    }
}
