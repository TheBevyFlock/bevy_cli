//! The [`BevyLintCallback`] definition and supporting code.

use std::sync::atomic::{AtomicPtr, Ordering};

use rustc_driver::Callbacks;
use rustc_interface::interface::Config;
use rustc_lint_defs::RegisteredTools;
use rustc_middle::ty::TyCtxt;
use rustc_session::utils::was_invoked_from_cargo;
use rustc_span::{Ident, Symbol};

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
#[doc(hidden)]
pub struct BevyLintCallback;

impl Callbacks for BevyLintCallback {
    fn config(&mut self, config: &mut Config) {
        crate::config::load_config(config);

        // Add `--cfg bevy_lint` so programs can conditionally configure lints.
        config.crate_cfg.push("bevy_lint".to_string());

        // We should be the only callback, meaning nothing else should register custom lints.
        debug_assert!(config.register_lints.is_none());

        config.register_lints = Some(Box::new(|_session, store| {
            crate::lints::register(store);
        }));

        // We should be the only callback, meaning nothing else should override the queries.
        debug_assert!(config.override_queries.is_none());

        config.override_queries = Some(|_session, providers| {
            // Save the original query so we can access it later.
            ORIGINAL_REGISTERED_TOOLS.store(
                providers.queries.registered_tools as *mut (),
                Ordering::Relaxed,
            );

            // Overwrite the query with our own custom version.
            providers.queries.registered_tools = registered_tools;
        });

        // Clone the input so we can `move` it into our custom `psess_created()`.
        let input = config.input.clone();

        config.psess_created = Some(Box::new(move |psess| {
            if !was_invoked_from_cargo() {
                return;
            }

            let file_depinfo = psess.file_depinfo.get_mut();

            for workspace in [false, true] {
                // Get the paths to the crate or workspace `Cargo.toml`, if they exist.
                let manifest_path = crate::utils::cargo::locate_manifest(&input, workspace);

                // If locating the manifest was successful, try to convert the path into a UTF-8
                // string that we can intern.
                if let Ok(path) = manifest_path
                    && let Some(path) = path.to_str()
                {
                    // Insert the manifest path into `file_depinfo`. Now if the manifest is
                    // changed, checks will re-run.
                    file_depinfo.insert(Symbol::intern(path));
                }
            }
        }));

        // There shouldn't be any existing extra symbols, as we should be the only callback
        // overriding them.
        debug_assert!(config.extra_symbols.is_empty());

        // Give the compiler a list of extra `Symbol`s to intern ahead of time. This helps us avoid
        // calling `Symbol::intern()` while linting. See the `sym` module for a more detailed
        // explanation.
        config.extra_symbols = crate::sym::extra_symbols();
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
