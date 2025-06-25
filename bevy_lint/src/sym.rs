//! Pre-interned [`Symbol`]s available in `const` contexts.
//!
//! [`Symbol`]s are [interned strings](https://en.wikipedia.org/wiki/String_interning) that are
//! cheap to store and compare. This module contains a list of pre-interned symbol constants that
//! are used in the linter. The only exception to this is the [`EXTRA_SYMBOLS`] constant, which
//! contains a complete ordered list of all symbols to be pre-interned.

use rustc_span::{Symbol, symbol::PREDEFINED_SYMBOLS_COUNT};

/// A helper used by [`declare_bevy_symbols!`] to extract its input.
///
/// ```
/// assert_eq!(extract_value!(Name), "Name");
/// assert_eq!(extract_value!(Name: "value"), "value");
/// ```
macro_rules! extract_value {
    ($name:ident) => {
        stringify!($name)
    };
    ($name:ident: $value:literal) => {
        $value
    };
}

/// Generates the [`Symbol`] constants and [`EXTRA_SYMBOLS`] from a list of name-value pairs.
///
/// # Example
///
/// ```
/// declare_bevy_symbols! {
///     // Interns the string "Hello, world" available as the constant named `Hello`.
///     Hello: "Hello, world!",
///     // Interns the string "bevy" available as the constant named `bevy`. This is the shorthand!
///     bevy,
/// }
/// ```
macro_rules! declare_bevy_symbols {
    {
        $($name:ident $(: $value:literal)?),* $(,)?
    } => {
        /// A list of strings that are pre-interned at the beginning of linting through
        /// [`Config::extra_symbols`](rustc_interface::interface::Config::extra_symbols).
        pub const EXTRA_SYMBOLS: &[&str] = &[
            $(
                extract_value!($name $(: $value)?)
            ),*
        ];

        $(
            #[doc = concat!("A pre-interned [`Symbol`] for the string \"", extract_value!($name $(: $value)?), "\".")]
            pub const $name: Symbol = Symbol::new(PREDEFINED_SYMBOLS_COUNT + ${index()});
        )*
    };
}

declare_bevy_symbols! {}
