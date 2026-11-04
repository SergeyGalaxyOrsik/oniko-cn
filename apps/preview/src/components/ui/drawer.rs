//! Drawer: a panel that slides in from an edge of the screen, for content that needs the user's attention.
//!
//! Built on the native `<dialog>` element opened with `showModal()`, so the browser and the
//! desktop webview provide the focus trap, `Escape` handling and the inert background.
//! Swipe gestures and snap points are not supported: the drawer opens and closes on click or key.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// The edge the drawer slides in from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerDirection {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

impl DrawerDirection {
    fn data(self) -> &'static str {
        match self {
            Self::Down => "down",
            Self::Up => "up",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    fn axis(self) -> &'static str {
        match self {
            Self::Down | Self::Up => "y",
            Self::Left | Self::Right => "x",
        }
    }

    fn dialog_class(self) -> &'static str {
        match self {
            Self::Down => "open:flex-col open:justify-end",
            Self::Up => "open:flex-col open:justify-start",
            Self::Left => "open:flex-row open:justify-start",
            Self::Right => "open:flex-row open:justify-end",
        }
    }

    fn wrapper_class(self) -> &'static str {
        match self {
            Self::Down | Self::Up => "w-full max-h-[calc(100dvh-6rem)]",
            Self::Left | Self::Right => "h-full w-3/4 sm:w-96",
        }
    }

    fn popup_class(self) -> &'static str {
        match self {
            Self::Down | Self::Up => "w-full",
            Self::Left | Self::Right => "h-full w-full",
        }
    }
}

#[derive(Clone, Copy)]
struct DrawerContext {
    is_open: Memo<bool>,
    state: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    direction: DrawerDirection,
    show_handle: bool,
}

impl DrawerContext {
    fn set_open(mut self, open: bool) {
        if self.controlled.peek().is_none() {
            self.state.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }
}

/// Owns the open state. Pass `open` to control it, or let it manage itself.
#[component]
pub fn Drawer(
    #[props(default)] direction: DrawerDirection,
    #[props(default)] show_handle: bool,
    #[props(default)] default_open: bool,
    open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let state = use_signal(|| default_open);
    let mut controlled = use_signal(|| open);
    use_effect(use_reactive!(|open| controlled.set(open)));
    let is_open = use_memo(move || controlled().unwrap_or(state()));
    use_context_provider(|| DrawerContext {
        is_open,
        state,
        controlled,
        on_open_change,
        direction,
        show_handle,
    });

    rsx! { {children} }
}

#[component]
pub fn DrawerTrigger(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "drawer-trigger",
            class: class,
            aria_haspopup: "dialog",
            aria_expanded: "{ctx.is_open}",
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                ctx.set_open(true);
            },
            ..attributes,
            {children}
        }
    }
}

/// The panel and its dimmed overlay. Renders nothing while the drawer is closed.
/// Pressing the overlay or `Escape` closes it.
#[component]
pub fn DrawerContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    if !(ctx.is_open)() {
        return rsx! {};
    }

    let id = use_hook(|| format!("drawer-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
    let direction = ctx.direction;
    let mut attributes = attributes;
    let class = cn!(
        "bg-popover text-popover-foreground text-sm data-[swipe-direction=down]:rounded-t-xl data-[swipe-direction=down]:border-t data-[swipe-direction=left]:rounded-r-xl data-[swipe-direction=left]:border-r data-[swipe-direction=right]:rounded-l-xl data-[swipe-direction=right]:border-l data-[swipe-direction=up]:rounded-b-xl data-[swipe-direction=up]:border-b group/drawer-popup relative flex min-h-0 flex-col outline-none",
        direction.popup_class(),
        take_class(&mut attributes).as_deref()
    );

    let open_modal = {
        let id = id.clone();
        move |_| {
            let id = id.clone();
            async move {
                let _ =
                    document::eval(&format!("document.getElementById('{id}')?.showModal()")).await;
            }
        }
    };

    rsx! {
        dialog {
            id: "{id}",
            "data-slot": "drawer-portal",
            class: cn!(
                "fixed inset-0 z-50 m-0 hidden h-full max-h-none w-full max-w-none border-0 bg-transparent p-0 open:flex backdrop:bg-transparent",
                direction.dialog_class()
            ),
            role: "presentation",
            onmounted: open_modal,
            oncancel: move |_| ctx.set_open(false),
            div {
                "data-slot": "drawer-overlay",
                class: "bg-black/10 supports-backdrop-filter:backdrop-blur-xs absolute inset-0",
                onclick: move |_| ctx.set_open(false),
            }
            div {
                "data-slot": "drawer-content",
                class: cn!(
                    "bg-popover text-popover-foreground flex h-auto flex-col text-sm data-[vaul-drawer-direction=bottom]:rounded-t-xl data-[vaul-drawer-direction=bottom]:border-t data-[vaul-drawer-direction=left]:rounded-r-xl data-[vaul-drawer-direction=left]:border-r data-[vaul-drawer-direction=right]:rounded-l-xl data-[vaul-drawer-direction=right]:border-l data-[vaul-drawer-direction=top]:rounded-b-xl data-[vaul-drawer-direction=top]:border-b relative",
                    direction.wrapper_class()
                ),
                div {
                    "data-slot": "drawer-popup",
                    "data-swipe-direction": direction.data(),
                    "data-swipe-axis": direction.axis(),
                    role: "dialog",
                    aria_modal: "true",
                    class: class,
                    ..attributes,
                    if ctx.show_handle {
                        div {
                            "data-slot": "drawer-swipe-handle",
                            class: "after:bg-muted after:block after:shrink-0 after:rounded-full group-data-[swipe-axis=y]/drawer-popup:h-3 group-data-[swipe-axis=y]/drawer-popup:w-full group-data-[swipe-axis=y]/drawer-popup:justify-center group-data-[swipe-axis=y]/drawer-popup:after:h-1.5 group-data-[swipe-axis=y]/drawer-popup:after:w-[100px] group-data-[swipe-direction=down]/drawer-popup:items-end group-data-[swipe-direction=up]/drawer-popup:items-start group-data-[swipe-axis=x]/drawer-popup:h-full group-data-[swipe-axis=x]/drawer-popup:w-3 group-data-[swipe-axis=x]/drawer-popup:items-center group-data-[swipe-axis=x]/drawer-popup:after:h-[100px] group-data-[swipe-axis=x]/drawer-popup:after:w-1.5 group-data-[swipe-direction=left]/drawer-popup:justify-start group-data-[swipe-direction=right]/drawer-popup:justify-end flex",
                        }
                    }
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn DrawerHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-0.5 p-4 md:gap-1.5 md:text-left flex flex-col text-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "drawer-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DrawerFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 p-4 mt-auto flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "drawer-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DrawerTitle(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-foreground font-medium",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h2 {
            "data-slot": "drawer-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DrawerDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground text-sm",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "drawer-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Closes the drawer. Runs `onclick` first.
#[component]
pub fn DrawerClose(
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "drawer-close",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                ctx.set_open(false);
            },
            ..attributes,
            {children}
        }
    }
}
