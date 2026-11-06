//! Toast examples, ported from shadcn/ui `examples/base/toast-*.tsx`, followed by the Sonner examples of
//! `examples/radix/sonner-*.tsx` (Sonner has no Base UI docs page of its own).

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::sonner::{
    self, Position, ToastOptions as SonnerOptions, Toaster as SonnerToaster,
};
use crate::components::ui::toast::{
    ToastActionOptions, ToastOptions, ToastPriority, ToastType, Toaster, toast,
};

async fn wait(milliseconds: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {milliseconds}))"
    ))
    .await;
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        Toaster {}
        SonnerToaster {}
        ToastDemo {}
        ToastTypes {}
        ToastPromise {}
        SonnerDemo {}
        SonnerTypes {}
        SonnerDescription {}
        SonnerPosition {}
    }
}

#[component]
fn ToastDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast()
                        .add(ToastOptions {
                            title: Some("Event created".into()),
                            description: Some("Sunday, December 3 at 9:00 AM".into()),
                            action: Some(ToastActionOptions::new("Undo", |id| toast().close(id))),
                            ..ToastOptions::default()
                        });
                },
                "Show Toast"
            }
        }
    }
}

#[component]
fn ToastTypes() -> Element {
    let add = |kind: Option<ToastType>, description: &str, priority: ToastPriority| {
        toast().add(ToastOptions {
            description: Some(description.into()),
            kind,
            priority,
            ..ToastOptions::default()
        });
    };

    rsx! {
        Example { title: "Types",
            div { class: "flex flex-wrap gap-2",
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| add(None, "Event has been created.", ToastPriority::Low),
                    "Default"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| add(
                        Some(ToastType::Success),
                        "Event has been created.",
                        ToastPriority::Low,
                    ),
                    "Success"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| add(
                        Some(ToastType::Info),
                        "Arrive 10 minutes before the event.",
                        ToastPriority::Low,
                    ),
                    "Info"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| add(
                        Some(ToastType::Warning),
                        "The event cannot start before 8:00 AM.",
                        ToastPriority::Low,
                    ),
                    "Warning"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| add(
                        Some(ToastType::Error),
                        "The event could not be created.",
                        ToastPriority::High,
                    ),
                    "Error"
                }
            }
        }
    }
}

#[component]
fn ToastPromise() -> Element {
    rsx! {
        Example { title: "Promise",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast()
                        .promise(
                            async {
                                wait(2000).await;
                                Ok::<_, ()>("Event")
                            },
                            "Creating event…",
                            |name| format!("{name} created."),
                            |_| "Could not create event.".to_string(),
                        );
                },
                "Create Event"
            }
        }
    }
}

#[component]
fn SonnerDemo() -> Element {
    rsx! {
        Example { title: "Sonner: Demo",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    sonner::toast(
                        "Event has been created",
                        SonnerOptions {
                            description: Some("Sunday, December 03, 2023 at 9:00 AM".into()),
                            action: Some(sonner::ToastAction::new("Undo", || {})),
                            ..SonnerOptions::default()
                        },
                    );
                },
                "Show Toast"
            }
        }
    }
}

#[component]
fn SonnerTypes() -> Element {
    rsx! {
        Example { title: "Sonner: Types",
            div { class: "flex flex-wrap gap-2",
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        sonner::toast("Event has been created", SonnerOptions::default());
                    },
                    "Default"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        sonner::toast_success("Event has been created", SonnerOptions::default());
                    },
                    "Success"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        sonner::toast_info(
                            "Be at the area 10 minutes before the event time",
                            SonnerOptions::default(),
                        );
                    },
                    "Info"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        sonner::toast_warning(
                            "Event start time cannot be earlier than 8am",
                            SonnerOptions::default(),
                        );
                    },
                    "Warning"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        sonner::toast_error("Event has not been created", SonnerOptions::default());
                    },
                    "Error"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        sonner::toast_promise(
                            async {
                                wait(2000).await;
                                Ok::<_, ()>("Event")
                            },
                            "Loading...",
                            |name| format!("{name} has been created"),
                            |_| "Error".to_string(),
                        );
                    },
                    "Promise"
                }
            }
        }
    }
}

#[component]
fn SonnerDescription() -> Element {
    rsx! {
        Example { title: "Sonner: Description",
            Button {
                variant: ButtonVariant::Outline,
                class: "w-fit",
                onclick: move |_| {
                    sonner::toast(
                        "Event has been created",
                        SonnerOptions {
                            description: Some("Monday, January 3rd at 6:00pm".into()),
                            ..SonnerOptions::default()
                        },
                    );
                },
                "Show Toast"
            }
        }
    }
}

#[component]
fn SonnerPosition() -> Element {
    let positions = [
        (Position::TopLeft, "Top Left"),
        (Position::TopCenter, "Top Center"),
        (Position::TopRight, "Top Right"),
        (Position::BottomLeft, "Bottom Left"),
        (Position::BottomCenter, "Bottom Center"),
        (Position::BottomRight, "Bottom Right"),
    ];

    rsx! {
        Example { title: "Sonner: Position",
            div { class: "flex flex-wrap justify-center gap-2",
                for (position , label) in positions {
                    Button {
                        key: "{label}",
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            sonner::toast(
                                "Event has been created",
                                SonnerOptions {
                                    position: Some(position),
                                    ..SonnerOptions::default()
                                },
                            );
                        },
                        "{label}"
                    }
                }
            }
        }
    }
}
