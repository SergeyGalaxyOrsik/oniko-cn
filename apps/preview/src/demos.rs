use crate::components::ui::button::{Button, ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::kbd::{Kbd, KbdGroup};
use crate::components::ui::label::Label;
use crate::components::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::components::ui::menubar::{
    Menubar, MenubarCheckboxItem, MenubarContent, MenubarGroup, MenubarItem, MenubarItemVariant,
    MenubarLabel, MenubarMenu, MenubarRadioGroup, MenubarRadioItem, MenubarSeparator,
    MenubarShortcut, MenubarSub, MenubarSubContent, MenubarSubTrigger, MenubarTrigger,
};
use crate::components::ui::message_scroller::{
    MessageScroller, MessageScrollerButton, MessageScrollerContent, MessageScrollerItem,
    MessageScrollerPosition, MessageScrollerProvider, MessageScrollerTarget,
    MessageScrollerViewport, use_message_scroller,
};
use crate::components::ui::native_select::{
    NativeSelect, NativeSelectOptGroup, NativeSelectOption, NativeSelectSize,
};
use crate::components::ui::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink,
    NavigationMenuList, NavigationMenuTrigger,
};
use crate::components::ui::pagination::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationNext, PaginationPrevious,
};
use crate::components::ui::popover::{
    Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverHeader, PopoverSide,
    PopoverTitle, PopoverTrigger,
};
use crate::components::ui::progress::{
    Progress, ProgressIndicator, ProgressLabel, ProgressTrack, ProgressValue,
};
use crate::components::ui::questionnaire::{
    Questionnaire, QuestionnaireActions, QuestionnaireChoice, QuestionnaireChoices,
    QuestionnaireDescription, QuestionnaireError, QuestionnaireInput, QuestionnaireItem,
    QuestionnaireNext, QuestionnairePrevious, QuestionnaireProgress, QuestionnaireSkip,
    QuestionnaireSubmit, QuestionnaireTitle,
};
use crate::components::ui::radio_group::{RadioGroup, RadioGroupItem};
use crate::components::ui::resizable::{
    ResizableDirection, ResizableHandle, ResizablePanel, ResizablePanelGroup,
};
use crate::components::ui::scroll_area::{ScrollArea, ScrollOrientation};
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger,
    SelectTriggerSize, SelectValue,
};
use crate::components::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetSide,
    SheetTitle, SheetTrigger,
};
use crate::components::ui::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton,
    SidebarMenuButtonSize, SidebarMenuButtonVariant, SidebarMenuItem, SidebarProvider,
    SidebarSeparator, SidebarSide, SidebarTrigger, SidebarVariant,
};
use crate::components::ui::skeleton::Skeleton;
use crate::components::ui::slider::{Slider, SliderOrientation};
use crate::components::ui::spinner::Spinner;
use crate::components::ui::switch::{Switch, SwitchSize};
use crate::components::ui::tabs::{
    Tabs, TabsContent, TabsList, TabsOrientation, TabsTrigger, TabsVariant,
};
use crate::components::ui::textarea::Textarea;
use crate::components::ui::toast::{ToastOptions, ToastProvider, ToastVariant, Toaster, use_toast};
use crate::components::ui::toggle::{Toggle, ToggleSize, ToggleVariant};
use crate::components::ui::toggle_group::{
    ToggleGroup, ToggleGroupItem, ToggleGroupOrientation, ToggleGroupType,
};
use crate::components::ui::tooltip::{
    Tooltip, TooltipAlign, TooltipContent, TooltipProvider, TooltipSide, TooltipTrigger,
};
use crate::components::ui::trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceDescription,
    TraceIcon, TraceLine, TraceStatus, TraceTitle,
};
use crate::components::ui::typeset::{
    Blockquote, H1, H2, H3, H4, InlineCode, List, ListItem, ListKind, P, ParagraphVariant, Typeset,
};
use dioxus::prelude::*;

const SECTION: &str = "flex flex-col gap-3";
const TITLE: &str = "text-sm font-medium text-muted-foreground";
const BOX: &str = "flex flex-wrap items-center gap-3 rounded-lg border p-6";

/// Sections for the components added in the second batch.
#[component]
pub fn MoreDemos() -> Element {
    rsx! {
        ToastProvider {
            Toaster {}
            SimpleDemos {}
            FormDemos {}
            OverlayDemos {}
            LayoutDemos {}
        }
    }
}

#[component]
fn SimpleDemos() -> Element {
    let toast = use_toast();
    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Kbd" }
            div { class: BOX,
                Kbd { "K" }
                KbdGroup {
                    Kbd { "Ctrl" }
                    Kbd { "Shift" }
                    Kbd { "P" }
                }
                KbdGroup {
                    Kbd { "⌘" }
                    Kbd { "K" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Marker" }
            div { class: "flex flex-col gap-3 rounded-lg border p-6",
                Marker {
                    MarkerIcon { "✓" }
                    MarkerContent { "Explored 4 files" }
                }
                Marker { variant: MarkerVariant::Border,
                    MarkerIcon { "i" }
                    MarkerContent { "Opened implementation notes" }
                }
                Marker { variant: MarkerVariant::Separator,
                    MarkerContent { "Today" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Progress" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                Progress { value: Some(33.0),
                    ProgressTrack { ProgressIndicator {} }
                }
                Progress { value: Some(56.0),
                    ProgressLabel { "Uploading" }
                    ProgressValue {}
                    ProgressTrack { ProgressIndicator {} }
                }
                Progress { value: Some(20.0), max: 40.0,
                    ProgressTrack { ProgressIndicator {} }
                }
                Progress { value: None,
                    ProgressTrack { ProgressIndicator {} }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Skeleton" }
            div { class: "flex items-center gap-4 rounded-lg border p-6",
                Skeleton { class: "size-12 rounded-full" }
                div { class: "flex flex-col gap-2",
                    Skeleton { class: "h-4 w-56" }
                    Skeleton { class: "h-4 w-40" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Spinner" }
            div { class: BOX,
                Spinner {}
                Spinner { class: "size-6" }
                Spinner { class: "size-8 text-muted-foreground" }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Toast" }
            div { class: BOX,
                for variant in [
                    ToastVariant::Default,
                    ToastVariant::Success,
                    ToastVariant::Info,
                    ToastVariant::Warning,
                    ToastVariant::Error,
                ] {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            let mut options = ToastOptions::new(format!("{} toast", variant.name()));
                            options.variant = variant;
                            toast.show(options);
                        },
                        "{variant.name()}"
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Typeset" }
            div { class: "rounded-lg border p-6",
                Typeset {
                    H1 { "Heading one" }
                    H2 { "Heading two" }
                    H3 { "Heading three" }
                    H4 { "Heading four" }
                    P { "Default paragraph with " InlineCode { "inline code" } "." }
                    P { variant: ParagraphVariant::Lead, "Lead paragraph." }
                    P { variant: ParagraphVariant::Large, "Large paragraph." }
                    P { variant: ParagraphVariant::Small, "Small paragraph." }
                    P { variant: ParagraphVariant::Muted, "Muted paragraph." }
                    Blockquote { "A quoted line." }
                    List {
                        ListItem { "Unordered item" }
                        ListItem { "Second item" }
                    }
                    List { kind: ListKind::Ordered,
                        ListItem { "First" }
                        ListItem { "Second" }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Trace" }
            div { class: "rounded-lg border p-6",
                Trace {
                    TraceLine {
                        TraceIcon { "⌕" }
                        TraceTitle { "Searched for minimumReleaseAge" }
                    }
                    TraceLine { status: TraceStatus::Running,
                        TraceIcon { "▶" }
                        TraceTitle { "Running the build" }
                    }
                    TraceLine { status: TraceStatus::Error,
                        TraceIcon { "!" }
                        TraceTitle { "Command failed" }
                        TraceDescription { "exit 1" }
                    }
                    TraceLine { status: TraceStatus::Pending,
                        TraceIcon { "…" }
                        TraceTitle { "Waiting for approval" }
                    }
                    TraceCollapsible {
                        TraceCollapsibleTrigger {
                            TraceLine {
                                TraceIcon { ">" }
                                TraceTitle { "Ran node --version" }
                                TraceDescription { "12 ms" }
                            }
                        }
                        TraceCollapsibleContent { "v24.18.0" }
                    }
                }
            }
        }
    }
}

#[component]
fn FormDemos() -> Element {
    let mut plan = use_signal(|| "free".to_string());
    let mut notifications = use_signal(|| true);
    let mut volume = use_signal(|| vec![40.0]);
    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Label and Textarea" }
            div { class: "flex flex-col gap-3 rounded-lg border p-6",
                Label { r#for: "notes", "Notes" }
                Textarea { id: "notes", placeholder: "Type your message here." }
                Textarea { placeholder: "Disabled", disabled: true }
                Textarea { placeholder: "Invalid", "aria-invalid": "true" }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Native Select" }
            div { class: "flex flex-col gap-3 rounded-lg border p-6",
                NativeSelect {
                    NativeSelectOption { value: "", "Select a fruit" }
                    NativeSelectOption { value: "apple", "Apple" }
                    NativeSelectOption { value: "banana", "Banana" }
                }
                NativeSelect { size: NativeSelectSize::Sm,
                    NativeSelectOption { value: "", "Small" }
                    NativeSelectOption { value: "a", "A" }
                }
                NativeSelect { disabled: true,
                    NativeSelectOption { value: "x", "Disabled" }
                }
                NativeSelect {
                    NativeSelectOptGroup { label: "Frontend",
                        NativeSelectOption { value: "react", "React" }
                        NativeSelectOption { value: "svelte", "Svelte" }
                    }
                    NativeSelectOptGroup { label: "Backend",
                        NativeSelectOption { value: "rust", "Rust" }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Select" }
            div { class: BOX,
                Select { default_value: "apple".to_string(),
                    SelectTrigger {
                        SelectValue { placeholder: "Pick a fruit" }
                    }
                    SelectContent {
                        SelectGroup {
                            SelectLabel { "Fruit" }
                            SelectItem { value: "apple", "Apple" }
                            SelectSeparator {}
                            SelectItem { value: "pear", "Pear" }
                            SelectItem { value: "plum", disabled: true, "Plum" }
                        }
                    }
                }
                Select { disabled: true,
                    SelectTrigger { size: SelectTriggerSize::Sm,
                        SelectValue { placeholder: "Disabled, small" }
                    }
                    SelectContent {
                        SelectItem { value: "x", "X" }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Radio Group" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                RadioGroup {
                    value: Some(plan()),
                    on_value_change: move |value: String| plan.set(value),
                    div { class: "flex items-center gap-2",
                        RadioGroupItem { id: "plan-free", value: "free" }
                        Label { r#for: "plan-free", "Free" }
                    }
                    div { class: "flex items-center gap-2",
                        RadioGroupItem { id: "plan-pro", value: "pro" }
                        Label { r#for: "plan-pro", "Pro" }
                    }
                }
                RadioGroup { default_value: "a".to_string(), disabled: true,
                    RadioGroupItem { value: "a" }
                    RadioGroupItem { value: "b" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Switch" }
            div { class: BOX,
                Switch {
                    checked: Some(notifications()),
                    on_checked_change: move |value| notifications.set(value),
                }
                Switch { default_checked: true }
                Switch { size: SwitchSize::Sm, default_checked: true }
                Switch { disabled: true }
                Switch { checked: Some(true), disabled: true }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Slider" }
            div { class: "flex flex-col gap-6 rounded-lg border p-6",
                Slider {
                    value: Some(volume()),
                    on_value_change: move |value: Vec<f64>| volume.set(value),
                }
                Slider { default_value: vec![20.0, 70.0] }
                Slider { default_value: vec![50.0], step: 10.0 }
                Slider { default_value: vec![40.0], disabled: true }
                div { class: "h-40",
                    Slider {
                        default_value: vec![30.0],
                        orientation: SliderOrientation::Vertical,
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Toggle" }
            div { class: BOX,
                for variant in [ToggleVariant::Default, ToggleVariant::Outline] {
                    for size in [ToggleSize::Sm, ToggleSize::Default, ToggleSize::Lg] {
                        Toggle { variant, size, default_pressed: true, "Aa" }
                    }
                }
                Toggle { disabled: true, "Disabled" }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Toggle Group" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                for kind in [ToggleGroupType::Single, ToggleGroupType::Multiple] {
                    for variant in [ToggleVariant::Default, ToggleVariant::Outline] {
                        ToggleGroup {
                            kind,
                            variant,
                            default_value: vec!["bold".to_string()],
                            ToggleGroupItem { value: "bold", "B" }
                            ToggleGroupItem { value: "italic", "I" }
                            ToggleGroupItem { value: "underline", disabled: true, "U" }
                        }
                    }
                }
                ToggleGroup { orientation: ToggleGroupOrientation::Vertical,
                    size: ToggleSize::Sm,
                    ToggleGroupItem { value: "left", "L" }
                    ToggleGroupItem { value: "center", "C" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Questionnaire" }
            div { class: "rounded-lg border p-6",
                Questionnaire {
                    QuestionnaireProgress {}
                    QuestionnaireItem { index: 0, name: "direction", required: true,
                        QuestionnaireTitle { "What should we prototype next?" }
                        QuestionnaireDescription { "Choose one direction." }
                        QuestionnaireChoices {
                            QuestionnaireChoice { value: "delegation", "Delegation" }
                            QuestionnaireChoice { value: "questions", "Question prompts" }
                        }
                        QuestionnaireError {}
                    }
                    QuestionnaireItem { index: 1, name: "features", multiple: true,
                        QuestionnaireTitle { "Which features matter?" }
                        QuestionnaireChoices {
                            QuestionnaireChoice { value: "a", "Alpha" }
                            QuestionnaireChoice { value: "b", "Beta" }
                        }
                    }
                    QuestionnaireItem { index: 2, name: "notes",
                        QuestionnaireTitle { "Anything else?" }
                        QuestionnaireInput { aria_label: "Notes", placeholder: "Type here" }
                    }
                    QuestionnaireActions {
                        QuestionnairePrevious { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Back" }
                        QuestionnaireSkip { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Skip" }
                        QuestionnaireNext { "Next" }
                        QuestionnaireSubmit { "Submit" }
                    }
                }
            }
        }
    }
}

#[component]
fn OverlayDemos() -> Element {
    let mut show_bar = use_signal(|| true);
    let mut theme = use_signal(|| "light".to_string());
    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Menubar" }
            div { class: "rounded-lg border p-6",
                Menubar {
                    MenubarMenu {
                        MenubarTrigger { "File" }
                        MenubarContent {
                            MenubarItem {
                                "New Tab"
                                MenubarShortcut { "⌘T" }
                            }
                            MenubarItem { variant: MenubarItemVariant::Destructive, "Delete" }
                            MenubarItem { inset: true, disabled: true, "Disabled" }
                            MenubarSeparator {}
                            MenubarLabel { "Label" }
                            MenubarGroup {
                                MenubarItem { "Grouped" }
                            }
                        }
                    }
                    MenubarMenu {
                        MenubarTrigger { "View" }
                        MenubarContent {
                            MenubarCheckboxItem {
                                checked: show_bar(),
                                onchecked_change: move |value| show_bar.set(value),
                                "Show bar"
                            }
                            MenubarSeparator {}
                            MenubarRadioGroup {
                                value: theme(),
                                on_value_change: move |value| theme.set(value),
                                MenubarRadioItem { value: "light".to_string(), "Light" }
                                MenubarRadioItem { value: "dark".to_string(), "Dark" }
                            }
                            MenubarSub {
                                MenubarSubTrigger { "Share" }
                                MenubarSubContent {
                                    MenubarItem { "Email" }
                                    MenubarItem { "Copy link" }
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Navigation Menu" }
            div { class: "min-h-40 rounded-lg border p-6",
                NavigationMenu {
                    NavigationMenuList {
                        NavigationMenuItem {
                            NavigationMenuTrigger { "Getting started" }
                            NavigationMenuContent {
                                NavigationMenuLink { href: "#intro", "Introduction" }
                                NavigationMenuLink { href: "#install", active: true, "Installation" }
                            }
                        }
                        NavigationMenuItem {
                            NavigationMenuTrigger { "Components" }
                            NavigationMenuContent {
                                NavigationMenuLink { href: "#button", "Button" }
                                NavigationMenuLink { href: "#card", "Card" }
                            }
                        }
                        NavigationMenuItem {
                            NavigationMenuLink { href: "#docs", "Docs" }
                        }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Popover" }
            div { class: BOX,
                for side in [PopoverSide::Top, PopoverSide::Right, PopoverSide::Bottom, PopoverSide::Left] {
                    Popover {
                        PopoverTrigger {
                            class: button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
                            "{side:?}"
                        }
                        PopoverContent { side, align: PopoverAlign::Center,
                            PopoverHeader {
                                PopoverTitle { "Dimensions" }
                                PopoverDescription { "Set the dimensions for the layer." }
                            }
                        }
                    }
                }
                Popover {
                    PopoverTrigger {
                        class: button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
                        "Align start"
                    }
                    PopoverContent { align: PopoverAlign::Start,
                        PopoverHeader {
                            PopoverTitle { "Aligned" }
                        }
                    }
                }
                Popover {
                    PopoverTrigger {
                        class: button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
                        "Align end"
                    }
                    PopoverContent { align: PopoverAlign::End,
                        PopoverHeader {
                            PopoverTitle { "Aligned" }
                        }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Tooltip" }
            div { class: BOX,
                TooltipProvider {
                    for side in [TooltipSide::Top, TooltipSide::Right, TooltipSide::Bottom, TooltipSide::Left] {
                        Tooltip {
                            TooltipTrigger {
                                Button { variant: ButtonVariant::Outline, "{side:?}" }
                            }
                            TooltipContent { side, "{side:?} tooltip" }
                        }
                    }
                    Tooltip {
                        TooltipTrigger {
                            Button { variant: ButtonVariant::Outline, "Align start" }
                        }
                        TooltipContent { align: TooltipAlign::Start, "Start aligned" }
                    }
                    Tooltip {
                        TooltipTrigger {
                            Button { variant: ButtonVariant::Outline, "Align end" }
                        }
                        TooltipContent { align: TooltipAlign::End, "End aligned" }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Sheet" }
            div { class: BOX,
                for (side , label) in [
                    (SheetSide::Top, "Top"),
                    (SheetSide::Right, "Right"),
                    (SheetSide::Bottom, "Bottom"),
                    (SheetSide::Left, "Left"),
                ] {
                    Sheet {
                        SheetTrigger { variant: ButtonVariant::Outline, "{label}" }
                        SheetContent { side,
                            SheetHeader {
                                SheetTitle { "Edit profile" }
                                SheetDescription { "Make changes to your profile." }
                            }
                            SheetFooter {
                                SheetClose { "Close" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarVariants() -> Element {
    rsx! {
        for (side , variant , collapsible , label) in [
            (SidebarSide::Right, SidebarVariant::Sidebar, SidebarCollapsible::Icon, "Right, icon"),
            (SidebarSide::Left, SidebarVariant::Floating, SidebarCollapsible::Offcanvas, "Floating"),
            (SidebarSide::Left, SidebarVariant::Inset, SidebarCollapsible::None, "Inset, fixed"),
        ] {
            div { class: "relative h-72 overflow-hidden rounded-lg border",
                SidebarProvider {
                    Sidebar { side, variant, collapsible,
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "{label}" }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        SidebarMenuItem {
                                            SidebarMenuButton { "Item" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    SidebarInset {
                        div { class: "p-4",
                            SidebarTrigger {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LayoutDemos() -> Element {
    rsx! {
        div { class: SECTION,
            h2 { class: TITLE, "Tabs" }
            div { class: "flex flex-col gap-6 rounded-lg border p-6",
                Tabs { default_value: "account".to_string(), class: "w-full",
                    TabsList {
                        TabsTrigger { value: "account".to_string(), "Account" }
                        TabsTrigger { value: "password".to_string(), "Password" }
                    }
                    TabsContent { value: "account".to_string(), "Account panel" }
                    TabsContent { value: "password".to_string(), "Password panel" }
                }
                Tabs { default_value: "overview".to_string(), class: "w-full",
                    TabsList { variant: TabsVariant::Line,
                        TabsTrigger { value: "overview".to_string(), "Overview" }
                        TabsTrigger { value: "analytics".to_string(), "Analytics" }
                    }
                    TabsContent { value: "overview".to_string(), "Overview panel" }
                    TabsContent { value: "analytics".to_string(), "Analytics panel" }
                }
                Tabs {
                    default_value: "one".to_string(),
                    orientation: TabsOrientation::Vertical,
                    class: "w-full",
                    TabsList {
                        TabsTrigger { value: "one".to_string(), "One" }
                        TabsTrigger { value: "two".to_string(), "Two" }
                    }
                    TabsContent { value: "one".to_string(), "Vertical one" }
                    TabsContent { value: "two".to_string(), "Vertical two" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Pagination" }
            div { class: "rounded-lg border p-6",
                Pagination {
                    PaginationContent {
                        PaginationItem {
                            PaginationPrevious { href: "#" }
                        }
                        PaginationItem {
                            PaginationLink { href: "#", "1" }
                        }
                        PaginationItem {
                            PaginationLink { href: "#", is_active: true, "2" }
                        }
                        PaginationItem {
                            PaginationLink { href: "#", "3" }
                        }
                        PaginationItem {
                            PaginationEllipsis {}
                        }
                        PaginationItem {
                            PaginationNext { href: "#" }
                        }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Scroll Area" }
            div { class: "flex flex-wrap gap-4 rounded-lg border p-6",
                ScrollArea { class: "h-40 w-64 rounded-md border p-4",
                    for i in 1..=30 {
                        p { class: "text-sm", "Line {i}" }
                    }
                }
                ScrollArea {
                    orientation: ScrollOrientation::Horizontal,
                    class: "w-64 rounded-md border p-4",
                    div { class: "flex w-max gap-4",
                        for i in 1..=12 {
                            div { class: "size-16 shrink-0 rounded-md bg-muted", "{i}" }
                        }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Scroll Area (both axes)" }
            div { class: "rounded-lg border p-6",
                ScrollArea {
                    orientation: ScrollOrientation::Both,
                    class: "h-40 w-64 rounded-md border p-4",
                    div { class: "h-80 w-[600px] rounded-md bg-muted p-4", "Scrolls in both directions" }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Resizable" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                div { class: "h-40 w-full rounded-md border",
                    ResizablePanelGroup { direction: ResizableDirection::Horizontal,
                        ResizablePanel { default_size: 50.0, min_size: 20.0,
                            div { class: "p-4", "One" }
                        }
                        ResizableHandle { with_handle: true }
                        ResizablePanel { default_size: 50.0,
                            div { class: "p-4", "Two" }
                        }
                    }
                }
                div { class: "h-48 w-full rounded-md border",
                    ResizablePanelGroup { direction: ResizableDirection::Vertical,
                        ResizablePanel { default_size: 30.0, max_size: 80.0,
                            div { class: "p-4", "Top" }
                        }
                        ResizableHandle {}
                        ResizablePanel {
                            div { class: "p-4", "Bottom" }
                        }
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Message Scroller" }
            div { class: "flex flex-col gap-4 rounded-lg border p-6",
                for (position , label) in [
                    (MessageScrollerPosition::Start, "start"),
                    (MessageScrollerPosition::End, "end"),
                    (MessageScrollerPosition::LastAnchor, "last-anchor"),
                ] {
                    MessageScrollerProvider { default_scroll_position: position,
                        MessageScroller { class: "relative h-40 rounded-md border",
                            MessageScrollerViewport {
                                MessageScrollerContent {
                                    MessageScrollerItem { message_id: "1", "Hello ({label})" }
                                    MessageScrollerItem { message_id: "2", scroll_anchor: true,
                                        "Anchored turn"
                                    }
                                    for i in 3..=12 {
                                        MessageScrollerItem { message_id: "{i}", "Reply {i}" }
                                    }
                                }
                            }
                            MessageScrollerButton { target: MessageScrollerTarget::End, "Latest" }
                            MessageScrollerButton { target: MessageScrollerTarget::Start, "Top" }
                        }
                        ScrollToMessageTwo {}
                    }
                }
            }
        }

        div { class: SECTION,
            h2 { class: TITLE, "Sidebar" }
            div { class: "relative h-96 overflow-hidden rounded-lg border",
                SidebarProvider {
                    Sidebar {
                        side: SidebarSide::Left,
                        variant: SidebarVariant::Sidebar,
                        collapsible: SidebarCollapsible::Offcanvas,
                        SidebarHeader { "Header" }
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Application" }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        SidebarMenuItem {
                                            SidebarMenuButton { is_active: true, "Home" }
                                        }
                                        SidebarMenuItem {
                                            SidebarMenuButton {
                                                variant: SidebarMenuButtonVariant::Outline,
                                                size: SidebarMenuButtonSize::Sm,
                                                "Outline small"
                                            }
                                        }
                                        SidebarMenuItem {
                                            SidebarMenuButton { size: SidebarMenuButtonSize::Lg, "Large" }
                                        }
                                    }
                                }
                            }
                        }
                        SidebarSeparator {}
                        SidebarFooter { "Footer" }
                    }
                    SidebarInset {
                        div { class: "p-4",
                            SidebarTrigger {}
                            p { class: "mt-2 text-sm", "Main content" }
                        }
                    }
                }
            }
            SidebarVariants {}
        }
    }
}

#[component]
fn ScrollToMessageTwo() -> Element {
    let scroller = use_message_scroller();
    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| scroller.scroll_to_message("2"),
            "Scroll to message 2"
        }
        Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| scroller.scroll_to_start(),
            "Scroll to start"
        }
        Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| scroller.scroll_to_end(),
            "Scroll to end"
        }
    }
}
