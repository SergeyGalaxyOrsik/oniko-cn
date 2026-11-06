//! Field examples, ported from shadcn/ui `examples/base/field-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::field::{
    Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldLegendVariant,
    FieldOrientation, FieldSeparator, FieldSet, FieldTitle,
};
use crate::components::ui::input::Input;
use crate::components::ui::radio_group::{RadioGroup, RadioGroupItem};
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,
};
use crate::components::ui::slider::Slider;
use crate::components::ui::switch::Switch;
use crate::components::ui::textarea::Textarea;

#[component]
pub fn Examples() -> Element {
    rsx! {
        FieldDemo {}
        FieldInput {}
        FieldTextarea {}
        FieldSelect {}
        FieldSlider {}
        FieldFieldset {}
        FieldCheckbox {}
        FieldRadio {}
        FieldSwitch {}
        FieldChoiceCard {}
        FieldGroupExample {}
        FieldRtl {}
        FieldResponsive {}
    }
}

const MONTHS: [&str; 12] = [
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
];
const YEARS: [&str; 6] = ["2024", "2025", "2026", "2027", "2028", "2029"];
const ARABIC_MONTHS: [&str; 12] = [
    "٠١", "٠٢", "٠٣", "٠٤", "٠٥", "٠٦", "٠٧", "٠٨", "٠٩", "١٠", "١١", "١٢",
];

#[component]
fn FieldDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-[800px] p-6 md:h-[850px]",
            div { class: "w-full max-w-md",
                form {
                    FieldGroup {
                        FieldSet {
                            FieldLegend { "Payment Method" }
                            FieldDescription { "All transactions are secure and encrypted" }
                            FieldGroup {
                                Field {
                                    FieldLabel { r#for: "checkout-7j9-card-name-43j", "Name on Card" }
                                    Input {
                                        id: "checkout-7j9-card-name-43j",
                                        placeholder: "Evil Rabbit",
                                        required: true,
                                    }
                                }
                                Field {
                                    FieldLabel { r#for: "checkout-7j9-card-number-uw1", "Card Number" }
                                    Input {
                                        id: "checkout-7j9-card-number-uw1",
                                        placeholder: "1234 5678 9012 3456",
                                        required: true,
                                    }
                                    FieldDescription { "Enter your 16-digit card number" }
                                }
                                div { class: "grid grid-cols-3 gap-4",
                                    Field {
                                        FieldLabel { r#for: "checkout-exp-month-ts6", "Month" }
                                        Select {
                                            SelectTrigger { id: "checkout-exp-month-ts6",
                                                SelectValue { placeholder: "MM" }
                                            }
                                            SelectContent {
                                                SelectGroup {
                                                    for month in MONTHS {
                                                        SelectItem {
                                                            key: "{month}",
                                                            value: month,
                                                            label: month,
                                                            "{month}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Field {
                                        FieldLabel { r#for: "checkout-7j9-exp-year-f59", "Year" }
                                        Select {
                                            SelectTrigger { id: "checkout-7j9-exp-year-f59",
                                                SelectValue { placeholder: "YYYY" }
                                            }
                                            SelectContent {
                                                SelectGroup {
                                                    for year in YEARS {
                                                        SelectItem {
                                                            key: "{year}",
                                                            value: year,
                                                            label: year,
                                                            "{year}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Field {
                                        FieldLabel { r#for: "checkout-7j9-cvv", "CVV" }
                                        Input { id: "checkout-7j9-cvv", placeholder: "123", required: true }
                                    }
                                }
                            }
                        }
                        FieldSeparator {}
                        FieldSet {
                            FieldLegend { "Billing Address" }
                            FieldDescription { "The billing address associated with your payment method" }
                            FieldGroup {
                                Field { orientation: FieldOrientation::Horizontal,
                                    Checkbox { id: "checkout-7j9-same-as-shipping-wgm", default_checked: true }
                                    FieldLabel {
                                        r#for: "checkout-7j9-same-as-shipping-wgm",
                                        class: "font-normal",
                                        "Same as shipping address"
                                    }
                                }
                            }
                        }
                        FieldSet {
                            FieldGroup {
                                Field {
                                    FieldLabel { r#for: "checkout-7j9-optional-comments", "Comments" }
                                    Textarea {
                                        id: "checkout-7j9-optional-comments",
                                        placeholder: "Add any additional comments",
                                        class: "resize-none",
                                    }
                                }
                            }
                        }
                        Field { orientation: FieldOrientation::Horizontal,
                            Button { r#type: "submit", "Submit" }
                            Button { variant: ButtonVariant::Outline, r#type: "button", "Cancel" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldInput() -> Element {
    rsx! {
        Example { title: "Input",
            FieldSet { class: "w-full max-w-xs",
                FieldGroup {
                    Field {
                        FieldLabel { r#for: "username", "Username" }
                        Input { id: "username", r#type: "text", placeholder: "Max Leiter" }
                        FieldDescription { "Choose a unique username for your account." }
                    }
                    Field {
                        FieldLabel { r#for: "password", "Password" }
                        FieldDescription { "Must be at least 8 characters long." }
                        Input { id: "password", r#type: "password", placeholder: "••••••••" }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldTextarea() -> Element {
    rsx! {
        Example { title: "Textarea",
            FieldSet { class: "w-full max-w-xs",
                FieldGroup {
                    Field {
                        FieldLabel { r#for: "feedback", "Feedback" }
                        Textarea {
                            id: "feedback",
                            placeholder: "Your feedback helps us improve...",
                            rows: "4",
                        }
                        FieldDescription { "Share your thoughts about our service." }
                    }
                }
            }
        }
    }
}

const DEPARTMENTS: [(&str, &str); 8] = [
    ("Engineering", "engineering"),
    ("Design", "design"),
    ("Marketing", "marketing"),
    ("Sales", "sales"),
    ("Customer Support", "support"),
    ("Human Resources", "hr"),
    ("Finance", "finance"),
    ("Operations", "operations"),
];

#[component]
fn FieldSelect() -> Element {
    rsx! {
        Example { title: "Select",
            Field { class: "w-full max-w-xs",
                FieldLabel { "Department" }
                Select {
                    SelectTrigger {
                        SelectValue { placeholder: "Choose department" }
                    }
                    SelectContent {
                        SelectGroup {
                            for (label , value) in DEPARTMENTS {
                                SelectItem { key: "{value}", value, label, "{label}" }
                            }
                        }
                    }
                }
                FieldDescription { "Select your department or area of work." }
            }
        }
    }
}

#[component]
fn FieldSlider() -> Element {
    let mut value = use_signal(|| vec![200.0, 800.0]);
    let low = value().first().copied().unwrap_or_default();
    let high = value().get(1).copied().unwrap_or_default();

    rsx! {
        Example { title: "Slider",
            Field { class: "w-full max-w-xs",
                FieldTitle { "Price Range" }
                FieldDescription {
                    "Set your budget range ($"
                    span { class: "font-medium tabular-nums", "{low}" }
                    " - "
                    span { class: "font-medium tabular-nums", "{high}" }
                    ")."
                }
                Slider {
                    value: value(),
                    on_value_change: move |next| value.set(next),
                    max: 1000.0,
                    min: 0.0,
                    step: 10.0,
                    class: "mt-2 w-full",
                    "aria-label": "Price Range",
                }
            }
        }
    }
}

#[component]
fn FieldFieldset() -> Element {
    rsx! {
        Example { title: "Fieldset",
            FieldSet { class: "w-full max-w-sm",
                FieldLegend { "Address Information" }
                FieldDescription { "We need your address to deliver your order." }
                FieldGroup {
                    Field {
                        FieldLabel { r#for: "street", "Street Address" }
                        Input { id: "street", r#type: "text", placeholder: "123 Main St" }
                    }
                    div { class: "grid grid-cols-2 gap-4",
                        Field {
                            FieldLabel { r#for: "city", "City" }
                            Input { id: "city", r#type: "text", placeholder: "New York" }
                        }
                        Field {
                            FieldLabel { r#for: "zip", "Postal Code" }
                            Input { id: "zip", r#type: "text", placeholder: "90502" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldCheckbox() -> Element {
    rsx! {
        Example { title: "Checkbox", class: "h-[32rem]",
            FieldGroup { class: "w-full max-w-xs",
                FieldSet {
                    FieldLegend { variant: FieldLegendVariant::Label, "Show these items on the desktop" }
                    FieldDescription { "Select the items you want to show on the desktop." }
                    FieldGroup { class: "gap-3",
                        Field { orientation: FieldOrientation::Horizontal,
                            Checkbox { id: "finder-pref-9k2-hard-disks-ljj" }
                            FieldLabel { r#for: "finder-pref-9k2-hard-disks-ljj", class: "font-normal",
                                "Hard disks"
                            }
                        }
                        Field { orientation: FieldOrientation::Horizontal,
                            Checkbox { id: "finder-pref-9k2-external-disks-1yg" }
                            FieldLabel { r#for: "finder-pref-9k2-external-disks-1yg", class: "font-normal",
                                "External disks"
                            }
                        }
                        Field { orientation: FieldOrientation::Horizontal,
                            Checkbox { id: "finder-pref-9k2-cds-dvds-fzt" }
                            FieldLabel { r#for: "finder-pref-9k2-cds-dvds-fzt", class: "font-normal",
                                "CDs, DVDs, and iPods"
                            }
                        }
                        Field { orientation: FieldOrientation::Horizontal,
                            Checkbox { id: "finder-pref-9k2-connected-servers-6l2" }
                            FieldLabel {
                                r#for: "finder-pref-9k2-connected-servers-6l2",
                                class: "font-normal",
                                "Connected servers"
                            }
                        }
                    }
                }
                FieldSeparator {}
                Field { orientation: FieldOrientation::Horizontal,
                    Checkbox { id: "finder-pref-9k2-sync-folders-nep", default_checked: true }
                    FieldContent {
                        FieldLabel { r#for: "finder-pref-9k2-sync-folders-nep",
                            "Sync Desktop & Documents folders"
                        }
                        FieldDescription {
                            "Your Desktop & Documents folders are being synced with iCloud Drive. You can access them from other devices."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldRadio() -> Element {
    rsx! {
        Example { title: "Radio",
            FieldSet { class: "w-full max-w-xs",
                FieldLegend { variant: FieldLegendVariant::Label, "Subscription Plan" }
                FieldDescription { "Yearly and lifetime plans offer significant savings." }
                RadioGroup { default_value: "monthly",
                    Field { orientation: FieldOrientation::Horizontal,
                        RadioGroupItem { value: "monthly", id: "plan-monthly" }
                        FieldLabel { r#for: "plan-monthly", class: "font-normal",
                            "Monthly ($9.99/month)"
                        }
                    }
                    Field { orientation: FieldOrientation::Horizontal,
                        RadioGroupItem { value: "yearly", id: "plan-yearly" }
                        FieldLabel { r#for: "plan-yearly", class: "font-normal",
                            "Yearly ($99.99/year)"
                        }
                    }
                    Field { orientation: FieldOrientation::Horizontal,
                        RadioGroupItem { value: "lifetime", id: "plan-lifetime" }
                        FieldLabel { r#for: "plan-lifetime", class: "font-normal",
                            "Lifetime ($299.99)"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldSwitch() -> Element {
    rsx! {
        Example { title: "Switch",
            Field { orientation: FieldOrientation::Horizontal, class: "w-fit",
                FieldLabel { r#for: "2fa", "Multi-factor authentication" }
                Switch { id: "2fa" }
            }
        }
    }
}

#[component]
fn FieldChoiceCard() -> Element {
    rsx! {
        Example { title: "Choice Card",
            FieldGroup { class: "w-full max-w-xs",
                FieldSet {
                    FieldLegend { variant: FieldLegendVariant::Label, "Compute Environment" }
                    FieldDescription { "Select the compute environment for your cluster." }
                    RadioGroup { default_value: "kubernetes",
                        FieldLabel { r#for: "kubernetes-r2h",
                            Field { orientation: FieldOrientation::Horizontal,
                                FieldContent {
                                    FieldTitle { "Kubernetes" }
                                    FieldDescription { "Run GPU workloads on a K8s cluster." }
                                }
                                RadioGroupItem { value: "kubernetes", id: "kubernetes-r2h" }
                            }
                        }
                        FieldLabel { r#for: "vm-z4k",
                            Field { orientation: FieldOrientation::Horizontal,
                                FieldContent {
                                    FieldTitle { "Virtual Machine" }
                                    FieldDescription { "Access a cluster to run GPU workloads." }
                                }
                                RadioGroupItem { value: "vm", id: "vm-z4k" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldGroupExample() -> Element {
    rsx! {
        Example { title: "Field Group", class: "h-96",
            FieldGroup { class: "w-full max-w-xs",
                FieldSet {
                    FieldLabel { "Responses" }
                    FieldDescription {
                        "Get notified when ChatGPT responds to requests that take time, like research or image generation."
                    }
                    FieldGroup { "data-slot": "checkbox-group",
                        Field { orientation: FieldOrientation::Horizontal,
                            Checkbox { id: "push", default_checked: true, disabled: true }
                            FieldLabel { r#for: "push", class: "font-normal", "Push notifications" }
                        }
                    }
                }
                FieldSeparator {}
                FieldSet {
                    FieldLabel { "Tasks" }
                    FieldDescription {
                        "Get notified when tasks you've created have updates. "
                        a { href: "#", "Manage tasks" }
                    }
                    FieldGroup { "data-slot": "checkbox-group",
                        Field { orientation: FieldOrientation::Horizontal,
                            Checkbox { id: "push-tasks" }
                            FieldLabel { r#for: "push-tasks", class: "font-normal", "Push notifications" }
                        }
                        Field { orientation: FieldOrientation::Horizontal,
                            Checkbox { id: "email-tasks" }
                            FieldLabel { r#for: "email-tasks", class: "font-normal", "Email notifications" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-auto p-6",
            div { class: "w-full max-w-md py-6", dir: "rtl",
                form {
                    FieldGroup {
                        FieldSet {
                            FieldLegend { "طريقة الدفع" }
                            FieldDescription { "جميع المعاملات آمنة ومشفرة" }
                            FieldGroup {
                                Field {
                                    FieldLabel { r#for: "checkout-7j9-card-name-43j-rtl", "الاسم على البطاقة" }
                                    Input {
                                        id: "checkout-7j9-card-name-43j-rtl",
                                        placeholder: "Evil Rabbit",
                                        required: true,
                                    }
                                }
                                Field {
                                    FieldLabel { r#for: "checkout-7j9-card-number-uw1-rtl", "رقم البطاقة" }
                                    Input {
                                        id: "checkout-7j9-card-number-uw1-rtl",
                                        placeholder: "1234 5678 9012 3456",
                                        required: true,
                                    }
                                    FieldDescription { "أدخل رقم البطاقة المكون من 16 رقمًا" }
                                }
                                div { class: "grid grid-cols-3 gap-4",
                                    Field {
                                        FieldLabel { r#for: "checkout-exp-month-ts6-rtl", "الشهر" }
                                        Select {
                                            SelectTrigger { id: "checkout-exp-month-ts6-rtl",
                                                SelectValue { placeholder: "ش.ش" }
                                            }
                                            SelectContent { dir: "rtl",
                                                SelectGroup {
                                                    for (value , label) in MONTHS.into_iter().zip(ARABIC_MONTHS) {
                                                        SelectItem { key: "{value}", value, label, "{label}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Field {
                                        FieldLabel { r#for: "checkout-7j9-exp-year-f59-rtl", "السنة" }
                                        Select {
                                            SelectTrigger { id: "checkout-7j9-exp-year-f59-rtl",
                                                SelectValue { placeholder: "YYYY" }
                                            }
                                            SelectContent { dir: "rtl",
                                                SelectGroup {
                                                    for year in YEARS {
                                                        SelectItem {
                                                            key: "{year}",
                                                            value: year,
                                                            label: year,
                                                            "{year}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Field {
                                        FieldLabel { r#for: "checkout-7j9-cvv-rtl", "CVV" }
                                        Input { id: "checkout-7j9-cvv-rtl", placeholder: "123", required: true }
                                    }
                                }
                            }
                        }
                        FieldSeparator {}
                        FieldSet {
                            FieldLegend { "عنوان الفوترة" }
                            FieldDescription { "عنوان الفوترة المرتبط بطريقة الدفع الخاصة بك" }
                            FieldGroup {
                                Field { orientation: FieldOrientation::Horizontal,
                                    Checkbox { id: "checkout-7j9-same-as-shipping-wgm-rtl", default_checked: true }
                                    FieldLabel {
                                        r#for: "checkout-7j9-same-as-shipping-wgm-rtl",
                                        class: "font-normal",
                                        "نفس عنوان الشحن"
                                    }
                                }
                            }
                        }
                        FieldSet {
                            FieldGroup {
                                Field {
                                    FieldLabel { r#for: "checkout-7j9-optional-comments-rtl", "تعليقات" }
                                    Textarea {
                                        id: "checkout-7j9-optional-comments-rtl",
                                        placeholder: "أضف أي تعليقات إضافية",
                                        class: "resize-none",
                                    }
                                }
                            }
                        }
                        Field { orientation: FieldOrientation::Horizontal,
                            Button { r#type: "submit", "إرسال" }
                            Button { variant: ButtonVariant::Outline, r#type: "button", "إلغاء" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldResponsive() -> Element {
    rsx! {
        Example { title: "Responsive Layout", class: "h-[650px] p-6 md:h-[500px] md:p-10",
            div { class: "w-full max-w-lg",
                form {
                    FieldSet {
                        FieldLegend { "Profile" }
                        FieldDescription { "Fill in your profile information." }
                        FieldGroup {
                            Field { orientation: FieldOrientation::Responsive,
                                FieldContent {
                                    FieldLabel { r#for: "name", "Name" }
                                    FieldDescription { "Provide your full name for identification" }
                                }
                                Input { id: "name", placeholder: "Evil Rabbit", required: true }
                            }
                            Field { orientation: FieldOrientation::Responsive,
                                Button { r#type: "submit", "Submit" }
                                Button { r#type: "button", variant: ButtonVariant::Outline, "Cancel" }
                            }
                        }
                    }
                }
            }
        }
    }
}
