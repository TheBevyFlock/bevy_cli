#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::bevy_platform_alternative_exists)]

fn main() {
    let _ = std::time::Instant::now();
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::time::Instant` can be replaced with the `no_std` compatible type
    // bevy_platform::time::Instant
}
