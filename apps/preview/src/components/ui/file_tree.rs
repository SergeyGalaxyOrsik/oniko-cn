//! File tree: displays a nested list of folders and files that can be expanded and selected.
//!
//! Folders are built on `Collapsible`. Files and folders are plain elements, so a folder's
//! children are written as ordinary nested `FileTreeFolder` and `FileTreeFile` calls.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

const ROW_CLASS: &str = "flex w-full min-w-0 items-center gap-1.5 rounded-md px-2 py-1 text-left text-sm text-foreground outline-none hover:bg-accent hover:text-accent-foreground focus-visible:ring-2 focus-visible:ring-ring/50 data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground";

#[derive(Clone, Copy)]
struct FileTreeContext {
    internal_expanded: Signal<Vec<String>>,
    controlled_expanded: Signal<Option<Vec<String>>>,
    internal_selected: Signal<Option<String>>,
    controlled_selected: Signal<Option<String>>,
    on_expanded_change: Option<EventHandler<Vec<String>>>,
    on_select: Option<EventHandler<String>>,
}

impl FileTreeContext {
    fn expanded(self) -> Vec<String> {
        (self.controlled_expanded)().unwrap_or_else(|| (self.internal_expanded)())
    }

    fn is_expanded(self, path: &str) -> bool {
        self.expanded().iter().any(|open| open == path)
    }

    fn set_expanded(mut self, path: &str, open: bool) {
        let mut next = self.expanded();
        next.retain(|item| item != path);
        if open {
            next.push(path.to_owned());
        }
        if self.controlled_expanded.peek().is_none() {
            self.internal_expanded.set(next.clone());
        }
        if let Some(handler) = self.on_expanded_change {
            handler.call(next);
        }
    }

    fn selected(self) -> Option<String> {
        (self.controlled_selected)().or_else(|| (self.internal_selected)())
    }

    fn select(mut self, path: &str) {
        if self.controlled_selected.peek().is_none() {
            self.internal_selected.set(Some(path.to_owned()));
        }
        if let Some(handler) = self.on_select {
            handler.call(path.to_owned());
        }
    }
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn chevron_glyph() -> Element {
    rsx! {
        svg {
            "data-slot": "file-tree-chevron",
            class: "size-4 shrink-0 text-muted-foreground transition-transform group-data-[state=open]:rotate-90",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: "m9 18 6-6-6-6" }
        }
    }
}

fn folder_glyph() -> Element {
    rsx! {
        svg {
            "data-slot": "file-tree-folder-glyph",
            class: "size-4 shrink-0",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" }
        }
    }
}

fn file_glyph() -> Element {
    rsx! {
        svg {
            "data-slot": "file-tree-file-glyph",
            class: "size-4 shrink-0",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
            path { d: "M14 2v4a2 2 0 0 0 2 2h4" }
        }
    }
}

/// The root of the tree. Holds which folders are expanded and which path is selected.
///
/// `expanded` and `selected` are controlled when set; otherwise the tree keeps its own state.
#[component]
pub fn FileTree(
    expanded: Option<Vec<String>>,
    #[props(default)] default_expanded: Vec<String>,
    selected: Option<String>,
    on_expanded_change: Option<EventHandler<Vec<String>>>,
    on_select: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-0.5 text-sm",
        take_class(&mut attributes).as_deref()
    );
    let internal_expanded = use_signal(|| default_expanded);
    let controlled_expanded = use_prop(expanded);
    let internal_selected = use_signal(|| None::<String>);
    let controlled_selected = use_prop(selected);
    use_context_provider(|| FileTreeContext {
        internal_expanded,
        controlled_expanded,
        internal_selected,
        controlled_selected,
        on_expanded_change,
        on_select,
    });

    rsx! {
        div {
            "data-slot": "file-tree",
            role: "tree",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A folder row that expands to show its children. Its name is shown with a folder icon.
#[component]
pub fn FileTreeFolder(
    path: String,
    name: String,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let context = use_context::<FileTreeContext>();
    let open = context.is_expanded(&path);
    let toggle_path = path.clone();

    rsx! {
        div {
            "data-slot": "file-tree-folder",
            class: class,
            ..attributes,
            Collapsible {
                open: Some(open),
                on_open_change: move |next: bool| context.set_expanded(&toggle_path, next),
                CollapsibleTrigger {
                    "data-slot": "file-tree-folder-trigger",
                    role: "treeitem",
                    class: cn!("group", ROW_CLASS),
                    {chevron_glyph()}
                    FileTreeIcon { {folder_glyph()} }
                    FileTreeName { "{name}" }
                }
                CollapsibleContent {
                    "data-slot": "file-tree-folder-content",
                    role: "group",
                    class: "flex flex-col gap-0.5 pl-4",
                    {children}
                }
            }
        }
    }
}

/// A file row. Clicking it or pressing Enter or Space selects its `path`.
///
/// Put `FileTreeIcon`, `FileTreeName` and `FileTreeActions` in `children` when you need them.
#[component]
pub fn FileTreeFile(
    path: String,
    name: String,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(ROW_CLASS, take_class(&mut attributes));
    let context = use_context::<FileTreeContext>();
    let selected = context.selected().as_deref() == Some(path.as_str());
    let click_path = path.clone();
    let key_path = path.clone();

    rsx! {
        div {
            "data-slot": "file-tree-file",
            "data-selected": selected.then_some("true"),
            role: "treeitem",
            tabindex: "0",
            aria_selected: selected,
            class: class,
            onclick: move |_| context.select(&click_path),
            onkeydown: move |event: KeyboardEvent| {
                if matches!(event.key().to_string().as_str(), "Enter" | " ") {
                    event.prevent_default();
                    context.select(&key_path);
                }
            },
            ..attributes,
            FileTreeIcon { {file_glyph()} }
            FileTreeName { "{name}" }
            {children}
        }
    }
}

/// The icon slot of a row.
#[component]
pub fn FileTreeIcon(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex shrink-0 items-center text-muted-foreground",
        take_class(&mut attributes)
    );

    rsx! {
        span {
            "data-slot": "file-tree-icon",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The name of a row, truncated when it does not fit.
#[component]
pub fn FileTreeName(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("min-w-0 flex-1 truncate", take_class(&mut attributes));

    rsx! {
        span {
            "data-slot": "file-tree-name",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Controls placed at the end of a row, such as a status badge or a menu button.
/// Clicks and key presses here do not select the row.
#[component]
pub fn FileTreeActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "ml-auto flex items-center gap-1",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "file-tree-actions",
            class: class,
            onclick: move |event| event.stop_propagation(),
            onkeydown: move |event| event.stop_propagation(),
            ..attributes,
            {children}
        }
    }
}
