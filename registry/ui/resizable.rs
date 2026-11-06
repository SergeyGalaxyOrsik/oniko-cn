//! Resizable: panels separated by handles that the user drags or moves with the arrow keys.
//! ResizablePanelGroup: lays out its panels along a orientation and shares their sizes.
//! ResizablePanel: one panel, sized in percent of the group.
//! ResizableHandle: the separator between two neighbouring panels.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const KEYBOARD_STEP: f64 = 5.0;

const GROUP_BASE: &str = "cn-resizable-panel-group flex h-full w-full aria-[orientation=vertical]:flex-col";

const HANDLE_BASE: &str = "cn-resizable-handle relative flex w-px items-center justify-center bg-border ring-offset-background after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2 focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-hidden aria-[orientation=horizontal]:h-px aria-[orientation=horizontal]:w-full aria-[orientation=horizontal]:after:left-0 aria-[orientation=horizontal]:after:h-1 aria-[orientation=horizontal]:after:w-full aria-[orientation=horizontal]:after:translate-x-0 aria-[orientation=horizontal]:after:-translate-y-1/2 [&[aria-orientation=horizontal]>div]:rotate-90";

const GRIP_BASE: &str = "cn-resizable-handle-icon z-10 flex shrink-0";

/// Parses a size such as `"50%"` or `50` into percent of the group.
fn parse_percent(size: &str) -> Option<f64> {
    size.trim().trim_end_matches('%').trim().parse().ok()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResizableOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ResizableOrientation {
    fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
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
    /// Group length in pixels along the orientation; known once the group has been measured.
    length: Option<f64>,
}

#[derive(Clone, Copy)]
struct ResizableContext {
    orientation: ResizableOrientation,
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
    #[props(default)] orientation: ResizableOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        GROUP_BASE,
        take_class(&mut attributes).as_deref()
    );

    let slots = use_signal(Vec::new);
    let group = use_signal(|| None);
    let drag = use_signal(|| None);
    let panel_count = use_signal(|| 0);
    let handle_count = use_signal(|| 0);
    let ctx = use_context_provider(|| ResizableContext {
        orientation,
        slots,
        group,
        drag,
        panel_count,
        handle_count,
    });

    rsx! {
        div {
            "data-slot": "resizable-panel-group",
            "aria-orientation": orientation.name(),
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
                let position = ctx.orientation.axis(point.x, point.y);
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
    /// Initial size, such as `"50%"`. Panels without one share the remaining space.
    default_size: Option<String>,
    min_size: Option<String>,
    max_size: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ResizableContext>();
    let index = use_hook(|| {
        ctx.register_panel(PanelSlot {
            size: default_size.as_deref().and_then(parse_percent),
            min: min_size.as_deref().and_then(parse_percent).unwrap_or(0.0),
            max: max_size.as_deref().and_then(parse_percent).unwrap_or(100.0),
        })
    });
    let size = effective_sizes(&ctx.slots.read())
        .get(index)
        .copied()
        .unwrap_or(100.0);
    let style = format!("flex: 0 1 {size:.4}%; min-width: 0; min-height: 0; overflow: hidden;");

    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

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
            "aria-orientation": ctx.orientation.separator_orientation(),
            class: class,
            onpointerdown: move |event| {
                let point = event.client_coordinates();
                let origin = ctx.orientation.axis(point.x, point.y);
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
                    let length = ctx.orientation.axis(rect.size.width, rect.size.height);
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
                let step = match (ctx.orientation, event.key()) {
                    (ResizableOrientation::Horizontal, Key::ArrowLeft)
                    | (ResizableOrientation::Vertical, Key::ArrowUp) => -KEYBOARD_STEP,
                    (ResizableOrientation::Horizontal, Key::ArrowRight)
                    | (ResizableOrientation::Vertical, Key::ArrowDown) => KEYBOARD_STEP,
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
                div { class: GRIP_BASE }
            }
        }
    }
}
