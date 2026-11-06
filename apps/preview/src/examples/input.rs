//! Input examples, ported from shadcn/ui `examples/base/input-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::Info;

use super::Example;
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;
use crate::components::ui::field::{
    Field, FieldDescription, FieldGroup, FieldLabel, FieldOrientation,
};
use crate::components::ui::input::Input;
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupInput, InputGroupText,
};
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        InputDemo {}
        InputBasic {}
        InputField {}
        InputFieldgroup {}
        InputDisabled {}
        InputInvalid {}
        InputFile {}
        InputInline {}
        InputGrid {}
        InputRequired {}
        InputBadge {}
        InputInputGroup {}
        InputButtonGroup {}
        InputForm {}
        InputRtl {}
    }
}

#[component]
fn InputDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "*:max-w-xs",
            Field {
                FieldLabel { r#for: "input-demo-api-key", "API Key" }
                Input { id: "input-demo-api-key", r#type: "password", placeholder: "sk-..." }
                FieldDescription { "Your API key is encrypted and stored securely." }
            }
        }
    }
}

#[component]
fn InputBasic() -> Element {
    rsx! {
        Example { title: "Basic", class: "*:max-w-xs",
            Input { placeholder: "Enter text" }
        }
    }
}

#[component]
fn InputField() -> Element {
    rsx! {
        Example { title: "Field", class: "*:max-w-xs",
            Field {
                FieldLabel { r#for: "input-field-username", "Username" }
                Input {
                    id: "input-field-username",
                    r#type: "text",
                    placeholder: "Enter your username",
                }
                FieldDescription { "Choose a unique username for your account." }
            }
        }
    }
}

#[component]
fn InputFieldgroup() -> Element {
    rsx! {
        Example { title: "Field Group", class: "*:max-w-xs",
            FieldGroup {
                Field {
                    FieldLabel { r#for: "fieldgroup-name", "Name" }
                    Input { id: "fieldgroup-name", placeholder: "Jordan Lee" }
                }
                Field {
                    FieldLabel { r#for: "fieldgroup-email", "Email" }
                    Input {
                        id: "fieldgroup-email",
                        r#type: "email",
                        placeholder: "name@example.com",
                    }
                    FieldDescription { "We'll send updates to this address." }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    Button { r#type: "reset", variant: ButtonVariant::Outline, "Reset" }
                    Button { r#type: "submit", "Submit" }
                }
            }
        }
    }
}

#[component]
fn InputDisabled() -> Element {
    rsx! {
        Example { title: "Disabled", class: "*:max-w-xs",
            Field { "data-disabled": "true",
                FieldLabel { r#for: "input-demo-disabled", "Email" }
                Input {
                    id: "input-demo-disabled",
                    r#type: "email",
                    placeholder: "Email",
                    disabled: true,
                }
                FieldDescription { "This field is currently disabled." }
            }
        }
    }
}

#[component]
fn InputInvalid() -> Element {
    rsx! {
        Example { title: "Invalid", class: "*:max-w-xs",
            Field { "data-invalid": "true",
                FieldLabel { r#for: "input-invalid", "Invalid Input" }
                Input { id: "input-invalid", placeholder: "Error", "aria-invalid": "true" }
                FieldDescription { "This field contains validation errors." }
            }
        }
    }
}

#[component]
fn InputFile() -> Element {
    rsx! {
        Example { title: "File", class: "*:max-w-xs",
            Field {
                FieldLabel { r#for: "picture", "Picture" }
                Input { id: "picture", r#type: "file" }
                FieldDescription { "Select a picture to upload." }
            }
        }
    }
}

#[component]
fn InputInline() -> Element {
    rsx! {
        Example { title: "Inline", class: "*:max-w-xs",
            Field { orientation: FieldOrientation::Horizontal,
                Input { r#type: "search", placeholder: "Search..." }
                Button { "Search" }
            }
        }
    }
}

#[component]
fn InputGrid() -> Element {
    rsx! {
        Example { title: "Grid", class: "p-6",
            FieldGroup { class: "grid max-w-sm grid-cols-2",
                Field {
                    FieldLabel { r#for: "first-name", "First Name" }
                    Input { id: "first-name", placeholder: "Jordan" }
                }
                Field {
                    FieldLabel { r#for: "last-name", "Last Name" }
                    Input { id: "last-name", placeholder: "Lee" }
                }
            }
        }
    }
}

#[component]
fn InputRequired() -> Element {
    rsx! {
        Example { title: "Required", class: "*:max-w-xs",
            Field {
                FieldLabel { r#for: "input-required",
                    "Required Field "
                    span { class: "text-destructive", "*" }
                }
                Input { id: "input-required", placeholder: "This field is required", required: true }
                FieldDescription { "This field must be filled out." }
            }
        }
    }
}

#[component]
fn InputBadge() -> Element {
    rsx! {
        Example { title: "Badge", class: "*:max-w-xs",
            Field {
                FieldLabel { r#for: "input-badge",
                    "Webhook URL "
                    Badge { variant: BadgeVariant::Secondary, class: "ml-auto", "Beta" }
                }
                Input {
                    id: "input-badge",
                    r#type: "url",
                    placeholder: "https://api.example.com/webhook",
                }
            }
        }
    }
}

#[component]
fn InputInputGroup() -> Element {
    rsx! {
        Example { title: "Input Group", class: "*:max-w-xs",
            Field {
                FieldLabel { r#for: "input-group-url", "Website URL" }
                InputGroup {
                    InputGroupInput { id: "input-group-url", placeholder: "example.com" }
                    InputGroupAddon {
                        InputGroupText { "https://" }
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Info { size: 16 }
                    }
                }
            }
        }
    }
}

#[component]
fn InputButtonGroup() -> Element {
    rsx! {
        Example { title: "Button Group", class: "*:max-w-xs",
            Field {
                FieldLabel { r#for: "input-button-group", "Search" }
                ButtonGroup {
                    Input { id: "input-button-group", placeholder: "Type to search..." }
                    Button { variant: ButtonVariant::Outline, "Search" }
                }
            }
        }
    }
}

const COUNTRIES: [(&str, &str); 3] = [
    ("United States", "us"),
    ("United Kingdom", "uk"),
    ("Canada", "ca"),
];

#[component]
fn InputForm() -> Element {
    rsx! {
        Example { title: "Form", class: "h-[32rem]",
            form { class: "w-full max-w-sm",
                FieldGroup {
                    Field {
                        FieldLabel { r#for: "form-name", "Name" }
                        Input {
                            id: "form-name",
                            r#type: "text",
                            placeholder: "Evil Rabbit",
                            required: true,
                        }
                    }
                    Field {
                        FieldLabel { r#for: "form-email", "Email" }
                        Input { id: "form-email", r#type: "email", placeholder: "john@example.com" }
                        FieldDescription { "We'll never share your email with anyone." }
                    }
                    div { class: "grid grid-cols-2 gap-4",
                        Field {
                            FieldLabel { r#for: "form-phone", "Phone" }
                            Input { id: "form-phone", r#type: "tel", placeholder: "+1 (555) 123-4567" }
                        }
                        Field {
                            FieldLabel { r#for: "form-country", "Country" }
                            Select { default_value: "us",
                                SelectTrigger { id: "form-country",
                                    SelectValue {}
                                }
                                SelectContent {
                                    SelectGroup {
                                        for (label , value) in COUNTRIES {
                                            SelectItem { key: "{value}", value, label, "{label}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Field {
                        FieldLabel { r#for: "form-address", "Address" }
                        Input { id: "form-address", r#type: "text", placeholder: "123 Main St" }
                    }
                    Field { orientation: FieldOrientation::Horizontal,
                        Button { r#type: "button", variant: ButtonVariant::Outline, "Cancel" }
                        Button { r#type: "submit", "Submit" }
                    }
                }
            }
        }
    }
}

#[component]
fn InputRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "*:max-w-xs",
            Field { dir: "rtl",
                FieldLabel { r#for: "input-rtl-api-key", "مفتاح API" }
                Input {
                    id: "input-rtl-api-key",
                    r#type: "password",
                    placeholder: "sk-...",
                    dir: "rtl",
                }
                FieldDescription { "مفتاح API الخاص بك مشفر ومخزن بأمان." }
            }
        }
    }
}
