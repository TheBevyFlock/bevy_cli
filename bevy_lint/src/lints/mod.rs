use rustc_lint::{Lint, LintStore};

pub mod insert_event_resource;
pub mod main_return_without_appexit;
pub mod panicking_query_methods;

pub static LINTS: &[&Lint] = &[
    insert_event_resource::INSERT_EVENT_RESOURCE,
    main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT,
    panicking_query_methods::PANICKING_QUERY_METHODS,
];

pub(crate) fn register_passes(store: &mut LintStore) {
    store.register_late_pass(|_| Box::new(insert_event_resource::InsertEventResource));
    store.register_late_pass(|_| Box::new(main_return_without_appexit::MainReturnWithoutAppExit));
    store.register_late_pass(|_| Box::new(panicking_query_methods::PanickingQueryMethods));
}
