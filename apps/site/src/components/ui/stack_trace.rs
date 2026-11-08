//! Stack trace: a failed tool call and its stack frames, built from `Trace`. The error is the
//! title. Your own frames link to the file and line, and framework frames fold into a line of
//! their own.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceStatus, TraceTitle,
};

struct Frame<'a> {
    line: &'a str,
    name: &'a str,
    file: &'a str,
    line_number: &'a str,
    internal: bool,
}

/// Parses `at name (file:line:column)`. A line in another shape has no name or location.
fn parse_frame(line: &str) -> Frame<'_> {
    let located = line
        .strip_prefix("at ")
        .and_then(|rest| rest.strip_suffix(')'))
        .and_then(|rest| rest.split_once(" ("))
        .and_then(|(name, location)| {
            let (location, _column) = location.rsplit_once(':')?;
            let (file, line_number) = location.rsplit_once(':')?;
            Some((name, file, line_number))
        });
    let (name, file, line_number) = located.unwrap_or(("", "", ""));
    Frame {
        line,
        name,
        file,
        line_number,
        internal: line.contains("node_modules") || line.contains("node:"),
    }
}

#[component]
fn Icon(paths: &'static [&'static str]) -> Element {
    rsx! {
        TraceIcon {
            svg {
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                for d in paths {
                    path { d: *d }
                }
            }
        }
    }
}

/// An error and its stack, as one line of a `Trace`. `stack` is the first line of an error
/// stack followed by one frame per line.
#[component]
pub fn StackTrace(stack: String, #[props(default)] default_open: bool) -> Element {
    let mut lines = stack.lines().map(str::trim);
    let error = lines.next().unwrap_or_default();
    let frames: Vec<Frame> = lines.map(parse_frame).collect();
    let internal: Vec<&Frame> = frames.iter().filter(|frame| frame.internal).collect();

    rsx! {
        TraceCollapsible { default_open,
            TraceCollapsibleTrigger {
                TraceLine { status: TraceStatus::Error,
                    Icon { paths: &[
                        "M2 12a10 10 0 1 0 20 0 10 10 0 1 0-20 0",
                        "m15 9-6 6",
                        "m9 9 6 6",
                    ] }
                    TraceTitle { "Failed: {error}" }
                }
            }
            TraceCollapsibleContent { keep_mounted: true, class: "flex flex-col gap-2",
                p { class: "font-mono text-xs text-destructive", "{error}" }
                Trace {
                    for frame in frames.iter().filter(|frame| !frame.internal) {
                        a {
                            key: "{frame.line}",
                            href: "#{frame.file}:{frame.line_number}",
                            TraceLine {
                                Icon { paths: &[
                                    "M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z",
                                    "M14 2v5a1 1 0 0 0 1 1h5",
                                    "M10 12.5 8 15l2 2.5",
                                    "m14 12.5 2 2.5-2 2.5",
                                ] }
                                TraceTitle { "{frame.name} in {frame.file}:{frame.line_number}" }
                            }
                        }
                    }
                    TraceCollapsible {
                        TraceCollapsibleTrigger {
                            TraceLine {
                                Icon { paths: &[
                                    "M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z",
                                    "M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12",
                                    "M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17",
                                ] }
                                TraceTitle { "{internal.len()} framework frames" }
                            }
                        }
                        TraceCollapsibleContent {
                            ol { class: "flex flex-col gap-1 overflow-x-auto rounded-md bg-muted p-3 font-mono text-xs",
                                for frame in internal {
                                    li { key: "{frame.line}", class: "whitespace-nowrap", "{frame.line}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
