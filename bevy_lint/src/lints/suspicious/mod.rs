//! Lints that checks for suspicious, potentially incorrect code.
//!
//! Unlike [`correctness`](super::correctness) lints, these may have false positives that you need
//! to `#[allow(...)]`.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::declare_group;

pub mod insert_event_resource;
pub mod insert_unit_bundle;
pub mod iter_current_update_events;

declare_group! {
    pub(crate) static SUSPICIOUS = {
        name: "bevy::suspicious",
        level: Level::Warn,
    };
}
