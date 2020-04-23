#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    bare_trait_objects,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::pedantic,
    clippy::result_unwrap_used
)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![allow(clippy::match_bool)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

#[macro_use]
mod general_macros; // macros must be declared before trait modules

mod check;
mod overflow;
mod saturate;
mod wrap;

pub use check::Check;
pub use overflow::Overflow;
pub use saturate::Saturate;
pub use wrap::Wrap;
