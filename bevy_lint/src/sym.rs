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

#![allow(
    non_upper_case_globals,
    reason = "Symbol constants are named as-is so it is easy to see what strings they represent."
)]

use clippy_utils::sym::EXTRA_SYMBOLS as CLIPPY_SYMBOLS;
/// These are symbols that we use but are already interned by either the compiler or Clippy.
pub use clippy_utils::sym::filter;
pub use rustc_span::sym::{bevy_ecs, plugin, reflect};
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
    add_systems,
    app,
    App,
    base,
    bevy_app,
    bevy_ptr,
    bevy_reflect,
    bevy_render,
    bevy,
    camera,
    Camera,
    change_detection,
    collections,
    commands,
    Commands,
    component,
    Component,
    deferred_world,
    Deferred,
    DeferredWorld,
    entity_ref,
    EntityCommands,
    EntityMut,
    event,
    Event,
    Events,
    FilteredEntityMut,
    FixedUpdate,
    init_resource,
    insert_resource,
    iter_current_update_events,
    main_schedule,
    Mut,
    MutUntyped,
    NonSendMut,
    PartialReflect,
    Plugin,
    PtrMut,
    query,
    Query,
    Reflect,
    ResMut,
    resource,
    Resource,
    run,
    schedule,
    set,
    spawn,
    system_param,
    system,
    SystemSet,
    Update,
    With,
    world,
    World,
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
