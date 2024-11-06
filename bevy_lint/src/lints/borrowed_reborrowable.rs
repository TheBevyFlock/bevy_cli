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
//! # Known Issues
//!
//! This lint does not currently support closures.
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
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Interner, Ty, TyKind, TypeVisitable, TypeVisitor};
use rustc_session::declare_lint_pass;
use rustc_span::{
    def_id::LocalDefId,
    symbol::{kw, Ident},
    Span,
};

declare_bevy_lint! {
    pub BORROWED_REBORROWABLE,
    PEDANTIC,
    "parameter takes a mutable reference to a re-borrowable type",
}

declare_lint_pass! {
    BorrowedReborrowable => [BORROWED_REBORROWABLE.lint]
}

impl<'tcx> LateLintPass<'tcx> for BorrowedReborrowable {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        _: &'tcx Body<'tcx>,
        _: Span,
        def_id: LocalDefId,
    ) {
        // Closures are not currently supported, as `tcx.fn_sig()` crashes for them.
        if let FnKind::Closure = kind {
            return;
        }

        // We use `instantiate_identity` to discharge the binder since we don't
        // mind using placeholders for any bound arguments
        let fn_sig = cx.tcx.fn_sig(def_id).instantiate_identity();

        let arg_names = cx.tcx.fn_arg_names(def_id);

        let args = fn_sig.inputs().skip_binder();

        for (arg_index, arg_ty) in args.iter().enumerate() {
            let TyKind::Ref(region, ty, Mutability::Mut) = arg_ty.kind() else {
                // We only care about `&mut` parameters
                continue;
            };

            let arg_ident = arg_names[arg_index];
            if arg_ident.name == kw::SelfLower {
                // Skip `&mut self` parameters
                continue;
            }

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
                // without getting: `error[E0515]: cannot return value referencing function
                // parameter `commands` ``
                continue;
            }

            let arg_ident = arg_names[arg_index];
            let span = decl.inputs[arg_index].span.to(arg_ident.span);

            span_lint_and_sugg(
                cx,
                BORROWED_REBORROWABLE.lint,
                span,
                reborrowable.message(),
                reborrowable.help(),
                reborrowable.suggest(arg_ident, peeled_ty.to_string()),
                // Not machine-applicable since the function body may need to
                // also be updated to account for the removed ref
                Applicability::MaybeIncorrect,
            );
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Reborrowable {
    Commands,
    // Deferred,
    // DeferredWorld,
    EntityCommands,
    // EntityMut,
    // EntityMutExcept,
    // FilteredEntityMut,
    // FilteredResourcesMut,
    // Mut,
    // MutUntyped,
    NonSendMut,
    // PtrMut,
    Query,
    // QueryIterationCursor,
    ResMut,
}

impl Reborrowable {
    fn try_from_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
        const PATH_MAP: &[(&[&str], Reborrowable)] = &[
            (&crate::paths::COMMANDS, Reborrowable::Commands),
            (&crate::paths::ENTITY_COMMANDS, Reborrowable::EntityCommands),
            (&crate::paths::QUERY, Reborrowable::Query),
            (&crate::paths::RES_MUT, Reborrowable::ResMut),
            (&crate::paths::NON_SEND_MUT, Reborrowable::NonSendMut),
        ];

        for &(path, reborrowable) in PATH_MAP {
            if match_type(cx, ty, path) {
                return Some(reborrowable);
            }
        }

        None
    }

    fn message(&self) -> String {
        let name = self.name();
        format!("parameter takes `&mut {name}` instead of a re-borrowed `{name}`",)
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Commands => "Commands",
            Self::EntityCommands => "EntityCommands",
            Self::Query => "Query",
            Self::ResMut => "ResMut",
            Self::NonSendMut => "NonSendMut",
        }
    }

    fn help(&self) -> String {
        let name = self.name();
        format!("use `{name}` instead")
    }

    fn suggest(&self, ident: Ident, ty: String) -> String {
        format!("mut {ident}: {ty}")
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
