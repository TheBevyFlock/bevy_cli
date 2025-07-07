#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::fixed_update_schedule)]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        //~v ERROR: the `FixedUpdate` schedule is disallowed
        .add_systems(FixedUpdate, hello_world);

    // Ensure the lint can be muted by annotating the expression.
    app.add_systems(
        #[expect(bevy::fixed_update_schedule)]
        FixedUpdate,
        hello_world,
    );

    app.run();
}

fn hello_world() {
    println!("hello world!");
}
