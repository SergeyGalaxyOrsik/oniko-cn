//! Combobox: a text input that filters a popup list of options and selects one of them, or several as chips.
//!
//! The popup lives in a `popover="manual"` element, so it sits in the top layer and is never clipped by
//! an ancestor. A small script positions it from `side`, `align`, `side_offset` and `align_offset`,
//! flips and shifts it at the viewport edges and publishes `--anchor-width`, `--available-height` and
//! `--transform-origin`. Items stay mounted while they are filtered out, so they keep their label
//! registered; the root filters by label, or by the `filter` callback. Focus stays in the input; the
//! highlighted item is marked with `data-highlighted` and announced through `aria-activedescendant`.
//! `ArrowDown` and `ArrowUp` move the highlight, `Enter` picks it, `Escape` closes the popup, and with
//! `multiple` an empty input removes the last chip on `Backspace`. No `dioxus-primitives` crate is used:
//! it is not published on crates.io.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{Button, ButtonSize, ButtonVariant, button_variants};
use super::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupInput, input_group_button_variants,
};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const SESSION_SCRIPT: &str = r#"
const input = document.getElementById("__INPUT__");
const triggerEl = document.getElementById("__TRIGGER__");
const positioner = document.getElementById("__POSITIONER__");
const popup = document.getElementById("__POPUP__");
if (!positioner || !popup) { dioxus.send("ready"); return; }
const anchor = document.getElementById("__ANCHOR__") || (input && !popup.contains(input) ? input : null) || triggerEl;
if (!anchor) { dioxus.send("ready"); return; }
const cfg = { side: "__SIDE__", align: "__ALIGN__", sideOffset: __SIDE_OFFSET__, alignOffset: __ALIGN_OFFSET__, auto: __AUTO__ };
const PAD = 5;
const ac = new AbortController();
const on = (target, type, handler, capture = false) => target.addEventListener(type, handler, { signal: ac.signal, capture });
const options = () => [...popup.querySelectorAll('[role="option"]:not([aria-disabled="true"])')];
if (!positioner.matches(":popover-open")) positioner.showPopover();

const opposite = { top: "bottom", bottom: "top", left: "right", right: "left" };
const clamp = (value, min, max) => Math.max(min, Math.min(value, max));

function place() {
  const a = anchor.getBoundingClientRect();
  const vw = document.documentElement.clientWidth;
  const vh = document.documentElement.clientHeight;
  const rtl = getComputedStyle(popup).direction === "rtl";
  const set = (name, value) => positioner.style.setProperty(name, value);
  set("--anchor-width", a.width + "px");
  set("--anchor-height", a.height + "px");
  positioner.style.left = "0px";
  positioner.style.top = "0px";
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
  positioner.dataset.side = reported; popup.dataset.side = reported;
  positioner.dataset.align = cfg.align; popup.dataset.align = cfg.align;
}

let raf = 0;
const schedule = () => { cancelAnimationFrame(raf); raf = requestAnimationFrame(place); };
on(window, "resize", schedule);
on(window, "scroll", (event) => { if (!popup.contains(event.target)) schedule(); }, true);

let current = null;
function highlight(option, scroll) {
  popup.querySelectorAll("[data-highlighted]").forEach((node) => node.removeAttribute("data-highlighted"));
  current = option;
  if (option) {
    option.setAttribute("data-highlighted", "");
    if (input) input.setAttribute("aria-activedescendant", option.id);
    if (scroll) option.scrollIntoView({ block: "nearest" });
  } else if (input) {
    input.removeAttribute("aria-activedescendant");
  }
}
on(popup, "pointermove", (event) => {
  const option = event.target.closest('[role="option"]:not([aria-disabled="true"])');
  if (option && option !== current) highlight(option, false);
});
on(popup, "pointerleave", () => highlight(null, false));
const observer = new MutationObserver(() => {
  if (current && !current.isConnected) highlight(null, false);
  if (cfg.auto && !current) { const first = options()[0]; if (first) highlight(first, true); }
});
observer.observe(popup, { childList: true, subtree: true });

on(document, "keydown", (event) => {
  const key = event.key;
  if (key === "Escape") {
    event.preventDefault(); event.stopPropagation();
    dioxus.send("escape");
    return;
  }
  const target = event.target;
  if (target !== input && !popup.contains(target)) return;
  if (key === "Tab") { dioxus.send("tab"); return; }
  if (key === "ArrowDown" || key === "ArrowUp") {
    event.preventDefault(); event.stopPropagation();
    const items = options();
    if (!items.length) return;
    const i = items.indexOf(current);
    const last = items.length - 1;
    const next = key === "ArrowDown" ? (i < 0 ? 0 : (i + 1) % items.length) : (i < 0 ? last : (i - 1 + items.length) % items.length);
    highlight(items[next], true);
    return;
  }
  if (key === "Enter" && current && current.isConnected) {
    event.preventDefault(); event.stopPropagation();
    current.click();
  }
}, true);
on(document, "pointerdown", (event) => {
  const target = event.target;
  const group = input ? input.closest('[data-slot="input-group"], [data-slot="combobox-chips"]') : null;
  const inside = popup.contains(target) || anchor.contains(target) || (group && group.contains(target)) || (triggerEl && triggerEl.contains(target));
  if (!inside) dioxus.send("outside");
}, true);

place();
await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
place();
const selected = popup.querySelector('[role="option"][data-selected]');
const first = selected || (cfg.auto ? options()[0] : null);
if (first) highlight(first, true);
if (input && document.activeElement !== input) input.focus({ preventScroll: true });
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
observer.disconnect();
cancelAnimationFrame(raf);
if (input) input.removeAttribute("aria-activedescendant");
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

/// Removes a plain attribute from the spread attributes and returns its text.
fn take_attr(attributes: &mut Vec<Attribute>, name: &str) -> Option<String> {
    let mut found = None;
    attributes.retain(|attr| {
        if attr.name != name || attr.namespace.is_some() {
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

#[derive(Clone, Debug, PartialEq)]
struct Entry {
    id: usize,
    value: String,
    label: String,
    group: Option<usize>,
}

#[derive(Clone, Copy)]
struct ComboboxContext {
    id: usize,
    multiple: Signal<bool>,
    controlled: Signal<Option<String>>,
    internal: Signal<Option<String>>,
    controlled_values: Signal<Option<Vec<String>>>,
    internal_values: Signal<Vec<String>>,
    controlled_input: Signal<Option<String>>,
    internal_input: Signal<String>,
    /// Whether the user has typed since the popup opened. Only then does the text filter the list.
    typed: Signal<bool>,
    controlled_open: Signal<Option<bool>>,
    internal_open: Signal<bool>,
    entries: Signal<Vec<Entry>>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    auto_highlight: Signal<bool>,
    input_id: Signal<String>,
    trigger_id: Signal<String>,
    trigger: Signal<Option<Rc<MountedData>>>,
    input_in_popup: Signal<bool>,
    filter: Option<Callback<(String, String), bool>>,
    on_value_change: Option<EventHandler<Option<String>>>,
    on_values_change: Option<EventHandler<Vec<String>>>,
    on_input_value_change: Option<EventHandler<String>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl ComboboxContext {
    fn value(self) -> Option<String> {
        match (self.controlled)() {
            Some(value) => Some(value),
            None => (self.internal)(),
        }
    }

    fn values(self) -> Vec<String> {
        if (self.multiple)() {
            (self.controlled_values)().unwrap_or_else(|| (self.internal_values)())
        } else {
            self.value().into_iter().collect()
        }
    }

    fn is_selected(self, value: &str) -> bool {
        self.values().iter().any(|selected| selected == value)
    }

    fn label_of(self, value: &str) -> String {
        self.entries
            .read()
            .iter()
            .find(|entry| entry.value == value)
            .map_or_else(|| value.to_string(), |entry| entry.label.clone())
    }

    fn is_open(self) -> bool {
        (self.controlled_open)().unwrap_or_else(|| (self.internal_open)())
    }

    fn set_open(self, open: bool) {
        if open && ((self.disabled)() || (self.read_only)()) {
            return;
        }
        if !open {
            let mut typed = self.typed;
            typed.set(false);
            if *self.multiple.peek() {
                self.set_input_text(String::new());
            }
        }
        let mut internal = self.internal_open;
        if self.controlled_open.peek().is_none() {
            internal.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }

    fn set_input_text(self, text: String) {
        let mut internal = self.internal_input;
        if self.controlled_input.peek().is_none() {
            internal.set(text.clone());
        }
        if let Some(handler) = self.on_input_value_change {
            handler.call(text);
        }
    }

    /// The text shown in the input.
    fn input_text(self) -> String {
        if let Some(text) = (self.controlled_input)() {
            return text;
        }
        if (self.typed)() || (self.multiple)() {
            return (self.internal_input)();
        }
        self.value()
            .map(|value| self.label_of(&value))
            .unwrap_or_else(|| (self.internal_input)())
    }

    fn query(self) -> String {
        if (self.typed)() {
            self.input_text()
        } else {
            String::new()
        }
    }

    fn matches(self, label: &str) -> bool {
        let query = self.query();
        if query.is_empty() {
            return true;
        }
        match self.filter {
            Some(filter) => filter.call((query, label.to_string())),
            None => label.to_lowercase().contains(&query.to_lowercase()),
        }
    }

    fn visible_in(self, group: Option<usize>) -> bool {
        self.entries
            .read()
            .iter()
            .any(|entry| (group.is_none() || entry.group == group) && self.matches(&entry.label))
    }

    fn has_value(self) -> bool {
        !self.values().is_empty() || !self.input_text().is_empty()
    }

    fn popup_id(self) -> String {
        format!("combobox-{}-popup", self.id)
    }

    fn positioner_id(self) -> String {
        format!("combobox-{}-positioner", self.id)
    }

    fn list_id(self) -> String {
        format!("combobox-{}-list", self.id)
    }

    fn focus_input(self) {
        let script = format!(
            "document.getElementById({:?})?.focus()",
            self.input_id.peek().as_str()
        );
        spawn(async move {
            let _ = document::eval(&script).await;
        });
    }

    fn focus_trigger(self) {
        if let Some(trigger) = self.trigger.peek().clone() {
            spawn(async move {
                let _ = trigger.set_focus(true).await;
            });
        }
    }

    fn close_and_restore_focus(self) {
        self.set_open(false);
        if *self.input_in_popup.peek() {
            self.focus_trigger();
        }
    }

    fn set_values(self, values: Vec<String>) {
        let mut internal = self.internal_values;
        if self.controlled_values.peek().is_none() {
            internal.set(values.clone());
        }
        if let Some(handler) = self.on_values_change {
            handler.call(values);
        }
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

    fn select(self, value: String) {
        let mut typed = self.typed;
        typed.set(false);
        if *self.multiple.peek() {
            let mut values = self.values();
            if let Some(position) = values.iter().position(|selected| *selected == value) {
                values.remove(position);
            } else {
                values.push(value);
            }
            self.set_values(values);
            self.set_input_text(String::new());
            self.focus_input();
        } else {
            self.set_input_text(self.label_of(&value));
            self.set_value(Some(value));
            self.close_and_restore_focus();
            if !*self.input_in_popup.peek() {
                self.focus_input();
            }
        }
    }

    fn remove(self, value: &str) {
        let values: Vec<String> = self
            .values()
            .into_iter()
            .filter(|selected| selected != value)
            .collect();
        self.set_values(values);
    }

    fn clear(self) {
        let mut typed = self.typed;
        typed.set(false);
        if *self.multiple.peek() {
            self.set_values(Vec::new());
        } else {
            self.set_value(None);
        }
        self.set_input_text(String::new());
        self.focus_input();
    }

    fn on_input(self, text: String) {
        let mut typed = self.typed;
        typed.set(true);
        if !*self.multiple.peek() && text.is_empty() && self.value().is_some() {
            self.set_value(None);
        }
        self.set_input_text(text);
        self.set_open(true);
    }

    fn on_keydown(self, event: &KeyboardEvent) {
        let open = self.is_open();
        match event.key() {
            Key::ArrowDown | Key::ArrowUp if !open => {
                event.prevent_default();
                self.set_open(true);
            }
            Key::Backspace if *self.multiple.peek() && self.input_text().is_empty() => {
                if let Some(last) = self.values().last().cloned() {
                    self.remove(&last);
                }
            }
            _ => {}
        }
    }
}

/// Owns the selection, the input text and the open state.
///
/// Without `multiple`, `value` and `on_value_change` hold one selection and `default_value` is its
/// initial value. With `multiple`, use `values`, `default_values` and `on_values_change`. Items are
/// matched by the `value` of a `ComboboxItem`.
#[component]
pub fn Combobox(
    #[props(default)] multiple: bool,
    value: Option<String>,
    default_value: Option<String>,
    on_value_change: Option<EventHandler<Option<String>>>,
    values: Option<Vec<String>>,
    #[props(default)] default_values: Vec<String>,
    on_values_change: Option<EventHandler<Vec<String>>>,
    /// Controlled input text.
    input_value: Option<String>,
    default_input_value: Option<String>,
    on_input_value_change: Option<EventHandler<String>>,
    open: Option<bool>,
    #[props(default)] default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    /// Highlight the first item whenever the list changes, so `Enter` picks it.
    #[props(default)]
    auto_highlight: bool,
    /// Decides whether an item with the given label matches the typed text: `(query, label)`.
    /// The default is a case-insensitive substring match.
    filter: Option<Callback<(String, String), bool>>,
    #[props(default)] disabled: bool,
    #[props(default)] read_only: bool,
    #[props(default)] required: bool,
    /// Name of the hidden input that submits the selection with a form.
    name: Option<String>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let multiple_signal = use_prop(multiple);
    let controlled = use_prop(value);
    let internal = use_signal(|| default_value);
    let controlled_values = use_prop(values);
    let internal_values = use_signal(|| default_values);
    let controlled_input = use_prop(input_value);
    let internal_input = use_signal(|| default_input_value.unwrap_or_default());
    let typed = use_signal(|| false);
    let controlled_open = use_prop(open);
    let internal_open = use_signal(|| default_open);
    let entries = use_signal(Vec::new);
    let disabled_signal = use_prop(disabled);
    let read_only_signal = use_prop(read_only);
    let auto_highlight = use_prop(auto_highlight);
    let input_id = use_signal(|| format!("combobox-{id}-input"));
    let trigger_id = use_signal(|| format!("combobox-{id}-trigger"));
    let trigger = use_signal(|| None::<Rc<MountedData>>);
    let input_in_popup = use_signal(|| false);
    let ctx = use_context_provider(|| ComboboxContext {
        id,
        multiple: multiple_signal,
        controlled,
        internal,
        controlled_values,
        internal_values,
        controlled_input,
        internal_input,
        typed,
        controlled_open,
        internal_open,
        entries,
        disabled: disabled_signal,
        read_only: read_only_signal,
        auto_highlight,
        input_id,
        trigger_id,
        trigger,
        input_in_popup,
        filter,
        on_value_change,
        on_values_change,
        on_input_value_change,
        on_open_change,
    });

    rsx! {
        {children}
        if let Some(name) = name {
            for value in ctx.values() {
                input {
                    key: "{value}",
                    r#type: "hidden",
                    name: name.clone(),
                    value: value,
                    disabled: disabled,
                    required: required,
                }
            }
        }
    }
}

/// Renders the selected label (or `render`'s output for the selected values), without a wrapper element.
#[component]
pub fn ComboboxValue(render: Option<Callback<Vec<String>, Element>>) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let values = ctx.values();
    if let Some(render) = render {
        return render.call(values);
    }
    let text = values
        .iter()
        .map(|value| ctx.label_of(value))
        .collect::<Vec<_>>()
        .join(", ");

    rsx! { "{text}" }
}

/// The button that toggles the popup. Plain by default; pass `variant` or `size` to give it the button
/// look. A chevron follows its children.
#[component]
pub fn ComboboxTrigger(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut trigger_id = ctx.trigger_id;
    let mut trigger = ctx.trigger;
    let mut attributes = attributes;
    let caller = cn!(
        "[&_svg:not([class*='size-'])]:size-4",
        take_class(&mut attributes).as_deref()
    );
    let class = if variant.is_some() || size.is_some() {
        button_variants(
            variant.unwrap_or_default(),
            size.unwrap_or_default(),
            Some(caller.as_str()),
        )
    } else {
        caller
    };
    let slot = take_attr(&mut attributes, "data-slot").unwrap_or_else(|| "combobox-trigger".into());
    let id =
        take_attr(&mut attributes, "id").unwrap_or_else(|| format!("combobox-{}-trigger", ctx.id));
    if *trigger_id.peek() != id {
        trigger_id.set(id.clone());
    }
    let open = ctx.is_open();
    let disabled = (ctx.disabled)();

    rsx! {
        button {
            id: id,
            "data-slot": slot,
            "data-popup-open": open.then_some(""),
            "data-pressed": open.then_some(""),
            "data-disabled": disabled.then_some(""),
            r#type: "button",
            tabindex: "-1",
            aria_haspopup: "listbox",
            aria_expanded: open,
            aria_controls: open.then(|| ctx.list_id()),
            disabled: disabled,
            class: class,
            onmounted: move |event| trigger.set(Some(event.data())),
            onclick: move |_| {
                ctx.set_open(!open);
                if !*ctx.input_in_popup.peek() {
                    ctx.focus_input();
                }
            },
            ..attributes,
            {children}
            svg {
                class: "text-muted-foreground size-4 pointer-events-none",
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

/// Empties the selection and the input. Shown only while there is something to clear.
#[component]
fn ComboboxClear(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    if !ctx.has_value() {
        return rsx! {};
    }
    let mut attributes = attributes;
    let class = cn!("", take_class(&mut attributes).as_deref());
    attributes.push(Attribute::new("class", class, None, false));
    attributes.push(Attribute::new("data-slot", "combobox-clear", None, false));
    attributes.push(Attribute::new("aria-label", "Clear", None, false));
    attributes.push(Attribute::new("tabindex", "-1", None, false));

    rsx! {
        InputGroupButton {
            variant: ButtonVariant::Ghost,
            size: InputGroupButtonSize::IconXs,
            attributes,
            onclick: move |_| ctx.clear(),
            svg {
                class: "pointer-events-none",
                "aria-hidden": "true",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M18 6 6 18" }
                path { d: "m6 6 12 12" }
            }
        }
    }
}

/// Marks that the input is rendered inside the popup, so the popup is anchored to the trigger.
#[derive(Clone, Copy)]
struct InPopup;

/// The input with its trigger and clear buttons, inside an input group. `class` goes to the group,
/// other attributes to the input. `children` render after the buttons, for example an addon.
#[component]
pub fn ComboboxInput(
    #[props(default)] disabled: bool,
    #[props(default = true)] show_trigger: bool,
    #[props(default)] show_clear: bool,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut input_id = ctx.input_id;
    let mut input_in_popup = ctx.input_in_popup;
    let in_popup = try_use_context::<InPopup>().is_some();
    if in_popup && !*input_in_popup.peek() {
        input_in_popup.set(true);
    }
    let mut attributes = attributes;
    let group_class = cn!("w-auto", take_class(&mut attributes).as_deref());
    let id =
        take_attr(&mut attributes, "id").unwrap_or_else(|| format!("combobox-{}-input", ctx.id));
    if *input_id.peek() != id {
        input_id.set(id.clone());
    }
    let open = ctx.is_open();
    let disabled = disabled || (ctx.disabled)();
    attributes.push(Attribute::new("id", id, None, false));
    attributes.push(Attribute::new("role", "combobox", None, false));
    attributes.push(Attribute::new("autocomplete", "off", None, false));
    attributes.push(Attribute::new("aria-autocomplete", "list", None, false));
    attributes.push(Attribute::new(
        "aria-expanded",
        open.to_string(),
        None,
        false,
    ));
    attributes.push(Attribute::new("aria-haspopup", "listbox", None, false));
    attributes.push(Attribute::new(
        "aria-controls",
        open.then(|| ctx.list_id()),
        None,
        false,
    ));
    attributes.push(Attribute::new(
        "data-popup-open",
        open.then_some(""),
        None,
        false,
    ));
    attributes.push(Attribute::new("value", ctx.input_text(), None, true));
    attributes.push(Attribute::new(
        "disabled",
        disabled.then_some(""),
        None,
        false,
    ));
    attributes.push(Attribute::new(
        "readonly",
        (ctx.read_only)().then_some(""),
        None,
        false,
    ));
    let trigger_class = input_group_button_variants(
        ButtonVariant::Ghost,
        InputGroupButtonSize::IconXs,
        Some("group-has-data-[slot=combobox-clear]/input-group:hidden data-pressed:bg-transparent"),
    );

    rsx! {
        InputGroup { class: group_class,
            InputGroupInput {
                attributes,
                oninput: move |event: FormEvent| ctx.on_input(event.value()),
                onclick: move |_| {
                    if !ctx.is_open() {
                        ctx.set_open(true);
                    }
                },
                onkeydown: move |event: KeyboardEvent| ctx.on_keydown(&event),
            }
            InputGroupAddon { align: InputGroupAlign::InlineEnd,
                if show_trigger {
                    ComboboxTrigger {
                        "data-slot": "input-group-button",
                        class: trigger_class,
                        disabled,
                    }
                }
                if show_clear {
                    ComboboxClear { disabled }
                }
            }
            {children}
        }
    }
}

/// Which side of the anchor the popup opens on. The inline sides follow the text direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComboboxSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
    InlineStart,
    InlineEnd,
}

impl ComboboxSide {
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
pub enum ComboboxAlign {
    #[default]
    Start,
    Center,
    End,
}

impl ComboboxAlign {
    fn name(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

/// An element the popup is positioned against instead of the input. Create it with
/// `use_combobox_anchor` and pass it to `ComboboxChips` and to `ComboboxContent`.
#[derive(Clone, Debug, PartialEq)]
pub struct ComboboxAnchor {
    id: String,
}

pub fn use_combobox_anchor() -> ComboboxAnchor {
    let id = use_hook(|| {
        format!(
            "combobox-anchor-{}",
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        )
    });
    ComboboxAnchor { id }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Placement {
    side: ComboboxSide,
    side_offset: f64,
    align: ComboboxAlign,
    align_offset: f64,
}

/// The popup under the anchor. It stays mounted while closed so its items keep their labels registered.
#[component]
pub fn ComboboxContent(
    #[props(default)] side: ComboboxSide,
    #[props(default = 6.0)] side_offset: f64,
    #[props(default)] align: ComboboxAlign,
    #[props(default = 0.0)] align_offset: f64,
    anchor: Option<ComboboxAnchor>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    use_context_provider(|| InPopup);
    let mut phase = use_signal(|| Phase::Closed);
    let placement = use_prop(Placement {
        side,
        side_offset,
        align,
        align_offset,
    });
    let anchor_id = use_prop(anchor.as_ref().map(|anchor| anchor.id.clone()));

    use_effect(move || {
        let open = ctx.is_open();
        let current = *phase.peek();
        if open && matches!(current, Phase::Closed | Phase::Ending) {
            phase.set(Phase::Starting);
            let placement = *placement.peek();
            let script = SESSION_SCRIPT
                .replace("__INPUT__", &ctx.input_id.peek())
                .replace("__TRIGGER__", &ctx.trigger_id.peek())
                .replace("__ANCHOR__", &anchor_id.peek().clone().unwrap_or_default())
                .replace("__POSITIONER__", &ctx.positioner_id())
                .replace("__POPUP__", &ctx.popup_id())
                .replace("__SIDE__", placement.side.name())
                .replace("__ALIGN__", placement.align.name())
                .replace("__SIDE_OFFSET__", &placement.side_offset.to_string())
                .replace("__ALIGN_OFFSET__", &placement.align_offset.to_string())
                .replace(
                    "__AUTO__",
                    if *ctx.auto_highlight.peek() {
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
                        "outside" | "tab" => ctx.set_open(false),
                        "escape" => ctx.close_and_restore_focus(),
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
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 *:data-[slot=input-group]:bg-input/30 *:data-[slot=input-group]:border-input/30 max-h-72 min-w-36 overflow-hidden rounded-md shadow-md ring-1 duration-100 *:data-[slot=input-group]:m-1 *:data-[slot=input-group]:mb-0 *:data-[slot=input-group]:h-8 *:data-[slot=input-group]:shadow-none data-[side=inline-start]:slide-in-from-right-2 data-[side=inline-end]:slide-in-from-left-2 animate-none! relative bg-popover/70 before:pointer-events-none before:absolute before:inset-0 before:-z-1 before:rounded-[inherit] before:backdrop-blur-2xl before:backdrop-saturate-150 **:data-[slot$=-item]:focus:bg-foreground/10 **:data-[slot$=-item]:data-highlighted:bg-foreground/10 **:data-[slot$=-separator]:bg-foreground/5 **:data-[slot$=-trigger]:focus:bg-foreground/10 **:data-[slot$=-trigger]:aria-expanded:bg-foreground/10! **:data-[variant=destructive]:focus:bg-foreground/10! **:data-[variant=destructive]:text-accent-foreground! **:data-[variant=destructive]:**:text-accent-foreground! group/combobox-content max-h-(--available-height) w-(--anchor-width) max-w-(--available-width) min-w-[calc(var(--anchor-width)+--spacing(7))] origin-(--transform-origin) data-[chips=true]:min-w-(--anchor-width)",
        take_class(&mut attributes).as_deref()
    );
    let phase = phase();
    let empty = !ctx.visible_in(None);

    rsx! {
        div {
            id: ctx.positioner_id(),
            popover: "manual",
            class: "isolate fixed inset-auto z-50 m-0 overflow-visible border-0 bg-transparent p-0 text-inherit",
            div {
                id: ctx.popup_id(),
                "data-slot": "combobox-content",
                "data-chips": anchor.is_some().to_string(),
                "data-empty": empty.then_some(""),
                "data-open": phase.open(),
                "data-closed": phase.closed(),
                "data-starting-style": phase.starting(),
                "data-ending-style": phase.ending(),
                class: class,
                ..attributes,
                {children}
            }
        }
    }
}

#[component]
pub fn ComboboxList(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut attributes = attributes;
    let class = cn!(
        "no-scrollbar max-h-[min(calc(--spacing(72)---spacing(9)),calc(var(--available-height)---spacing(9)))] scroll-py-1 overflow-y-auto p-1 data-empty:p-0 overscroll-contain",
        take_class(&mut attributes).as_deref()
    );
    let empty = !ctx.visible_in(None);

    rsx! {
        div {
            id: ctx.list_id(),
            "data-slot": "combobox-list",
            "data-empty": empty.then_some(""),
            role: "listbox",
            aria_multiselectable: (ctx.multiple)().then_some("true"),
            class: class,
            // Keeps focus in the input while the pointer works the list.
            onmousedown: move |event| event.prevent_default(),
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy)]
struct GroupContext {
    id: usize,
}

/// One option. `value` identifies it; `label` is what is matched against the typed text and shown in
/// the input after selection, and defaults to `value`. Pass `label` whenever `children` is not plain text.
#[component]
pub fn ComboboxItem(
    value: String,
    label: Option<String>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let group = try_use_context::<GroupContext>().map(|group| group.id);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let label_text = label.unwrap_or_else(|| value.clone());
    let value = use_prop(value);
    let label = use_prop(label_text);

    use_effect(move || {
        let mut entries = ctx.entries;
        let entry = Entry {
            id,
            value: value(),
            label: label(),
            group,
        };
        let mut entries = entries.write();
        entries.retain(|existing| existing.id != id);
        entries.push(entry);
    });
    use_drop(move || {
        let mut entries = ctx.entries;
        entries.write().retain(|entry| entry.id != id);
    });

    if !ctx.matches(&label()) {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "data-highlighted:bg-accent data-highlighted:text-accent-foreground not-data-[variant=destructive]:data-highlighted:**:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm [&_svg:not([class*='size-'])]:size-4 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        take_class(&mut attributes).as_deref()
    );
    let item_value = value();
    let selected = ctx.is_selected(&item_value);

    rsx! {
        div {
            id: "combobox-{ctx.id}-item-{id}",
            "data-slot": "combobox-item",
            "data-selected": selected.then_some(""),
            "data-disabled": disabled.then_some(""),
            role: "option",
            aria_selected: selected,
            aria_disabled: disabled.then_some("true"),
            class: class,
            onclick: move |_| {
                if !disabled {
                    ctx.select(item_value.clone());
                }
            },
            ..attributes,
            {children}
            if selected {
                span { class: "pointer-events-none absolute right-2 flex size-4 items-center justify-center",
                    svg {
                        class: "pointer-events-none",
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

/// Groups items under a `ComboboxLabel`. It hides itself while none of its items match.
#[component]
pub fn ComboboxGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    use_context_provider(|| GroupContext { id });
    let mut attributes = attributes;
    let class = cn!("", take_class(&mut attributes).as_deref());
    let visible = ctx.visible_in(Some(id));

    rsx! {
        div {
            "data-slot": "combobox-group",
            role: "group",
            hidden: !visible,
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ComboboxLabel(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground px-2 py-1.5 text-xs",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "combobox-label",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Wraps the items of one group. Renders no element, as in Base UI.
#[component]
pub fn ComboboxCollection(children: Element) -> Element {
    rsx! { {children} }
}

/// Shown while no item matches the typed text.
#[component]
pub fn ComboboxEmpty(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground hidden w-full justify-center py-2 text-center text-sm group-data-empty/combobox-content:flex",
        take_class(&mut attributes).as_deref()
    );
    let empty = !ctx.visible_in(None);

    rsx! {
        div {
            "data-slot": "combobox-empty",
            aria_live: "polite",
            class: class,
            ..attributes,
            if empty {
                {children}
            }
        }
    }
}

#[component]
pub fn ComboboxSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-border -mx-1 my-1 h-px",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "combobox-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}

/// The box that holds the chips and their input, for `multiple`. Pass the `use_combobox_anchor` handle
/// as `anchor` to make the popup follow its width.
#[component]
pub fn ComboboxChips(
    anchor: Option<ComboboxAnchor>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut attributes = attributes;
    let class = cn!(
        "dark:bg-input/30 border-input focus-within:border-ring focus-within:ring-ring/50 has-aria-invalid:ring-destructive/20 dark:has-aria-invalid:ring-destructive/40 has-aria-invalid:border-destructive dark:has-aria-invalid:border-destructive/50 flex min-h-9 flex-wrap items-center gap-1.5 rounded-md border bg-transparent bg-clip-padding px-2.5 py-1.5 text-sm shadow-xs transition-[color,box-shadow] focus-within:ring-3 has-aria-invalid:ring-3 has-data-[slot=combobox-chip]:px-1.5",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            id: anchor.map(|anchor| anchor.id),
            "data-slot": "combobox-chips",
            role: "toolbar",
            class: class,
            onclick: move |_| ctx.focus_input(),
            ..attributes,
            {children}
        }
    }
}

/// One selected value in `ComboboxChips`, with a remove button unless `show_remove` is false.
#[component]
pub fn ComboboxChip(
    value: String,
    #[props(default = true)] show_remove: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut attributes = attributes;
    let class = cn!(
        "bg-muted text-foreground flex h-[calc(--spacing(5.5))] w-fit items-center justify-center gap-1 rounded-sm px-1.5 text-xs font-medium whitespace-nowrap has-data-[slot=combobox-chip-remove]:pr-0 has-disabled:pointer-events-none has-disabled:cursor-not-allowed has-disabled:opacity-50",
        take_class(&mut attributes).as_deref()
    );
    let remove_value = value.clone();
    let disabled = (ctx.disabled)();

    rsx! {
        div {
            "data-slot": "combobox-chip",
            class: class,
            ..attributes,
            {children}
            if show_remove {
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::IconXs,
                    disabled,
                    attributes: vec![
                        Attribute::new("data-slot", "combobox-chip-remove", None, false),
                        Attribute::new("class", "-ml-1 opacity-50 hover:opacity-100", None, false),
                        Attribute::new("aria-label", "Remove", None, false),
                        Attribute::new("tabindex", "-1", None, false),
                    ],
                    onclick: move |event: MouseEvent| {
                        event.stop_propagation();
                        ctx.remove(&remove_value);
                        ctx.focus_input();
                    },
                    svg {
                        class: "pointer-events-none",
                        "aria-hidden": "true",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M18 6 6 18" }
                        path { d: "m6 6 12 12" }
                    }
                }
            }
        }
    }
}

/// The text input inside `ComboboxChips`.
#[component]
pub fn ComboboxChipsInput(
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut input_id = ctx.input_id;
    let mut attributes = attributes;
    let class = cn!(
        "min-w-16 flex-1 outline-none",
        take_class(&mut attributes).as_deref()
    );
    let id =
        take_attr(&mut attributes, "id").unwrap_or_else(|| format!("combobox-{}-input", ctx.id));
    if *input_id.peek() != id {
        input_id.set(id.clone());
    }
    let open = ctx.is_open();

    rsx! {
        input {
            id: id,
            "data-slot": "combobox-chip-input",
            "data-popup-open": open.then_some(""),
            role: "combobox",
            autocomplete: "off",
            aria_autocomplete: "list",
            aria_expanded: open,
            aria_haspopup: "listbox",
            aria_controls: open.then(|| ctx.list_id()),
            value: ctx.input_text(),
            disabled: (ctx.disabled)(),
            readonly: (ctx.read_only)(),
            class: class,
            oninput: move |event| ctx.on_input(event.value()),
            onclick: move |_| {
                if !ctx.is_open() {
                    ctx.set_open(true);
                }
            },
            onkeydown: move |event| ctx.on_keydown(&event),
            ..attributes,
        }
    }
}
