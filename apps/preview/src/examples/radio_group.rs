//! RadioGroup examples, ported from shadcn/ui `examples/base/radio-group-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::field::{
    Field, FieldContent, FieldDescription, FieldLabel, FieldLegend, FieldLegendVariant,
    FieldOrientation, FieldSet, FieldTitle,
};
use crate::components::ui::label::Label;
use crate::components::ui::radio_group::{RadioGroup, RadioGroupItem};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            RadioGroupDemo {}
            RadioGroupDescription {}
            RadioGroupChoiceCard {}
            RadioGroupFieldset {}
            RadioGroupDisabled {}
            RadioGroupInvalid {}
            RadioGroupRtl {}
        }
    }
}

#[component]
fn RadioGroupDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            RadioGroup { default_value: "comfortable", class: "w-fit",
                div { class: "flex items-center gap-3",
                    RadioGroupItem { value: "default", id: "r1" }
                    Label { r#for: "r1", "Default" }
                }
                div { class: "flex items-center gap-3",
                    RadioGroupItem { value: "comfortable", id: "r2" }
                    Label { r#for: "r2", "Comfortable" }
                }
                div { class: "flex items-center gap-3",
                    RadioGroupItem { value: "compact", id: "r3" }
                    Label { r#for: "r3", "Compact" }
                }
            }
        }
    }
}

#[component]
fn RadioGroupDescription() -> Element {
    rsx! {
        Example { title: "Description",
            RadioGroup { default_value: "comfortable", class: "w-fit",
                Field { orientation: FieldOrientation::Horizontal,
                    RadioGroupItem { value: "default", id: "desc-r1" }
                    FieldContent {
                        FieldLabel { r#for: "desc-r1", "Default" }
                        FieldDescription { "Standard spacing for most use cases." }
                    }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    RadioGroupItem { value: "comfortable", id: "desc-r2" }
                    FieldContent {
                        FieldLabel { r#for: "desc-r2", "Comfortable" }
                        FieldDescription { "More space between elements." }
                    }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    RadioGroupItem { value: "compact", id: "desc-r3" }
                    FieldContent {
                        FieldLabel { r#for: "desc-r3", "Compact" }
                        FieldDescription { "Minimal spacing for dense layouts." }
                    }
                }
            }
        }
    }
}

#[component]
fn RadioGroupChoiceCard() -> Element {
    rsx! {
        Example { title: "Choice Card",
            RadioGroup { default_value: "plus", class: "max-w-sm",
                FieldLabel { r#for: "plus-plan",
                    Field { orientation: FieldOrientation::Horizontal,
                        FieldContent {
                            FieldTitle { "Plus" }
                            FieldDescription { "For individuals and small teams." }
                        }
                        RadioGroupItem { value: "plus", id: "plus-plan" }
                    }
                }
                FieldLabel { r#for: "pro-plan",
                    Field { orientation: FieldOrientation::Horizontal,
                        FieldContent {
                            FieldTitle { "Pro" }
                            FieldDescription { "For growing businesses." }
                        }
                        RadioGroupItem { value: "pro", id: "pro-plan" }
                    }
                }
                FieldLabel { r#for: "enterprise-plan",
                    Field { orientation: FieldOrientation::Horizontal,
                        FieldContent {
                            FieldTitle { "Enterprise" }
                            FieldDescription { "For large teams and enterprises." }
                        }
                        RadioGroupItem { value: "enterprise", id: "enterprise-plan" }
                    }
                }
            }
        }
    }
}

#[component]
fn RadioGroupFieldset() -> Element {
    rsx! {
        Example { title: "Fieldset",
            FieldSet { class: "w-full max-w-xs",
                FieldLegend { variant: FieldLegendVariant::Label, "Subscription Plan" }
                FieldDescription { "Yearly and lifetime plans offer significant savings." }
                RadioGroup { default_value: "monthly",
                    Field { orientation: FieldOrientation::Horizontal,
                        RadioGroupItem { value: "monthly", id: "plan-monthly" }
                        FieldLabel { r#for: "plan-monthly", class: "font-normal", "Monthly ($9.99/month)" }
                    }
                    Field { orientation: FieldOrientation::Horizontal,
                        RadioGroupItem { value: "yearly", id: "plan-yearly" }
                        FieldLabel { r#for: "plan-yearly", class: "font-normal", "Yearly ($99.99/year)" }
                    }
                    Field { orientation: FieldOrientation::Horizontal,
                        RadioGroupItem { value: "lifetime", id: "plan-lifetime" }
                        FieldLabel { r#for: "plan-lifetime", class: "font-normal", "Lifetime ($299.99)" }
                    }
                }
            }
        }
    }
}

#[component]
fn RadioGroupDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            RadioGroup { default_value: "option2", class: "w-fit",
                Field { orientation: FieldOrientation::Horizontal, "data-disabled": "true",
                    RadioGroupItem { value: "option1", id: "disabled-1", disabled: true }
                    FieldLabel { r#for: "disabled-1", class: "font-normal", "Disabled" }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    RadioGroupItem { value: "option2", id: "disabled-2" }
                    FieldLabel { r#for: "disabled-2", class: "font-normal", "Option 2" }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    RadioGroupItem { value: "option3", id: "disabled-3" }
                    FieldLabel { r#for: "disabled-3", class: "font-normal", "Option 3" }
                }
            }
        }
    }
}

#[component]
fn RadioGroupInvalid() -> Element {
    rsx! {
        Example { title: "Invalid",
            FieldSet { class: "w-full max-w-xs",
                FieldLegend { variant: FieldLegendVariant::Label, "Notification Preferences" }
                FieldDescription { "Choose how you want to receive notifications." }
                RadioGroup { default_value: "email",
                    Field { orientation: FieldOrientation::Horizontal, "data-invalid": "true",
                        RadioGroupItem { value: "email", id: "invalid-email", "aria-invalid": "true" }
                        FieldLabel { r#for: "invalid-email", class: "font-normal", "Email only" }
                    }
                    Field { orientation: FieldOrientation::Horizontal, "data-invalid": "true",
                        RadioGroupItem { value: "sms", id: "invalid-sms", "aria-invalid": "true" }
                        FieldLabel { r#for: "invalid-sms", class: "font-normal", "SMS only" }
                    }
                    Field { orientation: FieldOrientation::Horizontal, "data-invalid": "true",
                        RadioGroupItem { value: "both", id: "invalid-both", "aria-invalid": "true" }
                        FieldLabel { r#for: "invalid-both", class: "font-normal", "Both Email & SMS" }
                    }
                }
            }
        }
    }
}

#[component]
fn RadioGroupRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            RadioGroup { default_value: "comfortable", class: "w-fit", dir: "rtl",
                Field { orientation: FieldOrientation::Horizontal,
                    RadioGroupItem { value: "default", id: "r1-rtl", dir: "rtl" }
                    FieldContent {
                        FieldLabel { r#for: "r1-rtl", dir: "rtl", "افتراضي" }
                        FieldDescription { dir: "rtl", "تباعد قياسي لمعظم حالات الاستخدام." }
                    }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    RadioGroupItem { value: "comfortable", id: "r2-rtl", dir: "rtl" }
                    FieldContent {
                        FieldLabel { r#for: "r2-rtl", dir: "rtl", "مريح" }
                        FieldDescription { dir: "rtl", "مساحة أكبر بين العناصر." }
                    }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    RadioGroupItem { value: "compact", id: "r3-rtl", dir: "rtl" }
                    FieldContent {
                        FieldLabel { r#for: "r3-rtl", dir: "rtl", "مضغوط" }
                        FieldDescription { dir: "rtl", "تباعد أدنى للتخطيطات الكثيفة." }
                    }
                }
            }
        }
    }
}
