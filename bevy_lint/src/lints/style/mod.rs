//! Lints that encourage idiomatic code.
//!
//! These lints are opinionated and may be freely disabled if you disagree with their suggestions.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod bevy_platform_alternative_exists;
pub mod unconventional_naming;

pub(crate) struct Style;

impl LintGroup for Style {
    const NAME: &str = "bevy::style";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[
        bevy_platform_alternative_exists::BEVY_PLATFORM_ALTERNATIVE_EXISTS,
        unconventional_naming::UNCONVENTIONAL_NAMING,
    ];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| {
            Box::new(bevy_platform_alternative_exists::BevyPlatformAlternativeExists)
        });
        store.register_late_pass(|_| Box::new(unconventional_naming::UnconventionalNaming));
    }
}
