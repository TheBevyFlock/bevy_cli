#![feature(register_tool)]
#![register_tool(bevy)]
//~v NOTE: the lint level is defined here
#![deny(bevy::missing_copy)]

#[derive(Clone)]
//~| HELP: `Copy` can be automatically derived
//~v ERROR: defined a unit struct without a `Copy` implementation
pub struct IsDefaultUiCamera;

// This should not raise an ERROR, since `Copy` is derived.
#[derive(Copy, Clone)]
pub struct DeriveCopy;

// This should not raise an ERROR, since `Copy` is implemented.
#[derive(Clone)]
pub struct ImplCopy;

impl Copy for ImplCopy {}

// This should not raise an ERROR, since this is not a unit struct.
pub struct ComponentWithFields(#[allow(dead_code)] f32);

#[allow(bevy::missing_copy)]
// This should not raise an ERROR, since the lint is silenced.
pub struct AllowMissingCopy;
