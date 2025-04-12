//! Lints that checks for suspicious, potentially incorrect code.
//!
//! Unlike [`correctness`](super::correctness) lints, these may have false positives that you need
//! to `#[allow(...)]`.
//!
//! These lints are **warn** by default.

use rustc_lint::{Level, Lint};

use crate::lint::{LintGroup, LintGroup2};

pub mod insert_event_resource;
pub mod insert_unit_bundle;
pub mod iter_current_update_events;

pub(crate) struct Suspicious;

impl LintGroup2 for Suspicious {
    const NAME: &str = "bevy::suspicious";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[
        insert_event_resource::INSERT_EVENT_RESOURCE.lint,
        insert_unit_bundle::INSERT_UNIT_BUNDLE.lint,
        iter_current_update_events::ITER_CURRENT_UPDATE_EVENTS.lint,
    ];
}

pub(crate) static SUSPICIOUS: &LintGroup = &LintGroup {
    name: "bevy::suspicious",
    level: Level::Warn,
};
