//! Attachment examples, ported from shadcn/ui `examples/base/attachment-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    Check, Clock, Copy, FileCode, FileExclamationPoint, FileSearch, FileText, RefreshCw, Table, X,
};

use super::Example;
use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription,
    AttachmentGroup, AttachmentMedia, AttachmentMediaVariant, AttachmentOrientation,
    AttachmentSize, AttachmentState, AttachmentTitle, AttachmentTrigger,
};
use crate::components::ui::dialog::{
    Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle,
};
use crate::components::ui::spinner::Spinner;

const PREVIEW_CLASS: &str = "h-auto theme-blue bg-surface dark:bg-background";

struct Image {
    name: &'static str,
    meta: &'static str,
    src: &'static str,
    alt: &'static str,
}

const IMAGES: [Image; 3] = [
    Image {
        name: "workspace.png",
        meta: "PNG · 820 KB",
        src: "https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80",
        alt: "Workspace",
    },
    Image {
        name: "desk-reference.jpg",
        meta: "JPG · 1.1 MB",
        src: "https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80",
        alt: "Desk",
    },
    Image {
        name: "office-reference.jpg",
        meta: "JPG · 940 KB",
        src: "https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80",
        alt: "Office",
    },
];

#[component]
pub fn Examples() -> Element {
    rsx! {
        AttachmentDemo {}
        AttachmentImage {}
        AttachmentStates {}
        AttachmentSizes {}
        AttachmentGroupDemo {}
        AttachmentTriggerDemo {}
    }
}

#[component]
fn AttachmentDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: PREVIEW_CLASS,
            div { class: "mx-auto flex w-full max-w-sm flex-col gap-3 py-12",
                AttachmentGroup {
                    for image in IMAGES {
                        Attachment {
                            key: "{image.name}",
                            orientation: AttachmentOrientation::Vertical,
                            AttachmentMedia { variant: AttachmentMediaVariant::Image,
                                img { src: image.src, alt: image.alt }
                            }
                            AttachmentContent {
                                AttachmentTitle { "{image.name}" }
                                AttachmentDescription { "{image.meta}" }
                            }
                        }
                    }
                }
                Attachment { state: AttachmentState::Uploading, class: "w-full",
                    AttachmentMedia {
                        Spinner {}
                    }
                    AttachmentContent {
                        AttachmentTitle { "sales-dashboard.pdf" }
                        AttachmentDescription { "Uploading · 64%" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Cancel upload",
                            X { size: 16 }
                        }
                    }
                }
                Attachment { class: "w-full",
                    AttachmentMedia {
                        FileCode { size: 16 }
                    }
                    AttachmentContent {
                        AttachmentTitle { "message-renderer.tsx" }
                        AttachmentDescription { "TypeScript · 12 KB" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove message-renderer.tsx",
                            X { size: 16 }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttachmentImage() -> Element {
    rsx! {
        Example { title: "Image", class: PREVIEW_CLASS,
            div { class: "mx-auto w-full max-w-sm py-12",
                AttachmentGroup { class: "w-full",
                    for image in IMAGES {
                        Attachment {
                            key: "{image.name}",
                            orientation: AttachmentOrientation::Vertical,
                            AttachmentMedia { variant: AttachmentMediaVariant::Image,
                                img { src: image.src, alt: image.alt }
                            }
                            AttachmentContent {
                                AttachmentTitle { "{image.name}" }
                                AttachmentDescription { "{image.meta}" }
                            }
                            AttachmentActions {
                                AttachmentAction { aria_label: "Remove {image.name}",
                                    X { size: 16 }
                                }
                            }
                            AttachmentTrigger {
                                href: image.src,
                                target: "_blank",
                                rel: "noreferrer",
                                aria_label: "Open {image.name}",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttachmentStates() -> Element {
    rsx! {
        Example { title: "States", class: PREVIEW_CLASS,
            div { class: "mx-auto flex w-full max-w-sm flex-col gap-2 py-12",
                Attachment { state: AttachmentState::Idle, class: "w-full",
                    AttachmentMedia {
                        Clock { size: 16 }
                    }
                    AttachmentContent {
                        AttachmentTitle { "selected-file.pdf" }
                        AttachmentDescription { "Ready to upload" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove selected-file.pdf",
                            X { size: 16 }
                        }
                    }
                }
                Attachment { state: AttachmentState::Uploading, class: "w-full",
                    AttachmentMedia {
                        Spinner {}
                    }
                    AttachmentContent {
                        AttachmentTitle { "design-system.zip" }
                        AttachmentDescription { "Uploading · 64%" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Cancel upload",
                            X { size: 16 }
                        }
                    }
                }
                Attachment { state: AttachmentState::Processing, class: "w-full",
                    AttachmentMedia {
                        FileText { size: 16 }
                    }
                    AttachmentContent {
                        AttachmentTitle { "market-research.pdf" }
                        AttachmentDescription { "Processing document" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove market-research.pdf",
                            X { size: 16 }
                        }
                    }
                }
                Attachment { state: AttachmentState::Error, class: "w-full",
                    AttachmentMedia {
                        FileExclamationPoint { size: 16 }
                    }
                    AttachmentContent {
                        AttachmentTitle { "financial-model.xlsx" }
                        AttachmentDescription { "Upload failed. Try again." }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Retry upload",
                            RefreshCw { size: 16 }
                        }
                        AttachmentAction { aria_label: "Remove financial-model.xlsx",
                            X { size: 16 }
                        }
                    }
                }
                Attachment { state: AttachmentState::Done, class: "w-full",
                    AttachmentMedia {
                        Check { size: 16 }
                    }
                    AttachmentContent {
                        AttachmentTitle { "uploaded-report.pdf" }
                        AttachmentDescription { "Uploaded · 1.8 MB" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove uploaded-report.pdf",
                            X { size: 16 }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttachmentSizes() -> Element {
    rsx! {
        Example { title: "Sizes", class: PREVIEW_CLASS,
            div { class: "mx-auto flex w-full max-w-sm flex-col gap-3 py-12",
                Attachment { size: AttachmentSize::Default, class: "w-full",
                    AttachmentMedia {
                        FileText { size: 16 }
                    }
                    AttachmentContent {
                        AttachmentTitle { "Default attachment" }
                        AttachmentDescription { "PDF · 2.4 MB" }
                    }
                }
                Attachment { size: AttachmentSize::Sm, class: "w-full",
                    AttachmentMedia {
                        FileText { size: 16 }
                    }
                    AttachmentContent {
                        AttachmentTitle { "Small attachment" }
                        AttachmentDescription { "PDF · 2.4 MB" }
                    }
                }
                Attachment { size: AttachmentSize::Xs, class: "w-full",
                    AttachmentMedia {
                        FileText { size: 16 }
                    }
                    AttachmentContent {
                        AttachmentTitle { "Extra small attachment" }
                    }
                }
            }
        }
    }
}

enum GroupMedia {
    Icon(fn() -> Element),
    Image(&'static str),
}

struct GroupItem {
    name: &'static str,
    meta: &'static str,
    media: GroupMedia,
}

fn file_text_icon() -> Element {
    rsx! {
        FileText { size: 16 }
    }
}

fn table_icon() -> Element {
    rsx! {
        Table { size: 16 }
    }
}

fn file_code_icon() -> Element {
    rsx! {
        FileCode { size: 16 }
    }
}

#[component]
fn AttachmentGroupDemo() -> Element {
    let items = [
        GroupItem {
            name: "briefing-notes.pdf",
            meta: "PDF · 1.4 MB",
            media: GroupMedia::Icon(file_text_icon),
        },
        GroupItem {
            name: "workspace.png",
            meta: "PNG · 820 KB",
            media: GroupMedia::Image(
                "https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80",
            ),
        },
        GroupItem {
            name: "customers.csv",
            meta: "CSV · 18 KB",
            media: GroupMedia::Icon(table_icon),
        },
        GroupItem {
            name: "renderer.tsx",
            meta: "TSX · 12 KB",
            media: GroupMedia::Icon(file_code_icon),
        },
    ];

    rsx! {
        Example { title: "Group", class: PREVIEW_CLASS,
            div { class: "mx-auto w-full max-w-sm py-12",
                AttachmentGroup { class: "w-full",
                    for item in items {
                        Attachment { key: "{item.name}", class: "w-64",
                            match item.media {
                                GroupMedia::Image(src) => rsx! {
                                    AttachmentMedia { variant: AttachmentMediaVariant::Image,
                                        img { src: src, alt: item.name }
                                    }
                                },
                                GroupMedia::Icon(icon) => rsx! {
                                    AttachmentMedia {
                                        {icon()}
                                    }
                                },
                            }
                            AttachmentContent {
                                AttachmentTitle { "{item.name}" }
                                AttachmentDescription { "{item.meta}" }
                            }
                            AttachmentActions {
                                AttachmentAction { aria_label: "Remove {item.name}",
                                    X { size: 16 }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttachmentTriggerDemo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Example { title: "Trigger", class: PREVIEW_CLASS,
            div { class: "mx-auto w-full max-w-sm py-12",
                Dialog {
                    open: open(),
                    on_open_change: move |next| open.set(next),
                    Attachment { class: "w-full",
                        AttachmentMedia {
                            FileSearch { size: 16 }
                        }
                        AttachmentContent {
                            AttachmentTitle { "research-summary.pdf" }
                            AttachmentDescription { "Open preview dialog" }
                        }
                        AttachmentActions {
                            AttachmentAction { aria_label: "Copy link",
                                Copy { size: 16 }
                            }
                            AttachmentAction { aria_label: "Remove research-summary.pdf",
                                X { size: 16 }
                            }
                        }
                        AttachmentTrigger {
                            aria_label: "Preview research-summary.pdf",
                            onclick: move |_| open.set(true),
                        }
                    }
                    DialogContent { class: "sm:max-w-md",
                        DialogHeader {
                            DialogTitle { "research-summary.pdf" }
                            DialogDescription {
                                "The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it."
                            }
                        }
                    }
                }
            }
        }
    }
}
