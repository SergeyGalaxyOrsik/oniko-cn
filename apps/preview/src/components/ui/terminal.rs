//! Terminal: displays command output in a monospace panel with a header, copy and clear actions, and a streaming indicator. ANSI escape sequences are removed, not rendered.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const COPY_SCRIPT: &str = "const text = await dioxus.recv(); try { await navigator.clipboard.writeText(text); dioxus.send(true); } catch (error) { dioxus.send(false); }";

#[derive(Clone, Copy)]
struct TerminalContext {
    output: Signal<String>,
    is_streaming: Signal<bool>,
    auto_scroll: Signal<bool>,
    on_clear: Signal<Option<EventHandler<()>>>,
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// Removes CSI, OSC and two-character escape sequences, keeping only the text.
fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\u{1b}' {
            out.push(c);
            continue;
        }
        match chars.peek().copied() {
            Some('[') => {
                chars.next();
                for next in chars.by_ref() {
                    if ('\u{40}'..='\u{7e}').contains(&next) {
                        break;
                    }
                }
            }
            Some(']') => {
                chars.next();
                while let Some(next) = chars.next() {
                    if next == '\u{7}' {
                        break;
                    }
                    if next == '\u{1b}' {
                        if chars.peek() == Some(&'\\') {
                            chars.next();
                        }
                        break;
                    }
                }
            }
            Some(_) => {
                for next in chars.by_ref() {
                    if !('\u{20}'..='\u{2f}').contains(&next) {
                        break;
                    }
                }
            }
            None => {}
        }
    }
    out
}

fn copy_icon(copied: bool) -> Element {
    rsx! {
        svg {
            width: "14",
            height: "14",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            if copied {
                path { d: "M5 12.5l4.5 4.5L19 7.5" }
            } else {
                path { d: "M9 9h11v11H9z" }
                path { d: "M5 15V5h10" }
            }
        }
    }
}

/// Root of the terminal. Pass `output` as plain text; escape sequences in it are stripped. Without children it renders the default header, output and actions.
#[component]
pub fn Terminal(
    output: String,
    #[props(default)] is_streaming: bool,
    #[props(default = true)] auto_scroll: bool,
    on_clear: Option<EventHandler<()>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col overflow-hidden rounded-lg border bg-muted text-foreground",
        take_class(&mut attributes).as_deref()
    );

    let output_signal = use_prop(strip_ansi(&output));
    let streaming = use_prop(is_streaming);
    let scroll = use_prop(auto_scroll);
    let clear = use_prop(on_clear);
    use_context_provider(|| TerminalContext {
        output: output_signal,
        is_streaming: streaming,
        auto_scroll: scroll,
        on_clear: clear,
    });

    let body = children.unwrap_or_else(|| {
        rsx! {
            TerminalHeader {
                TerminalTitle {}
                div {
                    "data-slot": "terminal-controls",
                    class: "flex items-center gap-1",
                    TerminalStatus {}
                    TerminalActions {
                        TerminalCopyButton {}
                        TerminalClearButton {}
                    }
                }
            }
            TerminalContent {}
        }
    });

    rsx! {
        div {
            "data-slot": "terminal",
            "data-streaming": is_streaming.then_some("true"),
            class: class,
            ..attributes,
            {body}
        }
    }
}

#[component]
pub fn TerminalHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center justify-between border-b px-4 py-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "terminal-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The title with a terminal icon. Pass children to replace the text "Terminal".
#[component]
pub fn TerminalTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex min-w-0 items-center gap-2 text-sm text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );
    let label = children.unwrap_or_else(|| rsx! { "Terminal" });

    rsx! {
        div {
            "data-slot": "terminal-title",
            class: class,
            ..attributes,
            svg {
                "data-slot": "terminal-title-icon",
                class: "shrink-0",
                width: "14",
                height: "14",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                path { d: "m4 17 6-6-6-6M12 19h8" }
            }
            {label}
        }
    }
}

/// A pulsing dot shown while `is_streaming` is set. Pass children to add a label. Renders nothing otherwise.
#[component]
pub fn TerminalStatus(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<TerminalContext>();
    if !(context.is_streaming)() {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "flex items-center gap-2 text-xs text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "terminal-status",
            class: class,
            ..attributes,
            span {
                "data-slot": "terminal-status-indicator",
                class: "inline-block size-2 animate-pulse rounded-full bg-primary",
                "aria-hidden": "true",
            }
            {children}
        }
    }
}

#[component]
pub fn TerminalActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center gap-1",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "terminal-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Copies the terminal output, without escape sequences, to the clipboard. Shows a check mark for two seconds after a successful copy.
#[component]
pub fn TerminalCopyButton(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<TerminalContext>();
    let mut attributes = attributes;
    let own_class = cn!(
        "shrink-0 text-muted-foreground hover:text-foreground",
        take_class(&mut attributes).as_deref()
    );
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        Some(own_class.as_str()),
    );
    let mut copied = use_signal(|| false);
    let label = if copied() { "Copied" } else { "Copy output" };
    let content = children.unwrap_or_else(|| copy_icon(copied()));

    rsx! {
        button {
            "data-slot": "terminal-copy-button",
            r#type: "button",
            aria_label: label,
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                let text = (context.output)();
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
            {content}
        }
    }
}

/// Clears the output through `on_clear` on `Terminal`. Renders nothing when `on_clear` is not set.
#[component]
pub fn TerminalClearButton(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<TerminalContext>();
    let Some(on_clear) = (context.on_clear)() else {
        return rsx! {};
    };

    let mut attributes = attributes;
    let own_class = cn!(
        "shrink-0 text-muted-foreground hover:text-foreground",
        take_class(&mut attributes).as_deref()
    );
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        Some(own_class.as_str()),
    );
    let content = children.unwrap_or_else(|| {
        rsx! {
            svg {
                width: "14",
                height: "14",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                path { d: "M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" }
            }
        }
    });

    rsx! {
        button {
            "data-slot": "terminal-clear-button",
            r#type: "button",
            aria_label: "Clear output",
            class: class,
            onclick: move |_| on_clear.call(()),
            ..attributes,
            {content}
        }
    }
}

/// The output itself. It scrolls to the bottom when the output changes, unless `auto_scroll` is off. While streaming, a blinking cursor follows the text.
#[component]
pub fn TerminalContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<TerminalContext>();
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let mut attributes = attributes;
    let class = cn!(
        "max-h-96 overflow-auto p-4 font-mono text-sm leading-relaxed",
        take_class(&mut attributes).as_deref()
    );

    use_effect(move || {
        let _ = (context.output)();
        if (context.auto_scroll)() {
            let _ = document::eval(&format!(
                "const el = document.getElementById('terminal-content-{id}'); if (el) el.scrollTop = el.scrollHeight;"
            ));
        }
    });

    let streaming = (context.is_streaming)();
    let text = (context.output)();
    let body = children.unwrap_or_else(|| {
        rsx! {
            pre {
                "data-slot": "terminal-output",
                class: "m-0 whitespace-pre-wrap break-words",
                "{text}"
                if streaming {
                    span {
                        "data-slot": "terminal-cursor",
                        class: "ml-0.5 inline-block h-4 w-2 animate-pulse bg-foreground align-middle",
                        "aria-hidden": "true",
                    }
                }
            }
        }
    });

    rsx! {
        div {
            "data-slot": "terminal-content",
            id: "terminal-content-{id}",
            class: class,
            ..attributes,
            {body}
        }
    }
}
