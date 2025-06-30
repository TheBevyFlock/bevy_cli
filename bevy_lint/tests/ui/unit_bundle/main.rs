#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unit_bundle)]

use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut commands: Commands) {
    commands.spawn((
        Name::new("Decal"),
        Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
        //~^ ERROR: created a `Bundle` containing a unit `()`
        (
            no_op(),
            //~^ ERROR: created a `Bundle` containing a unit `()`
            GlobalTransform::IDENTITY,
            (
                (),
                //~^ ERROR: created a `Bundle` containing a unit `()`
            ),
            {
                no_op();
                Transform::default()
            },
        ),
    ));
    Commands::spawn(
        &mut commands,
        (
            Name::new("Decal"),
            Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
            //~^ ERROR: created a `Bundle` containing a unit `()`
            (
                no_op(),
                //~^ ERROR: created a `Bundle` containing a unit `()`
                GlobalTransform::IDENTITY,
                (
                    (),
                    //~^ ERROR: created a `Bundle` containing a unit `()`
                ),
                {
                    no_op();
                    Transform::default()
                },
            ),
        ),
    );
}

fn no_op() {}
