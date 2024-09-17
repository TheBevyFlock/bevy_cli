use clippy_utils::{
    diagnostics::span_lint, is_entrypoint_fn, sym, ty::match_type, visitors::for_each_expr,
};
use rustc_hir::{def_id::LocalDefId, intravisit::FnKind, Body, Expr, ExprKind, FnDecl, FnRetTy};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::Span;
use std::ops::ControlFlow;

declare_tool_lint! {
    pub bevy::MAIN_RETURN_WITHOUT_APPEXIT,
    Warn,
    "an entrypoint that calls `App::run()` does not return `AppExit`"
}

declare_lint_pass! {
    MainReturnWithoutAppExit => [MAIN_RETURN_WITHOUT_APPEXIT]
}

impl<'tcx> LateLintPass<'tcx> for MainReturnWithoutAppExit {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        declaration: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        local_def_id: LocalDefId,
    ) {
        // We're looking for `fn main()` with no return type that calls `App::run()`.
        if is_entrypoint_fn(cx, local_def_id.into())
            && let FnRetTy::DefaultReturn(_) = declaration.output
        {
            // Iterate over each expression within the entrypoint function, finding and reporting
            // `App::run()` calls.
            for_each_expr(cx, body, |expr| find_app_run_call(cx, expr));
        }
    }
}

fn find_app_run_call<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> ControlFlow<()> {
    // Find a method call that matches `.run()`.
    if let ExprKind::MethodCall(path, src, _, span) = expr.kind
        && path.ident.name == sym!(run)
    {
        // Get the type of `src` for `src.run()`.
        let ty = cx.typeck_results().expr_ty(src);

        // If `src` is a Bevy `App`, emit the lint.
        if match_type(cx, ty, &["bevy_app", "app", "App"]) {
            span_lint(
                cx,
                MAIN_RETURN_WITHOUT_APPEXIT,
                span,
                MAIN_RETURN_WITHOUT_APPEXIT.desc,
            );
        }
    }

    ControlFlow::Continue(())
}
