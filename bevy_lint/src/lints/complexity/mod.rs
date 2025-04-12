//! Lints that offer suggestions on how to simplify your code.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint};

use crate::lint::{LintGroup, LintGroup2};

pub(crate) struct Complexity;

impl LintGroup2 for Complexity {
    const NAME: &str = "bevy::complexity";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[];
}

pub(crate) static COMPLEXITY: &LintGroup = &LintGroup {
    name: "bevy::complexity",
    level: Level::Warn,
};
