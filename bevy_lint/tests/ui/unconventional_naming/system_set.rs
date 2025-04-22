#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unconventional_naming)]
//~^ NOTE: the lint level is defined here
#![allow(dead_code)]
use bevy::prelude::*;

//~v NOTE: `SystemSet` implemented here
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudio;
//~^ ERROR: unconventional type name for a `SystemSet`
//~| NOTE: structures that implement `SystemSet` should end in `Systems`
//~| HELP: rename `MyAudio`

// This should not raise an error since the Set ends in `Set`
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudioSystems;

//~v NOTE: `SystemSet` implemented here
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudioSystem;
//~^ ERROR: unconventional type name for a `SystemSet`
//~| NOTE: structures that implement `SystemSet` should end in `Systems`
//~| HELP: rename `MyAudioSystem`

//~v NOTE: `SystemSet` implemented here
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudioSet;
//~^ ERROR: unconventional type name for a `SystemSet`
//~| NOTE: structures that implement `SystemSet` should end in `Systems`
//~| HELP: rename `MyAudioSet`

//~v NOTE: `SystemSet` implemented here
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudioSteps;
//~^ ERROR: unconventional type name for a `SystemSet`
//~| NOTE: structures that implement `SystemSet` should end in `Systems`
//~| HELP: rename `MyAudioSteps`
