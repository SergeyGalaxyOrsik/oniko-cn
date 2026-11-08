//! Progress: displays how far a task has come, as a bar with an optional label and value.
//! `Progress` renders its children, then the track and indicator. Add `ProgressLabel` and
//! `ProgressValue` as children. Pass `value: None` for an indeterminate task.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const PROGRESS_BASE: &str = "flex flex-wrap gap-3";
const PROGRESS_TRACK_BASE: &str =
    "bg-muted h-1 rounded-md relative flex w-full items-center overflow-x-hidden";
const PROGRESS_INDICATOR_BASE: &str = "bg-primary h-full transition-all";
const PROGRESS_LABEL_BASE: &str = "text-xs/relaxed font-medium";
const PROGRESS_VALUE_BASE: &str = "text-muted-foreground ml-auto text-xs/relaxed tabular-nums";

#[derive(Clone, Copy, Debug, PartialEq)]
struct ProgressState {
    value: Option<f64>,
    min: f64,
    max: f64,
}

impl ProgressState {
    fn value(self) -> Option<f64> {
        self.value.map(|value| value.clamp(self.min, self.max))
    }

    fn percent(self) -> Option<f64> {
        let span = self.max - self.min;
        let value = self.value()?;
        if span > 0.0 {
            Some(((value - self.min) / span * 100.0).clamp(0.0, 100.0))
        } else {
            Some(0.0)
        }
    }

    fn status(self) -> &'static str {
        match self.value() {
            None => "indeterminate",
            Some(value) if value >= self.max => "complete",
            Some(_) => "progressing",
        }
    }

    fn text(self) -> String {
        self.value()
            .map(|value| format!("{value:.0}%"))
            .unwrap_or_default()
    }
}

#[derive(Clone, Copy)]
struct ProgressContext {
    id: usize,
    state: Signal<ProgressState>,
    has_label: Signal<bool>,
}

#[component]
pub fn Progress(
    value: Option<f64>,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let next = ProgressState { value, min, max };
    let mut state = use_signal(|| next);
    if *state.peek() != next {
        state.set(next);
    }
    let has_label = use_signal(|| false);
    use_context_provider(|| ProgressContext {
        id,
        state,
        has_label,
    });

    let mut attributes = attributes;
    let class = cn!(PROGRESS_BASE, take_class(&mut attributes).as_deref());
    let value_text = match next.value() {
        Some(_) => next.text(),
        None => "indeterminate progress".to_string(),
    };

    rsx! {
        div {
            "data-slot": "progress",
            "data-status": next.status(),
            role: "progressbar",
            "aria-valuemin": "{min}",
            "aria-valuemax": "{max}",
            "aria-valuenow": next.value().map(|value| value.to_string()),
            "aria-valuetext": value_text,
            "aria-labelledby": has_label().then(|| format!("progress-label-{id}")),
            class: class,
            ..attributes,
            {children}
            ProgressTrack {
                ProgressIndicator {}
            }
        }
    }
}

#[component]
pub fn ProgressTrack(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ProgressContext>();
    let mut attributes = attributes;
    let class = cn!(PROGRESS_TRACK_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "progress-track",
            "data-status": context.state.read().status(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ProgressIndicator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ProgressContext>();
    let state = *context.state.read();
    let width = state.percent().map(|percent| format!("width: {percent}%"));

    let mut attributes = attributes;
    let class = cn!(
        PROGRESS_INDICATOR_BASE,
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "progress-indicator",
            "data-status": state.status(),
            style: width,
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ProgressLabel(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ProgressContext>();
    let mut has_label = context.has_label;
    use_effect(move || has_label.set(true));
    use_drop(move || has_label.set(false));

    let mut attributes = attributes;
    let class = cn!(PROGRESS_LABEL_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        span {
            id: "progress-label-{context.id}",
            "data-slot": "progress-label",
            "data-status": context.state.read().status(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The formatted value, such as `56%`. Children replace the default text.
#[component]
pub fn ProgressValue(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ProgressContext>();
    let state = *context.state.read();

    let mut attributes = attributes;
    let class = cn!(PROGRESS_VALUE_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "progress-value",
            "data-status": state.status(),
            "aria-hidden": "true",
            class: class,
            ..attributes,
            {children.unwrap_or_else(|| rsx! { "{state.text()}" })}
        }
    }
}
