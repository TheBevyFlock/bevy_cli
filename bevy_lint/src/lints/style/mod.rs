//! Lints that encourage idiomatic code.
//!
//! These lints are opinionated and may be freely disabled if you disagree with their suggestions.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod unconventional_naming;

pub(crate) struct Style;

impl LintGroup for Style {
    const NAME: &str = "bevy::style";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[unconventional_naming::UNCONVENTIONAL_NAMING];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| {
            Box::new(unconventional_naming::UnconventionalNaming)
        });
    }

    fn register_lints(store: &mut LintStore) {
        store.register_lints(Self::LINTS);

        // `plugin_not_ending_in_plugin` was merged into `unconventional_naming` in v0.3.0. This
        // helps users of v0.2.0 migrate to v0.3.0, but should be removed before v0.4.0 is
        // released.
        store.register_renamed(
            "bevy::plugin_not_ending_in_plugin",
            "bevy::unconventional_naming",
        );
    }
}
