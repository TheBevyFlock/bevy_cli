#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_unit_bundle)]

use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new().add_systems(Startup, test);
}

fn test(mut commands: Commands) {
    commands.spawn((
        Name::new("Decal"),
        Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
        //~^ ERROR: inserted a `Bundle` containing a unit `()` type
        example(),
        //~^ ERROR: inserted a `Bundle` containing a unit `()` type
        (),
        //~^ ERROR: inserted a `Bundle` containing a unit `()` type
        {
            example();

            Transform::from_translation(Vec3::ONE)
        },
    ));
}

fn example() {
    let foo = 1;
}
