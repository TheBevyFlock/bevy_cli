//@aux-build:../auxiliary/proc_macros.rs

//@no-rustfix
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::borrowed_reborrowable)]

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;

macro_rules! local_macro {
    () => {
        let mut world = World::new();
        let mut commands = world.commands();
        //~| HELP: use `Commands` instead
        //~v ERROR: parameter takes `&mut Commands` instead of a re-borrowed `Commands`
        let closure = |commands: &mut Commands| {
            commands.spawn_empty();
        };
        closure(&mut commands);
    };
}

fn main() {
    let mut world = World::new();
    let mut commands = world.commands();
    local_macro!();
    external! {
        let closure = |$commands: &mut Commands| {
            $commands.spawn_empty();
        };
        closure(&mut $commands);
    }

    let closure = external! {
         |$commands: &mut Commands| {
            $commands.spawn_empty();
        }
    };
    closure(&mut commands);

    let mut commands = external!($world.commands());

    //~| HELP: use `Commands` instead
    //~v ERROR: parameter takes `&mut Commands` instead of a re-borrowed `Commands`
    let closure = |commands: &mut Commands| {
        commands.spawn_empty();
    };

    closure(&mut commands);

    external! {
        mutable_reference(&mut $commands);
        immutable_reference(&$commands);
        _ = mutable_reference_return(&mut $commands);
        _ = mutable_reference_bounded_return(&mut $commands);
        _ = mutable_reference_bounded_return_complex(&mut $commands);
        fn immutable_reference(_commands: &Commands) {
        }
        fn mutable_reference(commands: &mut Commands) {
            commands.spawn_empty();
        }

        fn mutable_reference_return<'a>(_commands: &'a mut Commands) -> usize {
            123
        }

        fn mutable_reference_bounded_return<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
            commands.spawn_empty()
        }

        fn mutable_reference_bounded_return_complex<'a>(
            commands: &'a mut Commands,
        ) -> Vec<(usize, EntityCommands<'a>)> {
            vec![(1, commands.spawn_empty())]
        }
    }
}
