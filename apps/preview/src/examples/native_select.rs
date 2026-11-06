//! Native Select examples, ported from shadcn/ui `examples/base/native-select-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::native_select::{
    NativeSelect, NativeSelectOptGroup, NativeSelectOption,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        NativeSelectDemo {}
        NativeSelectGroups {}
        NativeSelectDisabled {}
        NativeSelectInvalid {}
        NativeSelectRtl {}
    }
}

#[component]
fn NativeSelectDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            NativeSelect {
                NativeSelectOption { value: "", "Select status" }
                NativeSelectOption { value: "todo", "Todo" }
                NativeSelectOption { value: "in-progress", "In Progress" }
                NativeSelectOption { value: "done", "Done" }
                NativeSelectOption { value: "cancelled", "Cancelled" }
            }
        }
    }
}

#[component]
fn NativeSelectGroups() -> Element {
    rsx! {
        Example { title: "Groups",
            NativeSelect {
                NativeSelectOption { value: "", "Select department" }
                NativeSelectOptGroup { label: "Engineering",
                    NativeSelectOption { value: "frontend", "Frontend" }
                    NativeSelectOption { value: "backend", "Backend" }
                    NativeSelectOption { value: "devops", "DevOps" }
                }
                NativeSelectOptGroup { label: "Sales",
                    NativeSelectOption { value: "sales-rep", "Sales Rep" }
                    NativeSelectOption { value: "account-manager", "Account Manager" }
                    NativeSelectOption { value: "sales-director", "Sales Director" }
                }
                NativeSelectOptGroup { label: "Operations",
                    NativeSelectOption { value: "support", "Customer Support" }
                    NativeSelectOption { value: "product-manager", "Product Manager" }
                    NativeSelectOption { value: "ops-manager", "Operations Manager" }
                }
            }
        }
    }
}

#[component]
fn NativeSelectDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            NativeSelect { disabled: true,
                NativeSelectOption { value: "", "Disabled" }
                NativeSelectOption { value: "apple", "Apple" }
                NativeSelectOption { value: "banana", "Banana" }
                NativeSelectOption { value: "blueberry", "Blueberry" }
            }
        }
    }
}

#[component]
fn NativeSelectInvalid() -> Element {
    rsx! {
        Example { title: "Invalid",
            NativeSelect { "aria-invalid": "true",
                NativeSelectOption { value: "", "Error state" }
                NativeSelectOption { value: "apple", "Apple" }
                NativeSelectOption { value: "banana", "Banana" }
                NativeSelectOption { value: "blueberry", "Blueberry" }
            }
        }
    }
}

#[component]
fn NativeSelectRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            NativeSelect { dir: "rtl",
                NativeSelectOption { value: "", "اختر الحالة" }
                NativeSelectOption { value: "todo", "مهام" }
                NativeSelectOption { value: "in-progress", "قيد التنفيذ" }
                NativeSelectOption { value: "done", "منجز" }
                NativeSelectOption { value: "cancelled", "ملغي" }
            }
        }
    }
}
