//! Lints that checks for suspicious, potentially incorrect code.
//!
//! Unlike [`correctness`](super::correctness) lints, these may have false positives that you need
//! to `#[allow(...)]`.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod insert_message_resource;
pub mod iter_current_update_messages;
pub mod unit_in_bundle;

pub(crate) struct Suspicious;

impl LintGroup for Suspicious {
    const NAME: &str = "bevy::suspicious";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[
        insert_message_resource::INSERT_MESSAGE_RESOURCE,
        iter_current_update_messages::ITER_CURRENT_UPDATE_MESSAGES,
        unit_in_bundle::UNIT_IN_BUNDLE,
    ];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| Box::new(insert_message_resource::InsertMessageResource));
        store.register_late_pass(|_| {
            Box::new(iter_current_update_messages::IterCurrentUpdateMessages)
        });
        store.register_late_pass(|_| Box::new(unit_in_bundle::UnitInBundle));

        // These help users migrate from v0.4.0 to v0.5.0. These lines should be removed before
        // v0.6.0 is released.
        store.register_renamed("bevy::insert_event_resource", "bevy::insert_message_resource");
        store.register_renamed("bevy::iter_current_update_events", "bevy::iter_current_update_messages");
    }

    fn register_lints(store: &mut LintStore) {
        store.register_lints(Self::LINTS);
    }
}
