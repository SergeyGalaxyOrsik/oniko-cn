//! Component gallery for oniko-cn. Every page ports the examples of one
//! shadcn/ui docs page, so the two can be compared side by side.

#[allow(
    dead_code,
    reason = "`oniko add` installs whole components; the examples use only some parts"
)]
mod components;
mod examples;

use dioxus::prelude::*;
use examples::{Group, PAGES, PageExamples};
use lucide_dioxus::{Moon, Sun};

const TAILWIND: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut dark = use_signal(|| false);
    let mut current = use_signal(|| "button".to_string());
    let title = PAGES
        .iter()
        .find(|p| p.slug == current())
        .map_or("", |p| p.title);
    use_effect(move || {
        current.read();
        document::eval("document.getElementById('page')?.scrollTo(0, 0)");
    });

    rsx! {
        document::Stylesheet { href: TAILWIND }
        div { class: if dark() { "dark" } else { "" },
            div { class: "flex h-screen bg-background text-foreground antialiased",
                nav { class: "flex w-60 shrink-0 flex-col gap-6 overflow-y-auto border-r px-3 py-6",
                    div { class: "flex items-center justify-between px-2",
                        span { class: "text-sm font-semibold", "oniko-cn" }
                        button {
                            class: "inline-flex size-8 items-center justify-center rounded-md hover:bg-muted",
                            aria_label: "Toggle theme",
                            onclick: move |_| dark.toggle(),
                            if dark() {
                                Sun { size: 16 }
                            } else {
                                Moon { size: 16 }
                            }
                        }
                    }
                    for group in [Group::Components, Group::AiElements] {
                        div { class: "flex flex-col gap-0.5",
                            h4 { class: "px-2 pb-1 text-xs font-medium text-muted-foreground",
                                "{group.title()}"
                            }
                            for page in PAGES.iter().filter(move |p| p.group == group) {
                                button {
                                    key: "{page.slug}",
                                    class: "rounded-md px-2 py-1 text-left text-sm hover:bg-muted data-[active=true]:bg-muted data-[active=true]:font-medium",
                                    "data-active": current() == page.slug,
                                    onclick: move |_| current.set(page.slug.to_string()),
                                    "{page.title}"
                                }
                            }
                        }
                    }
                }
                main { id: "page", class: "min-w-0 flex-1 overflow-y-auto",
                    div { class: "mx-auto flex max-w-3xl flex-col gap-10 px-6 py-10",
                        h1 { class: "text-2xl font-semibold tracking-tight", "{title}" }
                        PageExamples { key: "{current}", slug: current() }
                    }
                }
            }
        }
    }
}
