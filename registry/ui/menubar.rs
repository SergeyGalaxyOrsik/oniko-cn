//! Menubar: a horizontal bar of menus that open from their triggers, with actions, toggles and choices.
//!
//! Each `MenubarMenu` owns one trigger and one panel. Hovering another trigger while a menu is open switches
//! to it, and `ArrowLeft` and `ArrowRight` move between menus. Items are native buttons with `role="menuitem"`,
//! so `Enter` and `Space` activate them. `ArrowUp`, `ArrowDown`, `Home` and `End` move focus inside a panel,
//! and `Escape` closes the menu and returns focus to its trigger. The panel is positioned under the trigger with
//! CSS and is not portalled, so an ancestor with `overflow: hidden` clips it. No `dioxus-primitives` crate is
//! used: it is not published on crates.io, so panels are not flipped at viewport edges.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const ITEM_BASE: &str = "group/menubar-item relative flex w-full cursor-default items-center text-left outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn trigger_id(menu: usize) -> String {
    format!("menubar-trigger-{menu}")
}

fn content_id(menu: usize) -> String {
    format!("menubar-content-{menu}")
}

fn run_script(script: String) {
    spawn(async move {
        let _ = document::eval(&script).await;
    });
}

fn focus_element(id: String) {
    run_script(format!("document.getElementById('{id}')?.focus()"));
}

fn focus_menu_item(content_id: String, target: &'static str) {
    run_script(format!(
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
    ));
}

/// Clicks the neighbouring trigger when a menu is open, so it opens; otherwise only focuses it.
fn move_to_menu(bar_id: String, from_trigger: String, step: i32, was_open: bool) {
    let action = if was_open {
        "next.click()"
    } else {
        "next.focus()"
    };
    run_script(format!(
        r#"
        const bar = document.getElementById('{bar_id}');
        const triggers = bar ? [...bar.querySelectorAll('[data-slot="menubar-trigger"]')] : [];
        const index = triggers.findIndex((t) => t.id === '{from_trigger}');
        const next = triggers[(index + {step} + triggers.length) % triggers.length];
        if (next) {{ {action}; }}
        "#
    ));
}

#[derive(Clone, Copy)]
struct MenubarContext {
    id: usize,
    /// The menu whose panel is open, identified by its `MenuContext` id.
    open: Signal<Option<usize>>,
}

impl MenubarContext {
    fn bar_id(self) -> String {
        format!("menubar-{}", self.id)
    }
}

#[derive(Clone, Copy)]
struct MenuContext {
    id: usize,
}

fn close_menu(ctx: MenubarContext, menu: usize) {
    let mut open = ctx.open;
    open.set(None);
    focus_element(trigger_id(menu));
}

/// The bar. Its `role="menubar"` element is raised above the transparent layer that closes open menus,
/// so the triggers stay clickable and hoverable while a menu is open.
#[component]
pub fn Menubar(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let open = use_signal(|| None::<usize>);
    use_context_provider(|| MenubarContext { id, open });
    let mut attributes = attributes;
    let class = cn!(
        "cn-menubar relative flex items-center",
        if open().is_some() { "z-[60]" } else { "z-50" },
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        if open().is_some() {
            div {
                "data-slot": "menubar-overlay",
                class: "fixed inset-0 z-40",
                onclick: move |_| {
                    let mut open = open;
                    open.set(None);
                },
            }
        }
        div {
            id: format!("menubar-{id}"),
            "data-slot": "menubar",
            role: "menubar",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One menu of a `Menubar`: a trigger and its panel. Owns no state of its own.
#[component]
pub fn MenubarMenu(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    use_context_provider(|| MenuContext { id });
    let mut attributes = attributes;
    let class = cn!("relative", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "menubar-menu",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The button that opens its menu. Hovering it while another menu is open switches to this one.
#[component]
pub fn MenubarTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<MenubarContext>();
    let menu = use_context::<MenuContext>();
    let open = (ctx.open)() == Some(menu.id);
    let mut attributes = attributes;
    let class = cn!(
        "cn-menubar-trigger flex items-center outline-none select-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            id: trigger_id(menu.id),
            "data-slot": "menubar-trigger",
            "data-state": if open { "open" } else { "closed" },
            r#type: "button",
            role: "menuitem",
            aria_haspopup: "menu",
            aria_expanded: open,
            class: class,
            onclick: move |_| {
                let mut signal = ctx.open;
                signal.set(if open { None } else { Some(menu.id) });
            },
            onmouseenter: move |_| {
                if (ctx.open)().is_some_and(|current| current != menu.id) {
                    let mut signal = ctx.open;
                    signal.set(Some(menu.id));
                }
            },
            onkeydown: move |event| match event.key() {
                Key::ArrowDown => {
                    event.prevent_default();
                    let mut signal = ctx.open;
                    signal.set(Some(menu.id));
                }
                Key::ArrowLeft => {
                    event.prevent_default();
                    move_to_menu(ctx.bar_id(), trigger_id(menu.id), -1, open);
                }
                Key::ArrowRight => {
                    event.prevent_default();
                    move_to_menu(ctx.bar_id(), trigger_id(menu.id), 1, open);
                }
                _ => {}
            },
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenubarItemVariant {
    #[default]
    Default,
    Destructive,
}

impl MenubarItemVariant {
    fn data(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
        }
    }
}

/// The panel of a `MenubarMenu`. Renders nothing while its menu is closed.
#[component]
pub fn MenubarContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<MenubarContext>();
    let menu = use_context::<MenuContext>();
    if (ctx.open)() != Some(menu.id) {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "cn-menubar-content cn-menubar-content-logical absolute top-full left-0 z-50 mt-1 flex flex-col outline-none",
        take_class(&mut attributes).as_deref()
    );
    let id = content_id(menu.id);

    rsx! {
        div {
            id: id.clone(),
            "data-slot": "menubar-content",
            "data-open": "",
            "data-side": "bottom",
            role: "menu",
            tabindex: "-1",
            class: class,
            onmounted: {
                let id = id.clone();
                move |_| focus_element(id.clone())
            },
            onkeydown: move |event| {
                let target = match event.key() {
                    Key::ArrowDown => "down",
                    Key::ArrowUp => "up",
                    Key::Home => "first",
                    Key::End => "last",
                    Key::ArrowLeft => {
                        event.prevent_default();
                        move_to_menu(ctx.bar_id(), trigger_id(menu.id), -1, true);
                        return;
                    }
                    Key::ArrowRight => {
                        event.prevent_default();
                        move_to_menu(ctx.bar_id(), trigger_id(menu.id), 1, true);
                        return;
                    }
                    Key::Escape => {
                        close_menu(ctx, menu.id);
                        return;
                    }
                    Key::Tab => {
                        let mut open = ctx.open;
                        open.set(None);
                        return;
                    }
                    _ => return,
                };
                event.prevent_default();
                focus_menu_item(id.clone(), target);
            },
            ..attributes,
            {children}
        }
    }
}

/// Runs `onclick`, then closes the menu. Disabled items ignore clicks.
#[component]
pub fn MenubarItem(
    #[props(default)] variant: MenubarItemVariant,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<MenubarContext>();
    let menu = use_context::<MenuContext>();
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "cn-menubar-item",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "menubar-item",
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
                close_menu(ctx, menu.id);
            },
            ..attributes,
            {children}
        }
    }
}

/// A menu item that toggles `checked`. The caller owns the state and receives the new value in `onchecked_change`.
#[component]
pub fn MenubarCheckboxItem(
    #[props(default)] checked: bool,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    onchecked_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<MenubarContext>();
    let menu = use_context::<MenuContext>();
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "cn-menubar-checkbox-item",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "menubar-checkbox-item",
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
                close_menu(ctx, menu.id);
            },
            ..attributes,
            span {
                "data-slot": "menubar-item-indicator",
                class: "cn-menubar-checkbox-item-indicator pointer-events-none absolute flex items-center justify-center",
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

/// Groups `MenubarRadioItem`s. The caller owns `value` and receives picks in `on_value_change`.
#[component]
pub fn MenubarRadioGroup(
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
            "data-slot": "menubar-radio-group",
            role: "group",
            ..attributes,
            {children}
        }
    }
}

/// A menu item that is checked when its `value` equals the group's value.
#[component]
pub fn MenubarRadioItem(
    value: String,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<MenubarContext>();
    let menu = use_context::<MenuContext>();
    let group = use_context::<RadioGroupContext>();
    let checked = (group.value)() == value;
    let mut attributes = attributes;
    let class = cn!(
        ITEM_BASE,
        "cn-menubar-radio-item",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "menubar-radio-item",
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
                close_menu(ctx, menu.id);
            },
            ..attributes,
            span {
                "data-slot": "menubar-item-indicator",
                class: "cn-menubar-radio-item-indicator pointer-events-none absolute flex items-center justify-center",
                if checked {
                    DotIcon {}
                }
            }
            {children}
        }
    }
}

#[component]
pub fn MenubarGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "menubar-group",
            role: "group",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MenubarLabel(
    #[props(default)] inset: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-menubar-label", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "menubar-label",
            "data-inset": inset.then_some(""),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MenubarSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-menubar-separator",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "menubar-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}

#[component]
pub fn MenubarShortcut(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-menubar-shortcut",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "menubar-shortcut",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone)]
struct SubContext {
    id: String,
    open: Signal<bool>,
}

/// A nested menu. Owns the open state of its `MenubarSubContent`.
#[component]
pub fn MenubarSub(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("menubar-sub-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
    let open = use_signal(|| false);
    use_context_provider(|| SubContext { id, open });
    let mut attributes = attributes;
    let class = cn!("relative", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "menubar-sub",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MenubarSubTrigger(
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
        "cn-menubar-sub-trigger",
        take_class(&mut attributes).as_deref()
    );
    let id = format!("{}-trigger", sub.id);

    rsx! {
        button {
            id: id,
            "data-slot": "menubar-sub-trigger",
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
            onkeydown: move |event| {
                if event.key() == Key::ArrowRight {
                    event.stop_propagation();
                    event.prevent_default();
                    let mut open_signal = sub.open;
                    open_signal.set(true);
                }
            },
            ..attributes,
            {children}
            ChevronIcon {}
        }
    }
}

/// The nested panel. Renders nothing while its `MenubarSub` is closed. `ArrowLeft` closes it.
#[component]
pub fn MenubarSubContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let sub = use_context::<SubContext>();
    if !(sub.open)() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "cn-menubar-sub-content absolute top-0 left-full z-50 ml-1 flex flex-col outline-none",
        take_class(&mut attributes).as_deref()
    );
    let trigger = format!("{}-trigger", sub.id);
    let content = format!("{}-content", sub.id);

    rsx! {
        div {
            id: content.clone(),
            "data-slot": "menubar-sub-content",
            "data-open": "",
            "data-side": "right",
            role: "menu",
            tabindex: "-1",
            class: class,
            onmounted: {
                let content = content.clone();
                move |_| focus_element(content.clone())
            },
            onkeydown: move |event| {
                if event.key() == Key::ArrowLeft {
                    event.stop_propagation();
                    event.prevent_default();
                    let mut open_signal = sub.open;
                    open_signal.set(false);
                    focus_element(trigger.clone());
                }
            },
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
