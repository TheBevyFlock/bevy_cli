//! `bevy_lint` is a custom linter for the [Bevy game engine](https://bevyengine.org), similar to
//! [Clippy](https://doc.rust-lang.org/stable/clippy).
//!
//! This is the primary documentation for its lints and lint groups. `bevy_lint` is not intended to
//! be consumed as a library. You can find the documentation for individual lints and their groups
//! in the [`lints`] module.
//!
//! <!--
//! Override these links to point to the local copy of the docs.
//! For more info on how this works, see <https://linebender.org/blog/doc-include/>.
//! -->
//! [**Documentation**]: crate
//! [**All Lints**]: crate::lints
#![doc = include_str!("../README.md")]
// Enables linking to `rustc` crates.
#![feature(rustc_private)]
// Allows chaining `if let` multiple times using `&&`.
#![feature(let_chains)]
// Used to access the index of repeating macro input in `declare_bevy_symbols!`.
#![feature(macro_metavar_expr)]
// Warn on internal `rustc` lints that check for poor usage of internal compiler APIs. Note that
// you also need to pass `-Z unstable-options` to `rustc` for this to be enabled:
// `RUSTFLAGS="-Zunstable-options" cargo check`
#![warn(rustc::internal)]
#![allow(
    rustc::usage_of_ty_tykind,
    reason = "Many false positives without a valid replacement."
)]

// This is a list of every single `rustc` crate used within this library. If you need another, add
// it here!
extern crate rustc_abi;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_analysis;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_lint_defs;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_type_ir;

mod callback;
mod config;
mod lint;
pub mod lints;
mod paths;
mod sym;
mod utils;

pub use self::callback::BevyLintCallback;
