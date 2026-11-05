//! Slider: lets the user pick a value, or a range of values, by dragging thumbs along a track.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SliderOrientation {
    fn is_vertical(self) -> bool {
        self == Self::Vertical
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

const ROOT_BASE: &str = "data-vertical:min-h-40 relative flex w-full touch-none items-center select-none data-vertical:h-full data-vertical:w-auto data-vertical:flex-col data-disabled:pointer-events-none data-disabled:opacity-50";

const TRACK_BASE: &str = "bg-muted rounded-full data-horizontal:h-1.5 data-horizontal:w-full data-vertical:h-full data-vertical:w-1.5 relative grow overflow-hidden";

const RANGE_BASE: &str = "bg-primary absolute data-horizontal:top-0 data-horizontal:bottom-0 data-vertical:left-0 data-vertical:right-0";

const THUMB_BASE: &str = "border-primary ring-ring/50 size-4 rounded-full border bg-white shadow-sm transition-[color,box-shadow] hover:ring-4 focus-visible:ring-4 focus-visible:outline-hidden absolute block shrink-0 outline-none data-horizontal:top-1/2 data-horizontal:-translate-x-1/2 data-horizontal:-translate-y-1/2 data-vertical:left-1/2 data-vertical:-translate-x-1/2 data-vertical:translate-y-1/2";

/// Where the track sits on screen, along the slider's axis.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Track {
    start: f64,
    length: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DragState {
    Idle,
    /// Pointer is down, but the track has not been measured yet.
    Pending,
    Active {
        thumb: usize,
        track: Track,
    },
}

fn clamp(value: f64, low: f64, high: f64) -> f64 {
    value.max(low).min(high)
}

fn axis(x: f64, y: f64, vertical: bool) -> f64 {
    if vertical { y } else { x }
}

/// Fraction of the track from its minimum end (bottom for vertical, left for horizontal).
fn fraction(track: Track, position: f64, vertical: bool) -> f64 {
    if track.length <= 0.0 {
        return 0.0;
    }
    let along = (position - track.start) / track.length;
    clamp(if vertical { 1.0 - along } else { along }, 0.0, 1.0)
}

fn snap(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let snapped = if step > 0.0 {
        min + ((value - min) / step).round() * step
    } else {
        value
    };
    clamp(snapped, min, max)
}

fn percent(value: f64, min: f64, max: f64) -> f64 {
    let span = max - min;
    if span > 0.0 {
        clamp((value - min) / span * 100.0, 0.0, 100.0)
    } else {
        0.0
    }
}

/// Keeps a thumb between its neighbours so that thumbs never cross.
fn constrain(values: &[f64], index: usize, value: f64, min: f64, max: f64) -> f64 {
    let low = index
        .checked_sub(1)
        .and_then(|previous| values.get(previous))
        .copied()
        .unwrap_or(min);
    let high = values.get(index + 1).copied().unwrap_or(max);
    clamp(value, low, high)
}

/// Returns `values` with thumb `index` moved to `raw`, snapped to the step and kept in bounds.
fn update(values: &[f64], index: usize, raw: f64, min: f64, max: f64, step: f64) -> Vec<f64> {
    let mut next = values.to_vec();
    if index < next.len() {
        let snapped = snap(raw, min, max, step);
        next[index] = constrain(values, index, snapped, min, max);
    }
    next
}

fn nearest(values: &[f64], value: f64) -> usize {
    values
        .iter()
        .enumerate()
        .min_by(|a, b| (a.1 - value).abs().total_cmp(&(b.1 - value).abs()))
        .map(|(index, _)| index)
        .unwrap_or(0)
}

/// The new value for a key press, or `None` if the key does not move the thumb.
fn key_target(key: Key, current: f64, min: f64, max: f64, step: f64) -> Option<f64> {
    match key {
        Key::ArrowRight | Key::ArrowUp => Some(current + step),
        Key::ArrowLeft | Key::ArrowDown => Some(current - step),
        Key::PageUp => Some(current + step * 10.0),
        Key::PageDown => Some(current - step * 10.0),
        Key::Home => Some(min),
        Key::End => Some(max),
        _ => None,
    }
}

fn commit(
    mut internal: Signal<Vec<f64>>,
    controlled: bool,
    on_value_change: Option<EventHandler<Vec<f64>>>,
    next: Vec<f64>,
) {
    if !controlled {
        internal.set(next.clone());
    }
    if let Some(handler) = on_value_change {
        handler.call(next);
    }
}

/// A slider with one thumb, or with several thumbs for a range. Pass `value` to control it, or leave it out and use `default_value`.
#[component]
pub fn Slider(
    /// Controlled values, one per thumb. When omitted, the slider keeps its own values.
    value: Option<Vec<f64>>,
    /// Initial values of an uncontrolled slider: one value gives one thumb, two values give a range.
    #[props(default = vec![0.0])]
    default_value: Vec<f64>,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default)] orientation: SliderOrientation,
    #[props(default)] disabled: bool,
    on_value_change: Option<EventHandler<Vec<f64>>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(ROOT_BASE, take_class(&mut attributes).as_deref());

    let controlled = value.is_some();
    let internal = use_signal(|| default_value);
    let values: Vec<f64> = match &value {
        Some(values) => values.clone(),
        None => internal.read().clone(),
    }
    .into_iter()
    .map(|item| clamp(item, min, max))
    .collect();

    let vertical = orientation.is_vertical();
    let mut drag = use_signal(|| DragState::Idle);
    let mut track_el = use_signal(|| None::<Rc<MountedData>>);

    let from = if values.len() > 1 {
        values.iter().copied().fold(max, f64::min)
    } else {
        min
    };
    let to = values.iter().copied().fold(from, f64::max);
    let range_start = percent(from, min, max);
    let range_size = percent(to, min, max) - range_start;
    let range_style = if vertical {
        format!("bottom: {range_start:.4}%; height: {range_size:.4}%;")
    } else {
        format!("left: {range_start:.4}%; width: {range_size:.4}%;")
    };

    let pointer_values = values.clone();
    let move_values = values.clone();

    rsx! {
        div {
            "data-slot": "slider",
            "data-orientation": orientation.as_str(),
            "data-disabled": disabled.then_some("true"),
            class: class,
            onpointerdown: move |event| {
                if disabled {
                    return;
                }
                let Some(element) = track_el.peek().clone() else {
                    return;
                };
                let point = event.client_coordinates();
                let position = axis(point.x, point.y, vertical);
                let values = pointer_values.clone();
                drag.set(DragState::Pending);
                spawn(async move {
                    let Ok(rect) = element.get_client_rect().await else {
                        drag.set(DragState::Idle);
                        return;
                    };
                    if *drag.peek() != DragState::Pending {
                        return;
                    }
                    let track = Track {
                        start: axis(rect.origin.x, rect.origin.y, vertical),
                        length: axis(rect.size.width, rect.size.height, vertical),
                    };
                    let raw = min + fraction(track, position, vertical) * (max - min);
                    let thumb = nearest(&values, raw);
                    drag.set(DragState::Active { thumb, track });
                    let next = update(&values, thumb, raw, min, max, step);
                    commit(internal, controlled, on_value_change, next);
                });
            },
            onpointermove: move |event| {
                let DragState::Active { thumb, track } = *drag.peek() else {
                    return;
                };
                if event.held_buttons().is_empty() {
                    drag.set(DragState::Idle);
                    return;
                }
                let point = event.client_coordinates();
                let raw = min + fraction(track, axis(point.x, point.y, vertical), vertical) * (max - min);
                let next = update(&move_values, thumb, raw, min, max, step);
                commit(internal, controlled, on_value_change, next);
            },
            onpointerup: move |_| {
                drag.set(DragState::Idle);
            },
            onpointercancel: move |_| {
                drag.set(DragState::Idle);
            },
            ..attributes,
            div {
                "data-slot": "slider-track",
                "data-orientation": orientation.as_str(),
                class: TRACK_BASE,
                onmounted: move |event| track_el.set(Some(event.data())),
                div {
                    "data-slot": "slider-range",
                    "data-orientation": orientation.as_str(),
                    class: RANGE_BASE,
                    style: range_style,
                }
            }
            for (index, thumb_value) in values.iter().copied().enumerate() {
                {
                    let thumb_values = values.clone();
                    let thumb_style = if vertical {
                        format!("bottom: {:.4}%;", percent(thumb_value, min, max))
                    } else {
                        format!("left: {:.4}%;", percent(thumb_value, min, max))
                    };
                    rsx! {
                        div {
                            "data-slot": "slider-thumb",
                            "data-orientation": orientation.as_str(),
                            "data-disabled": disabled.then_some("true"),
                            role: "slider",
                            tabindex: if disabled { "-1" } else { "0" },
                            "aria-orientation": orientation.as_str(),
                            aria_valuemin: "{min}",
                            aria_valuemax: "{max}",
                            aria_valuenow: "{thumb_value}",
                            aria_disabled: disabled.then_some("true"),
                            class: THUMB_BASE,
                            style: thumb_style,
                            onkeydown: move |event| {
                                if disabled {
                                    return;
                                }
                                let Some(current) = thumb_values.get(index).copied() else {
                                    return;
                                };
                                let Some(target) = key_target(event.key(), current, min, max, step) else {
                                    return;
                                };
                                event.prevent_default();
                                let next = update(&thumb_values, index, target, min, max, step);
                                commit(internal, controlled, on_value_change, next);
                            },
                        }
                    }
                }
            }
        }
    }
}
