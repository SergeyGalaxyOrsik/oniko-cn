use crate::components::ui::test_results::{
    Test, TestDuration, TestError, TestErrorMessage, TestErrorStack, TestName, TestResults,
    TestResultsContent, TestResultsDuration, TestResultsHeader, TestResultsProgress,
    TestResultsSummary, TestRunSummary, TestStatus, TestStatusIcon, TestSuite, TestSuiteContent,
    TestSuiteName, TestSuiteStats,
};
use dioxus::prelude::*;

#[component]
pub fn TestResultsDemo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Test Results" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                TestResults {
                    summary: TestRunSummary {
                        passed: 42,
                        failed: 2,
                        skipped: 1,
                        total: 45,
                        duration_ms: Some(3210),
                    },
                    TestResultsHeader {
                        TestResultsSummary {}
                        TestResultsDuration {}
                    }
                    TestResultsProgress {}
                    TestResultsContent {
                        TestSuite { name: "auth::session", status: TestStatus::Passed, default_open: false,
                            TestSuiteName {
                                TestSuiteStats { passed: 12 }
                            }
                            TestSuiteContent {
                                Test { name: "creates a session token", status: TestStatus::Passed, duration_ms: 14 }
                                Test { name: "expires after the TTL", status: TestStatus::Passed, duration_ms: 8 }
                            }
                        }
                        TestSuite { name: "api::users", status: TestStatus::Failed,
                            TestSuiteName {
                                TestSuiteStats { passed: 3, failed: 1, skipped: 1 }
                            }
                            TestSuiteContent {
                                Test { name: "returns the user profile", status: TestStatus::Passed, duration_ms: 120 }
                                Test { name: "rejects an invalid id", status: TestStatus::Failed, duration_ms: 340,
                                    TestError {
                                        TestErrorMessage { "expected 400, received 500" }
                                        TestErrorStack {
                                            "at parse_user_id (src/api/users.rs:42)\nat handle_request (src/api/mod.rs:88)"
                                        }
                                    }
                                }
                                Test { name: "deletes a user", status: TestStatus::Skipped }
                            }
                        }
                        TestSuite { name: "store::sync", status: TestStatus::Running,
                            TestSuiteName {
                                TestSuiteStats { passed: 0 }
                            }
                            TestSuiteContent {
                                Test { name: "syncs pending writes", status: TestStatus::Running }
                            }
                        }
                    }
                }
                TestResults {
                    TestResultsHeader {
                        TestResultsSummary {}
                        TestResultsDuration {}
                    }
                    TestResultsProgress {}
                }
                div { class: "flex flex-wrap items-center gap-4 text-sm",
                    for status in [TestStatus::Passed, TestStatus::Failed, TestStatus::Skipped, TestStatus::Running] {
                        span { class: "inline-flex items-center gap-2",
                            TestStatusIcon { status: status }
                            TestName { name: status.as_str() }
                        }
                    }
                    TestDuration { duration_ms: 1500 }
                }
            }
        }
    }
}
