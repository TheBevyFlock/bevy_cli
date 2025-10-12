use clippy_utils::{diagnostics::span_lint_and_help, source::snippet, ty::ty_from_hir_ty};
use rustc_lint::{LateContext, LateLintPass};

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
        if let Some(bevy_platform_alternative) = BevyPlatformType::try_from_ty(cx, ty) {
            span_lint_and_help(
                cx,
                BEVY_PLATFORM_ALTERNATIVE_EXISTS,
                hir_ty.span,
                BEVY_PLATFORM_ALTERNATIVE_EXISTS.desc,
                None,
                format!(
                    "the type `{}` can be replaced with the `no_std` compatible type {}",
                    snippet(cx.tcx.sess, hir_ty.span, ""),
                    bevy_platform_alternative.full_path()
                ),
            );
        }
    }
}

/// Creates an enum containing all the types form `bevy_platform` as variants.
///
/// # Example
///
/// ```ignore
/// declare_bevy_platform_types! {
///    // The variant name => [`PathLookup`] that matches the equivalent type in the std.
///    CustomType => CUSTOMTYPE,
///    // Optional the module path can be passed too, default is `bevy_platform::<variant_name>`.
///    // If an additional module path is added, it will result in: `bevy_platform::custom::thread::CustomType`.
///    CustomType("custom::thread") => BARRIER,
/// ```
macro_rules! declare_bevy_platform_types {
    (
    $(
        $variant:ident $(($mod_path:expr))? => $path:ident
    )
    ,+$(,)?
    ) => {
        #[derive(Copy, Clone)]
        pub enum BevyPlatformType {
            $(
                $variant,
            )+
        }

        impl BevyPlatformType{
            /// Try to create a [`BevyPlatformType`] from the given [`Ty`].
            pub fn try_from_ty(cx: &LateContext<'_>, ty: rustc_middle::ty::Ty<'_>) -> Option<Self> {
                use crate::paths::bevy_platform_types::*;
                $(
                    if $path.matches_ty(cx, ty) {
                        Some(Self::$variant)
                    } else
                )+
                {
                    None
                }
            }

            ///Returns a string identifying this [`BevyPlatformType`]. This string is suitable for user output.
            pub const fn full_path(&self) -> &'static str {
                match self {
                    $(Self::$variant => concat!("bevy_platform", $("::", $mod_path,)? "::" , stringify!($variant)),)+
                }
            }
        }
    };
}

declare_bevy_platform_types! {
    Barrier("sync") => BARRIER,
    BarrierWaitResult("sync") => BARRIERWAITRESULT,
    HashMap("collection") => HASHMAP,
    HashSet("collection") => HASHSET,
    Instant("time") => INSTANT,
    LazyLock("sync") => LAZYLOCK,
    LockResult("sync") => LOCKRESULT,
    Mutex("sync") => MUTEX,
    MutexGuard("sync") => MUTEXGUARD,
    Once("sync")=> ONCE,
    OnceLock("sync") => ONCELOCK,
    OnceState("sync") => ONCESTATE,
    PoisonError("sync") => POISONERROR,
    RwLock("sync") => RWLOCK,
    RwLockReadGuard("sync") => RWLOCKREADGUARD,
    RwLockWriteGuard("sync") => RWLOCKWRITEGUARD,
    SyncCell("sync") => SYNCCELL,
    SyncUnsafeCell("cell") => SYNCUNSAFECELL,
    TryLockError("sync") => TRYLOCKERROR,
    TryLockResult("sync") => TRYLOCKRESULT,
}
