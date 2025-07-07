#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::update_schedule)]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        //~v ERROR: the `Update` schedule is disallowed
        .add_systems(Update, hello_world);

    // Ensure the lint can be muted by annotating the expression.
    #[allow(bevy::update_schedule)]
    app.add_systems(Update, hello_world);

    app.run();
}

fn hello_world() {
    println!("hello world!");
}
