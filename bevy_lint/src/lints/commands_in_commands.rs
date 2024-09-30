//! TODO: Description
//!
//! # Motivation
//!
//! TODO
//!
//! # Known issues
//!
//! This lint will not check implicit commands (functions with the signature `fn(&mut World)`),
//! since it cannot determine if the function is a command or a system.
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! struct MyCommand;
//!
//! impl Command for MyCommand {
//!     fn apply(world: &mut World) {
//!         // This will be checked...
//!         world.commands().spawn_empty();
//!     }
//! }
//!
//! fn my_command(world: &mut World) {
//!     // ...but this will not.
//!     world.commands().spawn_empty();
//! }
//! ```
//!
//! # Example
//!
//! TODO

use rustc_lint::LateLintPass;
use rustc_session::declare_lint_pass;

use crate::declare_bevy_lint;

declare_bevy_lint! {
    pub COMMANDS_IN_COMMANDS,
    COMPLEXITY,
    "called `World::commands()` within a `Command` implementation"
}

declare_lint_pass! {
    CommandsInCommands => [COMMANDS_IN_COMMANDS.lint]
}

impl<'tcx> LateLintPass<'tcx> for CommandsInCommands {}
