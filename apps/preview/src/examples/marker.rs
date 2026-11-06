//! Marker examples, ported from shadcn/ui `examples/base/marker-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{BookOpenCheck, FileText, GitBranch, RotateCcw, Search};

use super::Example;
use crate::components::ui::marker::{
    Marker, MarkerContent, MarkerIcon, MarkerVariant, marker_variants,
};
use crate::components::ui::spinner::Spinner;
use crate::components::ui::toast::{ToastOptions, ToastProvider, Toaster, toast};

const PREVIEW: &str = "h-auto theme-blue";

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            MarkerDemo {}
            MarkerVariantsDemo {}
            MarkerStatusDemo {}
            MarkerShimmerDemo {}
            MarkerSeparatorDemo {}
            MarkerBorderDemo {}
            MarkerIconDemo {}
            MarkerLinkButtonDemo {}
        }
    }
}

#[component]
fn MarkerDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: PREVIEW,
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Marker {
                    MarkerIcon {
                        GitBranch {}
                    }
                    MarkerContent { "Switched to a new branch" }
                }
                Marker { role: "status",
                    MarkerIcon {
                        Spinner {}
                    }
                    MarkerContent { class: "shimmer", "Thinking..." }
                }
                Marker { variant: MarkerVariant::Separator,
                    MarkerContent { "Conversation compacted" }
                }
                Marker {
                    MarkerIcon {
                        Search {}
                    }
                    MarkerContent { "Explored 4 files" }
                }
            }
        }
    }
}

#[component]
fn MarkerVariantsDemo() -> Element {
    rsx! {
        Example { title: "Variants", class: PREVIEW,
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Marker {
                    MarkerContent { "A default marker for inline notes." }
                }
                Marker { variant: MarkerVariant::Separator,
                    MarkerContent { "A separator marker" }
                }
                Marker { variant: MarkerVariant::Border,
                    MarkerContent { "A border marker for row boundaries." }
                }
            }
        }
    }
}

#[component]
fn MarkerStatusDemo() -> Element {
    rsx! {
        Example { title: "Status", class: PREVIEW,
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Marker { role: "status",
                    MarkerIcon {
                        Spinner {}
                    }
                    MarkerContent { "Compacting conversation" }
                }
                Marker { variant: MarkerVariant::Separator, role: "status",
                    MarkerIcon {
                        Spinner {}
                    }
                    MarkerContent { "Running tests" }
                }
            }
        }
    }
}

#[component]
fn MarkerShimmerDemo() -> Element {
    rsx! {
        Example { title: "Shimmer", class: PREVIEW,
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Marker { role: "status",
                    MarkerContent { class: "shimmer", "Thinking..." }
                }
                Marker { variant: MarkerVariant::Separator, role: "status",
                    MarkerContent { class: "shimmer", "Reading 4 files" }
                }
            }
        }
    }
}

#[component]
fn MarkerSeparatorDemo() -> Element {
    rsx! {
        Example { title: "Separator", class: PREVIEW,
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Marker { variant: MarkerVariant::Separator,
                    MarkerContent { "Today" }
                }
                Marker { variant: MarkerVariant::Separator,
                    MarkerContent { "Worked for 42s" }
                }
                Marker { variant: MarkerVariant::Separator,
                    MarkerContent { "Conversation compacted" }
                }
            }
        }
    }
}

#[component]
fn MarkerBorderDemo() -> Element {
    rsx! {
        Example { title: "Border", class: PREVIEW,
            div { class: "flex w-full max-w-sm flex-col gap-3 py-12",
                Marker { variant: MarkerVariant::Border,
                    MarkerIcon {
                        GitBranch {}
                    }
                    MarkerContent { "Switched to release-candidate" }
                }
                Marker { variant: MarkerVariant::Border,
                    MarkerIcon {
                        Search {}
                    }
                    MarkerContent { "Reviewed 8 related files" }
                }
                Marker { variant: MarkerVariant::Border,
                    MarkerIcon {
                        FileText {}
                    }
                    MarkerContent { "Opened implementation notes" }
                }
            }
        }
    }
}

#[component]
fn MarkerIconDemo() -> Element {
    rsx! {
        Example { title: "Icon", class: PREVIEW,
            div { class: "flex w-full max-w-sm flex-col gap-12 py-12",
                Marker {
                    MarkerIcon {
                        GitBranch {}
                    }
                    MarkerContent { "Switched to a new branch" }
                }
                Marker { variant: MarkerVariant::Separator,
                    MarkerIcon {
                        Search {}
                    }
                    MarkerContent { "Explored 4 files" }
                }
                Marker { class: "flex-col",
                    MarkerIcon {
                        BookOpenCheck {}
                    }
                    MarkerContent { "Syncing completed" }
                }
            }
        }
    }
}

#[component]
fn MarkerLinkButtonDemo() -> Element {
    rsx! {
        Example { title: "Link / Button", class: PREVIEW,
            ToastProvider {
                LinkButtonMarkers {}
                Toaster {}
            }
        }
    }
}

#[component]
fn LinkButtonMarkers() -> Element {
    let toast = toast();

    rsx! {
        div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
            a {
                "data-slot": "marker",
                "data-variant": MarkerVariant::Default.as_str(),
                href: "#links-and-buttons",
                class: marker_variants(MarkerVariant::Default, None),
                MarkerIcon {
                    GitBranch {}
                }
                MarkerContent { "View the pull request" }
            }
            button {
                "data-slot": "marker",
                "data-variant": MarkerVariant::Default.as_str(),
                r#type: "button",
                class: marker_variants(MarkerVariant::Default, Some("transition-colors hover:text-foreground")),
                onclick: move |_| {
                    toast.add(ToastOptions::new("You clicked the revert button"));
                },
                MarkerIcon {
                    RotateCcw {}
                }
                MarkerContent { "Revert this change" }
            }
        }
    }
}
