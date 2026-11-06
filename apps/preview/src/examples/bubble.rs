//! Bubble examples, ported from shadcn/ui `examples/base/bubble-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{Check, ChevronDown, Info};

use super::Example;
use crate::components::ui::bubble::{
    Bubble, BubbleAlign, BubbleContent, BubbleContentButton, BubbleGroup, BubbleReactions,
    BubbleReactionsSide, BubbleVariant,
};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::collapsible::{Collapsible, CollapsibleTrigger};
use crate::components::ui::popover::{
    Popover, PopoverContent, PopoverDescription, PopoverHeader, PopoverTitle, PopoverTrigger,
};
use crate::components::ui::toast::{ToastOptions, ToastProvider, ToastType, Toaster, toast};
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

#[component]
pub fn Examples() -> Element {
    rsx! {
        BubbleDemo {}
        BubbleVariantsDemo {}
        BubbleAlignmentDemo {}
        BubbleGroupDemo {}
        BubbleLinkButtonDemo {}
        BubbleReactionsDemo {}
        BubbleCollapsibleDemo {}
        BubbleTooltipDemo {}
        BubblePopoverDemo {}
    }
}

#[component]
fn BubbleDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Bubble { align: BubbleAlign::End,
                    BubbleContent { "Hey there! what's up?" }
                }
                BubbleGroup {
                    Bubble { variant: BubbleVariant::Muted,
                        BubbleContent { "Hey! Want to see chat bubbles?" }
                    }
                    Bubble { variant: BubbleVariant::Muted,
                        BubbleContent {
                            "I can group messages, switch sides, and keep the whole thread easy to scan."
                        }
                        BubbleReactions { role: "img", aria_label: "Reaction: thumbs up",
                            span { "👍" }
                        }
                    }
                }
                Bubble { align: BubbleAlign::End,
                    BubbleContent { "Sure. Hit me with your best demo." }
                }
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent {
                        "Yes. You are reading a demo that is demoing itself. Very meta. Very on-brand."
                    }
                    BubbleReactions {
                        role: "img",
                        aria_label: "Reactions: thumbs up, fire, eyes, and 2 more",
                        span { "👍" }
                        span { "🔥" }
                        span { "👀" }
                        span { "+2" }
                    }
                }
            }
        }
    }
}

#[component]
fn BubbleVariantsDemo() -> Element {
    rsx! {
        Example { title: "Variants", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-12 py-12",
                Bubble {
                    BubbleContent { "This is the default primary bubble." }
                }
                Bubble { variant: BubbleVariant::Secondary, align: BubbleAlign::End,
                    BubbleContent { "This is the secondary variant." }
                }
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent {
                        "This one is muted. It uses a lower emphasis color for the chat bubble."
                    }
                    BubbleReactions { role: "img", aria_label: "Reaction: thumbs up",
                        span { "👍" }
                    }
                }
                Bubble { variant: BubbleVariant::Tinted, align: BubbleAlign::End,
                    BubbleContent {
                        "This one is tinted. The tint is a softer color derived from the primary color."
                    }
                }
                Bubble { variant: BubbleVariant::Outline,
                    BubbleContent { "We can also use an outlined variant." }
                }
                Bubble { variant: BubbleVariant::Destructive, align: BubbleAlign::End,
                    BubbleContent { "Or a destructive variant with a reaction." }
                    BubbleReactions { role: "img", aria_label: "Reaction: fire",
                        span { "🔥" }
                    }
                }
                Bubble { variant: BubbleVariant::Ghost,
                    BubbleContent {
                        div { class: "flex flex-col gap-4",
                            p {
                                "Ghost bubbles work for assistant text, "
                                strong { "markdown" }
                                ", and other content that should not be framed."
                            }
                            p {
                                "This is perfect for assistant messages that should not have a frame and can take the full width of the container. You can also render "
                                code { class: "rounded-sm bg-muted px-1 py-0.5 font-mono text-[0.9em]",
                                    "code"
                                }
                                " in it."
                            }
                            p { "Ghost bubbles are full width and can take the full width of the container." }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BubbleAlignmentDemo() -> Element {
    rsx! {
        Example { title: "Alignment", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent {
                        "This bubble is aligned to the start. This is the default alignment."
                    }
                }
                Bubble { align: BubbleAlign::End,
                    BubbleContent { "This bubble is aligned to the end. Use this for user messages." }
                }
            }
        }
    }
}

#[component]
fn BubbleGroupDemo() -> Element {
    rsx! {
        Example { title: "Group", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent { "Can you tell me what's the issue?" }
                }
                BubbleGroup {
                    Bubble { align: BubbleAlign::End,
                        BubbleContent { "You tell me!" }
                    }
                    Bubble { align: BubbleAlign::End,
                        BubbleContent { "It worked yesterday. You broke it!" }
                    }
                    Bubble { align: BubbleAlign::End,
                        BubbleContent { "Find the bug and fix it." }
                        BubbleReactions { aria_label: "Reactions: eyes", align: BubbleAlign::Start,
                            span { "👀" }
                        }
                    }
                }
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent {
                        "Want me to diff yesterday's you against today's you? It's a bit embarrassing."
                    }
                }
            }
        }
    }
}

#[component]
fn BubbleLinkButtonDemo() -> Element {
    rsx! {
        Example { title: "Link / Button", class: "h-auto theme-blue",
            ToastProvider {
                LinkButtonBubbles {}
                Toaster {}
            }
        }
    }
}

#[component]
fn LinkButtonBubbles() -> Element {
    let toast = toast();
    let messages = [
        ("I forgot my password", "You clicked forgot password"),
        (
            "I need help with my subscription",
            "You clicked help with subscription",
        ),
        (
            "Something else. Talk to a human.",
            "You clicked something else. Talk to a human.",
        ),
    ];

    rsx! {
        div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent { "How can I help you today?" }
            }
            BubbleGroup {
                for (label , message) in messages {
                    Bubble {
                        key: "{label}",
                        variant: BubbleVariant::Tinted,
                        align: BubbleAlign::End,
                        BubbleContentButton {
                            onclick: move |_| {
                                toast.add(ToastOptions::new(message));
                            },
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BubbleReactionsDemo() -> Element {
    rsx! {
        Example { title: "Reactions", class: "h-auto theme-blue",
            ToastProvider {
                ReactionBubbles {}
                Toaster {}
            }
        }
    }
}

#[component]
fn ReactionBubbles() -> Element {
    let toast = toast();

    rsx! {
        div { class: "flex w-full max-w-sm flex-col gap-12 py-12",
            Bubble { variant: BubbleVariant::Muted, align: BubbleAlign::End,
                BubbleContent { "I don't need tests, I know my code works." }
                BubbleReactions {
                    align: BubbleAlign::Start,
                    role: "img",
                    aria_label: "Reactions: thumbs up, surprised",
                    span { "👍" }
                    span { "😮" }
                }
            }
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent { "Bold. Fine I'll add some tests. I'll let you know when they're done." }
                BubbleReactions { role: "img", aria_label: "Reactions: eyes, rocket, and 2 more",
                    span { "👀" }
                    span { "🚀" }
                    span { "+2" }
                }
            }
            Bubble { variant: BubbleVariant::Default, align: BubbleAlign::End,
                BubbleContent { "Tests passed on the first try. All 142 of them. Looking good!" }
                BubbleReactions {
                    side: BubbleReactionsSide::Top,
                    align: BubbleAlign::Start,
                    role: "img",
                    aria_label: "Reactions: party popper, clapping hands",
                    span { "🎉" }
                    span { "👏" }
                }
            }
            Bubble { variant: BubbleVariant::Destructive,
                BubbleContent { "Are you sure I can run this command?" }
                BubbleReactions {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Xs,
                        onclick: move |_| {
                            let mut options = ToastOptions::new("You clicked yes, running command...");
                            options.kind = Some(ToastType::Success);
                            toast.add(options);
                        },
                        "Yes, run it"
                    }
                }
            }
        }
    }
}

const COLLAPSIBLE_TEXT: &str = "The accessibility review found two focus states that were visually too subtle in dark mode.\n\nI checked the dialog, menu, and drawer paths because each one renders focusable controls inside a layered surface.\n\nThe dialog and drawer are fine. The menu needs the hover and focus tokens split so keyboard focus stays visible when the pointer is not involved.\n\nI also recommend keeping the change in the style file instead of the primitive so the other themes can choose their own focus treatment later.";

const PREVIEW_LENGTH: usize = 180;

#[component]
fn BubbleCollapsibleDemo() -> Element {
    let mut open = use_signal(|| false);
    let is_long = COLLAPSIBLE_TEXT.chars().count() > PREVIEW_LENGTH;
    let preview: String = COLLAPSIBLE_TEXT.chars().take(PREVIEW_LENGTH).collect();
    let shown = if open() || !is_long {
        COLLAPSIBLE_TEXT.to_string()
    } else {
        format!("{preview}...")
    };

    rsx! {
        Example { title: "Collapsible", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent { "How can I help you today?" }
                }
                Bubble { variant: BubbleVariant::Muted, align: BubbleAlign::End,
                    BubbleContent { class: "whitespace-pre-line",
                        Collapsible {
                            open: Some(open()),
                            on_open_change: move |next| open.set(next),
                            div { "{shown}" }
                            if is_long {
                                CollapsibleTrigger {
                                    class: button_variants(
                                        ButtonVariant::Link,
                                        ButtonSize::Default,
                                        Some("gap-1 p-0 text-muted-foreground"),
                                    ),
                                    if open() {
                                        "Show less"
                                    } else {
                                        "Show more"
                                    }
                                    span { "data-icon": "inline-end", class: "contents",
                                        ChevronDown { size: 16, class: "group-data-[state=open]/button:rotate-180" }
                                    }
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
fn BubbleTooltipDemo() -> Element {
    rsx! {
        Example { title: "Tooltip", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-4 py-12",
                Bubble { variant: BubbleVariant::Secondary,
                    BubbleContent { "Did you remove the stale route?" }
                }
                Bubble { align: BubbleAlign::End,
                    BubbleContent { "Yes, removed it from the registry." }
                    BubbleReactions {
                        Tooltip {
                            TooltipTrigger { class: button_variants(ButtonVariant::Ghost, ButtonSize::IconXs, None),
                                Check { size: 16 }
                            }
                            TooltipContent { "Read on Jan 5, 2026 at 4:32 PM" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BubblePopoverDemo() -> Element {
    rsx! {
        Example { title: "Popover", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-4 py-12",
                Bubble { align: BubbleAlign::End,
                    BubbleContent { "Run the build script." }
                }
                Bubble { variant: BubbleVariant::Destructive,
                    BubbleContent { "Failed to run the command." }
                    BubbleReactions {
                        Popover {
                            PopoverTrigger {
                                aria_label: "Show error details",
                                class: button_variants(
                                    ButtonVariant::Ghost,
                                    ButtonSize::IconXs,
                                    Some("aria-expanded:text-destructive"),
                                ),
                                Info { size: 16 }
                            }
                            PopoverContent {
                                PopoverHeader {
                                    PopoverTitle { class: "text-sm", "Command failed with exit code 1" }
                                    PopoverDescription { class: "text-sm",
                                        "ENOENT: no such file or directory, open pnpm-lock.yaml"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
