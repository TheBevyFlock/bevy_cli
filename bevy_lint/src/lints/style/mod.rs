//! Lints that encourage idiomatic code.
//!
//! These lints are opinionated and may be freely disabled if you disagree with their suggestions.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::declare_group;

pub mod plugin_not_ending_in_plugin;

declare_group! {
    pub(crate) static STYLE = {
        name: "bevy::style",
        level: Level::Warn,
    };
}
