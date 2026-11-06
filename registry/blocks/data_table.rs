//! DataTable: the shadcn/ui data table guide as one component. It composes `Table`, `Checkbox`,
//! `DropdownMenu`, `Button` and `Input`, with sorting, filtering, column visibility, row selection and
//! pagination kept in plain Rust state instead of TanStack Table.
//!
//! Describe each column with [`DataColumn`] and pass the rows as `data`. Selection is kept by row
//! position in `data`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::Rc;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{Button, ButtonSize, ButtonVariant};
use super::checkbox::Checkbox;
use super::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuCheckboxItem, DropdownMenuContent,
    DropdownMenuGroup, DropdownMenuTrigger,
};
use super::input::Input;
use super::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};

type ValueFn<T> = Rc<dyn Fn(&T) -> String>;
type CellFn<T> = Rc<dyn Fn(&T) -> Element>;
type HeaderFn = Rc<dyn Fn() -> Element>;

/// One column of a [`DataTable`].
///
/// `value` gives the text that is sorted and filtered; numbers sort as numbers. `cell` only changes how
/// the cell is drawn.
pub struct DataColumn<T> {
    id: &'static str,
    header: Option<HeaderFn>,
    value: ValueFn<T>,
    cell: Option<CellFn<T>>,
    sortable: bool,
    hideable: bool,
}

impl<T: 'static> DataColumn<T> {
    /// `id` names the column in the visibility menu and in `filter_column`.
    pub fn new(id: &'static str, value: impl Fn(&T) -> String + 'static) -> Self {
        Self {
            id,
            header: None,
            value: Rc::new(value),
            cell: None,
            sortable: false,
            hideable: true,
        }
    }

    /// Plain text header.
    pub fn title(self, title: &'static str) -> Self {
        self.header(move || rsx! { {title} })
    }

    pub fn header(mut self, render: impl Fn() -> Element + 'static) -> Self {
        self.header = Some(Rc::new(render));
        self
    }

    pub fn cell(mut self, render: impl Fn(&T) -> Element + 'static) -> Self {
        self.cell = Some(Rc::new(render));
        self
    }

    /// The header becomes a ghost button that sorts by this column.
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    /// Whether the "Columns" menu can hide this column. On by default.
    pub fn hideable(mut self, hideable: bool) -> Self {
        self.hideable = hideable;
        self
    }
}

impl<T> Clone for DataColumn<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            header: self.header.clone(),
            value: self.value.clone(),
            cell: self.cell.clone(),
            sortable: self.sortable,
            hideable: self.hideable,
        }
    }
}

impl<T> PartialEq for DataColumn<T> {
    fn eq(&self, other: &Self) -> bool {
        fn same<F: ?Sized>(a: &Option<Rc<F>>, b: &Option<Rc<F>>) -> bool {
            match (a, b) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
        }
        self.id == other.id
            && self.sortable == other.sortable
            && self.hideable == other.hideable
            && Rc::ptr_eq(&self.value, &other.value)
            && same(&self.header, &other.header)
            && same(&self.cell, &other.cell)
    }
}

/// The texts of the table, so it can be translated.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTableLabels {
    pub columns: String,
    pub no_results: String,
    pub previous: String,
    pub next: String,
    pub select_all: String,
    pub select_row: String,
    /// Between the selected count and the row count: "1 of 5".
    pub of: String,
    pub rows_selected: String,
}

impl Default for DataTableLabels {
    fn default() -> Self {
        Self {
            columns: "Columns".into(),
            no_results: "No results.".into(),
            previous: "Previous".into(),
            next: "Next".into(),
            select_all: "Select all".into(),
            select_row: "Select row".into(),
            of: "of".into(),
            rows_selected: "row(s) selected.".into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sort {
    column: &'static str,
    descending: bool,
}

/// Like the guide's header button: a click sorts ascending, and a click on an ascending column
/// flips it to descending.
fn toggle_sort(current: Option<Sort>, column: &'static str) -> Sort {
    let ascending_now = matches!(current, Some(s) if s.column == column && !s.descending);
    Sort {
        column,
        descending: ascending_now,
    }
}

fn compare_values(a: &str, b: &str) -> Ordering {
    match (a.trim().parse::<f64>(), b.trim().parse::<f64>()) {
        (Ok(x), Ok(y)) => x.total_cmp(&y),
        _ => a.to_lowercase().cmp(&b.to_lowercase()),
    }
}

/// Indices into `data` that match the filter, in sorted order.
fn filtered_rows<T>(
    data: &[T],
    columns: &[DataColumn<T>],
    filter: Option<(&'static str, &str)>,
    sort: Option<Sort>,
) -> Vec<usize> {
    let column_of = |id: &str| columns.iter().find(|c| c.id == id);
    let needle = filter.and_then(|(id, text)| Some((column_of(id)?, text.to_lowercase())));
    let mut rows: Vec<usize> = (0..data.len())
        .filter(|&i| match &needle {
            Some((column, text)) => (column.value)(&data[i]).to_lowercase().contains(text),
            None => true,
        })
        .collect();

    if let Some(Sort { column, descending }) = sort
        && let Some(column) = column_of(column)
    {
        rows.sort_by(|&a, &b| {
            let ord = compare_values(&(column.value)(&data[a]), &(column.value)(&data[b]));
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
fn ArrowUpDown() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m21 16-4 4-4-4" }
            path { d: "M17 20V4" }
            path { d: "m3 8 4-4 4 4" }
            path { d: "M7 4v16" }
        }
    }
}

#[component]
fn ChevronDown() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m6 9 6 6 6-6" }
        }
    }
}

/// A table over `data`, described by `columns`. Extra attributes, such as `class` and `dir`, go to the root.
#[component]
pub fn DataTable<T: Clone + PartialEq + 'static>(
    columns: Vec<DataColumn<T>>,
    data: Vec<T>,
    /// Adds a leading column of checkboxes, with "select all" for the rows on the page.
    #[props(default)]
    selectable: bool,
    /// Id of the column the filter input searches. Without it, there is no filter input.
    filter_column: Option<&'static str>,
    #[props(default = String::from("Filter..."))] filter_placeholder: String,
    /// Rows per page.
    #[props(default = 10)]
    page_size: usize,
    #[props(default)] labels: DataTableLabels,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("w-full", take_class(&mut attributes).as_deref());

    let mut sort = use_signal(|| None::<Sort>);
    let mut filter = use_signal(String::new);
    let mut hidden = use_signal(BTreeSet::<&'static str>::new);
    let mut selected = use_signal(BTreeSet::<usize>::new);
    let mut page = use_signal(|| 0usize);

    let filter_text = filter();
    let hidden_now = hidden();
    let picked = selected();
    let page_size = page_size.max(1);

    let rows = filtered_rows(
        &data,
        &columns,
        filter_column.map(|id| (id, filter_text.as_str())),
        sort(),
    );
    let page_count = rows.len().div_ceil(page_size).max(1);
    let current = page().min(page_count - 1);
    let page_rows: Vec<usize> = rows
        .iter()
        .copied()
        .skip(current * page_size)
        .take(page_size)
        .collect();

    let selected_count = rows.iter().filter(|i| picked.contains(i)).count();
    let page_selected = page_rows.iter().filter(|i| picked.contains(i)).count();
    let all_page_selected = !page_rows.is_empty() && page_selected == page_rows.len();
    let visible: Vec<&DataColumn<T>> = columns
        .iter()
        .filter(|c| !hidden_now.contains(c.id))
        .collect();
    let span = visible.len() + usize::from(selectable);
    let toggled_rows = page_rows.clone();

    rsx! {
        div { "data-slot": "data-table", class: class, ..attributes,
            div { class: "flex items-center gap-2 py-4",
                if filter_column.is_some() {
                    Input {
                        placeholder: filter_placeholder,
                        value: filter_text,
                        oninput: move |event: FormEvent| {
                            filter.set(event.value());
                            page.set(0);
                        },
                        class: "max-w-sm",
                    }
                }
                DropdownMenu {
                    DropdownMenuTrigger { variant: ButtonVariant::Outline, class: "ms-auto",
                        "{labels.columns} "
                        ChevronDown {}
                    }
                    DropdownMenuContent { align: DropdownMenuAlign::End, class: "w-44",
                        DropdownMenuGroup {
                            for column in columns.iter().filter(|c| c.hideable) {
                                DropdownMenuCheckboxItem {
                                    key: "{column.id}",
                                    class: "capitalize",
                                    checked: !hidden_now.contains(column.id),
                                    on_checked_change: {
                                        let id = column.id;
                                        move |shown: bool| {
                                            hidden
                                                .with_mut(|set| {
                                                    if shown {
                                                        set.remove(id);
                                                    } else {
                                                        set.insert(id);
                                                    }
                                                });
                                        }
                                    },
                                    "{column.id}"
                                }
                            }
                        }
                    }
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
                                        indeterminate: page_selected > 0 && !all_page_selected,
                                        on_checked_change: move |checked: bool| {
                                            selected
                                                .with_mut(|set| {
                                                    for index in &toggled_rows {
                                                        if checked {
                                                            set.insert(*index);
                                                        } else {
                                                            set.remove(index);
                                                        }
                                                    }
                                                });
                                        },
                                        aria_label: "{labels.select_all}",
                                    }
                                }
                            }
                            for column in visible.iter().copied() {
                                TableHead { key: "{column.id}",
                                    if column.sortable {
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            onclick: {
                                                let id = column.id;
                                                move |_| sort.set(Some(toggle_sort(sort(), id)))
                                            },
                                            if let Some(header) = &column.header {
                                                {header()}
                                            }
                                            ArrowUpDown {}
                                        }
                                    } else if let Some(header) = &column.header {
                                        {header()}
                                    }
                                }
                            }
                        }
                    }
                    TableBody {
                        if page_rows.is_empty() {
                            TableRow {
                                TableCell { colspan: "{span}", class: "h-24 text-center", "{labels.no_results}" }
                            }
                        }
                        for index in page_rows.iter().copied() {
                            TableRow {
                                key: "{index}",
                                "data-state": picked.contains(&index).then_some("selected"),
                                if selectable {
                                    TableCell {
                                        Checkbox {
                                            checked: picked.contains(&index),
                                            on_checked_change: move |checked: bool| {
                                                selected
                                                    .with_mut(|set| {
                                                        if checked {
                                                            set.insert(index);
                                                        } else {
                                                            set.remove(&index);
                                                        }
                                                    });
                                            },
                                            aria_label: "{labels.select_row}",
                                        }
                                    }
                                }
                                for column in visible.iter().copied() {
                                    TableCell { key: "{column.id}", {cell_content(column, &data[index])} }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex items-center justify-end gap-2 py-4",
                div { class: "flex-1 text-sm text-muted-foreground",
                    "{selected_count} {labels.of} {rows.len()} {labels.rows_selected}"
                }
                div { class: "flex gap-2",
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        disabled: current == 0,
                        onclick: move |_| page.set(current.saturating_sub(1)),
                        "{labels.previous}"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        disabled: current + 1 >= page_count,
                        onclick: move |_| page.set(current + 1),
                        "{labels.next}"
                    }
                }
            }
        }
    }
}
