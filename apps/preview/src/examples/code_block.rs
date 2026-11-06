//! Code Block examples, ported from shadcn/ui `examples/base/code-block-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ChevronDown, Download, FileCode, RotateCcw, Terminal, TextWrap};

use super::Example;
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::code_block::{
    CodeBlock, CodeBlockActions, CodeBlockContent, CodeBlockCopyButton, CodeBlockFooter,
    CodeBlockHeader, CodeBlockLine, CodeBlockLineVariant, CodeBlockSize, CodeBlockTitle,
    CodeBlockVariant, highlight_lines,
};
use crate::components::ui::message::{Message, MessageContent};
use crate::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        CodeBlockDemo {}
        CodeBlockBasic {}
        CodeBlockHeaderExample {}
        CodeBlockActionsExample {}
        CodeBlockFloatingActions {}
        CodeBlockLineNumbers {}
        CodeBlockStartLine {}
        CodeBlockWrap {}
        CodeBlockHighlightedLines {}
        CodeBlockDiff {}
        CodeBlockHeight {}
        CodeBlockShowMore {}
        CodeBlockShowMoreScroll {}
        CodeBlockSizeExample {}
        CodeBlockDestructive {}
        CodeBlockTrace {}
        CodeBlockTabs {}
        CodeBlockChat {}
        CodeBlockTypeset {}
        CodeBlockRtl {}
        CodeBlockHighlighting {}
        CodeBlockLanguages {}
        CodeBlockAnsi {}
        CodeBlockStreaming {}
        CodeBlockMarkdown {}
    }
}

/// One highlighted `CodeBlockLine` per line of `code`, like upstream's `useHighlightedCode(...).map(...)`.
fn highlighted(code: &str, language: &str) -> Element {
    rsx! {
        for (index , line) in highlight_lines(code, language).into_iter().enumerate() {
            CodeBlockLine { key: "{index}", {line} }
        }
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

fn download(value: &str, filename: &str) {
    let eval = document::eval(
        "const [value, filename] = await dioxus.recv(); const url = URL.createObjectURL(new Blob([value], { type: 'text/plain' })); const link = document.createElement('a'); link.href = url; link.download = filename; link.click(); URL.revokeObjectURL(url);",
    );
    let _ = eval.send((value.to_string(), filename.to_string()));
}

const ROUTE_CODE: &str = r#"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    system: "You are a helpful assistant. Answer in one short paragraph.",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"#;

#[component]
fn CodeBlockDemo() -> Element {
    let mut wrap = use_signal(|| false);
    let lines = highlight_lines(ROUTE_CODE, "ts");

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "w-full max-w-lg",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle {
                            FileCode {}
                            "app/api/chat/route.ts"
                        }
                        CodeBlockActions {
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::IconXs,
                                "aria-label": "Wrap lines",
                                "aria-pressed": wrap().to_string(),
                                class: "aria-pressed:text-foreground",
                                onclick: move |_| wrap.set(!wrap()),
                                TextWrap {}
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::IconXs,
                                "aria-label": "Download",
                                onclick: move |_| download(ROUTE_CODE, "route.ts"),
                                Download {}
                            }
                            CodeBlockCopyButton { value: ROUTE_CODE }
                        }
                    }
                    CodeBlockContent { wrap: wrap(), line_numbers: true,
                        for (index , line) in lines.into_iter().enumerate() {
                            CodeBlockLine {
                                key: "{index}",
                                variant: if (5..=9).contains(&index) { CodeBlockLineVariant::Highlighted } else { CodeBlockLineVariant::Default },
                                {line}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlockBasic() -> Element {
    let output = "$ pnpm build\n> next build\n\n ✓ Compiled successfully in 4.2s\n ✓ Generating static pages (12/12)\n\nRoute (app)                Size  First Load JS\n┌ ○ /                    1.9 kB         112 kB\n├ ○ /_not-found           993 B         103 kB\n└ ƒ /api/chat             141 B         102 kB";

    rsx! {
        Example { title: "Basic", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockContent { code: output }
                }
            }
        }
    }
}

#[component]
fn CodeBlockHeaderExample() -> Element {
    let command = "pnpm add ai @ai-sdk/react zod";
    let output = format!(
        "$ {command}\nPackages: +14\nProgress: resolved 412, reused 398, downloaded 14, added 14, done\nDone in 2.8s using pnpm v10.18.0"
    );

    rsx! {
        Example { title: "Header", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle {
                            Terminal {}
                            "Terminal"
                        }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: command }
                        }
                    }
                    CodeBlockContent { code: output }
                }
            }
        }
    }
}

#[component]
fn CodeBlockActionsExample() -> Element {
    let mut wrap = use_signal(|| true);
    let output = "$ pnpm lint\napps/web/components/message-list.tsx\n  18:22  error  'messages' is possibly 'undefined'. Add a guard before calling map  @typescript-eslint/no-unsafe-call\n  42:7   warning  React Hook React.useEffect has a missing dependency: 'scrollToBottom'  react-hooks/exhaustive-deps\n\n✖ 2 problems (1 error, 1 warning)";

    rsx! {
        Example { title: "Actions", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle {
                            Terminal {}
                            "Terminal"
                        }
                        CodeBlockActions {
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::IconXs,
                                "aria-label": "Wrap lines",
                                "aria-pressed": wrap().to_string(),
                                class: "aria-pressed:text-foreground",
                                onclick: move |_| wrap.set(!wrap()),
                                TextWrap {}
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::IconXs,
                                "aria-label": "Download",
                                onclick: move |_| download(output, "lint.log"),
                                Download {}
                            }
                            CodeBlockCopyButton { value: output }
                        }
                    }
                    CodeBlockContent { wrap: wrap(), code: output }
                }
            }
        }
    }
}

#[component]
fn CodeBlockFloatingActions() -> Element {
    let output = "$ pnpm add ai @ai-sdk/react zod\nProgress: resolved 412, reused 398, downloaded 14, added 14, done\ndependencies:\n+ @ai-sdk/react 3.0.12\n+ ai 6.0.12\n+ zod 4.1.12\nDone in 2.8s using pnpm v10.18.0";

    rsx! {
        Example { title: "Floating actions", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockActions {
                        CodeBlockCopyButton { value: output }
                    }
                    CodeBlockContent { code: output }
                }
            }
        }
    }
}

#[component]
fn CodeBlockLineNumbers() -> Element {
    let code = "{\n  \"name\": \"chat\",\n  \"private\": true,\n  \"scripts\": {\n    \"dev\": \"next dev\",\n    \"build\": \"next build\"\n  }\n}";

    rsx! {
        Example { title: "Line numbers", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockContent { line_numbers: true, code: code }
                }
            }
        }
    }
}

#[component]
fn CodeBlockStartLine() -> Element {
    let code = "  if (!session) {\n    redirect(\"/login\")\n  }\n\n  const projects = await db.project.findMany({\n    where: { ownerId: session.user.id },\n    orderBy: { updatedAt: \"desc\" },\n  })\n\n  return <ProjectList projects={projects} />\n}";

    rsx! {
        Example { title: "Start line", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle { "app/dashboard/page.tsx" }
                    }
                    CodeBlockContent { line_numbers: true, start_line: 995,
                        {highlighted(code, "tsx")}
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlockWrap() -> Element {
    let output = "Error: The model \"anthropic/claude-sonnet-4.5\" returned an error: Request too large. The prompt is 212,431 tokens and the context window is 200,000 tokens. Remove some messages or summarize the conversation and try again.";

    rsx! {
        Example { title: "Wrap", class: "h-auto",
            div { class: "flex w-full max-w-md flex-col gap-4",
                CodeBlock {
                    CodeBlockContent { code: output }
                }
                CodeBlock {
                    CodeBlockContent { wrap: true, code: output }
                }
            }
        }
    }
}

#[component]
fn CodeBlockHighlightedLines() -> Element {
    let code = "export default {\n  experimental: {\n    reactCompiler: true,\n    viewTransition: true,\n  },\n  images: {\n    remotePatterns: [{ hostname: \"avatars.githubusercontent.com\" }],\n  },\n}";
    let highlighted_lines = [2, 3];

    rsx! {
        Example { title: "Highlighted lines", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockContent { line_numbers: true,
                        for (index , line) in code.split('\n').enumerate() {
                            CodeBlockLine {
                                key: "{index}",
                                variant: if highlighted_lines.contains(&index) { CodeBlockLineVariant::Highlighted } else { CodeBlockLineVariant::Default },
                                "{line}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn diff_variant(line: &str) -> CodeBlockLineVariant {
    if line.starts_with('+') {
        CodeBlockLineVariant::Added
    } else if line.starts_with('-') {
        CodeBlockLineVariant::Removed
    } else {
        CodeBlockLineVariant::Default
    }
}

/// Drops the leading diff marker, which is not part of the code.
fn after_marker(line: &str) -> &str {
    line.get(1..).unwrap_or("")
}

#[component]
fn CodeBlockDiff() -> Element {
    let patch = " export function Greeting({ name }: { name: string }) {\n-  return <p>Hello, {name}</p>\n+  return <p className=\"text-sm text-muted-foreground\">\n+    Hello, {name}!\n+  </p>\n }";

    rsx! {
        Example { title: "Diff", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle { "components/greeting.tsx" }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: patch }
                        }
                    }
                    CodeBlockContent {
                        for (index , line) in patch.split('\n').enumerate() {
                            CodeBlockLine {
                                key: "{index}",
                                variant: diff_variant(line),
                                "{after_marker(line)}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlockHeight() -> Element {
    let logs = (0..40)
        .map(|index| {
            format!(
                "[{:02}] GET /api/chat 200 in {}ms",
                index + 1,
                12 + ((index * 7) % 40)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    rsx! {
        Example { title: "Height", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle { "Logs" }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: logs.clone() }
                        }
                    }
                    CodeBlockContent { class: "max-h-48 scroll-fade-y", code: logs }
                }
            }
        }
    }
}

const CHAT_COMPONENT: &str = r#""use client"

import * as React from "react"
import { useChat } from "@ai-sdk/react"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

export function Chat() {
  const [input, setInput] = React.useState("")
  const { messages, sendMessage, status } = useChat()

  return (
    <div className="flex flex-col gap-4">
      {messages.map((message) => (
        <div key={message.id}>
          {message.parts.map((part, index) =>
            part.type === "text" ? <p key={index}>{part.text}</p> : null
          )}
        </div>
      ))}
      <form
        onSubmit={(event) => {
          event.preventDefault()
          sendMessage({ text: input })
          setInput("")
        }}
      >
        <Input value={input} onChange={(event) => setInput(event.target.value)} />
        <Button disabled={status !== "ready"}>Send</Button>
      </form>
    </div>
  )
}"#;

#[component]
fn CodeBlockShowMore() -> Element {
    let mut expanded = use_signal(|| false);

    rsx! {
        Example { title: "Show more", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle { "components/chat.tsx" }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: CHAT_COMPONENT }
                        }
                    }
                    CodeBlockContent {
                        id: "code-block-show-more",
                        line_numbers: true,
                        class: if expanded() { "" } else { "max-h-64 scroll-fade-y overflow-y-hidden" },
                        {highlighted(CHAT_COMPONENT, "tsx")}
                    }
                    CodeBlockFooter { class: "justify-center",
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Xs,
                            "aria-expanded": expanded().to_string(),
                            "aria-controls": "code-block-show-more",
                            onclick: move |_| expanded.set(!expanded()),
                            ChevronDown { class: "transition-transform in-aria-expanded:rotate-180" }
                            if expanded() {
                                "Show less"
                            } else {
                                "Show more"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlockShowMoreScroll() -> Element {
    let mut expanded = use_signal(|| false);

    rsx! {
        Example { title: "Show more with scroll", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle { "components/chat.tsx" }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: CHAT_COMPONENT }
                        }
                    }
                    CodeBlockContent {
                        id: "code-block-show-more-scroll",
                        line_numbers: true,
                        class: if expanded() { "max-h-96 scroll-fade-y" } else { "max-h-64 scroll-fade-y overflow-y-hidden" },
                        {highlighted(CHAT_COMPONENT, "tsx")}
                    }
                    CodeBlockFooter { class: "justify-center",
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Xs,
                            "aria-expanded": expanded().to_string(),
                            "aria-controls": "code-block-show-more-scroll",
                            onclick: move |_| expanded.set(!expanded()),
                            ChevronDown { class: "transition-transform in-aria-expanded:rotate-180" }
                            if expanded() {
                                "Show less"
                            } else {
                                "Show more"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlockSizeExample() -> Element {
    let output = "$ pnpm add ai @ai-sdk/react zod\nProgress: resolved 412, reused 398, downloaded 14, added 14, done\nDone in 2.8s using pnpm v10.18.0";

    rsx! {
        Example { title: "Size", class: "h-auto",
            div { class: "flex w-full max-w-md flex-col gap-4",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle {
                            Terminal {}
                            "Terminal"
                        }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: output }
                        }
                    }
                    CodeBlockContent { wrap: true, code: output }
                }
                CodeBlock { size: CodeBlockSize::Sm,
                    CodeBlockHeader {
                        CodeBlockTitle {
                            Terminal {}
                            "Terminal"
                        }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: output }
                        }
                    }
                    CodeBlockContent { wrap: true, code: output }
                }
            }
        }
    }
}

#[component]
fn CodeBlockDestructive() -> Element {
    let error = "TypeError: Cannot read properties of undefined (reading 'map')\n    at MessageList (components/message-list.tsx:18:22)\n    at renderWithHooks (react-dom.development.js:16305:18)";

    rsx! {
        Example { title: "Destructive", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock { variant: CodeBlockVariant::Destructive,
                    CodeBlockHeader {
                        CodeBlockTitle { "Error" }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: error }
                        }
                    }
                    CodeBlockContent { wrap: true, code: error }
                }
            }
        }
    }
}

#[component]
fn CodeBlockTrace() -> Element {
    let output = "$ node --version\nv24.18.0";
    let error = "$ pnpm dev\nError: listen EADDRINUSE: address already in use :::3000\n    at Server.setupListenHandle [as _listen2] (node:net:1908:16)";

    rsx! {
        Example { title: "Trace", class: "h-auto",
            div { class: "w-full max-w-md",
                Trace {
                    TraceCollapsible { default_open: true,
                        TraceCollapsibleTrigger {
                            TraceLine {
                                TraceIcon {
                                    Terminal {}
                                }
                                TraceTitle { "Ran node --version" }
                            }
                        }
                        TraceCollapsibleContent {
                            CodeBlock { size: CodeBlockSize::Sm,
                                CodeBlockContent { wrap: true, class: "max-h-64", code: output }
                            }
                        }
                    }
                    TraceCollapsible { default_open: true,
                        TraceCollapsibleTrigger {
                            TraceLine { status: TraceStatus::Error,
                                TraceIcon {
                                    Terminal {}
                                }
                                TraceTitle { "Failed to run pnpm dev" }
                            }
                        }
                        TraceCollapsibleContent {
                            CodeBlock {
                                size: CodeBlockSize::Sm,
                                variant: CodeBlockVariant::Destructive,
                                CodeBlockContent { wrap: true, class: "max-h-64", code: error }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct TabFile {
    id: &'static str,
    title: &'static str,
    language: &'static str,
    code: &'static str,
}

const TAB_FILES: [TabFile; 3] = [
    TabFile {
        id: "route",
        title: "app/api/chat/route.ts",
        language: "ts",
        code: "import { streamText } from \"ai\"\n\nexport async function POST(request: Request) {\n  const { messages } = await request.json()\n  const result = streamText({ model: \"anthropic/claude-sonnet-4.5\", messages })\n\n  return result.toUIMessageStreamResponse()\n}",
    },
    TabFile {
        id: "chat",
        title: "components/chat.tsx",
        language: "tsx",
        code: "\"use client\"\n\nimport { useChat } from \"@ai-sdk/react\"\n\nexport function Chat() {\n  const { messages } = useChat()\n\n  return <MessageList messages={messages} />\n}",
    },
    TabFile {
        id: "env",
        title: ".env.local",
        language: "bash",
        code: "AI_GATEWAY_API_KEY=your-api-key",
    },
];

#[component]
fn CodeBlockTabs() -> Element {
    rsx! {
        Example { title: "Tabs", class: "h-auto",
            Tabs { default_value: "route", class: "w-full max-w-md",
                TabsList {
                    for file in TAB_FILES.iter() {
                        TabsTrigger {
                            key: "{file.id}",
                            value: file.id,
                            "{file.title.rsplit('/').next().unwrap_or(file.title)}"
                        }
                    }
                }
                for file in TAB_FILES.iter() {
                    TabsContent { key: "{file.id}", value: file.id,
                        CodeBlock {
                            CodeBlockHeader {
                                CodeBlockTitle { "{file.title}" }
                                CodeBlockActions {
                                    CodeBlockCopyButton { value: file.code }
                                }
                            }
                            CodeBlockContent { line_numbers: true, {highlighted(file.code, file.language)} }
                        }
                    }
                }
            }
        }
    }
}

const CHAT_ROUTE: &str = r#"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"#;

#[component]
fn CodeBlockChat() -> Element {
    rsx! {
        Example { title: "Chat", class: "h-auto",
            div { class: "flex w-full max-w-md flex-col gap-4 text-sm",
                div { class: "ml-auto flex max-w-[95%] flex-col",
                    Bubble { variant: BubbleVariant::Muted,
                        BubbleContent { "How do I stream a response?" }
                    }
                }
                Message {
                    MessageContent { class: "gap-3",
                        p { "Create a route handler that returns the stream:" }
                        CodeBlock {
                            CodeBlockHeader {
                                CodeBlockTitle { "app/api/chat/route.ts" }
                                CodeBlockActions {
                                    CodeBlockCopyButton { value: CHAT_ROUTE }
                                }
                            }
                            CodeBlockContent { {highlighted(CHAT_ROUTE, "ts")} }
                        }
                        p { "The client reads it with useChat." }
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlockTypeset() -> Element {
    let code = "import { streamText } from \"ai\"\n\nexport async function POST(request: Request) {\n  const { messages } = await request.json()\n  const result = streamText({ model: \"anthropic/claude-sonnet-4.5\", messages })\n\n  return result.toUIMessageStreamResponse()\n}";

    rsx! {
        Example { title: "Typeset", class: "h-auto",
            div { class: "typeset flex w-full max-w-md flex-col gap-4",
                h3 { class: "text-lg font-semibold", "Stream a response" }
                p {
                    "Create a route handler that calls "
                    code { class: "rounded-sm bg-muted px-1 py-0.5 font-mono text-[0.9em]",
                        "streamText"
                    }
                    " and returns the stream to the client."
                }
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle {
                            FileCode {}
                            "app/api/chat/route.ts"
                        }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: code }
                        }
                    }
                    CodeBlockContent { line_numbers: true, {highlighted(code, "ts")} }
                }
                p {
                    "The "
                    code { class: "rounded-sm bg-muted px-1 py-0.5 font-mono text-[0.9em]",
                        "useChat"
                    }
                    " hook reads the stream on the client and renders the answer while it is written."
                }
            }
        }
    }
}

#[component]
fn CodeBlockRtl() -> Element {
    let mut language = use_signal(|| "ar");
    let (dir, description, title) = match language() {
        "en" => (
            "ltr",
            "Install the packages, then start the dev server.",
            "Terminal",
        ),
        "he" => ("rtl", "התקן את החבילות, ואז הפעל את שרת הפיתוח.", "מסוף"),
        _ => ("rtl", "ثبّت الحزم، ثم شغّل خادم التطوير.", "الطرفية"),
    };
    let code = "pnpm add ai @ai-sdk/react zod\npnpm dev";

    rsx! {
        Example { title: "RTL", class: "h-auto",
            div { class: "flex w-full max-w-md flex-col gap-3 text-sm", dir: dir,
                div { class: "flex gap-1", dir: "ltr",
                    for (value , label) in [("en", "English"), ("ar", "العربية"), ("he", "עברית")] {
                        Button {
                            key: "{value}",
                            variant: if language() == value { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                            size: ButtonSize::Sm,
                            onclick: move |_| language.set(value),
                            "{label}"
                        }
                    }
                }
                p { "{description}" }
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle {
                            Terminal {}
                            "{title}"
                        }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: code }
                        }
                    }
                    CodeBlockContent { line_numbers: true, {highlighted(code, "bash")} }
                }
            }
        }
    }
}

#[component]
fn CodeBlockHighlighting() -> Element {
    let code = "{\n  \"location\": \"San Francisco\",\n  \"temperature\": 18,\n  \"unit\": \"celsius\",\n  \"conditions\": [\"fog\", \"wind\"]\n}";

    rsx! {
        Example { title: "Highlighting", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle { "getWeather output" }
                    }
                    CodeBlockContent { {highlighted(code, "json")} }
                }
            }
        }
    }
}

#[component]
fn CodeBlockLanguages() -> Element {
    let samples = [
        (
            "css",
            "app/globals.css",
            "@import \"tailwindcss\";\n\n:root {\n  --radius: 0.625rem;\n}",
        ),
        (
            "python",
            "main.py",
            "def greet(name: str) -> str:\n    return f\"Hello, {name}!\"",
        ),
        (
            "text",
            "notes.txt",
            "A language that is not in the list renders as plain text.",
        ),
    ];

    rsx! {
        Example { title: "Languages", class: "h-auto",
            div { class: "flex w-full max-w-md flex-col gap-4",
                for (language , title , code) in samples {
                    CodeBlock { key: "{language}",
                        CodeBlockHeader {
                            CodeBlockTitle { "{title}" }
                        }
                        CodeBlockContent { {highlighted(code, language)} }
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlockAnsi() -> Element {
    let output = "$ pnpm test\n\x1b[32m✓\x1b[39m src/format.test.ts \x1b[2m(4 tests)\x1b[22m\n\x1b[32m✓\x1b[39m src/parse.test.ts \x1b[2m(7 tests)\x1b[22m\n\x1b[31m✗\x1b[39m src/stream.test.ts \x1b[2m(3 tests | 1 failed)\x1b[22m\n\n\x1b[31mAssertionError: expected 'hello' to be 'hello world'\x1b[39m\n\nTests  \x1b[31m1 failed\x1b[39m | \x1b[32m13 passed\x1b[39m (14)";
    let plain = strip_ansi(output);

    rsx! {
        Example { title: "ANSI", class: "h-auto",
            div { class: "w-full max-w-md",
                CodeBlock {
                    CodeBlockHeader {
                        CodeBlockTitle {
                            Terminal {}
                            "Terminal"
                        }
                        CodeBlockActions {
                            CodeBlockCopyButton { value: plain }
                        }
                    }
                    CodeBlockContent { wrap: true, {highlighted(output, "ansi")} }
                }
            }
        }
    }
}

fn strip_ansi(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("\x1b[") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        match after.find('m') {
            Some(end) => rest = &after[end + 1..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

struct StreamBlock {
    title: Option<&'static str>,
    language: &'static str,
    code: &'static str,
    line_numbers: bool,
    highlight: &'static [usize],
    diff: bool,
}

const STREAM_BLOCKS: [StreamBlock; 5] = [
    StreamBlock {
        title: Some("hooks/use-debounce.ts"),
        language: "ts",
        line_numbers: true,
        highlight: &[],
        diff: false,
        code: "export function useDebounce<T>(value: T, delay = 300) {\n  const [debounced, setDebounced] = React.useState(value)\n\n  React.useEffect(() => {\n    const timeout = setTimeout(() => setDebounced(value), delay)\n    return () => clearTimeout(timeout)\n  }, [value, delay])\n\n  return debounced\n}",
    },
    StreamBlock {
        title: None,
        language: "bash",
        line_numbers: false,
        highlight: &[],
        diff: false,
        code: "pnpm dlx shadcn@latest add code-block",
    },
    StreamBlock {
        title: Some("components/search.tsx"),
        language: "tsx",
        line_numbers: true,
        highlight: &[],
        diff: true,
        code: " export function Search() {\n   const [query, setQuery] = React.useState(\"\")\n-  const results = useSearch(query)\n+  const debounced = useDebounce(query, 300)\n+  const results = useSearch(debounced)\n\n   return <SearchResults results={results} />\n }",
    },
    StreamBlock {
        title: Some("app/api/chat/route.ts"),
        language: "ts",
        line_numbers: true,
        highlight: &[5, 6, 7, 8],
        diff: false,
        code: "import { streamText } from \"ai\"\n\nexport async function POST(request: Request) {\n  const { messages } = await request.json()\n\n  const result = streamText({\n    model: \"anthropic/claude-sonnet-4.5\",\n    messages,\n  })\n\n  return result.toUIMessageStreamResponse()\n}",
    },
    StreamBlock {
        title: Some(".env.local"),
        language: "bash",
        line_numbers: false,
        highlight: &[],
        diff: false,
        code: "AI_GATEWAY_API_KEY=your-api-key",
    },
];

#[component]
fn StreamingBlock(index: usize, length: usize) -> Element {
    let block = &STREAM_BLOCKS[index];
    let full = block.code.chars().count();
    let code: String = block.code.chars().take(length).collect();
    let rows: Vec<&str> = code.split('\n').collect();
    let source = if block.diff {
        rows.iter()
            .map(|row| after_marker(row))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        code.clone()
    };
    let lines = highlight_lines(&source, block.language);

    rsx! {
        CodeBlock {
            if let Some(title) = block.title {
                CodeBlockHeader {
                    CodeBlockTitle { "{title}" }
                    CodeBlockActions {
                        CodeBlockCopyButton { value: source.clone(), disabled: length < full }
                    }
                }
            }
            CodeBlockContent { line_numbers: block.line_numbers,
                for (line_index , line) in lines.into_iter().enumerate() {
                    CodeBlockLine {
                        key: "{line_index}",
                        variant: if block.diff {
                            rows.get(line_index).map_or(CodeBlockLineVariant::Default, |row| diff_variant(row))
                        } else if block.highlight.contains(&line_index) {
                            CodeBlockLineVariant::Highlighted
                        } else {
                            CodeBlockLineVariant::Default
                        },
                        {line}
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlockStreaming() -> Element {
    let sizes: Vec<usize> = STREAM_BLOCKS
        .iter()
        .map(|block| block.code.chars().count())
        .collect();
    let total: usize = sizes.iter().sum();
    let mut length = use_signal(|| total);
    let offsets: Vec<usize> = (0..sizes.len()).map(|i| sizes[..i].iter().sum()).collect();

    rsx! {
        Example { title: "Streaming", class: "h-auto",
            div { class: "flex w-full max-w-md flex-col gap-4",
                for (index , offset) in offsets.iter().copied().enumerate() {
                    if length() > offset {
                        StreamingBlock {
                            key: "{index}",
                            index: index,
                            length: (length() - offset).min(sizes[index]),
                        }
                    }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "self-start",
                    disabled: length() < total,
                    onclick: move |_| {
                        length.set(0);
                        spawn(async move {
                            while length() < total {
                                sleep(16).await;
                                length.set(length() + 3);
                            }
                        });
                    },
                    RotateCcw {}
                    "Replay"
                }
            }
        }
    }
}

const MARKDOWN_RESPONSE: &str = "Wrap the value in a `useDebounce` hook. It returns the latest value once it stops changing for `delay` milliseconds.\n\n```tsx\nexport function useDebounce<T>(value: T, delay = 300) {\n  const [debounced, setDebounced] = React.useState(value)\n\n  React.useEffect(() => {\n    const timeout = setTimeout(() => setDebounced(value), delay)\n    return () => clearTimeout(timeout)\n  }, [value, delay])\n\n  return debounced\n}\n```\n\nPass the debounced value to your fetch instead of the raw input.";

enum Segment {
    Text(String),
    Code {
        language: String,
        code: String,
        complete: bool,
    },
}

/// Splits streamed markdown into paragraphs and fenced code blocks. An unclosed fence is an incomplete block.
fn split_markdown(text: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("```") {
        let before = rest[..start].trim();
        if !before.is_empty() {
            segments.push(Segment::Text(before.to_string()));
        }
        let after = &rest[start + 3..];
        let (language, body) = after.split_once('\n').unwrap_or((after, ""));
        match body.find("```") {
            Some(end) => {
                segments.push(Segment::Code {
                    language: language.trim().to_string(),
                    code: body[..end].trim_end_matches('\n').to_string(),
                    complete: true,
                });
                rest = &body[end + 3..];
            }
            None => {
                segments.push(Segment::Code {
                    language: language.trim().to_string(),
                    code: body.to_string(),
                    complete: false,
                });
                rest = "";
            }
        }
    }
    let tail = rest.trim();
    if !tail.is_empty() {
        segments.push(Segment::Text(tail.to_string()));
    }
    segments
}

#[component]
fn MarkdownText(text: String) -> Element {
    rsx! {
        p {
            for (index , part) in text.split('`').enumerate() {
                if index % 2 == 1 {
                    code {
                        key: "{index}",
                        class: "rounded-sm bg-muted px-1 py-0.5 font-mono text-[0.9em]",
                        "{part}"
                    }
                } else {
                    "{part}"
                }
            }
        }
    }
}

#[component]
fn CodeBlockMarkdown() -> Element {
    let mut streamed = use_signal(|| MARKDOWN_RESPONSE.chars().count());
    let total = MARKDOWN_RESPONSE.chars().count();
    let busy = streamed() < total;
    let text: String = MARKDOWN_RESPONSE.chars().take(streamed()).collect();

    rsx! {
        Example { title: "Markdown", class: "h-auto",
            div { class: "flex min-h-112 w-full max-w-md flex-col gap-4 py-6 text-sm",
                div { class: "ml-auto flex max-w-[95%] flex-col",
                    Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                        BubbleContent { "How do I debounce a search input in React?" }
                    }
                }
                Message {
                    MessageContent {
                        for (index , segment) in split_markdown(&text).into_iter().enumerate() {
                            match segment {
                                Segment::Text(text) => rsx! {
                                    MarkdownText { key: "{index}", text }
                                },
                                Segment::Code { language, code, complete } => {
                                    let title = if language.is_empty() { "text".to_string() } else { language.clone() };
                                    rsx! {
                                    CodeBlock { key: "{index}",
                                        CodeBlockHeader {
                                            CodeBlockTitle { "{title}" }
                                            CodeBlockActions {
                                                CodeBlockCopyButton { value: code.clone(), disabled: !complete }
                                            }
                                        }
                                        CodeBlockContent { {highlighted(&code, &language)} }
                                    }
                                    }
                                }
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
                            streamed.set(0);
                            spawn(async move {
                                while streamed() < total {
                                    sleep(30).await;
                                    streamed.set(streamed() + 4);
                                }
                            });
                        },
                        "Replay"
                    }
                }
            }
        }
    }
}
