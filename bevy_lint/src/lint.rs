use rustc_lint::{Level, Lint, LintId};

/// A Bevy lint definition and its associated group.
///
/// The level of the lint must be the same as the level of the group.
#[derive(Debug)]
pub struct BevyLint {
    pub lint: &'static Lint,
    pub group: &'static LintGroup,
}

impl BevyLint {
    pub fn id(&self) -> LintId {
        LintId::of(self.lint)
    }
}

/// Represents a lint group.
#[derive(PartialEq, Debug)]
pub struct LintGroup {
    /// The name of the lint group.
    ///
    /// This will be used when trying to enable / disable the group, such as through
    /// `#![allow(group)]`. By convention, this should start with `bevy::`.
    pub name: &'static str,

    // The default level all lints within this group should be.
    pub level: Level,
}

#[macro_export]
macro_rules! declare_bevy_lint {
    {
        $(#[$attr:meta])*
        $vis:vis $name:ident,
        $group:ident,
        $desc:expr$(,)?
    } => {
        $(#[$attr])*
        $vis static $name: &$crate::lint::BevyLint = &$crate::lint::BevyLint {
            lint: &::rustc_lint::Lint {
                name: concat!("bevy::", stringify!($name)),
                default_level: $crate::groups::$group.level,
                desc: $desc,
                edition_lint_opts: None,
                report_in_external_macro: false,
                future_incompatible: None,
                is_externally_loaded: true,
                feature_gate: None,
                crate_level_only: false,
            },
            group: &$crate::groups::$group,
        };
    };
}
