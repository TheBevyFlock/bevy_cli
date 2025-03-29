//! All lints offered by `bevy_lint`, organized by lint group.
//!
//! Each module contains the lints for that lint group. [`suspicious`], for example, contains the
//! documentation for [`suspicious::insert_event_resource`] and
//! [`suspicious::iter_current_update_events`], since they are both within the `bevy::suspicious`
//! lint group.
//!
//! Just like lints, lint groups that can be toggled together. The following lint groups are
//! enabled by default:
//!
//! - [`correctness`]
//! - [`suspicious`]
//! - [`complexity`]
//! - [`performance`]
//! - [`style`]
//!
//! The following groups are disabled by default:
//!
//! - [`pedantic`]
//! - [`restriction`]
//! - [`nursery`]

use crate::lint::BevyLint;
use rustc_lint::{Lint, LintStore};

mod cargo;

pub mod complexity;
pub mod correctness;
pub mod nursery;
pub mod pedantic;
pub mod performance;
pub mod restriction;
pub mod style;
pub mod suspicious;

pub(crate) static LINTS: &[&BevyLint] = &[
    pedantic::borrowed_reborrowable::BORROWED_REBORROWABLE,
    nursery::duplicate_bevy_dependencies::DUPLICATE_BEVY_DEPENDENCIES,
    suspicious::insert_event_resource::INSERT_EVENT_RESOURCE,
    suspicious::insert_unit_bundle::INSERT_UNIT_BUNDLE,
    suspicious::iter_current_update_events::ITER_CURRENT_UPDATE_EVENTS,
    pedantic::main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT,
    restriction::missing_reflect::MISSING_REFLECT,
    restriction::panicking_methods::PANICKING_METHODS,
    style::plugin_not_ending_in_plugin::PLUGIN_NOT_ENDING_IN_PLUGIN,
    nursery::zst_query::ZST_QUERY,
];

pub(crate) fn register_lints(store: &mut LintStore) {
    let lints: Vec<&Lint> = LINTS.iter().map(|x| x.lint).collect();
    store.register_lints(&lints);
}

pub(crate) fn register_passes(store: &mut LintStore) {
    store.register_late_pass(|_| {
        Box::new(pedantic::borrowed_reborrowable::BorrowedReborrowable::default())
    });
    store.register_late_pass(|_| Box::new(cargo::Cargo::default()));
    store.register_late_pass(|_| {
        Box::new(suspicious::insert_event_resource::InsertEventResource::default())
    });
    store.register_late_pass(|_| {
        Box::new(suspicious::insert_unit_bundle::InsertUnitBundle::default())
    });
    store.register_late_pass(|_| {
        Box::new(suspicious::iter_current_update_events::IterCurrentUpdateEvents::default())
    });
    store.register_late_pass(|_| {
        Box::new(pedantic::main_return_without_appexit::MainReturnWithoutAppExit::default())
    });
    store.register_late_pass(|_| Box::new(restriction::missing_reflect::MissingReflect::default()));
    store.register_late_pass(|_| {
        Box::new(restriction::panicking_methods::PanickingMethods::default())
    });
    store.register_late_pass(|_| {
        Box::new(style::plugin_not_ending_in_plugin::PluginNotEndingInPlugin::default())
    });
    store.register_late_pass(|_| Box::new(nursery::zst_query::ZstQuery::default()));
}
