use rustc_lint::{Lint, LintId, LintStore};

pub mod insert_event_resource;
pub mod main_return_without_appexit;

pub static CORRECTNESS: &[&Lint] = &[insert_event_resource::INSERT_EVENT_RESOURCE];
pub static SUSPICIOUS: &[&Lint] = &[];
pub static COMPLEXITY: &[&Lint] = &[];
pub static PERFORMANCE: &[&Lint] = &[];
pub static STYLE: &[&Lint] = &[main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT];
pub static RESTRICTION: &[&Lint] = &[];
pub static NURSERY: &[&Lint] = &[];

pub(crate) fn register_lints(store: &mut LintStore) {
    store.register_lints(CORRECTNESS);
    store.register_lints(SUSPICIOUS);
    store.register_lints(COMPLEXITY);
    store.register_lints(PERFORMANCE);
    store.register_lints(STYLE);
    store.register_lints(RESTRICTION);
    store.register_lints(NURSERY);

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
    register_group(store, "bevy::restriction", RESTRICTION);
    register_group(store, "bevy::nursery", NURSERY);
}

pub(crate) fn register_passes(store: &mut LintStore) {
    store.register_late_pass(|_| Box::new(insert_event_resource::InsertEventResource));
    store.register_late_pass(|_| Box::new(main_return_without_appexit::MainReturnWithoutAppExit));
}
