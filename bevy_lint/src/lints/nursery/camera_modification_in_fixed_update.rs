use clippy_utils::{sym, ty::match_type};
use rustc_hir::{ExprKind, QPath, def::Res};
use rustc_lint::{LateContext, LateLintPass};
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
        // Check for MethodCalls that match `App::add_systems(FIXED_UPDATE,T)`
        if let Some(MethodCall {
            method_path,
            args,
            ..
        }) = MethodCall::try_from(cx, expr)
            // MethodCall's identifier is `add_systems`
            && method_path.ident.name == self.add_systems
            //check if the first argument is the `FIXED_UPDATE` schedule
            && args.first().is_some_and(|arg| {
                let ty = cx.typeck_results().expr_ty(arg).peel_refs();
                match_type(cx, ty, &crate::paths::FIXED_UPDATE)
            })
        {
            // get the second argument
            let Some(systems) = args.get(1) else {
                // We should always have one or more systems
                return;
            };
            // TODO: handle tuples
            if let ExprKind::Path(QPath::Resolved(_, path)) = systems.kind {
                if let Res::Def(_, def_id) = path.res {
                    let ty = cx.tcx.fn_sig(def_id).skip_binder().inputs().skip_binder();
                }
            }
        }
    }
}
