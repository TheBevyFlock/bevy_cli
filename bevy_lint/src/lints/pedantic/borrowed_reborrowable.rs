//! Checks for function parameters that take a mutable reference to a re-borrowable type.
//!
//! This lint checks for the following re-borrowable types:
//!
//! - `Commands`
//! - `Deferred`
//! - `DeferredWorld`
//! - `EntityCommands`
//! - `EntityMut`
//! - `FilteredEntityMut`
//! - `Mut`
//! - `MutUntyped`
//! - `NonSendMut`
//! - `PtrMut`
//! - `Query`
//! - `ResMut`
//!
//! Though a type may be re-borrowable, there are circumstances where it cannot be easily
//! reborrowed. (Please see the [Examples](#example).) In these cases, no warning will be emitted.
//!
//! # Motivation
//!
//! Several Bevy types look like they are owned, when in reality they contain an `&mut` reference
//! to the data owned by the ECS. `Commands` and `Query` are examples of such types that _pretend_
//! to own data for better user ergonomics.
//!
//! This can be an issue when a user writes a function that takes a mutable reference to one of
//! these types, not realizing that it itself is _already_ a reference. These mutable references
//! can almost always be readily converted back to an owned instance of the type, which is a cheap
//! operation that avoids nested references.
//!
//! # Known Issues
//!
//! This lint does not currently support the [`Fn`] traits or function pointers. This means the
//! following types will not be caught by the lint:
//!
//! - `impl FnOnce(&mut Commands)`
//! - `Box<dyn FnMut(&mut Commands)>`
//! - `fn(&mut Commands)`
//!
//! For more information, please see [#174].
//!
//! [#174]: https://github.com/TheBevyFlock/bevy_cli/issues/174
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! fn system(mut commands: Commands) {
//!     helper_function(&mut commands);
//! }
//!
//! // This takes `&mut Commands`, but it doesn't need to!
//! fn helper_function(commands: &mut Commands) {
//!     // ...
//! }
//! #
//! # bevy::ecs::system::assert_is_system(system);
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! fn system(mut commands: Commands) {
//!     // Convert `&mut Commands` to `Commands`.
//!     helper_function(commands.reborrow());
//! }
//!
//! fn helper_function(mut commands: Commands) {
//!     // ...
//! }
//! #
//! # bevy::ecs::system::assert_is_system(system);
//! ```
//!
//! A type cannot be easily reborrowed when a function returns a reference with the same lifetime
//! as the borrowed type. The lint knows about this case, however, and will not emit any warning if
//! it knows the type cannot be re-borrowed:
//!
//! ```
//! # use bevy::{prelude::*, ecs::system::EntityCommands};
//! #
//! fn system(mut commands: Commands) {
//!     let entity_commands = helper_function(&mut commands);
//!     // ...
//! }
//!
//! // Note how this function returns a reference with the same lifetime as `Commands`.
//! fn helper_function<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
//!     commands.spawn_empty()
//! }
//! #
//! # bevy::ecs::system::assert_is_system(system);
//! ```

use std::ops::ControlFlow;

use crate::{declare_bevy_lint, declare_bevy_lint_pass};
use clippy_utils::{
    diagnostics::span_lint_and_sugg,
    source::{snippet, snippet_opt},
    ty::match_type,
};
use rustc_errors::Applicability;
use rustc_hir::{Body, FnDecl, MutTy, Mutability, intravisit::FnKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Interner, Ty, TyKind, TypeVisitable, TypeVisitor};
use rustc_span::{Span, def_id::LocalDefId, symbol::kw};

declare_bevy_lint! {
    pub BORROWED_REBORROWABLE,
    super::PEDANTIC,
    "function parameter takes a mutable reference to a re-borrowable type",
}

declare_bevy_lint_pass! {
    pub BorrowedReborrowable => [BORROWED_REBORROWABLE.lint],
}

impl<'tcx> LateLintPass<'tcx> for BorrowedReborrowable {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        fn_span: Span,
        def_id: LocalDefId,
    ) {
        // If the function originates from an external macro, skip this lint
        if fn_span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }

        let fn_sig = match kind {
            FnKind::Closure => cx.tcx.closure_user_provided_sig(def_id).value,
            // We use `instantiate_identity` to discharge the binder since we don't
            // mind using placeholders for any bound arguments
            _ => cx.tcx.fn_sig(def_id).instantiate_identity(),
        };

        // A list of argument types, used in the actual lint check.
        let arg_types = fn_sig.inputs().skip_binder();
        // A list of argument names, used to check and skip `&mut self`.
        let arg_names = cx.tcx.fn_arg_names(def_id);
        // A list of argument parameters, used to find the span of arguments.
        let arg_params = body.params;

        debug_assert_eq!(
            arg_types.len(),
            arg_names.len(),
            "there must be the same number of argument types and names"
        );
        debug_assert_eq!(
            arg_types.len(),
            arg_params.len(),
            "there must be the same number of argument types and parameters"
        );

        for (arg_index, arg_ty) in arg_types.iter().enumerate() {
            let TyKind::Ref(region, ty, Mutability::Mut) = arg_ty.kind() else {
                // We only care about `&mut` parameters
                continue;
            };

            // This lint would emit a warning on `&mut self` if `self` was reborrowable. This isn't
            // useful, though, because it would hurt the ergonomics of using methods of
            // reborrowable types.
            //
            // To avoid this, we skip any parameter named `self`. This won't false-positive on
            // other function arguments named `self`, since it is a special keyword that is
            // disallowed in other positions.
            if arg_names[arg_index].is_some_and(|ident| ident.name == kw::SelfLower) {
                continue;
            }

            let Some(reborrowable) = Reborrowable::try_from_ty(cx, *ty) else {
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

            // This tries to get the user-written form of `T` given the HIR representation for `&T`
            // / `&mut T`. If we cannot for whatever reason, we fallback to using
            // `Ty::to_string()` to get the fully-qualified form of `T`.
            //
            // For example, given a function signature like `fn(&mut Commands)`, we try to get the
            // snippet for just `Commands` but default to `bevy::prelude::Commands<'_, '_>` if we
            // cannot.
            let ty_snippet = match decl.inputs[arg_index].kind {
                // The `Ty` should be a `Ref`, since we proved that above.
                rustc_hir::TyKind::Ref(_, MutTy { ty: inner_ty, .. }) => {
                    // Get the snippet for the inner type.
                    snippet_opt(cx, inner_ty.span)
                }
                // If it's not a `Ref` for whatever reason, fallback to our default value.
                _ => None,
            }
            // We previously peeled the `&mut` reference, so `ty` is just the underlying `T`.
            .unwrap_or_else(|| ty.to_string());

            span_lint_and_sugg(
                cx,
                BORROWED_REBORROWABLE.lint,
                // The span contains both the argument name and type.
                arg_params[arg_index].span,
                reborrowable.message(),
                reborrowable.help(),
                reborrowable.suggest(cx, arg_params[arg_index].pat.span, ty_snippet),
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
    Deferred,
    DeferredWorld,
    EntityCommands,
    EntityMut,
    FilteredEntityMut,
    Mut,
    MutUntyped,
    NonSendMut,
    PtrMut,
    Query,
    ResMut,
}

impl Reborrowable {
    fn try_from_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
        use crate::paths::*;

        const PATH_MAP: &[(&[&str], Reborrowable)] = &[
            (&COMMANDS, Reborrowable::Commands),
            (&DEFERRED, Reborrowable::Deferred),
            (&DEFERRED_WORLD, Reborrowable::DeferredWorld),
            (&ENTITY_COMMANDS, Reborrowable::EntityCommands),
            (&ENTITY_MUT, Reborrowable::EntityMut),
            (&FILTERED_ENTITY_MUT, Reborrowable::FilteredEntityMut),
            (&MUT, Reborrowable::Mut),
            (&MUT_UNTYPED, Reborrowable::MutUntyped),
            (&NON_SEND_MUT, Reborrowable::NonSendMut),
            (&PTR_MUT, Reborrowable::PtrMut),
            (&QUERY, Reborrowable::Query),
            (&RES_MUT, Reborrowable::ResMut),
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
            Self::Deferred => "Deferred",
            Self::DeferredWorld => "DeferredWorld",
            Self::EntityCommands => "EntityCommands",
            Self::EntityMut => "EntityMut",
            Self::FilteredEntityMut => "FilteredEntityMut",
            Self::Mut => "Mut",
            Self::MutUntyped => "MutUntyped",
            Self::NonSendMut => "NonSendMut",
            Self::PtrMut => "PtrMut",
            Self::Query => "Query",
            Self::ResMut => "ResMut",
        }
    }

    fn help(&self) -> String {
        let name = self.name();
        format!("use `{name}` instead")
    }

    fn suggest(&self, cx: &LateContext, name: Span, ty: String) -> String {
        let name = snippet(cx, name, "_");
        format!("mut {name}: {ty}")
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
