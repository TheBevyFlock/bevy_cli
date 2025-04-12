//! Lints that offer suggestions on how to simplify your code.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup2;

pub(crate) struct Complexity;

impl LintGroup2 for Complexity {
    const NAME: &str = "bevy::complexity";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {}
}
