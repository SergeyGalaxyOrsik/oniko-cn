//! NavigationMenu: a horizontal bar of links and disclosure panels for site navigation.
//!
//! Each `NavigationMenuItem` owns one `NavigationMenuTrigger` and one `NavigationMenuContent` panel. Hovering another
//! trigger while a panel is open switches to it, and `ArrowLeft` and `ArrowRight` move between triggers. `ArrowDown`
//! opens the panel of the focused trigger and moves focus into it, and `Escape` closes it and returns focus to its
//! trigger. Clicking a link inside a panel closes the panel. The panel is positioned under the trigger with CSS and is
//! not portalled, so an ancestor with `overflow: hidden` clips it. No `dioxus-primitives` crate is used: it is not
//! published on crates.io and has no navigation menu, so panels are not flipped at viewport edges.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const TRIGGER_BASE: &str = "cn-navigation-menu-trigger group/navigation-menu-trigger inline-flex w-max items-center justify-center outline-none disabled:pointer-events-none";

/// Trigger classes for any element, e.g. a link that should look like a `NavigationMenuTrigger`.
pub fn navigation_menu_trigger_style(class: Option<&str>) -> String {
    cn!(TRIGGER_BASE, class)
}

fn trigger_id(item: usize) -> String {
    format!("navigation-menu-trigger-{item}")
}

fn content_id(item: usize) -> String {
    format!("navigation-menu-content-{item}")
}

fn run_script(script: String) {
    spawn(async move {
        let _ = document::eval(&script).await;
    });
}

fn focus_element(id: String) {
    run_script(format!("document.getElementById('{id}')?.focus()"));
}

/// Moves to the neighbouring trigger, opening its panel too when one was open.
fn move_to_trigger(nav_id: String, from_trigger: String, step: i32, was_open: bool) {
    let action = if was_open {
        "next.click()"
    } else {
        "next.focus()"
    };
    run_script(format!(
        r#"
        const nav = document.getElementById('{nav_id}');
        const triggers = nav ? [...nav.querySelectorAll('[data-slot="navigation-menu-trigger"]')] : [];
        const index = triggers.findIndex((t) => t.id === '{from_trigger}');
        const next = triggers[(index + {step} + triggers.length) % triggers.length];
        if (next) {{ {action}; }}
        "#
    ));
}

#[derive(Clone, Copy)]
struct NavigationMenuContext {
    id: usize,
    /// The item whose panel is open, identified by its `NavigationMenuItemContext` id.
    open: Signal<Option<usize>>,
}

impl NavigationMenuContext {
    fn nav_id(self) -> String {
        format!("navigation-menu-{}", self.id)
    }
}

#[derive(Clone, Copy)]
struct NavigationMenuItemContext {
    id: usize,
}

fn close(ctx: NavigationMenuContext) {
    let mut open = ctx.open;
    open.set(None);
}

fn close_and_focus(ctx: NavigationMenuContext, item: usize) {
    close(ctx);
    focus_element(trigger_id(item));
}

/// The navigation landmark. Its panels are raised above the transparent layer that closes open panels, so the
/// triggers stay clickable and hoverable while a panel is open.
#[component]
pub fn NavigationMenu(
    #[props(extends = GlobalAttributes, extends = nav)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let open = use_signal(|| None::<usize>);
    use_context_provider(|| NavigationMenuContext { id, open });
    let mut attributes = attributes;
    let class = cn!(
        "cn-navigation-menu group/navigation-menu relative flex max-w-max flex-1 items-center justify-center",
        if open().is_some() { "z-[60]" } else { "z-50" },
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        if open().is_some() {
            div {
                "data-slot": "navigation-menu-overlay",
                class: "fixed inset-0 z-40",
                onclick: move |_| {
                    let mut open = open;
                    open.set(None);
                },
            }
        }
        nav {
            id: format!("navigation-menu-{id}"),
            "data-slot": "navigation-menu",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn NavigationMenuList(
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-navigation-menu-list group flex flex-1 list-none items-center justify-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        ul {
            "data-slot": "navigation-menu-list",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One entry of a `NavigationMenuList`: a trigger and its panel.
#[component]
pub fn NavigationMenuItem(
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    use_context_provider(|| NavigationMenuItemContext { id });
    let mut attributes = attributes;
    let class = cn!("relative", take_class(&mut attributes).as_deref());

    rsx! {
        li {
            "data-slot": "navigation-menu-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The button that opens its item's panel. Hovering it while another panel is open switches to this one.
#[component]
pub fn NavigationMenuTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let item = use_context::<NavigationMenuItemContext>();
    let open = (ctx.open)() == Some(item.id);
    let mut attributes = attributes;
    let class = navigation_menu_trigger_style(take_class(&mut attributes).as_deref());

    rsx! {
        button {
            id: trigger_id(item.id),
            "data-slot": "navigation-menu-trigger",
            "data-state": if open { "open" } else { "closed" },
            "data-open": open.then_some(""),
            r#type: "button",
            aria_expanded: open,
            class: class,
            onclick: move |_| {
                let mut signal = ctx.open;
                signal.set(if open { None } else { Some(item.id) });
            },
            onmouseenter: move |_| {
                if (ctx.open)().is_some_and(|current| current != item.id) {
                    let mut signal = ctx.open;
                    signal.set(Some(item.id));
                }
            },
            onkeydown: move |event| match event.key() {
                Key::ArrowDown => {
                    event.prevent_default();
                    let mut signal = ctx.open;
                    signal.set(Some(item.id));
                }
                Key::ArrowLeft => {
                    event.prevent_default();
                    move_to_trigger(ctx.nav_id(), trigger_id(item.id), -1, open);
                }
                Key::ArrowRight => {
                    event.prevent_default();
                    move_to_trigger(ctx.nav_id(), trigger_id(item.id), 1, open);
                }
                Key::Escape if open => close(ctx),
                _ => {}
            },
            ..attributes,
            {children}
            svg {
                "data-slot": "navigation-menu-trigger-icon",
                "aria-hidden": "true",
                class: "cn-navigation-menu-trigger-icon",
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

/// The panel of a `NavigationMenuItem`. Renders nothing while its panel is closed. Clicking a link inside it closes it.
#[component]
pub fn NavigationMenuContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let item = use_context::<NavigationMenuItemContext>();
    if (ctx.open)() != Some(item.id) {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "cn-navigation-menu-content cn-navigation-menu-popup absolute top-full left-0 z-50 mt-1.5 w-max outline-none",
        take_class(&mut attributes).as_deref()
    );
    let id = content_id(item.id);

    rsx! {
        div {
            id: id.clone(),
            "data-slot": "navigation-menu-content",
            "data-state": "open",
            tabindex: "-1",
            class: class,
            onmounted: {
                let id = id.clone();
                move |_| focus_element(id.clone())
            },
            onclick: move |_| close(ctx),
            onkeydown: move |event| {
                if event.key() == Key::Escape {
                    event.prevent_default();
                    close_and_focus(ctx, item.id);
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// A link. Inside a `NavigationMenuContent` it closes the panel when clicked; `active` marks the current page.
#[component]
pub fn NavigationMenuLink(
    #[props(default)] active: bool,
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-navigation-menu-link",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        a {
            "data-slot": "navigation-menu-link",
            "data-active": active.then_some(""),
            aria_current: active.then_some("page"),
            class: class,
            ..attributes,
            {children}
        }
    }
}
