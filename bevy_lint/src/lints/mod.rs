use crate::lint::BevyLint;
use rustc_lint::{Lint, LintStore};

pub mod insert_event_resource;
pub mod main_return_without_appexit;
pub mod panicking_methods;

pub(crate) static LINTS: &[&BevyLint] = &[
    insert_event_resource::INSERT_EVENT_RESOURCE,
    main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT,
    panicking_methods::PANICKING_QUERY_METHODS,
    panicking_methods::PANICKING_WORLD_METHODS,
];

pub(crate) fn register_lints(store: &mut LintStore) {
    let lints: Vec<&Lint> = LINTS.iter().map(|x| x.lint).collect();
    store.register_lints(&lints);
}

pub(crate) fn register_passes(store: &mut LintStore) {
    store.register_late_pass(|_| Box::new(insert_event_resource::InsertEventResource));
    store.register_late_pass(|_| Box::new(main_return_without_appexit::MainReturnWithoutAppExit));
    store.register_late_pass(|_| Box::new(panicking_methods::PanickingMethods));
}
