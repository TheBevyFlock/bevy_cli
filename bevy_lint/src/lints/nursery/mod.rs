//! Unstable lints that may be removed at any time for any reason.
//!
//! These lints are **allow** by default.

use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod camera_modification_in_fixed_update;
pub mod duplicate_bevy_dependencies;
pub mod zst_query;

pub(crate) struct Nursery;

impl LintGroup for Nursery {
    const NAME: &str = "bevy::nursery";
    const LEVEL: Level = Level::Allow;
    const LINTS: &[&Lint] = &[
        camera_modification_in_fixed_update::CAMERA_MODIFICATION_IN_FIXED_UPDATE,
        duplicate_bevy_dependencies::DUPLICATE_BEVY_DEPENDENCIES,
        zst_query::ZST_QUERY,
    ];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| {
            Box::new(camera_modification_in_fixed_update::CameraModificationInFixedUpdate)
        });
        // `duplicate_bevy_dependencies` is a Cargo lint, so it does not have its own pass.
        store.register_late_pass(|_| Box::new(zst_query::ZstQuery));
    }
}
