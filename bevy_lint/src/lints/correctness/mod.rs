//! Lints that check for outright incorrect code.
//!
//! Unlike [`suspicious`](super::suspicious) lints, these are carefully picked to be free of false
//! positives. You should avoid `#[allow(...)]`-ing these lints without a _very_ good reason.
//!
//! These lints are **deny** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub(crate) struct Correctness;

impl LintGroup for Correctness {
    const NAME: &str = "bevy::correctness";
    const LEVEL: Level = Level::Deny;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {}
}
