#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::iter_current_update_events)]

use bevy::prelude::*;

#[derive(Event)]
struct MyEvent;

fn main() {
    App::new().add_event::<MyEvent>().add_systems(Update, my_system);
}

fn my_system(events: Res<Events<MyEvent>>) {
    for _event in events.iter_current_update_events() {
        //~^ ERROR: called `Events::<T>::iter_current_update_events()`
    }
}
