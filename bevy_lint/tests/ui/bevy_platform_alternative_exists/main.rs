#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::bevy_platform_alternative_exists)]
#![allow(dead_code)]

fn main() {
    let mut hash_map = std::collections::HashMap::new();
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::collections::HashMap` can be replaced with the `no_std` compatible
    // type bevy::platform::collections::HashMap
    hash_map.insert("foo", "bar");

    // compatible type bevy::platform::collections::HashMap
    //~| HELP: the type `std::collections::HashMap` can be replaced with the `no_std`
    //~v ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    let _declared_hash_map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::collections::HashMap<u32, u32>` can be replaced with the `no_std`
    // compatible type bevy::platform::collections::HashMap<u32, u32>

    let _arc = std::sync::Arc::new(10);
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::sync::Arc` can be replaced with the `no_std`
    // compatible type bevy::platform::sync::Arc

    let barrier = std::sync::Barrier::new(10);
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::sync::Barrier` can be replaced with the `no_std`
    // compatible type bevy::platform::sync::Barrier

    let _barrier_wait_result: std::sync::BarrierWaitResult = barrier.wait();
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::sync::BarrierWaitResult` can be replaced with the `no_std`
    // compatible type bevy::platform::sync::BarrierWaitResult

    let mut hash_set = std::collections::HashSet::new();
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::collections::HashSet` can be replaced with the `no_std`
    // compatible type bevy::platform::collections::HashSet

    hash_set.insert(1);

    // compatible type bevy::platform::collections::HashSet
    //~| HELP: the type `std::collections::HashSet` can be replaced with the `no_std`
    //~v ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    let _declared_hash_map: std::collections::HashSet<u32> = std::collections::HashSet::new();
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::collections::HashSet<u32>` can be replaced with the `no_std`
    // compatible type bevy::platform::collections::HashSet<u32>

    let _lazy = std::sync::LazyLock::new(|| "lazy");
    //~^ ERROR: Used type from the `std` that has an existing alternative from `bevy_platform`
    //~| HELP: the type `std::sync::LazyLock` can be replaced with the `no_std`
    // compatible type bevy::platform::sync::LazyLock
}
