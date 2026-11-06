//! Test Results examples, ported from shadcn/ui `examples/base/test-results-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::FlaskConical;

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::test_results::{Test, TestResults, TestStatus, TestSuite};
use crate::components::ui::trace::{Trace, TraceIcon, TraceLine, TraceStatus, TraceTitle};

#[component]
pub fn Examples() -> Element {
    rsx! {
        TestResultsDemo {}
        TestResultsSuites {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

fn test(name: &str, status: TestStatus) -> Test {
    Test {
        name: name.to_string(),
        status,
        error: None,
    }
}

fn failed(name: &str, error: &str) -> Test {
    Test {
        name: name.to_string(),
        status: TestStatus::Failed,
        error: Some(error.to_string()),
    }
}

fn suite(name: &str, tests: Vec<Test>) -> TestSuite {
    TestSuite {
        name: name.to_string(),
        tests,
    }
}

const ANSWER: &str = "One test failed. The payment client does not retry a declined card, so the second charge attempt never happens.";

fn demo_suites() -> Vec<TestSuite> {
    vec![
        suite(
            "cart.test.ts",
            vec![
                test("adds an item", TestStatus::Passed),
                test("removes an item", TestStatus::Passed),
                test("applies a discount code", TestStatus::Passed),
            ],
        ),
        suite(
            "payment.test.ts",
            vec![
                test("charges the saved card", TestStatus::Passed),
                failed(
                    "retries a declined card",
                    "Expected 2 charge attempts, received 1\n\n  at Object.<anonymous> (src/payment.test.ts:58:22)",
                ),
            ],
        ),
    ]
}

#[component]
fn TestResultsDemo() -> Element {
    let mut suites = use_signal(|| Some(demo_suites()));
    let mut text = use_signal(|| ANSWER.to_string());
    let mut busy = use_signal(|| false);
    let running = busy() && suites().is_none();

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                Message { align: MessageAlign::End,
                    MessageContent {
                        Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                            BubbleContent { "Run the checkout tests." }
                        }
                    }
                }
                if running || suites().is_some() {
                    Message {
                        MessageContent { class: "gap-4",
                            Trace {
                                if let Some(suites) = suites() {
                                    TestResults {
                                        default_open: suites
                                            .iter()
                                            .flat_map(|suite| &suite.tests)
                                            .any(|test| test.status == TestStatus::Failed),
                                        open_failures: true,
                                        suites,
                                    }
                                } else {
                                    TraceLine { status: TraceStatus::Running,
                                        TraceIcon {
                                            FlaskConical {}
                                        }
                                        TraceTitle { "Running pnpm test checkout" }
                                    }
                                }
                            }
                            if !text().is_empty() {
                                p { "{text}" }
                            }
                        }
                    }
                }
                if !busy() {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "mt-auto self-start",
                        onclick: move |_| {
                            suites.set(None);
                            text.set(String::new());
                            busy.set(true);
                            spawn(async move {
                                sleep(1500).await;
                                suites.set(Some(demo_suites()));
                                sleep(300).await;
                                text.set(ANSWER.to_string());
                                busy.set(false);
                            });
                        },
                        "Replay"
                    }
                }
            }
        }
    }
}

fn suites_data() -> Vec<TestSuite> {
    vec![
        suite(
            "auth.test.ts",
            vec![
                test("logs in with valid credentials", TestStatus::Passed),
                test("rejects an invalid password", TestStatus::Passed),
                test("refreshes an expired token", TestStatus::Passed),
            ],
        ),
        suite(
            "user-api.test.ts",
            vec![
                test("creates a user", TestStatus::Passed),
                failed(
                    "updates a user profile",
                    "Expected status 200, received 500\n\n  at Object.<anonymous> (src/user-api.test.ts:45:12)",
                ),
                test("deletes a user", TestStatus::Skipped),
            ],
        ),
        suite(
            "db.test.ts",
            vec![
                test("connects to the database", TestStatus::Passed),
                failed(
                    "times out a slow query",
                    "Exceeded timeout of 5000 ms for a test.\n\n  at Object.<anonymous> (src/db.test.ts:22:3)",
                ),
            ],
        ),
    ]
}

#[component]
fn TestResultsSuites() -> Element {
    rsx! {
        Example { title: "Suites", class: "h-auto",
            Trace { class: "max-w-sm",
                TestResults { default_open: true, suites: suites_data() }
            }
        }
    }
}
