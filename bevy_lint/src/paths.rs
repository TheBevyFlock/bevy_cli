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

/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_app/src/app.rs#L84>
pub static APP: PathLookup = type_path!(bevy_app::app::App);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/bundle/mod.rs#L200>
pub static BUNDLE: PathLookup = type_path!(bevy_ecs::bundle::Bundle);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_camera/src/camera.rs#L349>
pub static CAMERA: PathLookup = type_path!(bevy_camera::camera::Camera);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/system/commands/mod.rs#L105>
pub static COMMANDS: PathLookup = type_path!(bevy_ecs::system::commands::Commands);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/component/mod.rs#L509>
pub static COMPONENT: PathLookup = type_path!(bevy_ecs::component::Component);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/world/deferred_world.rs#L28>
pub static DEFERRED_WORLD: PathLookup = type_path!(bevy_ecs::world::deferred_world::DeferredWorld);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/system/system_param.rs#L1301>
pub static DEFERRED: PathLookup = type_path!(bevy_ecs::system::system_param::Deferred);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/system/commands/mod.rs#L1282>
pub static ENTITY_COMMANDS: PathLookup = type_path!(bevy_ecs::system::EntityCommands);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/world/entity_access/entity_mut.rs#L42>
pub static ENTITY_MUT: PathLookup = type_path!(bevy_ecs::world::entity_access::EntityMut);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/event/mod.rs#L88>
pub static EVENT: PathLookup = type_path!(bevy_ecs::event::Event);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/world/entity_access/filtered.rs#L358>
pub static FILTERED_ENTITY_MUT: PathLookup =
    type_path!(bevy_ecs::world::entity_access::FilteredEntityMut);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_app/src/main_schedule.rs#L133>
pub static FIXED_UPDATE: PathLookup = type_path!(bevy_app::main_schedule::FixedUpdate);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/message/mod.rs#L96>
pub static MESSAGE: PathLookup = type_path!(bevy_ecs::message::Message);
/// <https://githu.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/message/messages.rs#L95>
pub static MESSAGES: PathLookup = type_path!(bevy_ecs::message::Messages);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/change_detection/params.rs#L513>
pub static MUT_UNTYPED: PathLookup = type_path!(bevy_ecs::change_detection::params::MutUntyped);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/change_detection/params.rs#L415>
pub static MUT: PathLookup = type_path!(bevy_ecs::change_detection::params::Mut);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/change_detection/params.rs#L257>
pub static NON_SEND_MUT: PathLookup = type_path!(bevy_ecs::change_detection::params::NonSendMut);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_reflect/src/reflect.rs#L99>
pub static PARTIAL_REFLECT: PathLookup = type_path!(bevy_reflect::reflect::PartialReflect);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_app/src/plugin.rs#L57>
pub static PLUGIN: PathLookup = type_path!(bevy_app::plugin::Plugin);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ptr/src/lib.rs#L307>
pub static PTR_MUT: PathLookup = type_path!(bevy_ptr::PtrMut);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/system/query.rs#L485>
pub static QUERY: PathLookup = type_path!(bevy_ecs::system::query::Query);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_reflect/src/reflect.rs#L415>
pub static REFLECT: PathLookup = type_path!(bevy_reflect::reflect::Reflect);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/relationship/related_methods.rs#L565>
pub static RELATED_SPAWNER: PathLookup =
    type_path!(bevy_ecs::relationship::related_methods::RelatedSpawner);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/relationship/related_methods.rs#L611>
pub static RELATED_SPAWNER_COMMANDS: PathLookup =
    type_path!(bevy_ecs::relationship::related_methods::RelatedSpawnerCommands);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/change_detection/params.rs#L172>
pub static RES_MUT: PathLookup = type_path!(bevy_ecs::change_detection::params::ResMut);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/resource.rs#L75>
pub static RESOURCE: PathLookup = type_path!(bevy_ecs::resource::Resource);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/schedule/set.rs#L154>
pub static SYSTEM_SET: PathLookup = type_path!(bevy_ecs::schedule::set::SystemSet);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_app/src/main_schedule.rs#L173>
pub static UPDATE: PathLookup = type_path!(bevy_app::main_schedule::Update);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/query/filter.rs#L142>
pub static WITH: PathLookup = type_path!(bevy_ecs::query::filter::With);
/// <https://github.com/bevyengine/bevy/blob/v0.18.0/crates/bevy_ecs/src/world/mod.rs#L92>
pub static WORLD: PathLookup = type_path!(bevy_ecs::world::World);
