//! Checks for structures that implement `SystemSet` but whose names does not end in "Set".
//!
//! # Motivation
//!
//! It is a common practice to suffix a System Sets names with "Set" to signal how the type should
//! be used.
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
use rustc_hir::{HirId, Item, ItemKind, OwnerId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::symbol::Ident;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::impls_trait};

declare_bevy_lint! {
    pub UNCONVENTIONAL_NAMING,
    super::STYLE,
    "implemented `SystemSet` for a struct whose name does not end in \"Set\"",
}

declare_bevy_lint_pass! {
    pub UnconventionalNaming => [UNCONVENTIONAL_NAMING.lint],
}

impl<'tcx> LateLintPass<'tcx> for UnconventionalNaming {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &Item<'tcx>) {
        // Find `impl` items...
        if let ItemKind::Impl(impl_) = item.kind
            && impls_trait(cx, impl_, &crate::paths::SYSTEM_SET)
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

            // If the type's name ends in "Set", exit.
            if struct_name.as_str().ends_with("Set") {
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
                        "rename the SystemSet",
                        format!("{struct_name}Set"),
                        // There may be other references that also need to be renamed.
                        Applicability::MaybeIncorrect,
                    );

                    diag.span_note(item.span, "`SystemSet` implemented here");
                },
            );
        }
    }
}
