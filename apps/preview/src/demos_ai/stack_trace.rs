use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::stack_trace::{
    StackFileLocation, StackTrace, StackTraceActions, StackTraceContent, StackTraceCopyButton,
    StackTraceError, StackTraceErrorMessage, StackTraceErrorType, StackTraceExpandButton,
    StackTraceFrames, StackTraceHeader,
};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-col gap-4 rounded-lg border p-6";
const TRACE: &str = "TypeError: Cannot read properties of undefined (reading 'map')
    at renderList (src/components/list.rs:42:18)
    at ItemPanel (src/components/panel.rs:17:9)
    at src/main.rs:88:5
    at renderRow (node_modules/react-table/index.js:88:12)
    at Module._compile (node:internal/modules/cjs/loader:1554:14)";
const PLAIN_TRACE: &str = "Request failed after 3 retries
    at fetchUser (src/api/users.rs:12:5)
    at refreshSession (src/auth/session.rs:64:9)";

#[component]
pub fn StackTraceDemo() -> Element {
    let mut show_internal = use_signal(|| true);
    let mut last_location = use_signal(String::new);
    let toggle_label = if show_internal() {
        "Hide internal frames"
    } else {
        "Show internal frames"
    };
    let location = last_location();

    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Stack trace" }
            div { class: BOX,
                StackTrace {
                    trace: TRACE.to_string(),
                    default_open: true,
                    StackTraceHeader {
                        StackTraceError {
                            StackTraceErrorType {}
                            StackTraceErrorMessage {}
                        }
                        StackTraceActions {
                            StackTraceCopyButton {}
                            StackTraceExpandButton {}
                        }
                    }
                    StackTraceContent {
                        StackTraceFrames {
                            show_internal: show_internal(),
                            on_file_path_click: move |file: StackFileLocation| {
                                last_location.set(format!(
                                    "{}:{}:{}",
                                    file.path,
                                    file.line.unwrap_or(0),
                                    file.column.unwrap_or(0),
                                ));
                            },
                        }
                    }
                }
                div { class: "flex flex-wrap items-center gap-3",
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        onclick: move |_| show_internal.set(!show_internal()),
                        "{toggle_label}"
                    }
                    if !location.is_empty() {
                        span { class: "text-xs text-muted-foreground", "Opened {location}" }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Collapsed, no error type" }
            div { class: BOX,
                StackTrace { trace: PLAIN_TRACE.to_string(),
                    StackTraceHeader {
                        StackTraceError {
                            StackTraceErrorType {}
                            StackTraceErrorMessage {}
                        }
                        StackTraceActions {
                            StackTraceExpandButton {}
                        }
                    }
                    StackTraceContent {
                        StackTraceFrames {}
                    }
                }
            }
        }
    }
}
