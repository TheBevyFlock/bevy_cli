//! Lints that offer suggestions on how to simplify your code.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::lint::LintGroup;

pub(crate) static COMPLEXITY: &LintGroup = &LintGroup {
    name: "bevy::complexity",
    level: Level::Warn,
};
