//! `bevy_lint` is a Rust linter for the [Bevy game engine](https://bevyengine.org).
//!
//! This is the primary documentation for its supported lints and lint groups. It is not intended
//! to be consumed as a library.

// Enables linking to `rustc` crates.
#![feature(rustc_private)]
// Allows chaining `if let` multiple times using `&&`.
#![feature(let_chains)]
// Warn on internal `rustc` lints that check for poor usage of internal compiler APIs.
#![warn(rustc::internal)]

// This is a list of every single `rustc` crate used within this library. If you need another, add
// it here!
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod callback;
pub mod groups;
mod lint;
pub mod lints;
mod paths;

pub use self::callback::BevyLintCallback;
