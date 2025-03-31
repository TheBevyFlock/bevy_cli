//! All lints offered by `bevy_lint`.
//!
//! Click on each module to learn more about individual lints. Within each module is a static that
//! documents a lint's name, group, and short description, such as
//! [`missing_reflect::MISSING_REFLECT`].

use crate::lint::BevyLint;

// TODO: how to handle this?
pub mod cargo;

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
