//! Drawer: a panel that slides in from an edge of the screen, with swipe-to-dismiss, snap points and nested stacking.
//!
//! The drawer lives in a native `<dialog>` host opened with `showModal()`, so the browser and the
//! desktop webview provide the top layer, the inert page behind it and the focus trap. `Escape` is
//! handled here instead of by the browser so the exit transition can play: the popup keeps
//! `data-closed` and `data-ending-style` until its transitions have finished, then unmounts and
//! focus returns to the trigger. Page scroll is locked while a modal drawer is open.
//!
//! Swiping runs in a small script that writes `--drawer-swipe-movement-x/y`, `--drawer-swipe-progress`
//! and `--drawer-swipe-strength` on the popup and overlay, as the styles expect, and sets `data-swiping`
//! while a finger or the mouse drags. Snap points apply to drawers that swipe down.
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
const host = document.getElementById("__ID__");
if (!host) { dioxus.send("ready"); return; }
const dir = "__DIR__";
const axis = dir === "down" || dir === "up" ? "y" : "x";
const sign = dir === "down" || dir === "right" ? 1 : -1;
const mode = "__MODE__";
const snaps = __SNAPS__;
const useSnaps = snaps.length > 0 && dir === "down";
const popup = host.querySelector('[data-slot="drawer-popup"]');
const viewport = host.querySelector('[data-slot="drawer-viewport"]');
const overlay = host.querySelector('[data-slot="drawer-overlay"]');
host.addEventListener("cancel", (event) => event.preventDefault());
if (mode === "modal") {
  if (!host.open) host.showModal();
  __LOCK__
} else if (!host.open) {
  host.show();
}
if (!popup) { dioxus.send("ready"); return; }

const targets = [popup, overlay].filter(Boolean);
const setVar = (name, value) => targets.forEach((node) => node.style.setProperty(name, value));
const setSwiping = (on) => targets.forEach((node) => on ? node.setAttribute("data-swiping", "") : node.removeAttribute("data-swiping"));
const moveVar = axis === "y" ? "--drawer-swipe-movement-y" : "--drawer-swipe-movement-x";
setVar("--drawer-swipe-movement-x", "0px");
setVar("--drawer-swipe-movement-y", "0px");
setVar("--drawer-swipe-progress", "0");
setVar("--drawer-swipe-strength", "1");

const rem = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
const snapPx = (snap) => snap.u === "rem" ? snap.v * rem : snap.u === "px" ? snap.v : snap.v * window.innerHeight;
const popupSize = () => (axis === "y" ? popup.offsetHeight : popup.offsetWidth);
const offsets = () => snaps.map((snap) => Math.max(0, popupSize() - snapPx(snap)));
let snapIndex = 0;
const applySnap = () => {
  if (useSnaps) popup.style.setProperty("--drawer-snap-point-offset", offsets()[snapIndex] + "px");
};
if (!useSnaps && axis === "y") {
  const natural = popup.offsetHeight;
  popup.style.setProperty("--drawer-height", natural + "px");
  dioxus.send("height:" + natural);
}
applySnap();
window.addEventListener("resize", applySnap);

popup.style.touchAction = axis === "y" ? "pan-x" : "pan-y";
popup.querySelectorAll("*").forEach((node) => {
  const style = getComputedStyle(node);
  const scrolls = axis === "y" ? /(auto|scroll)/.test(style.overflowY) : /(auto|scroll)/.test(style.overflowX);
  if (scrolls) node.style.touchAction = "pan-x pan-y";
});

const blocked = (start) => {
  for (let node = start; node && node !== popup; node = node.parentElement) {
    const style = getComputedStyle(node);
    if (axis === "y") {
      if (/(auto|scroll)/.test(style.overflowY) && node.scrollHeight > node.clientHeight) {
        if (dir === "down" && node.scrollTop > 0) return true;
        if (dir === "up" && node.scrollTop + node.clientHeight < node.scrollHeight - 1) return true;
      }
    } else if (/(auto|scroll)/.test(style.overflowX) && node.scrollWidth > node.clientWidth) {
      if (dir === "right" && node.scrollLeft > 0) return true;
      if (dir === "left" && node.scrollLeft + node.clientWidth < node.scrollWidth - 1) return true;
    }
  }
  return false;
};

let drag = null;
let lastDragEnd = 0;
popup.addEventListener("pointerdown", (event) => {
  if (event.pointerType === "mouse" && event.button !== 0) return;
  if (event.target.closest("input, textarea, select, [contenteditable], [data-swipe-ignore]")) return;
  if (blocked(event.target)) return;
  drag = { id: event.pointerId, x: event.clientX, y: event.clientY, started: false, samples: [], c: 0, total: 0 };
});
popup.addEventListener("pointermove", (event) => {
  if (!drag || event.pointerId !== drag.id) return;
  const dx = event.clientX - drag.x;
  const dy = event.clientY - drag.y;
  const along = axis === "y" ? dy : dx;
  const across = axis === "y" ? dx : dy;
  if (!drag.started) {
    if (Math.abs(across) > 6 && Math.abs(across) > Math.abs(along)) { drag = null; return; }
    if (Math.abs(along) < 6) return;
    drag.started = true;
    popup.setPointerCapture(event.pointerId);
    setSwiping(true);
  }
  const base = useSnaps ? offsets()[snapIndex] : 0;
  const floor = useSnaps ? Math.min(...offsets()) : 0;
  let total = base + sign * along;
  if (total < floor) total = floor + (total - floor) * 0.2;
  const c = total - base;
  drag.c = c;
  drag.total = total;
  drag.samples.push({ t: performance.now(), c });
  if (drag.samples.length > 8) drag.samples.shift();
  setVar(moveVar, sign * c + "px");
  setVar("--drawer-swipe-progress", String(Math.max(0, Math.min(1, c / popupSize()))));
});
const finish = (event, cancelled) => {
  if (!drag || event.pointerId !== drag.id) return;
  const current = drag;
  drag = null;
  if (!current.started) return;
  const now = performance.now();
  lastDragEnd = now;
  const recent = current.samples.filter((sample) => now - sample.t < 120);
  let velocity = 0;
  if (recent.length > 1) {
    const first = recent[0];
    const last = recent[recent.length - 1];
    velocity = (last.c - first.c) / Math.max(1, last.t - first.t);
  }
  const size = popupSize();
  let close = false;
  let target = snapIndex;
  if (!cancelled) {
    if (useSnaps) {
      const offs = offsets();
      const projected = current.total + velocity * 150;
      let best = 0;
      offs.forEach((offset, index) => {
        if (Math.abs(offset - projected) < Math.abs(offs[best] - projected)) best = index;
      });
      const lowest = Math.max(...offs);
      if (Math.abs(size - projected) < Math.abs(offs[best] - projected) && (velocity > 0.2 || projected > lowest + (size - lowest) / 2)) close = true;
      else target = best;
    } else {
      close = current.c > size * 0.5 || (velocity > 0.4 && current.c > 12);
    }
  }
  if (close) {
    const remaining = Math.max(0, size - current.total);
    const strength = Math.max(0.1, Math.min(1, remaining / Math.max(velocity, 0.3) / 400));
    setVar("--drawer-swipe-strength", String(strength));
    dioxus.send("close");
    setTimeout(() => {
      if (host.isConnected && !popup.hasAttribute("data-ending-style")) {
        setVar(moveVar, "0px");
        setVar("--drawer-swipe-progress", "0");
        setVar("--drawer-swipe-strength", "1");
        setSwiping(false);
      }
    }, 800);
  } else {
    if (target !== snapIndex) {
      snapIndex = target;
      dioxus.send("snap:" + snapIndex);
    }
    applySnap();
    setVar(moveVar, "0px");
    setVar("--drawer-swipe-progress", "0");
    setSwiping(false);
  }
};
popup.addEventListener("pointerup", (event) => finish(event, false));
popup.addEventListener("pointercancel", (event) => finish(event, true));
popup.addEventListener("click", (event) => {
  if (performance.now() - lastDragEnd < 80) { event.stopPropagation(); event.preventDefault(); }
}, true);
if (viewport) {
  viewport.addEventListener("click", (event) => {
    if (event.target === viewport && performance.now() - lastDragEnd > 120) dioxus.send("outside");
  });
}

if (mode === "trap") {
  host.addEventListener("keydown", (event) => {
    if (event.key !== "Tab") return;
    const items = [...host.querySelectorAll('a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])')].filter((node) => node.offsetParent !== null);
    if (items.length === 0) return;
    const first = items[0];
    const last = items[items.length - 1];
    if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
    else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
  });
}
const onDown = (event) => {
  if (!popup.contains(event.target) && !event.target.closest("[data-popup-open]")) dioxus.send("outside");
};
if (mode !== "modal") document.addEventListener("pointerdown", onDown, true);

await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
dioxus.send("ready");
await new Promise((resolve) => {
  const timer = setInterval(() => {
    if (!host.isConnected) { clearInterval(timer); resolve(); }
  }, 200);
});
document.removeEventListener("pointerdown", onDown, true);
window.removeEventListener("resize", applySnap);
"#;

const STACK_SCRIPT: &str = r#"
const popup = document.getElementById("__ID__");
if (popup) {
  popup.style.setProperty("--nested-drawers", "__NESTED__");
  if (__FRONT__ > 0) popup.style.setProperty("--drawer-frontmost-height", "__FRONT__px");
  else popup.style.removeProperty("--drawer-frontmost-height");
}
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
  await Promise.race([Promise.all(running), new Promise((resolve) => setTimeout(resolve, 1500))]);
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

/// How the rest of the page behaves while the drawer is open.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerModal {
    /// Overlay, inert page, focus trap and scroll lock.
    #[default]
    True,
    /// No overlay; the page stays interactive and focus can leave the drawer.
    False,
    /// No overlay and the page stays interactive, but focus is trapped inside the drawer.
    TrapFocus,
}

impl DrawerModal {
    fn mode(self) -> &'static str {
        match self {
            Self::True => "modal",
            Self::False => "free",
            Self::TrapFocus => "trap",
        }
    }

    fn data(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::TrapFocus => "trap-focus",
        }
    }
}

/// The direction the drawer is swiped to dismiss it. The drawer slides in from the opposite edge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSwipeDirection {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

impl DrawerSwipeDirection {
    fn data(self) -> &'static str {
        match self {
            Self::Down => "down",
            Self::Up => "up",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    fn axis(self) -> &'static str {
        match self {
            Self::Down | Self::Up => "y",
            Self::Left | Self::Right => "x",
        }
    }
}

/// A height the drawer can rest at. `Fraction` is a share of the viewport height (`1.0` is full height).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerSnapPoint {
    Fraction(f64),
    Px(f64),
    Rem(f64),
}

impl DrawerSnapPoint {
    fn js(self) -> String {
        match self {
            Self::Fraction(value) => format!("{{u:\"f\",v:{value}}}"),
            Self::Px(value) => format!("{{u:\"px\",v:{value}}}"),
            Self::Rem(value) => format!("{{u:\"rem\",v:{value}}}"),
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

/// What an ancestor drawer needs to know about the drawers opened from it.
#[derive(Clone, Copy)]
struct StackLink {
    nested: Signal<usize>,
    frontmost: Signal<f64>,
    height: Signal<f64>,
}

#[derive(Clone, Copy)]
struct DrawerContext {
    id: usize,
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    modal: Signal<DrawerModal>,
    disable_pointer_dismissal: Signal<bool>,
    swipe_direction: Signal<DrawerSwipeDirection>,
    show_swipe_handle: Signal<bool>,
    snap_points: Signal<Vec<DrawerSnapPoint>>,
    on_snap_point_change: Option<EventHandler<DrawerSnapPoint>>,
    trigger: Signal<Option<Rc<MountedData>>>,
    has_title: Signal<bool>,
    has_description: Signal<bool>,
    own: StackLink,
    ancestors: Signal<Vec<StackLink>>,
}

impl DrawerContext {
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

    fn has_snap_points(self) -> bool {
        !(self.snap_points)().is_empty() && (self.swipe_direction)() == DrawerSwipeDirection::Down
    }

    fn enter_stack(self) {
        for link in self.ancestors.peek().iter() {
            let mut nested = link.nested;
            *nested.write() += 1;
        }
    }

    fn leave_stack(self) {
        let ancestors = self.ancestors.peek().clone();
        let parent_height = ancestors.first().map(|link| *link.height.peek());
        for link in ancestors {
            let mut nested = link.nested;
            let remaining = nested.peek().saturating_sub(1);
            nested.set(remaining);
            let mut frontmost = link.frontmost;
            frontmost.set(if remaining == 0 {
                0.0
            } else {
                parent_height.unwrap_or(0.0)
            });
        }
    }

    fn host_id(self) -> String {
        format!("drawer-{}", self.id)
    }

    fn popup_id(self) -> String {
        format!("drawer-{}-popup", self.id)
    }

    fn title_id(self) -> String {
        format!("drawer-{}-title", self.id)
    }

    fn description_id(self) -> String {
        format!("drawer-{}-description", self.id)
    }
}

#[derive(Clone, Copy)]
struct PortalState {
    phase: Signal<Phase>,
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
///
/// A `Drawer` rendered inside another drawer's content is nested: the parent stays mounted and stacks
/// behind it.
#[component]
pub fn Drawer(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(default)] modal: DrawerModal,
    /// Render a drag handle at the swiping edge of the popup.
    #[props(default)]
    show_swipe_handle: bool,
    /// Heights the drawer rests at, from the first (the initial one) to the last. Applies to drawers
    /// that swipe down.
    #[props(default)]
    snap_points: Vec<DrawerSnapPoint>,
    /// Called with the snap point the drawer settled on after a swipe.
    on_snap_point_change: Option<EventHandler<DrawerSnapPoint>>,
    #[props(default)] swipe_direction: DrawerSwipeDirection,
    /// Keep the drawer open when the page outside it is pressed.
    #[props(default)]
    disable_pointer_dismissal: bool,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let modal = use_prop(modal);
    let disable_pointer_dismissal = use_prop(disable_pointer_dismissal);
    let swipe_direction = use_prop(swipe_direction);
    let show_swipe_handle = use_prop(show_swipe_handle);
    let snap_points = use_prop(snap_points);
    let trigger = use_signal(|| None::<Rc<MountedData>>);
    let has_title = use_signal(|| false);
    let has_description = use_signal(|| false);
    let own = StackLink {
        nested: use_signal(|| 0),
        frontmost: use_signal(|| 0.0),
        height: use_signal(|| 0.0),
    };
    let parent = try_use_context::<DrawerContext>();
    let ancestors = use_signal(move || {
        parent
            .map(|parent| {
                let mut chain = vec![parent.own];
                chain.extend(parent.ancestors.peek().iter().copied());
                chain
            })
            .unwrap_or_default()
    });
    use_context_provider(|| DrawerContext {
        id,
        internal,
        controlled,
        on_open_change,
        modal,
        disable_pointer_dismissal,
        swipe_direction,
        show_swipe_handle,
        snap_points,
        on_snap_point_change,
        trigger,
        has_title,
        has_description,
        own,
        ancestors,
    });

    rsx! { {children} }
}

/// The button that opens the drawer. Plain by default; pass `variant` or `size` to give it the button look.
#[component]
pub fn DrawerTrigger(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
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
            "data-slot": "drawer-trigger",
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

/// Closes the drawer after running `onclick`. Plain by default; pass `variant` or `size` to give it the button look.
#[component]
pub fn DrawerClose(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
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
            "data-slot": "drawer-close",
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

/// The `<dialog>` host that carries the overlay and the viewport. Renders nothing while the drawer
/// is closed, and keeps rendering during the exit transition.
#[component]
fn DrawerPortal(children: Element) -> Element {
    let ctx = use_context::<DrawerContext>();
    let mut phase = use_signal(|| Phase::Closed);
    let locked = use_hook(|| Rc::new(Cell::new(false)));
    let stacked = use_hook(|| Rc::new(Cell::new(false)));
    use_context_provider(|| PortalState { phase });
    let id = ctx.host_id();

    use_drop({
        let locked = locked.clone();
        let stacked = stacked.clone();
        let id = id.clone();
        move || {
            if locked.get() {
                let _ = document::eval(&UNLOCK_SCRIPT.replace("__ID__", &id));
            }
            if stacked.replace(false) {
                ctx.leave_stack();
            }
        }
    });

    use_effect({
        let locked = locked.clone();
        let stacked = stacked.clone();
        let id = id.clone();
        move || {
            let open = ctx.is_open();
            let current = *phase.peek();
            if open && current == Phase::Closed {
                phase.set(Phase::Starting);
                if !stacked.replace(true) {
                    ctx.enter_stack();
                }
            } else if open && current == Phase::Ending {
                phase.set(Phase::Starting);
                if !stacked.replace(true) {
                    ctx.enter_stack();
                }
                spawn(async move {
                    let mut eval = document::eval(FRAMES_SCRIPT);
                    let _ = eval.recv::<bool>().await;
                    if *phase.peek() == Phase::Starting {
                        phase.set(Phase::Open);
                    }
                });
            } else if !open && matches!(current, Phase::Starting | Phase::Open) {
                phase.set(Phase::Ending);
                if stacked.replace(false) {
                    ctx.leave_stack();
                }
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
        (modal != DrawerModal::True).then_some("pointer-events-none")
    );
    let mount = {
        let locked = locked.clone();
        let id = id.clone();
        move |_| {
            let locked = locked.clone();
            let id = id.clone();
            async move {
                let modal = *ctx.modal.peek();
                let lock = if modal == DrawerModal::True {
                    locked.set(true);
                    LOCK_SCRIPT.replace("__ID__", &id)
                } else {
                    String::new()
                };
                let snaps = ctx
                    .snap_points
                    .peek()
                    .iter()
                    .map(|snap| snap.js())
                    .collect::<Vec<_>>()
                    .join(",");
                let script = MOUNT_SCRIPT
                    .replace("__LOCK__", &lock)
                    .replace("__ID__", &id)
                    .replace("__DIR__", ctx.swipe_direction.peek().data())
                    .replace("__MODE__", modal.mode())
                    .replace("__SNAPS__", &format!("[{snaps}]"));
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<String>().await {
                    match message.as_str() {
                        "ready" if *phase.peek() == Phase::Starting => phase.set(Phase::Open),
                        "outside" if !*ctx.disable_pointer_dismissal.peek() => ctx.request_close(),
                        "close" => ctx.request_close(),
                        other => {
                            if let Some(height) = other.strip_prefix("height:") {
                                if let Ok(height) = height.parse::<f64>() {
                                    let mut own = ctx.own.height;
                                    own.set(height);
                                    for link in ctx.ancestors.peek().iter() {
                                        let mut frontmost = link.frontmost;
                                        frontmost.set(height);
                                    }
                                }
                            } else if let Some(index) = other.strip_prefix("snap:")
                                && let Ok(index) = index.parse::<usize>()
                                && let Some(handler) = ctx.on_snap_point_change
                                && let Some(point) = ctx.snap_points.peek().get(index).copied()
                            {
                                handler.call(point);
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        dialog {
            id: "{id}",
            "data-slot": "drawer-portal",
            class: class,
            onmounted: mount,
            onkeydown: move |event| {
                if event.key() == Key::Escape && ctx.is_open() {
                    event.stop_propagation();
                    ctx.request_close();
                }
            },
            {children}
        }
    }
}

#[component]
fn DrawerOverlay(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    let phase = (use_context::<PortalState>().phase)();
    let mut attributes = attributes;
    let class = cn!(
        "bg-black/10 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50 min-h-dvh opacity-[max(var(--drawer-overlay-min-opacity,0),calc(1-var(--drawer-swipe-progress)))] transition-opacity duration-450 ease-[cubic-bezier(0.32,0.72,0,1)] select-none data-ending-style:pointer-events-none data-ending-style:opacity-0 data-ending-style:duration-[calc(var(--drawer-swipe-strength)*400ms)] data-snap-points:[--drawer-overlay-min-opacity:0.5] data-starting-style:opacity-0 data-swiping:duration-0 supports-[-webkit-touch-callout:none]:absolute",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "drawer-overlay",
            "data-snap-points": ctx.has_snap_points().then_some(""),
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

/// A grabbable bar at the swiping edge of the popup. Shown by `show_swipe_handle` on `Drawer`.
#[component]
pub fn DrawerSwipeHandle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "after:bg-muted after:block after:shrink-0 after:rounded-full group-data-[swipe-axis=y]/drawer-popup:h-3 group-data-[swipe-axis=y]/drawer-popup:w-full group-data-[swipe-axis=y]/drawer-popup:justify-center group-data-[swipe-axis=y]/drawer-popup:after:h-1.5 group-data-[swipe-axis=y]/drawer-popup:after:w-[100px] group-data-[swipe-direction=down]/drawer-popup:items-end group-data-[swipe-direction=up]/drawer-popup:items-start group-data-[swipe-axis=x]/drawer-popup:h-full group-data-[swipe-axis=x]/drawer-popup:w-3 group-data-[swipe-axis=x]/drawer-popup:items-center group-data-[swipe-axis=x]/drawer-popup:after:h-[100px] group-data-[swipe-axis=x]/drawer-popup:after:w-1.5 group-data-[swipe-direction=left]/drawer-popup:justify-start group-data-[swipe-direction=right]/drawer-popup:justify-end relative z-10 flex shrink-0 cursor-grab transition-opacity duration-200 group-data-nested-drawer-open/drawer-popup:opacity-0 group-data-nested-drawer-swiping/drawer-popup:opacity-100 group-data-[swipe-direction=left]/drawer-popup:order-last group-data-[swipe-direction=up]/drawer-popup:order-last active:cursor-grabbing",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "drawer-swipe-handle",
            aria_hidden: "true",
            class: class,
            ..attributes,
        }
    }
}

#[component]
fn DrawerPopup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    let phase = (use_context::<PortalState>().phase)();
    let direction = (ctx.swipe_direction)();
    let nested = (ctx.own.nested)();
    let popup_id = ctx.popup_id();

    use_effect({
        let popup_id = popup_id.clone();
        move || {
            let nested = (ctx.own.nested)();
            let frontmost = (ctx.own.frontmost)();
            let script = STACK_SCRIPT
                .replace("__ID__", &popup_id)
                .replace("__NESTED__", &nested.to_string())
                .replace("__FRONT__", &frontmost.to_string());
            let _ = document::eval(&script);
        }
    });

    let mut attributes = attributes;
    let class = cn!(
        // Base.
        "bg-popover text-popover-foreground text-sm data-[swipe-direction=down]:rounded-t-xl data-[swipe-direction=down]:border-t data-[swipe-direction=left]:rounded-r-xl data-[swipe-direction=left]:border-r data-[swipe-direction=right]:rounded-l-xl data-[swipe-direction=right]:border-l data-[swipe-direction=up]:rounded-b-xl data-[swipe-direction=up]:border-b group/drawer-popup pointer-events-auto fixed z-50 m-(--drawer-inset,0px) flex h-(--drawer-content-height) max-h-(--drawer-content-max-height,none) min-h-0 w-(--drawer-content-width,auto) transform-[translate3d(var(--translate-x,0px),var(--translate-y,0px),0)_scale(var(--stack-scale))] flex-col transition-[transform,height,opacity,filter] duration-450 ease-[cubic-bezier(0.22,1,0.36,1)] will-change-transform outline-none select-none [interpolate-size:allow-keywords]",
        // Nested.
        "data-nested-drawer-open:overflow-hidden data-nested-drawer-open:brightness-95",
        // Bleed.
        "after:pointer-events-none after:absolute after:bg-(--drawer-bleed-background,var(--color-popover)) data-[swipe-axis=x]:after:inset-y-0 data-[swipe-axis=x]:after:w-(--bleed) data-[swipe-axis=y]:after:inset-x-0 data-[swipe-axis=y]:after:h-(--bleed) data-[swipe-direction=down]:after:top-full data-[swipe-direction=left]:after:right-full data-[swipe-direction=right]:after:left-full data-[swipe-direction=up]:after:bottom-full",
        // Sizing.
        "[--drawer-content-height:var(--drawer-height,auto)] data-[swipe-axis=x]:[--drawer-content-width:75%] data-[swipe-axis=y]:[--drawer-content-max-height:calc(100dvh-6rem)] data-[swipe-axis=y]:data-snap-points:[--drawer-content-height:100dvh] data-[swipe-axis=x]:sm:[--drawer-content-width:24rem]",
        // Stack.
        "[--bleed:3rem] [--peek:1rem] [--stack-height:var(--drawer-frontmost-height,var(--drawer-height,0px))] [--stack-peek-offset:max(0px,calc((var(--nested-drawers)-var(--stack-progress))*var(--peek)))] [--stack-progress:clamp(0,var(--drawer-swipe-progress),1)] [--stack-scale-base:max(0,calc(1-(var(--nested-drawers)*var(--stack-step))))] [--stack-scale:clamp(0,calc(var(--stack-scale-base)+(var(--stack-step)*var(--stack-progress))),1)] [--stack-shrink:calc(1-var(--stack-scale))] [--stack-step:0.05]",
        // Transitions.
        "data-ending-style:transform-(--closed-transform) data-ending-style:opacity-[0.9999] data-ending-style:duration-[calc(var(--drawer-swipe-strength)*400ms)] data-nested-drawer-swiping:duration-0 data-ending-style:data-nested-drawer-swiping:duration-[calc(var(--drawer-swipe-strength)*400ms)] data-starting-style:transform-(--closed-transform) data-swiping:duration-0 data-ending-style:data-swiping:duration-[calc(var(--drawer-swipe-strength)*400ms)]",
        // Axis: y.
        "data-[swipe-axis=y]:inset-x-0 data-[swipe-axis=y]:data-nested-drawer-open:h-(--stack-height)",
        // Axis: x.
        "data-[swipe-axis=x]:inset-y-0 data-[swipe-axis=x]:flex-row",
        // Direction: down.
        "data-[swipe-direction=down]:bottom-0 data-[swipe-direction=down]:origin-bottom data-[swipe-direction=down]:[--closed-transform:translate3d(0,calc(100%+var(--drawer-inset,0px)+2px),0)] data-[swipe-direction=down]:[--translate-y:calc(var(--drawer-snap-point-offset,0px)+var(--drawer-swipe-movement-y)-var(--stack-peek-offset)-(var(--stack-shrink)*var(--stack-height)))]",
        // Direction: up.
        "data-[swipe-direction=up]:top-0 data-[swipe-direction=up]:origin-top data-[swipe-direction=up]:[--closed-transform:translate3d(0,calc(-100%-var(--drawer-inset,0px)-2px),0)] data-[swipe-direction=up]:[--translate-y:calc(var(--drawer-snap-point-offset,0px)+var(--drawer-swipe-movement-y)+var(--stack-peek-offset)+(var(--stack-shrink)*var(--stack-height)))]",
        // Direction: left.
        "data-[swipe-direction=left]:left-0 data-[swipe-direction=left]:origin-left data-[swipe-direction=left]:[--closed-transform:translate3d(calc(-100%-var(--drawer-inset,0px)-2px),0,0)] data-[swipe-direction=left]:[--translate-x:calc(var(--drawer-swipe-movement-x)+var(--stack-peek-offset)+(var(--stack-shrink)*100%))]",
        // Direction: right.
        "data-[swipe-direction=right]:right-0 data-[swipe-direction=right]:origin-right data-[swipe-direction=right]:[--closed-transform:translate3d(calc(100%+var(--drawer-inset,0px)+2px),0,0)] data-[swipe-direction=right]:[--translate-x:calc(var(--drawer-swipe-movement-x)-var(--stack-peek-offset)-(var(--stack-shrink)*100%))]",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            id: popup_id,
            "data-slot": "drawer-popup",
            "data-swipe-axis": direction.axis(),
            "data-swipe-direction": direction.data(),
            "data-snap-points": ctx.has_snap_points().then_some(""),
            "data-nested-drawer-open": (nested > 0).then_some(""),
            "data-open": phase.open(),
            "data-closed": phase.closed(),
            "data-starting-style": phase.starting(),
            "data-ending-style": phase.ending(),
            role: "dialog",
            aria_modal: (*ctx.modal.peek() == DrawerModal::True).then_some("true"),
            aria_labelledby: (ctx.has_title)().then(|| ctx.title_id()),
            aria_describedby: (ctx.has_description)().then(|| ctx.description_id()),
            tabindex: "-1",
            class: class,
            ..attributes,
            if (ctx.show_swipe_handle)() {
                DrawerSwipeHandle {}
            }
            div {
                "data-slot": "drawer-content",
                class: "flex min-h-0 flex-1 flex-col overflow-hidden overscroll-contain rounded-[inherit] transition-opacity duration-300 ease-[cubic-bezier(0.45,1.005,0,1.005)] select-text group-data-nested-drawer-open/drawer-popup:opacity-0 group-data-nested-drawer-swiping/drawer-popup:opacity-100 group-data-swiping/drawer-popup:select-none",
                {children}
            }
        }
    }
}

/// The drawer popup, its overlay and its swipe handle. Renders nothing while the drawer is closed.
#[component]
pub fn DrawerContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    let modal = (ctx.modal)();

    rsx! {
        DrawerPortal {
            if modal == DrawerModal::True {
                DrawerOverlay {}
            }
            div {
                "data-slot": "drawer-viewport",
                "data-modal": modal.data(),
                class: "pointer-events-none fixed inset-0 z-50 select-none data-[modal=true]:pointer-events-auto",
                DrawerPopup { attributes, {children} }
            }
        }
    }
}

#[component]
pub fn DrawerHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-0.5 p-4 pb-0 md:gap-1.5 md:text-left flex shrink-0 flex-col group-data-[swipe-axis=y]/drawer-popup:text-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "drawer-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DrawerFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 p-4 pt-0 mt-auto flex shrink-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "drawer-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DrawerTitle(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    let mut has_title = ctx.has_title;
    use_effect(move || has_title.set(true));
    use_drop(move || has_title.set(false));
    let mut attributes = attributes;
    let class = cn!(
        "text-foreground font-medium font-heading",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h2 {
            id: ctx.title_id(),
            "data-slot": "drawer-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DrawerDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    let mut has_description = ctx.has_description;
    use_effect(move || has_description.set(true));
    use_drop(move || has_description.set(false));
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground text-sm text-balance",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            id: ctx.description_id(),
            "data-slot": "drawer-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}
