//! Lints that make the linter incredibly nit-picky.
//!
//! For more information, please see [`PEDANTIC`](crate::groups::PEDANTIC).

use rustc_lint::Level;

use crate::declare_group;

pub mod borrowed_reborrowable;
pub mod main_return_without_appexit;

declare_group! {
    /// A group of lints that make the linter incredibly nit-picky.
    ///
    /// If you enable this group, expect to liberally apply `#[allow(...)]` attributes throughout your
    /// code.
    pub static PEDANTIC = {
        name: "bevy::pedantic",
        level: Level::Allow,
    };
}
