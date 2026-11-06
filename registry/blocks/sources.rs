//! Sources: a line that says how many links the agent used for a reply, like "Used 3 sources", and opens
//! into a link for each one. Built with `Trace`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceIcon,
    TraceLine, TraceTitle,
};

#[derive(Clone, PartialEq)]
pub struct Source {
    pub url: String,
    /// Falls back to the hostname of `url`.
    pub title: Option<String>,
}

fn hostname(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    rest.split(['/', '?', '#']).next().unwrap_or(rest)
}

fn icon(paths: Element) -> Element {
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
                {paths}
            }
        }
    }
}

/// The links of one reply. Drop repeated URLs before passing them in.
#[component]
pub fn Sources(sources: Vec<Source>, #[props(default)] default_open: bool) -> Element {
    rsx! {
        Trace {
            TraceCollapsible { default_open,
                TraceCollapsibleTrigger {
                    TraceLine {
                        {icon(rsx! {
                            circle { cx: "12", cy: "12", r: "10" }
                            path { d: "M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" }
                            path { d: "M2 12h20" }
                        })}
                        TraceTitle { "Used {sources.len()} sources" }
                    }
                }
                TraceCollapsibleContent {
                    Trace {
                        for source in sources.iter() {
                            a {
                                key: "{source.url}",
                                href: "{source.url}",
                                target: "_blank",
                                rel: "noreferrer",
                                TraceLine {
                                    TraceTitle {
                                        "{source.title.as_deref().unwrap_or_else(|| hostname(&source.url))}"
                                        span { class: "sr-only", " (opens in a new tab)" }
                                    }
                                    {icon(rsx! {
                                        path { d: "M7 7h10v10" }
                                        path { d: "M7 17 17 7" }
                                    })}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
