//! Supporting types and macros that help simplify developing a linter.

use rustc_lint::{Level, Lint, LintId, LintStore};

/// Represents a lint group that can control the level of a collection of lints.
pub trait LintGroup2 {
    /// The name of this lint group, starting with the `bevy::` prefix.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Lint group definition within `bevy_lint`.
    /// pub struct Foo;
    ///
    /// impl LintGroup2 for Foo {
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
}

/// A Bevy lint definition and its associated group.
///
/// The level of the lint must be the same as the level of the group.
#[derive(Debug)]
#[deprecated]
pub struct BevyLint {
    pub lint: &'static Lint,
}

/// Represents a lint group.
#[derive(PartialEq, Debug)]
#[deprecated]
pub struct LintGroup {
    /// The name of the lint group.
    ///
    /// This will be used when trying to enable / disable the group, such as through
    /// `#![allow(group)]`. By convention, this should start with `bevy::`.
    pub name: &'static str,

    // The default level all lints within this group should be.
    pub level: Level,
}

/// Creates a new [`BevyLint`].
///
/// # Example
///
/// ```ignore
/// declare_bevy_lint! {
///     // This lint will be named `bevy::lint_name`.
///     pub LINT_NAME,
///     // The path to the lint group static.
///     super::LINT_GROUP,
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
        $level:expr,
        $desc:expr,
        $(@report_in_external_macro = $report_in_external_macro:expr,)?
        $(@crate_level_only = $crate_level_only:expr,)?
        $(@eval_always = $eval_always:expr,)?
    } => {
        /// Click me for more information.
        ///
        /// ```ignore
        /// Lint {
        #[doc = concat!("    name: \"bevy::", stringify!($name), "\",")]
        #[doc = concat!("    group: ", stringify!($level), ",")]
        #[doc = concat!("    description: ", stringify!($desc), ",")]
        /// }
        /// ```
        $(#[$attr])*
        #[expect(deprecated)]
        $vis static $name: &$crate::lint::BevyLint = &$crate::lint::BevyLint {
            lint: &::rustc_lint::Lint {
                // Fields that are always configured by macro.
                name: concat!("bevy::", stringify!($name)),
                default_level: $level,
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
            },
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
///     pub LintPassName => [LINT_NAME.lint],
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
