//! Checks for `fn main()` entrypoints that call `App::run()` but do not return `AppExit`.
//!
//! This lint will not be emitted if `fn main()` returns a non-[`unit`] type, even if that type is
//! not `AppExit`.
//!
//! # Why is this bad?
//!
//! `AppExit` is used to determine whether the `App` exited successful or due to an error. Returning
//! it from `main()` will set the exit code, which allows external processes to detect whether there
//! was an error.
//!
//! # Example
//!
//! ```rust
//! # use bevy::prelude::*;
//! #
//! fn main() {
//!     App::new().run();
//! }
//! ```
//!
//! Use instead:
//!
//! ```rust
//! # use bevy::prelude::*;
//! #
//! fn main() -> AppExit {
//!     // Note the removed semicolon.
//!     App::new().run()
//! }
//! ```

use clippy_utils::{
    diagnostics::span_lint_and_then, is_entrypoint_fn, sym, ty::match_type, visitors::for_each_expr,
};
use rustc_errors::Applicability;
use rustc_hir::{
    def_id::LocalDefId, intravisit::FnKind, Body, ExprKind, FnDecl, FnRetTy, Ty, TyKind,
};
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
        // Look for `fn main()`.
        if is_entrypoint_fn(cx, local_def_id.into())
            // Ensure the function either returns nothing or the unit type. If the entrypoint
            // returns something else, we're assuming that the user knows what they're doing.
            && match declaration.output {
                // The function signature is the default `fn main()`.
                FnRetTy::DefaultReturn(_) => true,
                // The function signature is `fn main() -> ()`.
                FnRetTy::Return(&Ty { kind: TyKind::Tup(&[]), .. }) => true,
                _ => false,
            }
        {
            // Iterate over each expression within the entrypoint function, finding and reporting
            // `App::run()` calls.
            for_each_expr(cx, body, |expr| {
                // Find a method call that matches `.run()`.
                if let ExprKind::MethodCall(path, src, _, method_span) = expr.kind
                    && path.ident.name == sym!(run)
                {
                    // Get the type of `src` for `src.run()`. We peel away all references because
                    // both `App` and `&mut App` are allowed.
                    let ty = cx.typeck_results().expr_ty(src).peel_refs();

                    // If `src` is a Bevy `App`, emit the lint.
                    if match_type(cx, ty, &crate::paths::APP) {
                        span_lint_and_then(
                            cx,
                            MAIN_RETURN_WITHOUT_APPEXIT,
                            method_span,
                            MAIN_RETURN_WITHOUT_APPEXIT.desc,
                            |diag| {
                                diag.note("`App::run()` returns `AppExit`, which can be used to determine whether the app exited successfully or not");
                                match declaration.output {
                                    // When it is just `fn main()`, we need to suggest the `->`.
                                    FnRetTy::DefaultReturn(fn_return_span) => diag.span_suggestion(
                                        fn_return_span,
                                        "try",
                                        " -> AppExit",
                                        Applicability::MaybeIncorrect,
                                    ),
                                    // When it is `fn main() -> ()`, we just need to override `()`.
                                    FnRetTy::Return(&Ty {
                                        span: fn_return_span,
                                        ..
                                    }) => diag.span_suggestion(
                                        fn_return_span,
                                        "try",
                                        "AppExit",
                                        Applicability::MaybeIncorrect,
                                    ),
                                };
                            },
                        );
                    }
                }

                ControlFlow::<()>::Continue(())
            });
        }
    }
}
