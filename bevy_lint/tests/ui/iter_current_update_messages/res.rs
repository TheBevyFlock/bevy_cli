#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::iter_current_update_messages)]

use bevy::prelude::*;

#[derive(Message)]
struct MyMessage;

fn main() {
    App::new()
        .add_message::<MyMessage>()
        .add_systems(Update, my_system);
}

fn my_system(events: Res<Messages<MyMessage>>) {
    for _event in events.iter_current_update_messages() {
        //~^ ERROR: called `Messages::<T>::iter_current_update_messages()`
    }
}
