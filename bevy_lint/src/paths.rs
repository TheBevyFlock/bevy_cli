//! A collection of hardcoded type and function paths that the linter uses.
//!
//! Since Bevy is a 3rd-party crate, we cannot easily add diagnostic items to it. In lieu of this,
//! we hardcode the paths to the items we need here, for easy referencing.

use clippy_utils::paths::{PathLookup, PathNS};

use crate::sym;

/// Returns a new [`PathLookup`] in the [type namespace](PathNS::Type) for a given path.
///
/// `type_path!()` takes a `::`-separated list of identifiers. Each identifier should correspond to
/// a [`Symbol`](rustc_span::Symbol) in [`crate::sym`]. For example,
/// `type_path!(bevy_app::app::App)` creates a [`PathLookup`] in the [`PathNS::Type`] namespace
/// with the path `[sym::bevy_app, sym::app, sym::App]`.
macro_rules! type_path {
    ($first:ident $(:: $remaining:ident)*) => {
        PathLookup::new(PathNS::Type, &[sym::$first, $(sym::$remaining),*])
    };
}

// Keep the following list alphabetically sorted :)

/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_app/src/app.rs#L78>
pub static APP: PathLookup = type_path!(bevy_app::app::App);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/bundle.rs#L151>
pub static BUNDLE: PathLookup = type_path!(bevy_ecs::bundle::Bundle);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_render/src/camera/camera.rs#L346>
pub static CAMERA: PathLookup = type_path!(bevy_render::camera::camera::Camera);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/system/commands/mod.rs#L101>
pub static COMMANDS: PathLookup = type_path!(bevy_ecs::system::commands::Commands);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/component.rs#L485>
pub static COMPONENT: PathLookup = type_path!(bevy_ecs::component::Component);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/world/deferred_world.rs#L23>
pub static DEFERRED_WORLD: PathLookup = type_path!(bevy_ecs::world::deferred_world::DeferredWorld);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/system/system_param.rs#L1415>
pub static DEFERRED: PathLookup = type_path!(bevy_ecs::system::system_param::Deferred);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/system/commands/mod.rs#L1233>
pub static ENTITY_COMMANDS: PathLookup = type_path!(bevy_ecs::system::commands::EntityCommands);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/world/entity_ref.rs#L448>
pub static ENTITY_MUT: PathLookup = type_path!(bevy_ecs::world::entity_ref::EntityMut);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/event/base.rs#L48>
pub static EVENT: PathLookup = type_path!(bevy_ecs::event::base::Event);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/event/collections.rs#L94>
pub static EVENTS: PathLookup = type_path!(bevy_ecs::event::collections::Events);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/world/entity_ref.rs#L3687>
pub static FILTERED_ENTITY_MUT: PathLookup =
    type_path!(bevy_ecs::world::entity_ref::FilteredEntityMut);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_app/src/main_schedule.rs#L132>
pub static FIXED_UPDATE: PathLookup = type_path!(bevy_app::main_schedule::FixedUpdate);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/change_detection.rs#L1059>
pub static MUT_UNTYPED: PathLookup = type_path!(bevy_ecs::change_detection::MutUntyped);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/change_detection.rs#L959>
pub static MUT: PathLookup = type_path!(bevy_ecs::change_detection::Mut);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/change_detection.rs#L773>
pub static NON_SEND_MUT: PathLookup = type_path!(bevy_ecs::change_detection::NonSendMut);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_reflect/src/reflect.rs#L84>
pub static PARTIAL_REFLECT: PathLookup = type_path!(bevy_reflect::reflect::PartialReflect);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_app/src/plugin.rs#L57>
pub static PLUGIN: PathLookup = type_path!(bevy_app::plugin::Plugin);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ptr/src/lib.rs#L178>
pub static PTR_MUT: PathLookup = type_path!(bevy_ptr::PtrMut);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/system/query.rs#L482>
pub static QUERY: PathLookup = type_path!(bevy_ecs::system::query::Query);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_reflect/src/reflect.rs#L413>
pub static REFLECT: PathLookup = type_path!(bevy_reflect::reflect::Reflect);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/change_detection.rs#L714>
pub static RES_MUT: PathLookup = type_path!(bevy_ecs::change_detection::ResMut);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/resource.rs#L75>
pub static RESOURCE: PathLookup = type_path!(bevy_ecs::resource::Resource);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/schedule/set.rs#L35>
pub static SYSTEM_SET: PathLookup = type_path!(bevy_ecs::schedule::set::SystemSet);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/query/filter.rs#L137>
pub static WITH: PathLookup = type_path!(bevy_ecs::query::filter::With);
/// <https://github.com/bevyengine/bevy/blob/v0.16.0/crates/bevy_ecs/src/world/mod.rs#L94>
pub static WORLD: PathLookup = type_path!(bevy_ecs::world::World);
