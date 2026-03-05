//! Tests that `missing_reflect` can be applied to individual items without a warning.
//@check-pass

#![feature(register_tool)]
#![register_tool(bevy)]

#![deny(bevy::missing_reflect)]
#![deny(unused_attributes)]

use bevy::prelude::*;

#[allow(bevy::missing_reflect)]
#[derive(Component)]
pub struct MyComponent(NonReflect);

struct NonReflect;
