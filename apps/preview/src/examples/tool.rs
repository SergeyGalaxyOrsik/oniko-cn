//! Tool examples, ported from shadcn/ui `examples/base/tool-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ChevronRight, CircleCheck, FileText, LoaderCircle, Terminal};

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::code_block::{CodeBlock, CodeBlockContent, CodeBlockSize};
use crate::components::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemSize,
    ItemTitle, ItemVariant,
};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::sheet::{
    Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle, SheetTrigger,
};
use crate::components::ui::tool::{ToolLine, ToolState};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        ToolDemo {}
        ToolStates {}
        ToolSheet {}
        ToolApproval {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

#[component]
fn UserMessage(children: Element) -> Element {
    rsx! {
        Message { align: MessageAlign::End,
            MessageContent {
                Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                    BubbleContent { {children} }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Call {
    command: &'static str,
    state: ToolState,
}

const NODE_VERSION: &str = "v24.18.0";
const NPM_VERSION: &str = "11.6.2";
const PNPM_ERROR: &str = "command not found: pnpm";
const DEMO_ANSWER: &str = "Node 24 and npm 11 are installed. pnpm is not.";

fn demo_calls() -> Vec<Call> {
    vec![
        Call {
            command: "node --version",
            state: ToolState::Output(NODE_VERSION.into()),
        },
        Call {
            command: "npm --version",
            state: ToolState::Output(NPM_VERSION.into()),
        },
        Call {
            command: "pnpm --version",
            state: ToolState::Error(PNPM_ERROR.into()),
        },
    ]
}

async fn run_call(
    mut calls: Signal<Vec<Call>>,
    command: &'static str,
    wait_ms: u32,
    result: Result<&str, &str>,
) {
    calls.write().push(Call {
        command,
        state: ToolState::Input,
    });
    let index = calls.read().len() - 1;
    sleep(wait_ms).await;
    if let Some(call) = calls.write().get_mut(index) {
        call.state = match result {
            Ok(output) => ToolState::Output(output.to_string()),
            Err(error) => ToolState::Error(error.to_string()),
        };
    }
}

#[component]
fn ToolDemo() -> Element {
    let mut calls = use_signal(demo_calls);
    let mut text = use_signal(|| DEMO_ANSWER.to_string());
    let mut busy = use_signal(|| false);

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                UserMessage { "Which package managers are installed?" }
                if !calls().is_empty() || !text().is_empty() {
                    Message {
                        MessageContent { class: "gap-4",
                            if !calls().is_empty() {
                                Trace {
                                    for (index , call) in calls().into_iter().enumerate() {
                                        ToolLine {
                                            key: "{index}",
                                            command: call.command,
                                            default_open: !busy() && matches!(call.state, ToolState::Error(_)),
                                            streaming: busy(),
                                            state: call.state,
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
                                run_call(calls, "node --version", 900, Ok(NODE_VERSION)).await;
                                run_call(calls, "npm --version", 900, Ok(NPM_VERSION)).await;
                                run_call(calls, "pnpm --version", 900, Err(PNPM_ERROR)).await;
                                text.set(DEMO_ANSWER.to_string());
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

#[component]
fn ToolStates() -> Element {
    rsx! {
        Example { title: "States", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceCollapsible {
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                Terminal {}
                            }
                            TraceTitle { "Ran git status --short" }
                        }
                    }
                    TraceCollapsibleContent {
                        CodeBlock { size: CodeBlockSize::Sm,
                            CodeBlockContent {
                                wrap: true,
                                class: "max-h-64",
                                code: " M app/api/chat/route.ts\n M components/chat.tsx",
                            }
                        }
                    }
                }
                TraceCollapsible {
                    TraceCollapsibleTrigger {
                        TraceLine { status: TraceStatus::Error,
                            TraceIcon {
                                Terminal {}
                            }
                            TraceTitle { "Failed to run pnpm dev" }
                        }
                    }
                    TraceCollapsibleContent {
                        CodeBlock { size: CodeBlockSize::Sm,
                            CodeBlockContent {
                                wrap: true,
                                class: "max-h-64",
                                code: "Command timed out after 120 seconds.",
                            }
                        }
                    }
                }
                TraceLine {
                    TraceIcon {
                        Terminal {}
                    }
                    TraceTitle { "Denied rm -rf dist" }
                }
                TraceLine { status: TraceStatus::Running,
                    TraceIcon {
                        Terminal {}
                    }
                    TraceTitle { "Running pnpm test" }
                }
            }
        }
    }
}

const PACKAGE_JSON: &str = r#"{
  "name": "acme-chat",
  "private": true,
  "dependencies": {
    "@ai-sdk/react": "^4.0.0",
    "ai": "^7.0.0",
    "next": "^16.3.0"
  }
}"#;

#[derive(Clone, Copy, PartialEq)]
enum Turn {
    Empty,
    Reading,
    Read,
    Answered,
}

#[component]
fn ToolSheet() -> Element {
    let mut turn = use_signal(|| Turn::Empty);
    let mut open = use_signal(|| false);
    let busy = matches!(turn(), Turn::Reading);

    rsx! {
        Example { title: "Sheet", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                if turn() != Turn::Empty {
                    UserMessage { "Which AI SDK version does this app use?" }
                    Message {
                        MessageContent { class: "gap-4",
                            if turn() == Turn::Reading {
                                Trace {
                                    TraceLine { status: TraceStatus::Running,
                                        TraceIcon {
                                            FileText {}
                                        }
                                        TraceTitle { "Reading package.json" }
                                    }
                                }
                            } else {
                                Trace {
                                    Sheet { open: open(), on_open_change: move |next| open.set(next),
                                        SheetTrigger {
                                            TraceLine {
                                                TraceIcon {
                                                    FileText {}
                                                }
                                                TraceTitle { "Read package.json" }
                                                TraceIcon {
                                                    ChevronRight {}
                                                }
                                            }
                                        }
                                        SheetContent {
                                            SheetHeader {
                                                SheetTitle { class: "font-mono", "package.json" }
                                                SheetDescription { "{PACKAGE_JSON.lines().count()} lines" }
                                            }
                                            CodeBlock { class: "mx-6 mb-6 w-auto",
                                                CodeBlockContent { line_numbers: true, code: PACKAGE_JSON }
                                            }
                                        }
                                    }
                                }
                            }
                            if turn() == Turn::Answered {
                                p { "It uses ai 7 with @ai-sdk/react 4." }
                            }
                        }
                    }
                }
                if !busy {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "mt-auto self-start",
                        onclick: move |_| {
                            if turn() == Turn::Empty {
                                turn.set(Turn::Reading);
                                spawn(async move {
                                    sleep(900).await;
                                    turn.set(Turn::Read);
                                    sleep(300).await;
                                    turn.set(Turn::Answered);
                                });
                            } else {
                                turn.set(Turn::Empty);
                            }
                        },
                        if turn() == Turn::Empty {
                            "Send message"
                        } else {
                            "Reset"
                        }
                    }
                }
            }
        }
    }
}

const REASONING: &str = "The build output lives in dist. Deleting it cannot be undone, so I should ask before I run the command.";

#[derive(Clone, Copy, PartialEq)]
enum Approval {
    Empty,
    Thinking,
    Requested,
    Approved,
    Denied,
    Ran,
}

#[component]
fn ReasoningLine(text: String, running: bool) -> Element {
    let mut chosen = use_signal(|| None::<bool>);

    rsx! {
        TraceCollapsible {
            open: chosen().unwrap_or(running),
            on_open_change: move |next| chosen.set(Some(next)),
            TraceCollapsibleTrigger {
                TraceLine { status: if running { TraceStatus::Running } else { TraceStatus::Complete },
                    TraceIcon {
                        if running {
                            LoaderCircle { class: "animate-spin" }
                        } else {
                            CircleCheck {}
                        }
                    }
                    TraceTitle {
                        if running {
                            "Thinking"
                        } else {
                            "Thought"
                        }
                    }
                }
            }
            TraceCollapsibleContent { class: "whitespace-pre-wrap", "{text}" }
        }
    }
}

#[component]
fn ToolApproval() -> Element {
    let mut step = use_signal(|| Approval::Empty);
    let mut reasoning = use_signal(String::new);
    let mut answer = use_signal(String::new);
    let busy = step() == Approval::Thinking
        || (matches!(step(), Approval::Approved | Approval::Denied) && answer().is_empty());

    let mut respond = move |approved: bool| {
        step.set(if approved {
            Approval::Approved
        } else {
            Approval::Denied
        });
        spawn(async move {
            sleep(600).await;
            if approved {
                step.set(Approval::Ran);
                answer.set("Removed the dist folder.".to_string());
            } else {
                answer.set("Okay, I left the dist folder in place.".to_string());
            }
        });
    };

    rsx! {
        Example { title: "Approval", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                if step() != Approval::Empty {
                    UserMessage { "Clean up the build folder." }
                    Message {
                        MessageContent { class: "gap-4",
                            Trace {
                                ReasoningLine {
                                    text: reasoning(),
                                    running: step() == Approval::Thinking,
                                }
                            }
                            if step() == Approval::Requested {
                                Item { variant: ItemVariant::Outline, size: ItemSize::Sm,
                                    ItemMedia { variant: ItemMediaVariant::Icon,
                                        Terminal {}
                                    }
                                    ItemContent {
                                        ItemTitle { "Run rm -rf dist?" }
                                        ItemDescription { "The command runs in this project." }
                                    }
                                    ItemActions {
                                        Button {
                                            size: ButtonSize::Sm,
                                            variant: ButtonVariant::Outline,
                                            onclick: move |_| respond(false),
                                            "Deny"
                                        }
                                        Button { size: ButtonSize::Sm, onclick: move |_| respond(true), "Approve" }
                                    }
                                }
                            } else if step() != Approval::Thinking {
                                Trace {
                                    TraceLine {
                                        status: if matches!(step(), Approval::Approved) { TraceStatus::Running } else { TraceStatus::Complete },
                                        TraceIcon {
                                            Terminal {}
                                        }
                                        TraceTitle {
                                            match step() {
                                                Approval::Denied => "Denied rm -rf dist",
                                                Approval::Ran => "Ran rm -rf dist",
                                                _ => "Running rm -rf dist",
                                            }
                                        }
                                    }
                                }
                            }
                            if !answer().is_empty() {
                                p { "{answer}" }
                            }
                        }
                    }
                }
                if !busy && step() != Approval::Requested {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "mt-auto self-start",
                        onclick: move |_| {
                            if step() != Approval::Empty {
                                step.set(Approval::Empty);
                                reasoning.set(String::new());
                                answer.set(String::new());
                                return;
                            }
                            step.set(Approval::Thinking);
                            spawn(async move {
                                for word in REASONING.split_inclusive(' ') {
                                    sleep(40).await;
                                    reasoning.write().push_str(word);
                                }
                                sleep(400).await;
                                step.set(Approval::Requested);
                            });
                        },
                        if step() == Approval::Empty {
                            "Send message"
                        } else {
                            "Reset"
                        }
                    }
                }
            }
        }
    }
}
