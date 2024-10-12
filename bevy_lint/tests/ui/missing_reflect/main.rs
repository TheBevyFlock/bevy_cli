#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
#![deny(bevy::missing_reflect)]

use bevy::{ecs::component::StorageType, prelude::*};

#[derive(Component)]
struct Missing;
//~^ ERROR: defined a component, resource, or event without a `Reflect` implementation

struct MissingImpl;
//~^ ERROR: defined a component, resource, or event without a `Reflect` implementation

impl Component for MissingImpl {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

#[derive(Component, Reflect)]
struct Satisfied;

#[allow(bevy::missing_reflect)]
#[derive(Component)]
struct Muted;

#[allow(bevy::missing_reflect)]
struct MutedImpl;

impl Component for MutedImpl {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}
