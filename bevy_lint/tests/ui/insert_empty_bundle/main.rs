#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_empty_bundle)]

use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new().add_systems(Startup, test);
}

fn test(mut commands: Commands) {
    commands.spawn((
        Name::new("Decal"),
        Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
        //~^ ERROR: Expression returns `()` and results in an empty bundle being inserted
    ));
}
