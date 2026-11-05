//! ToggleGroup: a row or column of toggles that share one state. In `Single` mode at most one item is
//! pressed; in `Multiple` mode any number can be.
//!
//! Each item is a native button with `aria-pressed`, so `Space` and `Enter` toggle it. Arrow keys move
//! focus between enabled items along the group's orientation without changing their state. Pressed items
//! are the tab stops; while nothing is pressed, every item is one.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::toggle::{ToggleSize, ToggleVariant, toggle_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn move_focus(group_id: usize, step: i32) {
    spawn(async move {
        let script = format!(
            r#"
            const group = document.getElementById('toggle-group-{group_id}');
            const items = group ? [...group.querySelectorAll('[data-slot="toggle-group-item"]:not(:disabled)')] : [];
            if (items.length) {{
                const current = items.indexOf(document.activeElement);
                items[(current + {step} + items.length) % items.length].focus();
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

fn state(pressed: bool) -> &'static str {
    if pressed { "on" } else { "off" }
}

/// Whether the group allows one pressed item or several.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleGroupType {
    #[default]
    Single,
    Multiple,
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
    kind: Signal<ToggleGroupType>,
    internal: Signal<Vec<String>>,
    controlled: Signal<Option<Vec<String>>>,
    disabled: Signal<bool>,
    variant: Signal<ToggleVariant>,
    size: Signal<ToggleSize>,
    spacing: Signal<u8>,
    orientation: Signal<ToggleGroupOrientation>,
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
        match *self.kind.peek() {
            ToggleGroupType::Single => {
                next = if pressed {
                    Vec::new()
                } else {
                    vec![value.to_string()]
                };
            }
            ToggleGroupType::Multiple => {
                if pressed {
                    next.retain(|item| item != value);
                } else {
                    next.push(value.to_string());
                }
            }
        }
        if self.controlled.peek().is_none() {
            let mut internal = self.internal;
            internal.set(next.clone());
        }
        if let Some(handler) = self.on_value_change {
            handler.call(next);
        }
    }
}

/// Owns the pressed values. Pass `value` to control them, or leave it out and use `default_value`.
/// In `Single` mode the list holds at most one value.
#[component]
pub fn ToggleGroup(
    /// `Single` lets one item be pressed at a time; `Multiple` lets any number be pressed.
    #[props(default)]
    kind: ToggleGroupType,
    /// Controlled values. When set, `on_value_change` is the only way to change them.
    value: Option<Vec<String>>,
    /// Values pressed at first render when `value` is not controlled.
    #[props(default)]
    default_value: Vec<String>,
    on_value_change: Option<EventHandler<Vec<String>>>,
    #[props(default)] variant: ToggleVariant,
    #[props(default)] size: ToggleSize,
    /// Gap between items in quarter-rems. `0` joins the items into one connected control.
    #[props(default)]
    spacing: u8,
    #[props(default)] orientation: ToggleGroupOrientation,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_value);
    let controlled = use_prop(value);
    let kind = use_prop(kind);
    let disabled = use_prop(disabled);
    let variant = use_prop(variant);
    let size = use_prop(size);
    let spacing = use_prop(spacing);
    let orientation = use_prop(orientation);
    use_context_provider(|| ToggleGroupContext {
        id,
        kind,
        internal,
        controlled,
        disabled,
        variant,
        size,
        spacing,
        orientation,
        on_value_change,
    });

    let mut attributes = attributes;
    let flow = match orientation() {
        ToggleGroupOrientation::Horizontal => "flex-row items-center",
        ToggleGroupOrientation::Vertical => "flex-col items-stretch",
    };
    let class = cn!(
        "rounded-md data-[spacing=0]:data-[variant=outline]:shadow-xs group/toggle-group flex w-fit",
        flow,
        take_class(&mut attributes).as_deref()
    );
    let gap = format!("gap: {}rem", f32::from(spacing()) * 0.25);

    rsx! {
        div {
            id: "toggle-group-{id}",
            "data-slot": "toggle-group",
            "data-variant": variant_name(variant()),
            "data-size": size_name(size()),
            "data-spacing": spacing().to_string(),
            "data-orientation": orientation().name(),
            "data-disabled": disabled().then_some("true"),
            role: "group",
            aria_disabled: disabled(),
            style: gap,
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
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let group = use_context::<ToggleGroupContext>();
    let mut attributes = attributes;
    let toggled = toggle_variants((group.variant)(), (group.size)(), None);
    let class = cn!(
        "data-[state=on]:bg-muted group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:shadow-none group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-md group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-md group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-md group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-md",
        toggled,
        take_class(&mut attributes).as_deref()
    );

    let current = group.current();
    let pressed = current.contains(&value);
    let disabled = disabled || (group.disabled)();
    let orientation = (group.orientation)();
    let tabindex = if pressed || current.is_empty() {
        "0"
    } else {
        "-1"
    };

    rsx! {
        button {
            "data-slot": "toggle-group-item",
            "data-state": state(pressed),
            "data-variant": variant_name((group.variant)()),
            "data-size": size_name((group.size)()),
            "data-spacing": (group.spacing)().to_string(),
            "data-disabled": disabled.then_some("true"),
            r#type: "button",
            aria_pressed: pressed,
            tabindex: tabindex,
            disabled: disabled,
            class: class,
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
                let step = match (orientation, event.key()) {
                    (ToggleGroupOrientation::Horizontal, Key::ArrowRight)
                    | (ToggleGroupOrientation::Vertical, Key::ArrowDown) => 1,
                    (ToggleGroupOrientation::Horizontal, Key::ArrowLeft)
                    | (ToggleGroupOrientation::Vertical, Key::ArrowUp) => -1,
                    _ => 0,
                };
                if step != 0 {
                    event.prevent_default();
                    move_focus(group.id, step);
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
