use dioxus::prelude::*;

use crate::components::ui::button::{ButtonSize, ButtonVariant, button_variants};
use crate::icons::GithubIcon;
use crate::registry::components;
use crate::widgets::{CONTAINER, ThemeToggle};
use crate::{APP_URL, GITHUB_URL, LOGO, Route};

#[component]
pub fn SiteLayout() -> Element {
    rsx! {
        div { class: "flex min-h-screen flex-col bg-(--l-bg) text-(--l-fg)",
            SiteHeader {}
            main { class: "flex-1", Outlet::<Route> {} }
            SiteFooter {}
        }
    }
}

#[component]
fn SiteHeader() -> Element {
    let nav = "text-[13px] text-(--l-muted) transition-colors hover:text-(--l-fg)";
    rsx! {
        header { class: "sticky top-0 z-40 border-b border-(--l-ink)/6 bg-(--l-bg)/75 backdrop-blur-xl",
            div { class: "{CONTAINER} flex h-14 items-center gap-8",
                Link { to: Route::Home {}, class: "flex items-center gap-2 text-[15px] font-medium tracking-tight",
                    img { src: LOGO, alt: "", width: "20", height: "20", class: "rounded-[5px]" }
                    "Oniko"
                    span { class: "text-(--l-faint)", "UI" }
                }
                nav { aria_label: "Sections", class: "hidden items-center gap-6 md:flex",
                    Link { to: Route::Introduction {}, class: nav, "Docs" }
                    Link { to: Route::ComponentPage { name: "button".into() }, class: nav, "Components" }
                    a { href: APP_URL, class: nav, "Oniko app" }
                }
                div { class: "ml-auto flex items-center gap-2",
                    ThemeToggle {}
                    a {
                        href: GITHUB_URL,
                        target: "_blank",
                        rel: "noreferrer",
                        class: button_variants(ButtonVariant::Ghost, ButtonSize::IconSm, None),
                        aria_label: "GitHub",
                        GithubIcon {}
                    }
                    Link {
                        to: Route::Installation {},
                        class: button_variants(ButtonVariant::Default, ButtonSize::Sm, Some("hidden sm:inline-flex")),
                        "Get started"
                    }
                }
            }
        }
    }
}

#[component]
fn SiteFooter() -> Element {
    let link = "text-[13px] text-(--l-muted) hover:text-(--l-fg)";
    rsx! {
        footer { class: "border-t border-(--l-ink)/6 py-10",
            div { class: "{CONTAINER} flex flex-col gap-6 sm:flex-row sm:items-center sm:justify-between",
                div { class: "flex items-center gap-2 text-[13px]",
                    img { src: LOGO, alt: "", width: "18", height: "18", class: "rounded-[4px]" }
                    span { "Oniko UI" }
                    span { class: "text-(--l-faint)", "Components for Dioxus. MIT licensed." }
                }
                nav { aria_label: "Footer", class: "flex flex-wrap gap-x-6 gap-y-2",
                    Link { to: Route::Introduction {}, class: link, "Docs" }
                    Link { to: Route::Installation {}, class: link, "Installation" }
                    a { href: GITHUB_URL, class: link, "GitHub" }
                    a { href: APP_URL, class: link, "Oniko app" }
                }
            }
        }
    }
}

#[component]
pub fn DocsLayout() -> Element {
    let route = use_route::<Route>();
    let item = move |active: bool| {
        if active {
            "block rounded-md px-2.5 py-1.5 text-[13px] bg-(--l-ink)/5 text-(--l-fg)"
        } else {
            "block rounded-md px-2.5 py-1.5 text-[13px] text-(--l-muted) hover:text-(--l-fg)"
        }
    };

    rsx! {
        div { class: "{CONTAINER} flex gap-12 py-10",
            aside { class: "sticky top-24 hidden h-fit w-52 shrink-0 md:block",
                nav { aria_label: "Docs", class: "flex flex-col gap-6",
                    div {
                        p { class: "mb-2 px-2.5 text-[12px] font-medium text-(--l-faint)", "Getting started" }
                        Link { to: Route::Introduction {}, class: item(route == Route::Introduction {}), "Introduction" }
                        Link { to: Route::Installation {}, class: item(route == Route::Installation {}), "Installation" }
                    }
                    div {
                        p { class: "mb-2 px-2.5 text-[12px] font-medium text-(--l-faint)", "Components" }
                        for component in components() {
                            Link {
                                key: "{component.name}",
                                to: Route::ComponentPage { name: component.name.to_string() },
                                class: item(route == Route::ComponentPage { name: component.name.to_string() }),
                                "{component.title()}"
                            }
                        }
                    }
                }
            }
            article { class: "min-w-0 max-w-3xl flex-1", Outlet::<Route> {} }
        }
    }
}
