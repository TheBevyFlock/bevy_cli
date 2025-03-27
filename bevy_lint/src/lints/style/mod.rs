//! Lints that encourage idiomatic code.
//!
//! For more information, please see [`STYLE`](crate::groups::STYLE).

use rustc_lint::Level;

use crate::declare_group;

pub mod plugin_not_ending_in_plugin;

declare_group! {
    /// A group of lints that encourage idiomatic code.
    ///
    /// These lints are opinionated and may be freely disabled if you disagree with their suggestions.
    pub static STYLE = {
        name: "bevy::style",
        level: Level::Warn,
    };
}
