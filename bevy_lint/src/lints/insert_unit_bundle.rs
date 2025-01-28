//! Checks for a call to `Commands::spawn()` that inserts unit `()` as a component.
//!
//! # Motivation
//!
//! It is possible to use `Commands::spawn()` to spawn an entity with a unit `()` component, since
//! unit implements `Bundle`. Unit is not a `Component`, however, and will be ignored instead of
//! added to the entity.
//!
//! Trying to spawn an entity with unit is discouraged because, not only does it not do anything,
//! but it can lead to misleading code.
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! # use std::f32::consts::PI;
//! #
//! fn spawn_new(mut commands: Commands) {
//!     commands.spawn(());
//! }
//!
//! fn spawn_decal(mut commands: Commands) {
//!     commands.spawn((
//!         Name::new("Decal"),
//!         // This is misleading! `Transform::rotate_z()` returns a unit `()`, not a `Transform`!
//!         // No `Transform` will be inserted into the entity with this!
//!         Transform::from_translation(Vec3::new(0.75, 0.0, 0.0)).rotate_z(PI / 4.0),
//!     ));
//! }
//! #
//! # bevy::ecs::system::assert_is_system(spawn_new);
//! # bevy::ecs::system::assert_is_system(spawn_decal);
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! # use std::f32::consts::PI;
//! #
//! fn spawn_new(mut commands: Commands) {
//!     // `Commands::spawn_empty()` is preferred if you do not add any components.
//!     commands.spawn_empty();
//! }
//!
//! fn spawn_decal(mut commands: Commands) {
//!     commands.spawn((
//!         Name::new("Decal"),
//!         // `Transform::with_rotation()` returns a `Transform`, so it will be inserted into the
//!         // entity.
//!         Transform::from_translation(Vec3::new(0.75, 0.0, 0.0))
//!             .with_rotation(Quat::from_rotation_z(PI / 4.0)),
//!     ));
//! }
//! #
//! # bevy::ecs::system::assert_is_system(spawn_new);
//! # bevy::ecs::system::assert_is_system(spawn_decal);
//! ```

use clippy_utils::{diagnostics::span_lint_hir_and_then, sym, ty::match_type};
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Ty, TyKind};
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass};

declare_bevy_lint! {
    pub INSERT_UNIT_BUNDLE,
    SUSPICIOUS,
    "inserted a `Bundle` containing a unit `()` type",
}

declare_bevy_lint_pass! {
    pub InsertUnitBundle => [INSERT_UNIT_BUNDLE.lint],
    @default = {
        spawn: Symbol = sym!(spawn),
    },
}

impl<'tcx> LateLintPass<'tcx> for InsertUnitBundle {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // Find a method call.
        let ExprKind::MethodCall(path, src, args, method_span) = expr.kind else {
            return;
        };

        let src_ty = cx.typeck_results().expr_ty(src).peel_refs();

        // If the method call was not to `Commands::spawn()` we skip it.
        if !(match_type(cx, src_ty, &crate::paths::COMMANDS) && path.ident.name == self.spawn) {
            return;
        }

        // Extract the expression of the bundle being spawned.
        let [bundle_expr] = args else {
            return;
        };

        // Find the type of the bundle.
        let bundle_ty = cx.typeck_results().expr_ty(bundle_expr);

        // Special-case `commands.spawn(())` and suggest `Commands::spawn_empty()` instead.
        if bundle_ty.is_unit() {
            span_lint_hir_and_then(
                cx,
                INSERT_UNIT_BUNDLE.lint,
                bundle_expr.hir_id,
                bundle_expr.span,
                INSERT_UNIT_BUNDLE.lint.desc,
                |diag| {
                    diag.note("unit `()` types are skipped instead of spawned")
                        .span_suggestion(
                            method_span,
                            "try",
                            "spawn_empty()",
                            Applicability::MachineApplicable,
                        );
                },
            );

            return;
        }

        // Find the path to all units within the bundle type.
        let unit_paths = find_units_in_tuple(bundle_ty);

        // Emit the lint for all unit tuple paths.
        for path in unit_paths {
            let expr = path.into_expr(bundle_expr);

            span_lint_hir_and_then(
                cx,
                INSERT_UNIT_BUNDLE.lint,
                expr.hir_id,
                expr.span,
                INSERT_UNIT_BUNDLE.lint.desc,
                |diag| {
                    diag.note("unit `()` types are skipped instead of spawned");
                },
            );
        }
    }
}

/// Represents the path to an item within a nested tuple.
///
/// # Example
///
/// Each number within the [`TuplePath`] represents an index into the tuple. An empty path
/// represents the root tuple, while a path of `TuplePath([0])` represents the first item within
/// that tuple.
///
/// ```ignore
/// // TuplePath([])
/// (
///     // TuplePath([0])
///     Name::new("Foo"),
///     // TuplePath([1])
///     (
///         // TuplePath([1, 0])
///         (),
///         // TuplePath([1, 1])
///         Transform::default(),
///         // TuplePath([1, 2])
///         Visibility::Hidden,
///     ),
///     // TuplePath([2])
///     (),
/// )
/// ```
#[derive(Clone)]
#[repr(transparent)]
struct TuplePath(Vec<usize>);

impl TuplePath {
    /// Creates an empty [`TuplePath`].
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Pushes an index to the end of the path.
    fn push(&mut self, i: usize) {
        self.0.push(i);
    }

    /// Pops the last index in the path.
    fn pop(&mut self) -> Option<usize> {
        self.0.pop()
    }

    /// Finds the [`Expr`] of the item represented by this path given the root tuple.
    ///
    /// In the event the path is invalid in some way (such as if an expected tuple is not found),
    /// this will return the expression closest to the target.
    fn into_expr<'tcx>(self, root_tuple: &'tcx Expr<'tcx>) -> &'tcx Expr<'tcx> {
        let mut tuple = root_tuple;

        for i in self.0 {
            let ExprKind::Tup(items) = tuple.kind else {
                // If the path is invalid in some way, return the expression nearest to the target.
                // This is usually the case when the bundle is created outside of
                // `Commands::spawn()`, such as with `commands.spawn(my_helper())` instead of the
                // expected `commands.spawn((Foo, Bar, ()))`.
                return tuple;
            };

            tuple = &items[i];
        }

        tuple
    }
}

/// Returns the [`TuplePath`]s to all unit types within a tuple type.
///
/// # Example
///
/// Given a type:
///
/// ```ignore
/// type MyBundle = (
///     Name,
///     (
///         (),
///         Transform,
///         Visibility,
///     ),
///     (),
/// );
/// ```
///
/// This function would return:
///
/// ```ignore
/// [
///     TuplePath([1, 0]),
///     TuplePath([2]),
/// ]
/// ```
///
/// See [`TuplePath`]'s documentation for more information.
fn find_units_in_tuple(ty: Ty<'_>) -> Vec<TuplePath> {
    fn inner(ty: Ty<'_>, current_path: &mut TuplePath, unit_paths: &mut Vec<TuplePath>) {
        if let TyKind::Tuple(types) = ty.kind() {
            if types.is_empty() {
                unit_paths.push(current_path.clone());
                return;
            }

            for (i, ty) in types.into_iter().enumerate() {
                current_path.push(i);
                inner(ty, current_path, unit_paths);
                current_path.pop();
            }
        }
    }

    let mut current_path = TuplePath::new();
    let mut unit_paths = Vec::new();

    inner(ty, &mut current_path, &mut unit_paths);

    unit_paths
}
