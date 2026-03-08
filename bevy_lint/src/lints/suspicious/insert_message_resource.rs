//! Checks for the `Messages<T>` resource being manually inserted with `App::init_resource()` or
//! `App::insert_resource()` instead of with `App::add_message()`.
//!
//! # Motivation
//!
//! Unless you have intentionally and knowingly initialized the `Messages<T>` resource in this way,
//! messages and their resources should be initialized with `App::add_message()` because it
//! automatically handles dropping old messages. Just adding `Messages<T>` makes no such guarantee,
//! and will likely result in a memory leak.
//!
//! For more information, please see the documentation on [`App::add_message()`] and
//! [`Messages<T>`].
//!
//! [`Messages<T>`]: https://docs.rs/bevy/latest/ecs/message/struct.Messages.html
//! [`App::add_message()`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.add_message
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(Message)]
//! struct MyMessage;
//!
//! fn plugin(app: &mut App) {
//!     app.init_resource::<Messages<MyMessage>>();
//! }
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(Message)]
//! struct MyMessage;
//!
//! fn plugin(app: &mut App) {
//!     app.add_message::<MyMessage>();
//! }
//! ```

use std::borrow::Cow;

use clippy_utils::{
    diagnostics::span_lint_and_sugg,
    source::{snippet, snippet_with_applicability},
    ty::ty_from_hir_ty,
};
use rustc_errors::Applicability;
use rustc_hir::{Expr, GenericArg, GenericArgs, Path, PathSegment, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Ty, TyKind};

use crate::{
    declare_bevy_lint, declare_bevy_lint_pass, sym,
    utils::{
        hir_parse::{generic_args_snippet, span_args},
        method_call::MethodCall,
    },
};

declare_bevy_lint! {
    pub(crate) INSERT_MESSAGE_RESOURCE,
    super::Suspicious,
    "called `App::insert_resource(Messages<T>)` or `App::init_resource::<Messages<T>>()` instead of `App::add_message::<T>()`",
}

declare_bevy_lint_pass! {
    pub(crate) InsertMessageResource => [INSERT_MESSAGE_RESOURCE],
}

const HELP_MESSAGE: &str = "inserting an `Messages` resource does not fully setup that message";

impl<'tcx> LateLintPass<'tcx> for InsertMessageResource {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // skip expressions that originate from external macros
        if expr.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }

        // Find a method call.
        if let Some(method_call) = MethodCall::try_from(cx, expr) {
            // Get the type for `src` in `src.method()`. We peel all references because the type
            // could either be `App` or `&mut App`.
            let src_ty = cx
                .typeck_results()
                .expr_ty_adjusted(method_call.receiver)
                .peel_refs();

            // If `src` is not a Bevy `App`, exit.
            if !crate::paths::APP.matches_ty(cx, src_ty) {
                return;
            }

            // If the method is `App::insert_resource()` or `App::init_resource()`, check it with
            // its corresponding function.
            match method_call.method_path.ident.name {
                symbol if symbol == sym::insert_resource => {
                    check_insert_resource(cx, &method_call);
                }
                symbol if symbol == sym::init_resource => {
                    check_init_resource(cx, &method_call);
                }
                _ => {}
            }
        }
    }
}

/// Checks if `App::insert_resource()` inserts an `Messages<T>`, and emits a diagnostic if so.
fn check_insert_resource(cx: &LateContext<'_>, method_call: &MethodCall) {
    // Extract the argument if there is only 1 (which there should be!), else exit.
    let [arg] = method_call.args else {
        return;
    };

    // Find the type of `arg` in `App::insert_resource(&mut self, arg)`.
    let ty = cx.typeck_results().expr_ty_adjusted(arg);

    // If `arg` is `Messages<T>`, emit the lint.
    if crate::paths::MESSAGES.matches_ty(cx, ty) {
        let mut applicability = Applicability::MachineApplicable;

        let message_ty_snippet = extract_ty_message_snippet(ty, &mut applicability);
        let args_snippet = snippet(cx, span_args(method_call.args), "");
        let generics_snippet = generic_args_snippet(cx, method_call.method_path);

        if method_call.is_fully_qualified {
            let receiver_snippet = snippet(cx, method_call.receiver.span, "");
            span_lint_and_sugg(
                cx,
                INSERT_MESSAGE_RESOURCE,
                method_call.span,
                format!(
                    "called `App::insert_resource{generics_snippet}({receiver_snippet}, {args_snippet})` instead of `App::add_message::<{message_ty_snippet}>({receiver_snippet})`"
                ),
                HELP_MESSAGE,
                format!("App::add_message::<{message_ty_snippet}>({receiver_snippet})"),
                applicability,
            );
        } else {
            span_lint_and_sugg(
                cx,
                INSERT_MESSAGE_RESOURCE,
                method_call.span,
                format!(
                    "called `App::insert_resource{generics_snippet}({args_snippet})` instead of `App::add_message::<{message_ty_snippet}>()`"
                ),
                HELP_MESSAGE,
                format!("add_message::<{message_ty_snippet}>()"),
                applicability,
            );
        }
    }
}

/// Creates a string representation of type `T` for [`Ty`] `Messages<T>`.
///
/// This takes a mutable applicability reference, and will set it to
/// [`Applicability::HasPlaceholders`] if the type cannot be stringified.
fn extract_ty_message_snippet<'tcx>(
    messages_ty: Ty<'tcx>,
    applicability: &mut Applicability,
) -> Cow<'tcx, str> {
    const DEFAULT: Cow<str> = Cow::Borrowed("T");

    let TyKind::Adt(_, messages_arguments) = messages_ty.kind() else {
        if let Applicability::MachineApplicable = applicability {
            *applicability = Applicability::HasPlaceholders;
        }

        return DEFAULT;
    };

    let Some(message_snippet) = messages_arguments.iter().next() else {
        if let Applicability::MachineApplicable = applicability {
            *applicability = Applicability::HasPlaceholders;
        }

        return DEFAULT;
    };

    format!("{message_snippet:?}").into()
}

/// Checks if `App::init_resource()` inserts an `Messages<T>`, and emits a diagnostic if so.
fn check_init_resource<'tcx>(cx: &LateContext<'tcx>, method_call: &MethodCall<'tcx>) {
    if let Some(&GenericArgs {
        // `App::init_resource()` has one generic type argument: T.
        args: &[GenericArg::Type(resource_hir_ty)],
        ..
    }) = method_call.method_path.args
    {
        // Lower `rustc_hir::Ty` to `ty::Ty`, so we can inspect type information. For more
        // information, see <https://rustc-dev-guide.rust-lang.org/ty.html#rustc_hirty-vs-tyty>.
        let resource_ty = ty_from_hir_ty(cx, resource_hir_ty.as_unambig_ty());

        // If the resource type is `Messages<T>`, emit the lint.
        if crate::paths::MESSAGES.matches_ty(cx, resource_ty) {
            let mut applicability = Applicability::MachineApplicable;

            let message_ty_snippet = extract_hir_message_snippet(
                cx,
                resource_hir_ty.as_unambig_ty(),
                &mut applicability,
            );

            let args_snippet = snippet(cx, span_args(method_call.args), "");
            let generics_snippet = generic_args_snippet(cx, method_call.method_path);

            if method_call.is_fully_qualified {
                let receiver_snippet = snippet(cx, method_call.receiver.span, "");
                span_lint_and_sugg(
                    cx,
                    INSERT_MESSAGE_RESOURCE,
                    method_call.span,
                    format!(
                        "called `App::init_resource{generics_snippet}({receiver_snippet})` instead of `App::add_message::<{message_ty_snippet}>({receiver_snippet})`"
                    ),
                    HELP_MESSAGE,
                    format!("App::add_message::<{message_ty_snippet}>({receiver_snippet})"),
                    applicability,
                );
            } else {
                span_lint_and_sugg(
                    cx,
                    INSERT_MESSAGE_RESOURCE,
                    method_call.span,
                    format!(
                        "called `App::init_resource{generics_snippet}({args_snippet})` instead of `App::add_message::<{message_ty_snippet}>()`"
                    ),
                    HELP_MESSAGE,
                    format!("add_message::<{message_ty_snippet}>()"),
                    applicability,
                );
            }
        }
    }
}

/// Tries to extract the snippet `MyMessage` from the [`rustc_hir::Ty`] representing
/// `Messages<MyMessage>`.
///
/// Note that this works on a best-effort basis, and will return `"T"` if the type cannot be
/// extracted. If so, it will mutate the passed applicability to [`Applicability::HasPlaceholders`],
/// similar to [`snippet_with_applicability()`].
fn extract_hir_message_snippet<'tcx>(
    cx: &LateContext<'tcx>,
    messages_hir_ty: &rustc_hir::Ty<'tcx>,
    applicability: &mut Applicability,
) -> Cow<'static, str> {
    const DEFAULT: Cow<str> = Cow::Borrowed("T");

    // This is some crazy pattern matching. Let me walk you through it:
    let message_span = match messages_hir_ty.kind {
        // There are multiple kinds of HIR types, but we're looking for a path to a type
        // definition. This path is likely `Messages`, and contains the generic argument that we're
        // searching for.
        rustc_hir::TyKind::Path(QPath::Resolved(
            _,
            &Path {
                // There can be multiple segments in a path, such as if it were
                // `bevy::prelude::Messages`, but in this case we just care about the last:
                // `Messages`.
                segments:
                    &[
                        ..,
                        PathSegment {
                            // Find the arguments to `Messages<T>`, extracting `T`.
                            args:
                                Some(&GenericArgs {
                                    args: &[GenericArg::Type(ty)],
                                    ..
                                }),
                            ..
                        },
                    ],
                ..
            },
        )) => {
            // We now have the HIR type `T` for `Messages<T>`, let's return its span.
            ty.span
        }
        // Something in the above pattern matching went wrong, likely due to an edge case. For
        // this, we set the applicability to `HasPlaceholders` and return the default snippet.
        _ => {
            if let Applicability::MachineApplicable = applicability {
                *applicability = Applicability::HasPlaceholders;
            }

            return DEFAULT;
        }
    };

    // We now have the span to the message type, so let's try to extract it into a string.
    snippet_with_applicability(cx, message_span, &DEFAULT, applicability)
}
