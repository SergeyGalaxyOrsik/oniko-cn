//! Site-only building blocks. Library components live in `components::ui`.

use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::icons::{CheckIcon, CopyIcon, MoonIcon, SunIcon};

/// One column for everything, so every left edge lines up (as on oniko.orsik.site).
pub const CONTAINER: &str = "mx-auto w-full max-w-[1200px] px-5 sm:px-8";

/// A statement, then a muted sentence that finishes the thought.
pub const SECTION_TITLE: &str = "max-w-[24ch] text-[2rem] leading-[1.1] font-medium tracking-[-0.022em] text-balance md:text-[2.75rem]";

/// Reads the theme the inline script in `index.html` applied before the app loaded.
#[component]
pub fn ThemeToggle() -> Element {
    let mut dark = use_signal(|| false);
    use_future(move || async move {
        let current = document::eval("return document.documentElement.classList.contains('dark')")
            .join::<bool>()
            .await;
        if let Ok(current) = current {
            dark.set(current);
        }
    });

    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::IconSm,
            aria_label: "Toggle theme",
            onclick: move |_| {
                let next = !dark();
                dark.set(next);
                let name = if next { "dark" } else { "light" };
                document::eval(&format!(
                    "document.documentElement.classList.toggle('dark', {next}); localStorage.setItem('theme', '{name}')"
                ));
            },
            if dark() { SunIcon {} } else { MoonIcon {} }
        }
    }
}

/// A code sample with a copy button. `label` names the file or the shell.
#[component]
pub fn CodeBlock(
    code: String,
    #[props(default)] label: Option<String>,
    /// Caps the height and scrolls, for long files.
    #[props(default)]
    tall: bool,
) -> Element {
    let mut copied = use_signal(|| false);
    let text = code.clone();

    rsx! {
        div { class: "group/code relative overflow-hidden rounded-xl border border-(--l-ink)/8 bg-(--l-code)",
            if let Some(label) = label {
                div { class: "border-b border-(--l-ink)/6 px-4 py-2 font-mono text-[12px] text-(--l-muted)", "{label}" }
            }
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::IconXs,
                class: "absolute top-1.5 right-1.5 text-(--l-muted)",
                aria_label: "Copy code",
                onclick: move |_| {
                    let text = text.clone();
                    async move {
                        let js = format!(
                            "await navigator.clipboard.writeText({})",
                            serde_json::to_string(&text).unwrap_or_default()
                        );
                        if document::eval(&js).await.is_ok() {
                            copied.set(true);
                            let _ = document::eval("await new Promise(r => setTimeout(r, 1500))").await;
                            copied.set(false);
                        }
                    }
                },
                if copied() { CheckIcon {} } else { CopyIcon {} }
            }
            pre {
                class: "overflow-auto px-4 py-3.5 pr-10 font-mono text-[13px] leading-relaxed text-(--l-fg-3)",
                class: if tall { "max-h-[32rem]" },
                code { "{code}" }
            }
        }
    }
}
