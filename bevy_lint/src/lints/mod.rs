use rustc_lint::{Lint, LintId, LintStore};

pub mod insert_event_resource;
pub mod main_return_without_appexit;

/// A group of deny-by-default lints that check for outright wrong or useless code.
///
/// These lints are carefully picked to be free of false-positives. You should avoid
/// `#[allow(...)]`-ing these lints without a _very_ good reason.
pub static CORRECTNESS: &[&Lint] = &[];

/// A group similar to [`CORRECTNESS`] that checks for suspicious or usually wrong code.
///
/// As compared to [`CORRECTNESS`], it may be possible that the linted code may be written
/// intentionally. Even still, you usually want to fix these lints instead of `#[allow(...)]`-ing
/// them.
pub static SUSPICIOUS: &[&Lint] = &[insert_event_resource::INSERT_EVENT_RESOURCE];

/// A group that offers suggestions on how to simplify your code.
pub static COMPLEXITY: &[&Lint] = &[];

/// A group that suggests how to increase the performance of your code.
pub static PERFORMANCE: &[&Lint] = &[];

/// A group of lints that encourage idiomatic code.
///
/// These lints are opinionated and may be freely disabled if you disagree with their suggestions.
pub static STYLE: &[&Lint] = &[];

/// A group of lints that make the linter incredibly nit-picky.
///
/// If you enable this group, expect to liberally apply `#[allow(...)]` attributes throughout your
/// code.
pub static PEDANTIC: &[&Lint] = &[main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT];

/// A group of opt-in lints that restrict you from writing certain code.
///
/// These are designed for scenarios where you want to increase the consistency of your code-base
/// and reject certain patterns. They should not all be enabled at once, but instead specific lints
/// should be individually enabled.
pub static RESTRICTION: &[&Lint] = &[];

pub(crate) fn register_lints(store: &mut LintStore) {
    store.register_lints(CORRECTNESS);
    store.register_lints(SUSPICIOUS);
    store.register_lints(COMPLEXITY);
    store.register_lints(PERFORMANCE);
    store.register_lints(STYLE);
    store.register_lints(PEDANTIC);
    store.register_lints(RESTRICTION);

    /// Shorthand for registering a group of lints.
    fn register_group(store: &mut LintStore, name: &'static str, lints: &'static [&'static Lint]) {
        store.register_group(
            true,
            name,
            None,
            lints.iter().copied().map(LintId::of).collect(),
        );
    }

    register_group(store, "bevy::correctness", CORRECTNESS);
    register_group(store, "bevy::suspicious", SUSPICIOUS);
    register_group(store, "bevy::complexity", COMPLEXITY);
    register_group(store, "bevy::performance", PERFORMANCE);
    register_group(store, "bevy::style", STYLE);
    register_group(store, "bevy::pedantic", PEDANTIC);
    register_group(store, "bevy::restriction", RESTRICTION);
}

pub(crate) fn register_passes(store: &mut LintStore) {
    store.register_late_pass(|_| Box::new(insert_event_resource::InsertEventResource));
    store.register_late_pass(|_| Box::new(main_return_without_appexit::MainReturnWithoutAppExit));
}
