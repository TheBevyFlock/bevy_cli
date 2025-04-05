//! The library backend for the Bevy CLI.

pub mod build;
pub mod external_cli;
pub mod lint;
mod memory;
pub mod run;
pub mod template;
#[cfg(feature = "web")]
pub(crate) mod web;
