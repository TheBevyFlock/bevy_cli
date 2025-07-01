#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unit_bundle)]
#![allow(dead_code)]

use bevy::{
    ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands},
    prelude::*,
};

#[derive(Component)]
struct ComponentA;

#[derive(Component)]
struct ComponentB;

pub fn test_commands(
    mut commands_owned: Commands,
    commands_borrowed: &mut Commands,
    mut commands_boxed: Box<Commands>,
) {
    commands_owned.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
    commands_borrowed.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
    commands_boxed.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
}

pub fn test_world(mut world_owned: World, world_borrowed: &mut World, mut world_boxed: Box<World>) {
    world_owned.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
    world_borrowed.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
    world_boxed.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
}

pub fn test_related_spawner(
    mut related_spawner_owned: RelatedSpawner<'_, ChildOf>,
    related_spawner_borrowed: &mut RelatedSpawner<'_, ChildOf>,
    mut related_spawner_boxed: Box<RelatedSpawner<'_, ChildOf>>,
) {
    related_spawner_owned.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
    related_spawner_borrowed.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
    related_spawner_boxed.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
}

pub fn test_related_spawner_commands(
    mut related_spawner_commands_owned: RelatedSpawnerCommands<'_, ChildOf>,
    related_spawner_commands_borrowed: &mut RelatedSpawnerCommands<'_, ChildOf>,
    mut related_spawner_commands_boxed: Box<RelatedSpawnerCommands<'_, ChildOf>>,
) {
    related_spawner_commands_owned.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
    related_spawner_commands_borrowed.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
    related_spawner_commands_boxed.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
    //~| HELP: `spawn_empty()` is more efficient
}
