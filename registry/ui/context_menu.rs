//! ContextMenu: a floating menu that opens at the pointer on right click and runs an action or toggles an option.
//!
//! The menu is positioned with `position: fixed` at the pointer and dismissed by a transparent full-screen
//! layer, `Escape` or `Tab`. Arrow keys move focus between items and hovering an item focuses it. No
//! `dioxus-primitives` crate is used: it is not published on crates.io, so submenus are not provided and the
//! menu is not flipped at viewport edges.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const ITEM_BASE: &str = "group/context-menu-item relative flex w-full cursor-default items-center text-left outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn state(checked: bool) -> &'static str {
    if checked { "checked" } else { "unchecked" }
}

#[derive(Clone, Copy)]
struct ContextMenuContext {
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    position: Signal<(f64, f64)>,
    on_open_change: Option<EventHandler<bool>>,
    id: usize,
}

impl ContextMenuContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn set_open(mut self, open: bool) {
        if self.controlled.peek().is_none() {
            self.internal.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }

    fn open_at(mut self, x: f64, y: f64) {
        self.position.set((x, y));
        if !self.is_open() {
            self.set_open(true);
        }
    }
}

#[derive(Clone, Copy)]
struct RadioGroupContext {
    internal: Signal<String>,
    controlled: Signal<Option<String>>,
    on_value_change: Option<EventHandler<String>>,
}

impl RadioGroupContext {
    fn value(self) -> String {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn select(mut self, value: String) {
        if self.controlled.peek().is_none() {
            self.internal.set(value.clone());
        }
        if let Some(handler) = self.on_value_change {
            handler.call(value);
        }
    }
}

/// Owns the open state and the pointer position. Pass `open` and `on_open_change` to control it.
#[component]
pub fn ContextMenu(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let internal = use_signal(|| false);
    let controlled = use_prop(open);
    let position = use_signal(|| (0.0, 0.0));
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    use_context_provider(|| ContextMenuContext {
        internal,
        controlled,
        position,
        on_open_change,
        id,
    });

    rsx! { {children} }
}

/// The area that opens the menu on right click. Wraps the content the menu belongs to.
#[component]
pub fn ContextMenuTrigger(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ContextMenuContext>();
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "context-menu-trigger",
            class: class,
            oncontextmenu: move |event| {
                event.prevent_default();
                let point = event.client_coordinates();
                ctx.open_at(point.x, point.y);
            },
            ..attributes,
            {children}
        }
    }
}

fn move_focus(id: usize, step: i32) {
    let script = format!(
        r#"const root = document.getElementById("context-menu-{id}");
if (root) {{
  const items = Array.from(root.querySelectorAll("[role^='menuitem']:not(:disabled)"));
  if (items.length) {{
    const step = {step};
    const index = items.indexOf(document.activeElement);
    const next = index < 0 ? (step > 0 ? 0 : items.length - 1) : (index + step + items.length) % items.length;
    items[next].focus();
  }}
}}
return true;"#
    );
    spawn(async move {
        let _ = document::eval(&script).await;
    });
}

fn focus_item(node: Signal<Option<Rc<MountedData>>>) {
    if let Some(item) = node.peek().clone() {
        spawn(async move {
            let _ = item.set_focus(true).await;
        });
    }
}

/// The menu panel and a transparent layer behind it. Renders nothing while the menu is closed.
#[component]
pub fn ContextMenuContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ContextMenuContext>();
    if !ctx.is_open() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "cn-context-menu-content fixed z-50 outline-none",
        take_class(&mut attributes).as_deref()
    );
    let (x, y) = (ctx.position)();
    let id = ctx.id;

    rsx! {
        div {
            "data-slot": "context-menu-backdrop",
            class: "fixed inset-0 z-40",
            onclick: move |_| ctx.set_open(false),
            oncontextmenu: move |event| {
                event.prevent_default();
                let point = event.client_coordinates();
                ctx.open_at(point.x, point.y);
            },
        }
        div {
            "data-slot": "context-menu-content",
            "data-state": "open",
            id: "context-menu-{id}",
            role: "menu",
            tabindex: "-1",
            style: "left: {x}px; top: {y}px",
            class: class,
            onmounted: move |event| {
                let node = event.data();
                spawn(async move {
                    let _ = node.set_focus(true).await;
                });
            },
            onkeydown: move |event| {
                match event.key() {
                    Key::Escape | Key::Tab => ctx.set_open(false),
                    Key::ArrowDown => {
                        event.prevent_default();
                        move_focus(id, 1);
                    }
                    Key::ArrowUp => {
                        event.prevent_default();
                        move_focus(id, -1);
                    }
                    _ => {}
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContextMenuItemVariant {
    #[default]
    Default,
    Destructive,
}

impl ContextMenuItemVariant {
    fn data(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
        }
    }
}

/// An action. Runs `onclick`, then closes the menu.
#[component]
pub fn ContextMenuItem(
    #[props(default)] variant: ContextMenuItemVariant,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ContextMenuContext>();
    let mut node = use_signal(|| None::<Rc<MountedData>>);
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "cn-context-menu-item",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "context-menu-item",
            "data-variant": variant.data(),
            "data-inset": inset.then_some("true"),
            r#type: "button",
            role: "menuitem",
            tabindex: "-1",
            disabled: disabled,
            class: class,
            onmounted: move |event| node.set(Some(event.data())),
            onmouseenter: move |_| focus_item(node),
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

/// An option that can be checked. Pass `checked` to control it, or use `default_checked`.
/// Toggles, reports through `on_checked_change`, then closes the menu.
#[component]
pub fn ContextMenuCheckboxItem(
    checked: Option<bool>,
    #[props(default)] default_checked: bool,
    on_checked_change: Option<EventHandler<bool>>,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ContextMenuContext>();
    let mut internal = use_signal(|| default_checked);
    let mut node = use_signal(|| None::<Rc<MountedData>>);
    let is_checked = checked.unwrap_or(internal());
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "cn-context-menu-checkbox-item",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "context-menu-checkbox-item",
            "data-state": state(is_checked),
            "data-inset": inset.then_some("true"),
            r#type: "button",
            role: "menuitemcheckbox",
            tabindex: "-1",
            aria_checked: is_checked,
            disabled: disabled,
            class: class,
            onmounted: move |event| node.set(Some(event.data())),
            onmouseenter: move |_| focus_item(node),
            onclick: move |event| {
                if checked.is_none() {
                    internal.set(!is_checked);
                }
                if let Some(handler) = on_checked_change {
                    handler.call(!is_checked);
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                ctx.set_open(false);
            },
            ..attributes,
            span {
                "data-slot": "context-menu-item-indicator",
                class: "cn-context-menu-item-indicator pointer-events-none flex items-center justify-center",
                if is_checked {
                    svg {
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
            {children}
        }
    }
}

/// Groups radio items. Pass `value` to control it, or use `default_value`.
#[component]
pub fn ContextMenuRadioGroup(
    value: Option<String>,
    #[props(default)] default_value: String,
    on_value_change: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let internal = use_signal(|| default_value);
    let controlled = use_prop(value);
    use_context_provider(|| RadioGroupContext {
        internal,
        controlled,
        on_value_change,
    });

    rsx! {
        div {
            "data-slot": "context-menu-radio-group",
            role: "group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One option of the enclosing `ContextMenuRadioGroup`. Selecting it sets the group value, then closes the menu.
#[component]
pub fn ContextMenuRadioItem(
    value: String,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ContextMenuContext>();
    let group = use_context::<RadioGroupContext>();
    let mut node = use_signal(|| None::<Rc<MountedData>>);
    let is_checked = group.value() == value;
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "cn-context-menu-radio-item",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "context-menu-radio-item",
            "data-state": state(is_checked),
            "data-inset": inset.then_some("true"),
            r#type: "button",
            role: "menuitemradio",
            tabindex: "-1",
            aria_checked: is_checked,
            disabled: disabled,
            class: class,
            onmounted: move |event| node.set(Some(event.data())),
            onmouseenter: move |_| focus_item(node),
            onclick: move |event| {
                group.select(value.clone());
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                ctx.set_open(false);
            },
            ..attributes,
            span {
                "data-slot": "context-menu-item-indicator",
                class: "cn-context-menu-item-indicator pointer-events-none flex items-center justify-center",
                if is_checked {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        circle { cx: "12", cy: "12", r: "4" }
                    }
                }
            }
            {children}
        }
    }
}

#[component]
pub fn ContextMenuLabel(
    #[props(default)] inset: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-context-menu-label", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "context-menu-label",
            "data-inset": inset.then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ContextMenuSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-context-menu-separator", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "context-menu-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}

/// Keyboard shortcut hint, aligned to the end of an item.
#[component]
pub fn ContextMenuShortcut(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-context-menu-shortcut", take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "context-menu-shortcut",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Wraps related items so they can be grouped together.
#[component]
pub fn ContextMenuGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "context-menu-group",
            role: "group",
            class: class,
            ..attributes,
            {children}
        }
    }
}
