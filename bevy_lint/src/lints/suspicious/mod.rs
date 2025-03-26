//! Lints similar to [`correctness`](super::correctness) that checks for suspicious or usually
//! wrong code.
//!
//! For more information, please see [`SUSPICIOUS`](crate::groups::SUSPICIOUS).

pub mod insert_event_resource;
pub mod insert_unit_bundle;
pub mod iter_current_update_events;
