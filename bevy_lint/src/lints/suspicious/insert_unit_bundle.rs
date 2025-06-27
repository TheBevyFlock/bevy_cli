use clippy_utils::{diagnostics::span_lint_hir_and_then, sym, ty::match_type};
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Ty, TyKind};
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    /// Checks for calls to `Commands::spawn()` that inserts unit [`()`](unit) as a component.
    ///
    /// # Motivation
    ///
    /// It is possible to use `Commands::spawn()` to spawn an entity with a unit `()` component,
    /// since unit implements `Bundle`. Unit is not a `Component`, however, and will be ignored
    /// instead of added to the entity. Often, inserting a unit is unintentional and is a sign that
    /// the author intended to do something else.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use std::f32::consts::PI;
    /// #
    /// fn spawn(mut commands: Commands) {
    ///     commands.spawn(());
    ///
    ///     commands.spawn((
    ///         Name::new("Decal"),
    ///         // This is likely a mistake! `Transform::rotate_z()` returns a unit `()`, not a
    ///         // `Transform`! As such, no `Transform` will be inserted into the entity.
    ///         Transform::from_translation(Vec3::new(0.75, 0.0, 0.0))
    ///             .rotate_z(PI / 4.0),
    ///     ));
    /// }
    /// #
    /// # bevy::ecs::system::assert_is_system(spawn);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use std::f32::consts::PI;
    /// #
    /// fn spawn(mut commands: Commands) {
    ///     // `Commands::spawn_empty()` is preferred if you do not need any components.
    ///     commands.spawn_empty();
    ///
    ///     commands.spawn((
    ///         Name::new("Decal"),
    ///         // `Transform::with_rotation()` returns a `Transform`, which was likely the
    ///         // intended behavior.
    ///         Transform::from_translation(Vec3::new(0.75, 0.0, 0.0))
    ///             .with_rotation(Quat::from_rotation_z(PI / 4.0)),
    ///     ));
    /// }
    /// #
    /// # bevy::ecs::system::assert_is_system(spawn);
    /// ```
    pub INSERT_UNIT_BUNDLE,
    super::Suspicious,
    "inserted a `Bundle` containing a unit `()` type",
}

declare_bevy_lint_pass! {
    pub(crate) InsertUnitBundle => [INSERT_UNIT_BUNDLE],
    @default = {
        spawn: Symbol = sym!(spawn),
    },
}

impl<'tcx> LateLintPass<'tcx> for InsertUnitBundle {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // Find a method call.
        let Some(MethodCall {
            span,
            method_path,
            args,
            receiver,
            ..
        }) = MethodCall::try_from(cx, expr)
        else {
            return;
        };

        let src_ty = cx.typeck_results().expr_ty(receiver).peel_refs();

        // If the method call was not to `Commands::spawn()` or originates from an external macro,
        // we skip it.
        if !(span.in_external_macro(cx.tcx.sess.source_map())
            || match_type(cx, src_ty, &crate::paths::COMMANDS)
                && method_path.ident.name == self.spawn)
        {
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
                INSERT_UNIT_BUNDLE,
                bundle_expr.hir_id,
                bundle_expr.span,
                INSERT_UNIT_BUNDLE.desc,
                |diag| {
                    diag.note("unit `()` types are skipped instead of spawned")
                        .span_suggestion(
                            span,
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
                INSERT_UNIT_BUNDLE,
                expr.hir_id,
                expr.span,
                INSERT_UNIT_BUNDLE.desc,
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
