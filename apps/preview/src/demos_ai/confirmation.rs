use dioxus::prelude::*;

use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::confirmation::{
    Confirmation, ConfirmationAccepted, ConfirmationAction, ConfirmationActions,
    ConfirmationApproval, ConfirmationRejected, ConfirmationRequest, ConfirmationState,
    ConfirmationTitle,
};

#[component]
fn ConfirmationExample(
    state: ConfirmationState,
    approval: Option<ConfirmationApproval>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1",
            p { class: "font-mono text-xs text-muted-foreground", "{state.as_str()}" }
            Confirmation { approval: approval, state: state,
                ConfirmationTitle { "Run `cargo test --all`" }
                ConfirmationRequest { "The agent wants to run the full test suite." }
                ConfirmationActions {
                    ConfirmationAction { variant: ButtonVariant::Outline, "Deny" }
                    ConfirmationAction { "Approve" }
                }
                ConfirmationAccepted { "Approved. Running the tests." }
                ConfirmationRejected { "Denied. The tests were not run." }
            }
        }
    }
}

#[component]
pub fn ConfirmationDemo() -> Element {
    let mut approval = use_signal(|| None::<ConfirmationApproval>);
    let mut state = use_signal(|| ConfirmationState::ApprovalRequested);

    rsx! {
        div { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "Confirmation" }
            div { class: "flex flex-col gap-6 rounded-lg border p-6",
                div { class: "flex flex-col gap-3",
                    p { class: "text-xs text-muted-foreground", "Interactive" }
                    Confirmation { approval: approval(), state: state(),
                        ConfirmationTitle { "Delete the build directory" }
                        ConfirmationRequest { "The agent wants to run `rm -rf ./build`." }
                        ConfirmationActions {
                            ConfirmationAction {
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Sm,
                                onclick: move |_| {
                                    approval.set(Some(ConfirmationApproval {
                                        id: "call_rm_build".to_string(),
                                        approved: Some(false),
                                    }));
                                    state.set(ConfirmationState::OutputDenied);
                                },
                                "Deny"
                            }
                            ConfirmationAction {
                                onclick: move |_| {
                                    approval.set(Some(ConfirmationApproval {
                                        id: "call_rm_build".to_string(),
                                        approved: Some(true),
                                    }));
                                    state.set(ConfirmationState::OutputAvailable);
                                },
                                "Approve"
                            }
                        }
                        ConfirmationAccepted { "Approved. The build directory was removed." }
                        ConfirmationRejected { "Denied. The build directory was kept." }
                    }
                    if let Some(current) = approval() {
                        p { class: "text-xs text-muted-foreground", "Answered call {current.id}" }
                    }
                    button {
                        class: "w-fit text-xs underline",
                        r#type: "button",
                        onclick: move |_| {
                            approval.set(None);
                            state.set(ConfirmationState::ApprovalRequested);
                        },
                        "Reset"
                    }
                }
                ConfirmationExample { state: ConfirmationState::InputStreaming, approval: None }
                ConfirmationExample { state: ConfirmationState::InputAvailable, approval: None }
                ConfirmationExample {
                    state: ConfirmationState::ApprovalRequested,
                    approval: Some(ConfirmationApproval { id: "call_test".to_string(), approved: None }),
                }
                ConfirmationExample {
                    state: ConfirmationState::ApprovalResponded,
                    approval: Some(ConfirmationApproval { id: "call_test".to_string(), approved: Some(true) }),
                }
                ConfirmationExample {
                    state: ConfirmationState::OutputAvailable,
                    approval: Some(ConfirmationApproval { id: "call_test".to_string(), approved: Some(true) }),
                }
                ConfirmationExample {
                    state: ConfirmationState::OutputDenied,
                    approval: Some(ConfirmationApproval { id: "call_test".to_string(), approved: Some(false) }),
                }
                ConfirmationExample {
                    state: ConfirmationState::OutputError,
                    approval: Some(ConfirmationApproval { id: "call_test".to_string(), approved: Some(true) }),
                }
            }
        }
    }
}
