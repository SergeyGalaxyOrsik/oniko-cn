//! HoverCard: a preview card that opens when the pointer rests on its trigger or the trigger is focused.
//!
//! Like Base UI's `PreviewCard`, the root renders no element and the trigger is an `<a>`. The card opens
//! after the trigger's `delay` (600 ms) and closes `close_delay` (300 ms) after the pointer left both the
//! trigger and the card. Delays run `setTimeout` through `document::eval`, because `tokio::time` is not
//! available in the browser. The card is placed with `position: fixed` from the trigger's `MountedData`
//! rect, flipping and shifting at the viewport edges, and is not moved to the end of `<body>`, so an
//! ancestor with a `transform` becomes its containing block.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

impl Edge {
    fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
        }
    }

    fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    fn name(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Anchoring {
    Physical(Edge),
    InlineStart,
    InlineEnd,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, PartialEq)]
struct Request {
    side: Anchoring,
    align: Alignment,
    side_offset: f64,
    align_offset: f64,
}

struct Geometry {
    anchor: [f64; 4],
    popup: [f64; 2],
    viewport: [f64; 2],
    rtl: bool,
}

#[derive(Clone, PartialEq)]
struct Placed {
    side: &'static str,
    align: &'static str,
    style: String,
}

const COLLISION_PADDING: f64 = 5.0;
const ARROW_SIZE: f64 = 10.0;

fn shift(value: f64, length: f64, viewport: f64) -> f64 {
    let max = viewport - COLLISION_PADDING - length;
    if max < COLLISION_PADDING {
        COLLISION_PADDING
    } else {
        value.clamp(COLLISION_PADDING, max)
    }
}

/// Places the popup next to the anchor like Floating UI does for Base UI: the preferred side first, the
/// opposite side when the preferred one overflows more, then a shift along the cross axis to stay on screen.
fn compute(request: Request, geometry: &Geometry) -> Placed {
    let Geometry {
        anchor: [ax, ay, aw, ah],
        popup: [pw, ph],
        viewport: [vw, vh],
        rtl,
    } = *geometry;
    let Request {
        side,
        align,
        side_offset,
        align_offset,
    } = request;
    let inline_start_edge = if rtl { Edge::Right } else { Edge::Left };
    let edge = match side {
        Anchoring::Physical(edge) => edge,
        Anchoring::InlineStart => inline_start_edge,
        Anchoring::InlineEnd => inline_start_edge.opposite(),
    };

    let place = |edge: Edge| -> (f64, f64) {
        let cross_is_x = !edge.is_horizontal();
        let flipped = rtl && cross_is_x;
        let physical = match (align, flipped) {
            (Alignment::Start, true) => Alignment::End,
            (Alignment::End, true) => Alignment::Start,
            (other, _) => other,
        };
        let (start, anchor_len, popup_len) = if cross_is_x {
            (ax, aw, pw)
        } else {
            (ay, ah, ph)
        };
        let aligned = match physical {
            Alignment::Start => start,
            Alignment::Center => start + (anchor_len - popup_len) / 2.0,
            Alignment::End => start + anchor_len - popup_len,
        };
        let sign = if align == Alignment::End { -1.0 } else { 1.0 }
            * if flipped { -1.0 } else { 1.0 };
        let cross = aligned + sign * align_offset;
        match edge {
            Edge::Top => (cross, ay - ph - side_offset),
            Edge::Bottom => (cross, ay + ah + side_offset),
            Edge::Left => (ax - pw - side_offset, cross),
            Edge::Right => (ax + aw + side_offset, cross),
        }
    };
    let overflow = |edge: Edge, (x, y): (f64, f64)| match edge {
        Edge::Top => COLLISION_PADDING - y,
        Edge::Bottom => y + ph - (vh - COLLISION_PADDING),
        Edge::Left => COLLISION_PADDING - x,
        Edge::Right => x + pw - (vw - COLLISION_PADDING),
    };

    let mut chosen = edge;
    let mut position = place(edge);
    let preferred = overflow(edge, position);
    if preferred > 0.0 {
        let opposite = place(edge.opposite());
        if overflow(edge.opposite(), opposite) < preferred {
            chosen = edge.opposite();
            position = opposite;
        }
    }
    if chosen.is_horizontal() {
        position.1 = shift(position.1, ph, vh);
    } else {
        position.0 = shift(position.0, pw, vw);
    }
    let (x, y) = position;

    let (center_x, center_y) = (ax + aw / 2.0, ay + ah / 2.0);
    let relative_x = (center_x - x).clamp(0.0, pw);
    let relative_y = (center_y - y).clamp(0.0, ph);
    let origin = match chosen {
        Edge::Top => format!("{relative_x:.2}px 100%"),
        Edge::Bottom => format!("{relative_x:.2}px 0"),
        Edge::Left => format!("100% {relative_y:.2}px"),
        Edge::Right => format!("0 {relative_y:.2}px"),
    };
    let arrow_x = (center_x - x - ARROW_SIZE / 2.0).clamp(0.0, (pw - ARROW_SIZE).max(0.0));
    let arrow_y = (center_y - y - ARROW_SIZE / 2.0).clamp(0.0, (ph - ARROW_SIZE).max(0.0));
    let available_width = if chosen == Edge::Left {
        ax - side_offset
    } else if chosen == Edge::Right {
        vw - (ax + aw) - side_offset
    } else {
        vw
    } - 2.0 * COLLISION_PADDING;
    let available_height = if chosen == Edge::Top {
        ay - side_offset
    } else if chosen == Edge::Bottom {
        vh - (ay + ah) - side_offset
    } else {
        vh
    } - 2.0 * COLLISION_PADDING;

    let side_name = match side {
        Anchoring::Physical(_) => chosen.name(),
        Anchoring::InlineStart | Anchoring::InlineEnd => {
            if chosen == inline_start_edge {
                "inline-start"
            } else {
                "inline-end"
            }
        }
    };
    let style = format!(
        "position:fixed;visibility:visible;pointer-events:auto;top:{y:.2}px;left:{x:.2}px;width:max-content;--anchor-width:{aw:.2}px;--anchor-height:{ah:.2}px;--available-width:{:.2}px;--available-height:{:.2}px;--transform-origin:{origin};--arrow-x:{arrow_x:.2}px;--arrow-y:{arrow_y:.2}px;",
        available_width.max(0.0),
        available_height.max(0.0),
    );
    Placed {
        side: side_name,
        align: match align {
            Alignment::Start => "start",
            Alignment::Center => "center",
            Alignment::End => "end",
        },
        style,
    }
}

const HIDDEN_STYLE: &str =
    "position:fixed;top:0;left:0;width:max-content;visibility:hidden;pointer-events:none;";

/// Measures the anchor and the popup through `MountedData`; only the viewport size and the text
/// direction need a small script.
async fn measure(
    trigger: Option<Rc<MountedData>>,
    node: Option<Rc<MountedData>>,
    instance: u64,
    request: Request,
) -> Option<Placed> {
    let anchor = trigger?.get_client_rect().await.ok()?;
    let popup = node?.get_client_rect().await.ok()?;
    let mut eval = document::eval(&format!(
        r#"const el = document.querySelector('[data-overlay-positioner="{instance}"]');
dioxus.send([document.documentElement.clientWidth, document.documentElement.clientHeight, el ? getComputedStyle(el).direction === 'rtl' : false]);"#
    ));
    let (vw, vh, rtl) = eval.recv::<(f64, f64, bool)>().await.ok()?;
    Some(compute(
        request,
        &Geometry {
            anchor: [
                anchor.origin.x,
                anchor.origin.y,
                anchor.size.width,
                anchor.size.height,
            ],
            popup: [popup.size.width, popup.size.height],
            viewport: [vw, vh],
            rtl,
        },
    ))
}

/// Pokes Rust whenever the anchor or the popup resizes, or the page scrolls or resizes.
const WATCH_JS: &str = r#"const popup = '[data-overlay-positioner="__INSTANCE__"]';
const anchor = '[data-overlay-trigger="__KEY__"]';
let frame = 0;
const observer = new ResizeObserver(() => poke());
const off = () => {
  observer.disconnect();
  removeEventListener('scroll', poke, true);
  removeEventListener('resize', poke);
};
function poke() {
  if (!document.querySelector(popup)) return off();
  cancelAnimationFrame(frame);
  frame = requestAnimationFrame(() => dioxus.send(true));
}
const el = document.querySelector(popup);
if (el) observer.observe(el);
const tr = document.querySelector(anchor);
if (tr) observer.observe(tr);
addEventListener('scroll', poke, true);
addEventListener('resize', poke);"#;

/// Resolves once the finite animations on the popup are done, so the popup keeps its exit animation.
const EXIT_JS: &str = r#"const el = document.querySelector('[data-overlay-popup="__INSTANCE__"]');
if (el) {
  const running = el.getAnimations({ subtree: true })
    .filter((a) => a.effect?.getTiming().iterations !== Infinity)
    .map((a) => a.finished.catch(() => {}));
  await Promise.race([Promise.all(running), new Promise((resolve) => setTimeout(resolve, 1000))]);
}
dioxus.send(true);"#;

fn script(template: &str, key: u64, instance: u64) -> String {
    template
        .replace("__KEY__", &key.to_string())
        .replace("__INSTANCE__", &instance.to_string())
}

#[derive(Clone, Copy)]
struct Positioning {
    key: u64,
    instance: u64,
    trigger: Signal<Option<Rc<MountedData>>>,
    node: Signal<Option<Rc<MountedData>>>,
    placed: Signal<Option<Placed>>,
    request: Signal<Request>,
}

impl Positioning {
    fn update(self) {
        let mut placed = self.placed;
        spawn(async move {
            let next = measure(
                self.trigger.peek().clone(),
                self.node.peek().clone(),
                self.instance,
                *self.request.peek(),
            )
            .await;
            if let Some(next) = next
                && placed.peek().as_ref() != Some(&next)
            {
                placed.set(Some(next));
            }
        });
    }

    fn style(self) -> String {
        self.placed
            .read()
            .as_ref()
            .map_or_else(|| HIDDEN_STYLE.to_string(), |placed| placed.style.clone())
    }
}

/// Tracks where the popup sits relative to `trigger` and keeps it there while the page moves.
fn use_positioning(
    key: u64,
    trigger: Signal<Option<Rc<MountedData>>>,
    request: Request,
) -> Positioning {
    let instance = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let node = use_signal(|| None);
    let placed = use_signal(|| None);
    let request = use_prop(request);
    let positioning = Positioning {
        key,
        instance,
        trigger,
        node,
        placed,
        request,
    };
    use_effect(move || {
        let mut watch = document::eval(&script(WATCH_JS, key, instance));
        spawn(async move {
            while watch.recv::<bool>().await.is_ok() {
                positioning.update();
            }
        });
    });
    use_effect(move || {
        request();
        positioning.update();
    });
    positioning
}

/// Calls `on_exited` once the popup finished animating out after `open` turned false.
fn use_exit(open: bool, instance: u64, on_exited: EventHandler<()>) {
    use_effect(use_reactive!(|open| {
        if !open {
            let mut done = document::eval(&script(EXIT_JS, 0, instance));
            spawn(async move {
                if done.recv::<bool>().await.is_ok() {
                    on_exited.call(());
                }
            });
        }
    }));
}

fn attr(name: &'static str, value: impl Into<String>) -> Attribute {
    Attribute::new(name, value.into(), None, false)
}

const ESCAPE_JS: &str = r#"const popup = '[data-overlay-popup="__INSTANCE__"]';
function onKey(event) {
  if (!document.querySelector(popup)) return document.removeEventListener('keydown', onKey, true);
  if (event.key === 'Escape') dioxus.send(true);
}
document.addEventListener('keydown', onKey, true);"#;

/// Where the card goes relative to the trigger. `InlineStart` and `InlineEnd` follow the text direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HoverCardSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
    InlineStart,
    InlineEnd,
}

impl HoverCardSide {
    fn anchoring(self) -> Anchoring {
        match self {
            Self::Top => Anchoring::Physical(Edge::Top),
            Self::Right => Anchoring::Physical(Edge::Right),
            Self::Bottom => Anchoring::Physical(Edge::Bottom),
            Self::Left => Anchoring::Physical(Edge::Left),
            Self::InlineStart => Anchoring::InlineStart,
            Self::InlineEnd => Anchoring::InlineEnd,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::InlineStart => "inline-start",
            Self::InlineEnd => "inline-end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HoverCardAlign {
    Start,
    #[default]
    Center,
    End,
}

impl HoverCardAlign {
    fn alignment(self) -> Alignment {
        match self {
            Self::Start => Alignment::Start,
            Self::Center => Alignment::Center,
            Self::End => Alignment::End,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Clone, Copy)]
struct HoverCardContext {
    key: u64,
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    trigger: Signal<Option<Rc<MountedData>>>,
    delay: Signal<u32>,
    close_delay: Signal<u32>,
    trigger_delay: Signal<Option<u32>>,
    trigger_close_delay: Signal<Option<u32>>,
    /// Bumped on every request, so a pending timer that lost the race does nothing.
    pending: Signal<u64>,
}

impl HoverCardContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn commit(mut self, open: bool) {
        if self.is_open() == open {
            return;
        }
        if self.controlled.peek().is_none() {
            self.internal.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }

    fn request(mut self, open: bool) {
        let ticket = *self.pending.peek() + 1;
        self.pending.set(ticket);
        let delay = if open {
            self.trigger_delay.peek().unwrap_or(*self.delay.peek())
        } else {
            self.trigger_close_delay
                .peek()
                .unwrap_or(*self.close_delay.peek())
        };
        if delay == 0 {
            self.commit(open);
            return;
        }
        spawn(async move {
            let _ = document::eval(&format!(
                "await new Promise((resolve) => setTimeout(resolve, {delay}))"
            ))
            .await;
            if *self.pending.peek() == ticket {
                self.commit(open);
            }
        });
    }

    fn cancel(mut self) {
        let ticket = *self.pending.peek() + 1;
        self.pending.set(ticket);
    }
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`. Renders no
/// element: the trigger and the card become children of the parent.
#[component]
pub fn HoverCard(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    /// Default open delay in milliseconds for the trigger, 600 unless the trigger sets `delay`.
    open_delay: Option<u32>,
    /// Default close delay in milliseconds for the trigger, 300 unless the trigger sets `close_delay`.
    close_delay: Option<u32>,
    children: Element,
) -> Element {
    let key = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let trigger = use_signal(|| None);
    let delay = use_prop(open_delay.unwrap_or(600));
    let close_delay = use_prop(close_delay.unwrap_or(300));
    let trigger_delay = use_signal(|| None);
    let trigger_close_delay = use_signal(|| None);
    let pending = use_signal(|| 0_u64);
    use_context_provider(|| HoverCardContext {
        key,
        internal,
        controlled,
        on_open_change,
        trigger,
        delay,
        close_delay,
        trigger_delay,
        trigger_close_delay,
        pending,
    });

    children
}

/// The element that opens the card, an `<a>` by default (made focusable when it has no `href`). To render
/// another element, pass `render`: it receives the trigger's attributes (event handlers, data attributes,
/// `class`) and the children, and must spread the attributes on the element it returns.
#[component]
pub fn HoverCardTrigger(
    render: Option<Callback<(Vec<Attribute>, Element), Element>>,
    /// Milliseconds the pointer or focus must stay on the trigger before the card opens. Default 600.
    delay: Option<u32>,
    /// Milliseconds after the pointer leaves the trigger and the card before the card closes. Default 300.
    close_delay: Option<u32>,
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<HoverCardContext>();
    let open = context.is_open();
    let mut trigger = context.trigger;
    let mut delay_signal = context.trigger_delay;
    let mut close_delay_signal = context.trigger_close_delay;
    if *delay_signal.peek() != delay {
        delay_signal.set(delay);
    }
    if *close_delay_signal.peek() != close_delay {
        close_delay_signal.set(close_delay);
    }
    let mut attributes = attributes;
    let class = take_class(&mut attributes);

    let mut own = vec![
        attr("data-slot", "hover-card-trigger"),
        attr("data-overlay-trigger", context.key.to_string()),
        attr("tabindex", "0"),
    ];
    if open {
        own.push(attr("data-popup-open", ""));
    }
    if let Some(class) = class {
        own.push(attr("class", class));
    }
    own.push(dioxus::html::events::onmounted(
        move |event: MountedEvent| {
            trigger.set(Some(event.data()));
        },
    ));
    own.push(dioxus::html::events::onpointerenter(
        move |event: PointerEvent| {
            if event.pointer_type() != "touch" {
                context.request(true);
            }
        },
    ));
    own.push(dioxus::html::events::onpointerleave(move |_: PointerEvent| {
        context.request(false);
    }));
    own.push(dioxus::html::events::onfocus(move |_: FocusEvent| {
        context.request(true);
    }));
    own.push(dioxus::html::events::onblur(move |_: FocusEvent| {
        context.request(false);
    }));
    own.extend(attributes);

    match render {
        Some(render) => render.call((own, children)),
        None => rsx! {
            a { ..own, {children} }
        },
    }
}

/// The positioned card. Stays in the page while its exit animation runs.
#[component]
pub fn HoverCardContent(
    #[props(default)] side: HoverCardSide,
    #[props(default = 4.0)] side_offset: f64,
    #[props(default)] align: HoverCardAlign,
    #[props(default = 4.0)] align_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<HoverCardContext>();
    let open = context.is_open();
    let mut mounted = use_signal(|| open);
    use_effect(use_reactive!(|open| {
        if open {
            mounted.set(true);
        }
    }));

    if !open && !mounted() {
        return rsx! {};
    }

    rsx! {
        HoverCardPopup {
            open,
            side,
            align,
            side_offset,
            align_offset,
            on_exited: move |_| {
                if !context.is_open() {
                    mounted.set(false);
                }
            },
            attributes,
            {children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HoverCardPopupProps {
    open: bool,
    side: HoverCardSide,
    align: HoverCardAlign,
    side_offset: f64,
    align_offset: f64,
    on_exited: EventHandler<()>,
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
fn HoverCardPopup(props: HoverCardPopupProps) -> Element {
    let context = use_context::<HoverCardContext>();
    let HoverCardPopupProps {
        open,
        side,
        align,
        side_offset,
        align_offset,
        on_exited,
        attributes,
        children,
    } = props;
    let positioning = use_positioning(
        context.key,
        context.trigger,
        Request {
            side: side.anchoring(),
            align: align.alignment(),
            side_offset,
            align_offset,
        },
    );
    let instance = positioning.instance;
    let mut node = positioning.node;
    let placed = positioning.placed.read().clone();
    use_exit(open, instance, on_exited);

    use_effect(move || {
        let mut escape = document::eval(&script(ESCAPE_JS, context.key, instance));
        spawn(async move {
            while escape.recv::<bool>().await.is_ok() {
                context.cancel();
                context.commit(false);
            }
        });
    });

    let mut attributes = attributes;
    let class = cn!(
        "cn-hover-card-content cn-hover-card-content-logical z-50 origin-(--transform-origin) outline-hidden",
        take_class(&mut attributes).as_deref()
    );
    let side_name = placed.as_ref().map_or(side.as_str(), |placed| placed.side);
    let align_name = placed.as_ref().map_or(align.as_str(), |placed| placed.align);
    let visible = open && placed.is_some();

    rsx! {
        div {
            "data-slot": "hover-card-portal",
            "data-overlay-positioner": "{instance}",
            "data-side": side_name,
            "data-align": align_name,
            "data-open": open.then_some(""),
            "data-closed": (!open).then_some(""),
            class: "isolate z-50",
            style: positioning.style(),
            onmounted: move |event| {
                node.set(Some(event.data()));
                positioning.update();
            },
            div {
                "data-slot": "hover-card-content",
                "data-overlay-popup": "{instance}",
                "data-side": side_name,
                "data-align": align_name,
                "data-open": visible.then_some(""),
                "data-closed": (!open).then_some(""),
                "data-ending-style": (!open).then_some(""),
                class: class,
                onpointerenter: move |_| context.cancel(),
                onpointerleave: move |_| context.request(false),
                onfocusin: move |_| context.cancel(),
                ..attributes,
                {children}
            }
        }
    }
}
