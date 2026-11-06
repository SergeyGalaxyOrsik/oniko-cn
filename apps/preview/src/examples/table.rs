//! Table examples, ported from shadcn/ui `examples/base/table-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::Ellipsis;

use super::Example;
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuItem,
    DropdownMenuItemVariant, DropdownMenuSeparator, DropdownMenuTrigger,
};
use crate::components::ui::table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};

struct Invoice {
    invoice: &'static str,
    payment_status: &'static str,
    total_amount: &'static str,
    payment_method: &'static str,
}

const INVOICES: [Invoice; 7] = [
    Invoice {
        invoice: "INV001",
        payment_status: "Paid",
        total_amount: "$250.00",
        payment_method: "Credit Card",
    },
    Invoice {
        invoice: "INV002",
        payment_status: "Pending",
        total_amount: "$150.00",
        payment_method: "PayPal",
    },
    Invoice {
        invoice: "INV003",
        payment_status: "Unpaid",
        total_amount: "$350.00",
        payment_method: "Bank Transfer",
    },
    Invoice {
        invoice: "INV004",
        payment_status: "Paid",
        total_amount: "$450.00",
        payment_method: "Credit Card",
    },
    Invoice {
        invoice: "INV005",
        payment_status: "Paid",
        total_amount: "$550.00",
        payment_method: "PayPal",
    },
    Invoice {
        invoice: "INV006",
        payment_status: "Pending",
        total_amount: "$200.00",
        payment_method: "Bank Transfer",
    },
    Invoice {
        invoice: "INV007",
        payment_status: "Unpaid",
        total_amount: "$300.00",
        payment_method: "Credit Card",
    },
];

const PRODUCTS: [(&str, &str); 3] = [
    ("Wireless Mouse", "$29.99"),
    ("Mechanical Keyboard", "$129.99"),
    ("USB-C Hub", "$49.99"),
];

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            TableDemo {}
            TableFooterExample {}
            TableActions {}
            TableRtl {}
        }
    }
}

#[component]
fn TableDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-[30rem]",
            Table {
                TableCaption { "A list of your recent invoices." }
                TableHeader {
                    TableRow {
                        TableHead { class: "w-[100px]", "Invoice" }
                        TableHead { "Status" }
                        TableHead { "Method" }
                        TableHead { class: "text-right", "Amount" }
                    }
                }
                TableBody {
                    for invoice in INVOICES {
                        TableRow { key: "{invoice.invoice}",
                            TableCell { class: "font-medium", "{invoice.invoice}" }
                            TableCell { "{invoice.payment_status}" }
                            TableCell { "{invoice.payment_method}" }
                            TableCell { class: "text-right", "{invoice.total_amount}" }
                        }
                    }
                }
                TableFooter {
                    TableRow {
                        TableCell { colspan: 3, "Total" }
                        TableCell { class: "text-right", "$2,500.00" }
                    }
                }
            }
        }
    }
}

#[component]
fn TableFooterExample() -> Element {
    rsx! {
        Example { title: "Footer",
            Table {
                TableCaption { "A list of your recent invoices." }
                TableHeader {
                    TableRow {
                        TableHead { class: "w-[100px]", "Invoice" }
                        TableHead { "Status" }
                        TableHead { "Method" }
                        TableHead { class: "text-right", "Amount" }
                    }
                }
                TableBody {
                    for invoice in INVOICES.iter().take(3) {
                        TableRow { key: "{invoice.invoice}",
                            TableCell { class: "font-medium", "{invoice.invoice}" }
                            TableCell { "{invoice.payment_status}" }
                            TableCell { "{invoice.payment_method}" }
                            TableCell { class: "text-right", "{invoice.total_amount}" }
                        }
                    }
                }
                TableFooter {
                    TableRow {
                        TableCell { colspan: 3, "Total" }
                        TableCell { class: "text-right", "$2,500.00" }
                    }
                }
            }
        }
    }
}

#[component]
fn TableActions() -> Element {
    rsx! {
        Example { title: "Actions",
            Table {
                TableHeader {
                    TableRow {
                        TableHead { "Product" }
                        TableHead { "Price" }
                        TableHead { class: "text-right", "Actions" }
                    }
                }
                TableBody {
                    for (name , price) in PRODUCTS {
                        TableRow { key: "{name}",
                            TableCell { class: "font-medium", "{name}" }
                            TableCell { "{price}" }
                            TableCell { class: "text-right",
                                DropdownMenu {
                                    DropdownMenuTrigger {
                                        variant: ButtonVariant::Ghost,
                                        size: ButtonSize::Icon,
                                        class: "size-8",
                                        Ellipsis {}
                                        span { class: "sr-only", "Open menu" }
                                    }
                                    DropdownMenuContent { align: DropdownMenuAlign::End,
                                        DropdownMenuItem { "Edit" }
                                        DropdownMenuItem { "Duplicate" }
                                        DropdownMenuSeparator {}
                                        DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive, "Delete" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn arabic_status(status: &str) -> &'static str {
    match status {
        "Paid" => "مدفوع",
        "Pending" => "قيد الانتظار",
        _ => "غير مدفوع",
    }
}

fn arabic_method(method: &str) -> &'static str {
    match method {
        "Credit Card" => "بطاقة ائتمانية",
        "PayPal" => "PayPal",
        _ => "تحويل بنكي",
    }
}

#[component]
fn TableRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-auto",
            Table { dir: "rtl",
                TableCaption { "قائمة بفواتيرك الأخيرة." }
                TableHeader {
                    TableRow {
                        TableHead { class: "w-[100px]", "الفاتورة" }
                        TableHead { "الحالة" }
                        TableHead { "الطريقة" }
                        TableHead { class: "text-right", "المبلغ" }
                    }
                }
                TableBody {
                    for invoice in INVOICES {
                        TableRow { key: "{invoice.invoice}",
                            TableCell { class: "font-medium", "{invoice.invoice}" }
                            TableCell { "{arabic_status(invoice.payment_status)}" }
                            TableCell { "{arabic_method(invoice.payment_method)}" }
                            TableCell { class: "text-right", "{invoice.total_amount}" }
                        }
                    }
                }
                TableFooter {
                    TableRow {
                        TableCell { colspan: 3, "المجموع" }
                        TableCell { class: "text-right", "$2,500.00" }
                    }
                }
            }
        }
    }
}
