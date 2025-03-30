use rustc_data_structures::fx::FxHashMap;
use rustc_lint::{Lint, LintStore};

use crate::lints::{LINTS, cargo, nursery, pedantic, restriction, style, suspicious};

use super::Paths;

pub struct BevyV016 {
    paths: FxHashMap<&'static str, &'static [&'static str]>,
}

impl Default for BevyV016 {
    fn default() -> Self {
        let mut paths: FxHashMap<&'static str, &'static [&'static str]> = FxHashMap::default();

        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_app/src/app.rs#L78>
        paths.insert("APP", &["bevy_app", "app", "App"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/system/commands/command.rs#L48>
        paths.insert("COMMANDS", &["bevy_ecs", "system", "commands", "Commands"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/component.rs#L456>
        paths.insert("COMPONENT", &["bevy_ecs", "component", "Component"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/system/system_param.rs#L1378>
        paths.insert(
            "DEFERRED",
            &["bevy_ecs", "system", "system_param", "Deferred"],
        );
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/world/deferred_world.rs#L23>
        paths.insert(
            "DEFERRED_WORLD",
            &["bevy_ecs", "world", "deferred_world", "DeferredWorld"],
        );
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/system/commands/mod.rs#L1167>
        paths.insert(
            "ENTITY_COMMANDS",
            &["bevy_ecs", "system", "commands", "EntityCommands"],
        );
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/world/entity_ref.rs#L448>
        paths.insert(
            "ENTITY_MUT",
            &["bevy_ecs", "world", "entity_ref", "EntityMut"],
        );
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/event/base.rs#L48>
        paths.insert("EVENT", &["bevy_ecs", "event", "base", "Event"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/event/collections.rs#L94>
        paths.insert("EVENTS", &["bevy_ecs", "event", "collections", "Events"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/world/entity_ref.rs#L3687>
        paths.insert(
            "FILTERED_ENTITY_MUT",
            &["bevy_ecs", "world", "entity_ref", "FilteredEntityMut"],
        );
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/change_detection.rs#L920>
        paths.insert("MUT", &["bevy_ecs", "change_detection", "Mut"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/change_detection.rs#L1020>
        paths.insert(
            "MUT_UNTYPED",
            &["bevy_ecs", "change_detection", "MutUntyped"],
        );
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/change_detection.rs#L734>
        paths.insert(
            "NON_SEND_MUT",
            &["bevy_ecs", "change_detection", "NonSendMut"],
        );
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_app/src/plugin.rs#L57>
        paths.insert("PLUGIN", &["bevy_app", "plugin", "Plugin"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ptr/src/lib.rs#L178>
        paths.insert("PTR_MUT", &["bevy_ptr", "PtrMut"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/system/query.rs#L379>
        paths.insert("QUERY", &["bevy_ecs", "system", "query", "Query"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/query/state.rs#L66>
        paths.insert("QUERY_STATE", &["bevy_ecs", "query", "state", "QueryState"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_reflect/src/reflect.rs#L413>
        paths.insert("REFLECT", &["bevy_reflect", "reflect", "Reflect"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/change_detection.rs#L675>
        paths.insert("RES_MUT", &["bevy_ecs", "change_detection", "ResMut"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/resource.rs#L75>
        paths.insert("RESOURCE", &["bevy_ecs", "resource", "Resource"]);
        // <https://github.com/bevyengine/bevy/blob/release-0.16.0/crates/bevy_ecs/src/world/mod.rs#L94>
        paths.insert("WORLD", &["bevy_ecs", "world", "World"]);

        Self { paths }
    }
}

impl BevyV016 {
    pub fn register_lints(&self, store: &mut LintStore) {
        let lints: Vec<&Lint> = LINTS.iter().map(|x| x.lint).collect();
        store.register_lints(&lints);
    }

    pub fn register_passes(&self, store: &mut LintStore) {
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
        store.register_late_pass(|_| {
            Box::new(restriction::missing_reflect::MissingReflect::default())
        });
        store.register_late_pass(|_| {
            Box::new(restriction::panicking_methods::PanickingMethods::default())
        });
        store.register_late_pass(|_| {
            Box::new(style::plugin_not_ending_in_plugin::PluginNotEndingInPlugin::default())
        });
        store.register_late_pass(|_| Box::new(nursery::zst_query::ZstQuery::default()));
    }
}

impl Paths for BevyV016 {
    fn get(&self, key: &str) -> Option<&[&str]> {
        self.paths.get(key).copied()
    }
}
