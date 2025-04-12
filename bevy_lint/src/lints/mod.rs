//! All lints offered by `bevy_lint`, organized by lint group.
//!
//! Each module contains the lints for that lint group. [`suspicious`], for example, contains the
//! documentation for [`suspicious::insert_event_resource`] and
//! [`suspicious::iter_current_update_events`], since they are both within the `bevy::suspicious`
//! lint group.
//!
//! Just like lints, [lint groups that can be toggled together]. The following lint groups are
//! enabled by default:
//!
//! - [`correctness`]
//! - [`suspicious`]
//! - [`complexity`]
//! - [`performance`]
//! - [`style`]
//!
//! The following groups are disabled by default:
//!
//! - [`pedantic`]
//! - [`restriction`]
//! - [`nursery`]
//!
//! [lint groups that can be toggled together]: crate#toggling-lints-in-cargotoml

use crate::lint::LintGroup;
use rustc_lint::LintStore;

mod cargo;

pub mod complexity;
pub mod correctness;
pub mod nursery;
pub mod pedantic;
pub mod performance;
pub mod restriction;
pub mod style;
pub mod suspicious;

/// Registers all [`BevyLint`]s in [`LINTS`] with the [`LintStore`].
pub(crate) fn register_lints(store: &mut LintStore) {
    complexity::Complexity::register_lints(store);
    correctness::Correctness::register_lints(store);
    nursery::Nursery::register_lints(store);
    pedantic::Pedantic::register_lints(store);
    performance::Performance::register_lints(store);
    restriction::Restriction::register_lints(store);
    style::Style::register_lints(store);
    suspicious::Suspicious::register_lints(store);
}

/// Registers all lint passes with the [`LintStore`].
pub(crate) fn register_passes(store: &mut LintStore) {
    complexity::Complexity::register_passes(store);
    correctness::Correctness::register_passes(store);
    nursery::Nursery::register_passes(store);
    pedantic::Pedantic::register_passes(store);
    performance::Performance::register_passes(store);
    restriction::Restriction::register_passes(store);
    style::Style::register_passes(store);
    suspicious::Suspicious::register_passes(store);

    // The Cargo lint pass is not associated with a single lint group, so we register it
    // separately.
    store.register_late_pass(|_| Box::new(cargo::Cargo::default()));
}

/// Registers all [`LintGroup`]s in [`GROUPS`] with the [`LintStore`].
pub(crate) fn register_groups(store: &mut LintStore) {
    complexity::Complexity::register_group(store);
    correctness::Correctness::register_group(store);
    nursery::Nursery::register_group(store);
    pedantic::Pedantic::register_group(store);
    performance::Performance::register_group(store);
    restriction::Restriction::register_group(store);
    style::Style::register_group(store);
    suspicious::Suspicious::register_group(store);
}
