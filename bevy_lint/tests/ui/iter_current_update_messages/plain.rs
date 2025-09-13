#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::iter_current_update_messages)]

use bevy::prelude::*;

#[derive(Message)]
struct MyMessage;

fn main() {
    let messages: Messages<MyMessage> = Messages::default();
    let _ = messages.iter_current_update_messages();
    //~^ ERROR: called `Messages::<T>::iter_current_update_messages()`
}
