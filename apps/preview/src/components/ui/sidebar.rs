//! Sidebar: a composable, themeable side panel with a header, scrollable content, footer, groups and menus.
//!
//! `SidebarProvider` owns the open state (`default_open`, or `open` with `on_open_change`), remembers it in
//! the `sidebar_state` cookie, toggles it with Ctrl/Cmd+B and tells the parts whether the viewport is narrower
//! than 768px. `Sidebar` is a fixed panel on wider viewports (`side`, `variant`, `collapsible`) and a `Sheet`
//! on narrower ones. Read or change the state anywhere below the provider with `use_sidebar()`.
//!
//! Upstream's `render` prop becomes a `render` callback. It receives the attributes the part computed
//! (event handlers, `class`, `data-*`) and the children, and must spread the attributes on the element it
//! returns, for example `a { href: "#", ..attributes, {children} }`, or on a `CollapsibleTrigger` or
//! `DropdownMenuTrigger` through its `attributes` prop. `SidebarMenuButton` takes `tooltip` as a string.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{Button, ButtonSize, ButtonVariant};
use super::input::Input;
use super::separator::{Separator, SeparatorOrientation};
use super::sheet::{Sheet, SheetContent, SheetDescription, SheetHeader, SheetSide, SheetTitle};
use super::skeleton::Skeleton;
use super::tooltip::{Tooltip, TooltipAlign, TooltipContent, TooltipSide, TooltipTrigger};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const SIDEBAR_COOKIE_NAME: &str = "sidebar_state";
const SIDEBAR_COOKIE_MAX_AGE: u32 = 60 * 60 * 24 * 7;
const SIDEBAR_WIDTH: &str = "16rem";
const SIDEBAR_WIDTH_MOBILE: &str = "18rem";
const SIDEBAR_WIDTH_ICON: &str = "3rem";

const MOBILE_SCRIPT: &str = r#"
const mql = window.matchMedia("(max-width: 767px)");
function onChange() {
  try { dioxus.send(window.innerWidth < 768); }
  catch { mql.removeEventListener("change", onChange); }
}
mql.addEventListener("change", onChange);
dioxus.send(window.innerWidth < 768);
"#;

const SHORTCUT_SCRIPT: &str = r#"
function onKey(event) {
  if (event.key === "b" && (event.metaKey || event.ctrlKey)) {
    event.preventDefault();
    try { dioxus.send(true); }
    catch { window.removeEventListener("keydown", onKey); }
  }
}
window.addEventListener("keydown", onKey);
(window.__onikoSidebarOff = window.__onikoSidebarOff || {})["__ID__"] = () => window.removeEventListener("keydown", onKey);
"#;

/// What a `render` callback receives: the attributes the part computed and its children.
type Render = Callback<(Vec<Attribute>, Element), Element>;

fn attr(name: &'static str, value: impl Into<String>) -> Attribute {
    Attribute::new(name, value.into(), None, false)
}

fn flag(name: &'static str, on: bool) -> Option<Attribute> {
    on.then(|| attr(name, ""))
}

/// Removes the `style` attribute from a spread list and returns it, so a component can add its own.
fn take_style(attributes: &mut Vec<Attribute>) -> Option<String> {
    let mut styles: Vec<String> = Vec::new();
    attributes.retain(|attribute| {
        if attribute.name != "style" || attribute.namespace.is_some() {
            return true;
        }
        if let AttributeValue::Text(text) = &attribute.value {
            styles.push(text.clone());
        }
        false
    });
    (!styles.is_empty()).then(|| styles.join("; "))
}

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// True below 768px. Starts `false` until the browser or webview reports the width.
fn use_is_mobile() -> Signal<bool> {
    let mut is_mobile = use_signal(|| false);
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(MOBILE_SCRIPT);
            while let Ok(value) = eval.recv::<bool>().await {
                is_mobile.set(value);
            }
        });
    });
    is_mobile
}

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
pub enum SidebarState {
    #[default]
    Expanded,
    Collapsed,
}

impl SidebarState {
    /// Value of the `data-state` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expanded => "expanded",
            Self::Collapsed => "collapsed",
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
            Self::Default => "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
            Self::Outline => {
                "bg-background hover:bg-sidebar-accent hover:text-sidebar-accent-foreground shadow-[0_0_0_1px_var(--sidebar-border)] hover:shadow-[0_0_0_1px_var(--sidebar-accent)]"
            }
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
            Self::Default => "h-8 text-sm",
            Self::Sm => "h-7 text-xs",
            Self::Lg => "h-12 text-sm group-data-[collapsible=icon]:p-0!",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarMenuSubButtonSize {
    Sm,
    #[default]
    Md,
}

impl SidebarMenuSubButtonSize {
    /// Value of the `data-size` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// Shared state of the sidebars below a SidebarProvider, like upstream's `useSidebar()`.
#[derive(Clone, Copy)]
pub struct SidebarContext {
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    open_mobile: Signal<bool>,
    is_mobile: Signal<bool>,
}

impl SidebarContext {
    pub fn open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    pub fn state(self) -> SidebarState {
        if self.open() {
            SidebarState::Expanded
        } else {
            SidebarState::Collapsed
        }
    }

    pub fn set_open(self, value: bool) {
        if let Some(handler) = self.on_open_change {
            handler.call(value);
        }
        if self.controlled.peek().is_none() {
            let mut internal = self.internal;
            internal.set(value);
        }
        let _ = document::eval(&format!(
            "document.cookie = \"{SIDEBAR_COOKIE_NAME}={value}; path=/; max-age={SIDEBAR_COOKIE_MAX_AGE}\""
        ));
    }

    pub fn open_mobile(self) -> bool {
        (self.open_mobile)()
    }

    pub fn set_open_mobile(self, value: bool) {
        let mut open_mobile = self.open_mobile;
        open_mobile.set(value);
    }

    /// True below 768px, where the sidebar is shown in a sheet.
    pub fn is_mobile(self) -> bool {
        (self.is_mobile)()
    }

    /// Opens or closes the sheet on narrow viewports and the sidebar on wide ones.
    pub fn toggle(self) {
        if self.is_mobile() {
            self.set_open_mobile(!self.open_mobile());
        } else {
            self.set_open(!self.open());
        }
    }
}

/// Reads the sidebar state. Must be called inside a SidebarProvider.
pub fn use_sidebar() -> SidebarContext {
    use_context::<SidebarContext>()
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
#[component]
pub fn SidebarProvider(
    #[props(default = true)] default_open: bool,
    open: Option<bool>,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let open_mobile = use_signal(|| false);
    let is_mobile = use_is_mobile();
    let context = use_context_provider(|| SidebarContext {
        internal,
        controlled,
        on_open_change,
        open_mobile,
        is_mobile,
    });

    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(&SHORTCUT_SCRIPT.replace("__ID__", &id.to_string()));
            while eval.recv::<bool>().await.is_ok() {
                context.toggle();
            }
        });
    });
    use_drop(move || {
        let _ = document::eval(&format!(
            "window.__onikoSidebarOff?.[\"{id}\"]?.(); delete window.__onikoSidebarOff?.[\"{id}\"];"
        ));
    });

    let mut attributes = attributes;
    let class = cn!(
        "group/sidebar-wrapper flex min-h-svh w-full has-data-[variant=inset]:bg-sidebar",
        take_class(&mut attributes).as_deref()
    );
    let mut style =
        format!("--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON}");
    if let Some(extra) = take_style(&mut attributes) {
        style = format!("{style}; {extra}");
    }

    rsx! {
        div {
            "data-slot": "sidebar-wrapper",
            style: style,
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn Sidebar(
    #[props(default)] side: SidebarSide,
    #[props(default)] variant: SidebarVariant,
    #[props(default)] collapsible: SidebarCollapsible,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_sidebar();
    let state = context.state();
    let mut attributes = attributes;
    let class = take_class(&mut attributes);

    if collapsible == SidebarCollapsible::None {
        return rsx! {
            div {
                "data-slot": "sidebar",
                class: cn!(
                    "flex h-full w-(--sidebar-width) flex-col bg-sidebar text-sidebar-foreground",
                    class.as_deref()
                ),
                ..attributes,
                {children}
            }
        };
    }

    if context.is_mobile() {
        return rsx! {
            Sheet {
                open: context.open_mobile(),
                on_open_change: move |value| context.set_open_mobile(value),
                SheetContent {
                    side: match side {
                        SidebarSide::Left => SheetSide::Left,
                        SidebarSide::Right => SheetSide::Right,
                    },
                    show_close_button: false,
                    class: "w-(--sidebar-width) bg-sidebar p-0 text-sidebar-foreground [&>button]:hidden",
                    style: "--sidebar-width: {SIDEBAR_WIDTH_MOBILE}",
                    "data-sidebar": "sidebar",
                    "data-slot": "sidebar",
                    "data-mobile": "true",
                    SheetHeader { class: "sr-only",
                        SheetTitle { "Sidebar" }
                        SheetDescription { "Displays the mobile sidebar." }
                    }
                    div { class: "flex h-full w-full flex-col", {children} }
                }
            }
        };
    }

    let floating = matches!(variant, SidebarVariant::Floating | SidebarVariant::Inset);
    let collapsed = state == SidebarState::Collapsed;

    rsx! {
        div {
            class: "group peer hidden text-sidebar-foreground md:block",
            "data-state": state.as_str(),
            "data-collapsible": if collapsed { collapsible.as_str() } else { "" },
            "data-variant": variant.as_str(),
            "data-side": side.as_str(),
            "data-slot": "sidebar",
            div {
                "data-slot": "sidebar-gap",
                class: cn!(
                    "transition-[width] duration-200 ease-linear relative w-(--sidebar-width) bg-transparent",
                    "group-data-[collapsible=offcanvas]:w-0",
                    "group-data-[side=right]:rotate-180",
                    if floating {
                        "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4)))]"
                    } else {
                        "group-data-[collapsible=icon]:w-(--sidebar-width-icon)"
                    }
                ),
            }
            div {
                "data-slot": "sidebar-container",
                "data-side": side.as_str(),
                class: cn!(
                    "fixed inset-y-0 z-10 hidden h-svh w-(--sidebar-width) transition-[left,right,width] duration-200 ease-linear data-[side=left]:left-0 data-[side=left]:group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)] data-[side=right]:right-0 data-[side=right]:group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)] md:flex",
                    if floating {
                        "p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]"
                    } else {
                        "group-data-[collapsible=icon]:w-(--sidebar-width-icon) group-data-[side=left]:border-r group-data-[side=right]:border-l"
                    },
                    class.as_deref()
                ),
                ..attributes,
                div {
                    "data-sidebar": "sidebar",
                    "data-slot": "sidebar-inner",
                    class: "bg-sidebar group-data-[variant=floating]:ring-sidebar-border group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:shadow-sm group-data-[variant=floating]:ring-1 flex size-full flex-col",
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
    let context = use_sidebar();
    let mut attributes = attributes;
    let class = cn!("", take_class(&mut attributes).as_deref());
    let mut own = vec![
        attr("data-sidebar", "trigger"),
        attr("data-slot", "sidebar-trigger"),
        attr("class", class),
    ];
    own.extend(attributes);

    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::IconSm,
            attributes: own,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                context.toggle();
            },
            svg {
                class: "rtl:rotate-180",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                rect {
                    width: "18",
                    height: "18",
                    x: "3",
                    y: "3",
                    rx: "2",
                }
                path { d: "M9 3v18" }
            }
            span { class: "sr-only", "Toggle Sidebar" }
        }
    }
}

/// Strip along the edge of the sidebar that toggles it when clicked.
#[component]
pub fn SidebarRail(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let context = use_sidebar();
    let mut attributes = attributes;
    let class = cn!(
        "hover:after:bg-sidebar-border absolute inset-y-0 z-20 hidden w-4 transition-all ease-linear group-data-[side=left]:-right-4 group-data-[side=right]:left-0 after:absolute after:inset-y-0 after:start-1/2 after:w-[2px] sm:flex ltr:-translate-x-1/2 rtl:-translate-x-1/2",
        "in-data-[side=left]:cursor-w-resize in-data-[side=right]:cursor-e-resize",
        "[[data-side=left][data-state=collapsed]_&]:cursor-e-resize [[data-side=right][data-state=collapsed]_&]:cursor-w-resize",
        "group-data-[collapsible=offcanvas]:translate-x-0 group-data-[collapsible=offcanvas]:after:left-full hover:group-data-[collapsible=offcanvas]:bg-sidebar",
        "[[data-side=left][data-collapsible=offcanvas]_&]:-right-2",
        "[[data-side=right][data-collapsible=offcanvas]_&]:-left-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        button {
            "data-sidebar": "rail",
            "data-slot": "sidebar-rail",
            "aria-label": "Toggle Sidebar",
            tabindex: "-1",
            title: "Toggle Sidebar",
            r#type: "button",
            class: class,
            onclick: move |event| {
                context.toggle();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
        }
    }
}

/// Main content next to a sidebar. With `variant: SidebarVariant::Inset` it becomes a rounded, inset panel.
#[component]
pub fn SidebarInset(
    #[props(extends = GlobalAttributes, extends = main)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-background md:peer-data-[variant=inset]:m-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow-sm md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2 relative flex w-full flex-1 flex-col",
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
pub fn SidebarInput(
    onclick: Option<EventHandler<MouseEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-background h-8 w-full shadow-none",
        take_class(&mut attributes).as_deref()
    );
    let mut own = vec![
        attr("data-slot", "sidebar-input"),
        attr("data-sidebar", "input"),
        attr("class", class),
    ];
    own.extend(attributes);

    rsx! {
        Input {
            onclick,
            oninput,
            onchange,
            onfocus,
            onblur,
            onkeydown,
            attributes: own,
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
        "gap-2 p-2 flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-header",
            "data-sidebar": "header",
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
        "gap-2 p-2 flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-footer",
            "data-sidebar": "footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarSeparator(
    #[props(default)] orientation: SeparatorOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-sidebar-border mx-2 w-auto",
        take_class(&mut attributes).as_deref()
    );
    let mut own = vec![
        attr("data-slot", "sidebar-separator"),
        attr("data-sidebar", "separator"),
        attr("class", class),
    ];
    own.extend(attributes);

    rsx! {
        Separator { orientation, attributes: own }
    }
}

#[component]
pub fn SidebarContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "no-scrollbar gap-2 flex min-h-0 flex-1 flex-col overflow-auto group-data-[collapsible=icon]:overflow-hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-content",
            "data-sidebar": "content",
            class: class,
            ..attributes,
            {children}
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
        "p-2 relative flex w-full min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-group",
            "data-sidebar": "group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Heading of a group. Pass `render` to render another element, for example a `CollapsibleTrigger`.
#[component]
pub fn SidebarGroupLabel(
    render: Option<Render>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sidebar-foreground/70 ring-sidebar-ring h-8 rounded-md px-2 text-xs font-medium transition-[margin,opacity] duration-200 ease-linear group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0 focus-visible:ring-2 [&>svg]:size-4 flex shrink-0 items-center outline-hidden [&>svg]:shrink-0",
        take_class(&mut attributes).as_deref()
    );
    let mut own = vec![
        attr("class", class),
        attr("data-slot", "sidebar-group-label"),
        attr("data-sidebar", "group-label"),
    ];
    own.extend(attributes);

    match render {
        Some(render) => render.call((own, children)),
        None => rsx! {
            div { ..own, {children} }
        },
    }
}

/// Button in the corner of a group, such as an "add" button. Pass `render` to render another element.
#[component]
pub fn SidebarGroupAction(
    render: Option<Render>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground absolute top-3.5 right-3 w-5 rounded-md p-0 focus-visible:ring-2 [&>svg]:size-4 flex aspect-square items-center justify-center outline-hidden transition-transform group-data-[collapsible=icon]:hidden after:absolute after:-inset-2 md:after:hidden [&>svg]:shrink-0",
        take_class(&mut attributes).as_deref()
    );
    let mut own = vec![
        attr("class", class),
        attr("data-slot", "sidebar-group-action"),
        attr("data-sidebar", "group-action"),
    ];
    if let Some(handler) = onclick {
        own.push(dioxus::html::events::onclick(move |event| {
            handler.call(event);
        }));
    }
    own.extend(attributes);

    match render {
        Some(render) => render.call((own, children)),
        None => rsx! {
            button { r#type: "button", ..own, {children} }
        },
    }
}

#[component]
pub fn SidebarGroupContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("text-sm w-full", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "sidebar-group-content",
            "data-sidebar": "group-content",
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
        "gap-1 flex w-full min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        ul {
            "data-slot": "sidebar-menu",
            "data-sidebar": "menu",
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
            "data-sidebar": "menu-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

const MENU_BUTTON_BASE: &str = "ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground active:bg-sidebar-accent active:text-sidebar-accent-foreground data-active:bg-sidebar-accent data-active:text-sidebar-accent-foreground data-open:hover:bg-sidebar-accent data-open:hover:text-sidebar-accent-foreground gap-2 rounded-md p-2 text-left text-sm transition-[width,height,padding] group-has-data-[sidebar=menu-action]/menu-item:pr-8 group-data-[collapsible=icon]:size-8! group-data-[collapsible=icon]:p-2! focus-visible:ring-2 data-active:font-medium peer/menu-button group/menu-button flex w-full items-center overflow-hidden outline-hidden disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg]:size-4 [&_svg]:shrink-0 [&>span:last-child]:truncate";

/// Classes of a menu button, for another element that should look like one.
pub fn sidebar_menu_button_variants(
    variant: SidebarMenuButtonVariant,
    size: SidebarMenuButtonSize,
    class: Option<&str>,
) -> String {
    cn!(MENU_BUTTON_BASE, variant.class(), size.class(), class)
}

/// Item of a SidebarMenu. Set `is_active` on the current page's item. With `tooltip`, a tooltip shows the
/// label to the right while the sidebar is collapsed to icons. Pass `render` to render a link, a
/// `CollapsibleTrigger` or a `DropdownMenuTrigger` instead of a `button`.
#[component]
pub fn SidebarMenuButton(
    render: Option<Render>,
    #[props(default)] is_active: bool,
    #[props(default)] variant: SidebarMenuButtonVariant,
    #[props(default)] size: SidebarMenuButtonSize,
    tooltip: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_sidebar();
    let mut attributes = attributes;
    let class = sidebar_menu_button_variants(variant, size, take_class(&mut attributes).as_deref());
    let mut own = vec![
        attr("class", class),
        attr("data-slot", "sidebar-menu-button"),
        attr("data-sidebar", "menu-button"),
        attr("data-size", size.as_str()),
    ];
    own.extend(flag("data-active", is_active));
    if let Some(handler) = onclick {
        own.push(dioxus::html::events::onclick(move |event| {
            handler.call(event);
        }));
    }
    own.extend(attributes);

    let element = move |(attributes, children): (Vec<Attribute>, Element)| match render {
        Some(render) => render.call((attributes, children)),
        None => rsx! {
            button { r#type: "button", ..attributes, {children} }
        },
    };

    let Some(tooltip) = tooltip else {
        return element((own, children));
    };

    let hidden = context.state() != SidebarState::Collapsed || context.is_mobile();
    rsx! {
        Tooltip {
            TooltipTrigger { render: Callback::new(element), attributes: own, {children} }
            TooltipContent {
                side: TooltipSide::Right,
                align: TooltipAlign::Center,
                hidden,
                "{tooltip}"
            }
        }
    }
}

/// Button on the right of a menu item, such as a "more" menu. With `show_on_hover` it appears when the item
/// is hovered or focused. Pass `render` to render another element, for example a `DropdownMenuTrigger`.
#[component]
pub fn SidebarMenuAction(
    render: Option<Render>,
    #[props(default)] show_on_hover: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground peer-hover/menu-button:text-sidebar-accent-foreground absolute top-1.5 right-1 aspect-square w-5 rounded-md p-0 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5 peer-data-[size=sm]/menu-button:top-1 focus-visible:ring-2 [&>svg]:size-4 flex items-center justify-center outline-hidden transition-transform group-data-[collapsible=icon]:hidden after:absolute after:-inset-2 md:after:hidden [&>svg]:shrink-0",
        show_on_hover.then_some(
            "group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 peer-data-active/menu-button:text-sidebar-accent-foreground aria-expanded:opacity-100 md:opacity-0"
        ),
        take_class(&mut attributes).as_deref()
    );
    let mut own = vec![
        attr("class", class),
        attr("data-slot", "sidebar-menu-action"),
        attr("data-sidebar", "menu-action"),
    ];
    if let Some(handler) = onclick {
        own.push(dioxus::html::events::onclick(move |event| {
            handler.call(event);
        }));
    }
    own.extend(attributes);

    match render {
        Some(render) => render.call((own, children)),
        None => rsx! {
            button { r#type: "button", ..own, {children} }
        },
    }
}

#[component]
pub fn SidebarMenuBadge(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sidebar-foreground peer-hover/menu-button:text-sidebar-accent-foreground peer-data-active/menu-button:text-sidebar-accent-foreground pointer-events-none absolute right-1 flex h-5 min-w-5 rounded-md px-1 text-xs font-medium peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5 peer-data-[size=sm]/menu-button:top-1 items-center justify-center tabular-nums select-none group-data-[collapsible=icon]:hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-menu-badge",
            "data-sidebar": "menu-badge",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Placeholder for a menu item while it loads. Its text bar gets a width between 50% and 89%.
#[component]
pub fn SidebarMenuSkeleton(
    #[props(default)] show_icon: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    // A counter stands in for Math.random(): wasm has neither a clock nor an RNG without a JS crate.
    let width = use_hook(|| {
        static SEED: AtomicUsize = AtomicUsize::new(0);
        50 + (SEED.fetch_add(1, Ordering::Relaxed) * 17 + 7) % 40
    });
    let mut attributes = attributes;
    let class = cn!(
        "h-8 gap-2 rounded-md px-2 flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sidebar-menu-skeleton",
            "data-sidebar": "menu-skeleton",
            class: class,
            ..attributes,
            if show_icon {
                Skeleton {
                    class: "size-4 rounded-md",
                    "data-sidebar": "menu-skeleton-icon",
                }
            }
            Skeleton {
                class: "h-4 max-w-(--skeleton-width) flex-1",
                "data-sidebar": "menu-skeleton-text",
                style: "--skeleton-width: {width}%",
            }
        }
    }
}

#[component]
pub fn SidebarMenuSub(
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "border-sidebar-border mx-3.5 translate-x-px gap-1 border-l px-2.5 py-0.5 group-data-[collapsible=icon]:hidden flex min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        ul {
            "data-slot": "sidebar-menu-sub",
            "data-sidebar": "menu-sub",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SidebarMenuSubItem(
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "group/menu-sub-item relative",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        li {
            "data-slot": "sidebar-menu-sub-item",
            "data-sidebar": "menu-sub-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Link of a sub-menu. Pass `render` to put the attributes on your own element, such as a router link.
#[component]
pub fn SidebarMenuSubButton(
    render: Option<Render>,
    #[props(default)] size: SidebarMenuSubButtonSize,
    #[props(default)] is_active: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground active:bg-sidebar-accent active:text-sidebar-accent-foreground [&>svg]:text-sidebar-accent-foreground data-active:bg-sidebar-accent data-active:text-sidebar-accent-foreground h-7 gap-2 rounded-md px-2 focus-visible:ring-2 data-[size=md]:text-sm data-[size=sm]:text-xs [&>svg]:size-4 flex min-w-0 -translate-x-px items-center overflow-hidden outline-hidden group-data-[collapsible=icon]:hidden disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&>span:last-child]:truncate [&>svg]:shrink-0",
        take_class(&mut attributes).as_deref()
    );
    let mut own = vec![
        attr("class", class),
        attr("data-slot", "sidebar-menu-sub-button"),
        attr("data-sidebar", "menu-sub-button"),
        attr("data-size", size.as_str()),
    ];
    own.extend(flag("data-active", is_active));
    if let Some(handler) = onclick {
        own.push(dioxus::html::events::onclick(move |event| {
            handler.call(event);
        }));
    }
    own.extend(attributes);

    match render {
        Some(render) => render.call((own, children)),
        None => rsx! {
            a { ..own, {children} }
        },
    }
}
