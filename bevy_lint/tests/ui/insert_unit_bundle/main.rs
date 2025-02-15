//@aux-build:../auxiliary/proc_macros.rs
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_unit_bundle)]

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;
use std::f32::consts::PI;

macro_rules! local_macro {
    ($c:expr) => {
        $c.spawn((
            Name::new("Decal"),
            Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
            //~^ ERROR: inserted a `Bundle` containing a unit `()` type
            (
                no_op(),
                //~^ ERROR: inserted a `Bundle` containing a unit `()` type
                GlobalTransform::IDENTITY,
                (
                    (),
                    //~^ ERROR: inserted a `Bundle` containing a unit `()` type
                ),
                {
                    no_op();
                    Transform::default()
                },
            ),
        ));
    };
}

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut commands: Commands) {
    commands.spawn((
        Name::new("Decal"),
        Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
        //~^ ERROR: inserted a `Bundle` containing a unit `()` type
        (
            no_op(),
            //~^ ERROR: inserted a `Bundle` containing a unit `()` type
            GlobalTransform::IDENTITY,
            (
                (),
                //~^ ERROR: inserted a `Bundle` containing a unit `()` type
            ),
            {
                no_op();
                Transform::default()
            },
        ),
    ));
    Commands::spawn(
        &mut commands,
        (
            Name::new("Decal"),
            Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
            //~^ ERROR: inserted a `Bundle` containing a unit `()` type
            (
                no_op(),
                //~^ ERROR: inserted a `Bundle` containing a unit `()` type
                GlobalTransform::IDENTITY,
                (
                    (),
                    //~^ ERROR: inserted a `Bundle` containing a unit `()` type
                ),
                {
                    no_op();
                    Transform::default()
                },
            ),
        ),
    );
    local_macro!(commands);
}

fn no_op() {}

external! {
    fn my_system_external(mut commands: Commands) {
        commands.spawn((
            Name::new("Decal"),
            Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
            (
                no_op(),
                GlobalTransform::IDENTITY,
                (
                    (),
                ),
                {
                    no_op();
                    Transform::default()
                },
            ),
        ));
    }
}
