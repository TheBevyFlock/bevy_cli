use rustc_lint::Lint;

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

        pub static LINTS: &[&Lint] = &[
            $($(self::$module::$lint)*,)*
        ];
    };
}

define_lints! {}
