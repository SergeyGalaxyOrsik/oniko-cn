//! Chart examples, ported from shadcn/ui `examples/base/chart-*.tsx`.

use dioxus::prelude::*;
use oniko_cn::cn;

use super::Example;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::chart::{
    Bar, BarChart, CartesianGrid, ChartConfig, ChartConfigItem, ChartContainer, ChartLegend,
    ChartLegendContent, ChartTooltip, ChartTooltipContent, Datum, Margin, XAxis,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            ChartDemo {}
            ChartExample {}
            ChartExampleGrid {}
            ChartExampleAxis {}
            ChartExampleTooltip {}
            ChartExampleLegend {}
            ChartTooltipDemo {}
            ChartRtl {}
        }
    }
}

const DEMO_DATA: [(&str, i32, i32); 30] = [
    ("2024-04-01", 222, 150),
    ("2024-04-02", 97, 180),
    ("2024-04-03", 167, 120),
    ("2024-04-04", 242, 260),
    ("2024-04-05", 373, 290),
    ("2024-04-06", 301, 340),
    ("2024-04-07", 245, 180),
    ("2024-04-08", 409, 320),
    ("2024-04-09", 59, 110),
    ("2024-04-10", 261, 190),
    ("2024-04-11", 327, 350),
    ("2024-04-12", 292, 210),
    ("2024-04-13", 342, 380),
    ("2024-04-14", 137, 220),
    ("2024-04-15", 120, 170),
    ("2024-04-16", 138, 190),
    ("2024-04-17", 446, 360),
    ("2024-04-18", 364, 410),
    ("2024-04-19", 243, 180),
    ("2024-04-20", 89, 150),
    ("2024-04-21", 137, 200),
    ("2024-04-22", 224, 170),
    ("2024-04-23", 138, 230),
    ("2024-04-24", 387, 290),
    ("2024-04-25", 215, 250),
    ("2024-04-26", 75, 130),
    ("2024-04-27", 383, 420),
    ("2024-04-28", 122, 180),
    ("2024-04-29", 315, 240),
    ("2024-04-30", 454, 380),
];

const MONTHS: [(&str, i32, i32); 6] = [
    ("January", 186, 80),
    ("February", 305, 200),
    ("March", 237, 120),
    ("April", 73, 190),
    ("May", 209, 130),
    ("June", 214, 140),
];

fn month_data() -> Vec<Datum> {
    MONTHS
        .iter()
        .map(|(month, desktop, mobile)| {
            Datum::new()
                .with("month", *month)
                .with("desktop", *desktop)
                .with("mobile", *mobile)
        })
        .collect()
}

fn month_config() -> ChartConfig {
    ChartConfig::new()
        .item(
            "desktop",
            ChartConfigItem::new().label("Desktop").color("#2563eb"),
        )
        .item(
            "mobile",
            ChartConfigItem::new().label("Mobile").color("#60a5fa"),
        )
}

/// `2024-04-01` as `Apr 1`, or `Apr 1, 2024` with `year`.
fn format_date(value: &str, year: bool) -> String {
    const NAMES: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut parts = value
        .split('-')
        .map(|part| part.parse::<usize>().unwrap_or(1));
    let (y, m, d) = (
        parts.next().unwrap_or(1970),
        parts.next().unwrap_or(1),
        parts.next().unwrap_or(1),
    );
    let month = NAMES[m.clamp(1, 12) - 1];
    if year {
        format!("{month} {d}, {y}")
    } else {
        format!("{month} {d}")
    }
}

#[component]
fn ChartDemo() -> Element {
    let mut active_chart = use_signal(|| "desktop");
    let data: Vec<Datum> = DEMO_DATA
        .iter()
        .map(|(date, desktop, mobile)| {
            Datum::new()
                .with("date", *date)
                .with("desktop", *desktop)
                .with("mobile", *mobile)
        })
        .collect();
    let config = ChartConfig::new()
        .item("views", ChartConfigItem::new().label("Page Views"))
        .item(
            "desktop",
            ChartConfigItem::new()
                .label("Desktop")
                .color("var(--chart-2)"),
        )
        .item(
            "mobile",
            ChartConfigItem::new()
                .label("Mobile")
                .color("var(--chart-1)"),
        );
    let totals = [
        (
            "desktop",
            "Desktop",
            DEMO_DATA.iter().map(|r| r.1).sum::<i32>(),
        ),
        (
            "mobile",
            "Mobile",
            DEMO_DATA.iter().map(|r| r.2).sum::<i32>(),
        ),
    ];
    let active = active_chart();

    rsx! {
        Example { title: "Demo", class: "theme-blue",
            Card { class: "w-full py-0 pb-4",
                CardHeader { class: "flex flex-col items-stretch border-b p-0! sm:flex-row",
                    div { class: "flex flex-1 flex-col justify-center gap-1 px-6 pt-4 pb-3 sm:py-0!",
                        CardTitle { "Bar Chart - Interactive" }
                        CardDescription { "Showing total visitors for the last 3 months" }
                    }
                    div { class: "flex",
                        for (key , label , total) in totals {
                            button {
                                key: "{key}",
                                "data-active": "{active == key}",
                                class: "relative z-30 flex flex-1 flex-col justify-center gap-1 border-t px-6 py-4 text-left even:border-l data-[active=true]:bg-muted/50 sm:border-t-0 sm:border-l sm:px-8 sm:py-6",
                                onclick: move |_| active_chart.set(key),
                                span { class: "text-xs text-muted-foreground", "{label}" }
                                span { class: "text-lg leading-none font-bold sm:text-3xl",
                                    "{locale(total)}"
                                }
                            }
                        }
                    }
                }
                CardContent { class: "px-2 sm:p-6",
                    ChartContainer { config, class: "aspect-auto h-[250px] w-full",
                        BarChart {
                            accessibility_layer: true,
                            data,
                            margin: Margin {
                                left: 12.0,
                                right: 12.0,
                                ..Margin::default()
                            },
                            CartesianGrid { vertical: false }
                            XAxis {
                                data_key: "date",
                                tick_line: false,
                                axis_line: false,
                                tick_margin: 8.0,
                                min_tick_gap: 32.0,
                                tick_formatter: move |value: String| format_date(&value, false),
                            }
                            ChartTooltip {
                                content: rsx! {
                                    ChartTooltipContent {
                                        class: "w-[150px]",
                                        name_key: "views",
                                        label_formatter: move |(label, _): (Option<String>, _)| {
                                            let text = label.map(|l| format_date(&l, true)).unwrap_or_default();
                                            rsx! { "{text}" }
                                        },
                                    }
                                },
                            }
                            Bar { data_key: "{active}", fill: "var(--color-{active})" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ChartExample() -> Element {
    rsx! {
        Example { title: "Example",
            ChartContainer { config: month_config(), class: "min-h-[200px] w-full",
                BarChart { accessibility_layer: true, data: month_data(),
                    Bar {
                        data_key: "desktop",
                        fill: "var(--color-desktop)",
                        radius: 4.0,
                    }
                    Bar {
                        data_key: "mobile",
                        fill: "var(--color-mobile)",
                        radius: 4.0,
                    }
                }
            }
        }
    }
}

#[component]
fn ChartExampleGrid() -> Element {
    rsx! {
        Example { title: "Grid",
            ChartContainer { config: month_config(), class: "min-h-[200px] w-full",
                BarChart { accessibility_layer: true, data: month_data(),
                    CartesianGrid { vertical: false }
                    Bar {
                        data_key: "desktop",
                        fill: "var(--color-desktop)",
                        radius: 4.0,
                    }
                    Bar {
                        data_key: "mobile",
                        fill: "var(--color-mobile)",
                        radius: 4.0,
                    }
                }
            }
        }
    }
}

fn first_three(value: String) -> String {
    value.chars().take(3).collect()
}

#[component]
fn ChartExampleAxis() -> Element {
    rsx! {
        Example { title: "Axis",
            ChartContainer { config: month_config(), class: "min-h-[200px] w-full",
                BarChart { accessibility_layer: true, data: month_data(),
                    CartesianGrid { vertical: false }
                    XAxis {
                        data_key: "month",
                        tick_line: false,
                        tick_margin: 10.0,
                        axis_line: false,
                        tick_formatter: first_three,
                    }
                    Bar {
                        data_key: "desktop",
                        fill: "var(--color-desktop)",
                        radius: 4.0,
                    }
                    Bar {
                        data_key: "mobile",
                        fill: "var(--color-mobile)",
                        radius: 4.0,
                    }
                }
            }
        }
    }
}

#[component]
fn ChartExampleTooltip() -> Element {
    rsx! {
        Example { title: "Tooltip",
            ChartContainer { config: month_config(), class: "min-h-[200px] w-full",
                BarChart { accessibility_layer: true, data: month_data(),
                    CartesianGrid { vertical: false }
                    XAxis {
                        data_key: "month",
                        tick_line: false,
                        tick_margin: 10.0,
                        axis_line: false,
                        tick_formatter: first_three,
                    }
                    ChartTooltip { content: rsx! {
                        ChartTooltipContent {}
                    } }
                    Bar {
                        data_key: "desktop",
                        fill: "var(--color-desktop)",
                        radius: 4.0,
                    }
                    Bar {
                        data_key: "mobile",
                        fill: "var(--color-mobile)",
                        radius: 4.0,
                    }
                }
            }
        }
    }
}

#[component]
fn ChartExampleLegend() -> Element {
    rsx! {
        Example { title: "Legend",
            ChartContainer { config: month_config(), class: "min-h-[200px] w-full",
                BarChart { accessibility_layer: true, data: month_data(),
                    CartesianGrid { vertical: false }
                    XAxis {
                        data_key: "month",
                        tick_line: false,
                        tick_margin: 10.0,
                        axis_line: false,
                        tick_formatter: first_three,
                    }
                    ChartTooltip { content: rsx! {
                        ChartTooltipContent {}
                    } }
                    ChartLegend { content: rsx! {
                        ChartLegendContent {}
                    } }
                    Bar {
                        data_key: "desktop",
                        fill: "var(--color-desktop)",
                        radius: 4.0,
                    }
                    Bar {
                        data_key: "mobile",
                        fill: "var(--color-mobile)",
                        radius: 4.0,
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Indicator {
    Dot,
    Line,
    Dashed,
}

#[derive(Clone, PartialEq)]
struct TooltipRow {
    name: &'static str,
    value: i32,
    fill: &'static str,
}

#[component]
fn ChartTooltipDemo() -> Element {
    rsx! {
        Example { title: "Tooltip",
            div { class: "grid aspect-video w-full max-w-md justify-center text-foreground md:grid-cols-2 [&>div]:relative [&>div]:flex [&>div]:h-[137px] [&>div]:w-[224px] [&>div]:items-center [&>div]:justify-center [&>div]:p-4",
                div {
                    div { class: "absolute top-[45px] left-[-35px] z-10 text-sm font-medium",
                        "Label"
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 193 40",
                        width: "50",
                        height: "12",
                        fill: "none",
                        class: "absolute top-[50px] left-[5px] z-10",
                        g { clip_path: "url(#a)",
                            path { fill: "currentColor", d: "M173.928 21.13C115.811 44.938 58.751 45.773 0 26.141c4.227-4.386 7.82-2.715 10.567-1.88 21.133 5.64 42.9 6.266 64.457 7.101 31.066 1.253 60.441-5.848 89.183-17.335 1.268-.418 2.325-1.253 4.861-2.924-14.582-2.924-29.165 2.089-41.845-3.76.212-.835.212-1.879.423-2.714 9.51-.627 19.231-1.253 28.742-2.089 9.51-.835 18.808-1.88 28.318-2.506 6.974-.418 9.933 2.924 7.397 9.19-3.17 8.145-7.608 15.664-11.623 23.391-.423.836-1.057 1.88-1.902 2.298-2.325.835-4.65 1.044-7.186 1.67-.422-2.088-1.479-4.386-1.268-6.265.423-2.506 1.902-4.595 3.804-9.19Z" }
                        }
                        defs {
                            clipPath { id: "a",
                                path { fill: "currentColor", d: "M0 0h193v40H0z" }
                            }
                        }
                    }
                    TooltipDemo {
                        label: "Page Views",
                        rows: vec![
                            TooltipRow {
                                name: "Desktop",
                                value: 186,
                                fill: "var(--chart-1)",
                            },
                            TooltipRow {
                                name: "Mobile",
                                value: 80,
                                fill: "var(--chart-2)",
                            },
                        ],
                        class: "w-[8rem]",
                    }
                }
                div { class: "items-end",
                    div { class: "absolute top-[0px] left-[122px] z-10 text-sm font-medium",
                        "Name"
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "35",
                        height: "42",
                        fill: "none",
                        view_box: "0 0 122 148",
                        class: "absolute top-[10px] left-[85px] z-10 -scale-x-100",
                        g { clip_path: "url(#ab)",
                            path { fill: "currentColor", d: "M0 2.65c6.15-4.024 12.299-2.753 17.812-.847a115.56 115.56 0 0 1 21.84 10.59C70.4 32.727 88.849 61.744 96.483 97.54c1.908 9.108 2.544 18.639 3.817 29.017 8.481-4.871 12.934-14.402 21.416-19.909 1.061 4.236-1.06 6.989-2.756 9.319-6.998 9.531-14.207 19.062-21.63 28.382-3.604 4.448-6.36 4.871-10.177 1.059-8.058-7.837-12.935-17.368-14.42-28.382 0-.424.636-1.059 1.485-2.118 9.118 2.33 6.997 13.979 14.843 18.215 3.393-14.614.848-28.593-2.969-42.149-4.029-14.19-9.33-27.746-17.812-39.82-8.27-11.86-18.66-21.392-30.11-30.287C26.93 11.758 14.207 6.039 0 2.65Z" }
                        }
                        defs {
                            clipPath { id: "ab",
                                path { fill: "currentColor", d: "M0 0h122v148H0z" }
                            }
                        }
                    }
                    TooltipDemo {
                        label: "Browser",
                        hide_label: true,
                        rows: vec![
                            TooltipRow {
                                name: "Chrome",
                                value: 1286,
                                fill: "var(--chart-3)",
                            },
                            TooltipRow {
                                name: "Firefox",
                                value: 1000,
                                fill: "var(--chart-4)",
                            },
                        ],
                        indicator: Indicator::Dashed,
                        class: "w-[8rem]",
                    }
                }
                div { class: "hidden! md:flex!",
                    TooltipDemo {
                        label: "Page Views",
                        rows: vec![
                            TooltipRow {
                                name: "Desktop",
                                value: 12486,
                                fill: "var(--chart-3)",
                            },
                        ],
                        class: "w-[9rem]",
                        indicator: Indicator::Line,
                    }
                }
                div { class: "items-start! justify-start!",
                    div { class: "absolute top-[60px] left-[50px] z-10 text-sm font-medium",
                        "Indicator"
                    }
                    TooltipDemo {
                        label: "Browser",
                        hide_label: true,
                        rows: vec![
                            TooltipRow {
                                name: "Chrome",
                                value: 1286,
                                fill: "var(--chart-1)",
                            },
                        ],
                        indicator: Indicator::Dot,
                        class: "w-[8rem]",
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "15",
                        height: "34",
                        fill: "none",
                        view_box: "0 0 75 175",
                        class: "absolute top-[38px] left-[30px] z-10 rotate-[-40deg]",
                        g { clip_path: "url(#abc)",
                            path { fill: "currentColor", d: "M20.187 175c-4.439-2.109-7.186-2.531-8.032-4.008-3.17-5.484-6.763-10.968-8.454-17.084-5.073-16.242-4.439-32.694-1.057-49.146 5.707-28.053 18.388-52.942 34.24-76.565 1.692-2.531 3.171-5.063 4.862-7.805 0-.21-.211-.632-.634-1.265-4.65 1.265-9.511 2.53-14.161 3.585-2.537.422-5.496.422-8.032-.421-1.48-.422-3.593-2.742-3.593-4.219 0-1.898 1.48-4.218 2.747-5.906 1.057-1.054 2.96-1.265 4.65-1.687C35.406 7.315 48.088 3.729 60.98.776c10.99-2.53 14.584 1.055 13.95 11.812-.634 11.18-.846 22.358-1.268 33.326-.212 3.375-.846 6.96-1.268 10.757-8.878-4.007-8.878-4.007-12.048-38.177C47.03 33.259 38.153 49.289 29.91 65.741 21.667 82.193 16.17 99.49 13.212 117.84c-2.959 18.984.634 36.912 6.975 57.161Z" }
                        }
                        defs {
                            clipPath { id: "abc",
                                path { fill: "currentColor", d: "M0 0h75v175H0z" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Upstream's `TooltipDemo`: the markup of `ChartTooltipContent` with static rows.
#[component]
fn TooltipDemo(
    label: String,
    rows: Vec<TooltipRow>,
    #[props(default = Indicator::Dot)] indicator: Indicator,
    #[props(default)] hide_label: bool,
    #[props(default)] hide_indicator: bool,
    #[props(default)] class: String,
) -> Element {
    if rows.is_empty() {
        return rsx! {};
    }
    let nest_label = rows.len() == 1 && indicator != Indicator::Dot;
    let tooltip_label = (!hide_label).then(|| {
        rsx! {
            div { class: "font-medium", "{label}" }
        }
    });
    let class = cn!(
        "grid min-w-[8rem] items-start gap-1.5 rounded-lg border border-border/50 bg-background px-2.5 py-1.5 text-xs shadow-xl transition-all ease-in-out hover:-translate-y-0.5",
        class
    );

    rsx! {
        div { class: class,
            if !nest_label {
                {tooltip_label.clone()}
            }
            div { class: "grid gap-1.5",
                for (index , row) in rows.iter().enumerate() {
                    div {
                        key: "{index}",
                        class: cn!(
                            "flex w-full items-stretch gap-2 [&>svg]:h-2.5 [&>svg]:w-2.5 [&>svg]:text-muted-foreground",
                            (indicator == Indicator::Dot).then_some("items-center")
                        ),
                        if !hide_indicator {
                            div {
                                class: cn!(
                                    "shrink-0 rounded-[2px] border-(--color-border) bg-(--color-bg)",
                                    match indicator {
                                        Indicator::Dot => "h-2.5 w-2.5",
                                        Indicator::Line => "w-1",
                                        Indicator::Dashed => "w-0 border-[1.5px] border-dashed bg-transparent",
                                    },
                                    (nest_label && indicator == Indicator::Dashed).then_some("my-0.5")
                                ),
                                style: "--color-bg: {row.fill}; --color-border: {row.fill};",
                            }
                        }
                        div {
                            class: cn!(
                                "flex flex-1 justify-between leading-none",
                                if nest_label { "items-end" } else { "items-center" }
                            ),
                            div { class: "grid gap-1.5",
                                if nest_label {
                                    {tooltip_label.clone()}
                                }
                                span { class: "text-muted-foreground", "{row.name}" }
                            }
                            span { class: "font-mono font-medium text-foreground tabular-nums",
                                "{locale(row.value)}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn locale(value: i32) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (i, digit) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

const AR_MONTHS: [(&str, &str); 6] = [
    ("january", "يناير"),
    ("february", "فبراير"),
    ("march", "مارس"),
    ("april", "أبريل"),
    ("may", "مايو"),
    ("june", "يونيو"),
];

fn ar_month(key: &str) -> String {
    AR_MONTHS
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, label)| label.to_string())
        .unwrap_or_default()
}

#[component]
fn ChartRtl() -> Element {
    let config = ChartConfig::new()
        .item(
            "desktop",
            ChartConfigItem::new()
                .label("سطح المكتب")
                .color("var(--chart-2)"),
        )
        .item(
            "mobile",
            ChartConfigItem::new()
                .label("الجوال")
                .color("var(--chart-1)"),
        );
    let data = MONTHS
        .iter()
        .zip(AR_MONTHS)
        .map(|((_, desktop, mobile), (key, _))| {
            Datum::new()
                .with("month", key)
                .with("desktop", *desktop)
                .with("mobile", *mobile)
        })
        .collect::<Vec<_>>();

    rsx! {
        Example { title: "RTL",
            div { class: "w-full", dir: "rtl",
                ChartContainer { config, class: "min-h-[200px] w-full",
                    BarChart { accessibility_layer: true, data,
                        CartesianGrid { vertical: false }
                        XAxis {
                            data_key: "month",
                            tick_line: false,
                            tick_margin: 10.0,
                            axis_line: false,
                            tick_formatter: |value: String| ar_month(&value).chars().take(3).collect::<String>(),
                            reversed: true,
                        }
                        ChartTooltip {
                            content: rsx! {
                                ChartTooltipContent { label_formatter: move |(label, _): (Option<String>, _)| {
                                    let text = label.map(|l| ar_month(&l)).unwrap_or_default();
                                    rsx! { "{text}" }
                                } }
                            },
                            label_class_name: "w-32",
                        }
                        ChartLegend { content: rsx! {
                            ChartLegendContent {}
                        } }
                        Bar {
                            data_key: "desktop",
                            fill: "var(--color-desktop)",
                            radius: 4.0,
                        }
                        Bar {
                            data_key: "mobile",
                            fill: "var(--color-mobile)",
                            radius: 4.0,
                        }
                    }
                }
            }
        }
    }
}
