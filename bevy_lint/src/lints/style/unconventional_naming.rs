//! Checks for types that implement certain Bevy traits but do not follow that trait's naming
//! convention.
//!
//! This lint currently enforces the following conventions:
//!
//! |Trait|Convention|
//! |-|-|
//! |`Plugin`|Name ends in "Plugin"|
//! |`SystemSet`|Name ends in "Systems"|
//!
//! # Motivation
//!
//! Bevy provides several traits, such as `Plugin` and `SystemSet`, that designate the primary
//! purpose of a type. It is common for these types to follow certain naming conventions that
//! *signal* how it should be used. This lint helps enforce these conventions to ensure consistency
//! across the Bevy engine and ecosystem.
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! struct Physics;
//!
//! impl Plugin for Physics {
//! #     fn build(&self, app: &mut App) {}
//! #
//!     // ...
//! }
//!
//! #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
//! struct MyAudio;
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! struct PhysicsPlugin;
//!
//! impl Plugin for PhysicsPlugin {
//! #     fn build(&self, app: &mut App) {}
//! #
//!     // ...
//! }
//!
//! #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
//! struct MyAudioSystems;
//! ```

use clippy_utils::{diagnostics::span_lint_hir_and_then, path_res};
use rustc_errors::Applicability;
use rustc_hir::{HirId, Impl, Item, ItemKind, OwnerId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::symbol::Ident;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::impls_trait};

declare_bevy_lint! {
    pub(crate) UNCONVENTIONAL_NAMING,
    super::Style,
    "unconventional type name for a `Plugin` or `SystemSet`",
}

declare_bevy_lint_pass! {
    pub(crate) UnconventionalNaming => [UNCONVENTIONAL_NAMING],
}

impl<'tcx> LateLintPass<'tcx> for UnconventionalNaming {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &Item<'tcx>) {
        // Find `impl` items...
        if let ItemKind::Impl(ref impl_) = item.kind
            && let Some(conventional_name_impl) = TraitConvention::try_from_impl(cx, impl_)
        {
            // Try to resolve where this type was originally defined. This will result in a `DefId`
            // pointing to the original `struct Foo` definition, or `impl <T>` if it's a generic
            // parameter.
            let Some(struct_def_id) = path_res(cx, impl_.self_ty).opt_def_id() else {
                return;
            };

            // If this type is a generic parameter, exit. Their names, such as `T`, cannot be
            // referenced by others.
            if impl_
                .generics
                .params
                .iter()
                .any(|param| param.def_id.to_def_id() == struct_def_id)
            {
                return;
            }

            // Find the original name and span of the type.
            let Some(Ident {
                name: struct_name,
                span: struct_span,
            }) = cx.tcx.opt_item_ident(struct_def_id)
            else {
                return;
            };

            // skip lint if the struct was defined in an external macro
            if struct_span.in_external_macro(cx.tcx.sess.source_map()) {
                return;
            }

            // If the type's name matches the given convention
            if conventional_name_impl.matches_conventional_name(struct_name.as_str()) {
                return;
            }

            // Convert the `DefId` of the structure to a `LocalDefId`. If it cannot be converted
            // then the struct is from an external crate, in which case this lint should not be
            // emitted. (The user cannot easily rename that struct if they didn't define it.)
            let Some(struct_local_def_id) = struct_def_id.as_local() else {
                return;
            };

            // Convert struct `LocalDefId` to an `HirId` so that we can emit the lint for the
            // correct HIR node.
            let struct_hir_id: HirId = OwnerId {
                def_id: struct_local_def_id,
            }
            .into();

            span_lint_hir_and_then(
                cx,
                UNCONVENTIONAL_NAMING,
                struct_hir_id,
                struct_span,
                conventional_name_impl.lint_description(),
                |diag| {
                    diag.note(format!(
                        "structures that implement `{}` should end in \"{}\"",
                        conventional_name_impl.name(),
                        conventional_name_impl.suffix()
                    ));

                    diag.span_suggestion(
                        struct_span,
                        format!("rename `{}`", struct_name.as_str()),
                        conventional_name_impl.name_suggestion(struct_name.as_str()),
                        Applicability::MaybeIncorrect,
                    );

                    diag.span_note(
                        item.span,
                        format!("`{}` implemented here", conventional_name_impl.name()),
                    );
                },
            );
        }
    }
}

/// Collections of bevy traits where types that implement this trait should follow a specific naming
/// convention
enum TraitConvention {
    SystemSet,
    Plugin,
}

impl TraitConvention {
    /// check if this `impl` block implements a Bevy trait that should follow a naming pattern
    fn try_from_impl(cx: &LateContext, impl_: &Impl) -> Option<Self> {
        if impls_trait(cx, impl_, &crate::paths::SYSTEM_SET) {
            Some(TraitConvention::SystemSet)
        } else if impls_trait(cx, impl_, &crate::paths::PLUGIN) {
            Some(TraitConvention::Plugin)
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        match self {
            TraitConvention::SystemSet => "SystemSet",
            TraitConvention::Plugin => "Plugin",
        }
    }

    /// Returns the suffix that should be used when implementing this trait
    fn suffix(&self) -> &'static str {
        match self {
            TraitConvention::SystemSet => "Systems",
            TraitConvention::Plugin => "Plugin",
        }
    }

    fn lint_description(&self) -> String {
        format!("unconventional type name for a `{}`", self.name())
    }

    /// Test if the Structure name matches the naming convention
    fn matches_conventional_name(&self, struct_name: &str) -> bool {
        struct_name.ends_with(self.suffix())
    }

    /// Suggest a name for the Structure that matches the naming pattern
    fn name_suggestion(&self, struct_name: &str) -> String {
        match self {
            TraitConvention::SystemSet => {
                // There are several competing naming standards. These are a few that we specially
                // check for.
                const INCORRECT_SUFFIXES: [&str; 3] = ["System", "Set", "Steps"];

                // If the name ends in one of the other suffixes, strip it out and replace it with
                // "Systems". If a struct was originally named `FooSet`, this suggests `FooSystems`
                // instead of `FooSetSystems`.
                for incorrect_suffix in INCORRECT_SUFFIXES {
                    if let Some(stripped_name) = struct_name.strip_suffix(incorrect_suffix) {
                        return format!("{stripped_name}{}", self.suffix());
                    }
                }

                // If none of the special cases are matched, simply append the suffix.
                format!("{struct_name}{}", self.suffix())
            }
            TraitConvention::Plugin => {
                // If the name is prefixed with "Plugin", remove it and add it to the end.
                if let Some(stripped_name) = struct_name.strip_prefix("Plugin") {
                    return format!("{stripped_name}{}", self.suffix());
                }

                // If "Plugins" is plural instead of singular, remove the "s" to make it singular.
                if let Some(stripped_name) = struct_name.strip_suffix("Plugins") {
                    return format!("{stripped_name}{}", self.suffix());
                }

                // If none of the special cases are matched, simply append the suffix.
                format!("{struct_name}{}", self.suffix())
            }
        }
    }
}
