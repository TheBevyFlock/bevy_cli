//! All lints offered by `bevy_lint`.
//!
//! Click on each module to learn more about individual lints.

use crate::lint::BevyLint;
use rustc_lint::{Lint, LintStore};

pub mod borrow_of_reborrowable;
pub mod insert_event_resource;
pub mod main_return_without_appexit;
pub mod missing_reflect;
pub mod panicking_methods;
pub mod plugin_not_ending_in_plugin;

pub(crate) static LINTS: &[&BevyLint] = &[
    borrow_of_reborrowable::BORROW_OF_COMMANDS,
    borrow_of_reborrowable::BORROW_OF_QUERY,
    insert_event_resource::INSERT_EVENT_RESOURCE,
    main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT,
    panicking_methods::PANICKING_QUERY_METHODS,
    missing_reflect::MISSING_REFLECT,
    panicking_methods::PANICKING_WORLD_METHODS,
    plugin_not_ending_in_plugin::PLUGIN_NOT_ENDING_IN_PLUGIN,
];

pub(crate) fn register_lints(store: &mut LintStore) {
    let lints: Vec<&Lint> = LINTS.iter().map(|x| x.lint).collect();
    store.register_lints(&lints);
}

pub(crate) fn register_passes(store: &mut LintStore) {
    store.register_late_pass(|_| Box::new(borrow_of_reborrowable::BorrowOfReborrowable));
    store.register_late_pass(|_| Box::new(insert_event_resource::InsertEventResource));
    store.register_late_pass(|_| Box::new(main_return_without_appexit::MainReturnWithoutAppExit));
    store.register_late_pass(|_| Box::new(missing_reflect::MissingReflect));
    store.register_late_pass(|_| Box::new(panicking_methods::PanickingMethods));
    store.register_late_pass(|_| Box::new(plugin_not_ending_in_plugin::PluginNotEndingInPlugin));
}
