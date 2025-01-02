use std::ops::ControlFlow;

use clippy_utils::{diagnostics::span_lint, sym, ty::match_type, visitors::for_each_expr};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;
use rustc_span::Symbol;

use crate::declare_bevy_lint;

declare_bevy_lint! {
    pub INSERT_EMPTY_BUNDLE,
    SUSPICIOUS,
    "method returns `()` and will spawn an empty bundle",
}

impl_lint_pass! {
    InsertEmptyBundle => [INSERT_EMPTY_BUNDLE.lint]
}

pub struct InsertEmptyBundle {
    /// A cached [`Symbol`] representing the interned string `"spawn"`.
    spawn_symbol: Symbol,
}

impl Default for InsertEmptyBundle {
    fn default() -> Self {
        Self {
            spawn_symbol: sym!(spawn),
        }
    }
}

impl<'tcx> LateLintPass<'tcx> for InsertEmptyBundle {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // Find a method call.
        let ExprKind::MethodCall(path, src, args, _method_span) = expr.kind else {
            return;
        };

        let src_ty = cx.typeck_results().expr_ty(src).peel_refs();

        // If the method call was not to `commands.spawn` we skip it.
        if !(match_type(cx, src_ty, &crate::paths::COMMANDS)
            && path.ident.name == self.spawn_symbol)
        {
            return;
        }

        // iterate through all `Expr` inside the method `args` tuple, check if any return `()`
        for_each_expr(cx, args, |expr| {
            // get the expression of the Call
            let (expr, span) = match expr.kind {
                ExprKind::Call(expr, _) => (expr, expr.span),
                ExprKind::MethodCall(path, _, _, _) => (expr, path.ident.span),
                // If the expression was not of `kind` `Call` or `MethodCall`,
                // continue to the next Expression
                _ => return ControlFlow::<()>::Continue(()),
            };

            let ty = cx.typeck_results().type_dependent_def_id(expr.hir_id);

            if let Some(ty) = ty {
                // Check if the return type of a function signature is of type `unit`
                if cx
                    .tcx
                    .fn_sig(ty)
                    .skip_binder()
                    .output()
                    .skip_binder()
                    .is_unit()
                {
                    span_lint(
                        cx,
                        INSERT_EMPTY_BUNDLE.lint,
                        span,
                        "Expression returns `()` and results in an empty bundle being inserted",
                    );
                }
            }

            ControlFlow::<()>::Continue(())
        });
    }
}
