//! Checks for function parameters that take a mutable reference to a
//! re-borrwable type.
//!
//! # Motivation
//!
//! Mutable references to re-borrwable types can almost always be readily
//! converted back to an owned instance of itself, albeit with an appropriately
//! shorter lifetime.
//!
//! The only time this isn't true is when the function returns referenced data
//! that is bound to the mutable reference of the re-borrowable type.
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! fn system(mut commands: Commands) {
//!   helper_function(&mut commands);
//! }
//!
//! fn helper_function(commands: &mut Commands) {
//!   // ...
//! }
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! fn system(mut commands: Commands) {
//!   helper_function(commands.reborrow());
//! }
//!
//! fn helper_function(mut commands: Commands) {
//!   // ...
//! }
//! ```

use std::ops::ControlFlow;

use crate::declare_bevy_lint;
use clippy_utils::{diagnostics::span_lint_and_sugg, ty::match_type};
use rustc_errors::Applicability;
use rustc_hir::{intravisit::FnKind, Body, FnDecl, Mutability};
use rustc_lint::{LateContext, LateLintPass, Lint};
use rustc_middle::{
    ty::Interner,
    ty::{Ty, TyKind, TypeVisitable, TypeVisitor},
};
use rustc_session::declare_lint_pass;
use rustc_span::{def_id::LocalDefId, symbol::Ident, Span};

declare_bevy_lint! {
    pub BORROW_OF_COMMANDS,
    PEDANTIC,
    "parameter takes `&mut Commands` instead of a re-borrowed `Commands`",
}

declare_bevy_lint! {
    pub BORROW_OF_QUERY,
    PEDANTIC,
    "parameter takes `&mut Query` instead of a re-borrowed `Query`",
}

declare_lint_pass! {
    BorrowOfReborrowable => [BORROW_OF_COMMANDS.lint, BORROW_OF_QUERY.lint]
}

impl<'tcx> LateLintPass<'tcx> for BorrowOfReborrowable {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        _: &'tcx Body<'tcx>,
        _: Span,
        def_id: LocalDefId,
    ) {
        // We are already inside of the function item,
        // so we can use `instantiate_identity` to discharge the binder
        let fn_sig = cx.tcx.fn_sig(def_id).instantiate_identity();

        let arg_names = cx.tcx.fn_arg_names(def_id);

        fn_sig.inputs().map_bound(|args| {
            for (arg_index, arg_ty) in args.iter().enumerate() {
                let TyKind::Ref(region, ty, Mutability::Mut) = arg_ty.kind() else {
                    // We only care about `&mut` parameters
                    continue;
                };

                let peeled_ty = ty.peel_refs();

                let Some(reborrowable) = Reborrowable::try_from_ty(cx, peeled_ty) else {
                    // The type is not one of our known re-borrowable types
                    continue;
                };

                let is_output_bound_to_arg = fn_sig
                    .output()
                    .visit_with(&mut ContainsRegion(*region))
                    .is_break();

                if is_output_bound_to_arg {
                    // We don't want to suggest re-borrowing if the return type's
                    // lifetime is bound to the argument's reference.
                    // This is because it's impossible to convert something like:
                    // `for<'a> (&'a mut Commands<'_, '_>) -> EntityCommands<'a>`
                    // to something like:
                    // `for<'a> (Commands<'_, '_>) -> EntityCommands<'a>`
                    // without getting: `error[E0515]: cannot return value referencing function parameter `commands``
                    continue;
                }

                let arg_ident = arg_names[arg_index];
                let span = decl.inputs[arg_index].span.to(arg_ident.span);

                span_lint_and_sugg(
                    cx,
                    reborrowable.lint(),
                    span,
                    reborrowable.message(),
                    reborrowable.help(),
                    reborrowable.suggest(arg_ident, peeled_ty.to_string()),
                    // Not machine-applicable since the function body may need to
                    // also be updated to account for the removed ref
                    Applicability::MaybeIncorrect,
                );
            }
        });
    }
}

#[derive(Debug)]
enum Reborrowable {
    Commands,
    Query,
}

impl Reborrowable {
    fn try_from_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
        if match_type(cx, ty, &crate::paths::COMMANDS) {
            Some(Self::Commands)
        } else if match_type(cx, ty, &crate::paths::QUERY) {
            Some(Self::Query)
        } else {
            None
        }
    }

    fn lint(&self) -> &'static Lint {
        match self {
            Self::Commands => BORROW_OF_COMMANDS.lint,
            Self::Query => BORROW_OF_QUERY.lint,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Self::Commands => BORROW_OF_COMMANDS.lint.desc,
            Self::Query => BORROW_OF_QUERY.lint.desc,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Commands => "Commands",
            Self::Query => "Query",
        }
    }

    fn help(&self) -> String {
        format!("use `{}` instead", self.name())
    }

    fn suggest(&self, ident: Ident, ty: String) -> String {
        format!("mut {}: {}", ident, ty)
    }
}

/// [`TypeVisitor`] for checking if the given region is contained in the type.
struct ContainsRegion<I: Interner>(pub I::Region);

impl<I: Interner> TypeVisitor<I> for ContainsRegion<I> {
    type Result = ControlFlow<()>;

    fn visit_region(&mut self, r: I::Region) -> Self::Result {
        if self.0 == r {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
