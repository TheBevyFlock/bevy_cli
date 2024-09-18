//! TODO

use clippy_utils::{diagnostics::span_lint, sym, ty::match_type};
use rustc_hir::{Expr, ExprKind, GenericArg, GenericArgs, PathSegment};
use rustc_hir_analysis::lower_ty;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::Span;

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
        // Find a method call.
        if let ExprKind::MethodCall(path, src, args, method_span) = expr.kind {
            // Get the type for `src` in `src.method()`.
            let src_ty = cx.typeck_results().expr_ty(src);

            // If `src` is not a Bevy `App`, exit.
            if !match_type(cx, src_ty, &crate::paths::APP) {
                return;
            }

            // If the method is `App::init_resource()` or `App::insert_resource()`, check it with
            // its corresponding function.
            match path.ident.name {
                symbol if symbol == sym!(init_resource) => {
                    check_init_resource(cx, path, method_span)
                }
                symbol if symbol == sym!(insert_resource) => {
                    check_insert_resource(cx, args, method_span)
                }
                _ => {}
            }
        }
    }
}

fn check_init_resource<'tcx>(cx: &LateContext<'tcx>, path: &PathSegment<'tcx>, method_span: Span) {
    if let Some(&GenericArgs {
        // `App::init_resource()` has one generic type argument: T.
        args: &[GenericArg::Type(generic_ty)],
        ..
    }) = path.args
    {
        // Lower `rustc_hir::Ty` to `ty::Ty`, so we can inspect type information. For more
        // information, see <https://rustc-dev-guide.rust-lang.org/ty.html#rustc_hirty-vs-tyty>.
        // Note that `lower_ty()` is quasi-deprecated, and should be removed if a adequate
        // replacement is found.
        let generic_ty = lower_ty(cx.tcx, generic_ty);

        // If the generic argument is `Events<T>`, emit the lint.
        if match_type(cx, generic_ty, &crate::paths::EVENTS) {
            span_lint(
                cx,
                INIT_EVENT_RESOURCE,
                method_span,
                INIT_EVENT_RESOURCE.desc,
            );
        }
    }
}

fn check_insert_resource<'tcx>(cx: &LateContext<'tcx>, args: &[Expr], method_span: Span) {
    // Extract the argument if there is only 1 (which there should be!), else exit.
    let [arg] = args else {
        return;
    };

    // Find the type of `arg` in `App::insert_resource(arg)`.
    let ty = cx.typeck_results().expr_ty(arg);

    if match_type(cx, ty, &crate::paths::EVENTS) {
        span_lint(
            cx,
            INIT_EVENT_RESOURCE,
            method_span,
            INIT_EVENT_RESOURCE.desc,
        );
    }
}
