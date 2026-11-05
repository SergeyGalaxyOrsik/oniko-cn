use crate::components::ui::terminal::{
    Terminal, TerminalActions, TerminalClearButton, TerminalContent, TerminalCopyButton,
    TerminalHeader, TerminalStatus, TerminalTitle,
};
use dioxus::prelude::*;

const BUILD_LOG: &str = "\x1b[1m$ cargo build\x1b[0m\n\x1b[32m   Compiling\x1b[0m oniko-cn v0.1.0\n\x1b[32m    Finished\x1b[0m dev profile in 2.31s\n";
const TEST_LOG: &str = "$ npm test\n> running 12 suites\n  PASS src/auth.test.ts\n  PASS src/api.test.ts\n  RUN  src/store.test.ts";

#[component]
pub fn TerminalDemo() -> Element {
    let mut log = use_signal(|| BUILD_LOG.to_string());

    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Terminal" }
            div { class: "flex flex-col gap-4",
                Terminal {
                    output: log(),
                    on_clear: move |_| log.set(String::new()),
                }
                Terminal { output: TEST_LOG, is_streaming: true }
                Terminal {
                    output: TEST_LOG,
                    auto_scroll: false,
                    TerminalHeader {
                        TerminalTitle { "npm test" }
                        TerminalStatus { "Idle" }
                        TerminalActions {
                            TerminalCopyButton {}
                        }
                    }
                    TerminalContent {}
                }
                Terminal {
                    output: log(),
                    on_clear: move |_| log.set(String::new()),
                    is_streaming: true,
                    TerminalHeader {
                        TerminalTitle { "Streaming build" }
                        TerminalStatus { "Running" }
                        TerminalActions {
                            TerminalCopyButton {}
                            TerminalClearButton {}
                        }
                    }
                    TerminalContent {}
                }
            }
        }
    }
}
