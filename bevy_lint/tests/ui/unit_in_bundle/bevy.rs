// Disable Rustfix, as some diagnostics cannot be automatically fixed.
//@no-rustfix

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unit_in_bundle)]

use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, commands_system)
        .add_systems(Startup, world_system);
}

fn commands_system(mut commands: Commands) {
    commands
        .spawn(
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
        )
        .insert(
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
        )
        .insert_if(
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
            || true,
        );

    commands.spawn((
        Name::new("Decal"),
        //~v ERROR: created a `Bundle` containing a unit `()`
        Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
    ));
}

fn world_system(world: &mut World) {
    world
        .spawn((
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
            Name::new("Jeffrey"),
        ))
        .insert(
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
        );
}
