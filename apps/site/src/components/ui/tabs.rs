//! Tabs: a set of layered sections of content, where one section is shown at a time.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Moves focus between the enabled tabs of a list without selecting them, as Base UI does.
/// `LIST_ID`, `KEY` and `LOOP` are replaced before the script runs.
const MOVE_FOCUS_SCRIPT: &str = r#"(() => {
  const list = document.getElementById("LIST_ID");
  if (!list) return;
  const tabs = Array.from(list.querySelectorAll('[role="tab"]')).filter(
    (tab) => tab.closest('[role="tablist"]') === list && tab.getAttribute("aria-disabled") !== "true"
  );
  if (tabs.length === 0) return;
  const rtl = getComputedStyle(list).direction === "rtl";
  const from = tabs.indexOf(document.activeElement);
  const key = "KEY";
  let next;
  if (key === "first") next = 0;
  else if (key === "last") next = tabs.length - 1;
  else {
    const step = key === "next" ? 1 : key === "prev" ? -1 : key === "right" ? (rtl ? -1 : 1) : (rtl ? 1 : -1);
    next = from + step;
    if (next < 0 || next >= tabs.length) next = (next + tabs.length) % tabs.length;
  }
  tabs[next].focus();
})();"#;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl TabsOrientation {
    pub fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    #[default]
    Default,
    Line,
}

impl TabsVariant {
    pub fn name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Line => "line",
        }
    }
}

#[derive(Clone, Copy)]
struct TabsContext {
    id: usize,
    /// Set when the caller controls the selection through `value`.
    controlled: Signal<Option<String>>,
    /// Holds the selection when the caller does not control it.
    internal: Signal<String>,
    orientation: Signal<TabsOrientation>,
    on_value_change: Option<EventHandler<String>>,
}

impl TabsContext {
    fn current(&self) -> String {
        match &*self.controlled.read() {
            Some(value) => value.clone(),
            None => self.internal.read().clone(),
        }
    }

    fn select(mut self, next: &str) {
        let current = match &*self.controlled.peek() {
            Some(value) => value.clone(),
            None => self.internal.peek().clone(),
        };
        if current == next {
            return;
        }
        if self.controlled.peek().is_none() {
            self.internal.set(next.to_string());
        }
        if let Some(handler) = self.on_value_change {
            handler.call(next.to_string());
        }
    }
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// Stable, DOM-safe key for a tab value, used in element ids.
fn tab_key(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn trigger_id(root_id: usize, value: &str) -> String {
    format!("tabs-{root_id}-trigger-{}", tab_key(value))
}

fn panel_id(root_id: usize, value: &str) -> String {
    format!("tabs-{root_id}-content-{}", tab_key(value))
}

fn list_id(root_id: usize) -> String {
    format!("tabs-{root_id}-list")
}

/// Classes of the tab list for `variant`, for building your own list.
pub fn tabs_list_variants(variant: TabsVariant, class: Option<&str>) -> String {
    let look = match variant {
        TabsVariant::Default => "bg-muted",
        TabsVariant::Line => "gap-1 bg-transparent",
    };
    cn!(
        "rounded-lg p-[3px] group-data-horizontal/tabs:h-8 data-[variant=line]:rounded-none group/tabs-list inline-flex w-fit items-center justify-center text-muted-foreground group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col",
        look,
        class
    )
}

#[component]
pub fn Tabs(
    /// Value of the selected tab. Set it to control the selection; leave it unset to let the tabs manage it.
    value: Option<String>,
    /// Value of the tab selected at first when `value` is not set.
    #[props(default)]
    default_value: String,
    #[props(default)] orientation: TabsOrientation,
    /// Called with the value of a tab the user selects.
    on_value_change: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 group/tabs flex data-horizontal:flex-col",
        take_class(&mut attributes)
    );
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let controlled = use_prop(value);
    let orientation = use_prop(orientation);
    let internal = use_signal(|| default_value);
    use_context_provider(|| TabsContext {
        id,
        controlled,
        internal,
        orientation,
        on_value_change,
    });

    rsx! {
        div {
            "data-slot": "tabs",
            "data-orientation": orientation().name(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TabsList(
    #[props(default)] variant: TabsVariant,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = tabs_list_variants(variant, take_class(&mut attributes).as_deref());
    let root = use_context::<TabsContext>();
    let orientation = (root.orientation)().name();

    rsx! {
        div {
            "data-slot": "tabs-list",
            "data-variant": variant.name(),
            "data-orientation": orientation,
            id: list_id(root.id),
            role: "tablist",
            aria_orientation: orientation,
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TabsTrigger(
    /// Identifies the tab. It must match the `value` of its `TabsContent`.
    value: String,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-xs font-medium group-data-vertical/tabs:py-[calc(--spacing(1.25))] [&_svg:not([class*='size-'])]:size-3.5 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0",
        "group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent",
        "data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground",
        "after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100",
        take_class(&mut attributes)
    );
    let root = use_context::<TabsContext>();
    let active = root.current() == value;
    let orientation = (root.orientation)();
    let horizontal = orientation == TabsOrientation::Horizontal;
    let tab_id = trigger_id(root.id, &value);
    let controls = panel_id(root.id, &value);
    let list = list_id(root.id);
    let select_value = value.clone();

    rsx! {
        button {
            "data-slot": "tabs-trigger",
            "data-active": active.then_some(""),
            "data-disabled": disabled.then_some(""),
            "data-orientation": orientation.name(),
            r#type: "button",
            role: "tab",
            id: tab_id,
            aria_selected: active,
            aria_controls: controls,
            aria_disabled: disabled.then_some("true"),
            tabindex: if active { "0" } else { "-1" },
            class: class,
            onclick: move |event| {
                if disabled {
                    return;
                }
                root.select(&select_value);
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            onkeydown: move |event: KeyboardEvent| {
                let key = match event.key() {
                    Key::ArrowRight if horizontal => "right",
                    Key::ArrowLeft if horizontal => "left",
                    Key::ArrowDown if !horizontal => "next",
                    Key::ArrowUp if !horizontal => "prev",
                    Key::Home => "first",
                    Key::End => "last",
                    _ => "",
                };
                if !key.is_empty() {
                    event.prevent_default();
                    let js = MOVE_FOCUS_SCRIPT.replace("LIST_ID", &list).replace("KEY", key);
                    let _ = document::eval(&js);
                }
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TabsContent(
    /// Identifies the tab this panel belongs to.
    value: String,
    /// Keep the panel in the DOM, hidden, while its tab is not selected.
    #[props(default)]
    keep_mounted: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-xs/relaxed flex-1 outline-none",
        take_class(&mut attributes)
    );
    let root = use_context::<TabsContext>();
    let active = root.current() == value;
    if !active && !keep_mounted {
        return rsx! {};
    }

    rsx! {
        div {
            "data-slot": "tabs-content",
            "data-orientation": (root.orientation)().name(),
            "data-hidden": (!active).then_some(""),
            role: "tabpanel",
            id: panel_id(root.id, &value),
            aria_labelledby: trigger_id(root.id, &value),
            tabindex: if active { "0" } else { "-1" },
            hidden: (!active).then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}
