use rustc_lint::{Lint, LintStore};

pub mod init_event_resource;
pub mod main_return_without_appexit;

pub static LINTS: &[&Lint] = &[
    init_event_resource::INIT_EVENT_RESOURCE,
    main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT,
];

pub(crate) fn register_passes(store: &mut LintStore) {
    store.register_late_pass(|_| Box::new(init_event_resource::InitEventResource));
    store.register_late_pass(|_| Box::new(main_return_without_appexit::MainReturnWithoutAppExit));
}
