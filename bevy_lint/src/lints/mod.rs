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

/// Registers all lints, lint passes, and lint groups offered by `bevy_lint` into a given
/// [`LintStore`].
pub(crate) fn register(store: &mut LintStore) {
    complexity::Complexity::register(store);
    correctness::Correctness::register(store);
    nursery::Nursery::register(store);
    pedantic::Pedantic::register(store);
    performance::Performance::register(store);
    restriction::Restriction::register(store);
    style::Style::register(store);
    suspicious::Suspicious::register(store);

    // The Cargo lint pass is not associated with a single lint group, so we register it
    // separately.
    store.register_late_pass(|_| Box::new(cargo::Cargo));
}
