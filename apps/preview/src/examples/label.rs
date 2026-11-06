//! Label examples, ported from shadcn/ui `examples/base/label-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::field::{
    Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldOrientation, FieldSeparator,
    FieldSet,
};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,
};
use crate::components::ui::textarea::Textarea;

const MONTHS: [&str; 12] = [
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
];

const YEARS: [&str; 6] = ["2024", "2025", "2026", "2027", "2028", "2029"];

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            LabelDemo {}
            FieldDemo {}
            LabelRtl {}
        }
    }
}

#[component]
fn LabelDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex gap-2",
                Checkbox { id: "terms" }
                Label { r#for: "terms", "Accept terms and conditions" }
            }
        }
    }
}

#[component]
fn FieldDemo() -> Element {
    rsx! {
        Example { title: "Field", class: "h-[44rem]",
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
                                                        SelectItem { key: "{month}", value: month, "{month}" }
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
                                                        SelectItem { key: "{year}", value: year, "{year}" }
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
fn LabelRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            div { class: "flex gap-2", dir: "rtl",
                Checkbox { id: "terms-rtl", dir: "rtl" }
                Label { r#for: "terms-rtl", dir: "rtl", "قبول الشروط والأحكام" }
            }
        }
    }
}
