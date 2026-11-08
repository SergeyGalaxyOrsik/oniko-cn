//! Slider: lets the user pick a value, or a range of values, by dragging thumbs along a track.
//!
//! Each thumb has `role="slider"`. Arrow keys move it by one step (`Shift` or `PageUp`/`PageDown` by
//! `large_step`), `Home` and `End` jump to the ends. Thumbs never cross. Put `dir="rtl"` on the slider
//! to flip a horizontal one.
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

const ROOT_BASE: &str = "data-horizontal:w-full data-vertical:h-full";

const CONTROL_BASE: &str = "data-vertical:min-h-40 relative flex w-full touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col";

const TRACK_BASE: &str = "bg-muted rounded-md data-horizontal:h-1 data-horizontal:w-full data-vertical:h-full data-vertical:w-1 relative grow overflow-hidden select-none";

const RANGE_BASE: &str = "bg-primary select-none data-horizontal:h-full data-vertical:w-full";

const THUMB_BASE: &str = "border-ring ring-ring/30 relative size-3 rounded-md border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-2 focus-visible:ring-2 focus-visible:outline-hidden active:ring-2 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50";

/// Where the track sits on screen, along the slider's axis. `inset` is half a thumb, so that thumbs
/// stay inside the track at both ends.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Track {
    start: f64,
    length: f64,
    inset: f64,
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

/// Everything that decides how a raw position becomes a value.
#[derive(Clone, Copy)]
struct Scale {
    min: f64,
    max: f64,
    step: f64,
    min_steps_between_values: f64,
    vertical: bool,
    rtl: bool,
}

fn clamp(value: f64, low: f64, high: f64) -> f64 {
    value.max(low).min(high)
}

fn axis(x: f64, y: f64, vertical: bool) -> f64 {
    if vertical { y } else { x }
}

fn decimals(value: f64) -> i32 {
    let text = value.to_string();
    text.split_once('.')
        .map_or(0, |(_, fraction)| fraction.len() as i32)
}

/// Fraction of the track from its minimum end (bottom for vertical, left for horizontal, right in RTL).
fn fraction(track: Track, position: f64, scale: Scale) -> f64 {
    let usable = track.length - 2.0 * track.inset;
    if usable <= 0.0 {
        return 0.0;
    }
    let along = (position - track.start - track.inset) / usable;
    let flipped = if scale.vertical || scale.rtl {
        1.0 - along
    } else {
        along
    };
    clamp(flipped, 0.0, 1.0)
}

fn snap(value: f64, scale: Scale) -> f64 {
    let snapped = if scale.step > 0.0 {
        let steps = ((value - scale.min) / scale.step).round();
        let places = decimals(scale.step).max(decimals(scale.min));
        let factor = 10f64.powi(places);
        ((scale.min + steps * scale.step) * factor).round() / factor
    } else {
        value
    };
    clamp(snapped, scale.min, scale.max)
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
fn constrain(values: &[f64], index: usize, value: f64, scale: Scale) -> f64 {
    let gap = scale.min_steps_between_values * scale.step;
    let low = index
        .checked_sub(1)
        .and_then(|previous| values.get(previous))
        .map_or(scale.min, |previous| previous + gap);
    let high = values
        .get(index + 1)
        .map_or(scale.max, |following| following - gap);
    clamp(value, low, high.max(low))
}

/// Returns `values` with thumb `index` moved to `raw`, snapped to the step and kept in bounds.
fn update(values: &[f64], index: usize, raw: f64, scale: Scale) -> Vec<f64> {
    let mut next = values.to_vec();
    if index < next.len() {
        next[index] = constrain(values, index, snap(raw, scale), scale);
    }
    next
}

fn nearest(values: &[f64], value: f64) -> usize {
    values
        .iter()
        .enumerate()
        .min_by(|a, b| (a.1 - value).abs().total_cmp(&(b.1 - value).abs()))
        .map_or(0, |(index, _)| index)
}

/// The new value for a key press, or `None` if the key does not move the thumb.
fn key_target(key: Key, shift: bool, current: f64, large_step: f64, scale: Scale) -> Option<f64> {
    let step = if shift { large_step } else { scale.step };
    let right = if scale.rtl && !scale.vertical {
        -step
    } else {
        step
    };
    match key {
        Key::ArrowRight => Some(current + right),
        Key::ArrowLeft => Some(current - right),
        Key::ArrowUp => Some(current + step),
        Key::ArrowDown => Some(current - step),
        Key::PageUp => Some(current + large_step),
        Key::PageDown => Some(current - large_step),
        Key::Home => Some(scale.min),
        Key::End => Some(scale.max),
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
    /// Defaults to a single thumb at `min`.
    default_value: Option<Vec<f64>>,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    /// Step for `Shift` + arrow keys and `PageUp`/`PageDown`.
    #[props(default = 10.0)]
    large_step: f64,
    /// Minimum distance between thumbs, in steps.
    #[props(default = 0)]
    min_steps_between_values: u32,
    #[props(default)] orientation: SliderOrientation,
    #[props(default)] disabled: bool,
    on_value_change: Option<EventHandler<Vec<f64>>>,
    /// Fires when the user finishes a drag or a key press, with the final values.
    on_value_committed: Option<EventHandler<Vec<f64>>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let rtl = attributes.iter().any(|attribute| {
        attribute.name == "dir"
            && matches!(&attribute.value, dioxus::dioxus_core::AttributeValue::Text(text) if text == "rtl")
    });
    let mut attributes = attributes;
    let class = cn!(ROOT_BASE, take_class(&mut attributes).as_deref());

    let controlled = value.is_some();
    let internal = use_signal(|| default_value.unwrap_or_else(|| vec![min]));
    let values: Vec<f64> = match &value {
        Some(values) => values.clone(),
        None => internal.read().clone(),
    }
    .into_iter()
    .map(|item| clamp(item, min, max))
    .collect();

    let vertical = orientation.is_vertical();
    let scale = Scale {
        min,
        max,
        step,
        min_steps_between_values: f64::from(min_steps_between_values),
        vertical,
        rtl,
    };
    let orientation_name = orientation.as_str();
    let mut drag = use_signal(|| DragState::Idle);
    let mut track_el = use_signal(|| None::<Rc<MountedData>>);
    let mut thumb_els = use_signal(Vec::<Option<Rc<MountedData>>>::new);
    let dragging = *drag.read() != DragState::Idle;
    let dragging_attr = dragging.then_some("");
    let disabled_attr = disabled.then_some("");

    let from = if values.len() > 1 {
        values.iter().copied().fold(max, f64::min)
    } else {
        min
    };
    let to = values.iter().copied().fold(from, f64::max);
    let range_start = percent(from, min, max);
    let range_size = percent(to, min, max) - range_start;
    let range_style = match (vertical, rtl) {
        (true, _) => {
            format!("position: absolute; bottom: {range_start:.4}%; height: {range_size:.4}%;")
        }
        (false, true) => {
            format!("position: absolute; right: {range_start:.4}%; width: {range_size:.4}%;")
        }
        (false, false) => {
            format!("position: absolute; left: {range_start:.4}%; width: {range_size:.4}%;")
        }
    };

    let pointer_values = values.clone();
    let move_values = values.clone();
    let up_values = values.clone();

    rsx! {
        div {
            "data-slot": "slider",
            role: "group",
            "data-orientation": orientation_name,
            "data-disabled": disabled_attr,
            "data-dragging": dragging_attr,
            class: class,
            ..attributes,
            div {
                "data-orientation": orientation_name,
                "data-disabled": disabled_attr,
                "data-dragging": dragging_attr,
                class: CONTROL_BASE,
                onpointerdown: move |event| {
                    if disabled {
                        return;
                    }
                    let Some(element) = track_el.peek().clone() else {
                        return;
                    };
                    let first_thumb = thumb_els.peek().first().cloned().flatten();
                    let point = event.client_coordinates();
                    let position = axis(point.x, point.y, vertical);
                    let values = pointer_values.clone();
                    drag.set(DragState::Pending);
                    spawn(async move {
                        let Ok(rect) = element.get_client_rect().await else {
                            drag.set(DragState::Idle);
                            return;
                        };
                        let thumb_size = match first_thumb {
                            Some(thumb) => thumb.get_client_rect().await.map_or(0.0, |rect| {
                                axis(rect.size.width, rect.size.height, vertical)
                            }),
                            None => 0.0,
                        };
                        if *drag.peek() != DragState::Pending {
                            return;
                        }
                        let track = Track {
                            start: axis(rect.origin.x, rect.origin.y, vertical),
                            length: axis(rect.size.width, rect.size.height, vertical),
                            inset: thumb_size / 2.0,
                        };
                        let raw = min + fraction(track, position, scale) * (max - min);
                        let thumb = nearest(&values, raw);
                        drag.set(DragState::Active { thumb, track });
                        let next = update(&values, thumb, raw, scale);
                        commit(internal, controlled, on_value_change, next);
                        let target = thumb_els.peek().get(thumb).cloned().flatten();
                        if let Some(target) = target {
                            let _ = target.set_focus(true).await;
                        }
                    });
                },
                onpointermove: move |event| {
                    let DragState::Active { thumb, track } = *drag.peek() else {
                        return;
                    };
                    if event.held_buttons().is_empty() {
                        drag.set(DragState::Idle);
                        if let Some(handler) = on_value_committed {
                            handler.call(move_values.clone());
                        }
                        return;
                    }
                    let point = event.client_coordinates();
                    let raw = min
                        + fraction(track, axis(point.x, point.y, vertical), scale) * (max - min);
                    let next = update(&move_values, thumb, raw, scale);
                    commit(internal, controlled, on_value_change, next);
                },
                onpointerup: move |_| {
                    let was_active = matches!(*drag.peek(), DragState::Active { .. });
                    drag.set(DragState::Idle);
                    if was_active && let Some(handler) = on_value_committed {
                        handler.call(up_values.clone());
                    }
                },
                onpointercancel: move |_| {
                    drag.set(DragState::Idle);
                },
                div {
                    "data-slot": "slider-track",
                    "data-orientation": orientation_name,
                    "data-disabled": disabled_attr,
                    "data-dragging": dragging_attr,
                    class: TRACK_BASE,
                    onmounted: move |event| track_el.set(Some(event.data())),
                    div {
                        "data-slot": "slider-range",
                        "data-orientation": orientation_name,
                        "data-disabled": disabled_attr,
                        "data-dragging": dragging_attr,
                        class: RANGE_BASE,
                        style: range_style,
                    }
                }
                for (index, thumb_value) in values.iter().copied().enumerate() {
                    {
                        let thumb_values = values.clone();
                        let along = percent(thumb_value, min, max);
                        let thumb_style = match (vertical, rtl) {
                            (true, _) => format!(
                                "position: absolute; left: 50%; bottom: {along:.4}%; translate: -50% {along:.4}%;"
                            ),
                            (false, true) => format!(
                                "position: absolute; top: 50%; right: {along:.4}%; translate: {along:.4}% -50%;"
                            ),
                            (false, false) => format!(
                                "position: absolute; top: 50%; left: {along:.4}%; translate: -{along:.4}% -50%;"
                            ),
                        };
                        rsx! {
                            div {
                                "data-slot": "slider-thumb",
                                "data-index": "{index}",
                                "data-orientation": orientation_name,
                                "data-disabled": disabled_attr,
                                "data-dragging": dragging_attr,
                                role: "slider",
                                tabindex: if disabled { "-1" } else { "0" },
                                "aria-orientation": orientation_name,
                                "aria-valuemin": "{min}",
                                "aria-valuemax": "{max}",
                                "aria-valuenow": "{thumb_value}",
                                "aria-disabled": disabled.then_some("true"),
                                class: THUMB_BASE,
                                style: thumb_style,
                                onmounted: move |event| {
                                    let mut elements = thumb_els.write();
                                    if elements.len() <= index {
                                        elements.resize(index + 1, None);
                                    }
                                    elements[index] = Some(event.data());
                                },
                                onkeydown: move |event| {
                                    if disabled {
                                        return;
                                    }
                                    let Some(current) = thumb_values.get(index).copied() else {
                                        return;
                                    };
                                    let shift = event.modifiers().shift();
                                    let Some(target) = key_target(event.key(), shift, current, large_step, scale) else {
                                        return;
                                    };
                                    event.prevent_default();
                                    let next = update(&thumb_values, index, target, scale);
                                    commit(internal, controlled, on_value_change, next.clone());
                                    if let Some(handler) = on_value_committed {
                                        handler.call(next);
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
