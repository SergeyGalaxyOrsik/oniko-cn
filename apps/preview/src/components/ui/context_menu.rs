//! ContextMenu: a menu that opens at the pointer on right click or long press.
//!
//! A port of shadcn/ui's Base UI context menu. It reuses the popup, keyboard navigation, submenus and
//! item behavior of `DropdownMenu`, so it needs that component installed. The popup opens next to the
//! pointer (`side` right and `align` start by default), flips and shifts to stay inside the window, and
//! closes on `Escape`, on a click outside or when an item is chosen. A touch held for half a second opens
//! it too.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

pub use super::dropdown_menu::{
    DropdownMenuAlign as ContextMenuAlign, DropdownMenuItemVariant as ContextMenuItemVariant,
    DropdownMenuSide as ContextMenuSide,
};
use super::dropdown_menu::{
    DropdownMenuSub, MenuCheckboxItem, MenuItem, MenuPopup, MenuRadioGroup, MenuRadioItem,
    MenuState, MenuSubTrigger, use_menu_state,
};

const CONTENT_CLASS: &str = "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-md p-1 shadow-md ring-1 duration-100 data-[side=inline-start]:slide-in-from-right-2 data-[side=inline-end]:slide-in-from-left-2 animate-none! relative bg-popover/70 before:pointer-events-none before:absolute before:inset-0 before:-z-1 before:rounded-[inherit] before:backdrop-blur-2xl before:backdrop-saturate-150 **:data-[slot$=-item]:focus:bg-foreground/10 **:data-[slot$=-item]:data-highlighted:bg-foreground/10 **:data-[slot$=-separator]:bg-foreground/5 **:data-[slot$=-trigger]:focus:bg-foreground/10 **:data-[slot$=-trigger]:aria-expanded:bg-foreground/10! **:data-[variant=destructive]:focus:bg-foreground/10! **:data-[variant=destructive]:text-accent-foreground! **:data-[variant=destructive]:**:text-accent-foreground! z-50 max-h-(--available-height) origin-(--transform-origin) overflow-x-hidden overflow-y-auto outline-none";
const ITEM_CLASS: &str = "focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-2 rounded-sm px-2 py-1.5 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";
const SUB_TRIGGER_CLASS: &str = "focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground rounded-sm px-2 py-1.5 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&_svg]:pointer-events-none [&_svg]:shrink-0";
const CHECKBOX_ITEM_CLASS: &str = "focus:bg-accent focus:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";
const RADIO_ITEM_CLASS: &str = "focus:bg-accent focus:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";
const INDICATOR_CLASS: &str = "absolute right-2 pointer-events-none";

const LONG_PRESS_MS: u32 = 500;

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
/// Renders no element of its own.
#[component]
pub fn ContextMenu(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let state = use_menu_state(open, default_open, on_open_change, true, None);
    use_context_provider(|| state);

    rsx! {
        {children}
    }
}

/// Upstream's portal. The popup is already positioned with `position: fixed`, so this renders its children.
#[component]
pub fn ContextMenuPortal(children: Element) -> Element {
    rsx! {
        {children}
    }
}

/// The area that opens the menu on right click, and on a long press on touch screens.
#[component]
pub fn ContextMenuTrigger(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let state = use_context::<MenuState>();
    let mut attributes = attributes;
    let class = cn!("select-none", take_class(&mut attributes).as_deref());
    let mut press = use_signal(|| 0u64);

    rsx! {
        div {
            "data-slot": "context-menu-trigger",
            "data-popup-open": state.is_open().then_some(""),
            class: class,
            oncontextmenu: move |event| {
                event.prevent_default();
                let point = event.client_coordinates();
                state.open_at(point.x, point.y);
            },
            ontouchstart: move |event| {
                let Some(touch) = event.touches().first().map(|touch| touch.client_coordinates()) else {
                    return;
                };
                let ticket = *press.peek() + 1;
                press.set(ticket);
                spawn(async move {
                    let _ = document::eval(&format!("setTimeout(() => dioxus.send(true), {LONG_PRESS_MS})"))
                        .recv::<bool>()
                        .await;
                    if *press.peek() == ticket && !state.is_open() {
                        state.open_at(touch.x, touch.y);
                    }
                });
            },
            ontouchmove: move |_| {
                let ticket = *press.peek() + 1;
                press.set(ticket);
            },
            ontouchend: move |_| {
                let ticket = *press.peek() + 1;
                press.set(ticket);
            },
            ontouchcancel: move |_| {
                let ticket = *press.peek() + 1;
                press.set(ticket);
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ContextMenuContent(
    #[props(default)] align: ContextMenuAlign,
    #[props(default = 4.0)] align_offset: f64,
    #[props(default = ContextMenuSide::Right)] side: ContextMenuSide,
    #[props(default = 0.0)] side_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(CONTENT_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        MenuPopup {
            slot: "context-menu-content",
            class_name: class,
            side,
            align,
            side_offset,
            align_offset,
            attributes,
            {children}
        }
    }
}

#[component]
pub fn ContextMenuGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "context-menu-group",
            role: "group",
            ..attributes,
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
    let class = cn!(
        "text-muted-foreground px-2 py-1.5 text-xs font-medium data-inset:pl-8",
        take_class(&mut attributes).as_deref()
    );

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
pub fn ContextMenuItem(
    #[props(default)] variant: ContextMenuItemVariant,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    /// Close the menu after the item is clicked.
    #[props(default = true)]
    close_on_click: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(ITEM_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        MenuItem {
            slot: "context-menu-item",
            class_name: class,
            variant,
            inset,
            disabled,
            close_on_click,
            onclick,
            attributes,
            {children}
        }
    }
}

/// Owns the open state of a submenu. Renders no element of its own.
#[component]
pub fn ContextMenuSub(
    open: Option<bool>,
    #[props(default)] default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    rsx! {
        DropdownMenuSub { open, default_open, on_open_change, {children} }
    }
}

#[component]
pub fn ContextMenuSubTrigger(
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(SUB_TRIGGER_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        MenuSubTrigger {
            slot: "context-menu-sub-trigger",
            class_name: class,
            inset,
            disabled,
            attributes,
            {children}
        }
    }
}

/// The popup of a submenu. It opens to the right of its trigger.
#[component]
pub fn ContextMenuSubContent(
    #[props(default)] align: ContextMenuAlign,
    #[props(default = 4.0)] align_offset: f64,
    #[props(default = ContextMenuSide::Right)] side: ContextMenuSide,
    #[props(default = 0.0)] side_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        CONTENT_CLASS,
        "shadow-lg animate-none! relative bg-popover/70 before:pointer-events-none before:absolute before:inset-0 before:-z-1 before:rounded-[inherit] before:backdrop-blur-2xl before:backdrop-saturate-150 **:data-[slot$=-item]:focus:bg-foreground/10 **:data-[slot$=-item]:data-highlighted:bg-foreground/10 **:data-[slot$=-separator]:bg-foreground/5 **:data-[slot$=-trigger]:focus:bg-foreground/10 **:data-[slot$=-trigger]:aria-expanded:bg-foreground/10! **:data-[variant=destructive]:focus:bg-foreground/10! **:data-[variant=destructive]:text-accent-foreground! **:data-[variant=destructive]:**:text-accent-foreground!",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        MenuPopup {
            slot: "context-menu-sub-content",
            class_name: class,
            side,
            align,
            side_offset,
            align_offset,
            attributes,
            {children}
        }
    }
}

/// A menu item with a check indicator. Pass `checked` to control it, or use `default_checked`.
#[component]
pub fn ContextMenuCheckboxItem(
    checked: Option<bool>,
    #[props(default)] default_checked: bool,
    on_checked_change: Option<EventHandler<bool>>,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(default)] close_on_click: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(CHECKBOX_ITEM_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        MenuCheckboxItem {
            slot: "context-menu-checkbox-item",
            class_name: class,
            indicator_class: INDICATOR_CLASS,
            checked,
            default_checked,
            on_checked_change,
            inset,
            disabled,
            close_on_click,
            onclick,
            attributes,
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
    rsx! {
        MenuRadioGroup {
            slot: "context-menu-radio-group",
            value,
            default_value,
            on_value_change,
            attributes,
            {children}
        }
    }
}

/// A menu item that is checked when its `value` equals the group's value.
#[component]
pub fn ContextMenuRadioItem(
    value: String,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(default)] close_on_click: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(RADIO_ITEM_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        MenuRadioItem {
            slot: "context-menu-radio-item",
            class_name: class,
            indicator_class: INDICATOR_CLASS,
            value,
            inset,
            disabled,
            close_on_click,
            onclick,
            attributes,
            {children}
        }
    }
}

#[component]
pub fn ContextMenuSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-border -mx-1 my-1 h-px",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "context-menu-separator",
            role: "separator",
            "aria-orientation": "horizontal",
            class: class,
            ..attributes,
        }
    }
}

#[component]
pub fn ContextMenuShortcut(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "context-menu-shortcut",
            class: class,
            ..attributes,
            {children}
        }
    }
}
