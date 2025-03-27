#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::iter_current_update_events)]

use bevy::prelude::*;

#[derive(Event)]
struct MyEvent;

fn main() {
    let events: Events<MyEvent> = Events::default();
    let _ = events.iter_current_update_events();
    //~^ ERROR: called `Events::<T>::iter_current_update_events()`
}
