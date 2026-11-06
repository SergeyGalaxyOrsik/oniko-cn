//! Code block: displays code, terminal output or a diff in a surface with an optional header, footer and copy button.
//! `highlight_lines` colors code in pure Rust, with the GitHub light and dark colors, so it works on web and desktop.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::core::{DynamicNode, TemplateNode, VNode};
use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{Button, ButtonSize, ButtonVariant};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeBlockVariant {
    #[default]
    Default,
    Destructive,
}

impl CodeBlockVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => {
                "border border-input text-foreground dark:bg-input/30 has-[>[data-slot=code-block-content]:focus-visible]:border-ring"
            }
            Self::Destructive => {
                "border border-destructive/30 text-destructive dark:border-destructive/40"
            }
        }
    }

    /// Value of the `data-variant` attribute that the styles key off.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeBlockSize {
    #[default]
    Default,
    Sm,
}

impl CodeBlockSize {
    /// Value of the `data-size` attribute that the styles key off.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeBlockLineVariant {
    #[default]
    Default,
    Added,
    Removed,
    Highlighted,
}

impl CodeBlockLineVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Added => "bg-muted after:content-['+']",
            Self::Removed => "bg-destructive/10 after:text-destructive after:content-['−']",
            Self::Highlighted => "bg-muted",
        }
    }

    /// Value of the `data-variant` attribute that the styles key off.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Highlighted => "highlighted",
        }
    }
}

const COPY_SCRIPT: &str = "const text = await dioxus.recv(); try { await navigator.clipboard.writeText(text); dioxus.send(true); } catch (error) { dioxus.send(false); }";

#[component]
pub fn CodeBlock(
    #[props(default)] variant: CodeBlockVariant,
    #[props(default)] size: CodeBlockSize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "rounded-lg text-sm data-[size=sm]:text-xs has-[>[data-slot=code-block-content]:focus-visible]:ring-3 has-[>[data-slot=code-block-content]:focus-visible]:ring-ring/50 group/code-block relative flex w-full min-w-0 flex-col overflow-hidden",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block",
            "data-variant": variant.as_str(),
            "data-size": size.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CodeBlockHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 px-4 pt-4 text-muted-foreground group-data-[variant=destructive]/code-block:text-destructive group-data-[size=sm]/code-block:px-3 group-data-[size=sm]/code-block:pt-3 flex min-w-0 shrink-0 items-center *:has-[>[data-slot=code-block-title]]:min-w-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CodeBlockTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "font-sans text-sm font-medium text-foreground group-data-[variant=destructive]/code-block:text-destructive *:[svg]:me-1.5 [&_svg:not([class*='size-'])]:size-4 group-data-[size=sm]/code-block:text-xs group-data-[size=sm]/code-block:[&_svg:not([class*='size-'])]:size-3.5 min-w-0 truncate [&_svg]:pointer-events-none [&_svg]:inline-block [&_svg]:align-middle",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Holds buttons. Inside a header it sits at the end; as a direct child of `CodeBlock` it floats over the content.
#[component]
pub fn CodeBlockActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "-my-1 -me-1.5 gap-1 [[data-slot=code-block]>&]:m-0 [[data-slot=code-block]>&]:top-3 [[data-slot=code-block]>&]:end-3 group-data-[size=sm]/code-block:[[data-slot=code-block]>&]:top-2 group-data-[size=sm]/code-block:[[data-slot=code-block]>&]:end-2 [[data-slot=code-block]>&]:*:[button]:bg-background [[data-slot=code-block]>&]:*:[button]:ring-1 [[data-slot=code-block]>&]:*:[button]:ring-foreground/10 ms-auto flex shrink-0 items-center [[data-slot=code-block]>&]:absolute [[data-slot=code-block]>&]:z-10 [[data-slot=code-block]>&]:opacity-0 [[data-slot=code-block]>&]:transition-opacity [[data-slot=code-block]>&]:group-focus-within/code-block:opacity-100 [[data-slot=code-block]>&]:group-hover/code-block:opacity-100 [[data-slot=code-block]>&]:pointer-coarse:opacity-100",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Copies `value` to the clipboard. Shows a check and announces "Copied" for two seconds afterwards.
#[component]
pub fn CodeBlockCopyButton(
    value: String,
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconXs)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("", take_class(&mut attributes).as_deref());
    let mut copied = use_signal(|| false);
    // Each copy bumps the generation, so only the latest copy's timer resets the icon.
    let mut generation = use_signal(|| 0_u64);
    // The caller's attributes come after these, so they can override them.
    let own = [
        Attribute::new("data-slot", "code-block-copy-button", None, false),
        Attribute::new("data-copied", copied().then_some("true"), None, false),
        Attribute::new(
            "aria-label",
            if copied() { "Copied" } else { "Copy" },
            None,
            false,
        ),
        Attribute::new("class", class, None, false),
    ];
    attributes.splice(0..0, own);

    rsx! {
        Button {
            variant: variant,
            size: size,
            attributes: attributes,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                let text = value.clone();
                spawn(async move {
                    let mut eval = document::eval(COPY_SCRIPT);
                    let _ = eval.send(text);
                    let ok = eval.recv::<bool>().await.unwrap_or(false);
                    if !ok {
                        return;
                    }
                    let current = generation() + 1;
                    generation.set(current);
                    copied.set(true);
                    let _ = document::eval(
                        "await new Promise((resolve) => setTimeout(resolve, 2000))",
                    )
                    .await;
                    if generation() == current {
                        copied.set(false);
                    }
                });
            },
            svg {
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                if copied() {
                    path { d: "M20 6 9 17l-5-5" }
                } else {
                    rect {
                        width: "14",
                        height: "14",
                        x: "8",
                        y: "8",
                        rx: "2",
                        ry: "2",
                    }
                    path { d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" }
                }
            }
        }
        span { "aria-live": "polite", class: "sr-only",
            if copied() {
                "Copied"
            }
        }
    }
}

/// The code itself. Pass `CodeBlockLine` children for per-line styling or highlighting.
/// `code` is the stand-in for upstream's string child: with `line_numbers` it is split into
/// numbered lines, otherwise it renders as plain text.
#[component]
pub fn CodeBlockContent(
    #[props(default)] wrap: bool,
    #[props(default)] line_numbers: bool,
    #[props(default = 1)] start_line: i64,
    code: Option<String>,
    #[props(extends = GlobalAttributes, extends = pre)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "px-4 py-4 leading-relaxed group-has-data-[slot=code-block-header]/code-block:pt-2 group-has-data-[slot=code-block-footer]/code-block:pb-2 group-data-[size=sm]/code-block:px-3 group-data-[size=sm]/code-block:py-3 group-data-[size=sm]/code-block:group-has-data-[slot=code-block-header]/code-block:pt-1.5 min-h-0 min-w-0 scroll-fade-x scrollbar-thin overflow-auto text-start font-mono outline-none data-wrap:scroll-fade-y data-wrap:wrap-break-word data-wrap:whitespace-pre-wrap",
        take_class(&mut attributes).as_deref()
    );
    let style = format!("counter-reset: line {}", start_line - 1);

    rsx! {
        pre {
            "data-slot": "code-block-content",
            "data-wrap": wrap.then_some("true"),
            "data-numbered": line_numbers.then_some("true"),
            dir: "ltr",
            tabindex: "0",
            style: style,
            class: class,
            ..attributes,
            code {
                "data-slot": "code-block-code",
                class: "grid min-h-lh min-w-max content-start in-data-numbered:grid-cols-[auto_1fr] in-data-wrap:min-w-0",
                if let Some(code) = code {
                    if line_numbers {
                        for (index , line) in code.strip_suffix('\n').unwrap_or(&code).split('\n').enumerate() {
                            CodeBlockLine { key: "{index}", "{line}" }
                        }
                    } else {
                        "{code}"
                    }
                }
                {children}
            }
        }
    }
}

/// A node renders nothing when it has no content, so a line made of it needs a newline to survive copying.
fn is_blank(node: &VNode) -> bool {
    node.template.roots.iter().all(|root| match root {
        TemplateNode::Dynamic { id } => match &node.dynamic_nodes[*id] {
            DynamicNode::Text(text) => text.value.is_empty(),
            DynamicNode::Placeholder(_) => true,
            DynamicNode::Fragment(nodes) => nodes.iter().all(is_blank),
            DynamicNode::Component(_) => false,
        },
        TemplateNode::Text { text } => text.is_empty(),
        TemplateNode::Element { .. } => false,
    })
}

/// One line of code inside `CodeBlockContent`.
#[component]
pub fn CodeBlockLine(
    #[props(default)] variant: CodeBlockLineVariant,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "-mx-4 px-4 in-data-numbered:gap-x-3 before:min-w-6 before:text-end before:text-muted-foreground after:start-1 after:text-muted-foreground group-data-[size=sm]/code-block:-mx-3 group-data-[size=sm]/code-block:px-3 relative block min-h-lh after:absolute after:top-0 after:select-none in-data-numbered:col-span-2 in-data-numbered:grid in-data-numbered:grid-cols-subgrid in-data-numbered:[counter-increment:line] in-data-numbered:before:content-[counter(line)] in-data-numbered:before:select-none in-data-numbered:supports-[content:counter(line)_/_'']:before:content-[counter(line)_/_'']",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );
    let empty = children.as_ref().is_ok_and(is_blank);

    rsx! {
        span {
            "data-slot": "code-block-line",
            "data-variant": variant.as_str(),
            class: class,
            ..attributes,
            span { "data-slot": "code-block-line-content", class: "min-w-0",
                if empty {
                    "\n"
                } else {
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn CodeBlockFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 px-4 pb-4 text-muted-foreground group-data-[size=sm]/code-block:px-3 group-data-[size=sm]/code-block:pb-3 flex min-w-0 shrink-0 items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// GitHub light and dark colors, the same pair that shiki's `github-light` and `github-dark` themes use.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Plain,
    Comment,
    Keyword,
    Str,
    Constant,
    Function,
    Tag,
}

impl Kind {
    fn colors(self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Plain => None,
            Self::Comment => Some(("#6a737d", "#6a737d")),
            Self::Keyword => Some(("#d73a49", "#f97583")),
            Self::Str => Some(("#032f62", "#9ecbff")),
            Self::Constant => Some(("#005cc5", "#79b8ff")),
            Self::Function => Some(("#6f42c1", "#b392f0")),
            Self::Tag => Some(("#22863a", "#85e89d")),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Token {
    text: String,
    colors: Option<(&'static str, &'static str)>,
    dim: bool,
    bold: bool,
}

struct Syntax {
    line_comments: &'static [&'static str],
    block_comment: Option<(&'static str, &'static str)>,
    quotes: &'static [char],
    keywords: &'static [&'static str],
    constants: &'static [&'static str],
    case_insensitive: bool,
    /// An identifier followed by `:` is a property name.
    property_colon: Option<Kind>,
    /// A string followed by `:` is a key.
    string_keys: bool,
    /// A capitalized identifier is a type.
    capitalized_types: bool,
    /// The first word of a command is a function, and `$name` is a variable.
    shell: bool,
    /// `-` belongs to identifiers, as in CSS.
    dashed: bool,
}

const NONE: Syntax = Syntax {
    line_comments: &[],
    block_comment: None,
    quotes: &[],
    keywords: &[],
    constants: &[],
    case_insensitive: false,
    property_colon: None,
    string_keys: false,
    capitalized_types: false,
    shell: false,
    dashed: false,
};

const JS_KEYWORDS: &[&str] = &[
    "const",
    "let",
    "var",
    "function",
    "return",
    "if",
    "else",
    "for",
    "while",
    "do",
    "switch",
    "case",
    "break",
    "continue",
    "new",
    "class",
    "extends",
    "import",
    "export",
    "from",
    "default",
    "async",
    "await",
    "try",
    "catch",
    "finally",
    "throw",
    "typeof",
    "instanceof",
    "in",
    "of",
    "void",
    "delete",
    "yield",
    "static",
    "interface",
    "type",
    "enum",
    "implements",
    "public",
    "private",
    "protected",
    "readonly",
    "as",
    "satisfies",
    "declare",
    "namespace",
    "abstract",
];

fn syntax_for(language: &str) -> Option<Syntax> {
    let syntax = match language {
        "js" | "javascript" | "jsx" | "ts" | "typescript" | "tsx" => Syntax {
            line_comments: &["//"],
            block_comment: Some(("/*", "*/")),
            quotes: &['"', '\'', '`'],
            keywords: JS_KEYWORDS,
            constants: &["true", "false", "null", "undefined", "this", "super"],
            capitalized_types: true,
            ..NONE
        },
        "json" | "jsonc" => Syntax {
            quotes: &['"'],
            constants: &["true", "false", "null"],
            string_keys: true,
            ..NONE
        },
        "bash" | "sh" | "shell" | "shellscript" | "zsh" => Syntax {
            line_comments: &["#"],
            quotes: &['"', '\''],
            keywords: &[
                "if", "then", "else", "elif", "fi", "for", "while", "do", "done", "case", "esac",
                "in", "function", "return", "export", "local",
            ],
            shell: true,
            ..NONE
        },
        "css" => Syntax {
            block_comment: Some(("/*", "*/")),
            quotes: &['"', '\''],
            property_colon: Some(Kind::Constant),
            dashed: true,
            ..NONE
        },
        "python" | "py" => Syntax {
            line_comments: &["#"],
            quotes: &['"', '\''],
            keywords: &[
                "def", "return", "if", "elif", "else", "for", "while", "in", "import", "from",
                "as", "class", "lambda", "with", "try", "except", "finally", "raise", "pass",
                "break", "continue", "and", "or", "not", "is", "yield", "async", "await", "global",
                "nonlocal", "del", "assert",
            ],
            constants: &["True", "False", "None", "self"],
            capitalized_types: true,
            ..NONE
        },
        "sql" => Syntax {
            line_comments: &["--"],
            block_comment: Some(("/*", "*/")),
            quotes: &['\'', '"'],
            keywords: &[
                "select",
                "from",
                "where",
                "insert",
                "into",
                "values",
                "update",
                "set",
                "delete",
                "create",
                "table",
                "join",
                "left",
                "right",
                "inner",
                "outer",
                "on",
                "group",
                "by",
                "order",
                "having",
                "limit",
                "as",
                "and",
                "or",
                "not",
                "is",
                "in",
                "like",
                "distinct",
                "union",
                "all",
                "primary",
                "key",
                "foreign",
                "references",
                "default",
                "alter",
                "drop",
                "index",
                "offset",
                "between",
                "exists",
                "case",
                "when",
                "then",
                "else",
                "end",
            ],
            constants: &["null", "true", "false"],
            case_insensitive: true,
            ..NONE
        },
        "yaml" | "yml" => Syntax {
            line_comments: &["#"],
            quotes: &['"', '\''],
            constants: &["true", "false", "null"],
            property_colon: Some(Kind::Tag),
            ..NONE
        },
        _ => return None,
    };
    Some(syntax)
}

fn is_ident_start(c: char, syntax: &Syntax) -> bool {
    c.is_alphabetic() || c == '_' || c == '$' && !syntax.shell || c == '@' && syntax.dashed
}

fn is_ident_char(c: char, syntax: &Syntax) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' && !syntax.shell || c == '-' && syntax.dashed
}

fn starts_with_at(chars: &[char], at: usize, prefix: &str) -> bool {
    prefix
        .chars()
        .enumerate()
        .all(|(offset, c)| chars.get(at + offset) == Some(&c))
}

fn next_significant(chars: &[char], mut at: usize) -> Option<char> {
    while let Some(&c) = chars.get(at) {
        if c != ' ' && c != '\t' {
            return Some(c);
        }
        at += 1;
    }
    None
}

fn scan(code: &str, syntax: &Syntax) -> Vec<(Kind, String)> {
    let chars: Vec<char> = code.chars().collect();
    let mut tokens: Vec<(Kind, String)> = Vec::new();
    let mut push = |kind: Kind, text: String| {
        match tokens.last_mut() {
            Some((last, existing)) if *last == kind => existing.push_str(&text),
            _ => tokens.push((kind, text)),
        };
    };
    let mut at = 0;
    let mut command_position = syntax.shell;

    while at < chars.len() {
        let c = chars[at];

        if c == '\n' {
            command_position = syntax.shell;
            push(Kind::Plain, "\n".into());
            at += 1;
            continue;
        }

        if let Some(prefix) = syntax
            .line_comments
            .iter()
            .find(|prefix| starts_with_at(&chars, at, prefix))
        {
            // A `#` inside a word such as `foo#bar` is not a shell comment.
            let word_start = at == 0 || chars[at - 1].is_whitespace();
            if prefix != &"#" || !syntax.shell || word_start {
                let end = chars[at..]
                    .iter()
                    .position(|&c| c == '\n')
                    .map_or(chars.len(), |len| at + len);
                push(Kind::Comment, chars[at..end].iter().collect());
                at = end;
                continue;
            }
        }

        if let Some((open, close)) = syntax.block_comment
            && starts_with_at(&chars, at, open)
        {
            let mut end = at + open.len();
            while end < chars.len() && !starts_with_at(&chars, end, close) {
                end += 1;
            }
            end = (end + close.len()).min(chars.len());
            push(Kind::Comment, chars[at..end].iter().collect());
            at = end;
            continue;
        }

        if syntax.quotes.contains(&c) {
            let mut end = at + 1;
            while end < chars.len() {
                match chars[end] {
                    '\\' => end += 2,
                    '\n' if c != '`' => break,
                    q if q == c => {
                        end += 1;
                        break;
                    }
                    _ => end += 1,
                }
            }
            let end = end.min(chars.len());
            let key = syntax.string_keys && next_significant(&chars, end) == Some(':');
            let kind = if key { Kind::Constant } else { Kind::Str };
            push(kind, chars[at..end].iter().collect());
            command_position = false;
            at = end;
            continue;
        }

        if c.is_ascii_digit() || (c == '#' && syntax.dashed) {
            let mut end = at + 1;
            while end < chars.len()
                && (chars[end].is_alphanumeric() || chars[end] == '.' || chars[end] == '_')
            {
                end += 1;
            }
            push(Kind::Constant, chars[at..end].iter().collect());
            command_position = false;
            at = end;
            continue;
        }

        if syntax.shell && c == '$' {
            let mut end = at + 1;
            if chars.get(end) == Some(&'{') {
                while end < chars.len() && chars[end] != '}' {
                    end += 1;
                }
                end = (end + 1).min(chars.len());
            } else {
                while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }
            }
            // A lone `$` is a prompt, and the command follows it.
            if end > at + 1 {
                push(Kind::Constant, chars[at..end].iter().collect());
                command_position = false;
            } else {
                push(Kind::Plain, "$".into());
            }
            at = end;
            continue;
        }

        if syntax.shell
            && c == '-'
            && chars
                .get(at + 1)
                .is_some_and(|n| n.is_alphanumeric() || *n == '-')
            && (at == 0 || chars[at - 1].is_whitespace())
        {
            let mut end = at + 1;
            while end < chars.len()
                && (chars[end].is_alphanumeric() || chars[end] == '-' || chars[end] == '_')
            {
                end += 1;
            }
            push(Kind::Constant, chars[at..end].iter().collect());
            at = end;
            continue;
        }

        if is_ident_start(c, syntax) {
            let mut end = at + 1;
            while end < chars.len() && is_ident_char(chars[end], syntax) {
                end += 1;
            }
            let word: String = chars[at..end].iter().collect();
            let lookup = if syntax.case_insensitive {
                word.to_lowercase()
            } else {
                word.clone()
            };
            let next = next_significant(&chars, end);
            let kind = if syntax.keywords.contains(&lookup.as_str()) || word.starts_with('@') {
                Kind::Keyword
            } else if syntax.constants.contains(&lookup.as_str()) {
                Kind::Constant
            } else if let (Some(kind), Some(':')) = (syntax.property_colon, next) {
                kind
            } else if (syntax.shell && command_position)
                || (next == Some('(') && !syntax.shell)
                || (syntax.capitalized_types && word.starts_with(char::is_uppercase))
            {
                Kind::Function
            } else {
                Kind::Plain
            };
            push(kind, word);
            command_position = false;
            at = end;
            continue;
        }

        if syntax.shell && matches!(c, '|' | '&' | ';' | '(') {
            command_position = true;
        } else if !c.is_whitespace() {
            command_position = false;
        }
        push(Kind::Plain, c.to_string());
        at += 1;
    }

    tokens
}

fn split_lines(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut lines: Vec<Vec<Token>> = vec![Vec::new()];
    for token in tokens {
        let mut parts = token.text.split('\n');
        if let Some(first) = parts.next() {
            push_part(&mut lines, &token, first);
        }
        for part in parts {
            lines.push(Vec::new());
            push_part(&mut lines, &token, part);
        }
    }
    lines
}

fn push_part(lines: &mut [Vec<Token>], token: &Token, text: &str) {
    if text.is_empty() {
        return;
    }
    if let Some(line) = lines.last_mut() {
        line.push(Token {
            text: text.to_string(),
            ..token.clone()
        });
    }
}

const ANSI_COLORS: [(&str, &str); 8] = [
    ("#24292e", "#e1e4e8"),
    ("#d73a49", "#f97583"),
    ("#22863a", "#85e89d"),
    ("#b08800", "#ffea7f"),
    ("#005cc5", "#79b8ff"),
    ("#6f42c1", "#b392f0"),
    ("#1b7c83", "#56d4dd"),
    ("#6a737d", "#959da5"),
];

/// Colors from SGR escape sequences. Each line is read on its own, so it looks the same whether it arrives whole or streams.
fn scan_ansi(code: &str) -> Vec<Vec<Token>> {
    code.split('\n')
        .map(|line| {
            let mut tokens = Vec::new();
            let mut colors = None;
            let mut dim = false;
            let mut bold = false;
            let mut rest = line;
            while !rest.is_empty() {
                let escape = rest.find("\x1b[");
                let text = &rest[..escape.unwrap_or(rest.len())];
                if !text.is_empty() {
                    tokens.push(Token {
                        text: text.to_string(),
                        colors,
                        dim,
                        bold,
                    });
                }
                let Some(start) = escape else { break };
                let after = &rest[start + 2..];
                let Some(end) = after.find('m') else {
                    break;
                };
                for code in after[..end].split(';') {
                    match code.parse::<u32>().unwrap_or(0) {
                        0 => {
                            colors = None;
                            dim = false;
                            bold = false;
                        }
                        1 => bold = true,
                        2 => dim = true,
                        22 => {
                            dim = false;
                            bold = false;
                        }
                        39 => colors = None,
                        n @ 30..=37 => colors = ANSI_COLORS.get((n - 30) as usize).copied(),
                        n @ 90..=97 => colors = ANSI_COLORS.get((n - 90) as usize).copied(),
                        _ => {}
                    }
                }
                rest = &after[end + 1..];
            }
            tokens
        })
        .collect()
}

fn token_lines(code: &str, language: &str) -> Vec<Vec<Token>> {
    let language = language.to_lowercase();
    if language == "ansi" {
        return scan_ansi(code);
    }
    if language == "diff" {
        return code
            .split('\n')
            .map(|line| {
                let kind = match line.chars().next() {
                    Some('+') => Kind::Tag,
                    Some('-') => Kind::Keyword,
                    Some('@') => Kind::Constant,
                    _ => Kind::Plain,
                };
                vec![Token {
                    text: line.to_string(),
                    colors: kind.colors(),
                    dim: false,
                    bold: false,
                }]
            })
            .collect();
    }
    match syntax_for(&language) {
        Some(syntax) => split_lines(
            scan(code, &syntax)
                .into_iter()
                .map(|(kind, text)| Token {
                    text,
                    colors: kind.colors(),
                    dim: false,
                    bold: false,
                })
                .collect(),
        ),
        None => code
            .split('\n')
            .map(|line| {
                vec![Token {
                    text: line.to_string(),
                    colors: None,
                    dim: false,
                    bold: false,
                }]
            })
            .collect(),
    }
}

/// Highlights `code` and returns one element per line, ready to put inside a `CodeBlockLine`.
///
/// Counterpart of upstream's `useHighlightedCode`. Supported languages: `js`, `jsx`, `ts`, `tsx`,
/// `json`, `bash`, `css`, `python`, `sql`, `yaml`, `diff` and `ansi`. Any other language comes back as plain text.
/// It tokenizes the whole text on each call, which is cheap enough to call while code streams in.
pub fn highlight_lines(code: &str, language: &str) -> Vec<Element> {
    let code = code.strip_suffix('\n').unwrap_or(code);
    token_lines(code, language)
        .into_iter()
        .map(|line| {
            rsx! {
                for token in line {
                    if token.colors.is_none() && !token.dim && !token.bold {
                        "{token.text}"
                    } else {
                        span {
                            class: cn!(
                                token.colors.map(|_| "text-(--shiki-light) dark:text-(--shiki-dark)"),
                                token.dim.then_some("opacity-60"),
                                token.bold.then_some("font-bold")
                            ),
                            style: token
                                .colors
                                .map(|(light, dark)| format!("--shiki-light:{light};--shiki-dark:{dark}")),
                            "{token.text}"
                        }
                    }
                }
            }
        })
        .collect()
}
