//! AlertDialog: a modal dialog that interrupts the user with important content and expects a response.
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertDialogSize {
    #[default]
    Default,
    Sm,
}

impl AlertDialogSize {
    fn data(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

#[derive(Clone, Copy)]
struct AlertDialogContext {
    is_open: Memo<bool>,
    state: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl AlertDialogContext {
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
pub fn AlertDialog(
    #[props(default)] default_open: bool,
    open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let state = use_signal(|| default_open);
    let mut controlled = use_signal(|| open);
    use_effect(use_reactive!(|open| controlled.set(open)));
    let is_open = use_memo(move || controlled().unwrap_or(state()));
    use_context_provider(|| AlertDialogContext {
        is_open,
        state,
        controlled,
        on_open_change,
    });

    rsx! { {children} }
}

#[component]
pub fn AlertDialogTrigger(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<AlertDialogContext>();
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "alert-dialog-trigger",
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
#[component]
pub fn AlertDialogContent(
    #[props(default)] size: AlertDialogSize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<AlertDialogContext>();
    if !(ctx.is_open)() {
        return rsx! {};
    }

    let id = use_hook(|| format!("alert-dialog-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
    let mut attributes = attributes;
    let class = cn!(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-6 rounded-xl p-6 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-lg group/alert-dialog-content relative grid w-full outline-none",
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
            "data-slot": "alert-dialog-overlay",
            "data-open": "",
            class: "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50 m-0 hidden h-full max-h-none w-full max-w-none items-center justify-center border-0 p-4 open:flex backdrop:bg-transparent",
            role: "presentation",
            onmounted: open_modal,
            oncancel: move |_| ctx.set_open(false),
            div {
                "data-slot": "alert-dialog-content",
                "data-size": size.data(),
                "data-open": "",
                role: "alertdialog",
                aria_modal: "true",
                class: class,
                ..attributes,
                {children}
            }
        }
    }
}

#[component]
pub fn AlertDialogHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-6 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-left sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "alert-dialog-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AlertDialogFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col-reverse gap-2 group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 sm:flex-row sm:justify-end",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "alert-dialog-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// An icon or illustration shown beside the title.
#[component]
pub fn AlertDialogMedia(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-muted mb-2 inline-flex size-16 items-center justify-center rounded-md sm:group-data-[size=default]/alert-dialog-content:row-span-2 *:[svg:not([class*='size-'])]:size-8",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "alert-dialog-media",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AlertDialogTitle(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-lg font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h2 {
            "data-slot": "alert-dialog-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AlertDialogDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground *:[a]:hover:text-foreground text-sm text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "alert-dialog-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Confirms the dialog: runs `onclick`, then closes it.
#[component]
pub fn AlertDialogAction(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<AlertDialogContext>();
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "alert-dialog-action",
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

/// Dismisses the dialog without confirming.
#[component]
pub fn AlertDialogCancel(
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<AlertDialogContext>();
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "alert-dialog-cancel",
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
