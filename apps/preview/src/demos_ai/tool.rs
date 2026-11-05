use crate::components::ui::tool::{
    Tool, ToolContent, ToolHeader, ToolInput, ToolOutput, ToolState,
};
use dioxus::prelude::*;

const READ_INPUT: &str = "{\n  \"path\": \"src/auth/session.rs\"\n}";
const READ_OUTPUT: &str = "pub fn validate(token: &str) -> bool {\n    !token.is_empty()\n}";
const WRITE_INPUT: &str = "{\n  \"path\": \"src/auth/session.rs\",\n  \"create\": false\n}";

#[component]
pub fn ToolDemo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Tool" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                for state in [
                    ToolState::InputStreaming,
                    ToolState::InputAvailable,
                    ToolState::ApprovalRequested,
                    ToolState::ApprovalResponded,
                    ToolState::OutputAvailable,
                    ToolState::OutputError,
                    ToolState::OutputDenied,
                ] {
                    Tool { key: "{state.as_str()}",
                        ToolHeader { state: state, title: "Read file" }
                    }
                }
                Tool { default_open: true,
                    ToolHeader { state: ToolState::OutputAvailable, title: "read_file" }
                    ToolContent {
                        ToolInput { input: READ_INPUT }
                        ToolOutput { output: READ_OUTPUT }
                    }
                }
                Tool {
                    ToolHeader { state: ToolState::OutputError, tool_name: "write_file" }
                    ToolContent {
                        ToolInput { input: WRITE_INPUT }
                        ToolOutput { error_text: "Permission denied (os error 13)" }
                    }
                }
                Tool {
                    ToolHeader { state: ToolState::OutputDenied, tool_name: "delete_file" }
                    ToolContent {
                        ToolInput { input: "src/auth/session.rs" }
                        ToolOutput {}
                    }
                }
            }
        }
    }
}
