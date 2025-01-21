//! Utilities for building and running the app in the browser.

pub(crate) mod bundle;
pub(crate) mod profiles;
#[cfg(feature = "wasm-opt")]
pub(crate) mod wasm_opt;
