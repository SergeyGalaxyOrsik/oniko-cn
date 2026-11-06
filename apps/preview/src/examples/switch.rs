//! Switch examples, ported from shadcn/ui `examples/base/switch-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::field::{
    Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldOrientation, FieldTitle,
};
use crate::components::ui::label::Label;
use crate::components::ui::switch::{Switch, SwitchSize};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            SwitchDemo {}
            SwitchDescription {}
            SwitchChoiceCard {}
            SwitchDisabled {}
            SwitchInvalid {}
            SwitchSizes {}
            SwitchRtl {}
        }
    }
}

#[component]
fn SwitchDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex items-center space-x-2",
                Switch { id: "airplane-mode" }
                Label { r#for: "airplane-mode", "Airplane Mode" }
            }
        }
    }
}

#[component]
fn SwitchDescription() -> Element {
    rsx! {
        Example { title: "Description",
            Field { orientation: FieldOrientation::Horizontal, class: "max-w-sm",
                FieldContent {
                    FieldLabel { r#for: "switch-focus-mode", "Share across devices" }
                    FieldDescription { "Focus is shared across devices, and turns off when you leave the app." }
                }
                Switch { id: "switch-focus-mode" }
            }
        }
    }
}

#[component]
fn SwitchChoiceCard() -> Element {
    rsx! {
        Example { title: "Choice Card",
            FieldGroup { class: "w-full max-w-sm",
                FieldLabel { r#for: "switch-share",
                    Field { orientation: FieldOrientation::Horizontal,
                        FieldContent {
                            FieldTitle { "Share across devices" }
                            FieldDescription { "Focus is shared across devices, and turns off when you leave the app." }
                        }
                        Switch { id: "switch-share" }
                    }
                }
                FieldLabel { r#for: "switch-notifications",
                    Field { orientation: FieldOrientation::Horizontal,
                        FieldContent {
                            FieldTitle { "Enable notifications" }
                            FieldDescription { "Receive notifications when focus mode is enabled or disabled." }
                        }
                        Switch { id: "switch-notifications", default_checked: true }
                    }
                }
            }
        }
    }
}

#[component]
fn SwitchDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            Field { orientation: FieldOrientation::Horizontal, "data-disabled": "true", class: "w-fit",
                Switch { id: "switch-disabled-unchecked", disabled: true }
                FieldLabel { r#for: "switch-disabled-unchecked", "Disabled" }
            }
        }
    }
}

#[component]
fn SwitchInvalid() -> Element {
    rsx! {
        Example { title: "Invalid",
            Field { orientation: FieldOrientation::Horizontal, class: "max-w-sm", "data-invalid": "true",
                FieldContent {
                    FieldLabel { r#for: "switch-terms", "Accept terms and conditions" }
                    FieldDescription { "You must accept the terms and conditions to continue." }
                }
                Switch { id: "switch-terms", "aria-invalid": "true" }
            }
        }
    }
}

#[component]
fn SwitchSizes() -> Element {
    rsx! {
        Example { title: "Size",
            FieldGroup { class: "w-full max-w-[10rem]",
                Field { orientation: FieldOrientation::Horizontal,
                    Switch { id: "switch-size-sm", size: SwitchSize::Sm }
                    FieldLabel { r#for: "switch-size-sm", "Small" }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    Switch { id: "switch-size-default", size: SwitchSize::Default }
                    FieldLabel { r#for: "switch-size-default", "Default" }
                }
            }
        }
    }
}

#[component]
fn SwitchRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Field { orientation: FieldOrientation::Horizontal, class: "max-w-sm", dir: "rtl",
                FieldContent {
                    FieldLabel { r#for: "switch-focus-mode-rtl", dir: "rtl", "المشاركة عبر الأجهزة" }
                    FieldDescription { dir: "rtl",
                        "يتم مشاركة التركيز عبر الأجهزة، ويتم إيقاف تشغيله عند مغادرة التطبيق."
                    }
                }
                Switch { id: "switch-focus-mode-rtl", dir: "rtl" }
            }
        }
    }
}
