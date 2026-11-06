//! Checkbox examples, ported from shadcn/ui `examples/base/checkbox-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::field::{
    Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldLegendVariant,
    FieldOrientation, FieldSet, FieldTitle,
};
use crate::components::ui::label::Label;
use crate::components::ui::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};

struct Person {
    id: &'static str,
    name: &'static str,
    email: &'static str,
    role: &'static str,
}

const TABLE_DATA: [Person; 4] = [
    Person {
        id: "1",
        name: "Sarah Chen",
        email: "sarah.chen@example.com",
        role: "Admin",
    },
    Person {
        id: "2",
        name: "Marcus Rodriguez",
        email: "marcus.rodriguez@example.com",
        role: "User",
    },
    Person {
        id: "3",
        name: "Priya Patel",
        email: "priya.patel@example.com",
        role: "User",
    },
    Person {
        id: "4",
        name: "David Kim",
        email: "david.kim@example.com",
        role: "Editor",
    },
];

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            CheckboxDemo {}
            CheckboxInvalid {}
            CheckboxBasic {}
            CheckboxDescription {}
            CheckboxDisabled {}
            CheckboxGroup {}
            CheckboxInTable {}
            CheckboxRtl {}
        }
    }
}

#[component]
fn CheckboxDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-80",
            FieldGroup { class: "max-w-sm",
                Field { orientation: FieldOrientation::Horizontal,
                    Checkbox { id: "terms-checkbox" }
                    Label { r#for: "terms-checkbox", "Accept terms and conditions" }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    Checkbox { id: "terms-checkbox-2", default_checked: true }
                    FieldContent {
                        FieldLabel { r#for: "terms-checkbox-2", "Accept terms and conditions" }
                        FieldDescription { "By clicking this checkbox, you agree to the terms." }
                    }
                }
                Field { orientation: FieldOrientation::Horizontal, "data-disabled": "true",
                    Checkbox { id: "toggle-checkbox", disabled: true }
                    FieldLabel { r#for: "toggle-checkbox", "Enable notifications" }
                }
                FieldLabel {
                    Field { orientation: FieldOrientation::Horizontal,
                        Checkbox { id: "toggle-checkbox-2" }
                        FieldContent {
                            FieldTitle { "Enable notifications" }
                            FieldDescription { "You can enable or disable notifications at any time." }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CheckboxInvalid() -> Element {
    rsx! {
        Example { title: "Invalid State",
            FieldGroup { class: "mx-auto w-56",
                Field { orientation: FieldOrientation::Horizontal, "data-invalid": "true",
                    Checkbox { id: "terms-checkbox-invalid", "aria-invalid": "true" }
                    FieldLabel { r#for: "terms-checkbox-invalid", "Accept terms and conditions" }
                }
            }
        }
    }
}

#[component]
fn CheckboxBasic() -> Element {
    rsx! {
        Example { title: "Basic",
            FieldGroup { class: "mx-auto w-56",
                Field { orientation: FieldOrientation::Horizontal,
                    Checkbox { id: "terms-checkbox-basic" }
                    FieldLabel { r#for: "terms-checkbox-basic", "Accept terms and conditions" }
                }
            }
        }
    }
}

#[component]
fn CheckboxDescription() -> Element {
    rsx! {
        Example { title: "Description",
            FieldGroup { class: "mx-auto w-72",
                Field { orientation: FieldOrientation::Horizontal,
                    Checkbox { id: "terms-checkbox-desc", default_checked: true }
                    FieldContent {
                        FieldLabel { r#for: "terms-checkbox-desc", "Accept terms and conditions" }
                        FieldDescription { "By clicking this checkbox, you agree to the terms and conditions." }
                    }
                }
            }
        }
    }
}

#[component]
fn CheckboxDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            FieldGroup { class: "mx-auto w-56",
                Field { orientation: FieldOrientation::Horizontal, "data-disabled": "true",
                    Checkbox { id: "toggle-checkbox-disabled", disabled: true }
                    FieldLabel { r#for: "toggle-checkbox-disabled", "Enable notifications" }
                }
            }
        }
    }
}

#[component]
fn CheckboxGroup() -> Element {
    let items = [
        (
            "finder-pref-9k2-hard-disks-ljj-checkbox",
            "Hard disks",
            true,
        ),
        (
            "finder-pref-9k2-external-disks-1yg-checkbox",
            "External disks",
            true,
        ),
        (
            "finder-pref-9k2-cds-dvds-fzt-checkbox",
            "CDs, DVDs, and iPods",
            false,
        ),
        (
            "finder-pref-9k2-connected-servers-6l2-checkbox",
            "Connected servers",
            false,
        ),
    ];

    rsx! {
        Example { title: "Group",
            FieldSet {
                FieldLegend { variant: FieldLegendVariant::Label, "Show these items on the desktop:" }
                FieldDescription { "Select the items you want to show on the desktop." }
                FieldGroup { class: "gap-3",
                    for (id, label, checked) in items {
                        Field { key: "{id}", orientation: FieldOrientation::Horizontal,
                            Checkbox { id: id, default_checked: checked }
                            FieldLabel { r#for: id, class: "font-normal", "{label}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CheckboxInTable() -> Element {
    let mut selected = use_signal(|| vec!["1"]);
    let select_all = selected.read().len() == TABLE_DATA.len();

    rsx! {
        Example { title: "Table", class: "p-4 md:p-8",
            Table {
                TableHeader {
                    TableRow {
                        TableHead { class: "w-8",
                            Checkbox {
                                id: "select-all-checkbox",
                                checked: select_all,
                                on_checked_change: move |checked: bool| {
                                    if checked {
                                        selected.set(TABLE_DATA.iter().map(|row| row.id).collect());
                                    } else {
                                        selected.set(Vec::new());
                                    }
                                },
                            }
                        }
                        TableHead { "Name" }
                        TableHead { "Email" }
                        TableHead { "Role" }
                    }
                }
                TableBody {
                    for row in TABLE_DATA.iter() {
                        TableRow {
                            key: "{row.id}",
                            "data-state": selected.read().contains(&row.id).then_some("selected"),
                            TableCell {
                                Checkbox {
                                    id: "row-{row.id}-checkbox",
                                    checked: selected.read().contains(&row.id),
                                    on_checked_change: move |checked: bool| {
                                        let mut rows = selected.write();
                                        rows.retain(|id| *id != row.id);
                                        if checked {
                                            rows.push(row.id);
                                        }
                                    },
                                }
                            }
                            TableCell { class: "font-medium", "{row.name}" }
                            TableCell { "{row.email}" }
                            TableCell { "{row.role}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CheckboxRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-80",
            FieldGroup { class: "max-w-sm", dir: "rtl",
                Field { orientation: FieldOrientation::Horizontal,
                    Checkbox { id: "terms-checkbox-rtl" }
                    Label { r#for: "terms-checkbox-rtl", "قبول الشروط والأحكام" }
                }
                Field { orientation: FieldOrientation::Horizontal,
                    Checkbox { id: "terms-checkbox-2-rtl", default_checked: true }
                    FieldContent {
                        FieldLabel { r#for: "terms-checkbox-2-rtl", "قبول الشروط والأحكام" }
                        FieldDescription { "بالنقر على هذا المربع، فإنك توافق على الشروط." }
                    }
                }
                Field { orientation: FieldOrientation::Horizontal, "data-disabled": "true",
                    Checkbox { id: "toggle-checkbox-rtl", disabled: true }
                    FieldLabel { r#for: "toggle-checkbox-rtl", "تفعيل الإشعارات" }
                }
                FieldLabel {
                    Field { orientation: FieldOrientation::Horizontal,
                        Checkbox { id: "toggle-checkbox-2-rtl" }
                        FieldContent {
                            FieldTitle { "تفعيل الإشعارات" }
                            FieldDescription { "يمكنك تفعيل أو إلغاء تفعيل الإشعارات في أي وقت." }
                        }
                    }
                }
            }
        }
    }
}
