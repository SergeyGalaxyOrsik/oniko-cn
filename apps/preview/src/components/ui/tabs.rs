//! Tabs: a set of layered sections of content, where one section is shown at a time.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Moves focus to the next, previous, first or last enabled tab in a list, then activates it.
/// `LIST_ID` and `STEP` are replaced before the script runs.
const MOVE_FOCUS_SCRIPT: &str = r#"(() => {
  const list = document.getElementById("LIST_ID");
  if (!list) return;
  const tabs = Array.from(list.querySelectorAll('[role="tab"]')).filter((tab) => !tab.disabled);
  if (tabs.length === 0) return;
  const from = tabs.indexOf(document.activeElement);
  const step = "STEP";
  const next = step === "first" ? 0
    : step === "last" ? tabs.length - 1
    : step === "next" ? (from + 1) % tabs.length
    : (from - 1 + tabs.length) % tabs.length;
  tabs[next].focus();
  tabs[next].click();
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
        "gap-2 group/tabs flex data-horizontal:flex-col data-vertical:flex-row",
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
    let class = cn!(
        "rounded-lg p-[3px] group-data-horizontal/tabs:h-9 data-[variant=line]:rounded-none group/tabs-list inline-flex w-fit items-center justify-center group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col",
        take_class(&mut attributes)
    );
    let root = use_context::<TabsContext>();
    let orientation = (root.orientation)().name();

    rsx! {
        div {
            "data-slot": "tabs-list",
            "data-variant": variant.name(),
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
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-1.5 rounded-md border border-transparent px-2 py-1 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 relative inline-flex flex-1 items-center justify-center whitespace-nowrap outline-none data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm focus-visible:outline-2 focus-visible:outline-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        take_class(&mut attributes)
    );
    let root = use_context::<TabsContext>();
    let active = root.current() == value;
    let horizontal = (root.orientation)() == TabsOrientation::Horizontal;
    let state = if active { "active" } else { "inactive" };
    let tab_id = trigger_id(root.id, &value);
    let controls = panel_id(root.id, &value);
    let list = list_id(root.id);
    let select_value = value.clone();

    rsx! {
        button {
            "data-slot": "tabs-trigger",
            "data-state": state,
            "data-selected": active.then_some("true"),
            r#type: "button",
            role: "tab",
            id: tab_id,
            aria_selected: active,
            aria_controls: controls,
            tabindex: if active { "0" } else { "-1" },
            disabled: disabled,
            class: class,
            onclick: move |_| {
                if !disabled {
                    root.select(&select_value);
                }
            },
            onkeydown: move |event: KeyboardEvent| {
                let step = match event.key() {
                    Key::ArrowRight if horizontal => "next",
                    Key::ArrowLeft if horizontal => "prev",
                    Key::ArrowDown if !horizontal => "next",
                    Key::ArrowUp if !horizontal => "prev",
                    Key::Home => "first",
                    Key::End => "last",
                    _ => "",
                };
                if !step.is_empty() {
                    event.prevent_default();
                    let script = MOVE_FOCUS_SCRIPT
                        .replace("LIST_ID", &list)
                        .replace("STEP", step);
                    let _ = document::eval(&script);
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
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("text-sm flex-1 outline-none", take_class(&mut attributes));
    let root = use_context::<TabsContext>();
    let active = root.current() == value;
    let state = if active { "active" } else { "inactive" };

    rsx! {
        div {
            "data-slot": "tabs-content",
            "data-state": state,
            role: "tabpanel",
            id: panel_id(root.id, &value),
            aria_labelledby: trigger_id(root.id, &value),
            tabindex: "0",
            hidden: (!active).then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}
