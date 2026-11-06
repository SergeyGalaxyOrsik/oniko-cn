//! Menubar: a horizontal bar of menus with actions, toggles and choices.
//!
//! A port of shadcn/ui's Base UI menubar, built on `DropdownMenu` like upstream, so it needs that
//! component installed. Only one menu is open at a time. While one is open, hovering another trigger
//! opens it instead; `ArrowLeft` and `ArrowRight` move between triggers (and between open menus), and
//! `ArrowDown` on a trigger opens its menu on the first item. Triggers get `relative z-41` so they stay
//! reachable above the layer that catches clicks outside the open menu.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::dropdown_menu::{
    CONTENT_CLASS, DropdownMenu, DropdownMenuSub, ITEM_CLASS,
    LABEL_CLASS, MenuCheckboxItem, MenuItem, MenuPopup, MenuRadioGroup, MenuRadioItem,
    MenuSubTrigger, MenuTrigger, MenubarContext, MenubarMenuContext, SEPARATOR_CLASS,
    SHORTCUT_CLASS, SUB_CONTENT_CLASS, SUB_TRIGGER_CLASS,
};
pub use super::dropdown_menu::{
    DropdownMenuAlign as MenubarAlign, DropdownMenuItemVariant as MenubarItemVariant,
    DropdownMenuSide as MenubarSide,
};

static NEXT_MENU_ID: AtomicUsize = AtomicUsize::new(0);

const CHECKBOX_ITEM_CLASS: &str = "cn-menubar-checkbox-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";
const RADIO_ITEM_CLASS: &str = "cn-menubar-radio-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0";

#[component]
pub fn Menubar(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-menubar flex items-center", take_class(&mut attributes).as_deref());
    let open_menu = use_signal(|| None::<usize>);
    let triggers = use_signal(Vec::new);
    let active = use_signal(|| None::<usize>);
    let pending_focus = use_signal(|| None::<usize>);
    use_context_provider(|| MenubarContext {
        open_menu,
        triggers,
        active,
        pending_focus,
    });

    rsx! {
        div {
            "data-slot": "menubar",
            role: "menubar",
            "aria-orientation": "horizontal",
            "data-orientation": "horizontal",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Upstream's portal. The popup is already positioned with `position: fixed`, so this renders its children.
#[component]
pub fn MenubarPortal(children: Element) -> Element {
    rsx! {
        {children}
    }
}

/// One menu of the bar. It is open while it is the bar's open menu; pass `open` to control it instead.
/// Renders no element of its own.
#[component]
pub fn MenubarMenu(
    open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let bar = use_context::<MenubarContext>();
    let id = use_hook(|| NEXT_MENU_ID.fetch_add(1, Ordering::Relaxed));
    use_context_provider(|| MenubarMenuContext { id });
    let is_open = open.unwrap_or_else(|| (bar.open_menu)() == Some(id));

    rsx! {
        DropdownMenu {
            open: is_open,
            on_open_change: move |next: bool| {
                let mut open_menu = bar.open_menu;
                if next {
                    open_menu.set(Some(id));
                } else if *open_menu.peek() == Some(id) {
                    open_menu.set(None);
                }
                if let Some(handler) = on_open_change {
                    handler.call(next);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn MenubarTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-menubar-trigger flex items-center outline-hidden select-none relative z-41",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        MenuTrigger {
            slot: "menubar-trigger",
            class_name: class,
            attributes,
            {children}
        }
    }
}

#[component]
pub fn MenubarContent(
    #[props(default)] align: MenubarAlign,
    #[props(default = -4.0)] align_offset: f64,
    #[props(default)] side: MenubarSide,
    #[props(default = 8.0)] side_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        CONTENT_CLASS,
        "cn-menubar-content cn-menubar-content-logical cn-menu-target cn-menu-translucent",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        MenuPopup {
            slot: "menubar-content",
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
pub fn MenubarSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        SEPARATOR_CLASS,
        "cn-menubar-separator -mx-1 my-1 h-px",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "menubar-separator",
            role: "separator",
            "aria-orientation": "horizontal",
            class: class,
            ..attributes,
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
    let class = cn!(
        LABEL_CLASS,
        "cn-menubar-label",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "menubar-label",
            "data-inset": inset.then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MenubarItem(
    #[props(default)] variant: MenubarItemVariant,
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
    let class = cn!(
        ITEM_CLASS,
        "cn-menubar-item group/menubar-item",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        MenuItem {
            slot: "menubar-item",
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

#[component]
pub fn MenubarShortcut(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        SHORTCUT_CLASS,
        "cn-menubar-shortcut ml-auto",
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

/// A menu item with a check indicator. Pass `checked` to control it, or use `default_checked`.
#[component]
pub fn MenubarCheckboxItem(
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
            slot: "menubar-checkbox-item",
            class_name: class,
            indicator_class: "cn-menubar-checkbox-item-indicator pointer-events-none absolute flex items-center justify-center",
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
pub fn MenubarRadioGroup(
    value: Option<String>,
    #[props(default)] default_value: String,
    on_value_change: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        MenuRadioGroup {
            slot: "menubar-radio-group",
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
pub fn MenubarRadioItem(
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
            slot: "menubar-radio-item",
            class_name: class,
            indicator_class: "cn-menubar-radio-item-indicator pointer-events-none absolute flex items-center justify-center",
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

/// Owns the open state of a submenu. Renders no element of its own.
#[component]
pub fn MenubarSub(
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
pub fn MenubarSubTrigger(
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        SUB_TRIGGER_CLASS,
        "cn-menubar-sub-trigger",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        MenuSubTrigger {
            slot: "menubar-sub-trigger",
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
pub fn MenubarSubContent(
    #[props(default)] align: MenubarAlign,
    #[props(default = -3.0)] align_offset: f64,
    #[props(default = MenubarSide::Right)] side: MenubarSide,
    #[props(default = 0.0)] side_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        CONTENT_CLASS,
        SUB_CONTENT_CLASS,
        "cn-menubar-sub-content cn-menu-target cn-menu-translucent",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        MenuPopup {
            slot: "menubar-sub-content",
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
