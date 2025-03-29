//! Unstable lints that may be removed at any time for any reason.
//!
//! These lints are **allow** by default.

use rustc_lint::Level;

use crate::declare_group;

pub mod duplicate_bevy_dependencies;
pub mod zst_query;

declare_group! {
    pub(crate) static NURSERY = {
        name: "bevy::nursery",
        level: Level::Allow,
    };
}
