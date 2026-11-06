//! Drawer examples, ported from shadcn/ui `examples/base/drawer-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{
    Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger,
};
use crate::components::ui::drawer::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerModal,
    DrawerSnapPoint, DrawerSwipeDirection, DrawerTitle, DrawerTrigger,
};
use crate::components::ui::field::{
    Field, FieldContent, FieldDescription, FieldLabel, FieldOrientation, FieldTitle,
};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::radio_group::{RadioGroup, RadioGroupItem};
use crate::components::ui::toast::{ToastOptions, ToastProvider, Toaster, use_toast};

struct DeliveryTime {
    value: &'static str,
    id: &'static str,
    label: &'static str,
    description: &'static str,
    badge: Option<&'static str>,
}

const DELIVERY_TIMES: [DeliveryTime; 5] = [
    DeliveryTime {
        value: "asap",
        id: "delivery-asap",
        label: "Standard delivery",
        description: "25–35 min · Driver assigned now",
        badge: Some("Fastest"),
    },
    DeliveryTime {
        value: "5-00",
        id: "delivery-5-00",
        label: "5:00 PM – 5:15 PM",
        description: "Prep starts at 4:45 PM",
        badge: None,
    },
    DeliveryTime {
        value: "5-30",
        id: "delivery-5-30",
        label: "5:30 PM – 5:45 PM",
        description: "Good if you're heading home",
        badge: None,
    },
    DeliveryTime {
        value: "6-00",
        id: "delivery-6-00",
        label: "6:00 PM – 6:15 PM",
        description: "Most popular · High demand",
        badge: None,
    },
    DeliveryTime {
        value: "6-30",
        id: "delivery-6-30",
        label: "6:30 PM – 6:45 PM",
        description: "Last slot before kitchen closes",
        badge: None,
    },
];

/// Stands in for upstream's `useMediaQuery` and `useIsMobile` hooks. It reads the query once on mount.
fn use_media_query(query: &'static str) -> Signal<bool> {
    let mut matches = use_signal(|| false);
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(&format!(
                "dioxus.send(window.matchMedia('{query}').matches)"
            ));
            if let Ok(value) = eval.recv::<bool>().await {
                matches.set(value);
            }
        });
    });
    matches
}

fn use_is_mobile() -> Signal<bool> {
    use_media_query("(max-width: 767px)")
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            DrawerDemo {}
            DrawerWithSides {}
            DrawerSwipeHandleExample {}
            DrawerNested {}
            DrawerNonModal {}
            DrawerSnapPoints {}
            DrawerDialogDemo {}
        }
    }
}

#[component]
fn DrawerDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            ToastProvider {
                DrawerDemoInner {}
                Toaster {}
            }
        }
    }
}

#[component]
fn DrawerDemoInner() -> Element {
    let mut open = use_signal(|| false);
    let mut delivery_time = use_signal(|| "asap".to_string());
    let is_mobile = use_is_mobile();
    let toast = use_toast();

    let handle_confirm = move |_| {
        let selected = delivery_time();
        let Some(time) = DELIVERY_TIMES.iter().find(|time| time.value == selected) else {
            return;
        };
        open.set(false);
        let mut options = ToastOptions::new("Delivery time confirmed");
        options.description = Some(time.label.to_string());
        toast.show(options);
    };

    rsx! {
        Drawer {
            open: open(),
            on_open_change: move |next| open.set(next),
            show_swipe_handle: is_mobile(),
            swipe_direction: if is_mobile() { DrawerSwipeDirection::Down } else { DrawerSwipeDirection::Right },
            DrawerTrigger { variant: ButtonVariant::Secondary, "Open Drawer" }
            DrawerContent {
                DrawerHeader {
                    DrawerTitle { "Pick a delivery time" }
                    DrawerDescription { "We'll prepare your order as soon as possible." }
                }
                div { class: "flex-1 scroll-fade overflow-y-auto p-4",
                    RadioGroup {
                        value: delivery_time(),
                        on_value_change: move |next| delivery_time.set(next),
                        class: "gap-2",
                        for time in DELIVERY_TIMES.iter() {
                            FieldLabel { key: "{time.value}", r#for: time.id,
                                Field { orientation: FieldOrientation::Horizontal,
                                    FieldContent {
                                        FieldTitle { class: "flex items-center gap-2",
                                            "{time.label}"
                                            if let Some(badge) = time.badge {
                                                Badge { variant: BadgeVariant::Secondary, "{badge}" }
                                            }
                                        }
                                        FieldDescription { "{time.description}" }
                                    }
                                    RadioGroupItem { value: time.value, id: time.id }
                                }
                            }
                        }
                    }
                }
                DrawerFooter {
                    Button { onclick: handle_confirm, class: "h-[34px]", "Confirm Delivery Time" }
                    DrawerClose { variant: ButtonVariant::Outline, "Cancel" }
                }
            }
        }
    }
}

#[component]
fn DrawerWithSides() -> Element {
    rsx! {
        Example { title: "Sides",
            Drawer { swipe_direction: DrawerSwipeDirection::Left,
                DrawerTrigger { variant: ButtonVariant::Secondary, "Open Left Drawer" }
                DrawerContent {
                    DrawerHeader {
                        DrawerTitle { "Move Goal" }
                        DrawerDescription { "Set your daily activity goal." }
                    }
                    div { class: "flex-1 p-4",
                        div { class: "size-full rounded-2xl bg-muted" }
                    }
                    DrawerFooter {
                        DrawerClose { variant: ButtonVariant::Default, "Close" }
                    }
                }
            }
        }
    }
}

#[component]
fn DrawerSwipeHandleExample() -> Element {
    rsx! {
        Example { title: "Swipe Handle",
            Drawer { show_swipe_handle: true,
                DrawerTrigger { variant: ButtonVariant::Secondary, "Open Drawer" }
                DrawerContent {
                    DrawerHeader {
                        DrawerTitle { "Drawer" }
                        DrawerDescription { "Drawer with a swipe handle." }
                    }
                    div { class: "flex-1 p-4",
                        div { class: "rounded-2xl bg-muted group-data-[swipe-axis=x]/drawer-popup:size-full group-data-[swipe-axis=y]/drawer-popup:h-80 group-data-[swipe-axis=y]/drawer-popup:w-full" }
                    }
                    DrawerFooter {
                        DrawerClose { variant: ButtonVariant::Default, "Close" }
                    }
                }
            }
        }
    }
}

#[component]
fn DrawerNested() -> Element {
    let is_mobile = use_is_mobile();
    let swipe_direction = if is_mobile() {
        DrawerSwipeDirection::Down
    } else {
        DrawerSwipeDirection::Right
    };
    let panel = "bg-muted group-data-[swipe-axis=x]/drawer-popup:size-full group-data-[swipe-axis=y]/drawer-popup:aspect-video group-data-[swipe-axis=y]/drawer-popup:w-full";

    rsx! {
        Example { title: "Nested",
            Drawer { show_swipe_handle: is_mobile(), swipe_direction,
                DrawerTrigger { variant: ButtonVariant::Secondary, "Open Drawer" }
                DrawerContent {
                    DrawerHeader {
                        DrawerTitle { "Drawer" }
                        DrawerDescription { "Open another drawer from the same direction." }
                    }
                    div { class: "flex-1 p-4",
                        div { class: panel }
                    }
                    DrawerFooter {
                        Drawer { show_swipe_handle: is_mobile(), swipe_direction,
                            DrawerTrigger { variant: ButtonVariant::Outline, "Open Nested Drawer" }
                            DrawerContent {
                                DrawerHeader {
                                    DrawerTitle { "Nested Drawer" }
                                    DrawerDescription { "The parent drawer stays mounted behind this one." }
                                }
                                div { class: "flex-1 p-4",
                                    div { class: panel }
                                }
                                DrawerFooter {
                                    Drawer { show_swipe_handle: is_mobile(), swipe_direction,
                                        DrawerTrigger { variant: ButtonVariant::Outline, "Open Third Drawer" }
                                        DrawerContent {
                                            DrawerHeader {
                                                DrawerTitle { "Third Drawer" }
                                                DrawerDescription { "Two drawers are stacked behind this one." }
                                            }
                                            div { class: "flex-1 p-4",
                                                div { class: panel }
                                            }
                                            DrawerFooter {
                                                Drawer { show_swipe_handle: is_mobile(), swipe_direction,
                                                    DrawerTrigger { variant: ButtonVariant::Outline, "Open Fourth Drawer" }
                                                    DrawerContent {
                                                        DrawerHeader {
                                                            DrawerTitle { "Fourth Drawer" }
                                                            DrawerDescription { "This is the frontmost drawer in the stack." }
                                                        }
                                                        div { class: "flex-1 p-4",
                                                            div { class: panel }
                                                        }
                                                        DrawerFooter {
                                                            DrawerClose { variant: ButtonVariant::Outline, "Close" }
                                                        }
                                                    }
                                                }
                                                DrawerClose { variant: ButtonVariant::Outline, "Close" }
                                            }
                                        }
                                    }
                                    DrawerClose { variant: ButtonVariant::Outline, "Close" }
                                }
                            }
                        }
                        DrawerClose { variant: ButtonVariant::Outline, "Close" }
                    }
                }
            }
        }
    }
}

#[component]
fn DrawerNonModal() -> Element {
    rsx! {
        Example { title: "Non Modal",
            Drawer {
                modal: DrawerModal::False,
                disable_pointer_dismissal: true,
                swipe_direction: DrawerSwipeDirection::Right,
                DrawerTrigger { variant: ButtonVariant::Outline, "Non Modal" }
                DrawerContent {
                    DrawerHeader {
                        DrawerTitle { "Non Modal Drawer" }
                    }
                    div { class: "flex-1 p-4",
                        div { class: "rounded-2xl bg-muted group-data-[swipe-axis=x]/drawer-popup:size-full group-data-[swipe-axis=y]/drawer-popup:h-80 group-data-[swipe-axis=y]/drawer-popup:w-full" }
                    }
                    DrawerFooter {
                        DrawerClose { variant: ButtonVariant::Default, "Close" }
                    }
                }
            }
        }
    }
}

#[component]
fn DrawerSnapPoints() -> Element {
    rsx! {
        Example { title: "Snap Points",
            Drawer {
                snap_points: vec![DrawerSnapPoint::Rem(31.0), DrawerSnapPoint::Fraction(1.0)],
                show_swipe_handle: true,
                DrawerTrigger { variant: ButtonVariant::Outline, "Open Snap Drawer" }
                DrawerContent {
                    DrawerHeader {
                        DrawerTitle { "Snap points" }
                        DrawerDescription {
                            "Drag the drawer to snap between a compact peek and a near full-height view."
                        }
                    }
                    div { class: "flex-1 p-4",
                        div { class: "rounded-2xl bg-muted group-data-[swipe-axis=x]/drawer-popup:size-full group-data-[swipe-axis=y]/drawer-popup:h-80 group-data-[swipe-axis=y]/drawer-popup:w-full" }
                    }
                    DrawerFooter {
                        DrawerClose { variant: ButtonVariant::Default, "Close" }
                    }
                }
            }
        }
    }
}

#[component]
fn DrawerDialogDemo() -> Element {
    let mut open = use_signal(|| false);
    let is_desktop = use_media_query("(min-width: 768px)");

    rsx! {
        Example { title: "Responsive Dialog",
            if is_desktop() {
                Dialog { open: open(), on_open_change: move |next| open.set(next),
                    DialogTrigger { variant: ButtonVariant::Outline, "Edit Profile" }
                    DialogContent { class: "sm:max-w-[425px]",
                        DialogHeader {
                            DialogTitle { "Edit profile" }
                            DialogDescription { "Make changes to your profile here. Click save when you're done." }
                        }
                        ProfileForm {}
                    }
                }
            } else {
                Drawer { open: open(), on_open_change: move |next| open.set(next),
                    DrawerTrigger { variant: ButtonVariant::Outline, "Edit Profile" }
                    DrawerContent {
                        DrawerHeader { class: "text-left",
                            DrawerTitle { "Edit profile" }
                            DrawerDescription { "Make changes to your profile here. Click save when you're done." }
                        }
                        ProfileForm { class: "p-4" }
                    }
                }
            }
        }
    }
}

#[component]
fn ProfileForm(#[props(default)] class: String) -> Element {
    rsx! {
        form {
            class: "grid items-start gap-6 {class}",
            onsubmit: move |event| event.prevent_default(),
            div { class: "grid gap-3",
                Label { r#for: "email", "Email" }
                Input { r#type: "email", id: "email", value: "shadcn@example.com" }
            }
            div { class: "grid gap-3",
                Label { r#for: "username", "Username" }
                Input { id: "username", value: "@shadcn" }
            }
            Button { r#type: "submit", "Save changes" }
        }
    }
}
