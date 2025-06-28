//! A collection of hardcoded types and functions that the linter uses.
//!
//! Since Bevy is a 3rd-party crate, we cannot easily add diagnostic items to it. In lieu of this,
//! we hardcode the paths to the items we need here, for easy referencing.
//!
//! Also see: [`match_type()`](clippy_utils::ty::match_type),
//! [`match_def_path()`](clippy_utils::match_def_path).

/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_app/src/app.rs#L78>
pub const APP: [&str; 3] = ["bevy_app", "app", "App"];
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_render/src/camera/camera.rs#L346>
pub const CAMERA: [&str; 4] = ["bevy_render", "camera", "camera", "Camera"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/system/commands/command.rs#L48>
pub const COMMANDS: [&str; 4] = ["bevy_ecs", "system", "commands", "Commands"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/component.rs#L456>
pub const COMPONENT: [&str; 3] = ["bevy_ecs", "component", "Component"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/system/system_param.rs#L1378>
pub const DEFERRED: [&str; 4] = ["bevy_ecs", "system", "system_param", "Deferred"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/world/deferred_world.rs#L23>
pub const DEFERRED_WORLD: [&str; 4] = ["bevy_ecs", "world", "deferred_world", "DeferredWorld"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/system/commands/mod.rs#L1167>
pub const ENTITY_COMMANDS: [&str; 4] = ["bevy_ecs", "system", "commands", "EntityCommands"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/world/entity_ref.rs#L448>
pub const ENTITY_MUT: [&str; 4] = ["bevy_ecs", "world", "entity_ref", "EntityMut"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/event/base.rs#L48>
pub const EVENT: [&str; 4] = ["bevy_ecs", "event", "base", "Event"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/event/collections.rs#L94>
pub const EVENTS: [&str; 4] = ["bevy_ecs", "event", "collections", "Events"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/world/entity_ref.rs#L3687>
pub const FILTERED_ENTITY_MUT: [&str; 4] = ["bevy_ecs", "world", "entity_ref", "FilteredEntityMut"];
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_app/src/main_schedule.rs#L132>
pub const FIXED_UPDATE: [&str; 3] = ["bevy_app", "main_schedule", "FixedUpdate"];
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/query/filter.rs#L137>
pub const WITH: [&str; 4] = ["bevy_ecs", "query", "filter", "With"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/change_detection.rs#L920>
pub const MUT: [&str; 3] = ["bevy_ecs", "change_detection", "Mut"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/change_detection.rs#L1020>
pub const MUT_UNTYPED: [&str; 3] = ["bevy_ecs", "change_detection", "MutUntyped"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/change_detection.rs#L734>
pub const NON_SEND_MUT: [&str; 3] = ["bevy_ecs", "change_detection", "NonSendMut"];
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_reflect/src/reflect.rs#L84>
pub const PARTIAL_REFLECT: [&str; 3] = ["bevy_reflect", "reflect", "PartialReflect"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_app/src/plugin.rs#L57>
pub const PLUGIN: [&str; 3] = ["bevy_app", "plugin", "Plugin"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ptr/src/lib.rs#L178>
pub const PTR_MUT: [&str; 2] = ["bevy_ptr", "PtrMut"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/system/query.rs#L379>
pub const QUERY: [&str; 4] = ["bevy_ecs", "system", "query", "Query"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_reflect/src/reflect.rs#L413>
pub const REFLECT: [&str; 3] = ["bevy_reflect", "reflect", "Reflect"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/change_detection.rs#L675>
pub const RES_MUT: [&str; 3] = ["bevy_ecs", "change_detection", "ResMut"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/resource.rs#L75>
pub const RESOURCE: [&str; 3] = ["bevy_ecs", "resource", "Resource"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/schedule/set.rs>
pub const SYSTEM_SET: [&str; 4] = ["bevy_ecs", "schedule", "set", "SystemSet"];
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_app/src/main_schedule.rs#L172>
pub const UPDATE: [&str; 3] = ["bevy_app", "main_schedule", "Update"];
/// <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/world/mod.rs#L94>
pub const WORLD: [&str; 3] = ["bevy_ecs", "world", "World"];
