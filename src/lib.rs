//! The library backend for the Bevy CLI.

pub(crate) mod bin_target;
pub mod build;
pub mod config;
pub(crate) mod external_cli;
pub mod lint;
pub mod run;
pub mod template;
#[cfg(feature = "web")]
pub(crate) mod web;
