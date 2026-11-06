//! Inline Citation examples, ported from shadcn/ui `examples/base/inline-citation-demo.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};

#[derive(Clone, Copy, PartialEq)]
struct Source {
    url: &'static str,
    title: &'static str,
    description: &'static str,
}

const STREAMING: Source = Source {
    url: "https://nextjs.org/docs/app/getting-started/linking-and-navigating#streaming",
    title: "Linking and Navigating",
    description: "How the App Router streams a route to the browser.",
};
const SUSPENSE: Source = Source {
    url: "https://react.dev/reference/react/Suspense",
    title: "<Suspense>",
    description: "Display a fallback until its children have finished loading.",
};
const SERVER_COMPONENTS: Source = Source {
    url: "https://react.dev/reference/rsc/server-components",
    title: "Server Components",
    description: "Components that render ahead of time, on the server.",
};
const LOADING: Source = Source {
    url: "https://nextjs.org/docs/app/api-reference/file-conventions/loading",
    title: "loading.js",
    description: "Show an instant loading state for a route segment.",
};
const CHUNKED: Source = Source {
    url: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Transfer-Encoding",
    title: "Transfer-Encoding",
    description: "Send a response as a series of chunks.",
};

fn hostname(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    let host = rest.split(['/', '?', '#']).next().unwrap_or(rest);
    host.strip_prefix("www.").unwrap_or(host)
}

fn as_button((attributes, children): (Vec<Attribute>, Element)) -> Element {
    rsx! {
        button { r#type: "button", ..attributes, {children} }
    }
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        InlineCitationDemo {}
    }
}

#[component]
fn Citation(source: Source) -> Element {
    let host = hostname(source.url);

    rsx! {
        HoverCard {
            HoverCardTrigger {
                delay: 10,
                close_delay: 100,
                render: as_button,
                class: button_variants(ButtonVariant::Secondary, ButtonSize::Xs, None),
                "{host}"
            }
            HoverCardContent { class: "flex w-64 flex-col gap-1",
                div { class: "font-medium", "{source.title}" }
                p { class: "text-muted-foreground", "{source.description}" }
                a {
                    href: "{source.url}",
                    target: "_blank",
                    rel: "noreferrer",
                    class: "text-xs text-muted-foreground underline-offset-4 hover:underline",
                    "{host}"
                    span { class: "sr-only", " (opens in a new tab)" }
                }
            }
        }
    }
}

#[component]
fn InlineCitationDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "mx-auto flex max-w-sm flex-col gap-3 py-6 text-sm leading-relaxed",
                p {
                    "Next.js streams a route in chunks, so the page shows up before every request finishes. "
                    Citation { source: STREAMING }
                }
                p {
                    "Wrap a slow part of the page in a Suspense boundary and React sends the fallback first. When the data resolves, the finished HTML streams into the same response and replaces the fallback, with no extra request from the browser. "
                    Citation { source: SUSPENSE }
                    " Server Components help here, because their code never ships to the browser. "
                    Citation { source: SERVER_COMPONENTS }
                }
                p {
                    "A loading.tsx file does the same for a whole route segment. "
                    Citation { source: LOADING }
                    " It all runs over chunked transfer encoding, so there is nothing extra to set up. "
                    Citation { source: CHUNKED }
                }
            }
        }
    }
}
