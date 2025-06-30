//! Opt-in lints that restrict you from writing certain code patterns.
//!
//! These are designed for scenarios where you want to increase the consistency of your project by
//! rejecting certain patterns. These lints should not all be enabled as a group, but instead
//! should be chosen individually after reading the documentation.
//!
//! These lints are **allow** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod missing_reflect;
pub mod panicking_methods;

pub(crate) struct Restriction;

impl LintGroup for Restriction {
    const NAME: &str = "bevy::restriction";
    const LEVEL: Level = Level::Allow;
    const LINTS: &[&Lint] = &[
        missing_reflect::MISSING_REFLECT,
        panicking_methods::PANICKING_METHODS,
    ];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| Box::new(missing_reflect::MissingReflect));
        store.register_late_pass(|_| Box::new(panicking_methods::PanickingMethods));
    }
}
