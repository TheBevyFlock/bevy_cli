#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::camera_modification_in_fixed_update)]
#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
struct Hp;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        // .add_systems(FixedUpdate, move_camera)
        // .add_systems(FixedUpdate, move_camera_2)
        .add_systems(FixedUpdate, move_camera_3)
        //~^ ERROR: Camera modification in FixedUpdate shedule
        //~| HELP: Put System in Update instead
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera::default()));
}

fn move_camera(mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut query {
        transform.translation.x += 1.0;
    }
}

fn move_camera_2(mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut query {
        transform.translation.x += 1.0;
    }
}

fn move_camera_3(
    mut query: Query<(&mut Transform, &Hp, Entity), With<Camera>>,
    mut commands: Commands,
    _time: Res<Time>,
) {
    commands.spawn(Name::new("Camera3"));

    for (mut transform, _, _) in &mut query {
        transform.translation.x += 1.0;
    }
}
