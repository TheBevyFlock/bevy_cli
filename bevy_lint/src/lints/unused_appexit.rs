//! TODO

use rustc_hir::Expr;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;

use crate::declare_bevy_lint;

declare_bevy_lint! {
    pub UNUSED_APPEXIT,
    PEDANTIC,
    "called `App::run()` without handling the returned `AppExit`",
}

declare_lint_pass! {
    UnusedAppExit => [UNUSED_APPEXIT.lint]
}

impl<'tcx> LateLintPass<'tcx> for UnusedAppExit {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
        todo!()
    }
}
