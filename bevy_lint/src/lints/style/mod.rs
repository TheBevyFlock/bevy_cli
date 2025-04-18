//! Lints that encourage idiomatic code.
//!
//! These lints are opinionated and may be freely disabled if you disagree with their suggestions.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::lint::LintGroup;

pub mod plugin_not_ending_in_plugin;
pub mod unconventional_naming;

pub(crate) static STYLE: &LintGroup = &LintGroup {
    name: "bevy::style",
    level: Level::Warn,
};
