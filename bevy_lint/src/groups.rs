//! Lint groups that can be toggled together.
//!
//! Each [lint](crate::lints) is organized within a specific category, such as [`PERFORMANCE`] or
//! [`STYLE`]. The following groups are enabled by default:
//!
//! - [`CORRECTNESS`]
//! - [`SUSPICIOUS`]
//! - [`COMPLEXITY`]
//! - [`PERFORMANCE`]
//! - [`STYLE`]
//!
//! The following groups are disabled by default:
//!
//! - [`PEDANTIC`]
//! - [`RESTRICTION`]
//! - [`NURSERY`]

use crate::{
    lint::{BevyLint, LintGroup},
    lints::LINTS,
};
use rustc_lint::{Level, LintStore};

/// A macro for declaring [`LintGroup`]s that auto-generates a table with the name and default
/// level in the documentation.
macro_rules! declare_group {
    {
        $(#[$attr:meta])*
        $vis:vis static $static_name:ident = {
            name: $group_name:literal,
            level: $level:expr$(,)?
        };
    } => {
        $(#[$attr])*
        ///
        /// <table>
        ///     <tr>
        ///         <td>Name</td>
        #[doc = concat!("        <td><code>", stringify!($group_name), "</code></td>")]
        ///     </tr>
        ///     <tr>
        ///         <td>Default Level</td>
        #[doc = concat!("        <td><code>", stringify!($level), "</code></td>")]
        ///     </tr>
        /// </table>
        $vis static $static_name: &LintGroup = &LintGroup {
            name: $group_name,
            level: $level,
        };
    };
}

declare_group! {
    /// A group of deny-by-default lints that check for outright wrong or useless code.
    ///
    /// These lints are carefully picked to be free of false positives. You should avoid
    /// `#[allow(...)]`-ing these lints without a _very_ good reason.
    pub static CORRECTNESS = {
        name: "bevy::correctness",
        level: Level::Deny,
    };
}

declare_group! {
    /// A group similar to [`CORRECTNESS`] that checks for suspicious or usually wrong code.
    ///
    /// The linted code may have been written intentionally, but should probably still be fixed.
    pub static SUSPICIOUS = {
        name: "bevy::suspicious",
        level: Level::Warn,
    };
}

declare_group! {
    /// A group that offers suggestions on how to simplify your code.
    pub static COMPLEXITY = {
        name: "bevy::complexity",
        level: Level::Warn,
    };
}

declare_group! {
    /// A group that suggests how to increase the performance of your code.
    pub static PERFORMANCE = {
        name: "bevy::performance",
        level: Level::Warn,
    };
}

declare_group! {
    /// A group of lints that encourage idiomatic code.
    ///
    /// These lints are opinionated and may be freely disabled if you disagree with their suggestions.
    pub static STYLE = {
        name: "bevy::style",
        level: Level::Warn,
    };
}

declare_group! {
    /// A group of lints that make the linter incredibly nit-picky.
    ///
    /// If you enable this group, expect to liberally apply `#[allow(...)]` attributes throughout your
    /// code.
    pub static PEDANTIC = {
        name: "bevy::pedantic",
        level: Level::Allow,
    };
}

declare_group! {
    /// A group of opt-in lints that restrict you from writing certain code.
    ///
    /// These are designed for scenarios where you want to increase the consistency of your code-base
    /// and reject certain patterns. They should not all be enabled at once, but instead specific lints
    /// should be individually enabled.
    pub static RESTRICTION = {
        name: "bevy::restriction",
        level: Level::Allow,
    };
}

declare_group! {
    /// A group of unstable lints that may be removed at any time for any reason.
    pub static NURSERY = {
        name: "bevy::nursery",
        level: Level::Allow,
    };
}

/// A list of all [`LintGroup`]s.
///
/// If a group is not in this list, it will not be registered in [`register_groups()`].
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

/// Registers all [`LintGroup`]s in [`GROUPS`] with the [`LintStore`].
pub(crate) fn register_groups(store: &mut LintStore) {
    for &group in GROUPS {
        let lints = LINTS
            .iter()
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
