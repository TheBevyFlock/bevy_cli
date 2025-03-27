//! Lints that offer suggestions on how to simplify your code.
//!
//! For more information, please see [`COMPLEXITY`](crate::groups::COMPLEXITY).

use rustc_lint::Level;

use crate::declare_group;

declare_group! {
    /// A group that offers suggestions on how to simplify your code.
    pub static COMPLEXITY = {
        name: "bevy::complexity",
        level: Level::Warn,
    };
}
