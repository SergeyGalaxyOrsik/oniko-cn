//! Confirmation: asks the user to approve or reject a tool call before it runs, then shows the answer.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::item::{ItemSize, ItemVariant, item_variants};

/// Lifecycle of a tool call that may need approval. Mirrors the AI SDK tool part states.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConfirmationState {
    InputStreaming,
    InputAvailable,
    #[default]
    ApprovalRequested,
    ApprovalResponded,
    OutputAvailable,
    OutputDenied,
    OutputError,
}

impl ConfirmationState {
    /// Value of the `data-state` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InputStreaming => "input-streaming",
            Self::InputAvailable => "input-available",
            Self::ApprovalRequested => "approval-requested",
            Self::ApprovalResponded => "approval-responded",
            Self::OutputAvailable => "output-available",
            Self::OutputDenied => "output-denied",
            Self::OutputError => "output-error",
        }
    }
}

/// The approval request. `id` is passed back when the user answers; `approved` is `None` until they do.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfirmationApproval {
    pub id: String,
    pub approved: Option<bool>,
}

#[derive(Clone, Copy)]
struct ConfirmationContext {
    approval: Signal<Option<ConfirmationApproval>>,
    state: Signal<ConfirmationState>,
}

impl ConfirmationContext {
    fn is_requested(self) -> bool {
        (self.state)() == ConfirmationState::ApprovalRequested
    }

    fn approved(self) -> Option<bool> {
        (self.approval)().and_then(|approval| approval.approved)
    }
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

#[component]
pub fn Confirmation(
    approval: Option<ConfirmationApproval>,
    #[props(default)] state: ConfirmationState,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = item_variants(
        ItemVariant::Outline,
        ItemSize::Sm,
        take_class(&mut attributes).as_deref(),
    );
    let approval = use_prop(approval);
    let state = use_prop(state);
    use_context_provider(|| ConfirmationContext { approval, state });

    rsx! {
        div {
            "data-slot": "confirmation",
            "data-state": state().as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ConfirmationTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex min-w-0 flex-1 flex-col gap-1 text-sm font-medium",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "confirmation-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Shown only while the user has to answer.
#[component]
pub fn ConfirmationRequest(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<ConfirmationContext>();
    if !context.is_requested() {
        return rsx! {};
    }
    let class = cn!(
        "flex min-w-0 flex-1 flex-col gap-1 text-sm",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "confirmation-request",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Shown only after the user approved the request.
#[component]
pub fn ConfirmationAccepted(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<ConfirmationContext>();
    if context.approved() != Some(true) {
        return rsx! {};
    }
    let class = cn!(
        "text-sm text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "confirmation-accepted",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Shown only after the user rejected the request.
#[component]
pub fn ConfirmationRejected(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<ConfirmationContext>();
    if context.approved() != Some(false) {
        return rsx! {};
    }
    let class = cn!(
        "text-sm text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "confirmation-rejected",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Holds the Approve and Deny buttons. Shown only while the user has to answer.
#[component]
pub fn ConfirmationActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let context = use_context::<ConfirmationContext>();
    if !context.is_requested() {
        return rsx! {};
    }
    let class = cn!(
        "flex shrink-0 items-center gap-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "confirmation-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ConfirmationAction(
    #[props(default)] variant: ButtonVariant,
    #[props(default = ButtonSize::Sm)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "confirmation-action",
            r#type: "button",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
