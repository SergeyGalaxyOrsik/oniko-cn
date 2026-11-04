mod components;

use components::ui::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use components::ui::alert::{Alert, AlertAction, AlertDescription, AlertTitle, AlertVariant};
use components::ui::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogSize, AlertDialogTitle,
    AlertDialogTrigger,
};
use components::ui::aspect_ratio::AspectRatio;
use components::ui::avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
};
use components::ui::badge::{Badge, BadgeVariant, badge_variants};
use components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};
use components::ui::bubble::{
    Bubble, BubbleAlign, BubbleContent, BubbleContentButton, BubbleContentLink, BubbleGroup,
    BubbleReactions, BubbleReactionsSide, BubbleVariant,
};
use components::ui::button::{Button, ButtonSize, ButtonVariant};
use components::ui::button_group::{
    ButtonGroup, ButtonGroupOrientation, ButtonGroupSeparator, ButtonGroupText,
};
use components::ui::calendar::{Calendar, CalendarDate};
use components::ui::card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
use components::ui::carousel::{
    Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselOrientation, CarouselPrevious,
};
use components::ui::checkbox::Checkbox;
use components::ui::code_block::{
    CodeBlock, CodeBlockActions, CodeBlockContent, CodeBlockCopyButton, CodeBlockFooter,
    CodeBlockHeader, CodeBlockLine, CodeBlockLineVariant, CodeBlockSize, CodeBlockTitle,
    CodeBlockVariant,
};
use components::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use components::ui::combobox::{
    Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxLabel,
    ComboboxList,
};
use components::ui::command::{
    Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandSeparator,
    CommandShortcut,
};
use components::ui::context_menu::{
    ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem,
    ContextMenuItemVariant, ContextMenuLabel, ContextMenuRadioGroup, ContextMenuRadioItem,
    ContextMenuSeparator, ContextMenuShortcut, ContextMenuTrigger,
};
use components::ui::data_table::{DataColumn, DataTable};
use components::ui::date_picker::DatePicker;
use components::ui::dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use components::ui::drawer::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerDirection, DrawerFooter,
    DrawerHeader, DrawerTitle, DrawerTrigger,
};
use components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuCheckboxItem, DropdownMenuContent,
    DropdownMenuGroup, DropdownMenuItem, DropdownMenuItemVariant, DropdownMenuLabel,
    DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuShortcut,
    DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger,
};
use components::ui::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use components::ui::field::{
    Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend,
    FieldLegendVariant, FieldOrientation, FieldSeparator, FieldSet, FieldTitle,
};
use components::ui::hover_card::{
    HoverCard, HoverCardAlign, HoverCardContent, HoverCardSide, HoverCardTrigger,
};
use components::ui::input::Input;
use components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupInput, InputGroupText, InputGroupTextarea,
};
use components::ui::input_otp::{
    InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot, OtpPattern,
};
use components::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant,
};
use components::ui::separator::{Separator, SeparatorOrientation};
use components::ui::table::{
    Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow,
};
use dioxus::prelude::*;

const TAILWIND: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut dark = use_signal(|| false);
    let mut clicks = use_signal(|| 0);
    let mut collapsible_open = use_signal(|| false);
    let mut terms_accepted = use_signal(|| false);
    let mut framework = use_signal(|| "dioxus".to_string());

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

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Badge" }
                        div { class: "flex flex-wrap items-center gap-2 rounded-lg border p-6",
                            Badge { "Default" }
                            Badge { variant: BadgeVariant::Secondary, "Secondary" }
                            Badge { variant: BadgeVariant::Destructive, "Destructive" }
                            Badge { variant: BadgeVariant::Outline, "Outline" }
                            Badge { variant: BadgeVariant::Ghost, "Ghost" }
                            Badge { variant: BadgeVariant::Link, "Link" }
                            Badge { variant: BadgeVariant::Secondary, InfoIcon {} "With icon" }
                            Badge { variant: BadgeVariant::Outline, aria_invalid: "true", "Invalid" }
                            Badge { class: "rounded-md", "Class override" }
                            a { href: "#", class: badge_variants(BadgeVariant::Default, None), "As link" }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Alert" }
                        div { class: "flex flex-col gap-3 rounded-lg border p-6",
                            Alert {
                                InfoIcon {}
                                AlertTitle { "Heads up!" }
                                AlertDescription { "You can add components to your app using the cli." }
                            }
                            Alert { variant: AlertVariant::Destructive,
                                InfoIcon {}
                                AlertTitle { "Unable to process your payment." }
                                AlertDescription { "Please verify your billing information and try again." }
                            }
                            Alert {
                                AlertTitle { "Without an icon" }
                                AlertDescription { "Alerts also work with a title and description only." }
                            }
                            Alert {
                                InfoIcon {}
                                AlertTitle { "With an action" }
                                AlertDescription { "A button can sit in the top right corner." }
                                AlertAction {
                                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Xs, "Undo" }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Breadcrumb" }
                        div { class: "flex flex-col gap-6 rounded-lg border p-6",
                            Breadcrumb {
                                BreadcrumbList {
                                    BreadcrumbItem { BreadcrumbLink { href: "#", "Home" } }
                                    BreadcrumbSeparator {}
                                    BreadcrumbItem { BreadcrumbLink { href: "#", "Components" } }
                                    BreadcrumbSeparator {}
                                    BreadcrumbItem { BreadcrumbPage { "Breadcrumb" } }
                                }
                            }
                            Breadcrumb {
                                BreadcrumbList {
                                    BreadcrumbItem { BreadcrumbLink { href: "#", "Home" } }
                                    BreadcrumbSeparator {}
                                    BreadcrumbItem { BreadcrumbEllipsis {} }
                                    BreadcrumbSeparator {}
                                    BreadcrumbItem { BreadcrumbLink { href: "#", "Components" } }
                                    BreadcrumbSeparator {}
                                    BreadcrumbItem { BreadcrumbPage { "Breadcrumb" } }
                                }
                            }
                            Breadcrumb {
                                BreadcrumbList {
                                    BreadcrumbItem { BreadcrumbLink { href: "#", "Home" } }
                                    BreadcrumbSeparator { "/" }
                                    BreadcrumbItem { BreadcrumbLink { href: "#", "Docs" } }
                                    BreadcrumbSeparator { "/" }
                                    BreadcrumbItem { BreadcrumbPage { "Custom separator" } }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Button Group" }
                        div { class: "flex flex-col items-start gap-6 rounded-lg border p-6",
                            ButtonGroup {
                                Button { variant: ButtonVariant::Outline, "Archive" }
                                Button { variant: ButtonVariant::Outline, "Report" }
                                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon, aria_label: "More", MoonIcon {} }
                            }
                            ButtonGroup {
                                ButtonGroupText { "Sort by" }
                                Button { variant: ButtonVariant::Outline, "Date" }
                                ButtonGroupSeparator {}
                                Button { variant: ButtonVariant::Outline, "Name" }
                            }
                            ButtonGroup { orientation: ButtonGroupOrientation::Vertical,
                                Button { variant: ButtonVariant::Outline, "Top" }
                                ButtonGroupSeparator { orientation: ButtonGroupOrientation::Horizontal }
                                Button { variant: ButtonVariant::Outline, "Bottom" }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Aspect Ratio" }
                        div { class: "flex flex-wrap items-start gap-4 rounded-lg border p-6",
                            div { class: "w-64",
                                AspectRatio { ratio: 16.0 / 9.0, class: "flex items-center justify-center rounded-lg bg-muted text-sm text-muted-foreground", "16:9" }
                            }
                            div { class: "w-32",
                                AspectRatio { ratio: 1.0, class: "flex items-center justify-center rounded-lg bg-muted text-sm text-muted-foreground", "1:1" }
                            }
                            div { class: "w-24",
                                AspectRatio { ratio: 9.0 / 16.0, class: "flex items-center justify-center rounded-lg bg-muted text-sm text-muted-foreground", "9:16" }
                            }
                        }
                    }

                    Section { title: "Dialog",
                        Dialog {
                            DialogTrigger { variant: ButtonVariant::Outline, "Open dialog" }
                            DialogContent {
                                DialogHeader {
                                    DialogTitle { "Edit profile" }
                                    DialogDescription { "Make changes to your profile here. Click save when you're done." }
                                }
                                DialogFooter {
                                    DialogClose { variant: ButtonVariant::Outline, size: ButtonSize::Default, "Done" }
                                }
                            }
                        }
                        Dialog {
                            DialogTrigger { variant: ButtonVariant::Secondary, "Without close button" }
                            DialogContent { show_close_button: false,
                                DialogHeader {
                                    DialogTitle { "Keep going?" }
                                    DialogDescription { "Close with Escape or a click outside the window." }
                                }
                            }
                        }
                    }

                    Section { title: "Drawer",
                        for (label, direction) in [
                            ("Bottom", DrawerDirection::Down),
                            ("Top", DrawerDirection::Up),
                            ("Left", DrawerDirection::Left),
                            ("Right", DrawerDirection::Right),
                        ] {
                            Drawer { direction,
                                DrawerTrigger { variant: ButtonVariant::Outline, "{label}" }
                                DrawerContent {
                                    DrawerHeader {
                                        DrawerTitle { "Edit profile" }
                                        DrawerDescription { "Make changes to your profile here. Click save when you're done." }
                                    }
                                    div { class: "flex-1 overflow-y-auto p-4 text-sm", "Drawer body. Press Escape or click the overlay to close." }
                                    DrawerFooter {
                                        DrawerClose { variant: ButtonVariant::Default, "Save" }
                                        DrawerClose { "Cancel" }
                                    }
                                }
                            }
                        }
                        Drawer { show_handle: true,
                            DrawerTrigger { variant: ButtonVariant::Secondary, "With handle" }
                            DrawerContent {
                                DrawerHeader {
                                    DrawerTitle { "Share" }
                                    DrawerDescription { "Anyone with the link can view." }
                                }
                                DrawerFooter {
                                    DrawerClose { "Done" }
                                }
                            }
                        }
                    }

                    Section { title: "Input OTP",
                        InputOTP { max_length: 6,
                            InputOTPGroup {
                                InputOTPSlot { index: 0 }
                                InputOTPSlot { index: 1 }
                                InputOTPSlot { index: 2 }
                            }
                            InputOTPSeparator {}
                            InputOTPGroup {
                                InputOTPSlot { index: 3 }
                                InputOTPSlot { index: 4 }
                                InputOTPSlot { index: 5 }
                            }
                        }
                        InputOTP { max_length: 4, pattern: OtpPattern::Digits,
                            InputOTPGroup {
                                InputOTPSlot { index: 0 }
                                InputOTPSlot { index: 1 }
                                InputOTPSlot { index: 2 }
                                InputOTPSlot { index: 3 }
                            }
                        }
                        InputOTP { max_length: 6, pattern: OtpPattern::DigitsAndChars,
                            InputOTPGroup {
                                InputOTPSlot { index: 0, invalid: true }
                                InputOTPSlot { index: 1, invalid: true }
                                InputOTPSlot { index: 2, invalid: true }
                            }
                            InputOTPSeparator {}
                            InputOTPGroup {
                                InputOTPSlot { index: 3, invalid: true }
                                InputOTPSlot { index: 4, invalid: true }
                                InputOTPSlot { index: 5, invalid: true }
                            }
                        }
                        InputOTP { max_length: 6, disabled: true, default_value: "123456".to_string(),
                            InputOTPGroup {
                                InputOTPSlot { index: 0 }
                                InputOTPSlot { index: 1 }
                                InputOTPSlot { index: 2 }
                            }
                            InputOTPSeparator {}
                            InputOTPGroup {
                                InputOTPSlot { index: 3 }
                                InputOTPSlot { index: 4 }
                                InputOTPSlot { index: 5 }
                            }
                        }
                        OtpControlled {}
                    }

                    Section { title: "Checkbox",
                        Checkbox {}
                        Checkbox { default_checked: true }
                        Checkbox { disabled: true }
                        Checkbox { disabled: true, default_checked: true }
                        Checkbox {
                            checked: terms_accepted(),
                            on_checked_change: move |next| terms_accepted.set(next),
                        }
                        span { class: "text-sm text-muted-foreground",
                            if terms_accepted() { "Accepted" } else { "Not accepted" }
                        }
                    }

                    Section { title: "Combobox",
                        Combobox { class: "max-w-xs",
                            ComboboxInput { placeholder: "Search a framework..." }
                            ComboboxContent {
                                ComboboxEmpty { "No framework found." }
                                ComboboxList {
                                    ComboboxLabel { "Frameworks" }
                                    ComboboxItem { value: "dioxus", label: "Dioxus", "Dioxus" }
                                    ComboboxItem { value: "leptos", label: "Leptos", "Leptos" }
                                    ComboboxItem { value: "yew", label: "Yew", "Yew" }
                                    ComboboxItem { value: "sycamore", label: "Sycamore", "Sycamore" }
                                }
                            }
                        }
                        Combobox { class: "max-w-xs", default_value: Some("leptos".to_string()), disabled: true,
                            ComboboxInput { placeholder: "Disabled" }
                        }
                        Combobox {
                            class: "max-w-xs",
                            value: Some(framework()),
                            on_value_change: move |next| framework.set(next),
                            ComboboxInput { placeholder: "Controlled..." }
                            ComboboxContent {
                                ComboboxEmpty { "No framework found." }
                                ComboboxList {
                                    ComboboxItem { value: "dioxus", label: "Dioxus", "Dioxus" }
                                    ComboboxItem { value: "leptos", label: "Leptos", "Leptos" }
                                    ComboboxItem { value: "yew", label: "Yew", "Yew" }
                                }
                            }
                        }
                        span { class: "text-sm text-muted-foreground", "Selected: {framework()}" }
                    }

                    Section { title: "Table",
                        Table {
                            TableCaption { "Recent payments" }
                            TableHeader {
                                TableRow {
                                    TableHead { "ID" }
                                    TableHead { "Status" }
                                    TableHead { "Amount" }
                                }
                            }
                            TableBody {
                                for (id, status, amount) in payments() {
                                    TableRow { key: "{id}",
                                        TableCell { "{id}" }
                                        TableCell { "{status}" }
                                        TableCell { "${amount:.2}" }
                                    }
                                }
                            }
                        }
                    }

                    Section { title: "Data Table",
                        DataTable { columns: payment_columns(), data: payments() }
                        DataTable {
                            columns: payment_columns(),
                            data: payments(),
                            selectable: true,
                            page_size: 3,
                            filter_placeholder: "Filter payments...".to_string(),
                        }
                    }

                    Section { title: "Alert Dialog",
                        AlertDialog {
                            AlertDialogTrigger { variant: ButtonVariant::Outline, "Show dialog" }
                            AlertDialogContent {
                                AlertDialogHeader {
                                    AlertDialogTitle { "Are you absolutely sure?" }
                                    AlertDialogDescription {
                                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                                    }
                                }
                                AlertDialogFooter {
                                    AlertDialogCancel { "Cancel" }
                                    AlertDialogAction { "Continue" }
                                }
                            }
                        }
                        AlertDialog {
                            AlertDialogTrigger { variant: ButtonVariant::Destructive, "With media" }
                            AlertDialogContent {
                                AlertDialogHeader {
                                    AlertDialogMedia { InfoIcon {} }
                                    AlertDialogTitle { "Delete this project?" }
                                    AlertDialogDescription { "All files and deployments will be removed." }
                                }
                                AlertDialogFooter {
                                    AlertDialogCancel { "Keep it" }
                                    AlertDialogAction { variant: ButtonVariant::Destructive, "Delete" }
                                }
                            }
                        }
                        AlertDialog {
                            AlertDialogTrigger { variant: ButtonVariant::Secondary, "Small size" }
                            AlertDialogContent { size: AlertDialogSize::Sm,
                                AlertDialogHeader {
                                    AlertDialogTitle { "Leave without saving?" }
                                    AlertDialogDescription { "Your changes will be lost." }
                                }
                                AlertDialogFooter {
                                    AlertDialogCancel { "Stay" }
                                    AlertDialogAction { "Leave" }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Bubble" }
                        div { class: "flex flex-col gap-6 rounded-lg border bg-card p-6",
                            BubbleGroup {
                                Bubble { variant: BubbleVariant::Muted, BubbleContent { "Hey, is the new build ready?" } }
                                Bubble { align: BubbleAlign::End, BubbleContent { "Yes, shipping it now." } }
                            }
                            BubbleGroup {
                                Bubble { variant: BubbleVariant::Default, BubbleContent { "Default" } }
                                Bubble { variant: BubbleVariant::Secondary, BubbleContent { "Secondary" } }
                                Bubble { variant: BubbleVariant::Muted, BubbleContent { "Muted" } }
                                Bubble { variant: BubbleVariant::Tinted, BubbleContent { "Tinted" } }
                                Bubble { variant: BubbleVariant::Outline, BubbleContent { "Outline" } }
                                Bubble { variant: BubbleVariant::Ghost, BubbleContent { "Ghost" } }
                                Bubble { variant: BubbleVariant::Destructive, BubbleContent { "Destructive" } }
                            }
                            BubbleGroup {
                                Bubble { variant: BubbleVariant::Secondary,
                                    BubbleContentButton { onclick: move |_| clicks += 1, "Clickable button bubble ({clicks})" }
                                }
                                Bubble { variant: BubbleVariant::Outline,
                                    BubbleContentLink { href: "#", "Link bubble" }
                                }
                                Bubble { variant: BubbleVariant::Muted, align: BubbleAlign::End,
                                    BubbleContent { "With a reaction below" }
                                    BubbleReactions { "👍 2" }
                                }
                                Bubble { variant: BubbleVariant::Muted,
                                    BubbleContent { "With a reaction on top" }
                                    BubbleReactions { side: BubbleReactionsSide::Top, align: BubbleAlign::Start, "❤️" }
                                }
                            }
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

                    Section { title: "Avatar",
                        Avatar { size: AvatarSize::Sm,
                            AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                            AvatarFallback { "CN" }
                        }
                        Avatar {
                            AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                            AvatarFallback { "CN" }
                        }
                        Avatar { size: AvatarSize::Lg,
                            AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                            AvatarFallback { "CN" }
                        }
                        Avatar {
                            AvatarImage { src: "/broken.png", alt: "Broken" }
                            AvatarFallback { "ER" }
                        }
                        Avatar {
                            AvatarFallback { "AB" }
                            AvatarBadge {}
                        }
                        Avatar { size: AvatarSize::Lg,
                            AvatarFallback { "AB" }
                            AvatarBadge { PlusIcon {} }
                        }
                        AvatarGroup {
                            Avatar { AvatarFallback { "A" } }
                            Avatar { AvatarFallback { "B" } }
                            Avatar { AvatarFallback { "C" } }
                            AvatarGroupCount { "+3" }
                        }
                    }

                    Section { title: "States and composition",
                        Button { onclick: move |_| clicks += 1, PlusIcon {} "Clicked {clicks} times" }
                        Button { disabled: true, "Disabled" }
                        Button { variant: ButtonVariant::Secondary, class: "rounded-full px-5", "Class override" }
                        Button { variant: ButtonVariant::Outline, aria_invalid: "true", "Invalid" }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Accordion" }
                        div { class: "flex flex-col gap-6 rounded-lg border p-6",
                            Accordion { default_value: vec!["shipping".to_string()],
                                AccordionItem { value: "product",
                                    AccordionTrigger { "What are your shipping options?" }
                                    AccordionContent {
                                        p { "We offer standard (5-7 days), express (2-3 days), and overnight shipping." }
                                    }
                                }
                                AccordionItem { value: "shipping",
                                    AccordionTrigger { "What is your return policy?" }
                                    AccordionContent {
                                        p { "Returns are accepted within 30 days of purchase." }
                                        p { "Items must be unused and in their original packaging." }
                                    }
                                }
                                AccordionItem { value: "disabled", disabled: true,
                                    AccordionTrigger { "Disabled item" }
                                    AccordionContent { p { "You cannot open this." } }
                                }
                            }
                            Accordion { multiple: true,
                                AccordionItem { value: "a",
                                    AccordionTrigger { "Multiple: first" }
                                    AccordionContent { p { "Several items can stay open at once." } }
                                }
                                AccordionItem { value: "b",
                                    AccordionTrigger { "Multiple: second" }
                                    AccordionContent { p { "Open me too." } }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Collapsible" }
                        div { class: "flex flex-col gap-3 rounded-lg border p-6",
                            Collapsible { default_open: true,
                                CollapsibleTrigger { class: "text-sm font-medium", "Uncontrolled, starts open" }
                                CollapsibleContent { p { class: "text-sm", "Shown by default." } }
                            }
                            Collapsible { open: Some(collapsible_open()), on_open_change: move |next| collapsible_open.set(next),
                                CollapsibleTrigger { class: "text-sm font-medium", "Controlled: {collapsible_open()}" }
                                CollapsibleContent { p { class: "text-sm", "Toggled from the parent's signal." } }
                            }
                            Collapsible { disabled: true,
                                CollapsibleTrigger { class: "text-sm font-medium", "Disabled" }
                                CollapsibleContent { p { class: "text-sm", "You cannot open this." } }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Code Block" }
                        div { class: "grid gap-4 rounded-lg border p-6 sm:grid-cols-2",
                            CodeBlock {
                                CodeBlockHeader {
                                    CodeBlockTitle { "main.rs" }
                                    CodeBlockActions { CodeBlockCopyButton { value: "fn main()" } }
                                }
                                CodeBlockContent { "fn main()" }
                            }
                            CodeBlock {
                                CodeBlockActions { CodeBlockCopyButton { value: "cargo run" } }
                                CodeBlockContent { "cargo run" }
                            }
                            CodeBlock { size: CodeBlockSize::Sm,
                                CodeBlockHeader { CodeBlockTitle { "Terminal" } }
                                CodeBlockContent { "Compiling oniko-cn v0.1.0\nFinished dev profile" }
                            }
                            CodeBlock { variant: CodeBlockVariant::Destructive,
                                CodeBlockHeader { CodeBlockTitle { "Error" } }
                                CodeBlockContent { "error[E0308]: mismatched types" }
                            }
                            CodeBlock {
                                CodeBlockContent { line_numbers: true,
                                    CodeBlockLine { "let a = 1;" }
                                    CodeBlockLine { variant: CodeBlockLineVariant::Removed, "let b = 2;" }
                                    CodeBlockLine { variant: CodeBlockLineVariant::Added, "let b = 3;" }
                                    CodeBlockLine { variant: CodeBlockLineVariant::Highlighted, "let c = a + b;" }
                                }
                                CodeBlockFooter { "Diff with line numbers" }
                            }
                            CodeBlock {
                                CodeBlockContent { wrap: true, class: "max-h-24",
                                    "A long line that wraps instead of scrolling horizontally, so the whole sentence stays visible inside the block."
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Calendar" }
                        div { class: "grid gap-6 rounded-lg border p-6 sm:grid-cols-2",
                            CalendarDemo {}
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Card" }
                        div { class: "grid gap-4 rounded-lg border p-6 sm:grid-cols-2",
                            Card {
                                CardHeader {
                                    CardTitle { "Create project" }
                                    CardDescription { "Deploy your app in one click." }
                                }
                                CardContent { p { class: "text-sm", "Choose a framework and a region." } }
                                CardFooter { Button { variant: ButtonVariant::Outline, "Cancel" } }
                            }
                            Card {
                                CardHeader {
                                    CardTitle { "With action" }
                                    CardDescription { "A button sits in the top right corner." }
                                    CardAction { Button { variant: ButtonVariant::Ghost, size: ButtonSize::Icon, aria_label: "More", PlusIcon {} } }
                                }
                                CardContent { p { class: "text-sm", "Card content goes here." } }
                            }
                            Card { size: CardSize::Sm,
                                CardHeader {
                                    CardTitle { "Small size" }
                                    CardDescription { "Less padding and a smaller title." }
                                }
                                CardContent { p { class: "text-sm", "Compact layout." } }
                            }
                            Card { class: "rounded-md",
                                CardHeader { CardTitle { "Class override" } }
                                CardContent { p { class: "text-sm", "Only the radius changed." } }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Item" }
                        div { class: "grid gap-4 sm:grid-cols-2",
                            for (variant, label) in [(ItemVariant::Default, "Default"), (ItemVariant::Outline, "Outline"), (ItemVariant::Muted, "Muted")] {
                                Item { variant,
                                    ItemMedia { variant: ItemMediaVariant::Icon, InfoIcon {} }
                                    ItemContent {
                                        ItemTitle { "{label} item" }
                                        ItemDescription { "Media, text and an action." }
                                    }
                                    ItemActions { Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Action" } }
                                }
                            }
                            Item { size: ItemSize::Sm, variant: ItemVariant::Outline,
                                ItemContent {
                                    ItemTitle { "Small size" }
                                    ItemDescription { "Less padding and gap." }
                                }
                            }
                            Item { size: ItemSize::Xs, variant: ItemVariant::Outline,
                                ItemContent {
                                    ItemTitle { "Extra small size" }
                                }
                            }
                            Item { class: "rounded-md", variant: ItemVariant::Outline,
                                ItemMedia { variant: ItemMediaVariant::Image,
                                    div { class: "size-10 rounded-sm bg-muted" }
                                }
                                ItemContent {
                                    ItemTitle { "Image media" }
                                    ItemDescription { "Class override only changes the radius." }
                                }
                            }
                            Item { variant: ItemVariant::Outline, class: "sm:col-span-2",
                                ItemHeader { span { class: "text-xs text-muted-foreground", "Header" } }
                                ItemContent {
                                    ItemTitle { "With header and footer" }
                                    ItemDescription { "Header sits above, footer below the content." }
                                }
                                ItemFooter { span { class: "text-xs text-muted-foreground", "Footer" } }
                            }
                            ItemGroup { class: "sm:col-span-2",
                                Item { ItemContent { ItemTitle { "First item" } } }
                                ItemSeparator {}
                                Item { ItemContent { ItemTitle { "Second item" } } }
                                ItemSeparator {}
                                Item { ItemContent { ItemTitle { "Third item" } } }
                            }
                            div { class: "flex items-center gap-4 sm:col-span-2",
                                span { class: "text-sm", "Separator" }
                                Separator {}
                                Separator { orientation: SeparatorOrientation::Vertical, class: "h-6" }
                                span { class: "text-sm", "Vertical" }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3",
                        h2 { class: "text-sm font-medium text-muted-foreground", "Carousel" }
                        div { class: "flex flex-col gap-12 rounded-lg border p-6",
                            Carousel { class: "mx-12",
                                CarouselContent {
                                    for n in 1..=3 {
                                        CarouselItem {
                                            div { class: "flex aspect-video items-center justify-center rounded-md border text-4xl font-semibold", "{n}" }
                                        }
                                    }
                                }
                                CarouselPrevious {}
                                CarouselNext {}
                            }
                            Carousel { orientation: CarouselOrientation::Vertical, class: "my-12",
                                CarouselContent { class: "h-[200px]",
                                    for n in 1..=3 {
                                        CarouselItem {
                                            div { class: "flex h-full items-center justify-center rounded-md border text-4xl font-semibold", "{n}" }
                                        }
                                    }
                                }
                                CarouselPrevious {}
                                CarouselNext {}
                            }
                        }
                    }

                    Section { title: "Command",
                        CommandDemo {}
                    }

                    Section { title: "Context Menu", ContextMenuDemo {} }
                    Section { title: "Date Picker",
                        DatePicker { today: CalendarDate::new(2026, 11, 4) }
                        DatePickerDemo {}
                        DatePicker { disabled: true }
                        DatePicker {
                            today: CalendarDate::new(2026, 11, 4),
                            min: CalendarDate::new(2026, 11, 15),
                            default_month: CalendarDate::new(2026, 11, 1),
                            placeholder: "Pick a date after Nov 15",
                        }
                    }

                    Section { title: "Hover Card",
                        HoverCard {
                            HoverCardTrigger { class: "text-sm underline underline-offset-4", "@dioxus" }
                            HoverCardContent {
                                p { class: "text-sm font-medium", "Dioxus" }
                                p { class: "text-xs text-muted-foreground", "Build cross-platform apps with Rust." }
                            }
                        }
                        HoverCard { open_delay: 0, close_delay: 0,
                            HoverCardTrigger { class: "text-sm underline underline-offset-4", "No delay" }
                            HoverCardContent { side: HoverCardSide::Right,
                                p { class: "text-sm", "Opens and closes at once." }
                            }
                        }
                        HoverCard { open_delay: 0, close_delay: 0,
                            HoverCardTrigger { class: "text-sm underline underline-offset-4", "Top start" }
                            HoverCardContent { side: HoverCardSide::Top, align: HoverCardAlign::Start,
                                p { class: "text-sm", "Top, aligned to the start." }
                            }
                        }
                        HoverCard { open_delay: 0, close_delay: 0,
                            HoverCardTrigger { class: "text-sm underline underline-offset-4", "Bottom end" }
                            HoverCardContent { side: HoverCardSide::Bottom, align: HoverCardAlign::End,
                                p { class: "text-sm", "Bottom, aligned to the end." }
                            }
                        }
                        HoverCard { open_delay: 0, close_delay: 0,
                            HoverCardTrigger { class: "text-sm underline underline-offset-4", "Left" }
                            HoverCardContent { side: HoverCardSide::Left,
                                p { class: "text-sm", "Left side." }
                            }
                        }
                        HoverCardControlled {}
                    }

                    Section { title: "Input",
                        div { class: "flex w-full max-w-md flex-col gap-3",
                            Input { placeholder: "Plain input" }
                            Input { r#type: "email", placeholder: "Email" }
                            Input { r#type: "password", value: "secret" }
                            Input { placeholder: "Disabled", disabled: true }
                            Input { placeholder: "Invalid", aria_invalid: "true" }
                            Input { r#type: "file" }
                        }
                    }
                    Section { title: "Dropdown Menu", DropdownMenuDemo {} }
                    Section { title: "Empty",
                        Empty {
                            EmptyHeader {
                                EmptyMedia { variant: EmptyMediaVariant::Icon, PlusIcon {} }
                                EmptyTitle { "No projects yet" }
                                EmptyDescription { "Create your first project to get started." }
                            }
                            EmptyContent { Button { "Create project" } }
                        }
                        Empty {
                            EmptyHeader {
                                EmptyMedia { variant: EmptyMediaVariant::Default, InfoIcon {} }
                                EmptyTitle { "Nothing here" }
                                EmptyDescription { "Results will show up once data arrives." }
                            }
                        }
                    }

                    Section { title: "Input Group",
                        div { class: "flex w-full max-w-sm flex-col gap-3",
                            InputGroup {
                                InputGroupInput { placeholder: "Search..." }
                                InputGroupAddon { InfoIcon {} }
                            }
                            InputGroup {
                                InputGroupAddon { align: InputGroupAlign::InlineStart, InputGroupText { "https://" } }
                                InputGroupInput { placeholder: "example.com" }
                                InputGroupAddon { align: InputGroupAlign::InlineEnd, InputGroupText { ".com" } }
                            }
                            InputGroup {
                                InputGroupInput { placeholder: "Enter amount" }
                                InputGroupAddon { align: InputGroupAlign::InlineEnd,
                                    InputGroupText { "USD" }
                                }
                            }
                            InputGroup {
                                InputGroupAddon { align: InputGroupAlign::InlineEnd,
                                    InputGroupButton { variant: ButtonVariant::Ghost, size: InputGroupButtonSize::IconXs, aria_label: "Copy", PlusIcon {} }
                                }
                                InputGroupInput { placeholder: "Copy me", readonly: true, value: "oniko-cn" }
                            }
                            InputGroup {
                                InputGroupAddon { align: InputGroupAlign::BlockStart,
                                    InputGroupText { "Message" }
                                }
                                InputGroupTextarea { placeholder: "Ask, search or chat..." }
                                InputGroupAddon { align: InputGroupAlign::BlockEnd,
                                    InputGroupButton { variant: ButtonVariant::Outline, size: InputGroupButtonSize::IconSm, aria_label: "Attach", PlusIcon {} }
                                    InputGroupText { "52% of 200" }
                                    InputGroupButton { variant: ButtonVariant::Default, size: InputGroupButtonSize::Xs, class: "ml-auto rounded-full", "Send" }
                                }
                            }
                        }
                    }

                    Section { title: "Field",
                        div { class: "flex w-full max-w-md flex-col gap-6",
                            FieldSet {
                                FieldLegend { "Profile" }
                                FieldDescription { "This appears on invoices and emails." }
                                FieldGroup {
                                    Field {
                                        FieldLabel { r#for: "field-name", "Full name" }
                                        input { id: "field-name", class: "h-9 rounded-md border px-3", placeholder: "Evil Rabbit" }
                                        FieldDescription { "This appears on invoices and emails." }
                                    }
                                    Field { invalid: true,
                                        FieldLabel { r#for: "field-username", "Username" }
                                        input { id: "field-username", class: "h-9 rounded-md border px-3", aria_invalid: "true" }
                                        FieldError { "Choose another username." }
                                    }
                                    Field { orientation: FieldOrientation::Horizontal,
                                        input { id: "field-newsletter", r#type: "checkbox" }
                                        FieldLabel { r#for: "field-newsletter", "Subscribe to the newsletter" }
                                    }
                                }
                            }
                            FieldSeparator {}
                            FieldSet {
                                FieldLegend { variant: FieldLegendVariant::Label, "Address" }
                                FieldGroup {
                                    Field { orientation: FieldOrientation::Responsive,
                                        FieldContent {
                                            FieldLabel { r#for: "field-city", "City" }
                                            FieldDescription { "Where the order ships." }
                                        }
                                        input { id: "field-city", class: "h-9 rounded-md border px-3" }
                                    }
                                    FieldSeparator { "Or" }
                                    Field { disabled: true,
                                        FieldTitle { "Disabled field" }
                                        FieldDescription { "This field cannot be edited." }
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

#[component]
fn DropdownMenuDemo() -> Element {
    let mut status_bar = use_signal(|| true);
    let mut activity = use_signal(|| false);
    let mut position = use_signal(|| "bottom".to_string());

    rsx! {
        div { class: "flex flex-wrap items-start gap-3",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Open menu" }
                DropdownMenuContent {
                    DropdownMenuGroup {
                        DropdownMenuLabel { "My Account" }
                        DropdownMenuItem {
                            "Profile"
                            DropdownMenuShortcut { "⇧⌘P" }
                        }
                        DropdownMenuItem {
                            "Billing"
                            DropdownMenuShortcut { "⌘B" }
                        }
                        DropdownMenuItem { disabled: true, "Disabled" }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuLabel { inset: true, "View" }
                        DropdownMenuCheckboxItem {
                            checked: status_bar(),
                            onchecked_change: move |value: bool| status_bar.set(value),
                            "Status bar"
                        }
                        DropdownMenuCheckboxItem {
                            checked: activity(),
                            onchecked_change: move |value: bool| activity.set(value),
                            "Activity bar"
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuLabel { "Panel position" }
                        DropdownMenuRadioGroup {
                            value: position(),
                            on_value_change: move |value: String| position.set(value),
                            DropdownMenuRadioItem { value: "top", "Top" }
                            DropdownMenuRadioItem { value: "bottom", "Bottom" }
                            DropdownMenuRadioItem { value: "right", "Right" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuSub {
                        DropdownMenuSubTrigger { "Invite users" }
                        DropdownMenuSubContent {
                            DropdownMenuItem { "Email" }
                            DropdownMenuItem { "Message" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive, "Delete" }
                }
            }
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Secondary, "Aligned end" }
                DropdownMenuContent { align: DropdownMenuAlign::End,
                    DropdownMenuItem { "Copy link" }
                    DropdownMenuItem { "Duplicate" }
                }
            }
        }
    }
}

#[component]
fn ContextMenuDemo() -> Element {
    let mut show_labels = use_signal(|| true);
    let mut theme = use_signal(|| "light".to_string());
    let mut last_action = use_signal(|| "none".to_string());

    rsx! {
        ContextMenu {
            ContextMenuTrigger { class: "flex h-40 w-72 items-center justify-center rounded-md border border-dashed text-sm",
                "Right click here"
            }
            ContextMenuContent {
                ContextMenuLabel { "Page" }
                ContextMenuItem { onclick: move |_| last_action.set("Back".to_string()),
                    "Back"
                    ContextMenuShortcut { "Ctrl+[" }
                }
                ContextMenuItem { disabled: true, inset: true, "Forward" }
                ContextMenuSeparator {}
                ContextMenuCheckboxItem {
                    checked: Some(show_labels()),
                    on_checked_change: move |next| show_labels.set(next),
                    "Show labels"
                }
                ContextMenuSeparator {}
                ContextMenuRadioGroup {
                    value: Some(theme()),
                    on_value_change: move |next| theme.set(next),
                    ContextMenuRadioItem { value: "light".to_string(), "Light" }
                    ContextMenuRadioItem { value: "dark".to_string(), "Dark" }
                }
                ContextMenuSeparator {}
                ContextMenuItem {
                    variant: ContextMenuItemVariant::Destructive,
                    onclick: move |_| last_action.set("Delete".to_string()),
                    "Delete"
                }
            }
        }
        p { class: "text-sm text-muted-foreground", "Last action: {last_action}" }
    }
}

#[component]
fn DatePickerDemo() -> Element {
    let mut picked = use_signal(|| None::<CalendarDate>);
    let picked_text = picked().map_or("none".to_string(), |d| {
        format!("{}-{:02}-{:02}", d.year, d.month, d.day)
    });

    rsx! {
        div { class: "flex flex-col gap-3",
            DatePicker {
                selected: picked(),
                today: CalendarDate::new(2026, 11, 4),
                on_select: move |date| picked.set(Some(date)),
            }
            p { class: "text-xs text-muted-foreground", "Controlled. Selected: {picked_text}" }
        }
    }
}

#[component]
fn CalendarDemo() -> Element {
    let mut picked = use_signal(|| Some(CalendarDate::new(2026, 11, 10)));
    let mut disabled_pick = use_signal(|| None::<CalendarDate>);
    let picked_text = picked().map_or("none".to_string(), |d| {
        format!("{}-{:02}-{:02}", d.year, d.month, d.day)
    });

    rsx! {
        div { class: "flex flex-col gap-3",
            p { class: "text-xs text-muted-foreground", "Single date. Selected: {picked_text}" }
            Calendar {
                selected: picked(),
                today: CalendarDate::new(2026, 11, 4),
                on_select: move |date| picked.set(Some(date)),
            }
        }
        div { class: "flex flex-col gap-3",
            p { class: "text-xs text-muted-foreground",
                "Days before Nov 15 and the 20th are disabled."
            }
            Calendar {
                selected: disabled_pick(),
                default_month: CalendarDate::new(2026, 11, 1),
                min: CalendarDate::new(2026, 11, 15),
                disabled_dates: vec![CalendarDate::new(2026, 11, 20)],
                on_select: move |date| disabled_pick.set(Some(date)),
            }
        }
    }
}

#[component]
fn HoverCardControlled() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        HoverCard {
            open: open(),
            open_delay: 0,
            close_delay: 0,
            on_open_change: move |value| open.set(value),
            HoverCardTrigger { class: "text-sm underline underline-offset-4", "Controlled" }
            HoverCardContent {
                p { class: "text-sm", "Open state is owned by the parent." }
            }
        }
    }
}

#[component]
fn CommandDemo() -> Element {
    let mut picked = use_signal(|| "nothing yet".to_string());
    rsx! {
        div { class: "flex flex-col gap-3",
            Command { class: "w-80 rounded-lg border",
                CommandInput { placeholder: "Search actions..." }
                CommandList {
                    CommandEmpty { "No results found." }
                    CommandGroup { heading: "Suggestions",
                        CommandItem { value: "calendar", keywords: vec!["date".to_string()], on_select: move |v: String| picked.set(v), "Calendar" }
                        CommandItem { value: "search", on_select: move |v: String| picked.set(v), "Search" }
                        CommandItem { value: "disabled", disabled: true, "Disabled" }
                    }
                    CommandSeparator {}
                    CommandGroup { heading: "Settings",
                        CommandItem { value: "profile", on_select: move |v: String| picked.set(v), "Profile", CommandShortcut { "Ctrl P" } }
                        CommandItem { value: "billing", on_select: move |v: String| picked.set(v), "Billing", CommandShortcut { "Ctrl B" } }
                    }
                }
            }
            p { class: "text-sm text-muted-foreground", "Selected: {picked()}" }
        }
    }
}

#[component]
fn OtpControlled() -> Element {
    let mut code = use_signal(String::new);
    rsx! {
        div { class: "flex flex-col gap-2",
            InputOTP {
                max_length: 6,
                value: code(),
                on_value_change: move |next| code.set(next),
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                    InputOTPSlot { index: 2 }
                    InputOTPSlot { index: 3 }
                    InputOTPSlot { index: 4 }
                    InputOTPSlot { index: 5 }
                }
            }
            span { class: "text-sm text-muted-foreground", "Value: {code}" }
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

type Payment = (&'static str, &'static str, f64);

fn payments() -> Vec<Payment> {
    vec![
        ("728ed52f", "pending", 100.0),
        ("489e1d42", "success", 125.0),
        ("a1b2c3d4", "processing", 54.5),
        ("e5f6a7b8", "failed", 20.0),
        ("c9d0e1f2", "success", 310.0),
        ("13579bdf", "pending", 42.0),
        ("2468ace0", "success", 7.25),
    ]
}

fn payment_columns() -> Vec<DataColumn<Payment>> {
    vec![
        DataColumn::new("ID", |p: &Payment| p.0.to_string()).sortable(),
        DataColumn::new("Status", |p: &Payment| p.1.to_string()).sortable(),
        DataColumn::new("Amount", |p: &Payment| p.2.to_string())
            .cell(|p: &Payment| {
                let amount = format!("${:.2}", p.2);
                rsx! { div { class: "text-right font-medium", {amount} } }
            })
            .sortable(),
    ]
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

#[component]
fn InfoIcon() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "10" }
            path { d: "M12 16v-4M12 8h.01" }
        }
    }
}
