//! Chat examples, ported from shadcn/ui `examples/base/chat-*.tsx`.
//!
//! Upstream scripts the thread with `createChat` and `useChat`. Here local state and timers stand in for
//! them. The agent's reply is rendered by a small Markdown subset: paragraphs, lists, fenced code,
//! `code` and **bold**.

use dioxus::prelude::*;
use lucide_dioxus::{
    ArrowUp, CircleCheck, Globe, LoaderCircle, MessageCircleDashed, Paperclip, Plus, RotateCw,
    Telescope, Terminal,
};

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::code_block::{CodeBlock, CodeBlockContent, CodeBlockSize};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator,
    DropdownMenuSide, DropdownMenuTrigger,
};
use crate::components::ui::empty::{
    Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupTextarea,
};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};
use crate::components::ui::message_scroller::{
    MessageScroller, MessageScrollerButton, MessageScrollerContent, MessageScrollerItem,
    MessageScrollerProvider, MessageScrollerViewport,
};
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

const FALLBACK: &str = "This demo only knows the scripted turns. Press reset to start over.";

/// One scripted agent turn: optional reasoning, one `runCommand` tool call and the reply.
struct Turn {
    user: &'static str,
    reasoning: Option<&'static str>,
    command: &'static str,
    wait_ms: u32,
    stdout: &'static str,
    reply: &'static str,
}

const SCRIPT: [Turn; 3] = [
    Turn {
        user: "Add a dark mode toggle to the settings page.",
        reasoning: Some(
            "I need to know how themes are set up before I add a toggle. If next-themes is already in the layout, the toggle only has to call setTheme.",
        ),
        command: "rg -l ThemeProvider app",
        wait_ms: 800,
        stdout: "app/layout.tsx",
        reply: "next-themes is already set up in `app/layout.tsx`, so the toggle only has to call `setTheme`.\n\n**Changes**\n\n- Added `components/mode-toggle.tsx` with a Light and Dark option.\n- Put `<ModeToggle />` next to the Appearance heading in `app/settings/page.tsx`.\n\n```tsx\nconst { setTheme } = useTheme()\n```",
    },
    Turn {
        user: "Make it follow the system setting by default.",
        reasoning: Some(
            "The default theme lives on the ThemeProvider. I should check what it is set to now.",
        ),
        command: "rg -n defaultTheme app/layout.tsx",
        wait_ms: 800,
        stdout: "12:  <ThemeProvider attribute=\"class\" defaultTheme=\"light\">",
        reply: "It was set to `light`. I updated the `ThemeProvider` in `app/layout.tsx`:\n\n```tsx\n<ThemeProvider attribute=\"class\" defaultTheme=\"system\" enableSystem>\n```\n\nNew visitors now get the theme from their OS. The toggle also has a **System** option, so anyone who picked Light or Dark can switch back.",
    },
    Turn {
        user: "Run the build before I ship it.",
        reasoning: None,
        command: "pnpm build",
        wait_ms: 1600,
        stdout: "▲ Next.js 16.0.0\n\n✓ Compiled successfully in 4.2s\n✓ Linting and checking validity of types\n✓ Generating static pages (12/12)",
        reply: "The build passes, so it is ready to ship.\n\n- Compiled in **4.2s**.\n- No lint or type errors.\n- Generated all **12** static pages.",
    },
];

#[derive(Clone, PartialEq)]
enum Part {
    Reasoning {
        text: String,
        streaming: bool,
    },
    Tool {
        command: &'static str,
        output: Option<&'static str>,
    },
}

#[derive(Clone, PartialEq)]
struct ChatMessage {
    id: String,
    is_user: bool,
    text: String,
    parts: Vec<Part>,
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms})); return true;"
    ))
    .await;
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        super::Example {
            title: "Demo",
            class: "h-auto theme-blue bg-background p-0!",
            ChatDemo {}
        }
    }
}

#[component]
fn ChatDemo() -> Element {
    let mut messages = use_signal(Vec::<ChatMessage>::new);
    let mut busy = use_signal(|| false);
    let mut draft = use_signal(|| None::<String>);

    let next_turn = |messages: &[ChatMessage]| {
        let sent = messages.iter().filter(|message| message.is_user).count();
        (sent < SCRIPT.len()).then_some(sent)
    };
    let text = draft().unwrap_or_else(|| {
        next_turn(&messages.read()).map_or(String::new(), |turn| SCRIPT[turn].user.to_string())
    });
    let can_send = !text.trim().is_empty() && !busy();

    let send = use_callback(move |()| {
        if busy() {
            return;
        }
        let scripted = if draft.peek().is_none() {
            next_turn(&messages.peek())
        } else {
            None
        };
        let user_text = match (scripted, draft.peek().clone()) {
            (Some(turn), _) => SCRIPT[turn].user.to_string(),
            (None, text) => text.unwrap_or_default().trim().to_string(),
        };
        if user_text.is_empty() {
            return;
        }
        draft.set(None);
        busy.set(true);
        let id = messages.read().len();
        messages.write().push(ChatMessage {
            id: format!("user-{id}"),
            is_user: true,
            text: user_text,
            parts: Vec::new(),
        });
        spawn(async move {
            sleep(600).await;
            messages.write().push(ChatMessage {
                id: format!("assistant-{id}"),
                is_user: false,
                text: String::new(),
                parts: Vec::new(),
            });
            let reply = match scripted {
                Some(turn) => {
                    let turn = &SCRIPT[turn];
                    if let Some(reasoning) = turn.reasoning {
                        stream_reasoning(messages, reasoning).await;
                        sleep(400).await;
                    }
                    run_tool(messages, turn).await;
                    sleep(500).await;
                    turn.reply
                }
                None => FALLBACK,
            };
            for word in reply.split_inclusive(' ') {
                sleep(16).await;
                if let Some(last) = messages.write().last_mut() {
                    last.text.push_str(word);
                }
            }
            busy.set(false);
        });
    });

    let last_id = messages.read().last().map(|message| message.id.clone());

    rsx! {
        MessageScrollerProvider {
            div { class: "relative flex w-full flex-col gap-4",
                div { class: "flex h-200 w-full flex-col gap-4",
                    div { class: "flex items-start justify-between gap-4 p-8 pb-4",
                        div { class: "flex flex-col gap-1",
                            div { class: "font-medium", "New Thread" }
                            div { class: "text-sm text-muted-foreground", "Ask the agent to change your code." }
                        }
                        Tooltip {
                            TooltipTrigger {
                                render: move |(attributes, children): (Vec<Attribute>, Element)| rsx! {
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        size: ButtonSize::Icon,
                                        aria_label: "Reset conversation",
                                        disabled: busy(),
                                        attributes,
                                        onclick: move |_| {
                                            messages.write().clear();
                                            draft.set(None);
                                        },
                                        {children}
                                    }
                                },
                                RotateCw {}
                            }
                            TooltipContent {
                                p { "Reset" }
                            }
                        }
                    }
                    div { class: "min-h-0 flex-1",
                        if messages.read().is_empty() {
                            Empty { class: "h-full",
                                EmptyHeader {
                                    EmptyMedia { variant: EmptyMediaVariant::Icon,
                                        MessageCircleDashed {}
                                    }
                                    EmptyTitle { "What should we build?" }
                                    EmptyDescription {
                                        "The agent can read your code, run commands and make changes. Press send to start."
                                    }
                                }
                            }
                        } else {
                            MessageScroller {
                                MessageScrollerViewport {
                                    MessageScrollerContent { aria_busy: busy(), class: "gap-6 px-8 py-0",
                                        for message in messages.read().iter().cloned() {
                                            if message.is_user {
                                                MessageScrollerItem {
                                                    key: "{message.id}",
                                                    message_id: message.id.clone(),
                                                    scroll_anchor: true,
                                                    class: "motion-safe:animate-in fade-in-0 slide-in-from-bottom-2 duration-300",
                                                    Message { align: MessageAlign::End,
                                                        MessageContent {
                                                            Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                                                                BubbleContent { "{message.text}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                MessageScrollerItem { key: "{message.id}", message_id: message.id.clone(),
                                                    Message {
                                                        MessageContent { class: "gap-3",
                                                            if !message.parts.is_empty() {
                                                                Trace {
                                                                    for (index , part) in message.parts.iter().cloned().enumerate() {
                                                                        StepRow {
                                                                            key: "{index}",
                                                                            part,
                                                                            streaming: busy() && last_id.as_deref() == Some(message.id.as_str()),
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if !message.text.is_empty() {
                                                                Markdown { text: message.text.clone() }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                MessageScrollerButton {}
                            }
                        }
                    }
                    div { class: "p-8 pt-0",
                        form {
                            class: "w-full",
                            onsubmit: move |event| {
                                event.prevent_default();
                                send.call(());
                            },
                            InputGroup {
                                InputGroupTextarea {
                                    value: "{text}",
                                    placeholder: "Ask the agent...",
                                    aria_label: "Message",
                                    class: "min-h-14",
                                    oninput: move |event: FormEvent| draft.set(Some(event.value())),
                                    onkeydown: move |event: KeyboardEvent| {
                                        if event.key() == Key::Enter && !event.modifiers().shift() {
                                            event.prevent_default();
                                            send.call(());
                                        }
                                    },
                                }
                                InputGroupAddon { align: InputGroupAlign::BlockEnd, class: "pt-1",
                                    DropdownMenu {
                                        DropdownMenuTrigger {
                                            aria_label: "Add context",
                                            variant: ButtonVariant::Outline,
                                            size: ButtonSize::IconSm,
                                            Plus {}
                                        }
                                        DropdownMenuContent {
                                            align: DropdownMenuAlign::Start,
                                            side: DropdownMenuSide::Top,
                                            class: "w-44",
                                            DropdownMenuItem {
                                                Paperclip {}
                                                "Add Photos & Files"
                                            }
                                            DropdownMenuSeparator {}
                                            DropdownMenuItem {
                                                Telescope {}
                                                "Deep Research"
                                            }
                                            DropdownMenuItem {
                                                Globe {}
                                                "Web Search"
                                            }
                                        }
                                    }
                                    InputGroupButton {
                                        variant: ButtonVariant::Default,
                                        size: InputGroupButtonSize::IconSm,
                                        disabled: !can_send,
                                        class: "ml-auto",
                                        onclick: move |_| send.call(()),
                                        ArrowUp {}
                                        span { class: "sr-only", "Send" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn stream_reasoning(mut messages: Signal<Vec<ChatMessage>>, text: &str) {
    update_last(&mut messages, |message| {
        message.parts.push(Part::Reasoning {
            text: String::new(),
            streaming: true,
        });
    });
    for word in text.split_inclusive(' ') {
        sleep(20).await;
        update_last(&mut messages, |message| {
            if let Some(Part::Reasoning { text, .. }) = message.parts.last_mut() {
                text.push_str(word);
            }
        });
    }
    update_last(&mut messages, |message| {
        if let Some(Part::Reasoning { streaming, .. }) = message.parts.last_mut() {
            *streaming = false;
        }
    });
}

async fn run_tool(mut messages: Signal<Vec<ChatMessage>>, turn: &'static Turn) {
    update_last(&mut messages, |message| {
        message.parts.push(Part::Tool {
            command: turn.command,
            output: None,
        });
    });
    sleep(turn.wait_ms).await;
    update_last(&mut messages, |message| {
        if let Some(Part::Tool { output, .. }) = message.parts.last_mut() {
            *output = Some(turn.stdout);
        }
    });
}

fn update_last(messages: &mut Signal<Vec<ChatMessage>>, change: impl FnOnce(&mut ChatMessage)) {
    if let Some(last) = messages.write().last_mut() {
        change(last);
    }
}

#[component]
fn StepRow(part: Part, streaming: bool) -> Element {
    match part {
        Part::Reasoning { text, streaming } => rsx! {
            ReasoningRow { text, running: streaming }
        },
        Part::Tool { command, output } => rsx! {
            ToolRow { command, output, streaming }
        },
    }
}

#[component]
fn ReasoningRow(text: String, running: bool) -> Element {
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
fn ToolRow(command: &'static str, output: Option<&'static str>, streaming: bool) -> Element {
    if let Some(stdout) = output {
        return rsx! {
            TraceCollapsible {
                TraceCollapsibleTrigger {
                    TraceLine {
                        TraceIcon {
                            Terminal {}
                        }
                        TraceTitle { "Ran {command}" }
                    }
                }
                TraceCollapsibleContent {
                    CodeBlock { size: CodeBlockSize::Sm,
                        CodeBlockContent { wrap: true, class: "max-h-64", code: stdout.to_string() }
                    }
                }
            }
        };
    }

    rsx! {
        TraceLine { status: if streaming { TraceStatus::Running } else { TraceStatus::Error },
            TraceIcon {
                Terminal {}
            }
            TraceTitle {
                if streaming {
                    "Running {command}"
                } else {
                    "Failed to run {command}"
                }
            }
        }
    }
}

/// Renders `text` as paragraphs, bullet lists and fenced code.
#[component]
fn Markdown(text: String) -> Element {
    let mut blocks = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("```") {
            let mut code = Vec::new();
            for next in lines.by_ref() {
                if next.starts_with("```") {
                    break;
                }
                code.push(next);
            }
            let code = code.join("\n");
            blocks.push(rsx! {
                CodeBlock { size: CodeBlockSize::Sm,
                    CodeBlockContent { wrap: true, code }
                }
            });
        } else if line.starts_with("- ") {
            let mut items = vec![line];
            while let Some(next) = lines.next_if(|next| next.starts_with("- ")) {
                items.push(next);
            }
            blocks.push(rsx! {
                ul { class: "list-disc ps-5",
                    for item in items {
                        li { Inline { text: item.trim_start_matches("- ").to_string() } }
                    }
                }
            });
        } else if !line.trim().is_empty() {
            let line = line.to_string();
            blocks.push(rsx! {
                p {
                    Inline { text: line }
                }
            });
        }
    }

    rsx! {
        div { class: "flex flex-col gap-3",
            for (index , block) in blocks.into_iter().enumerate() {
                div { key: "{index}", {block} }
            }
        }
    }
}

/// `code` and **bold** spans.
#[component]
fn Inline(text: String) -> Element {
    let mut spans = Vec::new();
    let mut rest = text.as_str();
    while !rest.is_empty() {
        let code = rest.find('`');
        let bold = rest.find("**");
        let (start, marker) = match (code, bold) {
            (Some(code), Some(bold)) if bold < code => (bold, "**"),
            (Some(code), _) => (code, "`"),
            (None, Some(bold)) => (bold, "**"),
            (None, None) => {
                spans.push((rest.to_string(), ""));
                break;
            }
        };
        let after = &rest[start + marker.len()..];
        let Some(end) = after.find(marker) else {
            spans.push((rest.to_string(), ""));
            break;
        };
        if start > 0 {
            spans.push((rest[..start].to_string(), ""));
        }
        spans.push((after[..end].to_string(), marker));
        rest = &after[end + marker.len()..];
    }

    rsx! {
        for (index , (span , marker)) in spans.into_iter().enumerate() {
            match marker {
                "`" => rsx! {
                    code { key: "{index}", class: "rounded bg-muted px-1 py-0.5 font-mono text-[0.85em]", "{span}" }
                },
                "**" => rsx! {
                    strong { key: "{index}", class: "font-semibold", "{span}" }
                },
                _ => rsx! {
                    span { key: "{index}", "{span}" }
                },
            }
        }
    }
}
