//! Lints that suggest how to improve the performance of your code.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::lint::LintGroup;

pub(crate) static PERFORMANCE: &LintGroup = &LintGroup {
    name: "bevy::performance",
    level: Level::Warn,
};
