#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::events_resource_system_param)]

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, (bad_system,)).run();
}

//~| HELP: events Resource footgun
//~v ERROR: use of bad system parameter
fn bad_system(_resource_events: Res<Events<AppExit>>) {}
