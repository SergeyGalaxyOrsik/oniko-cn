//! Progress: displays how far a task has come, as a bar with an optional label and value.
//! ProgressTrack, ProgressIndicator, ProgressLabel and ProgressValue are its parts.
//! Pass `value: None` for an indeterminate task.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const PROGRESS_BASE: &str = "relative flex w-full flex-wrap items-center gap-3";
const PROGRESS_TRACK_BASE: &str =
    "bg-muted h-1.5 rounded-full relative flex w-full items-center overflow-hidden";
const PROGRESS_INDICATOR_BASE: &str = "bg-primary h-full transition-all";
const PROGRESS_LABEL_BASE: &str = "text-sm font-medium";
const PROGRESS_VALUE_BASE: &str = "text-muted-foreground ml-auto text-sm tabular-nums";

#[derive(Clone, Copy, Debug, PartialEq)]
struct ProgressState {
    value: Option<f64>,
    max: f64,
}

impl ProgressState {
    fn percent(self) -> Option<f64> {
        let value = self.value?;
        let percent = value / self.max * 100.0;
        if self.max > 0.0 && percent.is_finite() {
            Some(percent.clamp(0.0, 100.0))
        } else {
            Some(0.0)
        }
    }

    fn status(self) -> &'static str {
        match self.percent() {
            None => "indeterminate",
            Some(percent) if percent >= 100.0 => "complete",
            Some(_) => "progressing",
        }
    }
}

#[component]
pub fn Progress(
    value: Option<f64>,
    #[props(default = 100.0)] max: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let next = ProgressState { value, max };
    let mut state = use_context_provider(|| Signal::new(next));
    if *state.peek() != next {
        state.set(next);
    }

    let mut attributes = attributes;
    let class = cn!(PROGRESS_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "progress",
            "data-status": next.status(),
            role: "progressbar",
            "aria-valuemin": "0",
            "aria-valuemax": "{max}",
            "aria-valuenow": value,
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ProgressTrack(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(PROGRESS_TRACK_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "progress-track",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ProgressIndicator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let state = use_context::<Signal<ProgressState>>();
    let progress = *state.read();
    let width = match progress.percent() {
        Some(percent) => format!("{percent}%"),
        None => "30%".to_string(),
    };
    let pulse = if progress.value.is_none() {
        "animate-pulse"
    } else {
        ""
    };

    let mut attributes = attributes;
    let class = cn!(
        PROGRESS_INDICATOR_BASE,
        pulse,
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "progress-indicator",
            style: "width: {width}",
            class: class,
            ..attributes,
        }
    }
}

#[component]
pub fn ProgressLabel(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(PROGRESS_LABEL_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "progress-label",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ProgressValue(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let state = use_context::<Signal<ProgressState>>();
    let progress = *state.read();
    let text = match progress.percent() {
        Some(percent) => format!("{percent:.0}%"),
        None => String::new(),
    };

    let mut attributes = attributes;
    let class = cn!(PROGRESS_VALUE_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "progress-value",
            class: class,
            ..attributes,
            {children.unwrap_or_else(|| rsx! { "{text}" })}
        }
    }
}
