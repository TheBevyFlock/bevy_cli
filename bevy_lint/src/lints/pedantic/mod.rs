//! Lints that make the linter a nit-picky perfectionist.
//!
//! Unlike other lint groups, pedantic lints have limited value and may not always apply. Be sure
//! to read the motivation behind each lint in this category before enabling them, and expect to
//! liberally apply `#[allow(...)]` attributes throughout your code if you do use them.
//!
//! These lints are **allow** by default.

use rustc_lint::{Level, Lint};

use crate::lint::{LintGroup, LintGroup2};

pub mod borrowed_reborrowable;
pub mod main_return_without_appexit;

pub(crate) struct Pedantic;

impl LintGroup2 for Pedantic {
    const NAME: &str = "bevy::pedantic";
    const LEVEL: Level = Level::Allow;
    const LINTS: &[&Lint] = &[
        borrowed_reborrowable::BORROWED_REBORROWABLE.lint,
        main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT.lint,
    ];
}

pub(crate) static PEDANTIC: &LintGroup = &LintGroup {
    name: "bevy::pedantic",
    level: Level::Allow,
};
