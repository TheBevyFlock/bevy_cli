# Examples

The CLI makes it easier to build, run, and lint examples by automatically enabling examples' required features. Take Bevy's [`web_asset` example](https://github.com/bevyengine/bevy/blob/v0.18.0/examples/asset/web_asset.rs), for instance. It requires the `https` feature [in `Cargo.toml`](https://github.com/bevyengine/bevy/blob/v0.18.0/Cargo.toml#L2056-L2060):

```toml
[[example]]
name = "web_asset"
path = "examples/asset/web_asset.rs"
required-features = ["https"]
```

Running `cargo build --example web_asset` will fail with Cargo complaining that the `https` feature was not enabled. The Bevy CLI improves this situation by automatically enabling the feature for you:

```sh
# The CLI will automatically add `--feature https`, as that feature is required to run the example.
bevy build --example web_asset

# It also works when building and running for the web.
bevy run --example web_asset web
```

The CLI will automatically enable required features for examples when you run `bevy build`, `bevy run`, and `bevy lint`. Note that it will only do so when you build a specific example using the `--example` flag. If you try to build all examples with `--examples`, the CLI will not do anything extra.
