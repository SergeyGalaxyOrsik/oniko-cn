# oniko-cn

Beautiful, accessible components for [Dioxus](https://dioxuslabs.com), written
entirely in Rust, that run the same on **web and desktop**.

Like [shadcn/ui](https://ui.shadcn.com), this is not a dependency you import
components from: the CLI copies the component source into your crate, and from
then on the code is yours.

```sh
oniko init --registry <url-or-path>   # oniko.toml + theme CSS + module wiring
oniko add button                      # src/components/ui/button.rs
```

```rust
use components::ui::button::{Button, ButtonVariant};

rsx! {
    Button { variant: ButtonVariant::Outline, onclick: move |_| save(), "Save" }
}
```

## Layout

```text
crates/
  oniko-cn/        runtime crate: cn!() class merging, take_class() for spread attributes
  oniko-registry/  registry schema, style compiler, theme renderer, registry builder
  oniko-cli/       the `oniko` binary: init / add / list / build
registry/
  registry.json    item list (shadcn-compatible field names, Cargo crates as dependencies)
  ui/*.rs          component sources written with cn-* placeholder classes
  styles/*.css     8 styles (vega, nova, maia, lyra, mira, luma, rhea, sera)
  themes/*.json    base colors (neutral, stone, zinc, mauve, olive, mist, taupe)
  base.css         shared custom variants and keyframes
public/r/          `oniko build` output: what `init` / `add` read
apps/preview/      gallery app; its components are installed with `oniko add`
```

## How styles work

Component sources never hard-code a look. They use placeholder classes:

```rust
Self::Default => "cn-button-variant-default",
```

Each style sheet maps placeholders to Tailwind classes:

```css
.style-vega {
  .cn-button-variant-default { @apply bg-primary text-primary-foreground hover:bg-primary/80; }
}
```

`oniko build` compiles every component once per style, inlining the classes, so
the code a user installs contains plain Tailwind classes and no placeholders.
At runtime `cn!` merges the component's classes with the caller's `class`
(later wins: `class: "rounded-full"` overrides the style's radius).

Theme tokens are CSS variables (`--background`, `--primary`, ...) for `:root`
and `.dark`, mapped into Tailwind through `@theme inline`. `dx` picks up the
crate's `tailwind.css` automatically and writes `assets/tailwind.css`.

## Web and desktop

Components must work on every Dioxus renderer, so:

- no `web_sys` in components; DOM access goes through `MountedData` and
  `document::eval`;
- any JS helpers ship as assets via `asset!()`;
- every component is checked in `apps/preview` on both targets.

## Development

Read `AGENTS.md` before you change code. It lists the rules and how to add a component.

```sh
scripts/check.sh                   # fmt, clippy (web and desktop), tests
cargo run -p oniko-cli -- build    # registry/ -> public/r/
cd apps/preview && dx serve --web  # or: dx serve --desktop
```

## Credits

Styles, theme tokens and design are adapted from
[shadcn/ui](https://github.com/shadcn-ui/ui) (MIT). See `NOTICE`.
