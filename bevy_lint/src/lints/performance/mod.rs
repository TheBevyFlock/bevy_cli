//! Lints that suggest how to improve the performance of your code.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup2;

pub(crate) struct Performance;

impl LintGroup2 for Performance {
    const NAME: &str = "bevy::performance";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {}
}
