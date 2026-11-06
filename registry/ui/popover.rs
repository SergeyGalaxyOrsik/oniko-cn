//! Popover: displays rich content next to a trigger, opened by a click and closed by `Escape` or a click outside.
//!
//! Like Base UI's `Popover`, the root renders no element of its own, so the trigger sits directly in its
//! parent (a `ButtonGroup`, for example). The popup is placed with `position: fixed` from the trigger's
//! `MountedData` rect, honouring `side`, `align`, `side_offset` and `align_offset`, flipping to the opposite
//! side and shifting along the cross axis when it would leave the viewport. It is not moved to the end of
//! `<body>`, so an ancestor with a `transform` becomes its containing block. The first tabbable element in
//! the popup takes focus on open, and `Escape` returns focus to the trigger.
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

/// Closes on `Escape` (only the innermost open popover) and on a pointer press outside the popup and its
/// trigger. The script removes its own listeners once the popup is gone.
const DISMISS_JS: &str = r#"const layers = (window.__onikoPopoverLayers ??= []);
const popup = '[data-overlay-popup="__INSTANCE__"]';
const trigger = '[data-overlay-trigger="__KEY__"]';
layers.push('__INSTANCE__');
const off = () => {
  document.removeEventListener('pointerdown', onDown, true);
  document.removeEventListener('keydown', onKey, true);
  const i = layers.indexOf('__INSTANCE__');
  if (i >= 0) layers.splice(i, 1);
};
function onDown(event) {
  const p = document.querySelector(popup);
  if (!p) return off();
  const t = document.querySelector(trigger);
  if (p.contains(event.target) || (t && t.contains(event.target))) return;
  dioxus.send('outside');
}
function onKey(event) {
  if (event.key !== 'Escape') return;
  if (!document.querySelector(popup)) return off();
  for (let i = layers.length - 1; i >= 0; i--) {
    if (!document.querySelector('[data-overlay-popup="' + layers[i] + '"]')) layers.splice(i, 1);
  }
  if (layers[layers.length - 1] !== '__INSTANCE__') return;
  dioxus.send('escape');
}
document.addEventListener('pointerdown', onDown, true);
document.addEventListener('keydown', onKey, true);"#;

const FOCUS_JS: &str = r#"const p = document.querySelector('[data-overlay-popup="__INSTANCE__"]');
if (p) {
  const first = p.querySelector('a[href], button:not([disabled]), input:not([disabled]):not([type=hidden]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])');
  (first ?? p).focus({ preventScroll: true });
}"#;

#[derive(Clone, Copy)]
struct PopoverContext {
    key: u64,
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    trigger: Signal<Option<Rc<MountedData>>>,
    has_title: Signal<bool>,
    has_description: Signal<bool>,
}

impl PopoverContext {
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

    fn popup_id(self) -> String {
        format!("popover-{}", self.key)
    }

    fn close_and_focus_trigger(self) {
        self.set_open(false);
        if let Some(trigger) = self.trigger.peek().clone() {
            spawn(async move {
                let _ = trigger.set_focus(true).await;
            });
        }
    }
}

/// Where the popup goes relative to the trigger. `InlineStart` and `InlineEnd` follow the text direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
    InlineStart,
    InlineEnd,
}

impl PopoverSide {
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
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
}

impl PopoverAlign {
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

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`. Renders no
/// element: the trigger and the popup become children of the parent.
#[component]
pub fn Popover(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let key = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let trigger = use_signal(|| None);
    let has_title = use_signal(|| false);
    let has_description = use_signal(|| false);
    use_context_provider(|| PopoverContext {
        key,
        internal,
        controlled,
        on_open_change,
        trigger,
        has_title,
        has_description,
    });

    children
}

/// The element that toggles the popup, a `<button>` by default. Style it with `class`, for example with
/// `button_variants(...)`. To render another element, pass `render`: it receives the trigger's attributes
/// (event handlers, ARIA and data attributes, `class`) and the children, and must spread the attributes on
/// the element it returns.
#[component]
pub fn PopoverTrigger(
    render: Option<Callback<(Vec<Attribute>, Element), Element>>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<PopoverContext>();
    let open = context.is_open();
    let mut trigger = context.trigger;
    let mut attributes = attributes;
    let class = take_class(&mut attributes);

    let mut own = vec![
        attr("data-slot", "popover-trigger"),
        attr("data-overlay-trigger", context.key.to_string()),
        attr("aria-haspopup", "dialog"),
        attr("aria-expanded", if open { "true" } else { "false" }),
    ];
    if open {
        own.push(attr("data-popup-open", ""));
        own.push(attr("aria-controls", context.popup_id()));
    }
    if let Some(class) = class {
        own.push(attr("class", class));
    }
    own.push(dioxus::html::events::onmounted(move |event: MountedEvent| {
        trigger.set(Some(event.data()));
    }));
    own.push(dioxus::html::events::onclick(move |event: MouseEvent| {
        if let Some(handler) = onclick {
            handler.call(event);
        }
        context.set_open(!context.is_open());
    }));
    own.extend(attributes);

    match render {
        Some(render) => render.call((own, children)),
        None => rsx! {
            button { r#type: "button", ..own, {children} }
        },
    }
}

/// The positioned popup. Stays in the page while its exit animation runs.
#[component]
pub fn PopoverContent(
    #[props(default)] side: PopoverSide,
    #[props(default)] align: PopoverAlign,
    #[props(default = 4.0)] side_offset: f64,
    #[props(default = 0.0)] align_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<PopoverContext>();
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
        PopoverPopup {
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
struct PopoverPopupProps {
    open: bool,
    side: PopoverSide,
    align: PopoverAlign,
    side_offset: f64,
    align_offset: f64,
    on_exited: EventHandler<()>,
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
fn PopoverPopup(props: PopoverPopupProps) -> Element {
    let context = use_context::<PopoverContext>();
    let PopoverPopupProps {
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
        let mut dismiss = document::eval(&script(DISMISS_JS, context.key, instance));
        spawn(async move {
            while let Ok(kind) = dismiss.recv::<String>().await {
                if !context.is_open() {
                    continue;
                }
                if kind == "escape" {
                    context.close_and_focus_trigger();
                } else {
                    context.set_open(false);
                }
            }
        });
    });

    let mut focused = use_signal(|| false);
    use_effect(move || {
        if positioning.placed.read().is_some() && !*focused.peek() {
            focused.set(true);
            spawn(async move {
                let _ = document::eval(&script(FOCUS_JS, 0, instance)).await;
            });
        }
    });

    let mut attributes = attributes;
    let class = cn!(
        "cn-popover-content cn-popover-content-logical z-50 w-72 origin-(--transform-origin) outline-hidden",
        take_class(&mut attributes).as_deref()
    );
    let side_name = placed.as_ref().map_or(side.as_str(), |placed| placed.side);
    let align_name = placed.as_ref().map_or(align.as_str(), |placed| placed.align);
    let visible = open && placed.is_some();

    rsx! {
        div {
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
                "data-slot": "popover-content",
                "data-overlay-popup": "{instance}",
                "data-side": side_name,
                "data-align": align_name,
                "data-open": visible.then_some(""),
                "data-closed": (!open).then_some(""),
                "data-ending-style": (!open).then_some(""),
                id: context.popup_id(),
                role: "dialog",
                tabindex: "-1",
                aria_labelledby: (context.has_title)().then(|| format!("{}-title", context.popup_id())),
                aria_describedby: (context.has_description)().then(|| format!("{}-description", context.popup_id())),
                class: class,
                ..attributes,
                {children}
            }
        }
    }
}

/// Groups the title and description at the top of the popup.
#[component]
pub fn PopoverHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-popover-header", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "popover-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The popup's heading; it names the popup for assistive technology.
#[component]
pub fn PopoverTitle(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<PopoverContext>();
    let mut has_title = context.has_title;
    use_effect(move || has_title.set(true));
    use_drop(move || has_title.set(false));
    let mut attributes = attributes;
    let class = cn!("cn-popover-title", take_class(&mut attributes).as_deref());

    rsx! {
        h2 {
            "data-slot": "popover-title",
            id: "{context.popup_id()}-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Describes the popup for assistive technology.
#[component]
pub fn PopoverDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<PopoverContext>();
    let mut has_description = context.has_description;
    use_effect(move || has_description.set(true));
    use_drop(move || has_description.set(false));
    let mut attributes = attributes;
    let class = cn!(
        "cn-popover-description",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "popover-description",
            id: "{context.popup_id()}-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}
