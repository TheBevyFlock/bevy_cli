//! This test tracks the bug reported in [#141]. When this starts failing, the bug has been fixed.
//!
//! [#141]: https://github.com/TheBevyFlock/bevy_cli/issues/141

// Do not run `rustfix`, since the applied suggestions will cause a compile error.
//@no-rustfix

#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
//~v NOTE: the lint level is defined here
#![deny(bevy::missing_reflect)]

use bevy::prelude::*;

struct NonReflect(u64);

//~v NOTE: `Component` implemented here
#[derive(Component)]
//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a component without a `Reflect` implementation
struct MyComponent {
    reflect: u64,
    non_reflect: NonReflect,
}
