//! ToggleGroup: a row or column of toggles that share one state. By default at most one item is
//! pressed; pass `multiple` to allow any number.
//!
//! Each item is a native button with `aria-pressed`, so `Space` and `Enter` toggle it. Arrow keys along
//! the group's orientation, `Home` and `End` move focus between enabled items without changing their
//! state. Only one item is a tab stop: the one that last had focus, or the first enabled item.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::toggle::{ToggleSize, ToggleVariant, toggle_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const GROUP_BASE: &str = "rounded-md data-[size=sm]:rounded-[min(var(--radius-md),8px)] group/toggle-group flex w-fit flex-row items-center gap-[--spacing(var(--gap))] data-vertical:flex-col data-vertical:items-stretch";

const ITEM_BASE: &str = "group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-md group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-md group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-md group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-md shrink-0 focus:z-10 focus-visible:z-10 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t";

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn move_focus(group_id: usize, key: &'static str, horizontal: bool, loop_focus: bool) {
    spawn(async move {
        let script = format!(
            r#"
            const group = document.getElementById('toggle-group-{group_id}');
            const items = group ? [...group.querySelectorAll('[data-slot="toggle-group-item"]:not(:disabled)')] : [];
            if (items.length) {{
                const rtl = getComputedStyle(group).direction === 'rtl';
                const forward = {horizontal} ? (rtl ? 'ArrowLeft' : 'ArrowRight') : 'ArrowDown';
                const current = items.indexOf(document.activeElement);
                let next = current;
                if ('{key}' === 'Home') next = 0;
                else if ('{key}' === 'End') next = items.length - 1;
                else if ('{key}' === forward) next = current + 1;
                else next = current - 1;
                if (next >= items.length) next = {loop_focus} ? 0 : items.length - 1;
                if (next < 0) next = {loop_focus} ? items.length - 1 : 0;
                items[next].focus();
            }}
            "#
        );
        let _ = document::eval(&script).await;
    });
}

fn variant_name(variant: ToggleVariant) -> &'static str {
    match variant {
        ToggleVariant::Default => "default",
        ToggleVariant::Outline => "outline",
    }
}

fn size_name(size: ToggleSize) -> &'static str {
    match size {
        ToggleSize::Default => "default",
        ToggleSize::Sm => "sm",
        ToggleSize::Lg => "lg",
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToggleGroupOrientation {
    fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy)]
struct ToggleGroupContext {
    id: usize,
    multiple: Signal<bool>,
    internal: Signal<Vec<String>>,
    controlled: Signal<Option<Vec<String>>>,
    disabled: Signal<bool>,
    variant: Signal<Option<ToggleVariant>>,
    size: Signal<Option<ToggleSize>>,
    spacing: Signal<u8>,
    orientation: Signal<ToggleGroupOrientation>,
    loop_focus: Signal<bool>,
    items: Signal<Vec<(String, bool)>>,
    active: Signal<Option<String>>,
    on_value_change: Option<EventHandler<Vec<String>>>,
}

impl ToggleGroupContext {
    fn current(&self) -> Vec<String> {
        self.controlled
            .read()
            .clone()
            .unwrap_or_else(|| self.internal.read().clone())
    }

    fn toggle(&self, value: &str) {
        let mut next = self.current();
        let pressed = next.iter().any(|item| item == value);
        if *self.multiple.peek() {
            if pressed {
                next.retain(|item| item != value);
            } else {
                next.push(value.to_string());
            }
        } else {
            next = if pressed {
                Vec::new()
            } else {
                vec![value.to_string()]
            };
        }
        if self.controlled.peek().is_none() {
            let mut internal = self.internal;
            internal.set(next.clone());
        }
        if let Some(handler) = self.on_value_change {
            handler.call(next);
        }
    }

    fn is_tab_stop(&self, value: &str) -> bool {
        let items = self.items.read();
        let enabled = |name: &str| {
            items
                .iter()
                .any(|(item, disabled)| item == name && !disabled)
        };
        match self.active.read().as_deref() {
            Some(active) if enabled(active) => active == value,
            _ => items
                .iter()
                .find(|(_, disabled)| !disabled)
                .is_some_and(|(first, _)| first == value),
        }
    }
}

/// Owns the pressed values. Pass `value` to control them, or leave it out and use `default_value`.
/// Without `multiple` the list holds at most one value.
#[component]
pub fn ToggleGroup(
    /// Lets any number of items be pressed. Otherwise one item is pressed at a time.
    #[props(default)]
    multiple: bool,
    /// Controlled values. When set, `on_value_change` is the only way to change them.
    value: Option<Vec<String>>,
    /// Values pressed at first render when `value` is not controlled.
    #[props(default)]
    default_value: Vec<String>,
    on_value_change: Option<EventHandler<Vec<String>>>,
    /// Overrides the variant of every item.
    variant: Option<ToggleVariant>,
    /// Overrides the size of every item.
    size: Option<ToggleSize>,
    /// Gap between items in quarter-rems. `0` joins the items into one connected control.
    #[props(default = 2)]
    spacing: u8,
    #[props(default)] orientation: ToggleGroupOrientation,
    #[props(default)] disabled: bool,
    /// Wraps focus from the last item to the first, and back.
    #[props(default = true)]
    loop_focus: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_value);
    let controlled = use_prop(value);
    let multiple = use_prop(multiple);
    let disabled = use_prop(disabled);
    let variant = use_prop(variant);
    let size = use_prop(size);
    let spacing = use_prop(spacing);
    let orientation = use_prop(orientation);
    let loop_focus = use_prop(loop_focus);
    let items = use_signal(Vec::new);
    let active = use_signal(|| None);
    use_context_provider(|| ToggleGroupContext {
        id,
        multiple,
        internal,
        controlled,
        disabled,
        variant,
        size,
        spacing,
        orientation,
        loop_focus,
        items,
        active,
        on_value_change,
    });

    let mut attributes = attributes;
    let class = cn!(GROUP_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            id: "toggle-group-{id}",
            "data-slot": "toggle-group",
            "data-variant": variant().map(variant_name),
            "data-size": size().map(size_name),
            "data-spacing": spacing().to_string(),
            "data-orientation": orientation().name(),
            "data-multiple": multiple().then_some(""),
            "data-disabled": disabled().then_some(""),
            role: "group",
            style: "--gap: {spacing()}",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One item of a `ToggleGroup`. It is pressed when `value` is in the group's values.
#[component]
pub fn ToggleGroupItem(
    value: String,
    /// Used when the group sets no variant.
    #[props(default)]
    variant: ToggleVariant,
    /// Used when the group sets no size.
    #[props(default)]
    size: ToggleSize,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let group = use_context::<ToggleGroupContext>();
    let variant = (group.variant)().unwrap_or(variant);
    let size = (group.size)().unwrap_or(size);
    let disabled = disabled || (group.disabled)();

    let mut items = group.items;
    let registered = value.clone();
    use_effect(move || {
        let mut list = items.write();
        match list.iter_mut().find(|(item, _)| *item == registered) {
            Some(entry) => entry.1 = disabled,
            None => list.push((registered.clone(), disabled)),
        }
    });
    let leaving = value.clone();
    use_drop(move || items.write().retain(|(item, _)| *item != leaving));

    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        toggle_variants(variant, size, None),
        take_class(&mut attributes).as_deref()
    );

    let pressed = group.current().contains(&value);
    let orientation = (group.orientation)();
    let focus_value = value.clone();

    rsx! {
        button {
            "data-slot": "toggle-group-item",
            "data-variant": variant_name(variant),
            "data-size": size_name(size),
            "data-spacing": (group.spacing)().to_string(),
            "data-pressed": pressed.then_some(""),
            "data-disabled": disabled.then_some(""),
            r#type: "button",
            "aria-pressed": if pressed { "true" } else { "false" },
            tabindex: if group.is_tab_stop(&value) { "0" } else { "-1" },
            disabled: disabled,
            class: class,
            onfocus: move |_| {
                let mut active = group.active;
                active.set(Some(focus_value.clone()));
            },
            onclick: move |event| {
                if disabled {
                    return;
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                group.toggle(&value);
            },
            onkeydown: move |event| {
                let horizontal = orientation == ToggleGroupOrientation::Horizontal;
                let key = match event.key() {
                    Key::ArrowRight if horizontal => Some("ArrowRight"),
                    Key::ArrowLeft if horizontal => Some("ArrowLeft"),
                    Key::ArrowDown if !horizontal => Some("ArrowDown"),
                    Key::ArrowUp if !horizontal => Some("ArrowUp"),
                    Key::Home => Some("Home"),
                    Key::End => Some("End"),
                    _ => None,
                };
                if let Some(key) = key {
                    event.prevent_default();
                    move_focus(group.id, key, horizontal, (group.loop_focus)());
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
