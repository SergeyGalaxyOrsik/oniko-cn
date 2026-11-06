//! File tree: the files the agent changed, as folders you can open. A folder is a `Collapsible`
//! and a file is a `Button`. Render the nodes with `FileTreeNode`, which calls itself for the
//! children of a folder.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::button::{Button, ButtonSize, ButtonVariant};
use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[derive(Clone, PartialEq)]
pub enum FileNode {
    File {
        name: String,
        path: String,
    },
    Folder {
        name: String,
        path: String,
        children: Vec<FileNode>,
    },
}

impl FileNode {
    pub fn path(&self) -> &str {
        match self {
            Self::File { path, .. } | Self::Folder { path, .. } => path,
        }
    }
}

#[component]
fn Icon(paths: &'static [&'static str], class: Option<&'static str>) -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: class,
            for d in paths {
                path { d: *d }
            }
        }
    }
}

/// A folder or a file, with the path of the selected file highlighted.
#[component]
pub fn FileTreeNode(node: FileNode, selected: String, on_select: EventHandler<String>) -> Element {
    match node {
        FileNode::Folder { name, children, .. } => rsx! {
            Collapsible { default_open: true,
                CollapsibleTrigger {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    class: "w-full justify-start font-normal aria-expanded:bg-transparent aria-expanded:hover:bg-muted dark:aria-expanded:hover:bg-muted/50",
                    Icon {
                        class: "transition-transform group-data-panel-open/button:rotate-90",
                        paths: &["m9 18 6-6-6-6"],
                    }
                    Icon { paths: &["M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"] }
                    "{name}"
                }
                CollapsibleContent {
                    div { class: "ml-4 flex flex-col",
                        for child in children {
                            FileTreeNode {
                                key: "{child.path()}",
                                node: child.clone(),
                                selected: selected.clone(),
                                on_select,
                            }
                        }
                    }
                }
            }
        },
        FileNode::File { name, path } => {
            let current = selected == path;
            rsx! {
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    class: if current { "w-full justify-start font-normal bg-accent text-accent-foreground" } else { "w-full justify-start font-normal" },
                    aria_current: current.then_some("true"),
                    onclick: move |_| on_select.call(path.clone()),
                    Icon { paths: &[
                        "M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z",
                        "M14 2v5a1 1 0 0 0 1 1h5",
                    ] }
                    "{name}"
                }
            }
        }
    }
}
