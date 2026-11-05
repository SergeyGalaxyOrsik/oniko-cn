//! Popover: a floating panel that opens from a trigger and closes on outside click or `Escape`.
//!
//! The panel is focused when it opens. `Escape` closes it and returns focus to the trigger; a click outside
//! closes it through a transparent full-screen layer and leaves focus where the click put it. The panel is
//! positioned with CSS from `side` and `align` and is not portalled, so an ancestor with `overflow: hidden`
//! clips it. It does not flip at viewport edges. No `dioxus-primitives` crate is used: it is not published on
//! crates.io.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;

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
    trigger: Signal<Option<Rc<MountedData>>>,
}

impl PopoverContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn set_open(self, open: bool) {
        let mut internal = self.internal;
        if self.controlled.peek().is_none() {
            internal.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }

    fn close_and_focus_trigger(self) {
        self.set_open(false);
        if let Some(trigger) = self.trigger.peek().clone() {
            spawn(async move {
                let _ = trigger.set_focus(true).await;
            });
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl PopoverSide {
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
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
}

impl PopoverAlign {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

fn placement(side: PopoverSide, align: PopoverAlign) -> &'static str {
    use PopoverAlign::{Center, End, Start};
    use PopoverSide::{Bottom, Left, Right, Top};
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
    let class = cn!(
        "relative inline-block",
        take_class(&mut attributes).as_deref()
    );
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let trigger = use_signal(|| None::<Rc<MountedData>>);
    let context = use_context_provider(|| PopoverContext {
        internal,
        controlled,
        on_open_change,
        trigger,
    });

    rsx! {
        div {
            "data-slot": "popover",
            class: class,
            onkeydown: move |event| {
                if event.key() == Key::Escape && context.is_open() {
                    context.close_and_focus_trigger();
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// The button that toggles the panel. Style it with `class`, for example with `button_variants`.
#[component]
pub fn PopoverTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<PopoverContext>();
    let open = context.is_open();
    let mut trigger = context.trigger;
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "popover-trigger",
            "data-state": if open { "open" } else { "closed" },
            r#type: "button",
            aria_haspopup: "dialog",
            aria_expanded: open,
            class: class,
            onmounted: move |event| trigger.set(Some(event.data())),
            onclick: move |_| context.set_open(!open),
            ..attributes,
            {children}
        }
    }
}

/// The floating panel and a transparent layer behind it. Renders nothing while the popover is closed.
/// The panel takes focus when it opens.
#[component]
pub fn PopoverContent(
    #[props(default)] side: PopoverSide,
    #[props(default)] align: PopoverAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<PopoverContext>();
    if !context.is_open() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-4 rounded-md p-4 text-sm shadow-md ring-1 duration-100 absolute z-50 w-72 outline-none",
        placement(side, align),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "popover-overlay",
            class: "fixed inset-0 z-40",
            onclick: move |_| context.set_open(false),
        }
        div {
            "data-slot": "popover-content",
            "data-state": "open",
            "data-side": side.as_str(),
            "data-align": align.as_str(),
            role: "dialog",
            tabindex: "-1",
            class: class,
            onmounted: move |event| {
                let node = event.data();
                spawn(async move {
                    let _ = node.set_focus(true).await;
                });
            },
            ..attributes,
            {children}
        }
    }
}

/// Groups the title and description at the top of the panel.
#[component]
pub fn PopoverHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-1 text-sm",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "popover-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PopoverTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("font-medium", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "popover-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PopoverDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "popover-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}
