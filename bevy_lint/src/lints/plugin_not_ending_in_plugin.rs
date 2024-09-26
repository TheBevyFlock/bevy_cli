//! TODO

use crate::declare_bevy_lint;
use clippy_utils::{diagnostics::span_lint, match_def_path};
use rustc_hir::{def::Res, Item, ItemKind, QPath, TyKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;

declare_bevy_lint! {
    pub PLUGIN_NOT_ENDING_IN_PLUGIN,
    STYLE,
    "implemented `Plugin` for a structure whose name does not end in \"Plugin\"",
}

declare_lint_pass! {
    PluginNotEndingInPlugin => [PLUGIN_NOT_ENDING_IN_PLUGIN.lint]
}

impl<'tcx> LateLintPass<'tcx> for PluginNotEndingInPlugin {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &Item<'tcx>) {
        if let ItemKind::Impl(impl_) = item.kind
            && let Some(of_trait) = impl_.of_trait
            && let Res::Def(_, def_id) = of_trait.path.res
            && match_def_path(cx, def_id, &crate::paths::PLUGIN)
            && let TyKind::Path(QPath::Resolved(_, self_path)) = impl_.self_ty.kind
        {
            let Some(self_name) = self_path.segments.last() else {
                return;
            };

            if self_name.ident.as_str().ends_with("Plugin") {
                return;
            }

            span_lint(
                cx,
                PLUGIN_NOT_ENDING_IN_PLUGIN.lint,
                self_path.span,
                PLUGIN_NOT_ENDING_IN_PLUGIN.lint.desc,
            );
        }
    }
}
