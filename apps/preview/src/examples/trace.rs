//! Trace examples, ported from shadcn/ui `examples/base/trace-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    ArrowUpRight, Brain, ChevronRight, Circle, CircleAlert, CircleCheck, FilePen, FilePlus,
    FileText, FolderSearch, GitBranch, GitCommitHorizontal, Globe, LoaderCircle, Search, Terminal,
};

use super::Example;
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::code_block::{
    CodeBlock, CodeBlockContent, CodeBlockLine, CodeBlockLineVariant, CodeBlockSize,
    CodeBlockVariant,
};
use crate::components::ui::dialog::{
    Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle,
};
use crate::components::ui::message::{Message, MessageContent};
use crate::components::ui::sheet::{
    Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle,
};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceDescription,
    TraceIcon, TraceLine, TraceStatus, TraceTitle,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        TraceDemo {}
        TraceStatic {}
        TraceStatusExample {}
        TraceDescriptionDemo {}
        TraceCollapsibleDemo {}
        TraceOpen {}
        TraceLink {}
        TraceDialog {}
        TraceSheet {}
        TraceGroupDemo {}
        TraceSources {}
        TraceSteps {}
        TraceCommit {}
        TraceTurn {}
        TraceThinking {}
        TraceToolCalls {}
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}))"
    ))
    .await;
}

/// A chat turn from the user, on the end side, as upstream's `Message align="end"`.
#[component]
fn UserMessage(children: Element) -> Element {
    rsx! {
        div { class: "ml-auto flex max-w-[95%] flex-col",
            Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                BubbleContent { {children} }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum ToolState {
    Running,
    Done(String),
    Failed(String),
}

#[derive(Clone, PartialEq)]
enum Part {
    Reasoning {
        text: String,
        streaming: bool,
    },
    Tool {
        command: &'static str,
        state: ToolState,
    },
    Source {
        url: &'static str,
        title: &'static str,
    },
    Text(String),
}

/// Appends reasoning word by word, like a streamed response.
async fn stream_reasoning(mut parts: Signal<Vec<Part>>, text: &str, delay_ms: u32) {
    parts.write().push(Part::Reasoning {
        text: String::new(),
        streaming: true,
    });
    let index = parts.read().len() - 1;
    for word in text.split_inclusive(' ') {
        sleep(delay_ms).await;
        if let Some(Part::Reasoning { text, .. }) = parts.write().get_mut(index) {
            text.push_str(word);
        }
    }
    if let Some(Part::Reasoning { streaming, .. }) = parts.write().get_mut(index) {
        *streaming = false;
    }
}

async fn run_tool(
    mut parts: Signal<Vec<Part>>,
    command: &'static str,
    wait_ms: u32,
    result: Result<&str, &str>,
) {
    parts.write().push(Part::Tool {
        command,
        state: ToolState::Running,
    });
    let index = parts.read().len() - 1;
    sleep(wait_ms).await;
    if let Some(Part::Tool { state, .. }) = parts.write().get_mut(index) {
        *state = match result {
            Ok(stdout) => ToolState::Done(stdout.to_string()),
            Err(error) => ToolState::Failed(error.to_string()),
        };
    }
}

fn spinner(running: bool, idle: Element) -> Element {
    if running {
        rsx! {
            LoaderCircle { class: "animate-spin" }
        }
    } else {
        idle
    }
}

#[component]
fn ThinkingRow(text: String, running: bool, brain: bool) -> Element {
    let mut chosen = use_signal(|| None::<bool>);
    let title = match (brain, running) {
        (_, true) => "Thinking",
        (true, false) => "Thought",
        (false, false) => "Thought for 5s",
    };
    let line = rsx! {
        TraceLine { status: if running { TraceStatus::Running } else { TraceStatus::Complete },
            TraceIcon {
                if brain {
                    Brain {}
                } else {
                    {spinner(running, rsx! {
                        CircleCheck {}
                    })}
                }
            }
            TraceTitle { "{title}" }
        }
    };

    if text.is_empty() {
        return line;
    }

    rsx! {
        TraceCollapsible {
            open: chosen().unwrap_or(running),
            on_open_change: move |next| chosen.set(Some(next)),
            TraceCollapsibleTrigger { {line} }
            TraceCollapsibleContent { class: "whitespace-pre-wrap", "{text}" }
        }
    }
}

#[component]
fn ToolRow(
    command: &'static str,
    state: ToolState,
    streaming: bool,
    #[props(default)] default_open: bool,
    /// The tool-calls example shows every command with a terminal icon and a plain code block.
    #[props(default)]
    plain: bool,
) -> Element {
    match state {
        ToolState::Done(_) | ToolState::Failed(_) => {
            let (failed, output) = match &state {
                ToolState::Failed(error) => (true, error.clone()),
                ToolState::Done(stdout) => (false, stdout.clone()),
                ToolState::Running => (false, String::new()),
            };
            let verb = if failed { "Failed to run" } else { "Ran" };
            rsx! {
                TraceCollapsible { default_open: default_open,
                    TraceCollapsibleTrigger {
                        TraceLine { status: if failed { TraceStatus::Error } else { TraceStatus::Complete },
                            TraceIcon {
                                if plain {
                                    Terminal {}
                                } else if failed {
                                    CircleAlert {}
                                } else {
                                    CircleCheck {}
                                }
                            }
                            TraceTitle { "{verb} {command}" }
                        }
                    }
                    TraceCollapsibleContent {
                        CodeBlock {
                            size: CodeBlockSize::Sm,
                            variant: if failed && !plain { CodeBlockVariant::Destructive } else { CodeBlockVariant::Default },
                            CodeBlockContent { wrap: true, class: "max-h-64", code: output }
                        }
                    }
                }
            }
        }
        ToolState::Running => {
            let verb = if streaming {
                "Running"
            } else {
                "Failed to run"
            };
            rsx! {
            TraceLine { status: if streaming { TraceStatus::Running } else { TraceStatus::Error },
                TraceIcon {
                    {spinner(streaming, rsx! {
                        CircleAlert {}
                    })}
                }
                TraceTitle { "{verb} {command}" }
            }
            }
        }
    }
}

#[component]
fn SourceLinks(sources: Vec<(&'static str, &'static str)>) -> Element {
    rsx! {
        Trace {
            for (url , title) in sources {
                a { key: "{url}", href: url, target: "_blank", rel: "noreferrer",
                    TraceLine {
                        TraceIcon {
                            Globe {}
                        }
                        TraceTitle {
                            "{title}"
                            span { class: "sr-only", " (opens in a new tab)" }
                        }
                        TraceIcon {
                            ArrowUpRight {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SourcesRow(sources: Vec<(&'static str, &'static str)>, running: bool) -> Element {
    let verb = if running { "Using" } else { "Used" };
    rsx! {
        TraceCollapsible {
            TraceCollapsibleTrigger {
                TraceLine { status: if running { TraceStatus::Running } else { TraceStatus::Complete },
                    TraceIcon {
                        {spinner(running, rsx! {
                            Globe {}
                        })}
                    }
                    TraceTitle { "{verb} {sources.len()} sources" }
                }
            }
            TraceCollapsibleContent {
                SourceLinks { sources }
            }
        }
    }
}

const DEMO_REASONING: &str = "CI installs with a frozen lockfile, so the install stops when package.json and pnpm-lock.yaml disagree. I should check what the last commit changed, then reproduce the install.";
const DEMO_ANSWER: &str = "The last commit bumped a version in package.json without updating pnpm-lock.yaml, so the frozen install fails. Run pnpm install locally and commit the lockfile.";
const DEMO_ERROR: &str = "ERR_PNPM_OUTDATED_LOCKFILE  Cannot install with \"frozen-lockfile\" because pnpm-lock.yaml is not up to date with package.json";
const DEMO_SOURCES: [(&str, &str); 2] = [
    (
        "https://pnpm.io/cli/install#--frozen-lockfile",
        "pnpm install | pnpm.io",
    ),
    (
        "https://pnpm.io/continuous-integration",
        "Continuous Integration | pnpm.io",
    ),
];

fn demo_transcript() -> Vec<Part> {
    let mut parts = vec![
        Part::Reasoning {
            text: DEMO_REASONING.to_string(),
            streaming: false,
        },
        Part::Tool {
            command: "pnpm --version",
            state: ToolState::Done("10.18.0".into()),
        },
        Part::Tool {
            command: "git diff --stat HEAD~1 -- package.json pnpm-lock.yaml",
            state: ToolState::Done(
                " package.json | 2 +-\n 1 file changed, 1 insertion(+), 1 deletion(-)".into(),
            ),
        },
        Part::Tool {
            command: "pnpm install --frozen-lockfile",
            state: ToolState::Failed(DEMO_ERROR.into()),
        },
    ];
    parts.extend(
        DEMO_SOURCES
            .iter()
            .map(|&(url, title)| Part::Source { url, title }),
    );
    parts.push(Part::Text(DEMO_ANSWER.into()));
    parts
}

#[component]
fn TraceDemo() -> Element {
    let mut parts = use_signal(demo_transcript);
    let mut busy = use_signal(|| false);

    let rows: Vec<Part> = parts()
        .into_iter()
        .filter(|part| matches!(part, Part::Reasoning { .. } | Part::Tool { .. }))
        .collect();
    let sources: Vec<(&'static str, &'static str)> = parts()
        .iter()
        .filter_map(|part| match part {
            Part::Source { url, title } => Some((*url, *title)),
            _ => None,
        })
        .collect();
    let text = parts()
        .iter()
        .find_map(|part| match part {
            Part::Text(text) => Some(text.clone()),
            _ => None,
        })
        .unwrap_or_default();

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                UserMessage { "Why does pnpm install fail in CI?" }
                if !parts().is_empty() {
                    Message {
                        MessageContent { class: "gap-4",
                            if rows.len() + sources.len() > 0 {
                                Trace {
                                    for (index , part) in rows.iter().cloned().enumerate() {
                                        match part {
                                            Part::Reasoning { text, streaming } => rsx! {
                                                ThinkingRow { key: "{index}", text, running: busy() && streaming, brain: false }
                                            },
                                            Part::Tool { command, state } => rsx! {
                                                ToolRow {
                                                    key: "{index}",
                                                    command,
                                                    state,
                                                    streaming: busy(),
                                                    default_open: index == 2,
                                                }
                                            },
                                            _ => rsx! {},
                                        }
                                    }
                                    if !sources.is_empty() {
                                        SourcesRow { sources: sources.clone(), running: busy() && text.is_empty() }
                                    }
                                }
                            }
                            if !text.is_empty() {
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
                            parts.set(Vec::new());
                            busy.set(true);
                            spawn(async move {
                                stream_reasoning(parts, DEMO_REASONING, 40).await;
                                run_tool(parts, "pnpm --version", 700, Ok("10.18.0")).await;
                                run_tool(
                                        parts,
                                        "git diff --stat HEAD~1 -- package.json pnpm-lock.yaml",
                                        900,
                                        Ok(
                                            " package.json | 2 +-\n 1 file changed, 1 insertion(+), 1 deletion(-)",
                                        ),
                                    )
                                    .await;
                                run_tool(parts, "pnpm install --frozen-lockfile", 1200, Err(DEMO_ERROR)).await;
                                for (url, title) in DEMO_SOURCES {
                                    parts.write().push(Part::Source { url, title });
                                }
                                sleep(1000).await;
                                parts.write().push(Part::Text(DEMO_ANSWER.into()));
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
fn TraceStatic() -> Element {
    rsx! {
        Example { title: "Static", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceLine {
                    TraceIcon {
                        GitBranch {}
                    }
                    TraceTitle { "Created branch fix/frozen-lockfile" }
                }
            }
        }
    }
}

#[component]
fn TraceStatusExample() -> Element {
    rsx! {
        Example { title: "Status", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceLine {
                    TraceIcon {
                        CircleCheck {}
                    }
                    TraceTitle { "Ran pnpm install" }
                }
                TraceLine { status: TraceStatus::Error,
                    TraceIcon {
                        CircleAlert {}
                    }
                    TraceTitle { "Failed to run pnpm lint" }
                }
                TraceLine { status: TraceStatus::Running,
                    TraceIcon {
                        LoaderCircle { class: "animate-spin" }
                    }
                    TraceTitle { "Running pnpm build" }
                }
                TraceLine { status: TraceStatus::Pending,
                    TraceIcon {
                        Circle {}
                    }
                    TraceTitle { "Run pnpm test" }
                }
            }
        }
    }
}

#[component]
fn TraceDescriptionDemo() -> Element {
    rsx! {
        Example { title: "Description", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceLine {
                    TraceIcon {
                        FilePen {}
                    }
                    TraceTitle { "Edited components/settings/notification-preferences-form.tsx" }
                    TraceDescription { class: "font-mono", "+48 −12" }
                }
                TraceLine {
                    TraceIcon {
                        Terminal {}
                    }
                    TraceTitle { "Ran pnpm test" }
                    TraceDescription { "14s" }
                }
                TraceLine {
                    TraceIcon {
                        Search {}
                    }
                    TraceTitle { "Searched for useChat" }
                    TraceDescription { "8 results" }
                }
            }
        }
    }
}

#[component]
fn TraceCollapsibleDemo() -> Element {
    rsx! {
        Example { title: "Collapsible", class: "h-auto",
            div { class: "w-full max-w-sm",
                TraceCollapsible {
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
                            CodeBlockContent { wrap: true, class: "max-h-64", code: "$ node --version\nv24.18.0" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceOpen() -> Element {
    let mut open = use_signal(|| false);
    let verb = if open() { "Hide" } else { "Show" };

    rsx! {
        Example { title: "Open", class: "h-auto",
            div { class: "flex w-full max-w-sm flex-col items-start gap-4",
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
                            CodeBlockContent { wrap: true, class: "max-h-64", code: "$ node --version\nv24.18.0" }
                        }
                    }
                }
                TraceCollapsible {
                    open: open(),
                    on_open_change: move |next| open.set(next),
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                Terminal {}
                            }
                            TraceTitle { "Ran pnpm --version" }
                        }
                    }
                    TraceCollapsibleContent {
                        CodeBlock { size: CodeBlockSize::Sm,
                            CodeBlockContent { wrap: true, class: "max-h-64", code: "$ pnpm --version\n10.33.4" }
                        }
                    }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| open.set(!open()),
                    "{verb} the pnpm output"
                }
            }
        }
    }
}

#[component]
fn TraceLink() -> Element {
    rsx! {
        Example { title: "Link", class: "h-auto",
            Trace { class: "max-w-sm",
                a { href: "#link",
                    TraceLine {
                        TraceIcon {
                            FileText {}
                        }
                        TraceTitle { "Read apps/v4/lib/docs.ts" }
                        TraceIcon {
                            ChevronRight {}
                        }
                    }
                }
                a {
                    href: "https://docs.astro.build/en/install-and-setup/",
                    target: "_blank",
                    rel: "noreferrer",
                    TraceLine {
                        TraceIcon {
                            Globe {}
                        }
                        TraceTitle {
                            "Install Astro | docs.astro.build"
                            span { class: "sr-only", " (opens in a new tab)" }
                        }
                        TraceIcon {
                            ArrowUpRight {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceDialog() -> Element {
    let mut open = use_signal(|| false);
    let results = vec![
        (
            "https://nextjs.org/docs/app/guides/caching",
            "Caching build outputs",
        ),
        (
            "https://nextjs.org/docs/app/api-reference/components/image",
            "Image optimization reference",
        ),
        (
            "https://vercel.com/docs/deploy-hooks",
            "Deploy hooks and cache keys",
        ),
    ];

    rsx! {
        Example { title: "Dialog", class: "h-auto",
            Trace { class: "max-w-sm",
                Dialog { open: open(), on_open_change: move |next| open.set(next),
                    button { r#type: "button", onclick: move |_| open.set(true),
                        TraceLine {
                            TraceIcon {
                                Globe {}
                            }
                            TraceTitle { "Searched the web for cache keys" }
                            TraceIcon {
                                ChevronRight {}
                            }
                        }
                    }
                    DialogContent {
                        DialogHeader {
                            DialogTitle { "Searched the web for cache keys" }
                            DialogDescription { "Three results were used in the response." }
                        }
                        SourceLinks { sources: results }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceSheet() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Example { title: "Sheet", class: "h-auto",
            Trace { class: "max-w-sm",
                Sheet { open: open(), on_open_change: move |next| open.set(next),
                    button { r#type: "button", onclick: move |_| open.set(true),
                        TraceLine {
                            TraceIcon {
                                FolderSearch {}
                            }
                            TraceTitle { "Explored 4 files" }
                            TraceIcon {
                                ChevronRight {}
                            }
                        }
                    }
                    SheetContent {
                        SheetHeader {
                            SheetTitle { "Explored 4 files" }
                            SheetDescription { "Files read while preparing the response." }
                        }
                        ul { class: "flex flex-col gap-2 px-6 font-mono text-xs",
                            li { "apps/v4/lib/docs.ts" }
                            li { "apps/v4/lib/components.ts" }
                            li { "apps/v4/next.config.mjs" }
                            li { "apps/v4/package.json" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceGroupDemo() -> Element {
    rsx! {
        Example { title: "Group", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceLine {
                    TraceIcon {
                        Search {}
                    }
                    TraceTitle { "Searched for minimumReleaseAge" }
                }
                TraceLine {
                    TraceIcon {
                        FileText {}
                    }
                    TraceTitle { "Read pnpm-workspace.yaml" }
                }
                TraceLine {
                    TraceIcon {
                        Terminal {}
                    }
                    TraceTitle { "Ran pnpm install" }
                }
            }
        }
    }
}

#[component]
fn TraceSources() -> Element {
    let sources = vec![
        (
            "https://docs.astro.build/en/install-and-setup/",
            "Install Astro | docs.astro.build",
        ),
        (
            "https://www.npmjs.com/package/create-astro",
            "create-astro | npmjs.com",
        ),
        (
            "https://pnpm.io/settings#minimumreleaseage",
            "Settings: minimumReleaseAge | pnpm.io",
        ),
    ];

    rsx! {
        Example { title: "Sources", class: "h-auto",
            div { class: "w-full max-w-sm",
                TraceCollapsible { default_open: true,
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                Globe {}
                            }
                            TraceTitle { "Used {sources.len()} sources" }
                        }
                    }
                    TraceCollapsibleContent {
                        SourceLinks { sources }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceSteps() -> Element {
    rsx! {
        Example { title: "Steps", class: "h-auto",
            div { class: "w-full max-w-sm",
                TraceCollapsible { default_open: true,
                    TraceCollapsibleTrigger {
                        TraceLine { status: TraceStatus::Running,
                            TraceTitle { "Looking into the slow deploy" }
                        }
                    }
                    TraceCollapsibleContent {
                        Trace {
                            TraceCollapsible {
                                TraceCollapsibleTrigger {
                                    TraceLine {
                                        TraceIcon {
                                            Search {}
                                        }
                                        TraceTitle { "Searched for the checkout build log" }
                                    }
                                }
                                TraceCollapsibleContent { class: "flex flex-wrap gap-1.5",
                                    Badge { variant: BadgeVariant::Secondary, "vercel.com" }
                                    Badge { variant: BadgeVariant::Secondary, "github.com" }
                                }
                            }
                            a { href: "#steps",
                                TraceLine {
                                    TraceIcon {
                                        FileText {}
                                    }
                                    TraceTitle { "Read the last twelve deploys" }
                                    TraceIcon {
                                        ChevronRight {}
                                    }
                                }
                            }
                            TraceCollapsible {
                                TraceCollapsibleTrigger {
                                    TraceLine { status: TraceStatus::Running,
                                        TraceIcon {
                                            Search {}
                                        }
                                        TraceTitle { "Searching for the cache key" }
                                    }
                                }
                                TraceCollapsibleContent {
                                    CodeBlock { size: CodeBlockSize::Sm,
                                        CodeBlockContent {
                                            wrap: true,
                                            class: "max-h-64",
                                            code: "cache key image optimization",
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
}

struct CommitFile {
    path: &'static str,
    added: bool,
    additions: u32,
    deletions: u32,
    diff: &'static str,
}

const COMMIT_FILES: [CommitFile; 2] = [
    CommitFile {
        path: "app/login/page.tsx",
        added: false,
        additions: 2,
        deletions: 2,
        diff: "@@ -14,7 +14,7 @@ export default async function LoginPage({\n   searchParams,\n }) {\n   const { next } = await searchParams\n-  const redirectTo =\n-    next && next.startsWith(\"/\") ? next : \"/\"\n+  const redirectTo = safeRedirect(next)\n+\n   return <LoginForm redirectTo={redirectTo} />\n }",
    },
    CommitFile {
        path: "lib/auth/redirect.ts",
        added: true,
        additions: 7,
        deletions: 0,
        diff: "@@ -0,0 +1,7 @@\n+export function safeRedirect(next: string | undefined) {\n+  if (!next || !next.startsWith(\"/\") || next.startsWith(\"//\")) {\n+    return \"/\"\n+  }\n+\n+  return next\n+}",
    },
];

#[component]
fn DiffLines(diff: &'static str) -> Element {
    rsx! {
        for (index , line) in diff.split('\n').enumerate() {
            if line.starts_with("@@") {
                CodeBlockLine { key: "{index}", class: "text-muted-foreground", "{line}" }
            } else {
                CodeBlockLine {
                    key: "{index}",
                    variant: if line.starts_with('+') { CodeBlockLineVariant::Added } else if line.starts_with('-') { CodeBlockLineVariant::Removed } else { CodeBlockLineVariant::Default },
                    "{line.get(1..).unwrap_or(\"\")}"
                }
            }
        }
    }
}

#[component]
fn CommitFileSheet(index: usize) -> Element {
    let file = &COMMIT_FILES[index];
    let verb = if file.added { "Added" } else { "Edited" };
    let mut open = use_signal(|| false);

    rsx! {
        Sheet { open: open(), on_open_change: move |next| open.set(next),
            button { r#type: "button", onclick: move |_| open.set(true),
                TraceLine {
                    TraceIcon {
                        if file.added {
                            FilePlus {}
                        } else {
                            FilePen {}
                        }
                    }
                    TraceTitle { "{verb} {file.path}" }
                    TraceDescription { class: "font-mono", "+{file.additions} −{file.deletions}" }
                    TraceIcon {
                        ChevronRight {}
                    }
                }
            }
            SheetContent {
                SheetHeader {
                    SheetTitle { class: "font-mono", "{file.path}" }
                    SheetDescription { "{file.additions} additions, {file.deletions} deletions" }
                }
                CodeBlock { class: "mx-6 mb-6 w-auto",
                    CodeBlockContent {
                        DiffLines { diff: file.diff }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceCommit() -> Element {
    rsx! {
        Example { title: "Commit", class: "h-auto",
            Trace { class: "max-w-sm",
                TraceCollapsible { default_open: true,
                    TraceCollapsibleTrigger {
                        TraceLine {
                            TraceIcon {
                                GitCommitHorizontal {}
                            }
                            TraceTitle { "Committed a1b2c3: Fix the login redirect" }
                        }
                    }
                    TraceCollapsibleContent {
                        Trace {
                            for index in 0..COMMIT_FILES.len() {
                                CommitFileSheet { key: "{index}", index }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceTurn() -> Element {
    rsx! {
        Example { title: "Turn", class: "h-auto",
            div { class: "w-full max-w-sm",
                TraceCollapsible { default_open: true,
                    TraceCollapsibleTrigger {
                        TraceLine { status: TraceStatus::Error,
                            TraceTitle { "Worked for 2m 4s, 1 command failed" }
                        }
                    }
                    TraceCollapsibleContent { class: "flex flex-col gap-2",
                        Trace {
                            TraceCollapsible {
                                TraceCollapsibleTrigger {
                                    TraceLine {
                                        TraceIcon {
                                            Brain {}
                                        }
                                        TraceTitle { "Thought" }
                                        TraceDescription { "12s" }
                                    }
                                }
                                TraceCollapsibleContent { class: "whitespace-pre-wrap",
                                    "I should check which package managers are installed before I scaffold anything."
                                }
                            }
                            TraceCollapsible { default_open: true,
                                TraceCollapsibleTrigger {
                                    TraceLine { status: TraceStatus::Error,
                                        TraceIcon {
                                            Terminal {}
                                        }
                                        TraceTitle { "Ran 2 commands, 1 failed" }
                                    }
                                }
                                TraceCollapsibleContent {
                                    Trace {
                                        TraceLine {
                                            TraceIcon {
                                                Terminal {}
                                            }
                                            TraceTitle { "Ran node --version" }
                                        }
                                        TraceCollapsible {
                                            TraceCollapsibleTrigger {
                                                TraceLine { status: TraceStatus::Error,
                                                    TraceIcon {
                                                        Terminal {}
                                                    }
                                                    TraceTitle { "Failed to run pnpm --version" }
                                                }
                                            }
                                            TraceCollapsibleContent {
                                                CodeBlock {
                                                    size: CodeBlockSize::Sm,
                                                    variant: CodeBlockVariant::Destructive,
                                                    CodeBlockContent {
                                                        wrap: true,
                                                        class: "max-h-64",
                                                        code: "command not found: pnpm",
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        p { "Node is installed and pnpm is not. Reading the docs config." }
                        Trace {
                            a { href: "#turn",
                                TraceLine {
                                    TraceIcon {
                                        FileText {}
                                    }
                                    TraceTitle { "Read apps/v4/lib/docs.ts" }
                                    TraceIcon {
                                        ChevronRight {}
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

/// Consecutive parts of the same kind, so reasoning and tool calls share one `Trace` and text stays outside it.
fn group_parts(parts: &[Part]) -> Vec<(bool, Vec<Part>)> {
    let mut runs: Vec<(bool, Vec<Part>)> = Vec::new();
    for part in parts {
        let trace = !matches!(part, Part::Text(_));
        match runs.last_mut() {
            Some((kind, run)) if *kind == trace => run.push(part.clone()),
            _ => runs.push((trace, vec![part.clone()])),
        }
    }
    runs
}

#[component]
fn TraceRuns(parts: Vec<Part>, live: bool) -> Element {
    rsx! {
        for (run_index , (trace , run)) in group_parts(&parts).into_iter().enumerate() {
            if trace {
                Trace { key: "{run_index}",
                    for (index , part) in run.into_iter().enumerate() {
                        match part {
                            Part::Reasoning { text, streaming } => rsx! {
                                ThinkingRow { key: "{index}", text, running: live && streaming, brain: true }
                            },
                            Part::Tool { command, state } => rsx! {
                                ToolRow { key: "{index}", command, state, streaming: live, plain: true }
                            },
                            _ => rsx! {},
                        }
                    }
                }
            } else {
                for (index , part) in run.into_iter().enumerate() {
                    if let Part::Text(text) = part {
                        p { key: "{run_index}-{index}", "{text}" }
                    }
                }
            }
        }
    }
}

#[component]
fn TraceThinking() -> Element {
    let mut parts = use_signal(Vec::<Part>::new);
    let mut sent = use_signal(|| false);
    let mut busy = use_signal(|| false);
    let label = if sent() { "Reset" } else { "Send message" };

    rsx! {
        Example { title: "Thinking", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                if sent() {
                    UserMessage { "Why is the checkout deploy so slow?" }
                    Message {
                        MessageContent { class: "gap-4",
                            TraceRuns { parts: parts(), live: busy() }
                        }
                    }
                }
                if !busy() {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "mt-auto self-start",
                        onclick: move |_| {
                            if sent() {
                                sent.set(false);
                                parts.set(Vec::new());
                                return;
                            }
                            sent.set(true);
                            busy.set(true);
                            spawn(async move {
                                stream_reasoning(
                                        parts,
                                        "The build log shows install and compile finish in under a minute. Image optimization takes almost four, and it reruns on every deploy because the cache key includes the commit hash.",
                                        60,
                                    )
                                    .await;
                                sleep(400).await;
                                parts
                                    .write()
                                    .push(
                                        Part::Text(
                                            "Image optimization is the bottleneck. Key its cache on the lockfile and the image directory instead of the commit hash."
                                                .into(),
                                        ),
                                    );
                                busy.set(false);
                            });
                        },
                        "{label}"
                    }
                }
            }
        }
    }
}

#[component]
fn TraceToolCalls() -> Element {
    let mut parts = use_signal(Vec::<Part>::new);
    let mut sent = use_signal(|| false);
    let mut busy = use_signal(|| false);
    let label = if sent() { "Reset" } else { "Send message" };

    rsx! {
        Example { title: "Tool calls", class: "h-auto",
            div { class: "flex min-h-72 w-full max-w-sm flex-col gap-4 py-6 text-sm",
                if sent() {
                    UserMessage { "Which package managers are installed?" }
                    Message {
                        MessageContent { class: "gap-4",
                            TraceRuns { parts: parts(), live: busy() }
                        }
                    }
                }
                if !busy() {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "mt-auto self-start",
                        onclick: move |_| {
                            if sent() {
                                sent.set(false);
                                parts.set(Vec::new());
                                return;
                            }
                            sent.set(true);
                            busy.set(true);
                            spawn(async move {
                                run_tool(parts, "node --version", 900, Ok("v24.18.0")).await;
                                run_tool(parts, "npm --version", 900, Ok("11.6.2")).await;
                                run_tool(parts, "pnpm --version", 900, Err("command not found: pnpm")).await;
                                parts
                                    .write()
                                    .push(Part::Text("Node 24 and npm 11 are installed. pnpm is not.".into()));
                                busy.set(false);
                            });
                        },
                        "{label}"
                    }
                }
            }
        }
    }
}
