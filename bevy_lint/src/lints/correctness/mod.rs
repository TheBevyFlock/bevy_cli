//! Lints that check for outright incorrect code.
//!
//! Unlike [`suspicious`](super::suspicious) lints, these are carefully picked to be free of false
//! positives. You should avoid `#[allow(...)]`-ing these lints without a _very_ good reason.
//!
//! These lints are **deny** by default.

use rustc_lint::Level;

use crate::declare_group;

declare_group! {
    pub(crate) static CORRECTNESS = {
        name: "bevy::correctness",
        level: Level::Deny,
    };
}
