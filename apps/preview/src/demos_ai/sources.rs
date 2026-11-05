use dioxus::prelude::*;

use crate::components::ui::sources::{Source, Sources, SourcesContent, SourcesTrigger};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-col gap-4 rounded-lg border p-6";

#[component]
pub fn SourcesDemo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Sources" }
            div { class: BOX,
                Sources {
                    SourcesTrigger { count: 3 }
                    SourcesContent {
                        Source {
                            href: "https://dioxuslabs.com/learn/0.7/".to_string(),
                            title: Some("Dioxus 0.7 guide".to_string()),
                        }
                        Source {
                            href: "https://docs.rs/dioxus/latest/dioxus/".to_string(),
                            title: Some("dioxus API docs".to_string()),
                        }
                        Source {
                            href: "https://github.com/DioxusLabs/dioxus".to_string(),
                            title: Some("DioxusLabs/dioxus".to_string()),
                        }
                    }
                }
                Sources { default_open: true,
                    SourcesTrigger { count: 1 }
                    SourcesContent {
                        Source {
                            href: "https://developer.mozilla.org/docs/Web/API/Clipboard".to_string(),
                            title: Some("Clipboard API".to_string()),
                        }
                    }
                }
                Sources {
                    open: Some(open()),
                    on_open_change: move |next: bool| open.set(next),
                    SourcesTrigger {
                        "Read one page"
                    }
                    SourcesContent {
                        Source {
                            href: "https://dioxuslabs.com/learn/0.7/essentials/rsx".to_string(),
                            span { "RSX essentials" }
                        }
                    }
                }
            }
        }
    }
}
