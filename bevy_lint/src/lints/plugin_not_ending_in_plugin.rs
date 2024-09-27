//! Checks for types who implement `Plugin` but whose names does not end in "Plugin".
//!
//! This does _not_ check function-style plugins (`fn plugin(app: &mut App)`), only structures with
//! `Plugin` explicitly implemented with `impl Plugin for T`.
//!
//! # Motivation
//!
//! Unlike traits like [`Clone`] or [`Debug`], the primary purpose of a type that implements
//! `Plugin` is to be a Bevy plugin. As such, it is common practice to suffix plugin names with
//! "Plugin" to signal how they should be used.
//!
//! # Known issues
//!
//! Due to technical reasons, if you wish to silence this lint you need to annotate the
//! `impl Plugin for T` line with `#[allow(bevy::plugin_not_ending_in_plugin)]`, not the `struct T`
//! line.
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! struct Physics;
//!
//! impl Plugin for Physics {
//!     fn build(&self, app: &mut App) {
//!         // ...
//!     }
//! }
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! struct PhysicsPlugin;
//!
//! impl Plugin for PhysicsPlugin {
//!     fn build(&self, app: &mut App) {
//!         // ...
//!     }
//! }
//! ```

use crate::declare_bevy_lint;
use clippy_utils::{diagnostics::span_lint_and_then, match_def_path, path_res};
use rustc_errors::Applicability;
use rustc_hir::{def::Res, Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::symbol::Ident;

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
        // Find `impl` items...
        if let ItemKind::Impl(impl_) = item.kind
            // ...that implement a trait...
            && let Some(of_trait) = impl_.of_trait
            // ...where the trait is a path to user code... (I don't believe this will ever be
            // false, since the alternatives are primitives, `Self`, and others that wouldn't make
            // since in this scenario.)
            && let Res::Def(_, def_id) = of_trait.path.res
            // ...where the trait being implemented is Bevy's `Plugin`...
            && match_def_path(cx, def_id, &crate::paths::PLUGIN)
        {
            // Try to resolve the original definition of this type, finding its original name and
            // span. (We don't use the name from the path, since that can be spoofed through
            // `use Foo as FooPlugin`.)
            let Some(Ident {
                name: self_name,
                span: self_span,
            }) = path_res(cx, impl_.self_ty)
                .opt_def_id()
                .and_then(|def_id| cx.tcx.opt_item_ident(def_id))
            else {
                return;
            };

            // If the type's name ends in "Plugin", exit.
            if self_name.as_str().ends_with("Plugin") {
                return;
            }

            span_lint_and_then(
                cx,
                PLUGIN_NOT_ENDING_IN_PLUGIN.lint,
                self_span,
                PLUGIN_NOT_ENDING_IN_PLUGIN.lint.desc,
                |diag| {
                    diag.span_suggestion(
                        self_span,
                        "rename the plugin",
                        format!("{self_name}Plugin"),
                        // There may be other references that also need to be renamed.
                        Applicability::MaybeIncorrect,
                    );

                    diag.span_help(item.span, "`Plugin` implemented here");
                },
            );
        }
    }
}
