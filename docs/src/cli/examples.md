# Examples

The CLI makes it easy to build, run and lint examples by automatically enabling required features of the example.

## Build and run a specific example

Take Bevy's `web_asset` example, for instance. It requires the `https` feature in `Cargo.toml`:

```toml
[[example]]
name = "web_asset"
path = "examples/asset/web_asset.rs"
required-features = ["https"]
```

Running `cargo build --example web_asset` will fail with Cargo complaining that the `https` feature was not enabled. The Bevy CLI differs by automatically enabling the feature for you:

```sh
# The CLI will automatically add `--feature https`, as that feature is required to run the example.
bevy run --example web_asset

# It also works when building for the web.
bevy build --example web_asset web
```

## Build all examples

The CLI supports building all examples only for the native target, for the web this is not supported yet.
When building all examples, the CLI will build with `--all-features`.

```sh
bevy build --examples
```
