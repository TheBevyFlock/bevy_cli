//! The library backend for the prototype Bevy CLI.

pub(crate) mod bin_target;
pub mod commands;
pub(crate) mod common_args;
pub(crate) mod config;
pub(crate) mod external_cli;
#[cfg(feature = "web")]
pub(crate) mod web;
