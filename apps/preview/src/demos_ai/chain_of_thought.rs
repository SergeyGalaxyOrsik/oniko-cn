use dioxus::prelude::*;

use crate::components::ui::chain_of_thought::{
    ChainOfThought, ChainOfThoughtContent, ChainOfThoughtHeader, ChainOfThoughtImage,
    ChainOfThoughtSearchResult, ChainOfThoughtSearchResults, ChainOfThoughtStep,
    ChainOfThoughtStepStatus,
};

#[component]
pub fn ChainOfThoughtDemo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Chain of Thought" }
            div { class: "flex flex-col gap-6 rounded-lg border p-6",
                ChainOfThought {
                    ChainOfThoughtHeader {}
                    ChainOfThoughtContent {
                        ChainOfThoughtStep {
                            status: ChainOfThoughtStepStatus::Complete,
                            label: rsx! { "Read the refund policy" },
                            description: rsx! { "Policy v3, last updated in March." },
                        }
                        ChainOfThoughtStep {
                            status: ChainOfThoughtStepStatus::Complete,
                            label: rsx! { "Searched the help center" },
                            description: rsx! {
                                ChainOfThoughtSearchResults {
                                    ChainOfThoughtSearchResult { "refunds.md" }
                                    ChainOfThoughtSearchResult { "billing-faq" }
                                    ChainOfThoughtSearchResult { "annual-plans" }
                                }
                            },
                        }
                        ChainOfThoughtStep {
                            status: ChainOfThoughtStepStatus::Active,
                            label: rsx! { "Checking the order date" },
                            description: rsx! { "Order 4821 was placed 19 days ago." },
                            icon: rsx! {
                                span { class: "size-2 animate-pulse rounded-full bg-primary" }
                            },
                        }
                        ChainOfThoughtStep {
                            status: ChainOfThoughtStepStatus::Pending,
                            label: rsx! { "Draft the reply to the customer" },
                        }
                        ChainOfThoughtImage { caption: rsx! { "Order 4821 as shown in the admin panel" },
                            div { class: "h-20 w-full max-w-xs rounded bg-background" }
                        }
                    }
                }
            }
            div { class: "flex flex-col gap-6 rounded-lg border p-6",
                ChainOfThought { default_open: false,
                    ChainOfThoughtHeader { "Reasoning about the refund" }
                    ChainOfThoughtContent {
                        ChainOfThoughtStep {
                            status: ChainOfThoughtStepStatus::Complete,
                            label: rsx! { "The order is inside the 30-day window" },
                        }
                    }
                }
            }
        }
    }
}
