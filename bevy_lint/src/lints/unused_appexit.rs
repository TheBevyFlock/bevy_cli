//! TODO

use clippy_utils::{diagnostics::span_lint, is_expr_used_or_unified, sym, ty::match_type};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;
use rustc_span::Symbol;

use crate::declare_bevy_lint;

declare_bevy_lint! {
    pub UNUSED_APPEXIT,
    PEDANTIC,
    "called `App::run()` without handling the returned `AppExit`",
}

#[derive(Clone, Copy)]
pub struct UnusedAppExit {
    /// A cached [`Symbol`] representing the interned string `"run"`.
    ///
    /// This is used to avoid interning or comparing strings for every single method call in a
    /// crate.
    run_symbol: Symbol,
}

impl Default for UnusedAppExit {
    fn default() -> Self {
        Self {
            run_symbol: sym!(run),
        }
    }
}

impl_lint_pass! {
    UnusedAppExit => [UNUSED_APPEXIT.lint]
}

impl<'tcx> LateLintPass<'tcx> for UnusedAppExit {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
        // Find a method call that matches `.run()`.
        if let ExprKind::MethodCall(path, src, _, method_span) = expr.kind
            && path.ident.name == self.run_symbol
        {
            // Get the type of `src` for `src.run()`. We peel away all references because
            // both `App` and `&mut App` are allowed.
            let ty = cx.typeck_results().expr_ty(src).peel_refs();

            // If `src` is a Bevy `App` and the returned `AppExit` is not used, emit the lint.
            if match_type(cx, ty, &crate::paths::APP) && !is_expr_used_or_unified(cx.tcx, expr) {
                span_lint(
                    cx,
                    UNUSED_APPEXIT.lint,
                    method_span,
                    UNUSED_APPEXIT.lint.desc,
                );
            }
        }
    }
}
