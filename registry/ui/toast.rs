//! Toast: a stack of short messages that dismiss themselves after a while.
//!
//! Wrap the app in `ToastProvider` once, render `Toaster` where the stack should appear,
//! and call `use_toast().show(ToastOptions::new(...))` from any component below the provider.
//! `use_toast` panics when no `ToastProvider` is above the caller.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

pub const DEFAULT_TOAST_DURATION_MS: u32 = 5_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Info,
    Warning,
    Error,
}

impl ToastVariant {
    pub fn name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Success => "border-l-4 border-l-emerald-500",
            Self::Info => "border-l-4 border-l-sky-500",
            Self::Warning => "border-l-4 border-l-amber-500",
            Self::Error => "border-l-4 border-l-destructive",
        }
    }
}

#[derive(Clone)]
pub struct ToastAction {
    pub label: String,
    pub onclick: EventHandler<()>,
}

#[derive(Clone)]
pub struct ToastOptions {
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    /// Milliseconds until the toast dismisses itself. `0` keeps it until the user closes it.
    pub duration_ms: u32,
    pub action: Option<ToastAction>,
}

impl ToastOptions {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            variant: ToastVariant::default(),
            duration_ms: DEFAULT_TOAST_DURATION_MS,
            action: None,
        }
    }
}

#[derive(Clone)]
struct Toast {
    id: u64,
    options: ToastOptions,
}

#[derive(Clone, Copy)]
pub struct ToastHandle {
    toasts: Signal<Vec<Toast>>,
    next_id: Signal<u64>,
}

impl ToastHandle {
    /// Shows a toast and returns its id.
    pub fn show(&self, options: ToastOptions) -> u64 {
        let mut next_id = self.next_id;
        let id = {
            let mut counter = next_id.write();
            *counter += 1;
            *counter
        };

        let duration_ms = options.duration_ms;
        let mut toasts = self.toasts;
        toasts.write().push(Toast { id, options });

        if duration_ms > 0 {
            let handle = *self;
            spawn(async move {
                let _ = document::eval(&format!(
                    "await new Promise((resolve) => setTimeout(resolve, {duration_ms}))"
                ))
                .await;
                handle.dismiss(id);
            });
        }
        id
    }

    /// Removes a toast by id. Unknown or already dismissed ids are ignored.
    pub fn dismiss(&self, id: u64) {
        let mut toasts = self.toasts;
        toasts.write().retain(|toast| toast.id != id);
    }
}

pub fn use_toast() -> ToastHandle {
    use_context::<ToastHandle>()
}

#[component]
pub fn ToastProvider(children: Element) -> Element {
    let toasts = use_signal(Vec::new);
    let next_id = use_signal(|| 0_u64);
    use_context_provider(|| ToastHandle { toasts, next_id });

    rsx! {
        {children}
    }
}

#[component]
pub fn Toaster(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let handle = use_toast();
    let items = handle.toasts.read().clone();
    let mut attributes = attributes;
    let class = cn!(
        "fixed right-4 bottom-4 z-50 flex w-full max-w-sm flex-col gap-2 outline-none pointer-events-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "toaster",
            role: "status",
            "aria-live": "polite",
            class: class,
            ..attributes,
            for toast in items {
                {
                    let id = toast.id;
                    let variant = toast.options.variant;
                    let title = toast.options.title.clone();
                    let description = toast.options.description.clone();
                    let action = toast.options.action.clone().map(|action| (action.label, action.onclick));
                    rsx! {
                        div {
                            key: "{id}",
                            "data-slot": "toast",
                            "data-variant": variant.name(),
                            class: cn!(
                                "cn-toast pointer-events-auto relative flex w-full items-start gap-3 border bg-popover p-4 text-popover-foreground shadow-lg",
                                variant.class()
                            ),
                            div {
                                "data-slot": "toast-content",
                                class: "grid min-w-0 flex-1 gap-1",
                                div {
                                    "data-slot": "toast-title",
                                    class: "text-sm font-medium",
                                    "{title}"
                                }
                                if let Some(description) = description {
                                    div {
                                        "data-slot": "toast-description",
                                        class: "text-sm text-muted-foreground",
                                        "{description}"
                                    }
                                }
                            }
                            if let Some((label, onclick)) = action {
                                button {
                                    r#type: "button",
                                    "data-slot": "toast-action",
                                    class: button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
                                    onclick: move |_| {
                                        onclick.call(());
                                        handle.dismiss(id);
                                    },
                                    "{label}"
                                }
                            }
                            button {
                                r#type: "button",
                                "data-slot": "toast-close",
                                "aria-label": "Dismiss",
                                class: button_variants(ButtonVariant::Ghost, ButtonSize::IconSm, None),
                                onclick: move |_| handle.dismiss(id),
                                "×"
                            }
                        }
                    }
                }
            }
        }
    }
}
