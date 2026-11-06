//! Data Table examples, ported from shadcn/ui `examples/base/data-table-*.tsx`.
//!
//! The docs build the table with TanStack Table. Here the `DataTable` block keeps the same state
//! (sorting, filter, column visibility, row selection, page) in plain Rust.

use dioxus::prelude::*;
use lucide_dioxus::Ellipsis;

use super::Example;
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::data_table::{DataColumn, DataTable, DataTableLabels};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        DataTableDemo {}
        DataTableRtl {}
    }
}

#[derive(Clone, PartialEq)]
struct Payment {
    id: &'static str,
    amount: u32,
    status: &'static str,
    email: &'static str,
}

const PAYMENTS: [Payment; 5] = [
    Payment {
        id: "m5gr84i9",
        amount: 316,
        status: "success",
        email: "ken99@example.com",
    },
    Payment {
        id: "3u1reuv4",
        amount: 242,
        status: "success",
        email: "Abe45@example.com",
    },
    Payment {
        id: "derv1ws0",
        amount: 837,
        status: "processing",
        email: "Monserrat44@example.com",
    },
    Payment {
        id: "5kma53ae",
        amount: 874,
        status: "success",
        email: "Silas22@example.com",
    },
    Payment {
        id: "bhqecj4p",
        amount: 721,
        status: "failed",
        email: "carmella@example.com",
    },
];

/// The words and number format a payments table needs.
#[derive(Clone, Copy)]
struct Texts {
    status: &'static str,
    email: &'static str,
    amount: &'static str,
    actions: &'static str,
    copy_payment_id: &'static str,
    view_customer: &'static str,
    view_payment_details: &'static str,
    open_menu: &'static str,
    /// `success`, `processing`, `failed`, `pending`.
    statuses: [&'static str; 4],
    rtl: bool,
}

impl Texts {
    fn status_name(self, status: &str) -> &'static str {
        let index = ["success", "processing", "failed", "pending"]
            .iter()
            .position(|name| *name == status)
            .unwrap_or(0);
        self.statuses[index]
    }

    /// `Intl.NumberFormat(locale, { style: "currency", currency: "USD" })`.
    fn money(self, amount: u32) -> String {
        if self.rtl {
            let digits: String = format!("{amount}.00")
                .chars()
                .map(|c| match c {
                    '0'..='9' => {
                        char::from_u32(0x0660 + u32::from(c) - u32::from('0')).unwrap_or(c)
                    }
                    '.' => '\u{066B}',
                    other => other,
                })
                .collect();
            format!("\u{200F}{digits}\u{00A0}US$")
        } else {
            format!("${amount}.00")
        }
    }
}

fn payment_columns(t: Texts) -> Vec<DataColumn<Payment>> {
    let align = if t.rtl { "text-start" } else { "text-right" };
    vec![
        DataColumn::new("status", |p: &Payment| p.status.to_string())
            .title(t.status)
            .cell(move |p: &Payment| rsx! {
                div { class: "capitalize", "{t.status_name(p.status)}" }
            }),
        DataColumn::new("email", |p: &Payment| p.email.to_string())
            .title(t.email)
            .sortable()
            .cell(|p: &Payment| rsx! {
                div { class: "lowercase", "{p.email}" }
            }),
        DataColumn::new("amount", |p: &Payment| p.amount.to_string())
            .header(move || rsx! {
                div { class: align, "{t.amount}" }
            })
            .cell(move |p: &Payment| rsx! {
                div { class: "{align} font-medium", "{t.money(p.amount)}" }
            }),
        DataColumn::new("actions", |_: &Payment| String::new())
            .hideable(false)
            .cell(move |p: &Payment| {
                let id = p.id;
                rsx! {
                    DropdownMenu {
                        DropdownMenuTrigger { variant: ButtonVariant::Ghost, size: ButtonSize::IconXs,
                            span { class: "sr-only", "{t.open_menu}" }
                            Ellipsis {}
                        }
                        DropdownMenuContent {
                            align: if t.rtl { DropdownMenuAlign::Start } else { DropdownMenuAlign::End },
                            class: "w-44",
                            DropdownMenuGroup {
                                DropdownMenuLabel { "{t.actions}" }
                                DropdownMenuItem {
                                    onclick: move |_| {
                                        let _ = document::eval(&format!("navigator.clipboard.writeText('{id}')"));
                                    },
                                    "{t.copy_payment_id}"
                                }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuGroup {
                                DropdownMenuItem { "{t.view_customer}" }
                                DropdownMenuItem { "{t.view_payment_details}" }
                            }
                        }
                    }
                }
            }),
    ]
}

#[component]
fn DataTableDemo() -> Element {
    let columns = payment_columns(Texts {
        status: "Status",
        email: "Email",
        amount: "Amount",
        actions: "Actions",
        copy_payment_id: "Copy payment ID",
        view_customer: "View customer",
        view_payment_details: "View payment details",
        open_menu: "Open menu",
        statuses: ["success", "processing", "failed", "pending"],
        rtl: false,
    });
    rsx! {
        Example { title: "Demo", class: "items-start h-auto px-4 md:px-8",
            DataTable::<Payment> {
                columns,
                data: PAYMENTS.to_vec(),
                selectable: true,
                filter_column: "email",
                filter_placeholder: "Filter emails...",
            }
        }
    }
}

#[component]
fn DataTableRtl() -> Element {
    let columns = payment_columns(Texts {
        status: "الحالة",
        email: "البريد الإلكتروني",
        amount: "المبلغ",
        actions: "الإجراءات",
        copy_payment_id: "نسخ معرف الدفع",
        view_customer: "عرض العميل",
        view_payment_details: "عرض تفاصيل الدفع",
        open_menu: "فتح القائمة",
        statuses: ["ناجح", "قيد المعالجة", "فشل", "قيد الانتظار"],
        rtl: true,
    });
    let labels = DataTableLabels {
        columns: "الأعمدة".into(),
        no_results: "لا توجد نتائج.".into(),
        previous: "السابق".into(),
        next: "التالي".into(),
        select_all: "تحديد الكل".into(),
        select_row: "تحديد الصف".into(),
        of: "من".into(),
        rows_selected: "صف(وف) محدد.".into(),
    };
    rsx! {
        Example { title: "RTL", class: "items-start h-auto px-4 md:px-8",
            DataTable::<Payment> {
                columns,
                data: PAYMENTS.to_vec(),
                selectable: true,
                filter_column: "email",
                filter_placeholder: "تصفية البريد الإلكتروني...",
                labels,
                dir: "rtl",
            }
        }
    }
}
