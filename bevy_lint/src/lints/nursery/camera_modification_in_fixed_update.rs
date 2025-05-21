use clippy_utils::{sym, ty::match_type};
use rustc_hir::{ExprKind, QPath, def::Res};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Adt, GenericArgKind};
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    pub CAMERA_MODIFICATION_IN_FIXED_UPDATE,
    super::Nursery,
    "queried a zero-sized type",
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

        // TODO: handle tuples

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

                // search for type `Camera` in generic arguments to the `Query`
                for arg in args.iter() {
                    if let GenericArgKind::Type(inner_ty) = arg.unpack() {
                        dbg!(inner_ty);
                    }
                }
            }
        }
    }
}
