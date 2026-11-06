//! DropdownMenu: a menu of actions, toggles and choices that opens from a trigger.
//!
//! A port of shadcn/ui's Base UI dropdown menu. Items are real `div`s with `role="menuitem"` that take DOM
//! focus while highlighted (`data-highlighted`), so `Enter` and `Space` activate them and the styles'
//! `focus:` variants apply. `ArrowUp`/`ArrowDown` loop through the enabled items, `Home` and `End` jump,
//! typing a character jumps to the next item whose text starts with it, `Escape` closes the menu and returns
//! focus to the trigger, and a click outside closes it. Submenus open on hover, click, `Enter` or the
//! inline-end arrow and close on the inline-start arrow.
//!
//! The popup is positioned from `side`, `align`, `side_offset` and `align_offset` with `position: fixed`
//! and the trigger's `get_client_rect`. It flips to the opposite side when the preferred one has no room
//! and shifts along the cross axis to stay inside the window, but it is not repositioned after it opens.
//! It is not portalled: `DropdownMenuPortal` only renders its children. Fixed positioning keeps the popup
//! out of the way of `overflow: hidden` ancestors.
//!
//! The `Menu*` components, `MenubarContext` and the class constants at the top are shared with
//! `ContextMenu` and `Menubar`; use the `DropdownMenu*` parts in your own code.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

pub const CONTENT_CLASS: &str = "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-md p-1 shadow-md ring-1 duration-100 data-[side=inline-start]:slide-in-from-right-2 data-[side=inline-end]:slide-in-from-left-2 animate-none! relative bg-popover/70 before:pointer-events-none before:absolute before:inset-0 before:-z-1 before:rounded-[inherit] before:backdrop-blur-2xl before:backdrop-saturate-150 **:data-[slot$=-item]:focus:bg-foreground/10 **:data-[slot$=-item]:data-highlighted:bg-foreground/10 **:data-[slot$=-separator]:bg-foreground/5 **:data-[slot$=-trigger]:focus:bg-foreground/10 **:data-[slot$=-trigger]:aria-expanded:bg-foreground/10! **:data-[variant=destructive]:focus:bg-foreground/10! **:data-[variant=destructive]:text-accent-foreground! **:data-[variant=destructive]:**:text-accent-foreground! z-50 max-h-(--available-height) w-(--anchor-width) origin-(--transform-origin) overflow-x-hidden overflow-y-auto outline-none data-closed:overflow-hidden";
pub const SUB_CONTENT_CLASS: &str = "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-md p-1 shadow-lg ring-1 duration-100 animate-none! relative bg-popover/70 before:pointer-events-none before:absolute before:inset-0 before:-z-1 before:rounded-[inherit] before:backdrop-blur-2xl before:backdrop-saturate-150 **:data-[slot$=-item]:focus:bg-foreground/10 **:data-[slot$=-item]:data-highlighted:bg-foreground/10 **:data-[slot$=-separator]:bg-foreground/5 **:data-[slot$=-trigger]:focus:bg-foreground/10 **:data-[slot$=-trigger]:aria-expanded:bg-foreground/10! **:data-[variant=destructive]:focus:bg-foreground/10! **:data-[variant=destructive]:text-accent-foreground! **:data-[variant=destructive]:**:text-accent-foreground! w-auto";
pub const LABEL_CLASS: &str =
    "text-muted-foreground px-2 py-1.5 text-xs font-medium data-inset:pl-8";
pub const ITEM_CLASS: &str = "focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-2 rounded-sm px-2 py-1.5 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";
pub const SUB_TRIGGER_CLASS: &str = "focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-2 rounded-sm px-2 py-1.5 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none data-popup-open:bg-accent data-popup-open:text-accent-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0";
pub const CHECKBOX_ITEM_CLASS: &str = "focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";
pub const RADIO_ITEM_CLASS: &str = "focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";
pub const INDICATOR_CLASS: &str =
    "absolute right-2 flex items-center justify-center pointer-events-none";
pub const SEPARATOR_CLASS: &str = "bg-border -mx-1 my-1 h-px";
pub const SHORTCUT_CLASS: &str = "text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest";

/// How long a closed popup stays mounted so its exit animation can play.
const CLOSE_MS: u32 = 130;
const SUB_OPEN_DELAY_MS: u32 = 100;
const SUB_CLOSE_DELAY_MS: u32 = 150;
const COLLISION_PADDING: f64 = 5.0;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

fn next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

async fn sleep_ms(ms: u32) {
    let _ = document::eval(&format!("setTimeout(() => dioxus.send(true), {ms})"))
        .recv::<bool>()
        .await;
}

async fn next_frames() {
    let _ = document::eval(
        "requestAnimationFrame(() => requestAnimationFrame(() => dioxus.send(true)))",
    )
    .recv::<bool>()
    .await;
}

async fn viewport() -> (f64, f64) {
    let mut eval = document::eval("dioxus.send([window.innerWidth, window.innerHeight])");
    match eval.recv::<Vec<f64>>().await {
        Ok(size) if size.len() == 2 => (size[0], size[1]),
        _ => (1024.0, 768.0),
    }
}

fn click_element(id: String) {
    spawn(async move {
        let _ = document::eval(&format!("document.getElementById('{id}')?.click()")).await;
    });
}

fn focus_node(node: Option<Rc<MountedData>>) {
    if let Some(node) = node {
        spawn(async move {
            let _ = node.set_focus(true).await;
        });
    }
}

/// Moves focus between the enabled items of one popup. `action` is `first`, `last`, `next` or `prev`.
fn navigate(popup_id: String, action: &'static str) {
    spawn(async move {
        let script = format!(
            r#"
            const popup = document.getElementById('{popup_id}');
            const items = popup ? [...popup.querySelectorAll('[role^="menuitem"]')]
                .filter((e) => e.closest('[role="menu"]') === popup && !e.hasAttribute('data-disabled')) : [];
            if (items.length) {{
                const current = items.indexOf(document.activeElement);
                const last = items.length - 1;
                const next = {{
                    first: 0,
                    last,
                    next: current + 1 > last ? 0 : current + 1,
                    prev: current <= 0 ? last : current - 1,
                }}["{action}"];
                items[next].focus();
                items[next].scrollIntoView({{ block: 'nearest' }});
            }}
            "#
        );
        let _ = document::eval(&script).await;
    });
}

/// Focuses the next enabled item whose text starts with what was typed in the last half second.
fn typeahead(popup_id: String, key: String) {
    spawn(async move {
        let key = key
            .to_lowercase()
            .replace('\\', "\\\\")
            .replace('\'', "\\'");
        let script = format!(
            r#"
            const popup = document.getElementById('{popup_id}');
            if (popup) {{
                const now = Date.now();
                const state = popup.__typeahead && now - popup.__typeahead.at < 500 ? popup.__typeahead : {{ text: '' }};
                state.text += '{key}';
                state.at = now;
                popup.__typeahead = state;
                const items = [...popup.querySelectorAll('[role^="menuitem"]')]
                    .filter((e) => e.closest('[role="menu"]') === popup && !e.hasAttribute('data-disabled'));
                const current = items.indexOf(document.activeElement);
                const repeated = state.text.split('').every((c) => c === state.text[0]);
                const needle = repeated ? state.text[0] : state.text;
                const order = [...items.slice(repeated ? current + 1 : 0), ...items.slice(0, repeated ? current + 1 : 0)];
                const match = order.find((e) => e.textContent.trim().toLowerCase().startsWith(needle));
                if (match) {{
                    match.focus();
                    match.scrollIntoView({{ block: 'nearest' }});
                }}
            }}
            "#
        );
        let _ = document::eval(&script).await;
    });
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownMenuSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
    InlineStart,
    InlineEnd,
}

impl DropdownMenuSide {
    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::InlineStart => "inline-start",
            Self::InlineEnd => "inline-end",
        }
    }

    fn physical(self, rtl: bool) -> Self {
        match (self, rtl) {
            (Self::InlineStart, false) | (Self::InlineEnd, true) => Self::Left,
            (Self::InlineEnd, false) | (Self::InlineStart, true) => Self::Right,
            (side, _) => side,
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::InlineStart => Self::InlineEnd,
            Self::InlineEnd => Self::InlineStart,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownMenuAlign {
    #[default]
    Start,
    Center,
    End,
}

impl DropdownMenuAlign {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[derive(Clone, Copy)]
struct Request {
    side: DropdownMenuSide,
    align: DropdownMenuAlign,
    side_offset: f64,
    align_offset: f64,
    rtl: bool,
}

#[derive(Clone, Copy, PartialEq)]
struct Placement {
    left: f64,
    top: f64,
    side: DropdownMenuSide,
    align: DropdownMenuAlign,
    available_height: f64,
    origin_x: f64,
    origin_y: f64,
}

/// Space between the anchor and the window edge on `side`, after the collision padding.
fn space_on(side: DropdownMenuSide, anchor: Rect, viewport: (f64, f64)) -> f64 {
    match side {
        DropdownMenuSide::Top => anchor.y - COLLISION_PADDING,
        DropdownMenuSide::Bottom => viewport.1 - (anchor.y + anchor.h) - COLLISION_PADDING,
        DropdownMenuSide::Left => anchor.x - COLLISION_PADDING,
        _ => viewport.0 - (anchor.x + anchor.w) - COLLISION_PADDING,
    }
}

/// The height the popup may take before it is measured: the roomier of the preferred and the opposite side.
fn provisional_available_height(anchor: Rect, viewport: (f64, f64), request: Request) -> f64 {
    let side = request.side.physical(request.rtl);
    match side {
        DropdownMenuSide::Top | DropdownMenuSide::Bottom => {
            let room =
                space_on(side, anchor, viewport).max(space_on(side.opposite(), anchor, viewport));
            (room - request.side_offset).max(0.0)
        }
        _ => (viewport.1 - 2.0 * COLLISION_PADDING).max(0.0),
    }
}

fn shift(position: f64, size: f64, limit: f64) -> f64 {
    let max = limit - size - COLLISION_PADDING;
    if max < COLLISION_PADDING {
        return position;
    }
    position.clamp(COLLISION_PADDING, max)
}

fn compute_placement(
    anchor: Rect,
    popup: (f64, f64),
    viewport: (f64, f64),
    request: Request,
) -> Placement {
    let (pw, ph) = popup;
    let preferred = request.side.physical(request.rtl);
    let needed = |side: DropdownMenuSide| match side {
        DropdownMenuSide::Top | DropdownMenuSide::Bottom => ph + request.side_offset,
        _ => pw + request.side_offset,
    };
    let flipped = space_on(preferred, anchor, viewport) < needed(preferred)
        && space_on(preferred.opposite(), anchor, viewport) > space_on(preferred, anchor, viewport);
    let side = if flipped {
        preferred.opposite()
    } else {
        preferred
    };
    let reported_side = if flipped {
        request.side.opposite()
    } else {
        request.side
    };

    let rtl = request.rtl;
    let align = request.align;
    let along = if align == DropdownMenuAlign::End {
        -request.align_offset
    } else {
        request.align_offset
    };
    let horizontal = |along: f64| {
        let base = match (align, rtl) {
            (DropdownMenuAlign::Start, false) | (DropdownMenuAlign::End, true) => anchor.x,
            (DropdownMenuAlign::End, false) | (DropdownMenuAlign::Start, true) => {
                anchor.x + anchor.w - pw
            }
            (DropdownMenuAlign::Center, _) => anchor.x + (anchor.w - pw) / 2.0,
        };
        shift(base + if rtl { -along } else { along }, pw, viewport.0)
    };
    let vertical = |along: f64| {
        let base = match align {
            DropdownMenuAlign::Start => anchor.y,
            DropdownMenuAlign::End => anchor.y + anchor.h - ph,
            DropdownMenuAlign::Center => anchor.y + (anchor.h - ph) / 2.0,
        };
        shift(base + along, ph, viewport.1)
    };

    let (left, top) = match side {
        DropdownMenuSide::Top => (horizontal(along), anchor.y - ph - request.side_offset),
        DropdownMenuSide::Bottom => (horizontal(along), anchor.y + anchor.h + request.side_offset),
        DropdownMenuSide::Left => (anchor.x - pw - request.side_offset, vertical(along)),
        _ => (anchor.x + anchor.w + request.side_offset, vertical(along)),
    };

    let available_height = match side {
        DropdownMenuSide::Top | DropdownMenuSide::Bottom => {
            (space_on(side, anchor, viewport) - request.side_offset).max(0.0)
        }
        _ => (viewport.1 - 2.0 * COLLISION_PADDING).max(0.0),
    };
    let center_x = anchor.x + anchor.w / 2.0 - left;
    let center_y = anchor.y + anchor.h / 2.0 - top;
    let (origin_x, origin_y) = match side {
        DropdownMenuSide::Top => (center_x.clamp(0.0, pw), ph),
        DropdownMenuSide::Bottom => (center_x.clamp(0.0, pw), 0.0),
        DropdownMenuSide::Left => (pw, center_y.clamp(0.0, ph)),
        _ => (0.0, center_y.clamp(0.0, ph)),
    };

    Placement {
        left,
        top,
        side: reported_side,
        align,
        available_height,
        origin_x,
        origin_y,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum FocusTarget {
    /// Focus the popup itself, as after a pointer opens the menu.
    #[default]
    Popup,
    First,
    Last,
    /// Leave focus where it is, as when a submenu opens on hover.
    Keep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Closed,
    /// Mounted but invisible, waiting for the popup size to place it.
    Measuring,
    Open,
    /// Still mounted while the exit animation plays.
    Closing,
}

/// Open state and anchor of one menu or submenu.
#[derive(Clone, Copy)]
pub struct MenuState {
    id: usize,
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
    anchor: Signal<Option<Rc<MountedData>>>,
    point: Signal<Option<(f64, f64)>>,
    focus_on_open: Signal<FocusTarget>,
    modal: bool,
    /// For a submenu: the parent's "which submenu is open" signal, and this submenu's key in it.
    sub_of: Option<(Signal<Option<usize>>, usize)>,
}

impl MenuState {
    pub fn is_open(self) -> bool {
        if let Some(open) = (self.controlled)() {
            return open;
        }
        if let Some((open_sub, id)) = self.sub_of {
            return open_sub() == Some(id);
        }
        (self.internal)()
    }

    pub fn set_open(self, open: bool) {
        if self.controlled.peek().is_none() {
            if let Some((mut open_sub, id)) = self.sub_of {
                if open {
                    open_sub.set(Some(id));
                } else if *open_sub.peek() == Some(id) {
                    open_sub.set(None);
                }
            } else {
                let mut internal = self.internal;
                internal.set(open);
            }
        }
        if let Some(handler) = self.on_open_change {
            handler.call(open);
        }
    }

    /// Opens the menu with its popup placed at a window coordinate, as a context menu is.
    pub fn open_at(self, x: f64, y: f64) {
        let mut point = self.point;
        point.set(Some((x, y)));
        self.open_with_focus(FocusTarget::Popup);
    }

    fn open_with_focus(self, target: FocusTarget) {
        let mut focus_on_open = self.focus_on_open;
        focus_on_open.set(target);
        self.set_open(true);
    }

    fn close_and_focus_anchor(self) {
        self.set_open(false);
        focus_node(self.anchor.peek().clone());
    }

    fn trigger_dom_id(self) -> String {
        format!("oniko-menu-{}-trigger", self.id)
    }

    fn popup_dom_id(self) -> String {
        format!("oniko-menu-{}-popup", self.id)
    }
}

pub fn use_menu_state(
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    modal: bool,
    parent_open_sub: Option<Signal<Option<usize>>>,
) -> MenuState {
    let id = use_hook(next_id);
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let anchor = use_signal(|| None::<Rc<MountedData>>);
    let point = use_signal(|| None::<(f64, f64)>);
    let focus_on_open = use_signal(FocusTarget::default);
    MenuState {
        id,
        internal,
        controlled,
        on_open_change,
        anchor,
        point,
        focus_on_open,
        modal,
        sub_of: parent_open_sub.map(|open_sub| (open_sub, id)),
    }
}

/// What the items of one popup share: which item is highlighted and which submenu is open.
#[derive(Clone, Copy)]
pub struct MenuLevel {
    state: MenuState,
    /// The top-level menu, which closes when an item is chosen.
    root: MenuState,
    popup: Signal<Option<Rc<MountedData>>>,
    highlighted: Signal<Option<usize>>,
    open_sub: Signal<Option<usize>>,
    sub_token: Signal<u64>,
    rtl: bool,
    depth: usize,
}

impl MenuLevel {
    fn inline_end_key(self) -> Key {
        if self.rtl {
            Key::ArrowLeft
        } else {
            Key::ArrowRight
        }
    }

    fn inline_start_key(self) -> Key {
        if self.rtl {
            Key::ArrowRight
        } else {
            Key::ArrowLeft
        }
    }

    /// Closes the open submenu of this level unless the pointer reaches it first.
    fn schedule_sub_close(self) {
        if self.open_sub.peek().is_none() {
            return;
        }
        let mut token = self.sub_token;
        let ticket = *token.peek() + 1;
        token.set(ticket);
        let mut open_sub = self.open_sub;
        spawn(async move {
            sleep_ms(SUB_CLOSE_DELAY_MS).await;
            if *token.peek() == ticket {
                open_sub.set(None);
            }
        });
    }

    fn cancel_sub_close(self) {
        let mut token = self.sub_token;
        let ticket = *token.peek() + 1;
        token.set(ticket);
    }
}

/// Shared by the menus of one `Menubar`.
#[derive(Clone, Copy)]
pub struct MenubarContext {
    pub open_menu: Signal<Option<usize>>,
    /// Menus in document order with the DOM id of their trigger.
    pub triggers: Signal<Vec<(usize, String)>>,
    /// The trigger that is in the tab order.
    pub active: Signal<Option<usize>>,
    /// A menu that was reached by keyboard and should focus its first item when it opens.
    pub pending_focus: Signal<Option<usize>>,
}

impl MenubarContext {
    fn adjacent(self, current: usize, forward: bool) -> Option<(usize, String)> {
        let triggers = self.triggers.peek();
        let position = triggers.iter().position(|(id, _)| *id == current)?;
        let count = triggers.len();
        let next = if forward {
            (position + 1) % count
        } else {
            (position + count - 1) % count
        };
        triggers.get(next).cloned()
    }
}

/// Identifies a `MenubarMenu` for its trigger and popup.
#[derive(Clone, Copy)]
pub struct MenubarMenuContext {
    pub id: usize,
}

fn attribute_text(attributes: &[Attribute], name: &str) -> Option<String> {
    attributes.iter().find_map(|attribute| {
        if attribute.name != name {
            return None;
        }
        match &attribute.value {
            dioxus::core::AttributeValue::Text(text) => Some(text.clone()),
            _ => None,
        }
    })
}

/// Owns the open state. Pass `open` and `on_open_change` to control it, or use `default_open`.
/// Renders no element of its own.
#[component]
pub fn DropdownMenu(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    /// Dim and block the rest of the page with a transparent layer while the menu is open.
    #[props(default = true)]
    modal: bool,
    children: Element,
) -> Element {
    let state = use_menu_state(open, default_open, on_open_change, modal, None);
    use_context_provider(|| state);

    rsx! {
        {children}
    }
}

/// Upstream's portal. The popup is already positioned with `position: fixed`, so this renders its children.
#[component]
pub fn DropdownMenuPortal(children: Element) -> Element {
    rsx! {
        {children}
    }
}

/// The button that opens the menu. It is a plain `button`; pass `variant` or `size` to style it as a
/// `Button`, the way upstream passes `render={<Button />}`.
#[component]
pub fn DropdownMenuTrigger(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let caller_class = take_class(&mut attributes);
    let class = if variant.is_some() || size.is_some() {
        button_variants(
            variant.unwrap_or_default(),
            size.unwrap_or_default(),
            caller_class.as_deref(),
        )
    } else {
        cn!(caller_class.as_deref())
    };

    rsx! {
        MenuTrigger {
            slot: "dropdown-menu-trigger",
            class_name: class,
            attributes,
            {children}
        }
    }
}

/// The trigger button shared by `DropdownMenuTrigger` and `MenubarTrigger`.
#[component]
pub fn MenuTrigger(
    slot: &'static str,
    class_name: String,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let state = use_context::<MenuState>();
    let bar = try_use_context::<MenubarContext>();
    let bar_menu = try_use_context::<MenubarMenuContext>();
    let open = state.is_open();
    let mut anchor = state.anchor;
    let dom_id = state.trigger_dom_id();

    if let (Some(bar), Some(menu)) = (bar, bar_menu) {
        let mut triggers = bar.triggers;
        let registered_id = dom_id.clone();
        use_effect(move || {
            triggers.write().push((menu.id, registered_id.clone()));
        });
        use_drop(move || {
            if let Ok(mut triggers) = triggers.try_write() {
                triggers.retain(|(id, _)| *id != menu.id);
            }
        });
    }

    let tab_stop = match (bar, bar_menu) {
        (Some(bar), Some(menu)) => {
            let first = bar.triggers.read().first().map(|(id, _)| *id);
            let active = (bar.active)().or(first);
            Some(active == Some(menu.id))
        }
        _ => None,
    };

    rsx! {
        button {
            id: dom_id,
            "data-slot": slot,
            "data-popup-open": open.then_some(""),
            "data-pressed": open.then_some(""),
            r#type: "button",
            aria_haspopup: "menu",
            aria_expanded: open,
            tabindex: tab_stop.map(|stop| if stop { "0" } else { "-1" }),
            class: class_name,
            onmounted: move |event| anchor.set(Some(event.data())),
            onfocus: move |_| {
                if let (Some(bar), Some(menu)) = (bar, bar_menu) {
                    let mut active = bar.active;
                    active.set(Some(menu.id));
                }
            },
            onmouseenter: move |_| {
                if let (Some(bar), Some(menu)) = (bar, bar_menu) {
                    let mut open_menu = bar.open_menu;
                    if open_menu.peek().is_some_and(|current| current != menu.id) {
                        open_menu.set(Some(menu.id));
                    }
                }
            },
            onclick: move |_| state.set_open(!open),
            onkeydown: move |event| match event.key() {
                Key::ArrowDown | Key::Enter => {
                    event.prevent_default();
                    if open {
                        state.set_open(false);
                    } else {
                        state.open_with_focus(FocusTarget::First);
                    }
                }
                Key::Character(c) if c == " " => {
                    event.prevent_default();
                    if open {
                        state.set_open(false);
                    } else {
                        state.open_with_focus(FocusTarget::First);
                    }
                }
                Key::ArrowUp => {
                    event.prevent_default();
                    state.open_with_focus(FocusTarget::Last);
                }
                key @ (Key::ArrowLeft | Key::ArrowRight) => {
                    if let (Some(bar), Some(menu)) = (bar, bar_menu) {
                        event.prevent_default();
                        if let Some((next, next_dom_id)) = bar.adjacent(menu.id, key == Key::ArrowRight) {
                            let mut open_menu = bar.open_menu;
                            if open_menu.peek().is_some() {
                                let mut pending = bar.pending_focus;
                                pending.set(Some(next));
                                open_menu.set(Some(next));
                            } else {
                                spawn(async move {
                                    let _ = document::eval(&format!(
                                        "document.getElementById('{next_dom_id}')?.focus()"
                                    ))
                                    .await;
                                });
                            }
                        }
                    }
                }
                _ => {}
            },
            ..attributes,
            {children}
        }
    }
}

/// The menu popup, its positioner and the transparent layer behind it. Renders nothing while closed.
/// It is shared by `DropdownMenuContent`, `DropdownMenuSubContent`, `ContextMenuContent` and
/// `MenubarContent`.
#[component]
pub fn MenuPopup(
    slot: &'static str,
    class_name: String,
    #[props(default)] side: DropdownMenuSide,
    #[props(default)] align: DropdownMenuAlign,
    #[props(default = 4.0)] side_offset: f64,
    #[props(default)] align_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let state = use_context::<MenuState>();
    let parent = if state.sub_of.is_some() {
        try_use_context::<MenuLevel>()
    } else {
        None
    };
    let bar = try_use_context::<MenubarContext>();
    let bar_menu = try_use_context::<MenubarMenuContext>();
    let rtl = parent.is_some_and(|parent| parent.rtl)
        || attribute_text(&attributes, "dir").as_deref() == Some("rtl");

    let popup = use_signal(|| None::<Rc<MountedData>>);
    let highlighted = use_signal(|| None::<usize>);
    let open_sub = use_signal(|| None::<usize>);
    let sub_token = use_signal(|| 0u64);
    let level = use_context_provider(|| MenuLevel {
        state,
        root: parent.map_or(state, |parent| parent.root),
        popup,
        highlighted,
        open_sub,
        sub_token,
        rtl,
        depth: parent.map_or(0, |parent| parent.depth + 1),
    });

    let mut phase = use_signal(|| {
        if state.is_open() {
            Phase::Measuring
        } else {
            Phase::Closed
        }
    });
    let mut placement = use_signal(|| None::<Placement>);
    let mut anchor_vars = use_signal(|| None::<(f64, f64, f64)>);
    let mut starting = use_signal(|| false);
    let mut close_ticket = use_signal(|| 0u64);

    use_effect(move || {
        let open = state.is_open();
        let current = *phase.peek();
        if open {
            match current {
                Phase::Closed => phase.set(Phase::Measuring),
                Phase::Closing => {
                    let ticket = *close_ticket.peek() + 1;
                    close_ticket.set(ticket);
                    phase.set(Phase::Open);
                }
                Phase::Measuring | Phase::Open => {}
            }
        } else if matches!(current, Phase::Measuring | Phase::Open) {
            let ticket = *close_ticket.peek() + 1;
            close_ticket.set(ticket);
            phase.set(Phase::Closing);
            spawn(async move {
                sleep_ms(CLOSE_MS).await;
                if *close_ticket.peek() == ticket && *phase.peek() == Phase::Closing {
                    phase.set(Phase::Closed);
                    placement.set(None);
                    anchor_vars.set(None);
                }
            });
        }
    });

    use_effect(move || {
        if phase() != Phase::Open {
            return;
        }
        let mut target = *state.focus_on_open.peek();
        if let (Some(bar), Some(menu)) = (bar, bar_menu)
            && *bar.pending_focus.peek() == Some(menu.id)
        {
            let mut pending = bar.pending_focus;
            pending.set(None);
            target = FocusTarget::First;
        }
        let mut focus_on_open = state.focus_on_open;
        focus_on_open.set(FocusTarget::Popup);
        let popup_id = state.popup_dom_id();
        let node = popup.peek().clone();
        spawn(async move {
            // The popup only becomes focusable once the browser has applied `visibility: visible`.
            next_frames().await;
            match target {
                FocusTarget::Popup => focus_node(node),
                FocusTarget::First => navigate(popup_id, "first"),
                FocusTarget::Last => navigate(popup_id, "last"),
                FocusTarget::Keep => {}
            }
        });
    });

    let current_phase = phase();
    if current_phase == Phase::Closed {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(class_name, take_class(&mut attributes).as_deref());
    let popup_id = state.popup_dom_id();
    let request = Request {
        side,
        align,
        side_offset,
        align_offset,
        rtl,
    };

    let shown = matches!(current_phase, Phase::Open | Phase::Closing);
    let placed = placement();
    let mut style = String::from("position:fixed;");
    match placed {
        Some(placed) if shown => style.push_str(&format!(
            "visibility:visible;left:{:.2}px;top:{:.2}px;--available-height:{:.2}px;--transform-origin:{:.2}px {:.2}px;",
            placed.left, placed.top, placed.available_height, placed.origin_x, placed.origin_y
        )),
        _ => style.push_str("left:0;top:0;visibility:hidden;"),
    }
    if let Some((width, height, available)) = anchor_vars() {
        style.push_str(&format!(
            "--anchor-width:{width:.2}px;--anchor-height:{height:.2}px;"
        ));
        if placed.is_none() || !shown {
            style.push_str(&format!("--available-height:{available:.2}px;"));
        }
    }

    let used_side = placed.map_or(side, |placed| placed.side);
    let used_align = placed.map_or(align, |placed| placed.align);
    let open_now = current_phase == Phase::Open;
    let closing = current_phase == Phase::Closing;
    let show_overlay = level.depth == 0 && state.modal && !closing;
    let key_level = level;

    rsx! {
        if show_overlay {
            div {
                role: "presentation",
                "data-base-ui-inert": "",
                class: "fixed inset-0 z-40",
                oncontextmenu: move |event| {
                    event.prevent_default();
                    state.set_open(false);
                },
                onclick: move |_| state.set_open(false),
            }
        }
        div {
            class: "isolate z-50 outline-none",
            style: style,
            "data-side": used_side.as_str(),
            "data-align": used_align.as_str(),
            "data-open": open_now.then_some(""),
            "data-closed": closing.then_some(""),
            div {
                id: popup_id.clone(),
                "data-slot": slot,
                role: "menu",
                tabindex: "-1",
                "aria-orientation": "vertical",
                "aria-labelledby": state.trigger_dom_id(),
                "data-side": used_side.as_str(),
                "data-align": used_align.as_str(),
                "data-open": open_now.then_some(""),
                "data-closed": closing.then_some(""),
                "data-starting-style": (open_now && starting()).then_some(""),
                "data-ending-style": closing.then_some(""),
                class: class,
                onmounted: move |event| {
                    let node = event.data();
                    let mut popup = popup;
                    popup.set(Some(node.clone()));
                    if *phase.peek() != Phase::Measuring {
                        return;
                    }
                    spawn(async move {
                        let anchor = if let Some((x, y)) = *state.point.peek() {
                            Some(Rect { x, y, w: 0.0, h: 0.0 })
                        } else {
                            let anchor_node = state.anchor.peek().clone();
                            match anchor_node {
                                Some(node) => node.get_client_rect().await.ok().map(|rect| Rect {
                                    x: rect.origin.x,
                                    y: rect.origin.y,
                                    w: rect.size.width,
                                    h: rect.size.height,
                                }),
                                None => None,
                            }
                        };
                        let anchor = anchor.unwrap_or(Rect { x: 0.0, y: 0.0, w: 0.0, h: 0.0 });
                        let viewport = viewport().await;
                        anchor_vars.set(Some((
                            anchor.w,
                            anchor.h,
                            provisional_available_height(anchor, viewport, request),
                        )));
                        next_frames().await;
                        if *phase.peek() != Phase::Measuring {
                            return;
                        }
                        let Ok(rect) = node.get_client_rect().await else {
                            phase.set(Phase::Open);
                            return;
                        };
                        // The popup sits at left:0/top:0, so its origin is the offset of the fixed
                        // containing block (nonzero inside a transformed ancestor such as a dialog).
                        let mut placed = compute_placement(
                            anchor,
                            (rect.size.width, rect.size.height),
                            viewport,
                            request,
                        );
                        placed.left -= rect.origin.x;
                        placed.top -= rect.origin.y;
                        placement.set(Some(placed));
                        phase.set(Phase::Open);
                        starting.set(true);
                        next_frames().await;
                        starting.set(false);
                    });
                },
                onmouseenter: move |_| {
                    if let Some(parent) = parent {
                        parent.cancel_sub_close();
                    }
                },
                onkeydown: move |event| {
                    let level = key_level;
                    let key = event.key();
                    if key == Key::Escape {
                        event.stop_propagation();
                        event.prevent_default();
                        level.state.close_and_focus_anchor();
                    } else if key == Key::ArrowDown {
                        event.stop_propagation();
                        event.prevent_default();
                        navigate(popup_id.clone(), "next");
                    } else if key == Key::ArrowUp {
                        event.stop_propagation();
                        event.prevent_default();
                        navigate(popup_id.clone(), "prev");
                    } else if key == Key::Home {
                        event.stop_propagation();
                        event.prevent_default();
                        navigate(popup_id.clone(), "first");
                    } else if key == Key::End {
                        event.stop_propagation();
                        event.prevent_default();
                        navigate(popup_id.clone(), "last");
                    } else if key == Key::Tab {
                        event.stop_propagation();
                        event.prevent_default();
                        level.root.close_and_focus_anchor();
                    } else if key == level.inline_start_key() && level.depth > 0 {
                        event.stop_propagation();
                        event.prevent_default();
                        level.state.close_and_focus_anchor();
                    } else if matches!(key, Key::ArrowLeft | Key::ArrowRight) && level.depth == 0 {
                        if let (Some(bar), Some(menu)) = (bar, bar_menu) {
                            event.stop_propagation();
                            event.prevent_default();
                            if let Some((next, _)) = bar.adjacent(menu.id, key == Key::ArrowRight) {
                                let mut pending = bar.pending_focus;
                                pending.set(Some(next));
                                let mut open_menu = bar.open_menu;
                                open_menu.set(Some(next));
                            }
                        }
                    } else if let Key::Character(text) = key
                        && text != " "
                        && text.chars().count() == 1
                        && !event.modifiers().intersects(Modifiers::CONTROL | Modifiers::META | Modifiers::ALT)
                    {
                        event.stop_propagation();
                        event.prevent_default();
                        typeahead(popup_id.clone(), text);
                    }
                },
                ..attributes,
                {children}
            }
        }
    }
}

#[derive(Clone, Copy)]
struct ItemBinding {
    id: usize,
    level: MenuLevel,
    node: Signal<Option<Rc<MountedData>>>,
}

fn use_item() -> ItemBinding {
    let level = use_context::<MenuLevel>();
    let id = use_hook(next_id);
    let node = use_signal(|| None::<Rc<MountedData>>);
    ItemBinding { id, level, node }
}

impl ItemBinding {
    fn dom_id(self) -> String {
        format!("oniko-menu-item-{}", self.id)
    }

    fn highlighted(self) -> bool {
        (self.level.highlighted)() == Some(self.id)
    }

    /// Highlights the item under the pointer by moving focus to it.
    fn pointer_enter(self, disabled: bool) {
        if disabled {
            return;
        }
        focus_node(self.node.peek().clone());
    }

    fn pointer_leave(self) {
        if *self.level.highlighted.peek() == Some(self.id) {
            focus_node(self.level.popup.peek().clone());
        }
    }

    fn focus(self) {
        let mut highlighted = self.level.highlighted;
        highlighted.set(Some(self.id));
    }

    fn blur(self) {
        let mut highlighted = self.level.highlighted;
        if *highlighted.peek() == Some(self.id) {
            highlighted.set(None);
        }
    }

    fn key_activates(key: &Key) -> bool {
        matches!(key, Key::Enter) || matches!(key, Key::Character(c) if c == " ")
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

/// A menu item that runs `onclick` and, by default, closes the menu. Shared by the item parts of the
/// menu components.
#[component]
pub fn MenuItem(
    slot: &'static str,
    class_name: String,
    #[props(default)] variant: DropdownMenuItemVariant,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(default = true)] close_on_click: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let item = use_item();
    let level = item.level;
    let highlighted = item.highlighted();

    rsx! {
        div {
            id: item.dom_id(),
            "data-slot": slot,
            "data-inset": inset.then_some("true"),
            "data-variant": variant.data(),
            "data-highlighted": highlighted.then_some(""),
            "data-disabled": disabled.then_some(""),
            "aria-disabled": disabled.then_some("true"),
            role: "menuitem",
            tabindex: if highlighted { "0" } else { "-1" },
            class: class_name,
            onmounted: move |event| {
                let mut node = item.node;
                node.set(Some(event.data()));
            },
            onmouseenter: move |_| {
                item.pointer_enter(disabled);
                level.schedule_sub_close();
            },
            onmouseleave: move |_| item.pointer_leave(),
            onfocus: move |_| item.focus(),
            onblur: move |_| item.blur(),
            onclick: move |event| {
                if disabled {
                    return;
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                if close_on_click {
                    level.root.close_and_focus_anchor();
                }
            },
            onkeydown: move |event| {
                if ItemBinding::key_activates(&event.key()) {
                    event.prevent_default();
                    event.stop_propagation();
                    if !disabled {
                        click_element(item.dom_id());
                    }
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuItem(
    #[props(default)] variant: DropdownMenuItemVariant,
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
            slot: "dropdown-menu-item",
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

/// A menu item with a check indicator. Pass `checked` to control it, or use `default_checked`. It does
/// not close the menu unless `close_on_click` is set.
#[component]
pub fn MenuCheckboxItem(
    slot: &'static str,
    class_name: String,
    indicator_class: String,
    indicator_slot: Option<&'static str>,
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
    let item = use_item();
    let level = item.level;
    let highlighted = item.highlighted();
    let mut internal = use_signal(|| default_checked);
    let controlled = use_prop(checked);
    let is_checked = match controlled() {
        Some(checked) => checked,
        None => internal(),
    };

    rsx! {
        div {
            id: item.dom_id(),
            "data-slot": slot,
            "data-inset": inset.then_some("true"),
            "data-highlighted": highlighted.then_some(""),
            "data-disabled": disabled.then_some(""),
            "data-checked": is_checked.then_some(""),
            "data-unchecked": (!is_checked).then_some(""),
            "aria-disabled": disabled.then_some("true"),
            role: "menuitemcheckbox",
            aria_checked: is_checked,
            tabindex: if highlighted { "0" } else { "-1" },
            class: class_name,
            onmounted: move |event| {
                let mut node = item.node;
                node.set(Some(event.data()));
            },
            onmouseenter: move |_| {
                item.pointer_enter(disabled);
                level.schedule_sub_close();
            },
            onmouseleave: move |_| item.pointer_leave(),
            onfocus: move |_| item.focus(),
            onblur: move |_| item.blur(),
            onclick: move |event| {
                if disabled {
                    return;
                }
                let next = !is_checked;
                if controlled.peek().is_none() {
                    internal.set(next);
                }
                if let Some(handler) = on_checked_change {
                    handler.call(next);
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                if close_on_click {
                    level.root.close_and_focus_anchor();
                }
            },
            onkeydown: move |event| {
                if ItemBinding::key_activates(&event.key()) {
                    event.prevent_default();
                    event.stop_propagation();
                    if !disabled {
                        click_element(item.dom_id());
                    }
                }
            },
            ..attributes,
            span {
                class: indicator_class,
                "data-slot": indicator_slot,
                if is_checked {
                    span { "data-checked": "", CheckIcon {} }
                }
            }
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuCheckboxItem(
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
            slot: "dropdown-menu-checkbox-item",
            class_name: class,
            indicator_class: INDICATOR_CLASS,
            indicator_slot: "dropdown-menu-checkbox-item-indicator",
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

#[derive(Clone, Copy)]
struct RadioGroupContext {
    value: Signal<String>,
    controlled: Signal<Option<String>>,
    on_value_change: Option<EventHandler<String>>,
}

/// Groups radio items. Pass `value` to control it, or use `default_value`.
#[component]
pub fn MenuRadioGroup(
    slot: &'static str,
    value: Option<String>,
    #[props(default)] default_value: String,
    on_value_change: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let internal = use_signal(|| default_value.clone());
    let controlled = use_prop(value);
    use_context_provider(|| RadioGroupContext {
        value: internal,
        controlled,
        on_value_change,
    });

    rsx! {
        div {
            "data-slot": slot,
            role: "group",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuRadioGroup(
    value: Option<String>,
    #[props(default)] default_value: String,
    on_value_change: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        MenuRadioGroup {
            slot: "dropdown-menu-radio-group",
            value,
            default_value,
            on_value_change,
            attributes,
            {children}
        }
    }
}

/// A menu item that is checked when its `value` equals the group's value. It does not close the menu
/// unless `close_on_click` is set.
#[component]
pub fn MenuRadioItem(
    slot: &'static str,
    class_name: String,
    indicator_class: String,
    indicator_slot: Option<&'static str>,
    value: String,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(default)] close_on_click: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let item = use_item();
    let level = item.level;
    let highlighted = item.highlighted();
    let group = use_context::<RadioGroupContext>();
    let current = (group.controlled)().unwrap_or_else(|| (group.value)());
    let is_checked = current == value;
    let choice = value.clone();

    rsx! {
        div {
            id: item.dom_id(),
            "data-slot": slot,
            "data-inset": inset.then_some("true"),
            "data-highlighted": highlighted.then_some(""),
            "data-disabled": disabled.then_some(""),
            "data-checked": is_checked.then_some(""),
            "data-unchecked": (!is_checked).then_some(""),
            "aria-disabled": disabled.then_some("true"),
            role: "menuitemradio",
            aria_checked: is_checked,
            tabindex: if highlighted { "0" } else { "-1" },
            class: class_name,
            onmounted: move |event| {
                let mut node = item.node;
                node.set(Some(event.data()));
            },
            onmouseenter: move |_| {
                item.pointer_enter(disabled);
                level.schedule_sub_close();
            },
            onmouseleave: move |_| item.pointer_leave(),
            onfocus: move |_| item.focus(),
            onblur: move |_| item.blur(),
            onclick: move |event| {
                if disabled {
                    return;
                }
                if group.controlled.peek().is_none() {
                    let mut internal = group.value;
                    internal.set(choice.clone());
                }
                if let Some(handler) = group.on_value_change {
                    handler.call(choice.clone());
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                if close_on_click {
                    level.root.close_and_focus_anchor();
                }
            },
            onkeydown: move |event| {
                if ItemBinding::key_activates(&event.key()) {
                    event.prevent_default();
                    event.stop_propagation();
                    if !disabled {
                        click_element(item.dom_id());
                    }
                }
            },
            ..attributes,
            span {
                class: indicator_class,
                "data-slot": indicator_slot,
                if is_checked {
                    span { "data-checked": "", CheckIcon {} }
                }
            }
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuRadioItem(
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
            slot: "dropdown-menu-radio-item",
            class_name: class,
            indicator_class: INDICATOR_CLASS,
            indicator_slot: "dropdown-menu-radio-item-indicator",
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
pub fn DropdownMenuSub(
    open: Option<bool>,
    #[props(default)] default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let level = use_context::<MenuLevel>();
    let state = use_menu_state(
        open,
        default_open,
        on_open_change,
        false,
        Some(level.open_sub),
    );
    use_context_provider(|| state);
    use_effect(move || {
        if default_open {
            let mut open_sub = level.open_sub;
            open_sub.set(Some(state.id));
        }
    });

    rsx! {
        {children}
    }
}

/// The item that opens a submenu. It draws the chevron after its children.
#[component]
pub fn MenuSubTrigger(
    slot: &'static str,
    class_name: String,
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let state = use_context::<MenuState>();
    let item = use_item();
    let level = item.level;
    let highlighted = item.highlighted();
    let open = state.is_open();
    let mut anchor = state.anchor;
    let mut hover_token = use_signal(|| 0u64);

    rsx! {
        div {
            id: state.trigger_dom_id(),
            "data-slot": slot,
            "data-inset": inset.then_some("true"),
            "data-highlighted": highlighted.then_some(""),
            "data-disabled": disabled.then_some(""),
            "data-popup-open": open.then_some(""),
            "aria-disabled": disabled.then_some("true"),
            role: "menuitem",
            aria_haspopup: "menu",
            aria_expanded: open,
            tabindex: if highlighted { "0" } else { "-1" },
            class: class_name,
            onmounted: move |event| {
                let node = event.data();
                let mut item_node = item.node;
                item_node.set(Some(node.clone()));
                anchor.set(Some(node));
            },
            onmouseenter: move |_| {
                if disabled {
                    return;
                }
                item.pointer_enter(disabled);
                level.cancel_sub_close();
                if open {
                    return;
                }
                let ticket = *hover_token.peek() + 1;
                hover_token.set(ticket);
                spawn(async move {
                    sleep_ms(SUB_OPEN_DELAY_MS).await;
                    if *hover_token.peek() == ticket && !state.is_open() {
                        state.open_with_focus(FocusTarget::Keep);
                    }
                });
            },
            onmouseleave: move |_| {
                let ticket = *hover_token.peek() + 1;
                hover_token.set(ticket);
                item.pointer_leave();
            },
            onfocus: move |_| item.focus(),
            onblur: move |_| item.blur(),
            onclick: move |_| {
                if !disabled && !open {
                    state.open_with_focus(FocusTarget::Popup);
                }
            },
            onkeydown: move |event| {
                let key = event.key();
                if key == level.inline_end_key() || ItemBinding::key_activates(&key) {
                    event.prevent_default();
                    event.stop_propagation();
                    if !disabled {
                        state.open_with_focus(FocusTarget::First);
                    }
                }
            },
            ..attributes,
            {children}
            ChevronRightIcon {}
        }
    }
}

#[component]
pub fn DropdownMenuSubTrigger(
    #[props(default)] inset: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(SUB_TRIGGER_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        MenuSubTrigger {
            slot: "dropdown-menu-sub-trigger",
            class_name: class,
            inset,
            disabled,
            attributes,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuContent(
    #[props(default)] align: DropdownMenuAlign,
    #[props(default)] align_offset: f64,
    #[props(default)] side: DropdownMenuSide,
    #[props(default = 4.0)] side_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(CONTENT_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        MenuPopup {
            slot: "dropdown-menu-content",
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

/// The popup of a submenu. It opens to the right of its trigger.
#[component]
pub fn DropdownMenuSubContent(
    #[props(default)] align: DropdownMenuAlign,
    #[props(default = -3.0)] align_offset: f64,
    #[props(default = DropdownMenuSide::Right)] side: DropdownMenuSide,
    #[props(default = 0.0)] side_offset: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        CONTENT_CLASS,
        SUB_CONTENT_CLASS,
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        MenuPopup {
            slot: "dropdown-menu-sub-content",
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
    let class = cn!(LABEL_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "dropdown-menu-label",
            "data-inset": inset.then_some("true"),
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
    let class = cn!(SEPARATOR_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "dropdown-menu-separator",
            role: "separator",
            "aria-orientation": "horizontal",
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
    let class = cn!(SHORTCUT_CLASS, take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "dropdown-menu-shortcut",
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
            width: "24",
            height: "24",
            path { d: "M20 6 9 17l-5-5" }
        }
    }
}

#[component]
fn ChevronRightIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "24",
            height: "24",
            class: "rtl:rotate-180 ml-auto",
            path { d: "m9 18 6-6-6-6" }
        }
    }
}
