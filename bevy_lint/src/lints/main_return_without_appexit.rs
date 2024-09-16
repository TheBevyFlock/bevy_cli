use std::ops::ControlFlow;
use clippy_utils::{diagnostics::span_lint, is_entrypoint_fn, match_def_path, sym, visitors::for_each_expr};
use rustc_hir::{def_id::LocalDefId, intravisit::FnKind, Body, Expr, ExprKind, FnDecl, FnRetTy};
use rustc_lint::{LateContext, LateLintPass, Level, Lint, LintPass, LintVec};
use rustc_span::Span;

pub static MAIN_RETURN_WITHOUT_APPEXIT: &Lint = &Lint {
    name: "bevy::main_return_without_appexit",
    default_level: Level::Warn,
    desc: "an entrypoint that calls `App::run()` does not return `AppExit`",
    is_externally_loaded: true,
    ..Lint::default_fields_for_macro()
};

#[derive(Clone, Copy, Debug)]
pub struct MainReturnWithoutAppExit;

impl LintPass for MainReturnWithoutAppExit {
    fn name(&self) -> &'static str {
        MAIN_RETURN_WITHOUT_APPEXIT.name
    }
}

impl MainReturnWithoutAppExit {
    pub fn get_lints() -> LintVec {
        vec![MAIN_RETURN_WITHOUT_APPEXIT]
    }
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

        // May not be necessary if `match_ty` is used instead.
        let Some(adt) = ty.ty_adt_def() else {
            return ControlFlow::Continue(());
        };

        // Syms may be incorrect.
        if match_def_path(cx, adt.did(), &["bevy", "prelude", "App"]) {
            span_lint(cx, MAIN_RETURN_WITHOUT_APPEXIT, span, "AAA!!");
        }
    }

    ControlFlow::Continue(())
}
