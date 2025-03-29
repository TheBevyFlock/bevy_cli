//! Lints that suggest how to improve the performance of your code.
//!
//! These lints are **warn** by default.

use rustc_lint::Level;

use crate::declare_group;

declare_group! {
    pub(crate) static PERFORMANCE = {
        name: "bevy::performance",
        level: Level::Warn,
    };
}
