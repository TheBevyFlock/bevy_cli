#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unit_bundle)]

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut commands: Commands) {
    commands.spawn(());
    //~^ ERROR: created a `Bundle` containing a unit `()`
}
