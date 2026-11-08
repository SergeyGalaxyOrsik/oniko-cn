//! MessageScroller: a chat transcript scroller that anchors new turns, opens saved threads at a
//! chosen position, follows streamed replies, keeps its place when history loads above, and jumps
//! to any message.
//!
//! The scroll behavior is a port of `@shadcn/react/message-scroller`. It runs as one script per
//! `MessageScrollerProvider`, started with `document::eval`, so the same code drives the browser
//! and the desktop webview. The script finds the parts by their `data-message-scroller-*`
//! attributes and reports scroll state back to Rust, where the hooks read it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Where a saved transcript opens on its first non-empty render.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageScrollerDefaultScrollPosition {
    Start,
    #[default]
    End,
    /// The last `scroll_anchor` item, or the end when there is none or the last turn already fits.
    LastAnchor,
}

impl MessageScrollerDefaultScrollPosition {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::LastAnchor => "last-anchor",
        }
    }
}

/// Which transcript edge a `MessageScrollerButton` scrolls toward.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageScrollerButtonDirection {
    Start,
    #[default]
    End,
}

impl MessageScrollerButtonDirection {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// Viewport alignment for `scroll_to_message`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageScrollerScrollAlign {
    #[default]
    Start,
    Center,
    End,
    Nearest,
}

impl MessageScrollerScrollAlign {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Nearest => "nearest",
        }
    }
}

/// Native scroll behavior of a scroll command.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageScrollerScrollBehavior {
    #[default]
    Auto,
    Smooth,
    Instant,
}

impl MessageScrollerScrollBehavior {
    fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Smooth => "smooth",
            Self::Instant => "instant",
        }
    }
}

/// Options for `scroll_to_message`, `scroll_to_end` and `scroll_to_start`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MessageScrollerScrollOptions {
    /// Viewport edge or center to align the target to. Defaults to the start.
    pub align: Option<MessageScrollerScrollAlign>,
    /// Defaults to `Auto`.
    pub behavior: Option<MessageScrollerScrollBehavior>,
    /// Margin on the aligned edge, in pixels. Defaults to the provider's `scroll_margin`.
    pub scroll_margin: Option<f64>,
}

/// Which edges the viewport can still scroll toward.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MessageScrollerScrollable {
    /// Content is hidden above.
    pub start: bool,
    /// Content is hidden below.
    pub end: bool,
}

/// Where the reader is in the transcript.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageScrollerVisibilityState {
    /// The anchored turn the reader is in. Stays set after the anchor scrolls above the viewport.
    pub current_anchor_id: Option<String>,
    /// `message_id` values intersecting the viewport, in document order.
    pub visible_message_ids: Vec<String>,
}

#[derive(Clone, Copy, PartialEq)]
struct ProviderConfig {
    auto_scroll: bool,
    default_scroll_position: MessageScrollerDefaultScrollPosition,
    scroll_edge_threshold: f64,
    scroll_previous_item_peek: f64,
    scroll_margin: f64,
}

#[derive(Clone, Copy)]
struct MessageScrollerContext {
    id: CopyValue<String>,
    script: document::Eval,
    scrollable: Signal<MessageScrollerScrollable>,
    pending_scroll: Signal<bool>,
    visibility: Signal<MessageScrollerVisibilityState>,
    visibility_subscribers: CopyValue<usize>,
}

/// Sends one message, a tuple serialized as a JSON array, to the provider's scroller script.
/// A send fails only after the script has ended, when there is nothing left to drive.
macro_rules! send {
    ($context:expr, $message:expr) => {{
        let _ = $context.script.send($message);
    }};
}

/// Scroll commands for the nearest `MessageScrollerProvider`. Works anywhere inside the provider,
/// including controls rendered outside the `MessageScroller` frame.
#[derive(Clone, Copy)]
pub struct MessageScrollerCommands {
    context: MessageScrollerContext,
}

impl MessageScrollerCommands {
    /// Scrolls to the newest message. With `auto_scroll`, it also resumes following the live edge.
    pub fn scroll_to_end(&self, options: MessageScrollerScrollOptions) {
        send!(
            self.context,
            ("scrollToEnd", options.behavior.unwrap_or_default().as_str())
        );
    }

    /// Scrolls to the first message.
    pub fn scroll_to_start(&self, options: MessageScrollerScrollOptions) {
        send!(
            self.context,
            (
                "scrollToStart",
                options.behavior.unwrap_or_default().as_str()
            )
        );
    }

    /// Scrolls to the item with this `message_id`. Before any item has mounted, the jump is
    /// queued and runs once the item appears.
    pub fn scroll_to_message(&self, message_id: &str, options: MessageScrollerScrollOptions) {
        send!(
            self.context,
            (
                "scrollToMessage",
                message_id,
                options.align.map(MessageScrollerScrollAlign::as_str),
                options.behavior.map(MessageScrollerScrollBehavior::as_str),
                options.scroll_margin,
            )
        );
    }
}

fn use_scroller_context() -> MessageScrollerContext {
    use_context::<MessageScrollerContext>()
}

/// Returns the scroll commands of the nearest `MessageScrollerProvider`.
pub fn use_message_scroller() -> MessageScrollerCommands {
    MessageScrollerCommands {
        context: use_scroller_context(),
    }
}

/// Returns which edges the viewport can still scroll toward, and re-renders when that changes.
pub fn use_message_scroller_scrollable() -> MessageScrollerScrollable {
    *use_scroller_context().scrollable.read()
}

/// Returns the current anchored turn and the visible message ids. Tracking only runs while at
/// least one component uses this hook.
pub fn use_message_scroller_visibility() -> MessageScrollerVisibilityState {
    let context = use_scroller_context();
    use_hook(move || {
        let mut subscribers = context.visibility_subscribers;
        *subscribers.write() += 1;
        if *subscribers.read() == 1 {
            send!(context, ("observeVisibility",));
        }
    });
    use_drop(move || {
        let mut subscribers = context.visibility_subscribers;
        let remaining = subscribers.read().saturating_sub(1);
        *subscribers.write() = remaining;
        if remaining == 0 {
            send!(context, ("unobserveVisibility",));
        }
    });
    context.visibility.read().clone()
}

/// Returns true on the first render and whenever `value` differs from the previous render's.
fn use_changed<T: Clone + PartialEq + 'static>(value: &T) -> bool {
    let mut previous = use_hook(|| CopyValue::new(None::<T>));
    if previous.read().as_ref() == Some(value) {
        return false;
    }
    previous.set(Some(value.clone()));
    true
}

/// Headless root: owns the scroll state and behavior. Renders no element.
#[component]
pub fn MessageScrollerProvider(
    /// Follow new content at the bottom while the viewport is already at the end.
    #[props(default)]
    auto_scroll: bool,
    /// Opening position on the first non-empty render, applied once.
    #[props(default)]
    default_scroll_position: MessageScrollerDefaultScrollPosition,
    /// Distance from an edge that still counts as at the start or end.
    #[props(default = 8.0)]
    scroll_edge_threshold: f64,
    /// Extra top margin for a newly anchored row, added to `scroll_margin`.
    #[props(default = 64.0)]
    scroll_previous_item_peek: f64,
    /// Default margin on the aligned edge for commands and visibility.
    #[props(default = 0.0)]
    scroll_margin: f64,
    children: Element,
) -> Element {
    let context = use_hook(|| {
        let id = format!(
            "message-scroller-{}",
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        );
        MessageScrollerContext {
            id: CopyValue::new(id),
            script: document::eval(CONTROLLER_SCRIPT),
            scrollable: Signal::new(MessageScrollerScrollable::default()),
            pending_scroll: Signal::new(matches!(
                default_scroll_position,
                MessageScrollerDefaultScrollPosition::End
                    | MessageScrollerDefaultScrollPosition::LastAnchor
            )),
            visibility: Signal::new(MessageScrollerVisibilityState::default()),
            visibility_subscribers: CopyValue::new(0),
        }
    });
    use_context_provider(|| context);

    let config = ProviderConfig {
        auto_scroll,
        default_scroll_position,
        scroll_edge_threshold,
        scroll_previous_item_peek,
        scroll_margin,
    };
    if use_changed(&config) {
        send!(
            context,
            (
                "config",
                context.id.read().clone(),
                config.auto_scroll,
                config.default_scroll_position.as_str(),
                config.scroll_edge_threshold,
                config.scroll_previous_item_peek,
                config.scroll_margin,
            )
        );
    }

    use_future(move || async move {
        let mut script = context.script;
        let mut scrollable = context.scrollable;
        let mut pending_scroll = context.pending_scroll;
        let mut visibility = context.visibility;
        while let Ok(message) = script.recv::<Vec<String>>().await {
            match message.first().map(String::as_str) {
                Some("scrollable") => {
                    let next = MessageScrollerScrollable {
                        start: message.get(1).is_some_and(|value| value == "1"),
                        end: message.get(2).is_some_and(|value| value == "1"),
                    };
                    if *scrollable.peek() != next {
                        scrollable.set(next);
                    }
                }
                Some("pending") => {
                    let next = message.get(1).is_some_and(|value| value == "1");
                    if *pending_scroll.peek() != next {
                        pending_scroll.set(next);
                    }
                }
                Some("visibility") => {
                    let has_anchor = message.get(1).is_some_and(|value| value == "1");
                    let next = MessageScrollerVisibilityState {
                        current_anchor_id: if has_anchor {
                            message.get(2).cloned()
                        } else {
                            None
                        },
                        visible_message_ids: message.iter().skip(3).cloned().collect(),
                    };
                    if *visibility.peek() != next {
                        visibility.set(next);
                    }
                }
                _ => {}
            }
        }
    });

    use_effect(move || send!(context, ("bind",)));
    use_drop(move || send!(context, ("destroy",)));

    rsx! {
        {children}
    }
}

/// The styled frame. Fills its parent, so place it inside a height-constrained container.
#[component]
pub fn MessageScroller(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_scroller_context();
    let class = cn!(
        "group/message-scroller relative flex size-full min-h-0 flex-col overflow-hidden",
        take_class(&mut attributes)
    );
    let pending = *context.pending_scroll.read();

    rsx! {
        div {
            "data-slot": "message-scroller",
            "data-message-scroller-root": context.id.read().clone(),
            "data-pending-scroll": pending.then_some(""),
            class: class,
            onmounted: move |_| send!(context, ("bind",)),
            ..attributes,
            {children}
        }
    }
}

/// The scrollable element: a labelled, keyboard-focusable region. Keeps the first visible row in
/// place when older rows are prepended.
#[component]
pub fn MessageScrollerViewport(
    /// Keep the first visible `message_id` row stable when rows are prepended.
    #[props(default = true)]
    preserve_scroll_on_prepend: bool,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onscroll: Option<EventHandler<ScrollEvent>>,
    ontouchmove: Option<EventHandler<TouchEvent>>,
    onwheel: Option<EventHandler<WheelEvent>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_scroller_context();
    let class = cn!(
        "size-full min-h-0 min-w-0 scroll-fade-b scrollbar-thin scrollbar-gutter-stable overflow-y-auto overscroll-contain contain-content data-autoscrolling:scrollbar-thumb-transparent data-autoscrolling:scrollbar-track-transparent data-pending-scroll:invisible",
        take_class(&mut attributes)
    );
    let role = take_attribute(&mut attributes, "role").unwrap_or_else(|| "region".into());
    let label = take_attribute(&mut attributes, "aria-label").unwrap_or_else(|| "Messages".into());
    let tabindex = take_attribute(&mut attributes, "tabindex").unwrap_or_else(|| "0".into());
    let pending = *context.pending_scroll.read();

    if use_changed(&preserve_scroll_on_prepend) {
        send!(context, ("preserve", preserve_scroll_on_prepend));
    }

    rsx! {
        div {
            "data-slot": "message-scroller-viewport",
            "data-message-scroller-viewport": context.id.read().clone(),
            "data-pending-scroll": pending.then_some(""),
            role: role,
            aria_label: label,
            tabindex: tabindex,
            class: class,
            onmounted: move |_| send!(context, ("bind",)),
            onkeydown: move |event| {
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            onscroll: move |event| {
                if let Some(handler) = onscroll {
                    handler.call(event);
                }
            },
            ontouchmove: move |event| {
                if let Some(handler) = ontouchmove {
                    handler.call(event);
                }
            },
            onwheel: move |event| {
                if let Some(handler) = onwheel {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// The transcript container: a live region (`role="log"`, `aria-relevant="additions"`). Wrap each
/// direct child in a `MessageScrollerItem`.
#[component]
pub fn MessageScrollerContent(
    /// Classes for the internal tail spacer that makes room to anchor a turn near the top.
    spacer_class: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_scroller_context();
    let class = cn!(
        "gap-6 flex h-max min-h-full flex-col",
        take_class(&mut attributes)
    );
    let role = take_attribute(&mut attributes, "role").unwrap_or_else(|| "log".into());
    let relevant =
        take_attribute(&mut attributes, "aria-relevant").unwrap_or_else(|| "additions".into());

    rsx! {
        div {
            "data-slot": "message-scroller-content",
            "data-message-scroller-content": context.id.read().clone(),
            role: role,
            aria_relevant: relevant,
            class: class,
            onmounted: move |_| send!(context, ("bind",)),
            ..attributes,
            {children}
            div {
                aria_hidden: "true",
                "data-message-scroller-spacer": "",
                hidden: true,
                class: spacer_class,
            }
        }
    }
}

/// One transcript row: a message, marker, typing row, separator or "load earlier" row.
#[component]
pub fn MessageScrollerItem(
    /// Stable row id for `scroll_to_message`, visibility and prepend preservation.
    message_id: Option<String>,
    /// Marks the row that starts a turn: newly appended anchors settle near the top.
    #[props(default)]
    scroll_anchor: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "min-w-0 shrink-0 [contain-intrinsic-size:auto_10rem] [content-visibility:auto]",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "message-scroller-item",
            "data-message-id": message_id,
            "data-scroll-anchor": if scroll_anchor { "true" } else { "false" },
            class: class,
            ..attributes,
            {children}
        }
    }
}

fn button_variant_name(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => "default",
        ButtonVariant::Outline => "outline",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Ghost => "ghost",
        ButtonVariant::Destructive => "destructive",
        ButtonVariant::Link => "link",
    }
}

fn button_size_name(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "default",
        ButtonSize::Xs => "xs",
        ButtonSize::Sm => "sm",
        ButtonSize::Lg => "lg",
        ButtonSize::Icon => "icon",
        ButtonSize::IconXs => "icon-xs",
        ButtonSize::IconSm => "icon-sm",
        ButtonSize::IconLg => "icon-lg",
    }
}

/// Scrolls to the start or the end of the transcript. Inert and hidden until there is content in
/// its direction.
#[component]
pub fn MessageScrollerButton(
    #[props(default)] direction: MessageScrollerButtonDirection,
    #[props(default = ButtonVariant::Secondary)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconSm)] size: ButtonSize,
    /// Native scroll behavior when clicked.
    #[props(default = MessageScrollerScrollBehavior::Smooth)]
    behavior: MessageScrollerScrollBehavior,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let context = use_scroller_context();
    let scrollable = *context.scrollable.read();
    let active = match direction {
        MessageScrollerButtonDirection::Start => scrollable.start,
        MessageScrollerButtonDirection::End => scrollable.end,
    };
    let scroller_class = cn!(
        "absolute inset-s-1/2 -translate-x-1/2 border-border bg-background text-foreground transition-[translate,scale,opacity] duration-200 hover:bg-muted hover:text-foreground data-[active=false]:pointer-events-none data-[active=false]:scale-95 data-[active=false]:opacity-0 data-[active=false]:duration-400 data-[active=false]:ease-[cubic-bezier(0.7,0,0.84,0)] data-[active=true]:translate-y-0 data-[active=true]:scale-100 data-[active=true]:opacity-100 data-[active=true]:ease-[cubic-bezier(0.23,1,0.32,1)] data-[direction=end]:bottom-4 data-[direction=end]:data-[active=false]:translate-y-full data-[direction=start]:top-4 data-[direction=start]:data-[active=false]:-translate-y-full rtl:translate-x-1/2 data-[direction=start]:[&_svg]:rotate-180",
        take_class(&mut attributes)
    );
    let class = button_variants(variant, size, Some(&scroller_class));
    let tabindex = take_attribute(&mut attributes, "tabindex");
    let tabindex = if active { tabindex } else { Some("-1".into()) };
    let mut node = use_signal(|| None::<std::rc::Rc<MountedData>>);

    rsx! {
        button {
            "data-slot": "message-scroller-button",
            "data-direction": direction.as_str(),
            "data-variant": button_variant_name(variant),
            "data-size": button_size_name(size),
            "data-active": if active { "true" } else { "false" },
            r#type: "button",
            "inert": (!active).then_some("true"),
            tabindex: tabindex,
            class: class,
            onmounted: move |event| node.set(Some(event.data())),
            onclick: move |event| {
                if !active {
                    return;
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                if let Some(node) = node.peek().clone() {
                    spawn(async move {
                        let _ = node.set_focus(false).await;
                    });
                }
                let behavior = behavior.as_str();
                match direction {
                    MessageScrollerButtonDirection::Start => send!(context, ("scrollToStart", behavior)),
                    MessageScrollerButtonDirection::End => send!(context, ("scrollToEnd", behavior)),
                }
            },
            ..attributes,
            match children {
                Some(children) => children,
                None => rsx! {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 5v14" }
                        path { d: "m19 12-7 7-7-7" }
                    }
                    span { class: "sr-only",
                        if direction == MessageScrollerButtonDirection::End {
                            "Scroll to end"
                        } else {
                            "Scroll to start"
                        }
                    }
                },
            }
        }
    }
}

/// Removes a plain attribute from a spread list and returns its text value, so a part can supply
/// a default the caller may override.
fn take_attribute(attributes: &mut Vec<Attribute>, name: &str) -> Option<String> {
    let mut value = None;
    attributes.retain(|attribute| {
        if attribute.name != name || attribute.namespace.is_some() {
            return true;
        }
        value = match &attribute.value {
            AttributeValue::Text(text) => Some(text.clone()),
            AttributeValue::Int(number) => Some(number.to_string()),
            AttributeValue::Float(number) => Some(number.to_string()),
            AttributeValue::Bool(flag) => Some(flag.to_string()),
            _ => None,
        };
        false
    });
    value
}

const CONTROLLER_SCRIPT: &str = r#"
const EPSILON = 0.5;
const AUTOSCROLLING_CLEAR_DELAY = 180;
const USER_SCROLL_KEYS = new Set(["ArrowDown", "ArrowUp", "End", "Home", "PageDown", "PageUp", " "]);

const s = {
  id: null,
  autoScroll: false,
  defaultScrollPosition: "end",
  edge: 8,
  peek: 64,
  margin: 0,
  mode: "free-scrolling",
  autoscrolling: false,
  autoscrollingTimeout: null,
  streamingTurn: null,
  root: null,
  viewport: null,
  content: null,
  spacer: null,
  defaultApplied: false,
  firstItem: null,
  itemCount: 0,
  lastScrollTop: 0,
  pendingScrollToMessage: null,
  prependRestore: null,
  preserve: true,
  pendingDefault: false,
  spacerGap: 0,
  spacerHeight: 0,
  stateFrame: null,
  visibilityFrame: null,
  visibilityActive: false,
  visibilityObserver: null,
  visibleIds: new Set(),
  observed: new Set(),
  handledAnchors: new WeakSet(),
  scrollable: { start: false, end: false },
  visibilityKey: "",
  bound: false,
  destroyed: false,
  teardownViewport: null,
  teardownContent: null,
};

const raf = (fn) => window.requestAnimationFrame(() => { if (!s.destroyed) fn(); });

const readPx = (value) => {
  const number = Number.parseFloat(value || "");
  return Number.isFinite(number) ? number : 0;
};
const blockPadding = (element) => {
  const style = window.getComputedStyle(element);
  return {
    start: readPx(style.paddingBlockStart || style.paddingTop),
    end: readPx(style.paddingBlockEnd || style.paddingBottom),
  };
};
const contentBlockPadding = () => {
  const content = s.spacer && s.spacer.parentElement;
  return content ? blockPadding(content) : { start: 0, end: 0 };
};
const flexGap = (element) => {
  if (!element) return 0;
  const style = window.getComputedStyle(element);
  return readPx(style.rowGap === "normal" ? style.gap : style.rowGap);
};
const items = () =>
  s.content ? Array.from(s.content.children).filter((child) => child instanceof HTMLElement && child !== s.spacer) : [];
const isAnchor = (item) => item && item.dataset.scrollAnchor === "true";
const messageElement = (messageId) => {
  if (!s.content) return null;
  for (const element of s.content.querySelectorAll("[data-message-id]")) {
    if (element.dataset.messageId === messageId) return element;
  }
  return null;
};
const elementTop = (element) =>
  element.getBoundingClientRect().top - s.viewport.getBoundingClientRect().top + s.viewport.scrollTop;
const elementViewportTop = (element) =>
  element.getBoundingClientRect().top - s.viewport.getBoundingClientRect().top;
const maxScrollTop = () => Math.max(0, s.viewport.scrollHeight - s.viewport.clientHeight);
const contentBottom = () => {
  const padding = blockPadding(s.content);
  const viewportTop = s.viewport.getBoundingClientRect().top;
  const scrollTop = s.viewport.scrollTop;
  let bottom = padding.start + padding.end;
  for (const item of items()) {
    bottom = Math.max(bottom, item.getBoundingClientRect().bottom - viewportTop + scrollTop + padding.end);
  }
  return bottom;
};
const tailSpacerHeight = (scrollTop) => scrollTop + s.viewport.clientHeight - contentBottom();
const elementScrollTop = (element, align, scrollMargin) => {
  const viewport = s.viewport;
  const top = elementTop(element);
  const height = element.getBoundingClientRect().height;
  const padding = contentBlockPadding();
  if (align === "center") {
    const inset = Math.max(0, viewport.clientHeight - padding.start - padding.end);
    return top - padding.start - (inset - height) / 2 - scrollMargin;
  }
  if (align === "end") {
    return top - viewport.clientHeight + height + padding.end + scrollMargin;
  }
  if (align === "nearest") {
    const bottom = top + height;
    const viewTop = viewport.scrollTop + padding.start;
    const viewBottom = viewport.scrollTop + viewport.clientHeight - padding.end;
    if (top >= viewTop && bottom <= viewBottom) return viewport.scrollTop;
    if (top < viewTop) return top - padding.start - scrollMargin;
    return bottom - viewport.clientHeight + padding.end + scrollMargin;
  }
  return top - padding.start - scrollMargin;
};

const computeScrollable = () => {
  if (!s.viewport || !s.content) return { start: false, end: false };
  const bottom = contentBottom();
  return {
    start: s.viewport.scrollTop > s.edge,
    end: bottom - s.viewport.scrollTop - s.viewport.clientHeight > s.edge,
  };
};

const writeStateAttributes = (state) => {
  const scrollable = [state.start && "start", state.end && "end"].filter(Boolean).join(" ");
  for (const element of [s.root, s.viewport]) {
    if (!element) continue;
    if (scrollable) element.setAttribute("data-scrollable", scrollable);
    else element.removeAttribute("data-scrollable");
    element.toggleAttribute("data-autoscrolling", s.autoscrolling);
  }
};

const publishScrollable = (state) => {
  if (state.start === s.scrollable.start && state.end === s.scrollable.end) return;
  s.scrollable = state;
  dioxus.send(["scrollable", state.start ? "1" : "0", state.end ? "1" : "0"]);
};

const setPendingDefault = (pending) => {
  if (s.pendingDefault === pending) return;
  s.pendingDefault = pending;
  if (!pending) {
    for (const element of [s.root, s.viewport]) element && element.removeAttribute("data-pending-scroll");
  }
  dioxus.send(["pending", pending ? "1" : "0"]);
};
const markDefaultApplied = () => {
  s.defaultApplied = true;
  setPendingDefault(false);
};

const reconcileFollowMode = (scrollable) => {
  const scrollTop = s.viewport ? s.viewport.scrollTop : 0;
  const scrolledUp = scrollTop < s.lastScrollTop - EPSILON;
  s.lastScrollTop = scrollTop;
  if (s.autoScroll && !scrollable.end && s.spacerHeight === 0 && s.mode !== "settling-jump" && s.mode !== "anchored-to-message") {
    s.mode = "following-bottom";
  } else if (s.mode === "following-bottom" && scrollable.end && scrolledUp && !s.autoscrolling) {
    s.mode = "free-scrolling";
  }
};

const commitScrollState = () => {
  const next = computeScrollable();
  reconcileFollowMode(next);
  const published = s.mode === "following-bottom" ? { start: next.start, end: false } : next;
  writeStateAttributes(published);
  publishScrollable(published);
};

const scheduleStateCommit = () => {
  if (s.stateFrame !== null) return;
  s.stateFrame = window.requestAnimationFrame(() => {
    s.stateFrame = null;
    if (!s.destroyed) commitScrollState();
  });
};

const computeVisibility = () => {
  if (!s.content || !s.viewport) return { currentAnchorId: null, visibleMessageIds: [] };
  const viewportRect = s.viewport.getBoundingClientRect();
  const lineTop = viewportRect.top + s.margin + s.peek;
  const byLayout = typeof IntersectionObserver === "undefined";
  const visible = [];
  let currentAnchorId = null;
  for (const item of items()) {
    const messageId = item.dataset.messageId;
    if (!messageId) continue;
    const anchor = isAnchor(item);
    const rect = anchor || byLayout ? item.getBoundingClientRect() : null;
    const isVisible = byLayout && rect ? rect.bottom > lineTop && rect.top < viewportRect.bottom : s.visibleIds.has(messageId);
    if (isVisible) visible.push(messageId);
    if (anchor && rect && rect.top <= lineTop + EPSILON) currentAnchorId = messageId;
  }
  return { currentAnchorId, visibleMessageIds: visible };
};

const publishVisibility = (state) => {
  const key = JSON.stringify(state);
  if (key === s.visibilityKey) return;
  s.visibilityKey = key;
  dioxus.send([
    "visibility",
    state.currentAnchorId === null ? "0" : "1",
    state.currentAnchorId === null ? "" : state.currentAnchorId,
    ...state.visibleMessageIds,
  ]);
};

const scheduleVisibilitySync = () => {
  if (!s.visibilityActive || s.visibilityFrame !== null) return;
  s.visibilityFrame = window.requestAnimationFrame(() => {
    s.visibilityFrame = null;
    if (s.destroyed || !s.visibilityActive) return;
    publishVisibility(computeVisibility());
  });
};

const setAutoScrolling = (autoscrolling) => {
  if (s.autoscrollingTimeout !== null) {
    window.clearTimeout(s.autoscrollingTimeout);
    s.autoscrollingTimeout = null;
  }
  if (s.autoscrolling !== autoscrolling) {
    s.autoscrolling = autoscrolling;
    commitScrollState();
  }
  if (autoscrolling) {
    s.autoscrollingTimeout = window.setTimeout(() => {
      s.autoscrollingTimeout = null;
      s.autoscrolling = false;
      if (!s.destroyed) commitScrollState();
    }, AUTOSCROLLING_CLEAR_DELAY);
  }
};

const setTailSpacerHeight = (height) => {
  const spacer = s.spacer;
  if (!spacer) return;
  const next = Math.max(0, Math.ceil(height));
  if (s.spacerHeight === next) return;
  s.spacerHeight = next;
  spacer.hidden = next === 0;
  spacer.style.height = `${next}px`;
  spacer.style.marginTop = next > 0 ? `${-s.spacerGap}px` : "";
};

const scrollToPosition = (scrollTop, behavior = "auto", autoscrolling = false) => {
  const viewport = s.viewport;
  if (!viewport) return;
  const next = Math.max(0, scrollTop);
  if (Math.abs(viewport.scrollTop - next) <= EPSILON) {
    viewport.scrollTop = next;
    commitScrollState();
    return;
  }
  if (autoscrolling) setAutoScrolling(true);
  viewport.scrollTo({ top: next, behavior });
  scheduleStateCommit();
};

const scrollToStart = (behavior = "auto") => {
  if (!s.viewport) return false;
  setTailSpacerHeight(0);
  s.streamingTurn = null;
  s.mode = "free-scrolling";
  scrollToPosition(0, behavior);
  scheduleVisibilitySync();
  return true;
};

const scrollToEnd = (behavior = "auto") => {
  if (!s.viewport) return false;
  setTailSpacerHeight(0);
  s.streamingTurn = null;
  s.mode = s.autoScroll ? "following-bottom" : "free-scrolling";
  scrollToPosition(maxScrollTop(), behavior, true);
  scheduleVisibilitySync();
  return true;
};

const scrollToElement = (element, options = {}, keepPreviousPeek = false) => {
  const { content, viewport } = s;
  if (!content || !viewport || !content.contains(element)) return false;
  const align = options.align || "start";
  const behavior = options.behavior || "auto";
  const scrollMargin = options.scrollMargin ?? s.margin;
  const scrollTop = elementScrollTop(element, align, keepPreviousPeek ? scrollMargin + s.peek : scrollMargin);
  setTailSpacerHeight(tailSpacerHeight(scrollTop));
  s.prependRestore = { element, viewportTop: elementViewportTop(element) };
  s.mode = keepPreviousPeek ? "anchored-to-message" : "settling-jump";
  s.streamingTurn = keepPreviousPeek ? element : null;
  scrollToPosition(scrollTop, behavior);
  scheduleVisibilitySync();
  return true;
};

const reanchorToAnchoredMessage = () => {
  const element = s.streamingTurn;
  if (!element || !element.isConnected || s.mode !== "anchored-to-message") return false;
  return scrollToElement(element, { align: "start" }, true);
};

const scrollToMessage = (messageId, options) => {
  const element = messageElement(messageId);
  if (!element) {
    if (s.itemCount === 0) {
      s.pendingScrollToMessage = { messageId, options };
      markDefaultApplied();
      return true;
    }
    return false;
  }
  markDefaultApplied();
  if (scrollToElement(element, options)) {
    s.pendingScrollToMessage = null;
    return true;
  }
  s.pendingScrollToMessage = { messageId, options };
  return true;
};

const flushPendingScrollToMessage = () => {
  const pending = s.pendingScrollToMessage;
  if (!pending) return false;
  const element = messageElement(pending.messageId);
  if (!element || !scrollToElement(element, pending.options)) return false;
  s.pendingScrollToMessage = null;
  markDefaultApplied();
  return true;
};

const restorePrependedAnchor = () => {
  const anchor = s.prependRestore;
  if (!anchor || !s.viewport || !anchor.element.isConnected) return false;
  const delta = elementViewportTop(anchor.element) - anchor.viewportTop;
  if (Math.abs(delta) <= EPSILON) return false;
  s.viewport.scrollTop += delta;
  anchor.viewportTop = elementViewportTop(anchor.element);
  scheduleStateCommit();
  scheduleVisibilitySync();
  return true;
};

const capturePrependAnchor = () => {
  if (!s.content || !s.viewport) {
    s.prependRestore = null;
    return;
  }
  const viewportRect = s.viewport.getBoundingClientRect();
  let anchor = null;
  for (const item of items()) {
    if (!item.dataset.messageId) continue;
    const rect = item.getBoundingClientRect();
    if (rect.bottom > viewportRect.top && rect.top < viewportRect.bottom) {
      anchor = item;
      break;
    }
  }
  s.prependRestore = anchor ? { element: anchor, viewportTop: elementViewportTop(anchor) } : null;
};

const applyDefaultScrollPosition = () => {
  if (!s.defaultScrollPosition || s.defaultApplied || s.itemCount === 0) return false;
  let handled = false;
  if (s.defaultScrollPosition === "last-anchor") {
    const list = items();
    let anchor = null;
    for (let index = list.length - 1; index >= 0; index--) {
      if (isAnchor(list[index])) {
        anchor = list[index];
        break;
      }
    }
    if (!s.content || !s.viewport || !anchor) {
      handled = scrollToEnd("auto");
    } else {
      const lastTurnFits = contentBottom() - elementTop(anchor) <= s.viewport.clientHeight;
      handled = lastTurnFits ? scrollToEnd("auto") : scrollToElement(anchor, { align: "start" }, true);
    }
  } else {
    handled = s.defaultScrollPosition === "end" ? scrollToEnd("auto") : scrollToStart("auto");
  }
  if (!handled) return false;
  markDefaultApplied();
  return true;
};

const syncObservedItems = () => {
  const current = new Set();
  if (s.content) {
    for (const element of s.content.querySelectorAll("[data-message-id]")) current.add(element);
  }
  for (const element of s.observed) {
    if (!current.has(element)) {
      s.observed.delete(element);
      if (s.visibilityObserver) s.visibilityObserver.unobserve(element);
      const messageId = element.dataset.messageId;
      if (messageId && !messageElement(messageId)) s.visibleIds.delete(messageId);
    }
  }
  for (const element of current) {
    if (!s.observed.has(element)) {
      s.observed.add(element);
      if (s.visibilityObserver) s.visibilityObserver.observe(element);
    }
  }
};

const handleContentChange = () => {
  if (!s.content) return;
  syncObservedItems();
  const list = items();
  const previousItemCount = s.itemCount;
  const previousFirstItem = s.firstItem;
  s.itemCount = list.length;
  s.firstItem = list[0] || null;

  const reconcile = () => {
    if (flushPendingScrollToMessage()) return;
    if (previousItemCount === 0) {
      if (applyDefaultScrollPosition()) return;
      if (list.length > 0 && s.autoScroll && scrollToEnd("auto")) return;
      commitScrollState();
      scheduleVisibilitySync();
      return;
    }
    const previousFirstIndex = previousFirstItem ? list.indexOf(previousFirstItem) : -1;
    if (s.preserve && previousFirstIndex > 0) {
      restorePrependedAnchor();
      return;
    }
    if (list.length > previousItemCount) {
      let anchor = null;
      let anchors = 0;
      for (let index = previousItemCount; index < list.length; index++) {
        if (isAnchor(list[index])) {
          anchor = anchor || list[index];
          anchors += 1;
        }
      }
      if (anchor) {
        if (s.autoScroll && s.mode === "following-bottom" && anchors > 1) {
          scrollToEnd("auto");
          return;
        }
        scrollToElement(anchor, { align: "start" }, true);
        s.handledAnchors.add(anchor);
        return;
      }
    }
    if (list.length === previousItemCount) {
      const anchor = list.find((item) => isAnchor(item) && !s.handledAnchors.has(item));
      if (anchor) {
        scrollToElement(anchor, { align: "start" }, true);
        s.handledAnchors.add(anchor);
        return;
      }
    }
    if (s.mode === "following-bottom" && s.autoScroll) {
      scrollToEnd("auto");
    } else {
      commitScrollState();
      scheduleVisibilitySync();
    }
  };

  reconcile();
  capturePrependAnchor();
};

const drainReleasedSpacer = () => {
  if (!s.autoScroll || s.spacerHeight <= 0 || s.mode === "following-bottom" || s.mode === "anchored-to-message" || s.mode === "settling-jump" || s.autoscrolling) {
    return false;
  }
  if (!s.viewport || !s.content) return false;
  const bottom = contentBottom();
  const overlap = s.viewport.scrollTop + s.viewport.clientHeight - bottom;
  if (overlap <= 0) return false;
  const next = Math.max(0, bottom + s.spacerHeight - (s.viewport.scrollTop + s.viewport.clientHeight));
  if (next === 0) {
    scrollToEnd("auto");
    return true;
  }
  setTailSpacerHeight(next);
  return false;
};

const handleResize = () => {
  if (s.destroyed) return;
  if (s.mode === "following-bottom" && s.autoScroll) {
    scrollToEnd("auto");
    return;
  }
  const previousSpacerHeight = s.spacerHeight;
  if (reanchorToAnchoredMessage()) {
    if (s.autoScroll && previousSpacerHeight > 0 && s.spacerHeight === 0) scrollToEnd("auto");
    return;
  }
  if (drainReleasedSpacer()) return;
  scheduleStateCommit();
  scheduleVisibilitySync();
};

const userScrollIntent = () => {
  if (s.mode === "following-bottom" || s.mode === "anchored-to-message" || s.mode === "settling-jump") {
    s.streamingTurn = null;
    s.mode = "free-scrolling";
  }
};

const syncAfterScroll = () => {
  drainReleasedSpacer();
  commitScrollState();
  scheduleVisibilitySync();
  capturePrependAnchor();
};

const buildVisibilityObserver = () => {
  if (s.visibilityObserver) {
    s.visibilityObserver.disconnect();
    s.visibilityObserver = null;
  }
  if (!s.visibilityActive || !s.viewport || typeof IntersectionObserver === "undefined") return;
  s.visibilityObserver = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        const messageId = entry.target.dataset.messageId;
        if (!messageId) continue;
        if (entry.isIntersecting) s.visibleIds.add(messageId);
        else s.visibleIds.delete(messageId);
      }
      scheduleVisibilitySync();
    },
    { root: s.viewport, rootMargin: `${-(s.margin + s.peek)}px 0px 0px 0px`, threshold: [0, 0.01, 0.5, 1] },
  );
  for (const element of s.observed) s.visibilityObserver.observe(element);
};

const observeVisibility = () => {
  s.visibilityActive = true;
  s.visibleIds.clear();
  buildVisibilityObserver();
  scheduleVisibilitySync();
};

const unobserveVisibility = () => {
  s.visibilityActive = false;
  if (s.visibilityFrame !== null) {
    window.cancelAnimationFrame(s.visibilityFrame);
    s.visibilityFrame = null;
  }
  if (s.visibilityObserver) s.visibilityObserver.disconnect();
  s.visibilityObserver = null;
  s.visibleIds.clear();
  publishVisibility({ currentAnchorId: null, visibleMessageIds: [] });
};

const observeResize = (element) => {
  if (typeof ResizeObserver === "undefined") return () => {};
  let frame = 0;
  const observer = new ResizeObserver(() => {
    window.cancelAnimationFrame(frame);
    frame = window.requestAnimationFrame(handleResize);
  });
  observer.observe(element);
  return () => {
    window.cancelAnimationFrame(frame);
    observer.disconnect();
  };
};

const bindViewport = (viewport) => {
  const onScroll = () => syncAfterScroll();
  const onIntent = () => userScrollIntent();
  const onKeyDown = (event) => {
    if (USER_SCROLL_KEYS.has(event.key)) userScrollIntent();
  };
  viewport.addEventListener("scroll", onScroll, { passive: true });
  viewport.addEventListener("wheel", onIntent, { passive: true });
  viewport.addEventListener("touchmove", onIntent, { passive: true });
  viewport.addEventListener("keydown", onKeyDown);
  const stopResize = observeResize(viewport);
  return () => {
    viewport.removeEventListener("scroll", onScroll);
    viewport.removeEventListener("wheel", onIntent);
    viewport.removeEventListener("touchmove", onIntent);
    viewport.removeEventListener("keydown", onKeyDown);
    stopResize();
  };
};

const bindContent = (content) => {
  const mutations = new MutationObserver(() => handleContentChange());
  mutations.observe(content, { childList: true });
  const stopResize = observeResize(content);
  return () => {
    mutations.disconnect();
    stopResize();
  };
};

const find = (attribute) => {
  for (const element of document.querySelectorAll(`[${attribute}]`)) {
    if (element.getAttribute(attribute) === s.id) return element;
  }
  return null;
};

const bind = () => {
  const root = find("data-message-scroller-root");
  const viewport = find("data-message-scroller-viewport");
  const content = find("data-message-scroller-content");

  if (root !== s.root) {
    s.root = root;
  }
  if (viewport !== s.viewport) {
    if (s.teardownViewport) s.teardownViewport();
    s.teardownViewport = null;
    s.viewport = viewport;
    if (viewport) s.teardownViewport = bindViewport(viewport);
    buildVisibilityObserver();
  }
  writeStateAttributes(s.scrollable);
  if (!s.pendingDefault) {
    for (const element of [s.root, s.viewport]) element && element.removeAttribute("data-pending-scroll");
  }
  if (content !== s.content) {
    if (s.teardownContent) s.teardownContent();
    s.teardownContent = null;
    s.content = content;
    s.spacer = content ? content.querySelector(":scope > [data-message-scroller-spacer]") : null;
    s.spacerGap = flexGap(content);
    s.spacerHeight = 0;
    if (content) {
      handleContentChange();
      s.teardownContent = bindContent(content);
    }
  }

  if (!s.bound) {
    s.bound = true;
    if (!applyDefaultScrollPosition() && s.itemCount === 0) setPendingDefault(false);
    if (s.autoScroll && s.mode === "following-bottom" && s.itemCount > 0) scrollToEnd("auto");
    else commitScrollState();
  }
};

const configure = (message) => {
  const [, id, autoScroll, defaultScrollPosition, edge, peek, margin] = message;
  const first = s.id === null;
  s.id = id;
  const autoScrollChanged = s.autoScroll !== autoScroll;
  s.autoScroll = autoScroll;
  if (first) {
    s.mode = autoScroll ? "following-bottom" : "free-scrolling";
    s.pendingDefault = defaultScrollPosition === "end" || defaultScrollPosition === "last-anchor";
  } else if (s.defaultScrollPosition !== defaultScrollPosition) {
    s.defaultApplied = false;
  }
  const positionChanged = s.defaultScrollPosition !== defaultScrollPosition;
  s.defaultScrollPosition = defaultScrollPosition;
  const marginsChanged = s.peek !== peek || s.margin !== margin;
  s.edge = edge;
  s.peek = peek;
  s.margin = margin;
  if (first || !s.bound) return;
  if (marginsChanged) buildVisibilityObserver();
  if (positionChanged) applyDefaultScrollPosition();
  if (autoScrollChanged) {
    if (s.autoScroll && s.mode === "following-bottom" && s.itemCount > 0) scrollToEnd("auto");
    else commitScrollState();
  }
};

const destroy = () => {
  s.destroyed = true;
  if (s.teardownViewport) s.teardownViewport();
  if (s.teardownContent) s.teardownContent();
  if (s.visibilityObserver) s.visibilityObserver.disconnect();
  if (s.autoscrollingTimeout !== null) window.clearTimeout(s.autoscrollingTimeout);
  if (s.stateFrame !== null) window.cancelAnimationFrame(s.stateFrame);
  if (s.visibilityFrame !== null) window.cancelAnimationFrame(s.visibilityFrame);
};

const behaviorOf = (value) => (value === "smooth" || value === "instant" ? value : "auto");

while (!s.destroyed) {
  const message = await dioxus.recv();
  if (!Array.isArray(message)) continue;
  if (s.bound && ((s.viewport && !s.viewport.isConnected) || (s.content && !s.content.isConnected))) bind();
  switch (message[0]) {
    case "config":
      configure(message);
      break;
    case "bind":
      if (s.id !== null) bind();
      break;
    case "preserve":
      s.preserve = message[1] !== false;
      break;
    case "scrollToEnd":
      scrollToEnd(behaviorOf(message[1]));
      break;
    case "scrollToStart":
      scrollToStart(behaviorOf(message[1]));
      break;
    case "scrollToMessage": {
      const [, messageId, align, behavior, scrollMargin] = message;
      const options = {};
      if (align) options.align = align;
      if (behavior) options.behavior = behaviorOf(behavior);
      if (typeof scrollMargin === "number") options.scrollMargin = scrollMargin;
      scrollToMessage(messageId, options);
      break;
    }
    case "observeVisibility":
      observeVisibility();
      break;
    case "unobserveVisibility":
      unobserveVisibility();
      break;
    case "destroy":
      destroy();
      break;
  }
}
"#;
