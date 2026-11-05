//! Context: shows how much of a model's context window a conversation has used, in a hover card with a token and cost breakdown.
//!
//! Costs are not computed here: pass the per-part costs in `ContextUsage` from your own pricing table.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// Token counts and costs for one response. Costs are optional, so a missing price is shown as zero.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ContextUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub reasoning_tokens: u64,
    pub cache_tokens: u64,
    pub input_cost: Option<f64>,
    pub output_cost: Option<f64>,
    pub reasoning_cost: Option<f64>,
    pub cache_cost: Option<f64>,
}

impl ContextUsage {
    /// Sum of the known part costs, or `None` when no part has a cost.
    pub fn total_cost(&self) -> Option<f64> {
        let costs: Vec<f64> = [
            self.input_cost,
            self.output_cost,
            self.reasoning_cost,
            self.cache_cost,
        ]
        .into_iter()
        .flatten()
        .collect();
        (!costs.is_empty()).then(|| costs.iter().sum::<f64>())
    }
}

#[derive(Clone, Copy)]
struct ContextValue {
    used_tokens: Signal<u64>,
    max_tokens: Signal<u64>,
    usage: Signal<Option<ContextUsage>>,
}

impl ContextValue {
    fn percent(self) -> f64 {
        let max = (self.max_tokens)();
        if max == 0 {
            0.0
        } else {
            (self.used_tokens)() as f64 / max as f64 * 100.0
        }
    }
}

/// `12.3%`, with at most one decimal and no trailing `.0`.
fn format_percent(percent: f64) -> String {
    let rounded = (percent * 10.0).round() / 10.0;
    if rounded.fract() == 0.0 {
        format!("{rounded:.0}%")
    } else {
        format!("{rounded:.1}%")
    }
}

/// `999`, `1.2K`, `12K`, `1.2M`, `3B`.
fn format_compact(tokens: u64) -> String {
    const UNITS: [(f64, &str); 4] = [(1e12, "T"), (1e9, "B"), (1e6, "M"), (1e3, "K")];
    let value = tokens as f64;
    for (scale, suffix) in UNITS {
        if value >= scale {
            let scaled = value / scale;
            return if scaled < 10.0 {
                let text = format!("{scaled:.1}");
                format!("{}{suffix}", text.strip_suffix(".0").unwrap_or(&text))
            } else {
                format!("{scaled:.0}{suffix}")
            };
        }
    }
    tokens.to_string()
}

fn format_cost(cost: f64) -> String {
    format!("${cost:.2}")
}

/// Owns the usage state and the hover card. Put `ContextTrigger` and `ContextContent` inside it.
#[component]
pub fn Context(
    used_tokens: u64,
    max_tokens: u64,
    usage: Option<ContextUsage>,
    /// Milliseconds the pointer must rest on the trigger before the card opens.
    #[props(default = 10)]
    open_delay: u32,
    /// Milliseconds after the pointer leaves before the card closes.
    #[props(default = 100)]
    close_delay: u32,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("inline-block", take_class(&mut attributes).as_deref());
    // `use_prop` is a hook, so it cannot run inside the `use_context_provider` closure.
    let used_tokens = use_prop(used_tokens);
    let max_tokens = use_prop(max_tokens);
    let usage = use_prop(usage);
    use_context_provider(|| ContextValue {
        used_tokens,
        max_tokens,
        usage,
    });

    rsx! {
        div {
            "data-slot": "context",
            class: class,
            ..attributes,
            HoverCard { open_delay: open_delay, close_delay: close_delay, {children} }
        }
    }
}

/// The button-looking element that shows the percentage used and opens the card.
///
/// The button look comes from `ContextTrigger` itself; `attributes` and `class` go to the inner `span`.
#[component]
pub fn ContextTrigger(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = button_variants(ButtonVariant::Outline, ButtonSize::Sm, None);
    let inner_class = cn!(
        "inline-flex items-center gap-1.5",
        take_class(&mut attributes).as_deref()
    );
    let context = use_context::<ContextValue>();
    let percent = context.percent();

    rsx! {
        HoverCardTrigger { class: class,
            span {
                "data-slot": "context-trigger",
                class: inner_class,
                ..attributes,
                {children.unwrap_or_else(|| rsx! {
                    ContextIcon { percent: percent }
                    span { "{format_percent(percent)} context" }
                })}
            }
        }
    }
}

#[component]
fn ContextIcon(percent: f64) -> Element {
    const RADIUS: f64 = 10.0;
    let circumference = 2.0 * std::f64::consts::PI * RADIUS;
    let fraction = (percent / 100.0).clamp(0.0, 1.0);
    let offset = format!("{:.3}", circumference * (1.0 - fraction));
    let dash = format!("{circumference:.3}");

    rsx! {
        svg {
            "data-slot": "context-icon",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            "aria-hidden": "true",
            class: "shrink-0",
            circle {
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "2",
                opacity: "0.25",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_dasharray: dash,
                stroke_dashoffset: offset,
                transform: "rotate(-90 12 12)",
            }
        }
    }
}

/// The floating panel that appears on hover. Shown only while the card is open.
///
/// `attributes` and `class` go to the inner `div` that wraps the header, body and footer.
#[component]
pub fn ContextContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("divide-y", take_class(&mut attributes).as_deref());

    rsx! {
        HoverCardContent { class: "min-w-60 overflow-hidden p-0",
            div {
                "data-slot": "context-content",
                class: class,
                ..attributes,
                {children}
            }
        }
    }
}

/// Percentage, `used / max` tokens and a progress bar. Pass children to replace the default layout.
#[component]
pub fn ContextContentHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "w-full space-y-2 p-3",
        take_class(&mut attributes).as_deref()
    );
    let context = use_context::<ContextValue>();
    let percent = context.percent();
    let used = format_compact((context.used_tokens)());
    let max = format_compact((context.max_tokens)());

    rsx! {
        div {
            "data-slot": "context-content-header",
            class: class,
            ..attributes,
            {children.unwrap_or_else(|| rsx! {
                div { class: "flex items-center justify-between gap-3 text-xs",
                    span { class: "font-medium", "{format_percent(percent)}" }
                    span { class: "font-mono text-muted-foreground", "{used} / {max}" }
                }
                ContextProgress { value: percent }
            })}
        }
    }
}

#[component]
fn ContextProgress(value: f64) -> Element {
    let value = value.clamp(0.0, 100.0);
    let width = format!("width: {value:.2}%");
    let now = format!("{value:.1}");

    rsx! {
        div {
            "data-slot": "context-progress",
            role: "progressbar",
            aria_label: "Context used",
            aria_valuemin: "0",
            aria_valuemax: "100",
            aria_valuenow: now,
            class: "relative h-2 w-full overflow-hidden rounded-full bg-secondary",
            div {
                "data-slot": "context-progress-indicator",
                class: "h-full bg-primary transition-all",
                style: width,
            }
        }
    }
}

/// A padded area for your own content inside `ContextContent`.
#[component]
pub fn ContextContentBody(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("w-full p-3", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "context-content-body",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Total cost of the usage, or `$0.00` when no part has a cost. Pass children to replace it.
#[component]
pub fn ContextContentFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex w-full items-center justify-between gap-3 bg-secondary p-3 text-xs",
        take_class(&mut attributes).as_deref()
    );
    let context = use_context::<ContextValue>();
    let cost = (context.usage)()
        .and_then(|usage| usage.total_cost())
        .unwrap_or(0.0);

    rsx! {
        div {
            "data-slot": "context-content-footer",
            class: class,
            ..attributes,
            {children.unwrap_or_else(|| rsx! {
                span { class: "text-muted-foreground", "Total cost" }
                span { class: "font-mono", "{format_cost(cost)}" }
            })}
        }
    }
}

/// One row of the usage breakdown. Hidden when it has no tokens, unless children are given.
fn usage_row(
    slot: &'static str,
    label: &'static str,
    tokens: u64,
    cost: Option<f64>,
    attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center justify-between gap-3 text-xs",
        take_class(&mut attributes).as_deref()
    );
    if tokens == 0 && children.is_none() {
        return rsx! {};
    }
    let tokens_text = format!("{} tokens", format_compact(tokens));
    let cost_text = cost.map(format_cost);

    rsx! {
        div {
            "data-slot": slot,
            class: class,
            ..attributes,
            {children.unwrap_or_else(|| rsx! {
                span { class: "text-muted-foreground", "{label}" }
                span { class: "flex items-center gap-2 font-mono",
                    span { "{tokens_text}" }
                    if let Some(cost_text) = cost_text {
                        span { class: "text-muted-foreground", "{cost_text}" }
                    }
                }
            })}
        }
    }
}

#[component]
pub fn ContextInputUsage(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ContextValue>();
    let usage = (context.usage)().unwrap_or_default();
    usage_row(
        "context-input-usage",
        "Input",
        usage.input_tokens,
        usage.input_cost,
        attributes,
        children,
    )
}

#[component]
pub fn ContextOutputUsage(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ContextValue>();
    let usage = (context.usage)().unwrap_or_default();
    usage_row(
        "context-output-usage",
        "Output",
        usage.output_tokens,
        usage.output_cost,
        attributes,
        children,
    )
}

#[component]
pub fn ContextReasoningUsage(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ContextValue>();
    let usage = (context.usage)().unwrap_or_default();
    usage_row(
        "context-reasoning-usage",
        "Reasoning",
        usage.reasoning_tokens,
        usage.reasoning_cost,
        attributes,
        children,
    )
}

#[component]
pub fn ContextCacheUsage(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<ContextValue>();
    let usage = (context.usage)().unwrap_or_default();
    usage_row(
        "context-cache-usage",
        "Cache",
        usage.cache_tokens,
        usage.cache_cost,
        attributes,
        children,
    )
}
