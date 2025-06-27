use clippy_utils::{
    diagnostics::span_lint_and_help,
    source::{snippet, snippet_opt},
    ty::match_type,
};
use rustc_hir::Expr;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_span::Symbol;

use crate::{
    declare_bevy_lint, declare_bevy_lint_pass,
    utils::hir_parse::{MethodCall, generic_args_snippet, span_args},
};

declare_bevy_lint! {
    /// Checks for use of panicking methods of `World` when a non-panicking alternative exists.
    ///
    /// For instance, this will lint against `World::entity()`, recommending that
    /// `World::get_entity()` should be used instead.
    ///
    /// # Motivation
    ///
    /// Panicking is the nuclear option of error handling in Rust: it is meant for cases where
    /// recovery is near-impossible. As such, panicking is usually undesirable in long-running
    /// applications and games like what Bevy is used for. This lint aims to prevent unwanted
    /// crashes in these applications by forcing developers to handle the `Option` or `Result` in
    /// their code.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// #
    /// #[derive(Resource)]
    /// struct MyResource;
    ///
    /// fn panicking_world(world: &mut World) {
    ///     let resource = world.resource::<MyResource>();
    ///     // ...
    /// }
    /// #
    /// # bevy::ecs::system::assert_is_system(panicking_world);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// #
    ///
    /// #[derive(Resource)]
    /// struct MyResource;
    ///
    /// fn graceful_world(world: &mut World) {
    ///     let Some(resource) = world.get_resource::<MyResource>() else {
    ///         // Resource may not exist.
    ///         return;
    ///     };
    ///
    ///     // ...
    /// }
    /// #
    /// # bevy::ecs::system::assert_is_system(graceful_world);
    /// ```
    pub PANICKING_METHODS,
    super::Restriction,
    "called a method that can panic when a non-panicking alternative exists",
}

declare_bevy_lint_pass! {
    pub(crate) PanickingMethods => [PANICKING_METHODS],
}

impl<'tcx> LateLintPass<'tcx> for PanickingMethods {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // skip expressions that originate from external macros
        if expr.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }

        // Check if `expr` is a method call
        if let Some(MethodCall {
            span,
            method_path,
            args,
            receiver,
            is_fully_qulified,
        }) = MethodCall::try_from(cx, expr)
        {
            // get the `Ty` of the receiver, this can either be:
            //
            // for fully qualified method calls the first argument is `Self` and represents the
            // `Ty` we are looking for:
            //
            // Query::single(&foo, args);
            //              ^^^^^
            // for *not* fully qualified method calls:
            //
            // foo.single();
            // ^^^^^
            //
            // We peel all references to that `Foo`, `&Foo`, `&&Foo`, etc.
            let src_ty = cx.typeck_results().expr_ty(receiver).peel_refs();

            // Check if `src_ty` is a type that has panicking methods (e.g. `Query`), else exit.
            let Some(panicking_type) = PanickingType::try_from_ty(cx, src_ty) else {
                return;
            };

            // Get a list of methods that panic and their alternatives for the specific query
            // variant.
            let panicking_alternatives = panicking_type.alternatives();

            // Here we check if the method name matches one of methods in `panicking_alternatives`.
            // If it does match, we store the recommended alternative for reference in diagnostics
            // later. If nothing matches, we exit the entire function.
            let alternative = 'block: {
                for (panicking_method, alternative_method) in panicking_alternatives {
                    // If performance is an issue in the future, this could be cached.
                    let key = Symbol::intern(panicking_method);

                    if method_path.ident.name == key {
                        // It is one of the panicking methods. Write down the alternative and
                        // stop searching.
                        break 'block *alternative_method;
                    }
                }

                // If we reach this point, the method is not one we're searching for. In this
                // case, we exit.
                return;
            };

            // By this point, we've verified that `src` is a panicking type and the method is
            // one that panics with a viable alternative. Let's emit the lint.
            let (src_snippet, generics_snippet, args_snippet) = if is_fully_qulified {
                // When the method was a fully qualified method call, the beginning of the snippet
                // is just the `PanickingType`.
                let mut src_snippet = panicking_type.name().to_string();
                src_snippet.push_str("::");

                // Try to find the generic arguments of the method, if any exist. This can
                // either evaluate to `""` or `"::<A, B, C>"`.
                let generics_snippet = generic_args_snippet(cx, method_path);

                // The first argument to a fully qualified method call is the receiver (`Self`) and
                // is not part of the `args`
                let receiver_snippet = snippet(cx, receiver.span, "");

                // Try to find the string representation of the arguments to our panicking
                // method. See `span_args()` for more details on how this is
                // done.
                let args_snippet = snippet(cx, span_args(args), "");
                // If there are no args, just return the `receiver` as the only argument
                if args_snippet.is_empty() {
                    (src_snippet, generics_snippet, receiver_snippet)
                } else {
                    // If there are arguments in the method call, add them after the `receiver` and
                    // add the `,` as delimiter
                    (
                        src_snippet,
                        generics_snippet,
                        format!("{receiver_snippet}, {args_snippet}").into(),
                    )
                }
            }
            // The method was not a fully qualified call
            else {
                // Try to find the string representation of `src`. This usually returns
                // `my_query` without the trailing `.`, so we manually
                // append it. When the snippet cannot be found, we default
                // to the qualified `Type::` form.
                let src_snippet = snippet_opt(cx, receiver.span).map_or_else(
                    || format!("{}::", panicking_type.name()),
                    |mut s| {
                        s.push('.');
                        s
                    },
                );
                // Try to find the generic arguments of the method, if any exist. This can
                // either evaluate to `""` or `"::<A, B, C>"`.
                let generics_snippet = generic_args_snippet(cx, method_path);

                // Try to find the string representation of the arguments to our panicking
                // method. See `span_args()` for more details on how this is
                // done.
                let args_snippet = snippet(cx, span_args(args), "");
                (src_snippet, generics_snippet, args_snippet)
            };

            span_lint_and_help(
                cx,
                PANICKING_METHODS,
                span,
                format!(
                    "called a `{}` method that can panic when a non-panicking alternative exists",
                    panicking_type.name()
                ),
                None,
                // This usually ends up looking like: `query.get_many([e1, e2])`.
                format!(
                    "use `{src_snippet}{alternative}{generics_snippet}({args_snippet})` and handle the `Option` or `Result`"
                ),
            );
        }
    }
}

enum PanickingType {
    Query,
    World,
}

impl PanickingType {
    /// Returns the corresponding variant for the given [`Ty`], if it is supported by this lint.
    fn try_from_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
        if match_type(cx, ty, &crate::paths::QUERY) {
            Some(Self::Query)
        } else if match_type(cx, ty, &crate::paths::WORLD) {
            Some(Self::World)
        } else {
            None
        }
    }

    /// Returns a list of panicking methods for each of the supported types.
    ///
    /// Each item in the returned [`slice`] is of the format
    /// `(panicking_method, alternative_method)`.
    fn alternatives(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Query => &[("many", "get_many"), ("many_mut", "get_many_mut")],
            Self::World => &[
                ("entity", "get_entity"),
                ("entity_mut", "get_entity_mut"),
                ("resource", "get_resource"),
                ("resource_mut", "get_resource_mut"),
                ("resource_ref", "get_resource_ref"),
                ("non_send_resource", "get_non_send_resource"),
                ("non_send_resource_mut", "get_non_send_resource_mut"),
                ("run_schedule", "try_run_schedule"),
                ("schedule_scope", "try_schedule_scope"),
            ],
        }
    }

    /// Returns the name of the type this variant represents.
    fn name(&self) -> &'static str {
        match self {
            Self::Query => "Query",
            Self::World => "World",
        }
    }
}
