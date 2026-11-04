//! Registry schema, style compiler and builder for oniko-cn.
//!
//! The pipeline mirrors shadcn/ui:
//! 1. Component sources (`registry/ui/*.rs`) use placeholder classes such as
//!    `cn-button-variant-default` instead of real Tailwind classes.
//! 2. Each style (`registry/styles/style-<name>.css`) maps those placeholders
//!    to Tailwind classes via `@apply`.
//! 3. [`build`] compiles every item once per style, inlining the classes, and
//!    writes shadcn-compatible `registry-item` JSON that the CLI installs.

pub mod build;
pub mod schema;
pub mod style_map;
pub mod theme;
pub mod transform;

pub use schema::*;
