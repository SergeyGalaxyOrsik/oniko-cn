//! Select: a button that opens a popup list of options and picks one of them.
//!
//! The popup lives in a `popover="manual"` element, so it sits in the top layer and is never clipped
//! by an ancestor. A small script positions it from `side`, `align`, `side_offset` and `align_offset`,
//! flips and shifts it at the viewport edges and publishes `--anchor-width`, `--available-height` and
//! `--transform-origin`. With `align_item_with_trigger` the selected item is placed over the trigger.
//! Items stay mounted while the popup is closed, so they register their labels with the root and
//! `SelectValue` can show the chosen label straight away. `ArrowUp`, `ArrowDown`, `Home` and `End` move
//! between options, typing jumps to the next option that starts with the typed text, `Enter` and
//! `Space` pick the focused option, and `Escape` or `Tab` close the popup and return focus to the
//! trigger. No `dioxus-primitives` crate is used: it is not published on crates.io.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const SESSION_SCRIPT: &str = r#"
const trigger = document.getElementById("__TRIGGER__");
const positioner = document.getElementById("__POSITIONER__");
const popup = document.getElementById("__POPUP__");
if (!trigger || !positioner || !popup) { dioxus.send("ready"); return; }
const cfg = { side: "__SIDE__", align: "__ALIGN__", sideOffset: __SIDE_OFFSET__, alignOffset: __ALIGN_OFFSET__, alignItem: __ALIGN_ITEM__ };
const PAD = 5;
const ac = new AbortController();
const on = (target, type, handler, capture = false) => target.addEventListener(type, handler, { signal: ac.signal, capture });
const list = popup.querySelector('[role="listbox"]');
const up = popup.querySelector('[data-slot="select-scroll-up-button"]');
const down = popup.querySelector('[data-slot="select-scroll-down-button"]');
const options = () => [...popup.querySelectorAll('[role="option"]:not([aria-disabled="true"])')];
if (!positioner.matches(":popover-open")) positioner.showPopover();

const opposite = { top: "bottom", bottom: "top", left: "right", right: "left" };
const clamp = (value, min, max) => Math.max(min, Math.min(value, max));

function arrows() {
  const max = popup.scrollHeight - popup.clientHeight;
  if (up) { up.hidden = !(popup.scrollTop > 1); up.style.transform = "translateY(" + popup.scrollTop + "px)"; }
  if (down) { down.hidden = !(popup.scrollTop < max - 1); down.style.transform = "translateY(" + popup.scrollTop + "px)"; }
}

function place() {
  const a = trigger.getBoundingClientRect();
  const vw = document.documentElement.clientWidth;
  const vh = document.documentElement.clientHeight;
  const rtl = getComputedStyle(popup).direction === "rtl";
  const set = (name, value) => positioner.style.setProperty(name, value);
  const mark = (side) => {
    positioner.dataset.side = side; popup.dataset.side = side;
    positioner.dataset.align = cfg.align; popup.dataset.align = cfg.align;
  };
  set("--anchor-width", a.width + "px");
  set("--anchor-height", a.height + "px");
  positioner.style.left = "0px";
  positioner.style.top = "0px";

  if (cfg.alignItem) {
    set("--available-height", vh - 2 * PAD + "px");
    set("--available-width", vw - 2 * PAD + "px");
    popup.scrollTop = 0;
    const p = popup.getBoundingClientRect();
    const selected = list.querySelector('[role="option"][data-selected]') || options()[0];
    let top = a.top + a.height / 2 - p.height / 2;
    let left = rtl ? a.right - p.width : a.left;
    let center = p.height / 2;
    if (selected) {
      const s = selected.getBoundingClientRect();
      const t = (selected.firstElementChild || selected).getBoundingClientRect();
      const v = (trigger.querySelector('[data-slot="select-value"]') || trigger).getBoundingClientRect();
      center = s.top - p.top + s.height / 2;
      top = a.top + a.height / 2 - center;
      left = rtl ? v.right - (p.right - t.right) - p.width : v.left - (t.left - p.left);
    }
    const placedTop = clamp(top, PAD, Math.max(PAD, vh - p.height - PAD));
    const placedLeft = clamp(left, PAD, Math.max(PAD, vw - p.width - PAD));
    positioner.style.top = placedTop + "px";
    positioner.style.left = placedLeft + "px";
    set("--transform-origin", "center");
    mark("none");
    if (selected) popup.scrollTop = clamp(placedTop + center - (a.top + a.height / 2), 0, popup.scrollHeight - popup.clientHeight);
    arrows();
    return;
  }

  let side = cfg.side;
  if (side === "inline-start") side = rtl ? "right" : "left";
  if (side === "inline-end") side = rtl ? "left" : "right";
  const vertical = (s) => s === "top" || s === "bottom";
  const avail = (s) => Math.max(0, s === "bottom" ? vh - a.bottom - cfg.sideOffset - PAD
    : s === "top" ? a.top - cfg.sideOffset - PAD
    : s === "right" ? vw - a.right - cfg.sideOffset - PAD
    : a.left - cfg.sideOffset - PAD);
  const apply = (s, natural) => {
    set("--available-height", (natural || !vertical(s) ? vh - 2 * PAD : avail(s)) + "px");
    set("--available-width", (natural || vertical(s) ? vw - 2 * PAD : avail(s)) + "px");
  };
  apply(side, true);
  let r = popup.getBoundingClientRect();
  const flipped = (vertical(side) ? r.height : r.width) > avail(side) && avail(opposite[side]) > avail(side);
  if (flipped) side = opposite[side];
  apply(side, false);
  r = popup.getBoundingClientRect();
  const w = r.width, h = r.height;
  let left, top;
  if (vertical(side)) {
    top = side === "bottom" ? a.bottom + cfg.sideOffset : a.top - cfg.sideOffset - h;
    if (cfg.align === "start") left = rtl ? a.right - w - cfg.alignOffset : a.left + cfg.alignOffset;
    else if (cfg.align === "end") left = rtl ? a.left + cfg.alignOffset : a.right - w - cfg.alignOffset;
    else left = a.left + (a.width - w) / 2 + cfg.alignOffset;
  } else {
    left = side === "right" ? a.right + cfg.sideOffset : a.left - cfg.sideOffset - w;
    if (cfg.align === "start") top = a.top + cfg.alignOffset;
    else if (cfg.align === "end") top = a.bottom - h - cfg.alignOffset;
    else top = a.top + (a.height - h) / 2 + cfg.alignOffset;
  }
  left = clamp(left, PAD, Math.max(PAD, vw - w - PAD));
  top = clamp(top, PAD, Math.max(PAD, vh - h - PAD));
  positioner.style.left = left + "px";
  positioner.style.top = top + "px";
  const ox = vertical(side) ? clamp(a.left + a.width / 2 - left, 0, w) : (side === "right" ? 0 : w);
  const oy = vertical(side) ? (side === "bottom" ? 0 : h) : clamp(a.top + a.height / 2 - top, 0, h);
  set("--transform-origin", ox + "px " + oy + "px");
  let reported = cfg.side;
  if (flipped) reported = cfg.side === "inline-start" ? "inline-end" : cfg.side === "inline-end" ? "inline-start" : side;
  mark(reported);
  arrows();
}

let raf = 0;
const schedule = () => { cancelAnimationFrame(raf); raf = requestAnimationFrame(place); };
on(window, "resize", schedule);
if (!cfg.alignItem) on(window, "scroll", (event) => { if (!popup.contains(event.target)) schedule(); }, true);
on(popup, "scroll", arrows);

const focusOption = (option) => {
  option.focus({ preventScroll: true });
  option.scrollIntoView({ block: "nearest" });
};
on(popup, "focusin", (event) => {
  popup.querySelectorAll("[data-highlighted]").forEach((node) => node.removeAttribute("data-highlighted"));
  const option = event.target.closest && event.target.closest('[role="option"]');
  if (option) option.setAttribute("data-highlighted", "");
});
on(popup, "focusout", (event) => {
  if (event.target.removeAttribute) event.target.removeAttribute("data-highlighted");
});
on(popup, "pointermove", (event) => {
  const option = event.target.closest('[role="option"]:not([aria-disabled="true"])');
  if (option && document.activeElement !== option) option.focus({ preventScroll: true });
});
for (const [arrow, direction] of [[up, -1], [down, 1]]) {
  if (!arrow) continue;
  let frame = 0;
  const step = () => { popup.scrollTop += direction * 5; frame = requestAnimationFrame(step); };
  on(arrow, "pointerenter", () => { cancelAnimationFrame(frame); frame = requestAnimationFrame(step); });
  on(arrow, "pointerleave", () => cancelAnimationFrame(frame));
  on(popup, "pointerleave", () => cancelAnimationFrame(frame));
}

let buffer = "";
let timer = 0;
on(document, "keydown", (event) => {
  const key = event.key;
  const active = document.activeElement;
  const inside = popup.contains(active);
  if (key === "Escape") {
    event.preventDefault(); event.stopPropagation();
    dioxus.send("escape");
    return;
  }
  if (!inside && active !== trigger) return;
  if (key === "Tab") {
    event.preventDefault();
    dioxus.send("tab");
    return;
  }
  const items = options();
  if (["ArrowDown", "ArrowUp", "Home", "End"].includes(key)) {
    event.preventDefault(); event.stopPropagation();
    if (!items.length) return;
    const i = items.indexOf(active);
    const last = items.length - 1;
    const next = key === "Home" ? 0 : key === "End" ? last
      : key === "ArrowDown" ? (i < 0 ? 0 : (i + 1) % items.length)
      : (i < 0 ? last : (i - 1 + items.length) % items.length);
    focusOption(items[next]);
    return;
  }
  if ((key === "Enter" || (key === " " && !buffer)) && active.matches && active.matches('[role="option"]')) {
    event.preventDefault(); event.stopPropagation();
    active.click();
    return;
  }
  if (key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
    event.preventDefault(); event.stopPropagation();
    buffer += key.toLowerCase();
    clearTimeout(timer);
    timer = setTimeout(() => { buffer = ""; }, 500);
    const start = Math.max(0, items.indexOf(active));
    const ordered = items.slice(start + (buffer.length === 1 ? 1 : 0)).concat(items.slice(0, start + (buffer.length === 1 ? 1 : 0)));
    const match = ordered.find((item) => item.textContent.trim().toLowerCase().startsWith(buffer));
    if (match) focusOption(match);
  }
}, true);
on(document, "pointerdown", (event) => {
  if (!popup.contains(event.target) && !trigger.contains(event.target)) dioxus.send("outside");
}, true);

place();
await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
place();
const initial = list.querySelector('[role="option"][data-selected]') || null;
(initial || list).focus({ preventScroll: true });
if (initial) initial.scrollIntoView({ block: "nearest" });
dioxus.send("ready");
await new Promise((resolve) => {
  const poll = setInterval(() => {
    if (!popup.isConnected || popup.hasAttribute("data-ending-style") || popup.hasAttribute("data-closed")) {
      clearInterval(poll);
      resolve();
    }
  }, 50);
});
ac.abort();
cancelAnimationFrame(raf);
clearTimeout(timer);
"#;

const EXIT_SCRIPT: &str = r#"
const positioner = document.getElementById("__POSITIONER__");
const popup = document.getElementById("__POPUP__");
if (positioner) {
  await new Promise((resolve) => requestAnimationFrame(() => resolve()));
  const running = positioner.getAnimations({ subtree: true })
    .filter((animation) => {
      const timing = animation.effect && animation.effect.getComputedTiming();
      return timing && Number.isFinite(timing.endTime);
    })
    .map((animation) => animation.finished.catch(() => {}));
  await Promise.race([Promise.all(running), new Promise((resolve) => setTimeout(resolve, 1000))]);
  if (popup && popup.hasAttribute("data-ending-style") && positioner.matches(":popover-open")) positioner.hidePopover();
}
dioxus.send(true);
"#;

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// Removes the caller's `id` from the spread attributes, so the component can use it as the DOM id the
/// positioning script looks up.
fn take_id(attributes: &mut Vec<Attribute>) -> Option<String> {
    let mut found = None;
    attributes.retain(|attr| {
        if attr.name != "id" || attr.namespace.is_some() {
            return true;
        }
        if let dioxus::core::AttributeValue::Text(text) = &attr.value {
            found = Some(text.clone());
        }
        false
    });
    found
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

/// One entry of the `items` prop. A `value` of `None` is the "nothing selected" entry, whose label
/// `SelectValue` shows while no item is selected.
#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption {
    pub label: String,
    pub value: Option<String>,
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: Some(value.into()),
        }
    }

    /// The entry shown while nothing is selected.
    pub fn none(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Entry {
    id: usize,
    value: Option<String>,
    label: String,
}

#[derive(Clone, Copy)]
struct SelectContext {
    id: usize,
    /// Value from the `value` prop. An empty string means "controlled, nothing selected".
    controlled: Signal<Option<String>>,
    internal: Signal<Option<String>>,
    controlled_open: Signal<Option<bool>>,
    internal_open: Signal<bool>,
    items: Signal<Option<Vec<SelectOption>>>,
    entries: Signal<Vec<Entry>>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    trigger_id: Signal<String>,
    trigger: Signal<Option<Rc<MountedData>>>,
    on_value_change: Option<EventHandler<Option<String>>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl SelectContext {
    fn value(self) -> Option<String> {
        match (self.controlled)() {
            Some(value) if value.is_empty() => None,
            Some(value) => Some(value),
            None => (self.internal)(),
        }
    }

    fn is_open(self) -> bool {
        (self.controlled_open)().unwrap_or_else(|| (self.internal_open)())
    }

    fn set_open(self, open: bool) {
        if open && ((self.disabled)() || (self.read_only)()) {
            return;
        }
        let mut internal = self.internal_open;
        if self.controlled_open.peek().is_none() {
            internal.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }

    fn label_of(self, value: &Option<String>) -> Option<String> {
        if let Some(items) = &*self.items.read()
            && let Some(item) = items.iter().find(|item| item.value == *value)
        {
            return Some(item.label.clone());
        }
        self.entries
            .read()
            .iter()
            .find(|entry| entry.value == *value)
            .map(|entry| entry.label.clone())
    }

    fn popup_id(self) -> String {
        format!("select-{}-popup", self.id)
    }

    fn positioner_id(self) -> String {
        format!("select-{}-positioner", self.id)
    }

    fn list_id(self) -> String {
        format!("select-{}-list", self.id)
    }

    fn focus_trigger(self) {
        if let Some(trigger) = self.trigger.peek().clone() {
            spawn(async move {
                let _ = trigger.set_focus(true).await;
            });
        }
    }

    fn close_and_focus_trigger(self) {
        self.set_open(false);
        self.focus_trigger();
    }

    fn select(self, value: Option<String>) {
        let mut internal = self.internal;
        if self.controlled.peek().is_none() {
            internal.set(value.clone());
        }
        if let Some(handler) = self.on_value_change {
            handler.call(value);
        }
        self.close_and_focus_trigger();
    }
}

/// Owns the selection and the open state. Pass `value` to control the selection; an empty string
/// controls it with nothing selected.
#[component]
pub fn Select(
    /// Controlled selection. When set, the parent must update it from `on_value_change`.
    value: Option<String>,
    /// Selection at first render when `value` is not controlled.
    default_value: Option<String>,
    on_value_change: Option<EventHandler<Option<String>>>,
    /// Controlled open state.
    open: Option<bool>,
    #[props(default)] default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    /// The options and their labels, so `SelectValue` can show a label before the items have mounted.
    items: Option<Vec<SelectOption>>,
    #[props(default)] disabled: bool,
    #[props(default)] read_only: bool,
    #[props(default)] required: bool,
    /// Name of the hidden input that submits the selection with a form.
    name: Option<String>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let controlled = use_prop(value);
    let internal = use_signal(|| default_value);
    let controlled_open = use_prop(open);
    let internal_open = use_signal(|| default_open);
    let items = use_prop(items);
    let entries = use_signal(Vec::new);
    let disabled_signal = use_prop(disabled);
    let read_only_signal = use_prop(read_only);
    let trigger_id = use_signal(|| format!("select-{id}-trigger"));
    let trigger = use_signal(|| None::<Rc<MountedData>>);
    let ctx = use_context_provider(|| SelectContext {
        id,
        controlled,
        internal,
        controlled_open,
        internal_open,
        items,
        entries,
        disabled: disabled_signal,
        read_only: read_only_signal,
        trigger_id,
        trigger,
        on_value_change,
        on_open_change,
    });

    rsx! {
        {children}
        if let Some(name) = name {
            input {
                r#type: "hidden",
                name: name,
                value: ctx.value().unwrap_or_default(),
                disabled: disabled,
                required: required,
            }
        }
    }
}

/// Shows the label of the selected item, or the label of the `items` entry without a value, or
/// `placeholder` while nothing is selected.
#[component]
pub fn SelectValue(
    placeholder: Option<String>,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut attributes = attributes;
    let class = cn!("cn-select-value", take_class(&mut attributes).as_deref());
    let value = ctx.value();
    let text = match &value {
        Some(selected) => ctx
            .label_of(&value)
            .unwrap_or_else(|| selected.clone()),
        None => ctx
            .label_of(&None)
            .or(placeholder)
            .unwrap_or_default(),
    };

    rsx! {
        span {
            "data-slot": "select-value",
            "data-placeholder": value.is_none().then_some(""),
            class: class,
            ..attributes,
            "{text}"
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectTriggerSize {
    #[default]
    Default,
    Sm,
}

impl SelectTriggerSize {
    fn data(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

/// The button that opens the popup. Put a `SelectValue` inside it.
#[component]
pub fn SelectTrigger(
    #[props(default)] size: SelectTriggerSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut trigger_id = ctx.trigger_id;
    let mut trigger = ctx.trigger;
    let mut attributes = attributes;
    let class = cn!(
        "cn-select-trigger flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
        take_class(&mut attributes).as_deref()
    );
    let id = take_id(&mut attributes).unwrap_or_else(|| format!("select-{}-trigger", ctx.id));
    if *trigger_id.peek() != id {
        trigger_id.set(id.clone());
    }
    let open = ctx.is_open();
    let disabled = (ctx.disabled)();
    let placeholder = ctx.value().is_none().then_some("");

    rsx! {
        button {
            id: id,
            "data-slot": "select-trigger",
            "data-size": size.data(),
            "data-popup-open": open.then_some(""),
            "data-pressed": open.then_some(""),
            "data-placeholder": placeholder,
            "data-disabled": disabled.then_some(""),
            r#type: "button",
            role: "combobox",
            aria_haspopup: "listbox",
            aria_expanded: open,
            aria_controls: open.then(|| ctx.list_id()),
            aria_readonly: (ctx.read_only)().then_some("true"),
            disabled: disabled,
            class: class,
            onmounted: move |event| trigger.set(Some(event.data())),
            onclick: move |_| ctx.set_open(!open),
            onkeydown: move |event: KeyboardEvent| {
                if !open && matches!(event.key(), Key::ArrowDown | Key::ArrowUp) {
                    event.prevent_default();
                    ctx.set_open(true);
                }
            },
            ..attributes,
            {children}
            svg {
                class: "cn-select-trigger-icon pointer-events-none",
                "aria-hidden": "true",
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

/// Which side of the trigger the popup opens on. The inline sides follow the text direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
    InlineStart,
    InlineEnd,
}

impl SelectSide {
    fn name(self) -> &'static str {
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
pub enum SelectAlign {
    Start,
    #[default]
    Center,
    End,
}

impl SelectAlign {
    fn name(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Placement {
    side: SelectSide,
    side_offset: f64,
    align: SelectAlign,
    align_offset: f64,
    align_item_with_trigger: bool,
}

/// The popup under the trigger. It stays mounted while closed so its items can register their labels.
#[component]
pub fn SelectContent(
    #[props(default)] side: SelectSide,
    #[props(default = 4.0)] side_offset: f64,
    #[props(default)] align: SelectAlign,
    #[props(default = 0.0)] align_offset: f64,
    #[props(default = true)] align_item_with_trigger: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut phase = use_signal(|| Phase::Closed);
    let placement = use_prop(Placement {
        side,
        side_offset,
        align,
        align_offset,
        align_item_with_trigger,
    });

    use_effect(move || {
        let open = ctx.is_open();
        let current = *phase.peek();
        if open && matches!(current, Phase::Closed | Phase::Ending) {
            phase.set(Phase::Starting);
            let placement = *placement.peek();
            let script = SESSION_SCRIPT
                .replace("__TRIGGER__", &ctx.trigger_id.peek())
                .replace("__POSITIONER__", &ctx.positioner_id())
                .replace("__POPUP__", &ctx.popup_id())
                .replace("__SIDE__", placement.side.name())
                .replace("__ALIGN__", placement.align.name())
                .replace("__SIDE_OFFSET__", &placement.side_offset.to_string())
                .replace("__ALIGN_OFFSET__", &placement.align_offset.to_string())
                .replace(
                    "__ALIGN_ITEM__",
                    if placement.align_item_with_trigger {
                        "true"
                    } else {
                        "false"
                    },
                );
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<String>().await {
                    match message.as_str() {
                        "ready" if *phase.peek() == Phase::Starting => phase.set(Phase::Open),
                        "outside" => ctx.set_open(false),
                        "escape" | "tab" => ctx.close_and_focus_trigger(),
                        _ => {}
                    }
                }
            });
        } else if !open && matches!(current, Phase::Starting | Phase::Open) {
            phase.set(Phase::Ending);
            let script = EXIT_SCRIPT
                .replace("__POSITIONER__", &ctx.positioner_id())
                .replace("__POPUP__", &ctx.popup_id());
            spawn(async move {
                let mut eval = document::eval(&script);
                let _ = eval.recv::<bool>().await;
                if !ctx.is_open() && *phase.peek() == Phase::Ending {
                    phase.set(Phase::Closed);
                }
            });
        }
    });

    let mut attributes = attributes;
    let class = cn!(
        "cn-select-content cn-select-content-logical cn-menu-target cn-menu-translucent relative isolate z-50 max-h-(--available-height) w-(--anchor-width) origin-(--transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none",
        take_class(&mut attributes).as_deref()
    );
    let phase = phase();

    rsx! {
        div {
            id: ctx.positioner_id(),
            popover: "manual",
            class: "isolate fixed inset-auto z-50 m-0 overflow-visible border-0 bg-transparent p-0 text-inherit",
            div {
                id: ctx.popup_id(),
                "data-slot": "select-content",
                "data-align-trigger": align_item_with_trigger.to_string(),
                "data-open": phase.open(),
                "data-closed": phase.closed(),
                "data-starting-style": phase.starting(),
                "data-ending-style": phase.ending(),
                class: class,
                ..attributes,
                SelectScrollUpButton {}
                div {
                    id: ctx.list_id(),
                    role: "listbox",
                    tabindex: "-1",
                    class: "outline-none",
                    {children}
                }
                SelectScrollDownButton {}
            }
        }
    }
}

#[derive(Clone, Copy)]
struct GroupContext {
    label_id: usize,
}

#[component]
pub fn SelectGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let label_id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    use_context_provider(|| GroupContext { label_id });
    let mut attributes = attributes;
    let class = cn!("cn-select-group", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "select-group",
            role: "group",
            aria_labelledby: "select-group-label-{label_id}",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SelectLabel(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let group = try_use_context::<GroupContext>();
    let mut attributes = attributes;
    let class = cn!("cn-select-label", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            id: group.map(|group| format!("select-group-label-{}", group.label_id)),
            "data-slot": "select-label",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One option. Leave `value` out for the "nothing selected" entry of `items`. `label` is what
/// `SelectValue` shows after selection when `items` does not list the value; it defaults to `value`.
#[component]
pub fn SelectItem(
    value: Option<String>,
    label: Option<String>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut attributes = attributes;
    let class = cn!(
        "cn-select-item relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        take_class(&mut attributes).as_deref()
    );
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let label_text = label
        .or_else(|| value.clone())
        .unwrap_or_default();
    let value = use_prop(value);
    let label = use_prop(label_text);

    use_effect(move || {
        let mut entries = ctx.entries;
        let entry = Entry {
            id,
            value: value(),
            label: label(),
        };
        let mut entries = entries.write();
        entries.retain(|existing| existing.id != id);
        entries.push(entry);
    });
    use_drop(move || {
        let mut entries = ctx.entries;
        entries.write().retain(|entry| entry.id != id);
    });

    let item_value = value();
    let selected = ctx.value() == item_value;

    rsx! {
        div {
            "data-slot": "select-item",
            "data-selected": selected.then_some(""),
            "data-disabled": disabled.then_some(""),
            role: "option",
            aria_selected: selected,
            aria_disabled: disabled.then_some("true"),
            tabindex: "-1",
            class: class,
            onclick: move |_| {
                if !disabled {
                    ctx.select(item_value.clone());
                }
            },
            ..attributes,
            span { class: "cn-select-item-text shrink-0 whitespace-nowrap", {children} }
            if selected {
                span { class: "cn-select-item-indicator",
                    svg {
                        class: "cn-select-item-indicator-icon pointer-events-none",
                        "aria-hidden": "true",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M20 6 9 17l-5-5" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SelectSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-select-separator pointer-events-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "select-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}

/// Shown while the list can scroll up. Hovering it scrolls the list.
#[component]
pub fn SelectScrollUpButton(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-select-scroll-up-button absolute top-0 w-full",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "select-scroll-up-button",
            "aria-hidden": "true",
            hidden: true,
            class: class,
            ..attributes,
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m18 15-6-6-6 6" }
            }
        }
    }
}

/// Shown while the list can scroll down. Hovering it scrolls the list.
#[component]
pub fn SelectScrollDownButton(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-select-scroll-down-button absolute bottom-0 w-full",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "select-scroll-down-button",
            "aria-hidden": "true",
            hidden: true,
            class: class,
            ..attributes,
            svg {
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
