macro_rules! define_lints {
    {
        $(mod $module:ident {
            lints: [$($lint:ident),+],
            passes: [],
        })*
    } => {
        // Declare all modules as private.
        $(mod $module;)*

        // Re-export all lint definitions.
        $(pub use self::$module::{$($lint),*};)*
    };
}

define_lints! {}
