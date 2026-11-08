//! Accordion: a stack of headings that each reveal a section of content.
//!
//! Built on the panel animation of `Collapsible`: the panel carries `--accordion-panel-height`,
//! `data-starting-style` and `data-ending-style`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::collapsible::{use_before_match, use_panel_transition, use_prop};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Moves focus between the triggers of one accordion. `ROOT` and `KEY` are replaced before it runs.
const MOVE_FOCUS_SCRIPT: &str = r#"(() => {
  const root = document.getElementById("__ROOT__");
  if (!root) return;
  const triggers = Array.from(root.querySelectorAll('[data-slot="accordion-trigger"]')).filter(
    (t) => t.closest('[data-slot="accordion"]') === root && t.getAttribute("aria-disabled") !== "true"
  );
  if (triggers.length === 0) return;
  const rtl = getComputedStyle(root).direction === "rtl";
  const loop = __LOOP__;
  const from = triggers.indexOf(document.activeElement);
  const key = "__KEY__";
  let step = 0;
  if (key === "next") step = 1;
  else if (key === "prev") step = -1;
  else if (key === "right") step = rtl ? -1 : 1;
  else if (key === "left") step = rtl ? 1 : -1;
  let next;
  if (key === "first") next = 0;
  else if (key === "last") next = triggers.length - 1;
  else {
    next = from + step;
    if (next < 0 || next >= triggers.length) {
      if (!loop) return;
      next = (next + triggers.length) % triggers.length;
    }
  }
  triggers[next].focus();
})();"#;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl AccordionOrientation {
    pub fn name(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy)]
struct AccordionContext {
    id: usize,
    internal: Signal<Vec<String>>,
    controlled: Signal<Option<Vec<String>>>,
    multiple: Signal<bool>,
    disabled: Signal<bool>,
    orientation: Signal<AccordionOrientation>,
    loop_focus: Signal<bool>,
    keep_mounted: Signal<bool>,
    hidden_until_found: Signal<bool>,
    on_value_change: Option<EventHandler<Vec<String>>>,
}

impl AccordionContext {
    fn values(self) -> Vec<String> {
        match (self.controlled)() {
            Some(value) => value,
            None => (self.internal)(),
        }
    }

    fn is_open(self, value: &str) -> bool {
        self.values().iter().any(|v| v == value)
    }

    fn set_open(mut self, value: &str, open: bool) {
        let mut next = self.values();
        let present = next.iter().any(|v| v == value);
        if present == open {
            return;
        }
        if open {
            if !*self.multiple.peek() {
                next.clear();
            }
            next.push(value.to_string());
        } else {
            next.retain(|v| v != value);
        }
        if self.controlled.peek().is_none() {
            self.internal.set(next.clone());
        }
        if let Some(handler) = self.on_value_change {
            handler.call(next);
        }
    }
}

#[derive(Clone, Copy)]
struct ItemContext {
    value: Signal<String>,
    disabled: Signal<bool>,
    id: usize,
}

fn state(open: bool) -> &'static str {
    if open { "open" } else { "closed" }
}

#[component]
pub fn Accordion(
    /// Values of the open items. Set it to control the accordion.
    value: Option<Vec<String>>,
    /// Values of the items that start open.
    #[props(default)]
    default_value: Vec<String>,
    /// Allow several items to be open at once.
    #[props(default)]
    multiple: bool,
    #[props(default)] disabled: bool,
    #[props(default)] orientation: AccordionOrientation,
    /// Wrap keyboard focus from the last trigger to the first and back.
    #[props(default = true)]
    loop_focus: bool,
    #[props(default)] keep_mounted: bool,
    #[props(default)] hidden_until_found: bool,
    on_value_change: Option<EventHandler<Vec<String>>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "overflow-hidden rounded-md border flex w-full flex-col",
        take_class(&mut attributes)
    );
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_value);
    let controlled = use_prop(value);
    let multiple = use_prop(multiple);
    let disabled_signal = use_prop(disabled);
    let orientation_signal = use_prop(orientation);
    let loop_focus = use_prop(loop_focus);
    let keep_mounted = use_prop(keep_mounted);
    let hidden_until_found = use_prop(hidden_until_found);
    use_context_provider(|| AccordionContext {
        id,
        internal,
        controlled,
        multiple,
        disabled: disabled_signal,
        orientation: orientation_signal,
        loop_focus,
        keep_mounted,
        hidden_until_found,
        on_value_change,
    });

    rsx! {
        div {
            "data-slot": "accordion",
            id: "accordion-{id}",
            role: "region",
            "data-orientation": orientation.name(),
            "data-disabled": disabled.then_some(""),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AccordionItem(
    /// Identifies the item in the accordion's value. Defaults to a generated one.
    value: Option<String>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "data-open:bg-muted/50 not-last:border-b",
        take_class(&mut attributes)
    );
    let root = use_context::<AccordionContext>();
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let value = use_prop(value.unwrap_or_else(|| format!("accordion-item-{id}")));
    let disabled = use_prop(disabled || *root.disabled.read());
    use_context_provider(|| ItemContext {
        value,
        disabled,
        id,
    });

    let open = root.is_open(&value.read());
    rsx! {
        div {
            "data-slot": "accordion-item",
            "data-open": open.then_some(""),
            "data-disabled": disabled().then_some(""),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AccordionTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "**:data-[slot=accordion-trigger-icon]:text-muted-foreground gap-6 p-2 text-left text-xs/relaxed font-medium hover:underline **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none aria-disabled:pointer-events-none aria-disabled:opacity-50",
        take_class(&mut attributes)
    );
    let root = use_context::<AccordionContext>();
    let item = use_context::<ItemContext>();
    let value = item.value.read().clone();
    let open = root.is_open(&value);
    let disabled = (item.disabled)();
    let id = item.id;
    let horizontal = (root.orientation)() == AccordionOrientation::Horizontal;
    let loop_focus = *root.loop_focus.peek();
    let root_id = format!("accordion-{}", root.id);

    rsx! {
        h3 { class: "flex",
            button {
                "data-slot": "accordion-trigger",
                "data-panel-open": open.then_some(""),
                "data-disabled": disabled.then_some(""),
                r#type: "button",
                id: "accordion-trigger-{id}",
                aria_controls: "accordion-panel-{id}",
                aria_expanded: open,
                aria_disabled: disabled.then_some("true"),
                class: class,
                onclick: move |_| {
                    if !disabled {
                        root.set_open(&value, !open);
                    }
                },
                onkeydown: move |event: KeyboardEvent| {
                    let key = match event.key() {
                        Key::ArrowDown if !horizontal => "next",
                        Key::ArrowUp if !horizontal => "prev",
                        Key::ArrowRight if horizontal => "right",
                        Key::ArrowLeft if horizontal => "left",
                        Key::Home => "first",
                        Key::End => "last",
                        _ => return,
                    };
                    event.prevent_default();
                    let js = MOVE_FOCUS_SCRIPT
                        .replace("__ROOT__", &root_id)
                        .replace("__LOOP__", if loop_focus { "true" } else { "false" })
                        .replace("__KEY__", key);
                    let _ = document::eval(&js);
                },
                ..attributes,
                {children}
                ChevronDown {}
                ChevronUp {}
            }
        }
    }
}

#[component]
pub fn AccordionContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let inner_class = cn!(
        "pt-0 pb-4 h-(--accordion-panel-height) data-ending-style:h-0 data-starting-style:h-0 [&_a]:underline [&_a]:underline-offset-3 [&_a]:hover:text-foreground [&_p:not(:last-child)]:mb-4",
        take_class(&mut attributes)
    );
    let root = use_context::<AccordionContext>();
    let item = use_context::<ItemContext>();
    let value = item.value.read().clone();
    let open = root.is_open(&value);
    let dom_id = format!("accordion-panel-{}", item.id);
    let transition = use_panel_transition(dom_id.clone(), "--accordion-panel-height", open);
    let hidden_until_found = *root.hidden_until_found.read();
    let keep_mounted = *root.keep_mounted.read() || hidden_until_found;
    {
        let value = value.clone();
        use_before_match(
            dom_id.clone(),
            hidden_until_found,
            EventHandler::new(move |_| root.set_open(&value, true)),
        );
    }

    let mounted = transition.mounted();
    if !mounted && !keep_mounted {
        return rsx! {};
    }
    let shown = transition.shown();
    let hidden = if mounted {
        None
    } else if hidden_until_found {
        Some("until-found")
    } else {
        Some("true")
    };
    let height = transition.height_css();

    rsx! {
        div {
            "data-slot": "accordion-content",
            "data-open": shown.then_some(""),
            "data-closed": (!shown).then_some(""),
            "data-starting-style": transition.starting().then_some(""),
            "data-ending-style": transition.ending().then_some(""),
            "data-orientation": (root.orientation)().name(),
            role: "region",
            id: dom_id,
            aria_labelledby: "accordion-trigger-{item.id}",
            hidden: hidden,
            style: "--accordion-panel-height: {height}",
            class: "data-open:animate-accordion-down data-closed:animate-accordion-up px-2 text-xs/relaxed overflow-hidden",
            ..attributes,
            div { class: inner_class, {children} }
        }
    }
}

#[component]
fn ChevronDown() -> Element {
    rsx! {
        svg {
            "data-slot": "accordion-trigger-icon",
            class: "pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden",
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

#[component]
fn ChevronUp() -> Element {
    rsx! {
        svg {
            "data-slot": "accordion-trigger-icon",
            class: "pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline",
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
