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
            // ...and the type the trait is being implemented for has a user-defined name. See
            // `extract_path_from_hir_ty`'s documentation for more info on what this does.
            && let Some(self_path) = extract_path_from_hir_ty(impl_.self_ty)
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

/// A best-effort utilitiy that tries to extract the [`Path`] of an HIR [`Ty`] if the name can
/// easily be changed by the user.
///
/// Kinds of types that are extracted are paths (`module::submodule::Type`) and references to paths
/// (`&module::Type` or `*const Type`). Types that are not extracted, and just return [`None`],
/// include slices (`[T]`), trait objects (`dyn Trait`), tuples (`(A, B, ...)`), and more.
fn extract_path_from_hir_ty<'tcx>(hir_ty: &Ty<'tcx>) -> Option<&'tcx Path<'tcx>> {
    match hir_ty.kind {
        // The type is a path, such as `module::Type`.
        TyKind::Path(qpath) => match qpath {
            // If the qualified path points to a resolved piece of code, return that path.
            QPath::Resolved(_, path) => Some(path),
            // The alternatives are lang items (which cannot be renamed without `#![no_core]`) and
            // relative paths (`<T>::AssociatedType`), which also cannot be renamed easily.
            _ => None,
        },
        // If the type is a reference or pointer, recursively check the inner type. For instance,
        // `*const module::Type` would return `module::Type`, while `&[usize; 10]` would return
        // `None` because `[usize; 10]` returns `None`.
        TyKind::Ref(_, mut_ty) | TyKind::Ptr(mut_ty) => extract_path_from_hir_ty(mut_ty.ty),
        _ => None,
    }
}
