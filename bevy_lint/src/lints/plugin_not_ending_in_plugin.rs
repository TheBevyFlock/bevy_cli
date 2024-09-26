//! TODO

use crate::declare_bevy_lint;
use clippy_utils::{diagnostics::span_lint, match_def_path};
use rustc_hir::{def::Res, Item, ItemKind, Path, QPath, Ty, TyKind};
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
        // Find `impl T` items...
        if let ItemKind::Impl(impl_) = item.kind
            // ...that implement a trait for `T`...
            && let Some(of_trait) = impl_.of_trait
            // ...where the trait is a path to user code... (I don't believe this will ever be
            // false, since the alternatives are primitives, `Self`, and others that wouldn't make
            // since in this scenario.)
            && let Res::Def(_, def_id) = of_trait.path.res
            // ...where the trait being implemented is Bevy's `Plugin`...
            && match_def_path(cx, def_id, &crate::paths::PLUGIN)
            // ...and the type `Plugin` is being implemented for is a path to a user-defined type.
            // This purposefully evaluates as false for references, since implementing `Plugin` for
            // them is useless due to `Plugin`'s `'static` requirement. The other kinds of types,
            // such as lang items and primitives, are also skipped because they cannot be easily
            // renamed.
            && let TyKind::Path(QPath::Resolved(_, self_path)) = impl_.self_ty.kind
        {
            // Find the last segment of the path, such as `Foo` for `bar::baz::Foo`. This is
            // considered the name of the type.
            let Some(self_name) = self_path.segments.last() else {
                return;
            };

            // If the name ends with "Plugin", do not emit a lint.
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
