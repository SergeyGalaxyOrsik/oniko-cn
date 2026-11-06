//! Navigation Menu examples, ported from shadcn/ui `examples/base/navigation-menu-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{CircleAlert, CircleCheck, CircleDashed};

use super::Example;
use crate::components::ui::navigation_menu::{
    NavigationMenu, NavigationMenuAlign, NavigationMenuContent, NavigationMenuItem,
    NavigationMenuLink, NavigationMenuList, NavigationMenuTrigger, navigation_menu_trigger_style,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        NavigationMenuDemo {}
        NavigationMenuRtl {}
    }
}

const COMPONENTS: [(&str, &str, &str); 6] = [
    (
        "Alert Dialog",
        "/docs/primitives/alert-dialog",
        "A modal dialog that interrupts the user with important content and expects a response.",
    ),
    (
        "Hover Card",
        "/docs/primitives/hover-card",
        "For sighted users to preview content available behind a link.",
    ),
    (
        "Progress",
        "/docs/primitives/progress",
        "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
    ),
    (
        "Scroll-area",
        "/docs/primitives/scroll-area",
        "Visually or semantically separates content.",
    ),
    (
        "Tabs",
        "/docs/primitives/tabs",
        "A set of layered sections of content—known as tab panels—that are displayed one at a time.",
    ),
    (
        "Tooltip",
        "/docs/primitives/tooltip",
        "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.",
    ),
];

const COMPONENTS_AR: [(&str, &str, &str); 6] = [
    (
        "حوار التنبيه",
        "/docs/primitives/alert-dialog",
        "حوار نافذة يقطع المستخدم بمحتوى مهم ويتوقع استجابة.",
    ),
    (
        "بطاقة التحويم",
        "/docs/primitives/hover-card",
        "للمستخدمين المبصرين لمعاينة المحتوى المتاح خلف الرابط.",
    ),
    (
        "التقدم",
        "/docs/primitives/progress",
        "يعرض مؤشرًا يوضح تقدم إتمام المهمة، عادةً يتم عرضه كشريط تقدم.",
    ),
    (
        "منطقة التمرير",
        "/docs/primitives/scroll-area",
        "يفصل المحتوى بصريًا أو دلاليًا.",
    ),
    (
        "التبويبات",
        "/docs/primitives/tabs",
        "مجموعة من أقسام المحتوى المتعددة الطبقات—المعروفة بألواح التبويب—التي يتم عرضها واحدة في كل مرة.",
    ),
    (
        "تلميح",
        "/docs/primitives/tooltip",
        "نافذة منبثقة تعرض معلومات متعلقة بعنصر عندما يتلقى العنصر التركيز على لوحة المفاتيح أو عند تحويم الماوس فوقه.",
    ),
];

#[component]
fn ListItem(title: String, href: String, children: Element) -> Element {
    rsx! {
        li {
            NavigationMenuLink { href,
                div { class: "flex flex-col gap-1 text-sm",
                    div { class: "leading-none font-medium", "{title}" }
                    div { class: "line-clamp-2 text-muted-foreground", {children} }
                }
            }
        }
    }
}

#[component]
fn NavigationMenuDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-96",
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuTrigger { "Getting started" }
                        NavigationMenuContent {
                            ul { class: "w-96",
                                ListItem { href: "/docs", title: "Introduction",
                                    "Re-usable components built with Tailwind CSS."
                                }
                                ListItem { href: "/docs/installation", title: "Installation",
                                    "How to install dependencies and structure your app."
                                }
                                ListItem { href: "/docs/primitives/typography", title: "Typography",
                                    "Styles for headings, paragraphs, lists...etc"
                                }
                            }
                        }
                    }
                    NavigationMenuItem { class: "hidden md:flex",
                        NavigationMenuTrigger { "Components" }
                        NavigationMenuContent {
                            ul { class: "grid w-[400px] gap-2 md:w-[500px] md:grid-cols-2 lg:w-[600px]",
                                for (title , href , description) in COMPONENTS {
                                    ListItem { key: "{title}", title, href, "{description}" }
                                }
                            }
                        }
                    }
                    NavigationMenuItem {
                        NavigationMenuTrigger { "With Icon" }
                        NavigationMenuContent {
                            ul { class: "grid w-[200px]",
                                li {
                                    NavigationMenuLink { href: "#", class: "flex-row items-center gap-2",
                                        CircleAlert {}
                                        "Backlog"
                                    }
                                    NavigationMenuLink { href: "#", class: "flex-row items-center gap-2",
                                        CircleDashed {}
                                        "To Do"
                                    }
                                    NavigationMenuLink { href: "#", class: "flex-row items-center gap-2",
                                        CircleCheck {}
                                        "Done"
                                    }
                                }
                            }
                        }
                    }
                    NavigationMenuItem {
                        NavigationMenuLink {
                            href: "/docs",
                            class: navigation_menu_trigger_style(None),
                            "Docs"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavigationMenuRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-96",
            NavigationMenu { dir: "rtl", align: NavigationMenuAlign::End,
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuTrigger { "البدء" }
                        NavigationMenuContent { dir: "rtl", "data-lang": "ar",
                            ul { class: "w-96",
                                ListItem { href: "/docs", title: "مقدمة",
                                    "مكونات قابلة لإعادة الاستخدام مبنية باستخدام Tailwind CSS."
                                }
                                ListItem { href: "/docs/installation", title: "التثبيت",
                                    "كيفية تثبيت التبعيات وتنظيم تطبيقك."
                                }
                                ListItem { href: "/docs/primitives/typography", title: "الطباعة",
                                    "أنماط للعناوين والفقرات والقوائم...إلخ"
                                }
                            }
                        }
                    }
                    NavigationMenuItem { class: "hidden md:flex",
                        NavigationMenuTrigger { "المكونات" }
                        NavigationMenuContent { dir: "rtl", "data-lang": "ar",
                            ul { class: "grid w-[400px] gap-2 md:w-[500px] md:grid-cols-2 lg:w-[600px]",
                                for (title , href , description) in COMPONENTS_AR {
                                    ListItem { key: "{title}", title, href, "{description}" }
                                }
                            }
                        }
                    }
                    NavigationMenuItem {
                        NavigationMenuTrigger { "مع أيقونة" }
                        NavigationMenuContent { dir: "rtl", "data-lang": "ar",
                            ul { class: "grid w-[200px]",
                                li {
                                    NavigationMenuLink { href: "#", class: "flex-row items-center gap-2",
                                        CircleAlert {}
                                        "قائمة الانتظار"
                                    }
                                    NavigationMenuLink { href: "#", class: "flex-row items-center gap-2",
                                        CircleDashed {}
                                        "المهام"
                                    }
                                    NavigationMenuLink { href: "#", class: "flex-row items-center gap-2",
                                        CircleCheck {}
                                        "منجز"
                                    }
                                }
                            }
                        }
                    }
                    NavigationMenuItem {
                        NavigationMenuLink {
                            href: "/docs",
                            class: navigation_menu_trigger_style(None),
                            "data-lang": "ar",
                            "الوثائق"
                        }
                    }
                }
            }
        }
    }
}
