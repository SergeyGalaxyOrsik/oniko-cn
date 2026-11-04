//! DropdownMenu: a menu of actions, toggles and choices that opens from a trigger.
//!
//! Items are native buttons with `role="menuitem"`, so `Enter` and `Space` activate them. Arrow keys,
//! `Home` and `End` move focus between items, `Escape` closes the menu and returns focus to the trigger.
//! The panel is positioned under the trigger with CSS and dismissed by a transparent full-screen layer.
//! The panel is not portalled, so it is clipped by an ancestor with `overflow: hidden`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn focus_element(id: String) {
    spawn(async move {
        let _ = document::eval(&format!("document.getElementById('{id}')?.focus()")).await;
    });
}

fn focus_menu_item(content_id: String, target: &'static str) {
    spawn(async move {
        let script = format!(
            r#"
            const menu = document.getElementById('{content_id}');
            const items = menu ? [...menu.querySelectorAll('[role^="menuitem"]')].filter((e) => !e.disabled) : [];
            if (items.length) {{
                const current = items.indexOf(document.activeElement);
                const last = items.length - 1;
                const next = {{
                    first: 0,
                    last,
                    down: current + 1 > last ? 0 : current + 1,
                    up: current <= 0 ? last : current - 1,
                }}["{target}"];
                items[next].focus();
            }}
            "#
        );
        let _ = document::eval(&script).await;
    });
}

#[derive(Clone)]
struct DropdownMenuContext {
    id: String,
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl DropdownMenuContext {
    fn is_open(&self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn trigger_id(&self) -> String {
        format!("{}-trigger", self.id)
    }

    fn content_id(&self) -> String {
        format!("{}-content", self.id)
    }

    fn set_open(&self, open: bool) {
        if self.controlled.peek().is_none() {
            let mut internal = self.internal;
            internal.set(open);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }

    fn close_and_focus_trigger(&self) {
        self.set_open(false);
        focus_element(self.trigger_id());
    }
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
#[component]
pub fn DropdownMenu(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("dropdown-menu-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
    let mut attributes = attributes;
    let class = cn!(
        "relative inline-block",
        take_class(&mut attributes).as_deref()
    );
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    use_context_provider(|| DropdownMenuContext {
        id,
        internal,
        controlled,
        on_open_change,
    });

    rsx! {
        div {
            "data-slot": "dropdown-menu",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DropdownMenuContext>();
    let open = ctx.is_open();
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());
    let toggle = ctx.clone();

    rsx! {
        button {
            id: ctx.trigger_id(),
            "data-slot": "dropdown-menu-trigger",
            "data-state": if open { "open" } else { "closed" },
            r#type: "button",
            aria_haspopup: "menu",
            aria_expanded: open,
            class: class,
            onclick: move |_| toggle.set_open(!open),
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownMenuAlign {
    #[default]
    Start,
    End,
}

impl DropdownMenuAlign {
    fn class(self) -> &'static str {
        match self {
            Self::Start => "left-0",
            Self::End => "right-0",
        }
    }
}

/// The menu panel and its transparent layer. Renders nothing while the menu is closed.
#[component]
pub fn DropdownMenuContent(
    #[props(default)] align: DropdownMenuAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DropdownMenuContext>();
    if !ctx.is_open() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-md p-1 shadow-md ring-1 duration-100 absolute top-full z-50 mt-1 flex flex-col outline-none",
        align.class(),
        take_class(&mut attributes).as_deref()
    );
    let content_id = ctx.content_id();
    let key_ctx = ctx.clone();
    let overlay_ctx = ctx.clone();

    rsx! {
        div {
            "data-slot": "dropdown-menu-overlay",
            class: "fixed inset-0 z-40",
            onclick: move |_| overlay_ctx.set_open(false),
        }
        div {
            id: content_id.clone(),
            "data-slot": "dropdown-menu-content",
            "data-open": "",
            "data-side": "bottom",
            role: "menu",
            tabindex: "-1",
            class: class,
            onmounted: {
                let content_id = content_id.clone();
                move |_| focus_element(content_id.clone())
            },
            onkeydown: move |event| {
                let target = match event.key() {
                    Key::ArrowDown => "down",
                    Key::ArrowUp => "up",
                    Key::Home => "first",
                    Key::End => "last",
                    Key::Escape => {
                        key_ctx.close_and_focus_trigger();
                        return;
                    }
                    _ => return,
                };
                event.prevent_default();
                focus_menu_item(content_id.clone(), target);
            },
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownMenuItemVariant {
    #[default]
    Default,
    Destructive,
}

impl DropdownMenuItemVariant {
    fn data(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
        }
    }
}

const ITEM_BASE: &str = "relative flex w-full items-center text-left outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";

/// Runs `onclick`, then closes the menu. Disabled items ignore clicks.
#[component]
pub fn DropdownMenuItem(
    #[props(default)] variant: DropdownMenuItemVariant,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<DropdownMenuContext>();
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-2 rounded-sm px-2 py-1.5 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "dropdown-menu-item",
            "data-variant": variant.data(),
            "data-inset": inset.then_some(""),
            r#type: "button",
            role: "menuitem",
            tabindex: "-1",
            disabled: disabled,
            class: class,
            onclick: move |event| {
                if disabled {
                    return;
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                ctx.close_and_focus_trigger();
            },
            ..attributes,
            {children}
        }
    }
}

/// A menu item that toggles `checked`. The caller owns the state and receives the new value in `onchecked_change`.
#[component]
pub fn DropdownMenuCheckboxItem(
    #[props(default)] checked: bool,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    onchecked_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DropdownMenuContext>();
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "dropdown-menu-checkbox-item",
            "data-state": if checked { "checked" } else { "unchecked" },
            "data-inset": inset.then_some(""),
            r#type: "button",
            role: "menuitemcheckbox",
            aria_checked: checked,
            tabindex: "-1",
            disabled: disabled,
            class: class,
            onclick: move |_| {
                if disabled {
                    return;
                }
                if let Some(handler) = onchecked_change {
                    handler.call(!checked);
                }
                ctx.close_and_focus_trigger();
            },
            ..attributes,
            span {
                "data-slot": "dropdown-menu-item-indicator",
                class: "absolute right-2 flex items-center justify-center",
                if checked {
                    CheckIcon {}
                }
            }
            {children}
        }
    }
}

#[derive(Clone)]
struct RadioGroupContext {
    value: Signal<String>,
    on_value_change: Option<EventHandler<String>>,
}

/// Groups `DropdownMenuRadioItem`s. The caller owns `value` and receives picks in `on_value_change`.
#[component]
pub fn DropdownMenuRadioGroup(
    value: String,
    on_value_change: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let signal = use_prop(value);
    use_context_provider(|| RadioGroupContext {
        value: signal,
        on_value_change,
    });

    rsx! {
        div {
            "data-slot": "dropdown-menu-radio-group",
            role: "group",
            ..attributes,
            {children}
        }
    }
}

/// A menu item that is checked when its `value` equals the group's value.
#[component]
pub fn DropdownMenuRadioItem(
    value: String,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<DropdownMenuContext>();
    let group = use_context::<RadioGroupContext>();
    let checked = (group.value)() == value;
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "dropdown-menu-radio-item",
            "data-state": if checked { "checked" } else { "unchecked" },
            "data-inset": inset.then_some(""),
            r#type: "button",
            role: "menuitemradio",
            aria_checked: checked,
            tabindex: "-1",
            disabled: disabled,
            class: class,
            onclick: move |_| {
                if disabled {
                    return;
                }
                if let Some(handler) = group.on_value_change {
                    handler.call(value.clone());
                }
                ctx.close_and_focus_trigger();
            },
            ..attributes,
            span {
                "data-slot": "dropdown-menu-item-indicator",
                class: "absolute right-2 flex items-center justify-center",
                if checked {
                    DotIcon {}
                }
            }
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dropdown-menu-group",
            role: "group",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuLabel(
    #[props(default)] inset: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground px-2 py-1.5 text-xs font-medium data-inset:pl-8",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "dropdown-menu-label",
            "data-inset": inset.then_some(""),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-border -mx-1 my-1 h-px",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "dropdown-menu-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}

#[component]
pub fn DropdownMenuShortcut(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "dropdown-menu-shortcut",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy)]
struct SubContext {
    open: Signal<bool>,
}

/// A nested menu. Owns the open state of its `DropdownMenuSubContent`.
#[component]
pub fn DropdownMenuSub(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let open = use_signal(|| false);
    use_context_provider(|| SubContext { open });
    let mut attributes = attributes;
    let class = cn!("relative", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "dropdown-menu-sub",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuSubTrigger(
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let sub = use_context::<SubContext>();
    let open = (sub.open)();
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-2 rounded-sm px-2 py-1.5 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "dropdown-menu-sub-trigger",
            "data-state": if open { "open" } else { "closed" },
            "data-inset": inset.then_some(""),
            r#type: "button",
            role: "menuitem",
            aria_haspopup: "menu",
            aria_expanded: open,
            tabindex: "-1",
            disabled: disabled,
            class: class,
            onclick: move |_| {
                let mut open_signal = sub.open;
                let next = !open_signal();
                open_signal.set(next);
            },
            ..attributes,
            {children}
            ChevronIcon {}
        }
    }
}

/// The nested panel. Renders nothing while its `DropdownMenuSub` is closed.
#[component]
pub fn DropdownMenuSubContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let sub = use_context::<SubContext>();
    if !(sub.open)() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-md p-1 shadow-lg ring-1 duration-100 absolute top-0 left-full z-50 ml-1 flex flex-col outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "dropdown-menu-sub-content",
            "data-open": "",
            "data-side": "right",
            role: "menu",
            tabindex: "-1",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
fn CheckIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "16",
            height: "16",
            path { d: "M20 6 9 17l-5-5" }
        }
    }
}

#[component]
fn DotIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            width: "8",
            height: "8",
            circle { cx: "12", cy: "12", r: "12" }
        }
    }
}

#[component]
fn ChevronIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "16",
            height: "16",
            class: "ml-auto",
            path { d: "m9 18 6-6-6-6" }
        }
    }
}
