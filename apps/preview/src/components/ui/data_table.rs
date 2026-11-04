//! DataTable: a table with sorting, filtering, pagination and row selection.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::Rc;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{Button, ButtonSize, ButtonVariant};
use super::checkbox::Checkbox;
use super::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};

/// One column of a [`DataTable`].
///
/// `new` takes the text used for sorting and filtering. `cell` replaces how the
/// cell is drawn; it does not change what is sorted or filtered.
type ValueFn<T> = Rc<dyn Fn(&T) -> String>;
type CellFn<T> = Rc<dyn Fn(&T) -> Element>;

pub struct DataColumn<T> {
    header: &'static str,
    value: ValueFn<T>,
    cell: Option<CellFn<T>>,
    sortable: bool,
}

impl<T: 'static> DataColumn<T> {
    pub fn new(header: &'static str, value: impl Fn(&T) -> String + 'static) -> Self {
        Self {
            header,
            value: Rc::new(value),
            cell: None,
            sortable: false,
        }
    }

    pub fn cell(mut self, render: impl Fn(&T) -> Element + 'static) -> Self {
        self.cell = Some(Rc::new(render));
        self
    }

    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }
}

impl<T> Clone for DataColumn<T> {
    fn clone(&self) -> Self {
        Self {
            header: self.header,
            value: self.value.clone(),
            cell: self.cell.clone(),
            sortable: self.sortable,
        }
    }
}

impl<T> PartialEq for DataColumn<T> {
    fn eq(&self, other: &Self) -> bool {
        let cells_match = match (&self.cell, &other.cell) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        self.header == other.header
            && self.sortable == other.sortable
            && Rc::ptr_eq(&self.value, &other.value)
            && cells_match
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sort {
    column: usize,
    descending: bool,
}

/// Sorting cycles through ascending, descending and unsorted for the same column.
fn next_sort(current: Option<Sort>, column: usize) -> Option<Sort> {
    match current {
        Some(Sort {
            column: c,
            descending: false,
        }) if c == column => Some(Sort {
            column,
            descending: true,
        }),
        Some(Sort {
            column: c,
            descending: true,
        }) if c == column => None,
        _ => Some(Sort {
            column,
            descending: false,
        }),
    }
}

fn compare_values(a: &str, b: &str) -> Ordering {
    match (a.trim().parse::<f64>(), b.trim().parse::<f64>()) {
        (Ok(x), Ok(y)) => x.total_cmp(&y),
        _ => a.to_lowercase().cmp(&b.to_lowercase()),
    }
}

/// Indices into `data` in display order, after filtering and sorting.
fn visible_rows<T>(
    data: &[T],
    columns: &[DataColumn<T>],
    filter: &str,
    sort: Option<Sort>,
) -> Vec<usize> {
    let needle = filter.trim().to_lowercase();
    let mut rows: Vec<usize> = (0..data.len())
        .filter(|&i| {
            needle.is_empty()
                || columns
                    .iter()
                    .any(|c| (c.value)(&data[i]).to_lowercase().contains(&needle))
        })
        .collect();

    if let Some(Sort { column, descending }) = sort
        && let Some(col) = columns.get(column)
    {
        rows.sort_by(|&a, &b| {
            let ord = compare_values(&(col.value)(&data[a]), &(col.value)(&data[b]));
            if descending { ord.reverse() } else { ord }
        });
    }
    rows
}

fn cell_content<T>(column: &DataColumn<T>, row: &T) -> Element {
    match &column.cell {
        Some(render) => render(row),
        None => {
            let text = (column.value)(row);
            rsx! { {text} }
        }
    }
}

#[component]
fn SortIcon(sort: Option<bool>) -> Element {
    let path = match sort {
        None => "m21 16-4 4-4-4M17 20V4M3 8l4-4 4 4M7 4v16",
        Some(false) => "m5 12 7-7 7 7M12 19V5",
        Some(true) => "M12 5v14m7-7-7 7-7-7",
    };
    rsx! {
        svg {
            "data-slot": "data-table-sort-icon",
            class: "ml-2 size-4 shrink-0",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: path }
        }
    }
}

/// A table over `data`, described by `columns`. Rows are shown `page_size` at a time.
#[component]
pub fn DataTable<T: Clone + PartialEq + 'static>(
    columns: Vec<DataColumn<T>>,
    data: Vec<T>,
    /// Rows per page.
    #[props(default = 10)]
    page_size: usize,
    /// Adds a checkbox column. Selection is kept by row position in `data`.
    #[props(default)]
    selectable: bool,
    /// Placeholder of the filter box. The filter matches text in any column.
    filter_placeholder: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex w-full flex-col",
        take_class(&mut attributes).as_deref()
    );

    let mut filter = use_signal(String::new);
    let mut sort = use_signal(|| None::<Sort>);
    let mut page = use_signal(|| 0usize);
    let mut selected = use_signal(BTreeSet::<usize>::new);

    let query = filter();
    let sort_state = sort();
    let picked = selected();
    let page_size = page_size.max(1);

    let rows = visible_rows(&data, &columns, &query, sort_state);
    let total = rows.len();
    let page_count = total.div_ceil(page_size).max(1);
    let current = page().min(page_count - 1);
    let page_rows: Vec<usize> = rows
        .iter()
        .copied()
        .skip(current * page_size)
        .take(page_size)
        .collect();

    let selected_visible = rows.iter().filter(|i| picked.contains(i)).count();
    let all_page_selected = !page_rows.is_empty() && page_rows.iter().all(|i| picked.contains(i));
    let span = columns.len() + usize::from(selectable);
    let placeholder = filter_placeholder.unwrap_or_else(|| "Filter...".to_string());
    let page_label = format!("Page {} of {}", current + 1, page_count);
    let selection_label = format!("{selected_visible} of {total} row(s) selected.");
    let header_rows = page_rows.clone();

    rsx! {
        div { "data-slot": "data-table", class: class, ..attributes,
            div { "data-slot": "data-table-toolbar", class: "flex items-center py-4",
                input {
                    "data-slot": "data-table-filter",
                    class: "dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-9 rounded-md border bg-transparent px-2.5 py-1 text-base shadow-xs transition-[color,box-shadow] file:h-7 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm max-w-sm",
                    r#type: "text",
                    placeholder: placeholder,
                    value: query,
                    oninput: move |event| {
                        filter.set(event.value());
                        page.set(0);
                    },
                }
            }

            div { class: "overflow-hidden rounded-md border",
                Table {
                    TableHeader {
                        TableRow {
                            if selectable {
                                TableHead {
                                    Checkbox {
                                        checked: all_page_selected,
                                        aria_label: "Select all",
                                        on_checked_change: move |next: bool| {
                                            selected.with_mut(|set| {
                                                for index in &header_rows {
                                                    if next {
                                                        set.insert(*index);
                                                    } else {
                                                        set.remove(index);
                                                    }
                                                }
                                            });
                                        },
                                    }
                                }
                            }
                            for (position, column) in columns.iter().enumerate() {
                                TableHead { key: "{position}",
                                    if column.sortable {
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            size: ButtonSize::Sm,
                                            onclick: move |_| sort.set(next_sort(sort(), position)),
                                            {column.header}
                                            SortIcon {
                                                sort: match sort_state {
                                                    Some(s) if s.column == position => Some(s.descending),
                                                    _ => None,
                                                },
                                            }
                                        }
                                    } else {
                                        {column.header}
                                    }
                                }
                            }
                        }
                    }
                    TableBody {
                        if page_rows.is_empty() {
                            TableRow {
                                TableCell { colspan: "{span}", class: "h-24 text-center", "No results." }
                            }
                        }
                        for index in page_rows.iter().copied() {
                            TableRow { key: "{index}",
                                "data-state": picked.contains(&index).then_some("selected"),
                                if selectable {
                                    TableCell {
                                        Checkbox {
                                            checked: picked.contains(&index),
                                            aria_label: "Select row",
                                            on_checked_change: move |next: bool| {
                                                selected.with_mut(|set| {
                                                    if next {
                                                        set.insert(index);
                                                    } else {
                                                        set.remove(&index);
                                                    }
                                                });
                                            },
                                        }
                                    }
                                }
                                for (position, column) in columns.iter().enumerate() {
                                    TableCell { key: "{position}",
                                        {cell_content(column, &data[index])}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { "data-slot": "data-table-footer", class: "flex items-center justify-end gap-2 py-4",
                div { class: "flex-1 text-sm text-muted-foreground",
                    if selectable {
                        "{selection_label}"
                    }
                }
                span { class: "text-sm text-muted-foreground", "{page_label}" }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: current == 0,
                    onclick: move |_| page.set(current.saturating_sub(1)),
                    "Previous"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: current + 1 >= page_count,
                    onclick: move |_| page.set(current + 1),
                    "Next"
                }
            }
        }
    }
}
