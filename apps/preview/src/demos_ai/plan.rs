use dioxus::prelude::*;

use crate::components::ui::plan::{
    Plan, PlanAction, PlanContent, PlanDescription, PlanFooter, PlanHeader, PlanTitle, PlanTrigger,
};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-col gap-4 rounded-lg border p-6";

#[component]
pub fn PlanDemo() -> Element {
    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Plan" }
            div { class: BOX,
                Plan { is_streaming: true, default_open: true,
                    PlanHeader {
                        PlanTitle { "Migrate the session store" }
                        PlanDescription { "Drafting the steps for moving sessions from cookies to Redis." }
                        PlanAction {
                            PlanTrigger {}
                        }
                    }
                    PlanContent {
                        ol { class: "list-decimal space-y-2 pl-5 text-sm",
                            li { "Add a Redis client behind the existing session trait." }
                            li { "Write sessions on login and read them on each request." }
                            li { "Backfill active sessions during a rolling deploy." }
                        }
                    }
                    PlanFooter {
                        span { class: "text-xs text-muted-foreground", "Drafting 2 of 4 steps" }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Plan (finished, collapsed by default)" }
            div { class: BOX,
                Plan {
                    PlanHeader {
                        PlanTitle { "Upgrade the build pipeline" }
                        PlanDescription { "Four steps to move the CI jobs to the new runners." }
                        PlanAction {
                            PlanTrigger {}
                        }
                    }
                    PlanContent {
                        ol { class: "list-decimal space-y-2 pl-5 text-sm",
                            li { "Pin the toolchain version in rust-toolchain.toml." }
                            li { "Cache the cargo registry between jobs." }
                            li { "Run the integration suite on both runners." }
                            li { "Remove the old self-hosted agents." }
                        }
                    }
                    PlanFooter {
                        span { class: "text-xs text-muted-foreground", "4 of 4 steps done" }
                    }
                }
            }
        }
    }
}
