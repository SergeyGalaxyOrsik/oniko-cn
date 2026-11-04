//! Dialog: a window overlaid on the page that shows content and can be dismissed with the close button, `Escape` or a click outside it.
//!
//! Built on the native `<dialog>` element opened with `showModal()`, so the browser and the
//! desktop webview provide the focus trap, `Escape` handling and the inert background.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
struct DialogContext {
    is_open: Memo<bool>,
    state: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl DialogContext {
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
pub fn Dialog(
    #[props(default)] default_open: bool,
    open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let state = use_signal(|| default_open);
    let mut controlled = use_signal(|| open);
    use_effect(use_reactive!(|open| controlled.set(open)));
    let is_open = use_memo(move || controlled().unwrap_or(state()));
    use_context_provider(|| DialogContext {
        is_open,
        state,
        controlled,
        on_open_change,
    });

    rsx! { {children} }
}

#[component]
pub fn DialogTrigger(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "dialog-trigger",
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

/// The dialog window and its dimmed overlay. Renders nothing while the dialog is closed.
///
/// A click on the overlay outside the window closes the dialog. The close button is shown
/// unless `show_close_button` is `false`.
#[component]
pub fn DialogContent(
    #[props(default = true)] show_close_button: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();
    if !(ctx.is_open)() {
        return rsx! {};
    }

    let id = use_hook(|| format!("dialog-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
    let mut attributes = attributes;
    let class = cn!(
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-6 rounded-xl p-6 text-sm ring-1 duration-100 sm:max-w-md group/dialog-content relative w-full outline-none",
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
            "data-slot": "dialog-overlay",
            "data-open": "",
            class: "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50 m-0 hidden h-full max-h-none w-full max-w-none items-center justify-center border-0 p-4 open:flex backdrop:bg-transparent",
            role: "presentation",
            onmounted: open_modal,
            oncancel: move |_| ctx.set_open(false),
            onclick: move |_| ctx.set_open(false),
            div {
                "data-slot": "dialog-content",
                "data-open": "",
                role: "dialog",
                aria_modal: "true",
                class: class,
                onclick: move |event| event.stop_propagation(),
                ..attributes,
                {children}
                if show_close_button {
                    DialogClose { aria_label: "Close",
                        CloseIcon {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn DialogHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("gap-2", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "dialog-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DialogFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("gap-2", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "dialog-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DialogTitle(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "leading-none font-medium",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h2 {
            "data-slot": "dialog-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DialogDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "dialog-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Closes the dialog. Runs `onclick`, then closes.
#[component]
pub fn DialogClose(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconSm)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();
    let mut attributes = attributes;
    let extra = cn!(
        "absolute top-4 right-4",
        take_class(&mut attributes).as_deref()
    );
    let class = button_variants(variant, size, Some(extra.as_str()));

    rsx! {
        button {
            "data-slot": "dialog-close",
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
