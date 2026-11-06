//! Sidebar examples, ported from shadcn/ui `examples/base/sidebar-*.tsx`.
//!
//! Upstream shows each of these in its own iframe. Here every sidebar sits in a `Frame`, which makes the
//! fixed panel absolute inside a box of fixed height so several sidebars can share one page.

use dioxus::prelude::*;
use lucide_dioxus::{
    AudioWaveform, BadgeCheck, Bell, BookOpen, Bot, ChartPie, ChevronDown, ChevronRight, ChevronUp,
    ChevronsUpDown, Command, CreditCard, Ellipsis, Folder, Forward, Frame as FrameIcon,
    GalleryVerticalEnd, LifeBuoy, LogOut, Map as MapIcon, PanelLeftClose, PanelLeftOpen, Plus,
    Send, Settings2, Sparkles, SquareTerminal, Trash,
};

use super::Example;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuSide,
    DropdownMenuTrigger,
};
use crate::components::ui::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupAction,
    SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu,
    SidebarMenuAction, SidebarMenuBadge, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem,
    SidebarMenuSkeleton, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider,
    SidebarRail, SidebarSide, SidebarTrigger, SidebarVariant, use_sidebar,
};
use crate::components::ui::sonner::{ToastOptions, Toaster, toast};

const FRAME: &str = "h-[600px] items-stretch! justify-stretch! p-0! md:p-0! overflow-hidden";

type Render = (Vec<Attribute>, Element);

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            SidebarDemo {}
            SidebarHeaderExample {}
            SidebarFooterExample {}
            SidebarGroupExample {}
            SidebarGroupCollapsible {}
            SidebarGroupActionExample {}
            SidebarMenuExample {}
            SidebarMenuActionExample {}
            SidebarMenuSubExample {}
            SidebarMenuBadgeExample {}
            SidebarMenuCollapsible {}
            SidebarRsc {}
            SidebarControlled {}
            SidebarRtl {}
        }
    }
}

/// Fixed-height box that holds the sidebar's fixed panel.
#[component]
fn Frame(children: Element) -> Element {
    rsx! {
        div { class: "relative h-full w-full overflow-hidden [&_[data-slot=sidebar-container]]:absolute [&_[data-slot=sidebar-container]]:h-full [&_[data-slot=sidebar-wrapper]]:h-full [&_[data-slot=sidebar-wrapper]]:min-h-0",
            {children}
        }
    }
}

fn as_link((attributes, children): Render) -> Element {
    rsx! {
        a { href: "#", ..attributes, {children} }
    }
}

fn as_collapsible_trigger((attributes, children): Render) -> Element {
    rsx! {
        CollapsibleTrigger { attributes, {children} }
    }
}

fn as_dropdown_trigger((attributes, children): Render) -> Element {
    rsx! {
        DropdownMenuTrigger { attributes, {children} }
    }
}

const OPEN_MENU_BUTTON: &str =
    "aria-expanded:bg-sidebar-accent aria-expanded:text-sidebar-accent-foreground";

struct Team {
    name: &'static str,
    plan: &'static str,
    logo: fn(&'static str) -> Element,
}

fn gallery_vertical_end(class: &'static str) -> Element {
    rsx! {
        GalleryVerticalEnd { class }
    }
}

fn audio_waveform(class: &'static str) -> Element {
    rsx! {
        AudioWaveform { class }
    }
}

fn command(class: &'static str) -> Element {
    rsx! {
        Command { class }
    }
}

const TEAMS: [Team; 3] = [
    Team {
        name: "Acme Inc",
        plan: "Enterprise",
        logo: gallery_vertical_end,
    },
    Team {
        name: "Acme Corp.",
        plan: "Startup",
        logo: audio_waveform,
    },
    Team {
        name: "Evil Corp.",
        plan: "Free",
        logo: command,
    },
];

struct NavItem {
    title: &'static str,
    icon: fn() -> Element,
    is_active: bool,
    items: &'static [&'static str],
}

fn square_terminal() -> Element {
    rsx! {
        SquareTerminal {}
    }
}

fn bot() -> Element {
    rsx! {
        Bot {}
    }
}

fn book_open() -> Element {
    rsx! {
        BookOpen {}
    }
}

fn settings_2() -> Element {
    rsx! {
        Settings2 {}
    }
}

const NAV_MAIN: [NavItem; 4] = [
    NavItem {
        title: "Playground",
        icon: square_terminal,
        is_active: true,
        items: &["History", "Starred", "Settings"],
    },
    NavItem {
        title: "Models",
        icon: bot,
        is_active: false,
        items: &["Genesis", "Explorer", "Quantum"],
    },
    NavItem {
        title: "Documentation",
        icon: book_open,
        is_active: false,
        items: &["Introduction", "Get Started", "Tutorials", "Changelog"],
    },
    NavItem {
        title: "Settings",
        icon: settings_2,
        is_active: false,
        items: &["General", "Team", "Billing", "Limits"],
    },
];

struct Project {
    name: &'static str,
    icon: fn() -> Element,
}

fn frame_icon() -> Element {
    rsx! {
        FrameIcon {}
    }
}

fn pie_chart() -> Element {
    rsx! {
        ChartPie {}
    }
}

fn map_icon() -> Element {
    rsx! {
        MapIcon {}
    }
}

fn life_buoy() -> Element {
    rsx! {
        LifeBuoy {}
    }
}

fn send() -> Element {
    rsx! {
        Send {}
    }
}

const PROJECTS: [Project; 3] = [
    Project {
        name: "Design Engineering",
        icon: frame_icon,
    },
    Project {
        name: "Sales & Marketing",
        icon: pie_chart,
    },
    Project {
        name: "Travel",
        icon: map_icon,
    },
];

const PROJECTS_EXTENDED: [Project; 5] = [
    Project {
        name: "Design Engineering",
        icon: frame_icon,
    },
    Project {
        name: "Sales & Marketing",
        icon: pie_chart,
    },
    Project {
        name: "Travel",
        icon: map_icon,
    },
    Project {
        name: "Support",
        icon: life_buoy,
    },
    Project {
        name: "Feedback",
        icon: send,
    },
];

const USER_NAME: &str = "shadcn";
const USER_EMAIL: &str = "m@example.com";
const USER_AVATAR: &str = "https://github.com/shadcn.png";

#[component]
fn TeamSwitcher() -> Element {
    let sidebar = use_sidebar();
    let mut active = use_signal(|| 0_usize);
    let team = &TEAMS[active()];

    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu {
                    SidebarMenuButton {
                        size: SidebarMenuButtonSize::Lg,
                        class: OPEN_MENU_BUTTON,
                        render: as_dropdown_trigger,
                        div { class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground",
                            {(team.logo)("size-4")}
                        }
                        div { class: "grid flex-1 text-left text-sm leading-tight",
                            span { class: "truncate font-medium", "{team.name}" }
                            span { class: "truncate text-xs", "{team.plan}" }
                        }
                        ChevronsUpDown { class: "ml-auto" }
                    }
                    DropdownMenuContent {
                        class: "w-(--anchor-width) min-w-56 rounded-lg",
                        align: DropdownMenuAlign::Start,
                        side: if sidebar.is_mobile() { DropdownMenuSide::Bottom } else { DropdownMenuSide::Right },
                        side_offset: 4.0,
                        DropdownMenuGroup {
                            DropdownMenuLabel { class: "text-xs text-muted-foreground", "Teams" }
                            for (index , team) in TEAMS.iter().enumerate() {
                                DropdownMenuItem {
                                    key: "{team.name}",
                                    class: "gap-2 p-2",
                                    onclick: move |_| active.set(index),
                                    div { class: "flex size-6 items-center justify-center rounded-md border",
                                        {(team.logo)("size-3.5 shrink-0")}
                                    }
                                    "{team.name}"
                                    DropdownMenuShortcut { "⌘{index + 1}" }
                                }
                            }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuGroup {
                            DropdownMenuItem { class: "gap-2 p-2",
                                div { class: "flex size-6 items-center justify-center rounded-md border bg-transparent",
                                    Plus { class: "size-4" }
                                }
                                div { class: "font-medium text-muted-foreground", "Add team" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavMain() -> Element {
    rsx! {
        SidebarGroup {
            SidebarGroupLabel { "Platform" }
            SidebarMenu {
                for item in NAV_MAIN.iter() {
                    Collapsible {
                        key: "{item.title}",
                        default_open: item.is_active,
                        class: "group/collapsible",
                        SidebarMenuItem {
                            SidebarMenuButton {
                                tooltip: item.title,
                                render: as_collapsible_trigger,
                                {(item.icon)()}
                                span { "{item.title}" }
                                ChevronRight { class: "ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" }
                            }
                            CollapsibleContent {
                                SidebarMenuSub {
                                    for sub_item in item.items.iter() {
                                        SidebarMenuSubItem { key: "{sub_item}",
                                            SidebarMenuSubButton { render: as_link,
                                                span { "{sub_item}" }
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

#[component]
fn NavProjects() -> Element {
    let sidebar = use_sidebar();

    rsx! {
        SidebarGroup { class: "group-data-[collapsible=icon]:hidden",
            SidebarGroupLabel { "Projects" }
            SidebarMenu {
                for item in PROJECTS.iter() {
                    SidebarMenuItem { key: "{item.name}",
                        SidebarMenuButton { render: as_link,
                            {(item.icon)()}
                            span { "{item.name}" }
                        }
                        DropdownMenu {
                            SidebarMenuAction {
                                show_on_hover: true,
                                render: as_dropdown_trigger,
                                Ellipsis {}
                                span { class: "sr-only", "More" }
                            }
                            DropdownMenuContent {
                                class: "w-48 rounded-lg",
                                side: if sidebar.is_mobile() { DropdownMenuSide::Bottom } else { DropdownMenuSide::Right },
                                align: if sidebar.is_mobile() { DropdownMenuAlign::End } else { DropdownMenuAlign::Start },
                                DropdownMenuItem {
                                    Folder { class: "text-muted-foreground" }
                                    span { "View Project" }
                                }
                                DropdownMenuItem {
                                    Forward { class: "text-muted-foreground" }
                                    span { "Share Project" }
                                }
                                DropdownMenuSeparator {}
                                DropdownMenuItem {
                                    Trash { class: "text-muted-foreground" }
                                    span { "Delete Project" }
                                }
                            }
                        }
                    }
                }
                SidebarMenuItem {
                    SidebarMenuButton { class: "text-sidebar-foreground/70",
                        Ellipsis { class: "text-sidebar-foreground/70" }
                        span { "More" }
                    }
                }
            }
        }
    }
}

#[component]
fn NavUser() -> Element {
    let sidebar = use_sidebar();

    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu {
                    SidebarMenuButton {
                        size: SidebarMenuButtonSize::Lg,
                        class: OPEN_MENU_BUTTON,
                        render: as_dropdown_trigger,
                        Avatar { class: "h-8 w-8 rounded-lg",
                            AvatarImage { src: USER_AVATAR, alt: USER_NAME }
                            AvatarFallback { class: "rounded-lg", "CN" }
                        }
                        div { class: "grid flex-1 text-left text-sm leading-tight",
                            span { class: "truncate font-medium", "{USER_NAME}" }
                            span { class: "truncate text-xs", "{USER_EMAIL}" }
                        }
                        ChevronsUpDown { class: "ml-auto size-4" }
                    }
                    DropdownMenuContent {
                        class: "w-(--anchor-width) min-w-56 rounded-lg",
                        side: if sidebar.is_mobile() { DropdownMenuSide::Bottom } else { DropdownMenuSide::Right },
                        align: DropdownMenuAlign::End,
                        side_offset: 4.0,
                        DropdownMenuGroup {
                            DropdownMenuLabel { class: "p-0 font-normal",
                                div { class: "flex items-center gap-2 px-1 py-1.5 text-left text-sm",
                                    Avatar { class: "h-8 w-8 rounded-lg",
                                        AvatarImage { src: USER_AVATAR, alt: USER_NAME }
                                        AvatarFallback { class: "rounded-lg", "CN" }
                                    }
                                    div { class: "grid flex-1 text-left text-sm leading-tight",
                                        span { class: "truncate font-medium", "{USER_NAME}" }
                                        span { class: "truncate text-xs", "{USER_EMAIL}" }
                                    }
                                }
                            }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuGroup {
                            DropdownMenuItem {
                                Sparkles {}
                                "Upgrade to Pro"
                            }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuGroup {
                            DropdownMenuItem {
                                BadgeCheck {}
                                "Account"
                            }
                            DropdownMenuItem {
                                CreditCard {}
                                "Billing"
                            }
                            DropdownMenuItem {
                                Bell {}
                                "Notifications"
                            }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuGroup {
                            DropdownMenuItem {
                                LogOut {}
                                "Log out"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar { collapsible: SidebarCollapsible::Icon,
                        SidebarHeader {
                            TeamSwitcher {}
                        }
                        SidebarContent {
                            NavMain {}
                            NavProjects {}
                        }
                        SidebarFooter {
                            NavUser {}
                        }
                        SidebarRail {}
                    }
                    SidebarInset {
                        header { class: "flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12",
                            div { class: "flex items-center gap-2 px-4",
                                SidebarTrigger { class: "-ml-1" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarHeaderExample() -> Element {
    rsx! {
        Example { title: "Header", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarHeader {
                            SidebarMenu {
                                SidebarMenuItem {
                                    DropdownMenu {
                                        SidebarMenuButton {
                                            class: OPEN_MENU_BUTTON,
                                            render: as_dropdown_trigger,
                                            "Select Workspace"
                                            ChevronDown { class: "ml-auto" }
                                        }
                                        DropdownMenuContent { class: "w-(--anchor-width)",
                                            DropdownMenuItem {
                                                span { "Acme Inc" }
                                            }
                                            DropdownMenuItem {
                                                span { "Acme Corp." }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    SidebarInset {
                        header { class: "flex h-12 items-center justify-between px-4",
                            SidebarTrigger {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarFooterExample() -> Element {
    rsx! {
        Example { title: "Footer", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarHeader {}
                        SidebarContent {}
                        SidebarFooter {
                            SidebarMenu {
                                SidebarMenuItem {
                                    DropdownMenu {
                                        SidebarMenuButton {
                                            class: OPEN_MENU_BUTTON,
                                            render: as_dropdown_trigger,
                                            "Username"
                                            ChevronUp { class: "ml-auto" }
                                        }
                                        DropdownMenuContent {
                                            side: DropdownMenuSide::Top,
                                            class: "w-(--anchor-width)",
                                            DropdownMenuItem {
                                                span { "Account" }
                                            }
                                            DropdownMenuItem {
                                                span { "Billing" }
                                            }
                                            DropdownMenuItem {
                                                span { "Sign out" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    SidebarInset {
                        header { class: "flex h-12 items-center justify-between px-4",
                            SidebarTrigger {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarGroupExample() -> Element {
    rsx! {
        Example { title: "Group", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Help" }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        SidebarMenuItem {
                                            SidebarMenuButton {
                                                LifeBuoy {}
                                                "Support"
                                            }
                                        }
                                        SidebarMenuItem {
                                            SidebarMenuButton {
                                                Send {}
                                                "Feedback"
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

#[component]
fn SidebarGroupCollapsible() -> Element {
    rsx! {
        Example { title: "Collapsible Group", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarContent {
                            Collapsible { default_open: true, class: "group/collapsible",
                                SidebarGroup {
                                    SidebarGroupLabel {
                                        render: as_collapsible_trigger,
                                        class: "text-sm hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
                                        "Help"
                                        ChevronDown { class: "ml-auto transition-transform group-data-[state=open]/collapsible:rotate-180" }
                                    }
                                    CollapsibleContent {
                                        SidebarGroupContent {
                                            SidebarMenu {
                                                SidebarMenuItem {
                                                    SidebarMenuButton {
                                                        LifeBuoy {}
                                                        "Support"
                                                    }
                                                }
                                                SidebarMenuItem {
                                                    SidebarMenuButton {
                                                        Send {}
                                                        "Feedback"
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
}

#[component]
fn SidebarGroupActionExample() -> Element {
    rsx! {
        Example { title: "Group Action", class: FRAME,
            Frame {
                SidebarProvider {
                    Toaster { position: crate::components::ui::sonner::Position::BottomLeft }
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Projects" }
                                SidebarGroupAction {
                                    title: "Add Project",
                                    onclick: move |_| {
                                        toast("You clicked the group action!", ToastOptions::default());
                                    },
                                    Plus {}
                                    " "
                                    span { class: "sr-only", "Add Project" }
                                }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        SidebarMenuItem {
                                            SidebarMenuButton { render: as_link,
                                                FrameIcon {}
                                                span { "Design Engineering" }
                                            }
                                        }
                                        SidebarMenuItem {
                                            SidebarMenuButton { render: as_link,
                                                ChartPie {}
                                                span { "Sales & Marketing" }
                                            }
                                        }
                                        SidebarMenuItem {
                                            SidebarMenuButton { render: as_link,
                                                MapIcon {}
                                                span { "Travel" }
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

#[component]
fn SidebarMenuExample() -> Element {
    rsx! {
        Example { title: "Menu", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Projects" }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        for project in PROJECTS_EXTENDED.iter() {
                                            SidebarMenuItem { key: "{project.name}",
                                                SidebarMenuButton { render: as_link,
                                                    {(project.icon)()}
                                                    span { "{project.name}" }
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

#[component]
fn SidebarMenuActionExample() -> Element {
    rsx! {
        Example { title: "Menu Action", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Projects" }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        for project in PROJECTS_EXTENDED.iter() {
                                            SidebarMenuItem { key: "{project.name}",
                                                SidebarMenuButton {
                                                    render: as_link,
                                                    class: "group-has-[[aria-expanded=true]]/menu-item:bg-sidebar-accent",
                                                    {(project.icon)()}
                                                    span { "{project.name}" }
                                                }
                                                DropdownMenu {
                                                    SidebarMenuAction { render: as_dropdown_trigger,
                                                        Ellipsis {}
                                                        span { class: "sr-only", "More" }
                                                    }
                                                    DropdownMenuContent {
                                                        side: DropdownMenuSide::Right,
                                                        align: DropdownMenuAlign::Start,
                                                        DropdownMenuItem {
                                                            span { "Edit Project" }
                                                        }
                                                        DropdownMenuItem {
                                                            span { "Delete Project" }
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
    }
}

const MENU_SUB_ITEMS: [(&str, &[(&str, bool)]); 4] = [
    (
        "Getting Started",
        &[("Installation", false), ("Project Structure", false)],
    ),
    (
        "Build Your Application",
        &[
            ("Routing", false),
            ("Data Fetching", true),
            ("Rendering", false),
            ("Caching", false),
            ("Styling", false),
            ("Optimizing", false),
            ("Configuring", false),
            ("Testing", false),
            ("Authentication", false),
            ("Deploying", false),
            ("Upgrading", false),
            ("Examples", false),
        ],
    ),
    (
        "API Reference",
        &[
            ("Components", false),
            ("File Conventions", false),
            ("Functions", false),
            ("next.config.js Options", false),
            ("CLI", false),
            ("Edge Runtime", false),
        ],
    ),
    (
        "Architecture",
        &[
            ("Accessibility", false),
            ("Fast Refresh", false),
            ("Next.js Compiler", false),
            ("Supported Browsers", false),
            ("Turbopack", false),
        ],
    ),
];

#[component]
fn SidebarMenuSubExample() -> Element {
    rsx! {
        Example { title: "Menu Sub", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupContent {
                                    SidebarMenu {
                                        for (title , items) in MENU_SUB_ITEMS {
                                            SidebarMenuItem { key: "{title}",
                                                SidebarMenuButton { render: as_link,
                                                    span { "{title}" }
                                                }
                                                SidebarMenuSub {
                                                    for (sub_title , is_active) in items.iter().copied() {
                                                        SidebarMenuSubItem { key: "{sub_title}",
                                                            SidebarMenuSubButton { render: as_link, is_active,
                                                                span { "{sub_title}" }
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
        }
    }
}

const BADGES: [&str; 5] = ["24", "12", "3", "21", "8"];

#[component]
fn SidebarMenuBadgeExample() -> Element {
    rsx! {
        Example { title: "Menu Badge", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Projects" }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        for (project , badge) in PROJECTS_EXTENDED.iter().zip(BADGES) {
                                            SidebarMenuItem { key: "{project.name}",
                                                SidebarMenuButton {
                                                    render: as_link,
                                                    class: "group-has-[[aria-expanded=true]]/menu-item:bg-sidebar-accent",
                                                    {(project.icon)()}
                                                    span { "{project.name}" }
                                                }
                                                SidebarMenuBadge { "{badge}" }
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

#[component]
fn SidebarMenuCollapsible() -> Element {
    rsx! {
        Example { title: "Collapsible Menu", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupContent {
                                    SidebarMenu {
                                        for (index , (title , items)) in MENU_SUB_ITEMS.into_iter().enumerate() {
                                            Collapsible {
                                                key: "{title}",
                                                class: "group/collapsible",
                                                default_open: index == 0,
                                                SidebarMenuItem {
                                                    SidebarMenuButton { render: as_collapsible_trigger,
                                                        span { "{title}" }
                                                        ChevronRight { class: "ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90" }
                                                    }
                                                    CollapsibleContent {
                                                        SidebarMenuSub {
                                                            for (sub_title , _) in items.iter().copied() {
                                                                SidebarMenuSubItem { key: "{sub_title}",
                                                                    SidebarMenuSubButton { render: as_link,
                                                                        span { "{sub_title}" }
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
                }
            }
        }
    }
}

#[component]
fn SidebarRsc() -> Element {
    let mut loaded = use_signal(|| false);
    use_effect(move || {
        spawn(async move {
            let _ =
                document::eval("await new Promise((resolve) => setTimeout(resolve, 3000))").await;
            loaded.set(true);
        });
    });

    rsx! {
        Example { title: "RSC", class: FRAME,
            Frame {
                SidebarProvider {
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Projects" }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        if loaded() {
                                            for project in PROJECTS_EXTENDED.iter() {
                                                SidebarMenuItem { key: "{project.name}",
                                                    SidebarMenuButton { render: as_link,
                                                        {(project.icon)()}
                                                        span { "{project.name}" }
                                                    }
                                                }
                                            }
                                        } else {
                                            for index in 0..5 {
                                                SidebarMenuItem { key: "{index}",
                                                    SidebarMenuSkeleton { show_icon: true }
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

#[component]
fn SidebarControlled() -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        Example { title: "Controlled", class: FRAME,
            Frame {
                SidebarProvider { open: open(), on_open_change: move |value| open.set(value),
                    Sidebar {
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Projects" }
                                SidebarGroupContent {
                                    SidebarMenu {
                                        for project in PROJECTS_EXTENDED.iter() {
                                            SidebarMenuItem { key: "{project.name}",
                                                SidebarMenuButton { render: as_link,
                                                    {(project.icon)()}
                                                    span { "{project.name}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    SidebarInset {
                        header { class: "flex h-12 items-center justify-between px-4",
                            Button {
                                onclick: move |_| open.set(!open()),
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Ghost,
                                if open() {
                                    PanelLeftClose {}
                                } else {
                                    PanelLeftOpen {}
                                }
                                span {
                                    if open() {
                                        "Close"
                                    } else {
                                        "Open"
                                    }
                                    " Sidebar"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

type RtlNav = (&'static str, fn() -> Element, bool, [&'static str; 4]);
type RtlProject = (&'static str, fn() -> Element);

const RTL_NAV: [RtlNav; 4] = [
    (
        "ملعب",
        square_terminal,
        true,
        ["السجل", "المميز", "الإعدادات", ""],
    ),
    (
        "النماذج",
        bot,
        false,
        ["جينيسيس", "إكسبلورر", "كوانتوم", ""],
    ),
    (
        "التوثيق",
        book_open,
        false,
        ["مقدمة", "ابدأ", "الدروس", "سجل التغييرات"],
    ),
    (
        "الإعدادات",
        settings_2,
        false,
        ["عام", "الفريق", "الفوترة", "الحدود"],
    ),
];

const RTL_PROJECTS: [RtlProject; 3] = [
    ("هندسة التصميم", frame_icon),
    ("المبيعات والتسويق", pie_chart),
    ("السفر", map_icon),
];

#[component]
fn RtlContent() -> Element {
    let sidebar = use_sidebar();
    let side = if sidebar.is_mobile() {
        DropdownMenuSide::Bottom
    } else {
        DropdownMenuSide::InlineEnd
    };

    rsx! {
        Sidebar {
            collapsible: SidebarCollapsible::Icon,
            dir: "rtl",
            side: SidebarSide::Right,
            variant: SidebarVariant::Floating,
            SidebarHeader {
                SidebarMenu {
                    SidebarMenuItem {
                        SidebarMenuButton { size: SidebarMenuButtonSize::Lg, render: as_link,
                            div { class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground",
                                GalleryVerticalEnd { class: "size-4" }
                            }
                            div { class: "flex flex-col gap-0.5 leading-none",
                                span { class: "font-medium", "شركة أكمي" }
                                span { "المؤسسة" }
                            }
                        }
                    }
                }
            }
            SidebarContent {
                SidebarGroup {
                    SidebarGroupLabel { "المنصة" }
                    SidebarMenu {
                        for (title , icon , is_active , items) in RTL_NAV {
                            Collapsible {
                                key: "{title}",
                                default_open: is_active,
                                class: "group/collapsible",
                                SidebarMenuItem {
                                    SidebarMenuButton {
                                        tooltip: title,
                                        render: as_collapsible_trigger,
                                        {icon()}
                                        span { "{title}" }
                                        ChevronRight { class: "ms-auto transition-transform duration-200 group-data-open/collapsible:rotate-90 rtl:rotate-180 rtl:group-data-open/collapsible:rotate-90" }
                                    }
                                    CollapsibleContent {
                                        SidebarMenuSub {
                                            for sub_title in items.into_iter().filter(|title| !title.is_empty()) {
                                                SidebarMenuSubItem { key: "{sub_title}",
                                                    SidebarMenuSubButton { render: as_link,
                                                        span { "{sub_title}" }
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
                SidebarGroup { class: "group-data-[collapsible=icon]:hidden",
                    SidebarGroupLabel { "المشاريع" }
                    SidebarMenu {
                        for (name , icon) in RTL_PROJECTS {
                            SidebarMenuItem { key: "{name}",
                                SidebarMenuButton { render: as_link,
                                    {icon()}
                                    span { "{name}" }
                                }
                                DropdownMenu {
                                    SidebarMenuAction {
                                        show_on_hover: true,
                                        render: as_dropdown_trigger,
                                        Ellipsis {}
                                        span { class: "sr-only", "المزيد" }
                                    }
                                    DropdownMenuContent {
                                        class: "w-48 rounded-lg",
                                        side,
                                        align: if sidebar.is_mobile() { DropdownMenuAlign::End } else { DropdownMenuAlign::Start },
                                        dir: "rtl",
                                        DropdownMenuGroup {
                                            DropdownMenuItem {
                                                Folder { class: "text-muted-foreground" }
                                                span { "عرض المشروع" }
                                            }
                                            DropdownMenuItem {
                                                Forward { class: "text-muted-foreground" }
                                                span { "مشاركة المشروع" }
                                            }
                                        }
                                        DropdownMenuSeparator {}
                                        DropdownMenuGroup {
                                            DropdownMenuItem {
                                                Trash { class: "text-muted-foreground" }
                                                span { "حذف المشروع" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        SidebarMenuItem {
                            SidebarMenuButton { class: "text-sidebar-foreground/70",
                                Ellipsis { class: "text-sidebar-foreground/70" }
                                span { "المزيد" }
                            }
                        }
                    }
                }
            }
            SidebarFooter {
                SidebarMenu {
                    SidebarMenuItem {
                        DropdownMenu {
                            SidebarMenuButton {
                                size: SidebarMenuButtonSize::Lg,
                                class: OPEN_MENU_BUTTON,
                                render: as_dropdown_trigger,
                                Avatar { class: "rounded-lg",
                                    AvatarImage { src: USER_AVATAR, alt: USER_NAME }
                                    AvatarFallback { class: "rounded-lg", "CN" }
                                }
                                div { class: "grid flex-1 text-start text-sm leading-tight",
                                    span { class: "truncate font-medium", "{USER_NAME}" }
                                    span { class: "truncate text-xs", "{USER_EMAIL}" }
                                }
                                ChevronsUpDown { class: "ms-auto size-4" }
                            }
                            DropdownMenuContent {
                                class: "w-(--anchor-width) min-w-56 rounded-lg",
                                side,
                                align: DropdownMenuAlign::End,
                                side_offset: 4.0,
                                dir: "rtl",
                                DropdownMenuGroup {
                                    DropdownMenuLabel { class: "p-0 font-normal",
                                        div { class: "flex items-center gap-2 px-1 py-1.5 text-left text-sm",
                                            Avatar {
                                                AvatarImage { src: USER_AVATAR, alt: USER_NAME }
                                                AvatarFallback { class: "rounded-lg", "CN" }
                                            }
                                            div { class: "grid flex-1 text-start text-sm leading-tight",
                                                span { class: "truncate font-medium", "{USER_NAME}" }
                                                span { class: "truncate text-xs", "{USER_EMAIL}" }
                                            }
                                        }
                                    }
                                }
                                DropdownMenuSeparator {}
                                DropdownMenuGroup {
                                    DropdownMenuItem {
                                        Sparkles {}
                                        "ترقية إلى Pro"
                                    }
                                }
                                DropdownMenuSeparator {}
                                DropdownMenuGroup {
                                    DropdownMenuItem {
                                        BadgeCheck {}
                                        "الحساب"
                                    }
                                    DropdownMenuItem {
                                        CreditCard {}
                                        "الفوترة"
                                    }
                                    DropdownMenuItem {
                                        Bell {}
                                        "الإشعارات"
                                    }
                                }
                                DropdownMenuSeparator {}
                                DropdownMenuGroup {
                                    DropdownMenuItem {
                                        LogOut {}
                                        "تسجيل الخروج"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            SidebarRail {}
        }
        SidebarInset {
            header {
                class: "flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12",
                dir: "rtl",
                div { class: "flex items-center gap-2 px-4",
                    SidebarTrigger { class: "-ml-1" }
                }
            }
        }
    }
}

#[component]
fn SidebarRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: FRAME,
            Frame {
                div { class: "relative h-full", dir: "rtl",
                    SidebarProvider {
                        RtlContent {}
                    }
                }
            }
        }
    }
}
