//! File Tree examples, ported from shadcn/ui `examples/base/file-tree-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::file_tree::{FileNode, FileTreeNode};

#[component]
pub fn Examples() -> Element {
    rsx! {
        FileTreeDemo {}
    }
}

fn file(name: &str, path: &str) -> FileNode {
    FileNode::File {
        name: name.to_string(),
        path: path.to_string(),
    }
}

fn folder(name: &str, path: &str, children: Vec<FileNode>) -> FileNode {
    FileNode::Folder {
        name: name.to_string(),
        path: path.to_string(),
        children,
    }
}

fn files() -> Vec<FileNode> {
    vec![
        folder(
            "app",
            "app",
            vec![
                file("layout.tsx", "app/layout.tsx"),
                file("page.tsx", "app/page.tsx"),
            ],
        ),
        folder(
            "components",
            "components",
            vec![
                folder(
                    "ui",
                    "components/ui",
                    vec![file("button.tsx", "components/ui/button.tsx")],
                ),
                file("login-form.tsx", "components/login-form.tsx"),
            ],
        ),
    ]
}

#[component]
fn FileTreeDemo() -> Element {
    let mut selected = use_signal(|| "components/ui/button.tsx".to_string());

    rsx! {
        Example { title: "Demo", class: "h-auto",
            div { class: "flex w-full max-w-xs flex-col py-6 text-sm",
                for node in files() {
                    FileTreeNode {
                        key: "{node.path()}",
                        selected: selected(),
                        on_select: move |path| selected.set(path),
                        node,
                    }
                }
            }
        }
    }
}
