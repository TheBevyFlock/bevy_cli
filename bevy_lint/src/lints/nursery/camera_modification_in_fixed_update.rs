//! Checks for systems added to the `FixedUpdate` schedule that mutably query entities with a
//! `Camera` component.
//!
//! # Motivation
//!
//! Modifying the camera in `FixedUpdate` can cause jittery, inconsistent, or laggy visuals because
//! `FixedUpdate` may not run every render frame, especially on games with a high FPS.
//!
//! # Known Issues
//!
//! This lint only detects systems that explicitly use the `With<Camera>` query filter.
//!
//! # Example
//!
//! ```rust
//! # use bevy::prelude::*;
//! #
//! fn move_camera(mut query: Query<&mut Transform, With<Camera>>) {
//!     // ...
//! }
//!
//! fn main() {
//!     App::new()
//!         // Uh oh! This could cause issues because the camera may not move every frame!
//!         .add_systems(FixedUpdate, move_camera);
//! }
//! ```
//!
//! Use instead:
//!
//! ```rust
//! # use bevy::prelude::*;
//! #
//! fn move_camera(mut query: Query<&mut Transform, With<Camera>>) {
//!     // ...
//! }
//!
//! fn main() {
//!     App::new()
//!         // Much better. This will run every frame.
//!         .add_systems(Update, move_camera);
//! }
//! ```
//!
//! Any system that modifies the camera in a user-visible way should be run every render frame. The
//! `Update` schedule is a good choice for this, but it notably runs _after_ `FixedUpdate`. You can
//! use the `RunFixedMainLoop` schedule with the `RunFixedMainLoopSystem::BeforeFixedMainLoop`
//! system set to run a system before `FixedUpdate`:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>) {
//!     // ...
//! }
//!
//! fn main() {
//!     App::new()
//!         // In 3D games it is common for the player to move in the direction of the camera.
//!         // Because of this, we must rotate the camera before running the physics logic in
//!         // `FixedUpdate`. This will still run every render frame, though, so there won't be any
//!         // lag!
//!         .add_systems(
//!             RunFixedMainLoop,
//!             rotate_camera.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
//!         );
//! }
//! ```
//!
//! For more information, check out the
//! [physics in fixed timestep example](https://bevy.org/examples/movement/physics-in-fixed-timestep/).

use clippy_utils::{diagnostics::span_lint_and_help, sym, ty::match_type};
use rustc_hir::{ExprKind, QPath, def::Res};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Adt, GenericArgKind, TyKind};
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    pub(crate) CAMERA_MODIFICATION_IN_FIXED_UPDATE,
    super::Nursery,
    "camera modified in the `FixedUpdate` schedule",
}

declare_bevy_lint_pass! {
    pub(crate) CameraModificationInFixedUpdate => [CAMERA_MODIFICATION_IN_FIXED_UPDATE],
    @default = {
        add_systems: Symbol = sym!(add_systems),
    },
}

impl<'tcx> LateLintPass<'tcx> for CameraModificationInFixedUpdate {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if expr.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }

        let Some(MethodCall {
            method_path,
            args,
            receiver,
            ..
        }) = MethodCall::try_from(cx, expr)
        else {
            return;
        };

        let receiver_ty = cx.typeck_results().expr_ty(receiver).peel_refs();

        // Match calls to `App::add_systems(schedule, systems)`
        if !match_type(cx, receiver_ty, &crate::paths::APP)
            || method_path.ident.name != self.add_systems
        {
            return;
        }

        let [schedule, systems] = args else {
            return;
        };

        let schedule_ty = cx.typeck_results().expr_ty(schedule).peel_refs();

        // Skip if the schedule is not `FixedUpdate`
        if !match_type(cx, schedule_ty, &crate::paths::FIXED_UPDATE) {
            return;
        }

        // Collect all added system expressions
        let system_exprs = if let ExprKind::Tup(inner) = systems.kind {
            inner.iter().collect()
        } else {
            vec![systems]
        };

        // Resolve the function definition for each system
        for system_expr in system_exprs {
            if let ExprKind::Path(QPath::Resolved(_, path)) = system_expr.kind
                && let Res::Def(_, def_id) = path.res
            {
                let system_fn_sig = cx.tcx.fn_sig(def_id);
                // Iterate over the function parameter types of the system function
                for ty in system_fn_sig.skip_binder().inputs().skip_binder() {
                    let Adt(adt_def_id, args) = ty.kind() else {
                        continue;
                    };

                    // Check if the parameter is a `Query`
                    let adt_ty = cx.tcx.type_of(adt_def_id.did()).skip_binder();
                    if !match_type(cx, adt_ty, &crate::paths::QUERY) {
                        continue;
                    }

                    // Get the type arguments and ignore Lifetimes
                    let mut query_type_arguments =
                        args.iter()
                            .filter_map(|generic_arg| match generic_arg.unpack() {
                                GenericArgKind::Type(ty) => Some(ty),
                                _ => None,
                            });

                    let Some(query_data) = query_type_arguments.next() else {
                        return;
                    };

                    let Some(query_filters) = query_type_arguments.next() else {
                        return;
                    };

                    // Determine mutability of each queried component
                    let query_data_mutability = match query_data.kind() {
                        TyKind::Tuple(tys) => tys
                            .iter()
                            .filter_map(|ty| match ty.kind() {
                                TyKind::Ref(_, _, mutability) => Some(mutability),
                                _ => None,
                            })
                            .collect(),
                        TyKind::Ref(_, _, mutability) => vec![mutability],
                        _ => return,
                    };

                    // collect all query filters
                    let query_filters = if let TyKind::Tuple(inner) = query_filters.kind() {
                        inner.iter().collect()
                    } else {
                        vec![query_filters]
                    };

                    // Check for `With<Camera>` filter on a mutable query
                    for query_filter in query_filters {
                        // Check if the `With` `QueryFilter` was used.
                        if match_type(cx, query_filter, &crate::paths::WITH)
                        // Get the generic argument of the Filter
                        && let TyKind::Adt(_, with_args) = query_filter.kind()
                        // There can only be exactly one argument
                        && let Some(filter_component_arg) = with_args.iter().next()
                        // Get the type of the component the filter should filter for
                        && let GenericArgKind::Type(filter_component_ty) =
                            filter_component_arg.unpack()
                        // Check if Filter is of type `Camera`
                        && match_type(cx, filter_component_ty, &crate::paths::CAMERA)
                        // Emit lint if any `Camera` component is mutably borrowed
                        && query_data_mutability.iter().any(|mutability|match mutability {
                                rustc_ast::Mutability::Not => false,
                                rustc_ast::Mutability::Mut => true,
                            })
                        {
                            span_lint_and_help(
                                cx,
                                CAMERA_MODIFICATION_IN_FIXED_UPDATE,
                                path.span,
                                CAMERA_MODIFICATION_IN_FIXED_UPDATE.desc,
                                None,
                                "insert the system in the `Update` schedule instead",
                            );
                        }
                    }
                }
            }
        }
    }
}
