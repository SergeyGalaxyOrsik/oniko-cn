//! Sheet: a panel that slides in from an edge of the screen to show content beside the page.
//!
//! Built on the native `<dialog>` element opened with `showModal()`, so the browser and the
//! desktop webview provide the focus trap, `Escape` handling and the inert background.
//! The edge is chosen with `SheetSide` on `SheetContent`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// The edge of the screen the sheet slides in from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    fn data(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

#[derive(Clone, Copy)]
struct SheetContext {
    is_open: Memo<bool>,
    state: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl SheetContext {
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
pub fn Sheet(
    #[props(default)] default_open: bool,
    open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let state = use_signal(|| default_open);
    let mut controlled = use_signal(|| open);
    use_effect(use_reactive!(|open| controlled.set(open)));
    let is_open = use_memo(move || controlled().unwrap_or(state()));
    use_context_provider(|| SheetContext {
        is_open,
        state,
        controlled,
        on_open_change,
    });

    rsx! { {children} }
}

#[component]
pub fn SheetTrigger(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<SheetContext>();
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "sheet-trigger",
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

/// The sheet panel and its dimmed overlay. Renders nothing while the sheet is closed.
///
/// A click on the overlay, or `Escape`, closes the sheet. The close button is shown
/// unless `show_close_button` is `false`.
#[component]
pub fn SheetContent(
    #[props(default)] side: SheetSide,
    #[props(default = true)] show_close_button: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<SheetContext>();
    let id = use_hook(|| format!("sheet-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
    if !(ctx.is_open)() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "cn-sheet-content group/sheet-content outline-none",
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
            "data-slot": "sheet-portal",
            class: "fixed inset-0 z-50 m-0 hidden h-full max-h-none w-full max-w-none border-0 bg-transparent p-0 open:flex backdrop:bg-transparent",
            role: "presentation",
            onmounted: open_modal,
            oncancel: move |_| ctx.set_open(false),
            div {
                "data-slot": "sheet-overlay",
                class: "cn-sheet-overlay absolute inset-0",
                onclick: move |_| ctx.set_open(false),
            }
            div {
                "data-slot": "sheet-content",
                "data-side": side.data(),
                role: "dialog",
                aria_modal: "true",
                class: class,
                ..attributes,
                {children}
                if show_close_button {
                    SheetClose { aria_label: "Close",
                        CloseIcon {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn SheetHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sheet-header flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sheet-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SheetFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sheet-footer mt-auto flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sheet-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SheetTitle(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-sheet-title", take_class(&mut attributes).as_deref());

    rsx! {
        h2 {
            "data-slot": "sheet-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SheetDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sheet-description",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "sheet-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Closes the sheet. Runs `onclick` first.
#[component]
pub fn SheetClose(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconSm)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<SheetContext>();
    let mut attributes = attributes;
    let extra = cn!("cn-sheet-close", take_class(&mut attributes).as_deref());
    let class = button_variants(variant, size, Some(extra.as_str()));

    rsx! {
        button {
            "data-slot": "sheet-close",
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

#[component]
fn CloseIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "16",
            height: "16",
            path { d: "M18 6 6 18" }
            path { d: "m6 6 12 12" }
        }
    }
}
