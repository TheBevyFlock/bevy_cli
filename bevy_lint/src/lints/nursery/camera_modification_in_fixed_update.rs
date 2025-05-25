use clippy_utils::{diagnostics::span_lint_and_help, sym, ty::match_type};
use rustc_hir::{ExprKind, QPath, def::Res};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Adt, GenericArgKind, TyKind};
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    pub CAMERA_MODIFICATION_IN_FIXED_UPDATE,
    super::Nursery,
    "Camera modification in FixedUpdate schedule",
}

declare_bevy_lint_pass! {
    pub CameraModificationInFixedUpdate => [CAMERA_MODIFICATION_IN_FIXED_UPDATE],
    @default = {
        add_systems: Symbol = sym!(add_systems),
    },
}

impl<'tcx> LateLintPass<'tcx> for CameraModificationInFixedUpdate {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx rustc_hir::Expr<'tcx>) {
        let Some(MethodCall {
            method_path, args, ..
        }) = MethodCall::try_from(cx, expr)
        else {
            return;
        };

        // Check for MethodCalls that match `App::add_systems(impl ScheduleLabel,T)`
        if method_path.ident.name != self.add_systems {
            return;
        }

        // First argument must be the `ScheduleLabel`
        let Some(first_arg) = args.first() else {
            return;
        };

        let schedule_ty = cx.typeck_results().expr_ty(first_arg).peel_refs();

        // If the system is not run during `FIXED_UPDATE` skip checking its fn_sig
        if !match_type(cx, schedule_ty, &crate::paths::FIXED_UPDATE) {
            return;
        }

        // The system being added
        let Some(system_expr) = args.get(1) else {
            return;
        };

        // TODO: handle tuples (system registering, query_filter)

        // Get the function definition of this system
        if let ExprKind::Path(QPath::Resolved(_, path)) = system_expr.kind
            && let Res::Def(_, def_id) = path.res
        {
            // Get the function signature of the system
            let fn_sig = cx.tcx.fn_sig(def_id);
            // TODO: check if there can be multiple
            for ty in fn_sig.skip_binder().inputs().skip_binder() {
                let Adt(adt_def_id, args) = ty.kind() else {
                    continue;
                };

                // check if this structure is of type `Query`
                let adt_ty = cx.tcx.type_of(adt_def_id.did()).skip_binder();

                if !match_type(cx, adt_ty, &crate::paths::QUERY) {
                    continue;
                }

                // Get only the type arguments and ignore Lifetime arguments
                let mut query_type_arguments =
                    args.iter()
                        .filter_map(|generic_arg| match generic_arg.unpack() {
                            GenericArgKind::Type(ty) => Some(ty),
                            _ => None,
                        });

                // Get the generic query data type, if none is present return early.
                let Some(query_data) = query_type_arguments.next() else {
                    return;
                };

                // TODO: idk...
                let query_data_args: Vec<_> = match query_data.kind() {
                    TyKind::Tuple(tys) => tys
                        .iter()
                        .filter_map(|ty| match ty.kind() {
                            TyKind::Ref(_, _, mutability) => Some(mutability),
                            _ => None,
                        })
                        .collect(),
                    TyKind::Adt(_, generic_args) => generic_args
                        .iter()
                        .filter_map(|arg| match arg.unpack() {
                            GenericArgKind::Type(ty) => match ty.kind() {
                                TyKind::Ref(_, _, mutability) => Some(mutability),
                                _ => None,
                            },
                            _ => None,
                        })
                        .collect(),
                    _ => return,
                };

                // iterate over all Filters
                for query_filter in query_type_arguments {
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
                    // At this point we know, that the query_data is related to the Camera
                    // So we check if at least one query_data is borrowed mutably
                    && query_data_args.iter().any(|mutability|match mutability {
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
                            "Put System in Update instead",
                        );
                    }
                }
            }
        }
    }
}
