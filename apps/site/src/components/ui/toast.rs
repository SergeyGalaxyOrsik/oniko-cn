//! Toast: a succinct message that is displayed temporarily, ported from Base UI's `Toast`.
//!
//! Render `Toaster` once near the root of the app, then call `toast().add(...)` from anywhere, the way
//! `toast.add(...)` is called upstream. The manager is a global signal, so no context is needed at the call
//! site. Toasts stack at the bottom right: the newest sits in front, older ones peek out behind it and fan
//! out while the viewport is hovered or focused, which also pauses their timers. Swipe a toast down or to
//! the right to dismiss it. Timers and animation waits run a small script through `document::eval`,
//! because `tokio::time` is not available in the browser. Toasts are not moved to the end of `<body>`,
//! and the viewport's `F6` hotkey and the offscreen announcer of Base UI are not ported.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::future::Future;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static TOASTS: GlobalSignal<Vec<ToastItem>> = Signal::global(Vec::new);
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

const DEFAULT_TIMEOUT_MS: u32 = 5_000;
const SWIPE_THRESHOLD: f64 = 40.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Info,
    Warning,
    Error,
    Loading,
}

impl ToastType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Loading => "loading",
        }
    }
}

/// `High` toasts are announced assertively.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPriority {
    #[default]
    Low,
    High,
}

/// The action button of a toast, like `actionProps` upstream. `on_click` receives the toast's id.
#[derive(Clone)]
pub struct ToastActionOptions {
    pub label: String,
    pub on_click: Option<Rc<dyn Fn(u64)>>,
}

impl ToastActionOptions {
    pub fn new(label: impl Into<String>, on_click: impl Fn(u64) + 'static) -> Self {
        Self {
            label: label.into(),
            on_click: Some(Rc::new(on_click)),
        }
    }
}

impl PartialEq for ToastActionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
            && match (&self.on_click, &other.on_click) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

/// Options for `toast().add(...)` and `toast().update(...)`. Fields left as `None` keep their value on update.
#[derive(Clone, Default, PartialEq)]
pub struct ToastOptions {
    pub title: Option<String>,
    pub description: Option<String>,
    pub kind: Option<ToastType>,
    pub priority: ToastPriority,
    /// Milliseconds until the toast closes itself. `Some(0)` keeps it until it is closed. Defaults to the
    /// provider's `timeout`, and to no timeout for `ToastType::Loading`.
    pub timeout: Option<u32>,
    pub action: Option<ToastActionOptions>,
}

impl ToastOptions {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..Self::default()
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ToastItem {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub kind: Option<ToastType>,
    pub priority: ToastPriority,
    pub timeout: Option<u32>,
    pub action: Option<ToastActionOptions>,
    height: f64,
    closing: bool,
}

/// The handle behind `toast()`. It is `Copy`; every handle talks to the same global list of toasts.
#[derive(Clone, Copy, Default)]
pub struct ToastManager;

/// Returns the global toast manager, like `toast` upstream.
pub fn toast() -> ToastManager {
    ToastManager
}

impl ToastManager {
    /// Shows a toast and returns its id.
    pub fn add(self, options: ToastOptions) -> u64 {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        TOASTS.write().push(ToastItem {
            id,
            title: options.title,
            description: options.description,
            kind: options.kind,
            priority: options.priority,
            timeout: options.timeout,
            action: options.action,
            height: 0.0,
            closing: false,
        });
        id
    }

    /// Starts closing a toast; it leaves the list once its exit transition is done.
    pub fn close(self, id: u64) {
        if let Some(item) = TOASTS.write().iter_mut().find(|item| item.id == id) {
            item.closing = true;
        }
    }

    /// Closes every toast.
    pub fn close_all(self) {
        for item in TOASTS.write().iter_mut() {
            item.closing = true;
        }
    }

    /// Changes the fields of a toast that `options` sets.
    pub fn update(self, id: u64, options: ToastOptions) {
        if let Some(item) = TOASTS.write().iter_mut().find(|item| item.id == id) {
            item.title = options.title.or(item.title.take());
            item.description = options.description.or(item.description.take());
            item.kind = options.kind.or(item.kind);
            item.timeout = options.timeout.or(item.timeout);
            item.action = options.action.or(item.action.take());
        }
    }

    /// Shows a loading toast, then turns it into a success or an error toast when `future` finishes.
    pub fn promise<T: 'static, E: 'static>(
        self,
        future: impl Future<Output = Result<T, E>> + 'static,
        loading: impl Into<String>,
        success: impl FnOnce(T) -> String + 'static,
        error: impl FnOnce(E) -> String + 'static,
    ) -> u64 {
        let id = self.add(ToastOptions {
            title: Some(loading.into()),
            kind: Some(ToastType::Loading),
            timeout: Some(0),
            ..ToastOptions::default()
        });
        dioxus::core::spawn_forever(async move {
            let (title, kind) = match future.await {
                Ok(value) => (success(value), ToastType::Success),
                Err(failure) => (error(failure), ToastType::Error),
            };
            if let Some(item) = TOASTS.write().iter_mut().find(|item| item.id == id) {
                item.title = Some(title);
                item.kind = Some(kind);
                item.timeout = None;
            }
        });
        id
    }

    /// The toasts in the order they were added.
    pub fn toasts(self) -> Vec<ToastItem> {
        TOASTS.read().clone()
    }
}

#[derive(Clone, Copy)]
struct ToastProviderContext {
    limit: Signal<usize>,
    timeout: Signal<u32>,
    /// True while the viewport is hovered or focused.
    expanded: Signal<bool>,
}

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

async fn sleep(milliseconds: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {milliseconds}))"
    ))
    .await;
}

/// Sets the stack limit and the default timeout for the toasts below it. Renders no element.
#[component]
pub fn ToastProvider(
    /// How many toasts stay visible in the stack; older ones fade out.
    #[props(default = 3)]
    limit: usize,
    /// Default milliseconds before a toast closes itself. `0` keeps toasts until they are closed.
    #[props(default = DEFAULT_TIMEOUT_MS)]
    timeout: u32,
    children: Element,
) -> Element {
    let limit = use_prop(limit);
    let timeout = use_prop(timeout);
    let expanded = use_signal(|| false);
    use_context_provider(|| ToastProviderContext {
        limit,
        timeout,
        expanded,
    });

    children
}

/// Where toasts render in the page. Renders no element: upstream moves them to the end of `<body>`.
#[component]
pub fn ToastPortal(children: Element) -> Element {
    children
}

/// The fixed region at the bottom right of the window that holds the stack. Hovering or focusing it
/// expands the stack and pauses the timers.
#[component]
pub fn ToastViewport(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ToastProviderContext>();
    let mut expanded = context.expanded;
    let mut attributes = attributes;
    let class = cn!(
        "pointer-events-none fixed inset-x-4 bottom-4 z-50 mx-auto w-auto max-w-sm outline-none sm:right-4 sm:left-auto sm:mx-0 sm:w-full",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "toast-viewport",
            "data-expanded": expanded().then_some(""),
            role: "region",
            aria_live: "polite",
            aria_label: "Notifications",
            tabindex: "-1",
            class: class,
            onpointerenter: move |_| expanded.set(true),
            onpointerleave: move |_| expanded.set(false),
            onfocusin: move |_| expanded.set(true),
            onfocusout: move |_| expanded.set(false),
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy)]
struct ToastRootContext {
    toast: Signal<ToastItem>,
    behind: Signal<bool>,
    expanded: Signal<bool>,
}

const EXIT_JS: &str = r#"await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
const el = document.querySelector('[data-toast-root="__ID__"]');
if (el) {
  const running = el.getAnimations()
    .filter((a) => a.effect?.getTiming().iterations !== Infinity)
    .map((a) => a.finished.catch(() => {}));
  await Promise.race([Promise.all(running), new Promise((resolve) => setTimeout(resolve, 1000))]);
}
dioxus.send(true);"#;

const FRAME_JS: &str = r#"await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
dioxus.send(true);"#;

/// Measures the natural height of a toast by letting it size itself for a moment.
const MEASURE_JS: &str = r#"const el = document.querySelector('[data-toast-root="__ID__"]');
if (!el) {
  dioxus.send(0);
} else {
  const previous = el.style.height;
  el.style.height = 'auto';
  const height = el.offsetHeight;
  el.style.height = previous;
  dioxus.send(height);
}"#;

/// Not passed through `cn!`: its class merge drops arbitrary `[transform:...]` utilities that differ only by variant.
fn toast_class() -> String {
    [
        "rounded-md group/toast pointer-events-auto absolute right-0 bottom-0 z-[calc(1000-var(--toast-index))] w-full origin-bottom border bg-popover text-popover-foreground shadow-lg will-change-transform outline-none select-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "[--gap:0.75rem] [--height:var(--toast-frontmost-height,var(--toast-height))] [--offset-y:calc(var(--toast-offset-y)*-1+calc(var(--toast-index)*var(--gap)*-1)+var(--toast-swipe-movement-y))] [--peek:0.75rem] [--scale:calc(max(0,1-(var(--toast-index)*0.1)))] [--shrink:calc(1-var(--scale))]",
        "h-(--height) [transform:translateX(var(--toast-swipe-movement-x))_translateY(calc(var(--toast-swipe-movement-y)-(var(--toast-index)*var(--peek))-(var(--shrink)*var(--height))))_scale(var(--scale))] [transition:transform_500ms_cubic-bezier(0.22,1,0.36,1),opacity_500ms,height_150ms]",
        "after:absolute after:top-full after:left-0 after:h-[calc(var(--gap)+1px)] after:w-full after:content-['']",
        "data-expanded:h-(--toast-height) data-expanded:[transform:translateX(var(--toast-swipe-movement-x))_translateY(var(--offset-y))]",
        "data-limited:opacity-0 data-starting-style:[transform:translateY(150%)]",
        "[&[data-ending-style]:not([data-limited]):not([data-swipe-direction])]:[transform:translateY(150%)]",
        "data-ending-style:data-[swipe-direction=down]:[transform:translateY(calc(var(--toast-swipe-movement-y)+150%))]",
        "data-ending-style:data-[swipe-direction=left]:[transform:translateX(calc(var(--toast-swipe-movement-x)-150%))_translateY(var(--offset-y))]",
        "data-ending-style:data-[swipe-direction=right]:[transform:translateX(calc(var(--toast-swipe-movement-x)+150%))_translateY(var(--offset-y))]",
        "data-ending-style:data-[swipe-direction=up]:[transform:translateY(calc(var(--toast-swipe-movement-y)-150%))]",
        "data-expanded:data-ending-style:data-[swipe-direction=down]:[transform:translateY(calc(var(--toast-swipe-movement-y)+150%))]",
        "data-expanded:data-ending-style:data-[swipe-direction=left]:[transform:translateX(calc(var(--toast-swipe-movement-x)-150%))_translateY(var(--offset-y))]",
        "data-expanded:data-ending-style:data-[swipe-direction=right]:[transform:translateX(calc(var(--toast-swipe-movement-x)+150%))_translateY(var(--offset-y))]",
        "data-expanded:data-ending-style:data-[swipe-direction=up]:[transform:translateY(calc(var(--toast-swipe-movement-y)-150%))]",
    ]
    .join(" ")
}

/// One toast of the stack. `toast` is an item of `toast().toasts()`.
#[component]
pub fn Toast(
    toast: ToastItem,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let provider = use_context::<ToastProviderContext>();
    let item = use_prop(toast.clone());
    let id = toast.id;
    let expanded = (provider.expanded)();

    let (index, offset, front_height) = {
        let all = TOASTS.read();
        let newer = all.iter().skip_while(|other| other.id != id).skip(1);
        let (count, offset) = newer.fold((0, 0.0), |(count, offset), other| {
            (count + 1, offset + other.height)
        });
        (
            count,
            offset,
            all.last().map_or(0.0, |newest| newest.height),
        )
    };
    let limited = index >= (provider.limit)();
    let behind = index > 0 && !expanded;

    let mut entered = use_signal(|| false);
    use_effect(move || {
        let mut frame = document::eval(FRAME_JS);
        spawn(async move {
            if frame.recv::<bool>().await.is_ok() {
                entered.set(true);
            }
        });
    });

    let timeout_ms = toast.timeout.unwrap_or_else(|| {
        if toast.kind == Some(ToastType::Loading) {
            0
        } else {
            *provider.timeout.peek()
        }
    });
    let mut ticket = use_signal(|| 0_u64);
    let paused = provider.expanded;
    use_effect(use_reactive!(|timeout_ms| {
        let current = *ticket.peek() + 1;
        ticket.set(current);
        if timeout_ms > 0 {
            spawn(async move {
                let mut remaining = i64::from(timeout_ms);
                loop {
                    sleep(100).await;
                    if *ticket.peek() != current {
                        break;
                    }
                    if !*paused.peek() {
                        remaining -= 100;
                        if remaining <= 0 {
                            ToastManager.close(id);
                            break;
                        }
                    }
                }
            });
        }
    }));

    let closing = toast.closing;
    use_effect(use_reactive!(|closing| {
        if closing {
            let mut done = document::eval(&EXIT_JS.replace("__ID__", &id.to_string()));
            spawn(async move {
                if done.recv::<bool>().await.is_ok() {
                    TOASTS.write().retain(|other| other.id != id);
                }
            });
        }
    }));

    let signature = format!(
        "{:?}{:?}{:?}{}",
        toast.title,
        toast.description,
        toast.kind,
        toast.action.is_some()
    );
    use_effect(use_reactive!(|signature| {
        let _ = signature;
        let mut measured = document::eval(&MEASURE_JS.replace("__ID__", &id.to_string()));
        spawn(async move {
            if let Ok(height) = measured.recv::<f64>().await
                && let Some(item) = TOASTS.write().iter_mut().find(|item| item.id == id)
                && (item.height - height).abs() > 0.5
            {
                item.height = height;
            }
        });
    }));

    let mut swipe = use_signal(|| (0.0_f64, 0.0_f64));
    let mut origin = use_signal(|| None::<(f64, f64)>);
    let mut swipe_direction = use_signal(|| None::<&'static str>);
    let finish_swipe = move || {
        let (x, y) = *swipe.peek();
        origin.set(None);
        if x.abs().max(y.abs()) > SWIPE_THRESHOLD {
            swipe_direction.set(Some(if y.abs() >= x.abs() { "down" } else { "right" }));
            ToastManager.close(id);
        } else {
            swipe.set((0.0, 0.0));
        }
    };

    let behind = use_prop(behind);
    use_context_provider(|| ToastRootContext {
        toast: item,
        behind,
        expanded: provider.expanded,
    });

    let mut attributes = attributes;
    let class = format!(
        "{} {}",
        toast_class(),
        take_class(&mut attributes).unwrap_or_default()
    );
    let (swipe_x, swipe_y) = swipe();
    let mut style = format!(
        "--toast-index:{index};--toast-offset-y:{offset}px;--toast-swipe-movement-x:{swipe_x}px;--toast-swipe-movement-y:{swipe_y}px;"
    );
    if toast.height > 0.0 {
        style.push_str(&format!("--toast-height:{}px;", toast.height));
    }
    if front_height > 0.0 {
        style.push_str(&format!("--toast-frontmost-height:{front_height}px;"));
    }
    if origin().is_some() {
        style.push_str("transition:none;");
    }

    rsx! {
        div {
            "data-slot": "toast",
            "data-toast-root": "{id}",
            "data-type": toast.kind.map(ToastType::as_str),
            "data-expanded": expanded.then_some(""),
            "data-limited": limited.then_some(""),
            "data-starting-style": (!entered()).then_some(""),
            "data-ending-style": toast.closing.then_some(""),
            "data-swiping": origin().is_some().then_some(""),
            "data-swipe-direction": swipe_direction(),
            role: "dialog",
            tabindex: "0",
            aria_modal: "false",
            aria_live: if toast.priority == ToastPriority::High { "assertive" } else { "polite" },
            class: class,
            style: style,
            onkeydown: move |event| {
                if event.key() == Key::Escape {
                    ToastManager.close(id);
                }
            },
            onpointerdown: move |event| {
                let point = event.client_coordinates();
                origin.set(Some((point.x, point.y)));
            },
            onpointermove: move |event| {
                if let Some((start_x, start_y)) = origin() {
                    let point = event.client_coordinates();
                    let dx = (point.x - start_x).max(0.0);
                    let dy = (point.y - start_y).max(0.0);
                    swipe.set(if dy >= dx { (0.0, dy) } else { (dx, 0.0) });
                }
            },
            onpointerup: move |_| {
                if origin.peek().is_some() {
                    let mut finish = finish_swipe;
                    finish();
                }
            },
            onpointerleave: move |_| {
                if origin.peek().is_some() {
                    let mut finish = finish_swipe;
                    finish();
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ToastContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ToastRootContext>();
    let mut attributes = attributes;
    let class = cn!(
        "flex h-full items-center gap-3 overflow-hidden p-4 transition-opacity duration-250 ease-[cubic-bezier(0.22,1,0.36,1)] data-behind:opacity-0 data-expanded:opacity-100",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "toast-content",
            "data-behind": (context.behind)().then_some(""),
            "data-expanded": (context.expanded)().then_some(""),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The toast's title. Without children it shows the title of the toast.
#[component]
pub fn ToastTitle(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ToastRootContext>();
    let item = (context.toast)();
    let mut attributes = attributes;
    let class = cn!(
        "text-sm font-medium",
        take_class(&mut attributes).as_deref()
    );

    if children.is_none() && item.title.is_none() {
        return rsx! {};
    }
    rsx! {
        h2 {
            "data-slot": "toast-title",
            "data-type": item.kind.map(ToastType::as_str),
            class: class,
            ..attributes,
            if let Some(children) = children {
                {children}
            } else if let Some(title) = item.title {
                "{title}"
            }
        }
    }
}

/// The toast's description. Without children it shows the description of the toast.
#[component]
pub fn ToastDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ToastRootContext>();
    let item = (context.toast)();
    let mut attributes = attributes;
    let class = cn!(
        "text-sm text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    if children.is_none() && item.description.is_none() {
        return rsx! {};
    }
    rsx! {
        p {
            "data-slot": "toast-description",
            "data-type": item.kind.map(ToastType::as_str),
            class: class,
            ..attributes,
            if let Some(children) = children {
                {children}
            } else if let Some(description) = item.description {
                "{description}"
            }
        }
    }
}

/// The toast's action button, an outline `Button` look. Renders nothing when the toast has no action.
#[component]
pub fn ToastAction(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ToastRootContext>();
    let item = (context.toast)();
    let Some(action) = item.action else {
        return rsx! {};
    };
    let mut attributes = attributes;
    let class = button_variants(
        ButtonVariant::Outline,
        ButtonSize::Sm,
        Some(&cn!("shrink-0", take_class(&mut attributes).as_deref())),
    );
    let on_click = action.on_click;
    let id = item.id;

    rsx! {
        button {
            "data-slot": "toast-action",
            r#type: "button",
            class: class,
            onclick: move |_| {
                if let Some(on_click) = &on_click {
                    on_click(id);
                }
            },
            ..attributes,
            if let Some(children) = children {
                {children}
            } else {
                "{action.label}"
            }
        }
    }
}

/// Closes the toast. Shows an X icon unless it has children.
#[component]
pub fn ToastClose(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ToastRootContext>();
    let id = (context.toast)().id;
    let mut attributes = attributes;
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        Some(&cn!(
            "relative shrink-0 text-muted-foreground after:absolute after:-inset-2 after:content-[''] hover:text-foreground",
            take_class(&mut attributes).as_deref()
        )),
    );

    rsx! {
        button {
            "data-slot": "toast-close",
            r#type: "button",
            aria_label: "Close toast",
            class: class,
            onclick: move |_| ToastManager.close(id),
            ..attributes,
            if let Some(children) = children {
                {children}
            } else {
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    width: "24",
                    height: "24",
                    "aria-hidden": "true",
                    path { d: "M18 6 6 18" }
                    path { d: "m6 6 12 12" }
                }
            }
        }
    }
}

/// The status icon of a toast type; renders nothing for a toast without a type.
#[component]
pub fn ToastIcon(kind: Option<ToastType>) -> Element {
    let Some(kind) = kind else {
        return rsx! {};
    };

    rsx! {
        span {
            "data-slot": "toast-icon",
            class: "shrink-0 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                width: "24",
                height: "24",
                "aria-hidden": "true",
                class: match kind {
                    ToastType::Error => "text-destructive",
                    ToastType::Loading => "animate-spin",
                    _ => "",
                },
                match kind {
                    ToastType::Success => rsx! {
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "m16 9-5.5 5.5L8 12" }
                    },
                    ToastType::Info => rsx! {
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "M12 16v-4" }
                        path { d: "M12 8h.01" }
                    },
                    ToastType::Warning => rsx! {
                        path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" }
                        path { d: "M12 9v4" }
                        path { d: "M12 17h.01" }
                    },
                    ToastType::Error => rsx! {
                        path { d: "m15 9-6 6" }
                        path { d: "M2.586 16.726A2 2 0 0 1 2 15.312V8.688a2 2 0 0 1 .586-1.414l4.688-4.688A2 2 0 0 1 8.688 2h6.624a2 2 0 0 1 1.414.586l4.688 4.688A2 2 0 0 1 22 8.688v6.624a2 2 0 0 1-.586 1.414l-4.688 4.688a2 2 0 0 1-1.414.586H8.688a2 2 0 0 1-1.414-.586z" }
                        path { d: "m9 9 6 6" }
                    },
                    ToastType::Loading => rsx! {
                        path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
                    },
                }
            }
        }
    }
}

/// Renders every toast of the manager the way the default `Toaster` upstream does.
#[component]
fn ToastList() -> Element {
    let toasts = TOASTS.read().clone();

    rsx! {
        for item in toasts {
            Toast { key: "{item.id}", toast: item.clone(),
                ToastContent {
                    ToastIcon { kind: item.kind }
                    div { class: "flex min-w-0 flex-1 flex-col gap-1",
                        ToastTitle {}
                        ToastDescription {}
                    }
                    ToastAction {}
                    ToastClose {}
                }
            }
        }
    }
}

/// `ToastProvider`, `ToastPortal`, `ToastViewport` and the list of toasts in one. Put it once near the root
/// and pass the app as `children`, like upstream.
#[component]
pub fn Toaster(
    #[props(default = 3)] limit: usize,
    #[props(default = DEFAULT_TIMEOUT_MS)] timeout: u32,
    children: Option<Element>,
) -> Element {
    rsx! {
        ToastProvider { limit, timeout,
            if let Some(children) = children {
                {children}
            }
            ToastPortal {
                ToastViewport {
                    ToastList {}
                }
            }
        }
    }
}
