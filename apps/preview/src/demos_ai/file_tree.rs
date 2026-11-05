use dioxus::prelude::*;

use crate::components::ui::file_tree::{FileTree, FileTreeActions, FileTreeFile, FileTreeFolder};

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-col gap-3 rounded-lg border p-6";

#[component]
pub fn FileTreeDemo() -> Element {
    let mut selected = use_signal(|| "src/components/button.rs".to_string());

    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "File Tree" }
            div { class: BOX,
                FileTree {
                    selected: Some(selected()),
                    default_expanded: vec![
                        "src".to_string(),
                        "src/components".to_string(),
                    ],
                    on_select: move |path: String| selected.set(path),
                    FileTreeFolder { path: "src", name: "src",
                        FileTreeFolder { path: "src/components", name: "components",
                            FileTreeFolder { path: "src/components/ui", name: "ui",
                                FileTreeFile { path: "src/components/ui/button.rs", name: "button.rs" }
                                FileTreeFile { path: "src/components/ui/card.rs", name: "card.rs" }
                            }
                            FileTreeFile { path: "src/components/app.rs", name: "app.rs" }
                        }
                        FileTreeFolder { path: "src/hooks", name: "hooks",
                            FileTreeFile { path: "src/hooks/use_prop.rs", name: "use_prop.rs" }
                        }
                        FileTreeFile { path: "src/main.rs", name: "main.rs" }
                    }
                    FileTreeFolder { path: "docs", name: "docs",
                        FileTreeFile { path: "docs/README.md", name: "README.md" }
                    }
                    FileTreeFile { path: "Cargo.toml", name: "Cargo.toml" }
                }
                p { class: "text-sm text-muted-foreground",
                    "Selected: {selected}"
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "File Tree with status actions" }
            div { class: BOX,
                FileTree {
                    default_expanded: vec!["src".to_string()],
                    FileTreeFolder { path: "src", name: "src",
                        FileTreeFile { path: "src/lib.rs", name: "lib.rs",
                            FileTreeActions {
                                span { class: "text-xs font-medium text-amber-600", "M" }
                            }
                        }
                        FileTreeFile { path: "src/parser.rs", name: "parser.rs",
                            FileTreeActions {
                                span { class: "text-xs font-medium text-emerald-600", "A" }
                            }
                        }
                        FileTreeFile { path: "src/legacy.rs", name: "legacy.rs",
                            FileTreeActions {
                                span { class: "text-xs font-medium text-red-600", "D" }
                            }
                        }
                    }
                }
            }
        }
    }
}
