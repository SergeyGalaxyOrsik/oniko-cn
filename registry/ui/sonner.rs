//! Sonner: an opinionated toast component, a port of shadcn/ui's `Toaster` around the Sonner library.
//!
//! Render `Toaster` once near the root of the app, then call `toast(...)`, `toast_success(...)`,
//! `toast_promise(...)` and the like from anywhere, the way `toast(...)` and `toast.success(...)` are called
//! upstream. The list of toasts is a global signal, so no context is needed at the call site. The stack,
//! the hover expansion, the swipe to dismiss and the six positions follow Sonner's own stylesheet, written
//! here as Tailwind classes on the elements and driven by the same `data-*` attributes and CSS variables
//! (`--index`, `--toasts-before`, `--offset`, `--initial-height`, `--front-toast-height`). Timers run a small
//! script through `document::eval`, because `tokio::time` is not available in the browser. Not ported:
//! rich colors, `invert`, custom toast JSX, the `alt+T` hotkey and per-toast class overrides.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::future::Future;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static TOASTS: GlobalSignal<Vec<SonnerToast>> = Signal::global(Vec::new);
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

const SWIPE_THRESHOLD: f64 = 45.0;
const TIME_BEFORE_UNMOUNT_MS: u32 = 200;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl Position {
    fn y(self) -> &'static str {
        match self {
            Self::TopLeft | Self::TopCenter | Self::TopRight => "top",
            _ => "bottom",
        }
    }

    fn x(self) -> &'static str {
        match self {
            Self::TopLeft | Self::BottomLeft => "left",
            Self::TopCenter | Self::BottomCenter => "center",
            _ => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastKind {
    #[default]
    Default,
    Success,
    Info,
    Warning,
    Error,
    Loading,
}

impl ToastKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Loading => "loading",
        }
    }
}

/// The action button of a toast, like `action` upstream.
#[derive(Clone)]
pub struct ToastAction {
    pub label: String,
    pub on_click: Rc<dyn Fn()>,
}

impl ToastAction {
    pub fn new(label: impl Into<String>, on_click: impl Fn() + 'static) -> Self {
        Self {
            label: label.into(),
            on_click: Rc::new(on_click),
        }
    }
}

impl PartialEq for ToastAction {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && Rc::ptr_eq(&self.on_click, &other.on_click)
    }
}

/// The second argument of `toast(...)`.
#[derive(Clone, Default, PartialEq)]
pub struct ToastOptions {
    pub description: Option<String>,
    pub action: Option<ToastAction>,
    /// Overrides the position of the `Toaster` for this toast.
    pub position: Option<Position>,
    /// Milliseconds until the toast closes itself. `Some(0)` keeps it until it is dismissed.
    pub duration_ms: Option<u32>,
    /// Updates the toast with this id instead of adding one.
    pub id: Option<u64>,
}

#[derive(Clone, PartialEq)]
struct SonnerToast {
    id: u64,
    title: String,
    kind: ToastKind,
    options: ToastOptions,
    height: f64,
    removed: bool,
    swipe_out: bool,
}

fn push(title: String, kind: ToastKind, options: ToastOptions) -> u64 {
    if let Some(id) = options.id
        && let Some(existing) = TOASTS.write().iter_mut().find(|toast| toast.id == id)
    {
        existing.title = title;
        existing.kind = kind;
        existing.options = ToastOptions {
            id: None,
            ..options
        };
        existing.removed = false;
        return id;
    }
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    TOASTS.write().push(SonnerToast {
        id,
        title,
        kind,
        options,
        height: 0.0,
        removed: false,
        swipe_out: false,
    });
    id
}

/// Shows a toast and returns its id.
pub fn toast(message: impl Into<String>, options: ToastOptions) -> u64 {
    push(message.into(), ToastKind::Default, options)
}

pub fn toast_success(message: impl Into<String>, options: ToastOptions) -> u64 {
    push(message.into(), ToastKind::Success, options)
}

pub fn toast_info(message: impl Into<String>, options: ToastOptions) -> u64 {
    push(message.into(), ToastKind::Info, options)
}

pub fn toast_warning(message: impl Into<String>, options: ToastOptions) -> u64 {
    push(message.into(), ToastKind::Warning, options)
}

pub fn toast_error(message: impl Into<String>, options: ToastOptions) -> u64 {
    push(message.into(), ToastKind::Error, options)
}

/// A toast with a spinner that stays until it is dismissed or updated through `ToastOptions::id`.
pub fn toast_loading(message: impl Into<String>, options: ToastOptions) -> u64 {
    push(message.into(), ToastKind::Loading, options)
}

/// Shows a loading toast, then turns it into a success or an error toast when `future` finishes.
pub fn toast_promise<T: 'static, E: 'static>(
    future: impl Future<Output = Result<T, E>> + 'static,
    loading: impl Into<String>,
    success: impl FnOnce(T) -> String + 'static,
    error: impl FnOnce(E) -> String + 'static,
) -> u64 {
    let id = toast_loading(loading, ToastOptions::default());
    dioxus::core::spawn_forever(async move {
        let options = ToastOptions {
            id: Some(id),
            ..ToastOptions::default()
        };
        match future.await {
            Ok(value) => toast_success(success(value), options),
            Err(failure) => toast_error(error(failure), options),
        };
    });
    id
}

/// Dismisses the toast with this id, or every toast for `None`.
pub fn dismiss(id: Option<u64>) {
    for toast in TOASTS.write().iter_mut() {
        if id.is_none_or(|id| id == toast.id) {
            toast.removed = true;
        }
    }
}

async fn sleep(milliseconds: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {milliseconds}))"
    ))
    .await;
}

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

const FRAME_JS: &str = r#"await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
dioxus.send(true);"#;

/// Measures the natural height of a toast by letting it size itself for a moment.
const MEASURE_JS: &str = r#"const el = document.querySelector('[data-sonner-toast][data-toast-id="__ID__"]');
if (!el) {
  dioxus.send(0);
} else {
  const previous = el.style.height;
  el.style.height = 'auto';
  const height = el.offsetHeight;
  el.style.height = previous;
  dioxus.send(height);
}"#;

#[derive(Clone, Copy)]
struct ToasterSettings {
    expanded: Signal<bool>,
    duration: Signal<u32>,
    visible_toasts: Signal<usize>,
    gap: Signal<f64>,
    close_button: Signal<bool>,
}

/// Renders the toasts. Put it once near the root of the app.
#[component]
pub fn Toaster(
    #[props(default)] position: Position,
    /// Keeps the stack expanded instead of collapsing it behind the newest toast.
    #[props(default)]
    expand: bool,
    /// How many toasts of a position are visible at once.
    #[props(default = 3)]
    visible_toasts: usize,
    /// Default milliseconds before a toast closes itself.
    #[props(default = 4_000)]
    duration: u32,
    /// Space between toasts when the stack is expanded, in pixels.
    #[props(default = 14.0)]
    gap: f64,
    /// Shows a close button on each toast.
    #[props(default)]
    close_button: bool,
    #[props(extends = GlobalAttributes, extends = ol)] attributes: Vec<Attribute>,
) -> Element {
    let hovered = use_signal(|| false);
    let expanded = use_signal(|| false);
    let duration = use_prop(duration);
    let visible_toasts = use_prop(visible_toasts);
    let gap_signal = use_prop(gap);
    let close_button = use_prop(close_button);
    let mut expanded_state = expanded;
    let is_expanded = expand || hovered();
    if *expanded_state.peek() != is_expanded {
        expanded_state.set(is_expanded);
    }
    use_context_provider(|| ToasterSettings {
        expanded,
        duration,
        visible_toasts,
        gap: gap_signal,
        close_button,
    });

    let toasts = TOASTS.read().clone();
    let positions = [
        Position::TopLeft,
        Position::TopCenter,
        Position::TopRight,
        Position::BottomLeft,
        Position::BottomCenter,
        Position::BottomRight,
    ];
    let mut attributes = attributes;
    let class = cn!("toaster group", take_class(&mut attributes).as_deref());

    rsx! {
        section {
            aria_label: "Notifications alt+T",
            tabindex: "-1",
            aria_live: "polite",
            aria_relevant: "additions text",
            aria_atomic: "false",
            for place in positions {
                {
                    let group: Vec<SonnerToast> = toasts
                        .iter()
                        .filter(|toast| toast.options.position.unwrap_or(position) == place)
                        .rev()
                        .cloned()
                        .collect();
                    if group.is_empty() {
                        rsx! {}
                    } else {
                        rsx! {
                            ToastGroup {
                                key: "{place.y()}-{place.x()}",
                                place,
                                toasts: group,
                                hovered,
                                class: class.clone(),
                                attributes: attributes.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ToastGroupProps {
    place: Position,
    /// Newest first.
    toasts: Vec<SonnerToast>,
    hovered: Signal<bool>,
    class: String,
    attributes: Vec<Attribute>,
}

#[component]
fn ToastGroup(props: ToastGroupProps) -> Element {
    let settings = use_context::<ToasterSettings>();
    let ToastGroupProps {
        place,
        toasts,
        mut hovered,
        class,
        attributes,
    } = props;
    let expanded = (settings.expanded)();
    let gap = (settings.gap)();
    let front_height = toasts.first().map_or(0.0, |toast| toast.height);
    let (y, x) = (place.y(), place.x());
    let mut style = format!(
        "--width:356px;--gap:{gap}px;--offset-top:24px;--offset-right:24px;--offset-bottom:24px;--offset-left:24px;--normal-bg:var(--popover);--normal-text:var(--popover-foreground);--normal-border:var(--border);--border-radius:var(--radius);"
    );
    if front_height > 0.0 {
        style.push_str(&format!("--front-toast-height:{front_height}px;"));
    }
    let position_class = format!(
        "fixed z-[999999999] m-0 w-(--width) list-none p-0 outline-none {} {}",
        if y == "top" { "top-(--offset-top)" } else { "bottom-(--offset-bottom)" },
        match x {
            "left" => "left-(--offset-left)",
            "right" => "right-(--offset-right)",
            _ => "left-1/2 -translate-x-1/2",
        }
    );
    let count = toasts.len();
    let mut offsets = Vec::with_capacity(count);
    let mut running = 0.0;
    for toast in &toasts {
        offsets.push(running);
        running += toast.height + gap;
    }

    rsx! {
        ol {
            "data-sonner-toaster": "true",
            "data-x-position": x,
            "data-y-position": y,
            "data-lifted": expanded.then_some("true"),
            tabindex: "-1",
            class: cn!(position_class, class),
            style: style,
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            ..attributes,
            for (index , toast) in toasts.into_iter().enumerate() {
                SonnerToastView {
                    key: "{toast.id}",
                    toast,
                    place,
                    index,
                    count,
                    offset: offsets[index],
                }
            }
        }
    }
}

fn toast_class() -> String {
    [
        "cn-toast group/toast absolute z-(--z-index) box-border flex w-(--width) touch-none items-center gap-1.5 border border-(--normal-border) bg-(--normal-bg) p-4 text-[13px] text-(--normal-text) shadow-[0_4px_12px_rgba(0,0,0,0.1)] outline-none [overflow-wrap:anywhere]",
        "[--y:var(--y-start)] [--lift-amount:calc(var(--lift)*var(--gap))] opacity-0 [transform:var(--y)] [transition:transform_0.4s,opacity_0.4s,height_0.4s,box-shadow_0.2s]",
        "data-[x-position=left]:left-0 data-[x-position=center]:left-0 data-[x-position=right]:right-0",
        "data-[y-position=bottom]:bottom-0 data-[y-position=bottom]:[--y-start:translateY(100%)] data-[y-position=bottom]:[--lift:-1]",
        "data-[y-position=top]:top-0 data-[y-position=top]:[--y-start:translateY(-100%)] data-[y-position=top]:[--lift:1]",
        "data-[mounted=true]:[--y:translateY(0)] data-[mounted=true]:opacity-100",
        "data-[expanded=false]:data-[front=false]:[--scale:calc(1-var(--toasts-before)*0.05)] data-[expanded=false]:data-[front=false]:[--y:translateY(calc(var(--lift-amount)*var(--toasts-before)))_scale(var(--scale))] data-[expanded=false]:data-[front=false]:h-(--front-toast-height)",
        "*:[transition:opacity_0.4s] data-[expanded=false]:data-[front=false]:*:opacity-0",
        "data-[visible=false]:pointer-events-none data-[visible=false]:opacity-0",
        "data-[mounted=true]:data-[expanded=true]:[--y:translateY(calc(var(--lift)*var(--offset)))] data-[mounted=true]:data-[expanded=true]:h-(--initial-height)",
        "data-[removed=true]:data-[front=true]:data-[swipe-out=false]:[--y:translateY(calc(var(--lift)*-100%))] data-[removed=true]:data-[front=true]:data-[swipe-out=false]:opacity-0",
        "data-[removed=true]:data-[front=false]:data-[swipe-out=false]:data-[expanded=true]:[--y:translateY(calc(var(--lift)*var(--offset)+var(--lift)*-100%))] data-[removed=true]:data-[front=false]:data-[swipe-out=false]:data-[expanded=true]:opacity-0",
        "data-[removed=true]:data-[front=false]:data-[swipe-out=false]:data-[expanded=false]:[--y:translateY(40%)] data-[removed=true]:data-[front=false]:data-[swipe-out=false]:data-[expanded=false]:opacity-0 data-[removed=true]:data-[front=false]:data-[swipe-out=false]:data-[expanded=false]:[transition:transform_0.5s,opacity_0.2s]",
        "data-[swiping=true]:[transform:var(--y)_translateX(var(--swipe-amount-x,0px))_translateY(var(--swipe-amount-y,0px))] data-[swiping=true]:transition-none",
        "data-[swipe-out=true]:opacity-0 data-[swipe-out=true]:[transform:var(--y)_translateX(calc(var(--swipe-amount-x,0px)*3))_translateY(calc(var(--swipe-amount-y,0px)*3))]",
        "after:absolute after:left-0 after:h-[calc(var(--gap)+1px)] after:w-full after:content-[''] data-[y-position=bottom]:after:bottom-full data-[y-position=top]:after:top-full",
    ]
    .join(" ")
}

#[derive(Props, Clone, PartialEq)]
struct SonnerToastViewProps {
    toast: SonnerToast,
    place: Position,
    /// Position in the stack; 0 is the newest toast.
    index: usize,
    count: usize,
    offset: f64,
}

#[component]
fn SonnerToastView(props: SonnerToastViewProps) -> Element {
    let settings = use_context::<ToasterSettings>();
    let SonnerToastViewProps {
        toast,
        place,
        index,
        count,
        offset,
    } = props;
    let id = toast.id;
    let expanded = (settings.expanded)();
    let visible = index < (settings.visible_toasts)();

    let mut mounted = use_signal(|| false);
    use_effect(move || {
        let mut frame = document::eval(FRAME_JS);
        spawn(async move {
            if frame.recv::<bool>().await.is_ok() {
                mounted.set(true);
            }
        });
    });

    let duration = toast
        .options
        .duration_ms
        .unwrap_or_else(|| {
            if toast.kind == ToastKind::Loading {
                0
            } else {
                *settings.duration.peek()
            }
        });
    let mut ticket = use_signal(|| 0_u64);
    let paused = settings.expanded;
    let kind_key = toast.kind.as_str();
    use_effect(use_reactive!(|duration, kind_key| {
        let _ = kind_key;
        let current = *ticket.peek() + 1;
        ticket.set(current);
        if duration > 0 {
            spawn(async move {
                let mut remaining = i64::from(duration);
                loop {
                    sleep(100).await;
                    if *ticket.peek() != current {
                        break;
                    }
                    if !*paused.peek() {
                        remaining -= 100;
                        if remaining <= 0 {
                            dismiss(Some(id));
                            break;
                        }
                    }
                }
            });
        }
    }));

    let removed = toast.removed;
    use_effect(use_reactive!(|removed| {
        if removed {
            spawn(async move {
                sleep(TIME_BEFORE_UNMOUNT_MS).await;
                TOASTS.write().retain(|other| other.id != id);
            });
        }
    }));

    let signature = format!(
        "{}{:?}{}",
        toast.title,
        toast.options.description,
        toast.options.action.is_some()
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
    let (y, x) = (place.y(), place.x());
    let finish_swipe = move || {
        let (dx, dy) = *swipe.peek();
        origin.set(None);
        if dx.abs().max(dy.abs()) > SWIPE_THRESHOLD {
            if let Some(item) = TOASTS.write().iter_mut().find(|item| item.id == id) {
                item.swipe_out = true;
            }
            dismiss(Some(id));
        } else {
            swipe.set((0.0, 0.0));
        }
    };
    let toast_sign = |position: &str| if position == "left" || position == "top" { -1.0 } else { 1.0 };
    let (x_sign, y_sign) = (toast_sign(x), toast_sign(y));

    let (swipe_x, swipe_y) = swipe();
    let style = format!(
        "--index:{index};--toasts-before:{index};--z-index:{};--offset:{offset}px;--initial-height:{}px;--swipe-amount-x:{swipe_x}px;--swipe-amount-y:{swipe_y}px;",
        count - index,
        toast.height
    );
    let action = toast.options.action.clone();
    let description = toast.options.description.clone();
    let show_close = (settings.close_button)();

    rsx! {
        li {
            "data-sonner-toast": "",
            "data-toast-id": "{id}",
            "data-styled": "true",
            "data-mounted": mounted().to_string(),
            "data-visible": visible.to_string(),
            "data-y-position": y,
            "data-x-position": x,
            "data-index": "{index}",
            "data-front": (index == 0).to_string(),
            "data-removed": toast.removed.to_string(),
            "data-swipe-out": toast.swipe_out.to_string(),
            "data-swiping": origin().is_some().then_some("true"),
            "data-expanded": expanded.to_string(),
            "data-type": toast.kind.as_str(),
            "data-dismissible": "true",
            tabindex: "0",
            role: "status",
            aria_live: "polite",
            aria_atomic: "true",
            class: toast_class(),
            style: style,
            onpointerdown: move |event| {
                let point = event.client_coordinates();
                origin.set(Some((point.x, point.y)));
            },
            onpointermove: move |event| {
                if let Some((start_x, start_y)) = origin() {
                    let point = event.client_coordinates();
                    let dx = ((point.x - start_x) * x_sign).max(0.0) * x_sign;
                    let dy = ((point.y - start_y) * y_sign).max(0.0) * y_sign;
                    swipe.set(if x != "center" && dx.abs() > dy.abs() { (dx, 0.0) } else { (0.0, dy) });
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
            if show_close {
                button {
                    r#type: "button",
                    aria_label: "Close toast",
                    "data-close-button": "true",
                    class: "absolute top-0 left-0 z-10 flex size-5 -translate-x-[35%] -translate-y-[35%] cursor-pointer items-center justify-center rounded-full border border-(--normal-border) bg-(--normal-bg) p-0 text-(--normal-text) opacity-0 transition-opacity group-hover/toast:opacity-100 focus-visible:opacity-100",
                    onclick: move |_| dismiss(Some(id)),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        width: "12",
                        height: "12",
                        path { d: "M18 6 6 18" }
                        path { d: "m6 6 12 12" }
                    }
                }
            }
            if toast.kind != ToastKind::Default {
                div {
                    "data-icon": "",
                    class: "relative -ml-[3px] mr-1 flex h-4 w-4 shrink-0 items-center justify-start",
                    SonnerIcon { kind: toast.kind }
                }
            }
            div { "data-content": "", class: "flex flex-col gap-0.5",
                div { "data-title": "", class: "leading-normal font-medium text-inherit",
                    "{toast.title}"
                }
                if let Some(description) = description {
                    div {
                        "data-description": "",
                        class: "leading-[1.4] font-normal text-(--normal-text)/80",
                        "{description}"
                    }
                }
            }
            if let Some(action) = action {
                button {
                    r#type: "button",
                    "data-button": "true",
                    "data-action": "true",
                    class: "ml-auto flex h-6 shrink-0 cursor-pointer items-center justify-center rounded px-2 text-xs font-medium text-(--normal-bg) outline-none [background:var(--normal-text)]",
                    onclick: move |_| {
                        (action.on_click)();
                        dismiss(Some(id));
                    },
                    "{action.label}"
                }
            }
        }
    }
}

#[component]
fn SonnerIcon(kind: ToastKind) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "24",
            height: "24",
            class: if kind == ToastKind::Loading { "size-4 animate-spin" } else { "size-4" },
            match kind {
                ToastKind::Success => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    path { d: "m16 9-5.5 5.5L8 12" }
                },
                ToastKind::Info => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    path { d: "M12 16v-4" }
                    path { d: "M12 8h.01" }
                },
                ToastKind::Warning => rsx! {
                    path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" }
                    path { d: "M12 9v4" }
                    path { d: "M12 17h.01" }
                },
                ToastKind::Error => rsx! {
                    path { d: "m15 9-6 6" }
                    path { d: "M2.586 16.726A2 2 0 0 1 2 15.312V8.688a2 2 0 0 1 .586-1.414l4.688-4.688A2 2 0 0 1 8.688 2h6.624a2 2 0 0 1 1.414.586l4.688 4.688A2 2 0 0 1 22 8.688v6.624a2 2 0 0 1-.586 1.414l-4.688 4.688a2 2 0 0 1-1.414.586H8.688a2 2 0 0 1-1.414-.586z" }
                    path { d: "m9 9 6 6" }
                },
                ToastKind::Loading => rsx! {
                    path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
                },
                ToastKind::Default => rsx! {},
            }
        }
    }
}
