//! Message Scroller examples, ported from shadcn/ui `examples/base/message-scroller-*.tsx`.
//!
//! Upstream streams scripted replies with `@shadcn/helpers/ai-sdk` and animates rows with
//! `motion`. Here `use_scripted_chat` plays the same scripts word by word, and `MessageAnimated`
//! runs the entrance presets with the Web Animations API through `document::eval`.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUp, Globe, MessageCircleDashed, Paperclip, Plus, RotateCw, Telescope};

use crate::components::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger,
};
use crate::components::ui::empty::{
    Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use crate::components::ui::hover_card::{
    HoverCard, HoverCardAlign, HoverCardContent, HoverCardSide, HoverCardTrigger,
};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
};
use crate::components::ui::marker::{Marker, MarkerContent, MarkerVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent, MessageHeader};
use crate::components::ui::message_scroller::{
    MessageScroller, MessageScrollerButton, MessageScrollerContent,
    MessageScrollerDefaultScrollPosition, MessageScrollerItem, MessageScrollerProvider,
    MessageScrollerScrollAlign, MessageScrollerScrollBehavior, MessageScrollerScrollOptions,
    MessageScrollerScrollable, MessageScrollerViewport, use_message_scroller,
    use_message_scroller_scrollable, use_message_scroller_visibility,
};
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,
};
use crate::components::ui::slider::Slider;
use crate::components::ui::tabs::{Tabs, TabsList, TabsTrigger};
use crate::components::ui::toast::{ToastOptions, ToastProvider, Toaster, toast};
use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem};
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

const PREVIEW_CLASS: &str = "h-auto theme-blue bg-surface dark:bg-background p-4 min-[480px]:p-8 min-[560px]:p-10 sm:px-10 sm:py-16";

#[component]
pub fn Examples() -> Element {
    rsx! {
        super::Example { title: "Demo", class: PREVIEW_CLASS, MessageScrollerDemo {} }
        super::Example { title: "Anchoring Turns", class: PREVIEW_CLASS, MessageScrollerAnchoring {} }
        super::Example { title: "Group Chat", class: PREVIEW_CLASS, MessageScrollerGroupChat {} }
        super::Example { title: "Keeping Context Visible", class: PREVIEW_CLASS, MessageScrollerPreviousContext {} }
        super::Example { title: "Following the Live Edge", class: PREVIEW_CLASS, MessageScrollerStreaming {} }
        super::Example { title: "Opening Saved Threads", class: PREVIEW_CLASS, MessageScrollerOpeningPosition {} }
        super::Example { title: "Loading Earlier Messages", class: PREVIEW_CLASS, MessageScrollerLoadHistory {} }
        super::Example { title: "Animating New Messages", class: PREVIEW_CLASS, MessageScrollerAnimation {} }
        super::Example { title: "Jumping to Messages", class: PREVIEW_CLASS, MessageScrollerCommands {} }
        super::Example { title: "Tracking the Reader's Position", class: PREVIEW_CLASS, MessageScrollerVisibility {} }
        super::Example { title: "Reading Scroll State", class: PREVIEW_CLASS, MessageScrollerScrollableExample {} }
    }
}

// ---------------------------------------------------------------------------
// Scripted chat, the stand-in for `createChat()` + `useChat()`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    User,
    Assistant,
}

#[derive(Clone, Debug, PartialEq)]
struct ChatMessage {
    id: String,
    role: Role,
    text: String,
}

impl ChatMessage {
    fn new(id: impl Into<String>, role: Role, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            role,
            text: text.into(),
        }
    }

    fn is_user(&self) -> bool {
        self.role == Role::User
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChatStatus {
    Ready,
    Submitted,
    Streaming,
}

impl ChatStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Submitted => "submitted",
            Self::Streaming => "streaming",
        }
    }
}

/// A scripted conversation: each pair is one user turn and the assistant reply to it.
type Script = &'static [(&'static str, &'static str)];

/// The first `count` messages of a script, counting user and assistant messages separately.
fn script_messages(prefix: &str, script: Script, count: usize) -> Vec<ChatMessage> {
    script
        .iter()
        .enumerate()
        .flat_map(|(index, (user, assistant))| {
            [
                ChatMessage::new(format!("{prefix}-{index}-user"), Role::User, *user),
                ChatMessage::new(
                    format!("{prefix}-{index}-assistant"),
                    Role::Assistant,
                    *assistant,
                ),
            ]
        })
        .take(count)
        .collect()
}

/// Splits text into word-plus-whitespace deltas, like upstream `splitTextDeltas`.
fn text_deltas(text: &str) -> Vec<String> {
    let mut deltas = Vec::new();
    let mut current = String::new();
    let mut in_space = false;
    for character in text.chars() {
        let space = character.is_whitespace();
        if !space && in_space && !current.is_empty() {
            deltas.push(std::mem::take(&mut current));
        }
        in_space = space;
        current.push(character);
    }
    if !current.is_empty() {
        deltas.push(current);
    }
    deltas
}

fn paragraphs(text: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    for line in text.split('\n') {
        if line.trim().is_empty() {
            if !current.is_empty() {
                paragraphs.push(current.join("\n").trim().to_string());
                current.clear();
            }
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        paragraphs.push(current.join("\n").trim().to_string());
    }
    paragraphs.retain(|paragraph| !paragraph.is_empty());
    paragraphs
}

fn trimmed_text(text: &str) -> String {
    if text.chars().count() > 42 {
        format!("{}...", text.chars().take(39).collect::<String>())
    } else {
        text.to_string()
    }
}

async fn sleep(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms})); return true;"
    ))
    .await;
}

#[derive(Clone, Copy, PartialEq)]
struct ScriptedChat {
    prefix: &'static str,
    script: Script,
    initial: usize,
    sleep_ms: u32,
    delay_ms: u32,
    messages: Signal<Vec<ChatMessage>>,
    status: Signal<ChatStatus>,
}

impl ScriptedChat {
    fn is_busy(&self) -> bool {
        matches!(
            *self.status.read(),
            ChatStatus::Submitted | ChatStatus::Streaming
        )
    }

    fn next_turn(&self) -> Option<usize> {
        let sent = self
            .messages
            .read()
            .iter()
            .filter(|message| message.is_user())
            .count();
        (sent < self.script.len()).then_some(sent)
    }

    fn next_text(&self) -> Option<&'static str> {
        self.next_turn().map(|turn| self.script[turn].0)
    }

    fn reset(&self) {
        let mut messages = self.messages;
        let mut status = self.status;
        messages.set(script_messages(self.prefix, self.script, self.initial));
        status.set(ChatStatus::Ready);
    }

    fn send(&self) {
        if self.is_busy() {
            return;
        }
        let Some(turn) = self.next_turn() else {
            return;
        };
        let chat = *self;
        let mut messages = self.messages;
        let mut status = self.status;
        let (user, assistant) = self.script[turn];
        messages.write().push(ChatMessage::new(
            format!("{}-{turn}-user", chat.prefix),
            Role::User,
            user,
        ));
        status.set(ChatStatus::Submitted);
        spawn(async move {
            if chat.sleep_ms > 0 {
                sleep(chat.sleep_ms).await;
            }
            let id = format!("{}-{turn}-assistant", chat.prefix);
            for delta in text_deltas(assistant) {
                if chat.delay_ms > 0 {
                    sleep(chat.delay_ms).await;
                }
                let mut list = messages.write();
                match list.last_mut() {
                    Some(last) if last.id == id => last.text.push_str(&delta),
                    _ => list.push(ChatMessage::new(id.clone(), Role::Assistant, delta)),
                }
                drop(list);
                if *status.peek() != ChatStatus::Streaming {
                    status.set(ChatStatus::Streaming);
                }
            }
            status.set(ChatStatus::Ready);
        });
    }
}

fn use_scripted_chat(
    prefix: &'static str,
    script: Script,
    initial: usize,
    sleep_ms: u32,
    delay_ms: u32,
) -> ScriptedChat {
    let messages = use_signal(|| script_messages(prefix, script, initial));
    let status = use_signal(|| ChatStatus::Ready);
    ScriptedChat {
        prefix,
        script,
        initial,
        sleep_ms,
        delay_ms,
        messages,
        status,
    }
}

const LAUNCH_SCRIPT: Script = &[
    (
        "I'm building a chat for our app and the scroll behavior is driving me nuts. Every time the AI streams a reply, the whole thread jumps around.",
        "That's the classic streaming scroll problem. Wrap your message list in `MessageScroller` and turn on `autoScroll` — the viewport pins to the bottom as tokens arrive, so users always see the latest text land in place.\n\nThe important part: it only auto-scrolls while the reader is already at the bottom. The moment they scroll up to read something earlier, auto-scroll backs off and their position is preserved. You get smooth streaming without fighting the user's intent.",
    ),
    (
        "Okay, but when someone sends a new message the view still feels jarring — like the whole conversation reloads from the top.",
        "MessageScrollerItem fixes that with turn anchoring. Set `scrollAnchor` on the turn that should settle near the top instead of blindly snapping to the document bottom.\n\nIt also leaves a small peek of the previous exchange visible above the anchor, so context isn't lost. The reply starts in view without that disorienting jump you get from a plain overflow container.",
    ),
    (
        "And if they've scrolled up to re-read an older answer? I don't want to yank them back down.",
        "You won't. Auto-scroll only runs when the viewport is already pinned to the bottom, so scrolling up is a deliberate opt-out — their place in the thread stays put even as new tokens keep arriving below.\n\nWhen there is content they haven't seen yet, `MessageScrollerButton` appears at the bottom of the viewport. One tap jumps them back to the newest message and re-engages auto-scroll. Same pattern as Slack or iMessage: quiet when you're caught up, helpful when you're not.",
    ),
    (
        "Last one — does this work with assistive tech?",
        "`MessageScrollerContent` sets `role=\"log\"` and `aria-relevant=\"additions\"` by default, so screen readers announce new messages as they stream in.\n\nThe scroll button is a real `<button>` with an sr-only label, and it's removed from the tab order when you're already at the bottom — no ghost focus stops.",
    ),
];

// ---------------------------------------------------------------------------
// `MessageAnimated`, the stand-in for upstream's `components/message-animated.tsx`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AnimationPreset {
    id: &'static str,
    name: &'static str,
    keyframes: &'static str,
    options: &'static str,
}

const MESSAGE_ANIMATIONS: &[AnimationPreset] = &[
    AnimationPreset {
        id: "fade",
        name: "Fade",
        keyframes: "[{ opacity: 0 }, { opacity: 1 }]",
        options: "{ duration: 220, easing: 'ease-out' }",
    },
    AnimationPreset {
        id: "slide-up",
        name: "Slide Up",
        keyframes: "[{ opacity: 0, transform: 'translateY(10px)' }, { opacity: 1, transform: 'none' }]",
        options: "{ duration: 260, easing: 'cubic-bezier(0.16, 1, 0.3, 1)' }",
    },
    AnimationPreset {
        id: "slide-side",
        name: "Slide Side",
        keyframes: "[{ opacity: 0, transform: 'translateX(18px)' }, { opacity: 1, transform: 'none' }]",
        options: "{ duration: 280, easing: 'cubic-bezier(0.16, 1, 0.3, 1)' }",
    },
    AnimationPreset {
        id: "pop",
        name: "Pop",
        keyframes: "[{ opacity: 0, transform: 'translateY(6px) scale(0.94)', transformOrigin: '100% 100%' }, { opacity: 1, transform: 'none', transformOrigin: '100% 100%' }]",
        options: "{ duration: 320, easing: 'cubic-bezier(0.22, 1, 0.36, 1)' }",
    },
    AnimationPreset {
        id: "spring-bounce",
        name: "Spring Bounce",
        keyframes: "[{ opacity: 0, transform: 'translateY(12px) scale(0.96)' }, { opacity: 1, transform: 'none' }]",
        options: "{ duration: 380, easing: 'cubic-bezier(0.34, 1.4, 0.64, 1)' }",
    },
    AnimationPreset {
        id: "blur-fade",
        name: "Blur Fade",
        keyframes: "[{ opacity: 0, filter: 'blur(4px)', transform: 'translateY(6px)' }, { opacity: 1, filter: 'blur(0px)', transform: 'none' }]",
        options: "{ duration: 280, easing: 'ease-out' }",
    },
    AnimationPreset {
        id: "scale-fade",
        name: "Scale Fade",
        keyframes: "[{ opacity: 0, transform: 'scale(0.98)' }, { opacity: 1, transform: 'none' }]",
        options: "{ duration: 240, easing: 'ease-out' }",
    },
];

fn preset_by_id(id: &str) -> AnimationPreset {
    MESSAGE_ANIMATIONS
        .iter()
        .copied()
        .find(|preset| preset.id == id)
        .unwrap_or(MESSAGE_ANIMATIONS[1])
}

/// Plays a preset on the row with `message_id` once, when this component mounts inside it.
#[component]
fn EntranceAnimation(message_id: String, preset: AnimationPreset) -> Element {
    use_hook(move || {
        let script = format!(
            "if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return; \
             const id = {message_id:?}; \
             const row = Array.from(document.querySelectorAll('[data-message-id]')).find((node) => node.dataset.messageId === id); \
             if (row) row.animate({keyframes}, {options});",
            keyframes = preset.keyframes,
            options = preset.options,
        );
        spawn(async move {
            let _ = document::eval(&script).await;
        });
    });
    rsx! {}
}

#[component]
fn MessageAnimated(
    message: ChatMessage,
    #[props(default = preset_by_id("slide-up"))] animation_preset: AnimationPreset,
    #[props(default = BubbleVariant::Ghost)] assistant_variant: BubbleVariant,
    scroll_anchor: Option<bool>,
    #[props(default = BubbleVariant::Muted)] user_variant: BubbleVariant,
) -> Element {
    let is_user = message.is_user();
    let variant = if is_user {
        user_variant
    } else {
        assistant_variant
    };
    let row = rsx! {
        Message { align: if is_user { MessageAlign::End } else { MessageAlign::Start },
            MessageContent {
                Bubble { variant,
                    BubbleContent { class: "space-y-2",
                        for (index, paragraph) in paragraphs(&message.text).into_iter().enumerate() {
                            p { key: "{index}", class: "whitespace-pre-wrap", "{paragraph}" }
                        }
                    }
                }
            }
        }
    };

    if is_user {
        rsx! {
            MessageScrollerItem {
                message_id: message.id.clone(),
                scroll_anchor: scroll_anchor.unwrap_or(true),
                {row}
                EntranceAnimation { message_id: message.id.clone(), preset: animation_preset }
            }
        }
    } else {
        rsx! {
            MessageScrollerItem {
                message_id: message.id.clone(),
                scroll_anchor: scroll_anchor.unwrap_or(false),
                {row}
            }
        }
    }
}

/// A row of the static transcripts: one message in a bubble, split into paragraphs.
#[component]
fn TranscriptMessage(message: ChatMessage) -> Element {
    let is_user = message.is_user();
    rsx! {
        Message { align: if is_user { MessageAlign::End } else { MessageAlign::Start },
            MessageContent {
                Bubble { variant: if is_user { BubbleVariant::Muted } else { BubbleVariant::Ghost },
                    BubbleContent { class: "space-y-2",
                        for (index, paragraph) in paragraphs(&message.text).into_iter().enumerate() {
                            p { key: "{index}", class: "whitespace-pre-wrap", "{paragraph}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ResetButton(label: &'static str, disabled: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Icon,
                    aria_label: label,
                    disabled,
                    onclick: move |event| onclick.call(event),
                    RotateCw {}
                }
            }
            TooltipContent {
                p { "Reset" }
            }
        }
    }
}

/// The read-only composer of the streaming demos: the queued message and a send button.
#[component]
fn ScriptedComposer(
    chat: ScriptedChat,
    empty_text: &'static str,
    addon: Option<Element>,
) -> Element {
    let status = *chat.status.read();
    let next = chat.next_text();
    let busy = chat.is_busy();
    rsx! {
        form {
            class: "w-full",
            onsubmit: move |event| {
                event.prevent_default();
                chat.send();
            },
            InputGroup {
                div { class: "h-14 w-full px-3 py-2.5",
                    span {
                        class: "line-clamp-2 opacity-60 data-[status=ready]:opacity-100",
                        "data-status": status.as_str(),
                        if let Some(text) = next {
                            "{text}"
                        } else {
                            span { class: "text-muted-foreground", "{empty_text}" }
                        }
                    }
                }
                InputGroupAddon { align: InputGroupAlign::BlockEnd, class: "pt-1",
                    DropdownMenu {
                        DropdownMenuTrigger {
                            aria_label: "Add files",
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::IconSm,
                            Plus {}
                        }
                        DropdownMenuContent { align: DropdownMenuAlign::Start, class: "w-44",
                            DropdownMenuItem {
                                Paperclip {}
                                "Add Photos & Files"
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuItem {
                                lucide_dioxus::Image {}
                                "Create Image"
                            }
                            DropdownMenuItem {
                                Telescope {}
                                "Deep Research"
                            }
                            DropdownMenuItem {
                                Globe {}
                                "Web Search"
                            }
                        }
                    }
                    {addon}
                    InputGroupButton {
                        variant: ButtonVariant::Default,
                        size: InputGroupButtonSize::IconSm,
                        disabled: next.is_none() || busy,
                        class: "ml-auto",
                        onclick: move |_| chat.send(),
                        ArrowUp {}
                        span { class: "sr-only", "Send" }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-demo

#[component]
fn MessageScrollerDemo() -> Element {
    let chat = use_scripted_chat("demo", LAUNCH_SCRIPT, 0, 1000, 20);
    let busy = chat.is_busy();
    let messages = chat.messages.read().clone();

    rsx! {
        MessageScrollerProvider {
            div { class: "relative flex flex-col gap-4",
                Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                    CardHeader { class: "gap-1 border-b",
                        CardTitle { "New Chat" }
                        CardDescription { "How can I help you today?" }
                        CardAction {
                            ResetButton {
                                label: "Reset conversation",
                                disabled: busy,
                                onclick: move |_| chat.reset(),
                            }
                        }
                    }
                    CardContent { class: "flex-1 overflow-hidden p-0",
                        if messages.is_empty() {
                            Empty { class: "h-full",
                                EmptyHeader {
                                    EmptyMedia { variant: EmptyMediaVariant::Icon, MessageCircleDashed {} }
                                    EmptyTitle { "Morning, shadcn!" }
                                    EmptyDescription {
                                        "What are we working on today? Press send to start a new conversation"
                                    }
                                }
                            }
                        } else {
                            MessageScroller {
                                MessageScrollerViewport {
                                    MessageScrollerContent { aria_busy: busy, class: "p-(--card-spacing)",
                                        for message in messages {
                                            MessageAnimated {
                                                key: "{message.id}",
                                                scroll_anchor: message.is_user(),
                                                message,
                                            }
                                        }
                                    }
                                }
                                MessageScrollerButton {}
                            }
                        }
                    }
                    CardFooter { class: "flex-col gap-2",
                        ScriptedComposer { chat, empty_text: "No messages queued. Reset the conversation." }
                    }
                }
                div { class: "px-0.5 text-center text-xs text-muted-foreground",
                    "Demo is read only. Press send to send messages."
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-anchoring

const ANCHORING_MESSAGES: &[(&str, Role, &str)] = &[
    (
        "anchor-1-user",
        Role::User,
        "Can you show me how anchoring behaves when a new prompt starts the turn?",
    ),
    (
        "anchor-1-assistant",
        Role::Assistant,
        "Append the user prompt first, then append the assistant response. With User selected, the prompt settles near the top and the assistant response fills in below it.",
    ),
    (
        "anchor-2-user",
        Role::User,
        "What changes when assistant messages are the anchor?",
    ),
    (
        "anchor-2-assistant",
        Role::Assistant,
        "Now each assistant response is the item `MessageScroller` keeps in view. This is useful when the reply is the moment you want readers to land on after each turn.",
    ),
    (
        "anchor-3-user",
        Role::User,
        "Can I switch roles and keep adding turns?",
    ),
    (
        "anchor-3-assistant",
        Role::Assistant,
        "Yes. The next appended message with the selected role becomes the anchor, so you can compare user and assistant anchoring without resetting the demo.",
    ),
];

#[component]
fn MessageScrollerAnchoring() -> Element {
    let mut anchor_role = use_signal(|| Role::User);
    let mut messages = use_signal(Vec::<ChatMessage>::new);
    let mut message_index = use_signal(|| 0usize);
    let next = ANCHORING_MESSAGES.get(message_index()).copied();
    let list = messages.read().clone();
    let role = anchor_role();

    rsx! {
        div { class: "relative flex flex-col gap-4",
            Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                CardHeader { class: "border-b",
                    CardTitle { "Anchoring Turns" }
                    CardDescription { "Choose which role settles near the top edge." }
                    CardAction {
                        Button {
                            r#type: "button",
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Icon,
                            aria_label: "Reset anchored turns",
                            disabled: list.is_empty(),
                            onclick: move |_| {
                                messages.set(Vec::new());
                                message_index.set(0);
                            },
                            RotateCw {}
                        }
                    }
                }
                CardContent { class: "min-h-0 flex-1 overflow-hidden p-0",
                    if list.is_empty() {
                        Empty { class: "h-full",
                            EmptyHeader {
                                EmptyMedia { variant: EmptyMediaVariant::Icon, MessageCircleDashed {} }
                                EmptyTitle { "No anchored messages yet" }
                                EmptyDescription { "Send the first message to see the selected role anchor." }
                            }
                        }
                    } else {
                        MessageScrollerProvider {
                            MessageScroller {
                                MessageScrollerViewport {
                                    MessageScrollerContent { class: "p-(--card-spacing)",
                                        for message in list {
                                            MessageAnimated {
                                                key: "{message.id}",
                                                scroll_anchor: message.role == role,
                                                user_variant: BubbleVariant::Muted,
                                                assistant_variant: BubbleVariant::Ghost,
                                                message,
                                            }
                                        }
                                    }
                                }
                                MessageScrollerButton {}
                            }
                        }
                    }
                }
                CardFooter {
                    ToggleGroup {
                        aria_label: "Select scroll anchor role",
                        value: vec![if role == Role::User { "user".to_string() } else { "assistant".to_string() }],
                        on_value_change: move |value: Vec<String>| {
                            let next = match value.first().map(String::as_str) {
                                Some("user") => Role::User,
                                Some("assistant") => Role::Assistant,
                                _ => return,
                            };
                            anchor_role.set(next);
                            messages.set(Vec::new());
                            message_index.set(0);
                        },
                        ToggleGroupItem { value: "user", aria_label: "Anchor user messages", "User" }
                        ToggleGroupItem { value: "assistant", aria_label: "Anchor assistant messages", "Assistant" }
                    }
                    Button {
                        r#type: "button",
                        size: ButtonSize::Icon,
                        class: "ml-auto",
                        disabled: next.is_none(),
                        onclick: move |_| {
                            let Some((id, role, text)) = next else {
                                return;
                            };
                            messages.write().push(ChatMessage::new(id, role, text));
                            message_index += 1;
                        },
                        ArrowUp {}
                        span { class: "sr-only", "Send Message" }
                    }
                }
            }
            div { class: "mx-auto max-w-xs px-0.5 text-center text-xs text-muted-foreground",
                "Toggle the anchor role, then send messages to compare where turns settle."
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-group-chat

const CURRENT_USER: &str = "Grace";

#[derive(Clone, Copy, PartialEq)]
enum GroupChatItem {
    Event {
        id: &'static str,
        text: &'static str,
        scroll_anchor: bool,
    },
    Message {
        id: &'static str,
        sender: &'static str,
        assistant: bool,
        text: &'static str,
        scroll_anchor: bool,
    },
}

const GROUP_INITIAL_ITEMS: &[GroupChatItem] = &[
    GroupChatItem::Message {
        id: "group-1",
        sender: "Grace",
        assistant: false,
        text: "@mary, the astrophage line keeps matching Venus energy output. Can you check my math?",
        scroll_anchor: false,
    },
    GroupChatItem::Message {
        id: "group-2",
        sender: "Mary (Agent)",
        assistant: true,
        text: "Yes. Confirmed. The curve points to a microorganism harvesting stellar energy and breeding near carbon dioxide. If @rocky agrees, this is the clue we need.",
        scroll_anchor: false,
    },
    GroupChatItem::Message {
        id: "group-3",
        sender: "Grace",
        assistant: false,
        text: "ping @rocky",
        scroll_anchor: true,
    },
];

const ROCKY_MARKER: GroupChatItem = GroupChatItem::Event {
    id: "group-4",
    text: "Rocky has joined the chat",
    scroll_anchor: true,
};

const ROCKY_MESSAGE: GroupChatItem = GroupChatItem::Message {
    id: "group-5",
    sender: "Rocky",
    assistant: false,
    text: "Amaze. Astrophage eats light, makes heat, goes to carbon dioxide. Rocky has fuel model. Grace is smart.",
    scroll_anchor: false,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum RockyTurn {
    Idle,
    Marker,
    Message,
}

#[component]
fn MessageScrollerGroupChat() -> Element {
    let mut demo_key = use_signal(|| 0u32);
    let mut rocky_turn = use_signal(|| RockyTurn::Idle);
    let turn = rocky_turn();
    let mut items = GROUP_INITIAL_ITEMS.to_vec();
    if turn != RockyTurn::Idle {
        items.push(ROCKY_MARKER);
    }
    if turn == RockyTurn::Message {
        items.push(ROCKY_MESSAGE);
    }
    let button_label = if turn == RockyTurn::Idle {
        "Add Rocky"
    } else {
        "Send Message as Rocky"
    };
    let key = demo_key();

    rsx! {
        MessageScrollerProvider {
            div { class: "relative flex flex-col gap-4",
                Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                    CardHeader { class: "gap-1 border-b",
                        CardTitle { "Group Chat" }
                        CardDescription {
                            "A group chat with several participants and an assistant. The Marker is marked as a turn."
                        }
                        CardAction {
                            ResetButton {
                                label: "Reset conversation",
                                disabled: turn == RockyTurn::Idle,
                                onclick: move |_| {
                                    rocky_turn.set(RockyTurn::Idle);
                                    demo_key += 1;
                                },
                            }
                        }
                    }
                    CardContent { class: "min-h-0 flex-1 p-0",
                        MessageScrollerProvider {
                            for key in [key] {
                                MessageScroller { key: "{key}",
                                    MessageScrollerViewport {
                                        MessageScrollerContent { class: "p-(--card-spacing)",
                                            for item in items.iter().copied() {
                                                match item {
                                                    GroupChatItem::Message { id, .. } => rsx! {
                                                        GroupChatMessage { key: "{id}", item }
                                                    },
                                                    GroupChatItem::Event { id, scroll_anchor, .. } => rsx! {
                                                        GroupChatMarker { key: "{id}", item, scroll_anchor }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                    MessageScrollerButton {}
                                }
                            }
                        }
                    }
                    CardFooter { class: "flex flex-col items-center gap-2 border-t",
                        Button {
                            r#type: "button",
                            disabled: turn == RockyTurn::Message,
                            onclick: move |_| {
                                rocky_turn.set(if turn == RockyTurn::Idle { RockyTurn::Marker } else { RockyTurn::Message });
                            },
                            class: "w-full",
                            variant: ButtonVariant::Secondary,
                            "{button_label}"
                        }
                        p { class: "text-xs text-muted-foreground",
                            if turn == RockyTurn::Idle {
                                "This will create a marker and make it the anchor"
                            } else {
                                "Now send Rocky's reply into the conversation"
                            }
                        }
                    }
                }
                div { class: "mx-auto max-w-sm px-0.5 text-center text-xs text-balance text-muted-foreground",
                    "When a user joins, a marker is created. scrollAnchor on the marker marks it as the next turn"
                }
            }
        }
    }
}

#[component]
fn GroupChatMessage(item: GroupChatItem) -> Element {
    let GroupChatItem::Message {
        id,
        sender,
        assistant,
        text,
        scroll_anchor,
    } = item
    else {
        return rsx! {};
    };
    let is_current_user = sender == CURRENT_USER;
    let variant = if is_current_user {
        BubbleVariant::Muted
    } else if assistant {
        BubbleVariant::Ghost
    } else {
        BubbleVariant::Tinted
    };

    rsx! {
        MessageScrollerItem { message_id: id, scroll_anchor,
            Message { align: if is_current_user { MessageAlign::End } else { MessageAlign::Start },
                MessageContent {
                    if !is_current_user {
                        MessageHeader { "{sender}" }
                    }
                    Bubble { variant,
                        BubbleContent { "{text}" }
                    }
                }
            }
        }
    }
}

#[component]
fn GroupChatMarker(item: GroupChatItem, #[props(default)] scroll_anchor: bool) -> Element {
    let GroupChatItem::Event { text, .. } = item else {
        return rsx! {};
    };
    rsx! {
        MessageScrollerItem { scroll_anchor,
            Marker { variant: MarkerVariant::Separator,
                MarkerContent { "{text}" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-previous-context

const DEFAULT_PEEK: f64 = 64.0;

#[component]
fn MessageScrollerPreviousContext() -> Element {
    let mut demo_key = use_signal(|| 0u32);
    let mut peek = use_signal(|| DEFAULT_PEEK);
    let chat = use_scripted_chat("context", LAUNCH_SCRIPT, 2, 1000, 35);
    let busy = chat.is_busy();
    let messages = chat.messages.read().clone();
    let key = demo_key();
    let peek_value = peek();

    rsx! {
        for key in [key] {
            MessageScrollerProvider {
                key: "{key}",
                scroll_margin: 24.0,
                scroll_previous_item_peek: peek_value,
                div { class: "relative flex flex-col gap-4",
                    Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                        CardHeader { class: "gap-1 border-b",
                            CardTitle { "Keeping Context Visible" }
                            CardDescription { "New turns keep part of the previous reply in view." }
                            CardAction {
                                ResetButton {
                                    label: "Reset context example",
                                    disabled: busy,
                                    onclick: move |_| {
                                        chat.reset();
                                        peek.set(DEFAULT_PEEK);
                                        demo_key += 1;
                                    },
                                }
                            }
                        }
                        CardContent { class: "flex-1 overflow-hidden p-0",
                            MessageScroller {
                                MessageScrollerViewport {
                                    MessageScrollerContent { aria_busy: busy, class: "p-(--card-spacing)",
                                        for message in messages.iter().cloned() {
                                            MessageAnimated {
                                                key: "{message.id}",
                                                scroll_anchor: message.is_user(),
                                                message,
                                            }
                                        }
                                    }
                                }
                                MessageScrollerButton {}
                            }
                        }
                        CardFooter { class: "flex-col gap-2",
                            ScriptedComposer {
                                chat,
                                empty_text: "No messages queued. Reset the context.",
                                addon: rsx! {
                                    div { class: "flex w-28 items-center gap-2",
                                        span { class: "text-xs text-muted-foreground tabular-nums", "{peek_value}px" }
                                        Slider {
                                            aria_label: "Previous context peek",
                                            value: vec![peek_value],
                                            min: 64.0,
                                            max: 128.0,
                                            step: 1.0,
                                            disabled: busy,
                                            on_value_change: move |value: Vec<f64>| {
                                                peek.set(value.first().copied().unwrap_or(DEFAULT_PEEK));
                                            },
                                        }
                                    }
                                },
                            }
                        }
                    }
                    div { class: "px-0.5 text-center text-xs text-muted-foreground",
                        "Adjust the slider and send. Observe the previous message peak"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-streaming

#[component]
fn MessageScrollerStreaming() -> Element {
    let chat = use_scripted_chat("streaming", LAUNCH_SCRIPT, 0, 1000, 20);
    let busy = chat.is_busy();
    let messages = chat.messages.read().clone();

    rsx! {
        MessageScrollerProvider { auto_scroll: true,
            div { class: "relative flex flex-col gap-4",
                Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                    CardHeader { class: "gap-1 border-b",
                        CardTitle { "Streaming Messages" }
                        CardDescription { "Auto-scroll follows the live edge of the conversation." }
                        CardAction {
                            ResetButton {
                                label: "Reset stream",
                                disabled: messages.is_empty() || busy,
                                onclick: move |_| chat.reset(),
                            }
                        }
                    }
                    CardContent { class: "flex-1 overflow-hidden p-0",
                        if messages.is_empty() {
                            Empty { class: "h-full",
                                EmptyHeader {
                                    EmptyMedia { variant: EmptyMediaVariant::Icon, MessageCircleDashed {} }
                                    EmptyTitle { "Ready to Stream" }
                                    EmptyDescription { "Press send to stream a scripted launch summary." }
                                }
                            }
                        } else {
                            MessageScroller {
                                MessageScrollerViewport {
                                    MessageScrollerContent { aria_busy: busy, class: "p-(--card-spacing)",
                                        for message in messages {
                                            MessageAnimated {
                                                key: "{message.id}",
                                                scroll_anchor: message.is_user(),
                                                message,
                                            }
                                        }
                                    }
                                }
                                MessageScrollerButton {}
                            }
                        }
                    }
                    CardFooter { class: "flex-col gap-2",
                        ScriptedComposer { chat, empty_text: "No messages queued. Reset the stream." }
                    }
                }
                div { class: "px-0.5 text-center text-xs text-muted-foreground",
                    "Streaming is simulated. `autoScroll` is enabled."
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-opening-position

const OPENING_MESSAGES: &[(&str, Role, &str)] = &[
    (
        "open-1",
        Role::User,
        "This is the first message the user sent in the conversation.",
    ),
    (
        "open-2",
        Role::Assistant,
        "Workspace creation rose 8%, but first invite completion only rose 2%.",
    ),
    (
        "open-3",
        Role::User,
        "This is the last message the user sent in the conversation.",
    ),
    (
        "open-4",
        Role::Assistant,
        "Start with the invite step. Teams are creating workspaces but waiting to add collaborators.\n\nRecommended follow-up:\n\n1. Compare invite drop-off by account size.\n2. Check whether users who skip invites still return within 24 hours.\n3. Review the empty-state copy on the first project screen.\n4. Segment activation by template, since template users may not need invites right away.\n\nIf that pattern holds, the next experiment should make collaboration useful earlier instead of prompting for invites harder.",
    ),
];

const POSITIONS: &[&str] = &["start", "end", "last-anchor"];

#[component]
fn MessageScrollerOpeningPosition() -> Element {
    let mut position_key = use_signal(|| 0u32);
    let mut position = use_signal(|| "last-anchor".to_string());

    rsx! {
        div { class: "relative flex flex-col gap-4",
            Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                CardHeader { class: "gap-1 border-b",
                    CardTitle { "Opening Position" }
                    CardDescription { "Choose where a saved transcript opens." }
                }
                CardContent { class: "flex-1 overflow-hidden p-0",
                    MessageScrollerProvider {
                        OpeningPositionScroller { position: position(), position_key: position_key() }
                    }
                }
                CardFooter { class: "flex items-center justify-center border-t",
                    Tabs {
                        value: position(),
                        on_value_change: move |value: String| {
                            if POSITIONS.contains(&value.as_str()) {
                                position.set(value);
                                position_key += 1;
                            }
                        },
                        class: "w-full",
                        TabsList { class: "w-full",
                            for option in POSITIONS {
                                TabsTrigger { key: "{option}", value: *option, "{option}" }
                            }
                        }
                    }
                }
            }
            div { class: "mx-auto max-w-sm px-0.5 text-center text-xs text-muted-foreground",
                "Toggle the defaultScrollPosition to see where the transcript starts when you open the thread"
            }
        }
    }
}

#[component]
fn OpeningPositionScroller(position: String, position_key: u32) -> Element {
    let scroller = use_message_scroller();

    use_effect(use_reactive!(|(position, position_key)| {
        let _ = position_key;
        match position.as_str() {
            "start" => scroller.scroll_to_start(MessageScrollerScrollOptions {
                behavior: Some(MessageScrollerScrollBehavior::Auto),
                ..Default::default()
            }),
            "end" => scroller.scroll_to_end(MessageScrollerScrollOptions {
                behavior: Some(MessageScrollerScrollBehavior::Auto),
                ..Default::default()
            }),
            _ => scroller.scroll_to_message(
                "open-3",
                MessageScrollerScrollOptions {
                    align: Some(MessageScrollerScrollAlign::Start),
                    behavior: Some(MessageScrollerScrollBehavior::Auto),
                    scroll_margin: Some(64.0),
                },
            ),
        }
    }));

    rsx! {
        MessageScroller {
            MessageScrollerViewport {
                MessageScrollerContent { class: "p-(--card-spacing)",
                    for (id, role, text) in OPENING_MESSAGES.iter().copied() {
                        MessageScrollerItem { key: "{id}", message_id: id, scroll_anchor: role == Role::User,
                            TranscriptMessage { message: ChatMessage::new(id, role, text) }
                        }
                    }
                }
            }
            MessageScrollerButton {}
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-load-history

const HISTORY_SCRIPT: Script = &[
    (
        "Can you summarize the incident channel?",
        "The first alert was a delayed export job. It started backing up around 09:42 UTC and triggered the warning once the retry queue crossed the threshold.\n\nNo customer-facing checkout paths were affected, but exports for larger workspaces were running about 12 minutes behind.",
    ),
    (
        "Was checkout affected?",
        "No checkout errors were reported. Payment authorization, order creation, and confirmation emails stayed inside their normal latency bands.\n\nThe only elevated metric was export queue depth, which maps to analytics downloads instead of checkout.",
    ),
    (
        "What changed in the last deploy?",
        "Only the export queue worker changed. The deploy moved large CSV jobs onto the shared retry policy, which made each failed attempt hold a worker slot longer than before.\n\nThe app deploy did not include checkout, pricing, or billing API changes.",
    ),
    (
        "Do we need to roll back?",
        "Not yet. Queue depth is recovering after we reduced retry concurrency, and the oldest pending job is now under five minutes old.\n\nKeep rollback ready if the queue starts climbing again, but the current trend points toward recovery.",
    ),
    (
        "Keep watching for customer-visible issues.",
        "I will watch the queue and support tags for another 15 minutes. I am tracking export failures, delayed download requests, and any support thread that mentions missing reports.\n\nIf those stay quiet through the next batch window, we can close this as an internal degradation.",
    ),
];

const INITIAL_VISIBLE_COUNT: usize = 5;

#[component]
fn MessageScrollerLoadHistory() -> Element {
    rsx! {
        ToastProvider {
            LoadHistory {}
            Toaster {}
        }
    }
}

#[component]
fn LoadHistory() -> Element {
    let toast = toast();
    let history = use_hook(|| script_messages("history", HISTORY_SCRIPT, usize::MAX));
    let mut demo_key = use_signal(|| 0u32);
    let mut visible_count = use_signal(|| INITIAL_VISIBLE_COUNT);
    let count = visible_count();
    let visible: Vec<ChatMessage> = history[history.len().saturating_sub(count)..].to_vec();
    let can_load_history = count < history.len();
    let total = history.len();
    let key = demo_key();

    rsx! {
        MessageScrollerProvider {
            div { class: "relative flex flex-col gap-4",
                Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                    CardHeader { class: "gap-1 border-b",
                        CardTitle { "Load History" }
                        CardDescription { "Prepended messages keep your place." }
                        CardAction {
                            ResetButton {
                                label: "Reset loaded messages",
                                disabled: count == INITIAL_VISIBLE_COUNT,
                                onclick: move |_| {
                                    visible_count.set(INITIAL_VISIBLE_COUNT);
                                    demo_key += 1;
                                },
                            }
                        }
                    }
                    CardContent { class: "flex-1 overflow-hidden p-0",
                        for key in [key] {
                            MessageScroller { key: "{key}",
                                MessageScrollerViewport {
                                    MessageScrollerContent { class: "p-(--card-spacing)",
                                        for message in visible.iter().cloned() {
                                            MessageScrollerItem { key: "{message.id}", message_id: message.id.clone(),
                                                TranscriptMessage { message }
                                            }
                                        }
                                        MessageScrollerItem { scroll_anchor: false,
                                            Marker { variant: MarkerVariant::Separator,
                                                MarkerContent { "End of Conversation" }
                                            }
                                        }
                                    }
                                }
                                MessageScrollerButton {}
                            }
                        }
                    }
                    CardFooter { class: "flex flex-col items-center gap-2 border-t",
                        Button {
                            r#type: "button",
                            disabled: !can_load_history,
                            onclick: move |_| {
                                visible_count.set(total);
                                toast.add(ToastOptions {
                                    description: Some("Scroll up to see earlier messages.".into()),
                                    ..ToastOptions::new("History loaded")
                                });
                            },
                            class: "w-full",
                            variant: ButtonVariant::Secondary,
                            if can_load_history { "Load History" } else { "History Loaded" }
                        }
                        p { class: "text-xs text-muted-foreground",
                            "Restore earlier messages while keeping your place."
                        }
                    }
                }
                div { class: "mx-auto max-w-sm px-0.5 text-center text-xs text-balance text-muted-foreground",
                    "Click Load History to load the entire conversation"
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-animation

const ANIMATION_SCRIPT: Script = &[
    (
        "Can user messages pop in like iMessage without breaking anchoring?",
        "Yes. Animate the user row with transform and opacity, and let the assistant response stream normally below it.\n\nThat keeps the row measurement predictable while still giving the newly sent bubble a more tactile entrance.",
    ),
    (
        "What makes the animation feel more like iMessage?",
        "Use a quick spring from the trailing edge: a little scale, a small upward move, and no layout animation.\n\nThe bubble feels tactile, but the measured row stays predictable, so anchoring and auto-scroll do not have to fight a changing layout.",
    ),
    (
        "Can I switch between presets while testing the same thread?",
        "Yes. Keep the conversation in place while you change the preset, then send the next message to compare the new entrance against the same context.\n\nThat makes it easier to judge the difference between a subtle fade, a snappy pop, and a more dramatic 3D tilt without rebuilding the scenario each time.",
    ),
];

#[component]
fn MessageScrollerAnimation() -> Element {
    let chat = use_scripted_chat("animation", ANIMATION_SCRIPT, 0, 1000, 15);
    let mut preset_id = use_signal(|| "fade".to_string());
    let busy = chat.is_busy();
    let next = chat.next_text();
    let preset = preset_by_id(&preset_id.read());
    let messages = chat.messages.read().clone();

    rsx! {
        div { class: "relative flex flex-col gap-4",
            Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                CardHeader { class: "border-b",
                    CardTitle { "Animation" }
                    CardDescription {
                        "Choose how user messages are animated when they are added to the conversation."
                    }
                    CardAction { class: "flex items-center gap-2",
                        Button {
                            r#type: "button",
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Icon,
                            aria_label: "Reset animated messages",
                            disabled: messages.is_empty() || busy,
                            onclick: move |_| chat.reset(),
                            RotateCw {}
                        }
                    }
                }
                CardContent { class: "min-h-0 flex-1 overflow-hidden p-0",
                    if messages.is_empty() {
                        Empty { class: "h-full",
                            EmptyHeader {
                                EmptyMedia { variant: EmptyMediaVariant::Icon, MessageCircleDashed {} }
                                EmptyTitle { "No Messages Yet" }
                                EmptyDescription { "Click the button below to send the first message." }
                            }
                        }
                    } else {
                        MessageScrollerProvider {
                            MessageScroller {
                                MessageScrollerViewport {
                                    MessageScrollerContent { aria_busy: busy, class: "p-(--card-spacing)",
                                        for message in messages {
                                            MessageAnimated {
                                                key: "{message.id}",
                                                animation_preset: preset,
                                                user_variant: BubbleVariant::Muted,
                                                assistant_variant: BubbleVariant::Ghost,
                                                message,
                                            }
                                        }
                                    }
                                }
                                MessageScrollerButton {}
                            }
                        }
                    }
                }
                CardFooter { class: "border-t",
                    Select {
                        value: preset.id.to_string(),
                        on_value_change: move |value: Option<String>| {
                            if let Some(value) = value {
                                preset_id.set(value);
                            }
                        },
                        SelectTrigger { aria_label: "Animation preset",
                            SelectValue {}
                        }
                        SelectContent {
                            SelectGroup {
                                for animation in MESSAGE_ANIMATIONS.iter() {
                                    SelectItem {
                                        key: "{animation.id}",
                                        value: animation.id,
                                        label: animation.name,
                                        "{animation.name}"
                                    }
                                }
                            }
                        }
                    }
                    Button {
                        r#type: "button",
                        size: ButtonSize::Icon,
                        class: "ml-auto",
                        disabled: next.is_none() || busy,
                        onclick: move |_| chat.send(),
                        ArrowUp {}
                        span { class: "sr-only", "Send Message" }
                    }
                }
            }
            div { class: "mx-auto max-w-sm px-0.5 text-center text-xs text-balance text-muted-foreground",
                "Select an animation then click send to see it in action."
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-commands

const COMMAND_MESSAGES: &[(&str, Role, &str)] = &[
    (
        "command-activation",
        Role::User,
        "We're seeing activation dip after workspace creation. Can you help me find the likely step?",
    ),
    (
        "command-activation-reply",
        Role::Assistant,
        "The sharpest drop is between creating the workspace and inviting the first teammate.\n\nWorkspace creation is still healthy, but the invite step is where users pause. That suggests the product is asking for collaboration before the user has enough confidence in the workspace.",
    ),
    (
        "command-compare",
        Role::User,
        "What should I compare before we change the onboarding flow?",
    ),
    (
        "command-compare-reply",
        Role::Assistant,
        "Compare three cohorts:\n\n1. Users who choose a template before inviting teammates.\n2. Users who start from a blank workspace.\n3. Users who skip invites and return within 24 hours.\n\nIf template users invite faster, the fix is probably better first-run guidance rather than a louder invite prompt.",
    ),
    (
        "command-experiment",
        Role::User,
        "Can you turn that into an experiment?",
    ),
    (
        "command-experiment-reply",
        Role::Assistant,
        "Yes. Create a variant that shows a short checklist after workspace creation:\n\n- Pick a template.\n- Add one project detail.\n- Invite a teammate when the workspace has context.\n\nMeasure first invite completion, 24-hour return rate, and whether teams create a second project.",
    ),
    (
        "command-risk",
        Role::User,
        "What's the risk if we delay the invite prompt?",
    ),
    (
        "command-risk-reply",
        Role::Assistant,
        "The main risk is reducing team creation for accounts that already know who they want to invite.\n\nTo protect that path, keep the invite action visible in the header and only change the primary empty-state guidance. That gives confident teams a direct route without forcing uncertain users through the invite step too early.",
    ),
];

#[component]
fn StaticTranscript(messages: &'static [(&'static str, Role, &'static str)]) -> Element {
    rsx! {
        for (id, role, text) in messages.iter().copied() {
            MessageScrollerItem { key: "{id}", message_id: id, scroll_anchor: role == Role::User,
                TranscriptMessage { message: ChatMessage::new(id, role, text) }
            }
        }
    }
}

#[component]
fn MessageScrollerCommands() -> Element {
    rsx! {
        MessageScrollerProvider { default_scroll_position: MessageScrollerDefaultScrollPosition::End,
            div { class: "relative flex flex-col gap-4",
                Card { class: "mx-auto h-140 w-full max-w-sm gap-0",
                    CardHeader { class: "gap-1 border-b",
                        CardTitle { "Commands" }
                        CardDescription { "Drive the transcript from outside." }
                        CardAction {
                            CommandMenu {}
                        }
                    }
                    CardContent { class: "flex-1 overflow-hidden p-0",
                        MessageScroller {
                            MessageScrollerViewport {
                                MessageScrollerContent { class: "p-(--card-spacing)",
                                    StaticTranscript { messages: COMMAND_MESSAGES }
                                }
                            }
                            MessageScrollerButton {}
                        }
                    }
                }
                div { class: "mx-auto max-w-sm px-0.5 text-center text-xs text-balance text-muted-foreground",
                    "Use the controls to jump to any message in the conversation."
                }
            }
        }
    }
}

#[component]
fn CommandMenu() -> Element {
    let scroller = use_message_scroller();
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { r#type: "button", variant: ButtonVariant::Secondary, "Jump to..." }
            DropdownMenuContent { align: DropdownMenuAlign::End, class: "w-64",
                DropdownMenuGroup {
                    DropdownMenuLabel { "Conversations" }
                    for (id, _, text) in COMMAND_MESSAGES.iter().copied().filter(|(_, role, _)| *role == Role::User) {
                        DropdownMenuItem {
                            key: "{id}",
                            onclick: move |_| {
                                scroller.scroll_to_message(id, MessageScrollerScrollOptions {
                                    align: Some(MessageScrollerScrollAlign::Start),
                                    behavior: Some(MessageScrollerScrollBehavior::Smooth),
                                    scroll_margin: None,
                                });
                            },
                            span { class: "line-clamp-1 min-w-0", {trimmed_text(text)} }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-visibility

const VISIBILITY_MESSAGES: &[(&str, Role, &str)] = &[
    (
        "vis-brief",
        Role::User,
        "Review the incident handoff and tell me what to read first.",
    ),
    (
        "vis-brief-reply",
        Role::Assistant,
        "Start with the summary and the impact section. The regression affected the upload queue, but the recovery path completed for every queued job.",
    ),
    ("vis-impact", Role::User, "What was the customer impact?"),
    (
        "vis-impact-reply",
        Role::Assistant,
        "Impact was limited to delayed processing.\n\nNo records were dropped, and the reconciliation worker confirmed each retry batch. Support saw confusion from two customers, but there were no checkout or billing errors.",
    ),
    ("vis-actions", Role::User, "What actions are open?"),
    (
        "vis-actions-reply",
        Role::Assistant,
        "Keep the retry window enabled until the next deploy, then add a queue-depth alert as the long-term fix.\n\nThe alert should fire on sustained queue growth, not a single short spike.",
    ),
    (
        "vis-checklist",
        Role::User,
        "Give me the follow-up checklist.",
    ),
    (
        "vis-checklist-reply",
        Role::Assistant,
        "After that, compare the queue recovery graph with the deploy timeline so the handoff shows exactly when processing returned to baseline. That makes it easier for support and engineering to answer the same customer questions without re-reading the whole incident thread.\n\nI would also add a short owner note beside each follow-up item. The checklist is small, but ownership keeps the retry-window decision, alert tuning, and support macro from drifting into separate follow-up conversations.\n\nKeep the retry window enabled until the next deploy, then add a queue-depth alert as the long-term fix.\n\nThe alert should fire on sustained queue growth, not a single short spike.",
    ),
];

#[component]
fn MessageScrollerVisibility() -> Element {
    rsx! {
        MessageScrollerProvider { scroll_margin: 12.0,
            div { class: "relative flex flex-col gap-4",
                div { class: "relative mx-auto w-full max-w-sm",
                    Card { class: "h-140 w-full gap-0",
                        CardHeader { class: "gap-1 border-b",
                            CardTitle { "Transcript Outline" }
                            CardDescription { "Track the current anchored turn." }
                        }
                        CardContent { class: "flex-1 overflow-hidden p-0",
                            MessageScroller {
                                MessageScrollerViewport {
                                    MessageScrollerContent { class: "p-(--card-spacing)",
                                        StaticTranscript { messages: VISIBILITY_MESSAGES }
                                    }
                                }
                                MessageScrollerButton {}
                            }
                        }
                    }
                    div { class: "absolute top-1/2 -right-12 -translate-y-1/2",
                        TranscriptOutline {}
                    }
                }
                div { class: "mx-auto max-w-sm px-0.5 text-center text-xs text-muted-foreground",
                    "Open the outline to jump between anchored turns as you read."
                }
            }
        }
    }
}

#[component]
fn TranscriptOutline() -> Element {
    let scroller = use_message_scroller();
    let visibility = use_message_scroller_visibility();
    let current = visibility.current_anchor_id;
    let user_messages: Vec<(&str, &str)> = VISIBILITY_MESSAGES
        .iter()
        .filter(|(_, role, _)| *role == Role::User)
        .map(|(id, _, text)| (*id, *text))
        .collect();

    rsx! {
        HoverCard {
            HoverCardTrigger {
                role: "button",
                aria_label: "Open transcript outline",
                class: "flex h-9 w-9 flex-col items-center justify-center gap-1 rounded-md transition-colors outline-none focus-visible:ring-3 focus-visible:ring-ring/50",
                for (id, _) in user_messages.iter().copied() {
                    span {
                        key: "{id}",
                        "data-current": if current.as_deref() == Some(id) { "true" } else { "false" },
                        class: "h-0.5 w-4 rounded-full bg-muted-foreground/40 data-[current=true]:bg-foreground",
                    }
                }
            }
            HoverCardContent {
                align: HoverCardAlign::Center,
                side: HoverCardSide::Left,
                class: "flex w-64 flex-col gap-1 rounded-2xl p-1",
                for (id, text) in user_messages.iter().copied() {
                    button {
                        key: "{id}",
                        r#type: "button",
                        aria_current: if current.as_deref() == Some(id) { Some("location") } else { None },
                        class: "flex min-h-7 items-center rounded-xl px-2 py-1.5 text-left text-sm transition-colors outline-none hover:bg-accent hover:text-accent-foreground focus-visible:bg-accent focus-visible:text-accent-foreground aria-current:bg-accent aria-current:text-accent-foreground",
                        onclick: move |_| {
                            scroller.scroll_to_message(id, MessageScrollerScrollOptions {
                                align: Some(MessageScrollerScrollAlign::Start),
                                behavior: Some(MessageScrollerScrollBehavior::Smooth),
                                scroll_margin: None,
                            });
                        },
                        span { class: "line-clamp-1 min-w-0", {trimmed_text(text)} }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// message-scroller-scrollable

const SCROLLABLE_TEXT: &str = "is synced. The scrollable hook updates as the viewport moves.\n\nWhen the reader is at the first message, the footer should only point them down. Once they move into the middle of the transcript, it should explain that both directions are available.\n\nAt the latest message, the footer should switch again and only point them back up.";

fn scrollable_messages() -> Vec<ChatMessage> {
    (0..12)
        .map(|index| {
            let id = format!("scrollable-{}", index + 1);
            if index % 2 == 0 {
                ChatMessage::new(
                    id,
                    Role::User,
                    format!("Review scroll checkpoint {}.", index + 1),
                )
            } else {
                ChatMessage::new(
                    id,
                    Role::Assistant,
                    format!("Checkpoint {} {SCROLLABLE_TEXT}", index + 1),
                )
            }
        })
        .collect()
}

#[component]
fn MessageScrollerScrollableExample() -> Element {
    rsx! {
        div { class: "mx-auto flex w-full max-w-sm flex-col gap-4",
            Card { class: "h-140 w-full gap-0 overflow-hidden",
                CardHeader { class: "gap-1 border-b",
                    CardTitle { "Scroll Status" }
                    CardDescription { "Where the reader can go scroll to based on current scroll position." }
                }
                MessageScrollerProvider { default_scroll_position: MessageScrollerDefaultScrollPosition::Start,
                    CardContent { class: "flex-1 overflow-hidden p-0",
                        MessageScroller {
                            MessageScrollerViewport {
                                MessageScrollerContent { class: "gap-4 p-(--card-spacing)",
                                    for message in scrollable_messages() {
                                        MessageAnimated {
                                            key: "{message.id}",
                                            scroll_anchor: message.is_user(),
                                            user_variant: BubbleVariant::Muted,
                                            assistant_variant: BubbleVariant::Ghost,
                                            message,
                                        }
                                    }
                                }
                            }
                            MessageScrollerButton {}
                        }
                    }
                    ScrollStateFooter {}
                }
            }
            div { class: "px-0.5 text-center text-xs text-muted-foreground",
                "Scroll the transcript to see the footer update."
            }
        }
    }
}

#[component]
fn ScrollStateFooter() -> Element {
    let scrollable = use_message_scroller_scrollable();
    rsx! {
        CardFooter { class: "justify-center border-t text-center text-sm text-muted-foreground",
            {scroll_status(scrollable)}
        }
    }
}

fn scroll_status(
    MessageScrollerScrollable { start, end }: MessageScrollerScrollable,
) -> &'static str {
    match (start, end) {
        (true, true) => "You can scroll both ways.",
        (false, true) => "You are at the top. You can only scroll down.",
        (true, false) => "You are at the bottom. You can only scroll up.",
        (false, false) => "All messages fit in the viewport.",
    }
}
