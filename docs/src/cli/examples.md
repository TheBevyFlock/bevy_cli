# Examples

The CLI makes it easy to build, run and lint examples by automatically enabling required features of the example.

## Build and run a specific example

```sh
# Run the `web_asset` example from https://github.com/bevyengine/bevy that requires the feature `https`.
bevy run --example web_asset
info: enabling required_features: ["https"], for example: web_asset

# Build the `web_asset` example in the web.
bevy build --example web_asset web
info: enabling required_features: ["https"], for example: web_asset
```

## Build all examples

The CLI supports building all examples only for the native target, for the web this is not supported yet.
When building all examples, the CLI will build with `--all-features`.

```sh
bevy build --examples
```
