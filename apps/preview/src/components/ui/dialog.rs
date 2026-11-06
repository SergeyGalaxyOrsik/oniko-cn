//! Dialog: a window overlaid on the page that shows content and can be dismissed with the close button, `Escape` or a click outside it.
//!
//! The window lives in a native `<dialog>` host opened with `showModal()`, so the browser and the
//! desktop webview provide the top layer, the inert page behind it and the focus trap. `Escape` is
//! handled here instead of by the browser so the exit animation can play: the popup keeps
//! `data-closed` and `data-ending-style` until its CSS animations and transitions have finished,
//! then unmounts and focus returns to the trigger. Page scroll is locked while a modal dialog is open.
//! `modal` also accepts `False` (no overlay, page stays interactive) and `TrapFocus`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const LOCK_SCRIPT: &str = r#"
const locks = (window.__onikoLocks = window.__onikoLocks || new Set());
const root = document.documentElement;
if (locks.size === 0) {
  const gutter = window.innerWidth - root.clientWidth;
  root.dataset.onikoOverflow = root.style.overflow;
  root.dataset.onikoPadding = root.style.paddingRight;
  root.style.overflow = "hidden";
  if (gutter > 0) root.style.paddingRight = gutter + "px";
}
locks.add("__ID__");
"#;

const UNLOCK_SCRIPT: &str = r#"
const locks = window.__onikoLocks;
const root = document.documentElement;
if (locks && locks.delete("__ID__") && locks.size === 0) {
  root.style.overflow = root.dataset.onikoOverflow || "";
  root.style.paddingRight = root.dataset.onikoPadding || "";
  delete root.dataset.onikoOverflow;
  delete root.dataset.onikoPadding;
}
"#;

const MOUNT_SCRIPT: &str = r#"
const el = document.getElementById("__ID__");
if (!el) { dioxus.send("ready"); return; }
const mode = "__MODE__";
el.addEventListener("cancel", (event) => event.preventDefault());
if (mode === "modal") {
  if (!el.open) el.showModal();
  __LOCK__
} else if (!el.open) {
  el.show();
}
if (mode === "trap") {
  el.addEventListener("keydown", (event) => {
    if (event.key !== "Tab") return;
    const items = [...el.querySelectorAll('a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])')].filter((node) => node.offsetParent !== null);
    if (items.length === 0) return;
    const first = items[0];
    const last = items[items.length - 1];
    if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
    else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
  });
}
const onDown = (event) => {
  const popup = el.querySelector('[data-slot="__POPUP__"]');
  if (popup && !popup.contains(event.target) && !event.target.closest("[data-popup-open]")) dioxus.send("outside");
};
const dismiss = mode !== "modal" && __DISMISS__;
if (dismiss) document.addEventListener("pointerdown", onDown, true);
await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
dioxus.send("ready");
await new Promise((resolve) => {
  const timer = setInterval(() => {
    if (!el.isConnected) { clearInterval(timer); resolve(); }
  }, 200);
});
document.removeEventListener("pointerdown", onDown, true);
"#;

const EXIT_SCRIPT: &str = r#"
const el = document.getElementById("__ID__");
if (el) {
  await new Promise((resolve) => requestAnimationFrame(() => resolve()));
  const running = el.getAnimations({ subtree: true })
    .filter((animation) => {
      const timing = animation.effect && animation.effect.getComputedTiming();
      return timing && Number.isFinite(timing.endTime);
    })
    .map((animation) => animation.finished.catch(() => {}));
  await Promise.race([Promise.all(running), new Promise((resolve) => setTimeout(resolve, 1000))]);
}
dioxus.send(true);
"#;

const FRAMES_SCRIPT: &str = "await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))); dioxus.send(true);";

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// How the rest of the page behaves while the dialog is open.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogModal {
    /// Overlay, inert page, focus trap and scroll lock.
    #[default]
    True,
    /// No overlay; the page stays interactive and focus can leave the dialog.
    False,
    /// No overlay and the page stays interactive, but focus is trapped inside the dialog.
    TrapFocus,
}

impl DialogModal {
    fn mode(self) -> &'static str {
        match self {
            Self::True => "modal",
            Self::False => "free",
            Self::TrapFocus => "trap",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Closed,
    Starting,
    Open,
    Ending,
}

impl Phase {
    fn open(self) -> Option<&'static str> {
        matches!(self, Self::Starting | Self::Open).then_some("")
    }

    fn closed(self) -> Option<&'static str> {
        (self == Self::Ending).then_some("")
    }

    fn starting(self) -> Option<&'static str> {
        (self == Self::Starting).then_some("")
    }

    fn ending(self) -> Option<&'static str> {
        (self == Self::Ending).then_some("")
    }
}

#[derive(Clone, Copy)]
struct DialogContext {
    id: usize,
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    modal: Signal<DialogModal>,
    disable_pointer_dismissal: Signal<bool>,
    trigger: Signal<Option<Rc<MountedData>>>,
    has_title: Signal<bool>,
    has_description: Signal<bool>,
}

impl DialogContext {
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

    fn request_close(self) {
        if self.is_open() {
            self.set_open(false);
        }
    }

    fn host_id(self) -> String {
        format!("dialog-{}", self.id)
    }

    fn popup_id(self) -> String {
        format!("dialog-{}-popup", self.id)
    }

    fn title_id(self) -> String {
        format!("dialog-{}-title", self.id)
    }

    fn description_id(self) -> String {
        format!("dialog-{}-description", self.id)
    }
}

#[derive(Clone, Copy)]
struct PortalState {
    phase: Signal<Phase>,
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
#[component]
pub fn Dialog(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(default)] modal: DialogModal,
    /// Keep the dialog open when the page outside it is pressed.
    #[props(default)]
    disable_pointer_dismissal: bool,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let modal = use_prop(modal);
    let disable_pointer_dismissal = use_prop(disable_pointer_dismissal);
    let trigger = use_signal(|| None::<Rc<MountedData>>);
    let has_title = use_signal(|| false);
    let has_description = use_signal(|| false);
    use_context_provider(|| DialogContext {
        id,
        internal,
        controlled,
        on_open_change,
        modal,
        disable_pointer_dismissal,
        trigger,
        has_title,
        has_description,
    });

    rsx! { {children} }
}

/// The button that opens the dialog. Plain by default; pass `variant` or `size` to give it the button look.
#[component]
pub fn DialogTrigger(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();
    let mut own = use_signal(|| None::<Rc<MountedData>>);
    let mut trigger = ctx.trigger;
    let open = ctx.is_open();
    let mut attributes = attributes;
    let caller = take_class(&mut attributes);
    let class = if variant.is_some() || size.is_some() {
        button_variants(
            variant.unwrap_or_default(),
            size.unwrap_or_default(),
            caller.as_deref(),
        )
    } else {
        cn!(caller.as_deref())
    };

    rsx! {
        button {
            "data-slot": "dialog-trigger",
            "data-popup-open": open.then_some(""),
            r#type: "button",
            aria_haspopup: "dialog",
            aria_expanded: open,
            aria_controls: open.then(|| ctx.popup_id()),
            class: class,
            onmounted: move |event| own.set(Some(event.data())),
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                trigger.set(own.peek().clone());
                ctx.set_open(!open);
            },
            ..attributes,
            {children}
        }
    }
}

/// Closes the dialog after running `onclick`. Plain by default; pass `variant` or `size` to give it the button look.
#[component]
pub fn DialogClose(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();
    let mut attributes = attributes;
    let caller = take_class(&mut attributes);
    let class = if variant.is_some() || size.is_some() {
        button_variants(
            variant.unwrap_or_default(),
            size.unwrap_or_default(),
            caller.as_deref(),
        )
    } else {
        cn!(caller.as_deref())
    };

    rsx! {
        button {
            "data-slot": "dialog-close",
            r#type: "button",
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

/// The `<dialog>` host that carries the overlay and the popup. Renders nothing while the dialog is
/// closed, and keeps rendering during the exit animation.
#[component]
fn DialogPortal(children: Element) -> Element {
    let ctx = use_context::<DialogContext>();
    let mut phase = use_signal(|| Phase::Closed);
    let locked = use_hook(|| Rc::new(Cell::new(false)));
    use_context_provider(|| PortalState { phase });
    let id = ctx.host_id();

    use_drop({
        let locked = locked.clone();
        let id = id.clone();
        move || {
            if locked.get() {
                let _ = document::eval(&UNLOCK_SCRIPT.replace("__ID__", &id));
            }
        }
    });

    use_effect({
        let locked = locked.clone();
        let id = id.clone();
        move || {
            let open = ctx.is_open();
            let current = *phase.peek();
            if open && current == Phase::Closed {
                phase.set(Phase::Starting);
            } else if open && current == Phase::Ending {
                phase.set(Phase::Starting);
                spawn(async move {
                    let mut eval = document::eval(FRAMES_SCRIPT);
                    let _ = eval.recv::<bool>().await;
                    if *phase.peek() == Phase::Starting {
                        phase.set(Phase::Open);
                    }
                });
            } else if !open && matches!(current, Phase::Starting | Phase::Open) {
                phase.set(Phase::Ending);
                let locked = locked.clone();
                let id = id.clone();
                spawn(async move {
                    let mut eval = document::eval(&EXIT_SCRIPT.replace("__ID__", &id));
                    let _ = eval.recv::<bool>().await;
                    if ctx.is_open() || *phase.peek() != Phase::Ending {
                        return;
                    }
                    phase.set(Phase::Closed);
                    if locked.replace(false) {
                        let _ = document::eval(&UNLOCK_SCRIPT.replace("__ID__", &id));
                    }
                    // The page is inert until the dialog host is gone, so wait for the unmount.
                    let mut frames = document::eval(FRAMES_SCRIPT);
                    let _ = frames.recv::<bool>().await;
                    if let Some(trigger) = ctx.trigger.peek().clone() {
                        let _ = trigger.set_focus(true).await;
                    }
                });
            }
        }
    });

    if phase() == Phase::Closed {
        return rsx! {};
    }

    let modal = (ctx.modal)();
    let class = cn!(
        "fixed inset-0 z-50 m-0 h-full max-h-none w-full max-w-none overflow-visible border-0 bg-transparent p-0 backdrop:bg-transparent",
        (modal != DialogModal::True).then_some("pointer-events-none *:pointer-events-auto")
    );
    let mount = {
        let locked = locked.clone();
        let id = id.clone();
        move |_| {
            let locked = locked.clone();
            let id = id.clone();
            async move {
                let modal = *ctx.modal.peek();
                if modal == DialogModal::True {
                    locked.set(true);
                }
                let lock = if modal == DialogModal::True {
                    LOCK_SCRIPT.replace("__ID__", &id)
                } else {
                    String::new()
                };
                let script = MOUNT_SCRIPT
                    .replace("__LOCK__", &lock)
                    .replace("__ID__", &id)
                    .replace("__MODE__", modal.mode())
                    .replace("__POPUP__", "dialog-content")
                    .replace(
                        "__DISMISS__",
                        if *ctx.disable_pointer_dismissal.peek() {
                            "false"
                        } else {
                            "true"
                        },
                    );
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<String>().await {
                    match message.as_str() {
                        "ready" if *phase.peek() == Phase::Starting => phase.set(Phase::Open),
                        "outside" => ctx.request_close(),
                        _ => {}
                    }
                }
            }
        }
    };

    rsx! {
        dialog {
            id: "{id}",
            "data-slot": "dialog-portal",
            class: class,
            onmounted: mount,
            onkeydown: move |event| {
                if event.key() == Key::Escape {
                    ctx.request_close();
                }
            },
            {children}
        }
    }
}

#[component]
fn DialogOverlay(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<DialogContext>();
    let phase = (use_context::<PortalState>().phase)();
    let mut attributes = attributes;
    let class = cn!(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "dialog-overlay",
            "data-open": phase.open(),
            "data-closed": phase.closed(),
            "data-starting-style": phase.starting(),
            "data-ending-style": phase.ending(),
            role: "presentation",
            class: class,
            onclick: move |_| {
                if !*ctx.disable_pointer_dismissal.peek() {
                    ctx.request_close();
                }
            },
            ..attributes,
        }
    }
}

#[component]
fn DialogPopup(
    show_close_button: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();
    let phase = (use_context::<PortalState>().phase)();
    let mut attributes = attributes;
    let class = cn!(
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-6 rounded-xl p-6 text-sm ring-1 duration-100 sm:max-w-md fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            id: ctx.popup_id(),
            "data-slot": "dialog-content",
            "data-open": phase.open(),
            "data-closed": phase.closed(),
            "data-starting-style": phase.starting(),
            "data-ending-style": phase.ending(),
            role: "dialog",
            aria_modal: (*ctx.modal.peek() == DialogModal::True).then_some("true"),
            aria_labelledby: (ctx.has_title)().then(|| ctx.title_id()),
            aria_describedby: (ctx.has_description)().then(|| ctx.description_id()),
            tabindex: "-1",
            class: class,
            ..attributes,
            {children}
            if show_close_button {
                DialogClose {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::IconSm,
                    class: "absolute top-4 right-4",
                    CloseIcon {}
                    span { class: "sr-only", "Close" }
                }
            }
        }
    }
}

/// The dialog window and its dimmed overlay. Renders nothing while the dialog is closed.
///
/// The close button is shown unless `show_close_button` is `false`.
#[component]
pub fn DialogContent(
    #[props(default = true)] show_close_button: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();

    rsx! {
        DialogPortal {
            if (ctx.modal)() == DialogModal::True {
                DialogOverlay {}
            }
            DialogPopup { show_close_button, attributes, {children} }
        }
    }
}

#[component]
pub fn DialogHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "dialog-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The action row at the bottom. `show_close_button` appends an outline "Close" button.
#[component]
pub fn DialogFooter(
    #[props(default)] show_close_button: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 flex flex-col-reverse sm:flex-row sm:justify-end",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "dialog-footer",
            class: class,
            ..attributes,
            {children}
            if show_close_button {
                button {
                    "data-slot": "button",
                    r#type: "button",
                    class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                    onclick: move |_| ctx.set_open(false),
                    "Close"
                }
            }
        }
    }
}

#[component]
pub fn DialogTitle(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DialogContext>();
    let mut has_title = ctx.has_title;
    use_effect(move || has_title.set(true));
    use_drop(move || has_title.set(false));
    let mut attributes = attributes;
    let class = cn!(
        "leading-none font-medium",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h2 {
            id: ctx.title_id(),
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
    let ctx = use_context::<DialogContext>();
    let mut has_description = ctx.has_description;
    use_effect(move || has_description.set(true));
    use_drop(move || has_description.set(false));
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            id: ctx.description_id(),
            "data-slot": "dialog-description",
            class: class,
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
