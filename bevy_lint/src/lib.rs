// Enables linking to `rustc` crates.
#![feature(rustc_private)]

// This is a list of every single `rustc` crate used within this library. If you need another, add
// it here!
extern crate rustc_driver;
extern crate rustc_interface;

mod callback;

pub use self::callback::BevyLintCallback;
