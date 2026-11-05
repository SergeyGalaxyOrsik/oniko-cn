use dioxus::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::chat::{Chat, ChatComposer, ChatMessages};
use crate::components::ui::textarea::Textarea;

#[component]
pub fn ChatDemo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Chat" }
            div { class: "h-96 overflow-hidden rounded-lg border",
                Chat {
                    ChatMessages {
                        div { class: "flex justify-end",
                            div { class: "max-w-[80%] rounded-lg bg-primary px-3 py-2 text-sm text-primary-foreground",
                                "Can you summarise the refund policy for annual plans?"
                            }
                        }
                        div { class: "flex justify-start",
                            div { class: "max-w-[80%] rounded-lg bg-muted px-3 py-2 text-sm",
                                "Annual plans can be refunded in full within 30 days of purchase. After that, the refund is prorated by the unused months."
                            }
                        }
                        div { class: "flex justify-end",
                            div { class: "max-w-[80%] rounded-lg bg-primary px-3 py-2 text-sm text-primary-foreground",
                                "Draft a reply to order 4821 using that."
                            }
                        }
                        div { class: "flex justify-start",
                            div { class: "max-w-[80%] rounded-lg bg-muted px-3 py-2 text-sm",
                                "Order 4821 is 19 days old, so it is still inside the full refund window."
                            }
                        }
                    }
                    ChatComposer {
                        div { class: "flex items-end gap-2",
                            Textarea { placeholder: "Ask a follow-up", rows: "2", class: "min-h-0" }
                            Button { "Send" }
                        }
                    }
                }
            }
        }
    }
}
