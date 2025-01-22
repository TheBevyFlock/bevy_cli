use std::sync::atomic::{AtomicPtr, Ordering};

use rustc_driver::Callbacks;
use rustc_interface::interface::Config;
use rustc_lint_defs::RegisteredTools;
use rustc_middle::ty::TyCtxt;
use rustc_span::Ident;

/// A pointer to the original [`registered_tools()`](TyCtxt::registered_tools) query function.
///
/// # Safety
///
/// This pointer must be of the type [`fn(TyCtxt<'tcx>, ()) -> RegisteredTools`](fn).
static ORIGINAL_REGISTERED_TOOLS: AtomicPtr<()> = {
    fn default(_: TyCtxt<'_>, _: ()) -> RegisteredTools {
        unreachable!("This function will be overwritten when `BevyLintCallback::config()` is run.");
    }

    AtomicPtr::new(default as *mut ())
};

/// The `rustc` [`Callbacks`] that register Bevy's lints.
pub struct BevyLintCallback;

impl Callbacks for BevyLintCallback {
    fn config(&mut self, config: &mut Config) {
        crate::config::load_config(config);

        // We're overwriting `register_lints`, but we don't want to completely delete the original
        // function. Instead, we save it so we can call it ourselves inside its replacement.
        let previous = config.register_lints.take();

        config.register_lints = Some(Box::new(move |session, store| {
            // If there was a previous `register_lints`, call it first.
            if let Some(previous) = &previous {
                (previous)(session, store);
            }

            crate::lints::register_lints(store);
            crate::lints::register_passes(store);
            crate::groups::register_groups(store);
        }));

        config.override_queries = Some(|_session, providers| {
            // Save the original query so we can access it later.
            ORIGINAL_REGISTERED_TOOLS.store(
                providers.queries.registered_tools as *mut (),
                Ordering::Relaxed,
            );

            // Overwrite the query with our own custom version.
            providers.queries.registered_tools = registered_tools;
        });
    }
}

/// A custom version of the [`registered_tools()`](TyCtxt::registered_tools) query that
/// automatically adds "bevy" as a tool.
fn registered_tools<'tcx>(tcx: TyCtxt<'tcx>, _: ()) -> RegisteredTools {
    // Fetch the original version of the query.
    //
    // SAFETY: The pointer is guaranteed to be a `fn(TyCtxt<'tcx>, ()) -> RegisteredTools` as per
    // `ORIGINAL_REGISTERED_TOOLS`'s safety docs.
    let original: fn(TyCtxt<'tcx>, ()) -> RegisteredTools =
        unsafe { std::mem::transmute(ORIGINAL_REGISTERED_TOOLS.load(Ordering::Relaxed)) };

    let mut tools = (original)(tcx, ());

    tools.insert(Ident::from_str("bevy"));

    tools
}
