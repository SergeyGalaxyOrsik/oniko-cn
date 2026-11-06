//! Tabs examples, ported from shadcn/ui `examples/base/tabs-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{AppWindow, Code};

use super::Example;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::tabs::{
    Tabs, TabsContent, TabsList, TabsOrientation, TabsTrigger, TabsVariant,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        TabsDemo {}
        TabsLine {}
        TabsVertical {}
        TabsDisabled {}
        TabsIcons {}
        TabsRtl {}
    }
}

struct Panel {
    value: &'static str,
    label: &'static str,
    description: &'static str,
    content: &'static str,
}

const PANELS: [Panel; 4] = [
    Panel {
        value: "overview",
        label: "Overview",
        description: "View your key metrics and recent project activity. Track progress across all your active projects.",
        content: "You have 12 active projects and 3 pending tasks.",
    },
    Panel {
        value: "analytics",
        label: "Analytics",
        description: "Track performance and user engagement metrics. Monitor trends and identify growth opportunities.",
        content: "Page views are up 25% compared to last month.",
    },
    Panel {
        value: "reports",
        label: "Reports",
        description: "Generate and download your detailed reports. Export data in multiple formats for analysis.",
        content: "You have 5 reports ready and available to export.",
    },
    Panel {
        value: "settings",
        label: "Settings",
        description: "Manage your account preferences and options. Customize your experience to fit your needs.",
        content: "Configure notifications, security, and themes.",
    },
];

const PANELS_AR: [Panel; 4] = [
    Panel {
        value: "overview",
        label: "نظرة عامة",
        description: "عرض مقاييسك الرئيسية وأنشطة المشروع الأخيرة. تتبع التقدم عبر جميع مشاريعك النشطة.",
        content: "لديك ١٢ مشروعًا نشطًا و٣ مهام معلقة.",
    },
    Panel {
        value: "analytics",
        label: "التحليلات",
        description: "تتبع مقاييس الأداء ومشاركة المستخدمين. راقب الاتجاهات وحدد فرص النمو.",
        content: "زادت مشاهدات الصفحة بنسبة ٢٥٪ مقارنة بالشهر الماضي.",
    },
    Panel {
        value: "reports",
        label: "التقارير",
        description: "إنشاء وتنزيل تقاريرك التفصيلية. تصدير البيانات بتنسيقات متعددة للتحليل.",
        content: "لديك ٥ تقارير جاهزة ومتاحة للتصدير.",
    },
    Panel {
        value: "settings",
        label: "الإعدادات",
        description: "إدارة تفضيلات حسابك وخياراته. تخصيص تجربتك لتناسب احتياجاتك.",
        content: "تكوين الإشعارات والأمان والسمات.",
    },
];

#[component]
fn TabsDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-96",
            Tabs { default_value: "overview", class: "w-[400px]",
                TabsList {
                    for panel in PANELS.iter() {
                        TabsTrigger { key: "{panel.value}", value: panel.value, "{panel.label}" }
                    }
                }
                for panel in PANELS.iter() {
                    TabsContent { key: "{panel.value}", value: panel.value,
                        Card {
                            CardHeader {
                                CardTitle { "{panel.label}" }
                                CardDescription { "{panel.description}" }
                            }
                            CardContent { class: "text-sm text-muted-foreground", "{panel.content}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TabsLine() -> Element {
    rsx! {
        Example { title: "Line", class: "h-96",
            Tabs { default_value: "overview",
                TabsList { variant: TabsVariant::Line,
                    TabsTrigger { value: "overview", "Overview" }
                    TabsTrigger { value: "analytics", "Analytics" }
                    TabsTrigger { value: "reports", "Reports" }
                }
            }
        }
    }
}

#[component]
fn TabsVertical() -> Element {
    rsx! {
        Example { title: "Vertical", class: "h-96",
            Tabs { default_value: "account", orientation: TabsOrientation::Vertical,
                TabsList {
                    TabsTrigger { value: "account", "Account" }
                    TabsTrigger { value: "password", "Password" }
                    TabsTrigger { value: "notifications", "Notifications" }
                }
            }
        }
    }
}

#[component]
fn TabsDisabled() -> Element {
    rsx! {
        Example { title: "Disabled", class: "h-96",
            Tabs { default_value: "home",
                TabsList {
                    TabsTrigger { value: "home", "Home" }
                    TabsTrigger { value: "settings", disabled: true, "Disabled" }
                }
            }
        }
    }
}

#[component]
fn TabsIcons() -> Element {
    rsx! {
        Example { title: "Icons", class: "h-96",
            Tabs { default_value: "preview",
                TabsList {
                    TabsTrigger { value: "preview",
                        AppWindow {}
                        "Preview"
                    }
                    TabsTrigger { value: "code",
                        Code {}
                        "Code"
                    }
                }
            }
        }
    }
}

#[component]
fn TabsRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-96",
            Tabs { default_value: "overview", class: "w-full max-w-sm", dir: "rtl",
                TabsList { dir: "rtl",
                    for panel in PANELS_AR.iter() {
                        TabsTrigger { key: "{panel.value}", value: panel.value, "{panel.label}" }
                    }
                }
                for panel in PANELS_AR.iter() {
                    TabsContent { key: "{panel.value}", value: panel.value,
                        Card { dir: "rtl",
                            CardHeader {
                                CardTitle { "{panel.label}" }
                                CardDescription { "{panel.description}" }
                            }
                            CardContent { class: "text-sm text-muted-foreground", "{panel.content}" }
                        }
                    }
                }
            }
        }
    }
}
