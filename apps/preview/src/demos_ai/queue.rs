use dioxus::prelude::*;

use crate::components::ui::queue::{
    Queue, QueueItem, QueueItemAction, QueueItemActions, QueueItemAttachment, QueueItemContent,
    QueueItemDescription, QueueItemFile, QueueItemImage, QueueItemIndicator, QueueList,
    QueueSection, QueueSectionContent, QueueSectionLabel, QueueSectionTrigger,
};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const IMAGE: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'><rect width='32' height='32' fill='%23e4e4e7'/><circle cx='16' cy='13' r='6' fill='%2371717a'/><path d='M4 32c2-7 8-10 12-10s10 3 12 10z' fill='%2371717a'/></svg>";

#[component]
pub fn QueueDemo() -> Element {
    let mut queued_open = use_signal(|| true);

    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Queue" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                Queue {
                    QueueSection {
                        open: Some(queued_open()),
                        on_open_change: move |next: bool| queued_open.set(next),
                        QueueSectionTrigger {
                            QueueSectionLabel {
                                count: Some(2),
                                label: "Queued".to_string(),
                                icon: None,
                            }
                        }
                        QueueSectionContent {
                            QueueList {
                                QueueItem {
                                    div { class: "flex items-center gap-2",
                                        QueueItemIndicator {}
                                        QueueItemContent { "Fix the flaky login test" }
                                        QueueItemActions {
                                            QueueItemAction { "aria-label": "Remove from queue",
                                                svg {
                                                    class: "size-3.5",
                                                    view_box: "0 0 24 24",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_width: "2",
                                                    stroke_linecap: "round",
                                                    "aria-hidden": "true",
                                                    path { d: "M18 6 6 18" }
                                                    path { d: "m6 6 12 12" }
                                                }
                                            }
                                        }
                                    }
                                    QueueItemDescription { "Retries on the CI runner" }
                                }
                                QueueItem {
                                    div { class: "flex items-center gap-2",
                                        QueueItemIndicator {}
                                        QueueItemContent { "Draft the changelog for v0.4" }
                                    }
                                    QueueItemDescription { "Uses the notes from the last sprint" }
                                    QueueItemAttachment {
                                        QueueItemImage { src: IMAGE }
                                        QueueItemFile { "release-notes.md" }
                                    }
                                }
                            }
                        }
                    }
                    QueueSection {
                        default_open: false,
                        QueueSectionTrigger {
                            QueueSectionLabel {
                                count: None,
                                label: "Completed".to_string(),
                                icon: None,
                            }
                        }
                        QueueSectionContent {
                            QueueList {
                                QueueItem {
                                    div { class: "flex items-center gap-2",
                                        QueueItemIndicator { completed: true }
                                        QueueItemContent { completed: true, "Bump the Dioxus version" }
                                    }
                                    QueueItemDescription { completed: true, "Merged in #214" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
