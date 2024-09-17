// Enables linking to `rustc` crates.
#![feature(rustc_private)]
// Allows chaining `if let` multiple times using `&&`.
#![feature(let_chains)]

// This is a list of every single `rustc` crate used within this library. If you need another, add
// it here!
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

mod callback;
pub mod lints;

pub use self::callback::BevyLintCallback;
