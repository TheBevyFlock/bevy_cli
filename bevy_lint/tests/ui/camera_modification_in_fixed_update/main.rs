#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::camera_modification_in_fixed_update)]

use bevy::prelude::*;

#[derive(Component)]
struct Hp;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        // This should not raise an error since its in `Startup`
        .add_systems(Startup, move_camera)
        .add_systems(FixedUpdate, move_camera)
        //~^ ERROR: camera modified in the `FixedUpdate` schedule
        //~| HELP: insert the system in the `Update` schedule instead
        .add_systems(FixedUpdate, move_camera_tuple_data)
        //~^ ERROR: camera modified in the `FixedUpdate` schedule
        //~| HELP: insert the system in the `Update` schedule instead
        // This should not raise an error since its in not mutably borrowing any data
        .add_systems(FixedUpdate, dont_mut_camera)
        //~| ERROR: camera modified in the `FixedUpdate` schedule
        //~v HELP: insert the system in the `Update` schedule instead
        .add_systems(FixedUpdate, (move_camera, move_camera_tuple_data))
        //~^ ERROR: camera modified in the `FixedUpdate` schedule
        //~| HELP: insert the system in the `Update` schedule instead
        .add_systems(FixedUpdate, multiple_queries)
        //~^ ERROR: camera modified in the `FixedUpdate` schedule
        //~| HELP: insert the system in the `Update` schedule instead
        .add_systems(FixedUpdate, multiple_none_mut_queries)
        .add_systems(FixedUpdate, multiple_query_filters)
        //~^ ERROR: camera modified in the `FixedUpdate` schedule
        //~| HELP: insert the system in the `Update` schedule instead
        .add_systems(FixedUpdate, multiple_none_mut_query_filters)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera::default()));
}

fn move_camera(mut _query: Query<&mut Transform, With<Camera>>) {}

fn dont_mut_camera(_query: Query<&Transform, With<Camera>>) {}

fn move_camera_tuple_data(
    mut _query: Query<(&mut Transform, &Hp, Entity), With<Camera>>,
    mut _commands: Commands,
    _time: Res<Time>,
) {
}

fn multiple_queries(
    mut _query: Query<(&mut Transform, Entity), With<Camera>>,
    mut _query2: Query<(&mut Hp, Entity), With<Player>>,
) {
}

fn multiple_none_mut_queries(
    _query: Query<(&Transform, Entity), With<Camera>>,
    mut _query2: Query<(&mut Hp, Entity), With<Player>>,
) {
}

fn multiple_query_filters(
    mut _query: Query<(&mut Transform, Entity), (With<Camera>, Without<Player>)>,
) {
}

fn multiple_none_mut_query_filters(
    _query: Query<(&Transform, Entity), (With<Camera>, Without<Player>)>,
) {
}
