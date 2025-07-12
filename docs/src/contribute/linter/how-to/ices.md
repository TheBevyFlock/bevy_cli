# Panic / ICE

An internal compiler error, also known as an ICE, happens whenever the linter or `rustc` crashes (usually by panicking). When the linter ICEs, it writes the CLI arguments, the backtrace, and the query trace to both the terminal and a file named `rustc-ice-TIMESTAMP.txt`.

## When is it ok to ICE?

Panicking should be avoid in almost all cases because it halts the entirety of the linter; usually when one lint pass fails, it shouldn't stop all other lint passes in the process. It's much better for lint passes to `return` for cases they cannot handle, because that lets other lints continue their checks.

Additionally, the output printed when the linter ICEs is quite unapproachable due to the large backtrace and lack of color. This can really hurt the user's experience, so panicking should be avoided most of the time.

Panicking, however, should be used to enforce invariants in your code. If a lint pass makes an assumption, and would break horribly if that assumption was incorrect, it should use panicking to verify the assumption is true.

For example, consider the following code. It depends on each input expression corresponding to each input type, so it uses an `assert!` to verify that.

```rust
/// INVARIANT: The `input_exprs` must be expressions passed to the function specified by
/// `fn_def_id`.
fn process_fn_inputs<'tcx>(
    tcx: TyCtxt<'tcx>,
    fn_def_id: DefId,
    input_exprs: &'tcx [Expr<'tcx>],
) {
    let fn_sig = tcx.fn_sig(fn_def_id).instantiate_identity();

    // Find the types of all input arguments to the function.
    let input_tys: &'tcx [Ty<'tcx>] = fn_sig.inputs().skip_binder();

    // There should be the same amount of types as expressions, since they should correspond to
    // each other. If that isn't the case, we should panic.
    assert_eq!(
        input_tys.len(),
        input_exprs.len(),
        "there is a different amount of input expressions and input types for function {fn_def_id}",
    );

    // ...
}
```

## How to ICE

### Use `bevy_lint`'s panicking macros

`bevy_lint` has custom variants of the panicking macros (`panic!`, `assert!`, etc.) that should be preferred, as they provide better user-facing diagnostics.

```rust
// Don't do this.
std::panic!("Uh oh!");

// Do this instead! :)
crate::panic!("Uh oh!");
```

<div class="warning">

`bevy_lint`'s macros require `rustc`'s diagnostics to be setup, so they will not work in tests and the `bevy_lint` binary. In these cases, you should use the standard library versions instead.

</div>

### Panic for a specific `Span`

`bevy_lint` has custom panicking macros that can be emitted to a specific `Span`. These should be preferred in lint passes, as they tell you what piece of code caused the linter to ICE.

```rust
use crate::{assert, span_assert};

fn check(condition: bool, span: Span) {
    // Don't do this.
    assert!(condition);

    // Do this instead! :)
    span_assert!(span, condition);
}
```

### Write a clear error message

When making an assertion, make sure to include a clear error message for when it fails.

```rust
// Don't do this.
assert_eq!(a.len(), b.len());

// Do this instead! :)
assert_eq!(
    a.len(),
    b.len(),
    "{a:?} and {b:?} must be the same length for this lint to continue",
);
```

Remember that these macros support string formatting, just like `format!`, which is really useful for providing context to the error.

### Use debug assertions for expensive checks

If an assertion is particularly slow, you can switch it to its `debug` version, which will only be enabled for non-`--release` builds. This should not be done for cheap assertions, however, since it may lead to invalid states in the program.
