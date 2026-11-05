//! Stack trace: displays an error and its call frames in a collapsible monospace panel, with copy and expand controls.
//!
//! The raw trace is parsed in plain Rust: the first line is the error (`Type: message`),
//! every following line is a frame (`at fn (path:line:col)` or `at path:line:col`).
//! Frames under `node_modules`, `node:` or `internal/` are marked internal and can be hidden.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

const COPY_SCRIPT: &str = "const text = await dioxus.recv(); try { await navigator.clipboard.writeText(text); dioxus.send(true); } catch (error) { dioxus.send(false); }";

/// A `path:line:column` reference taken from a frame. `line` and `column` are absent when the frame has none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StackFileLocation {
    pub path: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

/// One call frame. `location` is the text after the function name, as written in the trace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StackFrame {
    pub function: Option<String>,
    pub location: String,
    pub file: StackFileLocation,
    pub internal: bool,
}

/// The error line split into type and message, followed by the frames.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParsedStackTrace {
    pub error_type: Option<String>,
    pub message: String,
    pub frames: Vec<StackFrame>,
}

/// Splits a raw trace into its error and frames. Blank lines are skipped.
pub fn parse_stack_trace(trace: &str) -> ParsedStackTrace {
    let mut lines = trace.lines().map(str::trim).filter(|line| !line.is_empty());
    let header = lines.next().unwrap_or_default();
    let (error_type, message) = match header.split_once(": ") {
        Some((kind, message)) if is_error_type(kind) => {
            (Some(kind.to_string()), message.to_string())
        }
        _ => (None, header.to_string()),
    };
    ParsedStackTrace {
        error_type,
        message,
        frames: lines.map(parse_frame).collect(),
    }
}

fn is_error_type(kind: &str) -> bool {
    !kind.is_empty() && !kind.contains(char::is_whitespace)
}

fn parse_frame(line: &str) -> StackFrame {
    let body = line.strip_prefix("at ").unwrap_or(line).trim();
    let (function, location) = match body
        .strip_suffix(')')
        .and_then(|rest| rest.rsplit_once(" ("))
    {
        Some((name, inside)) => (
            Some(name.trim().to_string()).filter(|n| !n.is_empty()),
            inside,
        ),
        None => (None, body),
    };
    StackFrame {
        function,
        location: location.to_string(),
        file: parse_file_location(location),
        internal: location.contains("node_modules")
            || location.starts_with("node:")
            || location.contains("internal/"),
    }
}

fn parse_file_location(text: &str) -> StackFileLocation {
    let parts: Vec<&str> = text.rsplitn(3, ':').collect();
    match parts.as_slice() {
        [column, line, path] if is_number(column) && is_number(line) => StackFileLocation {
            path: path.to_string(),
            line: line.parse().ok(),
            column: column.parse().ok(),
        },
        [line, path] if is_number(line) => StackFileLocation {
            path: path.to_string(),
            line: line.parse().ok(),
            column: None,
        },
        _ => StackFileLocation {
            path: text.to_string(),
            line: None,
            column: None,
        },
    }
}

fn is_number(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit())
}

#[derive(Clone, Copy)]
struct StackTraceContext {
    trace: Signal<String>,
}

#[component]
pub fn StackTrace(
    trace: String,
    open: Option<bool>,
    #[props(default)] default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "group/stack-trace flex w-full flex-col overflow-hidden rounded-md border bg-card font-mono text-sm text-card-foreground",
        take_class(&mut attributes).as_deref()
    );
    let mut trace_signal = use_signal(|| trace.clone());
    if *trace_signal.peek() != trace {
        trace_signal.set(trace);
    }
    use_context_provider(|| StackTraceContext {
        trace: trace_signal,
    });

    rsx! {
        div {
            "data-slot": "stack-trace",
            class: class,
            ..attributes,
            Collapsible {
                open: open,
                default_open: default_open,
                on_open_change: on_open_change,
                {children}
            }
        }
    }
}

#[component]
pub fn StackTraceHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center justify-between gap-2 px-3 py-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "stack-trace-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn StackTraceError(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex min-w-0 items-center gap-2 text-destructive",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "stack-trace-error",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The error type, such as `TypeError`. Renders nothing when the error line has no type.
#[component]
pub fn StackTraceErrorType(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<StackTraceContext>();
    let parsed = parse_stack_trace(&context.trace.read());
    let Some(kind) = parsed.error_type else {
        return rsx! {};
    };
    let class = cn!(
        "shrink-0 font-semibold",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "stack-trace-error-type",
            class: class,
            ..attributes,
            "{kind}"
        }
    }
}

#[component]
pub fn StackTraceErrorMessage(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<StackTraceContext>();
    let parsed = parse_stack_trace(&context.trace.read());
    let class = cn!("min-w-0 truncate", take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "stack-trace-error-message",
            class: class,
            ..attributes,
            {parsed.message.clone()}
        }
    }
}

#[component]
pub fn StackTraceActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex shrink-0 items-center gap-1",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "stack-trace-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Copies the raw trace to the clipboard. Shows a check mark for two seconds after a successful copy.
#[component]
pub fn StackTraceCopyButton(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<StackTraceContext>();
    let own_class = cn!(take_class(&mut attributes));
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        Some(own_class.as_str()),
    );
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            "data-slot": "stack-trace-copy-button",
            r#type: "button",
            aria_label: "Copy stack trace",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                let text = context.trace.peek().to_string();
                spawn(async move {
                    let mut eval = document::eval(COPY_SCRIPT);
                    let _ = eval.send(text);
                    let ok = eval.recv::<bool>().await.unwrap_or(false);
                    copied.set(ok);
                    if ok {
                        let _ = document::eval(
                            "await new Promise((resolve) => setTimeout(resolve, 2000))",
                        )
                        .await;
                        copied.set(false);
                    }
                });
            },
            ..attributes,
            if copied() {
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    "aria-hidden": "true",
                    path { d: "M5 12.5l4.5 4.5L19 7.5" }
                }
            } else {
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    "aria-hidden": "true",
                    path { d: "M9 9h11v11H9z" }
                    path { d: "M5 15V5h10" }
                }
            }
        }
        span {
            class: "sr-only",
            role: "status",
            aria_live: "polite",
            if copied() {
                "Copied"
            }
        }
    }
}

/// Opens and closes the frames. Rotates its chevron while open.
#[component]
pub fn StackTraceExpandButton(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("flex shrink-0", take_class(&mut attributes).as_deref());
    let trigger_class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        Some("transition-transform [&[data-state=open]>svg]:rotate-180"),
    );

    rsx! {
        div {
            "data-slot": "stack-trace-expand-button",
            class: class,
            ..attributes,
            CollapsibleTrigger {
                class: trigger_class,
                aria_label: "Show or hide stack trace",
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    "aria-hidden": "true",
                    path { d: "M6 9l6 6 6-6" }
                }
            }
        }
    }
}

/// Holds the frames. Its content is unmounted while the trace is collapsed.
#[component]
pub fn StackTraceContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("block", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "stack-trace-content",
            class: class,
            ..attributes,
            CollapsibleContent {
                {children}
            }
        }
    }
}

/// The call frames, one per line. Set `show_internal` to false to hide frames from `node_modules`, `node:` and `internal/`.
/// Pass `on_file_path_click` to turn each location into a button.
#[component]
pub fn StackTraceFrames(
    #[props(default = true)] show_internal: bool,
    on_file_path_click: Option<EventHandler<StackFileLocation>>,
    #[props(extends = GlobalAttributes, extends = ol)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<StackTraceContext>();
    let parsed = parse_stack_trace(&context.trace.read());
    let class = cn!(
        "max-h-72 overflow-auto border-t bg-muted px-3 py-2 text-xs",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        ol {
            "data-slot": "stack-trace-frames",
            class: class,
            ..attributes,
            for (index, frame) in parsed
                .frames
                .iter()
                .enumerate()
                .filter(|(_, frame)| show_internal || !frame.internal)
            {
                li {
                    key: "{index}",
                    "data-slot": "stack-trace-frame",
                    "data-internal": frame.internal.then_some("true"),
                    class: if frame.internal { "flex min-w-0 gap-2 py-0.5 whitespace-pre text-muted-foreground" } else { "flex min-w-0 gap-2 py-0.5 whitespace-pre text-foreground" },
                    if let Some(function) = frame.function.clone() {
                        span {
                            "data-slot": "stack-trace-frame-function",
                            class: "shrink-0 font-semibold",
                            "{function}"
                        }
                    }
                    if on_file_path_click.is_some() {
                        button {
                            "data-slot": "stack-trace-frame-location",
                            r#type: "button",
                            class: "min-w-0 truncate text-left underline-offset-2 hover:underline",
                            onclick: {
                                let file = frame.file.clone();
                                move |_| {
                                    if let Some(handler) = on_file_path_click {
                                        handler.call(file.clone());
                                    }
                                }
                            },
                            {frame.location.clone()}
                        }
                    } else {
                        span {
                            "data-slot": "stack-trace-frame-location",
                            class: "min-w-0 truncate",
                            {frame.location.clone()}
                        }
                    }
                }
            }
        }
    }
}
