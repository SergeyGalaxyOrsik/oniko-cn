//! Resizable: panels separated by handles that the user drags or moves with the arrow keys.
//! ResizablePanelGroup: lays out its panels along a direction and shares their sizes.
//! ResizablePanel: one panel, sized in percent of the group.
//! ResizableHandle: the separator between two neighbouring panels.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const KEYBOARD_STEP: f64 = 5.0;

const GROUP_BASE: &str = "flex h-full w-full";

const PANEL_BASE: &str = "relative min-h-0 min-w-0 overflow-hidden";

const HANDLE_BASE: &str = "bg-border focus-visible:ring-ring focus-visible:ring-1 focus-visible:outline-hidden relative flex shrink-0 items-center justify-center select-none touch-none aria-[orientation=horizontal]:h-px aria-[orientation=horizontal]:w-full aria-[orientation=vertical]:h-full aria-[orientation=vertical]:w-px after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2 aria-[orientation=horizontal]:after:left-0 aria-[orientation=horizontal]:after:h-1 aria-[orientation=horizontal]:after:w-full aria-[orientation=horizontal]:after:translate-x-0 aria-[orientation=horizontal]:after:-translate-y-1/2 [&[aria-orientation=horizontal]>div]:rotate-90";

const GRIP_BASE: &str = "bg-border h-6 w-1 rounded-lg z-10 flex shrink-0";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl ResizableDirection {
    fn layout_class(self) -> &'static str {
        match self {
            Self::Horizontal => "flex-row",
            Self::Vertical => "flex-col",
        }
    }

    /// The handle is a line across the panels, so its ARIA orientation is the perpendicular one.
    fn separator_orientation(self) -> &'static str {
        match self {
            Self::Horizontal => "vertical",
            Self::Vertical => "horizontal",
        }
    }

    fn axis(self, x: f64, y: f64) -> f64 {
        match self {
            Self::Horizontal => x,
            Self::Vertical => y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PanelSlot {
    /// Size in percent of the group. `None` shares whatever space the fixed panels leave.
    size: Option<f64>,
    min: f64,
    max: f64,
}

#[derive(Clone, Debug)]
struct Drag {
    handle: usize,
    origin: f64,
    sizes: Vec<f64>,
    /// Group length in pixels along the direction; known once the group has been measured.
    length: Option<f64>,
}

#[derive(Clone, Copy)]
struct ResizableContext {
    direction: ResizableDirection,
    slots: Signal<Vec<PanelSlot>>,
    group: Signal<Option<Rc<MountedData>>>,
    drag: Signal<Option<Drag>>,
    panel_count: Signal<usize>,
    handle_count: Signal<usize>,
}

impl ResizableContext {
    /// Panels and handles take their index from render order, which matches DOM order.
    fn register_panel(self, slot: PanelSlot) -> usize {
        let index = *self.panel_count.peek();
        let mut panel_count = self.panel_count;
        panel_count.set(index + 1);
        let mut slots = self.slots;
        slots.write().push(slot);
        index
    }

    fn register_handle(self) -> usize {
        let index = *self.handle_count.peek();
        let mut handle_count = self.handle_count;
        handle_count.set(index + 1);
        index
    }
}

/// Sizes in percent, with `None` panels sharing the remaining space equally.
fn effective_sizes(slots: &[PanelSlot]) -> Vec<f64> {
    let fixed: f64 = slots.iter().filter_map(|slot| slot.size).sum();
    let free = slots.iter().filter(|slot| slot.size.is_none()).count();
    let share = if free == 0 {
        0.0
    } else {
        (100.0 - fixed).max(0.0) / free as f64
    };
    slots
        .iter()
        .map(|slot| slot.size.unwrap_or(share))
        .collect()
}

/// Moves the boundary after panel `handle` by `delta` percent, keeping both neighbours within their limits.
fn resize_pair(sizes: &[f64], slots: &[PanelSlot], handle: usize, delta: f64) -> Vec<f64> {
    let mut next = sizes.to_vec();
    let (Some(&first), Some(&second)) = (sizes.get(handle), sizes.get(handle + 1)) else {
        return next;
    };
    let (Some(a), Some(b)) = (slots.get(handle), slots.get(handle + 1)) else {
        return next;
    };
    let total = first + second;
    let low = a.min.max(total - b.max);
    let high = a.max.min(total - b.min);
    if low > high {
        return next;
    }
    let first_size = (first + delta).clamp(low, high);
    next[handle] = first_size;
    next[handle + 1] = total - first_size;
    next
}

fn commit(mut slots: Signal<Vec<PanelSlot>>, sizes: &[f64]) {
    let mut slots = slots.write();
    for (slot, size) in slots.iter_mut().zip(sizes) {
        slot.size = Some(*size);
    }
}

#[component]
pub fn ResizablePanelGroup(
    #[props(default)] direction: ResizableDirection,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        GROUP_BASE,
        direction.layout_class(),
        take_class(&mut attributes).as_deref()
    );

    let slots = use_signal(Vec::new);
    let group = use_signal(|| None);
    let drag = use_signal(|| None);
    let panel_count = use_signal(|| 0);
    let handle_count = use_signal(|| 0);
    let ctx = use_context_provider(|| ResizableContext {
        direction,
        slots,
        group,
        drag,
        panel_count,
        handle_count,
    });

    rsx! {
        div {
            "data-slot": "resizable-panel-group",
            class: class,
            onmounted: move |event| {
                let mut group = ctx.group;
                group.set(Some(event.data()));
            },
            onpointermove: move |event| {
                let Some(drag) = ctx.drag.peek().clone() else {
                    return;
                };
                if event.held_buttons().is_empty() {
                    let mut current = ctx.drag;
                    current.set(None);
                    return;
                }
                let Some(length) = drag.length else {
                    return;
                };
                if length <= 0.0 {
                    return;
                }
                let point = event.client_coordinates();
                let position = ctx.direction.axis(point.x, point.y);
                let delta = (position - drag.origin) / length * 100.0;
                let next = resize_pair(&drag.sizes, &ctx.slots.peek(), drag.handle, delta);
                commit(ctx.slots, &next);
            },
            onpointerup: move |_| {
                if ctx.drag.peek().is_some() {
                    let mut drag = ctx.drag;
                    drag.set(None);
                }
            },
            onpointercancel: move |_| {
                if ctx.drag.peek().is_some() {
                    let mut drag = ctx.drag;
                    drag.set(None);
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ResizablePanel(
    default_size: Option<f64>,
    #[props(default = 0.0)] min_size: f64,
    #[props(default = 100.0)] max_size: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ResizableContext>();
    let index = use_hook(|| {
        ctx.register_panel(PanelSlot {
            size: default_size,
            min: min_size,
            max: max_size,
        })
    });
    let size = effective_sizes(&ctx.slots.read())
        .get(index)
        .copied()
        .unwrap_or(100.0);
    let style = format!("flex: 0 1 {size:.4}%;");

    let mut attributes = attributes;
    let class = cn!(PANEL_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "resizable-panel",
            class: class,
            style: style,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ResizableHandle(
    #[props(default)] with_handle: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<ResizableContext>();
    let index = use_hook(|| ctx.register_handle());

    let mut attributes = attributes;
    let class = cn!(HANDLE_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "resizable-handle",
            role: "separator",
            tabindex: "0",
            "aria-orientation": ctx.direction.separator_orientation(),
            class: class,
            onpointerdown: move |event| {
                let point = event.client_coordinates();
                let origin = ctx.direction.axis(point.x, point.y);
                let sizes = effective_sizes(&ctx.slots.peek());
                let mut drag = ctx.drag;
                drag.set(Some(Drag {
                    handle: index,
                    origin,
                    sizes,
                    length: None,
                }));
                let group = ctx.group.peek().clone();
                spawn(async move {
                    let Some(group) = group else {
                        return;
                    };
                    let Ok(rect) = group.get_client_rect().await else {
                        return;
                    };
                    let length = ctx.direction.axis(rect.size.width, rect.size.height);
                    let mut drag = ctx.drag;
                    if let Some(active) = drag
                        .write()
                        .as_mut()
                        .filter(|active| active.handle == index)
                    {
                        active.length = Some(length);
                    }
                });
            },
            onkeydown: move |event| {
                let step = match (ctx.direction, event.key()) {
                    (ResizableDirection::Horizontal, Key::ArrowLeft)
                    | (ResizableDirection::Vertical, Key::ArrowUp) => -KEYBOARD_STEP,
                    (ResizableDirection::Horizontal, Key::ArrowRight)
                    | (ResizableDirection::Vertical, Key::ArrowDown) => KEYBOARD_STEP,
                    _ => return,
                };
                event.prevent_default();
                let slots = ctx.slots.peek().clone();
                let sizes = effective_sizes(&slots);
                let next = resize_pair(&sizes, &slots, index, step);
                commit(ctx.slots, &next);
            },
            ..attributes,
            if with_handle {
                div {
                    "data-slot": "resizable-handle-icon",
                    class: GRIP_BASE,
                }
            }
        }
    }
}
