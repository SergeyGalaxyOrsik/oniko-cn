# How to write code in oniko-cn

oniko-cn is a shadcn/ui-style component library for Dioxus 0.7. Users never depend on the components as a crate. `oniko add` copies a component's source into their app, so every file in `registry/ui/` is code a stranger will read and edit.

Every component must work the same in a browser (`dx serve --web`) and in a desktop window (`dx serve --desktop`).

## Before you hand work back

Run `scripts/check.sh`. It runs `cargo fmt --check`, clippy with `-D warnings` for the web and desktop builds, and all tests. The tests in `crates/oniko-registry/tests/repo.rs` enforce most rules in this file, and each failure message says how to fix it.

If you changed a component or `apps/preview`, also build and look at both targets:

```sh
cd apps/preview
dx serve --web       # open the printed URL
dx serve --desktop
```

A passing `cargo check` does not prove a component renders. Look at it.

## Know which files are generated

| Path | Owner | How to change it |
| --- | --- | --- |
| `registry/ui/*.rs` | you | Edit directly. This is the source of every component. |
| `registry/registry.json` | you | One entry per component. |
| `registry/styles/*.css`, `registry/themes/*.json` | shadcn/ui | Copied from shadcn/ui. Change them only to add a missing `cn-*` rule. |
| `public/r/` | `oniko build` | Run `cargo run -p oniko-cli -- build`. Never edit by hand. |
| `apps/preview/src/components/ui/` | `oniko add` | Run `cargo run -p oniko-cli -- add <name> --overwrite --cwd apps/preview`. Never edit by hand. |
| `apps/preview/tailwind.css` | `oniko init` | Only the block between the `oniko-cn:begin` and `oniko-cn:end` markers is generated. |

To change a component, edit `registry/ui/<name>.rs`, rebuild `public/r`, then reinstall it into the preview.

## Add a component

1. Write `registry/ui/<name>.rs`, following the rules in the next section.
2. Add an entry to `registry/registry.json` with its `dependencies` (Cargo crates as `name@version`) and `registryDependencies`.
3. If a placeholder class has no rule in a style, add the `.cn-...` rule to every file in `registry/styles/`.
4. Run `cargo run -p oniko-cli -- build`.
5. Run `cargo run -p oniko-cli -- add <name> --cwd apps/preview` and add a section for it in `apps/preview/src/main.rs` that shows every variant and size.
6. Add a row to `COMPONENTS.md`.
7. Run `scripts/check.sh`, then look at the component in `dx serve --web` and `dx serve --desktop`.

## Write components the same way

`registry/ui/button.rs` is the reference. Copy its shape.

- **Put the look in placeholders.** Write colors, borders, radii, spacing, and typography as `cn-*` placeholder classes, such as `cn-button-variant-outline`. Write only layout and behavior classes as plain Tailwind, such as `inline-flex` or `disabled:pointer-events-none`. `oniko build` replaces each placeholder with the classes that each style defines, so one source serves all 8 styles.
- **Name placeholders after shadcn/ui.** Use `cn-<component>[-<part>][-variant-<v>|-size-<s>]`, the same names shadcn's style sheets use. Then the vendored styles already contain the rules.
- **Model variants as enums.** Each variant set is a `Copy` enum with a `#[default]` case and a `fn class(self) -> &'static str` that returns its placeholder. Do not use strings or booleans for variants.
- **Accept HTML attributes.** Add `#[props(extends = GlobalAttributes, extends = <element>)] attributes: Vec<Attribute>` and spread it with `..attributes`.
- **Merge the caller's class.** Call `take_class(&mut attributes)` and pass the result last to `cn!`, so `class: "rounded-full"` overrides the style.
- **Mark every element.** Set `"data-slot": "<component>[-<part>]"` on each element a component renders. Styles and parents target these.
- **Forward events explicitly.** Take event handlers as `Option<EventHandler<...>>` props and call them, as `Button` does with `onclick` and `onkeydown`.
- **Export the classes when another element needs the look.** For example, `button_variants(variant, size, class)` lets a link look like a button.
- **Split compound components into parts.** Export one component per part (`Card`, `CardHeader`, `CardTitle`) and share state through Dioxus context, not through props passed down by hand.
- **Start each file with its doc comment.** The doc comment says what the component is, plus the line `Installed by oniko-cn. This file is yours: edit it freely.`

## Keep components platform-neutral

The desktop target runs Rust natively and renders in a webview, so browser-only crates do not exist there.

- Do not use `web_sys`, `wasm_bindgen`, `js_sys`, or `gloo` in `registry/ui/`. A test fails if you do.
- Read layout and set focus through Dioxus `MountedData` (`get_client_rect`, `set_focus`).
- If you need JavaScript, run it with `document::eval`. Put any JS file in the component's assets and load it with `asset!()`. Never use npm packages.
- For overlays, focus traps, and keyboard navigation, use `dioxus-primitives` instead of writing your own. Write your own primitive only when no primitive covers the behavior, and say why in the PR.

## Keep the runtime crate small

`crates/oniko-cn` is compiled into every user app, including the WASM bundle. Add a dependency to it only if every component needs it. Helpers that only some components need belong in the component file or in a separate registry item of type `registry:lib`.

## Rust conventions

- **Edition 2024, `cargo fmt`, clippy clean.** The workspace forbids `unsafe` and denies `unwrap`, `expect`, `dbg!`, and `todo!` outside tests.
- **Errors.** Return `anyhow::Result` and add `.context(...)` where the CLI touches files, the network, or user config. Inside, pass typed data and do not re-validate it. Parse JSON and TOML into the types in `crates/oniko-registry/src/schema.rs` at the point where they are read.
- **Keep CLI commands thin.** Put logic in plain functions in `oniko-registry` or `project.rs` that take paths and data and can be unit tested. `main.rs` only parses arguments and prints.
- **Make commands safe to repeat.** Running `oniko init` or `oniko add` twice must leave the same files as running it once. Add a test when you add a command.
- **Tests.** Test what a caller can observe, such as the files a command writes or the string `cn!` returns. Do not assert on private helpers or exact error wording.
- **Comments.** Write a comment only for a reason the code cannot show. Do not narrate what the next line does.
- **Language.** Write code, comments, and docs in English. `COMPONENTS.md` is the exception and stays in Russian.

## Credit shadcn/ui

The styles, themes, and parts of `registry/base.css` come from shadcn/ui under the MIT license. Keep `NOTICE` up to date when you copy more of their files.
