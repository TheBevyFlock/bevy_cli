//@check-pass

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_unit_bundle)]

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut commands: Commands) {
    #[allow(bevy::insert_unit_bundle)]
    commands.spawn(());

    commands.spawn((
        Name::new("Decal"),
        #[allow(bevy::insert_unit_bundle)]
        (),
    ));
}
