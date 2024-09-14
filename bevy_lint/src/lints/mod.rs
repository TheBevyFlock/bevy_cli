use rustc_lint::Lint;

macro_rules! define_lints {
    {
        $(mod $module:ident {
            lint: $lint:ident$(,)?
        })*
    } => {
        // Declare all modules as private.
        $(pub mod $module;)*

        pub static LINTS: &[&Lint] = &[
            $(self::$module::$lint,)*
        ];
    };
}

define_lints! {}
