//! Pagination examples, ported from shadcn/ui `examples/base/pagination-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::field::{Field, FieldLabel, FieldOrientation};
use crate::components::ui::pagination::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationNext, PaginationPrevious,
};
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            PaginationDemo {}
            PaginationSimple {}
            PaginationIconsOnly {}
            PaginationRtl {}
        }
    }
}

#[component]
fn PaginationDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Pagination {
                PaginationContent {
                    PaginationItem {
                        PaginationPrevious { href: "#" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", "1" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", is_active: true, "2" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", "3" }
                    }
                    PaginationItem {
                        PaginationEllipsis {}
                    }
                    PaginationItem {
                        PaginationNext { href: "#" }
                    }
                }
            }
        }
    }
}

#[component]
fn PaginationSimple() -> Element {
    rsx! {
        Example { title: "Simple",
            Pagination {
                PaginationContent {
                    PaginationItem {
                        PaginationLink { href: "#", "1" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", is_active: true, "2" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", "3" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", "4" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", "5" }
                    }
                }
            }
        }
    }
}

#[component]
fn PaginationIconsOnly() -> Element {
    rsx! {
        Example { title: "Icons Only",
            div { class: "flex items-center justify-between gap-4",
                Field { orientation: FieldOrientation::Horizontal, class: "w-fit",
                    FieldLabel { r#for: "select-rows-per-page", "Rows per page" }
                    Select { default_value: "25".to_string(),
                        SelectTrigger { class: "w-20", id: "select-rows-per-page",
                            SelectValue {}
                        }
                        SelectContent {
                            SelectGroup {
                                SelectItem { value: "10", "10" }
                                SelectItem { value: "25", "25" }
                                SelectItem { value: "50", "50" }
                                SelectItem { value: "100", "100" }
                            }
                        }
                    }
                }
                Pagination { class: "mx-0 w-auto",
                    PaginationContent {
                        PaginationItem {
                            PaginationPrevious { href: "#" }
                        }
                        PaginationItem {
                            PaginationNext { href: "#" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PaginationRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Pagination { dir: "rtl",
                PaginationContent {
                    PaginationItem {
                        PaginationPrevious { href: "#", text: "السابق" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", "١" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", is_active: true, "٢" }
                    }
                    PaginationItem {
                        PaginationLink { href: "#", "٣" }
                    }
                    PaginationItem {
                        PaginationEllipsis {}
                    }
                    PaginationItem {
                        PaginationNext { href: "#", text: "التالي" }
                    }
                }
            }
        }
    }
}
