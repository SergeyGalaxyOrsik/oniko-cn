use dioxus::prelude::*;

use crate::Route;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant, button_variants};
use crate::demos::demo;
use crate::icons::{ArrowRightIcon, PlusIcon};
use crate::registry::{component, components};
use crate::widgets::{CONTAINER, CodeBlock, Preview, SECTION_TITLE};

const LOAD_STYLESHEET: &str = r#"rsx! {
    document::Stylesheet { href: asset!("/assets/tailwind.css") }
    // your app
}"#;

const USE_BUTTON: &str = r#"use crate::components::ui::button::Button;

rsx! {
    Button { onclick: move |_| println!("clicked"), "Click me" }
}"#;

const INSTALL_CLI: &str =
    "cargo install --git https://github.com/SergeyGalaxyOrsik/oniko-cn oniko-cli";

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Oniko UI: components for Dioxus" }
        Hero {}
        Statement {}
        HowItWorks {}
        ComponentIndex {}
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "relative overflow-hidden pt-16 md:pt-24",
            div {
                aria_hidden: "true",
                class: "pointer-events-none absolute inset-x-0 -top-40 mx-auto h-[520px] max-w-[1200px] bg-[radial-gradient(ellipse_at_center,var(--l-glow),transparent_65%)]",
            }
            div { class: "{CONTAINER} relative",
                h1 { class: "animate-rise max-w-[16ch] text-[2.75rem] leading-[1.03] font-medium tracking-[-0.03em] text-balance sm:text-6xl md:text-[4.5rem]",
                    "Components for Dioxus, written in Rust"
                }
                div { class: "animate-rise mt-6 flex flex-col gap-8 [animation-delay:80ms] md:flex-row md:items-end md:justify-between",
                    p { class: "max-w-[42ch] text-[17px] leading-relaxed text-(--l-muted)",
                        "Copy accessible components into your app and own the code. One codebase renders the same on the web and on the desktop."
                    }
                    div { class: "flex shrink-0 items-center gap-2",
                        Link {
                            to: Route::Installation {},
                            class: button_variants(ButtonVariant::Default, ButtonSize::Lg, Some("h-10 rounded-lg px-4")),
                            "Get started"
                        }
                        Link {
                            to: Route::ComponentPage { name: "button".into() },
                            class: "group flex h-10 items-center gap-1.5 rounded-lg px-3 text-sm text-(--l-fg-3) hover:text-(--l-fg)",
                            "Browse components"
                            ArrowRightIcon { class: "size-3.5 transition-transform group-hover:translate-x-0.5" }
                        }
                    }
                }
            }
            div { class: "{CONTAINER} animate-rise relative mt-14 [animation-delay:160ms] md:mt-20",
                HeroWindow {}
            }
        }
    }
}

/// A terminal next to the components it installed, both live.
#[component]
fn HeroWindow() -> Element {
    let mut clicks = use_signal(|| 0);
    rsx! {
        div { class: "grid overflow-hidden rounded-2xl border border-(--l-ink)/8 bg-(--l-panel) shadow-(--l-window-shadow) md:grid-cols-[1.1fr_1fr]",
            div { class: "border-b border-(--l-ink)/6 bg-(--l-code) p-6 font-mono text-[13px] leading-7 md:border-r md:border-b-0",
                div { class: "mb-4 flex gap-1.5",
                    span { class: "size-2.5 rounded-full bg-[#ff5f57]" }
                    span { class: "size-2.5 rounded-full bg-[#febc2e]" }
                    span { class: "size-2.5 rounded-full bg-[#28c840]" }
                }
                p { span { class: "text-(--l-faint)", "$ " } "oniko init --style mira" }
                p { class: "text-(--l-muted)", "  wrote oniko.toml and tailwind.css" }
                p { span { class: "text-(--l-faint)", "$ " } "oniko add button" }
                p { class: "text-(--l-muted)", "  src/components/ui/button.rs  created" }
                p { span { class: "text-(--l-faint)", "$ " } "dx serve --desktop" }
                p { class: "text-accent-soft", "  the same buttons, in a native window" }
            }
            div { class: "flex flex-col justify-center gap-6 p-8",
                div { class: "flex flex-wrap gap-2",
                    Button { "Default" }
                    Button { variant: ButtonVariant::Secondary, "Secondary" }
                    Button { variant: ButtonVariant::Outline, "Outline" }
                    Button { variant: ButtonVariant::Ghost, "Ghost" }
                }
                div { class: "flex flex-wrap items-center gap-2",
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| clicks += 1,
                        PlusIcon {}
                        "Clicked {clicks} times"
                    }
                    Button { variant: ButtonVariant::Destructive, "Delete" }
                    Button { variant: ButtonVariant::Link, "Learn more" }
                }
                p { class: "text-[13px] text-(--l-muted)",
                    "These are the real components, compiled to WebAssembly and running on this page."
                }
            }
        }
    }
}

#[component]
fn Statement() -> Element {
    let lit = "text-(--l-fg)";
    rsx! {
        section { class: "py-20 md:py-28",
            div { class: CONTAINER,
                p { class: "max-w-[34ch] text-2xl leading-[1.35] font-medium tracking-[-0.015em] text-(--l-faint) md:text-[2rem]",
                    "oniko-cn is not a crate you import. "
                    span { class: lit, "The CLI copies the source into your app" }
                    ", every component "
                    span { class: lit, "runs on the web and the desktop" }
                    ", and the look comes from "
                    span { class: lit, "eight styles you pick at init" }
                    "."
                }
            }
        }
    }
}

#[component]
fn HowItWorks() -> Element {
    rsx! {
        section { class: "border-t border-(--l-ink)/6 py-20 md:py-28",
            div { class: CONTAINER,
                h2 { class: SECTION_TITLE,
                    "Three commands. "
                    span { class: "text-(--l-faint)", "Then the code is yours." }
                }
                div { class: "mt-12 grid gap-3 md:grid-cols-3",
                    Step {
                        title: "Set up the crate",
                        text: "oniko init writes the theme, wires the modules and adds the one runtime crate.",
                        code: "oniko init --style mira",
                    }
                    Step {
                        title: "Add components",
                        text: "oniko add copies each component and the components it needs into src/components/ui.",
                        code: "oniko add button",
                    }
                    Step {
                        title: "Change anything",
                        text: "The file is plain Rust with Tailwind classes. Edit a variant, add a prop, delete what you don't use.",
                        code: "Self::Default => \"bg-primary ...\"",
                    }
                }
            }
        }
    }
}

#[component]
fn Step(title: String, text: String, code: String) -> Element {
    rsx! {
        div { class: "flex flex-col overflow-hidden rounded-2xl border border-(--l-ink)/8 bg-(--l-cell)",
            div { class: "p-5 sm:p-6",
                div { class: "rounded-lg border border-(--l-ink)/8 bg-(--l-code) px-3.5 py-2.5 font-mono text-[13px] text-(--l-fg-3)", "{code}" }
            }
            p { class: "px-5 pb-5 text-[15px] leading-relaxed text-(--l-muted) sm:px-6 sm:pb-6",
                span { class: "font-medium text-(--l-fg)", "{title}. " }
                "{text}"
            }
        }
    }
}

#[component]
fn ComponentIndex() -> Element {
    rsx! {
        section { class: "border-t border-(--l-ink)/6 py-20 md:py-28",
            div { class: CONTAINER,
                h2 { class: SECTION_TITLE,
                    "The components. "
                    span { class: "text-(--l-faint)", "More land every week." }
                }
                div { class: "mt-12 grid gap-3 sm:grid-cols-2 lg:grid-cols-3",
                    for c in components() {
                        Link {
                            key: "{c.name}",
                            to: Route::ComponentPage { name: c.name.to_string() },
                            class: "group rounded-2xl border border-(--l-ink)/8 bg-(--l-cell) p-5 transition-colors hover:bg-(--l-ink)/[0.03]",
                            p { class: "flex items-center justify-between text-[15px] font-medium",
                                "{c.title()}"
                                ArrowRightIcon { class: "size-3.5 text-(--l-faint) transition-transform group-hover:translate-x-0.5" }
                            }
                            p { class: "mt-1.5 text-[13px] leading-relaxed text-(--l-muted)", "{c.description()}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DocHeader(title: String, lead: String) -> Element {
    rsx! {
        document::Title { "{title}: Oniko UI" }
        h1 { class: "text-[2rem] leading-tight font-medium tracking-[-0.022em]", "{title}" }
        p { class: "mt-3 text-[17px] leading-relaxed text-(--l-muted)", "{lead}" }
    }
}

const PROSE_H2: &str = "mt-12 mb-4 text-xl font-medium tracking-[-0.01em]";
const PROSE_P: &str = "my-4 text-[15px] leading-relaxed text-(--l-fg-3)";

#[component]
pub fn Introduction() -> Element {
    rsx! {
        DocHeader {
            title: "Introduction",
            lead: "Accessible components for Dioxus that you copy into your app. Inspired by shadcn/ui, written in Rust.",
        }
        h2 { class: PROSE_H2, "You own the code" }
        p { class: PROSE_P,
            "oniko-cn is not a dependency you import components from. The oniko CLI copies each component's source into src/components/ui, and from then on the file is yours. To change how a button looks or behaves, edit button.rs."
        }
        h2 { class: PROSE_H2, "One codebase for web and desktop" }
        p { class: PROSE_P,
            "Components use only Dioxus APIs, never browser-only crates, so the same file renders in the browser and in a native desktop window. Every component is checked on both."
        }
        h2 { class: PROSE_H2, "Styles you pick once" }
        p { class: PROSE_P,
            "The registry ships eight styles from shadcn/ui: vega, nova, maia, lyra, mira, luma, rhea and sera. You choose one with oniko init. The installed code contains plain Tailwind classes for that style, and theme colors are CSS variables you can change at any time."
        }
        div { class: "mt-10 flex gap-2",
            Link {
                to: Route::Installation {},
                class: button_variants(ButtonVariant::Default, ButtonSize::Default, None),
                "Installation"
                ArrowRightIcon {}
            }
        }
    }
}

#[component]
pub fn Installation() -> Element {
    rsx! {
        DocHeader {
            title: "Installation",
            lead: "Add oniko-cn to a Dioxus 0.7 app in four steps.",
        }
        h2 { class: PROSE_H2, "1. Install the CLI" }
        CodeBlock { label: "terminal", code: INSTALL_CLI }
        h2 { class: PROSE_H2, "2. Create a Dioxus app" }
        p { class: PROSE_P, "Skip this step to add oniko-cn to an existing app." }
        CodeBlock { label: "terminal", code: "dx new my-app\ncd my-app" }
        h2 { class: PROSE_H2, "3. Set up oniko-cn" }
        p { class: PROSE_P,
            "Run init in the crate. It writes oniko.toml, puts the theme into tailwind.css, declares the components module and adds the oniko-cn crate to Cargo.toml."
        }
        CodeBlock { label: "terminal", code: "oniko init --style mira --base-color neutral" }
        p { class: PROSE_P, "Then load the stylesheet that dx compiles from tailwind.css in your root component:" }
        CodeBlock {
            label: "src/main.rs",
            code: LOAD_STYLESHEET,
        }
        h2 { class: PROSE_H2, "4. Add components" }
        CodeBlock { label: "terminal", code: "oniko add button" }
        p { class: PROSE_P, "Use it like any other Dioxus component:" }
        CodeBlock {
            label: "src/main.rs",
            code: USE_BUTTON,
        }
        p { class: PROSE_P, "Run it in the browser with dx serve --web, or in a desktop window with dx serve --desktop." }
    }
}

#[component]
pub fn ComponentPage(name: String) -> Element {
    let Some(c) = component(&name) else {
        return rsx! {
            DocHeader { title: "Not found", lead: "There is no component called {name}." }
        };
    };
    let demo = demo(c.name);

    rsx! {
        DocHeader { title: c.title().to_string(), lead: c.description().to_string() }
        if let Some(demo) = &demo {
            div { class: "mt-8", Preview { {(demo.preview)()} } }
        }
        h2 { class: PROSE_H2, "Installation" }
        CodeBlock { label: "terminal", code: "oniko add {c.name}" }
        if let Some(demo) = &demo {
            h2 { class: PROSE_H2, "Usage" }
            CodeBlock { code: demo.usage }
        }
        h2 { class: PROSE_H2, "Source" }
        p { class: PROSE_P, "This is the file oniko add writes, shown for the mira style." }
        CodeBlock { label: "src/components/ui/{c.name}.rs", code: c.source().to_string(), tall: true }
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "{CONTAINER} py-32",
            h1 { class: SECTION_TITLE, "Page not found." }
            p { class: "mt-4 text-(--l-muted)", "The page /{segments.join(\"/\")} does not exist." }
            div { class: "mt-8",
                Link { to: Route::Home {}, class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None), "Back home" }
            }
        }
    }
}
