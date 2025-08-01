//! Lints that checks for suspicious, potentially incorrect code.
//!
//! Unlike [`correctness`](super::correctness) lints, these may have false positives that you need
//! to `#[allow(...)]`.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod insert_event_resource;
pub mod iter_current_update_events;
pub mod unit_in_bundle;

pub(crate) struct Suspicious;

impl LintGroup for Suspicious {
    const NAME: &str = "bevy::suspicious";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[
        insert_event_resource::INSERT_EVENT_RESOURCE,
        iter_current_update_events::ITER_CURRENT_UPDATE_EVENTS,
        unit_in_bundle::UNIT_IN_BUNDLE,
    ];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| Box::new(insert_event_resource::InsertEventResource));
        store.register_late_pass(|_| Box::new(iter_current_update_events::IterCurrentUpdateEvents));
        store.register_late_pass(|_| Box::new(unit_in_bundle::UnitInBundle));
    }

    fn register_lints(store: &mut LintStore) {
        store.register_lints(Self::LINTS);

        // This helps users migrate to v0.4.0, but should be removed before v0.5.0 is released.
        store.register_renamed("bevy::insert_unit_bundle", "bevy::unit_in_bundle");
    }
}
