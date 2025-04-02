//! Checks for queries that query the data for a zero-sized type.
//!
//! # Motivation
//!
//! Zero-sized types (ZSTs) are types that have no size because they contain no runtime data. Any
//! information they may hold is known at compile-time in the form of [constant generics], which do
//! not need to be queried. As such, ZSTs are better used as query filters instead of query data.
//!
//! [constant generics]: https://doc.rust-lang.org/reference/items/generics.html#const-generics
//!
//! # Known Issues
//!
//! This lint raises false positives on queries like `Has<T>` and `AnyOf<T>` because they are ZSTs,
//! even though they still retrieve data from the ECS. Please see [#279] for more information.
//!
//! [#279]: https://github.com/TheBevyFlock/bevy_cli/issues/279
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! // This is a zero-sized type, sometimes known as a "marker component".
//! #[derive(Component)]
//! struct Player;
//!
//! fn move_player(mut query: Query<(&mut Transform, &Player)>) {
//!     for (transform, _) in query.iter_mut() {
//!         // ...
//!     }
//! }
//! #
//! # assert_eq!(std::mem::size_of::<Player>(), 0);
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(Component)]
//! struct Player;
//!
//! fn move_player(mut query: Query<&mut Transform, With<Player>>) {
//!     for transform in query.iter_mut() {
//!         // ...
//!     }
//! }
//! #
//! # assert_eq!(std::mem::size_of::<Player>(), 0);
//! ```

use crate::{
    declare_bevy_lint, declare_bevy_lint_pass,
    utils::hir_parse::{detuple, generic_type_at},
};
use clippy_utils::{
    diagnostics::span_lint_and_help,
    ty::{is_normalizable, match_type, ty_from_hir_ty},
};
use rustc_abi::Size;
use rustc_hir::AmbigArg;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{
    Ty,
    layout::{LayoutOf, TyAndLayout},
};

declare_bevy_lint! {
    pub ZST_QUERY,
    // This will eventually be a `RESTRICTION` lint, but due to
    // <https://github.com/TheBevyFlock/bevy_cli/issues/279> it is not yet ready for production.
    super::Nursery,
    "queried a zero-sized type",
}

declare_bevy_lint_pass! {
    pub ZstQuery => [ZST_QUERY],
}

impl<'tcx> LateLintPass<'tcx> for ZstQuery {
    fn check_ty(&mut self, cx: &LateContext<'tcx>, hir_ty: &'tcx rustc_hir::Ty<'tcx, AmbigArg>) {
        if hir_ty.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }
        let ty = ty_from_hir_ty(cx, hir_ty.as_unambig_ty());

        let Some(query_kind) = QueryKind::try_from_ty(cx, ty) else {
            return;
        };

        let Some(query_data_ty) = generic_type_at(cx, hir_ty.as_unambig_ty(), 2) else {
            return;
        };

        for hir_ty in detuple(*query_data_ty) {
            let ty = ty_from_hir_ty(cx, &hir_ty);

            // We want to make sure we're evaluating `Foo` and not `&Foo`/`&mut Foo`
            let peeled = ty.peel_refs();

            if !is_zero_sized(cx, peeled).unwrap_or_default() {
                continue;
            }

            // TODO: We can also special case `Option<&Foo>`/`Option<&mut Foo>` to
            //       instead suggest `Has<Foo>`
            span_lint_and_help(
                cx,
                ZST_QUERY,
                hir_ty.span,
                ZST_QUERY.desc,
                None,
                query_kind.help(peeled),
            );
        }
    }
}

enum QueryKind {
    Query,
}

impl QueryKind {
    fn try_from_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
        if match_type(cx, ty, &crate::paths::QUERY) {
            Some(Self::Query)
        } else {
            None
        }
    }

    fn help(&self, ty: Ty<'_>) -> String {
        // It should be noted that `With<Foo>` is not always the best filter to suggest.
        // While it's most often going to be what users want, there's also `Added<Foo>`
        // and `Changed<Foo>` which might be more appropriate in some cases
        // (i.e. users are calling `foo.is_added()` or `foo.is_changed()` in the body of
        // the system).
        // In the future, we might want to span the usage site of `is_added()`/`is_changed()`
        // and suggest to use `Added<Foo>`/`Changed<Foo>` instead.
        match self {
            Self::Query => format!(
                // NOTE: This isn't actually true, please see #279 for more info and how this will
                // be fixed!
                "zero-sized types do not retrieve any data, consider using a filter instead: `With<{ty}>`"
            ),
        }
    }
}

/// Checks if a type is zero-sized.
///
/// Returns:
/// - `Some(true)` if the type is most likely a ZST
/// - `Some(false)` if the type is most likely not a ZST
/// - `None` if we cannot determine the size (e.g., type is not normalizable)
fn is_zero_sized<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<bool> {
    // `cx.layout_of()` panics if the type is not normalizable.
    if !is_normalizable(cx, cx.param_env, ty) {
        return None;
    }

    // Note: we don't use `approx_ty_size` from `clippy_utils` here
    // because it will return `0` as the default value if the type is not
    // normalizable, which will put us at risk of emitting more false positives.
    if let Ok(TyAndLayout { layout, .. }) = cx.layout_of(ty) {
        Some(layout.size() == Size::ZERO)
    } else {
        None
    }
}
