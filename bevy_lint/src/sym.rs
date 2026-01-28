//! Pre-interned [`Symbol`]s available in `const` contexts.
//!
//! [`Symbol`]s are [interned strings](https://en.wikipedia.org/wiki/String_interning) that are
//! cheap to store and compare (they're secretly [`u32`]s!). This module contains a list of pre-
//! interned symbol constants that are used in the linter.
//!
//! # Symbol Offsets
//!
//! The linter allocates symbols in the following layout:
//!
//! <table>
//!     <thead>
//!         <tr>
//!             <th>Index</th>
//!             <th>Note</th>
//!         </tr>
//!     </thead>
//!     <tbody>
//!         <tr>
//!             <td>0</td>
//!             <td rowspan="4">Symbols in <code>rustc_span::sym</code></td>
//!         </tr>
//!         <tr>
//!             <td>1</td>
//!         </tr>
//!         <tr>
//!             <td>2</td>
//!         </tr>
//!         <tr>
//!             <td>...</td>
//!         </tr>
//!         <tr>
//!             <td><code>PREDEFINED_SYMBOLS_COUNT</code></td>
//!             <td rowspan="2">Symbols in <code>clippy_utils::sym</code></td>
//!         </tr>
//!         <tr>
//!             <td>...</td>
//!         </tr>
//!         <tr>
//!             <td><code>SYMBOL_OFFSET</code></td>
//!             <td rowspan="2">Symbols in <code>bevy_lint::sym</code></td>
//!         </tr>
//!         <tr>
//!             <td>...</td>
//!         </tr>
//!     </tbody>
//! </table>
//!
//! Note that the order here is important. [`clippy_utils`] expects its symbols to start at
//! [`PREDEFINED_SYMBOLS_COUNT`], which is why it's before Bevy's symbols.

#![expect(
    non_upper_case_globals,
    reason = "Symbol constants are named as-is so it is easy to see what strings they represent."
)]

use clippy_utils::sym::EXTRA_SYMBOLS as CLIPPY_SYMBOLS;
// These are symbols that we use but are already interned by either the compiler or Clippy.
pub use clippy_utils::sym::{app, filter};
pub use rustc_span::sym::{bevy_ecs, bundle, message, plugin, reflect};
use rustc_span::{Symbol, symbol::PREDEFINED_SYMBOLS_COUNT};

/// The starting offset used for the first Bevy-specific symbol.
///
/// This is used instead of [`PREDEFINED_SYMBOLS_COUNT`] because this takes into account Clippy's
/// pre-interned symbols as well.
const SYMBOL_OFFSET: u32 = PREDEFINED_SYMBOLS_COUNT + CLIPPY_SYMBOLS.len() as u32;

/// A helper used by `declare_bevy_symbols!` to extract its input.
///
/// ```ignore
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

/// Generates the [`Symbol`] constants and [`BEVY_SYMBOLS`] from a list of name-value pairs.
///
/// # Example
///
/// ```ignore
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
        const BEVY_SYMBOLS: &[&str] = &[
            $(
                extract_value!($name $(: $value)?)
            ),*
        ];

        $(
            #[doc = concat!("A pre-interned [`Symbol`] for the string \"", extract_value!($name $(: $value)?), "\".")]
            pub const $name: Symbol = Symbol::new(SYMBOL_OFFSET + ${index()});
        )*
    };
}

// Before adding a new symbol here, check that it doesn't exist yet in `rustc_span::sym` or
// `clippy_utils::sym`. Having duplicate symbols will cause the compiler to ICE! Also please keep
// this list alphabetically sorted :)
declare_bevy_symbols! {
    App,
    Bundle,
    Camera,
    Commands,
    Component,
    Deferred,
    DeferredWorld,
    EntityCommands,
    EntityMut,
    Event,
    FilteredEntityMut,
    FixedUpdate,
    Message,
    Messages,
    Mut,
    MutUntyped,
    NonSendMut,
    PartialReflect,
    Plugin,
    PtrMut,
    Query,
    Reflect,
    RelatedSpawner,
    RelatedSpawnerCommands,
    ResMut,
    Resource,
    SystemSet,
    Update,
    With,
    World,
    add_systems,
    bevy,
    bevy_app,
    bevy_camera,
    bevy_ptr,
    bevy_reflect,
    camera,
    change_detection,
    commands,
    component,
    deferred_world,
    entity_access,
    event,
    init_resource,
    insert_resource,
    iter_current_update_messages,
    main_schedule,
    params,
    query,
    related_methods,
    relationship,
    resource,
    run,
    schedule,
    set,
    spawn,
    system,
    system_param,
    world,
}

/// Returns a list of strings that should be supplied to
/// [`Config::extra_symbols`](rustc_interface::interface::Config::extra_symbols).
pub fn extra_symbols() -> Vec<&'static str> {
    let mut symbols = Vec::with_capacity(CLIPPY_SYMBOLS.len() + BEVY_SYMBOLS.len());

    // The Clippy symbols must be before the Bevy symbols, as `clippy_utils` depends on its
    // predefined symbols having specific values.
    symbols.extend_from_slice(CLIPPY_SYMBOLS);
    symbols.extend_from_slice(BEVY_SYMBOLS);

    symbols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_for_duplicate_symbols() {
        // Create the thread-local variables necessary to intern strings. These thread-locals only
        // live as long as the closure, and are unset at the end of this test.
        rustc_span::create_session_globals_then(
            rustc_lint_defs::Edition::Edition2024,
            // Also intern Clippy's symbols, as well as the Rust compiler's symbols. While the
            // interner has its own check for duplicate symbols, it doesn't display what those
            // duplicates are, so we purposefully avoid pre-interning `BEVY_SYMBOLS` and check for
            // duplicate symbols manually instead.
            CLIPPY_SYMBOLS,
            None,
            || {
                const UNIQUE_STRING: &str = "\
This is a string that we can be reasonably confident will not be in the interner ahead of time. The
interner allocates symbol IDs from 0 upwards. By interning a new string that the interner hasn't
seen before, we can find the largest symbol ID. For example, if there are 10 symbols in the
interner with IDs `0..=9`, inserting this string will return a symbol with ID 10. This lets us loop
through all symbols pre-interned by the Rust compiler and Clippy linter! :)";

                let upper_symbol = Symbol::intern(UNIQUE_STRING);

                let mut duplicate_symbols = Vec::new();

                for i in 0..upper_symbol.as_u32() {
                    let symbol = Symbol::new(i);
                    let symbol_str = symbol.as_str();

                    // Check if `BEVY_SYMBOLS` contains a string that is already interened. We can
                    // binary search for it, as `BEVY_SYMBOLS` is guaranteed to be sorted.
                    if BEVY_SYMBOLS.binary_search(&symbol_str).is_ok() {
                        // `BEVY_SYMBOLS` contains a duplicate, keep track of that.
                        duplicate_symbols.push(symbol_str.to_owned());
                    }
                }

                assert!(
                    duplicate_symbols.is_empty(),
                    "`BEVY_SYMBOLS` should not introduce symbols already added by the Rust compiler or Clippy. The following duplicate symbols were found: {duplicate_symbols:?}",
                );
            },
        );
    }

    #[test]
    fn bevy_symbols_are_sorted() {
        if !BEVY_SYMBOLS.is_sorted() {
            let mut sorted = Vec::from(BEVY_SYMBOLS);
            sorted.sort();

            let sorted_string = sorted.join(",\n    ");

            panic!(
                "`declare_bevy_symbols!` is not sorted, it should be:

declare_bevy_symbols! {{
    {sorted_string},
}}"
            );
        }
    }
}
