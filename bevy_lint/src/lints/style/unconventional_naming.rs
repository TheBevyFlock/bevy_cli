//! Checks for structures that implement a Bevy trait and do not follow the opinionated naming
//! Convention
//!
//! # Motivation
//!
//! To keep naming consistent, commonly used traits in Bevy should follow an opinionated naming
//! Pattern to easily understand how a type should be used.
//!
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
//! struct MyAudio;
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
//! struct MyAudioSet;
//! ```

use clippy_utils::{diagnostics::span_lint_hir_and_then, path_res};
use rustc_errors::Applicability;
use rustc_hir::{HirId, Impl, Item, ItemKind, OwnerId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::symbol::Ident;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::impls_trait};

declare_bevy_lint! {
    pub UNCONVENTIONAL_NAMING,
    super::STYLE,
    "Unconventional struct name for this trait impl",
}

declare_bevy_lint_pass! {
    pub UnconventionalNaming => [UNCONVENTIONAL_NAMING.lint],
}

impl<'tcx> LateLintPass<'tcx> for UnconventionalNaming {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &Item<'tcx>) {
        // Find `impl` items...
        if let ItemKind::Impl(impl_) = item.kind
            && let Some(conventional_name_impl) =
                ConventionalNameTraitImpl::try_from_impl(cx, impl_)
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
                UNCONVENTIONAL_NAMING.lint,
                struct_hir_id,
                struct_span,
                UNCONVENTIONAL_NAMING.lint.desc,
                |diag| {
                    diag.span_suggestion(
                        struct_span,
                        format!("rename the {conventional_name_impl}"),
                        conventional_name_impl.name_suggestion(struct_name.as_str()),
                        // There may be other references that also need to be renamed.
                        Applicability::MaybeIncorrect,
                    );

                    diag.span_note(
                        item.span,
                        format!("`{conventional_name_impl}` implemented here"),
                    );
                },
            );
        }
    }
}

enum ConventionalNameTraitImpl {
    SystemSet,
}

impl std::fmt::Display for ConventionalNameTraitImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConventionalNameTraitImpl::SystemSet => {
                write!(f, "SystemSet")
            }
        }
    }
}

impl ConventionalNameTraitImpl {
    /// check if this `impl` block implements a Bevy trait that should follow a naming pattern
    fn try_from_impl(cx: &LateContext, impl_: &Impl) -> Option<Self> {
        if impls_trait(cx, impl_, &crate::paths::SYSTEM_SET) {
            Some(ConventionalNameTraitImpl::SystemSet)
        } else {
            None
        }
    }

    /// Test if the Structure name matches the naming convention
    fn matches_conventional_name(&self, struct_name: &str) -> bool {
        match self {
            ConventionalNameTraitImpl::SystemSet => struct_name.ends_with("Set"),
        }
    }

    /// Suggest a name for the Structure that matches the naming pattern
    fn name_suggestion(&self, struct_name: &str) -> String {
        match self {
            ConventionalNameTraitImpl::SystemSet => {
                format!("{struct_name}Set")
            }
        }
    }
}
