use crate::{
    lint::{BevyLint, LintGroup},
    lints::LINTS,
};
use rustc_lint::LintStore;

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
