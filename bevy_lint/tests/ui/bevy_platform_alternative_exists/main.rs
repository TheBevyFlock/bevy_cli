#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::bevy_platform_alternative_exists)]

fn main() {
    let foo: std::time::Instant = std::time::Instant::now();
}
