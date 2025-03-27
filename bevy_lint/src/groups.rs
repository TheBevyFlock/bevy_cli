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
use rustc_lint::LintStore;

/// A macro for declaring [`LintGroup`]s that auto-generates a table with the name and default
/// level in the documentation.
#[macro_export]
#[doc(hidden)]
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
        $vis static $static_name: &$crate::lint::LintGroup = &$crate::lint::LintGroup {
            name: $group_name,
            level: $level,
        };
    };
}

/// A list of all [`LintGroup`]s.
///
/// If a group is not in this list, it will not be registered in [`register_groups()`].
static GROUPS: &[&LintGroup] = &[
    crate::lints::correctness::CORRECTNESS,
    crate::lints::suspicious::SUSPICIOUS,
    crate::lints::complexity::COMPLEXITY,
    crate::lints::performance::PERFORMANCE,
    crate::lints::style::STYLE,
    crate::lints::pedantic::PEDANTIC,
    crate::lints::restriction::RESTRICTION,
    crate::lints::nursery::NURSERY,
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
