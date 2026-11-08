//! AlertDialog: a modal dialog that interrupts the user with important content and expects a response.
//!
//! The window lives in a native `<dialog>` host opened with `showModal()`, so the browser and the
//! desktop webview provide the top layer, the inert page behind it and the focus trap. `Escape`
//! closes it; a click outside does not. `Escape` is handled here instead of by the browser so the
//! exit animation can play: the popup keeps `data-closed` and `data-ending-style` until its CSS
//! animations have finished, then unmounts and focus returns to the trigger. Page scroll is locked
//! while the dialog is open.
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
el.addEventListener("cancel", (event) => event.preventDefault());
if (!el.open) el.showModal();
__LOCK__
await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
dioxus.send("ready");
await new Promise((resolve) => {
  const timer = setInterval(() => {
    if (!el.isConnected) { clearInterval(timer); resolve(); }
  }, 200);
});
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
struct AlertDialogContext {
    id: usize,
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    trigger: Signal<Option<Rc<MountedData>>>,
    has_title: Signal<bool>,
    has_description: Signal<bool>,
}

impl AlertDialogContext {
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
        format!("alert-dialog-{}", self.id)
    }

    fn popup_id(self) -> String {
        format!("alert-dialog-{}-popup", self.id)
    }

    fn title_id(self) -> String {
        format!("alert-dialog-{}-title", self.id)
    }

    fn description_id(self) -> String {
        format!("alert-dialog-{}-description", self.id)
    }
}

#[derive(Clone, Copy)]
struct PortalState {
    phase: Signal<Phase>,
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
#[component]
pub fn AlertDialog(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let trigger = use_signal(|| None::<Rc<MountedData>>);
    let has_title = use_signal(|| false);
    let has_description = use_signal(|| false);
    use_context_provider(|| AlertDialogContext {
        id,
        internal,
        controlled,
        on_open_change,
        trigger,
        has_title,
        has_description,
    });

    rsx! { {children} }
}

/// The button that opens the dialog. Plain by default; pass `variant` or `size` to give it the button look.
#[component]
pub fn AlertDialogTrigger(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<AlertDialogContext>();
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
            "data-slot": "alert-dialog-trigger",
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
pub fn AlertDialogClose(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<AlertDialogContext>();
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
            "data-slot": "alert-dialog-close",
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
fn AlertDialogPortal(children: Element) -> Element {
    let ctx = use_context::<AlertDialogContext>();
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

    let class = "fixed inset-0 z-50 m-0 h-full max-h-none w-full max-w-none overflow-visible border-0 bg-transparent p-0 backdrop:bg-transparent";
    let mount = {
        let locked = locked.clone();
        let id = id.clone();
        move |_| {
            let locked = locked.clone();
            let id = id.clone();
            async move {
                locked.set(true);
                let script = MOUNT_SCRIPT
                    .replace("__LOCK__", &LOCK_SCRIPT.replace("__ID__", &id))
                    .replace("__ID__", &id);
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<String>().await {
                    match message.as_str() {
                        "ready" if *phase.peek() == Phase::Starting => phase.set(Phase::Open),
                        _ => {}
                    }
                }
            }
        }
    };

    rsx! {
        dialog {
            id: "{id}",
            "data-slot": "alert-dialog-portal",
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
fn AlertDialogOverlay(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let phase = (use_context::<PortalState>().phase)();
    let mut attributes = attributes;
    let class = cn!(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/80 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "alert-dialog-overlay",
            "data-open": phase.open(),
            "data-closed": phase.closed(),
            "data-starting-style": phase.starting(),
            "data-ending-style": phase.ending(),
            role: "presentation",
            class: class,
            ..attributes,
        }
    }
}

#[component]
fn AlertDialogPopup(
    size: AlertDialogSize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<AlertDialogContext>();
    let phase = (use_context::<PortalState>().phase)();
    let mut attributes = attributes;
    let class = cn!(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-3 rounded-xl p-4 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-64 data-[size=default]:sm:max-w-sm group/alert-dialog-content fixed top-1/2 left-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            id: ctx.popup_id(),
            "data-slot": "alert-dialog-content",
            "data-size": size.data(),
            "data-open": phase.open(),
            "data-closed": phase.closed(),
            "data-starting-style": phase.starting(),
            "data-ending-style": phase.ending(),
            role: "alertdialog",
            aria_modal: "true",
            aria_labelledby: (ctx.has_title)().then(|| ctx.title_id()),
            aria_describedby: (ctx.has_description)().then(|| ctx.description_id()),
            tabindex: "-1",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The dialog window and its dimmed overlay. Renders nothing while the dialog is closed.
/// A click outside the window does not close it.
#[component]
pub fn AlertDialogContent(
    #[props(default)] size: AlertDialogSize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        AlertDialogPortal {
            AlertDialogOverlay {}
            AlertDialogPopup { size, attributes, {children} }
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
        "grid grid-rows-[auto_1fr] place-items-center gap-1 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-4 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-left sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]",
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
        "bg-muted mb-2 inline-flex size-8 items-center justify-center rounded-md sm:group-data-[size=default]/alert-dialog-content:row-span-2 *:[svg:not([class*='size-'])]:size-4",
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
    let ctx = use_context::<AlertDialogContext>();
    let mut has_title = ctx.has_title;
    use_effect(move || has_title.set(true));
    use_drop(move || has_title.set(false));
    let mut attributes = attributes;
    let class = cn!(
        "text-sm font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 font-heading",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h2 {
            id: ctx.title_id(),
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
    let ctx = use_context::<AlertDialogContext>();
    let mut has_description = ctx.has_description;
    use_effect(move || has_description.set(true));
    use_drop(move || has_description.set(false));
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground *:[a]:hover:text-foreground text-xs/relaxed text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            id: ctx.description_id(),
            "data-slot": "alert-dialog-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The confirming `Button` of the dialog. It does not close the dialog by itself:
/// close it from `onclick` once the action succeeds, as upstream does.
#[component]
pub fn AlertDialogAction(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let extra = cn!("", take_class(&mut attributes).as_deref());
    let class = button_variants(variant, size, Some(extra.as_str()));

    rsx! {
        button {
            "data-slot": "alert-dialog-action",
            r#type: "button",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// Dismisses the dialog without confirming. Looks like an outline `Button` unless `variant` or `size` say otherwise.
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
    let extra = cn!("", take_class(&mut attributes).as_deref());
    let class = button_variants(variant, size, Some(extra.as_str()));

    rsx! {
        button {
            "data-slot": "alert-dialog-cancel",
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
