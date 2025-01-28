//! This test tracks the bug reported in [#248]. When this starts failing, the bug has been fixed.
//!
//! [#248]: https://github.com/TheBevyFlock/bevy_cli/issues/248

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_unit_bundle)]

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut commands: Commands) {
    commands.spawn(bundle());
    //~^ ERROR: inserted a `Bundle` containing a unit `()` type
}

fn bundle() -> (Name, ()) {
    // Error should be emitted here, not above.
    (Name::new("Foo"), ())
}
