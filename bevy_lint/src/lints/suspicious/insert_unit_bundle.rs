//! Checks for calls to `Commands::spawn()` that inserts unit [`()`](unit) as a component.
//!
//! # Motivation
//!
//! It is possible to use `Commands::spawn()` to spawn an entity with a unit `()` component, since
//! unit implements `Bundle`. Unit is not a `Component`, however, and will be ignored instead of
//! added to the entity. Often, inserting a unit is unintentional and is a sign that the author
//! intended to do something else.
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! # use std::f32::consts::PI;
//! #
//! fn spawn(mut commands: Commands) {
//!     commands.spawn(());
//!
//!     commands.spawn((
//!         Name::new("Decal"),
//!         // This is likely a mistake! `Transform::rotate_z()` returns a unit `()`, not a
//!         // `Transform`! As such, no `Transform` will be inserted into the entity.
//!         Transform::from_translation(Vec3::new(0.75, 0.0, 0.0))
//!             .rotate_z(PI / 4.0),
//!     ));
//! }
//! #
//! # bevy::ecs::system::assert_is_system(spawn);
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! # use std::f32::consts::PI;
//! #
//! fn spawn(mut commands: Commands) {
//!     // `Commands::spawn_empty()` is preferred if you do not need any components.
//!     commands.spawn_empty();
//!
//!     commands.spawn((
//!         Name::new("Decal"),
//!         // `Transform::with_rotation()` returns a `Transform`, which was likely the intended
//!         // behavior.
//!         Transform::from_translation(Vec3::new(0.75, 0.0, 0.0))
//!             .with_rotation(Quat::from_rotation_z(PI / 4.0)),
//!     ));
//! }
//! #
//! # bevy::ecs::system::assert_is_system(spawn);
//! ```

use clippy_utils::{diagnostics::span_lint, fn_def_id};
use rustc_hir::{Expr, ExprKind, def_id::DefId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{self, Ty};
use rustc_type_ir::PredicatePolarity;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, paths};

declare_bevy_lint! {
    pub(crate) INSERT_UNIT_BUNDLE,
    super::Suspicious,
    "inserted a `Bundle` containing a unit `()` type",
}

declare_bevy_lint_pass! {
    pub(crate) InsertUnitBundle => [INSERT_UNIT_BUNDLE],
}

impl<'tcx> LateLintPass<'tcx> for InsertUnitBundle {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }

        let (fn_id, fn_args, fn_arg_types) = match expr.kind {
            ExprKind::Call(_, fn_args) => {
                let Some(fn_id) = fn_def_id(cx, expr) else {
                    // This will be `None` if the function is a local closure. Since closures
                    // cannot have generic parameters, they cannot take bundles as an input, so we
                    // can skip them.
                    return;
                };

                let fn_arg_types = fn_arg_types(cx, fn_id);

                (fn_id, fn_args, fn_arg_types)
            }
            ExprKind::MethodCall(_, _, fn_args, _) => {
                let Some(fn_id) = fn_def_id(cx, expr) else {
                    // See comment for `ExprKind::Call` branch for why we return here.
                    return;
                };

                // The first argument is `&self` because it's a method. We skip it because `&self`
                // won't be in `args`, making the two slices two different lengths.
                let fn_arg_types = &fn_arg_types(cx, fn_id)[1..];

                (fn_id, fn_args, fn_arg_types)
            }
            _ => return,
        };

        debug_assert_eq!(fn_args.len(), fn_arg_types.len());

        let typeck_results = cx.typeck_results();

        for bundle_expr in filter_bundle_args(cx, fn_id, fn_args, fn_arg_types) {
            let bundle_ty = typeck_results.expr_ty(bundle_expr);

            for tuple_path in find_units_in_tuple(bundle_ty) {
                let unit_expr = tuple_path.into_expr(bundle_expr);

                span_lint(
                    cx,
                    INSERT_UNIT_BUNDLE,
                    unit_expr.span,
                    INSERT_UNIT_BUNDLE.desc,
                );
            }
        }
    }
}

/// Returns the arguments of a method call that are intended to be `Bundle`s.
///
/// `fn_id` should be the definition of the function itself, and `args` should be the arguments
/// passed to the function.
fn filter_bundle_args<'tcx>(
    cx: &LateContext<'tcx>,
    fn_id: DefId,
    fn_args: &'tcx [Expr<'tcx>],
    fn_arg_types: &[Ty<'tcx>],
) -> impl Iterator<Item = &'tcx Expr<'tcx>> {
    let bundle_bounded_generics: Vec<Ty<'_>> = bundle_bounded_generics(cx, fn_id);

    // Only yield arguments whose types are generic parameters that require the `Bundle` trait.
    fn_arg_types
        .iter()
        .enumerate()
        .filter(move |(_, arg)| bundle_bounded_generics.contains(arg))
        .map(|(i, _)| &fn_args[i])
}

/// Returns a list of types corresponding to the inputs of a function.
///
/// Notably, the returned types are not instantiated. Generic parameters will be preserved and not
/// filled in with actual types.
///
/// # Example
///
/// Running this function on the [`DefId`] of `foo()` will return `[usize, bool]`, while `bar()`
/// will return `[T, usize]`.
///
/// ```
/// fn foo(a: usize, b: bool) {}
/// fn bar<T: Bundle>(bundle: T, size: usize) {}
/// ```
fn fn_arg_types<'tcx>(cx: &LateContext<'tcx>, fn_id: DefId) -> &'tcx [Ty<'tcx>] {
    cx.tcx
        .fn_sig(fn_id)
        .instantiate_identity()
        .inputs()
        .skip_binder()
}

/// Returns a list of a generic parameters of a function that must implement `Bundle`.
///
/// Each returned [`Ty`] is guaranteed to be a [`ty::TyKind::Param`].
///
/// # Example
///
/// If run on the following function, this function would return `A` and `C` because they both
/// implement `Bundle`.
///
/// ```
/// fn my_function<A: Bundle, B: Clone, C: Bundle + Clone>(_: A, _: B, _: C) {
///     // ...
/// }
/// ```
fn bundle_bounded_generics<'tcx>(cx: &LateContext<'tcx>, fn_id: DefId) -> Vec<Ty<'tcx>> {
    let mut bundle_bounded_generics = Vec::new();

    // Fetch the parameter environment for the function, which contains all generic trait bounds.
    // (Such as the `T: Bundle` that we're looking for!) See
    // <https://rustc-dev-guide.rust-lang.org/typing_parameter_envs.html> for more information.
    let param_env = cx.tcx.param_env(fn_id);

    for clause in param_env.caller_bounds() {
        // We only want trait predicates, filtering out lifetimes and constants.
        if let Some(trait_predicate) = clause.as_trait_clause()
            // The `Bundle` trait doesn't require any bound vars, so we dispel the binder.
            && let Some(trait_predicate) = trait_predicate.no_bound_vars()
            && let ty::TraitPredicate {
                trait_ref,
                // Negative trait bounds, which are unstable, allow matching all types _except_
                // those with a specific trait. We don't want that, however, so we only match
                // positive trait bounds.
                polarity: PredicatePolarity::Positive,
            } = trait_predicate
            // Only match `T: Bundle` predicates.
            && paths::BUNDLE.matches(cx, trait_ref.def_id)
        {
            let self_ty = trait_ref.self_ty();

            debug_assert!(
                matches!(self_ty.kind(), ty::TyKind::Param(_)),
                "type from trait bound was expected to be a type parameter",
            );

            // At this point, we've confirmed the predicate is `T: Bundle`! Add it to the list to
            // be returned. :)
            bundle_bounded_generics.push(trait_ref.self_ty());
        }
    }

    bundle_bounded_generics
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
#[derive(Clone, Debug)]
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
        if let ty::TyKind::Tuple(types) = ty.kind() {
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
