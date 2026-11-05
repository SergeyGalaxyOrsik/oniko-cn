//! Test Results: summarizes a test run with counts, a progress bar and collapsible suites of tests.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::badge::{Badge, BadgeVariant};
use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TestStatus {
    #[default]
    Passed,
    Failed,
    Skipped,
    Running,
}

impl TestStatus {
    /// Value of the `data-status` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::Running => "running",
        }
    }

    fn text_class(self) -> &'static str {
        match self {
            Self::Passed => "text-emerald-600 dark:text-emerald-400",
            Self::Failed => "text-destructive",
            Self::Skipped => "text-muted-foreground",
            Self::Running => "text-blue-600 dark:text-blue-400",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TestRunSummary {
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub total: u32,
    pub duration_ms: Option<u64>,
}

#[derive(Clone, Copy)]
struct TestResultsContext {
    summary: Signal<Option<TestRunSummary>>,
}

#[derive(Clone, Copy)]
struct TestSuiteContext {
    name: Signal<String>,
    status: Signal<TestStatus>,
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn read_summary() -> Option<TestRunSummary> {
    try_use_context::<TestResultsContext>().and_then(|context| (context.summary)())
}

fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{ms}ms")
    } else {
        format!("{:.2}s", ms as f64 / 1000.0)
    }
}

fn percent(part: u32, total: u32) -> f64 {
    if total == 0 {
        0.0
    } else {
        f64::from(part) * 100.0 / f64::from(total)
    }
}

fn status_icon(status: TestStatus, class: &str) -> Element {
    let class = cn!(class, status.text_class());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: class,
            "aria-hidden": "true",
            circle { cx: "12", cy: "12", r: "10" }
            {match status {
                TestStatus::Passed => rsx! { path { d: "m9 12 2 2 4-4" } },
                TestStatus::Failed => rsx! {
                    path { d: "m15 9-6 6" }
                    path { d: "m9 9 6 6" }
                },
                TestStatus::Skipped => rsx! { path { d: "M8 12h8" } },
                TestStatus::Running => rsx! { path { d: "M12 6v6l4 2" } },
            }}
        }
    }
}

#[component]
pub fn TestResults(
    summary: Option<TestRunSummary>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col overflow-hidden rounded-xl border border-border bg-background",
        take_class(&mut attributes).as_deref()
    );
    let summary = use_prop(summary);
    use_context_provider(|| TestResultsContext { summary });

    rsx! {
        div {
            "data-slot": "test-results",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TestResultsHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-wrap items-center justify-between gap-4 border-b border-border px-4 py-3",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "test-results-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Passed, failed and skipped counts from the enclosing `TestResults` summary.
#[component]
pub fn TestResultsSummary(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-wrap items-center gap-2",
        take_class(&mut attributes).as_deref()
    );
    let summary = read_summary().unwrap_or_default();

    rsx! {
        div {
            "data-slot": "test-results-summary",
            class: class,
            ..attributes,
            Badge {
                variant: BadgeVariant::Secondary,
                class: "tabular-nums text-emerald-600 dark:text-emerald-400",
                {format!("{} passed", summary.passed)}
            }
            Badge {
                variant: BadgeVariant::Destructive,
                class: "tabular-nums",
                {format!("{} failed", summary.failed)}
            }
            Badge {
                variant: BadgeVariant::Outline,
                class: "tabular-nums text-muted-foreground",
                {format!("{} skipped", summary.skipped)}
            }
        }
    }
}

/// Run duration from the enclosing `TestResults` summary. Renders nothing when no duration is set.
#[component]
pub fn TestResultsDuration(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sm tabular-nums text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );
    let Some(duration_ms) = read_summary().and_then(|summary| summary.duration_ms) else {
        return rsx! {};
    };

    rsx! {
        span {
            "data-slot": "test-results-duration",
            class: class,
            ..attributes,
            {format_duration(duration_ms)}
        }
    }
}

/// Bar of passed and failed shares of the run, with a `passed/total` label.
#[component]
pub fn TestResultsProgress(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-1.5 px-4 py-3",
        take_class(&mut attributes).as_deref()
    );
    let summary = read_summary().unwrap_or_default();
    let passed = percent(summary.passed, summary.total);
    let failed = percent(summary.failed, summary.total);

    rsx! {
        div {
            "data-slot": "test-results-progress",
            class: class,
            ..attributes,
            span {
                "data-slot": "test-results-progress-label",
                class: "text-xs tabular-nums text-muted-foreground",
                {format!("{}/{}", summary.passed, summary.total)}
            }
            div {
                role: "progressbar",
                "aria-valuemin": "0",
                "aria-valuemax": "{summary.total}",
                "aria-valuenow": "{summary.passed}",
                "data-slot": "test-results-progress-track",
                class: "flex h-2 w-full overflow-hidden rounded-full bg-muted",
                div {
                    "data-slot": "test-results-progress-passed",
                    class: "h-full bg-emerald-500 transition-all",
                    style: "width: {passed}%",
                }
                div {
                    "data-slot": "test-results-progress-failed",
                    class: "h-full bg-destructive transition-all",
                    style: "width: {failed}%",
                }
            }
        }
    }
}

#[component]
pub fn TestResultsContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-2 p-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "test-results-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A named group of tests that collapses under its `TestSuiteName` trigger.
#[component]
pub fn TestSuite(
    name: String,
    #[props(default)] status: TestStatus,
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default = true)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col overflow-hidden rounded-md border border-border",
        take_class(&mut attributes).as_deref()
    );
    let name = use_prop(name);
    let status = use_prop(status);
    use_context_provider(|| TestSuiteContext { name, status });

    rsx! {
        div {
            "data-slot": "test-suite",
            "data-status": status().as_str(),
            class: class,
            ..attributes,
            Collapsible {
                open: open,
                default_open: default_open,
                on_open_change: on_open_change,
                class: "flex flex-col",
                {children}
            }
        }
    }
}

/// Trigger of a `TestSuite`: a chevron, the suite status and name, then `children` (e.g. `TestSuiteStats`).
/// It takes no extra attributes, because `CollapsibleTrigger` cannot receive them.
#[component]
pub fn TestSuiteName(children: Element) -> Element {
    let context = use_context::<TestSuiteContext>();
    let name = (context.name)();
    let status = (context.status)();

    rsx! {
        CollapsibleTrigger {
            "data-slot": "test-suite-name",
            class: "group flex w-full items-center justify-between gap-3 px-3 py-2 text-left text-sm font-medium transition-colors hover:bg-muted/50",
            span {
                "data-slot": "test-suite-name-label",
                class: "flex min-w-0 items-center gap-2",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    class: "size-4 shrink-0 transition-transform group-data-[state=closed]:-rotate-90",
                    "aria-hidden": "true",
                    path { d: "m6 9 6 6 6-6" }
                }
                {status_icon(status, "size-4 shrink-0")}
                span { class: "truncate", {name} }
            }
            {children}
        }
    }
}

/// Inline passed, failed and skipped counts of a suite. Each count is shown only when given.
#[component]
pub fn TestSuiteStats(
    passed: Option<u32>,
    failed: Option<u32>,
    skipped: Option<u32>,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex shrink-0 items-center gap-2 text-xs tabular-nums text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "test-suite-stats",
            class: class,
            ..attributes,
            {passed.map(|count| rsx! {
                span { "data-slot": "test-suite-stat-passed", class: "text-emerald-600 dark:text-emerald-400", {format!("{count} passed")} }
            })}
            {failed.map(|count| rsx! {
                span { "data-slot": "test-suite-stat-failed", class: "text-destructive", {format!("{count} failed")} }
            })}
            {skipped.map(|count| rsx! {
                span { "data-slot": "test-suite-stat-skipped", {format!("{count} skipped")} }
            })}
        }
    }
}

/// Content of a `TestSuite`, shown only while the suite is open.
#[component]
pub fn TestSuiteContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-1 border-t border-border py-1",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        CollapsibleContent {
            class: "flex flex-col",
            div {
                "data-slot": "test-suite-content",
                class: class,
                ..attributes,
                {children}
            }
        }
    }
}

/// A single test row with its status, name and duration. Put a `TestError` inside for a failure.
#[component]
pub fn Test(
    name: String,
    #[props(default)] status: TestStatus,
    duration_ms: Option<u64>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-2 px-3 py-1.5",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "test",
            "data-status": status.as_str(),
            class: class,
            ..attributes,
            div {
                "data-slot": "test-row",
                class: "flex items-center gap-2 text-sm",
                TestStatusIcon { status: status }
                TestName { name: name }
                TestDuration { duration_ms: duration_ms }
            }
            {children}
        }
    }
}

#[component]
pub fn TestStatusIcon(
    #[props(default)] status: TestStatus,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "inline-flex shrink-0 items-center",
        take_class(&mut attributes).as_deref()
    );
    let icon_class = if status == TestStatus::Running {
        "size-4 animate-pulse"
    } else {
        "size-4"
    };

    rsx! {
        span {
            "data-slot": "test-status",
            "data-status": status.as_str(),
            class: class,
            ..attributes,
            {status_icon(status, icon_class)}
        }
    }
}

#[component]
pub fn TestName(
    name: String,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "min-w-0 flex-1 truncate",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "test-name",
            class: class,
            ..attributes,
            {name}
        }
    }
}

/// Duration of a test. Renders nothing when `duration_ms` is `None`.
#[component]
pub fn TestDuration(
    duration_ms: Option<u64>,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "shrink-0 text-xs tabular-nums text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );
    let Some(duration_ms) = duration_ms else {
        return rsx! {};
    };

    rsx! {
        span {
            "data-slot": "test-duration",
            class: class,
            ..attributes,
            {format_duration(duration_ms)}
        }
    }
}

/// Red box that holds the message and stack of a failed test.
#[component]
pub fn TestError(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-2 rounded-md border border-destructive/30 bg-destructive/10 px-3 py-2 text-xs text-destructive",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "test-error",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TestErrorMessage(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("font-medium", take_class(&mut attributes).as_deref());

    rsx! {
        p {
            "data-slot": "test-error-message",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Preformatted stack trace of a failed test.
#[component]
pub fn TestErrorStack(
    #[props(extends = GlobalAttributes, extends = pre)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "overflow-x-auto font-mono whitespace-pre-wrap text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        pre {
            "data-slot": "test-error-stack",
            class: class,
            ..attributes,
            {children}
        }
    }
}
