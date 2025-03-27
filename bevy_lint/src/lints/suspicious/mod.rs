//! Lints similar to [`correctness`](super::correctness) that checks for suspicious or usually
//! wrong code.
//!
//! For more information, please see [`SUSPICIOUS`](crate::groups::SUSPICIOUS).

use rustc_lint::Level;

use crate::declare_group;

pub mod insert_event_resource;
pub mod insert_unit_bundle;
pub mod iter_current_update_events;

declare_group! {
    /// A group similar to [`CORRECTNESS`] that checks for suspicious or usually wrong code.
    ///
    /// The linted code may have been written intentionally, but should probably still be fixed.
    pub static SUSPICIOUS = {
        name: "bevy::suspicious",
        level: Level::Warn,
    };
}
