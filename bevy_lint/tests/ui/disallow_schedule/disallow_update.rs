#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::disallow_update)]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        //~| HELP: use the FixedUpdate schedule instead
        //~v ERROR: use of the Update schedule is disallowed
        .add_systems(Update, hello_world);

    // Ensure the lint can be muted by annotating the expression.
    #[allow(bevy::disallow_update)]
    app.add_systems(Update, hello_world);

    app.run();
}

fn hello_world() {
    println!("hello world!");
}
