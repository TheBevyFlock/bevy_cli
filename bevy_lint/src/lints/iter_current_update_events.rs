//! TODO

use clippy_utils::{diagnostics::span_lint, ty::match_type};
use rustc_hir::Expr;
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    pub ITER_CURRENT_UPDATE_EVENTS,
    SUSPICIOUS,
    "called `Events::<T>::iter_current_update_events()`",
}

declare_bevy_lint_pass! {
    pub IterCurrentUpdateEvents => [ITER_CURRENT_UPDATE_EVENTS.lint],

    @default = {
        iter_current_update_events: Symbol = Symbol::intern("iter_current_update_events"),
    },
}

impl<'tcx> LateLintPass<'tcx> for IterCurrentUpdateEvents {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let Some(method_call) = MethodCall::try_from(cx, expr) {
            let src_ty = cx
                .typeck_results()
                .expr_ty(method_call.receiver)
                .peel_refs();

            if !match_type(cx, src_ty, &crate::paths::EVENTS) {
                return;
            }

            if method_call.method_path.ident.name == self.iter_current_update_events {
                span_lint(
                    cx,
                    ITER_CURRENT_UPDATE_EVENTS.lint,
                    method_call.span,
                    ITER_CURRENT_UPDATE_EVENTS.lint.desc,
                );
            }
        }
    }
}
