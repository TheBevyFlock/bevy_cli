//! Lints that make the linter a nit-picky perfectionist.
//!
//! Unlike other lint groups, pedantic lints have limited value and may not always apply. Be sure
//! to read the motivation behind each lint in this category before enabling them, and expect to
//! liberally apply `#[allow(...)]` attributes throughout your code if you do use them.
//!
//! These lints are **allow** by default.

use rustc_lint::Level;

use crate::declare_group;

pub mod borrowed_reborrowable;
pub mod main_return_without_appexit;

declare_group! {
    pub(crate) static PEDANTIC = {
        name: "bevy::pedantic",
        level: Level::Allow,
    };
}
