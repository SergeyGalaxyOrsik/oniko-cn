mod components;

use components::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

const TAILWIND: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut dark = use_signal(|| false);
    let mut clicks = use_signal(|| 0);

    rsx! {
        document::Stylesheet { href: TAILWIND }
        div { class: if dark() { "dark" } else { "" },
            main { class: "min-h-screen bg-background text-foreground antialiased",
                div { class: "mx-auto flex max-w-3xl flex-col gap-10 px-6 py-10",
                    header { class: "flex items-center justify-between gap-4",
                        div {
                            h1 { class: "text-2xl font-semibold tracking-tight", "oniko-cn" }
                            p { class: "text-sm text-muted-foreground",
                                "Rust components for Dioxus: one codebase for web and desktop."
                            }
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Icon,
                            aria_label: "Toggle theme",
                            onclick: move |_| dark.toggle(),
                            if dark() { SunIcon {} } else { MoonIcon {} }
                        }
                    }

                    Section { title: "Variants",
                        Button { "Default" }
                        Button { variant: ButtonVariant::Outline, "Outline" }
                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                        Button { variant: ButtonVariant::Destructive, "Destructive" }
                        Button { variant: ButtonVariant::Link, "Link" }
                    }

                    Section { title: "Sizes",
                        Button { size: ButtonSize::Xs, "Extra small" }
                        Button { size: ButtonSize::Sm, "Small" }
                        Button { "Default" }
                        Button { size: ButtonSize::Lg, "Large" }
                        Button { size: ButtonSize::IconXs, variant: ButtonVariant::Outline, aria_label: "Add", PlusIcon {} }
                        Button { size: ButtonSize::IconSm, variant: ButtonVariant::Outline, aria_label: "Add", PlusIcon {} }
                        Button { size: ButtonSize::Icon, variant: ButtonVariant::Outline, aria_label: "Add", PlusIcon {} }
                        Button { size: ButtonSize::IconLg, variant: ButtonVariant::Outline, aria_label: "Add", PlusIcon {} }
                    }

                    Section { title: "States and composition",
                        Button { onclick: move |_| clicks += 1, PlusIcon {} "Clicked {clicks} times" }
                        Button { disabled: true, "Disabled" }
                        Button { variant: ButtonVariant::Secondary, class: "rounded-full px-5", "Class override" }
                        Button { variant: ButtonVariant::Outline, aria_invalid: "true", "Invalid" }
                    }
                }
            }
        }
    }
}

#[component]
fn Section(title: String, children: Element) -> Element {
    rsx! {
        section { class: "flex flex-col gap-3",
            h2 { class: "text-sm font-medium text-muted-foreground", "{title}" }
            div { class: "flex flex-wrap items-center gap-2 rounded-lg border p-6", {children} }
        }
    }
}

#[component]
fn PlusIcon() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round",
            path { d: "M5 12h14" }
            path { d: "M12 5v14" }
        }
    }
}

#[component]
fn MoonIcon() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
            path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
        }
    }
}

#[component]
fn SunIcon() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "4" }
            path { d: "M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" }
        }
    }
}
