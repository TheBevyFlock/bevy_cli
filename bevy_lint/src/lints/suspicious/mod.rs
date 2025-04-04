//! Lints that checks for suspicious, potentially incorrect code.
//!
//! Unlike [`correctness`](super::correctness) lints, these may have false positives that you need
//! to `#[allow(...)]`.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::lint::LintGroup;

pub mod insert_event_resource;
pub mod insert_unit_bundle;
pub mod iter_current_update_events;

pub(crate) static SUSPICIOUS: &LintGroup = &LintGroup {
    name: "bevy::suspicious",
    level: Level::Warn,
};
