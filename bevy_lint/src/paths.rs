//! A collection of hardcoded types and functions that the linter uses.
//!
//! Since Bevy is a 3rd-party crate, we cannot easily add diagnostic items to it. In lieu of this,
//! we hardcode the paths to the items we need here, for easy referencing.
//!
//! Also see: [`match_type()`](clippy_utils::ty::match_type),
//! [`match_def_path()`](clippy_utils::match_def_path).

pub const APP: [&str; 3] = ["bevy_app", "app", "App"];
pub const EVENTS: [&str; 3] = ["bevy_ecs", "event", "Events"];
pub const PLUGIN: [&str; 3] = ["bevy_app", "plugin", "Plugin"];
pub const QUERY: [&str; 4] = ["bevy_ecs", "system", "query", "Query"];
pub const QUERY_STATE: [&str; 4] = ["bevy_ecs", "query", "state", "QueryState"];
pub const WORLD: [&str; 3] = ["bevy_ecs", "world", "World"];
