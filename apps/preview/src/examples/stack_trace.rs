//! Stack Trace examples, ported from shadcn/ui `examples/base/stack-trace-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::stack_trace::StackTrace;
use crate::components::ui::tool::{ToolLine, ToolState};
use crate::components::ui::trace::Trace;

#[component]
pub fn Examples() -> Element {
    rsx! {
        StackTraceDemo {}
        StackTraceFrames {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

const DEMO_STACK: &str = "TypeError: Cannot read properties of undefined (reading 'email')
    at seedUsers (prisma/seed.ts:27:34)
    at main (prisma/seed.ts:51:9)
    at Module._compile (node:internal/modules/cjs/loader:1554:14)
    at Object.transformer (node_modules/tsx/dist/register.cjs:3:1104)
    at Module.load (node:internal/modules/cjs/loader:1289:32)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)";

const RESET_OUTPUT: &str = "Database reset successful

The following migrations have been applied:

migrations/
  └─ 20260912_init/
  └─ 20260920_add_profiles/";

const ANSWER: &str = "The migrations ran, but the seed failed in seedUsers. One fixture has no profile, so profile.email is undefined.";

#[derive(Clone, PartialEq)]
struct Call {
    command: &'static str,
    state: ToolState,
}

fn demo_calls() -> Vec<Call> {
    vec![
        Call {
            command: "pnpm prisma migrate reset --skip-seed",
            state: ToolState::Output(RESET_OUTPUT.into()),
        },
        Call {
            command: "pnpm prisma db seed",
            state: ToolState::Error(DEMO_STACK.into()),
        },
    ]
}

async fn run_call(
    mut calls: Signal<Vec<Call>>,
    command: &'static str,
    wait_ms: u32,
    state: ToolState,
) {
    calls.write().push(Call {
        command,
        state: ToolState::Input,
    });
    let index = calls.read().len() - 1;
    sleep(wait_ms).await;
    if let Some(call) = calls.write().get_mut(index) {
        call.state = state;
    }
}

#[component]
fn StackTraceDemo() -> Element {
    let mut calls = use_signal(demo_calls);
    let mut text = use_signal(|| ANSWER.to_string());
    let mut busy = use_signal(|| false);

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                Message { align: MessageAlign::End,
                    MessageContent {
                        Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                            BubbleContent { "Reset the database and seed it." }
                        }
                    }
                }
                if !calls().is_empty() || !text().is_empty() {
                    Message {
                        MessageContent { class: "gap-4",
                            if !calls().is_empty() {
                                Trace {
                                    for (index , call) in calls().into_iter().enumerate() {
                                        match call.state {
                                            ToolState::Error(stack) => rsx! {
                                                StackTrace { key: "{index}", stack, default_open: true }
                                            },
                                            state => rsx! {
                                                ToolLine {
                                                    key: "{index}",
                                                    command: call.command,
                                                    state,
                                                    streaming: busy(),
                                                }
                                            },
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
                            calls.set(Vec::new());
                            text.set(String::new());
                            busy.set(true);
                            spawn(async move {
                                run_call(
                                        calls,
                                        "pnpm prisma migrate reset --skip-seed",
                                        900,
                                        ToolState::Output(RESET_OUTPUT.into()),
                                    )
                                    .await;
                                run_call(
                                        calls,
                                        "pnpm prisma db seed",
                                        1200,
                                        ToolState::Error(DEMO_STACK.into()),
                                    )
                                    .await;
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

const FRAMES_STACK: &str = "TypeError: Cannot read properties of undefined (reading 'total')
    at formatInvoice (app/lib/billing.ts:42:31)
    at InvoiceRow (app/components/invoice-row.tsx:18:12)
    at renderWithHooks (node_modules/react-dom/cjs/react-dom.development.js:14985:18)
    at beginWork (node_modules/react-dom/cjs/react-dom.development.js:19049:16)
    at performUnitOfWork (node_modules/react-dom/cjs/react-dom.development.js:22776:12)
    at workLoopSync (node_modules/react-dom/cjs/react-dom.development.js:22707:5)";

#[component]
fn StackTraceFrames() -> Element {
    rsx! {
        Example { title: "Frames", class: "h-auto",
            Trace { class: "max-w-sm",
                StackTrace { stack: FRAMES_STACK, default_open: true }
            }
        }
    }
}
