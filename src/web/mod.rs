//! Utilities for building and running the app in the browser.

pub(crate) mod bundle;
#[cfg(feature = "wasm-opt")]
pub(crate) mod wasm_opt;
