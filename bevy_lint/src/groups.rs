use crate::{
    lint::{BevyLint, LintGroup},
    lints::LINTS,
};
use rustc_lint::{Level, LintStore};

/// A group of deny-by-default lints that check for outright wrong or useless code.
///
/// These lints are carefully picked to be free of false positives. You should avoid
/// `#[allow(...)]`-ing these lints without a _very_ good reason.
pub static CORRECTNESS: &LintGroup = &LintGroup {
    name: "bevy::correctness",
    level: Level::Deny,
};

/// A group similar to [`CORRECTNESS`] that checks for suspicious or usually wrong code.
///
/// The linted code may have been written intentionally, but should probably still be fixed.
pub static SUSPICIOUS: &LintGroup = &LintGroup {
    name: "bevy::suspicious",
    level: Level::Warn,
};

/// A group that offers suggestions on how to simplify your code.
pub static COMPLEXITY: &LintGroup = &LintGroup {
    name: "bevy::complexity",
    level: Level::Warn,
};

/// A group that suggests how to increase the performance of your code.
pub static PERFORMANCE: &LintGroup = &LintGroup {
    name: "bevy::performance",
    level: Level::Warn,
};

/// A group of lints that encourage idiomatic code.
///
/// These lints are opinionated and may be freely disabled if you disagree with their suggestions.
pub static STYLE: &LintGroup = &LintGroup {
    name: "bevy::style",
    level: Level::Warn,
};

/// A group of lints that make the linter incredibly nit-picky.
///
/// If you enable this group, expect to liberally apply `#[allow(...)]` attributes throughout your
/// code.
pub static PEDANTIC: &LintGroup = &LintGroup {
    name: "bevy::pedantic",
    level: Level::Allow,
};

/// A group of opt-in lints that restrict you from writing certain code.
///
/// These are designed for scenarios where you want to increase the consistency of your code-base
/// and reject certain patterns. They should not all be enabled at once, but instead specific lints
/// should be individually enabled.
pub static RESTRICTION: &LintGroup = &LintGroup {
    name: "bevy::restriction",
    level: Level::Allow,
};

/// A group of unstable lints that may be removed at any time for any reason.
pub static NURSERY: &LintGroup = &LintGroup {
    name: "bevy::nursery",
    level: Level::Allow,
};

static GROUPS: &[&LintGroup] = &[
    CORRECTNESS,
    SUSPICIOUS,
    COMPLEXITY,
    PERFORMANCE,
    STYLE,
    PEDANTIC,
    RESTRICTION,
    NURSERY,
];

pub(crate) fn register_groups(store: &mut LintStore) {
    for &group in GROUPS {
        let lints = LINTS
            .into_iter()
            .copied()
            // Only select lints of this specified group.
            .filter(|l| l.group == group)
            // Convert the lints into their `LintId`s.
            .map(BevyLint::id)
            // Collect into a `Vec`.
            .collect();

        store.register_group(true, group.name, None, lints);
    }
}
