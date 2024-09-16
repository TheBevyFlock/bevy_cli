use clippy_utils::{diagnostics::span_lint, is_entrypoint_fn};
use rustc_hir::{def_id::LocalDefId, intravisit::FnKind, Body, FnDecl, FnRetTy};
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
        _: &'tcx Body<'tcx>,
        _: Span,
        local_def_id: LocalDefId,
    ) {
        // We're looking for `fn main()` with no return type that calls `App::run()`.
        if is_entrypoint_fn(cx, local_def_id.into())
            && let FnRetTy::DefaultReturn(return_span) = declaration.output {
            span_lint(cx, MAIN_RETURN_WITHOUT_APPEXIT, return_span, "AAAA!");
        }
    }
}
