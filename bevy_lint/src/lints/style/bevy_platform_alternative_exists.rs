use clippy_utils::{diagnostics::span_lint_hir_and_then, source::snippet_opt, ty::ty_from_hir_ty};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;

use crate::{declare_bevy_lint, declare_bevy_lint_pass};

declare_bevy_lint! {
    pub(crate) BEVY_PLATFORM_ALTERNATIVE_EXISTS,
    super::Style,
    "Used type from the `std` that has an existing alternative from `bevy_platform`",
}

declare_bevy_lint_pass! {
    pub(crate) BevyPlatformAlternativeExists => [BEVY_PLATFORM_ALTERNATIVE_EXISTS],
}

impl<'tcx> LateLintPass<'tcx> for BevyPlatformAlternativeExists {
    fn check_ty(
        &mut self,
        cx: &LateContext<'tcx>,
        hir_ty: &'tcx rustc_hir::Ty<'tcx, rustc_hir::AmbigArg>,
    ) {
        if hir_ty.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }

        // lower the [`hir::Ty`] to a [`rustc_middle::ty::Ty`]
        let ty = ty_from_hir_ty(cx, hir_ty.as_unambig_ty());

        // Check if for the given `ty` an alternative from `bevy_platform` exists.
        if let Some(bevy_platform_alternative) = BevyPlatformTypes::try_from_ty(cx, ty) {
            span_lint_hir_and_then(
                cx,
                BEVY_PLATFORM_ALTERNATIVE_EXISTS,
                hir_ty.hir_id,
                hir_ty.span,
                BEVY_PLATFORM_ALTERNATIVE_EXISTS.desc,
                |diag| {
                    diag.note(format!(
                        "the type `{}` can be replaced with the no_std compatible type \"bevy_platform::{}\"",
                        snippet_opt(cx.tcx.sess, hir_ty.span),
                        conventional_name_impl.suffix()
                    ));

                    diag.span_suggestion(
                        struct_span,
                        format!("use `{}` instead", struct_name.as_str()),
                        conventional_name_impl.name_suggestion(struct_name.as_str()),
                        Applicability::MaybeIncorrect,
                    );
                },
            );
        }
    }
}

/// Represents all the types in the `bevy_platform` crate that
/// are drop in replacements for the equivalently named std types.
enum BevyPlatformTypes {
    // cell: https://github.com/bevyengine/bevy/blob/v0.17.2/crates/bevy_platform/src/cell/mod.rs
    SyncCell,
    SyncUnsafeCell,
    // collections: https://github.com/bevyengine/bevy/blob/v0.17.2/crates/bevy_platform/src/collections/mod.rs
    HashMap,
    HashSet,
    // sync: https://github.com/bevyengine/bevy/blob/v0.17.2/crates/bevy_platform/src/sync/mod.rs
    Barrier,
    BarrierWaitResult,
    LazyLock,
    Mutex,
    MutexGuard,
    Once,
    OnceLock,
    OnceState,
    LockResult,
    PoisonError,
    TryLockError,
    TryLockResult,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
    // time: https://github.com/bevyengine/bevy/blob/v0.17.2/crates/bevy_platform/src/time/mod.rs
    Instant,
}

impl BevyPlatformTypes {
    fn try_from_ty(cx: &LateContext, ty: Ty) -> Option<Self> {
        use crate::paths::bevy_platform_types::*;

        if SYNCCELL.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::SyncCell)
        } else if SYNCUNSAFECELL.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::SyncUnsafeCell)
        } else if HASHMAP.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::HashMap)
        } else if HASHSET.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::HashSet)
        } else if INSTANT.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::Instant)
        } else if BARRIER.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::Barrier)
        } else if BARRIERWAITRESULT.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::BarrierWaitResult)
        } else if LAZYLOCK.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::LazyLock)
        } else if MUTEX.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::Mutex)
        } else if MUTEXGUARD.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::MutexGuard)
        } else if ONCE.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::Once)
        } else if ONCELOCK.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::OnceLock)
        } else if ONCESTATE.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::OnceState)
        } else if LOCKRESULT.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::LockResult)
        } else if POISONERROR.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::PoisonError)
        } else if TRYLOCKERROR.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::TryLockError)
        } else if TRYLOCKRESULT.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::TryLockResult)
        } else if RWLOCK.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::RwLock)
        } else if RWLOCKREADGUARD.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::RwLockReadGuard)
        } else if RWLOCKWRITEGUARD.matches_ty(cx, ty) {
            Some(BevyPlatformTypes::RwLockWriteGuard)
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        match self {
            BevyPlatformTypes::SyncCell => "SyncCell",
            BevyPlatformTypes::SyncUnsafeCell => "SyncUnsafeCell",
            BevyPlatformTypes::HashMap => "HashMap",
            BevyPlatformTypes::HashSet => "HashSet",
            BevyPlatformTypes::Barrier => "Barrier",
            BevyPlatformTypes::BarrierWaitResult => "BarrierWaitResult",
            BevyPlatformTypes::LazyLock => "LazyLock",
            BevyPlatformTypes::Mutex => "Mutex",
            BevyPlatformTypes::MutexGuard => "MutexGuard",
            BevyPlatformTypes::Once => "Once",
            BevyPlatformTypes::OnceLock => "OnceLock",
            BevyPlatformTypes::OnceState => "OnceState",
            BevyPlatformTypes::LockResult => "LockResult",
            BevyPlatformTypes::PoisonError => "PoisonError",
            BevyPlatformTypes::TryLockError => "TryLockError",
            BevyPlatformTypes::TryLockResult => "TryLockResult",
            BevyPlatformTypes::RwLock => "RwLock",
            BevyPlatformTypes::RwLockReadGuard => "RwLockReadGuard",
            BevyPlatformTypes::RwLockWriteGuard => "RwLockWriteGuard",
            BevyPlatformTypes::Instant => "Instant",
        }
    }
}
