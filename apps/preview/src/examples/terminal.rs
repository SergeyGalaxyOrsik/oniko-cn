//! Terminal examples, ported from shadcn/ui `examples/base/terminal-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::Terminal;

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::code_block::{CodeBlock, CodeBlockContent, CodeBlockSize};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::terminal::{TerminalLine, TerminalState};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        TerminalDemo {}
        TerminalError {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

const COMMANDS: [(&str, &[&str]); 3] = [
    ("pnpm lint", &["> acme-chat@0.1.0 lint", "> eslint ."]),
    (
        "pnpm test",
        &[
            "> acme-chat@0.1.0 test",
            "> vitest run",
            "",
            " ✓ lib/ai.test.ts (6 tests) 41ms",
            " ✓ components/chat.test.tsx (9 tests) 212ms",
            "",
            " Test Files  2 passed (2)",
            "      Tests  15 passed (15)",
        ],
    ),
    (
        "pnpm build",
        &[
            "> acme-chat@0.1.0 build",
            "> next build",
            "",
            "   ▲ Next.js 16.3.0",
            "   Creating an optimized production build ...",
            " ✓ Compiled successfully in 4.2s",
            " ✓ Generating static pages (12/12)",
            "",
            "Route (app)                Size  First Load JS",
            "┌ ○ /                     1.9 kB       112 kB",
            "└ ƒ /api/chat              141 B       102 kB",
        ],
    ),
];

const ANSWER: &str = "Lint and tests pass, and the build compiled both routes.";

#[derive(Clone, PartialEq)]
struct Command {
    command: &'static str,
    output: String,
    state: TerminalState,
}

fn finished_commands() -> Vec<Command> {
    COMMANDS
        .iter()
        .map(|&(command, output)| Command {
            command,
            output: output.join("\n"),
            state: TerminalState::Exited(0),
        })
        .collect()
}

#[component]
fn TerminalDemo() -> Element {
    let mut commands = use_signal(finished_commands);
    let mut text = use_signal(|| ANSWER.to_string());
    let mut busy = use_signal(|| false);
    let last = commands().len().saturating_sub(1);

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                Message { align: MessageAlign::End,
                    MessageContent {
                        Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                            BubbleContent { "Run the checks, then build the app." }
                        }
                    }
                }
                if !commands().is_empty() || !text().is_empty() {
                    Message {
                        MessageContent { class: "gap-4",
                            if !commands().is_empty() {
                                Trace {
                                    for (index , run) in commands().into_iter().enumerate() {
                                        TerminalLine {
                                            key: "{index}",
                                            command: run.command,
                                            output: run.output,
                                            state: run.state,
                                            default_open: if busy() { None } else { Some(index == last) },
                                        }
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
                            commands.set(Vec::new());
                            text.set(String::new());
                            busy.set(true);
                            spawn(async move {
                                for (command, lines) in COMMANDS {
                                    commands
                                        .write()
                                        .push(Command {
                                            command,
                                            output: String::new(),
                                            state: TerminalState::Running,
                                        });
                                    let index = commands.read().len() - 1;
                                    for line in 0..lines.len() {
                                        if let Some(run) = commands.write().get_mut(index) {
                                            run.output = lines[..=line].join("\n");
                                        }
                                        sleep(200).await;
                                    }
                                    if let Some(run) = commands.write().get_mut(index) {
                                        run.state = TerminalState::Exited(0);
                                    }
                                }
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

const LINT_OUTPUT: &str = "> acme-chat@0.1.0 lint
> eslint .

app/api/chat/route.ts
  14:7  error  'model' is assigned a value but never used  no-unused-vars

components/chat.tsx
  42:5  error  React Hook useEffect has a missing dependency: 'messages'  react-hooks/exhaustive-deps

✖ 2 problems (2 errors, 0 warnings)

 ELIFECYCLE  Command failed with exit code 1.";

#[component]
fn TerminalError() -> Element {
    rsx! {
        Example { title: "Error", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceLine {
                    TraceIcon {
                        Terminal {}
                    }
                    TraceTitle { "Ran pnpm install" }
                }
                TraceCollapsible { default_open: true,
                    TraceCollapsibleTrigger {
                        TraceLine { status: TraceStatus::Error,
                            TraceIcon {
                                Terminal {}
                            }
                            TraceTitle { "pnpm lint exited with code 1" }
                        }
                    }
                    TraceCollapsibleContent {
                        CodeBlock { size: CodeBlockSize::Sm,
                            CodeBlockContent {
                                wrap: true,
                                class: "max-h-64",
                                code: LINT_OUTPUT,
                            }
                        }
                    }
                }
            }
        }
    }
}
