#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::bevy_platform_alternative_exists)]

use std::time;

#[allow(dead_code)]
struct Player {
    attack_cd: time::Instant,
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::time::Instant` can be replaced with the `no_std` compatible
    // type bevy::platform::time::Instant
}

fn main() {
    let mut hash_map = std::collections::HashMap::new();
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::collections::HashMap` can be replaced with the `no_std` compatible
    // type bevy::platform::collections::HashMap
    hash_map.insert("foo", "bar");

    let _time = std::time::Instant::now();
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::time::Instant` can be replaced with the `no_std` compatible
    // type bevy::platform::time::Instant

    // This should be fine even tho it will result in a `std::time::Instant` after full path
    // resolution.
    let _bevy_time = bevy::platform::time::Instant::now();
}
