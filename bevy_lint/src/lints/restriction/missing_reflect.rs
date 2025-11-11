//! Checks for components, resources, and events that do not implement `Reflect`.
//!
//! # Motivation
//!
//! Reflection lets programs inspect type information at runtime. It is commonly used by tools to
//! view and edit ECS information while the program is running. Reflection is opt-in, however, and
//! easy to forget since you need to `#[derive(Reflect)]` for each type that uses it.
//!
//! # Known issues
//!
//! This lint will suggest `#[derive(Reflect)]` even if it cannot be applied. (E.g. if one of the
//! fields does not implement `Reflect`.) For more information, please see [#141].
//!
//! [#141]: https://github.com/TheBevyFlock/bevy_cli/issues/141
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(Component)]
//! struct MyComponent;
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! // Remember to also register this component in the `App` type registry.
//! #[derive(Component, Reflect)]
//! struct MyComponent;
//! ```
//!
//! Often you'll only want to enable this lint for a specific module:
//!
//! <!-- We currently ignore this doc test because any reference to `bevy_lint` causes it to be
//! linked, which raises a compile error due to the linter's use of `rustc_private`. -->
//!
//! ```ignore
//! mod types {
//!     #![cfg_attr(bevy_lint, warn(bevy::missing_reflect))]
//! #
//! #   use bevy::prelude::*;
//!
//!     #[derive(Resource, Reflect)]
//!     struct Score(u32);
//!
//!     #[derive(Component, Reflect)]
//!     struct Happiness(i8);
//! }
//! ```
//!
//! For more information, please see [Toggling Lints in
//! Code](../../index.html#toggling-lints-in-code).

use clippy_utils::{
    diagnostics::span_lint_hir_and_then,
    sugg::DiagExt,
    ty::{implements_trait, ty_from_hir_ty},
};
use rustc_errors::Applicability;
use rustc_hir::ItemKind;
use rustc_lint::{LateContext, LateLintPass};

use crate::{
    declare_bevy_lint, declare_bevy_lint_pass, span_unreachable, utils::traits::TraitType,
};

declare_bevy_lint! {
    pub(crate) MISSING_REFLECT,
    super::Restriction,
    "defined a component, resource, or event without a `Reflect` implementation",
    // We only override `check_crate()`.
    @crate_level_only = true,
}

declare_bevy_lint_pass! {
    pub(crate) MissingReflect => [MISSING_REFLECT],
}

impl<'tcx> LateLintPass<'tcx> for MissingReflect {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        // Finds all types that implement `Reflect` in this crate.
        let reflected: Vec<TraitType> =
            TraitType::from_local_crate(cx, &crate::paths::REFLECT).collect();

        // Finds all non-`Reflect` types that implement `Event` in this crate.
        let events: Vec<TraitType> = TraitType::from_local_crate(cx, &crate::paths::EVENT)
            .filter(|trait_type| !reflected.contains(trait_type))
            .collect();

        // Finds all non-`Reflect` types that implement `Message` in this crate.
        let messages: Vec<TraitType> = TraitType::from_local_crate(cx, &crate::paths::MESSAGE)
            .filter(|trait_type| !reflected.contains(trait_type))
            .collect();

        // Finds all non-`Reflect` types that implement `Component` in this crate.
        let components: Vec<TraitType> = TraitType::from_local_crate(cx, &crate::paths::COMPONENT)
            .filter(|trait_type| !reflected.contains(trait_type))
            .collect();

        // Finds all non-`Reflect` types that implement `Resource` in this crate.
        let resources: Vec<TraitType> = TraitType::from_local_crate(cx, &crate::paths::RESOURCE)
            .filter(|trait_type| !reflected.contains(trait_type))
            .collect();

        let reflect_trait_def_ids = crate::paths::PARTIAL_REFLECT.get(cx);

        // Emit diagnostics for each of these types.
        for (checked_trait, trait_name, message_phrase) in [
            (events, "Event", "an event"),
            (messages, "Message", "a message"),
            (components, "Component", "a component"),
            (resources, "Resource", "a resource"),
        ] {
            for without_reflect in checked_trait {
                // Skip if a types originates from a foreign crate's macro
                if without_reflect
                    .item_span
                    .in_external_macro(cx.tcx.sess.source_map())
                {
                    continue;
                }

                // This lint is machine applicable unless any of the struct's fields do not
                // implement `PartialReflect`.
                let mut applicability = Applicability::MachineApplicable;

                // Find the `Item` definition of the struct missing `#[derive(Reflect)]`. We can use
                // `expect_owner()` because the HIR ID was originally created from a `LocalDefId`,
                // and we can use `expect_item()` because `TraitType::from_local_crate()` only
                // returns items.
                let without_reflect_item = cx
                    .tcx
                    .hir_expect_item(without_reflect.hir_id.expect_owner().def_id);

                // Extract a list of all fields within the structure definition.
                let fields = match without_reflect_item.kind {
                    ItemKind::Struct(_, _, data) => data.fields().to_vec(),
                    ItemKind::Enum(_, _, enum_def) => enum_def
                        .variants
                        .iter()
                        .flat_map(|variant| variant.data.fields())
                        .copied()
                        .collect(),
                    // Unions are explicitly unsupported by `#[derive(Reflect)]`, so we don't even
                    // both checking the fields and just set the applicability to "maybe incorrect".
                    ItemKind::Union(..) => {
                        applicability = Applicability::MaybeIncorrect;
                        Vec::new()
                    }
                    // This shouldn't be possible, as only structs, enums, and unions can implement
                    // traits, so panic if this branch is reached.
                    _ => span_unreachable!(
                        without_reflect.item_span,
                        "found a type that implements `Event`, `Component`, `Message`, or `Resource` but is not a struct, enum, or union",
                    ),
                };

                for field in fields {
                    let ty = ty_from_hir_ty(cx, field.ty);

                    // Check if the field's type implements the `PartialReflect` trait. If it does
                    // not, change the `Applicability` level to `MaybeIncorrect` because `Reflect`
                    // cannot be automatically derived.
                    if !reflect_trait_def_ids
                        .iter()
                        .any(|&trait_id| implements_trait(cx, ty, trait_id, &[]))
                    {
                        applicability = Applicability::MaybeIncorrect;
                        break;
                    }
                }

                span_lint_hir_and_then(
                    cx,
                    MISSING_REFLECT,
                    // This tells `rustc` where to search for `#[allow(...)]` attributes.
                    without_reflect.hir_id,
                    without_reflect.item_span,
                    format!("defined {message_phrase} without a `Reflect` implementation"),
                    |diag| {
                        diag.span_note(
                            without_reflect.impl_span,
                            format!("`{trait_name}` implemented here"),
                        )
                        .suggest_item_with_attr(
                            cx,
                            without_reflect.item_span,
                            "`Reflect` can be automatically derived",
                            "#[derive(Reflect)]",
                            // This suggestion may result in two consecutive
                            // `#[derive(...)]` attributes, but `rustfmt` merges them
                            // afterwards.
                            applicability,
                        );
                    },
                );
            }
        }
    }
}
