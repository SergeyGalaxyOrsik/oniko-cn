//! Message examples, ported from shadcn/ui `examples/base/message-*.tsx`.
//! The components page comes first, then the AI Elements page.

use dioxus::prelude::*;
use lucide_dioxus::{Check, Copy, Pencil, RefreshCcw, ThumbsDown, ThumbsUp};

use super::Example;
use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription,
    AttachmentMedia, AttachmentMediaVariant, AttachmentOrientation, AttachmentTitle,
};
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::bubble::{
    Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant,
};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::marker::{Marker, MarkerContent};
use crate::components::ui::message::{
    Message, MessageAlign, MessageAvatar, MessageContent, MessageFooter, MessageGroup,
    MessageHeader,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        MessageDemo {}
        MessageAvatarDemo {}
        MessageGroupDemo {}
        MessageHeaderFooterDemo {}
        MessageActionsDemo {}
        MessageAttachmentDemo {}
        MessagePattern {}
    }
}

#[component]
fn MessageDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-6 py-12",
                Message { align: MessageAlign::End,
                    MessageAvatar {
                        Avatar {
                            AvatarImage { src: "/avatars/10.png", alt: "@me" }
                            AvatarFallback { "ME" }
                        }
                    }
                    MessageContent {
                        Bubble {
                            BubbleContent { "Deploying to prod real quick." }
                        }
                    }
                }
                Message {
                    MessageAvatar {
                        Avatar {
                            AvatarImage { src: "/avatars/02.png", alt: "@rabbit" }
                            AvatarFallback { "R" }
                        }
                    }
                    MessageContent {
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "It's 4:55 PM. On a Friday." }
                        }
                    }
                }
                Message { align: MessageAlign::End,
                    MessageAvatar {
                        Avatar {
                            AvatarImage { src: "/avatars/10.png", alt: "@me" }
                            AvatarFallback { "ME" }
                        }
                    }
                    MessageContent {
                        Bubble {
                            BubbleContent { "It's a one-line change." }
                        }
                        MessageFooter { "Delivered" }
                    }
                }
                Message {
                    MessageAvatar {
                        Avatar {
                            AvatarImage { src: "/avatars/02.png", alt: "@rabbit" }
                            AvatarFallback { "R" }
                        }
                    }
                    MessageContent {
                        BubbleGroup {
                            Bubble { variant: BubbleVariant::Muted,
                                BubbleContent { "It's always a one-line change 😭." }
                            }
                            Bubble { variant: BubbleVariant::Muted,
                                BubbleContent { "Alright, let me take a look." }
                                BubbleReactions { aria_label: "Reactions: thumbs up",
                                    span { "👍" }
                                }
                            }
                        }
                    }
                }
                Marker { role: "status",
                    MarkerContent { class: "shimmer",
                        span { class: "font-medium", "Oliver" }
                        " is typing..."
                    }
                }
            }
        }
    }
}

#[component]
fn MessageAvatarDemo() -> Element {
    rsx! {
        Example { title: "Avatar", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-6 py-12",
                Message {
                    MessageAvatar {
                        Avatar {
                            AvatarImage { src: "/avatars/03.png", alt: "@avatar" }
                            AvatarFallback { "R" }
                        }
                    }
                    MessageContent {
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "The build failed during dependency installation." }
                        }
                    }
                }
                Message { align: MessageAlign::End,
                    MessageAvatar {
                        Avatar {
                            AvatarImage { src: "/avatars/10.png", alt: "@avatar" }
                            AvatarFallback { "R" }
                        }
                    }
                    MessageContent {
                        Bubble {
                            BubbleContent { "Can you share the exact error?" }
                        }
                    }
                }
                Message {
                    MessageAvatar {
                        Avatar {
                            AvatarImage { src: "/avatars/03.png", alt: "@avatar" }
                            AvatarFallback { "R" }
                        }
                    }
                    MessageContent {
                        BubbleGroup {
                            Bubble { variant: BubbleVariant::Muted,
                                BubbleContent { "Here's the error from the logs" }
                            }
                            Bubble { variant: BubbleVariant::Muted,
                                BubbleContent {
                                    "Something went wrong with the build. The libraries are not installed correctly. Try running the build again."
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
fn MessageGroupDemo() -> Element {
    rsx! {
        Example { title: "Group", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-6 py-12",
                MessageGroup {
                    Message {
                        MessageAvatar {}
                        MessageContent {
                            Bubble { variant: BubbleVariant::Muted,
                                BubbleContent { "I checked the registry addresses." }
                            }
                        }
                    }
                    Message {
                        MessageAvatar {
                            Avatar {
                                AvatarImage { src: "/avatars/02.png", alt: "@avatar" }
                                AvatarFallback { "CN" }
                            }
                        }
                        MessageContent {
                            Bubble { variant: BubbleVariant::Muted,
                                BubbleContent {
                                    "The component and example JSON now live under the UI registry."
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
fn MessageHeaderFooterDemo() -> Element {
    rsx! {
        Example { title: "Header and Footer", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Message {
                    MessageContent {
                        MessageHeader { "Olivia" }
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "I already checked the logs." }
                        }
                    }
                }
                Message { align: MessageAlign::End,
                    MessageContent {
                        Bubble {
                            BubbleContent { "Send the report to the team. Ping @shadcn if you need help." }
                        }
                        MessageFooter {
                            div {
                                "Read "
                                span { class: "font-normal", "Yesterday" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MessageActionsDemo() -> Element {
    rsx! {
        Example { title: "Actions", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Message {
                    MessageContent {
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "The install failure is coming from the workspace package." }
                        }
                        MessageFooter {
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Icon,
                                aria_label: "Copy",
                                title: "Copy",
                                Copy { size: 16 }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Icon,
                                aria_label: "Like",
                                title: "Like",
                                ThumbsUp { size: 16 }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Icon,
                                aria_label: "Dislike",
                                title: "Dislike",
                                ThumbsDown { size: 16 }
                            }
                        }
                    }
                }
                Message { align: MessageAlign::End,
                    MessageContent {
                        Bubble {
                            BubbleContent { "Okay drop me a link. Taking a look..." }
                        }
                        MessageFooter { class: "gap-2",
                            span { class: "font-normal text-destructive", "Failed to send" }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::IconXs,
                                title: "Retry",
                                aria_label: "Retry",
                                RefreshCcw { size: 16 }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MessageAttachmentDemo() -> Element {
    rsx! {
        Example { title: "Attachment", class: "h-auto theme-blue",
            div { class: "flex w-full max-w-sm flex-col gap-8 py-12",
                Message { align: MessageAlign::End,
                    MessageContent {
                        Attachment { orientation: AttachmentOrientation::Vertical,
                            AttachmentMedia { variant: AttachmentMediaVariant::Image,
                                img {
                                    src: "https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80",
                                    alt: "Workspace",
                                }
                            }
                        }
                        Bubble {
                            BubbleContent {
                                "Here's the image. Can you add it to the PDF? Use it for the cover page."
                            }
                        }
                    }
                }
                Message {
                    MessageContent {
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent {
                                "Done. Here's the PDF with the image added as the cover page."
                            }
                        }
                        Attachment {
                            AttachmentMedia {
                                lucide_dioxus::FileText { size: 16 }
                            }
                            AttachmentContent {
                                AttachmentTitle { "sales-dashboard.pdf" }
                                AttachmentDescription { "PDF · 2.4 MB" }
                            }
                            AttachmentActions {
                                AttachmentAction {
                                    title: "Download",
                                    aria_label: "Download",
                                    size: ButtonSize::IconSm,
                                    variant: ButtonVariant::Secondary,
                                    lucide_dioxus::Download { size: 16 }
                                }
                            }
                        }
                    }
                }
                Message { align: MessageAlign::End,
                    MessageContent {
                        Bubble {
                            BubbleContent { "Thanks. Looks good." }
                        }
                    }
                }
            }
        }
    }
}

const PATTERN_REPLY_INTRO: &str = "Install Dialog, then open it from the gear button in ";
const PATTERN_REPLY_TAIL: &str = "Wrap the button in a DialogTrigger and put the settings form in DialogContent. Focus moves into the dialog when it opens, and ";
const PATTERN_REPLY_COPY: &str = "Install Dialog, then open it from the gear button in `site-header.tsx`.\n\n```bash\nnpx shadcn@latest add dialog\n```\n\nWrap the button in a `DialogTrigger` and put the settings form in `DialogContent`. Focus moves into the dialog when it opens, and **Esc** closes it.";

#[component]
fn MessagePattern() -> Element {
    let mut copied = use_signal(|| false);
    let mut feedback = use_signal(|| None::<&'static str>);

    rsx! {
        Example { title: "Pattern (AI Elements)", class: "h-auto",
            div { class: "mx-auto flex w-full max-w-md flex-col gap-8 py-6 text-sm",
                Message { align: MessageAlign::End,
                    MessageContent { class: "gap-2",
                        Bubble { align: BubbleAlign::End, variant: BubbleVariant::Muted,
                            BubbleContent {
                                "How do I add a settings dialog? It should open from the gear icon in the header."
                            }
                        }
                        MessageFooter {
                            PatternAction { label: "Copy", Copy { size: 16 } }
                            PatternAction { label: "Edit", Pencil { size: 16 } }
                        }
                    }
                }
                Message {
                    MessageContent { class: "gap-2",
                        div { class: "flex flex-col gap-3",
                            p {
                                {PATTERN_REPLY_INTRO}
                                code { class: "rounded bg-muted px-1 py-0.5 font-mono text-[0.85em]",
                                    "site-header.tsx"
                                }
                                "."
                            }
                            pre { class: "overflow-x-auto rounded-lg bg-muted p-3 font-mono text-xs",
                                code { "npx shadcn@latest add dialog" }
                            }
                            p {
                                {PATTERN_REPLY_TAIL}
                                strong { "Esc" }
                                " closes it."
                            }
                        }
                        MessageFooter { class: "px-0",
                            PatternAction {
                                label: if copied() { "Copied" } else { "Copy" },
                                onclick: move |_| {
                                    let script = format!("navigator.clipboard.writeText({PATTERN_REPLY_COPY:?});");
                                    let _ = document::eval(&script);
                                    copied.set(true);
                                    spawn(async move {
                                        let _ = document::eval(
                                                "await new Promise((resolve) => setTimeout(resolve, 2000));",
                                            )
                                            .await;
                                        copied.set(false);
                                    });
                                },
                                if copied() {
                                    Check { size: 16 }
                                } else {
                                    Copy { size: 16 }
                                }
                            }
                            PatternAction {
                                label: "Good response",
                                pressed: feedback() == Some("like"),
                                onclick: move |_| {
                                    feedback.set(if feedback() == Some("like") { None } else { Some("like") });
                                },
                                ThumbsUp {
                                    size: 16,
                                    class: if feedback() == Some("like") { "fill-current" } else { "" },
                                }
                            }
                            PatternAction {
                                label: "Bad response",
                                pressed: feedback() == Some("dislike"),
                                onclick: move |_| {
                                    feedback
                                        .set(
                                            if feedback() == Some("dislike") { None } else { Some("dislike") },
                                        );
                                },
                                ThumbsDown {
                                    size: 16,
                                    class: if feedback() == Some("dislike") { "fill-current" } else { "" },
                                }
                            }
                            PatternAction { label: "Retry", RefreshCcw { size: 16 } }
                        }
                    }
                }
            }
        }
    }
}

/// Ghost icon button with a tooltip-style `title`, like upstream's local `MessageAction`.
#[component]
fn PatternAction(
    label: String,
    #[props(default)] pressed: Option<bool>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::IconSm,
            aria_label: label.clone(),
            title: label,
            aria_pressed: pressed.map(|pressed| pressed.to_string()),
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            {children}
        }
    }
}
