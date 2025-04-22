//! Supporting types and macros that help simplify developing a linter.

use rustc_lint::{Level, Lint, LintId, LintStore};

/// Represents a lint group that can control the level of a collection of lints.
pub trait LintGroup {
    /// The name of this lint group, starting with the `bevy::` prefix.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Lint group definition within `bevy_lint`.
    /// pub struct Foo;
    ///
    /// impl LintGroup for Foo {
    ///     const NAME: &str = "bevy::foo";
    ///     // ...
    /// }
    /// ```
    ///
    /// ```ignore
    /// // Enable this lint group in a user's code.
    /// #![warn(bevy::foo)]
    /// ```
    ///
    /// ```toml
    /// # Alternatively, enable this lint group in `Cargo.toml`.
    /// [package.metadata.bevy_lint]
    /// foo = "warn"
    /// ```
    const NAME: &str;

    /// The default level for all lints in this lint group.
    const LEVEL: Level;

    /// A list of all lints in this lint group.
    const LINTS: &[&Lint];

    /// Registers all of this lint group's lint passes into a given [`LintStore`].
    ///
    /// When implementing this function, you'll often be calling
    /// [`LintStore::register_late_pass()`] like so:
    ///
    /// ```ignore
    /// fn register_passes(store: &mut LintStore) {
    ///     store.register_late_pass(|_| Box::new(my_lint::MyLint::default()));
    /// }
    /// ```
    fn register_passes(store: &mut LintStore);

    /// Registers all of this group's lints into a given [`LintStore`].
    ///
    /// By default this will register all lints specified in [`Self::LINTS`].
    fn register_lints(store: &mut LintStore) {
        store.register_lints(Self::LINTS);
    }

    /// Registers this lint group into a given [`LintStore`].
    fn register_group(store: &mut LintStore) {
        let lint_ids = Self::LINTS.iter().map(|lint| LintId::of(lint)).collect();
        store.register_group(true, Self::NAME, None, lint_ids);
    }

    /// A utility functions that calls [`Self::register_lints()`], [`Self::register_passes()`], and
    /// [`Self::register_group()`] for the same [`LintStore`].
    fn register(store: &mut LintStore) {
        Self::register_passes(store);
        Self::register_lints(store);
        Self::register_group(store);
    }
}

/// Creates a new [`Lint`].
///
/// # Example
///
/// ```ignore
/// declare_bevy_lint! {
///     // This lint will be named `bevy::lint_name`.
///     pub LINT_NAME,
///     // The path to the lint group type.
///     super::LintGroup,
///     // The description printed by `bevy_lint_driver rustc -W help`, and sometimes also used in
///     // diagnostic messages.
///     "short description of lint",
///
///     // The following are optional fields, and may be excluded. They all default to false.
///     //
///     // Whether to report this lint, even if it is inside the expansion of an external macro.
///     @report_in_external_macro = false,
///     // Whether to only run this macro for the crate root. This should be enabled for lint
///     // passes that only override `check_crate()`.
///     @crate_level_only = false,
///     // The compiler can sometimes skip lint passes that are guaranteed not to run. This can
///     // disable that behavior.
///     @eval_always = false,
/// }
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! declare_bevy_lint {
    {
        $(#[$attr:meta])*
        $vis:vis $name:ident,
        $group:ty,
        $desc:expr,
        $(@report_in_external_macro = $report_in_external_macro:expr,)?
        $(@crate_level_only = $crate_level_only:expr,)?
        $(@eval_always = $eval_always:expr,)?
    } => {
        $(#[$attr])*
        $vis static $name: &::rustc_lint::Lint = &::rustc_lint::Lint {
            // Fields that are always configured by macro.
            name: concat!("bevy::", stringify!($name)),
            // The `*&` is a silly hack that appears to fix a compiler bug. Without it, lints will
            // misbehave, emitting diagnostics at the incorrect lint level (usually when the user
            // modifies the default lint level with `#[deny(...)]` and `#[warn(...)]`). I *think*
            // this is caused by a bug in MIR promotion, as the `*&` fix prevents this entire struct
            // from being constant evaluated, but I'm still unsure of the root cause.
            default_level: *&<$group as $crate::lint::LintGroup>::LEVEL,
            desc: $desc,

            // Fields that cannot be configured.
            edition_lint_opts: None,
            future_incompatible: None,
            feature_gate: None,
            is_externally_loaded: true,

            // Fields that may sometimes be configured by macro. These all default to false in
            // `Lint::default_fields_for_macro()`, but may be overridden to true.
            $(report_in_external_macro: $report_in_external_macro,)?
            $(crate_level_only: $crate_level_only,)?
            $(eval_always: $eval_always,)?

            ..::rustc_lint::Lint::default_fields_for_macro()
        };
    };
}

/// Creates a new [`LintPass`](rustc_lint::LintPass).
///
/// This is based on [`declare_lint_pass!`](rustc_lint_defs::declare_lint_pass), but supports more
/// options.
///
/// # Example
///
/// ```ignore
/// declare_bevy_lint_pass! {
///     // Declares which lints are emitted by this lint pass.
///     pub LintPassName => [LINT_NAME],
///
///     // The following are optional fields, and may be omitted.
///     //
///     // Declares fields of the lint pass that are set when `LintPassName::default()` is called.
///     @default = {
///         component: Symbol = Symbol::intern("component"),
///     },
/// }
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! declare_bevy_lint_pass {
    (
        $(#[$attr:meta])*
        $vis:vis $name:ident => [$($lint:expr),* $(,)?],

        $(
            @default = {
                $($default_field:ident: $default_ty:ty = $default_value:expr),* $(,)?
            },
        )?
    ) => {
        $(#[$attr])*
        $vis struct $name {
            $($($default_field: $default_ty),*)?
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                Self {
                    $($($default_field: $default_value),*)?
                }
            }
        }

        ::rustc_lint_defs::impl_lint_pass!($name => [$($lint),*]);
    };
}
