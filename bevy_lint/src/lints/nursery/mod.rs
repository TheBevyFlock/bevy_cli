//! Unstable lints that may be removed at any time for any reason.
//!
//! These lints are **allow** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::{LintGroup, LintGroup2};

pub mod duplicate_bevy_dependencies;
pub mod zst_query;

pub(crate) struct Nursery;

impl LintGroup2 for Nursery {
    const NAME: &str = "bevy::nursery";
    const LEVEL: Level = Level::Allow;
    const LINTS: &[&Lint] = &[
        duplicate_bevy_dependencies::DUPLICATE_BEVY_DEPENDENCIES.lint,
        zst_query::ZST_QUERY.lint,
    ];

    fn register_passes(store: &mut LintStore) {
        // `duplicate_bevy_dependencies` is a Cargo lint, so it does not have its own pass.
        store.register_late_pass(|_| Box::new(zst_query::ZstQuery::default()));
    }
}

pub(crate) static NURSERY: &LintGroup = &LintGroup {
    name: "bevy::nursery",
    level: Level::Allow,
};
