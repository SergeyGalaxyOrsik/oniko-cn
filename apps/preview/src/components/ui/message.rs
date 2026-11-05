//! Message: a chat message row with content, action buttons and navigation between alternative branches.
//! Content is passed as children, so it can hold a `Bubble` or any other element. `MessageResponse` renders children as-is, without markdown.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

/// Who wrote a message. Anything other than `User` is styled as the assistant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageFrom {
    User,
    #[default]
    Assistant,
    System,
}

impl MessageFrom {
    /// Value of the `data-from` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::System => "system",
        }
    }
}

#[derive(Clone, Copy)]
struct MessageBranchContext {
    current: Signal<usize>,
    total: Signal<usize>,
    on_branch_change: Option<EventHandler<usize>>,
}

impl MessageBranchContext {
    fn select(mut self, index: usize) {
        self.current.set(index);
        if let Some(handler) = self.on_branch_change {
            handler.call(index);
        }
    }

    fn previous(self) {
        let total = (self.total)();
        if total < 2 {
            return;
        }
        let current = (self.current)().min(total - 1);
        self.select(if current == 0 { total - 1 } else { current - 1 });
    }

    fn next(self) {
        let total = (self.total)();
        if total < 2 {
            return;
        }
        let current = (self.current)().min(total - 1);
        self.select(if current + 1 < total { current + 1 } else { 0 });
    }
}

fn chevron(path: &'static str) -> Element {
    rsx! {
        svg {
            "aria-hidden": "true",
            class: "pointer-events-none shrink-0",
            width: "14",
            height: "14",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: path }
        }
    }
}

#[component]
pub fn Message(
    #[props(default)] from: MessageFrom,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "group/message flex w-full max-w-[95%] flex-col gap-2 data-[from=user]:ml-auto data-[from=user]:justify-end",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "message",
            "data-from": from.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MessageContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex w-fit min-w-0 max-w-full flex-col gap-2 overflow-hidden text-sm text-foreground",
        "group-data-[from=user]/message:ml-auto group-data-[from=user]/message:rounded-lg group-data-[from=user]/message:bg-secondary group-data-[from=user]/message:px-4 group-data-[from=user]/message:py-3",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "message-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Renders children as-is. Markdown is left to the application.
#[component]
pub fn MessageResponse(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "size-full wrap-break-word [&>*:first-child]:mt-0 [&>*:last-child]:mb-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "message-response",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn MessageActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center gap-1",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "message-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// An icon button for a message. `tooltip` is shown as the native `title`, because `Tooltip` is not in the registry yet.
#[component]
pub fn MessageAction(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconSm)] size: ButtonSize,
    tooltip: Option<String>,
    label: Option<String>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());
    let screen_reader_text = label.or_else(|| tooltip.clone());

    rsx! {
        button {
            "data-slot": "message-action",
            r#type: "button",
            title: tooltip,
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
            if let Some(text) = screen_reader_text {
                span { "data-slot": "message-action-label", class: "sr-only", {text} }
            }
        }
    }
}

/// Holds alternative versions of one message. Its parts share state through context.
#[component]
pub fn MessageBranch(
    #[props(default)] default_branch: usize,
    on_branch_change: Option<EventHandler<usize>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let current = use_signal(|| default_branch);
    let total = use_signal(|| 0_usize);
    use_context_provider(|| MessageBranchContext {
        current,
        total,
        on_branch_change,
    });

    let mut attributes = attributes;
    let class = cn!(
        "grid w-full gap-2 [&>div]:pb-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "message-branch",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Shows one of `branches` at a time. Pass each alternative as its own element.
#[component]
pub fn MessageBranchContent(
    branches: Vec<Element>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let context = use_context::<MessageBranchContext>();
    let current = (context.current)();
    let count = branches.len();
    let mut total = context.total;
    if *total.peek() != count {
        total.set(count);
    }

    let mut attributes = attributes;
    let class = cn!("grid w-full gap-2", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "message-branch-content",
            class: class,
            ..attributes,
            for (index, branch) in branches.into_iter().enumerate() {
                div {
                    key: "{index}",
                    "data-slot": "message-branch-item",
                    class: if index == current { "grid gap-2 overflow-hidden [&>div]:pb-0" } else { "hidden" },
                    {branch}
                }
            }
        }
    }
}

/// Wraps the branch controls. Renders nothing while there is only one branch.
#[component]
pub fn MessageBranchSelector(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let total = (use_context::<MessageBranchContext>().total)();
    if total < 2 {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!(
        "flex w-fit items-stretch [&>*:not(:first-child)]:rounded-l-md [&>*:not(:last-child)]:rounded-r-md",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "message-branch-selector",
            role: "group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Moves to the previous branch, wrapping around to the last one.
#[component]
pub fn MessageBranchPrevious(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<MessageBranchContext>();
    let total = (context.total)();
    let mut attributes = attributes;
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        take_class(&mut attributes).as_deref(),
    );

    rsx! {
        button {
            "data-slot": "message-branch-previous",
            r#type: "button",
            aria_label: "Previous branch",
            disabled: total < 2,
            class: class,
            onclick: move |event| {
                context.previous();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children.unwrap_or_else(|| chevron("m15 18-6-6 6-6"))}
        }
    }
}

/// Moves to the next branch, wrapping around to the first one.
#[component]
pub fn MessageBranchNext(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Option<Element>,
) -> Element {
    let context = use_context::<MessageBranchContext>();
    let total = (context.total)();
    let mut attributes = attributes;
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        take_class(&mut attributes).as_deref(),
    );

    rsx! {
        button {
            "data-slot": "message-branch-next",
            r#type: "button",
            aria_label: "Next branch",
            disabled: total < 2,
            class: class,
            onclick: move |event| {
                context.next();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children.unwrap_or_else(|| chevron("m9 18 6-6-6-6"))}
        }
    }
}

/// Shows the current branch position, such as "2 of 3".
#[component]
pub fn MessageBranchPage(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let context = use_context::<MessageBranchContext>();
    let total = (context.total)();
    let current = (context.current)().min(total.saturating_sub(1));
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center justify-center whitespace-nowrap text-sm text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "message-branch-page",
            class: class,
            ..attributes,
            "{current + 1} of {total}"
        }
    }
}

#[component]
pub fn MessageToolbar(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mt-4 flex w-full items-center justify-between gap-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "message-toolbar",
            class: class,
            ..attributes,
            {children}
        }
    }
}
