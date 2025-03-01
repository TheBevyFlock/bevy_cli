//! Utilities for building and running the app in the browser.

pub(crate) mod build;
pub(crate) mod bundle;
pub(crate) mod profiles;
pub(crate) mod run;
pub(crate) mod serve;
#[cfg(feature = "wasm-opt")]
pub(crate) mod wasm_opt;
