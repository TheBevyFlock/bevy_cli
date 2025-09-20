#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::iter_current_update_messages)]

use bevy::prelude::*;

#[derive(Message)]
struct MyMessage;

fn main() {
    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    let messages: Events<MyMessage> = Events::default();
    let _ = messages.iter_current_update_messages();
    //~^ ERROR: called `Messages::<T>::iter_current_update_messages()`
}
