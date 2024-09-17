//! TODO

use clippy_utils::{diagnostics::span_lint, sym, ty::match_type};
use rustc_hir::{Expr, ExprKind, GenericArg, GenericArgs};
use rustc_hir_analysis::lower_ty;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_tool_lint! {
    pub bevy::INIT_EVENT_RESOURCE,
    Deny,
    "called `App::init_resource::<Events<T>>() instead of `App::add_event::<T>()`"
}

declare_lint_pass! {
    InitEventResource => [INIT_EVENT_RESOURCE]
}

impl<'tcx> LateLintPass<'tcx> for InitEventResource {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
        // Find a method call that matches `.init_resource()`.
        // TODO: Also check for `insert_resource()`.
        if let ExprKind::MethodCall(path, src, _, method_span) = expr.kind
            && path.ident.name == sym!(init_resource)
        {
            // Get the type for `src` in `src.init_resource()`.
            let src_ty = cx.typeck_results().expr_ty(src);

            // Ensure `src` is a Bevy `App`.
            if match_type(cx, src_ty, &crate::paths::APP)
                && let Some(&GenericArgs {
                    args: &[GenericArg::Type(generic_ty)],
                    ..
                }) = path.args
            {
                let generic_ty = dbg!(lower_ty(cx.tcx, generic_ty));

                if match_type(cx, generic_ty, &["bevy_ecs", "event", "Events"]) {
                    span_lint(
                        cx,
                        INIT_EVENT_RESOURCE,
                        method_span,
                        INIT_EVENT_RESOURCE.desc,
                    );
                }
            }
        }
    }
}
