use rustc_driver::Callbacks;
use rustc_interface::interface::Config;

pub struct BevyLintCallback;

impl Callbacks for BevyLintCallback {
    fn config(&mut self, config: &mut Config) {
        // We're overwriting `register_lints`, but we don't want to completely delete the original
        // function. Instead, we save it so we can call it ourselves inside its replacement.
        let previous = config.register_lints.take();

        config.register_lints = Some(Box::new(move |session, store| {
            // If there was a previous `register_lints`, call it first.
            if let Some(previous) = &previous {
                (previous)(session, store);
            }

            store.register_lints(crate::lints::LINTS);
            crate::lints::register_passes(store);
        }));
    }
}
