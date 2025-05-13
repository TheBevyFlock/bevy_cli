# Lint Module Docs

All lints, [which can be found here], place their documentation in their module. For example, `missing_reflect`'s docs are in [`src/lints/missing_reflect.rs`]. These docs must adhere to the following format:

````markdown
A single sentence description of what this lint checks for.

In the rare case this needs to be elaborated, an optional paragraph beneath the first sentence is
allowed. Use this to go into more detail on what is checked for, as well as what not is checked
for.

# Motivation

One to several paragraphs describing why this lint exists. For example: does this lint help catch
slow code, or does it suggest a more idiomatic version? Try to motivate why a user may want to
enable this lint.

Be careful to only answer _why_ this lint exists, not _what_ it does or _how_ it works. That is the
responsibility of the extended description.

# Known Issues

This is an optional section that describes what false negatives and false positives this lint may
have. (Usually to justify why a lint is in the `nursery` group, though not always.) If possible,
make sure to link to the issue in the
[issue tracker](https://github.com/TheBevyFlock/bevy_cli/issues) so users can comment on it.

# Example

```
// A snippet of code that would cause the lint to trigger. If a specific line would cause the lint,
// make sure to point it out with a comment.

// Note how this variable is not used:
let x = 10;

// If bodies of functions are not relevant to the lint, use `// ...` to signal that there may be
// other code there.
fn foo() {
    // ...
}
```

Use instead:

```
// The second code snippet should be a "fixed" version of the original. Comments from the original
// do not need to be copied over, but it may be useful to add a note on how or why the lint was
// fixed a specific way.

// `_x` will silence the lint, but `_` also works. You can also delete the line, if you truly do
// not want `10`.
let _x = 10;

fn foo() {
    // ...
}
```

If you wish to elaborate further on how to fix the lint or supply further examples, you may do so
here. If this is a Cargo lint, switch out the Rust code block with a TOML one for `Cargo.toml`.
````

[which can be found here]: https://thebevyflock.github.io/bevy_cli/api/bevy_lint/lints/index.html
[`src/lints/missing_reflect.rs`]: ../../src/lints/missing_reflect.rs
