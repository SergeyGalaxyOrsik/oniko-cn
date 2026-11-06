//! Chart: shadcn/ui's chart layer (`ChartContainer`, `ChartTooltip`, `ChartLegend`) over a small SVG bar chart.
//!
//! shadcn wraps recharts, which needs npm. Here the wrapper is ported as is (same classes, `data-slot`,
//! `--color-<key>` variables and `ChartStyle` output), and the part of recharts that the docs use is
//! written in this file: `BarChart` with `Bar`, `CartesianGrid`, `XAxis` and `YAxis`. It draws SVG with
//! recharts' class names (`recharts-cartesian-grid`, `recharts-bar-rectangle`, `recharts-tooltip-cursor`,
//! ...), so the overrides in `ChartContainer` apply unchanged. `Bar`, the axes, the grid, `ChartTooltip`
//! and `ChartLegend` draw nothing themselves: they register their props with the enclosing `BarChart`,
//! which lays everything out. The chart fills its `ChartContainer` and follows its size. With
//! `accessibility_layer`, focus the chart and use the arrow keys to move the active bar and its tooltip.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

/// Reports `true` whenever the element changes size.
const RESIZE_SCRIPT: &str = r#"
const el = document.getElementById("__ID__");
if (el) new ResizeObserver(() => dioxus.send(true)).observe(el);
"#;

const TOOLTIP_OFFSET: f64 = 10.0;
const BAR_GAP: f64 = 4.0;
const TICK_SIZE: f64 = 6.0;

fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

// MARK: Data

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Num(f64),
    Text(String),
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Num(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Num(f64::from(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl Value {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Num(value) => Some(*value),
            Self::Text(_) => None,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Num(value) => plain(*value),
            Self::Text(text) => text.clone(),
        }
    }
}

/// One row of chart data: a map from key to number or text.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Datum(BTreeMap<String, Value>);

impl Datum {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.0.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn text(&self, key: &str) -> Option<&str> {
        match self.0.get(key) {
            Some(Value::Text(text)) => Some(text),
            _ => None,
        }
    }
}

/// `1234.5` as `1,234.5`, like JavaScript's `toLocaleString()` in `en-US`.
fn to_locale(value: f64) -> String {
    let fixed = format!("{:.3}", value.abs());
    let fixed = fixed.trim_end_matches('0').trim_end_matches('.');
    let (int, frac) = fixed.split_once('.').unwrap_or((fixed, ""));
    let mut grouped = String::new();
    for (i, digit) in int.chars().enumerate() {
        if i > 0 && (int.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    let sign = if value < 0.0 { "-" } else { "" };
    if frac.is_empty() {
        format!("{sign}{grouped}")
    } else {
        format!("{sign}{grouped}.{frac}")
    }
}

/// `80.00000000000001` as `80`.
fn plain(value: f64) -> String {
    format!("{}", (value * 1e6).round() / 1e6)
}

// MARK: Config

const THEMES: [(&str, &str); 2] = [("light", ""), ("dark", ".dark")];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartTheme {
    pub light: String,
    pub dark: String,
}

/// A component without props, used as a series icon.
#[derive(Clone, Copy, Debug)]
pub struct ChartIcon(pub fn() -> Element);

impl PartialEq for ChartIcon {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.0, other.0)
    }
}

/// Label, icon and color (or light/dark colors) of one series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartConfigItem {
    pub label: Option<String>,
    pub icon: Option<ChartIcon>,
    pub color: Option<String>,
    pub theme: Option<ChartTheme>,
}

impl ChartConfigItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: fn() -> Element) -> Self {
        self.icon = Some(ChartIcon(icon));
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn theme(mut self, light: impl Into<String>, dark: impl Into<String>) -> Self {
        self.theme = Some(ChartTheme {
            light: light.into(),
            dark: dark.into(),
        });
        self
    }

    fn color_for(&self, theme: &str) -> Option<&str> {
        match (&self.theme, theme) {
            (Some(t), "light") => Some(&t.light),
            (Some(t), _) => Some(&t.dark),
            (None, _) => self.color.as_deref(),
        }
    }
}

/// Series settings by key, in the order they were added.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartConfig(Vec<(String, ChartConfigItem)>);

impl ChartConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, key: impl Into<String>, item: ChartConfigItem) -> Self {
        self.0.push((key.into(), item));
        self
    }

    pub fn get(&self, key: &str) -> Option<&ChartConfigItem> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, item)| item)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimension {
    pub width: f64,
    pub height: f64,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            width: 320.0,
            height: 200.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ChartContext {
    pub config: Signal<ChartConfig>,
    size: Signal<(f64, f64)>,
}

pub fn use_chart() -> ChartContext {
    use_context::<ChartContext>()
}

// MARK: Container

#[component]
pub fn ChartContainer(
    id: Option<String>,
    config: ChartConfig,
    #[props(default)] initial_dimension: Dimension,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-chart flex aspect-video justify-center text-xs [&_.recharts-cartesian-axis-tick_text]:fill-muted-foreground [&_.recharts-cartesian-grid_line[stroke='#ccc']]:stroke-border/50 [&_.recharts-curve.recharts-tooltip-cursor]:stroke-border [&_.recharts-dot[stroke='#fff']]:stroke-transparent [&_.recharts-layer]:outline-hidden [&_.recharts-polar-grid_[stroke='#ccc']]:stroke-border [&_.recharts-radial-bar-background-sector]:fill-muted [&_.recharts-rectangle.recharts-tooltip-cursor]:fill-muted [&_.recharts-reference-line_[stroke='#ccc']]:stroke-border [&_.recharts-sector]:outline-hidden [&_.recharts-sector[stroke='#fff']]:stroke-transparent [&_.recharts-surface]:outline-hidden",
        take_class(&mut attributes)
    );
    let unique = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let chart_id = format!("chart-{}", id.unwrap_or_else(|| unique.to_string()));
    let container_id = format!("{chart_id}-container");

    let config = use_prop(config);
    let mut size = use_signal(|| (initial_dimension.width, initial_dimension.height));
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_context_provider(|| ChartContext { config, size });

    let measure = move || {
        spawn(async move {
            let element = mounted.peek().clone();
            if let Some(element) = element
                && let Ok(rect) = element.get_client_rect().await
            {
                let next = (rect.width(), rect.height());
                if next.0 > 0.0 && next.1 > 0.0 && *size.peek() != next {
                    size.set(next);
                }
            }
        });
    };

    let observed = container_id.clone();
    use_effect(move || {
        let mut eval = document::eval(&RESIZE_SCRIPT.replace("__ID__", &observed));
        spawn(async move {
            while eval.recv::<bool>().await.is_ok() {
                measure();
            }
        });
    });

    rsx! {
        div {
            "data-slot": "chart",
            "data-chart": "{chart_id}",
            class: class,
            ..attributes,
            ChartStyle { id: chart_id.clone(), config: config() }
            div {
                id: container_id,
                class: "recharts-responsive-container",
                style: "width: 100%; height: 100%; min-width: 0;",
                onmounted: move |event| {
                    mounted.set(Some(event.data()));
                    measure();
                },
                {children}
            }
        }
    }
}

/// Defines `--color-<key>` for every series with a color, for light and dark mode.
#[component]
pub fn ChartStyle(id: String, config: ChartConfig) -> Element {
    let colored: Vec<_> = config
        .0
        .iter()
        .filter(|(_, item)| item.theme.is_some() || item.color.is_some())
        .collect();
    if colored.is_empty() {
        return rsx! {};
    }
    let css = THEMES
        .iter()
        .map(|(theme, prefix)| {
            let lines = colored
                .iter()
                .map(|(key, item)| {
                    item.color_for(theme)
                        .map(|color| format!("  --color-{key}: {color};"))
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("\n{prefix} [data-chart={id}] {{\n{lines}\n}}\n")
        })
        .collect::<Vec<_>>()
        .join("\n");

    rsx! {
        style { "{css}" }
    }
}

// MARK: Payload

/// One series at the hovered position (tooltip) or one legend entry.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartPayload {
    pub name: String,
    pub data_key: String,
    pub value: Option<Value>,
    pub color: String,
    /// The data row, `item.payload` upstream.
    pub payload: Datum,
}

/// Renders the tooltip label from the label text and the payload.
pub type LabelFormatter = Callback<(Option<String>, Vec<ChartPayload>), Element>;

/// Arguments of a tooltip `formatter`.
#[derive(Clone, Debug, PartialEq)]
pub struct FormatterArgs {
    pub value: Value,
    pub name: String,
    pub item: ChartPayload,
    pub index: usize,
    pub payload: Datum,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Indicator {
    #[default]
    Dot,
    Line,
    Dashed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VerticalAlign {
    Top,
    #[default]
    Bottom,
}

fn payload_config<'a>(
    config: &'a ChartConfig,
    item: &'a ChartPayload,
    key: &'a str,
) -> Option<&'a ChartConfigItem> {
    let named = match key {
        "name" => Some(item.name.as_str()),
        "dataKey" => Some(item.data_key.as_str()),
        "color" => Some(item.color.as_str()),
        _ => item.payload.text(key),
    };
    config.get(named.unwrap_or(key)).or_else(|| config.get(key))
}

// MARK: Tooltip

#[derive(Clone, Debug, Default, PartialEq)]
struct TooltipState {
    active: bool,
    label: Option<String>,
    payload: Vec<ChartPayload>,
    label_class_name: Option<String>,
}

#[derive(Clone, Copy)]
struct TooltipContext(Memo<TooltipState>);

/// Registers the tooltip with the enclosing chart. `content` defaults to `ChartTooltipContent {}`.
#[component]
pub fn ChartTooltip(
    content: Option<Element>,
    #[props(default = true)] cursor: bool,
    label_class_name: Option<String>,
) -> Element {
    use_part(
        TooltipSpec {
            content,
            cursor,
            label_class_name,
        },
        |parts, spec| parts.tooltip = spec,
    );
    rsx! {}
}

#[component]
pub fn ChartTooltipContent(
    class: Option<String>,
    #[props(default)] indicator: Indicator,
    #[props(default)] hide_label: bool,
    #[props(default)] hide_indicator: bool,
    label_formatter: Option<LabelFormatter>,
    label_class_name: Option<String>,
    formatter: Option<Callback<FormatterArgs, Element>>,
    color: Option<String>,
    name_key: Option<String>,
    label_key: Option<String>,
    /// Override what the chart reports, as recharts' `active`, `payload` and `label` props do.
    active: Option<bool>,
    payload: Option<Vec<ChartPayload>>,
    label: Option<String>,
) -> Element {
    let config = use_chart().config.read().clone();
    let state = try_use_context::<TooltipContext>()
        .map(|ctx| ctx.0.read().clone())
        .unwrap_or_default();
    let active = active.unwrap_or(state.active);
    let payload = payload.unwrap_or(state.payload);
    let label = label.or(state.label);
    let label_class_name = label_class_name.or(state.label_class_name);

    if !active || payload.is_empty() {
        return rsx! {};
    }

    let tooltip_label = (!hide_label)
        .then(|| {
            let item = &payload[0];
            let key = label_key
                .as_deref()
                .unwrap_or(if item.data_key.is_empty() {
                    &item.name
                } else {
                    &item.data_key
                });
            let item_config = payload_config(&config, item, key);
            let value = match (&label_key, &label) {
                (None, Some(label)) => Some(
                    config
                        .get(label)
                        .and_then(|c| c.label.clone())
                        .unwrap_or_else(|| label.clone()),
                ),
                _ => item_config.and_then(|c| c.label.clone()),
            };
            let class = cn!("font-medium", label_class_name.as_deref());
            if let Some(format) = label_formatter {
                let node = format.call((value, payload.clone()));
                Some(rsx! { div { class: class, {node} } })
            } else {
                value
                    .filter(|v| !v.is_empty())
                    .map(|value| rsx! { div { class: class, "{value}" } })
            }
        })
        .flatten();

    let nest_label = payload.len() == 1 && indicator != Indicator::Dot;

    rsx! {
        div { class: cn!("cn-chart-tooltip grid min-w-32 items-start", class),
            if !nest_label {
                {tooltip_label.clone()}
            }
            div { class: "grid gap-1.5",
                for (index , item) in payload.iter().enumerate() {
                    {
                        let key = name_key
                            .as_deref()
                            .unwrap_or(if item.name.is_empty() { &item.data_key } else { &item.name });
                        let item_config = payload_config(&config, item, key);
                        let indicator_color = color
                            .clone()
                            .or_else(|| item.payload.text("fill").map(str::to_string))
                            .unwrap_or_else(|| item.color.clone());
                        let row_class = cn!(
                            "flex w-full flex-wrap items-stretch gap-2 [&>svg]:h-2.5 [&>svg]:w-2.5 [&>svg]:text-muted-foreground",
                            (indicator == Indicator::Dot).then_some("items-center")
                        );
                        let indicator_class = cn!(
                            "shrink-0 rounded-[2px] border-(--color-border) bg-(--color-bg)",
                            match indicator {
                                Indicator::Dot => "h-2.5 w-2.5",
                                Indicator::Line => "w-1",
                                Indicator::Dashed => "w-0 border-[1.5px] border-dashed bg-transparent",
                            },
                            (nest_label && indicator == Indicator::Dashed).then_some("my-0.5")
                        );
                        let value_class = cn!(
                            "flex flex-1 justify-between leading-none",
                            if nest_label { "items-end" } else { "items-center" }
                        );
                        let name = item_config
                            .and_then(|c| c.label.clone())
                            .unwrap_or_else(|| item.name.clone());
                        let custom = formatter
                            .zip(item.value.clone())
                            .filter(|_| !item.name.is_empty())
                            .map(|(format, value)| {
                                format
                                    .call(FormatterArgs {
                                        value,
                                        name: item.name.clone(),
                                        item: item.clone(),
                                        index,
                                        payload: item.payload.clone(),
                                    })
                            });
                        rsx! {
                            div { key: "{index}", class: row_class,
                                if let Some(custom) = custom {
                                    {custom}
                                } else {
                                    if let Some(icon) = item_config.and_then(|c| c.icon) {
                                        {(icon.0)()}
                                    } else if !hide_indicator {
                                        div {
                                            class: indicator_class,
                                            style: "--color-bg: {indicator_color}; --color-border: {indicator_color};",
                                        }
                                    }
                                    div { class: value_class,
                                        div { class: "grid gap-1.5",
                                            if nest_label {
                                                {tooltip_label.clone()}
                                            }
                                            span { class: "text-muted-foreground", "{name}" }
                                        }
                                        if let Some(value) = &item.value {
                                            span { class: "font-mono font-medium text-foreground tabular-nums",
                                                match value {
                                                    Value::Num(n) => to_locale(*n),
                                                    Value::Text(t) => t.clone(),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// MARK: Legend

#[derive(Clone, Debug, Default, PartialEq)]
struct LegendState {
    payload: Vec<ChartPayload>,
    vertical_align: VerticalAlign,
}

#[derive(Clone, Copy)]
struct LegendContext(Memo<LegendState>);

/// Registers the legend with the enclosing chart. `content` defaults to `ChartLegendContent {}`.
#[component]
pub fn ChartLegend(
    content: Option<Element>,
    #[props(default)] vertical_align: VerticalAlign,
) -> Element {
    use_part(
        LegendSpec {
            content,
            vertical_align,
        },
        |parts, spec| parts.legend = spec,
    );
    rsx! {}
}

#[component]
pub fn ChartLegendContent(
    class: Option<String>,
    #[props(default)] hide_icon: bool,
    name_key: Option<String>,
    vertical_align: Option<VerticalAlign>,
    payload: Option<Vec<ChartPayload>>,
) -> Element {
    let config = use_chart().config.read().clone();
    let state = try_use_context::<LegendContext>()
        .map(|ctx| ctx.0.read().clone())
        .unwrap_or_default();
    let payload = payload.unwrap_or(state.payload);
    let vertical_align = vertical_align.unwrap_or(state.vertical_align);
    if payload.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            class: cn!(
                "flex items-center justify-center gap-4",
                if vertical_align == VerticalAlign::Top { "pb-3" } else { "pt-3" },
                class
            ),
            for (index , item) in payload.iter().enumerate() {
                {
                    let key = name_key.as_deref().unwrap_or(&item.data_key);
                    let item_config = payload_config(&config, item, key);
                    let icon = item_config.and_then(|c| c.icon).filter(|_| !hide_icon);
                    let label = item_config.and_then(|c| c.label.clone());
                    rsx! {
                        div {
                            key: "{index}",
                            class: "flex items-center gap-1.5 [&>svg]:h-3 [&>svg]:w-3 [&>svg]:text-muted-foreground",
                            if let Some(icon) = icon {
                                {(icon.0)()}
                            } else {
                                div {
                                    class: "h-2 w-2 shrink-0 rounded-[2px]",
                                    style: "background-color: {item.color};",
                                }
                            }
                            if let Some(label) = label {
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// MARK: Chart parts

#[derive(Clone, PartialEq)]
struct BarSpec {
    data_key: String,
    name: Option<String>,
    fill: String,
    radius: f64,
}

#[derive(Clone, PartialEq)]
struct GridSpec {
    horizontal: bool,
    vertical: bool,
}

#[derive(Clone, PartialEq)]
struct XAxisSpec {
    data_key: String,
    hide: bool,
    height: f64,
    tick_line: bool,
    axis_line: bool,
    tick_margin: f64,
    min_tick_gap: f64,
    reversed: bool,
    tick_formatter: Option<Callback<String, String>>,
}

#[derive(Clone, PartialEq)]
struct YAxisSpec {
    hide: bool,
    width: f64,
    tick_line: bool,
    axis_line: bool,
    tick_margin: f64,
    tick_formatter: Option<Callback<f64, String>>,
}

#[derive(Clone, PartialEq)]
struct TooltipSpec {
    content: Option<Element>,
    cursor: bool,
    label_class_name: Option<String>,
}

#[derive(Clone, PartialEq)]
struct LegendSpec {
    content: Option<Element>,
    vertical_align: VerticalAlign,
}

/// What the children of a `BarChart` registered, in place of recharts reading its React children.
#[derive(Clone, Default, PartialEq)]
struct Parts {
    bars: Vec<(u32, BarSpec)>,
    grid: Option<GridSpec>,
    x_axis: Option<XAxisSpec>,
    y_axis: Option<YAxisSpec>,
    tooltip: Option<TooltipSpec>,
    legend: Option<LegendSpec>,
}

#[derive(Clone, Copy)]
struct PartsContext(Signal<Parts>);

/// Registers `spec` with the enclosing `BarChart` while this component is mounted.
fn use_part<T: Clone + PartialEq + 'static>(
    spec: T,
    apply: impl Fn(&mut Parts, Option<T>) + Copy + 'static,
) {
    let mut parts = use_context::<PartsContext>().0;
    let spec = use_prop(spec);
    use_effect(move || {
        let spec = spec();
        if let Ok(mut parts) = parts.try_write() {
            apply(&mut parts, Some(spec));
        }
    });
    use_drop(move || {
        if let Ok(mut parts) = parts.try_write() {
            apply(&mut parts, None);
        }
    });
}

#[component]
pub fn Bar(
    data_key: String,
    name: Option<String>,
    #[props(default = "#8884d8".to_string())] fill: String,
    #[props(default)] radius: f64,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    use_part(
        BarSpec {
            data_key,
            name,
            fill,
            radius,
        },
        move |parts, spec| {
            parts.bars.retain(|(bar, _)| *bar != id);
            if let Some(spec) = spec {
                parts.bars.push((id, spec));
            }
        },
    );
    rsx! {}
}

#[component]
pub fn CartesianGrid(
    #[props(default = true)] horizontal: bool,
    #[props(default = true)] vertical: bool,
) -> Element {
    use_part(
        GridSpec {
            horizontal,
            vertical,
        },
        |parts, spec| parts.grid = spec,
    );
    rsx! {}
}

#[component]
pub fn XAxis(
    data_key: String,
    #[props(default)] hide: bool,
    #[props(default = 30.0)] height: f64,
    #[props(default = true)] tick_line: bool,
    #[props(default = true)] axis_line: bool,
    #[props(default = 2.0)] tick_margin: f64,
    #[props(default = 5.0)] min_tick_gap: f64,
    #[props(default)] reversed: bool,
    tick_formatter: Option<Callback<String, String>>,
) -> Element {
    use_part(
        XAxisSpec {
            data_key,
            hide,
            height,
            tick_line,
            axis_line,
            tick_margin,
            min_tick_gap,
            reversed,
            tick_formatter,
        },
        |parts, spec| parts.x_axis = spec,
    );
    rsx! {}
}

#[component]
pub fn YAxis(
    #[props(default)] hide: bool,
    #[props(default = 60.0)] width: f64,
    #[props(default = true)] tick_line: bool,
    #[props(default = true)] axis_line: bool,
    #[props(default = 2.0)] tick_margin: f64,
    tick_formatter: Option<Callback<f64, String>>,
) -> Element {
    use_part(
        YAxisSpec {
            hide,
            width,
            tick_line,
            axis_line,
            tick_margin,
            tick_formatter,
        },
        |parts, spec| parts.y_axis = spec,
    );
    rsx! {}
}

// MARK: Bar chart

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Margin {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: 5.0,
            right: 5.0,
            bottom: 5.0,
            left: 5.0,
        }
    }
}

/// The plot area, where bars and grid lines are drawn.
#[derive(Clone, Copy)]
struct Plot {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// Axis ticks that recharts would pick for `[0, max]` with five ticks.
fn nice_ticks(max: f64) -> Vec<f64> {
    if max <= 0.0 {
        return vec![0.0];
    }
    let rough = max / 4.0;
    let digits = rough.log10().floor() + 1.0;
    let unit = 10f64.powf(digits);
    let scale = if digits == 1.0 { 0.1 } else { 0.05 };
    let step = ((rough / unit / scale - 1e-9).ceil() * scale * unit * 1e6).round() / 1e6;
    let count = (max / step - 1e-9).ceil() as usize;
    (0..=count)
        .map(|i| (i as f64 * step * 1e6).round() / 1e6)
        .collect()
}

/// Which category labels fit: scanning from the last one, drop labels that would come closer
/// than `gap` to the previous kept one. Text widths are estimated; recharts measures them.
fn fitting_labels(labels: &[String], band: f64, gap: f64, reversed: bool) -> Vec<bool> {
    let width = |label: &String| label.chars().count() as f64 * 6.0;
    let mut visible = vec![false; labels.len()];
    let order: Vec<usize> = if reversed {
        (0..labels.len()).collect()
    } else {
        (0..labels.len()).rev().collect()
    };
    let mut last: Option<usize> = None;
    for i in order {
        let fits = last.is_none_or(|j| {
            let distance = band * (i as f64 - j as f64).abs();
            distance >= (width(&labels[i]) + width(&labels[j])) / 2.0 + gap
        });
        if fits {
            visible[i] = true;
            last = Some(i);
        }
    }
    visible
}

fn bar_path(x: f64, y: f64, width: f64, height: f64, radius: f64) -> String {
    let r = radius.min(width / 2.0).min(height / 2.0).max(0.0);
    let (w, h) = (width - 2.0 * r, height - 2.0 * r);
    format!(
        "M{},{y} h{w} a{r},{r},0,0,1,{r},{r} v{h} a{r},{r},0,0,1,-{r},{r} h-{w} a{r},{r},0,0,1,-{r},-{r} v-{h} a{r},{r},0,0,1,{r},-{r} Z",
        x + r
    )
}

#[component]
pub fn BarChart(
    data: Vec<Datum>,
    /// Makes the chart focusable; the arrow keys then move the active bar.
    #[props(default)]
    accessibility_layer: bool,
    #[props(default)] margin: Margin,
    children: Element,
) -> Element {
    let (width, height) = (use_chart().size)();
    let data = use_prop(data);
    let parts = use_signal(Parts::default);
    use_context_provider(|| PartsContext(parts));

    let mut active = use_signal(|| None::<usize>);
    let mut pointer_y = use_signal(|| 0.0);
    let mut shown = use_signal(|| false);
    let mut tip_mounted = use_signal(|| None::<Rc<MountedData>>);
    let mut tip_size = use_signal(|| (0.0, 0.0));
    let mut legend_height = use_signal(|| 28.0);

    let tooltip = use_memo(move || {
        let parts = parts.read();
        let data = data.read();
        let (Some(index), Some(row)) = (active(), active().and_then(|i| data.get(i))) else {
            return TooltipState::default();
        };
        let payload = parts
            .bars
            .iter()
            .map(|(_, bar)| ChartPayload {
                name: bar.name.clone().unwrap_or_else(|| bar.data_key.clone()),
                data_key: bar.data_key.clone(),
                value: row.get(&bar.data_key).cloned(),
                color: bar.fill.clone(),
                payload: row.clone(),
            })
            .collect();
        let label = parts
            .x_axis
            .as_ref()
            .and_then(|x| row.get(&x.data_key))
            .map_or_else(|| index.to_string(), Value::label);
        TooltipState {
            active: true,
            label: Some(label),
            payload,
            label_class_name: parts.tooltip.as_ref().and_then(|t| t.label_class_name.clone()),
        }
    });
    use_context_provider(|| TooltipContext(tooltip));

    let legend = use_memo(move || {
        let parts = parts.read();
        LegendState {
            payload: parts
                .bars
                .iter()
                .map(|(_, bar)| ChartPayload {
                    name: bar.name.clone().unwrap_or_else(|| bar.data_key.clone()),
                    data_key: bar.data_key.clone(),
                    value: None,
                    color: bar.fill.clone(),
                    payload: Datum::new(),
                })
                .collect(),
            vertical_align: parts.legend.as_ref().map(|l| l.vertical_align).unwrap_or_default(),
        }
    });
    use_context_provider(|| LegendContext(legend));

    // The tooltip's size decides on which side of the pointer it fits.
    use_effect(move || {
        active();
        spawn(async move {
            let element = tip_mounted.peek().clone();
            if let Some(element) = element
                && let Ok(rect) = element.get_client_rect().await
            {
                let next = (rect.width(), rect.height());
                if *tip_size.peek() != next {
                    tip_size.set(next);
                }
            }
        });
    });
    use_effect(move || {
        let visible = active().is_some();
        if *shown.peek() != visible {
            shown.set(visible);
        }
    });

    let parts_now = parts.read().clone();
    let data = data.read().clone();
    let count = data.len();
    let reversed = parts_now.x_axis.as_ref().is_some_and(|x| x.reversed);
    let x_axis = parts_now.x_axis.as_ref().filter(|x| !x.hide);
    let y_axis = parts_now.y_axis.as_ref().filter(|y| !y.hide);
    let legend_spec = parts_now.legend.as_ref();
    let legend_extent = legend_spec.map_or(0.0, |_| legend_height());
    let legend_on_top = legend_spec.is_some_and(|l| l.vertical_align == VerticalAlign::Top);
    let axis_height = x_axis.map_or(0.0, |x| x.height);
    let axis_width = y_axis.map_or(0.0, |y| y.width);
    let plot = Plot {
        x: margin.left + axis_width,
        y: margin.top + if legend_on_top { legend_extent } else { 0.0 },
        width: (width - margin.left - margin.right - axis_width).max(0.0),
        height: (height - margin.top - margin.bottom - axis_height - legend_extent).max(0.0),
    };

    let max = data
        .iter()
        .flat_map(|row| {
            parts_now
                .bars
                .iter()
                .filter_map(|(_, bar)| row.get(&bar.data_key).and_then(Value::as_f64))
        })
        .fold(0.0, f64::max);
    let ticks = nice_ticks(max);
    let top = ticks.last().copied().unwrap_or(0.0).max(1.0);
    let y_at = move |value: f64| plot.y + plot.height - value / top * plot.height;
    let band = if count == 0 { 0.0 } else { plot.width / count as f64 };
    let slot = move |index: usize| if reversed { count - 1 - index } else { index };
    let band_start = |index: usize| plot.x + slot(index) as f64 * band;

    let bar_count = parts_now.bars.len().max(1) as f64;
    let bar_width =
        ((band * 0.8 - (bar_count - 1.0) * BAR_GAP) / bar_count).max(0.0);

    let labels: Vec<String> = x_axis
        .map(|x| {
            data.iter()
                .enumerate()
                .map(|(i, row)| row.get(&x.data_key).map_or_else(|| i.to_string(), Value::label))
                .collect()
        })
        .unwrap_or_default();
    let visible_labels = x_axis
        .map(|x| fitting_labels(&labels, band, x.min_tick_gap, reversed))
        .unwrap_or_default();

    let tick_extent = |tick_line: bool| if tick_line { TICK_SIZE } else { 0.0 };
    let legend_edge = if legend_on_top {
        format!("top: {}px;", margin.top)
    } else {
        format!("bottom: {}px;", margin.bottom)
    };
    let current = active();
    let tooltip_spec = parts_now.tooltip.clone();
    let cursor_on = tooltip_spec.as_ref().is_some_and(|t| t.cursor);
    let (tip_w, tip_h) = tip_size();
    let anchor_x = current.map_or(0.0, |i| band_start(i) + band / 2.0);
    let anchor_y = pointer_y();
    let mut tip_x = anchor_x + TOOLTIP_OFFSET;
    if tip_x + tip_w > width {
        tip_x = anchor_x - tip_w - TOOLTIP_OFFSET;
    }
    let mut tip_y = anchor_y + TOOLTIP_OFFSET;
    if tip_y + tip_h > height {
        tip_y = anchor_y - tip_h - TOOLTIP_OFFSET;
    }
    let tip_style = format!(
        "pointer-events: none; position: absolute; top: 0; left: 0; visibility: {}; transform: translate({}px, {}px);{}",
        if current.is_some() { "visible" } else { "hidden" },
        tip_x.max(0.0),
        tip_y.max(0.0),
        if shown() && current.is_some() { " transition: transform 400ms ease;" } else { "" }
    );

    let mut move_to = move |index: usize, tallest: f64| {
        pointer_y.set(y_at(tallest));
        active.set(Some(index));
    };
    let tallest_in = {
        let data = data.clone();
        let keys: Vec<String> = parts_now.bars.iter().map(|(_, b)| b.data_key.clone()).collect();
        move |index: usize| {
            keys.iter()
                .filter_map(|key| data.get(index).and_then(|row| row.get(key)).and_then(Value::as_f64))
                .fold(0.0, f64::max)
        }
    };
    let mut on_key = {
        let tallest_in = tallest_in.clone();
        move |event: KeyboardEvent| {
            let forward = match event.key() {
                Key::ArrowRight => !reversed,
                Key::ArrowLeft => reversed,
                _ => return,
            };
            event.prevent_default();
            let index = active.peek().unwrap_or(0);
            let next = if forward { index + 1 } else { index.saturating_sub(1) };
            let next = next.min(count.saturating_sub(1));
            move_to(next, tallest_in(next));
        }
    };

    rsx! {
        div {
            class: "recharts-wrapper",
            style: "position: relative; cursor: default; width: {width}px; height: {height}px;",
            svg {
                class: "recharts-surface",
                width: "{width}",
                height: "{height}",
                view_box: "0 0 {width} {height}",
                role: if accessibility_layer { "application" } else { "" },
                tabindex: if accessibility_layer { "0" } else { "" },
                onfocus: move |_| {
                    if accessibility_layer && active.peek().is_none() && count > 0 {
                        move_to(slot(0), tallest_in(slot(0)));
                    }
                },
                onblur: move |_| active.set(None),
                onkeydown: move |event| {
                    if accessibility_layer {
                        on_key(event);
                    }
                },
                if let Some(grid) = &parts_now.grid {
                    g { class: "recharts-cartesian-grid",
                        if grid.horizontal {
                            g { class: "recharts-cartesian-grid-horizontal",
                                for (i , tick) in ticks.iter().enumerate() {
                                    line {
                                        key: "{i}",
                                        stroke: "#ccc",
                                        fill: "none",
                                        x: "{plot.x}",
                                        y: "{plot.y}",
                                        width: "{plot.width}",
                                        height: "{plot.height}",
                                        x1: "{plot.x}",
                                        x2: "{plot.x + plot.width}",
                                        y1: "{y_at(*tick)}",
                                        y2: "{y_at(*tick)}",
                                    }
                                }
                            }
                        }
                        if grid.vertical {
                            g { class: "recharts-cartesian-grid-vertical",
                                for i in 0..=count {
                                    line {
                                        key: "{i}",
                                        stroke: "#ccc",
                                        fill: "none",
                                        x: "{plot.x}",
                                        y: "{plot.y}",
                                        width: "{plot.width}",
                                        height: "{plot.height}",
                                        x1: "{plot.x + i as f64 * band}",
                                        x2: "{plot.x + i as f64 * band}",
                                        y1: "{plot.y}",
                                        y2: "{plot.y + plot.height}",
                                    }
                                }
                            }
                        }
                    }
                }
                if let (true, Some(index)) = (cursor_on, current) {
                    path {
                        class: "recharts-rectangle recharts-tooltip-cursor",
                        fill: "#ccc",
                        stroke: "none",
                        "pointer-events": "none",
                        d: bar_path(band_start(index), plot.y, band, plot.height, 0.0),
                    }
                }
                for (bar_index , (id , bar)) in parts_now.bars.iter().enumerate() {
                    g { key: "{id}", class: "recharts-layer recharts-bar",
                        g { class: "recharts-layer recharts-bar-rectangles",
                            for (i , row) in data.iter().enumerate() {
                                if let Some(value) = row.get(&bar.data_key).and_then(Value::as_f64) {
                                    g { key: "{i}", class: "recharts-layer recharts-bar-rectangle",
                                        path {
                                            class: "recharts-rectangle",
                                            fill: "{bar.fill}",
                                            d: bar_path(
                                                band_start(i) + band * 0.1 + bar_index as f64 * (bar_width + BAR_GAP),
                                                y_at(value),
                                                bar_width,
                                                value / top * plot.height,
                                                bar.radius,
                                            ),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(axis) = x_axis {
                    g { class: "recharts-layer recharts-cartesian-axis recharts-xAxis xAxis",
                        if axis.axis_line {
                            line {
                                class: "recharts-cartesian-axis-line",
                                stroke: "#666",
                                fill: "none",
                                x1: "{plot.x}",
                                x2: "{plot.x + plot.width}",
                                y1: "{plot.y + plot.height}",
                                y2: "{plot.y + plot.height}",
                            }
                        }
                        g { class: "recharts-cartesian-axis-ticks",
                            for (i , label) in labels.iter().enumerate() {
                                if visible_labels.get(i).copied().unwrap_or(false) {
                                    g { key: "{i}", class: "recharts-layer recharts-cartesian-axis-tick",
                                        if axis.tick_line {
                                            line {
                                                class: "recharts-cartesian-axis-tick-line",
                                                stroke: "#666",
                                                fill: "none",
                                                x1: "{band_start(i) + band / 2.0}",
                                                x2: "{band_start(i) + band / 2.0}",
                                                y1: "{plot.y + plot.height}",
                                                y2: "{plot.y + plot.height + TICK_SIZE}",
                                            }
                                        }
                                        text {
                                            class: "recharts-text recharts-cartesian-axis-tick-value",
                                            "orientation": "bottom",
                                            "text-anchor": "middle",
                                            width: "{band}",
                                            height: "{axis.height}",
                                            x: "{band_start(i) + band / 2.0}",
                                            y: "{plot.y + plot.height + tick_extent(axis.tick_line) + axis.tick_margin}",
                                            stroke: "none",
                                            fill: "#666",
                                            tspan {
                                                x: "{band_start(i) + band / 2.0}",
                                                dy: "0.71em",
                                                {
                                                    match axis.tick_formatter {
                                                        Some(format) => format.call(label.clone()),
                                                        None => label.clone(),
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(axis) = y_axis {
                    g { class: "recharts-layer recharts-cartesian-axis recharts-yAxis yAxis",
                        if axis.axis_line {
                            line {
                                class: "recharts-cartesian-axis-line",
                                stroke: "#666",
                                fill: "none",
                                x1: "{plot.x}",
                                x2: "{plot.x}",
                                y1: "{plot.y}",
                                y2: "{plot.y + plot.height}",
                            }
                        }
                        g { class: "recharts-cartesian-axis-ticks",
                            for (i , tick) in ticks.iter().enumerate() {
                                g { key: "{i}", class: "recharts-layer recharts-cartesian-axis-tick",
                                    if axis.tick_line {
                                        line {
                                            class: "recharts-cartesian-axis-tick-line",
                                            stroke: "#666",
                                            fill: "none",
                                            x1: "{plot.x}",
                                            x2: "{plot.x - TICK_SIZE}",
                                            y1: "{y_at(*tick)}",
                                            y2: "{y_at(*tick)}",
                                        }
                                    }
                                    text {
                                        class: "recharts-text recharts-cartesian-axis-tick-value",
                                        "orientation": "left",
                                        "text-anchor": "end",
                                        width: "{axis.width}",
                                        x: "{plot.x - tick_extent(axis.tick_line) - axis.tick_margin}",
                                        y: "{y_at(*tick)}",
                                        stroke: "none",
                                        fill: "#666",
                                        tspan {
                                            x: "{plot.x - tick_extent(axis.tick_line) - axis.tick_margin}",
                                            dy: "0.355em",
                                            {
                                                match axis.tick_formatter {
                                                    Some(format) => format.call(*tick),
                                                    None => plain(*tick),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if tooltip_spec.is_some() && count > 0 {
                div {
                    style: "position: absolute; left: {plot.x}px; top: {plot.y}px; width: {plot.width}px; height: {plot.height}px;",
                    onmousemove: move |event| {
                        let point = event.element_coordinates();
                        let index = ((point.x / band).floor().max(0.0) as usize).min(count - 1);
                        let index = if reversed { count - 1 - index } else { index };
                        pointer_y.set(plot.y + point.y);
                        if *active.peek() != Some(index) {
                            active.set(Some(index));
                        }
                    },
                    onmouseleave: move |_| active.set(None),
                }
            }
            if let Some(spec) = tooltip_spec {
                div {
                    class: "recharts-tooltip-wrapper",
                    style: tip_style,
                    onmounted: move |event| tip_mounted.set(Some(event.data())),
                    {spec.content.unwrap_or_else(|| rsx! { ChartTooltipContent {} })}
                }
            }
            if let Some(spec) = legend_spec {
                div {
                    class: "recharts-legend-wrapper",
                    style: "position: absolute; width: {width}px; height: auto; left: 0px; {legend_edge}",
                    onmounted: move |event| {
                        let element = event.data();
                        spawn(async move {
                            if let Ok(rect) = element.get_client_rect().await
                                && rect.height() > 0.0
                            {
                                legend_height.set(rect.height());
                            }
                        });
                    },
                    {spec.content.clone().unwrap_or_else(|| rsx! { ChartLegendContent {} })}
                }
            }
            {children}
        }
    }
}
