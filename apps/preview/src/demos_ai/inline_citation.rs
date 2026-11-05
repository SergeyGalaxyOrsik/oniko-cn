use dioxus::prelude::*;

use crate::components::ui::inline_citation::{
    InlineCitation, InlineCitationCard, InlineCitationCardBody, InlineCitationCardTrigger,
    InlineCitationCarousel, InlineCitationCarouselContent, InlineCitationCarouselHeader,
    InlineCitationCarouselIndex, InlineCitationCarouselItem, InlineCitationCarouselNext,
    InlineCitationCarouselPrev, InlineCitationQuote, InlineCitationSource, InlineCitationText,
};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-col gap-4 rounded-lg border p-6";

#[component]
pub fn InlineCitationDemo() -> Element {
    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Inline Citation" }
            div { class: BOX,
                p { class: "text-sm leading-relaxed",
                    "Rust 2024 stabilized let chains in the 1.88 release"
                    InlineCitation {
                        InlineCitationText { " for `if let` conditions" }
                        InlineCitationCard {
                            InlineCitationCardTrigger {
                                sources: vec!["https://blog.rust-lang.org/2025/06/26/rust-1.88.0/".to_string()],
                            }
                            InlineCitationCardBody {
                                InlineCitationSource {
                                    title: "Announcing Rust 1.88.0",
                                    url: "https://blog.rust-lang.org/2025/06/26/rust-1.88.0/",
                                    description: "Let chains are now stable in the 2024 edition, so conditions can combine `if let` patterns with boolean checks.",
                                }
                            }
                        }
                    }
                    "."
                }

                p { class: "text-sm leading-relaxed",
                    "Most teams now run their CI on ephemeral runners"
                    InlineCitation {
                        InlineCitationText { " with cached dependency layers" }
                        InlineCitationCard {
                            InlineCitationCardTrigger {
                                sources: vec![
                                    "https://docs.github.com/actions/using-workflows/caching-dependencies".to_string(),
                                    "https://depot.dev/docs/cache".to_string(),
                                    "https://buildkite.com/docs/pipelines/caching".to_string(),
                                ],
                            }
                            InlineCitationCardBody {
                                InlineCitationCarousel {
                                    InlineCitationCarouselHeader {
                                        InlineCitationCarouselPrev {}
                                        InlineCitationCarouselIndex {}
                                        InlineCitationCarouselNext {}
                                    }
                                    InlineCitationCarouselContent {
                                        InlineCitationCarouselItem {
                                            InlineCitationSource {
                                                title: "Caching dependencies to speed up workflows",
                                                url: "https://docs.github.com/actions/using-workflows/caching-dependencies",
                                                description: "Cache dependency paths between runs and restore them by a key built from the lock file.",
                                            }
                                            InlineCitationQuote {
                                                "Use a key that changes only when the lock file changes."
                                            }
                                        }
                                        InlineCitationCarouselItem {
                                            InlineCitationSource {
                                                title: "Remote build cache",
                                                url: "https://depot.dev/docs/cache",
                                                description: "Layer caches live outside the runner, so a fresh machine starts with warm layers.",
                                            }
                                        }
                                        InlineCitationCarouselItem {
                                            InlineCitationSource {
                                                title: "Pipeline caching",
                                                url: "https://buildkite.com/docs/pipelines/caching",
                                                description: "Agents share a cache bucket keyed by branch and dependency checksum.",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "."
                }

                p { class: "text-sm leading-relaxed",
                    "The vendor reports an uptime of 99.99 percent"
                    InlineCitation {
                        InlineCitationText { " over the last quarter" }
                        InlineCitationCard {
                            InlineCitationCardTrigger { sources: Vec::<String>::new() }
                            InlineCitationCardBody {
                                InlineCitationSource {
                                    title: "Status history",
                                    description: "The source did not record its URL.",
                                }
                            }
                        }
                    }
                    "."
                }
            }
        }
    }
}
