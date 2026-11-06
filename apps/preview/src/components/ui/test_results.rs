//! Test results: a test run by the agent, built from `Trace` and `CodeBlock`. The summary line
//! counts what passed and failed. Open it to see each suite, its tests and the error of each
//! failure.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::code_block::{CodeBlock, CodeBlockContent, CodeBlockSize};
use super::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
}

#[derive(Clone, PartialEq)]
pub struct Test {
    pub name: String,
    pub status: TestStatus,
    pub error: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<Test>,
}

fn count(tests: &[&Test], status: TestStatus) -> usize {
    tests.iter().filter(|test| test.status == status).count()
}

/// "5 tests, 1 failed, 1 skipped"
fn format_counts(tests: &[&Test]) -> String {
    let mut parts = vec![format!("{} tests", tests.len())];
    let failed = count(tests, TestStatus::Failed);
    let skipped = count(tests, TestStatus::Skipped);
    if failed > 0 {
        parts.push(format!("{failed} failed"));
    }
    if skipped > 0 {
        parts.push(format!("{skipped} skipped"));
    }
    parts.join(", ")
}

fn error_status(failed: bool) -> TraceStatus {
    if failed {
        TraceStatus::Error
    } else {
        TraceStatus::Complete
    }
}

#[component]
fn Icon(paths: &'static [&'static str]) -> Element {
    rsx! {
        TraceIcon {
            svg {
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                for d in paths {
                    path { d: *d }
                }
            }
        }
    }
}

const CHECK: &[&str] = &["M20 6 9 17l-5-5"];
const X: &[&str] = &["M18 6 6 18", "m6 6 12 12"];
const SKIPPED: &[&str] = &[
    "M10.1 2.182a10 10 0 0 1 3.8 0",
    "M13.9 21.818a10 10 0 0 1-3.8 0",
    "M17.609 3.721a10 10 0 0 1 2.69 2.7",
    "M2.182 13.9a10 10 0 0 1 0-3.8",
    "M20.279 17.609a10 10 0 0 1-2.7 2.69",
    "M21.818 10.1a10 10 0 0 1 0 3.8",
    "M3.721 6.391a10 10 0 0 1 2.7-2.69",
    "M6.391 20.279a10 10 0 0 1-2.69-2.7",
];
const FILE_CODE: &[&str] = &[
    "M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z",
    "M14 2v5a1 1 0 0 0 1 1h5",
    "M10 12.5 8 15l2 2.5",
    "m14 12.5 2 2.5-2 2.5",
];
const FLASK: &[&str] = &[
    "M14 2v6a2 2 0 0 0 .245.96l5.51 10.08A2 2 0 0 1 18 22H6a2 2 0 0 1-1.755-2.96l5.51-10.08A2 2 0 0 0 10 8V2",
    "M6.453 15h11.094",
    "M8.5 2h7",
];

/// One test as a line of a `Trace`. A failed test expands to show its error.
#[component]
pub fn TestRow(test: Test, #[props(default)] default_open: bool) -> Element {
    if test.status == TestStatus::Failed {
        return rsx! {
            TraceCollapsible { default_open,
                TraceCollapsibleTrigger {
                    TraceLine { status: TraceStatus::Error,
                        Icon { paths: X }
                        TraceTitle { "{test.name}" }
                    }
                }
                TraceCollapsibleContent {
                    CodeBlock { size: CodeBlockSize::Sm,
                        CodeBlockContent {
                            wrap: true,
                            class: "max-h-64",
                            code: test.error.unwrap_or_default(),
                        }
                    }
                }
            }
        };
    }

    let skipped = test.status == TestStatus::Skipped;
    rsx! {
        TraceLine {
            Icon { paths: if skipped { SKIPPED } else { CHECK } }
            TraceTitle {
                if skipped {
                    "Skipped {test.name}"
                } else {
                    "{test.name}"
                }
            }
        }
    }
}

/// A test run as one line of a `Trace`, with a collapsible line for each suite.
/// `default_open` opens the run, `open_failures` opens the error of each failed test.
/// Suites with a failure start open.
#[component]
pub fn TestResults(
    suites: Vec<TestSuite>,
    #[props(default)] default_open: bool,
    #[props(default)] open_failures: bool,
) -> Element {
    let tests: Vec<&Test> = suites.iter().flat_map(|suite| &suite.tests).collect();
    let failed = count(&tests, TestStatus::Failed) > 0;

    rsx! {
        TraceCollapsible { default_open,
            TraceCollapsibleTrigger {
                TraceLine { status: error_status(failed),
                    Icon { paths: FLASK }
                    TraceTitle { "Ran {format_counts(&tests)}" }
                }
            }
            TraceCollapsibleContent { keep_mounted: true,
                Trace {
                    for suite in suites.iter() {
                        TestSuiteLine { key: "{suite.name}", suite: suite.clone(), open_failures }
                    }
                }
            }
        }
    }
}

#[component]
fn TestSuiteLine(suite: TestSuite, open_failures: bool) -> Element {
    let tests: Vec<&Test> = suite.tests.iter().collect();
    let failed = count(&tests, TestStatus::Failed) > 0;

    rsx! {
        TraceCollapsible { default_open: failed,
            TraceCollapsibleTrigger {
                TraceLine { status: error_status(failed),
                    Icon { paths: FILE_CODE }
                    TraceTitle { "{suite.name}, {format_counts(&tests)}" }
                }
            }
            TraceCollapsibleContent { keep_mounted: true,
                Trace {
                    for test in suite.tests.iter() {
                        TestRow {
                            key: "{test.name}",
                            test: test.clone(),
                            default_open: open_failures,
                        }
                    }
                }
            }
        }
    }
}
