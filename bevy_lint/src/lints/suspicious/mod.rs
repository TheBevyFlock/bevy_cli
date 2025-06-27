//! Lints that checks for suspicious, potentially incorrect code.
//!
//! Unlike [`correctness`](super::correctness) lints, these may have false positives that you need
//! to `#[allow(...)]`.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub(crate) mod insert_event_resource;
pub(crate) mod insert_unit_bundle;
pub(crate) mod iter_current_update_events;

pub use self::{
    insert_event_resource::INSERT_EVENT_RESOURCE, insert_unit_bundle::INSERT_UNIT_BUNDLE,
    iter_current_update_events::ITER_CURRENT_UPDATE_EVENTS,
};

pub(crate) struct Suspicious;

impl LintGroup for Suspicious {
    const NAME: &str = "bevy::suspicious";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[
        insert_event_resource::INSERT_EVENT_RESOURCE,
        insert_unit_bundle::INSERT_UNIT_BUNDLE,
        iter_current_update_events::ITER_CURRENT_UPDATE_EVENTS,
    ];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| {
            Box::new(insert_event_resource::InsertEventResource::default())
        });
        store.register_late_pass(|_| Box::new(insert_unit_bundle::InsertUnitBundle::default()));
        store.register_late_pass(|_| {
            Box::new(iter_current_update_events::IterCurrentUpdateEvents::default())
        });
    }
}
