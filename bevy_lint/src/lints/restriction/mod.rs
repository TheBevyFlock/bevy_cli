//! Opt-in lints that restrict you from writing certain code patterns.
//!
//! These are designed for scenarios where you want to increase the consistency of your project by
//! rejecting certain patterns. These lints should not all be enabled as a group, but instead
//! should be chosen individually after reading the documentation.
//!
//! These lints are **allow** by default.

use rustc_lint::Level;

use crate::lint::LintGroup;

pub mod missing_reflect;
pub mod panicking_methods;

pub(crate) static RESTRICTION: &LintGroup = &LintGroup {
    name: "bevy::restriction",
    level: Level::Allow,
};
