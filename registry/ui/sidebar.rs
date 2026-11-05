//! Sidebar: a side panel with a header, scrollable content, footer and menus, that can collapse off-canvas or to icons.
//!
//! Parts: SidebarProvider, Sidebar, SidebarTrigger, SidebarRail, SidebarInset, SidebarHeader, SidebarContent,
//! SidebarFooter, SidebarSeparator, SidebarGroup, SidebarGroupLabel, SidebarGroupContent, SidebarMenu,
//! SidebarMenuItem, SidebarMenuButton. Use `use_sidebar()` inside a SidebarProvider to read or toggle the state.
//!
//! Not included: the mobile sheet (the sidebar is shown at every width), cookie persistence of the open state,
//! the Cmd/Ctrl+B shortcut, the `render` prop for swapping a menu button's element, and the group action,
//! menu action, badge, sub-menu, skeleton and input parts.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

impl SidebarSide {
    /// Value of the `data-side` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    /// Value of the `data-variant` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sidebar => "sidebar",
            Self::Floating => "floating",
            Self::Inset => "inset",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarCollapsible {
    /// Slides out of view when collapsed.
    #[default]
    Offcanvas,
    /// Shrinks to a narrow strip of icons when collapsed.
    Icon,
    /// Never collapses.
    None,
}

impl SidebarCollapsible {
    /// Value of the `data-collapsible` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offcanvas => "offcanvas",
            Self::Icon => "icon",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

impl SidebarMenuButtonVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-sidebar-menu-button-variant-default",
            Self::Outline => "cn-sidebar-menu-button-variant-outline",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl SidebarMenuButtonSize {
    /// Value of the `data-size` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Lg => "lg",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-sidebar-menu-button-size-default",
            Self::Sm => "cn-sidebar-menu-button-size-sm",
            Self::Lg => "cn-sidebar-menu-button-size-lg",
        }
    }
}

/// Shared open state of the sidebars below a SidebarProvider.
#[derive(Clone, Copy)]
pub struct SidebarContext {
    open: Signal<bool>,
}

impl SidebarContext {
    pub fn is_open(self) -> bool {
        (self.open)()
    }

    pub fn toggle(self) {
        let mut open = self.open;
        let next = !open();
        open.set(next);
    }
}

/// Reads the sidebar state. Must be called inside a SidebarProvider.
pub fn use_sidebar() -> SidebarContext {
    use_context::<SidebarContext>()
}

/// Provides the open state to the sidebar parts below it. Pass `open` to control the state from outside.
#[component]
pub fn SidebarProvider(
    #[props(default = true)] default_open: bool,
    open: Option<Signal<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let internal = use_signal(|| default_open);
    let open = open.unwrap_or(internal);
    use_context_provider(|| SidebarContext { open });

    let mut attributes = attributes;
    let class = cn!(
        "group/sidebar-wrapper flex min-h-svh w-full",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-wrapper",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Returns the classes of the gap (which reserves room in the page), the panel and its position.
fn sidebar_layout(
    side: SidebarSide,
    collapsible: SidebarCollapsible,
    open: bool,
) -> (&'static str, &'static str, &'static str) {
    let hidden = collapsible == SidebarCollapsible::Offcanvas && !open;
    let icon = collapsible == SidebarCollapsible::Icon && !open;
    let width = if icon { "w-12" } else { "w-64" };
    let gap = if hidden { "w-0" } else { width };
    let position = match (side, hidden) {
        (SidebarSide::Left, false) => "left-0",
        (SidebarSide::Left, true) => "-left-64",
        (SidebarSide::Right, false) => "right-0",
        (SidebarSide::Right, true) => "-right-64",
    };
    (gap, width, position)
}

#[component]
pub fn Sidebar(
    #[props(default)] side: SidebarSide,
    #[props(default)] variant: SidebarVariant,
    #[props(default)] collapsible: SidebarCollapsible,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let open = use_sidebar().is_open();
    let state = if open { "expanded" } else { "collapsed" };
    let collapsed_attr =
        (!open && collapsible != SidebarCollapsible::None).then_some(collapsible.as_str());
    let (gap, width, position) = sidebar_layout(side, collapsible, open);
    let surface = match (variant, side) {
        (SidebarVariant::Sidebar, SidebarSide::Left) => "border-r border-sidebar-border",
        (SidebarVariant::Sidebar, SidebarSide::Right) => "border-l border-sidebar-border",
        (SidebarVariant::Floating, _) => "p-2",
        (SidebarVariant::Inset, _) => "",
    };

    let mut attributes = attributes;
    let class = cn!(
        "group peer relative block",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar",
            "data-side": side.as_str(),
            "data-variant": variant.as_str(),
            "data-collapsible": collapsed_attr,
            "data-state": state,
            class: class,
            ..attributes,
            div {
                "data-slot": "sidebar-gap",
                class: cn!("cn-sidebar-gap relative h-svh bg-transparent", gap),
            }
            div {
                "data-slot": "sidebar-container",
                class: cn!(
                    "fixed inset-y-0 z-10 flex h-svh transition-[left,right,width] duration-200 ease-linear",
                    width,
                    position,
                    surface
                ),
                div {
                    "data-slot": "sidebar-inner",
                    class: "cn-sidebar-inner flex h-full w-full flex-col text-sidebar-foreground",
                    {children}
                }
            }
        }
    }
}

/// Button that opens and closes the sidebar.
#[component]
pub fn SidebarTrigger(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let sidebar = use_sidebar();
    let mut attributes = attributes;
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        take_class(&mut attributes).as_deref(),
    );

    rsx! {
        button {
            "data-slot": "sidebar-trigger",
            "type": "button",
            "aria-label": "Toggle sidebar",
            class: class,
            onclick: move |event| {
                sidebar.toggle();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            svg {
                "data-slot": "sidebar-trigger-icon",
                class: "size-4 shrink-0",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                rect { width: "18", height: "18", x: "3", y: "3", rx: "2" }
                path { d: "M9 3v18" }
            }
        }
    }
}

/// Strip along the edge of the sidebar that toggles it when clicked.
#[component]
pub fn SidebarRail(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let sidebar = use_sidebar();
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-rail absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 transition-all ease-linear after:absolute after:inset-y-0 after:left-1/2 after:w-0.5 group-data-[side=left]:-right-4 group-data-[side=right]:left-0 hover:group-data-[collapsible=offcanvas]:bg-sidebar sm:flex",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "sidebar-rail",
            "type": "button",
            "aria-label": "Toggle sidebar",
            title: "Toggle sidebar",
            class: class,
            onclick: move |event| {
                sidebar.toggle();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
        }
    }
}

/// Main content next to a sidebar. Use it with `variant: SidebarVariant::Inset` to get a rounded, inset panel.
#[component]
pub fn SidebarInset(
    #[props(extends = GlobalAttributes, extends = main)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-inset relative flex min-h-svh w-full min-w-0 flex-1 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        main {
            "data-slot": "sidebar-inset",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-header flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-content flex min-h-0 flex-1 flex-col overflow-auto group-data-[collapsible=icon]:overflow-hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-footer flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-separator h-px w-auto shrink-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-separator",
            role: "separator",
            "aria-orientation": "horizontal",
            class: class,
            ..attributes,
        }
    }
}

#[component]
pub fn SidebarGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-group relative flex w-full min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarGroupLabel(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-group-label flex shrink-0 items-center outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-group-label",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarGroupContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-group-content w-full",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-group-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarMenu(
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-menu flex w-full min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        ul {
            "data-slot": "sidebar-menu",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarMenuItem(
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "group/menu-item relative",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        li {
            "data-slot": "sidebar-menu-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Item of a SidebarMenu. Set `is_active` on the current page's item.
#[component]
pub fn SidebarMenuButton(
    #[props(default)] variant: SidebarMenuButtonVariant,
    #[props(default)] size: SidebarMenuButtonSize,
    #[props(default)] is_active: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-sidebar-menu-button peer/menu-button flex w-full items-center overflow-hidden text-left outline-none disabled:pointer-events-none disabled:opacity-50 [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0",
        variant.class(),
        size.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-slot": "sidebar-menu-button",
            "data-size": size.as_str(),
            "data-active": is_active.then_some("true"),
            "type": "button",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
