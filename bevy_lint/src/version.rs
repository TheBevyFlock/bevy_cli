use cargo_metadata::semver::Version;
use rustc_lint::{Lint, LintStore};

use crate::{
    groups::GROUPS,
    lint::BevyLint,
    lints::{
        LINTS, borrowed_reborrowable, cargo, insert_event_resource, insert_unit_bundle,
        main_return_without_appexit, missing_reflect, panicking_methods,
        plugin_not_ending_in_plugin, zst_query,
    },
};

pub enum BevyVersion {
    V0_15(V0_15),
}
pub struct V0_15;

impl V0_15 {
    const APP: [&str; 3] = ["bevy_app", "app", "App"];
    const COMMANDS: [&str; 4] = ["bevy_ecs", "system", "commands", "Commands"];
    const COMPONENT: [&str; 3] = ["bevy_ecs", "component", "Component"];
    const DEFERRED: [&str; 4] = ["bevy_ecs", "system", "system_param", "Deferred"];
    const DEFERRED_WORLD: [&str; 4] = ["bevy_ecs", "world", "deferred_world", "DeferredWorld"];
    const ENTITY_COMMANDS: [&str; 4] = ["bevy_ecs", "system", "commands", "EntityCommands"];
    const ENTITY_MUT: [&str; 4] = ["bevy_ecs", "world", "entity_ref", "EntityMut"];
    const EVENT: [&str; 4] = ["bevy_ecs", "event", "base", "Event"];
    const EVENTS: [&str; 4] = ["bevy_ecs", "event", "collections", "Events"];
    const FILTERED_ENTITY_MUT: [&str; 4] = ["bevy_ecs", "world", "entity_ref", "FilteredEntityMut"];
    const MUT: [&str; 3] = ["bevy_ecs", "change_detection", "Mut"];
    const MUT_UNTYPED: [&str; 3] = ["bevy_ecs", "change_detection", "MutUntyped"];
    const NON_SEND_MUT: [&str; 3] = ["bevy_ecs", "change_detection", "NonSendMut"];
    const PLUGIN: [&str; 3] = ["bevy_app", "plugin", "Plugin"];
    const PTR_MUT: [&str; 2] = ["bevy_ptr", "PtrMut"];
    const QUERY: [&str; 4] = ["bevy_ecs", "system", "query", "Query"];
    const QUERY_STATE: [&str; 4] = ["bevy_ecs", "query", "state", "QueryState"];
    const REFLECT: [&str; 3] = ["bevy_reflect", "reflect", "Reflect"];
    const RES_MUT: [&str; 3] = ["bevy_ecs", "change_detection", "ResMut"];
    const RESOURCE: [&str; 4] = ["bevy_ecs", "system", "system_param", "Resource"];
    const WORLD: [&str; 3] = ["bevy_ecs", "world", "World"];

    pub fn register_lints(store: &mut LintStore) {
        let lints: Vec<&Lint> = LINTS.iter().map(|x| x.lint).collect();
        store.register_lints(&lints);
    }

    pub fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| {
            Box::new(borrowed_reborrowable::BorrowedReborrowable::default())
        });
        store.register_late_pass(|_| Box::new(cargo::Cargo::default()));
        store.register_late_pass(|_| {
            Box::new(insert_event_resource::InsertEventResource::default())
        });
        store.register_late_pass(|_| {
            Box::new(main_return_without_appexit::MainReturnWithoutAppExit::default())
        });
        store.register_late_pass(|_| Box::new(missing_reflect::MissingReflect::default()));
        store.register_late_pass(|_| Box::new(panicking_methods::PanickingMethods::default()));
        store.register_late_pass(|_| {
            Box::new(plugin_not_ending_in_plugin::PluginNotEndingInPlugin::default())
        });
        store.register_late_pass(|_| Box::new(zst_query::ZstQuery::default()));
        store.register_late_pass(|_| Box::new(insert_unit_bundle::InsertUnitBundle::default()));
    }

    /// Registers all [`LintGroup`]s in [`GROUPS`] with the [`LintStore`].
    pub(crate) fn register_groups(store: &mut LintStore) {
        for &group in GROUPS {
            let lints = LINTS
                .iter()
                .copied()
                // Only select lints of this specified group.
                .filter(|l| l.group == group)
                // Convert the lints into their `LintId`s.
                .map(BevyLint::id)
                // Collect into a `Vec`.
                .collect();

            store.register_group(true, group.name, None, lints);
        }
    }

    pub fn version() -> Version {
        todo!()
    }
}
