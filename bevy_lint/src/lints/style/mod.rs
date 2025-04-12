//! Lints that encourage idiomatic code.
//!
//! These lints are opinionated and may be freely disabled if you disagree with their suggestions.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::lint::{LintGroup, LintGroup2};

pub mod plugin_not_ending_in_plugin;

pub(crate) struct Style;

impl LintGroup2 for Style {
    const NAME: &str = "bevy::style";
    const LEVEL: Level = Level::Warn;
}

pub(crate) static STYLE: &LintGroup = &LintGroup {
    name: "bevy::style",
    level: Level::Warn,
};
