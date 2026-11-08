// This tracks the issue that we cannot lint if only `std::time` is imported, if this tests starts
// failing, this known issue has been solved.
//@check-pass

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::bevy_platform_alternative_exists)]
#![allow(dead_code)]

use std::time;

fn main() {
    let _use_time = time::Instant::now();
}
