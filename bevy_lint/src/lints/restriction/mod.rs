//! Opt-in lints that restrict you from writing certain code.
//!
//! For more information, please see [`RESTRICTION`](crate::groups::RESTRICTION).

use rustc_lint::Level;

use crate::declare_group;

pub mod missing_reflect;
pub mod panicking_methods;

declare_group! {
    /// A group of opt-in lints that restrict you from writing certain code.
    ///
    /// These are designed for scenarios where you want to increase the consistency of your code-base
    /// and reject certain patterns. They should not all be enabled at once, but instead specific lints
    /// should be individually enabled.
    pub static RESTRICTION = {
        name: "bevy::restriction",
        level: Level::Allow,
    };
}
