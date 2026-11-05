use dioxus::prelude::*;

use crate::components::ui::context::{
    Context, ContextCacheUsage, ContextContent, ContextContentBody, ContextContentFooter,
    ContextContentHeader, ContextInputUsage, ContextOutputUsage, ContextReasoningUsage,
    ContextTrigger, ContextUsage,
};

#[component]
pub fn ContextDemo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Context" }
            div { class: "flex flex-col gap-2 rounded-lg border p-6",
                p { class: "text-xs text-muted-foreground", "Hover the trigger to open the usage card." }
                div { class: "flex flex-wrap items-center gap-3",
                    Context {
                        used_tokens: 42_000,
                        max_tokens: 200_000,
                        usage: Some(ContextUsage {
                            input_tokens: 31_200,
                            output_tokens: 6_400,
                            reasoning_tokens: 3_100,
                            cache_tokens: 1_300,
                            input_cost: Some(0.094),
                            output_cost: Some(0.096),
                            reasoning_cost: Some(0.047),
                            cache_cost: Some(0.004),
                        }),
                        ContextTrigger {}
                        ContextContent {
                            ContextContentHeader {}
                            ContextContentBody {
                                ContextInputUsage {}
                                ContextOutputUsage {}
                                ContextReasoningUsage {}
                                ContextCacheUsage {}
                            }
                            ContextContentFooter {}
                        }
                    }
                    Context {
                        used_tokens: 150_000,
                        max_tokens: 200_000,
                        usage: Some(ContextUsage {
                            input_tokens: 2_400_000,
                            output_tokens: 180_000,
                            ..Default::default()
                        }),
                        ContextTrigger { "150K of 200K" }
                        ContextContent {
                            ContextContentHeader { "Long session, compacting soon" }
                            ContextContentBody {
                                ContextInputUsage { "Input: 2.4M tokens" }
                                ContextOutputUsage { "Output: 180K tokens" }
                            }
                            ContextContentFooter { "Estimated cost unavailable" }
                        }
                    }
                    Context {
                        used_tokens: 0,
                        max_tokens: 0,
                        usage: None,
                        ContextTrigger {}
                        ContextContent {
                            ContextContentHeader {}
                            ContextContentBody {
                                ContextInputUsage {}
                                ContextCacheUsage {}
                            }
                            ContextContentFooter {}
                        }
                    }
                }
            }
        }
    }
}
