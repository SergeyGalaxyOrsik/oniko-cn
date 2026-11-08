//! NavigationMenu: a horizontal bar of links and triggers whose content opens in one shared popup.
//!
//! Like Base UI's `NavigationMenu`, the content of every `NavigationMenuItem` is shown inside a single
//! positioner, popup and viewport that `NavigationMenu` renders after its children. The popup follows the
//! active trigger, resizes to the active content and slides the content in the direction of the move
//! (`data-activation-direction`). Hovering a trigger opens it after `delay` milliseconds, hovering another
//! trigger switches at once, and leaving closes after `close_delay`. A click keeps the popup open until it is
//! clicked again. `ArrowLeft` and `ArrowRight` move through the list, `ArrowDown` opens the content and focuses
//! it, and `Escape` closes the popup and returns focus to its trigger. The popup is placed with
//! `position: fixed` from the trigger's `MountedData` rect, flipping and shifting to stay on screen. It is not
//! moved to the end of `<body>`, so an ancestor with a `transform` becomes its containing block.
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

fn attr(name: &'static str, value: impl Into<String>) -> Attribute {
    Attribute::new(name, value.into(), None, false)
}

/// Resolves after `ms` milliseconds. `tokio::time` does not exist on the web target.
async fn sleep(ms: u64) {
    let mut done = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms})); dioxus.send(true);"
    ));
    let _ = done.recv::<bool>().await;
}

/// Resolves after two animation frames, so a style applied on mount gets painted once.
async fn next_frame() {
    let mut done = document::eval(
        "requestAnimationFrame(() => requestAnimationFrame(() => dioxus.send(true)));",
    );
    let _ = done.recv::<bool>().await;
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
}

/// Which side of the trigger the popup opens on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavigationMenuSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl NavigationMenuSide {
    fn edge(self) -> Edge {
        match self {
            Self::Top => Edge::Top,
            Self::Right => Edge::Right,
            Self::Bottom => Edge::Bottom,
            Self::Left => Edge::Left,
        }
    }
}

/// How the popup lines up with the trigger along the other axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavigationMenuAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavigationMenuOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl NavigationMenuOrientation {
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Request {
    side: Edge,
    align: NavigationMenuAlign,
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

    let place = |edge: Edge| -> (f64, f64) {
        let cross_is_x = !edge.is_horizontal();
        let flipped = rtl && cross_is_x;
        let physical = match (align, flipped) {
            (NavigationMenuAlign::Start, true) => NavigationMenuAlign::End,
            (NavigationMenuAlign::End, true) => NavigationMenuAlign::Start,
            (other, _) => other,
        };
        let (start, anchor_len, popup_len) = if cross_is_x {
            (ax, aw, pw)
        } else {
            (ay, ah, ph)
        };
        let aligned = match physical {
            NavigationMenuAlign::Start => start,
            NavigationMenuAlign::Center => start + (anchor_len - popup_len) / 2.0,
            NavigationMenuAlign::End => start + anchor_len - popup_len,
        };
        let sign = if align == NavigationMenuAlign::End {
            -1.0
        } else {
            1.0
        } * if flipped { -1.0 } else { 1.0 };
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

    let mut chosen = side;
    let mut position = place(side);
    let preferred = overflow(side, position);
    if preferred > 0.0 {
        let opposite = place(side.opposite());
        if overflow(side.opposite(), opposite) < preferred {
            chosen = side.opposite();
            position = opposite;
        }
    }
    if chosen.is_horizontal() {
        position.1 = shift(position.1, ph, vh);
    } else {
        position.0 = shift(position.0, pw, vw);
    }
    let (x, y) = position;

    let relative_x = (ax + aw / 2.0 - x).clamp(0.0, pw);
    let relative_y = (ay + ah / 2.0 - y).clamp(0.0, ph);
    let origin = match chosen {
        Edge::Top => format!("{relative_x:.2}px 100%"),
        Edge::Bottom => format!("{relative_x:.2}px 0"),
        Edge::Left => format!("100% {relative_y:.2}px"),
        Edge::Right => format!("0 {relative_y:.2}px"),
    };
    let available_width = match chosen {
        Edge::Left => ax - side_offset,
        Edge::Right => vw - (ax + aw) - side_offset,
        _ => vw,
    } - 2.0 * COLLISION_PADDING;
    let available_height = match chosen {
        Edge::Top => ay - side_offset,
        Edge::Bottom => vh - (ay + ah) - side_offset,
        _ => vh,
    } - 2.0 * COLLISION_PADDING;

    let style = format!(
        "position:fixed;visibility:visible;top:{y:.2}px;left:{x:.2}px;--positioner-width:{pw:.2}px;--positioner-height:{ph:.2}px;--popup-width:{pw:.2}px;--popup-height:{ph:.2}px;--anchor-width:{aw:.2}px;--anchor-height:{ah:.2}px;--available-width:{:.2}px;--available-height:{:.2}px;--transform-origin:{origin};",
        available_width.max(0.0),
        available_height.max(0.0),
    );
    Placed {
        side: match chosen {
            Edge::Top => "top",
            Edge::Right => "right",
            Edge::Bottom => "bottom",
            Edge::Left => "left",
        },
        align: match align {
            NavigationMenuAlign::Start => "start",
            NavigationMenuAlign::Center => "center",
            NavigationMenuAlign::End => "end",
        },
        style,
    }
}

const HIDDEN_STYLE: &str = "position:fixed;top:0;left:0;visibility:hidden;pointer-events:none;";

/// Measures the active trigger and the natural size of the active content. The content is measured through its
/// scroll size because the popup scales while it enters, which would skew a bounding rect.
async fn measure(
    trigger: Option<Rc<MountedData>>,
    content: Option<Rc<MountedData>>,
    key: u64,
    request: Request,
) -> Option<Placed> {
    let anchor = trigger?.get_client_rect().await.ok()?;
    let popup = content?.get_scroll_size().await.ok()?;
    let mut eval = document::eval(&format!(
        r#"const el = document.querySelector('[data-nav-positioner="{key}"]');
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
            popup: [popup.width, popup.height],
            viewport: [vw, vh],
            rtl,
        },
    ))
}

/// Pokes Rust whenever the active trigger or content resizes, or the page scrolls or resizes. Replaces the
/// watcher of a previous activation.
const WATCH_JS: &str = r#"window.__onikoNavWatch ??= {};
window.__onikoNavWatch['__KEY__']?.();
const positioner = '[data-nav-positioner="__KEY__"]';
let frame = 0;
const observer = new ResizeObserver(() => poke());
const off = () => {
  observer.disconnect();
  removeEventListener('scroll', poke, true);
  removeEventListener('resize', poke);
  delete window.__onikoNavWatch['__KEY__'];
};
window.__onikoNavWatch['__KEY__'] = off;
function poke() {
  if (!document.querySelector(positioner)) return off();
  cancelAnimationFrame(frame);
  frame = requestAnimationFrame(() => dioxus.send(true));
}
for (const selector of ['[data-nav-anchor="__KEY__"]', '[data-nav-content="__KEY__"]']) {
  const el = document.querySelector(selector);
  if (el) observer.observe(el);
}
addEventListener('scroll', poke, true);
addEventListener('resize', poke);"#;

/// Resolves once the finite animations and transitions of the popup are done.
const EXIT_JS: &str = r#"const el = document.querySelector('[data-nav-popup="__KEY__"]');
if (el) {
  const running = el.getAnimations({ subtree: true })
    .filter((a) => a.effect?.getTiming().iterations !== Infinity)
    .map((a) => a.finished.catch(() => {}));
  await Promise.race([Promise.all(running), new Promise((resolve) => setTimeout(resolve, 1000))]);
}
dioxus.send(true);"#;

/// Closes on a pointer press or focus outside the menu and its popup, and on `Escape`. The script removes its
/// own listeners once the popup is gone.
const DISMISS_JS: &str = r#"const root = '[data-nav-root="__KEY__"]';
const popup = '[data-nav-popup="__KEY__"]';
const off = () => {
  document.removeEventListener('pointerdown', onOutside, true);
  document.removeEventListener('focusin', onOutside, true);
  document.removeEventListener('keydown', onKey, true);
};
function onOutside(event) {
  const p = document.querySelector(popup);
  if (!p) return off();
  const r = document.querySelector(root);
  if (p.contains(event.target) || (r && r.contains(event.target))) return;
  dioxus.send('outside');
}
function onKey(event) {
  if (event.key !== 'Escape') return;
  if (!document.querySelector(popup)) return off();
  dioxus.send('escape');
}
document.addEventListener('pointerdown', onOutside, true);
document.addEventListener('focusin', onOutside, true);
document.addEventListener('keydown', onKey, true);"#;

const FOCUSABLE: &str = "a[href], button:not([disabled]), input:not([disabled]):not([type=hidden]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex=\"-1\"])";

fn script(template: &str, key: u64) -> String {
    template.replace("__KEY__", &key.to_string())
}

fn trigger_dom_id(key: u64, value: &str) -> String {
    format!("navigation-menu-{key}-trigger-{value}")
}

fn run(script: String) {
    spawn(async move {
        let _ = document::eval(&script).await;
    });
}

/// One item's content, handed to the shared viewport.
#[derive(Clone, PartialEq)]
struct Slot {
    value: String,
    open: bool,
    present: bool,
    attributes: Vec<Attribute>,
    children: Element,
}

#[derive(Clone, Copy)]
struct NavigationMenuContext {
    key: u64,
    internal: Signal<Option<String>>,
    controlled: Signal<Option<Option<String>>>,
    on_value_change: Option<EventHandler<Option<String>>>,
    delay: Signal<u64>,
    close_delay: Signal<u64>,
    orientation: Signal<NavigationMenuOrientation>,
    /// Opened by a click or key press, so leaving the popup does not close it.
    sticky: Signal<bool>,
    /// Bumped to cancel a pending hover open or close.
    timer: Signal<u64>,
    /// The value shown last, kept while the popup animates out so it stays next to its trigger.
    shown: Signal<Option<String>>,
    mounted: Signal<bool>,
    direction: Signal<Option<&'static str>>,
    triggers: Signal<Vec<(String, Rc<MountedData>)>>,
    content: Signal<Option<Rc<MountedData>>>,
    slots: Signal<Vec<Slot>>,
    focus_content: Signal<u64>,
}

impl NavigationMenuContext {
    fn active(self) -> Option<String> {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn set_value(self, value: Option<String>) {
        let mut internal = self.internal;
        if self.controlled.peek().is_none() {
            internal.set(value.clone());
        }
        if let Some(handler) = self.on_value_change {
            handler.call(value);
        }
    }

    fn cancel_timer(self) -> u64 {
        let mut timer = self.timer;
        let next = *timer.peek() + 1;
        timer.set(next);
        next
    }

    fn close(self) {
        self.cancel_timer();
        let mut sticky = self.sticky;
        sticky.set(false);
        if self.active().is_some() {
            self.set_value(None);
        }
    }

    fn close_and_focus_trigger(self) {
        let shown = self.shown.peek().clone();
        self.close();
        if let Some(value) = shown {
            run(format!(
                "document.getElementById('{}')?.focus()",
                trigger_dom_id(self.key, &value)
            ));
        }
    }

    /// Makes `value` the open item. Moving between items records the direction for the slide animation.
    fn activate(self, value: String) {
        let previous = self.active();
        let triggers = self.triggers.peek().clone();
        let mut direction = self.direction;
        spawn(async move {
            let mut next = None;
            if let Some(previous) = previous.filter(|previous| *previous != value) {
                let find = |wanted: &str| {
                    triggers
                        .iter()
                        .find(|(candidate, _)| candidate == wanted)
                        .map(|(_, node)| node.clone())
                };
                if let (Some(from), Some(to)) = (find(&previous), find(&value))
                    && let (Ok(from), Ok(to)) =
                        (from.get_client_rect().await, to.get_client_rect().await)
                {
                    next = Some(if to.origin.x < from.origin.x {
                        "left"
                    } else {
                        "right"
                    });
                }
            }
            direction.set(next);
            self.set_value(Some(value));
        });
    }

    fn schedule_close(self) {
        let generation = self.cancel_timer();
        let delay = *self.close_delay.peek();
        let timer = self.timer;
        spawn(async move {
            sleep(delay).await;
            if *timer.peek() == generation && !*self.sticky.peek() {
                self.close();
            }
        });
    }

    fn request_focus_in_content(self) {
        let mut counter = self.focus_content;
        let next = *counter.peek() + 1;
        counter.set(next);
    }
}

#[derive(Clone)]
struct ItemContext {
    value: String,
}

/// The root of the menu. Renders the navigation landmark, and after its children the shared positioner,
/// popup and viewport that show the content of the open item.
#[component]
pub fn NavigationMenu(
    /// Where the popup lines up with the trigger.
    #[props(default)]
    align: NavigationMenuAlign,
    /// Controlled value of the open item, `None` when closed. When set, `on_value_change` is the only way to
    /// change it.
    value: Option<Option<String>>,
    /// Value of the item that is open at first render when `value` is not controlled.
    default_value: Option<String>,
    on_value_change: Option<EventHandler<Option<String>>>,
    /// Milliseconds a hover must last before the popup opens.
    #[props(default = 50)]
    delay: u64,
    /// Milliseconds the pointer may be away before the popup closes.
    #[props(default = 50)]
    close_delay: u64,
    #[props(default)] orientation: NavigationMenuOrientation,
    #[props(extends = GlobalAttributes, extends = nav)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let key = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_value.clone());
    let controlled = use_prop(value);
    let delay = use_prop(delay);
    let close_delay = use_prop(close_delay);
    let orientation_signal = use_prop(orientation);
    let sticky = use_signal(|| false);
    let timer = use_signal(|| 0);
    let shown = use_signal(|| default_value);
    let mounted = use_signal(|| false);
    let direction = use_signal(|| None);
    let triggers = use_signal(Vec::new);
    let content = use_signal(|| None);
    let slots = use_signal(Vec::new);
    let focus_content = use_signal(|| 0);
    let context = use_context_provider(|| NavigationMenuContext {
        key,
        internal,
        controlled,
        on_value_change,
        delay,
        close_delay,
        orientation: orientation_signal,
        sticky,
        timer,
        shown,
        mounted,
        direction,
        triggers,
        content,
        slots,
        focus_content,
    });

    let mut shown_signal = context.shown;
    let mut mounted_signal = context.mounted;
    use_effect(move || {
        let active = context.active();
        if active.is_some() {
            shown_signal.set(active);
            mounted_signal.set(true);
        } else if *mounted_signal.peek() {
            spawn(async move {
                let mut done = document::eval(&script(EXIT_JS, key));
                let _ = done.recv::<bool>().await;
                if context.active().is_none() {
                    mounted_signal.set(false);
                }
            });
        }
    });

    let mut attributes = attributes;
    let class = cn!(
        "max-w-max group/navigation-menu relative flex flex-1 items-center justify-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        nav {
            "data-slot": "navigation-menu",
            "data-nav-root": "{key}",
            "data-orientation": orientation.as_str(),
            aria_label: "Main",
            class: class,
            ..attributes,
            {children}
            if mounted() {
                NavigationMenuPositioner { align }
            }
        }
    }
}

/// The list that holds the items. `ArrowLeft`, `ArrowRight`, `Home` and `End` (or `ArrowUp` and `ArrowDown` for
/// a vertical menu) move focus through its triggers and links.
#[component]
pub fn NavigationMenuList(
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let list_id = format!("navigation-menu-{}-list", context.key);
    let mut attributes = attributes;
    let class = cn!(
        "gap-0 group flex flex-1 list-none items-center justify-center",
        take_class(&mut attributes).as_deref()
    );
    let vertical = (context.orientation)() == NavigationMenuOrientation::Vertical;

    rsx! {
        ul {
            id: list_id.clone(),
            "data-slot": "navigation-menu-list",
            "data-orientation": (context.orientation)().as_str(),
            class: class,
            onkeydown: move |event| {
                let key = event.key();
                let (previous, next) = if vertical {
                    (Key::ArrowUp, Key::ArrowDown)
                } else {
                    (Key::ArrowLeft, Key::ArrowRight)
                };
                let step = if key == previous {
                    -1
                } else if key == next {
                    1
                } else if key == Key::Home {
                    -1000
                } else if key == Key::End {
                    1000
                } else {
                    return;
                };
                event.prevent_default();
                let list_id = list_id.clone();
                spawn(async move {
                    let mut eval = document::eval(&format!(
                        r#"const list = document.getElementById('{list_id}');
const items = list ? [...list.querySelectorAll('[data-slot="navigation-menu-trigger"], [data-slot="navigation-menu-link"]')] : [];
const rtl = list ? getComputedStyle(list).direction === 'rtl' : false;
let step = {step};
if (!{vertical} && Math.abs(step) === 1 && rtl) step = -step;
const index = items.indexOf(document.activeElement);
let target;
if (Math.abs(step) > 1) target = step < 0 ? items[0] : items[items.length - 1];
else target = items[(index + step + items.length) % items.length];
target?.focus();
dioxus.send(target?.dataset.navValue ?? '');"#
                    ));
                    if let Ok(value) = eval.recv::<String>().await
                        && !value.is_empty()
                        && context.active().is_some()
                    {
                        context.activate(value);
                    }
                });
            },
            ..attributes,
            {children}
        }
    }
}

/// One entry of the list: a trigger with its content, or a link.
#[component]
pub fn NavigationMenuItem(
    /// Identifies the item in `value`. Generated when omitted.
    value: Option<String>,
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let generated = use_hook(|| format!("item-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
    let value = value.unwrap_or(generated);
    use_context_provider(|| ItemContext { value });
    let mut attributes = attributes;
    let class = cn!("relative", take_class(&mut attributes).as_deref());

    rsx! {
        li {
            "data-slot": "navigation-menu-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

const TRIGGER_STYLE: &str = "hover:bg-muted focus:bg-muted data-open:hover:bg-muted data-open:focus:bg-muted data-open:bg-muted/50 focus-visible:ring-ring/30 data-popup-open:bg-muted/50 data-popup-open:hover:bg-muted rounded-lg px-2.5 py-1.5 text-xs/relaxed font-medium transition-all focus-visible:ring-2 focus-visible:outline-1 disabled:opacity-50 group/navigation-menu-trigger inline-flex h-9 w-max items-center justify-center outline-none disabled:pointer-events-none";

/// Trigger classes for any element, e.g. a link that should look like a `NavigationMenuTrigger`.
pub fn navigation_menu_trigger_style(class: Option<&str>) -> String {
    cn!(TRIGGER_STYLE, class)
}

/// The button that opens its item's content. Hovering opens it, clicking pins it open.
#[component]
pub fn NavigationMenuTrigger(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let item = use_context::<ItemContext>();
    let open = context.active().as_ref() == Some(&item.value);
    let is_shown = context.shown.read().as_ref() == Some(&item.value) && (context.mounted)();
    let vertical = (context.orientation)() == NavigationMenuOrientation::Vertical;
    let mut attributes = attributes;
    let class = cn!(
        navigation_menu_trigger_style(None),
        "group",
        take_class(&mut attributes).as_deref()
    );
    let mut triggers = context.triggers;
    let value = item.value.clone();

    rsx! {
        button {
            id: trigger_dom_id(context.key, &value),
            r#type: "button",
            "data-slot": "navigation-menu-trigger",
            "data-nav-value": value.clone(),
            "data-nav-anchor": is_shown.then(|| context.key.to_string()),
            "data-popup-open": open.then_some(""),
            aria_expanded: open,
            aria_controls: open.then(|| format!("navigation-menu-{}-popup", context.key)),
            class: class,
            onmounted: {
                let value = value.clone();
                move |event: MountedEvent| {
                    let mut list = triggers.write();
                    list.retain(|(candidate, _)| *candidate != value);
                    list.push((value.clone(), event.data()));
                }
            },
            onmouseenter: {
                let value = value.clone();
                move |_| {
                    let generation = context.cancel_timer();
                    if context.active().is_some() {
                        if context.active().as_ref() != Some(&value) {
                            context.activate(value.clone());
                        }
                        return;
                    }
                    let delay = *context.delay.peek();
                    let value = value.clone();
                    spawn(async move {
                        sleep(delay).await;
                        if *context.timer.peek() == generation && context.active().is_none() {
                            context.activate(value);
                        }
                    });
                }
            },
            onmouseleave: move |_| {
                if !*context.sticky.peek() {
                    context.schedule_close();
                } else {
                    context.cancel_timer();
                }
            },
            onclick: {
                let value = value.clone();
                move |event: MouseEvent| {
                    if let Some(handler) = onclick {
                        handler.call(event.clone());
                    }
                    context.cancel_timer();
                    let mut sticky = context.sticky;
                    let is_open = context.active().as_ref() == Some(&value);
                    if is_open && *sticky.peek() {
                        context.close();
                        return;
                    }
                    sticky.set(true);
                    if !is_open {
                        context.activate(value.clone());
                    }
                    // A click made with the keyboard has no pointer position.
                    let coordinates = event.client_coordinates();
                    if coordinates.x == 0.0 && coordinates.y == 0.0 {
                        context.request_focus_in_content();
                    }
                }
            },
            onkeydown: {
                let value = value.clone();
                move |event| {
                    let key = event.key();
                    let opens = if vertical {
                        key == Key::ArrowRight || key == Key::ArrowLeft
                    } else {
                        key == Key::ArrowDown
                    };
                    let is_open = context.active().as_ref() == Some(&value);
                    if opens {
                        event.prevent_default();
                        context.cancel_timer();
                        let mut sticky = context.sticky;
                        sticky.set(true);
                        if !is_open {
                            context.activate(value.clone());
                        }
                        context.request_focus_in_content();
                    } else if key == Key::Tab && !event.modifiers().shift() && is_open {
                        event.prevent_default();
                        context.request_focus_in_content();
                    }
                }
            },
            ..attributes,
            {children}
            " "
            svg {
                class: "relative top-px ml-1 size-3 transition duration-300 group-data-open/navigation-menu-trigger:rotate-180 group-data-popup-open/navigation-menu-trigger:rotate-180",
                "aria-hidden": "true",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }
}

/// The content of an item. It renders nothing where it is written: while its item is open, its children appear
/// inside the shared viewport of `NavigationMenu`.
#[component]
pub fn NavigationMenuContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let item = use_context::<ItemContext>();
    let open = context.active().as_ref() == Some(&item.value);
    let mut present = use_signal(|| open);
    let mut slots = context.slots;
    let value = item.value;
    let exit_value = value.clone();

    use_effect(use_reactive!(|open| {
        if open {
            present.set(true);
        } else if *present.peek() {
            let value = exit_value.clone();
            spawn(async move {
                sleep(350).await;
                if context.active().as_ref() != Some(&value) {
                    present.set(false);
                }
            });
        }
    }));

    let slot = Slot {
        value: value.clone(),
        open,
        present: open || present(),
        attributes,
        children,
    };
    use_effect(use_reactive!(|slot| {
        let mut list = slots.write();
        match list
            .iter_mut()
            .find(|candidate| candidate.value == slot.value)
        {
            Some(existing) => *existing = slot,
            None => list.push(slot),
        }
    }));
    use_drop(move || slots.write().retain(|candidate| candidate.value != value));

    rsx! {}
}

#[component]
fn ContentView(slot: Slot) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let Slot {
        value,
        open,
        attributes,
        children,
        ..
    } = slot;
    let mut started = use_signal(|| false);
    let mut node = use_signal(|| None::<Rc<MountedData>>);
    let mut content = context.content;

    use_effect(move || {
        spawn(async move {
            next_frame().await;
            started.set(true);
        });
    });
    use_effect(use_reactive!(|open| {
        if open && let Some(node) = node.peek().clone() {
            content.set(Some(node));
        }
    }));

    let mut attributes = attributes;
    let class = cn!(
        "data-[motion^=from-]:animate-in data-[motion^=to-]:animate-out data-[motion^=from-]:fade-in data-[motion^=to-]:fade-out data-[motion=from-end]:slide-in-from-right-52 data-[motion=from-start]:slide-in-from-left-52 data-[motion=to-end]:slide-out-to-right-52 data-[motion=to-start]:slide-out-to-left-52 group-data-[viewport=false]/navigation-menu:bg-popover group-data-[viewport=false]/navigation-menu:text-popover-foreground group-data-[viewport=false]/navigation-menu:data-open:animate-in group-data-[viewport=false]/navigation-menu:data-closed:animate-out group-data-[viewport=false]/navigation-menu:data-closed:zoom-out-95 group-data-[viewport=false]/navigation-menu:data-open:zoom-in-95 group-data-[viewport=false]/navigation-menu:data-open:fade-in-0 group-data-[viewport=false]/navigation-menu:data-closed:fade-out-0 group-data-[viewport=false]/navigation-menu:ring-foreground/10 p-1.5 ease-[cubic-bezier(0.22,1,0.36,1)] group-data-[viewport=false]/navigation-menu:rounded-xl group-data-[viewport=false]/navigation-menu:shadow-md group-data-[viewport=false]/navigation-menu:ring-1 group-data-[viewport=false]/navigation-menu:duration-300 data-ending-style:data-activation-direction=left:translate-x-[50%] data-ending-style:data-activation-direction=right:translate-x-[-50%] data-starting-style:data-activation-direction=left:translate-x-[-50%] data-starting-style:data-activation-direction=right:translate-x-[50%] h-full w-auto transition-[opacity,transform,translate] duration-[0.35s] data-ending-style:opacity-0 data-starting-style:opacity-0 **:data-[slot=navigation-menu-link]:focus:ring-0 **:data-[slot=navigation-menu-link]:focus:outline-none",
        take_class(&mut attributes).as_deref()
    );
    // The size is the content's own, so the popup can animate to it. A content that is on its way out leaves
    // the layout, so the entering one is measured alone.
    let style = if open {
        "width:max-content;height:max-content;"
    } else {
        "position:absolute;top:0;inset-inline-start:0;width:max-content;height:max-content;"
    };

    rsx! {
        div {
            "data-slot": "navigation-menu-content",
            "data-nav-content": open.then(|| context.key.to_string()),
            "data-nav-value": value,
            "data-open": open.then_some(""),
            "data-closed": (!open).then_some(""),
            "data-starting-style": (open && !started()).then_some(""),
            "data-ending-style": (!open).then_some(""),
            "data-activation-direction": (context.direction)(),
            class: class,
            style: style,
            onmounted: move |event: MountedEvent| {
                node.set(Some(event.data()));
                if open {
                    content.set(Some(event.data()));
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// The popup shared by all items: a positioner that follows the active trigger, a popup that resizes to the
/// active content and a viewport that clips the content while it slides. `NavigationMenu` renders it; use it
/// directly only to change its side or offsets.
#[component]
pub fn NavigationMenuPositioner(
    #[props(default)] side: NavigationMenuSide,
    #[props(default = 8.0)] side_offset: f64,
    #[props(default)] align: NavigationMenuAlign,
    #[props(default = 0.0)] align_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let key = context.key;
    let request = use_prop(Request {
        side: side.edge(),
        align,
        side_offset,
        align_offset,
    });
    let mut placed = use_signal(|| None::<Placed>);
    let mut instant = use_signal(|| true);
    let mut started = use_signal(|| false);
    let open = context.active().is_some();

    let update = move || {
        let shown = context.shown.peek().clone();
        let trigger = shown.and_then(|shown| {
            context
                .triggers
                .peek()
                .iter()
                .find(|(candidate, _)| *candidate == shown)
                .map(|(_, node)| node.clone())
        });
        let content = context.content.peek().clone();
        let request = *request.peek();
        spawn(async move {
            if let Some(next) = measure(trigger, content, key, request).await
                && placed.peek().as_ref() != Some(&next)
            {
                placed.set(Some(next));
            }
        });
    };

    use_effect(move || {
        let _ = context.shown.read();
        let _ = context.content.read();
        let _ = request.read();
        update();
    });
    use_effect(move || {
        let _ = context.shown.read();
        let mut watch = document::eval(&script(WATCH_JS, key));
        spawn(async move {
            while watch.recv::<bool>().await.is_ok() {
                update();
            }
        });
    });
    use_effect(move || {
        let mut dismiss = document::eval(&script(DISMISS_JS, key));
        spawn(async move {
            while let Ok(kind) = dismiss.recv::<String>().await {
                if context.active().is_none() {
                    continue;
                }
                if kind == "escape" {
                    context.close_and_focus_trigger();
                } else {
                    context.close();
                }
            }
        });
    });
    use_effect(move || {
        if placed.read().is_some() && !*started.peek() {
            spawn(async move {
                next_frame().await;
                started.set(true);
                instant.set(false);
            });
        }
    });
    use_effect(move || {
        let requested = (context.focus_content)();
        if requested == 0 {
            return;
        }
        let _ = placed.read();
        run(format!(
            r#"requestAnimationFrame(() => requestAnimationFrame(() => {{
  const content = document.querySelector('[data-nav-content="{key}"]');
  const first = content?.querySelector('{FOCUSABLE}');
  (first ?? content)?.focus({{ preventScroll: true }});
}}));"#
        ));
    });

    let current = placed.read().clone();
    let style = current
        .as_ref()
        .map_or_else(|| HIDDEN_STYLE.to_string(), |placed| placed.style.clone());
    let side_name = current.as_ref().map_or("bottom", |placed| placed.side);
    let align_name = current.as_ref().map_or("start", |placed| placed.align);
    let mut attributes = attributes;
    let class = cn!(
        "transition-[top,left,right,bottom] duration-300 ease-[cubic-bezier(0.22,1,0.36,1)] data-[side=bottom]:before:top-[-10px] data-[side=bottom]:before:right-0 data-[side=bottom]:before:left-0 isolate z-50 h-(--positioner-height) w-(--positioner-width) max-w-(--available-width) duration-[0.35s] data-instant:transition-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-nav-positioner": "{key}",
            "data-open": open.then_some(""),
            "data-closed": (!open).then_some(""),
            "data-side": side_name,
            "data-align": align_name,
            "data-instant": instant().then_some(""),
            class: class,
            style: style,
            onmouseenter: move |_| {
                context.cancel_timer();
            },
            onmouseleave: move |_| {
                if !*context.sticky.peek() {
                    context.schedule_close();
                }
            },
            ..attributes,
            div {
                id: "navigation-menu-{key}-popup",
                "data-nav-popup": "{key}",
                "data-open": open.then_some(""),
                "data-closed": (!open).then_some(""),
                "data-side": side_name,
                "data-align": align_name,
                "data-starting-style": (open && !started()).then_some(""),
                "data-ending-style": (!open).then_some(""),
                class: "bg-popover text-popover-foreground ring-foreground/10 rounded-xl shadow ring-1 transition-all ease-[cubic-bezier(0.22,1,0.36,1)] outline-none data-ending-style:scale-90 data-ending-style:opacity-0 data-ending-style:duration-150 data-starting-style:scale-90 data-starting-style:opacity-0 data-[ending-style]:easing-[ease] xs:w-(--popup-width) relative h-(--popup-height) w-(--popup-width) origin-(--transform-origin) transition-[opacity,transform,width,height,scale,translate] duration-[0.35s]",
                div { class: "relative size-full overflow-hidden",
                    for slot in context.slots.read().iter().filter(|slot| slot.present).cloned() {
                        ContentView { key: "{slot.value}", slot }
                    }
                }
            }
        }
    }
}

/// A link. Inside a `NavigationMenuContent` it closes the popup when clicked; `active` marks the current page.
/// To render another element, pass `render`: it receives the link's attributes and children, and must spread the
/// attributes on the element it returns.
#[component]
pub fn NavigationMenuLink(
    #[props(default)] active: bool,
    render: Option<Callback<(Vec<Attribute>, Element), Element>>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = try_use_context::<NavigationMenuContext>();
    let mut attributes = attributes;
    let class = cn!(
        "data-[active=true]:focus:bg-muted data-[active=true]:hover:bg-muted data-[active=true]:bg-muted/50 focus-visible:ring-ring/30 hover:bg-muted focus:bg-muted flex items-center gap-1.5 rounded-lg p-2 text-xs/relaxed transition-all outline-none focus-visible:ring-2 focus-visible:outline-1 [&_svg:not([class*='size-'])]:size-4 in-data-[slot=navigation-menu-content]:rounded-md",
        take_class(&mut attributes).as_deref()
    );

    let mut own = vec![
        attr("data-slot", "navigation-menu-link"),
        attr("class", class),
    ];
    if active {
        own.push(attr("data-active", "true"));
        own.push(attr("aria-current", "page"));
    }
    own.push(dioxus::html::events::onclick(move |event: MouseEvent| {
        if let Some(handler) = onclick {
            handler.call(event);
        }
        if let Some(context) = context {
            context.close();
        }
    }));
    own.extend(attributes);

    match render {
        Some(render) => render.call((own, children)),
        None => rsx! {
            a { ..own, {children} }
        },
    }
}

/// A small arrow that points at the open trigger, drawn with `cn-navigation-menu-indicator-arrow`.
#[component]
pub fn NavigationMenuIndicator(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "data-[state=visible]:animate-in data-[state=hidden]:animate-out data-[state=hidden]:fade-out data-[state=visible]:fade-in top-full z-1 flex h-1.5 items-end justify-center overflow-hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "navigation-menu-indicator",
            "aria-hidden": "true",
            class: class,
            ..attributes,
            div { class: "bg-border rounded-tl-sm shadow-md relative top-[60%] h-2 w-2 rotate-45" }
        }
    }
}
