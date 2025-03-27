//! Lints that suggest how to increase the performance of your code.
//!
//! For more information, please see [`PERFORMANCE`](crate::groups::PERFORMANCE).

use rustc_lint::Level;

use crate::declare_group;

declare_group! {
    /// A group that suggests how to increase the performance of your code.
    pub static PERFORMANCE = {
        name: "bevy::performance",
        level: Level::Warn,
    };
}
