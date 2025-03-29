//! Lints that offer suggestions on how to simplify your code.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::declare_group;

declare_group! {
    pub(crate) static COMPLEXITY = {
        name: "bevy::complexity",
        level: Level::Warn,
    };
}
