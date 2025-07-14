//! The library backend for the prototype Bevy CLI.

pub(crate) mod bin_target;
pub mod commands;
pub(crate) mod config;
pub(crate) mod external_cli;
pub mod test;
#[cfg(feature = "web")]
pub(crate) mod web;
