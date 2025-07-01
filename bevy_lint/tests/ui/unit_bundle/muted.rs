//@check-pass

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unit_bundle)]

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut commands: Commands) {
    commands.spawn(
        #[expect(bevy::unit_bundle)]
        (),
    );

    commands.spawn((
        Name::new("Decal"),
        #[expect(bevy::unit_bundle)]
        (),
    ));
}
